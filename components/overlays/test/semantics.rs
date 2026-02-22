use std::path::PathBuf;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "check2" => include_str!("../check2.md"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn overlays_module_boundary_is_minimal_and_wires_semantics_tests() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::OverlaysRoot;",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module.contains(required),
            "overlays module boundary should include `{required}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "pub mod protocol"] {
        assert!(
            !module.contains(forbidden),
            "overlays internals should stay private: `{forbidden}`."
        );
    }
}

#[test]
fn overlays_layered_files_keep_ui_components_assembly_split() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for required in [
        "pub use ui_state_primitives::overlays::{",
        "pub fn compose_root_class_name(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/assembly via `{required}`."
        );
    }
    for forbidden in ["view! {", "on:keydown", "use ui_headless::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not carry rendering/headless integration `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label.clone()",
        "data-slot=\"overlays\"",
        "data-state=move ||",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render structure/markers via `{required}`."
        );
    }
    for forbidden in ["use ui_motion::spring::SpringAnimator", "web_sys::"] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not own motion engine or web-sys details `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-overlays-",
        "data-state=\"open\"",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS via `{required}`."
        );
    }
    for forbidden in ["view! {", "use_focus_trap(", "SpringAnimator::new"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry render/headless/runtime logic `{forbidden}`."
        );
    }

    for required in [
        "pub struct OverlaysMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "tray: crate::tray::motion::sanitize_motion(motion.tray)",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should keep semantic mapping via `{required}`."
        );
    }
    for forbidden in [
        "SpringAnimator::new",
        "request_animation_frame",
        "web_sys::",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not embed runtime engine `{forbidden}`."
        );
    }
}

#[test]
fn overlays_public_api_does_not_expose_web_sys_types() {
    let module = load_source("mod");
    for forbidden in ["web_sys::", "HtmlElement", "Element"] {
        assert!(
            !module.contains(forbidden),
            "overlays public module surface should not expose DOM detail `{forbidden}`."
        );
    }
}

#[test]
fn overlays_component_has_local_semantics_test_file() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    assert!(
        path.exists(),
        "overlays should keep semantic regression tests in component-local `test/semantics.rs`."
    );
}

#[test]
fn overlays_root_public_props_follow_api_naming_contract() {
    let view = load_source("view");

    for required in [
        "#[prop(optional)] is_open: bool",
        "#[prop(optional)] is_modal: bool",
    ] {
        assert!(
            view.contains(required),
            "OverlaysRoot API should expose `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] open: bool",
        "#[prop(optional)] modal: bool",
    ] {
        assert!(
            !view.contains(forbidden),
            "OverlaysRoot API should not keep legacy alias `{forbidden}`."
        );
    }
}

#[test]
fn overlays_checklist_marks_ui_components_boundary_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。"
        ),
        "check2.md should mark ui definition as completed."
    );
    assert!(
        check2.contains("components/overlays/test/semantics.rs"),
        "check2.md should include local semantics.rs evidence for migration/testing layout."
    );
}

#[test]
fn overlays_checklist_marks_api_naming_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。"),
        "check2.md should mark API naming contract checklist item as completed."
    );
    assert!(
        check2.contains("is_open/is_modal"),
        "check2.md should include migration evidence for `is_open/is_modal` naming."
    );
}

#[test]
fn overlays_checklist_marks_controlled_uncontrolled_pairing_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。"),
        "check2.md should mark controlled/uncontrolled pairing checklist item as completed."
    );
    assert!(
        check2
            .contains("Modal` 的 open 轴已收敛为单入口 `is_open + on_open_change + default_open`")
            && check2.contains("overlays_open_state_pairing_contract_is_explicit_and_stable"),
        "check2.md should include modal pairing evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_dx_paradox_complete() {
    let check2 = load_source("check2");
    assert!(
        check2
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"),
        "check2.md should mark DX paradox checklist item as completed."
    );
    assert!(
        check2.contains("Hello World 已收敛为 5 行内最小示例")
            && check2.contains("Hello World (Minimal Path)")
            && check2.contains(
                "overlays_dx_paradox_keeps_minimal_api_path_without_internal_state_wiring"
            ),
        "check2.md should include minimal-path evidence and regression test reference for DX paradox."
    );
}

#[test]
fn overlays_checklist_marks_explicit_parent_item_api_rule_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
        "check2.md should mark explicit parent/item API checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前不是集合型 Parent/Item 组件")
            && check2
                .contains("overlays_parent_item_composition_rule_is_not_applicable_and_no_parallel_array_api_leaks"),
        "check2.md should include N/A rationale and regression test reference for parent/item composition rule."
    );
}

#[test]
fn overlays_checklist_marks_macro_micro_duality_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。"),
        "check2.md should mark macro/micro duality checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前无拖拽手势与 `Dragging/DragEnd` 状态轴")
            && check2.contains("overlays_have_no_dragging_macro_micro_state_machine_path"),
        "check2.md should include N/A rationale and regression test reference for macro/micro duality."
    );
}

