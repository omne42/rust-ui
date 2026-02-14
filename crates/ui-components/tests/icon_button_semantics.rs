use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn icon_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "IconButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icon_button_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/icon_button/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::IconButton;"),
        "icon_button module should export `IconButton`."
    );
    assert!(
        crate_source.contains("pub use icon_button::IconButton;"),
        "crate root should re-export `IconButton`."
    );
}

#[test]
fn icon_button_logic_exposes_state_helpers() {
    let source = load_source("src/icon_button/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(aria_label: String, default: &str)",
        "pub fn resolve_state(input: IconButtonStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconButtonState)",
        "DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "IconButton logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn icon_button_view_uses_logic_state_contracts() {
    let source = load_source("src/icon_button/view.rs");

    for needle in [
        "pub fn IconButton(",
        "logic::normalize_optional_text(class_name)",
        "i18n::use_ui_i18n()",
        "i18n.strings::<CommonStrings>()",
        "logic::normalize_aria_label(aria_label, common.icon_button_aria_label.as_ref())",
        "logic::resolve_state(IconButtonStateInput {",
        "logic::compose_class_name(class_name, state)",
        "<Button",
        "on_press: Option<OnPress>",
        "data-slot=\"icon-button\"",
        "data-state=state.state_attr",
        "data-size-mode=state.size_mode_attr",
        "data-handler-source=state.handler_source_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-fallback-label=state.has_fallback_aria_label.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "IconButton view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn icon_button_styles_include_state_and_source_markers() {
    let source = load_source("src/icon_button/styles.rs");

    for selector in [
        ".ui-icon-button {",
        ".ui-icon-button[data-state=\"disabled\"]",
        ".ui-icon-button[data-size-mode=\"icon\"]",
        ".ui-icon-button[data-size-mode=\"custom\"]",
        ".ui-icon-button[data-handler-source=\"custom\"]",
        ".ui-icon-button[data-label-source=\"custom\"]",
        ".ui-icon-button[data-class-source=\"custom\"]",
        ".ui-icon-button[data-motion-source=\"custom\"]",
        ".ui-icon-button[data-custom-class=\"true\"]",
        ".ui-icon-button--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "IconButton styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn icon_button_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::icon_button::styles::CSS);"),
        "ui-components css aggregator should include icon_button styles."
    );
}

#[test]
fn icon_button_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "pub(super) fn icon_button() -> AnyView",
        "title=\"IconButton\"",
        "slug=\"icon-button\"",
        "State + Source Markers",
        "data-motion-source",
        "<IconButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra_icon_button docs page should contain `{needle}`."
        );
    }
}

#[test]
fn icon_button_docs_page_locks_custom_motion_marker_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }",
        "data-motion-source",
        "class_name=\"docs-icon-button-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icon button docs page should include `{needle}` for motion/source marker regression stability."
        );
    }
}

#[test]
fn icon_button_docs_actions_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn icon_button() -> AnyView",
        "title=\"IconButton\"",
        "slug=\"icon-button\"",
        "description=\"A Button wrapper that enforces accessible labeling and icon sizing while preserving motion/press semantics.\"",
        "<Playground",
        "title=\"on_press + variants\"",
        "code_signal=code",
        "<Playground title=\"Size + disabled matrix\" code_signal=states_code>",
        "<IconButton",
        "aria_label=\"Close dialog\".to_string()",
        "variant=ButtonVariant::Ghost",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for icon-button coverage.",
        );
    }
}

#[test]
fn icon_button_docs_actions_page_locks_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "\"close/search presses: \"",
        "{move || format!(\"{}/{}\", close_count.get(), search_count.get())}",
        "id_base=\"docs-icon-button-variant\".to_string()",
        "id_base=\"docs-icon-button-size\".to_string()",
        "aria_label=\"IconButton variant\".to_string()",
        "aria_label=\"IconButton size\".to_string()",
        "<Switch checked=search_disabled set_checked=set_search_disabled>",
        "\"Disable search button\"",
        "variant=variant",
        "size=size",
        "disabled=search_disabled",
        "aria_label=\"Search xs\".to_string() size=ButtonSize::IconXs",
        "aria_label=\"Search s\".to_string() size=ButtonSize::IconS",
        "aria_label=\"Search m\".to_string() size=ButtonSize::IconM",
        "aria_label=\"Search l\".to_string() size=ButtonSize::IconL",
        "aria_label=\"Search xl\".to_string() size=ButtonSize::IconXl",
        "aria_label=\"Close disabled\".to_string()",
        "aria_label=\"Search disabled\".to_string()",
        "variant=ButtonVariant::Secondary",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "icon-button docs state matrix should contain `{needle}`.",
        );
    }
}

#[test]
fn icon_button_docs_page_covers_primary_playgrounds() {
    let actions_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "pub(super) fn icon_button() -> AnyView",
        "title=\"IconButton\"",
        "slug=\"icon-button\"",
        "description=\"A Button wrapper that enforces accessible labeling and icon sizing while preserving motion/press semantics.\"",
        "code_signal=code",
        "<Playground title=\"Size + disabled matrix\" code_signal=states_code>",
        "<IconButton",
    ] {
        assert!(
            actions_source.contains(needle),
            "actions icon_button docs should include `{needle}` for primary playground coverage.",
        );
    }

    for needle in [
        "title=\"IconButton\"",
        "slug=\"icon-button\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            extra_source.contains(needle),
            "actions_extra_icon_button docs should include `{needle}` for marker playground coverage.",
        );
    }
}

#[test]
fn icon_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let actions_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra_icon_button.rs");

    for needle in [
        "title=\"on_press + variants\"",
        "aria_label=\"Close dialog\".to_string()",
        "variant=ButtonVariant::Ghost",
        "aria_label=\"Search\".to_string()",
        "variant=variant",
        "size=size",
        "disabled=search_disabled",
        "\"close/search presses: \"",
        "id_base=\"docs-icon-button-variant\".to_string()",
        "id_base=\"docs-icon-button-size\".to_string()",
        "aria_label=\"IconButton variant\".to_string()",
        "aria_label=\"IconButton size\".to_string()",
        "title=\"Size + disabled matrix\"",
        "aria_label=\"Search xs\".to_string() size=ButtonSize::IconXs",
        "aria_label=\"Search s\".to_string() size=ButtonSize::IconS",
        "aria_label=\"Search m\".to_string() size=ButtonSize::IconM",
        "aria_label=\"Search l\".to_string() size=ButtonSize::IconL",
        "aria_label=\"Search xl\".to_string() size=ButtonSize::IconXl",
        "aria_label=\"Close disabled\".to_string()",
        "disabled=true",
    ] {
        assert!(
            actions_source.contains(needle),
            "icon_button actions docs matrix should contain `{needle}`.",
        );
    }

    for needle in [
        "title=\"State + Source Markers\"",
        "aria_label=\"Inspect icon trigger\".to_string()",
        "size=ButtonSize::Lg",
        "motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }",
        "class_name=\"docs-icon-button-state\".to_string()",
        "\"presses: \"",
    ] {
        assert!(
            extra_source.contains(needle),
            "icon_button extra marker docs should contain `{needle}`.",
        );
    }
}
