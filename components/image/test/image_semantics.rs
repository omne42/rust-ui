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
        "pub struct ImageNormalizeInput",
        "pub struct ImageNormalizedProps",
        "pub enum ImageMotionSource",
        "pub enum ImageStatusSource",
        "pub struct ImageViewStateInput",
        "pub fn normalize_props(",
        "pub fn derive_view_state(",
        "pub fn apply_status_event(",
        "pub fn resolve_motion_source(",
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
        "let normalized = logic::normalize_props(logic::ImageNormalizeInput {",
        "let locale = locale_attrs(normalized.lang.clone(), dir);",
        "logic::derive_view_state(logic::ImageViewStateInput {",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadSucceeded)",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadFailed)",
        "let motion_source = logic::resolve_motion_source(motion);",
        "let motion = crate::motion::sanitize_motion(motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Image view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for disallowed in ["unwrap_or_default()", "unwrap_or("] {
        assert!(
            !view_source.contains(disallowed),
            "Image view should not hold local default fallback logic `{disallowed}`."
        );
    }
}

#[test]
fn image_discrete_axes_use_enum_contracts() {
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/image.rs");

    for needle in [
        "#[prop(optional)] radius: ImageRadius,",
        "#[prop(optional)] shadow: ImageShadow,",
        "pub enum ImageMotionSource",
        "data-motion-source=motion_source.as_attr()",
        "data-custom-motion=motion_source.is_custom().then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Image should enforce discrete axis enum contract `{needle}`."
        );
    }

    for needle in [
        "pub enum ImageStatus",
        "pub enum ImageRadius",
        "pub enum ImageShadow",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Image primitive discrete state must be enum-constrained: `{needle}`."
        );
    }

    for disallowed in [
        "radius: Option<String>",
        "shadow: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not model mutually-exclusive discrete state as free string `{disallowed}`."
        );
    }
}

#[test]
fn image_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("src/image/logic.rs");
    let view_source = load_source("src/image/view.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::image::{"),
        "Image logic should source state primitives from ui-state-primitives."
    );

    for needle in [
        "ImageStatus",
        "ImageStatusEvent",
        "ImageViewState",
        "derive_initial_status",
        "reduce_status",
        "resolve_view_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "Image logic should consume primitive capability `{needle}`."
        );
    }

    for disallowed in [
        "leptos::",
        "use crate::store",
        "global_store",
        "app_state",
        "redux",
    ] {
        assert!(
            !logic_source.contains(disallowed),
            "Image logic should not bind framework/business store detail `{disallowed}`."
        );
    }

    assert!(
        !view_source.contains("ui_state_primitives::"),
        "Image view should consume state capability through logic boundary instead of direct primitive import."
    );
}

#[test]
fn image_declares_async_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Async Contract"),
        "Image docs should explicitly declare async contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark async action protocol as N/A."
    );

    for disallowed in [
        "use_async_action",
        "is_loading:",
        "aria-busy",
        "on_retry",
        "#[prop(optional)] is_disabled: bool",
        "data-disabled=",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose app-level async protocol fragment `{disallowed}`."
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
        "data-status-source=move || status_source.get().as_attr()",
        "data-radius=radius.as_attr()",
        "data-shadow=shadow.as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-custom-motion=motion_source.is_custom().then_some(\"true\")",
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
fn image_motion_contract_maps_to_ui_motion_and_keeps_non_wasm_noop() {
    let motion_source = load_source("src/image/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "zoom_spring: ui_motion::presets::spring_soft()",
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Image motion should map semantic state to ui-motion contract and preserve non-wasm noop via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep a predictable non-wasm noop backend: missing `{needle}`."
        );
    }
}

#[test]
fn image_declares_motion_contractization() {
    let checklist_source = load_source("../../components/image/check2.md");
    let motion_source = load_source("src/image/motion.rs");
    let view_source = load_source("src/image/view.rs");

    assert!(
        checklist_source.contains("- [x] Motion 合同化"),
        "Image checklist should mark motion contractization item as completed with evidence."
    );

    for required in [
        "pub struct ImageMotion {",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0",
        "damping: if value.damping.is_finite() && value.damping > 0.0",
        "pub fn attach_zoom_motion(",
        "if ui_motion::web::prefers_reduced_motion()",
        "drop(style.set_property(\"--ui-image-zoom\", \"1\"));",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(required),
            "Image motion contractization guard should include `{required}`."
        );
    }

    assert!(
        view_source.contains("motion::attach_zoom_motion("),
        "Image view should keep motion attach boundary in view layer."
    );
}

#[test]
fn image_consumes_ui_theme_tokens_without_rebuilding_theme_layer() {
    let styles_source = load_source("src/image/styles.rs");
    let logic_source = load_source("src/image/logic.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");
    let ui_theme_tokens = load_source("../ui-theme/src/tokens.rs");
    let ui_theme_theme = load_source("../ui-theme/src/theme.rs");
    let ui_theme_css = load_source("../ui-theme/src/css.rs");
    let ui_theme_baseline = load_source("../ui-theme/tests/token_scale_baseline.rs");

    for needle in [
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border));",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
    ] {
        assert!(
            styles_source.contains(needle),
            "Image styles should consume ui-theme tokens through css variables: missing `{needle}`."
        );
    }

    for disallowed in ["use ui_theme", "ui_theme::Theme", "theme_to_css_variables"] {
        assert!(
            !logic_source.contains(disallowed),
            "Image logic must not rebuild theme layer; found `{disallowed}`."
        );
        assert!(
            !view_source.contains(disallowed),
            "Image view must not rebuild theme layer; found `{disallowed}`."
        );
        assert!(
            !motion_source.contains(disallowed),
            "Image motion must not rebuild theme layer; found `{disallowed}`."
        );
    }

    for needle in [
        "pub enum TokenScale",
        "pub struct ThemeTokens",
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "fn token_scale_baselines_are_regression_testable()",
    ] {
        assert!(
            ui_theme_tokens.contains(needle)
                || ui_theme_theme.contains(needle)
                || ui_theme_css.contains(needle)
                || ui_theme_baseline.contains(needle),
            "ui-theme baseline contract should expose `{needle}`."
        );
    }
}

