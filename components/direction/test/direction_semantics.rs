use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn direction_does_not_expose_view_module() {
    let source = load_source("../../components/direction/src/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Direction internals should stay private; found `pub mod view`."
    );
}

#[test]
fn direction_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/direction/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{DirectionMode, DirectionProvider};"),
        "ui-direction module should export `DirectionMode` and `DirectionProvider`."
    );
    assert!(
        crate_source.contains("pub use ui_direction as direction;"),
        "crate root should re-export ui-direction as `direction` module."
    );
    assert!(
        crate_source.contains("pub use direction::{DirectionMode, DirectionProvider};"),
        "crate root should re-export direction contracts."
    );
}

#[test]
fn direction_provider_exposes_slot_and_dir_contracts() {
    let source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "pub fn DirectionProvider(",
        "DirectionMode",
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};",
        "let contract = use_direction(DirectionA11yOptions { direction, lang });",
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "DirectionProvider should include `{needle}` for stable contract checks."
        );
    }
}

#[test]
fn direction_logic_consumes_state_primitives_contract() {
    let source = load_source("../../components/direction/src/logic.rs");

    for needle in [
        "ui_state_primitives::direction",
        "DirectionMode",
        "normalize_optional_text",
        "DirectionPropSource",
        "resolve_direction",
    ] {
        assert!(
            source.contains(needle),
            "Direction logic should consume status-primitives via `{needle}`."
        );
    }
}

#[test]
fn direction_api_naming_contract_is_documented_and_traceable() {
    let source = load_source("../../components/direction/check2.md");

    for needle in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "N/A（direction）：本组件无布尔状态轴、无事件回调轴、无默认值轴，不涉及 `is_*`/`on_*`/`default_*` 命名面。",
        "兼容策略：主命名固定为 `direction`，保留 `dir` 作为兼容别名输入；通过 `logic::resolve_direction` 统一归一，并暴露 `data-direction-source` 便于观测来源。",
        "迁移路径：文档与示例仅使用 `direction`；存量调用可继续使用 `dir`，后续增量代码禁止新增 `dir` 用法。",
    ] {
        assert!(
            source.contains(needle),
            "direction/check2 naming contract should keep `{needle}`.",
        );
    }
}

#[test]
fn direction_controlled_uncontrolled_contract_is_documented_as_na() {
    let source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");

    for needle in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "N/A（direction）：该组件不维护内部可变状态轴，仅消费传入方向并映射语义标记，不存在 `value/on_value_change/default_value` 协议面。",
        "约束：禁止新增 `default_direction`、`on_direction_change` 或内部 `Signal` 写回；若未来引入内部状态轴，必须成对补齐受控/非受控 API 与回归测试。",
    ] {
        assert!(
            source.contains(needle),
            "direction/check2 controlled-uncontrolled marker should keep `{needle}`.",
        );
    }

    for forbidden in [
        "default_direction",
        "on_direction_change",
        "ReadSignal<",
        "WriteSignal<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Direction should not expose controlled/uncontrolled axis token `{forbidden}`.",
        );
    }
}

#[test]
fn direction_default_value_priority_is_normalized_once_in_logic() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "规则（direction）：`direction` > `dir` > `DirectionMode::default()`，由 `logic::resolve_direction` 单点归一并以 `test/logic.rs` 回归锁定。",
        "`view.rs` 仅消费 `logic::resolve_direction` 的归一化结果与来源标记，不做 `unwrap_or*` 或条件分支兜底。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 default-source rule should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_direction(",
        "if let Some(direction) = direction {",
        "} else if let Some(direction) = dir {",
        "(DirectionMode::default(), DirectionPropSource::Default)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Direction logic should own default resolution rule fragment `{needle}`."
        );
    }

    assert!(
        view_source.contains(
            "let (direction, direction_source) = logic::resolve_direction(direction, dir);"
        ),
        "Direction view should consume logic-level normalization only."
    );
    for forbidden in [
        "unwrap_or(",
        "unwrap_or_else(",
        "if let Some(direction) = direction",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view must not duplicate default/fallback branch `{forbidden}`."
        );
    }
}

#[test]
fn direction_state_normalization_is_centralized_in_logic_layer() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "输入/输出（direction）：`view.rs` 仅接收类型化输入 `Option<DirectionMode>`（`direction/dir`），统一交给 `logic::resolve_direction`，输出固定为 `(DirectionMode, DirectionPropSource)`。",
        "事件路径（direction）：该组件无交互事件处理器（N/A），不存在在回调中重建状态机的入口。",
        "样式路径（direction）：`styles.rs` 仅提供静态布局规则，不含状态分支或状态判定逻辑。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 state-normalization marker should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_direction(",
        "direction: Option<DirectionMode>",
        "dir: Option<DirectionMode>",
        ") -> (DirectionMode, DirectionPropSource)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Direction logic should centralize typed normalization fragment `{needle}`."
        );
    }

    for needle in [
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Direction view should consume normalized output via `{needle}`."
        );
    }
    for forbidden in [
        "match direction",
        "if let Some(direction) = direction",
        "if let Some(direction) = dir",
        "on:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view must not rebuild state logic fragment `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains(".ui-direction-provider"),
        "Direction styles should remain static style contract."
    );
    for forbidden in ["data-direction", "if ", "match "] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles should not host state branching fragment `{forbidden}`."
        );
    }
}

#[test]
fn direction_discrete_state_axes_stay_enum_constrained() {
    let check_source = load_source("../../components/direction/check2.md");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/direction.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "`direction` 离散输入固定为 `DirectionMode` 枚举：`DirectionMode::{Ltr,Rtl}`，`direction/dir` 两个入口均为 `Option<DirectionMode>`，无字符串自由组合入口。",
        "本组件当前无外部字符串配置输入（N/A）；若未来引入字符串兼容层，必须先在 `logic.rs` 做字符串到 `DirectionMode` 的显式映射并补回归测试。",
        "约束：禁止新增 `Option<bool>`（如 `is_ltr/is_rtl`）来表达方向互斥状态，统一通过 `DirectionMode` 建模。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 discrete-state marker should keep `{needle}`."
        );
    }

    for needle in [
        "pub enum DirectionMode",
        "Ltr",
        "Rtl",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "#[default]",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Direction primitive should keep enum-based discrete state fragment `{needle}`."
        );
    }

    for needle in [
        "direction: Option<DirectionMode>",
        "dir: Option<DirectionMode>",
        "#[prop(optional)] direction: Option<DirectionModeImpl>",
        "#[prop(optional)] dir: Option<DirectionModeImpl>",
    ] {
        let found = logic_source.contains(needle) || view_source.contains(needle);
        assert!(
            found,
            "Direction component input should stay enum-typed via `{needle}`."
        );
    }

    for forbidden in [
        "Option<bool>",
        "is_ltr",
        "is_rtl",
        "direction: Option<String>",
        "dir: Option<String>",
        "direction: Option<&str>",
        "dir: Option<&str>",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Direction should not expose bool/string discrete-axis fragment `{forbidden}`."
        );
    }
}

#[test]
fn direction_state_primitive_source_boundary_stays_clean() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");

    for needle in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "`direction` 仅消费 `ui_state_primitives::direction::{DirectionMode, normalize_optional_text}`；组件层不重写方向状态原语与归一规则。",
        "`logic.rs` 仅做装配映射（`resolve_direction`、来源标记、class 归一），无业务 store 依赖与全局状态类型耦合。",
        "应用级全局状态接入（N/A）：本组件当前无业务 store 输入；若未来接入应用状态，必须先经桥接层映射到 `DirectionMode` 后再传入组件。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 primitive-source marker should keep `{needle}`."
        );
    }

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::direction::{DirectionMode, normalize_optional_text};"
        ),
        "Direction logic should consume primitives from ui-state-primitives."
    );

    for forbidden in [
        "pub enum DirectionMode",
        "impl DirectionMode",
        "pub fn normalize_optional_text(",
        "create_signal",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "GlobalStore",
        "AppStore",
        "use_store(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Direction component should not reimplement primitive or couple business store token `{forbidden}`."
        );
    }

    assert!(
        cargo_source.contains("ui-state-primitives"),
        "Direction component should keep ui-state-primitives dependency."
    );
}

#[test]
fn direction_stays_out_of_async_interaction_protocols() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "N/A（direction）：该组件仅做方向与语言语义装配，无远程请求、任务轮询或异步状态轴。",
        "约束：禁止在该组件新增 `is_loading`、`on_retry`、`aria-busy`、`use_async_action` 等异步协议碎片；若未来引入异步交互，必须统一复用 async action 原语并补语义回归。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 async marker should keep `{needle}`."
        );
    }

    for forbidden in [
        "is_loading",
        "on_retry",
        "aria-busy",
        "aria_busy",
        "use_async_action",
        "async fn",
        ".await",
        "spawn_local",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Direction should not include async interaction protocol token `{forbidden}`."
        );
    }
}

#[test]
fn direction_api_dx_surface_remains_low_friction() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "基础路径：`<DirectionProvider direction=DirectionMode::Ltr>...</DirectionProvider>` 即可使用，无需手动接线 `ui-state-primitives`/`ui-headless`。",
        "docs 最小示例：`layout_extra_direction.rs` 的 LTR Playground 代码片段为 3 行（<=5 行）且可直接运行。",
        "高级入口按需：仅在需要时使用 `dir`（兼容别名）、`lang`、`class_name`；基础路径不暴露内部状态对象。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 DX marker should keep `{needle}`."
        );
    }

    assert!(
        docs_source.contains("use ui_components::{DirectionMode, DirectionProvider};"),
        "Direction docs should keep direct component-level imports."
    );
    for forbidden in ["ui_state_primitives", "ui_headless::", "use_direction("] {
        assert!(
            !docs_source.contains(forbidden),
            "Direction docs should not require low-level wiring token `{forbidden}`."
        );
    }

    for forbidden in ["state:", "state =", "state: RwSignal", "state: ReadSignal"] {
        assert!(
            !view_source.contains(forbidden),
            "Direction API should not require internal state object token `{forbidden}`."
        );
    }

    let hello_world_snippet = r#"<DirectionProvider direction=DirectionMode::Ltr>
  <div class="docs-direction-demo">"Name → Value"</div>
</DirectionProvider>"#;
    assert!(
        docs_source.contains(hello_world_snippet),
        "Direction docs should include minimal hello-world snippet."
    );
    let hello_world_lines = hello_world_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        hello_world_lines <= 5,
        "Direction hello-world snippet should remain <= 5 lines, got {hello_world_lines}.",
    );
}

#[test]
fn direction_composition_contract_stays_na_and_non_composite() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A（direction）：该组件是单容器语义 provider，不建模离散 item 集合，不存在 `Parent/Item` 组合协议面。",
        "当前 API 仅暴露 `DirectionProvider` + `children`，不提供并行数组（`labels/titles`）或隐式索引配对入口。",
        "约束：若未来扩展为集合型组件，必须引入显式 item 语义结构并补组合契约测试，禁止并行数组语法糖直出。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 composition marker should keep `{needle}`."
        );
    }

    assert!(
        view_source.contains("pub fn DirectionProvider(")
            && view_source.contains("children: Children"),
        "Direction API should remain single-provider + children shape."
    );
    for forbidden in [
        "ItemSpec",
        "labels",
        "titles",
        "panels",
        "items:",
        "Vec<",
        "For each=",
        "index",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction source should not include composite item API token `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "ItemSpec"] {
        assert!(
            !docs_source.contains(forbidden),
            "Direction docs should not include composite sugar token `{forbidden}`."
        );
    }
}

#[test]
fn direction_macro_micro_drag_protocol_remains_out_of_scope() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");

    for needle in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "N/A（direction）：该组件无拖拽语义、无高频 pointer 交互、无 `motion.rs`，不存在 Macro/Micro 双状态机面。",
        "当前实现不包含 `Dragging`、`Action::DragEnd`、RAF 循环或 pointer move 连续采样逻辑。",
        "约束：若未来引入拖拽交互，必须在 `view/motion` 处理拖拽期本地循环，仅在结束时回流 `logic` 收敛，并补对应语义回归。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 macro-micro marker should keep `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "Direction should stay motion-free for current macro/micro scope."
    );
    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "drag_start",
        "drag_end",
        "pointermove",
        "mousemove",
        "touchmove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include macro/micro drag token `{forbidden}`."
        );
    }
}

#[test]
fn direction_two_pass_geometry_rendering_stays_out_of_scope() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A（direction）：该组件不依赖 DOM 几何测量，不实现 overlay 定位，不存在 `Intent -> Measure -> Rectification` 渲染链。",
        "当前实现不包含 `getBoundingClientRect`/尺寸采样/位置修正回写，也无防循环收敛分支。",
        "约束：若未来引入依赖几何测量的交互（如 popover 类定位），必须显式落地两段式流程并补幂等收敛回归测试。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 two-pass marker should keep `{needle}`."
        );
    }

    for forbidden in [
        "getBoundingClientRect",
        "client_width",
        "client_height",
        "offset_width",
        "offset_height",
        "measure",
        "Rectification",
        "ResizeObserver",
        "MutationObserver",
        "popover",
        "tooltip",
        "menu",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include two-pass geometry token `{forbidden}`."
        );
    }
}

#[test]
fn direction_registration_protocol_stays_out_of_scope_for_single_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "N/A（direction）：该组件是单容器方向语义 provider，不管理动态子项集合，不存在注册/反注册或导航排序协议面。",
        "当前实现仅暴露 `DirectionProvider + children`，无 `RegistrationContext`、`Register/Unregister`、`items_order` 或集合导航逻辑。",
        "约束：若未来扩展为多子项集合型组件，必须引入显式注册上下文并在 `logic.rs` 维护稳定 `items_order`，禁止依赖 `HashSet` 迭代顺序。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 registration marker should keep `{needle}`."
        );
    }

    assert!(
        view_source.contains("pub fn DirectionProvider(")
            && view_source.contains("children: Children"),
        "Direction API should remain single-provider + children shape."
    );

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "VecDeque",
        "next_item",
        "prev_item",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include registration protocol token `{forbidden}`."
        );
    }
}

#[test]
fn direction_slot_projection_strategy_stays_out_of_scope_for_single_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "N/A（direction）：该组件是单容器方向语义 provider，不提供多面板/多槽位投影策略，不存在 `Lazy/KeepAlive/Eager` 模式面。",
        "当前实现无 KeepAlive 挂载缓存、无隐藏态生命周期通知、无轮询/动画副作用调度路径。",
        "约束：若未来引入可切换投影模式，必须显式建模 `Lazy/KeepAlive/Eager`，并在 KeepAlive 隐藏时通过 `NotifyHidden`（或等价生命周期事件）暂停高耗能副作用并补回归测试。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 slot-projection marker should keep `{needle}`."
        );
    }

    assert!(
        view_source.contains("pub fn DirectionProvider(")
            && view_source.contains("children: Children"),
        "Direction API should remain single-provider + children shape."
    );

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "projection_mode",
        "suspend_effects",
        "resume_effects",
        "poll_interval",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include slot projection token `{forbidden}`."
        );
    }
}

