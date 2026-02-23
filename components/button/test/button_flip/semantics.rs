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
fn button_flip_module_reexports_flip_button_contracts() {
    let source = load_source("src/button/flip/mod.rs");

    for needle in [
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_flip module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_flip_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button;",
        "pub use button::flip::{FlipButton, FlipButtonMotion, FlipDirection};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_flip compatibility.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn ui_components_fixed_entry_files_follow_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    for required in ["lib.rs", "css.rs", "root.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "ui fixed entry file should exist: `{required}`."
        );
    }
    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "shared active_highlight primitive should exist in ui-visual-primitive."
    );

    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "ui should not introduce forbidden root file `{forbidden}`."
        );
    }

    let lib_source = load_source("src/lib.rs");
    for needle in [
        "mod css;",
        "#[cfg(feature = \"component-button\")]\npub mod button;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should include `{needle}`."
        );
    }

    let css_source = load_source("src/css.rs");
    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-button_flip\")]",
        "out.push_str(crate::button::flip::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css entry should include `{needle}`."
        );
    }

    let root_source = load_source("src/root.rs");
    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "ui root entry should include `{needle}`."
        );
    }
}

#[test]
fn flip_button_theme_pipeline_is_backed_by_ui_theme_layers() {
    let ui_theme_lib = load_source("../../crates/ui-theme/src/lib.rs");
    let ui_theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let ui_theme_theme = load_source("../../crates/ui-theme/src/theme.rs");
    let ui_theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let ui_root_source = load_source("src/root.rs");
    let flip_styles = load_source("src/button/flip/styles.rs");

    for needle in ["pub mod tokens;", "pub mod theme;", "pub mod css;"] {
        assert!(
            ui_theme_lib.contains(needle),
            "ui-theme should keep layered module boundary `{needle}`.",
        );
    }

    for needle in [
        "pub struct ThemeTokens",
        "pub struct ButtonMotionTokens",
        "pub struct ButtonLayoutTokens",
        "pub struct SemanticColorTokens",
        "pub struct SemanticRoleTokens",
    ] {
        assert!(
            ui_theme_tokens.contains(needle),
            "ui-theme token catalog should expose structured token types `{needle}`.",
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub struct Theme",
        "pub fn to_css_variables(&self) -> String",
        "pub fn button_motion_tokens(ctx: ThemeContext) -> ButtonMotionTokens",
        "pub fn button_layout_tokens(ctx: ThemeContext) -> ButtonLayoutTokens",
    ] {
        assert!(
            ui_theme_theme.contains(needle),
            "ui-theme mapping layer should keep axis/token resolution contract `{needle}`.",
        );
    }

    for needle in [
        "pub const BASE_CSS: &str = r#\"",
        "pub enum SemanticVariable",
        "pub struct SemanticOverrides",
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "--ui-system",
        "--ui-color",
        "--ui-scale",
    ] {
        assert!(
            ui_theme_css.contains(needle),
            "ui-theme css layer should keep css-variable generation contract `{needle}`.",
        );
    }

    for needle in [
        "use ui_theme::{SemanticOverrides, Theme, css};",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "data-theme-system=move || state.get().theme_system_attr",
        "data-theme-color=move || state.get().theme_color_attr",
        "data-theme-scale=move || state.get().theme_scale_attr",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "ui UiRoot should consume ui-theme as the single theme variable source `{needle}`.",
        );
    }

    for needle in [
        "var(--ui-flip-progress",
        "var(--ui-flip-front-offset",
        "var(--ui-flip-back-offset",
    ] {
        assert!(
            flip_styles.contains(needle),
            "flip button styles should consume css variables instead of local hardcoded theme values `{needle}`.",
        );
    }
}

#[test]
fn flip_button_visual_desire_reuses_theme_visual_baseline_gate() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let flip_docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "component_doc!(\n        \"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "theme visual baseline docs gate should include `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression gate should include `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
    ] {
        assert!(
            flip_docs_source.contains(needle),
            "flip button docs entry should stay under same default-theme quality gate `{needle}`.",
        );
    }
}

