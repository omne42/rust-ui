use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if rel_path == "../../apps/docs-app/src/pages/components/pages/overlays_extra.rs" {
        let parent = manifest_dir.join(rel_path);
        let tray_child = manifest_dir
            .join("../../apps/docs-app/src/pages/components/pages/overlays_extra/tray.rs");
        let parent_source = fs::read_to_string(&parent)
            .unwrap_or_else(|e| panic!("read_to_string failed for {parent:?}: {e}"));
        let tray_source = fs::read_to_string(&tray_child)
            .unwrap_or_else(|e| panic!("read_to_string failed for {tray_child:?}: {e}"));

        return format!(
            "{parent_source}\n{tray_source}",
            parent_source = parent_source,
            tray_source = tray_source.replace(
                "pub(crate) fn tray() -> AnyView {",
                "pub(super) fn tray() -> AnyView {",
            ),
        );
    }

    let mapped = match rel_path {
        "src/css.rs" => "../../crates/ui/src/css.rs".to_string(),
        _ if rel_path.starts_with("src/tray/") => {
            format!("src/{}", &rel_path["src/tray/".len()..])
        }
        _ => rel_path.to_string(),
    };
    let path = manifest_dir.join(mapped);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tray_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tray/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tray internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tray_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/tray/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use motion::TrayMotion;",
        "pub use view::Tray;",
        "pub use ui_state_primitives::tray::{TrayPartState, TrayPartStateInput, TraySlot};",
    ] {
        assert!(
            module_source.contains(needle),
            "tray module should include `{needle}` state contracts."
        );
    }

    assert!(
        crate_source.contains("pub use tray::{Tray, TrayMotion};")
            || (crate_source.contains("pub use tray::Tray;")
                && crate_source.contains("pub use tray::TrayMotion;")),
        "crate root should re-export `Tray` and `TrayMotion` contracts."
    );
}

#[test]
fn tray_logic_exposes_state_helpers() {
    let source = load_source("src/tray/logic.rs");

    for needle in [
        "pub use ui_state_primitives::tray::{",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_SHOW_CLOSE_BUTTON",
        "DEFAULT_FIXED_HEIGHT",
        "DEFAULT_DISMISSABLE",
        "DEFAULT_KEYBOARD_DISMISS_DISABLED",
        "normalize_optional_text",
        "normalize_required_text",
        "normalize_id_base",
        "normalize_defaults",
        "normalize_on_open_change",
        "normalize_on_close",
        "TrayDismissPolicy",
        "resolve_dismiss_policy",
        "resolve_close_effects",
        "resolve_open_signal",
        "normalize_state_inputs",
        "resolve_state",
        "pub fn compose_class_name(base_class_name: Option<String>, state: TrayPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Tray logic should include `{needle}` while consuming state primitives from ui-state-primitives."
        );
    }
}

#[test]
fn tray_state_primitives_source_is_ui_state_primitives_without_business_store_binding() {
    let logic_source = load_source("src/tray/logic.rs");
    let view_source = load_source("src/tray/view.rs");

    for needle in [
        "pub use ui_state_primitives::tray::{",
        "resolve_open_config(TrayOpenConfigInput {",
        "can_request_open_change(",
        "resolve_state(ui_state_primitives::tray::TrayPartStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tray logic should consume ui-state-primitives state contract via `{needle}`."
        );
    }

    for forbidden in [
        "use_store",
        "AppStore",
        "GlobalStore",
        "store::",
        "redux",
        "zustand",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Tray component should not bind business/global store type `{forbidden}` directly."
        );
    }

    for forbidden in ["pub enum TrayOpenMode", "pub struct TrayOpenConfigInput"] {
        assert!(
            !logic_source.contains(forbidden),
            "Tray logic should not reimplement open-state primitive `{forbidden}`."
        );
    }
}

