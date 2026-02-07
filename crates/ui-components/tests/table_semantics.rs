use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn table_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/table/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Table internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn table_uses_logic_state_model() {
    let logic_source = load_source("src/table/logic.rs");
    let view_source = load_source("src/table/view.rs");

    for needle in [
        "pub enum TableVariant",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_empty_text(",
        "pub fn normalize_columns(",
        "pub fn normalize_rows(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Table logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_columns(columns)",
        "logic::normalize_rows(rows, columns.len())",
        "logic::normalize_optional_text(caption)",
        "logic::normalize_empty_text(empty_label)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(TableStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Table view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn table_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/table/view.rs");

    for attr in [
        "data-slot=\"table\"",
        "data-variant=state.variant_attr",
        "data-density=state.density_attr",
        "data-layout=state.layout_attr",
        "data-state=state.data_state_attr",
        "data-striped=state.is_striped.then_some(\"true\")",
        "data-sticky-header=state.has_sticky_header.then_some(\"true\")",
        "data-has-caption=state.has_caption.then_some(\"true\")",
        "data-row-count=state.row_count.to_string()",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-slot=\"table-element\"",
        "data-slot=\"table-head\"",
        "data-slot=\"table-head-row\"",
        "data-slot=\"table-head-cell\"",
        "data-slot=\"table-body\"",
        "data-slot=\"table-row\"",
        "data-slot=\"table-cell\"",
        "role=\"region\"",
    ] {
        assert!(
            source.contains(attr),
            "Table should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn table_styles_include_variant_density_layout_and_markers() {
    let source = load_source("src/table/styles.rs");

    for selector in [
        ".ui-table--variant-default",
        ".ui-table[data-variant=\"default\"]",
        ".ui-table--variant-quiet",
        ".ui-table--variant-outline",
        ".ui-table--density-comfortable",
        ".ui-table[data-density=\"comfortable\"]",
        ".ui-table--density-compact",
        ".ui-table[data-density=\"compact\"]",
        ".ui-table--layout-auto",
        ".ui-table[data-layout=\"auto\"]",
        ".ui-table--layout-fixed",
        ".ui-table[data-layout=\"fixed\"]",
        ".ui-table--striped",
        ".ui-table[data-striped=\"true\"]",
        ".ui-table--sticky-header",
        ".ui-table[data-sticky-header=\"true\"]",
        ".ui-table--with-caption",
        ".ui-table[data-has-caption=\"true\"]",
        ".ui-table--empty",
        ".ui-table[data-state=\"empty\"]",
        ".ui-table--custom-class",
        ".ui-table[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Table styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
