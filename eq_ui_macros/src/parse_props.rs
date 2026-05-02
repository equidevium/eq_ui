//! Parse component props from a `#[component] fn` signature.

use syn::{FnArg, Ident, Pat, Result, Type, Attribute, Expr};

/// A single component prop extracted from the function signature.
#[derive(Debug)]
pub struct PropInfo {
    pub name: Ident,
    pub ty: Type,
    pub kind: PropKind,
    pub default_expr: Option<Expr>,
    pub skip: bool,
}

/// Classification of a prop for demo control generation.
#[derive(Debug, Clone, PartialEq)]
pub enum PropKind {
    /// `bool` → PropToggle
    Bool,
    /// `String` → PropInput
    String,
    /// `&'static str` → PropInput (value leaked via .leak())
    StaticStr,
    /// Named enum type (e.g. `SwitchSize`) → PropSelect via PlaygroundEnumInfo
    Enum(Ident),
    /// `Option<EventHandler<T>>` or `Option<Callback<T>>` → skip
    Handler,
    /// `Element` or `Option<Element>` or `children` → render with sample content
    Children,
    /// Anything else → skip with a comment
    Unknown,
}

/// Parse all props from the component function's arguments.
pub fn extract_props(sig: &syn::Signature) -> Result<Vec<PropInfo>> {
    let mut props = Vec::new();

    for arg in &sig.inputs {
        let FnArg::Typed(pat_ty) = arg else { continue };

        let name = match pat_ty.pat.as_ref() {
            Pat::Ident(pi) => pi.ident.clone(),
            _ => continue,
        };

        let ty = pat_ty.ty.as_ref().clone();
        let attrs = &pat_ty.attrs;

        let has_playground_skip = attrs.iter().any(|a| {
            a.path().is_ident("playground") && a.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false)
        });

        let default_expr = extract_default_value(attrs);
        let kind = classify_type(&name, &ty);

        let skip = has_playground_skip
            || matches!(kind, PropKind::Handler | PropKind::Unknown);

        props.push(PropInfo {
            name,
            ty,
            kind,
            default_expr,
            skip,
        });
    }

    Ok(props)
}

/// Classify a prop type into a PropKind.
fn classify_type(name: &Ident, ty: &Type) -> PropKind {
    let name_str = name.to_string();

    // `children` is always Children
    if name_str == "children" {
        return PropKind::Children;
    }
    // `class` is always skipped
    if name_str == "class" {
        return PropKind::Unknown;
    }

    let ty_str = type_to_string(ty);

    // bool
    if ty_str == "bool" {
        return PropKind::Bool;
    }

    // String
    if ty_str == "String" {
        return PropKind::String;
    }

    // &'static str
    if ty_str.contains("&'staticstr") || ty_str.contains("&'static str") {
        return PropKind::StaticStr;
    }

    // Option<Element>
    if ty_str.contains("Element") {
        return PropKind::Children;
    }

    // EventHandler / Callback patterns
    if ty_str.contains("EventHandler") || ty_str.contains("Callback") {
        return PropKind::Handler;
    }

    // Option<EventHandler<...>> / Option<Callback<...>>
    if ty_str.starts_with("Option") && (ty_str.contains("EventHandler") || ty_str.contains("Callback")) {
        return PropKind::Handler;
    }

    // If none of the above, assume it's an enum type.
    // Extract the ident from the type.
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let ident = &seg.ident;
            let ident_str = ident.to_string();
            // Skip common non-enum types
            if !["Option", "Vec", "Box", "Rc", "Arc", "Element"].contains(&ident_str.as_str()) {
                return PropKind::Enum(ident.clone());
            }
        }
    }

    PropKind::Unknown
}

/// Flatten a type to a string for simple pattern matching.
fn type_to_string(ty: &Type) -> String {
    quote::quote!(#ty).to_string().replace(' ', "")
}

/// Extract the default value from `#[props(default = VALUE)]`.
fn extract_default_value(attrs: &[Attribute]) -> Option<Expr> {
    for attr in attrs {
        if !attr.path().is_ident("props") {
            continue;
        }
        // Parse the content inside #[props(...)]
        let Ok(meta) = attr.parse_args::<syn::Meta>() else {
            // Try parsing as a list of metas
            if let Ok(nested) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            ) {
                for m in &nested {
                    if let syn::Meta::NameValue(nv) = m {
                        if nv.path.is_ident("default") {
                            return Some(nv.value.clone());
                        }
                    }
                }
            }
            continue;
        };
        if let syn::Meta::NameValue(nv) = meta {
            if nv.path.is_ident("default") {
                return Some(nv.value);
            }
        }
    }
    None
}
