use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn grid_does_not_expose_logic_or_render_modules() {
    let source = load_source("src/grid/mod.rs");

    for needle in ["pub mod logic", "pub mod render"] {
        assert!(
            !source.contains(needle),
            "Grid internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn grid_uses_logic_state_model() {
    let logic_source = load_source("src/grid/logic.rs");
    let render_source = load_source("src/grid/render.rs");

    for needle in [
        "pub enum GridColumns",
        "pub enum GridRows",
        "pub enum GridGap",
        "pub enum GridJustify",
        "pub enum GridAlign",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Grid logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(GridStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            render_source.contains(needle),
            "Grid render should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn grid_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/grid/render.rs");

    for attr in [
        "data-slot=\"grid\"",
        "data-columns=move || state.get().columns_attr",
        "data-rows=move || state.get().rows_attr",
        "data-gap=move || state.get().gap_attr",
        "data-justify=move || state.get().justify_attr",
        "data-align=move || state.get().align_attr",
        "data-dense=move || state.get().is_dense.then_some(\"true\")",
        "data-inline=move || state.get().is_inline.then_some(\"true\")",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Grid should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn grid_styles_include_layout_state_markers() {
    let source = load_source("src/grid/styles.rs");

    for selector in [
        ".ui-grid--inline",
        ".ui-grid[data-inline=\"true\"]",
        ".ui-grid--dense",
        ".ui-grid[data-dense=\"true\"]",
        ".ui-grid--columns-3",
        ".ui-grid[data-columns=\"auto-fit\"]",
        ".ui-grid--rows-equal",
        ".ui-grid[data-rows=\"compact\"]",
        ".ui-grid--justify-center",
        ".ui-grid[data-justify=\"stretch\"]",
        ".ui-grid--align-end",
        ".ui-grid[data-align=\"start\"]",
        ".ui-grid--gap-md",
        ".ui-grid[data-gap=\"none\"]",
        ".ui-grid--custom-class",
        ".ui-grid[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Grid styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn grid_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn grid() -> AnyView",
        "title=\"Grid\"",
        "slug=\"grid\"",
        "description=\"Spectrum-style grid layout primitive with centralized columns/rows/gap/alignment normalization and stable state-marker contracts.\"",
        "<Playground title=\"Columns + Gap\" code=columns_code>",
        "<Playground title=\"AutoFit + Dense + Equal Rows\" code=adaptive_code>",
        "<Grid",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra grid docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn grid_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Columns + Gap\"",
        "columns=GridColumns::Three",
        "gap=GridGap::Md",
        "aria_label=\"Overview cards grid\".to_string()",
        "title=\"AutoFit + Dense + Equal Rows\"",
        "columns=GridColumns::AutoFit",
        "rows=GridRows::Equal",
        "gap=GridGap::Lg",
        "justify=GridJustify::Stretch",
        "align=GridAlign::Stretch",
        "dense=true",
        "class_name=\"docs-grid-adaptive\".to_string()",
        "\"Revenue\"",
        "\"Users\"",
        "\"Latency\"",
        "\"Errors\"",
    ] {
        assert!(
            source.contains(needle),
            "grid docs playgrounds should contain `{needle}`."
        );
    }
}
