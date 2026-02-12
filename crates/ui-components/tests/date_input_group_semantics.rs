use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_input_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/date_input_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateInputGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_input_group_uses_logic_state_model() {
    let logic_source = load_source("src/date_input_group/logic.rs");
    let view_source = load_source("src/date_input_group/view.rs");

    for needle in [
        "pub enum DateInputGroupVariant",
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
            "DateInputGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(DateInputGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_input_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/date_input_group/view.rs");

    for attr in [
        "data-slot=\"date-input-group\"",
        "data-slot=\"date-input-group-prefix\"",
        "data-slot=\"date-input-group-input\"",
        "data-slot=\"date-input-group-segment\"",
        "data-slot=\"date-input-group-suffix\"",
        "data-variant=move || state.get().variant_attr",
        "data-width=move || state.get().width_attr",
        "data-state=move || state.get().data_state_attr",
        "data-full-width=move || state.get().is_full_width.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-segmented=move || state.get().is_segmented.then_some(\"true\")",
        "data-has-prefix=move || state.get().has_prefix.then_some(\"true\")",
        "data-has-suffix=move || state.get().has_suffix.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "DateInputGroup should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn date_input_group_styles_include_variant_and_state_markers() {
    let source = load_source("src/date_input_group/styles.rs");

    for selector in [
        ".ui-date-input-group--variant-primary",
        ".ui-date-input-group[data-variant=\"secondary\"]",
        ".ui-date-input-group--full-width",
        ".ui-date-input-group[data-width=\"full\"]",
        ".ui-date-input-group--disabled",
        ".ui-date-input-group[data-disabled=\"true\"]",
        ".ui-date-input-group--invalid",
        ".ui-date-input-group[data-invalid=\"true\"]",
        ".ui-date-input-group--segmented .ui-date-input-group__segment",
        ".ui-date-input-group[data-segmented=\"true\"] .ui-date-input-group__segment",
        ".ui-date-input-group--custom-class",
        ".ui-date-input-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "DateInputGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_input_group_supports_group_accessibility_and_children_layout() {
    let source = load_source("src/date_input_group/view.rs");

    for needle in [
        "<div",
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-disabled=disabled.then_some(\"true\")",
        "{children()}",
    ] {
        assert!(
            source.contains(needle),
            "DateInputGroup should include `{needle}` for accessibility and composition."
        );
    }
}

#[test]
fn date_input_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn date_input_group() -> AnyView",
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "description=\"Spectrum/HeroUI-style date-input grouping primitive with centralized variant/width/prefix-suffix state contracts and segmented slot markers.\"",
        "<Playground title=\"DateField + Prefix/Suffix\" code=code>",
        "<Playground title=\"Secondary + Full Width + Invalid\" code=states_code>",
        "<DateInputGroup",
        "variant=DateInputGroupVariant::Secondary",
        "full_width=true",
        "invalid=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups docs page should include `{needle}` for date_input_group primary coverage.",
        );
    }
}

#[test]
fn date_input_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "let (invoice_date, set_invoice_date) = signal(Some(\"2026-03-14\".to_string()));",
        "id_base=\"docs-date-input-group-invoice\".to_string()",
        "aria_label=\"Invoice date controls\".to_string()",
        "segmented=true",
        "prefix=move || view! { <span>\"📅\"</span> }",
        "suffix=move || view! { <span>\"UTC+0\"</span> }",
        "\"invoice date: \"",
        "let (ship_window, set_ship_window) = signal(Some(\"18:30\".to_string()));",
        "id_base=\"docs-date-input-group-time\".to_string()",
        "variant=DateInputGroupVariant::Secondary",
        "full_width=true",
        "invalid=true",
        "aria_label=\"Ship window controls\".to_string()",
        "class_name=\"docs-date-input-group-custom\".to_string()",
        "prefix=move || view! { <span>\"🕒\"</span> }",
        "suffix=move || view! { <span>\"5m\"</span> }",
        "minute_step=5",
        "\"ship window: \"",
    ] {
        assert!(
            source.contains(needle),
            "date_input_group docs playgrounds should contain `{needle}`.",
        );
    }
}
