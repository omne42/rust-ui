use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_resizable_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/resizable/test").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_ui_motion_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("../ui-motion/src/test").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn resizable_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/resizable/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Resizable internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn resizable_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/resizable/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Resizable;"),
        "resizable module should export `Resizable`."
    );
    assert!(
        module_source.contains("pub use logic::ResizableOrientation;"),
        "resizable module should export `ResizableOrientation`."
    );
    assert!(
        crate_source.contains("pub use resizable::{Resizable, ResizableOrientation};"),
        "crate root should re-export Resizable contracts."
    );
}

#[test]
fn resizable_does_not_add_unnecessary_spec_rs_surface() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/resizable/spec.rs");
    let mod_source = load_source("src/resizable/mod.rs");

    assert!(
        !spec_path.exists(),
        "Resizable is a simple component; `src/resizable/spec.rs` must not exist."
    );
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Resizable module must not expose spec surface `{forbidden}`."
        );
    }
}

#[test]
fn resizable_uses_logic_state_model() {
    let logic_source = load_source("src/resizable/logic.rs");
    let logic_checks_source = load_resizable_test_source("logic.rs");
    let logic_combined = format!("{logic_source}\n{logic_checks_source}");
    let view_source = load_source("src/resizable/view.rs");
    let headless_source = load_source("../ui-headless/src/resizable.rs");
    let primitive_source = load_source("../ui-state-primitives/src/resizable.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for needle in [
        "pub use ui_state_primitives::resizable::{",
        "ResizableOrientation",
        "ResizableState",
        "ResizableStateInput",
        "normalize_bounds",
        "normalize_split",
        "ResizableValueAxisInput",
        "ResizableDisabledInput",
        "ResizableHandleInput",
        "normalize_value_axis(",
        "normalize_disabled(",
        "normalize_handle(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_combined.contains(needle),
            "Resizable logic should include `{needle}` via primitive composition."
        );
    }

    for needle in [
        "pub fn use_resizable(options: ResizableOptions) -> ResizableAria",
        "split_from_drag(",
        "split_step_for_key(",
        "pub struct ResizableHandleAttrs",
        "pub struct ResizableHandlers",
        "pub struct ResizableContractState",
    ] {
        assert!(
            headless_source.contains(needle),
            "Resizable interaction/a11y contract should include `{needle}` in ui-headless."
        );
    }

    for needle in [
        "pub enum ResizableOrientation",
        "pub struct SplitBounds",
        "pub struct ResizableStateInput",
        "pub struct ResizableState",
        "pub fn normalize_bounds(",
        "pub fn normalize_split(",
        "pub fn split_from_drag(",
        "pub fn split_step_for_key(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Resizable primitives should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod resizable;"),
        "ui-state-primitives crate should export resizable primitive module."
    );

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::resizable::DEFAULT_ARIA_LABEL;",
        "pub const DEFAULT_SPLIT_PERCENT: f64 = ui_state_primitives::resizable::DEFAULT_SPLIT_PERCENT;",
        "pub const DEFAULT_MIN_SPLIT_PERCENT: f64 =",
        "ui_state_primitives::resizable::DEFAULT_MIN_SPLIT_PERCENT;",
        "pub const DEFAULT_MAX_SPLIT_PERCENT: f64 =",
        "ui_state_primitives::resizable::DEFAULT_MAX_SPLIT_PERCENT;",
    ] {
        assert!(
            load_source("src/resizable/mod.rs").contains(needle),
            "Resizable module constants should be sourced from ui-state-primitives; missing `{needle}`."
        );
    }

    for needle in [
        "headless::use_controllable_state(",
        "headless::use_resizable(ResizableOptions {",
        "logic::normalize_value_axis(logic::ResizableValueAxisInput {",
        "logic::normalize_disabled(logic::ResizableDisabledInput {",
        "logic::normalize_handle(logic::ResizableHandleInput {",
        "logic::compose_class_name(class_name.get_value(), resizable_aria.state.resolved.get())",
        "resizable_aria.handlers.on_pointer_move.run((",
        "on_handle_key_down",
        "event.shift_key()",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable view should derive behavior via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn resizable_supports_controlled_and_uncontrolled_split_state() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "value: Option<Signal<f64>>",
        "default_value: Option<f64>",
        "on_value_change: Option<Callback<f64>>",
        "split_percent: Option<Signal<f64>>",
        "default_split_percent: Option<f64>",
        "on_split_percent_change: Option<Callback<f64>>",
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_with_handle: Option<bool>",
        "with_handle: bool",
        "logic::normalize_value_axis(logic::ResizableValueAxisInput {",
        "logic::normalize_disabled(logic::ResizableDisabledInput {",
        "logic::normalize_handle(logic::ResizableHandleInput {",
        "headless::use_controllable_state(",
        "is_controlled = value_axis.value.is_some()",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should support `{needle}` for controllable split state."
        );
    }
}

#[test]
fn resizable_wires_pointer_drag_and_keyboard_contracts() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "on:pointermove=move |event: ev::PointerEvent|",
        "on:pointerup=move |_| resizable_aria.handlers.on_pointer_up.run(())",
        "on:pointerleave=move |_| resizable_aria.handlers.on_pointer_up.run(())",
        "on:pointerdown=move |event: ev::PointerEvent|",
        "on:keydown=move |event: ev::KeyboardEvent|",
        "role=resizable_aria.handle_attrs.role",
        "aria-valuemin=move || resizable_aria.handle_attrs.aria_valuemin.get()",
        "aria-valuemax=move || resizable_aria.handle_attrs.aria_valuemax.get()",
        "aria-valuenow=move || resizable_aria.handle_attrs.aria_valuenow.get()",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should wire `{needle}` for drag + keyboard + separator semantics."
        );
    }
}