#[test]
fn image_consumes_ui_headless_locale_and_hover_contracts() {
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(normalized.lang.clone(), dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "on:pointerenter=move |_| motion_state.hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| motion_state.hover.handlers.on_pointer_leave.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "Image view should mount ui-headless locale/hover contract `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{HoverOptions, use_hover};",
        "pub struct ImageMotionState",
        "pub hover: ui_headless::HoverState,",
        "let hover = use_hover(HoverOptions { is_disabled });",
    ] {
        assert!(
            motion_source.contains(needle),
            "Image motion should consume typed hover contract from ui-headless: `{needle}`."
        );
    }
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
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix: Loaded / Blurred / Fallback / Missing\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
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
        "let source_first_code = Signal::derive(move || {",
        "let controlled_contrast_code = Signal::derive(move || {",
        "let stream_snapshot_code = Signal::derive(move || {",
        "let basic_imports = \"use leptos::prelude::*;\\nuse ui::Image;\".to_string();",
        "let advanced_imports =",
        "<Image",
        "src=into_owned_string(src)",
        "alt=\"Demo image\".to_string()",
        "radius=ImageRadius::Lg",
        "shadow=ImageShadow::Md",
        "is_zoomed=true",
        "code_imports=basic_imports.clone()",
        "code_imports=advanced_imports.clone()",
        "test_css_source=test_css_source",
        "test_source_path=\"components/image/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "data-slot=\"image-streaming-policy\"",
        "data-slot=\"image-copy-ready-hint\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "copyable=true",
        "class_name=\"docs-image-source-copy\".to_string()",
        "controls=move || view! {",
        "SegmentedControl",
        "Switch checked=is_zoomed set_checked=set_is_zoomed",
        "Switch checked=is_blurred set_checked=set_is_blurred",
        "Switch checked=is_skeleton_disabled set_checked=set_is_skeleton_disabled",
        "Switch checked=with_fallback set_checked=set_with_fallback",
    ] {
        assert!(
            source.contains(needle),
            "image docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn image_semantic_contract_priority_is_data_aria_role_and_source_not_snapshot() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let local_semantics = load_source("../../components/image/test/semantics.rs");
    let workspace_semantics = load_source("image_semantics.rs");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "Image checklist should mark semantic-first contract item as completed."
    );
    for required in [
        "双层语义测试已覆盖 `data-state` / `data-status-source` / `data-motion-source` / `aria-hidden`",
        "`role`/键盘可达路径在当前组件职责下标注 `N/A`",
        "禁用 `assert_snapshot/insta` 作为验收主信号",
        "image_semantic_contract_priority_is_data_aria_role_and_source_not_snapshot",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 semantic-first evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Semantic Testing Contract"),
        "Image docs should keep semantic testing contract section."
    );
    for required in [
        "non-interactive image primitive; no key handler/role control contract",
        "no snapshot assertions are used as the primary acceptance signal",
    ] {
        assert!(
            readme_source.contains(required),
            "Image docs should keep semantic-first boundary fragment `{required}`."
        );
    }

    for required in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep semantic marker `{required}` as first-class contract."
        );
    }

    for required in [
        "fn image_semantic_test_matrix_is_contract_first_and_snapshot_free()",
        "fn image_semantic_contract_priority_is_data_aria_role_and_source_not_snapshot()",
    ] {
        assert!(
            local_semantics.contains(required) || workspace_semantics.contains(required),
            "semantic-first contract should keep regression anchor `{required}`."
        );
    }

    for disallowed in ["assert_snapshot", "insta::", "snapshot!"] {
        assert!(
            !local_semantics.contains(disallowed) && !workspace_semantics.contains(disallowed),
            "semantic-first contract should not depend on snapshot assertion `{disallowed}`."
        );
    }
}

#[test]
fn image_readme_includes_display_config_code_css_test_sections() {
    let source = load_source("src/image/README.md");

    for needle in [
        "# Image",
        "## Docs Playground 展示区",
        "Hello World (Default API)",
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Controlled vs Uncontrolled (N/A)",
        "Streaming Optional / Snapshot",
        "Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
        "## Documentation as Product Contract",
    ] {
        assert!(
            source.contains(needle),
            "image README should include `{needle}` for docs workbench guidance.",
        );
    }
}

#[test]
fn image_dx_paradox_keeps_hello_world_path_minimal() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    assert!(
        readme_source.contains("## Hello World"),
        "Image README should expose a hello world section."
    );

    let hello_world = r#"<Image
  src=Some("https://example.com/photo.jpg".to_string())
  alt="Cover".to_string()
/>"#;
    assert!(
        readme_source.contains(hello_world),
        "Image hello world should keep a <=5 line minimal call path."
    );

    for disallowed in ["state=", "controller=", "model=", "machine="] {
        assert!(
            !view_source.contains(disallowed),
            "Image basic API should not require internal state wiring `{disallowed}`."
        );
    }

    for needle in [
        "pub(super) fn image() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "let code = Signal::derive(move || {",
        "https://images.unsplash.com/photo-1516117172878-fd2c41f4a759",
        "<Image",
        "src=into_owned_string(src)",
        "alt=\"Demo image\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs app should expose minimal runnable image path `{needle}`."
        );
    }

    for disallowed in [
        "alt=\"Demo image\".to_string()\n                        radius=ImageRadius::Lg",
        "alt=\"Demo image\".to_string()\n                        shadow=ImageShadow::Md",
        "alt=\"Demo image\".to_string()\n                        is_zoomed=true",
    ] {
        assert!(
            !docs_source.contains(disallowed),
            "hello world display path should avoid advanced-only prop `{disallowed}`."
        );
    }
}

#[test]
fn image_docs_product_copy_paste_ready_contract() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    assert!(
        check2_source
            .contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground"),
        "Image checklist should mark docs-as-product item as completed."
    );
    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "Image checklist should mark source-first copy-paste-ready item as completed."
    );
    for required in [
        "Hello World (Default API)",
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Controlled vs Uncontrolled (N/A)",
        "Streaming Optional / Snapshot",
        "Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
        "test_source_path=\"components/image/src/view.rs\"",
        "Snippet(copyable=true)",
        "`component-image` feature + `UiRoot`/`inject-css`",
        "image_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 docs-as-product evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Documentation as Product Contract"),
        "Image README should explicitly declare documentation-as-product contract."
    );
    for required in [
        "Hello World (Default API)",
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Controlled vs Uncontrolled (N/A)",
        "Streaming Optional / Snapshot",
        "Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep docs-as-product fragment `{required}`."
        );
    }

    for required in [
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix: Loaded / Blurred / Fallback / Missing\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=basic_imports.clone()",
        "code_imports=advanced_imports.clone()",
        "data-slot=\"image-streaming-policy\"",
        "data-slot=\"image-copy-ready-hint\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "copyable=true",
        "class_name=\"docs-image-source-copy\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "Image docs page should keep docs-as-product contract fragment `{required}`."
        );
    }

    for required in [
        "pub fn compose_copy_ready_code(",
        "if !imports.trim().is_empty() {",
        "output.push_str(imports.trim_end());",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground copy helper should keep copy-paste-ready import injection fragment `{required}`."
        );
    }
}

#[test]
fn image_docs_examples_and_matrices_are_synced_with_logic_contract() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let view_source = load_source("src/image/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/image.rs");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "Image checklist should mark docs/examples/matrix sync item as completed."
    );
    for required in [
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Controlled vs Uncontrolled (N/A)",
        "radius=Lg` / `shadow=Sm`",
        "image_docs_examples_and_matrices_are_synced_with_logic_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 docs-sync evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Docs Playground 展示区"),
        "Image README should keep docs playground section."
    );
    for required in [
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Controlled vs Uncontrolled (N/A)",
        "Workbench: Display + Config + Code + CSS Test",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep docs-sync fragment `{required}`."
        );
    }

    for required in [
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix: Loaded / Blurred / Fallback / Missing\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Workbench: Display + Config + Code + CSS Test\"",
        "let (radius_index, set_radius_index) = signal(Some(2usize));",
        "let (shadow_index, set_shadow_index) = signal(Some(1usize));",
        "match radius_index.get().unwrap_or(2)",
        "_ => ImageRadius::Lg,",
        "match shadow_index.get().unwrap_or(1)",
        "_ => ImageShadow::Sm,",
        "is_skeleton_disabled=is_skeleton_disabled.get()",
        "is_blurred=is_blurred.get()",
        "is_zoomed=is_zoomed.get()",
        "radius=radius.get()",
        "shadow=shadow.get()",
        "motion=motion.get()",
        "class_name=class_name",
    ] {
        assert!(
            docs_source.contains(required),
            "Image docs page should keep docs-sync contract fragment `{required}`."
        );
    }

    for required in [
        "#[prop(optional, into)] fallback_src: Option<String>,",
        "#[prop(optional)] is_skeleton_disabled: bool,",
        "#[prop(optional)] is_blurred: bool,",
        "#[prop(optional)] is_zoomed: bool,",
        "#[prop(optional)] radius: ImageRadius,",
        "#[prop(optional)] shadow: ImageShadow,",
        "#[prop(optional)] motion: ImageMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep public API fragment `{required}` used by docs examples."
        );
    }

    for required in [
        "pub enum ImageRadius",
        "#[default]",
        "Lg,",
        "pub enum ImageShadow",
        "Sm,",
    ] {
        assert!(
            primitive_source.contains(required),
            "Image primitive should keep default enum fragment `{required}` used by docs matrix defaults."
        );
    }

    for disallowed in ["disable_skeleton", "fallback=", "zoomed=", "blurred="] {
        assert!(
            !docs_source.contains(disallowed),
            "Image docs should not drift to legacy/non-contract prop fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_documentation_as_product_is_beginner_friendly_and_default_first() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    assert!(
        check2_source.contains(
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"
        ),
        "Image checklist should mark beginner-friendly documentation item as completed."
    );
    for required in [
        "## Beginner-Friendly Path",
        "## Hello World",
        "先用起来，再进阶",
        "Hello World (Default API)",
        "image_documentation_as_product_is_beginner_friendly_and_default_first",
    ] {
        assert!(
            check2_source.contains(required)
                || readme_source.contains(required)
                || docs_source.contains(required),
            "Image beginner-friendly documentation contract should keep fragment `{required}`."
        );
    }

    for required in [
        "# Image",
        "## Beginner-Friendly Path",
        "## Hello World",
        "Minimal path is `src + alt` only",
        "Advanced behavior is opt-in via optional props",
        "## Docs Playground 展示区",
        "`Hello World (Default API)`",
        "State Matrix: Loaded / Blurred / Fallback / Missing",
        "Workbench: Display + Config + Code + CSS Test",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep beginner-friendly documentation fragment `{required}`."
        );
    }

    let hello_index = docs_source
        .find("title=\"Hello World (Default API)\"")
        .expect("docs page should contain Hello World playground title");
    let matrix_index = docs_source
        .find("title=\"State Matrix: Loaded / Blurred / Fallback / Missing\"")
        .expect("docs page should contain state matrix playground title");
    let workbench_index = docs_source
        .find("title=\"Workbench: Display + Config + Code + CSS Test\"")
        .expect("docs page should contain workbench playground title");
    assert!(
        hello_index < matrix_index && matrix_index < workbench_index,
        "docs page should keep default path first, then matrix, then advanced workbench"
    );

    for disallowed in [
        "Only source code is provided",
        "architecture-only docs",
        "requires manual ui-state-primitives wiring",
    ] {
        assert!(
            !readme_source.contains(disallowed),
            "Image README should not drift to non-beginner fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_docs_interactive_playground_contract_is_live_and_reproducible() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_image_contract.spec.mjs");

    assert!(
        check2_source.contains(
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"
        ),
        "Image checklist should mark interactive playground item as completed."
    );
    for required in [
        "Workbench: Display + Config + Code + CSS Test",
        "SegmentedControl",
        "ImageActualConfig",
        "AI Spec 子项在 `Image` 上按职责判定为 `N/A`",
        "docs_app_image_contract.spec.mjs",
        "image_docs_interactive_playground_contract_is_live_and_reproducible",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 interactive-playground evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Interactive Playground Contract"),
        "Image README should explicitly declare interactive playground contract."
    );
    for required in [
        "segmented controls: `source` / `radius` / `shadow` / `motion`",
        "switches: `is_zoomed` / `is_blurred` / `is_skeleton_disabled` / `with_fallback` / `custom_class`",
        "data-slot=\"image-workbench-stage\"",
        "ImageActualConfig",
        "AI Spec 子项适用性",
        "Spec 输入联动按职责为 `N/A`",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep interactive playground contract fragment `{required}`."
        );
    }

    for required in [
        "title=\"Workbench: Display + Config + Code + CSS Test\"",
        "description=\"Interactive panel with scoped CSS test + actual config snapshot.\"",
        "controls=move || view! {",
        "id_base=\"docs-image-source\".to_string()",
        "id_base=\"docs-image-radius\".to_string()",
        "id_base=\"docs-image-shadow\".to_string()",
        "id_base=\"docs-image-motion\".to_string()",
        "Switch checked=is_zoomed set_checked=set_is_zoomed>\"Zoomed\"</Switch>",
        "Switch checked=is_blurred set_checked=set_is_blurred>\"Blurred\"</Switch>",
        "Switch checked=is_skeleton_disabled set_checked=set_is_skeleton_disabled",
        "Switch checked=with_fallback set_checked=set_with_fallback>\"Use fallback\"</Switch>",
        "Switch checked=custom_class set_checked=set_custom_class>\"Custom class\"</Switch>",
        "data-slot=\"image-workbench-stage\"",
        "src=source",
        "fallback_src=fallback",
        "radius=radius.get()",
        "shadow=shadow.get()",
        "motion=motion.get()",
        "test_config_signal=actual_config",
        "ImageActualConfig",
        "state: source={}, fallback={}, zoomed={}, blurred={}",
    ] {
        assert!(
            docs_source.contains(required),
            "Image docs page should keep interactive playground contract fragment `{required}`."
        );
    }

    for required in [
        "docs-app image key flow is repeatable via semantic breakpoints",
        "[id^=\"docs-image-source\"]",
        "[data-slot=\"image-workbench-stage\"]",
        "[data-slot=\"playground-controls\"]",
    ] {
        assert!(
            e2e_source.contains(required),
            "Image e2e flow should keep interactive playground replay anchor `{required}`."
        );
    }
}

