use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/picker/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Picker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn picker_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/picker/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Picker;"),
        "picker module should export `Picker`."
    );
    assert!(
        crate_source.contains("pub use picker::Picker;"),
        "crate root should re-export `Picker`."
    );
}

#[test]
fn picker_logic_exposes_state_helpers() {
    let source = load_source("src/picker/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: PickerStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: PickerState)",
    ] {
        assert!(
            source.contains(needle),
            "Picker logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn picker_view_uses_logic_state_contracts() {
    let source = load_source("src/picker/view.rs");

    for needle in [
        "pub fn Picker(",
        "logic::normalize_optional_text(placeholder)",
        "logic::resolve_state(PickerStateInput {",
        "logic::compose_class_name(class_name_for_wrapper.clone(), state.get())",
        "<Select",
        "placement: PopoverPlacement",
        "motion: SelectMotion",
        "data-slot=\"picker\"",
        "data-state=move || state.get().state_attr",
        "data-selection=move || state.get().selection_attr",
        "data-disabled-options=move || state.get().disabled_options_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-initial-open=move || state.get().initial_open_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-handler-source=move || state.get().handler_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-placement-source=move || state.get().placement_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Picker view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn picker_styles_include_state_and_source_markers() {
    let source = load_source("src/picker/styles.rs");

    for selector in [
        ".ui-picker {",
        ".ui-picker[data-state=\"disabled\"]",
        ".ui-picker[data-state=\"empty\"]",
        ".ui-picker[data-selection=\"selected\"]",
        ".ui-picker[data-disabled-options=\"present\"]",
        ".ui-picker[data-open-mode=\"controlled\"]",
        ".ui-picker[data-open-mode=\"uncontrolled\"]",
        ".ui-picker[data-initial-open=\"open\"]",
        ".ui-picker[data-placeholder-source=\"custom\"]",
        ".ui-picker[data-handler-source=\"custom\"]",
        ".ui-picker[data-class-source=\"custom\"]",
        ".ui-picker[data-placement-source=\"custom\"]",
        ".ui-picker[data-motion-source=\"custom\"]",
        ".ui-picker--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Picker styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn picker_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::picker::styles::CSS);"),
        "ui-components css aggregator should include picker styles."
    );
}

#[test]
fn picker_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra_picker.rs");

    for needle in [
        "pub(super) fn picker() -> AnyView",
        "title=\"Picker\"",
        "slug=\"picker\"",
        "State + Source Markers",
        "data-motion-source",
        "<Picker",
    ] {
        assert!(
            source.contains(needle),
            "collections_extra_picker docs page should contain `{needle}`."
        );
    }
}