#[test]
fn resizable_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "data-slot=\"resizable\"",
        "data-orientation=move || resizable_aria.state.resolved.get().orientation_attr",
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-disabled=move || resizable_aria.state.resolved.get().disabled.then_some(\"true\")",
        "data-enabled=move || resizable_aria.state.resolved.get().enabled.then_some(\"true\")",
        "data-dragging=move || resizable_aria.state.resolved.get().dragging.then_some(\"true\")",
        "data-controlled=move || resizable_aria.state.resolved.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || resizable_aria.state.resolved.get().is_uncontrolled.then_some(\"true\")",
        "data-handle=move || resizable_aria.state.resolved.get().handle_attr",
        "data-class-source=move || resizable_aria.state.resolved.get().class_source_attr",
        "data-custom-class=move || resizable_aria.state.resolved.get().has_custom_class_name.then_some(\"true\")",
        "data-control-mode=value_axis.control_mode_attr",
        "data-value-source=value_axis.value_source_attr",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-disabled-source=disabled_state.disabled_source_attr",
        "data-handle-source=handle_state.with_handle_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should expose `{needle}` for stable styling/test contracts."
        );
    }
}

#[test]
fn resizable_styles_include_orientation_and_handle_markers() {
    let source = load_source("src/resizable/styles.rs");

    for needle in [
        ".ui-resizable {",
        "--ui-resizable-panel-duration: var(--ui-text-field-motion-duration, 180ms);",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg-muted);",
        ".ui-resizable[data-orientation=\"vertical\"]",
        ".ui-resizable__panel--first",
        "transition: flex-basis var(--ui-resizable-runtime-panel-duration, var(--ui-resizable-panel-duration))",
        ".ui-resizable__handle",
        ".ui-resizable__handle::after",
        ".ui-resizable[data-state=\"dragging\"] .ui-resizable__handle",
        ".ui-resizable--disabled",
        ".ui-resizable--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "Resizable styles should include `{needle}` marker contracts."
        );
    }
}

#[test]
fn resizable_styles_are_state_marker_driven_and_inline_style_is_css_var_only() {
    let styles_source = load_source("src/resizable/styles.rs");
    let view_source = load_source("src/resizable/view.rs");
    let motion_source = load_source("src/resizable/motion.rs");

    for needle in [
        ".ui-resizable[data-orientation=\"vertical\"]",
        ".ui-resizable[data-state=\"dragging\"] .ui-resizable__handle",
        ".ui-resizable[data-disabled=\"true\"]",
        ".ui-resizable__handle[data-dragging=\"true\"]",
        ".ui-resizable__handle[data-disabled=\"true\"]",
        ".ui-resizable[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Resizable styles should drive state via semantic marker `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "Resizable styles must not infer state from brittle DOM structure selector `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Resizable view should keep runtime style assignment centralized."
    );
    for needle in [
        "--ui-resizable-panel-duration:",
        "--ui-resizable-handle-duration:",
        "--ui-resizable-motion-easing:",
    ] {
        assert!(
            motion_source.contains(needle),
            "Resizable runtime style vars must stay CSS-variable-only; missing `{needle}`."
        );
    }

    for forbidden in [
        "background:",
        "border:",
        "display:",
        "position:",
        "padding:",
        "margin:",
    ] {
        assert!(
            !motion_source.contains(&format!("{} ", forbidden)),
            "Motion runtime vars must not carry business style token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("src/resizable/styles.rs");
    let view_source = load_source("src/resizable/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let resizable_module_source = load_source("src/resizable/mod.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "border: 1px solid var(--ui-border);",
        "border-radius: var(--ui-radius-md);",
        "background: var(--ui-bg-muted);",
        "color: var(--ui-fg);",
        "gap: var(--ui-space-3xs, 2px);",
        "background: var(--ui-accent);",
        "color: var(--ui-accent-fg);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Resizable styles token contract should include `{needle}`."
        );
    }

    for forbidden in [
        "rgb(",
        "hsl(",
        "#fff",
        "#000",
        ".bg-",
        ".text-",
        ".rounded-",
        ".shadow-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Resizable styles should avoid utility-first/raw color token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Resizable runtime style should only flow through the centralized motion CSS var string."
    );
    for forbidden in [
        "style=move || format!(",
        "style=\"background:",
        "style=\"color:",
        "style=\"border:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Resizable view must not inject business inline styles; found `{forbidden}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-resizable\")]")
            && css_source.contains("out.push_str(crate::resizable::styles::CSS);"),
        "Component CSS should be aggregated from styles.rs behind component-resizable feature gate."
    );
    assert!(
        root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated component CSS via push_components_css."
    );

    for forbidden in ["stylex", "styled(", "css!("] {
        assert!(
            !resizable_module_source.contains(forbidden),
            "Resizable surface should not adopt css-in-rust default token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn resizable() -> AnyView",
        "title=\"Resizable\"",
        "slug=\"resizable\"",
        "<Resizable",
    ] {
        assert!(
            docs.contains(needle),
            "Resizable docs page should contain `{needle}`."
        );
    }
}

#[test]
fn resizable_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn resizable() -> AnyView",
        "title=\"Resizable\"",
        "slug=\"resizable\"",
        "description=\"baseline-compatible panel splitter with controlled/uncontrolled split state, pointer + keyboard resize semantics, and baseline-style state data contracts.\"",
        "<Playground title=\"Horizontal + Handle Grip\" code_signal=horizontal_code>",
        "<Playground title=\"Controlled + Vertical Bounds\" code_signal=vertical_code>",
        "<Resizable",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs should include `{needle}` for resizable primary playground coverage.",
        );
    }
}

#[test]
fn resizable_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Horizontal + Handle Grip\"",
        "orientation=ResizableOrientation::Horizontal",
        "default_value=36.0",
        "is_with_handle=true",
        "\"Sidebar\"",
        "\"Content\"",
        "title=\"Controlled + Vertical Bounds\"",
        "orientation=ResizableOrientation::Vertical",
        "value=split",
        "on_value_change=on_split_change",
        "min_split_percent=25.0",
        "max_split_percent=80.0",
        "is_with_handle=true",
        "aria_label=\"Deployment regions split\".to_string()",
        "class_name=\"docs-resizable-custom\".to_string()",
        "\"Header\"",
        "\"Body\"",
        "controlled split:",
        "format!(\"{:.1}%\", split_raw.get())",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for resizable contracts.",
        );
    }
}

