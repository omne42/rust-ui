use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn image_does_not_expose_logic_module() {
    let source = load_source("src/image/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Image's `logic` module should stay private to avoid leaking internal view-state helpers into the public API."
    );
}

#[test]
fn image_requires_alt_text() {
    let source = load_source("src/image/view.rs");

    assert!(
        source.contains("alt: String"),
        "Image should require `alt` text to align with Spectrum-style accessibility expectations."
    );

    assert!(
        source.contains("alt=alt.clone()"),
        "Image should forward the provided `alt` text to the rendered <img>."
    );
}

#[test]
fn image_emits_expected_data_slots_and_state_attributes() {
    let source = load_source("src/image/view.rs");

    for attr in [
        "data-slot=\"image-wrapper\"",
        "data-loaded",
        "data-zoomed",
        "data-slot=\"image\"",
        "data-slot=\"image-fallback\"",
        "data-slot=\"image-skeleton\"",
    ] {
        assert!(
            source.contains(attr),
            "Image should set `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn image_uses_spring_driven_zoom_css_variable() {
    let styles = load_source("src/image/styles.rs");
    let motion = load_source("src/image/motion.rs");

    for needle in ["--ui-image-zoom", "transform: scale(var(--ui-image-zoom"] {
        assert!(
            styles.contains(needle),
            "Image styles should reference `{needle}` for spring-driven zoom feedback."
        );
    }

    assert!(
        motion.contains("--ui-image-zoom"),
        "Image motion should write `--ui-image-zoom` to drive zoom without rerendering."
    );
}

#[test]
fn image_skeleton_respects_reduced_motion() {
    let styles = load_source("src/image/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "Image skeleton shimmer should disable animation under reduced-motion via `{needle}`."
        );
    }
}

#[test]
fn image_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/image/motion.rs");
    let view_source = load_source("src/image/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ImageMotion) -> ImageMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Image motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::image::motion::sanitize_motion(motion);"),
        "Image view should sanitize motion before attaching zoom driver.",
    );
}

#[test]
fn image_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn image() -> AnyView",
        "title=\"Image\"",
        "slug=\"image\"",
        "Playground title=\"Image\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Image.",
        );
    }
}

#[test]
fn image_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let code = r#\"<Image src=Some(src.to_string()) alt=\"Demo\".to_string() />\"#;",
        "<Image",
        "src=src.to_string()",
        "alt=\"Demo image\".to_string()",
        "radius=ImageRadius::Lg",
        "shadow=ImageShadow::Md",
        "is_zoomed=true",
    ] {
        assert!(
            source.contains(needle),
            "image docs playground should contain `{needle}`.",
        );
    }
}