#[test]
fn flip_button_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("src/button/flip/check2.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "\"flip-button\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "flip button page should keep performance budget contract `{needle}`.",
        );
    }

    for needle in [
        "for (const slug of slugs.slice(0, limit)) {",
        "for (const slug of slugs) {",
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs e2e coverage should enforce performance regression guard `{needle}`.",
        );
    }

    assert!(
        pages_source.contains(
            "component_doc!(\"FlipButton\", \"flip-button\", \"Actions\", actions::flip_button)"
        ),
        "flip button docs page should stay in components coverage traversal.",
    );

    for needle in [
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "flip button performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn flip_button_tree_shaking_uses_component_feature_gates() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let button_mod_source = load_source("src/button/mod.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "component-button_flip = [\"component-button\"]",
        "#[cfg(feature = \"component-button\")]\npub mod button;",
        "#[cfg(feature = \"component-button_flip\")]\npub mod flip;",
        "#[cfg(feature = \"component-button_flip\")]\n    out.push_str(crate::button::flip::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || button_mod_source.contains(needle)
                || css_source.contains(needle),
            "flip button tree-shaking contract should keep feature gate `{needle}`.",
        );
    }

    for forbidden in [
        "component-button_flip = [\"all-components\"]",
        "component-button_flip = [\"web-demo-components\"]",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "flip button feature should not be coupled to aggregate feature gate `{forbidden}`.",
        );
    }
}

#[test]
fn docs_actions_page_covers_flip_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "<FlipButton",
        "FlipDirection::Top",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip-button coverage.",
        );
    }
}

#[test]
fn button_flip_logic_tracks_class_and_motion_source_markers() {
    let source = load_source("src/button/flip/logic.rs");

    for needle in [
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub has_custom_motion: bool",
        "pub class_source_attr: &'static str",
        "pub motion_source_attr: &'static str",
        "pub fn resolve_agent_contract(state: FlipButtonState) -> super::super::logic::ButtonAgentContract",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
        "super::super::logic::ButtonAgentStateAxis::Ready",
        "use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};",
        "let core = resolve_state_core(FlipButtonStateCoreInput {",
        "class_source_attr: core.class_source_attr",
        "motion_source_attr: core.motion_source_attr",
        "ui-flip-button--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "flip button logic should include `{needle}` for stable source-marker derivation.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn button_flip_motion_contract_defaults_and_sanitize_paths_are_locked() {
    let source = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub struct FlipButtonMotion",
        "spring: crate::button::motion::ButtonMotion::default().spring",
        "let sanitized = crate::button::motion::sanitize_motion(crate::button::motion::ButtonMotion {",
        "sanitize_spring_with_fallback(motion.spring, base.spring)",
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "flip button motion should include `{needle}` for baseline-level spring contract stability."
        );
    }
}

#[test]
fn button_flip_view_wires_motion_and_source_markers() {
    let source = load_source("src/button/flip/view.rs");

    for needle in [
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "let state = Signal::derive(move || {",
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));",
        "has_custom_motion,",
        "motion::attach_motion(node_ref, is_active, direction, motion)",
        "data-ui-agent-schema=move || agent_contract.get().schema_name",
        "data-ui-agent-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-capability-press=move || {",
        "data-ui-capability-focus=move || {",
        "data-ui-capability-hover=move || {",
        "data-ui-capability-popup-trigger=move || {",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "flip button view should include `{needle}` for stable motion/source marker contracts."
        );
    }
}

#[test]
fn button_flip_styles_include_source_marker_selectors() {
    let source = load_source("src/button/flip/styles.rs");

    for needle in [
        ".ui-flip-button[data-class-source=\"custom\"]",
        ".ui-flip-button--custom-class",
        ".ui-flip-button[data-custom-class=\"true\"]",
        ".ui-flip-button[data-motion-source=\"custom\"]",
        ".ui-flip-button--custom-motion",
        ".ui-flip-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "flip button styles should include `{needle}` for stable source-marker selectors."
        );
    }
}

#[test]
fn docs_actions_page_locks_flip_button_motion_narrative() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "description=\"baseline-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip-button motion/docs stability."
        );
    }
}