#[test]
fn resizable_semantic_checks_are_contract_first_and_snapshot_secondary() {
    let check2_source = load_source("src/resizable/check2.md");
    let suite_source = load_source("tests/resizable_semantics.rs");
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Semantic-test-first checklist should include `{needle}`."
        );
    }

    for needle in [
        "tests/resizable_semantics.rs",
        "fn resizable_wires_pointer_drag_and_keyboard_contracts()",
        "fn resizable_emits_baseline_root_state_data_attributes()",
        "fn resizable_supports_controlled_and_uncontrolled_split_state()",
        "fn resizable_agent_contract_markers_are_typed_and_snapshot_based()",
        "fn resizable_streaming_optional_contract_is_explicit_and_render_only()",
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "role=resizable_aria.handle_attrs.role",
        "aria-valuenow=move || resizable_aria.handle_attrs.aria_valuenow.get()",
        "on:keydown=move |event: ev::KeyboardEvent|",
    ] {
        assert!(
            suite_source.contains(needle) || view_source.contains(needle),
            "Semantic-test-first contract should include `{needle}`."
        );
    }

    for forbidden in [
        concat!("ins", "ta::assert_snapshot"),
        concat!("snap", "box::"),
        concat!("toMatch", "Snapshot("),
        concat!("pixel", "match("),
    ] {
        assert!(
            !suite_source.contains(forbidden),
            "Semantic suite must not regress to snapshot-first assertion token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_snapshot_is_base_capability_for_complete_configs() {
    let check2_source = load_source("src/resizable/check2.md");
    let view_source = load_source("src/resizable/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let logic_source = load_source("src/resizable/logic.rs");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Snapshot baseline checklist should include `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: ResizableOrientation",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] default_split_percent: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] on_split_percent_change: Option<Callback<f64>>",
        "#[prop(into)] first: ViewFn",
        "#[prop(into)] second: ViewFn",
        "{render_panel(",
        "\"resizable-panel-first\"",
        "\"resizable-panel-second\"",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Snapshot baseline render contract should include `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Horizontal + Handle Grip\" code_signal=horizontal_code>",
        "<Playground title=\"Controlled + Vertical Bounds\" code_signal=vertical_code>",
        "default_value=40.0",
        "value=split",
        "on_value_change=on_split_change",
        "first=move || view! { <div>\\\"Left\\\"</div> }",
        "second=move || view! { <div>\\\"Right\\\"</div> }",
    ] {
        assert!(
            docs_source.contains(needle),
            "Docs should demonstrate complete snapshot-ready config path marker `{needle}`."
        );
    }

    for needle in [
        "stream_fallback_attr: ResizableStreamFallback::Snapshot.as_attr(),",
        "stream_mode_attr: ResizableStreamMode::Snapshot.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Snapshot baseline mode should be typed and explicit via `{needle}`."
        );
    }
}

#[test]
fn resizable_streaming_optional_contract_is_explicit_and_render_only() {
    let check2_source = load_source("src/resizable/check2.md");
    let logic_source = load_source("src/resizable/logic.rs");
    let view_source = load_source("src/resizable/view.rs");
    let motion_source = load_source("src/resizable/motion.rs");
    let styles_source = load_source("src/resizable/styles.rs");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Streaming optional checklist should include `{needle}`."
        );
    }

    for needle in [
        "pub enum ResizableStreamSupport",
        "Unsupported",
        "pub enum ResizableStreamFallback",
        "Snapshot",
        "stream_support_attr: ResizableStreamSupport::Unsupported.as_attr(),",
        "stream_fallback_attr: ResizableStreamFallback::Snapshot.as_attr(),",
        "output_status_attr: output_status.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Streaming optional typed contract should include `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "role=resizable_aria.handle_attrs.role",
        "aria-label=move || resizable_aria.handle_attrs.aria_label.clone()",
        "aria-orientation=move || resizable_aria.handle_attrs.aria_orientation.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Streaming optional render contract should keep readable marker `{needle}`."
        );
    }

    for source in [&logic_source, &view_source, &motion_source, &styles_source] {
        for forbidden in [
            "retry",
            "reconnect",
            "backoff",
            "text/event-stream",
            "EventSource",
            "WebSocket",
            "ReadableStream",
            "aria-busy",
            "is_loading",
        ] {
            assert!(
                !source.contains(forbidden),
                "Streaming optional component should stay render-only and not own validation/retry protocol token `{forbidden}`."
            );
        }
    }
}

