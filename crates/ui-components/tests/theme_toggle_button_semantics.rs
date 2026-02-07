use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn theme_toggle_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_theme_toggle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ThemeToggleButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn theme_toggle_button_uses_logic_state_model() {
    let view_source = load_source("src/button_theme_toggle/view.rs");
    let logic_source = load_source("src/button_theme_toggle/logic.rs");

    for needle in [
        "pub struct ThemeToggleState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_modes(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub current_mode_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ThemeToggleButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let modes = logic::normalize_modes(modes);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let class = logic::compose_class_name(",
        "let state = Memo::new(move |_| {",
    ] {
        assert!(
            view_source.contains(needle),
            "ThemeToggleButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn theme_toggle_button_uses_button_wrapper_contract() {
    let source = load_source("src/button_theme_toggle/view.rs");

    for needle in [
        "<Button",
        "aria_label=aria_label",
        "class_name=class",
        "variant=variant",
        "size=size",
        "disabled=disabled",
        "on_press=on_press",
    ] {
        assert!(
            source.contains(needle),
            "ThemeToggleButton should forward `{needle}` to Button for consistent semantics."
        );
    }
}

#[test]
fn theme_toggle_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_theme_toggle/view.rs");

    for needle in [
        "data-slot=\"theme-toggle-icon\"",
        "data-state=move || if state.get().is_disabled { \"disabled\" } else { \"enabled\" }",
        "data-current-mode=move || state.get().current_mode_attr",
        "data-next-mode=move || state.get().next_mode_attr",
        "data-mode-count=move || state.get().mode_count.to_string()",
        "data-custom-modes=move || state.get().has_custom_modes.then_some(\"true\")",
        "data-custom-aria-label=move || state.get().has_custom_aria_label.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ThemeToggleButton should expose `{needle}` for Spectrum-style state inspection."
        );
    }
}

#[test]
fn theme_toggle_button_does_not_ignore_motion_contract() {
    let source = load_source("src/button_theme_toggle/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "ThemeToggleButton should honor `ThemeToggleMotion` rather than ignoring it."
    );
}

#[test]
fn theme_toggle_button_attaches_motion_driver() {
    let source = load_source("src/button_theme_toggle/view.rs");

    assert!(
        source.contains("attach_motion"),
        "ThemeToggleButton should attach its motion driver to deliver icon micro-interactions."
    );
}

#[test]
fn theme_toggle_button_styles_define_css_vars_for_motion() {
    let source = load_source("src/button_theme_toggle/styles.rs");

    for name in ["--ui-theme-toggle-rotate", "--ui-theme-toggle-scale"] {
        assert!(
            source.contains(name),
            "ThemeToggleButton styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn theme_toggle_motion_uses_spring_animator() {
    let source = load_source("src/button_theme_toggle/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "ThemeToggleMotion should animate via a spring to match the repo's motion spec."
    );
}