#[test]
fn image_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits() {
    let check2_source = load_source("../../components/image/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_image_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "Image checklist should mark E2E selector stability item as completed."
    );
    for required in [
        "e2e/tests/docs_app_image_contract.spec.mjs",
        "`data-component` / `data-slot` / `data-state`",
        "body:not(:has(#boot))",
        "idle|loading|loaded|error",
        "data-state=\"idle\"",
        "image_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 E2E selector stability evidence should keep `{required}`."
        );
    }

    for required in [
        "async function gotoImageDocsAndWaitSettled(page)",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"image\"]",
        "[data-slot=\"image-wrapper\"][data-state]",
        "toHaveAttribute(\"data-state\", /(idle|loading|loaded|error)/);",
        "[data-slot=\"image-workbench-stage\"]",
        "[data-slot=\"playground-toggle-settings\"]",
        "[data-slot=\"playground-controls\"]",
        "[id^=\"docs-image-source\"]",
        "toHaveAttribute(\"data-fallback\", \"true\");",
        "toHaveAttribute(\"data-state\", \"idle\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "Image e2e contract should keep semantic selector/wait fragment `{required}`."
        );
    }

    for disallowed in [
        "waitForTimeout(",
        "sleep(",
        "hasText:",
        "getByText(",
        "locator(\"text=",
        "getByRole(\"button\", { name:",
    ] {
        assert!(
            !e2e_source.contains(disallowed),
            "Image e2e contract should avoid fragile selector/wait fragment `{disallowed}`."
        );
    }

    assert!(
        docs_source.contains("data-slot=\"image-workbench-stage\""),
        "Image docs page should keep stable workbench stage selector for e2e anchoring."
    );
    for required in [
        "data-slot=\"playground-toggle-settings\"",
        "data-slot=\"playground-toggle-code\"",
        "data-slot=\"playground-toggle-test\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground shell should keep stable toggle selector hook `{required}`."
        );
    }
}

#[test]
fn image_repeatable_e2e_key_flow_contract_is_registered_with_semantic_breakpoints() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_image_contract.spec.mjs");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "Image checklist should mark repeatable key-flow e2e regression item as completed."
    );
    for required in [
        "docs-app image key flow is repeatable via semantic breakpoints",
        "gotoImageDocsAndWaitSettled",
        "data-slot=\"image-workbench-stage\"",
        "id^=\"docs-image-source\"",
        "data-fallback",
        "data-state",
        "data-status-source",
        "overlay/focus/keyboard 在该组件职责下为 `N/A`",
        "image_repeatable_e2e_key_flow_contract_is_registered_with_semantic_breakpoints",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 repeatable key-flow evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Repeatable E2E Key Flow Contract"),
        "Image README should explicitly declare repeatable E2E key-flow contract."
    );
    for required in [
        "e2e/tests/docs_app_image_contract.spec.mjs",
        "gotoImageDocsAndWaitSettled",
        "data-state",
        "data-status-source",
        "overlay/focus/keyboard are `N/A`",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep repeatable key-flow contract fragment `{required}`."
        );
    }

    for required in [
        "test(\"docs-app image key flow is repeatable via semantic breakpoints\"",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "const workbench = docsRoot",
        "[data-slot=\"image-workbench-stage\"]",
        "[data-slot=\"playground-toggle-settings\"]",
        "[id^=\"docs-image-source\"]",
        "toHaveAttribute(\"data-fallback\", \"true\");",
        "toHaveAttribute(\"data-state\", \"idle\");",
        "toHaveAttribute(\"data-status-source\", \"initial\");",
        "await page.reload();",
        "toHaveAttribute(\"data-status-source\", /(initial|event)/);",
    ] {
        assert!(
            e2e_source.contains(required),
            "Image e2e key-flow regression should keep semantic breakpoint fragment `{required}`."
        );
    }

    for disallowed in [
        "toHaveScreenshot(",
        "expect(page).toHaveScreenshot(",
        "compareScreenshots(",
    ] {
        assert!(
            !e2e_source.contains(disallowed),
            "Image key-flow regression should not use screenshot-only assertion `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_composition_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("### Composition Contract"),
        "Image docs should explicitly declare composition contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark composition axis as N/A."
    );

    for disallowed in [
        "ItemSpec",
        "labels",
        "titles",
        "panels",
        "children_slots",
        "items=",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose composite container API fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_macro_micro_drag_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Macro / Micro Interaction Contract"),
        "Image docs should explicitly declare macro/micro interaction contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark drag macro/micro axis as N/A."
    );

    for disallowed in [
        "Dragging",
        "DragEnd",
        "on:drag",
        "on:pointermove",
        "drag_state",
    ] {
        assert!(
            !view_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image should not expose drag state-machine fragment `{disallowed}`."
        );
    }

    for required in [
        "pub fn attach_zoom_motion(",
        "SpringAnimator::new",
        "animator.set_target(target);",
    ] {
        assert!(
            motion_source.contains(required),
            "Image motion should keep local hover micro-loop contract `{required}`."
        );
    }
}

#[test]
fn image_declares_two_pass_geometry_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Two-Pass Geometry Rendering Contract"),
        "Image docs should explicitly declare two-pass geometry rendering contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark two-pass geometry axis as N/A."
    );

    for disallowed in [
        "get_bounding_client_rect",
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "rectification",
        "measure_layout",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose geometry measurement/rectification fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_registration_protocol_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Registration Protocol Contract"),
        "Image docs should explicitly declare registration protocol contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark registration protocol axis as N/A."
    );

    for disallowed in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Vec<Item>",
        "children_slots",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose collection registration fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_slot_projection_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Slot Projection Contract"),
        "Image docs should explicitly declare slot projection contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark slot projection axis as N/A."
    );

    for disallowed in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "projection_mode",
        "keep_alive",
    ] {
        assert!(
            !view_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image should not expose slot-projection lifecycle fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_env_stream_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Env Streams Contract"),
        "Image docs should explicitly declare env streams contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark env streams axis as N/A."
    );

    for disallowed in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "ThemeChanged",
        "on:resize",
        "on:scroll",
        "debounce",
        "throttle",
        "match_media",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose env-stream fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_event_light_cone_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Event Light Cone Contract"),
        "Image docs should explicitly declare event light cone contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark event light cone axis as N/A."
    );

    for disallowed in [
        "Context Bus",
        "ContextBus",
        "SelectionState::All",
        "Selector",
        "prop drilling",
        "batch_select",
        "bulk_action",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose event-light-cone fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_causality_bus_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Causality Bus Contract"),
        "Image docs should explicitly declare causality bus contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark causality bus axis as N/A."
    );

    for disallowed in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "bus broadcast",
        "subscriber",
        "derive_command",
        "command_bus",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose causality-bus fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_focus_stack_gc_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## Focus Stack & GC Contract"),
        "Image docs should explicitly declare focus stack & gc contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark focus stack axis as N/A."
    );

    for disallowed in [
        "FallbackTo",
        "focus_manager",
        "restore_focus",
        "focus_stack",
        "document.body",
        "Selector(",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose overlay focus-stack fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_escape_hatch_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Escape Hatch / Foreign Zone Contract"),
        "Image docs should explicitly declare escape hatch / foreign zone contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark escape hatch axis as N/A."
    );

    for disallowed in [
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Mapbox",
        "leaflet",
        "chart_instance",
        "foreign_zone",
    ] {
        assert!(
            !view_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image should not expose command-style third-party integration fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_hydration_discontinuity_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let motion_source = load_source("src/image/motion.rs");
    let root_source = load_source("src/root.rs");

    assert!(
        readme_source.contains("## Hydration Discontinuity Contract"),
        "Image docs should explicitly declare hydration discontinuity contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark hydration discontinuity axis as N/A."
    );

    for required in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep deterministic id provider boundary `{required}`."
        );
    }

    for disallowed in [
        "SystemTime::now",
        "Instant::now",
        "Uuid::new_v4",
        "uuid::Uuid::new_v4",
        "Math::random",
        "rand::random",
        "thread_rng(",
    ] {
        assert!(
            !view_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image should not introduce hydration-unstable initializer fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_ssr_cross_platform_compile_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## SSR / Cross-Platform Compile Contract"),
        "Image docs should explicitly declare SSR/cross-platform compile contract."
    );

    for required in [
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-image,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui",
    ] {
        assert!(
            readme_source.contains(required) && check2_source.contains(required),
            "Image contract docs/checklist should keep compile-only evidence command `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "use leptos::wasm_bindgen::JsCast;",
        "let element: leptos::web_sys::HtmlElement = div.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(required),
            "Image motion should keep explicit platform split fragment `{required}`."
        );
    }

    let non_wasm_section = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("non-wasm branch should exist in image motion");
    let non_wasm_body = non_wasm_section
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(non_wasm_section);

    for disallowed in ["web_sys", "window(", "document(", "wasm_bindgen"] {
        assert!(
            !non_wasm_body.contains(disallowed),
            "non-wasm motion branch should not reference browser-only fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_ui_headless_web_ssr_feature_mutex_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Ui-Headless Web/SSR Feature Mutex Contract"),
        "Image docs should explicitly declare ui-headless web/ssr feature mutex contract."
    );

    for required in [
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
    ] {
        assert!(
            readme_source.contains(required) && check2_source.contains(required),
            "Image docs/checklist should keep ui-headless feature verification command `{required}`."
        );
    }

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless must keep web/ssr mutex compile guard `{required}`."
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "use ui_headless::{HoverOptions, use_hover};",
    ] {
        assert!(
            view_source.contains(required) || motion_source.contains(required),
            "Image should consume ui-headless contract without bypassing feature gate `{required}`."
        );
    }
}

