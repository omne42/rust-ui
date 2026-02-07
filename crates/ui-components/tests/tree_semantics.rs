use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tree_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tree/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tree internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tree_uses_logic_state_model() {
    let logic_source = load_source("src/tree/logic.rs");
    let view_source = load_source("src/tree/view.rs");

    for needle in [
        "pub enum TreeTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_nodes(",
        "pub fn collect_all_ids(",
        "pub fn collect_expandable_ids(",
        "pub fn sanitize_expanded_ids(",
        "pub fn sanitize_selected_id(",
        "pub fn toggle_expanded(",
        "pub fn flatten_visible_nodes(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tree logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_nodes(nodes)",
        "logic::collect_all_ids(&nodes)",
        "logic::collect_expandable_ids(&nodes)",
        "logic::flatten_visible_nodes(&nodes, &expanded_ids, selected_id.as_deref(), disabled)",
        "logic::resolve_state(TreeStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn tree_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/tree/view.rs");

    for attr in [
        "data-slot=\"tree\"",
        "data-tone=move || state.get().tone_attr",
        "data-density=move || state.get().density_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-node-count=move || state.get().node_count.to_string()",
        "data-visible-count=move || state.get().visible_count.to_string()",
        "data-expanded-count=move || state.get().expanded_count.to_string()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"tree-list\"",
        "data-slot=\"tree-node\"",
        "data-slot=\"tree-item\"",
        "data-slot=\"tree-chevron\"",
        "data-slot=\"tree-label\"",
        "role=\"tree\"",
        "role=\"treeitem\"",
    ] {
        assert!(
            source.contains(attr),
            "Tree should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn tree_styles_include_tone_density_depth_and_state_markers() {
    let source = load_source("src/tree/styles.rs");

    for selector in [
        ".ui-tree--tone-default",
        ".ui-tree[data-tone=\"default\"]",
        ".ui-tree--tone-quiet",
        ".ui-tree--tone-strong",
        ".ui-tree--density-comfortable",
        ".ui-tree[data-density=\"comfortable\"]",
        ".ui-tree--density-compact",
        ".ui-tree[data-density=\"compact\"]",
        ".ui-tree--disabled",
        ".ui-tree[data-disabled=\"true\"]",
        ".ui-tree--has-selection",
        ".ui-tree[data-has-selection=\"true\"]",
        ".ui-tree--custom-class",
        ".ui-tree[data-custom-class=\"true\"]",
        ".ui-tree__item--depth-0",
        ".ui-tree__item--depth-1",
        ".ui-tree__item--depth-5-plus",
        ".ui-tree__item--selected",
        ".ui-tree__item[data-selected=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tree styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
