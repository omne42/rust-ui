fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "component_manifest" => include_str!("../src/Component.toml"),
        "component_rbi" => include_str!("../src/kbd.rbi"),
        "protocol" => include_str!("../src/protocol.rs"),
        "ui_components_css" => include_str!("../../../crates/ui-components/src/css.rs"),
        "ui_components_lib" => include_str!("../../../crates/ui-components/src/lib.rs"),
        "ui_visual_active_highlight" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "ui_components_cargo" => include_str!("../../../crates/ui-components/Cargo.toml"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "perf_script" => include_str!("../../../scripts/check-ui-components-performance.sh"),
        "e2e_docs_coverage" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "e2e_kbd_contract" => include_str!("../../../e2e/tests/docs_app_kbd_contract.spec.mjs"),
        "docs_todo" => include_str!("../../../docs/plan/TODO.md"),
        "kbd_cargo" => include_str!("../Cargo.toml"),
        "web_demo_cargo" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "ui_components_root" => include_str!("../../../crates/ui-components/src/root.rs"),
        "semantics" => include_str!("../test/semantics.rs"),
        "readme" => include_str!("../src/README.md"),
        "docs_display" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs")
        }
        "docs_pages_catalog" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "docs_playground" => include_str!("../../../apps/docs-app/src/playground.rs"),
        "docs_heroui_strategy" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "primitive_kbd" => include_str!("../../../crates/ui-state-primitives/src/kbd.rs"),
        "check2" => include_str!("../check2.md"),
        "legacy_semantics" => include_str!("../../../components/kbd/test/kbd_semantics.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn kbd_semantics_tests_are_migrated_to_component_directory() {
    let module = load_source("mod");
    let legacy = load_source("legacy_semantics");

    assert!(
        module.contains("#[path = \"../test/semantics.rs\"]")
            && module.contains("mod semantics_tests;"),
        "kbd should wire local semantics suite from `components/kbd/src/mod.rs`."
    );
    assert!(
        legacy.contains("include!(\"../../components/kbd/test/semantics.rs\");"),
        "legacy semantics entry should bridge to `components/kbd/test/semantics.rs`."
    );
}

#[test]
fn kbd_component_keeps_ui_components_layering_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::KbdSize;",
        "pub use view::Kbd;",
    ] {
        assert!(
            module.contains(required),
            "kbd module boundary should contain `{required}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !module.contains(forbidden),
            "kbd internals should stay private (`{forbidden}`)."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "HtmlElement",
        "Element",
        "NodeRef",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd component surface should stay DOM-platform agnostic (`{forbidden}`)."
        );
    }
}

#[test]
fn kbd_component_consumes_primitives_without_reimplementing_state_machines() {
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        logic.contains(
            "pub use ui_state_primitives::kbd::{KbdSize, KbdState, KbdStateInput, resolve_state};"
        ),
        "kbd logic should consume state primitives from `ui-state-primitives`."
    );

    for forbidden in [
        "pub struct KbdStateInput",
        "pub struct KbdState",
        "pub fn resolve_state(input: KbdStateInput)",
    ] {
        assert!(
            !logic.contains(forbidden),
            "kbd logic should not reimplement primitive state contract (`{forbidden}`)."
        );
    }

    for required in [
        "pub struct KbdLogicInput",
        "pub struct KbdViewModel",
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "kbd should keep centralized logic-view contract marker `{required}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(keys)",
        "logic::resolve_state(KbdStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view should not derive state directly (`{forbidden}`)."
        );
    }
}

#[test]
fn kbd_checklist_marks_ui_components_boundary_complete_with_local_semantics_evidence() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/kbd/test/semantics.rs",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include ui-components evidence `{required}`."
        );
    }
}

#[test]
fn kbd_api_naming_contract_uses_na_for_absent_bool_callback_default_axes() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "#[prop(optional, into)] keys: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view.contains(required),
            "kbd public api should keep documented prop surface `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] is_",
        "#[prop(optional)] on_",
        "#[prop(optional)] default_",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd should not introduce fake naming axes (`{forbidden}`) when no such state exists."
        );
    }

    for required in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "已核验（kbd）：公开 API 仅 `size/keys/class_name/children`",
        "规则在本组件按 N/A 适配",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should keep naming-contract evidence `{required}`."
        );
    }
}

#[test]
fn kbd_default_value_normalization_is_centralized_in_logic() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize",
        "value.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic should own default-size normalization via `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
    ] {
        assert!(
            view.contains(required),
            "kbd view should consume logic-normalized default values via `{required}`."
        );
    }

    for forbidden in [
        "size.unwrap_or",
        "size.unwrap_or_default()",
        "if size.is_none()",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view should not implement fallback logic directly (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "默认值核验（kbd）：`size` 默认值仅在 `logic::normalize_size` 归一化",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include default-source evidence `{required}`."
        );
    }
}

#[test]
fn kbd_state_normalization_is_centralized_in_logic() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "let keys = normalize_optional_text(input.keys);",
        "let class_name = normalize_optional_text(input.class_name);",
        "let state = resolve_state(KbdStateInput {",
        "let class = compose_class_name(class_name, state);",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic should own state normalization pipeline via `{required}`."
        );
    }

    for required in [
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            view.contains(required),
            "kbd view should only consume logic-derived state via `{required}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(keys)",
        "logic::resolve_state(KbdStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view should not keep distributed state derivation (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "状态归一化核验（kbd）：`size/keys/class_name` 输入统一进入 `logic::resolve_view_model` 归一化并派生 `state/class`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include state-normalization evidence `{required}`."
        );
    }
}

#[test]
fn kbd_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_",
        "#[prop(optional)] on_",
        "use_controllable_state(",
        "request_change",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd should not expose half-controlled state axis (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "本组件不存在可控状态轴",
        "若未来新增可控轴，必须一次性提供 `value + on_value_change + default_value`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include controlled/uncontrolled evidence `{required}`."
        );
    }
}

#[test]
fn kbd_discrete_states_are_enum_constrained_without_bool_explosion() {
    let primitive = load_source("primitive_kbd");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum KbdSize",
        "#[default]",
        "pub struct KbdStateInput",
        "pub size: KbdSize",
        "pub state_attr: &'static str",
        "let (state_class, state_attr) = if input.has_keys {",
        "(\"ui-kbd--state-with-keys\", \"with-keys\")",
        "(\"ui-kbd--state-label-only\", \"label-only\")",
    ] {
        assert!(
            primitive.contains(required),
            "kbd primitive should keep closed discrete-state contract marker `{required}`."
        );
    }

    for required in [
        "pub struct KbdLogicInput",
        "pub size: Option<KbdSize>",
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize",
        "#[prop(optional)] size: Option<KbdSize>",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "kbd should keep typed discrete size/state integration marker `{required}`."
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "variant: Option<bool>",
        "mode: Option<bool>",
        "status: Option<bool>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not expose unconstrained string/bool discrete axes (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "离散状态核验（kbd）：`size` 由 `ui-state-primitives::kbd::KbdSize`（`enum`）建模",
        "不存在用多个 `Option<bool>` 表达互斥状态机",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include discrete-state evidence `{required}`."
        );
    }
}

#[test]
fn kbd_state_primitives_source_is_correct_without_store_coupling() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub use ui_state_primitives::kbd::{KbdSize, KbdState, KbdStateInput, resolve_state};",
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "let state = resolve_state(KbdStateInput {",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "kbd should consume ui-state-primitives and only map outputs (`{required}`)."
        );
    }

    for forbidden in [
        "pub struct KbdStateInput",
        "pub struct KbdState",
        "pub fn resolve_state(input: KbdStateInput)",
        "::store::",
        "use crate::store",
        "use app::store",
        "use ui_state_store",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not reimplement primitives or bind business store directly (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "状态原语来源核验（kbd）：`logic.rs` 仅通过 `pub use ui_state_primitives::kbd::{...}` 消费 `KbdSize/KbdState/KbdStateInput/resolve_state` 并做装配映射",
        "组件未引入业务 store 类型，也未在组件层重写状态原语实现",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include state-primitive-source evidence `{required}`."
        );
    }
}

#[test]
fn kbd_has_no_async_interaction_contract_and_marks_na_with_reason() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "is_loading",
        "on_retry",
        "aria-busy",
        "use_async_action(",
        "async fn ",
        "spawn_local(",
        "tokio::spawn(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not introduce async loading/retry protocol (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "N/A（kbd）：该组件仅做静态按键标签渲染，无远程请求与异步状态轴",
        "渲染层不挂载 `aria-busy`",
        "禁止在组件内自造加载/错误协议",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include async N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_dx_paradox_keeps_simple_api_and_docs_minimal_example() {
    let view = load_source("view");
    let readme = load_source("readme");
    let docs_display = load_source("docs_display");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "#[prop(optional, into)] keys: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: Children",
    ] {
        assert!(
            view.contains(required),
            "kbd should keep a minimal default API surface (`{required}`)."
        );
    }

    for forbidden in [
        "state:",
        "state=",
        "KbdStateInput",
        "KbdState",
        "ui_headless::",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd should not force internal state/headless wiring on basic usage (`{forbidden}`)."
        );
    }

    assert!(
        readme.contains("<Kbd keys=\"Ctrl\".to_string()>\"K\"</Kbd>"),
        "kbd readme should keep a one-line Hello World sample."
    );

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "<ComponentPage",
        "title=\"Kbd\"",
        "slug=\"kbd\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should provide minimal kbd docs entry (`{required}`)."
        );
    }

    for required in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "DX 核验（kbd）：基础调用为单行 `<Kbd keys=\"Ctrl\".to_string()>\"K\"</Kbd>`（≤ 5 行）",
        "公开 API 仅 `size/keys/class_name/children`",
        "不暴露内部 `state` 必填参数",
        "apps/docs-app/src/pages/components/pages/display.rs::kbd() 已提供最小可用示例与 Playground",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include DX-paradox evidence `{required}`."
        );
    }
}

#[test]
fn kbd_documentation_is_beginner_friendly_with_default_path_before_advanced_details() {
    let readme = load_source("readme");
    let docs_display = load_source("docs_display");
    let check2 = load_source("check2");

    for required in [
        "# Kbd",
        "## 快速开始（先用起来）",
        "### Hello World（最小可用）",
        "<Kbd keys=\"Ctrl\".to_string()>\"K\"</Kbd>",
        "### 常见用法",
        "默认 API 只要记住四个输入：`size`、`keys`、`class_name`、`children`。",
        "## 进阶用法（按需）",
        "## docs-app 入口（等价文档）",
        "Hello World (Default API)",
        "State Matrix (Size + Keys + Label-only)",
        "Controlled vs Uncontrolled (N/A)",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            readme.contains(required),
            "kbd readme should keep newcomer-oriented documentation marker `{required}`."
        );
    }

    let quick_start_index = readme
        .find("## 快速开始（先用起来）")
        .expect("kbd readme should contain quick-start section");
    let advanced_index = readme
        .find("## 进阶用法（按需）")
        .expect("kbd readme should contain advanced section");
    let architecture_index = readme
        .find("## 架构与边界（进阶阅读）")
        .expect("kbd readme should contain architecture section");
    assert!(
        quick_start_index < advanced_index && advanced_index < architecture_index,
        "kbd docs should keep 'default first, advanced later' reading path."
    );

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should keep newcomer-friendly progressive kbd docs path `{required}`."
        );
    }

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "已核验（文档入口存在）：`components/kbd/src/README.md` 存在且与 `apps/docs-app/src/pages/components/pages/display.rs::kbd()` 形成等价入口",
        "已核验（零门槛示例）：README 以“快速开始（先用起来）”开场",
        "已核验（先默认后进阶）：README 先给默认 API 路径与常见用法，再进入“进阶用法（按需）”与“架构与边界（进阶阅读）”",
        "docs-app 同步提供 `Hello World -> State Matrix -> Controlled vs Uncontrolled (N/A) -> Workbench` 渐进路径。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include beginner-friendly docs evidence `{required}`."
        );
    }
}

