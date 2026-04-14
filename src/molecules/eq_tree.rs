use super::eq_tree_styles as s;
use crate::theme::merge_classes;
use dioxus::document;
use dioxus::prelude::*;
use std::collections::HashSet;

#[cfg(feature = "playground")]
use crate::playground::playground_helpers::{
    CodeBlock, DemoSection, PropToggle, StyleInfo, format_catalog,
};
#[cfg(feature = "playground")]
use crate::atoms::{EqText, TextVariant, EqScrollableSpace};
#[cfg(feature = "playground")]
use crate::playground::playground_types::{ComponentDescriptor, ComponentCategory, UsageExample};

// ---------------------------------------------------------------------------
// TreeNode data model
// ---------------------------------------------------------------------------

/// A node in a tree structure.
///
/// Each node carries an `id`, a display `label`, an optional `parent_id`
/// (`None` for root-level nodes), and a list of `children`.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub parent_id: Option<String>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    // -- Construction -------------------------------------------------------

    /// Create a leaf node (no children, no parent).
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            parent_id: None,
            children: Vec::new(),
        }
    }

    /// Create a branch node with children.
    /// Automatically sets `parent_id` on each child.
    pub fn new_with_children(
        id: impl Into<String>,
        label: impl Into<String>,
        children: Vec<TreeNode>,
    ) -> Self {
        let id = id.into();
        let children = children
            .into_iter()
            .map(|mut child| {
                child.parent_id = Some(id.clone());
                child
            })
            .collect();
        Self {
            id,
            label: label.into(),
            parent_id: None,
            children,
        }
    }

    // -- Lookup -------------------------------------------------------------

    /// Find a node by its id anywhere in the subtree (including self).
    pub fn find_by_id(&self, target: &str) -> Option<&TreeNode> {
        if self.id == target {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_by_id(target) {
                return Some(found);
            }
        }
        None
    }

    /// Find the direct parent of the node with the given id.
    /// Returns `None` if the id is the current node or not found.
    pub fn find_parent_of(&self, target: &str) -> Option<&TreeNode> {
        for child in &self.children {
            if child.id == target {
                return Some(self);
            }
            if let Some(found) = child.find_parent_of(target) {
                return Some(found);
            }
        }
        None
    }

    /// Return the path from this node down to the target id (inclusive).
    /// Returns an empty vec if the target is not found.
    pub fn find_path_to(&self, target: &str) -> Vec<&TreeNode> {
        if self.id == target {
            return vec![self];
        }
        for child in &self.children {
            let mut path = child.find_path_to(target);
            if !path.is_empty() {
                path.insert(0, self);
                return path;
            }
        }
        Vec::new()
    }

    // -- Traversal ----------------------------------------------------------

    /// Flatten the entire subtree into a depth-first ordered vec.
    pub fn flatten(&self) -> Vec<&TreeNode> {
        let mut result = vec![self];
        for child in &self.children {
            result.extend(child.flatten());
        }
        result
    }

    /// Get the depth of a node relative to this node (self = 0).
    /// Returns `None` if the target is not in this subtree.
    pub fn depth_of(&self, target: &str) -> Option<usize> {
        if self.id == target {
            return Some(0);
        }
        for child in &self.children {
            if let Some(d) = child.depth_of(target) {
                return Some(d + 1);
            }
        }
        None
    }

    /// Whether this node has no children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Count all leaf nodes in this subtree.
    pub fn leaf_count(&self) -> usize {
        if self.is_leaf() {
            1
        } else {
            self.children.iter().map(|c| c.leaf_count()).sum()
        }
    }

    // -- Mutation -----------------------------------------------------------

    /// Add a child (leaf or branch) directly to this node.
    /// Automatically sets the child's `parent_id` to this node's id.
    pub fn add_child(&mut self, mut child: TreeNode) {
        child.parent_id = Some(self.id.clone());
        self.children.push(child);
    }

    /// Find a node anywhere in the subtree by `parent_id` and add a child to it.
    /// Returns `Ok(())` if the parent was found and the child was inserted,
    /// or `Err(child)` if the parent was not found (returns the child back).
    pub fn add_child_to(&mut self, parent_id: &str, child: TreeNode) -> Result<(), TreeNode> {
        if self.id == parent_id {
            self.add_child(child);
            return Ok(());
        }
        let mut orphan = child;
        for c in &mut self.children {
            match c.add_child_to(parent_id, orphan) {
                Ok(()) => return Ok(()),
                Err(returned) => orphan = returned,
            }
        }
        Err(orphan)
    }

    /// Remove a node by id from anywhere in the subtree.
    /// Returns the removed node if found.
    pub fn remove_node(&mut self, target_id: &str) -> Option<TreeNode> {
        if let Some(pos) = self.children.iter().position(|c| c.id == target_id) {
            return Some(self.children.remove(pos));
        }
        for child in &mut self.children {
            if let Some(removed) = child.remove_node(target_id) {
                return Some(removed);
            }
        }
        None
    }

    /// Remove all children from this node, turning it into a leaf.
    /// Returns the removed children.
    pub fn empty_node(&mut self) -> Vec<TreeNode> {
        std::mem::take(&mut self.children)
    }

    /// Move a node from its current position to a new parent within the tree.
    /// Returns `true` if the node was found and moved successfully.
    /// Returns `false` if the node or new parent was not found,
    /// or if the target would be moved under itself (cycle).
    pub fn move_node(&mut self, node_id: &str, new_parent_id: &str) -> bool {
        // Prevent moving a node under itself
        if node_id == new_parent_id {
            return false;
        }
        // Check the node being moved doesn't contain the target parent (cycle)
        if let Some(node) = self.find_by_id(node_id) {
            if node.find_by_id(new_parent_id).is_some() {
                return false;
            }
        }
        // Remove the node first, then insert under the new parent
        if let Some(removed) = self.remove_node(node_id) {
            if self.add_child_to(new_parent_id, removed).is_ok() {
                return true;
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// ARIA helpers – visible-node collection & parent lookup
// ---------------------------------------------------------------------------

/// Collect all currently visible nodes (depth-first) given the set of expanded
/// branch IDs. Returns `(node_id, depth, is_leaf)` tuples.
fn collect_visible(
    nodes: &[TreeNode],
    expanded: &HashSet<String>,
    depth: usize,
    out: &mut Vec<(String, usize, bool)>,
) {
    for node in nodes {
        let is_leaf = node.is_leaf();
        out.push((node.id.clone(), depth, is_leaf));
        if !is_leaf && expanded.contains(&node.id) {
            collect_visible(&node.children, expanded, depth + 1, out);
        }
    }
}

/// Walk the tree to find the direct parent of `target`. Returns `None` for
/// root-level nodes.
fn find_parent_id(nodes: &[TreeNode], target: &str) -> Option<String> {
    for node in nodes {
        if node.children.iter().any(|c| c.id == target) {
            return Some(node.id.clone());
        }
        if let Some(found) = find_parent_id(&node.children, target) {
            return Some(found);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// EqTree component
// ---------------------------------------------------------------------------

/// A collapsible tree view molecule.
///
/// Renders a list of `TreeNode`s with expand/collapse for branches
/// and click-to-select for leaves.
///
/// **Accessibility** – implements the WAI-ARIA [Tree View][tv] pattern:
/// `role="tree"` on the root, `role="treeitem"` on each node,
/// `role="group"` on children containers, `aria-expanded` / `aria-selected`
/// / `aria-level` / `aria-setsize` / `aria-posinset` on every item,
/// roving `tabindex` with programmatic focus, and full keyboard navigation
/// (Up / Down / Left / Right / Home / End / Enter / Space).
///
/// [tv]: https://www.w3.org/WAI/ARIA/apg/patterns/treeview/
#[component]
pub fn EqTree(
    /// The root-level nodes to display.
    nodes: Vec<TreeNode>,
    /// Fires with the node `id` when a leaf is clicked.
    on_select: EventHandler<String>,
    /// The currently selected node id (gets highlight style).
    #[props(into, default)]
    selected: Option<String>,
    /// When `true`, branch nodes show their direct child count, e.g. "Atoms (8)".
    #[props(default)]
    show_count: bool,
    /// Accessible label for screen readers (e.g. "File browser",
    /// "Component list"). Announced as "{label}, tree".
    #[props(into, default = "Tree".to_string())]
    aria_label: String,
    /// Optional class override - extend or replace default wrapper styles.
    #[props(into, default)]
    class: String,
) -> Element {
    // Stable unique prefix so every DOM id is unique even with multiple trees.
    let tree_id_prefix = use_hook(|| {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("eq-tree-{id}")
    });

    // Shared expansion state — replaces per-branch `use_signal(|| false)`.
    let mut expanded_set: Signal<HashSet<String>> = use_signal(|| HashSet::new());

    // Roving tabindex: the focused node id.
    let mut focused_id: Signal<String> = use_signal(|| String::new());

    let cls = merge_classes(s::TREE, &class);

    // Pre-compute values needed by the keyboard closure and the rsx iterator.
    let total_roots = nodes.len();
    let first_id = nodes.first().map(|n| n.id.clone()).unwrap_or_default();

    // Clone nodes for the keyboard handler closure (it needs its own copy).
    let nodes_kb = nodes.clone();
    let prefix_kb = tree_id_prefix.clone();

    rsx! {
        div {
            class: "{cls}",
            role: "tree",
            "aria-label": "{aria_label}",

            // ── Keyboard navigation ──────────────────────────────
            onkeydown: move |evt: Event<KeyboardData>| {
                let key = evt.key();
                let expanded = expanded_set();
                let mut visible: Vec<(String, usize, bool)> = Vec::new();
                collect_visible(&nodes_kb, &expanded, 1, &mut visible);
                if visible.is_empty() { return; }

                let cur = focused_id();
                let cur_idx = visible.iter().position(|(id, _, _)| *id == cur);

                // Helper: focus a node by index in the visible list.
                let mut focus = |idx: usize| {
                    let target = &visible[idx].0;
                    focused_id.set(target.clone());
                    let dom_id = format!("{}-{}", prefix_kb, target);
                    document::eval(&format!(
                        "document.getElementById('{dom_id}')?.focus()"
                    ));
                };

                if key == Key::ArrowDown {
                    evt.prevent_default();
                    match cur_idx {
                        Some(i) if i + 1 < visible.len() => focus(i + 1),
                        None => focus(0),
                        _ => {}
                    }
                } else if key == Key::ArrowUp {
                    evt.prevent_default();
                    match cur_idx {
                        Some(i) if i > 0 => focus(i - 1),
                        None => focus(visible.len() - 1),
                        _ => {}
                    }
                } else if key == Key::ArrowRight {
                    evt.prevent_default();
                    if let Some(i) = cur_idx {
                        let (ref id, _, is_leaf) = visible[i];
                        if !is_leaf {
                            if !expanded.contains(id) {
                                // Expand (focus stays on current node per WAI-ARIA).
                                let mut set = expanded_set();
                                set.insert(id.clone());
                                expanded_set.set(set);
                            } else if i + 1 < visible.len() && visible[i + 1].1 > visible[i].1 {
                                // Already expanded — move to first child.
                                focus(i + 1);
                            }
                        }
                    }
                } else if key == Key::ArrowLeft {
                    evt.prevent_default();
                    if let Some(i) = cur_idx {
                        let (ref id, _, is_leaf) = visible[i];
                        if !is_leaf && expanded.contains(id) {
                            // Collapse (focus stays).
                            let mut set = expanded_set();
                            set.remove(id);
                            expanded_set.set(set);
                        } else {
                            // Move to parent.
                            if let Some(parent) = find_parent_id(&nodes_kb, id) {
                                if let Some(pi) = visible.iter().position(|(pid, _, _)| *pid == parent) {
                                    focus(pi);
                                }
                            }
                        }
                    }
                } else if key == Key::Home {
                    evt.prevent_default();
                    focus(0);
                } else if key == Key::End {
                    evt.prevent_default();
                    focus(visible.len() - 1);
                } else if key == Key::Enter || key == Key::Character(" ".into()) {
                    evt.prevent_default();
                    if let Some(i) = cur_idx {
                        let (ref id, _, is_leaf) = visible[i];
                        if is_leaf {
                            on_select.call(id.clone());
                        } else {
                            let mut set = expanded_set();
                            if set.contains(id) {
                                set.remove(id);
                            } else {
                                set.insert(id.clone());
                            }
                            expanded_set.set(set);
                        }
                    }
                }
            },

            for (i, node) in nodes.into_iter().enumerate() {
                TreeBranch {
                    key: "{node.id}",
                    node: node,
                    on_select: on_select,
                    selected: selected.clone(),
                    show_count: show_count,
                    expanded_set: expanded_set,
                    focused_id: focused_id,
                    tree_id_prefix: tree_id_prefix.clone(),
                    first_node_id: first_id.clone(),
                    depth: 1,
                    set_size: total_roots,
                    pos_in_set: i + 1,
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Internal recursive renderer
// ---------------------------------------------------------------------------

#[component]
fn TreeBranch(
    node: TreeNode,
    on_select: EventHandler<String>,
    selected: Option<String>,
    show_count: bool,
    /// Shared expansion state managed by EqTree.
    expanded_set: Signal<HashSet<String>>,
    /// Currently focused node id (roving tabindex).
    focused_id: Signal<String>,
    /// DOM-id prefix unique to this tree instance.
    tree_id_prefix: String,
    /// ID of the first root node (fallback focus target).
    first_node_id: String,
    /// 1-based depth for `aria-level`.
    depth: usize,
    /// Number of siblings at this level (`aria-setsize`).
    set_size: usize,
    /// 1-based position among siblings (`aria-posinset`).
    pos_in_set: usize,
) -> Element {
    let is_leaf = node.is_leaf();
    let is_selected = selected.as_deref() == Some(node.id.as_str());
    let is_expanded = !is_leaf && expanded_set().contains(&node.id);

    // Roving tabindex: the focused node gets 0, everything else -1.
    // When nothing is focused yet the first root node wins.
    let cur_focused = focused_id();
    let is_focus_target = if cur_focused.is_empty() {
        node.id == first_node_id
    } else {
        cur_focused == node.id
    };

    let row_class = if is_selected { s::NODE_ACTIVE } else { s::NODE_ROW };

    let node_id_click = node.id.clone();
    let label = node.label.clone();
    let child_count = node.children.len();
    let chevron_rotate = if is_expanded { s::CHEVRON_EXPANDED } else { "" };
    let dom_id = format!("{}-{}", tree_id_prefix, node.id);
    let tab_idx = if is_focus_target { "0" } else { "-1" };

    rsx! {
        div {
            role: "treeitem",
            id: "{dom_id}",
            "aria-expanded": if !is_leaf { if is_expanded { "true" } else { "false" } } else { "" },
            "aria-selected": if is_selected { "true" } else { "false" },
            "aria-level": "{depth}",
            "aria-setsize": "{set_size}",
            "aria-posinset": "{pos_in_set}",
            "aria-label": "{label}",
            tabindex: "{tab_idx}",

            // Node row (visual only — interaction handled by treeitem above via keyboard)
            div {
                class: row_class,
                onclick: move |_| {
                    focused_id.set(node_id_click.clone());
                    if is_leaf {
                        on_select.call(node_id_click.clone());
                    } else {
                        let mut set = expanded_set();
                        if set.contains(&node_id_click) {
                            set.remove(&node_id_click);
                        } else {
                            set.insert(node_id_click.clone());
                        }
                        expanded_set.set(set);
                    }
                },

                // Chevron or spacer
                if !is_leaf {
                    svg {
                        class: "{s::CHEVRON} {chevron_rotate}",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        stroke: "currentColor",
                        "aria-hidden": "true",
                        path { d: "m9 5 7 7-7 7" }
                    }
                } else {
                    span { class: s::LEAF_SPACER, "aria-hidden": "true" }
                }

                // Label
                span { class: s::LABEL, "{label}" }

                // Direct child count for branches
                if show_count && !is_leaf {
                    span { class: s::COUNT, "aria-hidden": "true", "({child_count})" }
                }
            }

            // Children (shown when expanded)
            if !is_leaf && is_expanded {
                div {
                    class: s::CHILDREN,
                    role: "group",
                    for (i, child) in node.children.into_iter().enumerate() {
                        TreeBranch {
                            key: "{child.id}",
                            node: child,
                            on_select: on_select,
                            selected: selected.clone(),
                            show_count: show_count,
                            expanded_set: expanded_set,
                            focused_id: focused_id,
                            tree_id_prefix: tree_id_prefix.clone(),
                            first_node_id: first_node_id.clone(),
                            depth: depth + 1,
                            set_size: child_count,
                            pos_in_set: i + 1,
                        }
                    }
                }
            }
        }
    }
}

// ── Playground descriptor ──────────────────────────────────────────

#[cfg(feature = "playground")]
pub fn descriptor() -> ComponentDescriptor {
    ComponentDescriptor {
        id: "eq-tree",
        name: "EqTree",
        category: ComponentCategory::Molecule,
        description: "Collapsible tree view for hierarchical data. Branches expand to show children, \
                      leaves trigger selection events. Optional child count display.",
        style_tokens: || s::catalog(),
        usage_examples: || vec![
            UsageExample {
                label: "Basic",
                code: "let nodes = vec![\n    TreeNode::new_with_children(\"branch\", \"Branch\", vec![\n        TreeNode::new(\"leaf-1\", \"Leaf 1\"),\n        TreeNode::new(\"leaf-2\", \"Leaf 2\"),\n    ]),\n];\n\nEqTree {\n    nodes: nodes,\n    on_select: move |id: String| { /* handle */ },\n}".into(),
            },
            UsageExample {
                label: "With counts",
                code: "EqTree {\n    nodes,\n    selected: selected(),\n    on_select: move |id: String| selected.set(Some(id)),\n    show_count: true,\n}".into(),
            },
        ],
        render_demo: || rsx! { DemoEqTree {} },
        render_gallery: || rsx! { GalleryEqTree {} },
    }
}

// ── Interactive demo ───────────────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn DemoEqTree() -> Element {
    let mut selected = use_signal(|| Option::<String>::None);
    let mut show_count = use_signal(|| true);

    let tree = vec![
        TreeNode::new_with_children(
            "demo-branch-a",
            "Branch A",
            vec![
                TreeNode::new("leaf-1", "Leaf 1"),
                TreeNode::new("leaf-2", "Leaf 2"),
                TreeNode::new_with_children(
                    "nested-branch",
                    "Nested Branch",
                    vec![TreeNode::new("deep-leaf", "Deep Leaf")],
                ),
            ],
        ),
        TreeNode::new_with_children(
            "demo-branch-b",
            "Branch B",
            vec![
                TreeNode::new("leaf-3", "Leaf 3"),
                TreeNode::new("leaf-4", "Leaf 4"),
            ],
        ),
    ];

    let code = "let nodes = vec![\n    TreeNode::new_with_children(\"branch\", \"Branch\", vec![\n        TreeNode::new(\"leaf-1\", \"Leaf 1\"),\n        TreeNode::new(\"leaf-2\", \"Leaf 2\"),\n    ]),\n];\n\nEqTree {\n    nodes: nodes,\n    selected: selected(),\n    on_select: move |id: String| selected.set(Some(id)),\n    show_count: true,\n}".to_string();

    rsx! {
        DemoSection { title: "EqTree",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-3",
                EqText {
                    variant: TextVariant::Caption,
                    class: "font-semibold uppercase tracking-wider",
                    "Props"
                }
                PropToggle {
                    label: "show_count",
                    value: show_count(),
                    onchange: move |v: bool| show_count.set(v),
                }
            }
            div { class: "flex gap-6",
                div { class: "w-64 h-64 flex flex-col border border-[var(--color-card-border)] rounded-lg p-3",
                    EqScrollableSpace {
                        EqTree {
                            nodes: tree,
                            selected: selected(),
                            on_select: move |id: String| selected.set(Some(id)),
                            show_count: show_count(),
                        }
                    }
                }
                div { class: "flex-1 flex items-center justify-center rounded-lg border border-[var(--color-card-border)] p-6 min-h-[16rem]",
                    if let Some(id) = selected() {
                        EqText { variant: TextVariant::H3, "Selected: {id}" }
                    } else {
                        EqText { variant: TextVariant::Muted, "Click a leaf node" }
                    }
                }
            }
            StyleInfo { file: "eq_tree_styles.rs", styles: format_catalog(&s::catalog()) }
            CodeBlock { code }
        }
    }
}

// ── Gallery (compact showcase) ─────────────────────────────────────

#[cfg(feature = "playground")]
#[component]
fn GalleryEqTree() -> Element {
    let mut selected = use_signal(|| Option::<String>::None);

    let tree = vec![
        TreeNode::new_with_children(
            "section-a",
            "Section A",
            vec![
                TreeNode::new("item-a1", "Item A1"),
                TreeNode::new("item-a2", "Item A2"),
            ],
        ),
        TreeNode::new_with_children(
            "section-b",
            "Section B",
            vec![
                TreeNode::new("item-b1", "Item B1"),
                TreeNode::new_with_children(
                    "subsection",
                    "Subsection",
                    vec![TreeNode::new("item-b2", "Item B2")],
                ),
            ],
        ),
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "rounded-lg border border-[var(--color-card-border)] p-4 space-y-4",
                EqText { variant: TextVariant::Caption, class: "font-semibold uppercase tracking-wider", "Tree with Count" }

                div { class: "flex gap-4",
                    div { class: "w-48 h-40 border border-[var(--color-card-border)] rounded-lg p-3 overflow-auto",
                        EqTree {
                            nodes: tree.clone(),
                            selected: selected(),
                            on_select: move |id: String| selected.set(Some(id)),
                            show_count: true,
                        }
                    }
                    div { class: "flex-1 flex items-center justify-center text-sm text-[var(--color-label-secondary)]",
                        if selected().is_some() {
                            "Selected: {selected().unwrap_or_default()}"
                        } else {
                            "Select a node"
                        }
                    }
                }
            }
        }
    }
}