#[test]
fn resizable_streaming_definition_is_llm_output_modes_only() {
    let check2_source = load_source("src/resizable/check2.md");
    let logic_source = load_source("src/resizable/logic.rs");
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Streaming definition checklist should include `{needle}`."
        );
    }

    for needle in [
        "pub enum ResizableStreamSupport",
        "Unsupported",
        "pub enum ResizableStreamFallback",
        "Snapshot",
        "pub enum ResizableStreamMode",
        "stream_support_attr: ResizableStreamSupport::Unsupported.as_attr(),",
        "stream_fallback_attr: ResizableStreamFallback::Snapshot.as_attr(),",
        "stream_mode_attr: ResizableStreamMode::Snapshot.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Streaming/Snapshot mode contract should include typed marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "View should expose stream mode marker `{needle}` for machine-readable consumption."
        );
    }

    for forbidden in [
        "EventSource",
        "ReadableStream",
        "WebSocket",
        "ws://",
        "wss://",
        "text/event-stream",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Resizable should only declare LLM output mode metadata and must not implement transport token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_agent_contract_markers_are_typed_and_snapshot_based() {
    let logic_source = load_source("src/resizable/logic.rs");
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "pub enum ResizableAgentSchema",
        "pub enum ResizableAgentIntent",
        "pub enum ResizableAgentActionModel",
        "pub enum ResizableStreamSupport",
        "pub enum ResizableStreamFallback",
        "pub enum ResizableStreamMode",
        "pub enum ResizableOutputStatus",
        "pub struct ResizableAgentContract",
        "pub fn resolve_agent_contract(",
        "ui.resizable.agent-contract.v1",
        "adjust-split",
        "pointer+keyboard",
        "unsupported",
        "snapshot",
        "schema_attr: ResizableAgentSchema::V1.as_attr(),",
        "intent_attr: ResizableAgentIntent::AdjustSplit.as_attr(),",
        "action_model_attr: ResizableAgentActionModel::PointerKeyboard.as_attr(),",
        "stream_support_attr: ResizableStreamSupport::Unsupported.as_attr(),",
        "stream_fallback_attr: ResizableStreamFallback::Snapshot.as_attr(),",
        "stream_mode_attr: ResizableStreamMode::Snapshot.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Resizable logic should define typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = logic::resolve_agent_contract(value_axis.value_change_source);",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable view should mount agent contract marker `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "data-ui-intent=\"",
        "data-ui-action-model=\"",
        "data-ui-state-axis=\"",
        "data-ui-source-axis=\"",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Resizable render path must remain whitelist-only; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("src/resizable/logic.rs");
    let logic_checks_source = load_resizable_test_source("logic.rs");
    let logic_combined = format!("{logic_source}\n{logic_checks_source}");
    let view_source = load_source("src/resizable/view.rs");
    let suite_source = load_source("tests/resizable_semantics.rs");

    for needle in [
        "pub enum ResizableControlMode",
        "pub enum ResizableValueSource",
        "pub enum ResizableDefaultValueSource",
        "pub enum ResizableValueChangeSource",
        "pub enum ResizableDisabledSource",
        "pub enum ResizableHandleSource",
        "pub enum ResizableAgentSchema",
        "pub enum ResizableAgentIntent",
        "pub enum ResizableAgentActionModel",
        "pub enum ResizableStreamSupport",
        "pub enum ResizableStreamFallback",
        "pub enum ResizableStreamMode",
        "pub enum ResizableOutputStatus",
        "pub fn normalize_value_axis(input: ResizableValueAxisInput)",
        "pub fn normalize_disabled(input: ResizableDisabledInput)",
        "pub fn normalize_handle(input: ResizableHandleInput)",
        "normalize_split(",
        "normalize_bounds(",
    ] {
        assert!(
            logic_combined.contains(needle),
            "Type-constrained state contract should include `{needle}` in logic.rs."
        );
    }

    for forbidden in [
        "value_source_attr: String",
        "default_value_source_attr: String",
        "value_change_source_attr: String",
        "disabled_source_attr: String",
        "with_handle_source_attr: String",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "State/source attrs should be closed-set static markers, found stringly contract `{forbidden}`."
        );
    }

    for needle in [
        "data-control-mode=value_axis.control_mode_attr",
        "data-value-source=value_axis.value_source_attr",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-disabled-source=disabled_state.disabled_source_attr",
        "data-handle-source=handle_state.with_handle_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "View machine-readable contract should expose `{needle}`."
        );
    }

    assert!(
        suite_source.contains(
            "fn resizable_type_system_and_semantic_markers_form_machine_readable_contract()"
        ),
        "Semantics suite should expose a dedicated contract test entrypoint for fast failure localization."
    );
}

#[test]
fn resizable_docs_page_includes_api_state_and_source_first_sections() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "data-slot=\"resizable-api-matrix\"",
        "data-slot=\"resizable-api-rows\"",
        "data-slot=\"resizable-state-matrix\"",
        "data-slot=\"resizable-state-rows\"",
        "data-slot=\"resizable-source-first\"",
        "data-slot=\"resizable-source-paths\"",
        "data-slot=\"resizable-source-prerequisites\"",
        "Copy starter",
        "component-resizable",
        "crates/ui-layout/src/resizable/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "Resizable docs should include `{needle}` for docs/state/source matrix sync."
        );
    }
}

#[test]
fn resizable_readme_is_beginner_friendly_and_copy_paste_ready() {
    let source = load_source("src/resizable/README.md");

    for needle in [
        "# Resizable",
        "## Hello World",
        "## Controlled Split",
        "orientation=ResizableOrientation::Horizontal",
        "value=split",
        "on_value_change=Callback::new",
        "first=move || view! { <div>\"Sidebar\"</div> }",
        "second=move || view! { <div>\"Content\"</div> }",
    ] {
        assert!(
            source.contains(needle),
            "Resizable README should include beginner-first marker `{needle}`."
        );
    }
}

#[test]
fn resizable_e2e_selectors_are_semantic_and_waits_are_stable() {
    let source = load_source("../../e2e/tests/docs_app_resizable_contract.spec.mjs");

    for needle in [
        "page.goto(\"/#/components/resizable\")",
        "body:not(:has(#boot))",
        "[data-component=\"resizable\"]",
        "[data-slot=\"resizable\"]",
        "[data-slot=\"resizable\"][data-control-mode=\"controlled\"]",
        "data-ui-schema",
        "data-ui-stream-support",
        "ArrowDown",
        "aria-valuenow",
        "data-state",
        "data-idle",
        "data-dragging",
        "data-slot=\"resizable-handle\"",
    ] {
        assert!(
            source.contains(needle),
            "Resizable e2e contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout",
        "nth-child",
        "locator(\"text=",
        ".filter({ hasText:",
        "section.playground",
    ] {
        assert!(
            !source.contains(forbidden),
            "Resizable e2e should avoid brittle selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_component_files_keep_single_responsibility_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/resizable/mod.rs");
    let logic_source = load_source("src/resizable/logic.rs");
    let styles_source = load_source("src/resizable/styles.rs");
    let view_source = load_source("src/resizable/view.rs");
    let motion_source = load_source("src/resizable/motion.rs");

    for required in [
        "src/resizable/mod.rs",
        "src/resizable/logic.rs",
        "src/resizable/styles.rs",
        "src/resizable/view.rs",
        "src/resizable/motion.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "Resizable component file layout should include `{required}`."
        );
    }
    for forbidden in ["src/resizable/render.rs", "src/resizable/spec.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "Resizable component file layout should not include `{forbidden}`."
        );
    }

    assert!(
        mod_source.contains("mod logic;")
            && mod_source.contains("mod motion;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;"),
        "Resizable module should keep standard files wired."
    );
    for needle in [
        "pub use logic::ResizableOrientation;",
        "pub use motion::ResizableMotion;",
        "pub use view::Resizable;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable export `{needle}`."
        );
    }
    assert!(
        !mod_source.contains("spec.rs") && !mod_source.contains("pub mod spec"),
        "Resizable should not introduce unnecessary spec.rs exports."
    );
    for forbidden in [
        "use leptos::",
        "pub mod logic",
        "pub mod view",
        "#[component]",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should stay as a minimal export boundary; found `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("normalize_value_axis(")
            && logic_source.contains("normalize_disabled(")
            && logic_source.contains("normalize_handle("),
        "Resizable logic should stay focused on normalization and source markers."
    );
    for forbidden in [
        "NodeRef<",
        "web_sys",
        "style=",
        "view!",
        ".ui-resizable",
        "on:pointer",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs must not include DOM/style/render bindings; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "Resizable styles must stay token-first via CSS variables."
    );
    for forbidden in [
        "leptos::",
        "Signal<",
        "Callback<",
        "web_sys",
        "#[component]",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should stay static CSS-only; found `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("headless::use_resizable(ResizableOptions {"),
        "Resizable view should only mount headless contract outputs."
    );
    for forbidden in [
        "split_from_drag(",
        "split_step_for_key(",
        "resolve_state(ResizableStateInput",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs must not reimplement interaction/state primitives; found `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("pub fn attach_motion(")
            && motion_source.contains("default_text_field_motion_tokens()"),
        "Resizable motion should only map semantics to shared motion runtime."
    );
    for forbidden in [
        "request_animation_frame",
        "set_interval",
        "set_timeout",
        "spring::",
        "MotionKeyframe",
        "WAAPI",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not re-implement generic motion engines; found `{forbidden}`."
        );
    }
}