#[test]
fn image_declares_ui_motion_non_wasm_noop_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let motion_source = load_source("src/image/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    assert!(
        readme_source.contains("## Ui-Motion Non-Wasm No-op Contract"),
        "Image docs should explicitly declare ui-motion non-wasm no-op contract."
    );

    let required_command = "cargo check -p ui-motion";
    assert!(
        readme_source.contains(required_command) && check2_source.contains(required_command),
        "Image docs/checklist should keep ui-motion compile-only command `{required_command}`."
    );

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should keep non-wasm stub fragment `{required}`."
        );
    }

    let non_wasm_section = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("non-wasm image motion branch should exist");
    let non_wasm_body = non_wasm_section
        .split("#[cfg(test)]")
        .next()
        .unwrap_or(non_wasm_section);

    assert!(
        non_wasm_body.contains("std::hint::black_box(sanitize_motion(motion));"),
        "Image non-wasm motion branch should degrade to deterministic sanitize-only no-op."
    );

    for disallowed in [
        "SpringAnimator::new",
        "unwrap(",
        "expect(",
        "panic!(",
        "web_sys",
    ] {
        assert!(
            !non_wasm_body.contains(disallowed),
            "Image non-wasm motion branch should not depend on runtime animation handle or panic path `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_reduced_motion_ssr_wasm_branch_contract() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let styles_source = load_source("src/image/styles.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Reduced-Motion / SSR / Wasm Branch Contract"),
        "Image docs should explicitly declare reduced-motion/SSR/wasm branch contract."
    );

    for required in [
        "@media (prefers-reduced-motion: reduce)",
        "animation: none",
        "transform: none",
    ] {
        assert!(
            styles_source.contains(required),
            "Image styles should keep reduced-motion fallback fragment `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::spring::SpringAnimator::new",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(required),
            "Image motion should preserve wasm/non-wasm split fragment `{required}`."
        );
    }

    for required in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "logic::derive_view_state(logic::ImageViewStateInput {",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "Image semantic output should stay target-agnostic across SSR/wasm `{required}`."
        );
    }

    for disallowed in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
    ] {
        assert!(
            !view_source.contains(disallowed),
            "Image view semantic path should not split by platform fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_performance_governance_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let styles_source = load_source("src/image/styles.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Performance Governance Contract"),
        "Image docs should explicitly declare performance governance contract."
    );
    assert!(
        check2_source.contains("- [x] 语义测试与性能回归"),
        "Image checklist should keep semantic/performance regression gate completed for performance axis."
    );

    for required in [
        "first render budget",
        "critical update budget",
        "memory trend budget",
        "render_count",
    ] {
        assert!(
            readme_source.contains(required) || check2_source.contains(required),
            "Image performance contract should keep budget/verification anchor `{required}`."
        );
    }

    for required in [
        "let view_state = Memo::new(move |_| {",
        "logic::derive_view_state(logic::ImageViewStateInput {",
        "set_status.update(|value| {",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadSucceeded)",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadFailed)",
    ] {
        assert!(
            view_source.contains(required),
            "Image render/update path should keep performance-attribution anchor `{required}`."
        );
    }

    for required in [
        "pub fn normalize_props(",
        "pub fn derive_view_state(",
        "pub fn apply_status_event(",
    ] {
        assert!(
            logic_source.contains(required),
            "Image state path should keep performance-attribution anchor `{required}`."
        );
    }

    for required in ["var(--ui-", ".ui-image__img", ".ui-image__skeleton"] {
        assert!(
            styles_source.contains(required),
            "Image style path should keep static performance contract anchor `{required}`."
        );
    }

    for required in [
        "ui_motion::spring::SpringAnimator::new",
        "std::hint::black_box(sanitize_motion(motion));",
        "on_cleanup(move || {",
    ] {
        assert!(
            motion_source.contains(required),
            "Image motion path should keep performance-attribution anchor `{required}`."
        );
    }

    for disallowed in ["set_interval", "set_timeout", "request_animation_frame("] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image state/render path should avoid unbounded scheduler fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_view_macro_complexity_is_controlled() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let view_source = load_source("src/image/view.rs");

    assert!(
        readme_source.contains("## View Macro Complexity Contract"),
        "Image docs should explicitly declare view macro complexity contract."
    );

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 5,
        "Image view should keep bounded macro count after helper decomposition (expected <= 5, got {view_macro_count})."
    );

    let wrapper_shell_count = view_source.matches("data-slot=\"image-wrapper\"").count();
    assert_eq!(
        wrapper_shell_count, 1,
        "Image view should keep a single top-level wrapper shell."
    );

    let show_count = view_source.matches("<Show when=").count();
    assert!(
        show_count <= 4,
        "Image view should keep shallow conditional branch count (expected <= 4, got {show_count})."
    );

    let line_count = view_source.lines().count();
    assert!(
        line_count <= 180,
        "Image view source should stay compact enough to avoid giant macro body (got {line_count} lines)."
    );

    for required in [
        "data-slot=\"image-wrapper\"",
        "data-slot=\"image\"",
        "data-slot=\"image-fallback\"",
        "data-slot=\"image-skeleton\"",
    ] {
        assert!(
            view_source.contains(required),
            "Macro complexity control must preserve semantic sub-block `{required}`."
        );
    }

    assert!(
        check2_source.contains("`view!` 宏复杂度受控"),
        "Macro complexity governance checklist entry should stay explicit."
    );
}

#[test]
fn image_declares_functional_decomposition_preferred() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let view_source = load_source("src/image/view.rs");

    assert!(
        readme_source.contains("## Functional Decomposition Contract"),
        "Image docs should explicitly declare functional decomposition contract."
    );
    assert!(
        check2_source.contains("函数式拆分优先"),
        "Functional decomposition checklist entry should stay explicit."
    );

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "Image should expose only one public #[component] entry."
    );

    for required in [
        "fn render_blurred_layer(",
        "fn render_fallback_layer(",
        "fn render_image_layer(",
        "fn render_skeleton_layer()",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep helper fragment `{required}`."
        );
    }
}

#[test]
fn image_declares_static_fragments_are_constantized() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let view_source = load_source("src/image/view.rs");

    assert!(
        readme_source.contains("## Static Fragment Constantization Contract"),
        "Image docs should explicitly declare static fragment constantization contract."
    );
    assert!(
        check2_source.contains("静态片段常量化"),
        "Static fragment constantization checklist entry should stay explicit."
    );

    for required in [
        "const BLURRED_CLASS: &str = \"ui-image__blurred\";",
        "const FALLBACK_CLASS: &str = \"ui-image__fallback\";",
        "const IMAGE_CLASS: &str = \"ui-image__img\";",
        "const SKELETON_CLASS: &str = \"ui-image__skeleton\";",
        "const DECORATIVE_ALT_TEXT: &str = \"\";",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep static-fragment constant `{required}`."
        );
    }

    for required in [
        "class=BLURRED_CLASS",
        "class=FALLBACK_CLASS",
        "class=IMAGE_CLASS",
        "class=SKELETON_CLASS",
        "alt=DECORATIVE_ALT_TEXT",
        "data-slot=\"image-blurred\"",
        "data-slot=\"image-fallback\"",
        "data-slot=\"image\"",
        "data-slot=\"image-skeleton\"",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should mount constantized static fragment anchor `{required}`."
        );
    }
}