#[test]
fn button_flip_docs_top_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "<Playground title=\"Top flip\" code_signal=code>",
        "from=FlipDirection::Top",
        "<Button variant=ButtonVariant::Secondary>\"Front\"</Button>",
        "<Button variant=ButtonVariant::Accent>\"Back\"</Button>",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs top playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_direction_matrix_and_custom_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "from=FlipDirection::Bottom",
        "<Button variant=ButtonVariant::Secondary>\"Bottom\"</Button>",
        "from=FlipDirection::Left",
        "<Button variant=ButtonVariant::Secondary>\"Left\"</Button>",
        "from=FlipDirection::Right",
        "<Button variant=ButtonVariant::Secondary>\"Right\"</Button>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "class_name=\"docs-flip-button-custom\".to_string()",
        "<Button variant=ButtonVariant::Outline>\"Inspect\"</Button>",
        "<Button variant=ButtonVariant::Accent>\"Inspecting\"</Button>",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs matrix/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "Top flip",
        "Direction matrix",
        "Custom Class",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Top flip\" code_signal=code>",
        "from=FlipDirection::Top",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_button_contracts_are_testable_and_assertable() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let compat_semantics = load_source("tests/button_flip/semantics.rs");

    for needle in [
        "flip_button_machine_readable_state_contract_is_typed_and_marker_driven",
        "flip_button_emits_baseline_style_data_attributes",
        "flip_button_semantics_cover_pointer_focus_and_platform_paths",
        "ui_headless_feature_contract_keeps_web_and_ssr_mutually_exclusive",
        "ui_motion_and_flip_button_provide_non_wasm_safe_stub_path",
        "flip_button_public_api_does_not_leak_platform_private_types",
    ] {
        assert!(
            flip_semantics.contains(needle),
            "flip semantics suite should include contract assertion `{needle}`."
        );
    }

    for needle in [
        "button_flip_module_reexports_flip_button_contracts",
        "button_flip_view_wires_motion_and_source_markers",
        "button_flip_styles_include_source_marker_selectors",
        "button_flip_motion_contract_defaults_and_sanitize_paths_are_locked",
        "button_flip_docs_page_covers_primary_playgrounds",
    ] {
        assert!(
            compat_semantics.contains(needle),
            "compat semantics suite should include contract assertion `{needle}`."
        );
    }
}

#[test]
fn flip_button_exposes_required_semantic_markers() {
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "data-slot=\"flip-button\"",
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
        "data-from=move || state.get().direction_attr",
        "data-state=move || state.get().state_attr",
        "data-hover=move || state.get().hover_attr",
        "data-focus-within-state=move || state.get().focus_within_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-ui-agent-schema=move || agent_contract.get().schema_name",
        "data-ui-agent-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-capability-press=move || {",
        "data-ui-capability-focus=move || {",
        "data-ui-capability-hover=move || {",
        "data-ui-capability-popup-trigger=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button should expose required semantic marker `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn flip_button_invalid_states_are_constrained_or_normalized() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub enum FlipDirection",
        "pub from: Option<FlipDirection>",
        "let direction = input.from.unwrap_or_default();",
        ".motion",
        ".map(motion::sanitize_motion)",
        ".unwrap_or_default();",
        "super::super::logic::normalize_optional_text(input.class_name);",
        "let core = resolve_state_core(FlipButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "flip button should normalize invalid/unset state through typed + centralized logic `{needle}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "logic::resolve_state(FlipButtonStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button view should consume normalized state pipeline `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "crate::button::motion::sanitize_motion(",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            motion_source.contains(needle),
            "flip button motion should clamp invalid custom contract values `{needle}`."
        );
    }

    for forbidden in [
        "from: Option<String>",
        "from: Option<bool>",
        "mode: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "flip button should not keep weak invalid-state channel `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_is_explainable_for_humans_and_agents() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "description=\"baseline-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "Top flip",
        "Direction matrix",
        "Custom Class",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button should stay human-explainable in docs with `{needle}`."
        );
    }

    for needle in [
        "data-ui-agent-schema=move || agent_contract.get().schema_name",
        "data-ui-agent-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-capability-press=move || {",
        "data-ui-capability-focus=move || {",
        "data-ui-capability-hover=move || {",
        "data-ui-capability-popup-trigger=move || {",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button should stay machine-explainable via semantic marker `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "flip button should keep explicit explainable logic boundary `{needle}`."
        );
    }
}

