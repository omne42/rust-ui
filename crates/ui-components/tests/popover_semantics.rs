use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn popover_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/popover/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Popover internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn popover_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/popover/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Popover;"),
        "popover module should export `Popover`."
    );
    assert!(
        module_source.contains("pub struct PopoverPartStateInput"),
        "popover module should expose `PopoverPartStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use popover::Popover;")
            && crate_source.contains("pub use popover::PopoverMotion;"),
        "crate root should re-export `Popover` and `PopoverMotion` contracts."
    );
}

#[test]
fn popover_logic_exposes_state_helpers() {
    let source = load_source("src/popover/logic.rs");

    for needle in [
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn modal_attr(is_modal: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: PopoverPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: PopoverPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            source.contains(needle),
            "Popover logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn popover_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/popover/view.rs");

    for needle in [
        "default_prevented",
        "is_composing",
        "logic::should_close_on_escape(",
        "stop_propagation()",
    ] {
        assert!(
            source.contains(needle),
            "Popover should include `{needle}` for stable Escape-dismiss behavior."
        );
    }
}

#[test]
fn popover_view_uses_logic_state_contracts() {
    let source = load_source("src/popover/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(PopoverPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-modal=root_state.modal_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-placement=root_state.has_custom_placement.then_some(\"true\")",
        "data-non-modal=(!root_state.is_modal).then_some(\"true\")",
        "data-custom-modal=(!root_state.is_modal).then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=panel_state.slot_attr",
        "data-state=panel_state.state_attr",
        "data-modal=panel_state.modal_attr",
    ] {
        assert!(
            source.contains(needle),
            "Popover view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn popover_styles_include_state_and_source_markers() {
    let source = load_source("src/popover/styles.rs");

    for selector in [
        ".ui-popover[data-motion-source=\"custom\"]",
        ".ui-popover[data-custom-motion=\"true\"]",
        ".ui-popover[data-placement-source=\"custom\"]",
        ".ui-popover--custom-modal",
        ".ui-popover[data-modal-source=\"custom\"]",
        ".ui-popover[data-custom-modal=\"true\"]",
        ".ui-popover[data-modal=\"non-modal\"]",
        ".ui-popover[data-class-source=\"custom\"]",
        ".ui-popover[data-exit-source=\"custom\"]",
        ".ui-popover[data-state=\"open\"]",
        ".ui-popover[data-state=\"closed\"]",
        ".ui-popover__panel[data-state=\"panel\"]",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
        ".ui-popover__panel[data-placement=\"top-end\"]",
    ] {
        assert!(
            source.contains(selector),
            "Popover styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn popover_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::popover::styles::CSS);"),
        "ui-components css aggregator should include popover styles."
    );
}

#[test]
fn popover_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "State + Source Markers",
        "data-modal-source",
        "<Popover",
    ] {
        assert!(
            source.contains(needle),
            "popover docs page should contain `{needle}`."
        );
    }
}

#[test]
fn popover_motion_contract_exposes_default_and_placement_offset_helpers() {
    let mod_source = load_source("src/popover/mod.rs");
    let motion_source = load_source("src/popover/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::PopoverMotion;",
        "pub struct PopoverMotion",
        "fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64",
        "fn default_motion_matches_upstream_style_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Popover motion contract should include `{needle}` for baseline-level spring configuration and directional offsets."
        );
    }
}

#[test]
fn popover_motion_contract_sanitizes_custom_values() {
    let source = load_source("src/popover/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: PopoverMotion) -> PopoverMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset()",
    ] {
        assert!(
            source.contains(needle),
            "Popover motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn popover_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = PopoverMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "title=\"State + Source Markers\"",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect `data-modal-source`/`data-placement-source` while tuning PopoverMotion.",
    ] {
        assert!(
            source.contains(needle),
            "popover docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn popover_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "description=\"Positioned portal panel anchored to a trigger with baseline-style state markers and baseline-level spring motion contract. Requires presence to unmount after exit.\"",
        "<Playground title=\"Popover\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "code_signal=motion_code",
        "<Popover",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for popover primary playground coverage.",
        );
    }
}

#[test]
fn popover_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Popover\"",
        "anchor_ref=anchor_ref",
        "on_close=on_close",
        "on_exit_complete=on_exit_complete",
        "Positioned via anchor rect + CSS vars.",
        "title=\"State + Source Markers\"",
        "node_ref=custom_anchor_ref",
        "on_press=toggle_custom",
        "open=custom_open",
        "anchor_ref=custom_anchor_ref",
        "on_close=close_custom",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"popover-workbench-controls\"",
        "PopoverWorkbenchConfig",
        "class_name=\"docs-popover-workbench\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for popover contracts.",
        );
    }
}

#[test]
fn popover_check2_keeps_popover_scope_and_na_rationale_explicit() {
    let source = load_source("src/popover/check2.md");

    for needle in [
        "已核验（popover，2026-02-18）：本组件是 overlay 容器",
        "不承载远程异步流程与流式正文渲染职责",
        "相关条目按 N/A 语义核验并保持契约可追溯。",
    ] {
        assert!(
            source.contains(needle),
            "popover check2 should keep scoped rationale marker `{needle}`."
        );
    }
}

#[test]
fn popover_check2_marks_semantics_streaming_and_docs_contract_complete() {
    let source = load_source("src/popover/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
    ] {
        assert!(
            source.contains(needle),
            "popover check2 should mark semantics/docs governance item `{needle}` as complete."
        );
    }
}

#[test]
fn popover_check2_marks_final_merge_gates_complete() {
    let source = load_source("src/popover/check2.md");

    for needle in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "popover check2 should keep final merge gate marker `{needle}`."
        );
    }
}

#[test]
fn popover_check2_has_no_remaining_unchecked_items() {
    let source = load_source("src/popover/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "popover/check2.md should not keep unchecked checklist items once governance is complete."
    );
}

#[test]
fn popover_readme_covers_display_config_code_css_test_and_comparisons() {
    let source = load_source("src/popover/README.md");

    for needle in [
        "## Playground 展示区（Display / Config / Code / CSS Test）",
        "## 多场景对比展示",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "popover README should include `{needle}`."
        );
    }
}