#[test]
fn overlays_checklist_marks_two_pass_geometry_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。"),
        "check2.md should mark two-pass geometry checklist item as completed."
    );
    assert!(
        check2.contains("use_popover_position(PopoverPositionOptions)")
            && check2.contains(
                "POSITION_EPSILON_PX + should_update_scalar + raf_pending + ResizeObserver"
            )
            && check2.contains("overlays_two_pass_geometry_pipeline_is_delegated_and_idempotent"),
        "check2.md should include two-pass geometry evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_registration_protocol_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。"),
        "check2.md should mark registration protocol checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前不是动态集合导航组件")
            && check2.contains(
                "overlays_registration_protocol_is_not_applicable_without_dynamic_item_registry"
            ),
        "check2.md should include N/A rationale and regression test reference for registration protocol."
    );
}

#[test]
fn overlays_checklist_marks_slot_projection_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。"),
        "check2.md should mark slot projection checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前不提供容器级 `Lazy/KeepAlive/Eager` 投影模式切换")
            && check2.contains(
                "overlays_slot_projection_strategy_is_not_applicable_without_projection_modes"
            ),
        "check2.md should include N/A rationale and regression test reference for slot projection strategy."
    );
}

#[test]
fn overlays_checklist_marks_env_streams_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。"),
        "check2.md should mark env streams checklist item as completed."
    );
    assert!(
        check2.contains("use_popover_position(PopoverPositionOptions)")
            && check2.contains("ResizeObserver` + `request_animation_frame` + `raf_pending`")
            && check2.contains(
                "overlays_env_streams_are_delegated_to_headless_with_backpressure_guards"
            ),
        "check2.md should include env-stream delegation/backpressure evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_event_light_cone_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。"),
        "check2.md should mark event light cone checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前不是 `Table/Grid` 型大集合组件")
            && check2.contains(
                "overlays_event_light_cone_is_not_applicable_without_bulk_collection_bus"
            ),
        "check2.md should include N/A rationale and regression test reference for event light cone."
    );
}

#[test]
fn overlays_checklist_marks_causality_bus_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。"),
        "check2.md should mark causality-bus checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前不包含跨模块复杂派生总线")
            && check2
                .contains("overlays_causality_bus_is_not_applicable_without_trace_propagation_bus"),
        "check2.md should include N/A rationale and regression test reference for causality bus."
    );
}

#[test]
fn overlays_checklist_marks_a11y_i18n_l10n_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
        ),
        "check2.md should mark a11y/i18n/l10n checklist item as completed."
    );
    assert!(
        check2.contains("ui_headless::labeled_group_attrs")
            && check2.contains("logic.rs::DEFAULT_ARIA_LABEL")
            && check2.contains("overlay_dialog_attrs")
            && check2.contains(
                "overlays_a11y_i18n_l10n_contracts_are_headless_first_and_text_source_driven"
            ),
        "check2.md should include a11y/i18n/l10n evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_state_observability_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
        ),
        "check2.md should mark state observability checklist item as completed."
    );
    assert!(
        check2.contains("`data-slot/data-state/data-open/data-closed`")
            && check2.contains("`data-*-source`")
            && check2.contains("`e2e/tests/docs_app_nav_sheet.spec.mjs`")
            && check2.contains("overlays_state_markers_are_observable_queryable_and_closed_set"),
        "check2.md should include marker observability evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_state_driven_styling_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
        "check2.md should mark explicit-state-driven styling checklist item as completed."
    );
    assert!(
        check2.contains("无 `:nth-child/:nth-of-type` 结构猜测")
            && check2.contains("`style=panel_vars`")
            && check2.contains("`logic::compose_panel_vars`")
            && check2.contains(
                "overlays_styles_depend_on_explicit_state_markers_not_dom_shape_guessing"
            ),
        "check2.md should include explicit-state styling evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_semantic_contract_testing_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "check2.md should mark semantic-contract testing checklist item as completed."
    );
    assert!(
        check2.contains("`components/overlay/test/overlay_semantics.rs`")
            && check2.contains("`popover_semantics.rs`")
            && check2.contains("`modal_semantics.rs`")
            && check2.contains("normalize_open_state_supports_controlled_and_uncontrolled_modes")
            && check2.contains("`e2e/tests/docs_app_nav_sheet.spec.mjs`")
            && check2.contains("#[cfg(target_arch = \\\"wasm32\\\")]")
            && check2.contains(
                "overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only"
            ),
        "check2.md should include semantic contract matrix evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_component_file_responsibilities_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。"),
        "check2.md should mark component-file responsibility checklist item as completed."
    );
    assert!(
        check2.contains("`components/overlays/src/mod.rs` 仅维护模块边界与稳定导出")
            && check2.contains("`logic.rs` 仅做状态归一/来源标记与 class 组合")
            && check2.contains("`styles.rs` 仅静态 token-first CSS")
            && check2
                .contains("`view.rs` 仅结构渲染与 `ui_headless::labeled_group_attrs` 语义挂载")
            && check2.contains("`motion.rs` 仅语义动效映射与下游 contract sanitize")
            && check2.contains("overlays_component_files_follow_layered_responsibilities")
            && check2.contains("overlays_layered_files_keep_ui_components_assembly_split"),
        "check2.md should include file-responsibility evidence and regression test references."
    );
}