#[test]
fn resizable_view_macro_complexity_is_semantically_split() {
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "fn render_panel(",
        "fn render_handle_grip() -> impl IntoView",
        "{render_panel(",
        "\"resizable-panel-first\"",
        "\"resizable-panel-second\"",
        "{render_handle_grip()}",
        "#[component]",
        "pub fn Resizable(",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable view macro split contract should include `{needle}`."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "Resizable view should keep a single component entrypoint to avoid unnecessary abstraction noise."
    );

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        (3..=4).contains(&view_macro_count),
        "Resizable view macro complexity guard expected 3-4 semantic view blocks, found {view_macro_count}.",
    );
}

#[test]
fn resizable_prefers_functional_subviews_over_extra_components() {
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "fn render_panel(",
        "fn render_handle_grip() -> impl IntoView",
        "{render_panel(",
        "{render_handle_grip()}",
        "data-slot=slot",
        "\"resizable-panel-first\"",
        "\"resizable-panel-second\"",
        "data-slot=\"resizable-handle-grip\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Functional split contract should include `{needle}` in resizable view."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "Resizable should avoid local sub-component noise; only one #[component] entrypoint is allowed.",
    );
}

#[test]
fn resizable_ui_layout_entrypoint_files_follow_boundary_contract() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state_source =
        load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-resizable\")]",
        "pub mod resizable;",
        "pub use resizable::ResizableMotion;",
        "pub use resizable::{Resizable, ResizableOrientation};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout/lib.rs entrypoint contract should include `{needle}`."
        );
    }
    for forbidden in ["pub use web_sys::", "pub use leptos::web_sys::"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-layout/lib.rs should not expose platform-detail type `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-resizable\")]",
        "out.push_str(crate::resizable::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout/css.rs aggregation contract should include `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "semantic_overrides",
        "data-slot=\"ui-root\"",
    ] {
        assert!(
            root_source.contains(needle),
            "ui-layout/root.rs contract should include `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringConfig",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "ui-layout/active_highlight.rs contract should include `{needle}`."
        );
    }
    for forbidden in [
        "data-slot=\"resizable\"",
        "on_open_change",
        "aria_controls_when_open",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business semantic `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-layout/src/overlay_open.rs should not exist."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-layout/src/presence.rs should not exist."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-layout/src/a11y.rs should not exist."
    );

    assert!(
        headless_controllable_state_source.contains("pub fn use_controllable_state"),
        "open-state primitive should remain in ui-headless controllable_state."
    );
    assert!(
        headless_presence_source.contains("pub fn use_presence"),
        "presence primitive should remain in ui-headless presence."
    );
    assert!(
        headless_a11y_source.contains("pub fn aria_controls_when_open"),
        "shared a11y utility should remain in ui-headless/a11y.rs."
    );
}

#[test]
fn resizable_platform_tree_shaking_and_cross_layer_guards_hold() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let cargo_source = load_source("Cargo.toml");
    let motion_source = load_source("src/resizable/motion.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let primitive_source = load_source("../ui-state-primitives/src/resizable.rs");
    let headless_source = load_source("../ui-headless/src/resizable.rs");

    for needle in [
        "#[cfg(feature = \"component-resizable\")]",
        "pub mod resizable;",
        "pub use resizable::{Resizable, ResizableOrientation};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib export contract should include `{needle}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-resizable\")]")
            && css_source.contains("out.push_str(crate::resizable::styles::CSS);"),
        "ui-layout css aggregation should stay feature-gated for resizable."
    );
    assert!(
        cargo_source.contains("component-resizable = []"),
        "ui-layout Cargo features should expose component-resizable."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Resizable motion platform split should include `{needle}`."
        );
    }

    assert!(
        headless_lib_source
            .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless web/ssr mutual exclusion compile_error should stay in place."
    );
    assert!(
        ui_motion_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "ui-motion non-wasm stub path should remain available."
    );

    for forbidden in ["web_sys", "leptos::", "view!", "data-slot", "style="] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives resizable must stay pure; found forbidden token `{forbidden}`."
        );
    }
    for forbidden in [".ui-", "var(--ui-", "transition:", "animation:"] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless resizable must not include styling orchestration token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_ssr_cross_platform_compile_contract_is_explicit() {
    let motion_source = load_source("src/resizable/motion.rs");
    let view_source = load_source("src/resizable/view.rs");
    let logic_source = load_source("src/resizable/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/resizable.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = node.unchecked_into();",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Cross-platform motion contract should include `{needle}`."
        );
    }

    assert!(
        ui_motion_source.contains("#[cfg(not(target_arch = \"wasm32\"))]")
            && ui_motion_source.contains("pub fn prefers_reduced_motion() -> bool")
            && ui_motion_source.contains("pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}"),
        "ui-motion must provide predictable non-wasm stubs for SSR/tooling builds."
    );

    assert!(
        headless_lib_source
            .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should keep explicit web/ssr compile-time guard."
    );

    for forbidden in ["web_sys", "window.", "document."] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs must stay platform-agnostic; found `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "view.rs must not directly bind browser globals; found `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives must remain pure Rust; found `{forbidden}`."
        );
    }
}