#[test]
fn flip_button_changes_stay_in_correct_layers() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/button_flip.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub struct FlipButtonStateCoreInput",
        "pub struct FlipButtonStateCore",
        "pub fn resolve_state_core(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive layer should own reusable state core `{needle}`."
        );
    }

    for needle in [
        "use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};",
        "super::super::logic::normalize_optional_text(input.class_name);",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic layer should compose primitive/button shared contracts `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "A11yDirection, FocusWithinOptions, HoverOptions, locale_attrs, use_focus_within, use_hover,",
        "let normalized = logic::normalize_input(FlipButtonInputNormalizationInput {",
        "logic::resolve_state(FlipButtonStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "view layer should mount headless semantics and consume logic output `{needle}`."
        );
    }

    for needle in [
        "crate::button::motion::ButtonMotion::default().spring",
        "crate::button::motion::sanitize_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion layer should reuse shared button motion capability `{needle}`."
        );
    }

    for forbidden in ["leptos", "web_sys", "set_property("] {
        assert!(
            !primitive_source.contains(forbidden),
            "state primitive should not leak UI/runtime details `{forbidden}`."
        );
    }
}

#[test]
fn flip_button_maintains_consistent_naming_and_implementation_patterns() {
    let logic_source = load_source("src/button/flip/logic.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let motion_source = load_source("src/button/flip/motion.rs");
    let mod_source = load_source("src/button/flip/mod.rs");

    for needle in [
        "pub enum FlipDirection",
        "pub struct FlipButtonMotion",
        "pub struct FlipButtonInputNormalizationInput",
        "pub struct FlipButtonInputNormalization",
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_agent_contract(",
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "pub fn attach_motion(",
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
    ] {
        assert!(
            logic_source.contains(needle)
                || view_source.contains(needle)
                || motion_source.contains(needle)
                || mod_source.contains(needle),
            "flip button should preserve consistent naming/pattern contract `{needle}`."
        );
    }

    for needle in [
        "super::super::logic::normalize_optional_text(input.class_name);",
        "super::super::logic::resolve_agent_contract_for_state_axis(",
        "crate::button::motion::sanitize_motion(",
    ] {
        assert!(
            logic_source.contains(needle) || motion_source.contains(needle),
            "flip button should reuse shared button conventions instead of diverging `{needle}`.",
        );
    }
}

#[test]
fn flip_button_naming_stays_consistent_with_library_conventions() {
    let mod_source = load_source("src/button/flip/mod.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
        "pub struct FlipButtonInputNormalizationInput",
        "pub struct FlipButtonInputNormalization",
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            mod_source.contains(needle) || logic_source.contains(needle),
            "flip button should keep library-consistent symbol naming `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button public/view naming should stay consistent `{needle}`."
        );
    }

    for needle in [
        "from=FlipDirection::Top",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs naming should stay aligned with component API `{needle}`."
        );
    }

    for forbidden in ["className=", "customClass=", "flipFrom=", "direction="] {
        assert!(
            !docs_source.contains(forbidden),
            "flip button API should avoid alias drift naming `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_has_no_half_controlled_state_axes() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let flip_docs_section = docs_source
        .split("pub(super) fn flip_button() -> AnyView")
        .nth(1)
        .and_then(|tail| tail.split("\npub(super) fn ").next())
        .unwrap_or("");

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "let hover = use_hover(HoverOptions { is_disabled: false });",
        "let focus_within = use_focus_within(FocusWithinOptions { is_disabled: false });",
        "is_hovered: hover.is_hovered.get(),",
        "is_focus_within: focus_within.is_focus_within.get(),",
        "let core = resolve_state_core(FlipButtonStateCoreInput {",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "flip button should keep derived internal interaction state path `{needle}`.",
        );
    }

    for forbidden in [
        "default_open",
        "on_open_change",
        "open: Option<",
        "default_value",
        "on_value_change",
        "value: Option<",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !flip_docs_section.contains(forbidden),
            "flip button should not expose half-controlled state API token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_behavior_contract_is_covered_by_state_and_interaction_assertions() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "flip_button_direction_axis_is_type_constrained",
        "flip_button_semantics_cover_pointer_focus_and_platform_paths",
        "flip_button_motion_sanitizes_custom_contract_values",
        "flip_button_view_avoids_hidden_state_machine_decisions",
        "flip_button_emits_baseline_style_data_attributes",
    ] {
        assert!(
            flip_semantics.contains(needle),
            "behavior contract should be covered by targeted semantics assertion `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_state(FlipButtonStateInput {",
        "motion::attach_motion(node_ref, is_active, direction, motion)",
        "on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())",
        "on:focusin=move |_| focus_within.handlers.on_focus_in.run(())",
        "on:focusout=move |_| focus_within.handlers.on_focus_out.run(())",
        "data-state=move || state.get().state_attr",
        "data-hover=move || state.get().hover_attr",
        "data-focus-within-state=move || state.get().focus_within_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "behavior path should stay observable and wired in view contract `{needle}`."
        );
    }
}

#[test]
fn flip_button_architecture_boundary_is_preserved() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let button_flip_semantics = load_source("tests/button_flip/semantics.rs");

    for needle in [
        "flip_button_changes_stay_in_correct_layers",
        "flip_button_state_primitive_stays_dom_and_style_free",
        "flip_button_headless_primitives_stay_visual_and_motion_free",
        "flip_button_view_avoids_hidden_state_machine_decisions",
        "flip_button_delegates_reusable_state_core_to_state_primitives",
        "flip_button_reuses_button_capabilities_to_avoid_cross_component_drift",
        "flip_button_public_api_does_not_leak_platform_private_types",
    ] {
        assert!(
            flip_semantics.contains(needle) || button_flip_semantics.contains(needle),
            "architecture boundary should be guarded by contract test `{needle}`."
        );
    }
}