#[test]
fn kbd_non_composite_api_marks_parent_item_rule_as_na() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for forbidden in [
        "KbdParent",
        "KbdItem",
        "ItemSpec",
        "labels:",
        "titles:",
        "panels:",
        "items:",
        "RegistrationContext",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd should not expose composite parent/item or parallel-slot contracts (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A（kbd）：`kbd` 为单节点展示组件（非容器型组合组件）",
        "不存在 `Parent/Item` 语义树或并行数组槽位输入（`labels + children`、`titles + panels`）",
        "禁止引入此类语法糖",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include non-composite N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_macro_micro_duality_rule_is_na_for_non_drag_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "requestAnimationFrame",
        "raf",
        "motion::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not carry drag-loop macro/micro state machine markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "N/A（kbd）：`kbd` 为静态按键标签展示组件，无拖拽/高频连续交互场景",
        "不存在 `Dragging` 本地循环、`Action::DragEnd` 收敛动作与逐帧回流链路",
        "未引入组件级 `motion.rs`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include macro/micro-duality N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_two_pass_rendering_rule_is_na_for_non_measured_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "ResizeObserver",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "NodeRef",
        "web_sys::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include geometry measurement two-pass pipeline markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A（kbd）：`kbd` 为静态按键标签展示组件，不依赖 DOM 几何测量与定位修正",
        "不存在 `Intent -> Measure -> Rectification` 双阶段回路",
        "不存在 `getBoundingClientRect/ResizeObserver` 测量路径与幂等收敛控制逻辑",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include two-pass-rendering N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_registration_protocol_rule_is_na_for_non_collection_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include dynamic collection registration protocol markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "N/A（kbd）：`kbd` 为单节点展示组件，不管理动态子项集合",
        "不存在 `RegistrationContext`、`Register/Unregister` 注册流与 `items_order` 导航序维护",
        "未使用 `HashSet` 迭代顺序驱动交互",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include registration-protocol N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_slot_projection_rule_is_na_for_non_container_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "ProjectionMode",
        "suspend_on_hidden",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include container slot-projection lifecycle markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "N/A（kbd）：`kbd` 为单节点展示组件，不承载容器投影策略",
        "不存在 `Lazy/KeepAlive/Eager` 投影模式与 `NotifyHidden` 生命周期通知链路",
        "无隐藏态轮询/动画副作用需要暂停",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include slot-projection N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_env_stream_rule_is_na_for_non_subscribing_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "on:resize",
        "debounce",
        "throttle",
        "ThemeChanged",
        "Action::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include env-stream sampling/action fan-out markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "N/A（kbd）：`kbd` 为静态按键标签展示组件，不订阅 `Resize/Theme/Intersection` 环境流",
        "不存在环境事件采样、防抖、`BreakpointChanged` 等高层 `Action` 回流链路",
        "无原始事件洪泛入口",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include env-stream N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_event_light_cone_rule_is_na_for_non_collection_batch_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "Table",
        "Grid",
        "batch",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include event-light-cone batch-collection markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "N/A（kbd）：`kbd` 为单节点展示组件，不涉及大型集合批量操作",
        "不存在 `Context Bus + Selector` 分发链路与 `SelectionState::All` 状态压缩建模",
        "不存在 O(N) 级向下 prop drilling 路径",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include event-light-cone N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_causality_bus_rule_is_na_for_non_derived_bus_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "Causality",
        "Bus",
        "broadcast",
        "subscriber",
        "dispatch",
        "event_bus",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not include causality-bus multi-hop chain markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "N/A（kbd）：`kbd` 为静态按键标签展示组件，不存在复杂派生命令总线",
        "未引入 `TraceId` 透传链路",
        "不存在“触发 -> 派生 -> 广播 -> 订阅者”多跳因果路径",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include causality-bus N/A evidence `{required}`."
        );
    }
}

#[test]
fn kbd_a11y_i18n_l10n_contract_is_present_without_hardcoded_copy() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "<kbd",
        "data-slot=\"kbd\"",
        "keys",
        "children()",
        "data-slot=\"kbd-label\"",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep native semantics and prop-driven visible text marker `{required}`."
        );
    }

    for forbidden in [
        "role=\"button\"",
        "aria-label=\"",
        "\"Ctrl\"",
        "\"Enter\"",
        "\"Esc\"",
        "use ui_headless::a11y",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd should not hardcode business copy or interactive-role a11y overrides (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "已核验（kbd）：组件根节点使用原生语义元素 `<kbd>`",
        "用户可见文本仅来源于 `keys` 与 `children` 输入，无硬编码业务文案",
        "`lang/dir` 由宿主上下文继承消费",
        "组件层未引入平行 A11y 工具实现",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include a11y/i18n/l10n evidence `{required}`."
        );
    }
}

#[test]
fn kbd_state_markers_are_observable_queryable_and_enumerable() {
    let primitive = load_source("primitive_kbd");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd should expose stable observable/queryable state marker `{required}`."
        );
    }

    for required in [
        "pub fn as_attr(self) -> &'static str {",
        "KbdSize::Sm => \"sm\"",
        "KbdSize::Md => \"md\"",
        "(\"ui-kbd--state-with-keys\", \"with-keys\")",
        "(\"ui-kbd--state-label-only\", \"label-only\")",
    ] {
        assert!(
            primitive.contains(required),
            "kbd primitive should keep closed enumerable marker mapping `{required}`."
        );
    }

    for required in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "已核验（kbd）：根节点稳定输出 `data-slot/data-size/data-state/data-keys/data-custom-class`",
        "`data-size`（`sm|md`）与 `data-state`（`with-keys|label-only`）均来自 primitive 封闭集合映射",
        "状态来源通过 `data-custom-class`（是否外部 class）与 `data-keys`（是否存在 keys 输入）区分",
        "`kbd` 为非交互展示元素，`aria-*` 状态轴在本组件按 N/A 适用",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include observable-state marker evidence `{required}`."
        );
    }
}

#[test]
fn kbd_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        ".ui-kbd--size-sm,",
        ".ui-kbd[data-size=\"sm\"]",
        ".ui-kbd--size-md,",
        ".ui-kbd[data-size=\"md\"]",
        ".ui-kbd--state-with-keys,",
        ".ui-kbd[data-state=\"with-keys\"]",
        ".ui-kbd[data-keys=\"true\"]",
        ".ui-kbd--state-label-only,",
        ".ui-kbd[data-state=\"label-only\"]",
        ".ui-kbd--custom-class,",
        ".ui-kbd[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "kbd styles should branch via explicit state marker `{required}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        "> .ui-kbd__",
        ".ui-kbd .ui-kbd__",
    ] {
        assert!(
            !styles.contains(forbidden),
            "kbd styles should not infer state from fragile DOM structure (`{forbidden}`)."
        );
    }

    for required in [
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd view should expose state markers for visual switching via `{required}`."
        );
    }

    for forbidden in ["style=", "style:top", "style:left", "style:display"] {
        assert!(
            !view.contains(forbidden),
            "kbd view should not embed business style logic inline (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "已核验（kbd）：`styles.rs` 状态分支仅使用稳定 class 与语义标记",
        "未使用 `:nth-child`/深层级结构猜测",
        "`view.rs` 未注入业务 inline style",
        "视觉切换由 `data-state/data-keys/data-custom-class` 直接解释",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include explicit-style-state evidence `{required}`."
        );
    }
}

#[test]
fn kbd_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_inline_style_rules() {
    let css = load_source("ui_components_css");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-kbd\")]",
        "out.push_str(crate::kbd::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css.contains(required),
            "ui-components css aggregation should keep cascade-layer contract marker `{required}`."
        );
    }

    for forbidden in [
        "style=",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view should reject ordinary inline style rule injection (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "已核验（聚合层）：`crates/ui-components/src/css.rs::push_components_css` 使用 `out.push_str(\"\\n@layer ui {\\n\")`",
        "已核验（运行时样式边界）：`components/kbd/src/view.rs` 不含 `style=`/`style:top`/`style:left` 等普通内联样式写法",
        "N/A（kbd，运行时数值注入）：`kbd` 为静态展示组件",
        "回归约束：若后续引入运行时动态样式，仅允许 `style:--ui-*` 自定义变量注入",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include cascade-layer evidence `{required}`."
        );
    }
}

#[test]
fn kbd_ui_components_entrypoints_are_wired_and_forbidden_component_side_files_absent() {
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let ui_visual_active_highlight = load_source("ui_visual_active_highlight");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(feature = \"component-kbd\")]",
        "pub use ui_kbd as kbd;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib entry should keep required export marker `{required}`."
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-kbd\")]",
        "out.push_str(crate::kbd::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css entry should keep required aggregation marker `{required}`."
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot entry should keep centralized injection/i18n marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            ui_visual_active_highlight.contains(required),
            "active_highlight shared primitive should keep generic capability marker `{required}`."
        );
    }

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ui_components_overlay_open =
        workspace_root.join("crates/ui-components/src/overlay_open.rs");
    let ui_components_presence = workspace_root.join("crates/ui-components/src/presence.rs");
    let ui_components_a11y = workspace_root.join("crates/ui-components/src/a11y.rs");
    let ui_headless_controllable_state =
        workspace_root.join("crates/ui-headless/src/controllable_state.rs");
    let ui_headless_presence = workspace_root.join("crates/ui-headless/src/presence.rs");
    let ui_headless_a11y = workspace_root.join("crates/ui-headless/src/a11y.rs");

    assert!(
        !ui_components_overlay_open.exists()
            && !ui_components_presence.exists()
            && !ui_components_a11y.exists(),
        "ui-components forbidden entrypoint files should stay absent (overlay_open/presence/a11y)."
    );
    assert!(
        ui_headless_controllable_state.exists()
            && ui_headless_presence.exists()
            && ui_headless_a11y.exists(),
        "headless canonical entrypoint files should exist (controllable_state/presence/a11y)."
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "已核验（`lib.rs` 入口与导出面）：`crates/ui-components/src/lib.rs` 通过 `#[cfg(feature = \"component-kbd\")] pub use ui_kbd as kbd;`",
        "已核验（`css.rs` 聚合入口）：`crates/ui-components/src/css.rs::push_components_css` 负责组件 CSS 聚合",
        "已核验（`root.rs` 注入职责）：`crates/ui-components/src/root.rs::UiRoot` 集中注入 `base css + theme vars + (optional) components css`",
        "已核验（`active_highlight.rs` 落点）：`crates/ui-visual-primitive/src/active_highlight.rs` 提供共享高亮样式与 motion driver",
        "已核验（禁止文件不存在）：`crates/ui-components/src/overlay_open.rs`、`crates/ui-components/src/presence.rs`、`crates/ui-components/src/a11y.rs` 均不存在",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include ui-components entrypoint evidence `{required}`."
        );
    }
}