#[test]
fn direction_env_streams_pipeline_stays_out_of_scope_for_single_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "N/A（direction）：该组件仅装配方向语义上下文，不消费 `Resize/Theme/Intersection` 环境流，不存在采样/防抖后推送 `Action` 的链路。",
        "当前实现无 `ResizeObserver`/`IntersectionObserver`/`matchMedia` 订阅、无防抖节流器、无 `BreakpointChanged` 等环境语义动作。",
        "约束：若未来引入环境订阅，必须在 `view.rs` 完成采样与防抖并映射为类型化 `Action` 推送到 `logic.rs`，禁止原始事件直接洪泛到逻辑层。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 env-streams marker should keep `{needle}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "ThemeChanged",
        "debounce",
        "throttle",
        "on:resize",
        "on:scroll",
        "window()",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include env stream token `{forbidden}`."
        );
    }
}

#[test]
fn direction_event_light_cone_stays_out_of_scope_for_single_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "N/A（direction）：该组件不建模大型集合与批量操作，不存在 `Context Bus + Selector` 事件光锥协议面。",
        "当前实现无 `SelectionState::All`、无批量选择压缩态、无 O(N) 逐层 prop drilling 的集合分发路径。",
        "约束：若未来扩展为集合批处理组件，必须引入 Context Bus + Selector 与压缩状态表达，禁止把批量状态按 O(N) 透传到子树。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 event-light-cone marker should keep `{needle}`."
        );
    }

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "bulk_select",
        "prop drilling",
        "for child in",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include event light cone token `{forbidden}`."
        );
    }
}

#[test]
fn direction_causality_bus_stays_out_of_scope_for_single_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    for needle in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "N/A（direction）：该组件仅提供方向语义上下文装配，不承载复杂派生总线与跨订阅者广播链路。",
        "当前实现无 `TraceId` 透传、无命令总线广播、无“触发 -> 派生 -> 广播 -> 订阅”链式编排。",
        "约束：若未来引入复杂派生总线，必须在命令与广播链路中透传稳定 `TraceId` 并补因果链回归测试，禁止中途丢失追踪上下文。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 causality-bus marker should keep `{needle}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "cause_bus",
        "dispatch_command",
        "broadcast",
        "subscriber",
        "event_bus",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Direction should not include causality bus token `{forbidden}`."
        );
    }
}

#[test]
fn direction_a11y_i18n_contract_is_present_without_hardcoded_copy() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let headless_direction_source = load_source("../../crates/ui-headless/src/direction.rs");

    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "交互语义子项 N/A（direction）：该组件是非交互方向语义容器，不暴露可聚焦交互元素，因此无 `role`/`aria-*`/键盘路径约束面。",
        "A11y/i18n 接入：`view.rs` 通过 `ui_headless::use_direction` 挂载 `lang`/`dir`，并透传 `data-direction` 语义标记。",
        "文案约束：组件 `view.rs` 不包含用户可见文案；可见文本由调用方 children/应用层 i18n 提供。",
        "共享工具来源：方向与 locale 语义由 `crates/ui-headless/src/direction.rs` 内部调用 `crates/ui-headless/src/a11y.rs`（`locale_attrs`）统一处理，组件层不重复实现。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 a11y-i18n marker should keep `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};",
        "let contract = use_direction(DirectionA11yOptions { direction, lang });",
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-direction=contract.attrs.data_direction",
    ] {
        assert!(
            view_source.contains(needle),
            "Direction view should keep a11y/i18n mount fragment `{needle}`."
        );
    }

    for forbidden in ["Name → Value", "الاسم ← القيمة", "aria-label=", "role="] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view should not hardcode copy or interactive semantic token `{forbidden}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "pub fn use_direction(options: DirectionOptions) -> DirectionContract",
        "let locale = locale_attrs(options.lang, Some(a11y_dir));",
    ] {
        assert!(
            headless_direction_source.contains(needle),
            "ui-headless direction should depend on shared a11y helper `{needle}`."
        );
    }
}

#[test]
fn direction_state_markers_are_observable_queryable_and_enumerable() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "direction 关键状态轴通过 `data-direction` 暴露（封闭集合：`ltr|rtl`），状态来源通过 `data-direction-source` 暴露（封闭集合：`direction|dir-alias|default`）。",
        "选择器约束：自动化与回归测试以 `data-slot=\"direction-provider\"` + `data-direction` + `data-direction-source` 为主，不依赖 DOM 结构顺序。",
        "`aria-*` 子项 N/A（direction）：该组件为非交互语义容器，无 disabled/selected/focus-visible/loading 等交互态，不额外定义组件私有 `aria-*` 状态轴。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 state-marker marker should keep `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
        "Self::Direction => \"direction\"",
        "Self::DirAlias => \"dir-alias\"",
        "Self::Default => \"default\"",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Direction should keep stable marker token `{needle}`."
        );
    }

    for forbidden in [
        "open",
        "expanded",
        "disabled",
        "selected",
        "focus-visible",
        "loading",
        ":nth-child",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view should not include unrelated interactive marker token `{forbidden}`."
        );
    }
}

#[test]
fn direction_styles_depend_on_stable_markers_or_class_not_dom_shape() {
    let check_source = load_source("../../components/direction/check2.md");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let view_source = load_source("../../components/direction/src/view.rs");

    for needle in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "direction 当前样式为静态稳定 class（`.ui-direction-provider`），无状态分支选择器，不依赖 DOM 结构猜测。",
        "`view.rs` 不透传业务内联样式（无 `style=` 业务逻辑），仅挂载语义标记与 class。",
        "状态解释路径固定：方向状态由 `data-direction`/`data-direction-source` 语义标记提供；样式层不通过节点存在性做隐式推断。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 style-explicit-state marker should keep `{needle}`."
        );
    }

    assert!(
        styles_source.contains(".ui-direction-provider"),
        "Direction styles should keep stable class selector marker."
    );

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        " > ",
        " + ",
        " ~ ",
        "[data-direction]",
        "[aria-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles should not include fragile selector token `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "style:--"] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view should not include runtime inline style token `{forbidden}`."
        );
    }
}

#[test]
fn direction_tests_prioritize_semantic_contracts_over_snapshots() {
    let check_source = load_source("../../components/direction/check2.md");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");
    let cross_crate_semantics_source = load_source("tests/direction_semantics.rs");

    for needle in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "语义契约覆盖：`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 均断言 `lang/dir/data-direction/data-direction-source` 与来源封闭集合。",
        "分支矩阵 N/A（direction）：该组件无受控/非受控轴、无 disabled/键盘/指针交互路径、无 wasm/ssr 分支差异逻辑；适用分支已由语义标记断言覆盖。",
        "快照策略：当前 direction 组件测试不使用 snapshot 断言替代语义契约，主验证路径为可枚举语义标记与来源契约断言。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 semantic-test marker should keep `{needle}`."
        );
    }

    for needle in [
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
        "Self::Direction => \"direction\"",
        "Self::DirAlias => \"dir-alias\"",
        "Self::Default => \"default\"",
    ] {
        assert!(
            component_semantics_source.contains(needle)
                || cross_crate_semantics_source.contains(needle),
            "Direction semantic tests should include contract marker `{needle}`."
        );
    }

    for forbidden in [
        "assert_snapshot",
        "assert_debug_snapshot",
        "assert_json_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden)
                && !cross_crate_semantics_source.contains(forbidden),
            "Direction tests should not include snapshot assertion token `{forbidden}`."
        );
    }
}

#[test]
fn direction_component_file_responsibilities_are_well_scoped() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");

    for needle in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "`mod.rs` 当前仅维护模块边界与稳定导出（`DirectionMode`/`DirectionProvider`），未承载实现细节。",
        "`logic.rs` 仅包含输入归一与来源标记（`resolve_direction`/`DirectionPropSource`/`compose_class_name`），无 DOM 操作。",
        "`styles.rs` 仅包含静态 `.ui-direction-provider` 样式规则，不含主题常量分支与业务语义文案。",
        "`view.rs` 仅负责 Leptos 结构渲染与 `ui_headless::use_direction` 契约挂载，关键状态决策统一来自 `logic.rs`。",
        "`motion.rs` 子项 N/A（direction）：该组件无动效语义与 attach 需求，目录不含 `motion.rs`，且不依赖 `ui-motion`。",
    ] {
        assert!(
            check_source.contains(needle),
            "direction/check2 file-responsibility marker should keep `{needle}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{DirectionMode, DirectionProvider};",
    ] {
        assert!(
            mod_source.contains(needle),
            "Direction mod.rs should include boundary token `{needle}`."
        );
    }
    for forbidden in [
        "pub mod view",
        "pub mod logic",
        "mod motion;",
        "pub mod motion",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Direction mod.rs should not expose implementation token `{forbidden}`."
        );
    }

    for needle in [
        "pub enum DirectionPropSource",
        "pub fn resolve_direction(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Direction logic.rs should include normalization token `{needle}`."
        );
    }
    for forbidden in ["view!", "web_sys", "HtmlElement", "style="] {
        assert!(
            !logic_source.contains(forbidden),
            "Direction logic.rs should not contain view/dom/style token `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains(".ui-direction-provider"),
        "Direction styles.rs should keep static class rule token."
    );
    for forbidden in ["if ", "match ", "var(--ui-", "color:", "content:"] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles.rs should not include style-branch/theme-copy token `{forbidden}`."
        );
    }

    for needle in [
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "let contract = use_direction(DirectionA11yOptions { direction, lang });",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Direction view.rs should include structure + headless mount token `{needle}`."
        );
    }
    for forbidden in [
        "match direction",
        "if let Some(direction) = direction",
        "attach_motion",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view.rs should not include hidden state/motion token `{forbidden}`."
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "Direction component should remain motion-free for file-responsibility contract."
    );
}

#[test]
fn direction_headless_contract_is_exported_from_ui_headless() {
    let source = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "pub mod direction;",
        "pub use direction::{",
        "DirectionContract",
        "use_direction",
    ] {
        assert!(
            source.contains(needle),
            "ui-headless should export direction contract symbol `{needle}`."
        );
    }
}

#[test]
fn direction_component_stays_motion_free_for_static_contract() {
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");

    assert!(
        !cargo_source.contains("ui-motion"),
        "Direction should remain motion-free for static direction context contract."
    );

    for forbidden in [
        "attach_motion",
        "ui_motion",
        "spring",
        "keyframe",
        "transition",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Direction view should not include motion driver fragment `{forbidden}`.",
        );
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles should not include motion implementation fragment `{forbidden}`.",
        );
    }
}

#[test]
fn direction_component_does_not_define_private_theme_token_system() {
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");

    for forbidden in [
        "--ui-direction-",
        "color-scheme",
        "ThemeContext",
        "theme_to_css_variables",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles should not define private theme/token surface `{forbidden}`.",
        );
    }

    assert!(
        !cargo_source.contains("ui-theme"),
        "Direction should not depend on ui-theme directly for this static layout-only contract."
    );
}

#[test]
fn direction_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "pub(super) fn direction_provider() -> AnyView",
        "title=\"DirectionProvider\"",
        "slug=\"direction-provider\"",
        "<DirectionProvider",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_direction docs page should contain `{needle}`."
        );
    }
}

#[test]
fn direction_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "pub(super) fn direction_provider() -> AnyView",
        "title=\"DirectionProvider\"",
        "slug=\"direction-provider\"",
        "description=\"baseline/Radix-compatible direction context wrapper with normalized `direction`/`dir` props and stable slot + data-direction contracts.\"",
        "<Playground",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<Playground title=\"RTL Direction + Class\"",
        "code_signal=rtl_code",
        "<DirectionProvider",
        "DirectionMode::Ltr",
        "DirectionMode::Rtl",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_direction docs page should include `{needle}` for direction primary coverage.",
        );
    }
}

#[test]
fn direction_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");

    for needle in [
        "<DirectionProvider direction=DirectionMode::Ltr>",
        "\"Name → Value\"",
        "direction=DirectionMode::Rtl",
        "class_name=\"docs-direction-rtl\".to_string()",
        "\"الاسم ← القيمة\"",
    ] {
        assert!(
            source.contains(needle),
            "direction docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn direction_spec_file_policy_stays_na_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let protocol_source = load_source("../../components/direction/src/protocol.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let spec_path = manifest_dir.join("../../components/direction/src/spec.rs");

    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "N/A（direction/spec.rs）：direction 属于简单语义容器组件，当前不引入 `src/spec.rs`，组件说明保持在 `check2.md` 与 docs 页面。",
        "当前若需最小协议约束，使用 `protocol.rs` 的版本化结构体（`schema_version`）承载；不把其升级为复杂 `spec.rs` builder 体系。",
        "约束：未来若确需新增 `spec.rs`，必须同步提交契约测试、版本迁移说明与引入理由；否则判为不通过。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should preserve spec-file discipline token `{needle}`.",
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Direction mod.rs should not expose spec module token `{forbidden}`.",
        );
    }

    assert!(
        !spec_path.exists(),
        "Direction is a simple component and should not define `../../components/direction/src/spec.rs`.",
    );

    for needle in [
        "schema_version",
        "DirectionSchemaVersion",
        "DirectionProtocolPayload",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Direction protocol.rs should keep minimal schema contract token `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn direction_provider() -> AnyView",
        "slug=\"direction-provider\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Direction docs page should continue serving component documentation token `{needle}`.",
        );
    }
}

#[test]
fn direction_token_first_static_style_contract_is_enforced() {
    let check_source = load_source("../../components/direction/check2.md");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let ui_components_css_source = load_source("src/css.rs");
    let ui_components_root_source = load_source("src/root.rs");

    for needle in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "direction 样式仅定义于 `components/direction/src/styles.rs`（`CSS` 常量）；`crates/ui-components/src/css.rs` 通过 `#[cfg(feature = \"component-direction\")]` 聚合，`UiRoot` 经 `crate::css::push_components_css` 注入。",
        "direction 当前仅保留布局防御规则 `min-inline-size: 0;`；无颜色/间距/圆角/阴影私有常量。视觉 token 子项 N/A（该组件不承载视觉语义设计）。",
        "`view.rs` 不包含 `style=` 内联业务样式逻辑，运行时不拼装组件私有样式，仅挂载稳定 class 与语义标记。",
        "组件实现不引入 Utility-First 或 CSS-in-Rust 机制（如 `@apply`/`tw-`/`stylist`）；Utility 类使用限定在 `apps/*` 应用层示例。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep token-first style contract token `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        ".ui-direction-provider",
        "min-inline-size: 0;",
    ] {
        assert!(
            styles_source.contains(needle),
            "Direction styles should keep static style fragment `{needle}`."
        );
    }

    for forbidden in ["var(--ui-direction-", "@apply", "tw-", "style!(", "css!"] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction styles should not include utility/css-in-rust/private-token token `{forbidden}`."
        );
    }

    assert!(
        ui_components_css_source.contains("#[cfg(feature = \"component-direction\")]")
            && ui_components_css_source.contains("out.push_str(crate::direction::styles::CSS);"),
        "ui-components css.rs should aggregate direction CSS behind component feature gate."
    );

    assert!(
        ui_components_root_source.contains("if inject_components_css.get_value() {")
            && ui_components_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated component CSS via push_components_css."
    );

    assert!(
        !view_source.contains("style="),
        "Direction view should not include inline style business logic."
    );

    for forbidden in ["stylist", "tailwind", "css-in-rust"] {
        assert!(
            !cargo_source.contains(forbidden),
            "Direction component manifest should avoid utility/css-in-rust dependency token `{forbidden}`."
        );
    }
}