#[test]
fn flip_button_docs_and_examples_stay_in_sync_with_component_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "<Playground title=\"Top flip\" code_signal=code>",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "from=FlipDirection::Top",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs/examples should stay synchronized with flip-button contract `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "pub enum FlipDirection",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "docs-API examples should match component contract symbol `{needle}`."
        );
    }
}

#[test]
fn flip_button_follows_token_first_static_style_contract() {
    let styles_source = load_source("src/button/flip/styles.rs");
    let css_source = load_source("src/css.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "#[cfg(feature = \"component-button_flip\")]",
        "out.push_str(crate::button::flip::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "flip button css should be aggregated via feature-gated static style entry `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-flip-progress",
        "var(--ui-flip-front-offset",
        "var(--ui-flip-back-offset",
    ] {
        assert!(
            styles_source.contains(needle),
            "flip button should keep token/custom-property-first static style contract `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("style="),
        "flip button view should avoid inline style logic and rely on static css contracts.",
    );
}

#[test]
fn flip_button_semantics_checks_prioritize_contract_assertions_over_snapshots() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let compat_semantics = load_source("tests/button_flip/semantics.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let styles_source = load_source("src/button/flip/styles.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    for needle in [
        "flip_button_emits_baseline_style_data_attributes",
        "flip_button_styles_include_state_marker_contracts",
        "flip_button_semantics_cover_pointer_focus_and_platform_paths",
        "flip_button_behavior_contract_is_covered_by_state_and_interaction_assertions",
        "data-state=move || state.get().state_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            flip_semantics.contains(needle) || compat_semantics.contains(needle),
            "semantics suite should assert contract marker/behavior `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot", "insta::"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "component contract should not rely on snapshot mechanism token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_docs_app_syncs_examples_parameter_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "<Playground title=\"Top flip\" code_signal=code>",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "from=FlipDirection::Top",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app flip button page should keep parameter/state matrix contract `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "pub enum FlipDirection",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "docs parameter naming should stay synchronized with component API symbol `{needle}`."
        );
    }
}

#[test]
fn flip_button_docs_app_provides_interactive_playground() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "code_signal=interactive_code",
        "let direction_options = vec![",
        "let persisted_workbench_state = load_flip_button_workbench_state();",
        "signal(Some(initial_workbench_state.direction_index));",
        "<SegmentedControl",
        "selected_index=interactive_direction_index",
        "set_selected_index=set_interactive_direction_index",
        "data-slot=\"flip-button-workbench-controls\"",
        "data-slot=\"flip-button-workbench\"",
        "data-slot=\"flip-button-workbench-canvas\"",
        "Switch checked=workbench_persist_state set_checked=set_workbench_persist_state",
        "\"Persist workbench state\"",
        "from=FlipDirection::Top",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should provide interactive playground capability `{needle}`."
        );
    }
}

#[test]
fn flip_button_docs_are_beginner_friendly_with_simple_default_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "description=\"baseline-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "<Playground title=\"Top flip\" code_signal=code>",
        "<FlipButton",
        "from=FlipDirection::Top",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should keep beginner-friendly default path `{needle}`."
        );
    }

    for forbidden in ["ui_state_primitives", "ui_headless::", "state=..."] {
        assert!(
            !docs_source.contains(forbidden),
            "flip button docs should not require beginner users to wire internal layers `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_dx_paradox_keeps_simple_default_path_and_optional_advanced_paths() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/flip/view.rs");

    let default_snippet = "<FlipButton\n  from=FlipDirection::Top\n  front=move || view! { <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }\n  back=move || view! { <Button variant=ButtonVariant::Accent>\"Back\"</Button> }\n/>";
    assert!(
        docs_source.contains(default_snippet),
        "flip button docs should keep direct copy-paste default path snippet.",
    );
    assert_eq!(
        default_snippet.lines().count(),
        5,
        "flip button default path should remain within 5 lines for beginner DX."
    );

    for needle in [
        "<Playground title=\"Top flip\" code_signal=code>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "title=\"Interactive Playground\"",
        "code_signal=interactive_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should separate default and advanced paths `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional, into)] from: Option<FlipDirection>,",
        "#[prop(optional, into)] motion: Option<FlipButtonMotion>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(into)] front: ViewFn,",
        "#[prop(into)] back: ViewFn,",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button public API should keep simple defaults and opt-in advanced props `{needle}`.",
        );
    }

    for forbidden in ["#[prop(into)] state:", "#[prop(into)] machine:"] {
        assert!(
            !view_source.contains(forbidden),
            "flip button should not require internal wiring prop `{forbidden}` for baseline usage.",
        );
    }

    for forbidden in ["ui_state_primitives::", "ui_headless::", "state=..."] {
        assert!(
            !docs_source.contains(forbidden),
            "flip button docs should not require internal wiring token `{forbidden}` for baseline usage.",
        );
    }
}

