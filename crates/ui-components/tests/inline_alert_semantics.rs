use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn inline_alert_does_not_ignore_motion_contract() {
    let source = load_source("src/inline_alert/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "InlineAlert should honor `InlineAlertMotion` rather than ignoring it."
    );
}

#[test]
fn inline_alert_attaches_motion_driver() {
    let source = load_source("src/inline_alert/view.rs");

    assert!(
        source.contains("attach_motion"),
        "InlineAlert should attach its motion driver to deliver spring-based reveal motion."
    );
}

#[test]
fn inline_alert_exposes_motion_source_markers() {
    let source = load_source("src/inline_alert/view.rs");

    for needle in [
        "data-slot=\"inline-alert\"",
        "data-motion-source=if motion == InlineAlertMotion::default()",
        "data-custom-motion=(motion != InlineAlertMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "InlineAlert should expose `{needle}` for Spectrum/HeroUI motion inspection."
        );
    }
}

#[test]
fn inline_alert_styles_use_only_css_variables_for_motion() {
    let source = load_source("src/inline_alert/styles.rs");

    for name in [
        "--ui-inline-alert-opacity",
        "--ui-inline-alert-translate-y",
        "--ui-inline-alert-scale",
        ".ui-inline-alert[data-motion-source=\"custom\"]",
        ".ui-inline-alert[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(name),
            "InlineAlert styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn inline_alert_motion_uses_spring_animator() {
    let source = load_source("src/inline_alert/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "InlineAlert motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn inline_alert_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/inline_alert/motion.rs");

    for needle in [
        "pub struct InlineAlertMotion",
        "fn default_motion_matches_inline_alert_spring_contract()",
        "fn supports_custom_spring_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "InlineAlert motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}
