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
fn icon_button_uses_logic_state_model() {
    let view_source = load_source("src/icon_button/view.rs");
    let logic_source = load_source("src/icon_button/logic.rs");

    for needle in [
        "pub struct IconButtonState",
        "pub fn normalize_aria_label(",
        "pub fn normalize_class_name(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub uses_icon_size: bool",
        "pub has_custom_press_handler: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "IconButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let class_name = logic::normalize_class_name(class_name);",
        "let (aria_label, has_explicit_aria_label) = logic::normalize_aria_label(aria_label);",
        "let state = logic::resolve_state(",
        "let class_name = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "IconButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn icon_button_requires_and_normalizes_accessible_name() {
    let view_source = load_source("src/icon_button/view.rs");
    let logic_source = load_source("src/icon_button/logic.rs");

    assert!(
        view_source.contains("aria_label: String"),
        "IconButton should require an `aria_label` input for accessible icon-only buttons."
    );
    assert!(
        view_source.contains("aria_label=aria_label"),
        "IconButton should forward normalized `aria_label` to the underlying Button."
    );
    assert!(
        logic_source.contains("\"Icon button\".to_string()"),
        "IconButton should provide a fallback label when the input label is blank."
    );
}

#[test]
fn icon_button_defaults_to_icon_size() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        source.contains("default = ButtonSize::Icon"),
        "IconButton should default to ButtonSize::Icon to match Spectrum-style icon button sizing."
    );
}

#[test]
fn icon_button_forwards_button_contract() {
    let source = load_source("src/icon_button/view.rs");

    for needle in [
        "use crate::button::{Button, ButtonMotion, ButtonSize, ButtonVariant};",
        "<Button",
        "disabled=disabled",
        "variant=variant",
        "size=size",
        "motion=motion",
        "class_name=class_name",
        "button_type=button_type",
        "aria_label=aria_label",
        "node_ref=node_ref",
        "on_press=on_press",
    ] {
        assert!(
            source.contains(needle),
            "IconButton should forward `{needle}` to Button so interaction and semantics stay centralized."
        );
    }
}

#[test]
fn icon_button_preserves_optional_press_handler_without_markup_branching() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        source.contains("let has_custom_press_handler = on_press.is_some();"),
        "IconButton should track whether a custom handler exists without duplicating markup."
    );
    assert!(
        !source.contains("unwrap_or_else(|| Callback::new"),
        "IconButton should not inject a synthetic no-op press handler."
    );
    assert!(
        !source.contains("match on_press"),
        "IconButton should avoid duplicating Button markup based on handler presence."
    );
}

#[test]
fn icon_button_does_not_use_web_sys_directly() {
    let source = load_source("src/icon_button/view.rs");

    assert!(
        !source.contains("web_sys"),
        "IconButton should not touch `web_sys` directly; it should delegate behavior to `ui-headless` via Button."
    );
}
