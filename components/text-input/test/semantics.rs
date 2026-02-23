use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above CARGO_MANIFEST_DIR"))
        .to_path_buf()
}

fn load_text_input_file(rel: &str) -> String {
    let path = workspace_root().join("components/text-input").join(rel);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read `{}`: {err}", path.display()))
}

#[test]
fn text_input_domain_module_is_feature_gated_assembly_only() {
    let source = load_text_input_file("src/mod.rs");

    for needle in [
        "#[cfg(feature = \"component-date_field\")]",
        "#[cfg(feature = \"component-input\")]",
        "#[cfg(feature = \"component-text_field\")]",
        "#[cfg(feature = \"component-textarea\")]",
        "#[cfg(feature = \"component-time_field\")]",
    ] {
        assert!(
            source.contains(needle),
            "text-input domain module should keep feature-gated assembly export `{needle}`."
        );
    }

    for forbidden in ["web_sys", "leptos::web_sys", "wasm_bindgen"] {
        assert!(
            !source.contains(forbidden),
            "text-input domain public module should not expose platform detail token `{forbidden}`."
        );
    }
}

#[test]
fn text_input_public_submodules_do_not_expose_platform_details() {
    let modules = [
        "date_field",
        "date_picker",
        "date_range_picker",
        "input",
        "input_otp",
        "number",
        "number_field",
        "search_field",
        "text",
        "text_area",
        "text_field",
        "textarea",
        "time_field",
    ];

    for module in modules {
        let path = format!("src/{module}/mod.rs");
        let source = load_text_input_file(&path);

        for forbidden in [
            "pub mod view;",
            "pub mod logic;",
            "web_sys",
            "leptos::web_sys",
        ] {
            assert!(
                !source.contains(forbidden),
                "{path} should keep `{forbidden}` internal."
            );
        }
    }
}

#[test]
fn text_input_has_canonical_semantics_entry_file_and_legacy_bridge_example() {
    let canonical = workspace_root().join("components/text-input/test/semantics.rs");
    assert!(
        canonical.exists(),
        "text-input should provide a canonical semantics.rs entry in test/."
    );

    let bridge = load_text_input_file("test/date_field_semantics.rs");
    assert!(
        bridge.contains("#[path = \"date_field/semantics.rs\"]"),
        "legacy semantics entry should be bridged to the new test/<component>/semantics.rs layout."
    );
}

#[test]
fn text_input_api_naming_contract_is_prefixed_with_compatible_alias_bridge() {
    let input_source = load_text_input_file("src/input/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "#[prop(optional)] set_value: Option<WriteSignal<String>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_read_only: Option<bool>",
        "#[prop(optional, into)] is_required: Option<Signal<bool>>",
        "#[prop(optional, into)] is_invalid: Option<Signal<bool>>",
        "#[prop(optional)] is_label_hidden: Option<bool>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
    ] {
        assert!(
            input_source.contains(needle),
            "input view should expose canonical prefixed API contract `{needle}`."
        );
    }

    let input_group_source = load_text_input_file("src/input/group/view.rs");
    for needle in [
        "#[prop(optional)] is_attached: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "let is_attached = is_attached.unwrap_or(attached);",
        "let is_disabled = is_disabled.unwrap_or(disabled);",
        "let is_invalid = is_invalid.unwrap_or(invalid);",
    ] {
        assert!(
            input_group_source.contains(needle),
            "input group should expose canonical prefixed API contract `{needle}`."
        );
    }

    let otp_source = load_text_input_file("src/input_otp/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "#[prop(optional)] set_value: Option<WriteSignal<String>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, into)] is_required: Option<Signal<bool>>",
        "#[prop(optional, into)] is_invalid: Option<Signal<bool>>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let on_value_change = on_value_change",
    ] {
        assert!(
            otp_source.contains(needle),
            "input otp should expose canonical prefixed API contract `{needle}`."
        );
    }

    let number_field_source = load_text_input_file("src/number_field/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<i64>>",
        "#[prop(optional)] default_value: Option<i64>",
        "#[prop(optional)] on_value_change: Option<Callback<i64>>",
        "#[prop(optional)] set_value: Option<WriteSignal<i64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, into)] is_required: Option<Signal<bool>>",
        "#[prop(optional, into)] is_invalid: Option<Signal<bool>>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let on_value_change = on_value_change",
    ] {
        assert!(
            number_field_source.contains(needle),
            "number field should expose canonical prefixed API contract `{needle}`."
        );
    }

    let date_picker_source = load_text_input_file("src/date_picker/view.rs");
    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_show_outside_days: Option<bool>",
        "let is_show_outside_days = is_show_outside_days.unwrap_or(show_outside_days);",
    ] {
        assert!(
            date_picker_source.contains(needle),
            "date picker should expose canonical prefixed API contract `{needle}`."
        );
    }

    let date_range_picker_source = load_text_input_file("src/date_range_picker/view.rs");
    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] default_start_day: Option<u8>",
        "#[prop(optional)] on_start_day_change: Option<Callback<Option<u8>>>",
        "#[prop(optional)] default_end_day: Option<u8>",
        "#[prop(optional)] on_end_day_change: Option<Callback<Option<u8>>>",
        "#[prop(optional)] is_show_outside_days: Option<bool>",
    ] {
        assert!(
            date_range_picker_source.contains(needle),
            "date range picker should expose canonical prefixed API contract `{needle}`."
        );
    }

    let text_source = load_text_input_file("src/text/view.rs");
    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_truncated: Option<bool>",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
    ] {
        assert!(
            text_source.contains(needle),
            "text component should expose canonical prefixed API contract `{needle}`."
        );
    }
}

