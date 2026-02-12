use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn combobox_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/combobox/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Combobox internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn combobox_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/combobox/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Combobox;"),
        "combobox module should export `Combobox`.",
    );
    assert!(
        crate_source.contains("pub use combobox::Combobox;"),
        "crate root should re-export `Combobox`.",
    );
}

#[test]
fn combobox_logic_exposes_state_helpers() {
    let source = load_source("src/combobox/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_label(",
        "pub fn resolve_state(input: ComboboxStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: ComboboxState)",
        "DEFAULT_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Combobox logic should include `{needle}` for centralized source/state contracts.",
        );
    }
}

#[test]
fn combobox_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/combobox/view.rs");

    for needle in [
        "logic::resolve_label(label)",
        "logic::resolve_state(ComboboxStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "data-slot=\"combobox\"",
        "data-state=move || state.get().state_attr",
        "data-selection=move || state.get().selection_attr",
        "data-options=move || state.get().options_attr",
        "data-requirement=move || state.get().requirement_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Combobox view should include `{needle}` for stable marker contracts.",
        );
    }
}

#[test]
fn combobox_styles_include_state_and_source_markers() {
    let source = load_source("src/combobox/styles.rs");

    for selector in [
        ".ui-combobox {",
        ".ui-combobox[data-state=\"disabled\"]",
        ".ui-combobox[data-selection=\"out-of-range\"]",
        ".ui-combobox[data-options=\"has-disabled\"]",
        ".ui-combobox[data-requirement=\"required\"]",
        ".ui-combobox[data-label-source=\"custom\"]",
        ".ui-combobox[data-description-source=\"custom\"]",
        ".ui-combobox[data-error-source=\"custom\"]",
        ".ui-combobox[data-placeholder-source=\"custom\"]",
        ".ui-combobox[data-motion-source=\"custom\"]",
        ".ui-combobox[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Combobox styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn combobox_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::combobox::styles::CSS);"),
        "ui-components css aggregator should include combobox styles.",
    );
}

#[test]
fn combobox_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "pub(super) fn combobox() -> AnyView",
        "title=\"Combobox\"",
        "slug=\"combobox\"",
        "State + Source Markers",
        "data-placeholder-source",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_combobox docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn combobox_docs_invalid_disabled_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "title=\"Invalid + Disabled Option\"",
        "id_base=\"docs-combobox-state\".to_string()",
        "label=\"Stateful language\".to_string()",
        "disabled_indices=vec![3]",
        "invalid=Signal::derive(move || invalid.get())",
        "error=\"Language is required\".to_string()",
        "Mark invalid",
    ] {
        assert!(
            source.contains(needle),
            "Combobox docs invalid/disabled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn combobox_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-combobox-markers\".to_string()",
        "required=Signal::derive(|| true)",
        "invalid=Signal::derive(move || marker_invalid.get())",
        "disabled_indices=vec![3]",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Selection is required\".to_string()",
        "placeholder=\"Type to filter\".to_string()",
        "class_name=\"docs-combobox-state\".to_string()",
        "let mut marker_motion = ComboBoxMotion::default();",
        "marker_motion.popover.offset_y_px = 10.0",
        "marker_motion.highlight.spring.stiffness = 260.0",
        "motion=marker_motion",
    ] {
        assert!(
            source.contains(needle),
            "Combobox docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn combobox_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "pub(super) fn combobox() -> AnyView",
        "title=\"Combobox\"",
        "slug=\"combobox\"",
        "description=\"Spectrum-compatible combobox alias for upstream naming parity, preserving ComboBox accessibility, state contracts, and HeroUI-level panel/highlight motion.\"",
        "<Playground title=\"Basic Selection\" code=basic_code>",
        "<Playground title=\"Invalid + Disabled Option\" code=state_code>",
        "title=\"State + Source Markers\"",
        "data-state",
        "data-selection",
        "data-options",
        "data-requirement",
        "data-label-source",
        "data-description-source",
        "data-error-source",
        "data-placeholder-source",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_combobox docs page should include `{needle}` for primary coverage.",
        );
    }
}

#[test]
fn combobox_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_combobox.rs");

    for needle in [
        "id_base=\"docs-combobox-basic\".to_string()",
        "label=\"Language\".to_string()",
        "description=\"Pick one runtime language\".to_string()",
        "\"selected: \"",
        "id_base=\"docs-combobox-state\".to_string()",
        "label=\"Stateful language\".to_string()",
        "disabled_indices=vec![3]",
        "invalid=Signal::derive(move || invalid.get())",
        "error=\"Language is required\".to_string()",
        "\"Clear invalid\"",
        "\"Mark invalid\"",
        "id_base=\"docs-combobox-markers\".to_string()",
        "label=\"Technology stack\".to_string()",
        "required=Signal::derive(|| true)",
        "invalid=Signal::derive(move || marker_invalid.get())",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Selection is required\".to_string()",
        "placeholder=\"Type to filter\".to_string()",
        "class_name=\"docs-combobox-state\".to_string()",
        "motion=marker_motion",
    ] {
        assert!(
            source.contains(needle),
            "combobox docs playgrounds should contain `{needle}`.",
        );
    }
}
