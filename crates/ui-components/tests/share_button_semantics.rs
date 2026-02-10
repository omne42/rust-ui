use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn share_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_share/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ShareButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn share_button_uses_logic_state_model() {
    let view_source = load_source("src/button_share/view.rs");
    let logic_source = load_source("src/button_share/logic.rs");

    for needle in [
        "pub struct ShareButtonStateInput",
        "pub struct ShareButtonState",
        "pub struct ResolvedShareItems",
        "pub fn normalize_optional_text(",
        "pub fn resolve_items(",
        "pub fn resolve_state(input: ShareButtonStateInput)",
        "pub fn compose_class_name(",
        "pub items_source_attr: &'static str",
        "pub handler_source_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ShareButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let label = logic::normalize_optional_text(label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let provided_item_count = items.len();",
        "let resolved_items = logic::resolve_items(&items);",
        "let state = logic::resolve_state(ShareButtonStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn share_button_uses_flip_button_and_button_group_composition() {
    let source = load_source("src/button_share/view.rs");

    for needle in [
        "<FlipButton",
        "from=from",
        "motion=motion.flip",
        "front=move ||",
        "back=move ||",
        "<ButtonGroup attached=true>",
        "data-slot=\"share-button-front\"",
        "data-slot=\"share-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should compose flip/share surfaces via `{needle}`."
        );
    }
}

#[test]
fn share_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_share/view.rs");

    for needle in [
        "data-slot=\"share-button\"",
        "data-state=state.state_attr",
        "data-provided-count=state.provided_item_count.to_string()",
        "data-count=state.resolved_item_count.to_string()",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-items-source=state.items_source_attr",
        "data-icon=state.icon_placement_attr",
        "data-label-source=state.label_source_attr",
        "data-handler-source=state.handler_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-custom-motion=(motion != ShareButtonMotion::default()).then_some(\"true\")",
        "data-slot=\"share-button-platform\"",
        "data-platform=platform_attr",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should expose `{needle}` for Spectrum-style state inspection."
        );
    }
}

#[test]
fn share_button_maps_icon_button_size_and_platform_icons() {
    let source = load_source("src/button_share/view.rs");

    for needle in [
        "let icon_button_size = logic::resolve_icon_button_size(size);",
        "size=icon_button_size",
        "SharePlatform::Github",
        "SharePlatform::X",
        "SharePlatform::Facebook",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should include `{needle}` for icon-button behavior and platform coverage."
        );
    }
}

#[test]
fn share_button_preserves_optional_press_handler_without_markup_branching() {
    let source = load_source("src/button_share/view.rs");

    for needle in [
        "let on_icon_press = StoredValue::new(on_icon_press);",
        "if let Some(cb) = on_icon_press {",
        "cb.run(platform);",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should wire optional callbacks via `{needle}`."
        );
    }

    assert!(
        !source.contains("match on_icon_press"),
        "ShareButton should avoid duplicating markup based on handler presence."
    );
}

#[test]
fn share_button_styles_include_state_marker_contracts() {
    let source = load_source("src/button_share/styles.rs");

    for selector in [
        ".ui-share-button--state-ready",
        ".ui-share-button[data-state=\"ready\"]",
        ".ui-share-button--icon-prefix",
        ".ui-share-button[data-icon=\"none\"] [data-slot=\"share-button-trigger-icon\"]",
        ".ui-share-button__platform[data-platform=\"github\"] .ui-button",
        ".ui-share-button--custom-class",
        ".ui-share-button[data-custom-class=\"true\"]",
        ".ui-share-button[data-motion-source=\"custom\"]",
        ".ui-share-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ShareButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn share_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button_share/motion.rs");

    for needle in [
        "pub struct ShareButtonMotion",
        "fn default_motion_matches_flip_button_defaults()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}