#[test]
fn text_input_controlled_uncontrolled_triplets_are_complete_for_value_axes() {
    let input_source = load_text_input_file("src/input/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
    ] {
        assert!(
            input_source.contains(needle),
            "Input value axis should provide controlled/uncontrolled triplet `{needle}`."
        );
    }

    let input_otp_source = load_text_input_file("src/input_otp/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
    ] {
        assert!(
            input_otp_source.contains(needle),
            "InputOtp value axis should provide controlled/uncontrolled triplet `{needle}`."
        );
    }

    let number_field_source = load_text_input_file("src/number_field/view.rs");
    for needle in [
        "#[prop(optional, into)] value: Option<Signal<i64>>",
        "#[prop(optional)] default_value: Option<i64>",
        "#[prop(optional)] on_value_change: Option<Callback<i64>>",
        "let controlled_default_value = logic::normalize_default_value(default_value);",
        "use_controllable_state(value, Some(controlled_default_value), on_value_change)",
    ] {
        assert!(
            number_field_source.contains(needle),
            "NumberField value axis should provide controlled/uncontrolled triplet `{needle}`."
        );
    }

    let date_picker_source = load_text_input_file("src/date_picker/view.rs");
    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "selected_day: Option<Signal<Option<u8>>>",
        "default_selected_day: Option<u8>",
        "on_selected_day_change: Option<Callback<Option<u8>>>",
    ] {
        assert!(
            date_picker_source.contains(needle),
            "DatePicker should keep complete controlled/uncontrolled triplets for each state axis `{needle}`."
        );
    }
}

#[test]
fn text_input_default_value_normalization_is_owned_by_logic_layer() {
    for view_path in [
        "src/input/view.rs",
        "src/input_otp/view.rs",
        "src/number_field/view.rs",
    ] {
        let view_source = load_text_input_file(view_path);
        assert!(
            !view_source.contains("default_value.unwrap_or_default()"),
            "{view_path} should not normalize default_value directly in view.rs."
        );
    }

    for logic_path in [
        "src/input/logic.rs",
        "src/input_otp/logic.rs",
        "src/number_field/logic.rs",
    ] {
        let logic_source = load_text_input_file(logic_path);
        assert!(
            logic_source.contains("pub fn normalize_default_value"),
            "{logic_path} should own default_value normalization in logic.rs."
        );
    }
}