#[test]
fn kbd_semantics_contract_tests_exist_and_do_not_depend_on_visual_snapshots() {
    let semantics = load_source("semantics");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "fn kbd_a11y_i18n_l10n_contract_is_present_without_hardcoded_copy()",
        "fn kbd_state_markers_are_observable_queryable_and_enumerable()",
        "fn kbd_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn kbd_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet()",
    ] {
        assert!(
            semantics.contains(required),
            "kbd semantics suite should include contract-oriented test `{required}`."
        );
    }

    for forbidden in [
        "insta::assert_snapshot!",
        "insta::assert_debug_snapshot!",
        "insta::assert_yaml_snapshot!",
        "assert_snapshot!(",
    ] {
        assert!(
            !semantics.contains(forbidden),
            "kbd semantics validation should not be replaced by visual snapshot assertion (`{forbidden}`)."
        );
    }

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "on:pointerup",
        "disabled=",
        "aria-disabled",
        "aria-busy",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd non-interactive contract should keep disabled/keyboard/pointer paths N/A (`{forbidden}`)."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd contract should avoid platform-forked semantics for SSR/wasm (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "已核验（kbd）：语义断言由 `components/kbd/test/semantics.rs` 提供",
        "当前无视觉快照断言替代语义断言",
        "受控/非受控` 轴在 `kbd_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet` 以 N/A 约束",
        "`disabled/键盘/指针` 对 `kbd`（非交互展示组件）按 N/A",
        "SSR/wasm 语义一致性通过“无平台分支、无交互事件分支”的源码契约断言保障",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include semantic-contract testing evidence `{required}`."
        );
    }
}

#[test]
fn kbd_semantics_priority_rule_is_checked_with_contract_focused_regressions() {
    let semantics = load_source("semantics");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "fn kbd_a11y_i18n_l10n_contract_is_present_without_hardcoded_copy()",
        "fn kbd_state_markers_are_observable_queryable_and_enumerable()",
        "fn kbd_semantics_contract_tests_exist_and_do_not_depend_on_visual_snapshots()",
    ] {
        assert!(
            semantics.contains(required),
            "kbd semantics suite should keep contract-first regression anchor `{required}`."
        );
    }

    for required in [
        "<kbd",
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd semantics priority should bind to machine-readable marker `{required}`."
        );
    }

    for forbidden in [
        "insta::assert_snapshot!",
        "insta::assert_debug_snapshot!",
        "insta::assert_yaml_snapshot!",
        "assert_snapshot!(",
    ] {
        assert!(
            !semantics.contains(forbidden),
            "kbd semantics priority should not regress to visual snapshot assertions (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "已核验（语义测试落点）：`components/kbd/test/semantics.rs` 持续覆盖 `data-*` 状态来源",
        "已核验（契约聚焦）：断言聚焦 `data-slot/data-size/data-state/data-keys/data-custom-class` 与 `<kbd>` 原生语义路径",
        "已核验（非视觉快照替代）：语义套件禁止 `insta::assert_*snapshot` 与 `assert_snapshot!`",
        "回归约束：后续新增/变更语义字段（`data-*`/`aria-*`/source markers）必须先更新 `components/kbd/test/semantics.rs` 再更新清单，未补测试不得勾选。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include semantics-priority evidence `{required}`."
        );
    }
}

#[test]
fn kbd_e2e_selectors_use_semantic_markers_and_wasm_ready_settled_waits() {
    let e2e_kbd_contract = load_source("e2e_kbd_contract");
    let check2 = load_source("check2");

    for required in [
        "await page.goto(\"/#/components/kbd\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "const docsRoot = page.locator('[data-component=\"kbd\"]').first();",
        "const settledKbd = docsRoot.locator('[data-slot=\"kbd\"][data-size][data-state]').first();",
        "[data-slot=\"kbd\"][data-size=\"md\"][data-state=\"with-keys\"][data-keys=\"true\"]",
        "[data-slot=\"kbd\"][data-size=\"md\"][data-state=\"label-only\"]",
        "[data-slot=\"kbd\"][data-size=\"sm\"][data-state=\"with-keys\"][data-keys=\"true\"][data-custom-class=\"true\"]",
        "filter({ has: docsRoot.locator('[data-slot=\"kbd-workbench-controls\"]') })",
        "await expect(preview).toHaveAttribute(\"data-state\", \"label-only\");",
        "await expect(preview).toHaveAttribute(\"data-custom-class\", \"true\");",
    ] {
        assert!(
            e2e_kbd_contract.contains(required),
            "kbd e2e contract should keep semantic-selector and settled-wait marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "page.waitForTimeout(",
        "setTimeout(",
        "sleep(",
    ] {
        assert!(
            !e2e_kbd_contract.contains(forbidden),
            "kbd e2e contract should avoid fixed-sleep waits (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "已核验（kbd E2E 语义选择器）：新增 `e2e/tests/docs_app_kbd_contract.spec.mjs`",
        "已核验（WASM 稳定等待）：测试统一使用 `await page.locator(\"body:not(:has(#boot))\").waitFor()` 作为 wasm-ready 断点",
        "并以 `toHaveAttribute(data-*)` 作为 settled 条件；未使用固定 sleep。",
        "N/A（kbd 异步/动画 ready/settled）：`kbd` 为静态展示组件，无异步请求与组件动画路径（无 `attach_motion`）",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include e2e-selector-stability evidence `{required}`."
        );
    }
}

#[test]
fn kbd_critical_flow_is_in_repeatable_e2e_suite_with_semantic_breakpoints() {
    let e2e_kbd_contract = load_source("e2e_kbd_contract");
    let check2 = load_source("check2");

    for required in [
        "async function runKbdWorkbenchFlow(docsRoot) {",
        "test(\"docs-app kbd workbench flow is repeatable with semantic breakpoints\"",
        "await runKbdWorkbenchFlow(docsRoot);",
        "await page.reload();",
        "await runKbdWorkbenchFlow(reloadedRoot);",
        "await expect(preview).toHaveAttribute(\"data-size\", \"sm\");",
        "await expect(preview).toHaveAttribute(\"data-state\", \"label-only\");",
        "await expect(preview).toHaveAttribute(\"data-custom-class\", \"true\");",
        "await expect(preview).toHaveAttribute(\"data-keys\", \"true\");",
    ] {
        assert!(
            e2e_kbd_contract.contains(required),
            "kbd critical-flow e2e suite should keep repeatable semantic breakpoint marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "page.waitForTimeout(",
        "setTimeout(",
        "sleep(",
    ] {
        assert!(
            !e2e_kbd_contract.contains(forbidden),
            "kbd critical-flow e2e should avoid fixed sleeps (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "已核验（kbd 关键流程）：`e2e/tests/docs_app_kbd_contract.spec.mjs` 新增可重复流程 `docs-app kbd workbench flow is repeatable with semantic breakpoints`",
        "并复用同一 `runKbdWorkbenchFlow` 流程函数。",
        "已核验（可定位断点）：流程断言均落在语义契约标记",
        "N/A（kbd 高风险路径优先级）：`kbd` 非 overlay/async 组件",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include repeatable-critical-flow evidence `{required}`."
        );
    }
}

#[test]
fn kbd_docs_examples_parameter_matrix_and_state_matrix_are_synced_with_logic_defaults() {
    let docs_display = load_source("docs_display");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"kbd-state-matrix\"",
        "data-slot=\"kbd-state-rows\"",
        "\"data-size / data-state\"",
        "\"control mode\"",
        "\"disabled axis\"",
        "data-slot=\"kbd-parameter-matrix\"",
        "data-slot=\"kbd-parameter-rows\"",
        "\"size: Option&lt;KbdSize&gt;\"",
        "\"keys: Option&lt;String&gt;\"",
        "\"class_name: Option&lt;String&gt;\"",
        "\"children: Children\"",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs page should keep synced docs/example/matrix marker `{required}`."
        );
    }

    for required in [
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize {",
        "value.unwrap_or_default()",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String> {",
        "let trimmed = value.trim();",
        "(!trimmed.is_empty()).then(|| trimmed.into())",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic should keep documented defaults/normalization marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "#[prop(optional, into)] keys: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: Children",
    ] {
        assert!(
            view.contains(required),
            "kbd view api should stay aligned with documented parameter names `{required}`."
        );
    }

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "已核验（docs 页面同步）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 已包含 `Hello World`、`State Matrix (Size + Keys + Label-only)`、`Controlled vs Uncontrolled (N/A)`、`Workbench` 等示例路径，与当前组件行为一致。",
        "已核验（状态矩阵）：新增 `data-slot=\"kbd-state-matrix\"` 区块",
        "已核验（参数矩阵与默认值）：新增 `data-slot=\"kbd-parameter-matrix\"` 区块",
        "参数名与 `logic.rs` 一致（`size/keys/class_name/children`）",
        "`size=None -> Md`（`normalize_size -> unwrap_or_default()`）",
        "`keys/class_name` 空白裁剪为 `None`（`normalize_optional_text`）。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include docs-sync evidence `{required}`."
        );
    }
}

#[test]
fn kbd_component_file_responsibilities_are_enforced_with_motion_na() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::KbdSize;",
        "pub use view::Kbd;",
    ] {
        assert!(
            module.contains(required),
            "kbd mod boundary should keep minimal export surface `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub struct KbdLogicInput",
        "pub const CSS: &str",
        "#[component]",
    ] {
        assert!(
            !module.contains(forbidden),
            "kbd mod.rs should not host implementation details (`{forbidden}`)."
        );
    }

    for required in [
        "pub struct KbdLogicInput",
        "pub struct KbdViewModel",
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn compose_class_name(base_class_name: Option<String>, state: KbdState) -> String",
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic.rs should keep normalization/derivation responsibilities via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "<kbd",
        "data-slot=",
        "var(--ui-",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !logic.contains(forbidden),
            "kbd logic.rs should not contain view/style/DOM details (`{forbidden}`)."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-kbd",
        "var(--ui-",
        ".ui-kbd[data-size=\"sm\"]",
        ".ui-kbd[data-state=\"with-keys\"]",
    ] {
        assert!(
            styles.contains(required),
            "kbd styles.rs should keep static token-first css contract `{required}`."
        );
    }

    for forbidden in [
        "pub fn ",
        "#[component]",
        "view! {",
        "<kbd",
        "children()",
        "on:click",
    ] {
        assert!(
            !styles.contains(forbidden),
            "kbd styles.rs should not contain runtime/view logic (`{forbidden}`)."
        );
    }

    for required in [
        "#[component]",
        "pub fn Kbd(",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
        "<kbd",
        "data-slot=\"kbd\"",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            view.contains(required),
            "kbd view.rs should only render structure and mount semantics `{required}`."
        );
    }

    for forbidden in [
        "resolve_state(KbdStateInput",
        "normalize_size(",
        "pub const CSS: &str",
        "attach_motion",
        "stiffness",
        "damping",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view.rs should not own primitive internals/css/motion engine logic (`{forbidden}`)."
        );
    }

    for forbidden in ["attach_motion", "stiffness", "damping", "spring("] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !styles.contains(forbidden)
                && !view.contains(forbidden),
            "kbd static component should keep motion.rs path as N/A (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "已核验（kbd）：`mod.rs` 仅保留最小导出边界",
        "`logic.rs` 仅做 `size/keys/class_name` 归一、状态派生与来源标记装配",
        "`styles.rs` 仅提供 token-first 静态 CSS（`var(--ui-*)`）",
        "`view.rs` 仅渲染 `<kbd>` 结构并挂载语义标记",
        "`motion.rs` N/A（kbd）：组件无 open/close/enter/exit 等动效语义状态轴",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include file-responsibility evidence `{required}`."
        );
    }
}

