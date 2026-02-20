use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn image_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/image/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Image internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn image_uses_logic_state_model() {
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/image.rs");

    for needle in [
        "pub use ui_state_primitives::image::{",
        "ImageRadius",
        "ImageShadow",
        "ImageStatus",
        "ImageViewState",
        "normalize_optional_text",
        "resolve_view_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "Image logic should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub enum ImageStatus",
        "pub enum ImageRadius",
        "pub enum ImageShadow",
        "pub struct ImageViewState",
        "pub use crate::button::normalize_optional_text;",
        "pub fn resolve_view_state(",
        "pub fn as_attr(self) -> &'static str",
        "pub fn class_name(self) -> &'static str",
        "status_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Image primitives should include `{needle}` for centralized state derivation.",
        );
    }

    for needle in [
        "let src = logic::normalize_optional_text(src);",
        "let fallback_src = logic::normalize_optional_text(fallback_src);",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "logic::resolve_view_state(",
        "let motion = crate::motion::sanitize_motion(motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Image view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn image_requires_alt_text() {
    let source = load_source("src/image/view.rs");

    assert!(
        source.contains("alt: String"),
        "Image should require `alt` text to align with baseline-style accessibility expectations."
    );

    for needle in [
        "alt=move || alt.get_value()",
        "#[prop(optional, into)] lang: Option<String>,",
    ] {
        assert!(
            source.contains(needle),
            "Image view should keep `{needle}`."
        );
    }
}

#[test]
fn image_emits_expected_data_slots_and_state_attributes() {
    let source = load_source("src/image/view.rs");

    for attr in [
        "data-slot=\"image-wrapper\"",
        "data-state=move || view_state.get().status_attr",
        "data-loaded=move || view_state.get().is_loaded.then_some(\"true\")",
        "data-zoomed=is_zoomed.then_some(\"true\")",
        "data-fallback=move || view_state.get().show_fallback.then_some(\"true\")",
        "data-skeleton=move || view_state.get().show_skeleton.then_some(\"true\")",
        "data-blurred=move || view_state.get().show_blurred.then_some(\"true\")",
        "data-radius=radius.as_attr()",
        "data-shadow=shadow.as_attr()",
        "data-motion-source=motion_source",
        "data-custom-motion=(motion_source == \"custom\").then_some(\"true\")",
        "data-slot=\"image\"",
        "data-slot=\"image-fallback\"",
        "data-slot=\"image-skeleton\"",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "Image should set `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn image_uses_spring_driven_zoom_css_variable() {
    let styles = load_source("src/image/styles.rs");
    let motion = load_source("src/image/motion.rs");

    for needle in [
        "--ui-image-zoom",
        "transform: scale(var(--ui-image-zoom",
        ".ui-image[data-custom-motion=\"true\"] .ui-image__img",
    ] {
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
fn image_styles_include_state_marker_contracts() {
    let source = load_source("src/image/styles.rs");

    for selector in [
        ".ui-image[data-radius=\"sm\"]",
        ".ui-image[data-radius=\"md\"]",
        ".ui-image[data-radius=\"lg\"]",
        ".ui-image[data-radius=\"full\"]",
        ".ui-image[data-shadow=\"none\"]",
        ".ui-image[data-shadow=\"sm\"]",
        ".ui-image[data-shadow=\"md\"]",
        ".ui-image[data-state=\"loaded\"] .ui-image__skeleton",
        ".ui-image[data-loaded=\"true\"] .ui-image__skeleton",
    ] {
        assert!(
            source.contains(selector),
            "Image styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn image_skeleton_respects_reduced_motion() {
    let styles = load_source("src/image/styles.rs");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        "animation: none",
        "transform: none",
    ] {
        assert!(
            styles.contains(needle),
            "Image should disable motion-sensitive effects under reduced-motion via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn image_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/image/motion.rs");
    let view_source = load_source("src/image/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ImageMotion) -> ImageMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Image motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);")
            || view_source.contains("let motion = crate::image::motion::sanitize_motion(motion);"),
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
        "<Playground title=\"Image\" code_signal=code>",
        "<Playground title=\"Comparison Matrix: Loaded / Blurred / Fallback / Missing\" code_signal=matrix_code>",
        "<Playground\n                title=\"Workbench: Display + Config + Code + CSS Test\"",
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
        "let code = Signal::derive(move || {",
        "let workbench_code = Signal::derive(move || {",
        "let test_css_source = Signal::derive(move || {",
        "let actual_config = Signal::derive(move || {",
        "<Image",
        "src=src.to_string()",
        "alt=\"Demo image\".to_string()",
        "radius=ImageRadius::Lg",
        "shadow=ImageShadow::Md",
        "is_zoomed=true",
        "test_css_source=test_css_source",
        "test_source_path=\"components/image/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "controls=move || view! {",
        "SegmentedControl",
        "Switch checked=is_zoomed set_checked=set_is_zoomed",
        "Switch checked=is_blurred set_checked=set_is_blurred",
        "Switch checked=disable_skeleton set_checked=set_disable_skeleton",
        "Switch checked=with_fallback set_checked=set_with_fallback",
    ] {
        assert!(
            source.contains(needle),
            "image docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn image_readme_includes_display_config_code_css_test_sections() {
    let source = load_source("src/image/README.md");

    for needle in [
        "# Image",
        "## Docs Playground 展示区",
        "展示（Display）",
        "Config（配置面板）",
        "Code（代码面板）",
        "CSS Test（样式测试面板）",
        "Comparison Matrix: Loaded / Blurred / Fallback / Missing",
    ] {
        assert!(
            source.contains(needle),
            "image README should include `{needle}` for docs workbench guidance.",
        );
    }
}