#[test]
fn direction_visual_desire_reuses_global_theme_baseline_and_stays_non_visual() {
    let check_source = load_source("../../components/direction/check2.md");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "direction 为语义上下文容器组件（非视觉展示组件）；组件级视觉风格子项 N/A，但必须复用仓库级默认主题基线门禁而非绕过。",
        "docs 基线证据：`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs` 提供 `ThemeVisualBaseline` 页面，并覆盖 `Button/Input/Overlay` 的层级、对比与交互反馈。",
        "e2e 基线证据：`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 固化渲染断言与截图基线（`docs-app-theme-visual-baseline-*.png`，`E2E_VISUAL_BASELINE=on`）。",
        "direction 约束：`styles.rs` 不得引入渐变/阴影/动画等视觉风格常量，避免把全局美学职责回流到语义容器组件。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep visual-desire contract token `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "Theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            baseline_registry_source.contains(needle),
            "Docs pages registry should expose theme visual baseline token `{needle}`."
        );
    }

    for needle in [
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "Theme visual baseline e2e suite should include `{needle}`."
        );
    }

    for forbidden in [
        "background:",
        "color:",
        "box-shadow:",
        "text-shadow:",
        "border-radius:",
        "linear-gradient(",
        "transition:",
        "animation:",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Direction should remain non-visual utility and avoid style token `{forbidden}`."
        );
    }
}

#[test]
fn direction_tree_shaking_contract_is_feature_gated_and_budgeted() {
    let check_source = load_source("../../components/direction/check2.md");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget_env = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "direction package 特性门：`crates/ui-components/Cargo.toml` 中 `component-direction = [\"dep:ui-direction\"]`；未启用该 feature 时，`lib.rs` 不导出 `direction` 模块。",
        "direction 样式裁剪门：`crates/ui-components/src/css.rs` 仅在 `#[cfg(feature = \"component-direction\")]` 下聚合 `crate::direction::styles::CSS`，不存在无条件 direction CSS 注入。",
        "source 模式裁剪：按需引入 `components/direction/src/*` 即可，不依赖方向组件专属中央注册表；方向组件不引入额外可达映射表。",
        "仓库级树摇门禁：`scripts/check-ui-components-tree-shaking.sh` 固化最小特性树、反向依赖树、最小 wasm 编译与 release 体积预算校验。",
        "预算基线：`scripts/tree_shaking_budget.env` 维护 `TREE_SHAKING_BASELINE_RLIB_BYTES` 与 `TREE_SHAKING_MAX_RATIO_PERCENT`，阻断体积回归。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep tree-shaking contract token `{needle}`."
        );
    }

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "web-demo-components = [",
        "component-direction = [\"dep:ui-direction\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo should keep tree-shaking feature marker `{needle}`."
        );
    }

    assert!(
        ui_components_lib.contains(
            "#[cfg(feature = \"component-direction\")]\npub use ui_direction as direction;"
        ),
        "ui-components lib.rs should feature-gate direction export.",
    );
    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-direction\")]\n    out.push_str(crate::direction::styles::CSS);"
        ),
        "ui-components css.rs should feature-gate direction CSS aggregation.",
    );

    assert!(
        web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components.",
    );
    assert!(
        docs_app_cargo.contains("features = [\"inject-css\", \"all-components\"]"),
        "docs-app should explicitly opt into all-components to keep feature intent visible.",
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should keep guard `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget_env.contains(needle),
            "tree-shaking budget env should keep marker `{needle}`.",
        );
    }
}

#[test]
fn direction_type_system_and_semantic_markers_form_machine_readable_contract() {
    let check_source = load_source("../../components/direction/check2.md");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/direction.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "direction 离散轴全部类型化：输入为 `Option<DirectionMode>`（`DirectionMode::{Ltr,Rtl}`），来源轴为 `DirectionPropSource::{Direction,DirAlias,Default}`；无字符串协议与 bool 组合状态机。",
        "无效输入组合在类型层受限，缺省路径在 `logic::resolve_direction` 统一归一为 `DirectionMode::default()` 并携带来源标记，归一规则由 `test/logic.rs` 与语义测试共同覆盖。",
        "机器可读语义输出稳定：`view.rs` 挂载 `data-direction`（`ltr|rtl`）与 `data-direction-source`（`direction|dir-alias|default`）封闭集合，便于自动化检索与 Agent 消费。",
        "可持续反馈闭环：`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 对类型轴、归一路径与语义标记进行回归锁定，破坏契约可直接定位到具体层（primitive/logic/view）。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep typed-machine-readable contract token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionMode",
        "Ltr",
        "Rtl",
        "pub fn as_attr(self) -> &'static str",
    ] {
        assert!(
            primitive_source.contains(needle),
            "direction primitive should keep typed axis token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionPropSource",
        "Direction",
        "DirAlias",
        "Default",
        "pub fn resolve_direction(",
        "(DirectionMode::default(), DirectionPropSource::Default)",
        "\"direction\"",
        "\"dir-alias\"",
        "\"default\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep normalization/source token `{needle}`.",
        );
    }

    for needle in [
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should expose machine-readable marker `{needle}`.",
        );
    }

    for forbidden in ["is_ltr", "is_rtl", "Option<bool>"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "direction implementation should avoid bool-explosion token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_type_system_and_semantic_markers_form_machine_readable_contract()"
        ),
        "component-local semantics suite should keep mirrored machine-readable contract regression.",
    );
}

#[test]
fn direction_focus_stack_contract_is_not_applicable_and_private_node_refs_are_forbidden() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A（direction）：该组件为方向语义 provider，不创建 overlay、不管理焦点转移与恢复栈，不存在层叠弹层焦点回收协议面。",
        "当前实现无 `NodeRef` 持有、无 `document.body` 焦点回退、无私有 Focus Manager 分支；`view.rs` 仅挂载 `lang/dir/data-*` 语义标记。",
        "约束：若未来在 direction 范围新增 overlay/焦点恢复能力，必须接入全局 Focus Manager 契约（`FallbackTo/Selector` 等）并补对应语义与 e2e 回归，禁止组件内私存恢复目标节点。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep focus-stack contract token `{needle}`.",
        );
    }

    for forbidden in [
        "NodeRef",
        "node_ref",
        "document.body",
        "FallbackTo",
        "Selector",
        "FocusManager",
        "focus_manager",
        "Overlay",
        "overlay",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "direction implementation should remain focus-stack-free and avoid token `{forbidden}`.",
        );
    }

    for forbidden in ["ui-overlay", "ui-popover", "ui-dialog"] {
        assert!(
            !cargo_source.contains(forbidden),
            "direction Cargo should not depend on overlay/focus-stack crate token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_focus_stack_contract_is_not_applicable_and_private_node_refs_are_forbidden()"
        ),
        "component-local semantics suite should keep mirrored focus-stack contract regression.",
    );
}

#[test]
fn direction_escape_hatches_contract_is_not_applicable_and_foreign_zone_is_forbidden() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A（direction）：该组件仅负责方向语义装配，不集成命令式第三方实例（ECharts/Map/编辑器等），不存在 Foreign Zone 生命周期治理面。",
        "当前实现无第三方实例创建/销毁分支、无 `YieldControl/CleanupForeign`、无实例句柄透传到公共 API；`DirectionProvider` 对外仅暴露方向语义 props 与 children。",
        "约束：若未来确需接入命令式第三方能力，必须落入受控 Foreign Zone 并提供 `YieldControl/CleanupForeign` 清理语义；禁止把第三方实例或其原生句柄暴露到组件公共 API。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep escape-hatches contract token `{needle}`.",
        );
    }

    for forbidden in [
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "AMap",
        "BMap",
        "editor_instance",
        "third_party",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "direction implementation should remain foreign-zone-free and avoid token `{forbidden}`.",
        );
    }

    for forbidden in [
        "echarts", "mapbox", "leaflet", "gmap", "amap", "bmap", "monaco",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "direction Cargo should not depend on third-party imperative integration token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_escape_hatches_contract_is_not_applicable_and_foreign_zone_is_forbidden()",
        ),
        "component-local semantics suite should keep mirrored escape-hatches contract regression.",
    );
}

#[test]
fn direction_ssr_hydration_discontinuity_contract_uses_deterministic_id_provider_path() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let ui_components_root_source = load_source("src/root.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A（direction-local ID）：direction 组件不生成本地随机 ID，不依赖 `now()`/原生 UUID；`logic.rs` 仅做方向归一与来源标记，`view.rs` 仅挂载语义 attrs。",
        "确定性注入路径：`UiRoot` 通过 `id_seed` 调用 `provide_ui_id_provider(id_seed)`，统一提供可预测 ID 种子；direction 作为语义 provider 消费该全局上下文契约，不自建随机源。",
        "当前实现无 hydration 破坏源：组件代码中不存在 `SystemTime::now`/`js_sys::Date::now`/`uuid`/`rand` 初始化路径。",
        "约束：若 future 在 direction 引入需跨端稳定的局部 ID，必须改为消费 `IdProvider`，禁止在组件内直接引入随机或时间源。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep hydration-discontinuity contract token `{needle}`.",
        );
    }

    assert!(
        ui_components_root_source.contains("#[prop(optional, default = 1)] id_seed: u64")
            && ui_components_root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should keep deterministic id-seed injection path via provide_ui_id_provider."
    );
    assert!(
        headless_lib_source.contains(
            "pub use id_provider::{UiIdProvider, provide_ui_id_provider, use_ui_id_provider};"
        ),
        "ui-headless should export id-provider contract from a stable entrypoint."
    );

    for forbidden in [
        "SystemTime::now",
        "Date::now",
        "js_sys::Date::now",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "random::<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "direction implementation should avoid hydration-unstable token `{forbidden}`.",
        );
    }

    for forbidden in ["uuid", "rand"] {
        assert!(
            !cargo_source.contains(forbidden),
            "direction Cargo should not declare random-id dependency token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_ssr_hydration_discontinuity_contract_uses_deterministic_id_provider_path()",
        ),
        "component-local semantics suite should keep mirrored hydration-discontinuity contract regression.",
    );
}

#[test]
fn direction_ssr_and_cross_platform_compile_contract_is_documented_and_non_wasm_safe() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "compile-only 证据：`scripts/check-ui-components-platforms.sh` 覆盖默认 native（`cargo check -p ui-components`）、ssr native（`cargo check -p ui-headless --no-default-features --features ssr`）、web wasm（`cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web` 与 `cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css`）。",
        "平台分支治理：平台差异通过 target/feature 显式声明，不依赖运行时偶然行为；`ui-headless` 另有 web/ssr 互斥编译守卫。",
        "direction non-wasm 安全：`components/direction/src/mod.rs`、`components/direction/src/logic.rs`、`components/direction/src/view.rs` 不引用 `web-sys`/`window`/`document` 等浏览器对象。",
        "本地验证状态：已核对上述 compile-only 路径与源码约束；当前环境执行 `cargo` 仍可能触发 `Invalid cross-device link (os error 18)`，待环境恢复后重跑完整平台脚本。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep cross-platform contract token `{needle}`.",
        );
    }

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui-components",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: web wasm path\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform check script should preserve compile-only evidence `{needle}`.",
        );
    }

    for forbidden in ["web_sys", "window", "document", "js_sys"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "direction source should remain non-wasm safe without browser token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_ssr_and_cross_platform_compile_contract_is_documented_and_non_wasm_safe()",
        ),
        "component-local semantics suite should keep mirrored cross-platform contract regression.",
    );
}

#[test]
fn direction_ui_headless_web_ssr_mutex_is_compile_error_guarded() {
    let check_source = load_source("../../components/direction/check2.md");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "互斥守卫存在：`crates/ui-headless/src/lib.rs` 声明 `#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(\"features \\\\`web\\\\` and \\\\`ssr\\\\` are mutually exclusive; enable exactly one\")`。",
        "direction 依赖路径未破坏约束：`components/direction/src/view.rs` 仅通过 `use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};` 消费 headless 语义契约，不引入并行 feature 绕过路径。",
        "双路径 compile-only 证据：`scripts/check-ui-components-platforms.sh` 包含 `cargo check -p ui-headless --no-default-features --features ssr` 与 `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`。",
        "互斥失败证据：同脚本含 `cargo check -p ui-headless --no-default-features --features web,ssr` 预期失败，并校验日志包含 `mutually exclusive`，防止“web+ssr 同开仍过编译”回归。",
        "本地验证状态：已核对源码守卫与脚本契约；当前环境执行 `cargo` 仍可能触发 `Invalid cross-device link (os error 18)`，待环境恢复后重跑脚本实测。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep ui-headless mutex contract token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless lib should keep web/ssr mutex guard `{needle}`.",
        );
    }

    assert!(
        view_source.contains(
            "use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};"
        ),
        "direction view should consume headless contract through stable import path."
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform check script should keep ui-headless mutex check fragment `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_ui_headless_web_ssr_mutex_is_compile_error_guarded()",),
        "component-local semantics suite should keep mirrored ui-headless mutex contract regression.",
    );
}

#[test]
fn direction_ui_motion_non_wasm_stub_contract_keeps_ssr_tooling_buildable() {
    let check_source = load_source("../../components/direction/check2.md");
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "stub 证据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(&(), ..)` no-op 实现；并含 `non_wasm_web_backend_is_predictable_noop` 测试。",
        "direction 组件降级路径：`components/direction/Cargo.toml` 不依赖 `ui-motion`，`components/direction/src/view.rs` 不调用 `attach_motion`，不存在“假设动画句柄必定存在”的前提。",
        "平台脚本证据：`scripts/check-ui-components-platforms.sh` 含 `cargo test -p ui-motion --test non_wasm_stub` 与 `cargo check -p ui-motion`，覆盖 SSR/tooling 场景的编译与 stub 回归。",
        "约束：若 future 在 direction 引入 `motion.rs`，必须显式走 non-wasm no-op 分支并保证不会 panic；禁止把 wasm-only 动效路径直接暴露到非 wasm。",
        "本地验证状态：已核对源码与脚本契约；当前环境执行 `cargo` 仍可能触发 `Invalid cross-device link (os error 18)`，待环境恢复后重跑平台脚本实测。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep ui-motion non-wasm contract token `{needle}`.",
        );
    }

    for needle in [
        "//! - Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion lib should keep non-wasm stub fragment `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion non-wasm coverage fragment `{needle}`.",
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "direction Cargo should remain motion-free for predictable non-wasm behavior.",
    );
    assert!(
        !view_source.contains("attach_motion"),
        "direction view should not assume motion driver presence.",
    );

    assert!(
        component_semantics_source.contains(
            "fn direction_ui_motion_non_wasm_stub_contract_keeps_ssr_tooling_buildable()",
        ),
        "component-local semantics suite should keep mirrored ui-motion non-wasm contract regression.",
    );
}