#[test]
fn tray_async_interaction_contract_is_na_with_explicit_reason() {
    let logic_source = load_source("src/tray/logic.rs");
    let view_source = load_source("src/tray/view.rs");
    let styles_source = load_source("src/tray/styles.rs");
    let readme_source = load_source("src/README.md");
    let checklist_source = load_source("check2.md");
    let src_checklist_source = load_source("src/check2.md");
    let combined = format!("{logic_source}\n{view_source}\n{styles_source}\n{readme_source}");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "on_retry",
        "retry_count",
        "data-loading",
        "on_error",
        "use_async_action",
        "create_resource(",
        "create_action(",
        "spawn_local(",
        "async fn",
        ".await",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not define per-component async loading/error protocol via `{forbidden}` when async interaction is N/A."
        );
    }

    for source in [checklist_source, src_checklist_source] {
        assert!(
            source.contains("- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。"),
            "Tray checklist should mark async semantic item as complete when N/A."
        );
        assert!(
            source.contains(
                "N/A：`Tray` 当前仅处理本地开合与可见性状态，无远程请求、无异步提交与失败重试路径。"
            ),
            "Tray checklist should record an explicit N/A reason for async interaction semantics."
        );
    }
}

#[test]
fn tray_dx_paradox_keeps_minimal_path_simple_and_advanced_controls_optional() {
    let view_source = load_source("src/tray/view.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let checklist_source = load_source("check2.md");
    let src_checklist_source = load_source("src/check2.md");

    for needle in [
        "## Hello World（最小可用）",
        "<Tray default_open=true id_base=\"docs-tray\".to_string() title=\"Notifications\".to_string()>",
        "<p>\"Tray body\"</p>",
    ] {
        assert!(
            readme_source.contains(needle),
            "Tray README should keep minimal Hello World path via `{needle}`."
        );
    }

    for forbidden in [
        "let (open, set_open) = signal(",
        "Signal::derive(",
        "on_open_change=on_open_change",
        "<Tray state=",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "Tray README Hello World should not require internal-state wiring token `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "id_base: String",
        "title: String",
        "children: ChildrenFn",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray API should keep simple defaults + optional advanced controls via `{needle}`."
        );
    }

    for forbidden in [
        "pub fn Tray(\n    state:",
        "#[prop(optional)] state:",
        "#[prop(optional)] machine:",
        "<Tray state=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tray API should not expose internal state object as required input `{forbidden}`."
        );
    }

    for needle in [
        "Playground title=\"Hello World (Minimal API)\" code_signal=hello_code",
        "`default_open + id_base + title`",
        "default_open=true",
        "id_base=\"docs-tray-hello\".to_string()",
        "title=\"Filters\".to_string()",
        "<div>\"Tray body content\"</div>",
        "title=\"Workbench (All API + Actual Config)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tray docs-app should expose minimal path and advanced path via `{needle}`."
        );
    }

    for source in [checklist_source, src_checklist_source] {
        assert!(
            source.contains(
                "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"
            ),
            "Tray checklist should mark DX paradox item as complete."
        );
    }
}

#[test]
fn tray_composition_api_rule_is_na_for_single_panel_overlay() {
    let view_source = load_source("src/tray/view.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let checklist_source = load_source("check2.md");
    let src_checklist_source = load_source("src/check2.md");
    let combined_docs = format!("{readme_source}\n{docs_source}");

    for source in [checklist_source, src_checklist_source] {
        assert!(
            source.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
            "Tray checklist should mark composition-api item as complete."
        );
        assert!(
            source.contains(
                "N/A：`Tray` 为单面板 overlay，不存在 `Parent/Item` 子项集合与索引配对场景。"
            ),
            "Tray checklist should explain why composition-api item is N/A."
        );
    }

    for needle in [
        "title: String",
        "children: ChildrenFn",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] footer: Option<ViewFn>",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray API should keep content semantics in one explicit component boundary via `{needle}`."
        );
    }

    for forbidden in [
        "labels + children",
        "titles + panels",
        "ItemSpec",
        "items: Vec",
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "<Tray labels=",
        "<Tray titles=",
        "<Tray panels=",
        "item_specs",
    ] {
        assert!(
            !view_source.contains(forbidden) && !combined_docs.contains(forbidden),
            "Tray should not expose parallel-array composition protocol `{forbidden}`."
        );
    }
}