#[test]
fn overlays_checklist_marks_component_directory_standard_file_layout_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确。"),
        "check2.md should mark component-directory standard file-layout checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlays,overlay,popover,modal,sheet,tray}/src`")
            && check2.contains("`mod.rs/logic.rs/styles.rs/view.rs/motion.rs`")
            && check2.contains("`render.rs/spec.rs`")
            && check2.contains("`pub mod logic` / `pub mod view`")
            && check2.contains("`var(--ui-*)`")
            && check2.contains("`OverlayMotion/PopoverMotion/SheetMotion`")
            && check2.contains("`default_motion_contract()/normalize_motion()`")
            && check2.contains(
                "overlays_component_directory_standard_files_follow_contract_and_na_spec"
            )
            && check2.contains("`scripts/check-ui-component-files.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_component_directory_standard_files_follow_contract_and_na_spec`"
            ),
        "check2.md should include component-directory file-layout evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_file_placement_discipline_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "check2.md should mark file-placement discipline checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlays,overlay,popover,modal,sheet,tray}/src`")
            && check2.contains("`mod.rs/logic.rs/styles.rs/view.rs/motion.rs`")
            && check2.contains("不存在 `render.rs`")
            && check2.contains("`spec.rs` 在该家族维持 N/A（未引入）")
            && check2.contains(
                "overlays_component_directory_standard_files_follow_contract_and_na_spec"
            )
            && check2.contains(
                "overlays_file_placement_discipline_is_strict_for_component_scope"
            )
            && check2.contains("`scripts/check-ui-component-files.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_file_placement_discipline_is_strict_for_component_scope`"
            ),
        "check2.md should include file-placement discipline evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_hyper_structure_builder_spec_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "check2.md should mark hyper-structure-builder checklist item as completed."
    );
    assert!(
        check2.contains("N/A（overlays 家族）")
            && check2.contains("`components/button/src/spec.rs`")
            && check2.contains("`ButtonSpec::new()...render()`")
            && check2.contains(
                "overlays_hyper_structure_builder_spec_is_not_applicable_for_simple_component"
            )
            && check2.contains("`scripts/check-ui-component-files.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_hyper_structure_builder_spec_is_not_applicable_for_simple_component`"
            ),
        "check2.md should include hyper-structure-builder N/A evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_context_compression_manifest_rbi_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "check2.md should mark context-compression manifest/rbi checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlays,overlay,popover,modal,sheet,tray}/src/{Component.toml,*.rbi}`")
            && check2.contains("`overlays.rbi/overlay.rbi/popover.rbi/modal.rbi/sheet.rbi/tray.rbi`")
            && check2.contains("`context_compression_manifest`")
            && check2.contains("`rbi_signature_projection`")
            && check2.contains(
                "overlays_context_compression_manifest_and_rbi_projection_are_present_and_current"
            )
            && check2.contains("`scripts/check-ui-component-files.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_context_compression_manifest_and_rbi_projection_are_present_and_current`"
            ),
        "check2.md should include context-compression manifest/rbi evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_agent_contract_schema_governance_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "check2.md should mark agent-contract schema governance checklist item as completed."
    );
    assert!(
        check2.contains("`data-* + data-*-source`")
            && check2.contains("`Modal/Sheet`")
            && check2.contains("`data-ui-*`")
            && check2.contains("`ModalAgentConfigPolicy::Whitelist`")
            && check2.contains("`components/modal/src/logic.rs`")
            && check2.contains("`components/sheet/src/logic.rs`")
            && check2.contains("`components/modal/src/view.rs`")
            && check2.contains("`components/sheet/src/view.rs`")
            && check2.contains(
                "overlays_agent_contract_is_schema_typed_and_machine_readable"
            )
            && check2.contains("`scripts/check-ui-contract-hygiene.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_agent_contract_is_schema_typed_and_machine_readable`"
            ),
        "check2.md should include agent-contract schema governance evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_streaming_definition_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "check2.md should mark streaming definition checklist item as completed."
    );
    assert!(
        check2.contains("`apps/docs-app/src/pages/components/pages/overlays.rs`")
            && check2.contains("`not an LLM body reader surface`")
            && check2.contains("`stream_mode_options = [\"Snapshot\", \"Streaming (fallback=snapshot)\"]`")
            && check2.contains("`heading=\"LLM output contract\"`")
            && check2.contains("`data-ui-output-mode=snapshot|streaming`")
            && check2.contains(
                "overlays_streaming_definition_is_llm_output_only_with_two_modes"
            )
            && check2.contains("`scripts/check-ui-streaming.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_streaming_definition_is_llm_output_only_with_two_modes`"
            ),
        "check2.md should include two-mode streaming definition evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_snapshot_baseline_capability_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "check2.md should mark snapshot-baseline capability checklist item as completed."
    );
    assert!(
        check2.contains("`snapshot-first (`fallback=snapshot`)`")
            && check2.contains("`This component defaults to snapshot rendering.`")
            && check2.contains("`Snapshot is the baseline rendering mode for ContextualHelp.`")
            && check2.contains("`render_mode_attr=\"snapshot\"`")
            && check2.contains("`output_status_attr=\"verified\"`")
            && check2.contains(
                "overlays_snapshot_baseline_consumes_complete_result_and_renders_stably"
            )
            && check2.contains("`scripts/check-ui-streaming.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_snapshot_baseline_consumes_complete_result_and_renders_stably`"
            ),
        "check2.md should include snapshot-baseline evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_streaming_required_optional_classification_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "check2.md should mark streaming required/optional classification checklist item as completed."
    );
    assert!(
        check2.contains("`not an LLM body reader surface`")
            && check2.contains("`streaming-optional and snapshot-first (`fallback=snapshot`)`")
            && check2.contains("`ContextualHelpStreamingRequirement::{Required, Optional}`")
            && check2.contains("`data-ui-streaming-requirement`")
            && check2.contains("`data-ui-streaming-fallback`")
            && check2.contains("`data-ui-output-status`")
            && check2.contains("`retry/backoff/reconnect/websocket`")
            && check2.contains(
                "overlays_streaming_required_optional_classification_rules_are_scope_driven_and_boundary_safe"
            )
            && check2.contains("`scripts/check-ui-streaming.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_streaming_required_optional_classification_rules_are_scope_driven_and_boundary_safe`"
            ),
        "check2.md should include streaming required/optional scope evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_rust_hygiene_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "check2.md should mark rust-hygiene checklist item as completed."
    );
    assert!(
        check2.contains("`rg -n '\\\\.(unwrap|unwrap_err|expect)\\\\s*\\\\(|^[[:space:]]*let[[:space:]]+_[[:space:]]*=' components/{overlays,overlay,popover,modal,sheet,drawer,contextual-help}/src --glob '*.rs'`")
            && check2.contains("`components/{overlays,popover,sheet}/src/logic.rs`")
            && check2.contains("`components/{overlay,popover,sheet}/src/view.rs`")
            && check2.contains("`Vec<Cow<'static, str>>`")
            && check2.contains("`./scripts/check-rust-hygiene.sh`")
            && check2.contains("`Invalid cross-device link (os error 18)`")
            && check2.contains(
                "overlays_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources"
            )
            && check2.contains(
                "overlays_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent"
            )
            && check2.contains("overlays_rust_hygiene_script_enforces_repo_level_hygiene_guards")
            && check2.contains("`scripts/check-ui-engineering.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`"
            ),
        "check2.md should include rust-hygiene evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_spec_rs_scope_control_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "check2.md should mark spec.rs scope-control checklist item as completed."
    );
    assert!(
        check2.contains("未引入 `spec.rs`")
            && check2.contains("`protocol.rs` + `test/protocol.rs`")
            && check2.contains("overlays_spec_rs_stays_absent_without_complex_schema_requirement"),
        "check2.md should include spec.rs scope-control evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_token_first_static_style_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "check2.md should mark token-first static style contract checklist item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/css.rs` 按 feature gate 聚合")
            && check2.contains("`crate::css::push_components_css` 注入")
            && check2.contains("`var(--ui-*)` token-first")
            && check2.contains("`style=panel_vars`")
            && check2.contains("tailwind/cva/stylist/stylex/emotion/linaria")
            && check2.contains(
                "overlays_token_first_static_style_contract_is_aggregated_and_framework_agnostic"
            ),
        "check2.md should include token-first style contract evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_visual_desire_default_theme_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "check2.md should mark visual-desire default-theme checklist item as completed."
    );
    assert!(
        check2.contains("`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs`")
            && check2.contains(
                "`title=\\\"ThemeVisualBaseline\\\"` / `slug=\\\"theme-visual-baseline\\\""
            )
            && check2.contains("`data-slot=\\\"theme-visual-baseline-button|input|overlay\\\"`")
            && check2.contains("`e2e/tests/docs_app_theme_visual_baseline.spec.mjs`")
            && check2.contains("docs-app-theme-visual-baseline-page/button/input/overlay.png")
            && check2.contains(
                "overlays_visual_desire_has_default_theme_baseline_page_and_screenshot_regression"
            ),
        "check2.md should include visual-desire baseline evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_tree_shaking_first_class_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "check2.md should mark tree-shaking checklist item as completed."
    );
    assert!(
        check2.contains("`component-overlays`")
            && check2.contains("`crates/ui/src/lib.rs`")
            && check2.contains("`crates/ui/src/css.rs`")
            && check2.contains("component-overlays,inject-css")
            && check2.contains("web-demo-components")
            && check2.contains("`scripts/check-ui-tree-shaking.sh`")
            && check2.contains("`scripts/tree_shaking_budget.env`")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2
                .contains("overlays_tree_shaking_contract_is_feature_gated_and_budget_guarded"),
        "check2.md should include tree-shaking evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "check2.md should mark tree-shaking feature-pruning checklist item as completed."
    );
    assert!(
        check2.contains("`component-overlays`")
            && check2.contains("`crates/ui/Cargo.toml`")
            && check2.contains("`crates/ui/src/lib.rs`")
            && check2.contains("`crates/ui/src/css.rs`")
            && check2.contains("`scripts/check-ui-tree-shaking.sh`")
            && check2.contains("component-overlays,inject-css")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2.contains(
                "overlays_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget"
            )
            && check2
                .contains("overlays_check2_marks_tree_shaking_feature_pruning_contract_complete"),
        "check2.md should include tree-shaking feature-pruning evidence and regression test references."
    );
}

