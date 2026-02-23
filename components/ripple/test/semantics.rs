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
fn ripple_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/ripple/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view", "pub mod motion"] {
        assert!(
            !source.contains(needle),
            "Ripple internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn ripple_consumes_state_primitives_state_model() {
    let logic_source = load_source("../../components/ripple/src/logic.rs");
    let view_source = load_source("../../components/ripple/src/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/ripple.rs");

    for needle in [
        "pub use ui_state_primitives::ripple::{",
        "RippleStateInput",
        "normalize_optional_text",
        "resolve_phase",
        "resolve_boundary",
        "resolve_state",
        "compose_class_name",
        "pub struct RippleRenderInput",
        "pub struct RippleRenderState",
        "pub fn resolve_render_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Ripple logic should consume primitives via `{needle}`."
        );
    }

    for forbidden in [
        "pub enum RipplePhase",
        "pub enum RippleBoundary",
        "pub struct RippleStateInput",
        "pub struct RippleState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Ripple logic must not re-implement primitive state model; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum RipplePhase",
        "pub enum RippleBoundary",
        "pub struct RippleStateInput",
        "pub struct RippleState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_phase(",
        "pub fn resolve_boundary(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "motion_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Ripple state primitive should include `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_render_state(logic::RippleRenderInput {",
        "is_bounded,",
        "motion,",
        "class_name,",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
    ] {
        assert!(
            view_source.contains(needle),
            "Ripple view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn ripple_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/ripple/src/view.rs");

    for attr in [
        "data-slot=\"ripple\"",
        "style=state.style_vars",
        "data-state=state.state.phase_attr",
        "data-phase-class=state.state.phase_class",
        "data-boundary=state.state.boundary_attr",
        "data-bounded=state.state.is_bounded.then_some(\"true\")",
        "data-unbounded=state.state.is_unbounded.then_some(\"true\")",
        "data-motion-source=state.state.motion_source_attr",
        "data-custom-motion=state.state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=state.state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.state.class_source_attr",
        "data-duration-ms=duration_ms_attr",
        "data-ui-schema=\"ripple.v1\"",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(attr),
            "Ripple should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn ripple_styles_include_state_and_source_contracts() {
    let source = load_source("../../components/ripple/src/styles.rs");

    for selector in [
        ".ui-ripple--state-animated",
        ".ui-ripple[data-state=\"static\"]",
        ".ui-ripple--boundary-bounded",
        ".ui-ripple[data-boundary=\"unbounded\"]",
        ".ui-ripple[data-bounded=\"true\"]",
        ".ui-ripple[data-unbounded=\"true\"]",
        ".ui-ripple--motion-custom",
        ".ui-ripple[data-motion-source=\"custom\"]",
        ".ui-ripple--custom-class",
        ".ui-ripple[data-custom-class=\"true\"]",
        ".ui-ripple[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Ripple styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn ripple_motion_sanitizes_and_supports_origin_triggering() {
    let source = load_source("../ui-visual-primitive/src/ripple.rs");

    for needle in [
        "default_text_field_motion_tokens",
        "pub fn easing()",
        "pub fn sanitize_duration_ms(",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "trigger_ripple_with_origin_internal",
        "pub fn trigger_ripple_at(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-ripple-origin-x",
        "--ui-ripple-origin-y",
        "--ui-ripple-duration-ms",
        "duration_ms: motion.duration_ms",
    ] {
        assert!(
            source.contains(needle),
            "Ripple motion should include `{needle}` for stable duration/origin contracts."
        );
    }
}

#[test]
fn ripple_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn motion_ripple() -> AnyView",
        "title=\"MotionRipple\"",
        "slug=\"motion-ripple\"",
        "title=\"Hello World\"",
        "title=\"Animation Matrix\"",
        "title=\"Custom Boundary + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for MotionRipple.",
        );
    }
}

#[test]
fn ripple_docs_default_path_is_before_advanced_examples() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    let hello_idx = source
        .find("title=\"Hello World\"")
        .unwrap_or_else(|| panic!("missing Hello World playground"));
    let matrix_idx = source
        .find("title=\"Animation Matrix\"")
        .unwrap_or_else(|| panic!("missing Animation Matrix playground"));
    let advanced_idx = source
        .find("title=\"Custom Boundary + Class\"")
        .unwrap_or_else(|| panic!("missing advanced playground"));

    assert!(
        hello_idx < matrix_idx && matrix_idx < advanced_idx,
        "docs should present default path first, then advanced examples"
    );
}

#[test]
fn ripple_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "<MotionRipple node_ref=ripple_ref motion=RippleMotion::default() />",
        "let matrix_code = Signal::derive(move || {",
        "duration_ms: 880",
        "let custom_code = Signal::derive(move || {",
        "is_bounded=false",
        "duration_ms: 620",
        "duration_ms: 520",
        "ui::ripple::trigger_ripple_at(",
        "18.0, 48.0",
        "class_name=\"docs-ripple-custom\".to_string()",
        "\"Unbounded + Origin\"",
    ] {
        assert!(
            source.contains(needle),
            "motion-ripple docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn ripple_api_naming_and_control_contract_are_explicit() {
    let source = load_source("../../components/ripple/src/view.rs");

    assert!(
        source.contains("#[prop(optional, into)] is_bounded: Option<bool>"),
        "Ripple boolean API should follow `is_*` naming."
    );

    for forbidden in [
        "#[prop(default = true)] bounded: bool",
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "on_value_change",
    ] {
        assert!(
            !source.contains(forbidden),
            "Ripple should not expose non-standard or half-controlled API `{forbidden}`."
        );
    }
}

#[test]
fn ripple_visual_desire_reuses_default_theme_baseline_and_visual_regression_gates() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should expose visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_source.contains(needle),
            "theme visual baseline e2e spec should include `{needle}`."
        );
    }

    for needle in [
        "### MotionRipple 同步记录（2026-02-17）",
        "`#/components/motion-ripple`",
        "`is_bounded/motion/class_name/lang/dir`",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI alignment doc should include MotionRipple token `{needle}`."
        );
    }
}