#[test]
fn direction_reduced_motion_ssr_wasm_contract_is_explicitly_na_and_semantics_stable() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "N/A（direction motion）：direction 为静态语义 provider，不含 `motion.rs`、不依赖 `ui-motion`、不执行动画 attach；`reduced-motion` 在该组件无额外分支面。",
        "SSR/hydration 稳定性：`view.rs` 仅输出 `lang/dir/data-direction/data-direction-source` 语义 attrs，`logic::resolve_direction` 为纯函数归一，SSR 与 hydration 首帧语义一致。",
        "wasm/SSR 语义一致：direction 不使用 `#[cfg(target_arch = \"wasm32\")]` 分叉渲染，wasm 侧不引入额外交互语义，契约与 SSR 保持同构。",
        "平台验证证据：`scripts/check-ui-components-platforms.sh` 包含 `cargo check -p ui-motion --target wasm32-unknown-unknown`、`cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css`，并覆盖多组件 `reduced-motion/ssr/wasm` 合同回归，保证基础平台路径持续可编译。",
        "本地验证状态：已核对源码与脚本契约；当前环境执行 `cargo` 仍可能触发 `Invalid cross-device link (os error 18)`，待环境恢复后重跑平台脚本实测。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep reduced-motion/ssr/wasm contract token `{needle}`.",
        );
    }

    assert!(
        !mod_source.contains("mod motion;") && !mod_source.contains("pub mod motion;"),
        "direction module should remain motion-free for explicit N/A boundary."
    );
    assert!(
        !cargo_source.contains("ui-motion"),
        "direction Cargo should not depend on ui-motion for reduced-motion/wasm behavior.",
    );
    for forbidden in [
        "attach_motion",
        "ui_motion",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should keep SSR/wasm semantic isomorphism and avoid `{forbidden}`.",
        );
    }
    for needle in [
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep stable semantic output fragment `{needle}`.",
        );
    }
    for forbidden in [
        "SystemTime::now",
        "Date::now",
        "js_sys::Date::now",
        "Uuid::new_v4",
        "rand::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "direction logic should stay hydration-stable and avoid `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "button_reduced_motion_and_ssr_wasm_semantics_contract_is_enforced",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/ssr/wasm evidence fragment `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_reduced_motion_ssr_wasm_contract_is_explicitly_na_and_semantics_stable()",
        ),
        "component-local semantics suite should keep mirrored reduced-motion/ssr/wasm contract regression.",
    );
}

#[test]
fn direction_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates() {
    let check_source = load_source("../../components/direction/check2.md");
    let performance_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let button_check2_source = load_source("../../components/button/check2.md");
    let input_check2_source = load_source("../../components/text-input/src/input/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A（direction 交互强度）：direction 为静态语义 provider，无高频交互循环、无动画驱动、无内部可变状态；关键路径仅 `logic::resolve_direction` 一次归一与语义 attrs 挂载。",
        "仓库级预算与阻断证据：`scripts/check-ui-components-performance.sh` 已纳入 `button_performance_governance_contract_is_budgeted_traceable_and_blocking`、`input_performance_governance_contract_is_budgeted_traceable_and_blocking`、`docs_perf_probe_budgets_are_wired_for_component_pages`、`perf_render_count_follow_up_is_tracked_in_plan`。",
        "预算基线来源：`components/button/check2.md` 与 `components/text-input/src/input/check2.md` 明确 `Button`/`Input` 初始化渲染次数预算为 `1`，并通过 docs `UiPerfProbe` + e2e 标记形成可重复基线与阻断。",
        "可归因性：direction 输出稳定 `data-direction`/`data-direction-source` 标记，不引入样式分支与动效分支，性能问题可直接归因到语义归一与渲染路径。",
        "`render_count` 后续：`docs/plan/TODO.md` 持续跟踪“建立 `render_count` 自动化回归（Button/Input/Accordion）”，当前按仓库约定使用 mount-only 等价证据过渡。",
        "本地验证状态：已核对性能门禁脚本与上游预算契约；当前环境执行 `cargo` 仍可能触发 `Invalid cross-device link (os error 18)`，待环境恢复后重跑性能脚本实测。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep performance governance contract token `{needle}`.",
        );
    }

    for needle in [
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            performance_script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
    ] {
        assert!(
            button_check2_source.contains(needle) && input_check2_source.contains(needle),
            "Button/Input baseline checklist should include `{needle}` for shared perf budget governance.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf follow-up plan should keep `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_direction(",
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        let found = logic_source.contains(needle) || view_source.contains(needle);
        assert!(
            found,
            "direction source should keep perf-attribution-friendly semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "attach_motion",
        "ui_motion",
        "requestAnimationFrame",
        "set_interval",
        "set_timeout",
        "Effect::new",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should stay mount-only and avoid perf-noise token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates()",
        ),
        "component-local semantics suite should keep mirrored performance governance contract regression.",
    );
}

#[test]
fn direction_view_macro_complexity_stays_single_block_and_flat_structure() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "N/A（direction 复杂布局）：`components/direction/src/view.rs` 仅包含 1 个 `view!` 块，结构为单层 `<div>{children()}</div>` 语义容器，无深嵌套布局与重复模板片段。",
        "子块拆分判定：当前组件无 header/body/item 等复合布局维度，不存在需要抽取语义子块的场景；若 future 引入复合结构，必须先拆为局部渲染函数再装配。",
        "宏展开风险控制：`view.rs` 不包含循环模板（如 `For`/`Indexed`）与大段静态重复片段，编译/wasm 体积异常时优先排查 `view!` 展开体量再决定重构。",
        "可回归证据：组件侧与跨 crate 语义测试锁定“`view!` 次数=1、无深层语义容器嵌套、无重复模板分支”契约，防止宏复杂度回退。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep macro-complexity contract token `{needle}`.",
        );
    }

    assert_eq!(
        view_source.matches("view!").count(),
        1,
        "direction view should keep exactly one view! macro block."
    );

    for needle in [
        "view! {",
        "<div",
        "data-slot=\"direction-provider\"",
        "{children()}",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep flat semantic fragment `{needle}`.",
        );
    }

    for forbidden in [
        "<For",
        "<Indexed",
        "<Show",
        "<Suspense",
        "match ",
        "if ",
        "render_",
        "fn render",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should avoid macro-complexity driver token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_view_macro_complexity_stays_single_block_and_flat_structure()",
        ),
        "component-local semantics suite should keep mirrored macro-complexity contract regression.",
    );
}

#[test]
fn direction_function_first_split_policy_keeps_single_component_entrypoint() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "N/A（direction 片段规模）：`components/direction/src/view.rs` 仅保留一个对外语义组件 `DirectionProvider`，当前无可复用的 header/body/item 子片段，不存在“新增局部 `#[component]`”压力面。",
        "升级边界：仅当出现独立 props 语义与可复用契约时才允许新增 `#[component]`；否则必须优先提取为普通渲染函数（`impl IntoView`/`View`）。",
        "抽象噪音防线：direction 现状要求 `#[component]` 计数维持单一入口（`DirectionProvider`），禁止为局部模板拆分引入额外组件层级。",
        "语义稳定性：拆分策略不得改变 `data-slot/data-direction/data-direction-source` 等对外语义标记，测试选择器保持稳定可定位。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep function-first split contract token `{needle}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "direction view should keep a single #[component] entrypoint."
    );

    for needle in [
        "#[component]",
        "pub fn DirectionProvider(",
        "-> impl IntoView",
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep split-policy semantic fragment `{needle}`.",
        );
    }

    for forbidden in [
        "fn DirectionProviderItem(",
        "#[component]\npub fn Item(",
        "#[component]\npub fn Section(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should avoid local component-noise token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_function_first_split_policy_keeps_single_component_entrypoint()",
        ),
        "component-local semantics suite should keep mirrored function-first split contract regression.",
    );
}

#[test]
fn direction_static_fragments_are_consolidated_and_do_not_bloat_view_templates() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A（direction 静态片段规模）：`components/direction/src/view.rs` 无复杂 SVG/长文案/页脚模板，仅渲染语义容器与 `children` 插槽，不存在重复静态片段构造。",
        "常量化落点：组件静态样式集中在 `components/direction/src/styles.rs` 的 `pub const CSS`，静态资源路径单一且可追踪。",
        "语义保持：该组件无静态富文本注入，不使用 `inner_html`；可访问语义由 `lang/dir/data-*` attrs 直接表达，不因常量化策略变化。",
        "维护约束：若 future 引入复杂静态 SVG/说明文案，必须优先提取为常量/模板并保持集中落点，禁止散落在多个 `view!` 分支。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep static-fragment contract token `{needle}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\"")
            && styles_source.contains(".ui-direction-provider"),
        "direction styles should keep single static CSS constant as the consolidated static fragment."
    );

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "Lorem ipsum",
        "repeat(",
        "concat!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should avoid static-template bloat token `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("{children()}"),
        "direction view should remain a slot wrapper rather than duplicating static long-form content."
    );

    assert!(
        component_semantics_source.contains(
            "fn direction_static_fragments_are_consolidated_and_do_not_bloat_view_templates()",
        ),
        "component-local semantics suite should keep mirrored static-fragment contract regression.",
    );
}

#[test]
fn direction_inner_html_contract_forbids_dynamic_injection_and_keeps_semantics_stable() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A（direction 富文本注入面）：`components/direction/src/view.rs` 不使用 `inner_html`，仅输出 `lang/dir/data-*` attrs 与 `children` 插槽，不存在 HTML 字符串注入路径。",
        "安全边界：组件无远端 HTML 输入、无模板拼接、无未清洗字符串直写 DOM；`logic.rs` 仅做方向归一与 class 组合，不承载富文本渲染职责。",
        "回归保障：组件侧与跨 crate 语义测试显式断言 `inner_html/dangerously_set_inner_html/set_inner_html` 缺席，并锁定语义标记输出不变。",
        "维护约束：若 future 必须引入 `inner_html`，仅允许受信任静态常量/白名单来源，并必须同步补齐语义与安全回归用例后方可通过。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep inner_html contract token `{needle}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "set_inner_html",
        "insert_adjacent_html",
        "from_html_unchecked",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "direction implementation should forbid html injection token `{forbidden}`.",
        );
    }

    for needle in [
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep stable semantic marker `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_inner_html_contract_forbids_dynamic_injection_and_keeps_semantics_stable()",
        ),
        "component-local semantics suite should keep mirrored inner_html contract regression.",
    );
}

#[test]
fn direction_wasm_debug_contract_is_globally_traceable_feature_isolated_and_non_polluting() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let docs_app_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let docs_app_debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let ui_components_cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A（direction 关键交互回放）：direction 为语义 provider，无本地状态机与高频交互链，不存在组件内“事件顺序/状态转移”回放面。",
        "全局可追踪能力：`crates/ui-headless/src/trace.rs` 提供 `UiTraceEvent { ts_ms, component, kind }` 与 `provide_ui_trace/use_ui_trace`；`apps/docs-app/src/lib.rs` 在 `cfg!(debug_assertions)` 下启用 trace。",
        "可视化入口：`apps/docs-app/src/debug_overlay.rs` 提供 `UiDebugOverlay`（含 inspect/events 面板），开发模式下在 docs-app 挂载。",
        "feature 隔离：`crates/ui-components/Cargo.toml` 提供 `*-wasm-debug` feature（如 `accordion-wasm-debug`/`button-wasm-debug` 等），默认生产路径不暴露调试 API。",
        "约束：direction 不新增私有 debug/replay 接口；若 future 引入复杂交互，必须接入 `UiTrace` 并通过 feature gate 隔离。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep wasm-debug contract token `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent",
        "pub ts_ms: u64",
        "pub component: &'static str",
        "pub kind: UiTraceEventKind",
        "pub fn provide_ui_trace(enabled: bool) -> UiTrace",
        "pub fn use_ui_trace() -> Option<UiTrace>",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep wasm-debug traceability fragment `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_lib_source.contains(needle),
            "docs app should keep debug overlay wiring fragment `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(",
        "#[cfg(target_arch = \"wasm32\")]",
        "ui_headless::UiTraceEventKind::Inspect",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            docs_app_debug_overlay_source.contains(needle),
            "docs debug overlay should keep visual debug entry fragment `{needle}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "calendar-wasm-debug = [\"component-calendar\", \"ui-calendar/wasm-debug\"]",
        "code-block-wasm-debug = [\"component-code_block\", \"ui-code-block/wasm-debug\"]",
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo_source.contains(needle),
            "ui-components Cargo should keep wasm-debug feature isolation fragment `{needle}`.",
        );
    }

    for forbidden in [
        "UiDebugOverlay",
        "provide_ui_trace",
        "use_ui_trace",
        "UiTraceEventKind",
        "debug_assertions",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "direction implementation should remain non-polluting and avoid debug token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_wasm_debug_contract_is_globally_traceable_feature_isolated_and_non_polluting()",
        ),
        "component-local semantics suite should keep mirrored wasm-debug contract regression.",
    );
}

#[test]
fn direction_dx_contract_uses_playground_hot_reload_and_marks_optional_persist_na() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_components_test_source =
        load_source("../../apps/docs-app/src/pages/components/test/mod.rs");
    let dev_docs_script_source = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script_source = load_source("../../scripts/dev-web-demo.sh");
    let dx_script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "N/A（direction 交互复杂度）：direction 为语义 provider，无内部可变状态与复杂交互，不需要独立 workbench 状态保留面；保持 Playground 单画布路径即可。",
        "样式热反馈路径：`scripts/dev-docs-app.sh` 与 `scripts/dev-web-demo.sh` 均使用 `trunk serve` watch；仓库 DX 门禁 `scripts/check-ui-components-dx.sh` 固化 `*_dx_playground_supports_css_hot_reload_without_wasm_rebuild` 合同。",
        "上下文保持与隔离画布：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 通过 `<Playground ...>` 提供 direction 隔离预览；`apps/docs-app/src/playground.rs` 以 `data-playground-scope` + 控制面板信号保持同页调试上下文。",
        "状态保留策略：direction 当前标记 optional persist 为 N/A；若 future 引入可交互状态，必须补 workbench 与可选状态保留开关，并新增对应 DX 语义回归。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep DX contract token `{needle}`.",
        );
    }

    for needle in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/docs-app\"",
    ] {
        assert!(
            dev_docs_script_source.contains(needle),
            "docs dev script should keep hot-reload dev loop fragment `{needle}`.",
        );
    }

    for needle in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/web-demo\"",
    ] {
        assert!(
            dev_web_script_source.contains(needle),
            "web-demo dev script should keep hot-reload dev loop fragment `{needle}`.",
        );
    }

    for needle in [
        "[dx] contract: playground css hot-reload path",
        "_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            dx_script_source.contains(needle),
            "DX gate script should keep hot-reload/workbench regression token `{needle}`.",
        );
    }

    for needle in [
        "<Playground",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<Playground title=\"RTL Direction + Class\"",
        "code_signal=rtl_code",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep isolated playground entry `{needle}`.",
        );
    }

    for needle in [
        "pub fn Playground(",
        "<section class=\"playground\" id=anchor_id data-slot=\"playground\">",
        "data-playground-scope",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep DX context/isolated-canvas fragment `{needle}`.",
        );
    }

    for needle in [
        "fn every_component_doc_page_renders_at_least_one_playground()",
        "provide_playground_registry()",
    ] {
        assert!(
            docs_components_test_source.contains(needle),
            "docs component tests should keep playground baseline fragment `{needle}`.",
        );
    }

    for forbidden in [
        "signal(",
        "create_signal",
        "set_interval",
        "requestAnimationFrame",
        "workbench",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "direction source should stay non-interactive for DX N/A scope; found `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_dx_contract_uses_playground_hot_reload_and_marks_optional_persist_na()",
        ),
        "component-local semantics suite should keep mirrored DX contract regression.",
    );
}