#[test]
fn tray_macro_micro_duality_is_not_applicable_without_dragging_loop() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");

    for needle in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "N/A：`Tray` 当前不提供拖拽/连续手势交互",
        "tray_macro_micro_duality_is_not_applicable_without_dragging_loop",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document macro/micro duality N/A contract via `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:pointermove",
        "on:pointerdown",
        "on:pointerup",
        "requestAnimationFrame",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose dragging macro/micro loop token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_two_pass_rendering_is_not_applicable_without_geometry_measurement_loop() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");

    for needle in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A：`Tray` 当前为固定底部 Sheet 装配，不依赖 DOM 几何测量闭环",
        "tray_two_pass_rendering_is_not_applicable_without_geometry_measurement_loop",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document two-pass rendering N/A contract via `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "requestAnimationFrame",
        "Intent -> Measure",
        "Rectification",
        "measure(",
        "measure_rect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose geometry two-pass rendering loop token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_registration_protocol_is_not_applicable_without_dynamic_item_collection() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");

    for needle in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "N/A：`Tray` 为单面板 overlay，不维护动态子项集合与顺序",
        "tray_registration_protocol_is_not_applicable_without_dynamic_item_collection",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document registration-protocol N/A contract via `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "Vec<TrayItem>",
        "register_item",
        "unregister_item",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose dynamic-item registration protocol token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_slot_projection_policy_is_not_applicable_without_keep_alive_axis() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "N/A：`Tray` 当前为单面板 `Sheet` 装配，不暴露 `Lazy/KeepAlive/Eager` 投影策略轴",
        "tray_slot_projection_policy_is_not_applicable_without_keep_alive_axis",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document slot-projection policy N/A contract via `{needle}`."
        );
    }

    let combined =
        format!("{view_source}\n{logic_source}\n{motion_source}\n{readme_source}\n{docs_source}");
    for forbidden in [
        "KeepAlive",
        "NotifyHidden",
        "notify_hidden",
        "Lazy",
        "Eager",
        "projection_policy",
        "slot_projection",
        "on_hidden",
        "on_hide",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose slot-projection keep-alive protocol token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_env_streams_are_not_applicable_without_environment_sampling_pipeline() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "N/A：`Tray` 当前无 Resize/Theme/Intersection 事件采样与订阅流",
        "tray_env_streams_are_not_applicable_without_environment_sampling_pipeline",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document env-streams N/A contract via `{needle}`."
        );
    }

    let combined =
        format!("{view_source}\n{logic_source}\n{motion_source}\n{readme_source}\n{docs_source}");
    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "MutationObserver",
        "match_media",
        "prefers-color-scheme",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "debounce",
        "throttle",
        "requestAnimationFrame",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose environment-stream pipeline token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_event_light_cone_is_not_applicable_without_large_collection_batch_ops() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "N/A：`Tray` 为单面板 overlay，不承载 `Table/Grid` 大型集合批处理",
        "tray_event_light_cone_is_not_applicable_without_large_collection_batch_ops",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document event-light-cone N/A contract via `{needle}`."
        );
    }

    let combined =
        format!("{view_source}\n{logic_source}\n{motion_source}\n{readme_source}\n{docs_source}");
    for forbidden in [
        "Context Bus",
        "SelectionState::All",
        "batch_select",
        "select_all_rows",
        "grid_selection",
        "table_selection",
        "prop drilling",
        "broadcast_selection",
        "row_ids",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose event-light-cone large-collection protocol token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_causality_bus_is_not_applicable_without_cross_subscriber_derived_bus() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");
    let motion_source = load_source("src/tray/motion.rs");
    let readme_source = load_source("src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "N/A：`Tray` 当前不承载复杂派生总线与多订阅者广播流",
        "tray_causality_bus_is_not_applicable_without_cross_subscriber_derived_bus",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tray checklist should document causality-bus N/A contract via `{needle}`."
        );
    }

    let combined =
        format!("{view_source}\n{logic_source}\n{motion_source}\n{readme_source}\n{docs_source}");
    for forbidden in [
        "TraceId",
        "trace_id",
        "Causality Bus",
        "causality_bus",
        "event_bus",
        "command_bus",
        "broadcast_to_subscribers",
        "dispatch_derived_action",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tray should not expose causality-bus trace propagation token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn tray_api_naming_contract_uses_is_prefix_for_bool_axes() {
    let source = load_source("src/tray/view.rs");
    let readme = load_source("src/README.md");

    for needle in [
        "is_open: Option<Signal<bool>>",
        "is_show_close_button: Option<bool>",
    ] {
        assert!(
            source.contains(needle),
            "Tray public API naming should include `{needle}`."
        );
    }

    for forbidden in [
        "pub fn Tray(\n    open: Signal<bool>",
        "#[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] show_close_button: bool",
    ] {
        assert!(
            !source.contains(forbidden),
            "Tray public API naming should not expose legacy alias `{forbidden}`."
        );
    }

    for needle in [
        "| `is_open` | `Option<Signal<bool>>` | `None` |",
        "| `is_show_close_button` | `Option<bool>` | `None`（归一化为 `true`） |",
    ] {
        assert!(
            readme.contains(needle),
            "Tray README should document migrated API naming `{needle}`."
        );
    }
}

#[test]
fn tray_open_axis_supports_controlled_and_uncontrolled_pairing() {
    let source = load_source("src/tray/view.rs");
    let readme = load_source("src/README.md");
    let manifest = load_source("src/Component.toml");
    let rbi = load_source("src/tray.rbi");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let defaults = logic::normalize_defaults(logic::TrayDefaultsInput {",
        "let open_state = logic::normalize_open_state(logic::TrayOpenStateInput {",
        "let open_source_attr = open_state.open_source_attr;",
        "let on_open_change = open_state.on_open_change;",
        "let close_effects =",
        "logic::resolve_close_effects(open_state.mode, open_state.has_open_change_handler);",
        "let (uncontrolled_open, set_uncontrolled_open) = signal(open_state.default_open);",
        "let resolved_open = logic::resolve_open_signal(",
        "let on_close = logic::normalize_on_close(on_close);",
        "if close_effects.should_close_uncontrolled {",
        "if close_effects.should_emit_open_change {",
        "set_uncontrolled_open.set(false);",
        "on_open_change.run(false);",
    ] {
        assert!(
            source.contains(needle),
            "Tray open axis pairing should include `{needle}`."
        );
    }

    for needle in [
        "| `default_open` | `Option<bool>` | `None`（归一化为 `false`） |",
        "| `on_open_change` | `Option<Callback<bool>>` | `None` |",
    ] {
        assert!(
            readme.contains(needle),
            "Tray README should document open-axis pairing field `{needle}`."
        );
    }

    for needle in [
        "name = \"is_open\"",
        "ty = \"Option<leptos::prelude::Signal<bool>>\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
    ] {
        assert!(
            manifest.contains(needle),
            "Tray component manifest should include `{needle}`."
        );
    }

    for needle in [
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(rbi.contains(needle), "Tray RBI should project `{needle}`.");
    }
}

#[test]
fn tray_default_value_normalization_is_single_sourced_in_logic() {
    let view = load_source("src/tray/view.rs");
    let logic = load_source("src/tray/logic.rs");

    for forbidden in [
        "#[prop(optional, default =",
        ".unwrap_or(",
        ".unwrap_or_else(",
    ] {
        assert!(
            !view.contains(forbidden),
            "Tray view should not perform local default fallback via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct TrayDefaultsInput",
        "pub struct TrayDefaults",
        "pub fn normalize_defaults(input: TrayDefaultsInput) -> TrayDefaults",
        "pub fn normalize_open_state(input: TrayOpenStateInput) -> TrayOpenState",
        "pub fn resolve_open_signal(open: Option<Signal<bool>>, fallback: Signal<bool>) -> Signal<bool>",
    ] {
        assert!(
            logic.contains(needle),
            "Tray logic should own centralized default normalization via `{needle}`."
        );
    }

    for needle in [
        "let defaults = logic::normalize_defaults(logic::TrayDefaultsInput {",
        "let open_state = logic::normalize_open_state(logic::TrayOpenStateInput {",
        "let resolved_open = logic::resolve_open_signal(",
        "let on_close = logic::normalize_on_close(on_close);",
        "let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);",
    ] {
        assert!(
            view.contains(needle),
            "Tray view should consume logic-normalized defaults via `{needle}`."
        );
    }
}

#[test]
fn tray_state_normalization_is_centralized_in_logic() {
    let view = load_source("src/tray/view.rs");
    let logic = load_source("src/tray/logic.rs");

    for forbidden in [
        "TrayStateInputs {",
        "TrayDescriptionMode::from_has_description(",
        "TrayFooterMode::from_has_footer(",
        "TrayCloseButtonMode::from_show_close_button(",
        "TraySizeMode::from_is_fixed_height(",
        "TrayDismissMode::from_is_dismissable(",
        "TrayKeyboardDismissMode::from_is_disabled(",
        "can_request_open_change(",
        "TrayOpenMode::",
    ] {
        assert!(
            !view.contains(forbidden),
            "Tray view should not reconstruct state-machine rules via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct TrayStateBoundaryInput {",
        "pub(crate) dismiss_policy: TrayDismissPolicy",
        "pub fn normalize_state_inputs(input: TrayStateBoundaryInput) -> TrayStateInputs",
        "pub fn resolve_states(input: TrayStateInputs) -> TrayResolvedStates",
        "pub fn resolve_dismiss_policy(",
        "pub fn resolve_close_effects(",
    ] {
        assert!(
            logic.contains(needle),
            "Tray logic should own typed state normalization contract `{needle}`."
        );
    }

    for needle in [
        "let dismiss_policy = logic::resolve_dismiss_policy(",
        "let close_effects =",
        "logic::resolve_close_effects(open_state.mode, open_state.has_open_change_handler);",
        "let state_inputs = logic::normalize_state_inputs(logic::TrayStateBoundaryInput {",
    ] {
        assert!(
            view.contains(needle),
            "Tray view should only consume logic-level normalization outputs `{needle}`."
        );
    }
}

#[test]
fn tray_composes_sheet_with_bottom_placement_and_motion_contract() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "<Sheet",
        "placement=SheetPlacement::Bottom",
        "aria_labelledby=panel_aria_labelledby",
        "aria_describedby=panel_aria_describedby",
        "is_dismissable=defaults.is_dismissable",
        "is_keyboard_dismiss_disabled=defaults.is_keyboard_dismiss_disabled",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Tray should compose Sheet with stable overlay + motion contracts (`{needle}`)."
        );
    }
}

#[test]
fn tray_view_uses_logic_state_contracts() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "let text = logic::normalize_text(logic::TrayTextInput {",
        "id_base,",
        "title,",
        "description,",
        "class_name,",
        "let state_inputs = logic::normalize_state_inputs(logic::TrayStateBoundaryInput {",
        "let resolved_states = logic::resolve_states(state_inputs);",
        "logic::compose_class_name(text.class_name, root_state)",
        "let title_id = format!(\"{}-title\", text.id_base);",
        "let description_id = format!(\"{}-description\", text.id_base);",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-open-source=open_source_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-size=root_state.size_attr",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-size-source=root_state.size_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-footer=(root_state.footer_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-close=(root_state.close_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-size=(root_state.size_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-dismiss=(root_state.dismiss_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-keyboard-dismiss=(root_state.keyboard_dismiss_source_attr == \"custom\").then_some(\"true\")",
        "data-slot=header_state.slot_attr",
        "data-slot=title_state.slot_attr",
        "data-slot=body_state.slot_attr",
        "data-slot=footer_state.slot_attr",
        "data-slot=close_state.slot_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tray view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn tray_uses_headless_overlay_a11y_contract() {
    let source = load_source("src/tray/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, TrayA11yOptions, use_tray_a11y};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let panel_a11y = use_tray_a11y(TrayA11yOptions {",
        "has_description: root_state.show_description",
        "let panel_aria_labelledby = panel_a11y.attrs.aria_labelledby;",
        "let panel_aria_describedby = panel_a11y.attrs.aria_describedby;",
        "let panel_lang = StoredValue::new(panel_a11y.attrs.lang);",
        "let panel_dir = panel_a11y.attrs.dir;",
        "let panel_description_a11y_state = panel_a11y.state.description_state.as_attr();",
        "let description_id = format!(\"{}-description\", text.id_base)",
        "aria_describedby=panel_aria_describedby",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
        "data-description-a11y=panel_description_a11y_state",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Tray should consume typed overlay A11y attrs from ui-headless (`{needle}`)."
        );
    }
}

#[test]
fn tray_a11y_i18n_contract_keeps_text_source_out_of_view() {
    let check2_source = load_source("check2.md");
    let src_check2_source = load_source("src/check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");

    for source in [check2_source, src_check2_source] {
        assert!(
            source.contains(
                "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
            ),
            "Tray checklist should mark A11y + i18n/l10n item as complete."
        );
        assert!(
            source.contains("tray_a11y_i18n_contract_keeps_text_source_out_of_view"),
            "Tray checklist should include regression pointer for A11y + i18n/l10n contract."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, TrayA11yOptions, use_tray_a11y};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let panel_a11y = use_tray_a11y(TrayA11yOptions {",
        "aria_label=close_label",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray view should expose A11y/i18n integration hook `{needle}`."
        );
    }

    for forbidden in ["\"Close tray\"", "DEFAULT_CLOSE_LABEL"] {
        assert!(
            !view_source.contains(forbidden),
            "Tray view should not hardcode user-visible fallback text token `{forbidden}`."
        );
    }

    for needle in [
        "pub const DEFAULT_CLOSE_LABEL: &str = \"Close tray\";",
        "close_label: input.close_label.unwrap_or(DEFAULT_CLOSE_LABEL),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tray logic should own fallback text normalization via `{needle}`."
        );
    }
}

#[test]
fn tray_state_observability_contract_uses_stable_data_and_aria_markers() {
    let check2_source = load_source("check2.md");
    let src_check2_source = load_source("src/check2.md");
    let view_source = load_source("src/tray/view.rs");
    let logic_source = load_source("src/tray/logic.rs");

    for source in [check2_source, src_check2_source] {
        assert!(
            source.contains(
                "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
            ),
            "Tray checklist should mark state observability item as complete."
        );
        assert!(
            source.contains("tray_state_observability_contract_uses_stable_data_and_aria_markers"),
            "Tray checklist should include regression pointer for state observability contract."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-open-source=open_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-size-source=root_state.size_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "aria_labelledby=panel_aria_labelledby",
        "aria_describedby=panel_aria_describedby",
        "data-description-a11y=panel_description_a11y_state",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray view should expose stable observable marker `{needle}`."
        );
    }

    for needle in [
        "pub enum TrayDescriptionMode {",
        "pub enum TrayFooterMode {",
        "pub enum TrayCloseButtonMode {",
        "pub enum TraySizeMode {",
        "pub enum TrayDismissMode {",
        "pub enum TrayKeyboardDismissMode {",
        "pub fn resolve_states(input: TrayStateInputs) -> TrayResolvedStates",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tray logic should keep closed-set state modeling via `{needle}`."
        );
    }

    for forbidden in [
        "data-state=move || format!(",
        "data-open-source=move || format!(",
        "data-description-source=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tray view should not construct free-form marker text via `{forbidden}`."
        );
    }
}

#[test]
fn tray_semantic_contract_tests_exist_and_do_not_rely_on_visual_snapshots() {
    let check2_source = load_source("check2.md");
    let src_check2_source = load_source("src/check2.md");
    let view_source = load_source("src/tray/view.rs");
    let semantics_source = load_source("test/semantics.rs");

    for source in [check2_source, src_check2_source] {
        assert!(
            source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
            "Tray checklist should mark semantic-contract testing item as complete."
        );
        assert!(
            source
                .contains("tray_semantic_contract_tests_exist_and_do_not_rely_on_visual_snapshots"),
            "Tray checklist should include regression pointer for semantic-contract testing."
        );
    }

    for needle in [
        "fn tray_open_axis_supports_controlled_and_uncontrolled_pairing()",
        "fn tray_uses_headless_overlay_a11y_contract()",
        "fn tray_state_observability_contract_uses_stable_data_and_aria_markers()",
        "fn tray_a11y_i18n_contract_keeps_text_source_out_of_view()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "Tray semantic test suite should include `{needle}` for key contract branches."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-source=open_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "aria_labelledby=panel_aria_labelledby",
        "aria_describedby=panel_aria_describedby",
    ] {
        assert!(
            view_source.contains(needle),
            "Tray view should expose semantic marker `{needle}` for contract-oriented assertions."
        );
    }

    for forbidden in [
        ["assert", "_snapshot!"].concat(),
        ["insta", "::assert"].concat(),
        ["to_match", "_snapshot"].concat(),
        [".", "snap"].concat(),
    ] {
        assert!(
            !semantics_source.contains(&forbidden),
            "Tray semantic test suite should not depend on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn tray_styles_include_state_and_source_markers() {
    let source = load_source("src/tray/styles.rs");

    for selector in [
        ".ui-tray[data-motion-source=\"custom\"]",
        ".ui-tray[data-custom-motion=\"true\"]",
        ".ui-tray--custom-description",
        ".ui-tray[data-custom-description=\"true\"]",
        ".ui-tray[data-description-source=\"custom\"]",
        ".ui-tray--custom-footer",
        ".ui-tray[data-custom-footer=\"true\"]",
        ".ui-tray[data-footer-source=\"custom\"]",
        ".ui-tray--custom-close",
        ".ui-tray[data-custom-close=\"true\"]",
        ".ui-tray[data-close-source=\"custom\"]",
        ".ui-tray--custom-size",
        ".ui-tray[data-custom-size=\"true\"]",
        ".ui-tray[data-size-source=\"custom\"]",
        ".ui-tray[data-dismiss-source=\"custom\"]",
        ".ui-tray[data-custom-dismiss=\"true\"]",
        ".ui-tray[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-tray[data-custom-keyboard-dismiss=\"true\"]",
        ".ui-tray--custom-id",
        ".ui-tray[data-id-source=\"custom\"]",
        ".ui-tray[data-custom-id=\"true\"]",
        ".ui-tray--custom-title",
        ".ui-tray[data-title-source=\"custom\"]",
        ".ui-tray[data-custom-title=\"true\"]",
        ".ui-tray[data-class-source=\"custom\"]",
        ".ui-tray[data-exit-source=\"custom\"]",
        ".ui-tray[data-custom-exit=\"true\"]",
        ".ui-tray--fixed-height",
        ".ui-tray[data-size=\"auto\"]",
        ".ui-tray--with-description",
        ".ui-tray[data-state=\"title-only\"]",
        ".ui-tray--close-shown .ui-tray__header",
        ".ui-tray[data-close-button=\"shown\"] .ui-tray__header",
        ".ui-tray[data-footer=\"present\"] .ui-tray__footer",
        ".ui-tray__header[data-slot=\"tray-header\"]",
        ".ui-tray__title[data-slot=\"tray-title\"]",
        ".ui-tray__body[data-slot=\"tray-body\"]",
        ".ui-tray__footer[data-slot=\"tray-footer\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tray styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn tray_style_contract_uses_explicit_markers_without_fragile_dom_assumptions() {
    let check2_source = load_source("check2.md");
    let src_check2_source = load_source("src/check2.md");
    let styles_source = load_source("src/tray/styles.rs");
    let view_source = load_source("src/tray/view.rs");

    for source in [check2_source, src_check2_source] {
        assert!(
            source.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
            "Tray checklist should mark explicit-state styling contract as complete."
        );
        assert!(
            source.contains(
                "tray_style_contract_uses_explicit_markers_without_fragile_dom_assumptions"
            ),
            "Tray checklist should include regression pointer for explicit-state styling contract."
        );
    }

    for selector in [
        ".ui-tray[data-state=\"with-description\"]",
        ".ui-tray[data-state=\"title-only\"]",
        ".ui-tray[data-close-button=\"shown\"] .ui-tray__header",
        ".ui-tray[data-close-button=\"hidden\"] .ui-tray__header",
        ".ui-tray[data-footer=\"present\"] .ui-tray__footer",
        ".ui-tray__header[data-slot=\"tray-header\"]",
        ".ui-tray__footer[data-slot=\"tray-footer\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Tray styles should drive visual state via semantic marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":has(", ":empty"] {
        assert!(
            !styles_source.contains(forbidden),
            "Tray styles should not rely on fragile structural selector `{forbidden}`."
        );
    }

    for needle in [
        "--ui-tray-custom-motion: 1;",
        "--ui-tray-description-source: custom;",
        "--ui-tray-footer-source: custom;",
    ] {
        assert!(
            styles_source.contains(needle),
            "Tray styles should express runtime variance through CSS custom property contract `{needle}`."
        );
    }

    for forbidden in ["style=", "style ="] {
        assert!(
            !view_source.contains(forbidden),
            "Tray view should not embed business styling logic through inline `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tray_motion_contract_exposes_default_and_custom_sheet_checks() {
    let source = load_source("src/tray/motion.rs");

    for needle in [
        "pub struct TrayMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Tray motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn tray_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::tray::styles::CSS);"),
        "ui css aggregator should include tray styles."
    );
}