#[test]
fn flip_button_snapshot_mode_is_default_contract() {
    let view_source = load_source("src/button/flip/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "#[prop(into)] front: ViewFn,",
        "#[prop(into)] back: ViewFn,",
        "<Playground title=\"Top flip\" code_signal=code>",
        "front=move || view! { <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }",
        "back=move || view! { <Button variant=ButtonVariant::Accent>\"Back\"</Button> }",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "flip button should keep snapshot-first full-content rendering contract `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] stream",
        "#[prop(optional, into)] delta",
        "data-stream-state",
        "on_stream",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "flip button should not require streaming-only contract token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_streaming_is_optional_with_snapshot_fallback() {
    let view_source = load_source("src/button/flip/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "data-slot=\"flip-button-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
        "<Playground title=\"Top flip\" code_signal=code>",
        "front=move || view! { <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }",
        "back=move || view! { <Button variant=ButtonVariant::Accent>\"Back\"</Button> }",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should keep explicit streaming fallback contract `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] stream",
        "#[prop(optional, into)] delta",
        "data-stream-state",
        "on_stream",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "flip button should keep streaming optional and avoid stream-only surface `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_docs_define_streaming_and_snapshot_modes() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "data-slot=\"flip-button-streaming-modes\"",
        "Streaming: render while the LLM is still generating. Snapshot: render once output is complete.",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should define llm render mode contract `{needle}`.",
        );
    }
}

#[test]
fn flip_button_source_first_docs_are_copy_paste_ready() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"flip-button-copy-ready-hint\"",
        "dependency: ui; source: crates/ui/src/button/flip/view.rs.",
        "<Playground title=\"Top flip\" code_signal=code>",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "title=\"Interactive Playground\"",
        "code_signal=interactive_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button docs should expose copy-ready/source-first contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should auto-compose copy-ready code with imports `{needle}`.",
        );
    }

    for needle in [
        "pub fn CodeBlock(",
        "<Button",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
        "on_press=copy_logic.copy",
    ] {
        assert!(
            code_block_source.contains(needle),
            "copy entry should reuse button capability through code-block copy action `{needle}`.",
        );
    }
}

#[test]
fn flip_button_dx_workbench_reuses_button_persist_pattern_and_isolated_canvas() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "const FLIP_BUTTON_WORKBENCH_STORAGE_KEY: &str = \"docs:flip-button:workbench:state\";",
        "load_flip_button_workbench_state()",
        "save_flip_button_workbench_state(FlipButtonWorkbenchState {",
        "clear_flip_button_workbench_state();",
        "Effect::new(move || {",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "data-slot=\"flip-button-workbench-controls\"",
        "data-slot=\"flip-button-workbench\"",
        "data-slot=\"flip-button-workbench-canvas\"",
        "Switch checked=workbench_persist_state set_checked=set_workbench_persist_state",
    ] {
        assert!(
            docs_source.contains(needle),
            "flip button dx workbench contract should include `{needle}`."
        );
    }
}