#[test]
fn image_declares_inner_html_safety_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Inner HTML Safety Contract"),
        "Image docs should explicitly declare inner-html safety contract."
    );
    assert!(
        check2_source.contains("`inner_html` 使用约束"),
        "Inner-html checklist entry should stay explicit."
    );

    for disallowed in [
        "inner_html=",
        "inner_html =",
        "set_inner_html(",
        "dangerously_set_inner_html",
        ".inner_html(",
        ".innerHTML",
    ] {
        assert!(
            !view_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image source should not contain inner-html injection fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_wasm_debug_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("src/image/check2.md");
    let cargo_source = load_source("../../components/image/Cargo.toml");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## WASM Debug Contract"),
        "Image docs should explicitly declare wasm debug contract."
    );
    assert!(
        check2_source.contains("WASM 调试要求"),
        "Wasm debug checklist entry should stay explicit."
    );
    assert!(
        cargo_source.contains("default = []") && cargo_source.contains("wasm-debug = []"),
        "Image cargo feature contract should keep wasm-debug opt-in with default-off baseline."
    );

    for required in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "on:load=on_load",
        "on:error=on_error",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadSucceeded)",
        "logic::apply_status_event(*value, logic::ImageStatusEvent::LoadFailed)",
    ] {
        assert!(
            view_source.contains(required),
            "Image debug trace/replay anchor should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("data-visual-baseline=\"image-default-theme\""),
        "Wasm debug contract should keep docs visual debug entry marker."
    );
    for disallowed in ["is_debug:", "debug_mode:", "on_debug_change"] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image public API should not leak debug-only prop fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_dx_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let styles_source = load_source("src/image/styles.rs");

    assert!(
        readme_source.contains("## DX Contract"),
        "Image docs should explicitly declare dx contract."
    );
    assert!(
        check2_source.contains("- [x] DX 要求"),
        "Image checklist should mark dx item as completed with scoped rationale."
    );
    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Image styles should expose static css contract for css-test workflow."
    );

    for required in [
        "title=\"Workbench: Display + Config + Code + CSS Test\"",
        "test_css_source=test_css_source",
        "test_source_path=\"components/image/src/styles.rs\".to_string()",
        "\"/* components/image/src/styles.rs */\\n{}\"",
        "let (source_index, set_source_index) = signal(Some(0usize));",
        "Switch checked=is_zoomed set_checked=set_is_zoomed>\"Zoomed\"</Switch>",
        "is_zoomed=is_zoomed.get()",
        "test_config_signal=actual_config",
        "ImageActualConfig",
        "data-visual-baseline=\"image-default-theme\"",
    ] {
        assert!(
            docs_source.contains(required),
            "Image dx contract should keep docs-app workbench evidence `{required}`."
        );
    }
}

#[test]
fn image_declares_engineering_capability_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let protocol_source = load_source("src/image/protocol.rs");
    let mod_source = load_source("src/image/mod.rs");
    let logic_source = load_source("src/image/logic.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Engineering Capability Contract"),
        "Image docs should explicitly declare engineering capability contract."
    );
    assert!(
        check2_source.contains("- [x] 工程能力统一"),
        "Image checklist should mark engineering capability item as completed."
    );

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum ImageComponentSchemaVersion",
        "pub struct ImageComponentSpec",
        "#[serde(default)]",
        "schema_version: ImageComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep structured serde contract `{required}`."
        );
    }

    for disallowed in [
        "tokio::",
        "async_std::",
        "async-std",
        "JoinHandle",
        "Runtime",
        "tracing::",
        "#[instrument]",
    ] {
        assert!(
            !mod_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !view_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image component layer should not leak runtime/tracing detail `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_a11y_i18n_l10n_contracts() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");

    assert!(
        readme_source.contains("## A11y / I18n / L10n Contract"),
        "Image docs should explicitly declare a11y/i18n/l10n contract."
    );

    for required in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "alt: String,",
        "alt=move || alt.get_value()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view_source.contains(required),
            "Image should keep a11y/i18n semantic contract `{required}`."
        );
    }

    for disallowed in [
        "role=\"button\"",
        "role=\"link\"",
        "on:keydown",
        "tabindex=",
        "Loading...",
        "Error",
        "Fallback text",
        "fn aria_",
    ] {
        assert!(
            !view_source.contains(disallowed) && !logic_source.contains(disallowed),
            "Image should not expose interactive/hardcoded-localized/duplicated-a11y fragment `{disallowed}`."
        );
    }
}

#[test]
fn image_state_markers_are_observable_retrievable_and_closed() {
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let logic_source = load_source("src/image/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/image.rs");

    assert!(
        readme_source.contains("## State Observability Contract"),
        "Image docs should explicitly declare state observability contract."
    );

    for marker in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-loaded=move || view_state.get().is_loaded.then_some(\"true\")",
        "data-fallback=move || view_state.get().show_fallback.then_some(\"true\")",
        "data-skeleton=move || view_state.get().show_skeleton.then_some(\"true\")",
        "data-blurred=move || view_state.get().show_blurred.then_some(\"true\")",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view_source.contains(marker),
            "Image should keep stable observable marker `{marker}`."
        );
    }

    for needle in [
        "pub enum ImageStatusSource",
        "Self::Initial => \"initial\"",
        "Self::Event => \"event\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "Image logic should keep closed status-source enum contract `{needle}`."
        );
    }

    for needle in [
        "pub enum ImageStatus",
        "pub fn as_attr(self) -> &'static str",
        "\"idle\"",
        "\"loading\"",
        "\"loaded\"",
        "\"error\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Image primitive should expose closed data-state value set: `{needle}`."
        );
    }
}

#[test]
fn image_styles_depend_on_explicit_state_markers_only() {
    let readme_source = load_source("src/image/README.md");
    let styles_source = load_source("src/image/styles.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Style Explicit-State Contract"),
        "Image docs should explicitly declare style explicit-state contract."
    );

    for required in [
        ".ui-image[data-state=\"loaded\"] .ui-image__skeleton",
        ".ui-image[data-loaded=\"true\"] .ui-image__skeleton",
        ".ui-image[data-radius=\"sm\"]",
        ".ui-image[data-shadow=\"md\"]",
        ".ui-image[data-custom-motion=\"true\"] .ui-image__img",
    ] {
        assert!(
            styles_source.contains(required),
            "Image styles should keep explicit state selector `{required}`."
        );
    }

    for disallowed in [":nth-child", ":nth-of-type", " > :not(", "style=\""] {
        assert!(
            !styles_source.contains(disallowed) && !view_source.contains(disallowed),
            "Image should not use fragile selector or inline style fragment `{disallowed}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-image-zoom\""),
        "Image motion runtime should update only CSS custom property for visual state changes."
    );
    for disallowed in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
    ] {
        assert!(
            !motion_source.contains(disallowed),
            "Image motion should avoid business layout inline style mutation `{disallowed}`."
        );
    }
}

#[test]
fn image_semantic_test_matrix_is_contract_first_and_snapshot_free() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");
    let local_semantics = load_source("../../components/image/test/semantics.rs");
    let workspace_semantics = load_source("tests/image_semantics.rs");

    assert!(
        readme_source.contains("## Semantic Testing Contract"),
        "Image docs should explicitly declare semantic testing contract."
    );
    assert!(
        check2_source.contains("- [x] 语义测试与性能回归"),
        "Image checklist should mark semantic/performance regression item as completed."
    );

    for required in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "aria-hidden=\"true\"",
        "on:pointerenter=move |_| motion_state.hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| motion_state.hover.handlers.on_pointer_leave.run(())",
        "use ui_headless::{HoverOptions, use_hover};",
    ] {
        assert!(
            view_source.contains(required) || motion_source.contains(required),
            "Image semantic matrix should keep required coverage anchor `{required}`."
        );
    }

    for required in [
        "fn image_declares_controlled_uncontrolled_axis_as_not_applicable()",
        "fn image_declares_async_axis_as_not_applicable()",
        "fn image_declares_focus_stack_gc_axis_as_not_applicable()",
        "fn image_declares_escape_hatch_axis_as_not_applicable()",
        "fn image_declares_hydration_discontinuity_axis_as_not_applicable()",
        "fn image_declares_ssr_cross_platform_compile_contract()",
        "fn image_declares_ui_headless_web_ssr_feature_mutex_contract()",
        "fn image_declares_ui_motion_non_wasm_noop_contract()",
        "fn image_declares_reduced_motion_ssr_wasm_branch_contract()",
        "fn image_declares_performance_governance_contract()",
        "fn image_declares_view_macro_complexity_is_controlled()",
        "fn image_declares_functional_decomposition_preferred()",
        "fn image_declares_static_fragments_are_constantized()",
        "fn image_declares_inner_html_safety_contract()",
        "fn image_declares_wasm_debug_contract()",
        "fn image_declares_dx_contract()",
        "fn image_declares_engineering_capability_contract()",
        "fn image_declares_defensive_variables_contract()",
        "fn image_declares_cascade_layer_contract()",
        "fn image_declares_a11y_i18n_l10n_contracts()",
        "fn image_declares_ui_components_fixed_entry_contract()",
        "fn image_declares_file_placement_discipline_contract()",
        "fn image_declares_hyper_structure_builder_axis_as_not_applicable()",
        "fn image_declares_context_compression_manifest_rbi_contract()",
        "fn image_declares_agent_contract_schema_markers()",
        "fn image_declares_motion_contractization()",
        "fn image_motion_contract_maps_to_ui_motion_and_keeps_non_wasm_noop()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            local_semantics.contains(required) || workspace_semantics.contains(required),
            "Image semantic matrix should cover branch `{required}`."
        );
    }

    for disallowed in ["assert_snapshot", "insta::", "snapshot!"] {
        assert!(
            !local_semantics.contains(disallowed) && !workspace_semantics.contains(disallowed),
            "Image semantic contract tests should not rely on snapshot assertion `{disallowed}`."
        );
    }
}