#[test]
fn text_input_state_normalization_is_centralized_in_logic_layer() {
    let input_view = load_text_input_file("src/input/view.rs");
    assert!(
        input_view
            .contains("logic::normalize_accessibility_state(logic::AccessibilityStateInput {"),
        "Input should centralize accessibility state normalization in logic.rs."
    );
    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_read_only.unwrap_or(read_only)",
        "is_required.unwrap_or(required)",
        "is_invalid.unwrap_or(invalid)",
        "is_label_hidden.unwrap_or(label_hidden)",
    ] {
        assert!(
            !input_view.contains(forbidden),
            "Input view.rs should not merge accessibility state directly via `{forbidden}`."
        );
    }

    let input_otp_view = load_text_input_file("src/input_otp/view.rs");
    assert!(
        input_otp_view
            .contains("logic::normalize_accessibility_state(logic::AccessibilityStateInput {"),
        "InputOtp should centralize accessibility state normalization in logic.rs."
    );
    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_required.unwrap_or(required)",
        "is_invalid.unwrap_or(invalid)",
    ] {
        assert!(
            !input_otp_view.contains(forbidden),
            "InputOtp view.rs should not merge accessibility state directly via `{forbidden}`."
        );
    }

    let number_field_view = load_text_input_file("src/number_field/view.rs");
    assert!(
        number_field_view
            .contains("logic::normalize_accessibility_state(logic::AccessibilityStateInput {"),
        "NumberField should centralize accessibility state normalization in logic.rs."
    );
    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_required.unwrap_or(required)",
        "is_invalid.unwrap_or(invalid)",
    ] {
        assert!(
            !number_field_view.contains(forbidden),
            "NumberField view.rs should not merge accessibility state directly via `{forbidden}`."
        );
    }

    let text_view = load_text_input_file("src/text/view.rs");
    assert!(
        text_view.contains("logic::normalize_accessibility_state(logic::AccessibilityStateInput {"),
        "Text should centralize accessibility state normalization in logic.rs."
    );
    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_truncated.unwrap_or(truncate)",
    ] {
        assert!(
            !text_view.contains(forbidden),
            "Text view.rs should not merge accessibility state directly via `{forbidden}`."
        );
    }

    for logic_path in [
        "src/input/logic.rs",
        "src/input_otp/logic.rs",
        "src/number_field/logic.rs",
        "src/text/logic.rs",
    ] {
        let logic_source = load_text_input_file(logic_path);
        assert!(
            logic_source.contains("pub struct AccessibilityStateInput"),
            "{logic_path} should define typed normalization input boundary."
        );
        assert!(
            logic_source.contains("pub fn normalize_accessibility_state"),
            "{logic_path} should centralize accessibility state normalization."
        );
    }
}

#[test]
fn text_input_discrete_type_axes_are_enum_constrained() {
    let input_logic = load_text_input_file("src/input/logic.rs");
    for needle in [
        "pub enum InputType",
        "pub struct InputTypeState",
        "pub fn normalize_input_type(input_type: Option<&'static str>) -> InputTypeState",
        "InputType::Custom",
    ] {
        assert!(
            input_logic.contains(needle),
            "Input logic should type-constrain discrete type axis via `{needle}`."
        );
    }

    let input_view = load_text_input_file("src/input/view.rs");
    for needle in [
        "let input_type_state = logic::normalize_input_type(input_type);",
        "type=input_type.as_html_attr()",
        "data-type-source=input_type_state.type_source_attr",
    ] {
        assert!(
            input_view.contains(needle),
            "Input view should consume typed type-axis contract via `{needle}`."
        );
    }
    assert!(
        !input_view.contains("type=input_type\n"),
        "Input view should not mount raw string `input_type` directly without enum mapping."
    );

    let text_field_logic = load_text_input_file("src/text_field/logic.rs");
    for needle in [
        "pub enum TextFieldInputType",
        "pub struct InputTypeState",
        "pub fn normalize_input_type(input_type: Option<&'static str>) -> InputTypeState",
        "TextFieldInputType::Custom",
    ] {
        assert!(
            text_field_logic.contains(needle),
            "TextField logic should type-constrain discrete type axis via `{needle}`."
        );
    }

    let text_field_view = load_text_input_file("src/text_field/view.rs");
    assert!(
        text_field_view.contains("type=input_type.as_html_attr()"),
        "TextField view should mount input type through typed enum mapping."
    );
}