#[test]
fn flip_button_heroui_alignment_docs_are_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let actions_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "### FlipButton 同步记录（2026-02-17）",
        "`FlipButton` 作为 `Button` 扩展能力，参数收敛为 `from`、`motion`、`class_name` 与显式 `front/back` 槽位；默认路径无需接线底层状态对象。",
        "`component_doc!(\"FlipButton\", \"flip-button\", \"Actions\", actions::flip_button)`",
        "Streaming Optional; fallback=snapshot.",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui alignment strategy should include flip button sync contract `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(\"FlipButton\", \"flip-button\", \"Actions\", actions::flip_button)",
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
    ] {
        assert!(
            pages_source.contains(needle) || actions_source.contains(needle),
            "flip button docs entry should stay discoverable and synchronized `{needle}`.",
        );
    }
}

#[test]
fn flip_button_implementation_covers_reduced_motion_ssr_and_wasm_paths() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_web = load_source("../../crates/ui-motion/src/web.rs");
    let flip_motion = load_source("src/button/flip/motion.rs");

    for needle in [
        "pub fn prefers_reduced_motion() -> bool",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            ui_motion_web.contains(needle),
            "ui-motion web backend should include reduced-motion downgrade `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should expose explicit wasm/non-wasm branch `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            flip_motion.contains(needle),
            "flip motion should keep wasm enhancement and non-wasm safe fallback `{needle}`."
        );
    }
}

#[test]
fn flip_button_wasm_debug_contract_reuses_button_debug_and_keeps_feature_isolated() {
    let flip_view_source = load_source("src/button/flip/view.rs");
    let flip_logic_source = load_source("src/button/flip/logic.rs");
    let button_view_source = load_source("src/button/view.rs");
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let docs_actions_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "default = [\"inject-css\", \"all-components\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui feature contract should keep `{needle}`."
        );
    }

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button wasm debug contract should keep `{needle}` for source/time/before/after and replay."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs visual debug entry should keep `{needle}`."
        );
    }

    for needle in [
        "front=move || view! { <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }",
        "back=move || view! { <Button variant=ButtonVariant::Accent>\"Back\"</Button> }",
    ] {
        assert!(
            docs_actions_source.contains(needle),
            "flip button docs should keep button-composed front/back path `{needle}`."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "record_transition(",
        "render_debug_panel(",
        "data-debug-source",
    ] {
        assert!(
            !flip_view_source.contains(forbidden),
            "FlipButton view should not duplicate button debug runtime token `{forbidden}`."
        );
        assert!(
            !flip_logic_source.contains(forbidden),
            "FlipButton logic should not duplicate button debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn flip_button_branch_coverage_reduced_motion_ssr_and_wasm_is_locked() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let compat_semantics = load_source("tests/button_flip/semantics.rs");
    let ui_motion_web = load_source("../../crates/ui-motion/src/web.rs");
    let flip_motion = load_source("src/button/flip/motion.rs");

    for needle in [
        "ui_motion_and_flip_button_provide_non_wasm_safe_stub_path",
        "ui_headless_feature_contract_keeps_web_and_ssr_mutually_exclusive",
        "flip_button_semantics_cover_pointer_focus_and_platform_paths",
        "flip_button_implementation_covers_reduced_motion_ssr_and_wasm_paths",
    ] {
        assert!(
            flip_semantics.contains(needle) || compat_semantics.contains(needle),
            "branch coverage evidence should include test `{needle}`."
        );
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool",
        "if prefers_reduced_motion() {",
    ] {
        assert!(
            ui_motion_web.contains(needle),
            "reduced-motion guard should stay present in ui-motion web backend `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            flip_motion.contains(needle),
            "flip motion should keep explicit wasm/non-wasm branch behavior `{needle}`."
        );
    }
}