#[test]
fn ripple_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-ripple = [\"dep:ui-ripple\"]",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-ripple\")]\npub use ui_ripple as ripple;"),
        "lib.rs should feature-gate ripple module export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-ripple\")]")
            && css_source.contains("out.push_str(crate::ripple::styles::CSS);"),
        "css.rs should gate ripple CSS aggregation behind component-ripple feature."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );

    for needle in [
        "cargo tree -e features -i ui -p ui --no-default-features --features",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn ripple_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let ripple_e2e_source = load_source("../../e2e/tests/docs_app_ripple_contract.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/ripple/src/check2.md");
    let view_source = load_source("../../components/ripple/src/view.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
        );
    }

    assert!(
        pages_source.contains("\"motion-ripple\""),
        "MotionRipple docs route should remain in docs perf probe traversal."
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "getAnimations().length",
        "toBeGreaterThan(0)",
        "toBe(0)",
        "await page.reload();",
    ] {
        assert!(
            ripple_e2e_source.contains(needle),
            "ripple e2e should keep repeatable settled/perf-triage token `{needle}`."
        );
    }

    assert!(
        todo_source.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "performance governance should keep render_count follow-up marker in TODO plan."
    );

    for needle in [
        "data-state=state.state.phase_attr",
        "data-boundary=state.state.boundary_attr",
        "data-motion-source=state.state.motion_source_attr",
        "data-class-source=state.state.class_source_attr",
        "data-duration-ms=duration_ms_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Ripple view should expose attribution marker `{needle}` for perf triage."
        );
    }

    for needle in [
        "性能治理：关键路径有预算（首次渲染/更新耗时/内存）",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "ripple checklist should keep perf governance baseline/follow-up token `{needle}`."
        );
    }
}

#[test]
fn ripple_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_ripple_contract.spec.mjs");
    let view_source = load_source("../../components/ripple/src/view.rs");
    let logic_source = load_source("../../components/ripple/src/logic.rs");
    let motion_source = load_source("../ui-visual-primitive/src/ripple.rs");
    let check2_source = load_source("../../components/ripple/src/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "shared wasm debug capability should remain feature-gated via `button-wasm-debug`."
    );
    assert!(
        !cargo_source.contains("ripple-wasm-debug"),
        "ripple should not introduce component-local wasm debug feature drift."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs debug visual entry should keep `{needle}`."
        );
    }

    for needle in [
        "events.push(event);",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=\"ripple.v1\"",
        "data-motion-source",
        "data-class-source",
        "await page.keyboard.press(\"Enter\");",
        "await page.reload();",
        "docs-app ripple key flow is repeatable with semantic breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ripple debug/replay path should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "trace.emit(",
        "data-debug-source",
        "request_replay.run(",
        "#[prop(optional)] debug",
        "ripple-wasm-debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ripple should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "ripple checklist should keep wasm-debug governance rule `{needle}`."
        );
    }
}

#[test]
fn ripple_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn motion_ripple() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Animation Matrix\"",
        "title=\"Custom Boundary + Class\"",
        "ui::ripple::trigger_ripple(",
        "ui::ripple::trigger_ripple_at(",
    ] {
        assert!(
            docs_source.contains(needle),
            "MotionRipple docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn ripple_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_ripple_contract.spec.mjs");
    let check2_source = load_source("../../components/ripple/src/check2.md");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let hello_ref: NodeRef<html::Span> = NodeRef::new();",
        "let default_ref: NodeRef<html::Span> = NodeRef::new();",
        "let on_unbounded_click = move |_| {",
        "trigger_ripple_at(",
    ] {
        assert!(
            docs_source.contains(needle),
            "MotionRipple docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "RIPPLE_WORKBENCH_STORAGE_KEY",
        "load_ripple_workbench_state(",
        "save_ripple_workbench_state(",
        "clear_ripple_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Ripple keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "body:not(:has(#boot))",
        "await page.reload();",
        "docs-app ripple key flow is repeatable with semantic breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ripple e2e should keep stable dev iteration marker `{needle}`."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "ripple checklist should keep DX governance rule `{required}`."
        );
    }
}