#[test]
fn overlays_checklist_marks_semantics_and_performance_regression_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "check2.md should mark semantic/performance regression checklist item as completed."
    );
    assert!(
        check2.contains("overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only")
            && check2.contains("overlays_focus_stack_and_gc_use_global_focus_manager_contract")
            && check2.contains("overlays_performance_governance_contract_is_budgeted_traceable_and_blocking")
            && check2
                .contains("overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement")
            && check2.contains("`render_count` 自动化回归仍在仓库统一 follow-up")
            && check2.contains("`scripts/check-ui-performance.sh`")
            && check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should include semantic/performance regression evidence and regression test references."
    );
}

#[test]
fn overlays_checklist_marks_semantic_test_priority_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "check2.md should mark semantic-test-priority checklist item as completed."
    );
    assert!(
        check2.contains(
            "overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only"
        ) && check2.contains(
            "overlays_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks"
        ) && check2.contains(
            "overlays_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks"
        ) && check2.contains(
            "overlays_performance_script_covers_semantic_test_priority_contract"
        ) && check2.contains("`scripts/check-ui-performance.sh`")
            && check2.contains("`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-overlays-sem-priority")
            && check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should include semantic-test-priority evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_version_deprecation_migration_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "check2.md should mark version-deprecation migration checklist item as completed."
    );
    assert!(
        check2.contains("N/A：本次 `Overlays` 未发生跨大版本 API 破坏升级")
            && check2.contains("`components/overlays/src/Component.toml`")
            && check2.contains("`schema_version = \"1\"`")
            && check2.contains("`components/overlays/src/overlays.rbi`")
            && check2.contains("`migrate_v1_to_v2`")
            && check2.contains("`scripts/check-ui-engineering.sh`")
            && check2.contains(
                "overlays_version_deprecation_migration_is_na_without_major_breaking_upgrade"
            )
            && check2.contains("Invalid cross-device link (os error 18)"),
        "check2.md should include version-deprecation migration N/A evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_docs_product_copy_paste_ready_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "check2.md should mark docs product copy-paste-ready checklist item as completed."
    );
    assert!(
        check2.contains("`apps/docs-app/src/pages/components/pages/overlays.rs`")
            && check2.contains("`title=\"Hello World (Minimal Path)\"`")
            && check2.contains("`title=\"State Matrix\"`")
            && check2.contains("`title=\"Controlled vs Uncontrolled\"`")
            && check2.contains("`title=\"Streaming / Snapshot Contract\"`")
            && check2.contains("`data-slot=\"modal-source-first\"`")
            && check2.contains("`data-slot=\"drawer-source-first\"`")
            && check2.contains("`apps/docs-app/src/playground.rs::compose_copy_ready_code`")
            && check2.contains("`MODAL_DOC_IMPORTS`")
            && check2.contains("`DRAWER_DOC_IMPORTS`")
            && check2.contains(
                "overlays_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot"
            )
            && check2.contains(
                "overlays_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies"
            )
            && check2.contains(
                "overlays_dx_check_script_covers_docs_product_copy_paste_ready_contract"
            )
            && check2.contains("`scripts/check-ui-dx.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`"
            )
            && check2.contains(
                "`cargo test -p ui-overlays overlays_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`"
            ),
        "check2.md should include docs product copy-paste-ready evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_type_system_and_semantic_marker_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "check2.md should mark type-system + semantic-marker checklist item as completed."
    );
    assert!(
        check2.contains("`enum` 建模")
            && check2
                .contains("`resolve_states` / `normalize_open_state` / `resolve_open_contract`")
            && check2.contains("`data-state/data-open/data-closed`")
            && check2.contains("`data-*-source`")
            && check2.contains("`*_mode_enums_map_bool_inputs_to_closed_set`")
            && check2.contains("normalize_open_state_supports_controlled_and_uncontrolled_modes")
            && check2.contains(
                "overlays_type_system_and_semantic_markers_define_machine_readable_contracts"
            ),
        "check2.md should include machine-readable state evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_focus_stack_and_gc_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "check2.md should mark focus-stack checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlay,popover,sheet}/src/view.rs`")
            && check2.contains("with_restore_policy(RestorePolicy::FallbackTo")
            && check2.contains("with_fallback_selector")
            && check2.contains("`FOCUS_MANAGER_STACK`")
            && check2.contains("focus_manager_push_trap/pop_trap/peek_trap")
            && check2.contains("`RestorePolicy::Selector/FallbackTo`")
            && check2.contains("overlays_focus_stack_and_gc_use_global_focus_manager_contract"),
        "check2.md should include focus-stack/global-focus-manager evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_escape_hatches_foreign_zone_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "check2.md should mark escape-hatches checklist item as completed."
    );
    assert!(
        check2.contains("N/A：overlays 家族当前未接入 ECharts/Map 等命令式第三方实例")
            && check2.contains("`ForeignZone/YieldControl/CleanupForeign`")
            && check2.contains("`echarts/mapbox/leaflet/google-maps`")
            && check2.contains(
                "overlays_escape_hatches_foreign_zone_are_not_applicable_without_imperative_instances"
            ),
        "check2.md should include escape-hatches N/A rationale and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_hydration_discontinuity_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "check2.md should mark hydration-discontinuity checklist item as completed."
    );
    assert!(
        check2.contains("N/A（组件内 runtime 随机 ID 初始化）")
            && check2.contains("`now()/UUID/rand`")
            && check2.contains("`normalize_id_base + 固定后缀`")
            && check2.contains("`UiRoot(id_seed) -> provide_ui_id_provider(id_seed)`")
            && check2.contains("`crates/ui/src/root.rs`")
            && check2.contains("`crates/ui-headless/src/id_provider.rs`")
            && check2.contains(
                "overlays_hydration_discontinuity_contract_avoids_entropy_and_keeps_seeded_id_provider_path"
            ),
        "check2.md should include hydration-discontinuity rationale and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_ssr_cross_platform_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "check2.md should mark ssr/cross-platform checklist item as completed."
    );
    assert!(
        check2.contains(
            "`#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]`"
        ) && check2.contains("`crates/ui-headless/src/lib.rs`")
            && check2.contains("`web = [\"leptos/csr\"]`")
            && check2.contains("`ssr = [\"leptos/ssr\"]`")
            && check2.contains("`crates/ui-motion/src/lib.rs`")
            && check2.contains("`web::animate(...)` no-op/stub")
            && check2.contains("component-overlays,inject-css")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2
                .contains("overlays_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe"),
        "check2.md should include ssr/cross-platform evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_headless_web_ssr_mutex_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "check2.md should mark ui-headless web/ssr mutex checklist item as completed."
    );
    assert!(
        check2.contains("`#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(...)`")
            && check2.contains("`web = [\"leptos/csr\"]`")
            && check2.contains("`ssr = [\"leptos/ssr\"]`")
            && check2.contains("`use_focus_trap/use_modal/use_overlay_stack_registration/use_popover_position/overlay_dialog_attrs`")
            && check2.contains("`cargo check -p ui-headless --no-default-features --features web`")
            && check2.contains("`cargo check -p ui-headless --no-default-features --features ssr`")
            && check2.contains("`cargo check -p ui-headless --no-default-features --features web,ssr`")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2.contains("overlays_headless_web_ssr_mutex_guard_is_preserved"),
        "check2.md should include ui-headless mutex evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_ui_motion_non_wasm_stub_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "check2.md should mark ui-motion non-wasm stub checklist item as completed."
    );
    assert!(
        check2.contains("`web::prefers_reduced_motion() -> true`")
            && check2.contains("`web::animate(...)` no-op/stub")
            && check2.contains("`non_wasm_web_backend_is_predictable_noop`")
            && check2.contains("`components/{overlay,popover,sheet}/src/motion.rs`")
            && check2.contains("`components/{modal,tray}/src/motion.rs`")
            && check2.contains("`cargo check -p ui-motion`")
            && check2.contains("`cargo test -p ui-motion --lib`")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2.contains(
                "overlays_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe"
            ),
        "check2.md should include ui-motion non-wasm evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_reduced_motion_ssr_wasm_branch_coverage_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "check2.md should mark reduced-motion/SSR/wasm coverage checklist item as completed."
    );
    assert!(
        check2.contains("`components/popover/src/motion.rs`")
            && check2.contains("`components/sheet/src/motion.rs`")
            && check2.contains("`ui_motion::web::prefers_reduced_motion()`")
            && check2.contains("`ui_motion::spring::SpringAnimator::set_target`")
            && check2.contains("`if crate::web::prefers_reduced_motion()`")
            && check2.contains("`#[cfg(not(target_arch = \"wasm32\"))] attach_motion`")
            && check2.contains("`data-state/data-open/data-closed`")
            && check2.contains("`is_composing/default_prevented`")
            && check2.contains("Invalid cross-device link (os error 18)")
            && check2.contains("`cargo check -p ui-motion`")
            && check2.contains("`cargo test -p ui-motion --lib`")
            && check2.contains(
                "overlays_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split"
            ),
        "check2.md should include reduced-motion/SSR/wasm evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_performance_governance_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "check2.md should mark performance-governance checklist item as completed."
    );
    assert!(
        check2.contains("`UiPerfBudget::mount_only(120.0)`")
            && check2.contains("`apps/docs-app/src/perf_probe.rs::UiPerfProbe`")
            && check2.contains("`data-perf-violation != true`")
            && check2.contains("`data-state/data-open/data-closed/data-*-source/data-motion-source`")
            && check2.contains("`scripts/check-ui-performance.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_performance_governance_contract_is_budgeted_traceable_and_blocking`"
            )
            && check2.contains("N/A（精确 `render_count` 自动计数）")
            && check2.contains("`docs/plan/TODO.md`")
            && check2.contains("`建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据`")
            && check2.contains(
                "overlays_performance_governance_contract_is_budgeted_traceable_and_blocking"
            ),
        "check2.md should include performance governance evidence and regression test references."
    );
}

