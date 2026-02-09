use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn native_select_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/native_select/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::NativeSelect;"),
        "native_select module should export `NativeSelect`."
    );
    assert!(
        module_source.contains("pub use logic::{DEFAULT_ARIA_LABEL, NativeSelectSize};"),
        "native_select module should export `NativeSelectSize` and `DEFAULT_ARIA_LABEL`."
    );
    assert!(
        crate_source.contains(
            "pub use native_select::{NativeSelect, NativeSelectOption, NativeSelectSize};"
        ),
        "crate root should re-export NativeSelect contracts."
    );
}

#[test]
fn native_select_uses_logic_state_model() {
    let view_source = load_source("src/native_select/view.rs");
    let logic_source = load_source("src/native_select/logic.rs");

    for needle in [
        "pub enum NativeSelectSize",
        "pub fn normalize_options(",
        "pub fn resolve_options(",
        "pub fn sanitize_selected_index(",
        "pub fn find_index_by_value(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "NativeSelect logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let selected_state = overlay_open::use_controllable_state(",
        "let selected_index = selected_state.value;",
        "let request_selected_index_change = selected_state.request_change;",
        "let state = Signal::derive(move ||",
        "logic::resolve_state(",
        "logic::compose_class_name(class_name.get_value(), &state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "NativeSelect view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn native_select_supports_controlled_uncontrolled_and_placeholder_contracts() {
    let source = load_source("src/native_select/view.rs");

    for needle in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional, into)] placeholder: Option<String>",
        "prop:value=move || selected_value.get()",
        "let on_change = move |ev: ev::Event| {",
        "let next_value = event_target_value(&ev);",
        "logic::find_index_by_value(&next_value, &resolved_options.get_untracked())",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect should include `{needle}` for controlled/uncontrolled and placeholder behavior."
        );
    }
}

#[test]
fn native_select_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/native_select/view.rs");

    for needle in [
        "data-slot=\"native-select\"",
        "data-state=move || state.get().data_state_attr",
        "data-size=move || state.get().size_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-options=move || state.get().has_options.then_some(\"true\")",
        "data-option-count=move || state.get().option_count.to_string()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-has-placeholder=move || state.get().has_placeholder.then_some(\"true\")",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-has-enabled-options=move || state.get().has_enabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-slot=\"native-select-control\"",
        "data-slot=\"native-select-indicator\"",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect should set `{needle}` for Spectrum-compatible selectors and state inspection."
        );
    }
}

#[test]
fn native_select_styles_include_size_invalid_disabled_and_empty_markers() {
    let source = load_source("src/native_select/styles.rs");

    for needle in [
        ".ui-native-select {",
        ".ui-native-select__control {",
        ".ui-native-select__indicator {",
        ".ui-native-select--size-sm .ui-native-select__control",
        ".ui-native-select--size-md .ui-native-select__control",
        ".ui-native-select--size-lg .ui-native-select__control",
        ".ui-native-select--invalid .ui-native-select__control",
        ".ui-native-select--disabled .ui-native-select__control",
        ".ui-native-select--empty .ui-native-select__control",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect styles should include `{needle}` for stable visual state contracts."
        );
    }
}
