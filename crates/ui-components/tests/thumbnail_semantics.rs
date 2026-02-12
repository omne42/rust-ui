use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn thumbnail_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/thumbnail/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Thumbnail internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn thumbnail_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/thumbnail/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Thumbnail;"),
        "thumbnail module should export `Thumbnail`."
    );
    assert!(
        crate_source.contains("pub use thumbnail::{Thumbnail, ThumbnailMotion, ThumbnailSize};"),
        "crate root should re-export Thumbnail contract."
    );
}

#[test]
fn thumbnail_attaches_motion_driver() {
    let source = load_source("src/thumbnail/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Thumbnail should attach its motion driver for focus/selection feedback."
    );
}

#[test]
fn thumbnail_emits_motion_marker_attributes() {
    let source = load_source("src/thumbnail/view.rs");

    for attr in [
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(attr),
            "Thumbnail should expose `{attr}` for stable motion-source inspection."
        );
    }
}

#[test]
fn thumbnail_styles_include_motion_marker_contracts() {
    let source = load_source("src/thumbnail/styles.rs");

    for selector in [
        ".ui-thumbnail[data-motion-source=\"custom\"]",
        ".ui-thumbnail[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Thumbnail styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn thumbnail_motion_uses_spring_animator() {
    let source = load_source("src/thumbnail/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Thumbnail motion should animate via springs to match the repo motion spec."
    );
}

#[test]
fn thumbnail_motion_contract_defaults_match_heroui_level_expectations() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "stiffness: 260.0",
        "damping: 19.0",
        "mass: 1.0",
        "active_scale: 1.03",
        "active_ring_opacity: 1.0",
        "pub fn disabled() -> Self",
        "enabled: false",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion contract should include `{needle}` for HeroUI-level defaults and disabled-path stability."
        );
    }
}

#[test]
fn thumbnail_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/thumbnail/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ThumbnailMotion) -> ThumbnailMotion",
        ".clamp(1.0, 1.2)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            source.contains(needle),
            "Thumbnail motion implementation should include `{needle}` to avoid HeroUI-level motion regressions."
        );
    }
}

#[test]
fn thumbnail_styles_use_css_variables_for_motion() {
    let source = load_source("src/thumbnail/styles.rs");

    for name in ["--ui-thumbnail-scale", "--ui-thumbnail-ring-opacity"] {
        assert!(
            source.contains(name),
            "Thumbnail styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn thumbnail_docs_page_exists_in_display_extra_thumbnail() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "pub(super) fn thumbnail() -> AnyView",
        "title=\"Thumbnail\"",
        "slug=\"thumbnail\"",
        "<Thumbnail",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_thumbnail docs page should contain `{needle}`."
        );
    }
}

#[test]
fn thumbnail_docs_page_includes_custom_motion_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "ThumbnailMotion {",
        "motion=custom_motion",
        "motion=ThumbnailMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_thumbnail docs page should include `{needle}` for custom motion contract demos."
        );
    }
}

#[test]
fn thumbnail_docs_default_and_state_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "<Playground title=\"Sizes\" code=size_code>",
        "size=ThumbnailSize::Size100",
        "size=ThumbnailSize::Size500",
        "size=ThumbnailSize::Size900",
        "alt=\"Landscape\"",
        "alt=\"Portrait\"",
        "alt=\"Panorama\"",
        "<Playground title=\"Cover + Background + Layer + Selected\" code=state_code>",
        "size=ThumbnailSize::Size600",
        "background=\"#0f172a\".to_string()",
        "cover=true",
        "layer=true",
        "selected=true",
        "focused=true",
        "class_name=\"docs-thumbnail-custom\".to_string()",
        "alt=\"Cover sample\"",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail docs default/state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn thumbnail_docs_custom_motion_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs");

    for needle in [
        "<Playground title=\"Custom Motion Contract\" code=motion_code>",
        "let custom_motion = ThumbnailMotion {",
        "active_scale: 1.08,",
        "active_ring_opacity: 0.9,",
        "..ThumbnailMotion::default()",
        "motion=custom_motion",
        "motion=ThumbnailMotion::disabled()",
        "alt=\"Hero motion contract\"",
        "alt=\"Reduced motion contract\"",
    ] {
        assert!(
            source.contains(needle),
            "thumbnail docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn thumbnail_docs_page_covers_primary_playgrounds() {
    thumbnail_docs_page_exists_in_display_extra_thumbnail();
}

#[test]
fn thumbnail_docs_playgrounds_lock_state_matrix_contract_values() {
    thumbnail_docs_default_and_state_playgrounds_lock_contract_values();
    thumbnail_docs_custom_motion_playground_locks_contract_values();
}