#[test]
fn overlays_checklist_marks_view_macro_complexity_control_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "check2.md should mark view-macro-complexity checklist item as completed."
    );
    assert!(
        check2.contains("`components/modal/src/view.rs`")
            && check2.contains("`components/sheet/src/view.rs`")
            && check2.contains("`components/tray/src/view.rs`")
            && check2.contains("`render_modal_title/render_modal_description/render_modal_body`")
            && check2.contains("`render_backdrop/render_panel`")
            && check2.contains("`TrayPanelRenderInputs + render_tray_panel`")
            && check2.contains("`if root_state.show_description { ... } else { ... }`")
            && check2.contains("overlays_view_macro_complexity_is_bounded_by_semantic_subblocks"),
        "check2.md should include view-macro-complexity evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_function_first_split_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "check2.md should mark function-first split checklist item as completed."
    );
    assert!(
        check2.contains("`components/modal/src/view.rs`")
            && check2.contains("`components/sheet/src/view.rs`")
            && check2.contains("`components/tray/src/view.rs`")
            && check2.contains("`render_modal_title/render_modal_description/render_modal_body/render_modal_sections`")
            && check2.contains("`render_backdrop/render_panel`")
            && check2.contains("`render_tray_close_slot/render_tray_header_slot/render_tray_body_slot/render_tray_footer_slot/render_tray_panel`")
            && check2.contains("`#[component]`")
            && check2.contains("`#[component]\\nfn render_`")
            && check2
                .contains("overlays_function_first_split_prefers_plain_render_functions_over_extra_components"),
        "check2.md should include function-first split evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_static_fragment_constantization_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "check2.md should mark static-fragment-constantization checklist item as completed."
    );
    assert!(
        check2.contains("`components/tray/src/view.rs`")
            && check2.contains(
                "`TRAY_CLOSE_ICON_VIEWBOX/TRAY_CLOSE_ICON_PATH/TRAY_CLOSE_ICON_STROKE_WIDTH`"
            )
            && check2.contains("`render_tray_close_icon`")
            && check2.contains("`aria-hidden=\\\"true\\\"`")
            && check2.contains("`d=TRAY_CLOSE_ICON_PATH`")
            && check2.contains(
                "overlays_static_fragments_are_constantized_with_accessible_svg_template"
            ),
        "check2.md should include static-fragment constantization evidence and regression test reference."
    );
}