#[test]
fn direction_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let check_source = load_source("../../components/direction/check2.md");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let protocol_source = load_source("../../components/direction/src/protocol.rs");
    let protocol_test_source = load_source("../../components/direction/test/protocol.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let engineering_script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "serde 路径（direction 适用）：`components/direction/src/protocol.rs` 提供 `DirectionComponentSchemaVersion` 与 `DirectionComponentSpec`，并以 `Serialize/Deserialize + schema_version` 形成结构化协议；`components/direction/test/protocol.rs` 锁定 serde 合同。",
        "tracing 路径（direction 轻组件 N/A）：direction 本体不定义私有 tracing target/span/event；交互追踪统一走全局 `ui-headless` trace 契约，避免组件各自发明埋点语义。",
        "async/runtime 边界：direction 无异步状态机与 runtime 绑定；`view.rs`/`logic.rs`/`mod.rs` 不暴露 `tokio`/`async-std`/runtime 类型到公共 API。",
        "仓库级门禁证据：`scripts/check-ui-components-engineering.sh` 固化 `serde schema + tracing semantics + runtime leakage` 三类合同检查，direction 需持续满足同一工程基线。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep engineering contract token `{needle}`.",
        );
    }

    for needle in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "pub mod protocol;",
    ] {
        let found = cargo_source.contains(needle) || mod_source.contains(needle);
        assert!(
            found,
            "direction component should keep serde protocol wiring fragment `{needle}`."
        );
    }

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum DirectionComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct DirectionComponentSpec",
        "pub schema_version: DirectionComponentSchemaVersion",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "direction protocol should keep structured serde schema fragment `{needle}`.",
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<DirectionComponentSchemaVersion>();",
        "assert_serde::<DirectionComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "direction protocol tests should keep serde contract fragment `{needle}`.",
        );
    }

    for needle in [
        "[engineering] contract: serde schema + structured migration errors",
        "[engineering] contract: tracing target semantics",
        "[engineering] contract: runtime boundary leakage",
    ] {
        assert!(
            engineering_script_source.contains(needle),
            "engineering gate script should keep baseline contract `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent",
        "pub component: &'static str",
        "pub kind: UiTraceEventKind",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace source should keep unified tracing contract fragment `{needle}`.",
        );
    }

    for forbidden in [
        "tracing::",
        "span!(",
        "event!(",
        "instrument(",
        "tokio::",
        "async_std::",
        "smol::",
        "Runtime",
        "Handle",
        "async fn",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "direction source should avoid engineering boundary leak token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries()",
        ),
        "component-local semantics suite should keep mirrored engineering contract regression.",
    );
}

#[test]
fn direction_styles_use_defensive_variable_fallback_chain() {
    let check_source = load_source("../../components/direction/check2.md");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let ui_theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let contract_hygiene_script_source =
        load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "direction 样式已改为双层链：`min-inline-size: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none))`，不再使用裸尺寸终值。",
        "Fallback SSOT：`--ui-fallback-min-inline-size-none` 由 `crates/ui-theme/src/css.rs` 统一输出，组件层仅消费变量。",
        "约束：`components/direction/src/styles.rs` 禁止新增 Hex/RGB/裸像素终值 fallback；新增样式必须先走 token 与 fallback 变量链。",
        "回归：direction 组件侧与 `crates/ui-components` 镜像语义测试锁定变量链与“无硬编码颜色/尺寸终值”契约。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep defensive-variable contract token `{needle}`.",
        );
    }

    for needle in [
        ".ui-direction-provider",
        "min-inline-size: var(",
        "--ui-min-inline-size-none",
        "var(--ui-fallback-min-inline-size-none)",
    ] {
        assert!(
            styles_source.contains(needle),
            "direction styles should keep defensive variable chain fragment `{needle}`.",
        );
    }

    for forbidden in ["min-inline-size: 0;", "rgb(", "rgba(", "hsl(", "hsla("] {
        assert!(
            !styles_source.contains(forbidden),
            "direction styles should avoid hard-coded terminal token `{forbidden}`.",
        );
    }

    assert!(
        ui_theme_css_source.contains("--ui-fallback-min-inline-size-none")
            && ui_theme_css_source.contains("0px"),
        "ui-theme css should define fallback terminal for min-inline-size SSOT."
    );

    for needle in [
        "styles keep defensive fallback chain with ui-theme SSOT terminals",
        "button_styles_use_defensive_variable_fallback_chain_locally",
    ] {
        assert!(
            contract_hygiene_script_source.contains(needle),
            "contract hygiene script should keep defensive-variable baseline token `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_styles_use_defensive_variable_fallback_chain()"),
        "component-local semantics suite should keep mirrored defensive-variable contract regression.",
    );
}

#[test]
fn direction_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let ui_components_css_source = load_source("src/css.rs");
    let ui_components_root_source = load_source("src/root.rs");
    let contract_hygiene_script_source =
        load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "聚合层证据：`crates/ui-components/src/css.rs` 在 `push_components_css` 入口以 `@layer ui` 包裹组件样式，并通过 `#[cfg(feature = \\\"component-direction\\\")] out.push_str(crate::direction::styles::CSS);` 聚合 direction CSS。",
        "组件运行时样式约束：`components/direction/src/view.rs` 不包含 `style=`/`style:`，direction 不在运行时拼接普通内联样式。",
        "数值调整策略（direction N/A）：当前组件无运行时数值调整路径；若 future 引入动态数值，必须仅通过 CSS Custom Properties（`style:--*`）透传，不得写 `style=\\\"top: ...\\\"` 等普通内联样式。",
        "回归：direction 组件侧与 `crates/ui-components` 镜像语义测试锁定 `@layer ui` 聚合与“无普通内联 style”契约。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep cascade-layer contract token `{needle}`.",
        );
    }

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-direction\")]",
        "out.push_str(crate::direction::styles::CSS);",
        "out.push_str(\"}\\n\");",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui-components css aggregation should keep cascade-layer fragment `{needle}`.",
        );
    }

    assert!(
        ui_components_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should continue injecting component css aggregated by push_components_css."
    );

    for forbidden in ["style=", "style:top", "style:left", "style:transform"] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should avoid plain inline style token `{forbidden}`.",
        );
    }

    for needle in [
        "css is aggregated in @layer ui and runtime style is css-variable-only",
        "button_cascade_layer_and_runtime_style_contract_is_enforced_locally",
    ] {
        assert!(
            contract_hygiene_script_source.contains(needle),
            "contract hygiene script should keep cascade-layer baseline token `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_cascade_layer_and_runtime_style_contract_is_enforced()"),
        "component-local semantics suite should keep mirrored cascade-layer contract regression.",
    );
}

#[test]
fn direction_motion_contract_is_explicitly_na_and_keeps_reduced_motion_noop_guards() {
    let check_source = load_source("../../components/direction/check2.md");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A（direction motion 面）：direction 为静态语义 provider，不存在 open/close/enter/exit 动效语义；目录无 `motion.rs`，不调用 `attach_motion`。",
        "reduced-motion/no-op 继承约束：`components/direction/Cargo.toml` 不依赖 `ui-motion`；该组件不自行实现动画执行器，non-wasm/SSR 安全降级由 `crates/ui-motion` 统一 no-op 契约兜底。",
        "平台门禁证据：`scripts/check-ui-components-platforms.sh` 固化 `cargo check -p ui-motion`、`cargo test -p ui-motion --test non_wasm_stub` 与多组件 motion contractualization 回归，确保 motion 基础设施持续满足 reduced-motion + non-wasm 可编译。",
        "约束：若 future 为 direction 引入 `motion.rs`，必须定义组件级 motion contract（参数与 attach 路径）并补 `reduced-motion + non-wasm no-op` 语义测试，禁止组件内自研 spring/driver。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep motion-contract token `{needle}`.",
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "direction Cargo should stay motion-runtime free for this N/A scope."
    );
    assert!(
        !mod_source.contains("mod motion;") && !mod_source.contains("pub mod motion;"),
        "direction module should not define a motion.rs module in current scope."
    );

    for forbidden in [
        "attach_motion",
        "stiffness",
        "damping",
        "prefers_reduced_motion",
        "animate(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "direction source should avoid motion-contract token `{forbidden}` in N/A scope.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo test -p ui-motion --test non_wasm_stub",
        "motion contractualization",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep motion baseline token `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_motion_contract_is_explicitly_na_and_keeps_reduced_motion_noop_guards()",
        ),
        "component-local semantics suite should keep mirrored motion-contract regression.",
    );
}

#[test]
fn direction_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check_source = load_source("../../components/direction/check2.md");
    let ui_components_lib_source = load_source("src/lib.rs");
    let ui_components_css_source = load_source("src/css.rs");
    let ui_components_root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state_source =
        load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "direction 入口落点：`lib.rs` 已在 `#[cfg(feature = \"component-direction\")]` 下 `pub use ui_direction as direction;`，并仅导出 `DirectionMode/DirectionProvider` 公共语义面。",
        "direction CSS 落点：`css.rs::push_components_css` 在 `@layer ui` 中按 `component-direction` 条件聚合 `crate::direction::styles::CSS`，无无条件注入。",
        "UiRoot 中央注入：`root.rs` 统一 `BASE_CSS + theme vars + optional components css`，并通过 `provide_ui_i18n` 提供全局 i18n 上下文。",
        "共享高亮能力落点：`ui-visual-primitive/src/active_highlight.rs` 仅提供 `CSS + ActiveHighlightMotion + attach_active_highlight_motion` 通用能力。",
        "禁止文件落点满足：`crates/ui-components/src/overlay_open.rs`、`presence.rs`、`a11y.rs` 均不存在；对应原语固定在 `ui-headless`。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep ui-components fixed-entry token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-direction\")]",
        "pub use ui_direction as direction;",
        "pub use direction::{DirectionMode, DirectionProvider};",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui-components lib entry should keep `{needle}` for direction.",
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-direction\")]",
        "out.push_str(crate::direction::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui-components css entry should keep `{needle}`.",
        );
    }

    for needle in [
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "ui-components root entry should keep `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "ui-visual-primitive active_highlight should keep `{needle}`.",
        );
    }

    for needle in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            headless_controllable_state_source.contains(needle),
            "ui-headless controllable_state should keep `{needle}`.",
        );
    }
    assert!(
        headless_presence_source.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "ui-headless presence primitive should expose use_presence contract.",
    );
    assert!(
        headless_a11y_source.contains("pub fn aria_controls_when_open("),
        "ui-headless a11y primitive should expose aria_controls_when_open contract.",
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for missing in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        let path = manifest_dir.join(missing);
        assert!(
            !path.exists(),
            "ui-components forbidden entry file should stay absent: {missing}",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_ui_components_fixed_entry_files_follow_layered_boundaries()"),
        "component-local semantics suite should keep mirrored ui-components fixed-entry regression.",
    );
}

