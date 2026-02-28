use super::eq_tree_styles as s;
use dioxus::prelude::*;

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
    /// Returns `true` if the parent was found and the child was inserted.
    pub fn add_child_to(&mut self, parent_id: &str, child: TreeNode) -> bool {
        if self.id == parent_id {
            self.add_child(child);
            return true;
        }
        for c in &mut self.children {
            if c.add_child_to(parent_id, child.clone()) {
                return true;
            }
        }
        false
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
        // Remove the node first
        if let Some(removed) = self.remove_node(node_id) {
            // Insert under the new parent
            if self.add_child_to(new_parent_id, removed) {
                return true;
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// EqTree component
// ---------------------------------------------------------------------------

/// A collapsible tree view molecule.
///
/// Renders a list of `TreeNode`s with expand/collapse for branches
/// and click-to-select for leaves.
#[component]
pub fn EqTree(
    /// The root-level nodes to display.
    nodes: Vec<TreeNode>,
    /// Fires with the node `id` when a leaf is clicked.
    on_select: EventHandler<String>,
    /// The currently selected node id (gets highlight style).
    #[props(into, default)]
    selected: Option<String>,
) -> Element {
    rsx! {
        div { class: s::TREE,
            for node in nodes {
                TreeBranch {
                    key: "{node.id}",
                    node: node,
                    on_select: on_select,
                    selected: selected.clone(),
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
) -> Element {
    let is_leaf = node.is_leaf();
    let is_selected = selected.as_deref() == Some(node.id.as_str());
    let mut expanded = use_signal(|| false);

    let row_class = if is_selected { s::NODE_ACTIVE } else { s::NODE_ROW };

    let node_id = node.id.clone();
    let label = node.label.clone();

    let chevron_rotate = if expanded() { s::CHEVRON_EXPANDED } else { "" };

    rsx! {
        div {
            // Node row
            div {
                class: row_class,
                onclick: move |_| {
                    if is_leaf {
                        on_select.call(node_id.clone());
                    } else {
                        expanded.set(!expanded());
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
                        path { d: "m9 5 7 7-7 7" }
                    }
                } else {
                    span { class: s::LEAF_SPACER }
                }

                // Label
                span { class: s::LABEL, "{label}" }
            }

            // Children (shown when expanded)
            if !is_leaf && expanded() {
                div { class: s::CHILDREN,
                    for child in node.children {
                        TreeBranch {
                            key: "{child.id}",
                            node: child,
                            on_select: on_select,
                            selected: selected.clone(),
                        }
                    }
                }
            }
        }
    }
}