#[test]
fn resizable_motion_non_wasm_noop_stub_contract_is_predictable() {
    let motion_source = load_source("src/resizable/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_checks_source = load_ui_motion_test_source("lib.rs");
    let ui_motion_combined = format!("{ui_motion_source}\n{ui_motion_checks_source}");
    let suite_source = load_source("tests/resizable_semantics.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_is_dragging: leptos::prelude::Signal<bool>",
        "sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Resizable motion non-wasm no-op branch should include `{needle}`."
        );
    }

    for forbidden in ["panic!(", "todo!(", "unimplemented!(", "unwrap()"] {
        assert!(
            !motion_source.contains(forbidden),
            "Resizable motion non-wasm branch should avoid unstable fallback `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_combined.contains(needle),
            "ui-motion should keep stable non-wasm no-op stub `{needle}`."
        );
    }

    assert!(
        suite_source.contains("fn resizable_motion_non_wasm_noop_stub_contract_is_predictable()"),
        "Semantics suite should expose dedicated non-wasm motion contract test for direct failure localization."
    );
}

#[test]
fn resizable_reduced_motion_ssr_wasm_branch_contract_is_consistent() {
    let motion_source = load_source("src/resizable/motion.rs");
    let view_source = load_source("src/resizable/view.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let suite_source = load_source("tests/resizable_semantics.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
        "drop(style.set_property(\"--ui-resizable-runtime-panel-duration\", \"1ms\"));",
        "drop(style.set_property(\"--ui-resizable-runtime-handle-duration\", \"1ms\"));",
        "drop(style.remove_property(\"--ui-resizable-runtime-panel-duration\"));",
        "drop(style.remove_property(\"--ui-resizable-runtime-handle-duration\"));",
        "sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Reduced-motion/SSR/wasm motion branch contract should include `{needle}`."
        );
    }

    assert!(
        ui_motion_source.contains("pub fn prefers_reduced_motion() -> bool {")
            && ui_motion_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "ui-motion should expose reduced-motion behavior and non-wasm stubs for SSR/tooling."
    );

    for needle in [
        "motion::attach_motion(root_ref, resizable_aria.state.is_dragging.into(), motion);",
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "View semantic contract should stay platform-independent and include `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "View semantic markup must not split by target-arch cfg; found `{forbidden}`."
        );
    }

    assert!(
        suite_source
            .contains("fn resizable_reduced_motion_ssr_wasm_branch_contract_is_consistent()"),
        "Semantics suite should provide dedicated branch-consistency test for direct failure localization."
    );
}

#[test]
fn resizable_performance_governance_budget_is_repeatable_attributable_and_blocking() {
    let check2_source = load_source("src/resizable/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let view_source = load_source("src/resizable/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "resizable/check2.md should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"resizable\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep budgeted perf probe contract `{needle}`."
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
            "e2e coverage should enforce blocking perf contract marker `{needle}`."
        );
    }

    for marker in [
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-control-mode=value_axis.control_mode_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-disabled-source=disabled_state.disabled_source_attr",
        "data-handle-source=handle_state.with_handle_source_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "resizable view should keep attributable perf marker `{marker}`."
        );
    }

    let derive_count = view_source.matches("Signal::derive(").count();
    assert!(
        derive_count <= 1,
        "resizable reactive budget exceeded: expected <= 1 Signal::derive, found {derive_count}.",
    );
    let effect_count = view_source.matches("Effect::new(").count();
    assert_eq!(
        effect_count, 0,
        "resizable view should keep predictable update budget and avoid Effect::new loops.",
    );

    assert!(
        todo_source.contains("render_count"),
        "performance governance should keep explicit render_count follow-up tracking until framework support lands.",
    );
}

#[test]
fn resizable_headless_web_ssr_mutual_exclusion_guard_is_preserved() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let view_source = load_source("src/resizable/view.rs");
    let headless_resizable_source = load_source("../ui-headless/src/resizable.rs");

    assert!(
        headless_lib_source.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]"),
        "ui-headless should gate simultaneous web+ssr via cfg(all(feature = \"web\", feature = \"ssr\"))."
    );
    assert!(
        headless_lib_source
            .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should emit compile_error when web and ssr are enabled together."
    );

    assert!(
        view_source.contains("headless::use_resizable(ResizableOptions {"),
        "Resizable component must consume ui-headless contracts and inherit its web/ssr guard."
    );
    assert!(
        headless_resizable_source
            .contains("pub fn use_resizable(options: ResizableOptions) -> ResizableAria"),
        "ui-headless resizable contract entrypoint should remain stable."
    );
}

#[test]
fn resizable_tree_shaking_feature_graph_contract_is_explicit_and_minimal() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo_source = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-resizable = []",
        "#[cfg(feature = \"component-resizable\")]",
        "pub mod resizable;",
        "out.push_str(crate::resizable::styles::CSS);",
        "ui-layout = { path = \"../../crates/ui-layout\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle)
                || web_demo_cargo_source.contains(needle),
            "Tree-shaking contract should include `{needle}`."
        );
    }

    assert!(
        !web_demo_cargo_source.contains("\"all-components\""),
        "web-demo should not pull `all-components` implicitly."
    );

    assert!(
        css_source.contains("#[cfg(feature = \"component-resizable\")]")
            && css_source.contains("out.push_str(crate::resizable::styles::CSS);"),
        "Resizable CSS must be feature-gated and only included when component-resizable is enabled."
    );
}

#[test]
fn resizable_visual_baseline_styles_cover_hierarchy_contrast_and_feedback() {
    let styles_source = load_source("src/resizable/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg-muted);",
        "color: var(--ui-fg);",
        ".ui-resizable__handle:focus-visible",
        ".ui-resizable[data-state=\"dragging\"] .ui-resizable__handle",
        "color: var(--ui-accent-fg);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Resizable default theme baseline should include `{needle}`."
        );
    }

    for needle in [
        "title=\"Resizable\"",
        "description=\"baseline-compatible panel splitter",
        "Horizontal + Handle Grip",
        "Controlled + Vertical Bounds",
    ] {
        assert!(
            docs_source.contains(needle),
            "Resizable docs baseline page should include `{needle}`."
        );
    }
}

