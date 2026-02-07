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
        "pub struct ShareButtonState",
        "pub struct ResolvedShareItems",
        "pub fn normalize_optional_text(",
        "pub fn resolve_items(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ShareButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let label = logic::normalize_optional_text(label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let resolved_items = logic::resolve_items(&items);",
        "let state = logic::resolve_state(",
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
        "data-state=if state.is_empty { \"empty\" } else { \"ready\" }",
        "data-count=state.item_count.to_string()",
        "data-icon=state.icon_placement_attr",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-default-items=state.uses_default_items.then_some(\"true\")",
        "data-custom-label=state.has_custom_label.then_some(\"true\")",
        "data-has-handler=state.has_custom_press_handler.then_some(\"true\")",
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