#[test]
fn kbd_component_directory_entry_files_are_present_with_correct_responsibility_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mod_path = src_dir.join("mod.rs");
    let logic_path = src_dir.join("logic.rs");
    let styles_path = src_dir.join("styles.rs");
    let view_path = src_dir.join("view.rs");
    let motion_path = src_dir.join("motion.rs");
    let spec_path = src_dir.join("spec.rs");
    let render_path = src_dir.join("render.rs");

    assert!(
        mod_path.exists() && logic_path.exists() && styles_path.exists() && view_path.exists(),
        "kbd component directory should keep required entry files (mod/logic/styles/view)."
    );
    assert!(
        !render_path.exists(),
        "kbd component directory should not drift to render.rs."
    );
    assert!(
        !motion_path.exists() && !spec_path.exists(),
        "kbd should keep motion.rs/spec.rs absent in current static/simple scope."
    );

    for required in [
        "pub use logic::KbdSize;",
        "pub use view::Kbd;",
        "mod logic;",
        "mod view;",
        "pub mod styles;",
    ] {
        assert!(
            module.contains(required),
            "kbd mod.rs should keep minimal stable export boundary marker `{required}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use logic::KbdViewModel",
    ] {
        assert!(
            !module.contains(forbidden),
            "kbd mod.rs should avoid over-exporting internals (`{forbidden}`)."
        );
    }

    for required in [
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic.rs should keep normalization/derivation ownership marker `{required}`."
        );
    }
    for forbidden in [
        "view! {",
        "<kbd",
        "web_sys::",
        "wasm_bindgen::",
        "pub const CSS",
    ] {
        assert!(
            !logic.contains(forbidden),
            "kbd logic.rs should not mix render/platform/style concern (`{forbidden}`)."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-kbd[data-state=\"with-keys\"]",
    ] {
        assert!(
            styles.contains(required),
            "kbd styles.rs should keep static token css marker `{required}`."
        );
    }
    for forbidden in ["pub fn ", "#[component]", "view! {", "<kbd", "on:click"] {
        assert!(
            !styles.contains(forbidden),
            "kbd styles.rs should not host runtime or render logic (`{forbidden}`)."
        );
    }

    for required in [
        "#[component]",
        "pub fn Kbd(",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
        "data-slot=\"kbd\"",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            view.contains(required),
            "kbd view.rs should keep structure + semantic mount marker `{required}`."
        );
    }
    for forbidden in [
        "resolve_state(KbdStateInput",
        "pub const CSS: &str",
        "attach_motion",
        "stiffness",
        "damping",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view.rs should not host primitive/style/motion decision logic (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "已核验（kbd）：`components/kbd/src` 存在 `mod.rs`、`logic.rs`、`styles.rs`、`view.rs`；不存在 `render.rs` 漂移文件。",
        "已核验（`mod.rs` 导出边界）：仅保留 `pub use logic::KbdSize;` 与 `pub use view::Kbd;` 最小对外面；未暴露 `logic/view` 内部实现模块为公共 API。",
        "已核验（`logic.rs` 职责）：仅承载 props 归一、状态派生、来源标记（`resolve_view_model`），不含 DOM/render/CSS 细节与可下沉原语重实现。",
        "已核验（`styles.rs` 职责）：仅承载静态 token-first CSS（`var(--ui-*)` + `var(--ui-fallback-*)`），无运行时逻辑与主题常量硬编码。",
        "已核验（`view.rs` 职责）：仅做 Leptos 结构渲染与语义标记挂载（`data-slot/data-size/data-state`），关键状态决策仍集中在 `logic.rs`。",
        "N/A（`motion.rs`/`spec.rs`，kbd）：`kbd` 为静态展示型简单组件，无独立动效语义轴与复杂外部 Schema 固化需求；`motion.rs` 与 `spec.rs` 当前不引入。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include component-directory entrypoint evidence `{required}`."
        );
    }
}

#[test]
fn kbd_file_placement_discipline_is_enforced_for_component_directory_layout() {
    let check2 = load_source("check2");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "protocol.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "kbd source directory should keep required layout file `{required}`."
        );
    }

    for forbidden in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "kbd source directory should not include `{forbidden}` in current static/simple scope."
        );
    }

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "已核验（kbd）：`components/kbd/src` 当前稳定落点为 `mod.rs`、`logic.rs`、`styles.rs`、`view.rs` + `protocol.rs`（内部版本化协议），且不存在 `render.rs` 漂移文件。",
        "N/A（`motion.rs`，kbd）：组件无独立动效语义轴，暂不引入 `motion.rs`",
        "N/A（`spec.rs`，kbd）：`kbd` 非复杂配置组件，暂无外部 Schema 固化需求；复杂组件才引入 `spec.rs`",
        "回归约束：目录结构新增/迁移必须先过语义测试（文件存在性 + 职责边界），禁止以 `render.rs` 或临时聚合文件绕过职责分层。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include file-placement discipline evidence `{required}`."
        );
    }
}

#[test]
fn kbd_spec_rs_is_not_introduced_for_simple_component_contract() {
    let module = load_source("mod");
    let check2 = load_source("check2");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "kbd should not add `spec.rs` for simple static component without external schema-solidification needs."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "kbd module boundary should not expose spec builder surface (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "已核验（kbd）：`components/kbd/src` 不存在 `spec.rs`",
        "`mod.rs` 未引入 `mod spec`/`pub mod spec`",
        "`kbd` 为静态展示型简单组件，不存在稳定外部 Schema 固化需求",
        "`components/kbd/src/protocol.rs` 仅承载内部最小版本化协议类型",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include spec.rs minimality evidence `{required}`."
        );
    }
}

#[test]
fn kbd_hyper_structure_builder_rule_is_na_for_simple_component_and_builder_surface_absent() {
    let module = load_source("mod");
    let protocol = load_source("protocol");
    let check2 = load_source("check2");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "kbd should keep `spec.rs` absent until it becomes a complex configurable component."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "KbdSpec::new()",
    ] {
        assert!(
            !module.contains(forbidden),
            "kbd module boundary should not expose Hyper-Structure builder surface (`{forbidden}`)."
        );
    }

    for forbidden in ["KbdSpec::new()", ".render()", "pub fn render("] {
        assert!(
            !protocol.contains(forbidden),
            "kbd protocol boundary should not leak public builder chain markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A（kbd）：`kbd` 为静态展示型简单组件，不属于复杂配置组件；当前不引入 `spec.rs` 建造者入口。",
        "已核验（kbd）：`components/kbd/src` 不存在 `spec.rs`",
        "公共 API 仅 `Kbd` 与 `KbdSize`。",
        "已核验（协议边界）：`components/kbd/src/protocol.rs` 仅承载内部最小版本化协议",
        "未暴露 `*Spec::new()...render()` 公共建造链。",
        "回归约束：若后续 `kbd` 演进为复杂配置组件，必须新增 `spec.rs` 并提供 `KbdSpec::new()...render()`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include Hyper-Structure builder evidence `{required}`."
        );
    }
}

#[test]
fn kbd_context_compression_manifest_and_rbi_are_present_and_kept_in_sync() {
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let check2 = load_source("check2");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert!(
        src_dir.join("Component.toml").exists() && src_dir.join("kbd.rbi").exists(),
        "kbd component source should include context-compression files (Component.toml + kbd.rbi)."
    );

    for required in [
        "schema_version = \"1\"",
        "id = \"ui-kbd\"",
        "name = \"Kbd\"",
        "crate = \"ui-kbd\"",
        "rbi = \"kbd.rbi\"",
        "mod_rs = \"mod.rs\"",
        "logic_rs = \"logic.rs\"",
        "styles_rs = \"styles.rs\"",
        "view_rs = \"view.rs\"",
        "snapshot = true",
        "streaming = false",
        "spec_builder = false",
        "motion_runtime = false",
        "\"data-slot\"",
        "\"data-size\"",
        "\"data-state\"",
        "\"data-keys\"",
        "\"data-custom-class\"",
    ] {
        assert!(
            manifest.contains(required),
            "kbd Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub type KbdSize = ui_state_primitives::kbd::KbdSize;",
        "pub mod styles {",
        "pub const CSS: &str;",
        "pub fn Kbd(",
        "size: Option<KbdSize>",
        "keys: Option<String>",
        "class_name: Option<String>",
        "children: leptos::children::Children,",
    ] {
        assert!(
            rbi.contains(required),
            "kbd RBI projection should keep stable public-signature marker `{required}`."
        );
    }

    for forbidden in [
        "pub struct KbdLogicInput",
        "pub struct KbdViewModel",
        "KbdComponentSpec",
    ] {
        assert!(
            !rbi.contains(forbidden),
            "kbd RBI should not expose internal logic/protocol surface (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "新增 `components/kbd/src/Component.toml`（能力清单）与 `components/kbd/src/kbd.rbi`（接口签名投影）",
        "`snapshot=true`、`streaming=false`、`spec_builder=false`、`motion_runtime=false`",
        "声明稳定语义标记集合 `data-slot/data-size/data-state/data-keys/data-custom-class`",
        "`kbd.rbi` 仅投影稳定公共接口签名，不暴露 `logic/protocol` 内部结构",
        "后续变更 `Kbd` props 或语义标记集合时，必须同步更新 `Component.toml + kbd.rbi`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include context-compression evidence `{required}`."
        );
    }
}

#[test]
fn kbd_agent_contract_schema_is_machine_readable_and_whitelist_guarded() {
    let manifest = load_source("component_manifest");
    let view = load_source("view");
    let logic = load_source("logic");
    let primitive = load_source("primitive_kbd");
    let check2 = load_source("check2");

    for required in [
        "[agent_contract]",
        "schema = \"ui.kbd.agent-contract/v1\"",
        "fields = [\"intent\", \"action\", \"state\", \"source\"]",
        "intent = \"display\"",
        "action = \"snapshot_render\"",
    ] {
        assert!(
            manifest.contains(required),
            "kbd manifest should provide typed Agent Contract schema marker `{required}`."
        );
    }

    for required in [
        "\"data-slot\"",
        "\"data-size\"",
        "\"data-state\"",
        "\"data-keys\"",
        "\"data-custom-class\"",
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            manifest.contains(required) || view.contains(required),
            "kbd should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::kbd::{KbdSize, KbdState, KbdStateInput, resolve_state};",
        "let state = resolve_state(KbdStateInput {",
        "pub enum KbdSize",
        "pub struct KbdState",
        "pub fn resolve_state(input: KbdStateInput) -> KbdState",
        "size_attr: input.size.as_attr()",
        "state_attr,",
    ] {
        assert!(
            logic.contains(required) || primitive.contains(required),
            "kbd should keep typed state->marker derivation path `{required}`."
        );
    }

    for forbidden in ["format!(\"data-", "format!(\"{}-{}\""] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not build semantic contracts from ad-hoc string concatenation (`{forbidden}`)."
        );
    }

    for required in [
        "[[agent_contract_whitelist]]",
        "allowed = [\"logic::resolve_view_model\", \"view::Kbd\", \"view::render_keys_slot\"]",
        "blocked = [\"inner_html\", \"dangerously_set_inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest.contains(required),
            "kbd manifest should keep whitelist render boundary marker `{required}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "已核验（Schema 化入口）：`components/kbd/src/Component.toml` 提供 `[agent_contract]`",
        "`fields=[\"intent\",\"action\",\"state\",\"source\"]`",
        "已核验（机器可读语义）：`components/kbd/src/view.rs` 稳定输出 `data-slot/data-size/data-state/data-keys/data-custom-class`",
        "已核验（类型化来源）：状态字段来自 `logic.rs -> ui_state_primitives::kbd::resolve_state(KbdStateInput)` 的类型化派生",
        "已核验（白名单边界）：`Component.toml` 的 `[[agent_contract_whitelist]]` 显式限制渲染链路",
        "N/A（`data-ui-schema` 扩展字段）：`kbd` 为简单静态展示组件",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include Agent Contract schema evidence `{required}`."
        );
    }
}

