use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn alert_banner_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/alert_banner/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AlertBanner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn alert_banner_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/alert_banner/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::AlertBanner;"),
        "alert_banner module should export `AlertBanner`."
    );
    assert!(
        crate_source
            .contains("pub use alert_banner::{AlertBanner, AlertBannerFill, AlertBannerMotion, AlertBannerTone};"),
        "crate root should re-export AlertBanner contract."
    );
}

#[test]
fn alert_banner_attaches_motion_driver() {
    let source = load_source("src/alert_banner/view.rs");

    assert!(
        source.contains("attach_motion"),
        "AlertBanner should attach its motion driver to deliver spring-based reveal motion."
    );
}

#[test]
fn alert_banner_exposes_motion_source_markers() {
    let source = load_source("src/alert_banner/view.rs");

    for needle in [
        "data-slot=\"alert-banner\"",
        "data-motion-source=if motion == AlertBannerMotion::default()",
        "data-custom-motion=(motion != AlertBannerMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "AlertBanner should expose `{needle}` for baseline motion inspection."
        );
    }
}

#[test]
fn alert_banner_motion_uses_spring_animator() {
    let source = load_source("src/alert_banner/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "AlertBanner motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn alert_banner_styles_use_only_css_variables_for_motion() {
    let source = load_source("src/alert_banner/styles.rs");

    for name in [
        "--ui-alert-banner-opacity",
        "--ui-alert-banner-translate-y",
        "--ui-alert-banner-scale",
        ".ui-alert-banner[data-motion-source=\"custom\"]",
        ".ui-alert-banner[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(name),
            "AlertBanner styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn alert_banner_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/alert_banner/motion.rs");

    for needle in [
        "pub struct AlertBannerMotion",
        "fn default_motion_matches_alert_banner_spring_contract()",
        "fn supports_custom_spring_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "AlertBanner motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn alert_banner_docs_page_exists_in_display_extra() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn alert_banner() -> AnyView",
        "title=\"AlertBanner\"",
        "slug=\"alert-banner\"",
        "<AlertBanner",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra docs page should contain `{needle}`."
        );
    }
}

#[test]
fn alert_banner_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/alert_banner/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AlertBannerMotion) -> AlertBannerMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let _ = sanitize_motion(motion);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            source.contains(needle),
            "AlertBanner motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn alert_banner_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn alert_banner() -> AnyView",
        "title=\"AlertBanner\"",
        "slug=\"alert-banner\"",
        "Playground title=\"Tone + Fill\"",
        "Playground title=\"Bold + Hidden Icon + Custom Class\"",
        "Playground title=\"Custom motion contract\"",
    ] {
        assert!(
            source.contains(needle),
            "display-extra docs page should contain `{needle}` for AlertBanner.",
        );
    }
}

#[test]
fn alert_banner_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Tone + Fill\"",
        "tone=AlertBannerTone::Info",
        "fill=AlertBannerFill::Border",
        "tone=AlertBannerTone::Negative",
        "fill=AlertBannerFill::Subtle",
        "title=\"Bold + Hidden Icon + Custom Class\"",
        "tone=AlertBannerTone::Notice",
        "fill=AlertBannerFill::Bold",
        "hide_icon=true",
        "class_name=\"docs-alert-banner-custom\".to_string()",
        "title=\"Custom motion contract\"",
        "motion=AlertBannerMotion {",
        "Inspect data-motion-source/data-custom-motion markers.",
    ] {
        assert!(
            source.contains(needle),
            "alert-banner docs playgrounds should contain `{needle}`.",
        );
    }
}