#[test]
fn direction_component_directory_standard_files_are_correct() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let cargo_source = load_source("../../components/direction/Cargo.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "`components/direction/src/mod.rs` 仅维护 `logic/styles/view/protocol` 边界并导出 `DirectionMode/DirectionProvider`；未过度公开内部实现模块。",
        "`components/direction/src/logic.rs` 仅做 `resolve_direction + DirectionPropSource + compose_class_name` 归一/来源标记，不含 DOM 操作与可下沉状态原语重写。",
        "`components/direction/src/styles.rs` 仅包含静态 `.ui-direction-provider` CSS，尺寸通过 `var(--ui-*, var(--ui-fallback-*))` 变量链消费，无写死主题常量。",
        "`components/direction/src/view.rs` 仅做 Leptos 结构渲染与 `ui_headless::use_direction` 语义挂载，关键状态统一来自 `logic::resolve_direction`；目录不存在 `render.rs`。",
        "`motion.rs` 子项 N/A（direction）：该组件为非交互语义 provider，目录无 `motion.rs`，不依赖 `ui-motion`，无 `XxxMotion/attach_motion` 实现面。",
        "`spec.rs` 子项 N/A（direction）：该组件保持简单 API，目录无 `spec.rs`，仅 button 等复杂组件需要该落点。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep directory-standard-files token `{needle}`.",
        );
    }

    for needle in [
        "mod logic;",
        "pub mod protocol;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{DirectionMode, DirectionProvider};",
    ] {
        assert!(
            mod_source.contains(needle),
            "direction mod.rs should keep boundary fragment `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod render;",
        "pub mod render;",
        "mod motion;",
        "pub mod motion;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "direction mod.rs should avoid boundary leak token `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum DirectionPropSource",
        "pub fn resolve_direction(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic.rs should keep normalization fragment `{needle}`.",
        );
    }

    for forbidden in [
        "view!",
        "web_sys",
        "wasm_bindgen",
        "NodeRef",
        "pub enum DirectionMode",
        "impl DirectionMode",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "direction logic.rs should avoid leaked concern token `{forbidden}`.",
        );
    }

    for needle in [
        "pub const CSS: &str",
        ".ui-direction-provider",
        "var(--ui-min-inline-size-none",
        "var(--ui-fallback-min-inline-size-none)",
    ] {
        assert!(
            styles_source.contains(needle),
            "direction styles.rs should keep static token-first fragment `{needle}`.",
        );
    }

    for forbidden in ["rgb(", "rgba(", "hsl(", "hsla(", "color: #"] {
        assert!(
            !styles_source.contains(forbidden),
            "direction styles.rs should avoid hardcoded theme terminal token `{forbidden}`.",
        );
    }

    for needle in [
        "use ui_headless::{DirectionOptions as DirectionA11yOptions, use_direction};",
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "let contract = use_direction(DirectionA11yOptions { direction, lang });",
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view.rs should keep structure + headless mount fragment `{needle}`.",
        );
    }

    for forbidden in ["mod render", "render.rs", "attach_motion", "XxxMotion"] {
        assert!(
            !view_source.contains(forbidden),
            "direction view.rs should avoid file-boundary drift token `{forbidden}`.",
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "direction component should not depend on ui-motion in this N/A scope.",
    );

    let component_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/direction");
    for required in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        assert!(
            component_dir.join(required).exists(),
            "direction component required file should exist: {required}",
        );
    }
    for forbidden in ["src/render.rs", "src/motion.rs", "src/spec.rs"] {
        assert!(
            !component_dir.join(forbidden).exists(),
            "direction component forbidden/na file should stay absent: {forbidden}",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_component_directory_standard_files_are_correct()"),
        "component-local semantics suite should keep mirrored directory-standard-files regression.",
    );
}

#[test]
fn direction_file_placement_discipline_is_enforced() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");
    let component_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/direction");

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "direction 核心落点满足：`components/direction/src/` 保持 `mod.rs + logic.rs + styles.rs + view.rs` 主路径；`mod.rs` 仅维护导出边界。",
        "`render.rs` 禁止项满足：目录中不存在 `render.rs`，渲染入口固定在 `view.rs`，无平行渲染模块漂移。",
        "`motion.rs` 子项 N/A（direction）：该组件是非交互语义 provider，无动效 contract 语义轴，不引入 `motion.rs` 与 `attach_motion`。",
        "`spec.rs` 子项 N/A（direction）：组件不属于复杂 Schema/builder 组件，目录无 `spec.rs`，避免无收益抽象扩张。",
        "扩展文件说明：`protocol.rs` 用于结构化协议（serde/schema version）并不替代核心落点职责；关键实现职责仍由 `mod/logic/styles/view` 承担。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep file-placement discipline token `{needle}`.",
        );
    }

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/protocol.rs",
    ] {
        assert!(
            component_dir.join(required).exists(),
            "direction file-placement required file should exist: {required}",
        );
    }

    for forbidden in ["src/render.rs", "src/motion.rs", "src/spec.rs"] {
        assert!(
            !component_dir.join(forbidden).exists(),
            "direction file-placement forbidden/na file should stay absent: {forbidden}",
        );
    }

    for forbidden in ["mod render;", "pub mod render;", "mod motion;", "mod spec;"] {
        assert!(
            !mod_source.contains(forbidden),
            "direction mod.rs should avoid file-placement drift token `{forbidden}`.",
        );
    }

    for forbidden in ["attach_motion", "render.rs"] {
        assert!(
            !view_source.contains(forbidden),
            "direction view.rs should avoid file-placement drift token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains("fn direction_file_placement_discipline_is_enforced()"),
        "component-local semantics suite should keep mirrored file-placement discipline regression.",
    );
}

#[test]
fn direction_hyper_structure_builder_contract_is_explicitly_na_for_simple_provider() {
    let check_source = load_source("../../components/direction/check2.md");
    let mod_source = load_source("../../components/direction/src/mod.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let styles_source = load_source("../../components/direction/src/styles.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let protocol_source = load_source("../../components/direction/src/protocol.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");
    let component_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/direction");

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A（direction）：该组件是轻量语义 provider，不属于复杂配置树组件；当前不引入 `src/spec.rs` 与 builder 渲染链。",
        "现有协议边界：`protocol.rs` 仅保留最小版本化结构（`DirectionComponentSpec + schema_version`）用于序列化兼容，不承担 UI builder 职责。",
        "禁止伪 builder：组件目录中不得出现 `*Spec::new()...render()` 风格 API 来包装简单容器语义，避免无收益抽象。",
        "升级触发条件：仅当 direction 演化为复杂 Schema 驱动组件（多层配置、可编排语义树）时，才允许新增 `spec.rs` 并同时补 builder 契约测试与版本迁移说明。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep hyper-structure-builder token `{needle}`.",
        );
    }

    assert!(
        !component_dir.join("src/spec.rs").exists(),
        "direction should keep spec.rs absent in current simple-provider scope.",
    );

    for needle in [
        "pub struct DirectionComponentSpec",
        "pub schema_version: DirectionComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "direction protocol.rs should keep minimal schema fragment `{needle}`.",
        );
    }

    for forbidden in ["Spec::new(", "fn render(", "impl DirectionComponentSpec {"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "direction source should avoid pseudo-builder token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_hyper_structure_builder_contract_is_explicitly_na_for_simple_provider()",
        ),
        "component-local semantics suite should keep mirrored hyper-structure-builder regression.",
    );
}

#[test]
fn direction_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check_source = load_source("../../components/direction/check2.md");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let rbi_source = load_source("../../components/direction/src/direction.rbi");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");
    let component_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/direction");

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "落点满足：`components/direction/src/Component.toml` 与 `components/direction/src/direction.rbi` 已补齐并与当前组件 API 对齐。",
        "Manifest 范围：`Component.toml` 明确 `files/capabilities/markers/agent_contract/output_state`，声明 `spec_builder=false`、`streaming=false` 与 `supports_lang_dir=true`，避免 AI 检索误判能力面。",
        "RBI 范围：`direction.rbi` 固定导出 `DirectionMode` 与 `DirectionProvider(...) -> IntoView` 的签名投影，并提供方向来源与 agent 状态枚举，保持机器可读接口稳定。",
        "边界约束：direction 作为轻量语义 provider，不引入 `Spec::new()...render()` builder 投影；若未来升级为复杂 spec 组件，必须同步演进 `Component.toml/.rbi` 与迁移说明。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep manifest+rbi token `{needle}`.",
        );
    }

    assert!(
        component_dir.join("src/Component.toml").exists(),
        "direction should keep context-compression manifest file present.",
    );
    assert!(
        component_dir.join("src/direction.rbi").exists(),
        "direction should keep RBI projection file present.",
    );

    for needle in [
        "id = \"ui-direction\"",
        "crate = \"ui-direction\"",
        "rbi = \"direction.rbi\"",
        "spec_builder = false",
        "streaming = false",
        "supports_lang_dir = true",
        "\"data-direction\"",
        "\"data-direction-source\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep manifest fragment `{needle}`.",
        );
    }

    for needle in [
        "pub type DirectionMode = ui_state_primitives::direction::DirectionMode;",
        "pub enum DirectionSource",
        "pub fn DirectionProvider(",
        "direction: Option<DirectionMode>",
        "dir: Option<DirectionMode>",
        "lang: Option<String>",
        "class_name: Option<String>",
        "children: leptos::children::Children",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "direction.rbi should keep projection fragment `{needle}`.",
        );
    }

    for forbidden in ["Spec::new(", "fn render(", "builder"] {
        assert!(
            !rbi_source.contains(forbidden),
            "direction.rbi should avoid pseudo-builder token `{forbidden}`.",
        );
    }

    for needle in [
        "#[component]",
        "pub fn DirectionProvider(",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should stay aligned with manifest/rbi contract via `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_context_compression_manifest_and_rbi_projection_are_present_and_current()",
        ),
        "component-local semantics suite should keep mirrored manifest+rbi regression.",
    );
}

#[test]
fn direction_agent_contract_schema_markers_are_typed_traceable_and_whitelisted() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let rbi_source = load_source("../../components/direction/src/direction.rbi");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "机器可读语义已落地：`view.rs` 在 `data-direction/data-direction-source` 基础上，新增 `data-ui-schema/version/intent/action/state/source/stream-support/stream-fallback/output-status` 稳定标记。",
        "类型化来源：`logic.rs` 新增 `DirectionAgentIntent/Action/Source/StreamSupport/StreamFallback/OutputStatus` 与 `DirectionAgentContract`，由 `resolve_agent_contract` 统一生成，避免 `view.rs` 散落字符串拼接。",
        "可追溯性：契约字段直接映射方向状态轴与来源轴（`DirectionMode` + `DirectionPropSource`），`intent/action/state/source` 均可枚举并可回归断言。",
        "白名单边界：`Component.toml` 的 `agent_contract.fields` 与 `markers.required` 明确允许字段集合，`direction.rbi` 同步投影签名；组件无 `inner_html`/脚本注入路径，渲染链路保持白名单约束。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep agent-contract token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionAgentIntent",
        "pub enum DirectionAgentAction",
        "pub enum DirectionAgentSource",
        "pub enum DirectionAgentStreamSupport",
        "pub enum DirectionAgentStreamFallback",
        "pub enum DirectionAgentOutputStatus",
        "pub struct DirectionAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep typed agent-contract fragment `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = logic::resolve_agent_contract(direction, direction_source);",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version",
        "data-ui-intent=agent_contract.intent.as_attr()",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-state=agent_contract.state.as_attr()",
        "data-ui-source=agent_contract.source.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep agent marker mount fragment `{needle}`.",
        );
    }

    for needle in [
        "\"data-ui-schema\"",
        "\"data-ui-intent\"",
        "\"data-ui-action\"",
        "\"data-ui-state\"",
        "\"data-ui-source\"",
        "\"data-ui-output-status\"",
        "fields = [\"intent\", \"action\", \"state\", \"source.direction\", \"output_status\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep agent-contract whitelist fragment `{needle}`.",
        );
    }

    for needle in [
        "pub struct DirectionAgentContract",
        "pub enum DirectionAgentIntent",
        "pub enum DirectionAgentAction",
        "pub enum DirectionAgentOutputStatus",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            rbi_source.contains(needle),
            "direction.rbi should keep agent-contract projection fragment `{needle}`.",
        );
    }

    for forbidden in ["inner_html", "dangerously_set_inner_html", "javascript:"] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should avoid script/html injection token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_agent_contract_schema_markers_are_typed_traceable_and_whitelisted()",
        ),
        "component-local semantics suite should keep mirrored agent-contract regression.",
    );
}

#[test]
fn direction_streaming_term_scope_is_documented_and_stays_snapshot_oriented() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "术语口径固定：`Streaming` 指 LLM 生成中增量渲染；`Snapshot` 指完整结果一次性渲染，二者仅描述输出时序，不引入额外交互语义。",
        "N/A（direction）：direction 是方向语义 provider，不是 LLM 正文阅读面，不承载 token/chunk 流式拼接渲染链路。",
        "当前契约落点：组件通过类型化 agent contract 输出 `data-ui-stream-support=\"optional\"` 与 `data-ui-stream-fallback=\"snapshot\"`，默认消费快照态输入。",
        "约束：组件侧禁止引入 `EventSource/WebSocket` 或自定义 chunk buffer；若 future 承载正文流式渲染，必须新增 streaming 专项状态轴与回归测试。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep streaming-term token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionAgentStreamSupport",
        "pub enum DirectionAgentStreamFallback",
        "Self::Optional => \"optional\"",
        "Self::Snapshot => \"snapshot\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep stream contract fragment `{needle}`.",
        );
    }

    for needle in [
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should expose stream marker `{needle}`.",
        );
    }

    for needle in ["streaming = false", "spec_builder = false"] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep snapshot-oriented capability marker `{needle}`.",
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "ReadableStream",
        "onmessage",
        "chunk",
        "stream_buffer",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "direction implementation should avoid streaming transport token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_streaming_term_scope_is_documented_and_stays_snapshot_oriented()"
        ),
        "component-local semantics suite should keep mirrored streaming-term regression.",
    );
}

#[test]
fn direction_snapshot_is_baseline_and_consumes_complete_config_stably() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "direction 已支持快照基线：在接收完整 props（`direction/dir/lang/class_name/children`）后，`logic::resolve_direction` 单次归一并稳定输出语义容器，无增量流式依赖。",
        "完整输入消费：组件可直接消费“完整配置结果”并一次性渲染 `lang/dir/data-direction/data-direction-source + data-ui-*`，不要求额外异步握手。",
        "非正文组件仍适用：尽管 direction 不展示正文内容，但作为语义 provider 必须在 snapshot 输入下稳定提供上下文；当前 `Component.toml` 已声明 `snapshot=true`、`streaming=false`。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep snapshot-baseline token `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] direction: Option<DirectionModeImpl>",
        "#[prop(optional)] dir: Option<DirectionModeImpl>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: Children",
        "let (direction, direction_source) = logic::resolve_direction(direction, dir);",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep complete-config snapshot fragment `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_direction(",
        "(DirectionMode::default(), DirectionPropSource::Default)",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep deterministic snapshot normalization fragment `{needle}`.",
        );
    }

    for needle in ["snapshot = true", "streaming = false"] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep snapshot baseline capability marker `{needle}`.",
        );
    }

    for forbidden in [
        "spawn_local",
        "async fn",
        ".await",
        "EventSource",
        "WebSocket",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "direction snapshot baseline should avoid async/stream transport token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_snapshot_is_baseline_and_consumes_complete_config_stably()"),
        "component-local semantics suite should keep mirrored snapshot-baseline regression.",
    );
}

#[test]
fn direction_streaming_requiredness_is_optional_with_snapshot_fallback_and_stable_status_markers() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "角色判定（direction）：该组件不是正文阅读面，归类为 `Streaming Optional`；无需在组件内实现增量 token 渲染主链。",
        "fallback 明确：`Component.toml` 显式 `streaming=false`，同时 agent contract 输出 `data-ui-stream-support=\"optional\"` 与 `data-ui-stream-fallback=\"snapshot\"`，固定快照降级路径。",
        "输出状态连续可读：`view.rs` 挂载 `data-ui-output-status`（当前 `verified`）并与 `lang/dir/data-direction/data-direction-source` 连续输出；`role/aria-*` 交互子项对 direction 为 N/A。",
        "职责边界：数据校验、断线恢复、重试由上层编排；direction 仅消费完整输入并稳定渲染语义上下文，不承载传输层策略。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep streaming-requiredness token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionAgentStreamSupport",
        "pub enum DirectionAgentStreamFallback",
        "pub enum DirectionAgentOutputStatus",
        "Self::Optional => \"optional\"",
        "Self::Snapshot => \"snapshot\"",
        "Self::Verified => \"verified\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep stream requiredness fragment `{needle}`.",
        );
    }

    for needle in [
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep stream/status marker fragment `{needle}`.",
        );
    }

    for needle in [
        "streaming = false",
        "snapshot = true",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"submittable\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep stream requiredness manifest marker `{needle}`.",
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "ReadableStream",
        "on_retry",
        "retry",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "direction implementation should avoid transport/retry responsibility token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_streaming_requiredness_is_optional_with_snapshot_fallback_and_stable_status_markers()",
        ),
        "component-local semantics suite should keep mirrored streaming-requiredness regression.",
    );
}

