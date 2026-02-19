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
            "InlineAlert should expose `{needle}` for baseline motion inspection."
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
            "InlineAlert motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn inline_alert_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/inline_alert/motion.rs");
    let view_source = load_source("src/inline_alert/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: InlineAlertMotion) -> InlineAlertMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "InlineAlert motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::inline_alert::motion::sanitize_motion(motion);"),
        "InlineAlert view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn inline_alert_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn inline_alert() -> AnyView",
        "title=\"InlineAlert\"",
        "slug=\"inline-alert\"",
        "description=\"Compact alert with tone/fill variants and optional icon.\"",
        "<Playground title=\"Inline alerts\" code_signal=code>",
        "<InlineAlert",
    ] {
        assert!(
            source.contains(needle),
            "display docs should include `{needle}` for inline_alert primary playground coverage.",
        );
    }
}

#[test]
fn inline_alert_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Inline alerts\"",
        "tone=InlineAlertTone::Info",
        "fill=InlineAlertFill::Subtle",
        "title=\"Info\".to_string()",
        "description=\"Subtle fill\".to_string()",
        "tone=InlineAlertTone::Negative",
        "fill=InlineAlertFill::Border",
        "title=\"Error\".to_string()",
        "description=\"Border fill\".to_string()",
        "\"This is an inline alert.\"",
        "\"Something went wrong.\"",
    ] {
        assert!(
            source.contains(needle),
            "inline_alert docs playgrounds should contain `{needle}`.",
        );
    }
}
