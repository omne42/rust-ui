use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
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