#[test]
fn direction_rust_hygiene_contract_is_enforced_for_non_test_code_and_string_cow_paths() {
    let check_source = load_source("../../components/direction/check2.md");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "非测试源码约束满足：`components/direction/src/logic.rs` 与 `components/direction/src/view.rs` 中不存在 `.unwrap(` / `.expect(` / `let _ =`；状态与属性挂载均为显式处理路径。",
        "字符串复制热点已收敛：`logic::compose_class_name` 返回 `Cow<'static, str>`，默认类名走 `Cow::Borrowed(\"ui-direction-provider\")`，仅在拼接自定义 class 时分配 `Cow::Owned(...)`。",
        "回归锁定：`components/direction/test/logic.rs` 增加 Borrowed/Owned 分支断言，防止后续回退为无条件 `String` 复制。",
        "执行记录：`./scripts/check-rust-hygiene.sh` 已执行；当前环境 `rg` 缺少 PCRE2 导致其前置 `check-api-contracts` 产生基线漂移失败，组件级定向扫描结论不受该环境噪声影响。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep rust-hygiene token `{needle}`.",
        );
    }

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "direction non-test source should not include forbidden hygiene token `{forbidden}`.",
        );
    }

    for needle in [
        "use std::borrow::Cow;",
        "pub fn compose_class_name(class_name: Option<String>) -> Cow<'static, str>",
        "Cow::Owned(format!(\"ui-direction-provider {class_name}\"))",
        "Cow::Borrowed(\"ui-direction-provider\")",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep rust-hygiene Cow contract fragment `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_rust_hygiene_contract_is_enforced_for_non_test_code_and_string_cow_paths()"),
        "component-local semantics suite should keep mirrored rust-hygiene regression.",
    );
}

#[test]
fn direction_tree_shaking_feature_pruning_is_gated_in_ui_components() {
    let check_source = load_source("../../components/direction/check2.md");
    let ui_components_cargo = load_source("src/../Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "组件特性注册已落位：`crates/ui-components/Cargo.toml` 定义 `component-direction = [\"dep:ui-direction\"]`，并将 `ui-direction` 作为 `optional` 依赖接入。",
        "模块导出受门控：`crates/ui-components/src/lib.rs` 仅在 `#[cfg(feature = \"component-direction\")]` 下导出 `pub use ui_direction as direction;`，未启用时不可达。",
        "CSS 聚合受门控：`crates/ui-components/src/css.rs` 仅在 `#[cfg(feature = \"component-direction\")]` 下注入 `crate::direction::styles::CSS`，不存在 direction CSS 的无条件全局聚合。",
        "特性树验证：执行 `cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-direction,inject-css`，输出仅含命令行特性 `component-direction` 与 `inject-css`，未出现 `all-components`。",
        "反向依赖验证：执行 `cargo tree -e features -i ui-components -p web-demo`，链路为 `web-demo-components`，未发现 `all-components` 被隐式拉起。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep tree-shaking token `{needle}`.",
        );
    }

    for needle in [
        "component-direction = [\"dep:ui-direction\"]",
        "ui-direction = { path = \"../../components/direction\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo should keep direction feature registration `{needle}`.",
        );
    }

    assert!(
        ui_components_lib.contains(
            "#[cfg(feature = \"component-direction\")]\npub use ui_direction as direction;"
        ),
        "ui-components lib should gate direction module export behind component-direction feature.",
    );

    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-direction\")]\n    out.push_str(crate::direction::styles::CSS);"
        ),
        "ui-components css aggregation should gate direction css behind component-direction feature.",
    );

    for needle in [
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should keep guard token `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_tree_shaking_feature_pruning_is_gated_in_ui_components()"),
        "component-local semantics suite should keep mirrored tree-shaking regression.",
    );
}

#[test]
fn direction_semantic_and_performance_regression_contract_is_covered_without_snapshot_bias() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let performance_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "语义覆盖证据：`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 已断言 `lang/dir/data-slot/data-direction/data-direction-source` 与来源封闭集合，主路径基于语义标记而非视觉快照。",
        "快照约束：已锁定“语义优先、快照仅补充”策略（`direction_tests_prioritize_semantic_contracts_over_snapshots`），避免以 snapshot 断言替代语义契约。",
        "`aria-*`/焦点流转适用性：N/A（direction）；该组件为方向语义 provider，不暴露可聚焦交互控件，无组件私有焦点流转状态机；交互焦点契约由上层可交互组件负责。",
        "性能与 `render_count` 适用性：N/A（direction 非高频/重型）；当前仅做一次方向归一与语义挂载，不属于必须单组件 `render_count` 预算对象。",
        "仓库级回归与预算阻断：`scripts/check-ui-components-performance.sh` 已纳入 `button/input` 的预算与 `render_count` 跟踪（初始化预算为 `1`）以及后续自动化补齐计划，direction 复用该基线治理。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep semantic+performance token `{needle}`.",
        );
    }

    for needle in [
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep semantic marker fragment `{needle}`.",
        );
    }

    for needle in [
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "perf_render_count_follow_up_is_tracked_in_plan",
        "docs_perf_probe_budgets_are_wired_for_component_pages",
    ] {
        assert!(
            performance_script_source.contains(needle),
            "performance script should keep budget guard token `{needle}`.",
        );
    }

    for mirrored in [
        "fn direction_tests_prioritize_semantic_contracts_over_snapshots()",
        "fn direction_focus_stack_contract_is_not_applicable_and_private_node_refs_are_forbidden()",
        "fn direction_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates()",
    ] {
        assert!(
            component_semantics_source.contains(mirrored),
            "component-local semantics suite should keep mirrored contract `{mirrored}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_semantic_and_performance_regression_contract_is_covered_without_snapshot_bias()",
        ),
        "component-local semantics suite should keep mirrored semantic+performance regression.",
    );
}

#[test]
fn direction_version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade() {
    let check_source = load_source("../../components/direction/check2.md");
    let protocol_source = load_source("../../components/direction/src/protocol.rs");
    let manifest_source = load_source("../../components/direction/src/Component.toml");
    let rbi_source = load_source("../../components/direction/src/direction.rbi");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A（本次改动范围）：当前提交未引入跨大版本 API 破坏升级；`DirectionProvider` 公共签名保持兼容，协议仍为 `schema_version = \"1\"` / `DirectionComponentSchemaVersion::V1`。",
        "兼容现状证据：`components/direction/src/protocol.rs` 仅定义 `V1`；`components/direction/src/Component.toml` 保持 `schema_version = \"1\"`；`components/direction/src/direction.rbi` 未引入 v2 并保持现有接口投影。",
        "迁移层状态：当前不存在 `migrate_v1_to_v2`，因为未发生破坏性升级；避免“为假问题预置迁移函数”导致无收益复杂度。",
        "升级触发门槛：若未来出现 `V2` 或 API 破坏变更，必须同步完成 Schema Registry 弃用窗口登记、纯函数迁移层（`migrate_v1_to_v2`）与回归测试，再允许勾选通过。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep version-migration token `{needle}`.",
        );
    }

    for needle in [
        "pub enum DirectionComponentSchemaVersion",
        "V1,",
        "pub struct DirectionComponentSpec",
        "pub schema_version: DirectionComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "direction protocol should keep stable v1 schema fragment `{needle}`.",
        );
    }

    for needle in [
        "schema_version = \"1\"",
        "snapshot = true",
        "streaming = false",
    ] {
        assert!(
            manifest_source.contains(needle),
            "direction Component.toml should keep stable schema/version marker `{needle}`.",
        );
    }

    for needle in [
        "pub type DirectionMode = ui_state_primitives::direction::DirectionMode;",
        "pub fn DirectionProvider(",
        "direction: Option<DirectionMode>",
        "dir: Option<DirectionMode>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "direction RBI should keep backward-compatible signature fragment `{needle}`.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "deprecated",
        "breaking_change",
        "codemod",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "direction implementation should avoid premature migration token `{forbidden}` when no breaking upgrade exists.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade()",
        ),
        "component-local semantics suite should keep mirrored version-migration regression.",
    );
}

#[test]
fn direction_docs_copy_paste_ready_contract_is_documented_and_enforced() {
    let check_source = load_source("../../components/direction/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_components_test_source =
        load_source("../../apps/docs-app/src/pages/components/test/mod.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "docs-app 落点：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 已提供 `Hello World`、`State Matrix (LTR / RTL / Default)`、`Controlled vs Uncontrolled (N/A for Direction)`、`Snapshot Contract` 四组 Playground。",
        "受控/非受控对照适配：direction 无内部可变状态轴，文档以 `direction` 与 `dir` 输入来源对照展示并显式标注 N/A，避免伪造不存在的受控状态机。",
        "流式/快照展示：页面新增 `data-slot=\"direction-streaming-policy\"` 与 `data-slot=\"direction-streaming-modes\"`，明确 `Streaming Optional; fallback=snapshot` 与 snapshot 渲染路径。",
        "Source-first 复制能力：每个 Playground 传入 `code_imports=DIRECTION_COPY_IMPORTS.to_string()`；`apps/docs-app/src/playground.rs` 通过 `compose_copy_ready_code` 自动补全缺失 imports，并在 Code 面板提供复制入口。",
        "Copy-Paste Ready 约束：`DIRECTION_COPY_IMPORTS` 固定包含 `use leptos::prelude::*;` 与 `use ui_components::{DirectionMode, DirectionProvider};`，保证复制代码可直接运行（在仓库 docs 约定上下文内）。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep docs copy-ready token `{needle}`.",
        );
    }

    for needle in [
        "const DIRECTION_COPY_IMPORTS: &str =",
        "use leptos::prelude::*;",
        "use ui_components::{DirectionMode, DirectionProvider};",
        "title=\"Hello World\"",
        "title=\"State Matrix (LTR / RTL / Default)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Direction)\"",
        "title=\"Snapshot Contract\"",
        "data-slot=\"direction-streaming-policy\"",
        "data-slot=\"direction-streaming-modes\"",
        "Streaming Optional; fallback=snapshot.",
        "code_imports=DIRECTION_COPY_IMPORTS.to_string()",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep copy-ready docs fragment `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports: String",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "compose_copy_ready_code(&snippet, &code_imports.get_value())",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep import-completion copy path fragment `{needle}`.",
        );
    }

    for needle in [
        "fn every_component_doc_page_renders_at_least_one_playground()",
        "provide_playground_registry()",
    ] {
        assert!(
            docs_components_test_source.contains(needle),
            "docs component tests should keep playground availability baseline `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_docs_copy_paste_ready_contract_is_documented_and_enforced()"),
        "component-local semantics suite should keep mirrored docs copy-ready regression.",
    );
}

#[test]
fn direction_semantic_contract_priority_is_enforced_with_mirrored_semantics_tests() {
    let check_source = load_source("../../components/direction/check2.md");
    let view_source = load_source("../../components/direction/src/view.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");
    let cross_crate_semantics_source = load_source("tests/direction_semantics.rs");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "语义测试落点：`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 均存在并持续镜像，保证组件内与聚合层对同一语义契约双向回归。",
        "契约覆盖证据：测试已锁定 `lang/dir/data-direction/data-direction-source/data-slot` 与来源封闭集合（`direction|dir-alias|default`）；`role/aria-*` 对 direction 为非交互 N/A，已通过文档与语义测试明确边界。",
        "快照约束证据：`direction_tests_prioritize_semantic_contracts_over_snapshots` 显式禁止 `assert_snapshot/assert_debug_snapshot/assert_json_snapshot/to_match_snapshot`，避免视觉快照替代语义断言。",
        "变更门禁：`direction_semantic_contract_priority_is_enforced_with_mirrored_semantics_tests` 要求该条保持勾选并校验语义字段断言与镜像函数存在；新增语义字段若未补测试将直接回归失败。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep semantic-priority token `{needle}`.",
        );
    }

    for needle in [
        "lang=contract.attrs.lang",
        "dir=contract.attrs.dir",
        "data-slot=\"direction-provider\"",
        "data-direction=contract.attrs.data_direction",
        "data-direction-source=direction_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "direction view should keep semantic-contract marker `{needle}`.",
        );
    }

    for forbidden in ["role=", "aria-"] {
        assert!(
            !view_source.contains(forbidden),
            "direction view should remain non-interactive and avoid token `{forbidden}`.",
        );
    }

    for needle in [
        "fn direction_tests_prioritize_semantic_contracts_over_snapshots()",
        "fn direction_state_markers_are_observable_queryable_and_enumerable()",
        "fn direction_semantic_contract_priority_is_enforced_with_mirrored_semantics_tests()",
    ] {
        assert!(
            component_semantics_source.contains(needle)
                && cross_crate_semantics_source.contains(needle),
            "component-local and cross-crate semantics suites should both keep `{needle}`.",
        );
    }

    for forbidden in [
        "assert_snapshot",
        "assert_debug_snapshot",
        "assert_json_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden)
                && !cross_crate_semantics_source.contains(forbidden),
            "semantic suites should forbid snapshot assertion token `{forbidden}`.",
        );
    }
}

#[test]
fn direction_e2e_selector_contract_uses_semantic_markers_and_stable_readiness() {
    let check_source = load_source("../../components/direction/check2.md");
    let direction_e2e_source = load_source("../../e2e/tests/docs_app_direction_contract.spec.mjs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 语义选择器证据：`e2e/tests/docs_app_direction_contract.spec.mjs` 仅使用 `data-component=\"direction-provider\"`、`data-slot=\"direction-state-matrix\"`、`data-slot=\"direction-provider\"`、`data-direction`、`data-direction-source`、`data-slot=\"direction-snapshot-demo\"` 组合定位，未使用文本定位或 DOM 层级猜测。",
        "WASM 稳定等待证据：用例在路由切换后统一先等待 `body:not(:has(#boot))`，并以 `await expect(...data-slot...)` 断言语义节点可见作为就绪条件，未使用固定 sleep。",
        "异步/动画适用性：N/A（direction）；该组件无异步状态机与动效流程，当前 E2E 重点锁定语义标记稳定性与 route repeatability，不需要额外 ready/settled 动画等待分支。",
        "高风险路径适用性：N/A（direction）；该组件不承载 overlay/focus trap/keyboard 导航/async 交互流，高风险回归优先级由对应交互组件承担；若 future 引入上述能力，必须补同级 Playwright 回归并显式断言语义断点。",
        "回归门禁：`direction_e2e_selector_contract_uses_semantic_markers_and_stable_readiness` 在组件侧与 `crates/ui-components` 镜像测试中强制校验 `data-*` 选择器、语义就绪等待与“无 fixed sleep”约束。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep e2e-selector token `{needle}`.",
        );
    }

    for needle in [
        "await page.goto(\"/#/components/direction-provider\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"direction-provider\"]",
        "[data-slot=\"direction-state-matrix\"]",
        "[data-slot=\"direction-provider\"][data-direction=\"ltr\"][data-direction-source=\"direction\"]",
        "[data-slot=\"direction-provider\"][data-direction=\"rtl\"][data-direction-source=\"direction\"]",
        "[data-slot=\"direction-provider\"][data-direction=\"ltr\"][data-direction-source=\"default\"]",
        "[data-slot=\"direction-snapshot-demo\"]",
    ] {
        assert!(
            direction_e2e_source.contains(needle),
            "direction E2E should keep semantic selector/readiness fragment `{needle}`.",
        );
    }

    for forbidden in [
        "getByText(",
        "waitForTimeout(",
        "setTimeout(",
        ".nth(",
        "sleep(",
    ] {
        assert!(
            !direction_e2e_source.contains(forbidden),
            "direction E2E should avoid unstable selector/wait token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_e2e_selector_contract_uses_semantic_markers_and_stable_readiness()",
        ),
        "component-local semantics suite should keep mirrored e2e-selector regression.",
    );
}