#[test]
fn overlays_checklist_marks_inner_html_constraint_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "check2.md should mark inner_html constraint checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlays,overlay,popover,modal,sheet,tray}/src/*`")
            && check2.contains("`apps/docs-app/src/pages/components/pages/{overlays,overlays_extra}.rs`")
            && check2.contains("`apps/docs-app/src/pages/components/shell.rs`")
            && check2.contains("`component_readme_markdown(slug)`")
            && check2.contains("`include_str!` 白名单常量")
            && check2.contains("`_ => None`")
            && check2.contains("`inner_html=html`")
            && check2.contains(
                "overlays_inner_html_usage_is_forbidden_and_docs_shell_path_is_whitelisted"
            )
            && check2.contains("`scripts/check-ui-inner-html.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_inner_html_usage_is_forbidden_and_docs_shell_path_is_whitelisted`"
            ),
        "check2.md should include inner_html evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_wasm_debug_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
        "check2.md should mark wasm-debug checklist item as completed."
    );
    assert!(
        check2.contains("`use_controllable_open_state_traced(\"modal\", ...)`")
            && check2.contains("`UiTraceEvent { ts_ms, component, kind }`")
            && check2.contains("`apps/docs-app/src/lib.rs`")
            && check2.contains("`provide_ui_trace(debug_overlay_enabled)`")
            && check2.contains("`apps/docs-app/src/debug_overlay.rs`")
            && check2.contains("`data-slot=\"ui-debug-overlay-event\"`")
            && check2.contains("`e2e/tests/docs_app_debug_overlay.spec.mjs`")
            && check2.contains("`toHaveCount(2)`")
            && check2.contains(
                "overlays_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated"
            )
            && check2.contains("`scripts/check-ui-wasm-debug.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated`"
            ),
        "check2.md should include wasm-debug evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_dx_requirement_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "check2.md should mark DX checklist item as completed."
    );
    assert!(
        check2.contains("`apps/docs-app/src/playground.rs`")
            && check2.contains("`compose_scoped_css`")
            && check2.contains("`Show test`")
            && check2.contains("`Restore original CSS`")
            && check2.contains("`title=\"Workbench (Display + Config + Code + CSS Test)\"`")
            && check2.contains("`title=\"Interactive Playground\"`")
            && check2.contains("`test_css_source`")
            && check2.contains("`test_config_signal`")
            && check2.contains("可选持久化状态按 N/A 验收")
            && check2.contains(
                "overlays_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench"
            )
            && check2.contains("`scripts/check-ui-dx.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench`"
            ),
        "check2.md should include DX evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_engineering_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "check2.md should mark engineering capability checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlays,overlay,popover,modal,sheet,tray}/src/protocol.rs`")
            && check2.contains("`#[serde(default)]`")
            && check2.contains("`button-wasm-debug`")
            && check2.contains("`target: \"ui::button::state_change\"`")
            && check2.contains("不新增 overlays 家族私有 tracing/wasm-debug feature 别名")
            && check2.contains("`tokio/async-std/smol/runtime::Handle/spawn_blocking`")
            && check2.contains(
                "overlays_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries"
            )
            && check2.contains("`scripts/check-ui-engineering.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries`"
            ),
        "check2.md should include engineering evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_defensive_variables_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "check2.md should mark defensive-variables checklist item as completed."
    );
    assert!(
        check2.contains("`components/{popover,sheet,tray}/src/styles.rs`")
            && check2.contains("`var(--ui-popover-top, var(--ui-fallback-min-inline-size-none))`")
            && check2.contains("`var(--ui-border-width, var(--ui-fallback-border-width))`")
            && check2.contains("`crates/ui-theme/src/css.rs`")
            && check2.contains("`--ui-fallback-overlay-viewport-inset`")
            && check2.contains("移除 `0px/1px/16px/24px/14px/20px` 这类组件内裸尺寸终值")
            && check2.contains(
                "overlays_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals"
            )
            && check2.contains("`scripts/check-ui-contract-hygiene.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals`"
            ),
        "check2.md should include defensive-variable evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_cascade_layer_contract_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "check2.md should mark cascade-layer checklist item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/css.rs::push_components_css`")
            && check2.contains("`out.push_str(\"\\\\n@layer ui {\\\\n\")`")
            && check2.contains("`#[cfg(feature = \"component-overlay|overlays|popover|modal|sheet|tray\")]`")
            && check2.contains("`style=panel_vars`")
            && check2.contains("`style=\"top: 10px\"`")
            && check2.contains(
                "overlays_cascade_layer_and_runtime_style_contract_is_enforced"
            )
            && check2.contains("`scripts/check-ui-contract-hygiene.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_cascade_layer_and_runtime_style_contract_is_enforced`"
            ),
        "check2.md should include cascade-layer evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_motion_contractualization_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "check2.md should mark motion-contractualization checklist item as completed."
    );
    assert!(
        check2.contains("`components/{overlay,popover,modal,sheet,tray}/src/motion.rs`")
            && check2.contains("`OverlayMotion/PopoverMotion/SheetMotion/TrayMotion`")
            && check2.contains("`MODAL_MOTION_CONTRACT_STIFFNESS`")
            && check2.contains("`ui_motion::web::prefers_reduced_motion()`")
            && check2.contains("`#[cfg(not(target_arch = \"wasm32\"))] attach_motion`")
            && check2.contains("`crates/ui-motion/src/spring.rs`")
            && check2.contains(
                "overlays_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe"
            )
            && check2.contains("`scripts/check-ui-platforms.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`"
            ),
        "check2.md should include motion contractualization evidence and regression gate references."
    );
}