#[test]
fn image_files_respect_layered_responsibilities() {
    let readme_source = load_source("src/image/README.md");
    let mod_source = load_source("src/image/mod.rs");
    let logic_source = load_source("src/image/logic.rs");
    let styles_source = load_source("src/image/styles.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## File Responsibility Contract"),
        "Image docs should explicitly declare file responsibility contract."
    );

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Image;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep module/export boundary fragment `{required}`."
        );
    }
    for disallowed in ["view! {", "set_property(", "fn normalize_props("] {
        assert!(
            !mod_source.contains(disallowed),
            "mod.rs should not hold implementation detail `{disallowed}`."
        );
    }

    for disallowed in [
        "view! {",
        "NodeRef<",
        "set_property(",
        ".ui-image",
        "on:pointerenter",
    ] {
        assert!(
            !logic_source.contains(disallowed),
            "logic.rs should not contain view/dom/style/runtime detail `{disallowed}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-image[data-state=\"loaded\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static css fragment `{required}`."
        );
    }
    for disallowed in [
        "view! {",
        "#[component]",
        "on:load=",
        "on:error=",
        "use leptos",
    ] {
        assert!(
            !styles_source.contains(disallowed),
            "styles.rs should not include render/event logic `{disallowed}`."
        );
    }

    for required in [
        "let normalized = logic::normalize_props(logic::ImageNormalizeInput {",
        "logic::derive_view_state(logic::ImageViewStateInput {",
        "let locale = locale_attrs(normalized.lang.clone(), dir);",
        "view! {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render/headless mount fragment `{required}`."
        );
    }
    for disallowed in ["ui_motion::spring::SpringAnimator::new", "pub const CSS"] {
        assert!(
            !view_source.contains(disallowed),
            "view.rs should not include motion engine/css bundle detail `{disallowed}`."
        );
    }

    for required in [
        "pub fn sanitize_motion(motion: ImageMotion) -> ImageMotion",
        "pub fn use_image_motion(is_disabled: bool) -> ImageMotionState",
        "pub fn attach_zoom_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract fragment `{required}`."
        );
    }
    for disallowed in [
        "view! {",
        "data-state=",
        "normalize_props(",
        "pub const CSS",
    ] {
        assert!(
            !motion_source.contains(disallowed),
            "motion.rs should not include view/logic/styles implementation detail `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_file_placement_discipline_contract() {
    let readme_source = load_source("src/image/README.md");

    assert!(
        readme_source.contains("## File Placement Discipline Contract"),
        "Image docs should explicitly declare file placement discipline contract."
    );

    for required in [
        "src/image/mod.rs",
        "src/image/logic.rs",
        "src/image/styles.rs",
        "src/image/view.rs",
        "src/image/motion.rs",
    ] {
        assert!(
            resolve_source_path(required).is_some(),
            "Image component should keep required source file `{required}`."
        );
    }

    assert!(
        resolve_source_path("src/image/protocol.rs").is_some(),
        "Image component should keep protocol.rs as schema boundary file."
    );
    assert!(
        resolve_source_path("src/image/render.rs").is_none(),
        "Image component should not introduce render.rs drift."
    );
    assert!(
        resolve_source_path("src/image/spec.rs").is_none(),
        "Image component should keep spec.rs absent for simple-scope contract."
    );
}

#[test]
fn image_declares_hyper_structure_builder_axis_as_not_applicable() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let protocol_source = load_source("src/image/protocol.rs");
    let mod_source = load_source("src/image/mod.rs");

    assert!(
        readme_source.contains("## Hyper-Structure Builder Contract"),
        "Image docs should explicitly declare hyper-structure builder contract."
    );
    assert!(
        readme_source.contains("`N/A`"),
        "Image docs should mark hyper-structure builder axis as N/A."
    );
    assert!(
        check2_source.contains("- [x] Hyper-Structure Builder（`spec.rs`）"),
        "Image checklist should mark hyper-structure builder axis as completed with N/A rationale."
    );

    assert!(
        resolve_source_path("src/image/spec.rs").is_none(),
        "simple Image component should keep spec.rs absent for non-complex scope."
    );
    assert!(
        protocol_source.contains("pub struct ImageComponentSpec"),
        "Image protocol should keep minimal versioned schema boundary."
    );

    for disallowed in [
        "ImageSpec::new(",
        "Spec::new(",
        ".render()",
        "render_spec(",
        "mod spec;",
    ] {
        assert!(
            !protocol_source.contains(disallowed) && !mod_source.contains(disallowed),
            "Image should not expose complex-builder fragment `{disallowed}` in non-complex scope."
        );
    }
}

#[test]
fn image_declares_context_compression_manifest_rbi_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let manifest_source = load_source("src/image/Component.toml");
    let rbi_source = load_source("src/image/image.rbi");

    assert!(
        readme_source.contains("## Context Compression (Manifest + RBI) Contract"),
        "Image docs should explicitly declare context compression manifest/rbi contract."
    );
    assert!(
        check2_source.contains("- [x] 上下文压缩协议（Manifest + RBI）"),
        "Image checklist should mark manifest+rbi item as completed."
    );

    assert!(
        resolve_source_path("src/image/Component.toml").is_some(),
        "Image component should keep src/image/Component.toml for manifest contract."
    );
    assert!(
        resolve_source_path("src/image/image.rbi").is_some(),
        "Image component should keep src/image/image.rbi for signature projection contract."
    );

    for required in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"Image\"",
        "crate = \"ui-image\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"signature-projection\"",
        "ty = \"src/image.rbi\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Image manifest should keep context-compression fragment `{required}`."
        );
    }

    for required in [
        "pub type ImageRadius = ui_state_primitives::image::ImageRadius;",
        "pub type ImageShadow = ui_state_primitives::image::ImageShadow;",
        "pub type ImageMotion = ui_image::ImageMotion;",
        "pub fn Image(",
        "alt: String",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            rbi_source.contains(required),
            "Image rbi should keep signature projection fragment `{required}`."
        );
    }
}

#[test]
fn image_declares_agent_contract_schema_markers() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let view_source = load_source("src/image/view.rs");
    let protocol_source = load_source("src/image/protocol.rs");
    let manifest_source = load_source("src/image/Component.toml");
    let rbi_source = load_source("src/image/image.rbi");

    assert!(
        readme_source.contains("## Agent Contract / Snapshot"),
        "Image docs should explicitly declare agent contract schema section."
    );
    assert!(
        check2_source.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化）"),
        "Image checklist should mark agent contract schema item as completed."
    );

    for required in [
        "data-ui-schema=protocol::IMAGE_AGENT_SCHEMA",
        "data-ui-intent=protocol::ImageAgentIntent::Display.as_attr()",
        "data-ui-action=move || protocol::action_from_status_source(status_source.get()).as_attr()",
        "data-ui-state=move || view_state.get().status_attr",
        "data-ui-status-source=move || status_source.get().as_attr()",
        "data-ui-motion-source=motion_source.as_attr()",
        "data-ui-content-source=move || protocol::content_source_from_view_state(view_state.get()).as_attr()",
        "data-ui-radius-source=external_prop_source",
        "data-ui-shadow-source=external_prop_source",
        "data-ui-stream-support=protocol::ImageStreamSupport::Optional.as_attr()",
        "data-ui-stream-fallback=protocol::ImageStreamFallback::Snapshot.as_attr()",
        "data-ui-llm-mode=protocol::ImageLlmRenderMode::Snapshot.as_attr()",
        "data-ui-output-status=protocol::ImageOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep agent-contract marker fragment `{required}`."
        );
    }

    for required in [
        "pub const IMAGE_AGENT_SCHEMA: &str = \"ui.image.agent-contract/v1\";",
        "pub enum ImageAgentIntent",
        "pub enum ImageAgentAction",
        "pub enum ImageAgentPropSource",
        "pub enum ImageContentSource",
        "pub enum ImageStreamSupport",
        "pub enum ImageStreamFallback",
        "pub enum ImageLlmRenderMode",
        "pub enum ImageOutputStatus",
        "pub const fn action_from_status_source(",
        "pub const fn content_source_from_view_state(",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep typed agent-contract schema fragment `{required}`."
        );
    }

    for required in [
        "name = \"agent-contract-schema\"",
        "name = \"agent_contract_schema_markers\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Image manifest should keep agent-contract schema capability fragment `{required}`."
        );
    }

    for required in [
        "pub const IMAGE_AGENT_SCHEMA: &'static str;",
        "pub enum ImageAgentIntent",
        "pub enum ImageAgentAction",
        "pub enum ImageAgentPropSource",
        "pub enum ImageContentSource",
        "pub enum ImageStreamSupport",
        "pub enum ImageStreamFallback",
        "pub enum ImageLlmRenderMode",
        "pub enum ImageOutputStatus",
    ] {
        assert!(
            rbi_source.contains(required),
            "Image rbi should keep agent-contract schema projection fragment `{required}`."
        );
    }

    for disallowed in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !view_source.contains(disallowed),
            "Image agent-contract render path should keep whitelist boundary and reject `{disallowed}`."
        );
    }
}

#[test]
fn image_limits_llm_rendering_to_streaming_snapshot_modes() {
    let check2_source = load_source("../../components/image/check2.md");
    let protocol_source = load_source("src/image/protocol.rs");
    let view_source = load_source("src/image/view.rs");

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "Image checklist should mark llm streaming/snapshot two-mode item as completed."
    );

    for required in [
        "pub enum ImageLlmRenderMode",
        "Streaming",
        "Snapshot",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep llm render-mode two-value fragment `{required}`."
        );
    }

    assert!(
        view_source.contains("data-ui-llm-mode=protocol::ImageLlmRenderMode::Snapshot.as_attr()"),
        "Image view should expose current llm render mode marker as snapshot baseline."
    );
}