#[test]
fn kbd_streaming_definition_is_explicit_and_component_is_snapshot_only() {
    let manifest = load_source("component_manifest");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "snapshot = true",
        "streaming = false",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
    ] {
        assert!(
            manifest.contains(required),
            "kbd manifest should keep explicit snapshot/streaming output contract marker `{required}`."
        );
    }

    for forbidden in [
        "project_streaming",
        "StreamingProjection",
        "is_complete(",
        "streaming_chunk",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd snapshot-only scope should not embed streaming-render pipeline (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "已核验（术语边界）：`Streaming` 与 `Snapshot` 仅描述 LLM 输出呈现时机",
        "已核验（kbd 当前模式）：`components/kbd/src/Component.toml` 声明 `snapshot=true`、`streaming=false`",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "若后续为 `kbd` 增加流式渲染能力，必须先在 `Component.toml` 将 `streaming` 显式置为 `true`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include streaming/snapshot term-definition evidence `{required}`."
        );
    }
}

#[test]
fn kbd_snapshot_is_baseline_capability_and_accepts_full_config_render() {
    let manifest = load_source("component_manifest");
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "snapshot = true",
        "streaming = false",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
    ] {
        assert!(
            manifest.contains(required),
            "kbd manifest should keep snapshot baseline capability marker `{required}`."
        );
    }

    for required in [
        "pub fn Kbd(",
        "#[prop(optional)] size: Option<KbdSize>",
        "#[prop(optional, into)] keys: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: Children,",
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
    ] {
        assert!(
            view.contains(required),
            "kbd view should accept full config input and normalize through logic (`{required}`)."
        );
    }

    for required in [
        "pub struct KbdLogicInput",
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "let size = normalize_size(input.size);",
        "let keys = normalize_optional_text(input.keys);",
        "let class_name = normalize_optional_text(input.class_name);",
        "let state = resolve_state(KbdStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic should consume full config via centralized normalization marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd view should render stable snapshot semantic output marker `{required}`."
        );
    }

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "已核验（组件能力声明）：`components/kbd/src/Component.toml` 显式声明 `snapshot=true`、`streaming=false`",
        "已核验（完整配置消费）：`components/kbd/src/view.rs::Kbd` 接收完整 props 组合",
        "并统一交给 `logic::resolve_view_model(KbdLogicInput { ... })` 归一后渲染",
        "已核验（稳定输出）：归一后固定输出 `<kbd>` 结构与语义标记",
        "回归约束：若后续调整组件输入轴或渲染链路，必须保持 `snapshot=true` 默认能力",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include snapshot-baseline evidence `{required}`."
        );
    }
}

#[test]
fn kbd_streaming_requirement_is_role_based_with_snapshot_fallback_and_upstream_ownership() {
    let manifest = load_source("component_manifest");
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "snapshot = true",
        "streaming = false",
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
        "[output_state]",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
    ] {
        assert!(
            manifest.contains(required),
            "kbd manifest should keep role-based streaming policy marker `{required}`."
        );
    }

    for required in [
        "<kbd",
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep continuous readable semantic markers in snapshot path (`{required}`)."
        );
    }

    for forbidden in [
        "project_streaming",
        "StreamingProjection",
        "is_complete(",
        "retry",
        "backoff",
        "reconnect",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not own streaming/retry/recovery responsibilities (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "已核验（职责判定）：`kbd` 为静态按键展示组件，不是正文阅读面，因此不属于 `Streaming Required`。",
        "已核验（Streaming Optional 落地）：`components/kbd/src/Component.toml` 显式声明 `snapshot=true`、`streaming=false`",
        "并通过 `[streaming_policy] required=false, fallback=\"snapshot\"` 固化“仅消费 Snapshot”的策略。",
        "已核验（状态连续可读）：`Component.toml` 的 `[output_state]` 显式声明 `default=\"verified\"`",
        "`view.rs` 稳定输出 `data-slot/data-size/data-state/data-keys/data-custom-class`",
        "已核验（上层职责边界）：`streaming_policy.owner=\"upstream\"`",
        "组件内未实现数据校验/断线恢复/重试逻辑",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include streaming-required/optional role evidence `{required}`."
        );
    }
}

#[test]
fn kbd_rust_hygiene_contract_forbids_unwrap_expect_and_tracks_cow_string_path() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let protocol = load_source("protocol");
    let check2 = load_source("check2");

    for forbidden in ["unwrap(", "expect(", "let _ ="] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !protocol.contains(forbidden),
            "kbd non-test source should keep rust hygiene ban marker (`{forbidden}`)."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-kbd\")",
        "Cow::Borrowed(\"ui-kbd--custom-class\")",
        "Cow::Owned(base_class_name)",
        "String::with_capacity(",
    ] {
        assert!(
            logic.contains(required),
            "kbd logic should keep Cow-based string-copy hygiene marker `{required}`."
        );
    }

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "已核验（kbd 非测试源码）：`components/kbd/src/{mod,logic,view,styles,protocol}.rs` 未出现 `unwrap()` / `expect()`",
        "已核验（字符串热点）：`components/kbd/src/logic.rs::compose_class_name` 已引入 `std::borrow::Cow<'static, str>`",
        "已执行：`./scripts/check-rust-hygiene.sh`；当前环境 `rg` 构建不支持 PCRE2",
        "`PCRE2 is not available in this build of ripgrep`",
        "后续新增非测试代码禁止引入 `unwrap/expect` 与裸 `let _ = ...`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include rust-hygiene evidence `{required}`."
        );
    }
}

#[test]
fn kbd_version_deprecation_migration_is_na_without_breaking_upgrade_and_has_future_boundary() {
    let protocol = load_source("protocol");
    let view = load_source("view");
    let rbi = load_source("component_rbi");
    let check2 = load_source("check2");

    for required in [
        "pub enum KbdComponentSchemaVersion",
        "V1",
        "pub struct KbdComponentSpec",
        "pub schema_version: KbdComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "kbd protocol should keep minimal versioned schema anchor `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "#[prop(optional, into)] keys: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: Children,",
        "pub fn Kbd(",
        "size: Option<KbdSize>",
        "keys: Option<String>",
        "class_name: Option<String>",
        "children: leptos::children::Children,",
    ] {
        assert!(
            view.contains(required) || rbi.contains(required),
            "kbd public API should stay stable and non-breaking marker `{required}`."
        );
    }

    for forbidden in [
        "schema_registry",
        "deprecation_start",
        "deprecation_end",
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "pub fn migrate_",
    ] {
        assert!(
            !protocol.contains(forbidden),
            "kbd protocol should not claim codemod/deprecation artifacts when no breaking upgrade is present (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A（kbd，本次改动）：当前提交未引入跨大版本 API 破坏升级",
        "已核验（协议现状）：`components/kbd/src/protocol.rs` 仅维护最小版本化协议",
        "未声明 `schema_registry` 弃用窗口，亦未出现 `migrate_v1_to_v2` 迁移函数。",
        "若后续发生跨大版本破坏性升级，必须同步补齐 `Schema Registry`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include version-deprecation migration governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_docs_copy_paste_ready_playgrounds_cover_required_paths_and_import_injection() {
    let docs_display = load_source("docs_display");
    let docs_playground = load_source("docs_playground");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "let hello_world_code = Signal::derive",
        "let state_matrix_code = Signal::derive",
        "let controlled_contrast_code = Signal::derive",
        "let stream_snapshot_code = Signal::derive",
        "let source_first_code = Signal::derive",
        "let kbd_imports = \"use leptos::prelude::*;\\nuse ui_components::{Kbd, KbdSize};\".to_string();",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=kbd_imports.clone()",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs page should keep copy-paste ready playground evidence `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
        "#[prop(optional, into)] code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            docs_playground.contains(required),
            "playground infra should keep import-injection copy path marker `{required}`."
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "已核验（kbd docs Playground）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 已提供 `Hello World (Default API)`、`State Matrix (Size + Keys + Label-only)`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)`。",
        "已核验（复制即运行）：上述 Playground 显式设置 `code_imports=kbd_imports`，并通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 在复制时自动补全缺失 imports。",
        "已核验（流式/快照展示）：`kbd` 页面以 `Streaming Optional / Snapshot` 明确静态展示组件路径，文档说明 `fallback=snapshot`，与 `components/kbd/src/Component.toml` 的 `streaming=false` 策略一致。",
        "N/A（运行时受控/非受控轴）：`Kbd` 无 `value/on_value_change/default_value` 内部状态轴；文档通过 `Controlled vs Uncontrolled (N/A)` 对照项显式标注边界，避免误导为可控状态机组件。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include docs copy-paste-ready evidence `{required}`."
        );
    }
}

#[test]
fn kbd_source_first_docs_are_copy_paste_ready_with_real_paths_and_prerequisites() {
    let docs_display = load_source("docs_display");
    let docs_playground = load_source("docs_playground");
    let check2 = load_source("check2");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=kbd_imports.clone()",
        "let source_first_code = Signal::derive",
        "let kbd_imports = \"use leptos::prelude::*;\\nuse ui_components::{Kbd, KbdSize};\".to_string();",
        "data-slot=\"kbd-source-first\"",
        "data-slot=\"kbd-source-prerequisites\"",
        "<code>\"component-kbd\"</code>",
        "<code>\"UiRoot\"</code>",
        "<code>\"inject-css\"</code>",
        "label=\"Copy Kbd starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-kbd-source-copy\".to_string()",
        "components/kbd/src/mod.rs",
        "components/kbd/src/logic.rs",
        "components/kbd/src/view.rs",
        "components/kbd/src/styles.rs",
        "size=KbdSize::Sm",
        "keys=\"Shift\".to_string()",
        "class_name=\"docs-kbd-custom\".to_string()",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs should keep source-first copy-paste-ready marker `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "#[prop(optional, into)] code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            docs_playground.contains(required),
            "playground infra should keep copy-ready import injection marker `{required}`."
        );
    }

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "已核验（一键复制 + 直接运行）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 提供 `Source-first Starter (Copy-Paste Ready)`",
        "并设置 `code_imports=kbd_imports`；复制链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐缺失 imports。",
        "已核验（真实源码落点 + 依赖前提）：`display.rs::kbd()` 新增 `data-slot=\"kbd-source-first\"` 区块",
        "`component-kbd` / `UiRoot + inject-css` 前提",
        "并列出 `components/kbd/src/{mod,logic,view,styles}.rs` 源码路径",
        "已核验（文档与实现同步）：`source_first_code`、`kbd_imports` 与 `Snippet(label=\"Copy Kbd starter\")` 共用当前 `Kbd` API（`size/keys/class_name`）示例",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include source-first copy-ready evidence `{required}`."
        );
    }
}