#[test]
fn resizable_visual_desire_contract_links_theme_baseline_and_screenshot_regression() {
    let theme_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let theme_e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "ButtonVariant::Accent",
        "ButtonVariant::Secondary",
        "ButtonVariant::Ghost",
        "is_clearable=true",
    ] {
        assert!(
            theme_docs_source.contains(needle),
            "Theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "const visualMode = process.env.E2E_VISUAL_BASELINE ?? \"off\";",
        "theme visual baseline renders button/input/overlay",
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "[data-slot=\"theme-visual-baseline-button\"] [data-slot=\"button\"]",
        "[data-slot=\"theme-visual-baseline-input\"] [data-slot=\"input\"]",
        "[data-slot=\"overlay\"][data-state=\"open\"]",
        "theme visual baseline screenshots",
        "toHaveScreenshot(",
        "\"docs-app-theme-visual-baseline-page.png\"",
        "\"docs-app-theme-visual-baseline-button.png\"",
        "\"docs-app-theme-visual-baseline-input.png\"",
        "\"docs-app-theme-visual-baseline-overlay.png\"",
        "{ animations: \"disabled\" }",
    ] {
        assert!(
            theme_e2e_source.contains(needle),
            "Theme visual baseline e2e regression should include `{needle}`."
        );
    }

    for needle in [
        "### Resizable 同步记录（2026-02-18）",
        "HeroUI 对齐结论：保持“默认路径零门槛，复杂控制按需显式开启”",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI alignment strategy should include `{needle}` for resizable."
        );
    }
}

#[test]
fn resizable_static_fragments_are_template_scoped_accessible_and_non_dynamic() {
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "fn render_handle_grip() -> impl IntoView",
        "<span class=\"ui-resizable__handle-grip\" data-slot=\"resizable-handle-grip\">",
        "<span class=\"ui-resizable__handle-dot\"></span>",
        "{render_handle_grip()}",
        "role=resizable_aria.handle_attrs.role",
        "aria-label=move || resizable_aria.handle_attrs.aria_label.clone()",
        "aria-orientation=move || resizable_aria.handle_attrs.aria_orientation.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable static fragment template contract should include `{needle}`."
        );
    }

    let dot_count = view_source
        .matches("<span class=\"ui-resizable__handle-dot\"></span>")
        .count();
    assert_eq!(
        dot_count, 3,
        "Resizable handle grip dots should remain a fixed static fragment (expected 3, got {dot_count})."
    );

    let grip_fn_count = view_source
        .matches("fn render_handle_grip() -> impl IntoView")
        .count();
    assert_eq!(
        grip_fn_count, 1,
        "Static grip template should have a single centralized change path."
    );

    for forbidden in ["inner_html", "format!("] {
        assert!(
            !view_source.contains(forbidden),
            "Static fragment path should avoid dynamic HTML/string assembly token `{forbidden}`."
        );
    }
}

#[test]
fn resizable_inner_html_contract_rejects_dynamic_html_sources() {
    let view_source = load_source("src/resizable/view.rs");
    let logic_source = load_source("src/resizable/logic.rs");
    let motion_source = load_source("src/resizable/motion.rs");
    let styles_source = load_source("src/resizable/styles.rs");

    for source in [&view_source, &logic_source, &motion_source, &styles_source] {
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
        ] {
            assert!(
                !source.contains(forbidden),
                "Resizable inner_html security contract should reject token `{forbidden}`."
            );
        }
    }

    for needle in [
        "role=resizable_aria.handle_attrs.role",
        "aria-label=move || resizable_aria.handle_attrs.aria_label.clone()",
        "aria-orientation=move || resizable_aria.handle_attrs.aria_orientation.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "When no inner_html is used, semantic accessibility markers should still be explicit (`{needle}`)."
        );
    }
}

#[test]
fn resizable_wasm_debug_contract_is_traceable_replayable_and_feature_isolated() {
    let view_source = load_source("src/resizable/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_resizable_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/resizable/mod.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-value-source=value_axis.value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "data-disabled-source=disabled_state.disabled_source_attr",
        "data-handle-source=handle_state.with_handle_source_attr",
        "data-control-mode=value_axis.control_mode_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "WASM debug traceability contract should expose semantic state/source marker `{needle}`."
        );
    }

    for needle in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"resizable\"][data-control-mode=\"controlled\"]",
        "const controlledHandle = controlledRoot.locator('[data-slot=\"resizable-handle\"]').first();",
        "await expect(controlledRoot).toHaveAttribute(\"data-state\", \"idle\");",
        "await expect(controlledRoot).toHaveAttribute(\"data-idle\", \"true\");",
        "await expect(controlledRoot).not.toHaveAttribute(\"data-dragging\", \"true\");",
        "await expect(controlledHandle).toHaveAttribute(\"aria-valuenow\", \"58.00\");",
        "await controlledHandle.focus();",
        "await page.keyboard.press(\"ArrowDown\");",
        "await expect(controlledHandle).toHaveAttribute(\"aria-valuenow\", \"60.00\");",
        "await expect(controlledRoot).toHaveAttribute(\"data-value-change-source\", \"on_value_change\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "WASM replay/debug e2e contract should include `{needle}`."
        );
    }

    let before_idx = e2e_source
        .find("await expect(controlledHandle).toHaveAttribute(\"aria-valuenow\", \"58.00\");")
        .expect("e2e contract should assert pre-interaction baseline value.");
    let press_idx = e2e_source
        .find("await page.keyboard.press(\"ArrowDown\");")
        .expect("e2e contract should include deterministic keyboard trigger.");
    let after_idx = e2e_source
        .find("await expect(controlledHandle).toHaveAttribute(\"aria-valuenow\", \"60.00\");")
        .expect("e2e contract should assert post-interaction value.");
    assert!(
        before_idx < press_idx && press_idx < after_idx,
        "WASM replay contract should preserve before -> event -> after ordering."
    );

    for needle in [
        "title=\"Resizable\"",
        "Playground title=\"Horizontal + Handle Grip\"",
        "Playground title=\"Controlled + Vertical Bounds\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Docs should provide a visible entrypoint for WASM debug and replay (`{needle}`)."
        );
    }

    for source in [&cargo_source, &mod_source, &lib_source] {
        for forbidden in [
            "resizable-wasm-debug",
            "component-resizable-wasm-debug",
            "cfg(feature = \"resizable-wasm-debug\")",
            "pub fn enable_resizable_debug",
            "pub struct ResizableDebug",
        ] {
            assert!(
                !source.contains(forbidden),
                "WASM debug helper must stay feature-isolated and out of public production API (`{forbidden}`)."
            );
        }
    }
}