#[test]
fn image_declares_snapshot_as_required_baseline_capability() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let protocol_source = load_source("src/image/protocol.rs");

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "Image checklist should mark snapshot baseline item as completed."
    );
    assert!(
        readme_source.contains("`Snapshot` is a required baseline capability for `Image`"),
        "Image docs should declare snapshot as required baseline capability."
    );

    for required in [
        "data-ui-stream-support=protocol::ImageStreamSupport::Optional.as_attr()",
        "data-ui-stream-fallback=protocol::ImageStreamFallback::Snapshot.as_attr()",
        "data-ui-llm-mode=protocol::ImageLlmRenderMode::Snapshot.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep snapshot baseline semantic marker `{required}`."
        );
    }

    for required in [
        "pub enum ImageStreamFallback",
        "Snapshot",
        "pub enum ImageLlmRenderMode",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep snapshot baseline type contract fragment `{required}`."
        );
    }
}

#[test]
fn image_declares_streaming_axis_by_component_responsibility() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let view_source = load_source("src/image/view.rs");
    let protocol_source = load_source("src/image/protocol.rs");

    assert!(
        check2_source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "Image checklist should mark streaming responsibility item as completed."
    );

    for required in [
        "streaming is `optional` instead of required",
        "fallback is explicitly `snapshot`",
        "data-ui-output-status",
        "data validation, reconnect, and retry belong to upper layers",
    ] {
        assert!(
            readme_source.contains(required),
            "Image docs should keep streaming-responsibility fragment `{required}`."
        );
    }

    for required in [
        "data-ui-stream-support=protocol::ImageStreamSupport::Optional.as_attr()",
        "data-ui-stream-fallback=protocol::ImageStreamFallback::Snapshot.as_attr()",
        "data-ui-output-status=protocol::ImageOutputStatus::Verified.as_attr()",
        "aria-hidden=\"true\"",
        "alt=move || alt.get_value()",
    ] {
        assert!(
            view_source.contains(required),
            "Image view should keep streaming optional semantic continuity marker `{required}`."
        );
    }

    for required in [
        "pub enum ImageStreamSupport",
        "Optional",
        "pub enum ImageStreamFallback",
        "Snapshot",
        "pub enum ImageOutputStatus",
        "Draft",
        "Verified",
        "Submittable",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep streaming/output-status closed contract fragment `{required}`."
        );
    }
}

#[test]
fn image_declares_rust_hygiene_contract() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let logic_source = load_source("src/image/logic.rs");
    let source_files = [
        load_source("src/image/mod.rs"),
        load_source("src/image/logic.rs"),
        load_source("src/image/motion.rs"),
        load_source("src/image/protocol.rs"),
        load_source("src/image/styles.rs"),
        load_source("src/image/view.rs"),
    ];

    assert!(
        check2_source.contains("- [x] 代码卫生（Rust Hygiene）"),
        "Image checklist should mark rust hygiene item as completed."
    );
    assert!(
        readme_source.contains("## Rust Hygiene Contract"),
        "Image docs should explicitly declare rust hygiene contract."
    );
    assert!(
        readme_source.contains("./scripts/check-rust-hygiene.sh"),
        "Image docs should keep rust hygiene verification command."
    );

    for source in &source_files {
        for disallowed in ["unwrap(", "expect("] {
            assert!(
                !source.contains(disallowed),
                "non-test Image source should forbid `{disallowed}`."
            );
        }

        let has_swallowed_let = source
            .lines()
            .map(str::trim_start)
            .any(|line| line.starts_with("let _ ="));
        assert!(
            !has_swallowed_let,
            "non-test Image source should forbid `let _ = ...` swallowing."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "fn compose_base_class(radius: ImageRadius, shadow: ImageShadow) -> Cow<'static, str>",
        "Cow::Borrowed(",
    ] {
        assert!(
            logic_source.contains(required),
            "Image logic should keep string-copy hotspot cow fragment `{required}`."
        );
    }
}

#[test]
fn image_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let protocol_source = load_source("src/image/protocol.rs");
    let manifest_source = load_source("src/image/Component.toml");

    assert!(
        check2_source.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "Image checklist should mark version deprecation migration item as completed."
    );
    for required in [
        "N/A：本次 `Image` 改动未引入跨大版本 API 破坏升级",
        "ImageComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.image.agent-contract/v1",
        "image_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 deprecation-migration N/A evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## Version Deprecation Migration Contract"),
        "Image docs should explicitly declare version deprecation migration contract."
    );
    for required in [
        "`N/A` for current `Image` change set",
        "ImageComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.image.agent-contract/v1",
        "Schema Registry must register a deprecation window",
        "migrate_v1_to_v2",
    ] {
        assert!(
            readme_source.contains(required),
            "Image docs should keep deprecation migration contract fragment `{required}`."
        );
    }

    for required in [
        "pub enum ImageComponentSchemaVersion",
        "#[default]",
        "V1,",
        "pub struct ImageComponentSpec",
        "pub schema_version: ImageComponentSchemaVersion,",
        "pub const IMAGE_AGENT_SCHEMA: &str = \"ui.image.agent-contract/v1\";",
    ] {
        assert!(
            protocol_source.contains(required),
            "Image protocol should keep v1 schema baseline fragment `{required}`."
        );
    }
    assert!(
        manifest_source.contains("schema_version = \"1\""),
        "Image component manifest should keep schema version at 1 without deprecation migration trigger."
    );

    for forbidden in [
        "V2",
        "migrate_v1_to_v2(",
        "deprecation_window",
        "deprecated_since",
    ] {
        assert!(
            !protocol_source.contains(forbidden) && !manifest_source.contains(forbidden),
            "Image source should not introduce migration implementation token `{forbidden}` without breaking upgrade."
        );
    }
}

#[test]
fn image_enforces_token_first_static_style_contract() {
    let readme_source = load_source("src/image/README.md");
    let styles_source = load_source("src/image/styles.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");
    let css_registry_source = load_source("src/css.rs");
    let ui_root_source = load_source("src/root.rs");

    assert!(
        readme_source.contains("## Token-First Static Style Contract"),
        "Image docs should explicitly declare token-first static style contract."
    );

    for required in [
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border));",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));",
        ".ui-image[data-radius=\"sm\"]",
        ".ui-image[data-shadow=\"md\"]",
        "transform: scale(var(--ui-image-zoom, var(--ui-fallback-image-zoom-initial)));",
    ] {
        assert!(
            styles_source.contains(required),
            "Image styles should keep token-first css fragment `{required}`."
        );
    }

    for disallowed in ["--image-", "--component-", "styled-components", "emotion::"] {
        assert!(
            !styles_source.contains(disallowed),
            "Image styles should not introduce non-ui token or css-in-rust default fragment `{disallowed}`."
        );
    }

    assert!(
        css_registry_source.contains("#[cfg(feature = \"component-image\")]")
            && css_registry_source.contains("out.push_str(crate::image::styles::CSS);"),
        "ui css registry should aggregate Image styles via feature-gated push."
    );
    assert!(
        ui_root_source.contains("if inject_components_css.get_value() {")
            && ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should remain the centralized component-css injection boundary."
    );

    assert!(
        !view_source.contains("style=\""),
        "Image view should not emit inline style for business visual logic."
    );
    assert!(
        motion_source.contains("set_property(\"--ui-image-zoom\""),
        "Image runtime should mutate only CSS custom property for motion."
    );
    for disallowed in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !motion_source.contains(disallowed),
            "Image motion should not write layout style property `{disallowed}`."
        );
    }
}

#[test]
fn image_declares_ui_components_fixed_entry_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../ui-headless/src/presence.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");

    assert!(
        readme_source.contains("## Ui-Components Fixed Entry Contract"),
        "Image docs should explicitly declare ui fixed entry contract."
    );
    assert!(
        check2_source.contains("- [x] `ui` 固定入口文件落点正确。"),
        "Image checklist should mark ui fixed entry item as completed."
    );

    assert!(
        lib_source.contains("#[cfg(feature = \"component-image\")]")
            && lib_source.contains("pub use ui_image as image;"),
        "ui lib.rs should keep Image export behind component-image feature gate."
    );
    for disallowed in ["pub use web_sys", "pub use leptos::web_sys"] {
        assert!(
            !lib_source.contains(disallowed),
            "ui lib.rs should not re-export platform detail fragment `{disallowed}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-image\")]",
        "out.push_str(crate::image::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui css registry should keep fixed entry fragment `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep centralized injection/i18n fragment `{required}`."
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "set_property(\"--ui-active-highlight-y\"",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight primitive should keep shared motion fragment `{required}`."
        );
    }
    for disallowed in ["ui_image", "ImageStatus", "data-state="] {
        assert!(
            !active_highlight_source.contains(disallowed),
            "active_highlight primitive should not include image business fragment `{disallowed}`."
        );
    }

    for (path, label) in [
        ("src/overlay_open.rs", "overlay_open.rs"),
        ("src/presence.rs", "presence.rs"),
        ("src/a11y.rs", "a11y.rs"),
    ] {
        assert!(
            resolve_source_path(path).is_none(),
            "ui fixed entry contract forbids `{label}` in src/."
        );
    }

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            controllable_state_source.contains(required)
                || presence_source.contains(required)
                || a11y_source.contains(required),
            "headless ownership boundary should keep `{required}`."
        );
    }
}