#[test]
fn kbd_heroui_alignment_docs_stay_synced_with_component_docs_entrypoints() {
    let docs_heroui_strategy = load_source("docs_heroui_strategy");
    let docs_pages_catalog = load_source("docs_pages_catalog");
    let docs_display = load_source("docs_display");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for required in [
        "### Kbd 同步记录（2026-02-20）",
        "参数主轴保持 `size/keys/class_name/children`",
        "component_doc!(\"Kbd\", \"kbd\", \"Display\", display::kbd)",
        "apps/docs-app/src/pages/components/pages/display.rs::kbd() 已覆盖 `Hello World (Default API)`",
        "Source-first / Copy-Paste Ready",
        "本轮仅为 Kbd 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论",
    ] {
        assert!(
            docs_heroui_strategy.contains(required),
            "heroui strategy doc should keep kbd sync marker `{required}`."
        );
    }

    for required in [
        "component_doc!(\"Kbd\", \"kbd\", \"Display\", display::kbd)",
        "pub(super) const CATALOG: &[ComponentDoc] = &[",
    ] {
        assert!(
            docs_pages_catalog.contains(required),
            "docs pages catalog should keep kbd indexable entry marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Kbd\"",
        "slug=\"kbd\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"kbd-source-first\"",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs page should keep parameter-model-synced entry marker `{required}`."
        );
    }

    for required in [
        "# Kbd",
        "## docs-app 入口（等价文档）",
        "apps/docs-app/src/pages/components/pages/display.rs",
    ] {
        assert!(
            readme.contains(required),
            "kbd readme should keep equivalent docs entry marker `{required}`."
        );
    }

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "已核验（对标策略文档同步）：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### Kbd 同步记录（2026-02-20）`",
        "已核验（组件文档入口可访问且可索引）：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"Kbd\", \"kbd\", \"Display\", display::kbd)` 暴露入口",
        "等价入口 `components/kbd/src/README.md` 已存在。",
        "已核验（实现与文档同频）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 的示例矩阵与 `Kbd` 当前 API（`size/keys/class_name`）保持一致",
        "N/A（研究文档追加）：本轮为参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include heroui-doc-sync evidence `{required}`."
        );
    }
}

#[test]
fn kbd_token_first_static_style_contract_is_enforced() {
    let styles = load_source("styles");
    let view = load_source("view");
    let css = load_source("ui_components_css");
    let root = load_source("ui_components_root");
    let check2 = load_source("check2");

    for required in [
        "pub const CSS: &str = r#\"",
        "background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
    ] {
        assert!(
            styles.contains(required),
            "kbd styles.rs should keep token-first static style contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-kbd\")]",
        "out.push_str(crate::kbd::styles::CSS);",
    ] {
        assert!(
            css.contains(required),
            "ui-components css aggregation should include kbd styles under feature gate `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root.contains(required),
            "UiRoot should inject aggregated component styles through documented path `{required}`."
        );
    }

    for forbidden in [
        "style=",
        "stylist",
        "emotion",
        "styled_components",
        "tailwind",
        "class=\"flex",
        "class=\"grid",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "kbd component layer should avoid utility-first/css-in-rust default patterns (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "已核验（kbd）：样式仅定义在 `components/kbd/src/styles.rs::CSS`",
        "`crates/ui-components/src/css.rs` 在 `component-kbd` feature 下聚合",
        "`UiRoot` 通过 `inject_components_css` 路径统一注入",
        "视觉值来源符合 token-first",
        "运行时未写业务 inline style",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include token-first static style contract evidence `{required}`."
        );
    }
}

#[test]
fn kbd_defensive_variable_contract_uses_two_level_fallback_without_hardcoded_terminal_values() {
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for required in [
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border));",
        "background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
    ] {
        assert!(
            styles.contains(required),
            "kbd styles should keep defensive variable fallback chain `{required}`."
        );
    }

    for forbidden in [
        "#fff",
        "#000",
        "16px",
        "12px",
        "20px",
        "24px",
        "border: 1px solid var(--ui-border);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "kbd styles should avoid hardcoded terminal style value (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "已核验（kbd）：`components/kbd/src/styles.rs` 的颜色/间距/圆角/边框/字体尺寸与行高均采用双层回退链",
        "尺寸终值已去裸常量化：`min-height` 改为基于 `--ui-component-height-100` 的比例计算",
        "已核验（kbd）：样式中不存在 Hex 颜色与 `12px/16px/20px/24px` 等裸终值",
        "回归约束：新增样式 token 必须遵循 `var(--ui-*, var(--ui-fallback-*))`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include defensive-variable evidence `{required}`."
        );
    }
}

#[test]
fn kbd_visual_desire_rule_is_tracked_with_component_evidence_and_repo_level_na() {
    let styles = load_source("styles");
    let docs_display = load_source("docs_display");
    let check2 = load_source("check2");

    for required in [
        "background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
    ] {
        assert!(
            styles.contains(required),
            "kbd default visual baseline should keep token-first aesthetic source `{required}`."
        );
    }

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Kbd\"",
        "title=\"State Matrix (Size + Keys + Label-only)\"",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should expose kbd default-theme visual baseline entry `{required}`."
        );
    }

    for required in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "已核验（kbd）：组件默认样式为 token-first 语义键帽视觉",
        "docs-app 在 `display.rs::kbd()` 提供默认主题下的矩阵与 Workbench 展示入口",
        "N/A（kbd，仓库级治理）：`Button/Input/Overlay` 的截图基线与 HeroUI 级视觉回归属于跨组件质量门禁",
        "按第 0 节升级为仓库级任务跟踪",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should keep visual-desire evidence/N.A. governance marker `{required}`."
        );
    }
}

#[test]
fn kbd_tree_shaking_contract_is_feature_gated_and_not_forced_by_web_demo_defaults() {
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let web_demo_cargo = load_source("web_demo_cargo");
    let check2 = load_source("check2");

    for required in [
        "component-kbd = [\"dep:ui-kbd\"]",
        "all-components = [",
        "default = [\"inject-css\", \"all-components\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components feature map should expose expected tree-shaking anchors `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-kbd\")]",
        "pub use ui_kbd as kbd;",
        "#[cfg(feature = \"inject-css\")]",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib export surface should keep feature-gated kbd path `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-kbd\")]",
        "out.push_str(crate::kbd::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css aggregation should gate kbd css by component feature `{required}`."
        );
    }

    let required = "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }";
    assert!(
        web_demo_cargo.contains(required),
        "web-demo should avoid implicit all-components feature lift via `{required}`."
    );

    for required in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "已核验（kbd）：`crates/ui-components/Cargo.toml` 存在 `component-kbd = [\"dep:ui-kbd\"]`",
        "`crates/ui-components/src/lib.rs` 与 `crates/ui-components/src/css.rs` 对 `kbd` 导出/样式聚合均受 `#[cfg(feature = \"component-kbd\")]` 门控",
        "反向依赖核验：`apps/web-demo/Cargo.toml` 对 `ui-components` 使用 `default-features = false`",
        "最小特性核验：`cargo tree -e features -p ui-components --no-default-features --features component-kbd,inject-css`",
        "N/A（kbd，仓库级治理）：CI“最小特性 wasm 编译任务”与“产物体积预算阈值”属于仓库级流水线策略",
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "已核验（特性树注册）：`crates/ui-components/Cargo.toml` 存在 `component-kbd = [\"dep:ui-kbd\"]`",
        "已核验（`lib.rs` 门控）：`crates/ui-components/src/lib.rs` 通过 `#[cfg(feature = \"component-kbd\")] pub use ui_kbd as kbd;` 条件导出",
        "已核验（`css.rs` 门控）：`crates/ui-components/src/css.rs` 在 `#[cfg(feature = \"component-kbd\")]` 下才执行 `out.push_str(crate::kbd::styles::CSS);`",
        "已核验（反向依赖）：`apps/web-demo/Cargo.toml` 对 `ui-components` 使用 `default-features = false`",
        "回归约束：后续新增导出/样式聚合路径必须保持 feature 门控",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include tree-shaking evidence/governance marker `{required}`."
        );
    }
}

#[test]
fn kbd_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive = load_source("primitive_kbd");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum KbdSize",
        "pub struct KbdStateInput",
        "pub struct KbdState",
        "pub fn as_attr(self) -> &'static str {",
        "KbdSize::Sm => \"sm\"",
        "KbdSize::Md => \"md\"",
    ] {
        assert!(
            primitive.contains(required),
            "kbd primitive should keep type-level discrete-state contract `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] size: Option<KbdSize>",
        "pub fn normalize_size(value: Option<KbdSize>) -> KbdSize",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "let state = resolve_state(KbdStateInput {",
        "has_keys: keys.is_some()",
        "has_custom_class_name: class_name.is_some()",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "kbd logic/view should keep normalized typed-input pipeline marker `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] size: Option<String>",
        "size: Option<String>",
        "variant: Option<String>",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "kbd should not regress to free-form string state protocol (`{forbidden}`)."
        );
    }

    for required in [
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "kbd view should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "已核验（kbd）：离散状态轴由 `ui-state-primitives::kbd::KbdSize`（`enum`）与 `KbdState` 建模",
        "组件公开输入使用 `Option<KbdSize>`（非字符串协议）",
        "无效状态收敛：`logic.rs` 通过 `normalize_size/normalize_optional_text/resolve_view_model` 统一归一化",
        "最终状态仅映射到封闭集合 `data-size=sm|md` 与 `data-state=with-keys|label-only`",
        "机器可读语义：`view.rs` 稳定输出 `data-size/data-state/data-keys/data-custom-class`",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should keep type-system + semantic-marker evidence `{required}`."
        );
    }
}

#[test]
fn kbd_focus_stack_rule_is_na_for_non_overlay_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "NodeRef",
        "document.body",
        "FallbackTo",
        "focus_manager",
        "restore_focus",
        "focus_stack",
        "Selector::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not embed overlay focus-stack machinery (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A（kbd）：`kbd` 为静态展示组件，非层叠 `Overlay`",
        "组件实现未持有 `NodeRef`，也未引入 `FallbackTo/Selector` 焦点管理链路",
        "边界约束：若后续 `kbd` 演进为可聚焦叠层交互组件，必须接入统一 Focus Manager",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include focus-stack N/A boundary evidence `{required}`."
        );
    }
}

#[test]
fn kbd_escape_hatch_rule_is_na_for_non_foreign_integration_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "JsValue",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not embed imperative third-party foreign-zone integration markers (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A（kbd）：`kbd` 为静态展示组件，无 ECharts/Map 等命令式第三方集成需求",
        "组件未暴露第三方实例类型，也不存在 `YieldControl/CleanupForeign` 外交特区桥接链路",
        "边界约束：若后续引入命令式第三方渲染，必须在受控 Foreign Zone 中封装生命周期清理",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include escape-hatch N/A boundary evidence `{required}`."
        );
    }
}

