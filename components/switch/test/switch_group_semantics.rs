use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    if let Some(suffix) = rel_path.strip_prefix("src/switch/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        return workspace_dir.join("components/switch/src").join(suffix);
    }

    manifest_dir.join(rel_path)
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn switch_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/switch/group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SwitchGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn switch_group_uses_logic_state_model() {
    let logic_source = load_source("src/switch/group/logic.rs");
    let view_source = load_source("src/switch/group/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch_group.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for needle in [
        "pub use ui_state_primitives::switch_group::",
        "pub fn resolve_ids(",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_description(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
        "pub fn resolve_state(input: SwitchGroupStateInput) -> SwitchGroupState",
        "pub fn compose_describedby(state: SwitchGroupState, ids: &SwitchGroupIds) -> Option<String>",
        "ui_state_primitives::switch_group::resolve_state(input)",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SwitchGroup logic should include `{needle}` while delegating state derivation."
        );
    }

    for needle in [
        "pub enum SwitchGroupOrientation",
        "pub enum SwitchGroupTone",
        "pub struct SwitchGroupStateInput",
        "pub struct SwitchGroupState",
        "pub fn resolve_state(input: SwitchGroupStateInput) -> SwitchGroupState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "SwitchGroup primitive should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod switch_group;"),
        "ui-state-primitives should export `pub mod switch_group;`."
    );

    for needle in [
        "logic::resolve_ids(id_base)",
        "logic::normalize_label(label)",
        "logic::normalize_description(description)",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_error_message(error_message, is_invalid)",
        "logic::resolve_state(SwitchGroupStateInput {",
        "logic::compose_describedby(state.get(), &group_ids)",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "SwitchGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "let mut ids_out = Vec::new();",
        "ids_out.push(group_ids.description_id.clone())",
        "ids_out.push(group_ids.error_id.clone())",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SwitchGroup view should not rebuild describedby state aggregation in view layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn switch_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/switch/group/view.rs");

    for attr in [
        "data-slot=\"switch-group\"",
        "data-slot=\"switch-group-label\"",
        "data-slot=\"switch-group-group\"",
        "data-slot=\"switch-group-description\"",
        "data-slot=\"switch-group-error\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-has-label=move || state.get().has_label.then_some(\"true\")",
        "data-has-description=move || state.get().has_description.then_some(\"true\")",
        "data-has-error=move || state.get().has_error_message.then_some(\"true\")",
        "data-shows-error=move || state.get().shows_error.then_some(\"true\")",
        "data-has-messages=move || state.get().has_messages.then_some(\"true\")",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "SwitchGroup should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn switch_group_styles_include_state_markers() {
    let source = load_source("src/switch/group/styles.rs");

    for selector in [
        ".ui-switch-group--orientation-vertical",
        ".ui-switch-group[data-orientation=\"horizontal\"]",
        ".ui-switch-group--tone-default",
        ".ui-switch-group[data-tone=\"muted\"]",
        ".ui-switch-group--required .ui-switch-group__label::after",
        ".ui-switch-group[data-required=\"true\"] .ui-switch-group__label::after",
        ".ui-switch-group--invalid .ui-switch-group__group",
        ".ui-switch-group[data-invalid=\"true\"] .ui-switch-group__group",
        ".ui-switch-group--disabled",
        ".ui-switch-group[data-disabled=\"true\"]",
        ".ui-switch-group--custom-class",
        ".ui-switch-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "SwitchGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn switch_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn switch_group() -> AnyView",
        "title=\"SwitchGroup\"",
        "slug=\"switch-group\"",
        "description=\"baseline-style switch grouping primitive with centralized orientation/tone/validation/message-state contracts and stable data markers.\"",
        "<Playground title=\"Required + Description\" code_signal=base_code>",
        "<Playground title=\"Horizontal + Invalid + Disabled + Custom Class\" code_signal=states_code>",
        "<SwitchGroup",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups switch_group docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn switch_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "title=\"Required + Description\"",
        "is_required=true",
        "aria_label=\"Notification switches\".to_string()",
        "title=\"Horizontal + Invalid + Disabled + Custom Class\"",
        "orientation=SwitchGroupOrientation::Horizontal",
        "tone=SwitchGroupTone::Muted",
        "is_invalid=true",
        "is_disabled=true",
        "error_message=\"At least one critical channel must stay enabled.\".to_string()",
        "class_name=\"docs-switch-group-custom\".to_string()",
        "<Switch checked=critical_alerts set_checked=set_critical_alerts is_disabled=true>",
        "<Switch checked=maintenance_mode set_checked=set_maintenance_mode is_disabled=true>",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups switch_group docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}

#[test]
fn switch_group_public_boolean_props_use_is_prefix_and_drop_legacy_aliases() {
    let source = load_source("src/switch/group/view.rs");

    for needle in [
        "#[prop(optional)] is_required: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_invalid: bool,",
    ] {
        assert!(
            source.contains(needle),
            "SwitchGroup public bool prop should use `is_*` naming `{needle}`.",
        );
    }

    for legacy in [
        "#[prop(optional)] required: bool,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] invalid: bool,",
    ] {
        assert!(
            !source.contains(legacy),
            "SwitchGroup should remove legacy bool prop alias `{legacy}`.",
        );
    }
}

#[test]
fn switch_group_discrete_axes_are_typed_enums_not_stringly_props() {
    let view_source = load_source("src/switch/group/view.rs");
    let logic_source = load_source("src/switch/group/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch_group.rs");

    for needle in [
        "#[prop(optional)] orientation: SwitchGroupOrientation,",
        "#[prop(optional)] tone: SwitchGroupTone,",
        "pub use ui_state_primitives::switch_group::{",
        "pub enum SwitchGroupOrientation",
        "pub enum SwitchGroupTone",
        "pub struct SwitchGroupStateInput",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "SwitchGroup discrete axes should stay enum-typed; missing `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] orientation: Option<String>",
        "#[prop(optional, into)] tone: Option<String>",
        "orientation: Option<bool>",
        "tone: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SwitchGroup should not model discrete axes with string/bool unions; found `{forbidden}`.",
        );
    }
}
