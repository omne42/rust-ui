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
    ] {
        assert!(
            source.contains(name),
            "AlertBanner styles should define `{name}` so motion updates only touch CSS variables."
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