#[test]
fn image_declares_defensive_variables_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let styles_source = load_source("src/image/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    assert!(
        readme_source.contains("## Defensive Variables Contract"),
        "Image docs should explicitly declare defensive variables contract."
    );
    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）"),
        "Image checklist should mark defensive variables item as completed."
    );

    for required in [
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-radius-full, var(--ui-fallback-radius-full))",
        "var(--ui-image-blur, var(--ui-fallback-image-blur))",
        "var(--ui-image-blur-scale, var(--ui-fallback-image-blur-scale))",
        "var(--ui-image-blur-opacity, var(--ui-fallback-image-blur-opacity))",
        "var(--ui-image-skeleton-duration, var(--ui-fallback-image-skeleton-duration))",
        "var(--ui-image-shimmer-start, var(--ui-fallback-image-shimmer-start))",
        "var(--ui-image-shimmer-end, var(--ui-fallback-image-shimmer-end))",
    ] {
        assert!(
            styles_source.contains(required),
            "Image styles should keep defensive variable chain `{required}`."
        );
    }

    for disallowed in [
        "999px",
        "14px",
        "1.12",
        "0.45",
        "220% 100%",
        "1.3s",
        "120% 0",
        "-120% 0",
        "1px solid var(--ui-border)",
    ] {
        assert!(
            !styles_source.contains(disallowed),
            "Image styles should not keep bare terminal constant `{disallowed}`."
        );
    }

    for required in [
        "--ui-radius-full: 999px;",
        "--ui-fallback-radius-full: 999px;",
        "--ui-image-blur: 14px;",
        "--ui-fallback-image-blur: 14px;",
        "--ui-image-skeleton-duration: 1.3s;",
        "--ui-fallback-image-skeleton-duration: 1.3s;",
        "--ui-image-shimmer-start: 120%;",
        "--ui-fallback-image-shimmer-end: -120%;",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should remain the ssot fallback source for defensive variable `{required}`."
        );
    }
}

#[test]
fn image_declares_cascade_layer_contract() {
    let readme_source = load_source("src/image/README.md");
    let check2_source = load_source("../../components/image/check2.md");
    let css_registry_source = load_source("src/css.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Cascade Layer Contract"),
        "Image docs should explicitly declare cascade layer contract."
    );
    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）"),
        "Image checklist should mark cascade layer item as completed."
    );

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-image\")]",
        "out.push_str(crate::image::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_registry_source.contains(required),
            "ui css registry should keep cascade-layer aggregation fragment `{required}`."
        );
    }

    for disallowed in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style:top=",
        "style:left=",
        "style:width=",
        "style:height=",
    ] {
        assert!(
            !view_source.contains(disallowed) && !motion_source.contains(disallowed),
            "Image component should not emit fragile inline layout style fragment `{disallowed}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-image-zoom\""),
        "Image runtime style adjustment should go through css custom property mutation."
    );
    for disallowed in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
    ] {
        assert!(
            !motion_source.contains(disallowed),
            "Image runtime should not mutate layout style property `{disallowed}`."
        );
    }
}

#[test]
fn image_visual_desire_contract_is_documented_and_mounted_in_docs_app() {
    let checklist_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    assert!(
        checklist_source.contains("- [x] 默认主题美学质量达标（Visual Desire）"),
        "Image checklist should mark visual desire item as completed with scoped rationale."
    );

    for required in [
        "## Visual Desire Contract",
        "HeroUI",
        "Button/Input/Overlay",
        "Default Theme Visual Baseline (Visual Desire)",
        "data-visual-baseline=\"image-default-theme\"",
        "State Matrix: Loaded / Blurred / Fallback / Missing",
    ] {
        assert!(
            readme_source.contains(required) || docs_source.contains(required),
            "Image visual desire contract should keep evidence fragment `{required}`."
        );
    }

    for disallowed in [
        "Bootstrap 3",
        "legacy bootstrap",
        "unstyled fallback demo only",
    ] {
        assert!(
            !readme_source.contains(disallowed) && !docs_source.contains(disallowed),
            "Image visual desire contract should not regress to coarse legacy baseline `{disallowed}`."
        );
    }
}

#[test]
fn image_heroui_benchmark_docs_and_component_docs_are_synced() {
    let check2_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let heroui_strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步"),
        "Image checklist should mark heroui benchmark docs-sync item as completed."
    );
    for required in [
        "Image 同步记录（2026-02-20）",
        "component_doc!(\"Image\", \"image\", \"Display\", display::image)",
        "docs/research/spectrum-heroui-style-interface-study.md",
        "image_heroui_benchmark_docs_and_component_docs_are_synced",
    ] {
        assert!(
            check2_source.contains(required),
            "components/image/check2 heroui docs-sync evidence should keep `{required}`."
        );
    }

    assert!(
        readme_source.contains("## HeroUI Benchmark Sync Contract"),
        "Image README should explicitly declare heroui benchmark sync contract."
    );
    for required in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "component_doc!(\"Image\", \"image\", \"Display\", display::image)",
        "slug=\"image\"",
        "docs/research/spectrum-heroui-style-interface-study.md",
        "only code updates without synchronized docs updates",
    ] {
        assert!(
            readme_source.contains(required),
            "Image README should keep heroui docs-sync contract fragment `{required}`."
        );
    }

    for required in [
        "### Image 同步记录（2026-02-20）",
        "`Image` 参数主轴保持",
        "component_doc!(\"Image\", \"image\", \"Display\", display::image)",
        "title=\"Image\"",
        "slug=\"image\"",
        "components/image/src/README.md",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            heroui_strategy_source.contains(required),
            "heroui strategy doc should keep image sync fragment `{required}`."
        );
    }

    assert!(
        docs_pages_source
            .contains("component_doc!(\"Image\", \"image\", \"Display\", display::image)"),
        "docs page registry should keep indexable Image entry."
    );
    for required in ["title=\"Image\"", "slug=\"image\""] {
        assert!(
            docs_display_source.contains(required),
            "Image docs page should keep indexable doc entry fragment `{required}`."
        );
    }
}

#[test]
fn image_tree_shaking_contract_is_feature_gated_and_css_is_prunable() {
    let checklist_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        checklist_source.contains("- [x] Tree Shaking 是一等能力"),
        "Image checklist should mark tree-shaking item as completed with verifiable evidence."
    );
    assert!(
        checklist_source.contains("- [x] Tree Shaking & 特性剪裁"),
        "Image checklist should mark tree-shaking feature-pruning gate item as completed."
    );
    assert!(
        readme_source.contains("## Tree Shaking Contract"),
        "Image docs should explicitly declare tree-shaking contract."
    );

    for required in [
        "component-image = [\"dep:ui-image\"]",
        "ui-image = { path = \"../../components/image\", optional = true }",
        "all-components = [",
        "component-image",
    ] {
        assert!(
            cargo_source.contains(required),
            "ui cargo feature tree should include `{required}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-image\")]")
            && lib_source.contains("pub use ui_image as image;"),
        "Image export in ui lib.rs should remain behind component-image feature gate."
    );
    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-image\")]\n    out.push_str(crate::image::styles::CSS);"
        ),
        "Image css injection should remain behind component-image feature gate."
    );
    assert_eq!(
        css_source
            .matches("out.push_str(crate::image::styles::CSS);")
            .count(),
        1,
        "Image css should be aggregated through a single feature-gated entry point."
    );

    for disallowed in [
        "COMPONENT_REGISTRY",
        "ALL_COMPONENTS_REGISTRY",
        "HashMap::<&'static str",
    ] {
        assert!(
            !lib_source.contains(disallowed) && !css_source.contains(disallowed),
            "tree-shaking contract should not depend on unconditional central registry `{disallowed}`."
        );
    }
}

#[test]
fn image_type_and_semantic_markers_form_machine_readable_contract() {
    let checklist_source = load_source("../../components/image/check2.md");
    let readme_source = load_source("src/image/README.md");
    let logic_source = load_source("src/image/logic.rs");
    let view_source = load_source("src/image/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/image.rs");

    assert!(
        checklist_source.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态"),
        "Image checklist should mark type+semantic machine-readable item as completed."
    );
    assert!(
        readme_source.contains("## Type + Semantic Machine-Readable Contract"),
        "Image docs should explicitly declare type+semantic machine-readable contract."
    );

    for required in [
        "pub enum ImageMotionSource",
        "pub enum ImageStatusSource",
        "pub fn normalize_props(",
        "pub fn derive_view_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "Image logic should keep typed normalization contract fragment `{required}`."
        );
    }

    for required in [
        "pub enum ImageStatus",
        "pub enum ImageRadius",
        "pub enum ImageShadow",
    ] {
        assert!(
            primitive_source.contains(required),
            "Image primitive should keep enum-constrained discrete axis `{required}`."
        );
    }

    for marker in [
        "data-state=move || view_state.get().status_attr",
        "data-status-source=move || status_source.get().as_attr()",
        "data-motion-source=motion_source.as_attr()",
        "data-radius=radius.as_attr()",
        "data-shadow=shadow.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Image view should expose machine-readable semantic marker `{marker}`."
        );
    }
}

#[test]
fn image_keeps_spec_rs_absent_for_simple_component_scope() {
    let readme_source = load_source("src/image/README.md");
    let mod_source = load_source("src/image/mod.rs");
    let logic_source = load_source("src/image/logic.rs");
    let view_source = load_source("src/image/view.rs");
    let motion_source = load_source("src/image/motion.rs");

    assert!(
        readme_source.contains("## Spec.rs Policy Contract"),
        "Image docs should explicitly declare spec.rs policy contract."
    );
    assert!(
        resolve_source_path("src/image/spec.rs").is_none(),
        "simple Image component should not add src/image/spec.rs without schema-level justification"
    );

    for disallowed in ["mod spec;", "pub mod spec;", "ImageSpec", "render_spec("] {
        assert!(
            !mod_source.contains(disallowed)
                && !logic_source.contains(disallowed)
                && !view_source.contains(disallowed)
                && !motion_source.contains(disallowed),
            "Image component should not expose spec.rs-oriented symbol `{disallowed}`."
        );
    }
}
