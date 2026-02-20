use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn disclosure_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Disclosure internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn disclosure_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Disclosure should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn disclosure_uses_headless_hooks() {
    let source = load_source("src/view.rs");

    for needle in [
        "use_button",
        "use_focus_ring",
        "use_hover",
        "disclosure_trigger_attrs(",
    ] {
        assert!(
            source.contains(needle),
            "Disclosure should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn disclosure_attaches_motion_drivers() {
    let source = load_source("src/view.rs");

    for needle in ["attach_indicator_motion", "attach_panel_motion"] {
        assert!(
            source.contains(needle),
            "Disclosure should attach `{needle}` for baseline-style spring motion."
        );
    }
}

#[test]
fn disclosure_emits_baseline_style_data_attributes() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"disclosure\"",
        "data-slot=\"disclosure-trigger\"",
        "data-slot=\"disclosure-label\"",
        "data-slot=\"disclosure-indicator\"",
        "data-slot=\"disclosure-panel\"",
        "data-slot=\"disclosure-panel-surface\"",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-open-control-mode=open_control_mode",
        "data-open-controlled=is_open_controlled.then_some(\"true\")",
        "data-open-uncontrolled=(!is_open_controlled).then_some(\"true\")",
        "data-default-open-source=default_open_source",
        "data-open=move || if open.get() { Some(\"true\") } else { None }",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-hovered",
        "data-pressed",
        "data-disabled=disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Disclosure should set `{attr}` to support baseline-style styling and regression testing."
        );
    }
}

#[test]
fn disclosure_uses_logic_state_model() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/disclosure.rs");

    for needle in [
        "pub use ui_state_primitives::disclosure::{DisclosureState, DisclosureStateInput};",
        "pub fn resolve_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Disclosure logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "pub struct DisclosureStateInput",
        "pub struct DisclosureState",
        "pub fn resolve_state(input: DisclosureStateInput) -> DisclosureState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Disclosure primitive should include `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        view_source.contains("logic::resolve_state(open.get(), disabled)"),
        "Disclosure view should derive root state through resolve_state."
    );
}

#[test]
fn disclosure_ids_and_aria_contract_are_wired() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "pub struct DisclosureIds",
        "trigger_id: format!(\"{id_base}-trigger\")",
        "panel_id: format!(\"{id_base}-panel\")",
    ] {
        assert!(
            logic_source.contains(needle),
            "Disclosure logic should define `{needle}` for stable id generation."
        );
    }

    for needle in [
        "aria-expanded",
        "aria-controls",
        "role=\"region\"",
        "aria-labelledby=trigger_id",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Disclosure should wire `{needle}` for accessible disclosure semantics."
        );
    }
}

#[test]
fn disclosure_styles_include_motion_marker_contracts() {
    let source = load_source("src/styles.rs");

    for selector in [
        ".ui-disclosure[data-motion-source=\"custom\"]",
        ".ui-disclosure[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Disclosure styles should include `{selector}` as stable custom-motion selectors."
        );
    }

    for token_var in [
        "var(--ui-font-size-200, 14px)",
        "var(--ui-font-weight-semibold, 600)",
    ] {
        assert!(
            source.contains(token_var),
            "Disclosure styles should stay token-first via `{token_var}`."
        );
    }
}

#[test]
fn disclosure_styles_define_motion_css_vars() {
    let source = load_source("src/styles.rs");

    for var in [
        "--ui-disclosure-indicator-rotation",
        "--ui-disclosure-panel-height",
        "--ui-disclosure-panel-opacity",
        "--ui-disclosure-panel-y",
    ] {
        assert!(
            source.contains(var),
            "Disclosure styles should define `{var}` so motion can update without re-rendering."
        );
    }
}

#[test]
fn disclosure_motion_is_spring_driven() {
    let source = load_source("src/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Disclosure motion should use SpringAnimator to match the motion spec."
    );
}

#[test]
fn disclosure_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DisclosureMotion) -> DisclosureMotion",
        "fn sanitize_spring(value: SpringConfig) -> SpringConfig",
        "closed_rotation_deg:",
        "open_rotation_deg:",
        "panel_offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "Disclosure motion should include `{needle}` so invalid custom values cannot leak into runtime animation state.",
        );
    }
}

#[test]
fn disclosure_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn disclosure() -> AnyView",
        "title=\"Disclosure\"",
        "slug=\"disclosure\"",
        "description=\"Single disclosure panel with baseline-level spring motion and baseline-style root state attrs.\"",
        "<Playground title=\"Controlled\" code_signal=code>",
        "<Playground title=\"Disabled\" code_signal=states_code>",
        "<Playground\n                title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "<Disclosure",
        "on_open_change=on_open_change",
        "default_open=false",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for disclosure coverage.",
        );
    }
}

#[test]
fn disclosure_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let (open, set_open) = signal(true);",
        "let on_open_change = Callback::new(move |next: bool| set_open.set(next));",
        "id_base=\"docs-disclosure\".to_string()",
        "label=\"Details\".to_string()",
        "\"Hidden content\"",
        "\"Uses the same open-state contract as overlays.\"",
        "\"open: \"",
        "id_base=\"docs-disclosure-disabled\".to_string()",
        "label=\"Disabled details\".to_string()",
        "\"Disabled content\"",
        "\"Disabled disclosure keeps trigger non-interactive.\"",
        "\"disabled: true\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=disclosure_test_css_source",
        "test_config_signal=disclosure_actual_config",
        "controls=move || view! {",
        "DisclosureActualConfig",
        "\"Configured Disclosure\"",
        "\"Reference Disclosure\"",
    ] {
        assert!(
            source.contains(needle),
            "disclosure docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn disclosure_readme_documents_display_config_code_css_test_sections() {
    let source = load_source("src/README.md");

    for needle in [
        "## Playground 展示区（展示 / config / code / css test）",
        "Workbench (Display + Config + Code + CSS Test)",
        "Config：Workbench test 面板输出 `DisclosureActualConfig`",
        "## 对比场景",
        "## Source-first",
    ] {
        assert!(
            source.contains(needle),
            "disclosure README should include `{needle}`.",
        );
    }
}

#[test]
fn disclosure_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`Disclosure` 无远程请求与异步状态轴",
        "Streaming Optional",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "disclosure/check2.md should pin completion marker `{needle}`."
        );
    }
}

#[test]
fn disclosure_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Disclosure check2.md should not keep unchecked checklist items after completion."
    );
}