#[test]
fn resizable_static_fragment_and_wasm_debug_rules_are_explicit_and_safe() {
    let view_source = load_source("src/resizable/view.rs");
    let cargo_source = load_source("Cargo.toml");

    for needle in [
        "<span class=\"ui-resizable__handle-dot\"></span>",
        "data-value-source=value_axis.value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable static/semantic fragment contract should include `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("resizable-wasm-debug"),
        "Resizable debug helpers should not leak dedicated wasm debug feature into production feature surface."
    );
}

#[test]
fn resizable_engineering_contract_is_runtime_agnostic_and_structured() {
    let mod_source = load_source("src/resizable/mod.rs");
    let logic_source = load_source("src/resizable/logic.rs");
    let view_source = load_source("src/resizable/view.rs");
    let motion_source = load_source("src/resizable/motion.rs");
    let styles_source = load_source("src/resizable/styles.rs");
    let check2_source = load_source("src/resizable/check2.md");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &motion_source,
        &styles_source,
    ] {
        for forbidden in [
            "serde::",
            "Serialize",
            "Deserialize",
            "tracing::",
            "#[instrument",
            "tokio::",
            "async_std::",
            "smol::",
            "Runtime",
            "JoinHandle",
            "async fn",
            "impl Future",
        ] {
            assert!(
                !source.contains(forbidden),
                "Engineering contract should stay runtime-agnostic and avoid leaking `{forbidden}` in resizable implementation."
            );
        }
    }

    for needle in [
        "pub use logic::ResizableOrientation;",
        "pub use motion::ResizableMotion;",
        "pub use view::Resizable;",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] on_split_percent_change: Option<Callback<f64>>",
    ] {
        assert!(
            mod_source.contains(needle) || view_source.contains(needle),
            "Engineering contract should keep callback-based API surface marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`Resizable` 无 spec 序列化输入与异步运行时边界；公共 API 未泄露 runtime 细节",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 engineering checklist should keep explicit N/A rationale marker `{needle}`."
        );
    }
}

#[test]
fn resizable_dx_workbench_context_and_fast_style_feedback_contract_is_explicit() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_resizable_contract.spec.mjs");
    let styles_source = load_source("src/resizable/styles.rs");
    let cargo_source = load_source("Cargo.toml");

    for needle in [
        "title=\"Resizable\"",
        "Playground title=\"Horizontal + Handle Grip\"",
        "Playground title=\"Controlled + Vertical Bounds\"",
        "controlled split:",
        "Source-first / Copy-Paste Ready",
    ] {
        assert!(
            docs_source.contains(needle),
            "DX/workbench contract should expose isolated practice surface marker `{needle}`."
        );
    }

    for needle in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"resizable\"][data-control-mode=\"controlled\"]",
        "await controlledHandle.focus();",
        "await page.keyboard.press(\"ArrowDown\");",
        "await expect(controlledHandle).toHaveAttribute(\"aria-valuenow\", \"60.00\");",
        "await expect(controlledRoot).toHaveAttribute(\"data-idle\", \"true\");",
        "await expect(controlledRoot).not.toHaveAttribute(\"data-dragging\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "DX context-preservation contract should include replayable interaction marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout", "setTimeout", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "DX regression should avoid brittle fixed-wait token `{forbidden}`."
        );
    }

    let var_count = styles_source.matches("var(--ui-").count();
    assert!(
        var_count >= 10,
        "DX fast-style-feedback contract expects token-first CSS variables (found {var_count})."
    );

    for forbidden in [
        "resizable-dx-dev",
        "resizable-workbench-debug",
        "component-resizable-devtools",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "DX helpers must stay out of production feature surface (`{forbidden}`)."
        );
    }
}

#[test]
fn resizable_dx_contract_provides_live_playground_and_copy_ready_surface() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_resizable_contract.spec.mjs");

    for needle in [
        "Playground title=\"Horizontal + Handle Grip\"",
        "Playground title=\"Controlled + Vertical Bounds\"",
        "controlled split:",
        "Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "Resizable DX surface should include `{needle}`."
        );
    }

    assert!(
        e2e_source.contains("playground source remains copy-paste ready"),
        "Resizable e2e should lock copy-paste-ready behavior."
    );
}

#[test]
fn resizable_semantics_suite_is_contract_first_not_snapshot_only() {
    let suite_source = load_source("tests/resizable_semantics.rs");

    for needle in [
        "fn resizable_wires_pointer_drag_and_keyboard_contracts()",
        "fn resizable_emits_baseline_root_state_data_attributes()",
        "fn resizable_supports_controlled_and_uncontrolled_split_state()",
        "fn resizable_agent_contract_markers_are_typed_and_snapshot_based()",
        "fn resizable_platform_tree_shaking_and_cross_layer_guards_hold()",
        "role=resizable_aria.handle_attrs.role",
        "aria-valuemin=move || resizable_aria.handle_attrs.aria_valuemin.get()",
        "data-state=move || resizable_aria.state.resolved.get().state_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "on:pointermove=move |event: ev::PointerEvent|",
        "on:keydown=move |event: ev::KeyboardEvent|",
    ] {
        assert!(
            suite_source.contains(needle),
            "Resizable semantics suite should keep contract-first assertion marker `{needle}`."
        );
    }

    for forbidden in [
        concat!("ins", "ta::"),
        concat!("snap", "box::"),
        concat!("toMatch", "Snapshot("),
        concat!("pixel", "match("),
    ] {
        assert!(
            !suite_source.contains(forbidden),
            "Resizable semantics suite must not degrade to snapshot-only verification token `{forbidden}`."
        );
    }
}
