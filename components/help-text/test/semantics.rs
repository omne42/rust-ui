use std::fs;
use std::path::Path;

fn load_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn help_text_ui_components_layering_contract_is_wired_to_four_layers() {
    let logic_source = load_component_source("logic.rs");
    let view_source = load_component_source("view.rs");
    let motion_source = load_component_source("motion.rs");
    let styles_source = load_component_source("styles.rs");

    assert!(
        logic_source.contains("ui_state_primitives::help_text"),
        "help-text logic should consume state primitives instead of implementing state machines locally."
    );
    assert!(
        logic_source.contains("resolve_locale_attrs")
            && logic_source.contains("resolve_error_live_region_attrs")
            && logic_source.contains("resolve_agent_contract_attrs")
            && logic_source.contains("resolve_render_model")
            && logic_source.contains("HelpTextLogicInput")
            && logic_source.contains("HelpTextMessageKind")
            && logic_source.contains("HelpTextDataState")
            && logic_source.contains("HELP_TEXT_AGENT_SCHEMA"),
        "help-text logic should expose locale/live-region/render-model helpers and typed discrete state enums."
    );
    assert!(
        view_source.contains("logic::resolve_locale_attrs(lang, dir)")
            && view_source.contains("role=error_live_region.role")
            && view_source.contains("aria-live=error_live_region.aria_live")
            && view_source.contains("aria-label=aria_label")
            && view_source.contains(
                "aria-disabled=move || state.get_value().is_disabled.then_some(\"true\")"
            )
            && view_source
                .contains("aria-invalid=move || state.get_value().is_invalid.then_some(\"true\")")
            && view_source.contains("logic::resolve_render_model(logic::HelpTextLogicInput {")
            && view_source.contains("logic::resolve_agent_contract_attrs(resolved_state)")
            && view_source.contains("data-slot=\"help-text\"")
            && view_source.contains("state.get_value().message_kind.as_attr()")
            && view_source.contains("state.get_value().data_state.as_attr()")
            && view_source.contains("state.get_value().aria_source.as_attr()")
            && view_source.contains("state.get_value().error_source.as_attr()")
            && view_source.contains("state.get_value().class_source.as_attr()")
            && view_source
                .contains("data-ui-schema=move || agent_contract.get_value().data_ui_schema")
            && view_source
                .contains("data-ui-intent=move || agent_contract.get_value().data_ui_intent")
            && view_source
                .contains("data-ui-action=move || agent_contract.get_value().data_ui_action")
            && view_source
                .contains("data-ui-state=move || agent_contract.get_value().data_ui_state")
            && view_source
                .contains("data-ui-source=move || agent_contract.get_value().data_ui_source")
            && view_source.contains(
                "data-ui-output-status=move || agent_contract.get_value().data_ui_output_status"
            )
            && !view_source.contains("inner_html")
            && !view_source.contains("dangerously_set_inner_html")
            && !view_source.contains("logic::resolve_state(HelpTextStateInput {")
            && !view_source.contains("normalize_aria_label(aria_label)")
            && !view_source.contains("normalize_error_message(error_message, is_invalid)")
            && !view_source.contains("has_description = description.is_some()")
            && !view_source.contains("message_kind_attr")
            && !view_source.contains("data_state_attr")
            && !view_source.contains("aria_source_attr")
            && !view_source.contains("error_source_attr")
            && !view_source.contains("class_source_attr")
            && !view_source.contains("unwrap_or_default"),
        "help-text view should mount headless-derived semantics instead of hardcoding role/live settings."
    );
    assert!(
        motion_source.contains("default_text_field_motion_tokens")
            && motion_source.contains("ui_motion::web::animate"),
        "help-text motion should map component semantics to ui-motion contracts and theme tokens."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "help-text styles should stay token-first and consume shared ui-theme variables."
    );
}

#[test]
fn help_text_public_api_stays_platform_agnostic() {
    let mod_source = load_component_source("mod.rs");
    let view_source = load_component_source("view.rs");

    for needle in [
        "pub use view::HelpText;",
        "pub use motion::HelpTextMotion;",
        "pub use logic::A11yDirection;",
    ] {
        assert!(
            mod_source.contains(needle),
            "help-text public API should export `{needle}` from stable component entrypoints."
        );
    }

    assert!(
        !mod_source.contains("web_sys") && !view_source.contains("web_sys"),
        "help-text public surface should not expose DOM/web-sys detail types."
    );
}

#[test]
fn help_text_public_boolean_props_follow_is_prefix_contract() {
    let view_source = load_component_source("view.rs");

    for needle in [
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_error_icon_visible: bool,",
    ] {
        assert!(
            view_source.contains(needle),
            "help-text public boolean props should use is_* naming; missing `{needle}`."
        );
    }

    for legacy in [
        "#[prop(optional)] invalid: bool,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] show_error_icon: bool,",
    ] {
        assert!(
            !view_source.contains(legacy),
            "legacy boolean prop alias `{legacy}` should be removed to avoid naming drift."
        );
    }
}

#[test]
fn help_text_is_stateless_and_does_not_expose_fake_controlled_triplets() {
    let view_source = load_component_source("view.rs");

    for required in [
        "#[prop(optional)] tone: HelpTextTone,",
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_error_icon_visible: bool,",
    ] {
        assert!(
            view_source.contains(required),
            "help-text should keep `{required}` as direct input axes."
        );
    }

    for forbidden in [
        "default_tone",
        "default_is_invalid",
        "default_is_disabled",
        "default_is_error_icon_visible",
        "on_tone_change",
        "on_invalid_change",
        "on_is_invalid_change",
        "on_disabled_change",
        "on_is_disabled_change",
        "on_error_icon_visible_change",
        "on_is_error_icon_visible_change",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "help-text should stay stateless and must not expose fake controlled API `{forbidden}`."
        );
    }
}