#[test]
fn kbd_hydration_discontinuity_rule_is_na_for_idless_static_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let root = load_source("ui_components_root");
    let check2 = load_source("check2");

    for forbidden in [
        "now()",
        "SystemTime::now",
        "Instant::now",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "js_sys::Date::now",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should not use nondeterministic time/random id sources (`{forbidden}`)."
        );
    }

    assert!(
        root.contains("provide_ui_id_provider(id_seed);"),
        "ui root should expose deterministic id provider entry for components that need ids."
    );

    for required in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A（kbd）：`kbd` 为静态展示组件，不生成运行时随机 ID，也不依赖时间戳初始化",
        "`logic/view` 不含 `now()`/UUID/随机源调用",
        "边界约束：若后续 `kbd` 引入动态 ID 语义，必须经 `UiRoot` 提供的 `IdProvider` 确定性种子链路接入",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include hydration-discontinuity N/A boundary evidence `{required}`."
        );
    }
}

#[test]
fn kbd_ssr_and_cross_platform_compile_only_contract_is_recorded_and_safe() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let kbd_cargo = load_source("kbd_cargo");
    let check2 = load_source("check2");

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen::",
        "window.",
        "document.",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd should keep non-wasm path free from browser-only bindings (`{forbidden}`)."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"ssr\")]",
        "#[cfg(feature = \"web\")]",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd should not fork behavior by platform feature in component source (`{forbidden}`)."
        );
    }

    assert!(
        kbd_cargo.contains("[features]") && kbd_cargo.contains("default = []"),
        "kbd cargo should keep explicit feature boundary with empty defaults."
    );

    for required in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "已核验（kbd）：`components/kbd/src/{mod,logic,view,styles}.rs` 不含 `web-sys`/`js-sys`/`wasm-bindgen` 依赖",
        "不含 `#[cfg(target_arch = \"wasm32\")]`、`#[cfg(feature = \"ssr\")]` 分叉",
        "cargo check -p ui-kbd --target wasm32-unknown-unknown",
        "cargo check -p ui-kbd --target x86_64-unknown-linux-gnu",
        "cargo check -p ui-kbd",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include cross-platform compile-only evidence `{required}`."
        );
    }
}

#[test]
fn kbd_headless_web_ssr_feature_mutex_contract_is_guarded() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let kbd_cargo = load_source("kbd_cargo");
    let ui_headless_lib = load_source("ui_headless_lib");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should keep web/ssr feature mutex guard `{required}`."
        );
    }

    assert!(
        !kbd_cargo.contains("ui-headless"),
        "kbd crate should not directly depend on ui-headless in current static component scope."
    );

    for forbidden in ["ui_headless::", "use ui_headless"] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd source should not directly wire ui-headless contracts in this component (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "已核验：`crates/ui-headless/src/lib.rs` 存在 `#[cfg(all(feature = \"web\", feature = \"ssr\"))]` 与 `compile_error!(...)`",
        "N/A（kbd）：`ui-kbd` 当前未依赖 `ui-headless`",
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include ui-headless web/ssr mutex evidence `{required}`."
        );
    }
}

#[test]
fn kbd_motion_non_wasm_noop_stub_contract_is_guarded() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let kbd_cargo = load_source("kbd_cargo");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op/stub contract anchor `{required}`."
        );
    }

    assert!(
        !kbd_cargo.contains("ui-motion"),
        "kbd crate should not directly depend on ui-motion in current static component scope."
    );

    for forbidden in ["ui_motion::", "motion::", "attach_motion", "animate("] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd source should not directly wire motion runtime contracts (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "已核验：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web` stub",
        "N/A（kbd）：`kbd` 为静态展示组件，未引入 `motion.rs`、未依赖 `ui-motion`、未调用 `attach/animate`",
        "cargo check -p ui-motion --target x86_64-unknown-linux-gnu",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion non_wasm_web_backend_is_predictable_noop -- --exact",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include ui-motion non-wasm no-op/stub evidence `{required}`."
        );
    }
}

#[test]
fn kbd_reduced_motion_ssr_wasm_branch_contract_is_na_and_semantically_stable() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for forbidden in [
        "attach_motion",
        "prefers_reduced_motion",
        "animate(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"ssr\")]",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd should keep reduced-motion/SSR/wasm branch handling N/A without platform-forked code (`{forbidden}`)."
        );
    }

    for required in [
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep stable semantic markers across SSR/wasm paths (`{required}`)."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should provide non-wasm reduced-motion-safe no-op backend anchor `{required}`."
        );
    }

    for required in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "N/A（kbd）：`kbd` 为静态展示组件，无 `motion.rs`/`attach_motion`/动画句柄路径",
        "已核验（kbd）：`components/kbd/src/{mod,logic,view,styles}.rs` 无 `wasm32/ssr` 条件分支与浏览器 API 依赖",
        "根节点稳定输出 `data-slot/data-size/data-state`",
        "依赖保障：`crates/ui-motion/src/lib.rs` non-wasm 分支提供 `prefers_reduced_motion() -> true` 与 `animate(...)` no-op",
        "边界约束：若后续引入 wasm 增强交互或组件动效，必须保持 SSR 与 wasm 语义标记集合一致",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include reduced-motion/SSR/wasm branch evidence `{required}`."
        );
    }
}

#[test]
fn kbd_motion_contract_is_na_without_component_motion_rs_and_attach_pipeline() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");
    assert!(
        !motion_path.exists(),
        "kbd should keep motion.rs absent for static display scope without motion semantic axis."
    );

    for forbidden in [
        "attach_motion",
        "stiffness",
        "damping",
        "ui_motion::",
        "motion::",
        "animate(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd source should keep motion-contract executor out of component layer (`{forbidden}`)."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep reduced-motion/non-wasm no-op backbone marker `{required}`."
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A（kbd）：`kbd` 为静态展示组件，无 open/close/enter/exit/active 等动效语义状态轴",
        "已核验（实现边界）：`components/kbd/src/{mod,logic,view,styles}.rs` 不含 `attach_motion`、`stiffness`、`damping`、`ui_motion::` 调用",
        "依赖保障：`crates/ui-motion/src/lib.rs` 在 non-wasm 路径提供 `prefers_reduced_motion() -> true` 与 `animate(...)` no-op",
        "回归约束：若后续为 `kbd` 引入动效语义，必须新增 `motion.rs` 承载 `stiffness/damping` 合同并通过 `attach_motion` 挂载",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include motion-contract evidence `{required}`."
        );
    }
}

#[test]
fn kbd_performance_governance_is_mount_only_traceable_and_backed_by_repo_gates() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let perf_script = load_source("perf_script");
    let e2e_docs_coverage = load_source("e2e_docs_coverage");
    let docs_todo = load_source("docs_todo");
    let check2 = load_source("check2");

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointer",
        "create_signal(",
        "RwSignal<",
        "Memo::new(",
        "Effect::new(",
        "attach_motion",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd should stay mount-only in current scope without update-loop perf paths (`{forbidden}`)."
        );
    }

    for required in [
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(required),
            "shared performance gate script should include `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_docs_coverage.contains(required),
            "docs e2e coverage should enforce perf probe contract marker `{required}`."
        );
    }

    assert!(
        docs_todo.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "repo plan should keep render_count automation follow-up task for shared performance governance."
    );

    for required in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "已核验（kbd）：`kbd` 为静态展示组件（无异步、无动效 attach、无交互状态更新环路），关键路径为 mount-only",
        "共享预算与阻断链路：`scripts/check-ui-components-performance.sh` 已纳入",
        "`e2e/tests/docs_app_components_coverage.spec.mjs` 持续断言 `data-perf-budget-*` 并阻断 `data-perf-violation=true`",
        "render_count 跟进状态：`docs/plan/TODO.md` 保留“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”",
        "N/A（kbd，组件级）：`Button`、`Input` 初始化渲染预算为 `1` 属于跨组件基线",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include performance-governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_semantics_and_performance_regression_contract_is_covered_with_clear_na_boundaries() {
    let semantics = load_source("semantics");
    let view = load_source("view");
    let logic = load_source("logic");
    let docs_todo = load_source("docs_todo");
    let check2 = load_source("check2");

    for required in [
        "fn kbd_semantics_contract_tests_exist_and_do_not_depend_on_visual_snapshots()",
        "fn kbd_state_markers_are_observable_queryable_and_enumerable()",
        "fn kbd_a11y_i18n_l10n_contract_is_present_without_hardcoded_copy()",
        "fn kbd_performance_governance_is_mount_only_traceable_and_backed_by_repo_gates()",
    ] {
        assert!(
            semantics.contains(required),
            "kbd suite should include semantic/performance regression anchor `{required}`."
        );
    }

    for forbidden in [
        "insta::assert_snapshot!",
        "insta::assert_debug_snapshot!",
        "insta::assert_yaml_snapshot!",
        "assert_snapshot!(",
    ] {
        assert!(
            !semantics.contains(forbidden),
            "kbd checks should not rely on visual snapshots as semantic/perf substitute (`{forbidden}`)."
        );
    }

    for required in [
        "data-slot=\"kbd\"",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
        "data-keys=view_model.state.has_keys.then_some(\"true\")",
        "data-custom-class=view_model.state.has_custom_class_name.then_some(\"true\")",
        "<kbd",
    ] {
        assert!(
            view.contains(required),
            "kbd view should expose stable data markers and native semantic root `{required}`."
        );
    }

    for forbidden in [
        "on:click",
        "on:keydown",
        "tabindex=",
        "aria-busy",
        "aria-disabled",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "kbd should keep interaction/focus-flow markers N/A in this static scope (`{forbidden}`)."
        );
    }

    assert!(
        docs_todo.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "repo plan should keep render_count automation follow-up task for heavy/interactive components."
    );

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "已核验（语义断言覆盖）：`components/kbd/test/semantics.rs` 覆盖 `data-*` 语义标记",
        "已核验（`aria-*`/焦点流转适用性）：`kbd` 为非交互展示组件",
        "源码不含 `on:click/on:keydown/tabindex` 交互链路",
        "已核验（性能回归证据）：`kbd_performance_governance_is_mount_only_traceable_and_backed_by_repo_gates`",
        "N/A（kbd，`render_count=1` 基线）：该硬预算主要适用于高频/重型交互组件（如 Button/Input）",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include semantic+performance regression evidence `{required}`."
        );
    }
}