#[test]
fn flip_button_default_accessibility_contract_is_usable() {
    let flip_semantics = load_source("tests/flip_button/semantics.rs");
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "flip_button_uses_headless_hover_and_focus_within_hooks",
        "flip_button_mounts_ui_headless_contract_in_view_boundary",
        "flip_button_semantics_cover_pointer_focus_and_platform_paths",
        "flip_button_emits_baseline_style_data_attributes",
    ] {
        assert!(
            flip_semantics.contains(needle),
            "a11y baseline should be covered by semantics assertion `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "A11yDirection, FocusWithinOptions, HoverOptions, locale_attrs, use_focus_within, use_hover,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let locale = locale_attrs(super::super::logic::normalize_optional_text(lang), dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "on:focusin=move |_| focus_within.handlers.on_focus_in.run(())",
        "on:focusout=move |_| focus_within.handlers.on_focus_out.run(())",
        "data-ui-capability-focus=move || {",
        "data-focus-within-state=move || state.get().focus_within_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button should expose default focus accessibility contract `{needle}`."
        );
    }

    for forbidden in ["\"Front\"", "\"Back\""] {
        assert!(
            !view_source.contains(forbidden),
            "flip button view should not hardcode user-visible text `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_view_macro_complexity_is_controlled() {
    let view_source = load_source("src/button/flip/view.rs");
    let line_count = view_source.lines().count();
    let view_macro_count = view_source.matches("view! {").count();

    assert!(
        line_count <= 140,
        "flip button view.rs should stay compact; expected <= 140 lines, got {line_count}.",
    );
    assert_eq!(
        view_macro_count, 1,
        "flip button should keep a single primary `view!` block to avoid macro-overgrown layout."
    );

    for forbidden in [
        "match ",
        "for ",
        "while ",
        "<Playground",
        "SegmentedControl",
        "interactive_direction_index",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "flip button view should avoid oversized branching/layout assembly token `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_prefers_functional_split_over_extra_local_components() {
    let view_source = load_source("src/button/flip/view.rs");
    let logic_source = load_source("src/button/flip/logic.rs");
    let styles_source = load_source("src/button/flip/styles.rs");
    let motion_source = load_source("src/button/flip/motion.rs");

    assert!(
        view_source.contains("#[component]\npub fn FlipButton("),
        "flip button should keep a single public component boundary."
    );
    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "flip button should avoid extra local `#[component]` wrappers for simple view fragments."
    );

    let forbidden = "#[component]";
    assert!(
        !logic_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !motion_source.contains(forbidden),
        "non-view layers should not introduce component-level rendering boundary `{forbidden}`.",
    );
}

#[test]
fn flip_button_static_fragments_stay_minimal_and_slot_driven() {
    let view_source = load_source("src/button/flip/view.rs");

    for needle in [
        "<div class=\"ui-flip-button__face ui-flip-button__front\" data-slot=\"flip-button-front\">",
        "{front.get_value().run()}",
        "<div class=\"ui-flip-button__face ui-flip-button__back\" data-slot=\"flip-button-back\">",
        "{back.get_value().run()}",
    ] {
        assert!(
            view_source.contains(needle),
            "flip button view should keep static shell minimal and project slot content via `{needle}`.",
        );
    }

    assert_eq!(
        view_source
            .matches("data-slot=\"flip-button-front\"")
            .count(),
        1,
        "flip button front static shell should be declared once."
    );
    assert_eq!(
        view_source
            .matches("data-slot=\"flip-button-back\"")
            .count(),
        1,
        "flip button back static shell should be declared once."
    );

    for forbidden in [
        "<svg",
        "<path",
        "<footer",
        "Streaming Optional; fallback=snapshot.",
        "Copy-ready snippets prepend imports automatically",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "flip button component view should avoid embedding complex/long static fragment `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_button_contract.spec.mjs");

    for needle in [
        "docs-app flip-button keeps stable semantic selectors and settled contract states",
        "await page.goto(\"/#/components/flip-button\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "const root = page.locator('[data-slot=\"flip-button\"]').first();",
        "const playgrounds = root.locator(\"section.playground\");",
        "const topFlip = playgrounds.nth(0).locator('[data-slot=\"flip-button\"]').first();",
        "const topButton = topFlip.locator('[data-slot=\"button\"]').first();",
        "await expect(topFlip).toHaveAttribute(\"data-from\", \"top\");",
        "await expect(topFlip).toHaveAttribute(\"data-state\", /(active|inactive)/);",
        "await expect(topFlip).toHaveAttribute(\"data-hover\", /(hovered|not-hovered)/);",
        "await expect(topFlip).toHaveAttribute(",
        "\"data-focus-within-state\",",
    ] {
        assert!(
            e2e_source.contains(needle),
            "flip button e2e selector/waiting contract should include `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "flip button e2e should rely on semantic settled waits, not timer API `{forbidden}`.",
        );
    }
}

#[test]
fn flip_button_e2e_key_flow_is_repeatable_with_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_flip_button_contract.spec.mjs");

    for needle in [
        "docs-app flip-button key interaction flow is repeatable with semantic breakpoints",
        "const topFlip = root",
        ".locator(\"section.playground\")",
        ".locator('[data-slot=\"flip-button\"]')",
        "const topButton = topFlip.locator('[data-slot=\"button\"]').first();",
        "await topFlip.hover();",
        "await topButton.focus();",
        "await topButton.evaluate((el) => el.blur());",
        "await page.locator(\"body\").hover();",
        "await expect(topFlip).toHaveAttribute(\"data-state\", \"active\");",
        "await expect(topFlip).toHaveAttribute(\"data-state\", \"inactive\");",
        "await expect(topFlip).toHaveAttribute(\"data-hover\", \"hovered\");",
        "await expect(topFlip).toHaveAttribute(\"data-hover\", \"not-hovered\");",
        "await expect(topFlip).toHaveAttribute(\"data-focus-within-state\", \"focus-within\");",
        "await expect(topFlip).toHaveAttribute(\"data-focus-within-state\", \"no-focus-within\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "flip button e2e key-flow contract should include semantic breakpoint `{needle}`.",
        );
    }
}
