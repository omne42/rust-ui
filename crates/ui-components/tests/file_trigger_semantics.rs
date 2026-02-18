use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn file_trigger_does_not_expose_logic_module() {
    let source = load_source("src/file_trigger/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "FileTrigger's `logic` module should stay private to avoid leaking DOM/web-sys details into the public API."
    );
}

#[test]
fn file_trigger_uses_logic_state_model() {
    let logic_source = load_source("src/file_trigger/logic.rs");
    let view_source = load_source("src/file_trigger/view.rs");

    for needle in [
        "pub struct FileTriggerStateInput",
        "pub struct FileTriggerState",
        "pub fn resolve_state(input: FileTriggerStateInput)",
        "pub fn compose_class_name(state: FileTriggerState)",
        "pub motion_source_attr: &'static str",
        "pub has_custom_motion: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "FileTrigger logic should include `{needle}` for centralized state derivation.",
        );
    }

    for needle in [
        "let state = Signal::derive(move ||",
        "super::logic::resolve_state(super::logic::FileTriggerStateInput {",
        "let class = Signal::derive(move || super::logic::compose_class_name(state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "FileTrigger view should derive wrapper state through logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn file_trigger_clears_input_value_before_click() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("input.set_value(\"\")"),
        "FileTrigger should clear the input value before invoking `click()` so selecting the same file twice still triggers `change`."
    );
}

#[test]
fn file_trigger_forwards_motion_to_button() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("motion=motion.trigger"),
        "FileTrigger should forward its motion contract to the internal Button trigger."
    );
}

#[test]
fn file_trigger_emits_motion_source_markers() {
    let source = load_source("src/file_trigger/view.rs");

    for needle in [
        "data-slot=\"file-trigger\"",
        "data-state=move || state.get().state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "FileTrigger should expose `{needle}` for baseline motion inspection."
        );
    }
}

#[test]
fn file_trigger_input_is_hidden_from_tab_order() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "FileTrigger should set `tabindex=\"-1\"` on the hidden input to avoid it receiving focus."
    );

    assert!(
        source.contains("aria-hidden=\"true\""),
        "FileTrigger should set `aria-hidden=\"true\"` on the hidden input to keep the accessibility tree focused on the trigger."
    );
}

#[test]
fn file_trigger_supports_directory_and_capture_attrs() {
    let source = load_source("src/file_trigger/view.rs");

    assert!(
        source.contains("set_attribute(\"webkitdirectory\""),
        "FileTrigger should support directory selection via the `webkitdirectory` attribute."
    );

    assert!(
        source.contains("set_attribute(\"capture\""),
        "FileTrigger should support media capture via the `capture` attribute."
    );
}

#[test]
fn file_trigger_styles_include_motion_marker_contracts() {
    let source = load_source("src/file_trigger/styles.rs");

    for selector in [
        ".ui-file-trigger--disabled",
        ".ui-file-trigger[data-disabled=\"true\"]",
        ".ui-file-trigger[data-motion-source=\"custom\"]",
        ".ui-file-trigger--custom-motion",
        ".ui-file-trigger[data-custom-motion=\"true\"]",
        ".ui-file-trigger__input",
    ] {
        assert!(
            source.contains(selector),
            "FileTrigger styles should include `{selector}` as stable contracts."
        );
    }
}

#[test]
fn file_trigger_motion_contract_exposes_default_and_custom_trigger_tests() {
    let source = load_source("src/file_trigger/motion.rs");

    for needle in [
        "pub struct FileTriggerMotion",
        "pub trigger: ButtonMotion",
        "fn default_motion_uses_default_button_motion_contract()",
        "fn supports_custom_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "FileTrigger motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn file_trigger_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/file_trigger/motion.rs");
    let view_source = load_source("src/file_trigger/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FileTriggerMotion) -> FileTriggerMotion",
        "trigger: crate::button::motion::sanitize_motion(motion.trigger)",
        "fn sanitize_motion_delegates_to_button_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "FileTrigger motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::file_trigger::motion::sanitize_motion(motion);"),
        "FileTrigger view should sanitize motion before forwarding to Button.",
    );
}

#[test]
fn file_trigger_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "pub(super) fn file_trigger() -> AnyView",
        "title=\"FileTrigger\"",
        "slug=\"file-trigger\"",
        "description=\"A Button that forwards to an invisible <input type=file>.\"",
        "<Playground title=\"Pick files\" code_signal=code>",
        "<Playground title=\"Pick files with custom motion\" code_signal=motion_code>",
        "<FileTrigger",
    ] {
        assert!(
            source.contains(needle),
            "files docs page should include `{needle}` for file_trigger primary playground coverage.",
        );
    }
}

#[test]
fn file_trigger_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "title=\"Pick files\"",
        "<FileTrigger multiple=true on_files=on_files>",
        "\"Pick files\"",
        "\"No files selected.\"",
        "title=\"Pick files with custom motion\"",
        "motion=FileTriggerMotion {",
        "hover_scale: 1.04",
        "tap_scale: 0.94",
        "on_files=on_custom_files",
        "\"Pick files (custom motion)\"",
        "\"No files selected (custom motion example).\"",
    ] {
        assert!(
            source.contains(needle),
            "file_trigger docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn file_trigger_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/file_trigger/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "file_trigger/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn file_trigger_check2_marks_semantics_first_testing_complete() {
    let check2_source = load_source("src/file_trigger/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "file_trigger/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn file_trigger_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/file_trigger/check2.md");

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
            check2_source.contains(needle),
            "file_trigger/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn file_trigger_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/file_trigger/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "file_trigger/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