#[test]
fn kbd_view_macro_complexity_is_bounded_and_semantically_chunked() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    let view_macro_count = view.matches("view! {").count();
    assert!(
        (1..=2).contains(&view_macro_count),
        "kbd view macro count should stay bounded (expected 1..=2, got {view_macro_count})."
    );

    for required in [
        "let view_model = logic::resolve_view_model(KbdLogicInput {",
        "data-slot=\"kbd\"",
        "data-slot=\"kbd-keys\"",
        "data-slot=\"kbd-label\"",
        "fn render_keys_slot(keys: Option<String>) -> impl IntoView {",
        "{render_keys_slot(view_model.keys)}",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep semantic chunk marker `{required}`."
        );
    }

    for forbidden in [
        "match view_model",
        "if view_model.state",
        "for item in",
        "while ",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd view should avoid growing into complex state/layout control flow (`{forbidden}`)."
        );
    }

    assert!(
        logic.contains("pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel"),
        "kbd complexity boundary relies on centralized logic view-model derivation."
    );

    for required in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "已核验（kbd）：`components/kbd/src/view.rs` 仅包含一个主 `view!` 块（根 `<kbd>`）与一个内联 keys 子块（`Option::map`）",
        "语义分块清晰：`kbd` 主体与 `kbd-keys`/`kbd-label` 子块在同一渲染上下文中显式分离",
        "回归约束：若后续引入复杂布局导致 `view!` 体量异常增长，必须优先按语义提取局部渲染函数",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include view-macro complexity governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_prefers_functional_fragment_splitting_over_extra_component() {
    let view = load_source("view");
    let check2 = load_source("check2");

    let component_count = view.matches("#[component]").count();
    assert!(
        component_count == 1,
        "kbd should keep exactly one component entry and avoid fragment over-componentization (got {component_count})."
    );

    for required in [
        "fn render_keys_slot(keys: Option<String>) -> impl IntoView {",
        "keys.map(|keys| view! { <span class=\"ui-kbd__keys\" data-slot=\"kbd-keys\">{keys}</span> })",
        "{render_keys_slot(view_model.keys)}",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep functional fragment split marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]\nfn KbdKeys",
        "#[component]\nfn KbdLabel",
        "component KbdKeys",
        "component KbdLabel",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd should not promote local fragments into extra components (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "已核验（kbd）：`view.rs` 将轻逻辑 keys 片段提取为普通函数 `render_keys_slot(keys: Option<String>) -> impl IntoView`",
        "主组件仅保留 `#[component] pub fn Kbd(...)`",
        "语义稳定性：函数化后仍保持 `data-slot=\"kbd-keys\"`/`data-slot=\"kbd-label\"` 标记不变",
        "边界约束：若后续出现新的轻逻辑片段，优先继续函数化拆分",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include functional-splitting governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_static_fragment_constantization_rule_is_na_and_keeps_semantics_stable() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "<svg",
        "inner_html=",
        "dangerously_set_inner_html",
        "footer",
        "<article",
        "<section",
    ] {
        assert!(
            !view.contains(forbidden),
            "kbd should not embed heavy static fragment classes that require constant/template extraction (`{forbidden}`)."
        );
    }

    for required in [
        "fn render_keys_slot(keys: Option<String>) -> impl IntoView {",
        "data-slot=\"kbd\"",
        "data-slot=\"kbd-keys\"",
        "data-slot=\"kbd-label\"",
        "<kbd",
    ] {
        assert!(
            view.contains(required),
            "kbd view should keep concentrated static fragment marker `{required}`."
        );
    }

    for required in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A（kbd）：当前组件不存在复杂 SVG/长说明文本/页脚等重静态片段",
        "已核验（kbd）：静态子块入口集中在 `view.rs`（`render_keys_slot` + 主 `Kbd` 结构）",
        "语义保障：函数化与静态片段收敛后仍保持 `<kbd>` 语义与 `data-slot=\"kbd\"` / `data-slot=\"kbd-keys\"` / `data-slot=\"kbd-label\"` 稳定",
        "边界约束：若后续引入复杂纯静态片段（如大段 SVG 或长文本模板），必须优先常量化/模板化并集中落点",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include static-fragment constantization evidence `{required}`."
        );
    }
}

#[test]
fn kbd_inner_html_usage_is_forbidden_without_trusted_constant_and_security_regression() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "set_inner_html(",
        ".set_inner_html(",
        "insert_adjacent_html",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd should not include raw html injection path (`{forbidden}`)."
        );
    }

    for required in [
        "<kbd",
        "data-slot=\"kbd\"",
        "data-slot=\"kbd-keys\"",
        "data-slot=\"kbd-label\"",
    ] {
        assert!(
            view.contains(required),
            "kbd should keep typed semantic-template path marker `{required}`."
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A（kbd）：组件当前不使用 `inner_html`",
        "已核验（kbd）：`src/{mod,logic,view,styles}.rs` 不含 `inner_html`/`dangerously_set_inner_html`/`set_inner_html` 调用",
        "安全回归约束：若后续确需引入 `inner_html`，仅允许受信任编译期常量并必须同步补充语义测试与安全回归说明。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include inner_html security-governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_wasm_debug_contract_is_docs_scoped_and_feature_isolated() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs_display = load_source("docs_display");
    let kbd_cargo = load_source("kbd_cargo");
    let check2 = load_source("check2");

    for required in [
        "pub fn resolve_view_model(input: KbdLogicInput) -> KbdViewModel",
        "let state = resolve_state(KbdStateInput {",
        "data-size=view_model.state.size_attr",
        "data-state=view_model.state.state_attr",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "kbd should keep deterministic prop->state derivation marker `{required}` for dev traceability."
        );
    }

    assert!(
        !view.contains("on:"),
        "kbd should remain a static display component without event-chain replay surface."
    );

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"kbd-workbench-controls\"",
        "KbdActualConfig {",
    ] {
        assert!(
            docs_display.contains(required),
            "docs-app should keep kbd visual debug/workbench anchor `{required}`."
        );
    }

    assert!(
        kbd_cargo.contains("[features]") && kbd_cargo.contains("default = []"),
        "kbd crate should keep explicit empty default feature boundary."
    );
    for forbidden in ["debug =", "devtools", "wasm-debug"] {
        assert!(
            !kbd_cargo.contains(forbidden),
            "kbd cargo should not expose production-facing debug feature `{forbidden}`."
        );
    }

    for required in ["pub use logic::KbdSize;", "pub use view::Kbd;"] {
        assert!(
            module.contains(required),
            "kbd public api should stay minimal without debug exports (`{required}`)."
        );
    }
    for forbidden in [
        "pub use logic::KbdDebug",
        "pub use view::KbdDebug",
        "pub mod debug",
    ] {
        assert!(
            !module.contains(forbidden),
            "kbd module should not leak debug surfaces (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A（kbd，状态追踪/交互回放）：`kbd` 为展示型静态组件",
        "已核验（开发可视化入口）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 提供 `Playground` 与 `Workbench (Display + Config + Code + CSS Test)`",
        "已核验（feature 隔离与产物纯净）：`components/kbd/Cargo.toml` 仅声明 `default = []` 且无 debug/devtools feature",
        "回归约束：若后续为 `kbd` 引入交互状态，必须新增 feature 隔离的调试探针（默认关闭），并补充事件序列/状态转移可回放的语义回归测试。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include wasm-debug governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_dx_contract_keeps_workbench_isolation_and_context_retention() {
    let styles = load_source("styles");
    let view = load_source("view");
    let docs_display = load_source("docs_display");
    let check2 = load_source("check2");

    assert!(
        styles.contains("pub const CSS: &str = r#\""),
        "kbd styles should remain centralized in static CSS for tooling-driven fast feedback."
    );
    assert!(
        !view.contains("style="),
        "kbd view should avoid inline style logic that weakens stylesheet-centric DX iteration."
    );

    for required in [
        "pub(super) fn kbd() -> AnyView {",
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/kbd/src/styles.rs\".to_string()",
        "data-slot=\"kbd-workbench-controls\"",
        "let (workbench_size_key, set_workbench_size_key) = signal(\"md\".to_string());",
        "let (workbench_keys, set_workbench_keys) = signal(\"Ctrl\".to_string());",
        "let (workbench_label, set_workbench_label) = signal(\"K\".to_string());",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs should keep DX workbench/context-retention anchor `{required}`."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "N/A（kbd，热重载机制归属）：是否“无需重编 wasm”由 `apps/*` 开发工具链（Trunk/Leptos dev server）决定",
        "已核验（Workbench 隔离画布）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 提供独立 `Playground` 与 `Workbench (Display + Config + Code + CSS Test)`",
        "已核验（上下文保持与可选状态保留）：workbench 使用本地 signals（`workbench_size_key/workbench_keys/workbench_label/workbench_custom_class`）维持当前调试上下文",
        "回归约束：若后续引入复杂交互状态，必须在 docs workbench 提供可选状态保留开关（默认关闭）并保持不进入组件公共 API。",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include DX governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_engineering_contract_keeps_serde_internal_and_runtime_agnostic_public_api() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let protocol = load_source("protocol");
    let kbd_cargo = load_source("kbd_cargo");
    let check2 = load_source("check2");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum KbdComponentSchemaVersion",
        "pub struct KbdComponentSpec",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "pub schema_version: KbdComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "kbd protocol should keep minimal structured serde/version marker `{required}`."
        );
    }

    for forbidden in [
        "use tracing::",
        "tracing::",
        "trace!(",
        "debug!(",
        "info!(",
        "warn!(",
        "error!(",
        "tokio::",
        "async_std::",
        "async fn ",
        ".await",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "kbd source should keep runtime-agnostic and side-effect-light boundary (`{forbidden}`)."
        );
    }

    for forbidden in ["tokio =", "async-std", "tracing ="] {
        assert!(
            !kbd_cargo.contains(forbidden),
            "kbd crate should not bind to runtime/tracing package detail `{forbidden}`."
        );
    }

    for required in [
        "pub use logic::KbdSize;",
        "pub use view::Kbd;",
        "serde = { version = \"1.0\", features = [\"derive\"] }",
    ] {
        assert!(
            module.contains(required) || kbd_cargo.contains(required),
            "kbd should keep focused public API and serialization dependency marker `{required}`."
        );
    }

    for forbidden in ["pub use protocol::", "pub mod protocol;", "pub use tokio::"] {
        assert!(
            !module.contains(forbidden),
            "kbd module should not leak protocol/runtime internals (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "已核验（serde 协议路径）：`components/kbd/src/protocol.rs` 提供最小版本化协议类型 `KbdComponentSchemaVersion` / `KbdComponentSpec`",
        "N/A（kbd，tracing 埋点）：`kbd` 为纯展示型同步组件",
        "N/A（kbd，async runtime）：组件源码与依赖未引入 `tokio`/`async-std` 运行时类型",
        "回归约束：若后续引入异步或复杂流程，必须沿用统一结构化错误与 tracing 语义",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include engineering-governance evidence `{required}`."
        );
    }
}

#[test]
fn kbd_interactive_playground_contract_is_locked_with_docs_and_e2e_evidence() {
    let docs_display = load_source("docs_display");
    let e2e_kbd_contract = load_source("e2e_kbd_contract");
    let check2 = load_source("check2");

    for required in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"kbd-workbench-controls\"",
        "test_config_signal=workbench_actual_config",
        "let (workbench_size_key, set_workbench_size_key) = signal(\"md\".to_string());",
        "let (workbench_keys, set_workbench_keys) = signal(\"Ctrl\".to_string());",
        "let (workbench_label, set_workbench_label) = signal(\"K\".to_string());",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
    ] {
        assert!(
            docs_display.contains(required),
            "kbd docs interactive playground should keep workbench marker `{required}`."
        );
    }

    for required in ["runKbdWorkbenchFlow", "await page.reload();"] {
        assert!(
            e2e_kbd_contract.contains(required),
            "kbd e2e contract should keep repeatable playground flow marker `{required}`."
        );
    }

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "已核验（kbd Interactive Playground）：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 提供 `Workbench (Display + Config + Code + CSS Test)`",
        "已核验（基础 props + 状态切换 + 反馈观察）：workbench 控件区 `data-slot=\"kbd-workbench-controls\"`",
        "N/A（AI Spec 联动示例）：`kbd` 非 AI Spec 组件",
        "已核验（可重复验收路径）：`e2e/tests/docs_app_kbd_contract.spec.mjs` 的 `runKbdWorkbenchFlow` + `page.reload()` 回放覆盖交互路径",
    ] {
        assert!(
            check2.contains(required),
            "kbd checklist should include interactive-playground evidence `{required}`."
        );
    }
}