#[test]
fn tray_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn tray() -> AnyView",
        "title=\"Tray\"",
        "slug=\"tray\"",
        "State + Source Markers",
        "data-size-source",
        "<Tray",
    ] {
        assert!(
            source.contains(needle),
            "tray docs page should contain `{needle}`."
        );
    }
}

#[test]
fn tray_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "let custom_motion = TrayMotion {",
        "sheet: ui::SheetMotion {",
        "initial_offset_px: 46.0",
        "id_base=\"docs-tray-fixed\".to_string()",
        "motion=custom_motion",
        "is_fixed_height=true",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "is_show_close_button=false",
        "class_name=\"docs-tray-custom\".to_string()",
        "Inspect data-size-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "tray docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tray_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/tray/motion.rs");
    let view_source = load_source("src/tray/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TrayMotion) -> TrayMotion",
        "sheet: crate::sheet::SheetMotion",
        "crate::sheet::motion::sanitize_motion(motion.sheet)",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_offset_range()",
        "fn sanitize_motion_delegates_to_sheet_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tray motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::tray::motion::sanitize_motion(motion);"),
        "Tray view should sanitize motion before forwarding it to Sheet.",
    );
}

#[test]
fn tray_docs_page_covers_primary_playgrounds() {
    tray_docs_page_contains_state_source_playground();
}

#[test]
fn tray_docs_playgrounds_lock_state_matrix_contract_values() {
    tray_docs_custom_motion_playground_locks_contract_values();
}