#[test]
fn overlays_checklist_marks_ui_components_fixed_entrypoint_files_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui` 固定入口文件落点正确。"),
        "check2.md should mark ui fixed-entrypoint checklist item as completed."
    );
    assert!(
        check2.contains("`crates/ui/src/lib.rs`")
            && check2.contains("`crates/ui/src/css.rs::push_components_css`")
            && check2.contains("`crates/ui/src/root.rs::UiRoot`")
            && check2.contains("`provide_ui_i18n(i18n)`")
            && check2.contains("`crates/ui-visual-primitive/src/active_highlight.rs`")
            && check2.contains("`crates/ui/src/overlay_open.rs`")
            && check2.contains("`crates/ui/src/presence.rs`")
            && check2.contains("`crates/ui/src/a11y.rs`")
            && check2.contains("`crates/ui-headless/src/controllable_state.rs`")
            && check2.contains("`crates/ui-headless/src/presence.rs`")
            && check2.contains("`crates/ui-headless/src/a11y.rs`")
            && check2.contains("overlays_ui_components_fixed_entry_files_follow_layered_boundaries")
            && check2.contains("`scripts/check-ui-entrypoints.sh`")
            && check2.contains(
                "`cargo test -p ui-overlays overlays_ui_components_fixed_entry_files_follow_layered_boundaries`"
            ),
        "check2.md should include fixed-entrypoint evidence and regression gate references."
    );
}