#[test]
fn direction_key_flow_is_repeatable_and_reports_semantic_breakpoints() {
    let check_source = load_source("../../components/direction/check2.md");
    let direction_e2e_source = load_source("../../e2e/tests/docs_app_direction_contract.spec.mjs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "关键流程用例：`e2e/tests/docs_app_direction_contract.spec.mjs` 的 `docs-app direction provider route flow is repeatable` 覆盖“进入 direction 页面 -> 跳转 spacer -> 返回 direction 页面”可重复流程，纳入 Playwright 回归集合。",
        "断点可定位：流程以 `data-slot=\"direction-snapshot-demo\"` 作为语义就绪断点，并断言 `[data-slot=\"direction-provider\"][lang=\"en\"][data-direction=\"ltr\"][data-direction-source=\"direction\"]` 的 `dir/lang` 属性；失败可直接定位语义契约断裂点。",
        "高风险路径适用性：N/A（direction）；该组件不承载 overlay/focus trap/keyboard/async 流程，高风险交互回归由对应交互组件承担。",
        "扩展约束：若 future 在 direction 引入 overlay/focus/keyboard/async 能力，必须新增同级 Playwright 关键流程回归（含 ready/settled 条件与语义断点断言）后方可维持勾选。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep key-flow token `{needle}`.",
        );
    }

    for needle in [
        "test(\"docs-app direction provider route flow is repeatable\"",
        "await page.goto(\"/#/components/direction-provider\");",
        "await page.goto(\"/#/components/spacer\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"direction-snapshot-demo\"]",
        "[data-slot=\"direction-provider\"][lang=\"en\"][data-direction=\"ltr\"][data-direction-source=\"direction\"]",
        "await expect(snapshotProvider).toHaveAttribute(\"dir\", \"ltr\");",
        "await expect(snapshotProvider).toHaveAttribute(\"lang\", \"en\");",
    ] {
        assert!(
            direction_e2e_source.contains(needle),
            "direction E2E key-flow suite should keep fragment `{needle}`.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "screenshot(", "page.content()"] {
        assert!(
            !direction_e2e_source.contains(forbidden),
            "direction key-flow should avoid opaque visual-only breakpoint token `{forbidden}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_key_flow_is_repeatable_and_reports_semantic_breakpoints()"),
        "component-local semantics suite should keep mirrored key-flow regression.",
    );
}

#[test]
fn direction_docs_examples_and_matrices_are_synced_with_logic_contract() {
    let check_source = load_source("../../components/direction/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let logic_source = load_source("../../components/direction/src/logic.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "docs 同步落点：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 已覆盖 `Hello World`、`RTL Direction + Class`、`State Matrix (LTR / RTL / Default)`、`Controlled vs Uncontrolled (N/A for Direction)`、`Snapshot Contract` 示例。",
        "参数矩阵已落地：页面新增 `data-slot=\"direction-parameter-matrix\"` 与 `data-slot=\"direction-parameter-rows\"`，文档化 `direction/dir/lang/class_name` 参数语义与默认值（`None`）。",
        "状态矩阵已落地：页面通过 `data-slot=\"direction-state-matrix\"` 明确展示 `LTR/RTL/Default` 三态，覆盖方向来源路径。",
        "API/默认值一致性：文档新增 `data-slot=\"direction-default-rule\"`，明确 `logic::resolve_direction(direction, dir)` 采用 `direction > dir > DirectionMode::default()`，并对应 `data-direction-source=direction|dir-alias|default`。",
        "适用性说明：`disabled/size/variant` 对 direction 为 N/A（该组件是语义 provider，无对应状态轴）；文档以受控别名对照与方向状态矩阵覆盖本组件实际可变维度。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep docs-matrix token `{needle}`.",
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix (LTR / RTL / Default)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Direction)\"",
        "data-slot=\"direction-state-matrix\"",
        "data-slot=\"direction-parameter-matrix\"",
        "data-slot=\"direction-parameter-rows\"",
        "direction: Option&lt;DirectionMode&gt;",
        "dir: Option&lt;DirectionMode&gt;",
        "lang: Option&lt;String&gt;",
        "class_name: Option&lt;String&gt;",
        "data-slot=\"direction-default-rule\"",
        "logic::resolve_direction(direction, dir)",
        "direction > dir > DirectionMode::default()",
        "direction|dir-alias|default",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep docs/matrix fragment `{needle}`.",
        );
    }

    for forbidden in ["default_direction", "on_direction_change"] {
        assert!(
            !docs_page_source.contains(forbidden),
            "direction docs should avoid stale API token `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn resolve_direction(",
        "if let Some(direction) = direction {",
        "} else if let Some(direction) = dir {",
        "(DirectionMode::default(), DirectionPropSource::Default)",
    ] {
        assert!(
            logic_source.contains(needle),
            "direction logic should keep default-resolution rule fragment `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_docs_examples_and_matrices_are_synced_with_logic_contract()"),
        "component-local semantics suite should keep mirrored docs-matrix regression.",
    );
}

#[test]
fn direction_documentation_entry_is_beginner_friendly_and_ordered_for_progression() {
    let check_source = load_source("../../components/direction/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let docs_components_pages_source =
        load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_components_test_source =
        load_source("../../apps/docs-app/src/pages/components/test/mod.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "等价文档入口已存在：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 提供 `DirectionProvider` 文档页；`apps/docs-app/src/pages/components/pages.rs` 将 `layout_extra_direction::DIRECTION_PROVIDER_DOC` 纳入组件目录索引，可稳定访问 `/#/components/direction-provider`。",
        "零门槛路径已落地：页面首个 Playground 为 `Hello World`（3 行可运行示例），并新增 `data-slot=\"direction-docs-beginner-path\"` 明确“先复制最小示例再扩展”。",
        "先上手再进阶：文档新增 `data-slot=\"direction-docs-advanced-path\"`，引导随后查看 `RTL Direction + Class`、`State Matrix (LTR / RTL / Default)`、`Controlled vs Uncontrolled (N/A for Direction)`、`Parameter Matrix` 与 `Default Resolution Rule`。",
        "文档非机器独享：页面包含可复制示例、状态矩阵与参数矩阵，避免仅暴露源码或抽象术语；并通过 docs 组件测试注册表保证入口持续存在。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep documentation-product token `{needle}`.",
        );
    }

    for needle in [
        "pub(super) const DIRECTION_PROVIDER_DOC: ComponentDoc = ComponentDoc {",
        "name: \"DirectionProvider\"",
        "slug: \"direction-provider\"",
        "data-slot=\"direction-docs-beginner-path\"",
        "data-slot=\"direction-docs-advanced-path\"",
        "title=\"Hello World\"",
        "title=\"RTL Direction + Class\"",
        "title=\"State Matrix (LTR / RTL / Default)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Direction)\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep beginner-docs fragment `{needle}`.",
        );
    }

    let beginner_idx = docs_page_source
        .find("data-slot=\"direction-docs-beginner-path\"")
        .expect("direction docs should define beginner path marker");
    let hello_idx = docs_page_source
        .find("title=\"Hello World\"")
        .expect("direction docs should keep Hello World example");
    let advanced_idx = docs_page_source
        .find("data-slot=\"direction-docs-advanced-path\"")
        .expect("direction docs should define advanced path marker");
    let rtl_idx = docs_page_source
        .find("title=\"RTL Direction + Class\"")
        .expect("direction docs should keep advanced RTL example");

    assert!(
        beginner_idx < hello_idx && hello_idx < advanced_idx && advanced_idx < rtl_idx,
        "direction docs should keep beginner-first then advanced progression order."
    );

    for needle in [
        "mod layout_extra_direction;",
        "layout_extra_direction::DIRECTION_PROVIDER_DOC,",
    ] {
        assert!(
            docs_components_pages_source.contains(needle),
            "docs component pages registry should keep direction entry fragment `{needle}`.",
        );
    }

    for needle in [
        "\"direction\" => &[\"direction-provider\"]",
        "fn every_component_doc_page_renders_at_least_one_playground()",
    ] {
        assert!(
            docs_components_test_source.contains(needle),
            "docs component tests should keep discoverability fragment `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_documentation_entry_is_beginner_friendly_and_ordered_for_progression()"
        ),
        "component-local semantics suite should keep mirrored documentation-product regression.",
    );
}

#[test]
fn direction_interactive_playground_supports_realtime_props_and_repeatable_flow() {
    let check_source = load_source("../../components/direction/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let direction_e2e_source = load_source("../../e2e/tests/docs_app_direction_contract.spec.mjs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "交互入口已落地：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 新增 `Interactive Playground (Props + State Switch + Feedback)`，并提供稳定选择器 `data-slot=\"direction-interactive-playground\"`。",
        "基础 props 调整已覆盖：控制面板提供 `direction/dir/default` 来源切换、`ltr/rtl` 值切换、`lang(none/en/ar)` 切换、`class_name` 开关与 `reset`，均通过 `data-slot=\"direction-interactive-*\"` 暴露。",
        "状态切换与反馈可观测：预览区实时更新 `DirectionProvider` 语义标记（`data-direction`/`data-direction-source`/`lang`/`dir`），并在 `data-slot=\"direction-interactive-feedback\"` 输出当前来源与方向反馈。",
        "AI Spec 适用性：N/A（direction 为语义上下文 provider，非 AI Spec 组件；无独立 `spec.rs` 输入模型与渲染器联动面）。",
        "可重复关键流程已固化：`e2e/tests/docs_app_direction_contract.spec.mjs` 新增 `docs-app direction provider interactive playground updates semantic markers deterministically`，按固定序列切换控制项并断言语义标记收敛。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep interactive-playground token `{needle}`.",
        );
    }

    for needle in [
        "title=\"Interactive Playground (Props + State Switch + Feedback)\"",
        "data-slot=\"direction-interactive-playground\"",
        "data-slot=\"direction-interactive-controls\"",
        "data-slot=\"direction-interactive-source-direction\"",
        "data-slot=\"direction-interactive-source-dir-alias\"",
        "data-slot=\"direction-interactive-source-default\"",
        "data-slot=\"direction-interactive-value-ltr\"",
        "data-slot=\"direction-interactive-value-rtl\"",
        "data-slot=\"direction-interactive-lang-none\"",
        "data-slot=\"direction-interactive-lang-en\"",
        "data-slot=\"direction-interactive-lang-ar\"",
        "data-slot=\"direction-interactive-class-toggle\"",
        "data-slot=\"direction-interactive-reset\"",
        "data-slot=\"direction-interactive-preview\"",
        "data-slot=\"direction-interactive-feedback\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep interactive-playground fragment `{needle}`.",
        );
    }

    for needle in [
        "test(\"docs-app direction provider interactive playground updates semantic markers deterministically\"",
        "[data-slot=\"direction-interactive-playground\"]",
        "[data-slot=\"direction-interactive-preview\"]",
        "[data-slot=\"direction-interactive-feedback\"]",
        "[data-slot=\"direction-interactive-value-rtl\"]",
        "[data-slot=\"direction-interactive-source-dir-alias\"]",
        "[data-slot=\"direction-interactive-lang-ar\"]",
        "[data-slot=\"direction-interactive-class-toggle\"]",
        "[data-slot=\"direction-interactive-source-default\"]",
        "[data-slot=\"direction-interactive-reset\"]",
        "toHaveAttribute(\"data-direction-source\", \"dir-alias\")",
        "toHaveAttribute(\"data-direction-source\", \"default\")",
        "toHaveAttribute(\"data-current-source\", \"default\")",
    ] {
        assert!(
            direction_e2e_source.contains(needle),
            "direction interactive E2E contract should keep `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_interactive_playground_supports_realtime_props_and_repeatable_flow()",
        ),
        "component-local semantics suite should keep mirrored interactive-playground regression.",
    );
}

#[test]
fn direction_source_first_docs_are_copy_paste_ready_and_traceable() {
    let check_source = load_source("../../components/direction/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_direction.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "复制能力已落地：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 新增 `data-slot=\"direction-source-first\"` 区块，包含 `Snippet(copyable=true)` 的一键复制入口（`label=\"Copy DirectionProvider starter\"`）。",
        "复制代码可直接运行：`DIRECTION_SOURCE_FIRST_SNIPPET` 固定包含 `use leptos::prelude::*;` 与 `use ui_components::{DirectionMode, DirectionProvider};`，并提供最小可用 `DirectionProvider` 示例。",
        "源码与依赖可追溯：文档新增 `data-slot=\"direction-source-paths\"`（`mod.rs/logic.rs/view.rs/styles.rs/protocol.rs`）和 `data-slot=\"direction-source-prerequisites\"`（`component-direction`、`inject-css`），避免“复制即报错”盲区。",
        "同步策略已显式化：`data-slot=\"direction-source-sync-note\"` 约束 starter snippet 与 Hello World 同步更新；`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 镜像回归锁定该契约，防止示例漂移。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep source-first docs token `{needle}`.",
        );
    }

    for needle in [
        "const DIRECTION_SOURCE_FIRST_SNIPPET: &str =",
        "data-slot=\"direction-source-first\"",
        "data-slot=\"direction-source-first-copy-hint\"",
        "text=DIRECTION_SOURCE_FIRST_SNIPPET.to_string()",
        "label=\"Copy DirectionProvider starter\".to_string()",
        "copyable=true",
        "data-slot=\"direction-source-paths\"",
        "components/direction/src/mod.rs",
        "components/direction/src/logic.rs",
        "components/direction/src/view.rs",
        "components/direction/src/styles.rs",
        "components/direction/src/protocol.rs",
        "data-slot=\"direction-source-prerequisites\"",
        "component-direction",
        "inject-css",
        "data-slot=\"direction-source-sync-note\"",
        "direction=DirectionMode::Ltr",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "direction docs page should keep source-first fragment `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: String",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "compose_copy_ready_code(&snippet, &code_imports.get_value())",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy pipeline should keep `{needle}`.",
        );
    }

    assert!(
        component_semantics_source
            .contains("fn direction_source_first_docs_are_copy_paste_ready_and_traceable()"),
        "component-local semantics suite should keep mirrored source-first regression.",
    );
}

#[test]
fn direction_heroui_strategy_doc_and_docs_entry_are_synced_for_parameter_changes() {
    let check_source = load_source("../../components/direction/check2.md");
    let heroui_strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let heroui_research_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");
    let docs_components_pages_source =
        load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_components_test_source =
        load_source("../../apps/docs-app/src/pages/components/test/mod.rs");
    let component_semantics_source = load_source("../../components/direction/test/semantics.rs");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "对标文档已同步：`docs/spec/heroui-parameter-design-strategy.md` 新增 `Direction 同步记录（2026-02-20）`，明确 `direction/dir` 语义、归一优先级（`direction > dir > default`）与 `data-direction-source` 封闭集合契约。",
        "组件文档入口可访问：`apps/docs-app/src/pages/components/pages.rs` 保持 `layout_extra_direction::DIRECTION_PROVIDER_DOC` 索引，`apps/docs-app/src/pages/components/test/mod.rs` 保持 `\"direction\" => &[\"direction-provider\"]` 可检索映射。",
        "研究文档补充判定：本轮仅为 `DirectionProvider` 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "回归约束：`components/direction/test/semantics.rs` 与 `components/direction/test/direction_semantics.rs` 镜像断言该条勾选状态、HeroUI 对标文档段落和 docs 索引入口，防止“仅代码更新无文档更新”回归。",
    ] {
        assert!(
            check_source.contains(needle),
            "Direction checklist should keep HeroUI-doc-sync token `{needle}`.",
        );
    }

    for needle in [
        "### Direction 同步记录（2026-02-20）",
        "`DirectionProvider` 继续保持语义 provider 定位，参数主轴为 `direction/dir/lang/class_name`",
        "归一优先级保持 `direction > dir > DirectionMode::default()`",
        "`data-direction-source=direction|dir-alias|default`",
        "`apps/docs-app/src/pages/components/pages.rs` 通过 `layout_extra_direction::DIRECTION_PROVIDER_DOC` 暴露 `slug=\"direction-provider\"`",
        "\"direction\" => &[\"direction-provider\"]",
        "`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 已覆盖 `Hello World`、`State Matrix`、`Interactive Playground`、`Source-first / Copy-Paste Ready`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI strategy doc should keep direction sync fragment `{needle}`.",
        );
    }

    for needle in [
        "mod layout_extra_direction;",
        "layout_extra_direction::DIRECTION_PROVIDER_DOC,",
    ] {
        assert!(
            docs_components_pages_source.contains(needle),
            "docs component pages registry should keep direction index fragment `{needle}`.",
        );
    }

    for needle in [
        "\"direction\" => &[\"direction-provider\"]",
        "fn every_component_doc_page_renders_at_least_one_playground()",
    ] {
        assert!(
            docs_components_test_source.contains(needle),
            "docs component tests should keep direction discoverability fragment `{needle}`.",
        );
    }

    for needle in [
        "HeroUI — Button:",
        "HeroUI — Custom Styles / classNames:",
        "HeroUI — Select:",
    ] {
        assert!(
            heroui_research_source.contains(needle),
            "HeroUI research baseline should keep reference marker `{needle}`.",
        );
    }

    assert!(
        component_semantics_source.contains(
            "fn direction_heroui_strategy_doc_and_docs_entry_are_synced_for_parameter_changes()",
        ),
        "component-local semantics suite should keep mirrored HeroUI-doc-sync regression.",
    );
}
