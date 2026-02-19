use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn scroll_shadow_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/scroll_shadow/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ScrollShadow internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_api_naming_contract_is_consistent_without_alias_drift() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "无布尔状态轴、无事件回调轴、无默认值轴，因此不存在前缀漂移与同义别名并存问题。",
        "scroll_shadow_api_naming_contract_is_consistent_without_alias_drift",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep api naming contract marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow public API should keep expected prop declaration `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] is_",
        "#[prop(optional)] on_",
        "#[prop(optional)] default_",
        "#[prop(optional, into)] is_",
        "#[prop(optional, into)] on_",
        "#[prop(optional, into)] default_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow should not introduce alias-drift token `{forbidden}` without a matching controllable axis."
        );
    }
}

#[test]
fn scroll_shadow_controlled_uncontrolled_contract_is_explicitly_not_applicable() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "N/A：`ScrollShadow` 无对外可控状态轴，阴影状态由滚动位置与视口尺寸即时派生",
        "scroll_shadow_controlled_uncontrolled_contract_is_explicitly_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep controlled/uncontrolled N/A marker `{needle}`."
        );
    }

    for forbidden in [
        "on_value_change",
        "default_value",
        "on_open_change",
        "default_open",
        "use_controllable",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ScrollShadow should not expose partial controllable-axis token `{forbidden}`."
        );
    }

    for required in [
        "pub fn compute_scroll_shadow_edges(",
        "pub fn resolve_edge_state(",
        "pub fn edge_state_attr(",
    ] {
        assert!(
            primitive_source.contains(required),
            "ScrollShadow state primitive should keep derived-state path `{required}` for non-controllable axis."
        );
    }

    for required in [
        "compute_scroll_shadow_edges",
        "resolve_edge_state",
        "resolve_semantic_state",
    ] {
        assert!(
            logic_source.contains(required),
            "ScrollShadow logic should consume/re-export primitive symbol `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_async_contract_is_explicitly_not_applicable() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");

    for needle in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "N/A：`ScrollShadow` 无远程请求、无异步状态轴、无异步失败恢复路径需求",
        "scroll_shadow_async_contract_is_explicitly_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep async N/A marker `{needle}`."
        );
    }

    for forbidden in [
        "is_loading",
        "aria-busy",
        "use_async_action",
        "on_retry",
        "retry",
        "create_resource",
        "spawn_local",
        "async fn",
        "Future",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ScrollShadow should not declare async protocol token `{forbidden}`."
        );
    }

    for required in [
        "on:scroll=on_scroll",
        "compute_scroll_shadow_edges",
        "resolve_edge_state",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "ScrollShadow should keep sync derived-state path token `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_api_dx_paradox_contract_is_satisfied() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));

    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "基础 API 仅需 `<ScrollShadow>{children}</ScrollShadow>`",
        "scroll_shadow_api_dx_paradox_contract_is_satisfied",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep DX marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::",
        "#[prop()] state",
        "#[prop(optional)] state",
        "#[prop(optional, into)] state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow basic API should not force internal wiring token `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
        "children: Children",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow component API should keep simple signature token `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ScrollShadow>",
        "<div class=\"docs-scroll-shadow-item\">\"Activity\"</div>",
        r#"<ScrollShadow>
  <div class="docs-scroll-shadow-item">Activity</div>
</ScrollShadow>"#,
    ] {
        assert!(
            docs_scroll_shadow_section.contains(needle),
            "ScrollShadow docs should keep minimal default path token `{needle}`."
        );
    }

    assert!(
        !docs_scroll_shadow_section.contains("ui-state-primitives")
            && !docs_scroll_shadow_section.contains("ui-headless")
            && !docs_scroll_shadow_section.contains("state="),
        "ScrollShadow docs Hello World should not require users to wire internal state primitives."
    );
}

#[test]
fn scroll_shadow_composite_api_contract_is_explicitly_not_applicable() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));

    for needle in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A：`ScrollShadow` 为单容器滚动阴影组件，不提供 `Item` 子结构与多槽位配对语义",
        "scroll_shadow_composite_api_contract_is_explicitly_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep composite API N/A marker `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] labels",
        "#[prop(optional)] titles",
        "#[prop(optional)] panels",
        "#[prop(optional)] items",
        "ItemSpec",
        "<Item",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view_source.contains(forbidden) && !docs_scroll_shadow_section.contains(forbidden),
            "ScrollShadow should not expose composite parallel-slot API token `{forbidden}`."
        );
    }

    for required in [
        "children: Children",
        "<ScrollShadow>",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
    ] {
        assert!(
            view_source.contains(required) || docs_scroll_shadow_section.contains(required),
            "ScrollShadow should keep non-composite default path token `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_default_value_source_is_logic_only() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "DEFAULT_MAX_HEIGHT_PX",
        "scroll_shadow_default_value_source_is_logic_only",
    ] {
        assert!(
            check2_source.contains(needle) || logic_source.contains(needle),
            "default source contract marker `{needle}` should be present in check2/logic."
        );
    }

    for needle in [
        "pub const DEFAULT_MAX_HEIGHT_PX: u32 = 192;",
        "let custom_max_height_px = normalize_max_height(input.max_height_px);",
        "let max_height_px = custom_max_height_px.unwrap_or(DEFAULT_MAX_HEIGHT_PX);",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ScrollShadow primitive should keep single default-source token `{needle}`."
        );
    }

    for forbidden in [
        "let Some(px) = max_height_px.get_value() else",
        "max_height_px.get_value()",
        "let set_max_height =",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should not re-implement default fallback via `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=logic::compose_inline_style(state.max_height_px)"),
        "ScrollShadow view should consume normalized `state.max_height_px` only."
    );

    for forbidden in [
        "var(--ui-scroll-shadow-max-h,",
        "calc(var(--ui-space-lg) * 12)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should not define fallback default source `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_state_normalization_is_centralized_in_logic_layer() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "ScrollShadowSemanticInput",
        "resolve_semantic_state",
        "ScrollShadowSemanticState",
        "scroll_shadow_state_normalization_is_centralized_in_logic_layer",
    ] {
        assert!(
            check2_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "state normalization contract marker `{needle}` should be present."
        );
    }

    for needle in [
        "pub struct ScrollShadowSemanticInput",
        "pub struct ScrollShadowSemanticState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ScrollShadow primitive should keep centralized semantic derivation token `{needle}`."
        );
    }

    for needle in ["ScrollShadowSemanticInput", "resolve_semantic_state"] {
        assert!(
            logic_source.contains(needle),
            "ScrollShadow logic should consume/re-export semantic primitive token `{needle}`."
        );
    }

    for needle in [
        "let semantic_state = Memo::new(move |_| {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow view should consume centralized semantic state via `{needle}`."
        );
    }

    for forbidden in [
        "logic::edge_state_attr(shadow_top.get(), shadow_bottom.get())",
        "logic::is_scrollable(shadow_top.get(), shadow_bottom.get())",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should not rebuild state-machine rule `{forbidden}`."
        );
    }

    for forbidden in [
        "then_some(\"true\")",
        "resolve_semantic_state(",
        "edge_state_attr(",
        "is_scrollable(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should consume markers only and avoid semantic derivation token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_discrete_state_axes_are_enum_typed_and_closed() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "ScrollShadowEdgeState",
        "scroll_shadow_discrete_state_axes_are_enum_typed_and_closed",
    ] {
        assert!(
            check2_source.contains(needle) || logic_source.contains(needle),
            "discrete-state contract marker `{needle}` should be present in check2/logic."
        );
    }

    for needle in [
        "pub enum ScrollShadowEdgeState",
        "None,",
        "Top,",
        "Bottom,",
        "Both,",
        "pub struct ScrollShadowSemanticInput {",
        "pub edge_state: ScrollShadowEdgeState,",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
        "edge_state: edge_state.get(),",
    ] {
        assert!(
            primitive_source.contains(needle) || view_source.contains(needle),
            "ScrollShadow discrete state should keep typed enum token `{needle}`."
        );
    }

    assert!(
        logic_source.contains("ScrollShadowEdgeState"),
        "ScrollShadow logic should consume/re-export enum-typed discrete state."
    );

    for forbidden in [
        "shadow_top: shadow_top.get()",
        "shadow_bottom: shadow_bottom.get()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should not pass bool-combo state token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_uses_logic_state_model() {
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "pub use ui_state_primitives::scroll_shadow::{",
        "ScrollShadowStateInput",
        "ScrollShadowState",
        "ScrollShadowSemanticInput",
        "pub fn normalize_optional_text(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollShadow logic should include `{needle}` for assembly and primitive consumption."
        );
    }

    for needle in [
        "pub struct ScrollShadowStateInput",
        "pub struct ScrollShadowState",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
        "pub fn resolve_edge_state(",
        "pub fn edge_state_attr(",
        "pub fn is_scrollable(",
        "pub struct ScrollShadowSemanticInput",
        "pub struct ScrollShadowSemanticState",
        "pub fn resolve_semantic_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ScrollShadow state primitive should define `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::compose_class_name(class_name, state)",
        "let semantic_state = Memo::new(move |_| {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
        "style=logic::compose_inline_style(state.max_height_px)",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_state_primitives_are_sourced_from_ui_state_primitives() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for needle in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "scroll_shadow_state_primitives_are_sourced_from_ui_state_primitives",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep state-primitive source marker `{needle}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::scroll_shadow::{"),
        "ScrollShadow logic should re-export primitive contract from ui-state-primitives."
    );

    for forbidden in [
        "pub struct ScrollShadowStateInput {",
        "pub struct ScrollShadowState {",
        "pub enum ScrollShadowEdgeState {",
        "pub struct ScrollShadowSemanticInput {",
        "pub struct ScrollShadowSemanticState {",
        "pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ScrollShadow logic should not re-implement state primitive `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ScrollShadowStateInput {",
        "pub struct ScrollShadowState {",
        "pub enum ScrollShadowEdgeState {",
        "pub struct ScrollShadowSemanticInput {",
        "pub struct ScrollShadowSemanticState {",
        "pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
        "#[cfg(test)]",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives scroll_shadow primitive should define `{needle}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod scroll_shadow;"),
        "ui-state-primitives lib should export `scroll_shadow` module."
    );

    for needle in [
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow view should consume logic-mapped primitive token `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/scroll_shadow/view.rs");

    for attr in [
        "data-slot=\"scroll-shadow\"",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "class:ui-scroll-shadow--shadow-top=move || semantic_state.get().shadow_top_attr.is_some()",
        "class:ui-scroll-shadow--shadow-bottom=move || {",
        "semantic_state.get().shadow_bottom_attr.is_some()",
        "class:ui-scroll-shadow--scrollable=move || semantic_state.get().is_scrollable",
    ] {
        assert!(
            source.contains(attr),
            "ScrollShadow should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn scroll_shadow_styles_include_state_marker_contracts() {
    let source = load_source("src/scroll_shadow/styles.rs");

    for selector in [
        ".ui-scroll-shadow--scrollable",
        ".ui-scroll-shadow[data-scrollable=\"true\"]",
        ".ui-scroll-shadow--max-height-custom .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow[data-max-height=\"custom\"] .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow--shadow-top::before",
        ".ui-scroll-shadow[data-shadow-top=\"true\"]::before",
        ".ui-scroll-shadow[data-state=\"both\"]::before",
        ".ui-scroll-shadow--shadow-bottom::after",
        ".ui-scroll-shadow[data-shadow-bottom=\"true\"]::after",
        ".ui-scroll-shadow[data-state=\"both\"]::after",
        "--ui-scroll-shadow-max-h",
        "var(--ui-space-md)",
        "var(--ui-bg)",
    ] {
        assert!(
            source.contains(selector),
            "ScrollShadow styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn scroll_shadow_state_observability_contract_uses_stable_closed_markers() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for needle in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "data-state` 由 `ScrollShadowEdgeState::{none,top,bottom,both}` 封闭集合驱动",
        "scroll_shadow_state_observability_contract_uses_stable_closed_markers",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep observability contract marker `{needle}`."
        );
    }

    for marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollShadow view should expose stable observability marker `{marker}`."
        );
    }

    for edge in [
        "ScrollShadowEdgeState::None => \"none\"",
        "ScrollShadowEdgeState::Top => \"top\"",
        "ScrollShadowEdgeState::Bottom => \"bottom\"",
        "ScrollShadowEdgeState::Both => \"both\"",
    ] {
        assert!(
            primitive_source.contains(edge),
            "ScrollShadow primitive should keep closed edge-state mapping `{edge}`."
        );
    }

    for source_marker in [
        "max_height_attr: if has_custom_max_height {",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(source_marker),
            "ScrollShadow primitive should keep explicit source marker token `{source_marker}`."
        );
    }
}

#[test]
fn scroll_shadow_styles_depend_on_explicit_state_markers_only() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");

    for needle in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "view.rs` 运行时仅透传 `--ui-scroll-shadow-max-h` 这一必要 CSS 变量",
        "scroll_shadow_styles_depend_on_explicit_state_markers_only",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep explicit-style-state marker `{needle}`."
        );
    }

    for selector in [
        ".ui-scroll-shadow--scrollable",
        ".ui-scroll-shadow[data-scrollable=\"true\"]",
        ".ui-scroll-shadow--max-height-custom .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow[data-max-height=\"custom\"] .ui-scroll-shadow__viewport",
        ".ui-scroll-shadow--shadow-top::before",
        ".ui-scroll-shadow[data-shadow-top=\"true\"]::before",
        ".ui-scroll-shadow[data-state=\"both\"]::before",
        ".ui-scroll-shadow--shadow-bottom::after",
        ".ui-scroll-shadow[data-shadow-bottom=\"true\"]::after",
        ".ui-scroll-shadow[data-state=\"both\"]::after",
    ] {
        assert!(
            styles_source.contains(selector),
            "ScrollShadow styles should use explicit state selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should not use structural-guess selector `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=logic::compose_inline_style(state.max_height_px)"),
        "ScrollShadow view should only pass required CSS variable `--ui-scroll-shadow-max-h`."
    );
    assert_eq!(
        view_source.matches("style=").count(),
        1,
        "ScrollShadow view should avoid additional inline style logic."
    );
}

#[test]
fn scroll_shadow_token_first_static_style_contract_is_satisfied() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "scroll_shadow_token_first_static_style_contract_is_satisfied",
        "`component-scroll_shadow` feature",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep token-first contract marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-scroll-shadow-max-h)",
        "var(--ui-space-md)",
        "var(--ui-bg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "scroll_shadow/styles.rs should keep token-first static css token `{needle}`."
        );
    }

    for forbidden in [
        "style=",
        "format!(",
        "style.set_property(",
        "stylers::",
        "stylist::",
        "use stylist",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "scroll_shadow/styles.rs should not carry runtime or css-in-rust token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=logic::compose_inline_style(state.max_height_px)"),
        "scroll_shadow/view.rs should pass only required runtime css variable."
    );
    assert_eq!(
        view_source.matches("style=").count(),
        1,
        "scroll_shadow/view.rs should not embed extra inline business style logic."
    );

    for needle in [
        "#[cfg(feature = \"component-scroll_shadow\")]",
        "out.push_str(crate::scroll_shadow::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout/css.rs should aggregate ScrollShadow css via feature gate `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject component css via centralized path `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_tree_shaking_contract_is_feature_gated_and_css_scoped() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "component-scroll_shadow",
        "cargo tree -e features -p ui-layout --no-default-features --features component-scroll_shadow,inject-css",
        "cargo tree -e features -i ui-layout -p web-demo",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_shadow,inject-css",
        "scroll_shadow_tree_shaking_contract_is_feature_gated_and_css_scoped",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep tree-shaking marker `{needle}`."
        );
    }

    for needle in [
        "component-scroll_shadow = []",
        "\"component-scroll_shadow\",",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-layout/Cargo.toml should keep scroll-shadow feature token `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-scroll_shadow\")]",
        "pub mod scroll_shadow;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout/lib.rs should keep feature-gated scroll-shadow module token `{needle}`."
        );
    }

    for (idx, _) in lib_source.match_indices("pub mod scroll_shadow;") {
        let prefix = &lib_source[..idx];
        let prev_line = prefix
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or_default()
            .trim();
        assert_eq!(
            prev_line, "#[cfg(feature = \"component-scroll_shadow\")]",
            "ui-layout/lib.rs must gate `pub mod scroll_shadow;` with feature cfg."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-scroll_shadow\")]",
        "out.push_str(crate::scroll_shadow::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout/css.rs should keep feature-gated css aggregation token `{needle}`."
        );
    }

    for (idx, _) in css_source.match_indices("out.push_str(crate::scroll_shadow::styles::CSS);") {
        let prefix = &css_source[..idx];
        let prev_line = prefix
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or_default()
            .trim();
        assert_eq!(
            prev_line, "#[cfg(feature = \"component-scroll_shadow\")]",
            "ui-layout/css.rs must gate scroll-shadow css aggregation with feature cfg."
        );
    }
}

#[test]
fn scroll_shadow_type_system_and_semantic_markers_form_machine_readable_contract() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "ScrollShadowEdgeState",
        "normalize_max_height",
        "scroll_shadow_type_system_and_semantic_markers_form_machine_readable_contract",
    ] {
        assert!(
            check2_source.contains(needle) || primitive_source.contains(needle),
            "type+semantic contract marker `{needle}` should be present in check2/primitive."
        );
    }

    for needle in [
        "pub enum ScrollShadowEdgeState",
        "None,",
        "Top,",
        "Bottom,",
        "Both,",
        "pub fn as_str(self) -> &'static str",
        "pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32>",
        "max_height_px.filter(|value| *value > 0)",
        "pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
        "edge_state_attr: edge_state_attr(has_top, has_bottom)",
        "scrollable_attr: is_scrollable.then_some(\"true\")",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives scroll_shadow should keep typed/normalized token `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::scroll_shadow::{",
        "ScrollShadowEdgeState",
        "ScrollShadowSemanticInput",
        "ScrollShadowStateInput",
        "resolve_semantic_state",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "scroll_shadow logic should consume primitive typed contract `{needle}`."
        );
    }

    for marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "scroll_shadow view should expose machine-readable semantic marker `{marker}`."
        );
    }

    for forbidden in [
        "edge_state: String",
        "edge_state: &str",
        "max_height_px: String",
        "data-state=move || format!(",
    ] {
        assert!(
            !primitive_source.contains(forbidden) && !view_source.contains(forbidden),
            "scroll_shadow should avoid stringly-typed state contract token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_paths_websys_free() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_shadow,inject-css",
        "cargo check -p ui-layout --no-default-features --features component-scroll_shadow,inject-css",
        "cargo check -p ui-layout",
        "scroll_shadow_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_paths_websys_free",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep platform contract marker `{needle}`."
        );
    }

    assert!(
        view_source
            .matches("#[cfg(target_arch = \"wasm32\")]")
            .count()
            >= 3,
        "scroll_shadow/view.rs should guard wasm-only branches via explicit cfg markers."
    );

    for needle in [
        "let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);",
        "let resize_closure = StoredValue::new_local(",
        "use leptos::wasm_bindgen::{JsCast, closure::Closure};",
        "leptos::web_sys::ResizeObserver::new",
        "on_cleanup(move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "scroll_shadow/view.rs should keep wasm-scoped browser branch token `{needle}`."
        );
    }

    assert!(
        !view_source.contains("use leptos::web_sys"),
        "scroll_shadow/view.rs should not have unguarded `use leptos::web_sys` imports."
    );

    for forbidden in ["web_sys", "ResizeObserver", "js_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden) && !styles_source.contains(forbidden),
            "logic/styles must stay platform-agnostic; found `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_reduced_motion_ssr_wasm_contract_is_consistent() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_shadow,inject-css",
        "cargo check -p ui-layout --no-default-features --features component-scroll_shadow,inject-css",
        "cargo check -p ui-layout",
        "scroll_shadow_reduced_motion_ssr_wasm_contract_is_consistent",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep reduced-motion/SSR/wasm marker `{needle}`."
        );
    }

    for forbidden in ["transition:", "animation:", "prefers-reduced-motion"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should keep reduced-motion safe no-timeline token `{forbidden}`."
        );
    }

    assert!(
        view_source
            .matches("#[cfg(target_arch = \"wasm32\")]")
            .count()
            >= 3,
        "scroll_shadow/view.rs should keep wasm-only enhancement behind explicit cfg."
    );

    for semantic_marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "ScrollShadow should keep stable semantic marker across SSR/wasm `{semantic_marker}`."
        );
    }

    for wasm_only in [
        "let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);",
        "leptos::web_sys::ResizeObserver::new",
    ] {
        assert!(
            view_source.contains(wasm_only),
            "ScrollShadow wasm branch should keep enhancement token `{wasm_only}`."
        );
    }
}

#[test]
fn scroll_shadow_performance_governance_budget_is_repeatable_attributable_and_blocking() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "UiPerfBudget::mount_only(120.0)",
        "data-perf-violation",
        "edge_state.get_untracked() != next_state",
        "scroll_shadow_performance_governance_budget_is_repeatable_attributable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget baseline token `{needle}`."
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
            "e2e coverage should keep blocking perf assertion `{needle}`."
        );
    }

    for marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "if edge_state.get_untracked() != next_state {",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollShadow should keep attributable perf marker or write-guard `{marker}`."
        );
    }

    let memo_count = view_source.matches("Memo::new(").count();
    assert!(
        memo_count <= 1,
        "ScrollShadow reactive budget exceeded: expected <= 1 `Memo::new`, found {memo_count}.",
    );
    let effect_count = view_source.matches("Effect::new(").count();
    assert!(
        effect_count <= 1,
        "ScrollShadow reactive budget exceeded: expected <= 1 `Effect::new`, found {effect_count}.",
    );
    let derive_count = view_source.matches("Signal::derive(").count();
    assert_eq!(
        derive_count, 0,
        "ScrollShadow should avoid `Signal::derive` loops for predictable update budget.",
    );

    for forbidden in [
        "animation:",
        "transition:",
        "SpringAnimator::new",
        "ui_motion",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "ScrollShadow should keep performance-safe no-motion-loop token `{forbidden}`.",
        );
    }

    assert!(
        todo_source.contains("render_count"),
        "Performance governance should keep explicit render_count follow-up tracking until framework support lands.",
    );
}

#[test]
fn scroll_shadow_view_macro_complexity_is_bounded_and_non_redundant() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "view!` 数量 `<= 1`",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_shadow,inject-css",
        "scroll_shadow_view_macro_complexity_is_bounded_and_non_redundant",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep view-macro complexity marker `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 1,
        "ScrollShadow view macro complexity regression: expected <= 1 `view!` block, found {view_macro_count}.",
    );

    let template_div_count = view_source.matches("<div").count();
    assert!(
        template_div_count <= 2,
        "ScrollShadow template should stay two-level (`root + viewport`), found {template_div_count} `<div` nodes.",
    );

    for singleton in [
        "data-slot=\"scroll-shadow\"",
        "data-slot=\"scroll-shadow-viewport\"",
        "on:scroll=on_scroll",
        "{children()}",
    ] {
        assert_eq!(
            view_source.matches(singleton).count(),
            1,
            "ScrollShadow view should keep single semantic slot/binding `{singleton}` to avoid repeated macro fragments.",
        );
    }

    assert!(
        !view_source.contains("fn render_"),
        "ScrollShadow does not require local render helper extraction at current macro complexity."
    );
}

#[test]
fn scroll_shadow_functional_split_policy_remains_noise_free_for_simple_component() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");

    assert!(
        view_source.contains("pub fn ScrollShadow("),
        "ScrollShadow should keep one explicit public component entry."
    );
    assert!(
        view_source.contains(") -> impl IntoView {"),
        "ScrollShadow should keep function-style return (`impl IntoView`)."
    );
    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ScrollShadow should not introduce extra local #[component] abstractions for simple view fragments."
    );

    for forbidden in ["#[component]\nfn", "fn render_", "fn sub_component"] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow simple view should avoid unnecessary local abstraction token `{forbidden}`."
        );
    }

    for marker in [
        "data-slot=\"scroll-shadow\"",
        "data-slot=\"scroll-shadow-viewport\"",
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollShadow semantic marker `{marker}` should remain stable under function-split policy."
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "N/A：`ScrollShadow` 当前 `view.rs` 仅一个顶层组件入口",
        "scroll_shadow_functional_split_policy_remains_noise_free_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep function-split governance marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_static_fragment_constantization_contract_is_not_applicable_and_remains_clean() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A：`ScrollShadow` 组件自身不承载复杂静态资源",
        "styles.rs::CSS",
        "scroll_shadow_static_fragment_constantization_contract_is_not_applicable_and_remains_clean",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep static-fragment contract marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("<div").count(),
        2,
        "ScrollShadow should keep a minimal two-node static skeleton (`root + viewport`) without repeated static fragment construction."
    );

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "const STATIC_",
        "const TEMPLATE_",
        "<h1",
        "<p",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should avoid complex static-fragment token `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "ScrollShadow static style asset should stay centralized in `styles.rs::CSS`."
    );
}

#[test]
fn scroll_shadow_inner_html_contract_remains_not_applicable_and_injection_free() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`ScrollShadow` 组件实现不使用 `inner_html`",
        "scroll_shadow_inner_html_contract_remains_not_applicable_and_injection_free",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep inner_html safety marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "onerror=",
        "onclick=",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "ScrollShadow should remain injection-free and avoid forbidden token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("{children()}"),
        "ScrollShadow should continue to render typed children slot rather than HTML string injection."
    );
}

#[test]
fn scroll_shadow_headless_web_ssr_mutex_contract_is_preserved() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let components_cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "scroll_shadow_headless_web_ssr_mutex_contract_is_preserved",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep headless mutex marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless/lib.rs should keep web/ssr mutex guard `{needle}`."
        );
    }

    assert!(
        components_cargo_source.contains("ui-headless = { path = \"../ui-headless\" }"),
        "ui-layout should depend on ui-headless via centralized crate dependency."
    );
    assert!(
        !components_cargo_source
            .contains("ui-headless = { path = \"../ui-headless\", features = [\"web\", \"ssr\"] }"),
        "ui-layout must not force-enable both ui-headless web+ssr features."
    );

    for forbidden in ["ui_headless::", "use ui_headless"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "scroll_shadow component should not directly couple to ui-headless token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let semantics_source = load_source("tests/scroll_shadow_semantics.rs");

    for needle in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot",
        "受控/非受控与 disabled 轴为 N/A",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep semantic-testing marker `{needle}`."
        );
    }

    for semantic_test in [
        "fn scroll_shadow_emits_baseline_style_state_data_attributes()",
        "fn scroll_shadow_state_observability_contract_uses_stable_closed_markers()",
        "fn scroll_shadow_controlled_uncontrolled_contract_is_explicitly_not_applicable()",
        "fn scroll_shadow_async_contract_is_explicitly_not_applicable()",
        "fn scroll_shadow_motion_contract_avoids_component_local_motion_engine_and_relies_on_global_stub()",
    ] {
        assert!(
            semantics_source.contains(semantic_test),
            "ScrollShadow semantic suite should include contract-oriented test `{semantic_test}`."
        );
    }

    let snapshot_macro = ["assert_", "snapshot!("].concat();
    let insta_snapshot_macro = ["insta::assert_", "snapshot!("].concat();
    let jest_snapshot_call = ["toMatch", "Snapshot("].concat();

    for forbidden_snapshot in [
        snapshot_macro.as_str(),
        insta_snapshot_macro.as_str(),
        jest_snapshot_call.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden_snapshot),
            "ScrollShadow semantic suite should not rely on visual snapshot token `{forbidden_snapshot}`."
        );
    }

    for interaction_marker in [
        "on:scroll=on_scroll",
        "compute_scroll_shadow_edges",
        "resolve_edge_state",
        "fn scroll_shadow_motion_contract_avoids_component_local_motion_engine_and_relies_on_global_stub()",
    ] {
        assert!(
            view_source.contains(interaction_marker)
                || semantics_source.contains(interaction_marker),
            "ScrollShadow semantic contract should cover interaction/platform marker `{interaction_marker}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_semantic_contract_first_testing_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let semantics_source = load_source("tests/scroll_shadow_semantics.rs");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot",
        "scroll_shadow_check2_marks_semantic_contract_first_testing_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep semantic-contract-first testing evidence `{needle}`."
        );
    }

    for semantic_marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "ScrollShadow view should keep semantic marker `{semantic_marker}`."
        );
        assert!(
            semantics_source.contains(semantic_marker.split('=').next().unwrap_or("")),
            "ScrollShadow semantics suite should cover marker family for `{semantic_marker}`."
        );
    }

    for semantic_test in [
        "fn scroll_shadow_emits_baseline_style_state_data_attributes()",
        "fn scroll_shadow_state_observability_contract_uses_stable_closed_markers()",
        "fn scroll_shadow_state_normalization_is_centralized_in_logic_layer()",
        "fn scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot()",
    ] {
        assert!(
            semantics_source.contains(semantic_test),
            "ScrollShadow semantics suite should keep contract test `{semantic_test}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_e2e_selector_stability_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "scroll_shadow_check2_marks_e2e_selector_stability_complete",
        "scroll_shadow_e2e_contract_uses_semantic_selectors_and_stable_wasm_wait_strategy",
        "scroll_shadow_e2e_scope_marks_async_motion_ready_settled_as_na_for_sync_scroll_container",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep E2E-stability completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_e2e_contract_uses_semantic_selectors_and_stable_wasm_wait_strategy() {
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let docs_layout_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for required in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "value.includes(\"/components/\")",
        "href.split(\"/components/\")[1]",
    ] {
        assert!(
            e2e_source.contains(required),
            "docs-app coverage E2E should keep stable semantic selector/wait marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn scroll_shadow() -> AnyView {",
        "slug=\"scroll-shadow\"",
        "<ScrollShadow>",
        "data-slot=\"scroll-shadow\"",
    ] {
        assert!(
            docs_layout_source.contains(required) || view_source.contains(required),
            "ScrollShadow docs/render path should keep semantic routing marker `{required}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "docs-app coverage E2E should avoid unstable wait token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_e2e_scope_marks_async_motion_ready_settled_as_na_for_sync_scroll_container() {
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for required in [
        "on:scroll=on_scroll",
        "compute_scroll_shadow_edges",
        "resolve_edge_state",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "ScrollShadow should keep sync semantic-derivation path marker `{required}`."
        );
    }

    for forbidden in [
        "create_resource",
        "spawn_local",
        "async fn",
        "Future",
        "is_loading",
        "aria-busy",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ScrollShadow should keep async-ready/settled branch N/A without `{forbidden}`."
        );
    }

    for forbidden_motion in [
        "transition:",
        "animation:",
        "@keyframes",
        "data-motion-state",
    ] {
        assert!(
            !styles_source.contains(forbidden_motion),
            "ScrollShadow styles should remain non-animated and avoid `{forbidden_motion}`."
        );
    }

    assert!(
        !manifest_dir.join("src/scroll_shadow/motion.rs").exists(),
        "ScrollShadow should keep component-local motion path N/A for ready/settled branch."
    );

    for forbidden_e2e in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden_e2e),
            "E2E wait strategy should stay semantic-ready based without `{forbidden_e2e}`."
        );
    }
}

#[test]
fn scroll_shadow_component_file_responsibilities_are_layered_correctly() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_file = manifest_dir.join("src/scroll_shadow/motion.rs");

    for needle in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "scroll_shadow_component_file_responsibilities_are_layered_correctly",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep component-file responsibility marker `{needle}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ScrollShadowEdges, compute_scroll_shadow_edges};",
        "pub use view::ScrollShadow;",
    ] {
        assert!(
            mod_source.contains(needle),
            "scroll_shadow/mod.rs should keep minimal export boundary `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub const CSS",
        "fn ScrollShadow(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "scroll_shadow/mod.rs should not carry implementation detail token `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::scroll_shadow::{",
        "pub fn normalize_optional_text(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "scroll_shadow/logic.rs should keep assembly-layer token `{needle}`."
        );
    }

    for forbidden in [
        "NodeRef",
        "web_sys",
        "ResizeObserver",
        "view! {",
        "style=",
        "on:scroll",
        "pub const CSS",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "scroll_shadow/logic.rs should not carry view/platform/style token `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-scroll-shadow-max-h)",
        "var(--ui-space-md)",
        "var(--ui-bg)",
        "data-state=\"both\"",
    ] {
        assert!(
            styles_source.contains(needle),
            "scroll_shadow/styles.rs should keep token-first static-style token `{needle}`."
        );
    }

    for forbidden in [
        "format!(",
        "web_sys",
        "ResizeObserver",
        "on:scroll",
        "NodeRef",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "scroll_shadow/styles.rs should not include runtime/view token `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn ScrollShadow(",
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "scroll_shadow/view.rs should keep structure + logic-mount token `{needle}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "pub use ui_state_primitives::scroll_shadow::{",
        "max_height_px.unwrap_or(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "scroll_shadow/view.rs should not absorb logic/style primitive-definition token `{forbidden}`."
        );
    }

    assert!(
        !motion_file.exists(),
        "scroll_shadow/motion.rs should remain absent while motion contract is N/A for this component."
    );
}

#[test]
fn scroll_shadow_spec_file_is_not_introduced_for_simple_component() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_file = manifest_dir.join("src/scroll_shadow/spec.rs");

    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "N/A：`ScrollShadow` 为简单容器组件，不存在外部 Schema 契约或复杂配置固化需求",
        "scroll_shadow_spec_file_is_not_introduced_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep spec-file governance marker `{needle}`."
        );
    }

    assert!(
        !spec_file.exists(),
        "scroll_shadow/spec.rs should not exist for simple component contracts."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "scroll_shadow/mod.rs should not expose spec module token `{forbidden}`."
        );
    }

    for required in [
        "pub(super) fn scroll_shadow() -> AnyView",
        "title=\"ScrollShadow\"",
    ] {
        assert!(
            docs_source.contains(required),
            "ScrollShadow docs should stay in docs page instead of adding `spec.rs`; missing `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_motion_contract_avoids_component_local_motion_engine_and_relies_on_global_stub() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_file = manifest_dir.join("src/scroll_shadow/motion.rs");

    for needle in [
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep motion governance marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_motion",
        "attach_motion",
        "spring::",
        "keyframes::",
        "MotionKeyframe",
        "MotionOptions",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ScrollShadow should not embed component-local motion engine token `{forbidden}`."
        );
    }

    for forbidden in ["transition:", "animation:", "prefers-reduced-motion"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should avoid local timeline token `{forbidden}` and keep motion N/A."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm stub contract `{needle}`."
        );
    }

    assert!(
        !motion_file.exists(),
        "ScrollShadow currently declares motion N/A and should not require `src/scroll_shadow/motion.rs`."
    );
}

#[test]
fn scroll_shadow_theme_contract_is_token_first_and_avoids_hardcoded_visual_constants() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    let theme_markers = [
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-layout/src/<component>/styles.rs` 消费。",
        "新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。",
    ];
    for marker in theme_markers {
        assert!(
            check2_source.contains(marker),
            "scroll_shadow/check2.md should keep theme governance marker `{marker}`."
        );
    }

    for needle in [
        "var(--ui-scroll-shadow-max-h)",
        "height: var(--ui-space-md);",
        "var(--ui-bg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "ScrollShadow styles should consume theme/token variable `{needle}`."
        );
    }

    for forbidden in ["280px", "18px", "120ms", " ease"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollShadow styles should not keep hardcoded visual constant `{forbidden}`."
        );
    }

    for needle in ["--ui-space-md", "--ui-space-lg", "--ui-bg"] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css variable emitter should provide `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn scroll_shadow() -> AnyView",
        "title=\"ScrollShadow\"",
        "slug=\"scroll-shadow\"",
        "Playground title=\"Default Scrollable\"",
        "Playground title=\"Custom Height + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for ScrollShadow.",
        );
    }
}

#[test]
fn scroll_shadow_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Default Scrollable\"",
        "<ScrollShadow max_height_px=160>",
        "{(1..=20)",
        "class=\"docs-scroll-shadow-item\"",
        "title=\"Custom Height + Class\"",
        "<ScrollShadow max_height_px=120 class_name=\"docs-scroll-shadow-custom\".to_string()>",
        "{(1..=16)",
        "{format!(\"Notification {idx}\")}",
    ] {
        assert!(
            source.contains(needle),
            "scroll-shadow docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "scroll_shadow_check2_marks_documentation_as_product_complete",
        "scroll_shadow_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "scroll_shadow_docs_are_beginner_friendly_with_default_then_advanced_path",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_layout_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let readme_path = manifest_dir.join("src/scroll_shadow/README.md");

    let has_equivalent_docs_app_entry = docs_index_source.contains("component_doc!(")
        && docs_index_source.contains("\"ScrollShadow\"")
        && docs_index_source.contains("\"scroll-shadow\"")
        && docs_index_source.contains("layout::scroll_shadow")
        && docs_layout_source.contains("pub(super) fn scroll_shadow() -> AnyView {")
        && docs_layout_source.contains("title=\"ScrollShadow\"")
        && docs_layout_source.contains("slug=\"scroll-shadow\"");

    assert!(
        readme_path.exists() || has_equivalent_docs_app_entry,
        "ScrollShadow should provide README or equivalent docs-app entry."
    );
}

#[test]
fn scroll_shadow_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));

    let hello_idx = docs_scroll_shadow_section
        .find("Playground title=\"Hello World\"")
        .unwrap_or_else(|| panic!("ScrollShadow docs should include Hello World playground."));
    let default_idx = docs_scroll_shadow_section
        .find("Playground title=\"Default Scrollable\"")
        .unwrap_or_else(|| {
            panic!("ScrollShadow docs should include Default Scrollable playground.")
        });
    let advanced_idx = docs_scroll_shadow_section
        .find("Playground title=\"Custom Height + Class\"")
        .unwrap_or_else(|| panic!("ScrollShadow docs should include advanced playground."));

    assert!(
        hello_idx < default_idx && default_idx < advanced_idx,
        "ScrollShadow docs should present default path before advanced controls."
    );

    for needle in [
        r#"<ScrollShadow>
  <div class="docs-scroll-shadow-item">Activity</div>
</ScrollShadow>"#,
        "<ScrollShadow>",
        "<div class=\"docs-scroll-shadow-item\">\"Activity\"</div>",
    ] {
        assert!(
            docs_scroll_shadow_section.contains(needle),
            "ScrollShadow docs should keep zero-threshold Hello World marker `{needle}`."
        );
    }

    for forbidden in ["ui-state-primitives", "ui-headless", "state=", "Signal<"] {
        assert!(
            !docs_scroll_shadow_section.contains(forbidden),
            "ScrollShadow beginner docs should avoid architecture wiring token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_interactive_playground_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "非 AI Spec 组件",
        "scroll_shadow_check2_marks_interactive_playground_complete",
        "scroll_shadow_interactive_playground_supports_props_state_feedback_preview",
        "scroll_shadow_playground_acceptance_surface_is_repeatable_via_docs_coverage_e2e",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep interactive-playground completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_interactive_playground_supports_props_state_feedback_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Default Scrollable\" code_signal=default_code>",
        "<Playground title=\"Custom Height + Class\" code_signal=custom_class_code>",
        "<ScrollShadow max_height_px=160>",
        "<ScrollShadow max_height_px=120 class_name=\"docs-scroll-shadow-custom\".to_string()>",
        "{(1..=20)",
        "{(1..=16)",
    ] {
        assert!(
            docs_scroll_shadow_section.contains(needle),
            "ScrollShadow docs playground section should include interactive marker `{needle}`."
        );
    }

    for needle in [
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground runtime should keep interactive preview capability marker `{needle}`."
        );
    }

    for needle in [
        "on:scroll=on_scroll",
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollShadow view should expose interaction-feedback semantic marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_playground_acceptance_surface_is_repeatable_via_docs_coverage_e2e() {
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "await page.goto(\"/#/components\");",
        "const slugs = uniq(",
        "href.split(\"/components/\")[1]",
        "for (const slug of slugs",
        "await page.goto(`/#/components/${slug}`);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs-app coverage E2E should keep repeatable acceptance-surface marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "docs-app coverage E2E should avoid flaky wait token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../ui-components/src/code_block/view.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");

    for needle in [
        "let hello_code = Signal::derive(move || {",
        "let default_code = Signal::derive(move || {",
        "let custom_class_code = Signal::derive(move || {",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Default Scrollable\" code_signal=default_code>",
        "<Playground title=\"Custom Height + Class\" code_signal=custom_class_code>",
    ] {
        assert!(
            docs_scroll_shadow_section.contains(needle),
            "ScrollShadow docs should keep source-first example marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let resolved_code = Signal::derive(move || {",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep copy-ready import/code pipeline marker `{needle}`."
        );
    }

    assert!(
        playground_source.contains(
            "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";"
        ) || playground_source.contains(
            "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_layout::*;\";"
        ),
        "Playground default imports should keep copy-ready root import (`ui_components` global default or `ui_layout` explicit)."
    );

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "data-slot=\"code-block-code\"",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep one-click copy marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ScrollShadowState) -> String",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ScrollShadow implementation should keep API/source-sync marker `{needle}`."
        );
    }

    for needle in [
        "<ScrollShadow max_height_px=160>",
        "<ScrollShadow max_height_px=120 class_name=\"docs-scroll-shadow-custom\".to_string()>",
        "<div class=\"docs-scroll-shadow-item\">Activity</div>",
    ] {
        assert!(
            docs_scroll_shadow_section.contains(needle),
            "ScrollShadow docs snippet should stay synced with API marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_source_first_copy_paste_ready_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "component-scroll_shadow",
        "crates/ui-layout/src/scroll_shadow/{mod,logic,view,styles}.rs",
        "scroll_shadow_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
        "scroll_shadow_check2_marks_source_first_copy_paste_ready_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep source-first copy-ready completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_entry_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "### ScrollShadow 同步记录（2026-02-18）",
        "参数模型同步：`ScrollShadow` 维持 layout primitive 定位",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"ScrollShadow\", \"scroll-shadow\", \"Layout\", layout::scroll_shadow)` 暴露入口",
        "示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout.rs::scroll_shadow()` 保持 `Hello World`、`Default Scrollable`、`Custom Height + Class`",
        "Source-first / Copy-Paste Ready：ScrollShadow playground 示例继续通过 `code_signal` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code`",
        "HeroUI 对齐结论：保持“先用起来，再进阶”的体验路径",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep ScrollShadow sync marker `{needle}`."
        );
    }

    let has_docs_index_entry = pages_index_source.contains("component_doc!(")
        && pages_index_source.contains("\"ScrollShadow\"")
        && pages_index_source.contains("\"scroll-shadow\"")
        && pages_index_source.contains("layout::scroll_shadow");
    assert!(
        has_docs_index_entry,
        "ScrollShadow docs index entry should remain discoverable via component_doc markers."
    );

    for needle in [
        "pub(super) fn scroll_shadow() -> AnyView",
        "slug=\"scroll-shadow\"",
    ] {
        assert!(
            docs_entry_source.contains(needle),
            "ScrollShadow docs page should keep indexable marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
        "ScrollShadowStateInput",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ScrollShadowState) -> String",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "ScrollShadow parameter model marker `{needle}` should stay in implementation."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "scroll_shadow_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "scroll_shadow_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep HeroUI/docs-sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_status_primitives_remains_dom_and_style_free() {
    let primitives_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");

    for forbidden in [
        "use leptos",
        "leptos::",
        "web_sys::",
        "wasm_bindgen",
        "view! {",
        "NodeRef<",
        "on:click",
        "style=",
        ".ui-",
        "var(--ui-",
    ] {
        assert!(
            !primitives_source.contains(forbidden),
            "ui-state-primitives scroll_shadow contract should avoid DOM/style dependency `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_ui_headless_remains_visual_and_motion_free() {
    let headless_scroll_area_source = load_source("../ui-headless/src/scroll_area.rs");

    for forbidden in [
        ".ui-",
        "ui-scroll-shadow",
        "class=",
        "var(--ui-",
        "Spring",
        "keyframe",
        "animate(",
        "request_animation_frame",
    ] {
        assert!(
            !headless_scroll_area_source.contains(forbidden),
            "ui-headless scroll-area contract should avoid visual/motion orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ScrollAreaRootAttrs",
        "pub struct ScrollAreaViewportAttrs",
        "pub struct ScrollAreaSemanticState",
        "pub struct ScrollAreaContract",
        "pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract",
    ] {
        assert!(
            headless_scroll_area_source.contains(required),
            "ui-headless should keep typed semantic output marker `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_view_keeps_decisions_in_logic_layer() {
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");

    for required in [
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollShadow view should consume centralized logic output via `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::scroll_shadow::resolve_state(",
        "ui_state_primitives::scroll_shadow::resolve_semantic_state(",
        "normalize_max_height(",
        "DEFAULT_MAX_HEIGHT_PX",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should not hide key state-decision rule `{forbidden}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::scroll_shadow::{",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ScrollShadowState) -> String",
    ] {
        assert!(
            logic_source.contains(required),
            "ScrollShadow key decision rule should stay centralized in logic layer `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract() {
    let view_source = load_source("src/scroll_shadow/view.rs");
    let primitives_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");
    let semantics_test_source = load_source("tests/scroll_shadow_semantics.rs");

    for required in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollShadow parameter naming/type contract should include `{required}`."
        );
    }

    for required in [
        "pub struct ScrollShadowStateInput",
        "pub struct ScrollShadowState",
        "pub const DEFAULT_MAX_HEIGHT_PX: u32 = 192;",
        "pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState",
    ] {
        assert!(
            primitives_source.contains(required),
            "ScrollShadow parameter/default normalization contract should keep `{required}`."
        );
    }

    for required in [
        "scroll_shadow_api_naming_contract_is_consistent_without_alias_drift",
        "scroll_shadow_default_value_source_is_logic_only",
        "scroll_shadow_state_normalization_is_centralized_in_logic_layer",
        "scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot",
    ] {
        assert!(
            semantics_test_source.contains(required),
            "ScrollShadow semantics suite should keep parameter-contract regression guard `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_parallel_array_api_is_absent_for_scroll_shadow_scope() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for forbidden in [
        "labels + children",
        "titles + panels",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
    ] {
        assert!(
            !docs_source.contains(forbidden) && !view_source.contains(forbidden),
            "ScrollShadow scope should avoid parallel-array/implicit semantic token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_public_api_does_not_leak_platform_or_runtime_types() {
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "web_sys::",
        "leptos::web_sys",
        "wasm_bindgen",
        "tokio::",
        "async_std::",
        "runtime::Handle",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !lib_source.contains(forbidden),
            "ScrollShadow public API boundary should avoid leaking platform/runtime token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_no_temporary_patch_contract_drift_tokens_in_scroll_shadow_scope() {
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");

    for forbidden in [
        "TODO temporary",
        "TEMP FIX",
        "HACK",
        "workaround",
        "quick fix",
        "remove later",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow should avoid temporary patch contract-drift marker `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless() {
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let headless_scroll_area_source = load_source("../ui-headless/src/scroll_area.rs");

    for required in [
        "pub use ui_state_primitives::scroll_shadow::{",
        "pub struct ScrollShadowStateInput",
        "pub struct ScrollShadowState",
        "pub struct ScrollShadowSemanticInput",
        "pub struct ScrollShadowSemanticState",
        "pub fn resolve_state(input: ScrollShadowStateInput) -> ScrollShadowState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
        "pub mod scroll_area;",
        "pub struct ScrollAreaContract",
    ] {
        assert!(
            logic_source.contains(required)
                || primitives_source.contains(required)
                || headless_lib_source.contains(required)
                || headless_scroll_area_source.contains(required),
            "ScrollShadow reusable state invariant should stay sunk to primitive/headless marker `{required}`."
        );
    }

    for forbidden in [
        "pub enum LocalScrollShadowState",
        "pub enum ScrollShadowMachine",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ScrollShadow logic should not keep reusable state machine locally `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "scroll_shadow_anti_pattern_status_primitives_remains_dom_and_style_free",
        "scroll_shadow_anti_pattern_ui_headless_remains_visual_and_motion_free",
        "scroll_shadow_anti_pattern_view_keeps_decisions_in_logic_layer",
        "scroll_shadow_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract",
        "scroll_shadow_anti_pattern_parallel_array_api_is_absent_for_scroll_shadow_scope",
        "scroll_shadow_anti_pattern_public_api_does_not_leak_platform_or_runtime_types",
        "scroll_shadow_anti_pattern_no_temporary_patch_contract_drift_tokens_in_scroll_shadow_scope",
        "scroll_shadow_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep forbidden anti-pattern completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "UiTraceEvent { ts_ms, component, kind }",
        "scroll_shadow_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep wasm-debug governance marker `{needle}`."
        );
    }

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-layout Cargo features should keep global wasm-debug opt-in token `{needle}`."
        );
    }

    for forbidden in [
        "scroll_shadow-wasm-debug",
        "component-scroll_shadow-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "ScrollShadow should not expose component-local wasm-debug feature `{forbidden}`."
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]",
        "mod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-layout crate root should keep shared wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep debug visual entry marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "UiTraceEventKind::Inspect",
        "events.push(event);",
        ".take(40)",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`."
        );
    }

    for marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "on:scroll=on_scroll",
        "let edges = compute_scroll_shadow_edges(scroll_top, client_height, scroll_height);",
        "let next_state = logic::resolve_edge_state(edges.top, edges.bottom);",
        "if edge_state.get_untracked() != next_state {",
        "set_edge_state.set(next_state);",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollShadow should keep traceable semantic/interaction marker `{marker}`."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug_proxy!",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "request_replay.run(",
        "UiDebugOverlay",
        "provide_ui_trace",
        "trace.emit(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ScrollShadow component layer should not leak debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_dx_playground_supports_hot_css_iteration_and_marks_persist_state_na() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dev_docs_script = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script = load_source("../../scripts/dev-web-demo.sh");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "compose_scoped_css + data-playground-scope + Show test + Restore original CSS",
        "可选状态保留在本组件文档场景按 N/A 处理",
        "scroll_shadow_dx_playground_supports_hot_css_iteration_and_marks_persist_state_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep DX governance marker `{needle}`."
        );
    }

    for needle in [
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "\"Restore original CSS\"",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep hot-css/workbench marker `{needle}`."
        );
    }

    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));

    for needle in [
        "pub(super) fn scroll_shadow() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Default Scrollable\" code_signal=default_code>",
        "<Playground title=\"Custom Height + Class\" code_signal=custom_class_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollShadow docs should keep isolated playground/workbench entry `{needle}`."
        );
    }

    for forbidden in [
        "Persist workbench state",
        "load_scroll_shadow_workbench_state",
        "save_scroll_shadow_workbench_state",
        "clear_scroll_shadow_workbench_state",
    ] {
        assert!(
            !docs_scroll_shadow_section.contains(forbidden),
            "ScrollShadow docs should keep persist-state path N/A and avoid token `{forbidden}`."
        );
    }

    for needle in ["#!/usr/bin/env bash", "trunk serve --open true"] {
        assert!(
            dev_docs_script.contains(needle) && dev_web_script.contains(needle),
            "dev scripts should keep fast local iteration entry `{needle}`."
        );
    }

    for forbidden in ["#[prop(optional)] state:", "#[prop(optional)] debug_state:"] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow base API should not force state/debug object wiring `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_engineering_contract_stays_spec_free_tracing_aligned_and_runtime_agnostic() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let combined = [
        load_source("src/scroll_shadow/mod.rs"),
        load_source("src/scroll_shadow/logic.rs"),
        load_source("src/scroll_shadow/view.rs"),
        load_source("src/scroll_shadow/styles.rs"),
    ]
    .join("\n");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/scroll_shadow/spec.rs");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "`crates/ui-layout/Cargo.toml` 保持 `component-scroll_shadow = []` 且不引入 `dep:serde/dep:serde_json`",
        "scroll_shadow_engineering_contract_stays_spec_free_tracing_aligned_and_runtime_agnostic",
    ] {
        assert!(
            check2_source.contains(needle),
            "scroll_shadow/check2.md should keep engineering governance marker `{needle}`."
        );
    }

    assert!(
        !spec_path.exists(),
        "ScrollShadow should keep spec/serde schema path as N/A for simple component scope."
    );

    for needle in [
        "component-scroll_shadow = []",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "Cargo feature table should keep engineering baseline marker `{needle}`."
        );
    }

    for forbidden in [
        "component-scroll_shadow = [\"dep:serde\"",
        "component-scroll_shadow = [\"dep:serde_json\"",
        "component-scroll_shadow = [\"dep:tracing\"",
        "scroll_shadow-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "ScrollShadow feature contract should remain minimal and avoid `{forbidden}`."
        );
    }

    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow component surface should not leak engineering/runtime token `{forbidden}`."
        );
    }

    for forbidden in [
        "spawn_local(",
        "spawn(",
        "async fn ",
        "Future<",
        "tokio::",
        "async_std::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow async boundary should remain runtime-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "pub fn ScrollShadow(",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
        "children: Children",
    ] {
        assert!(
            combined.contains(needle),
            "ScrollShadow public API should remain runtime-opaque and keep `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "UiTraceEventKind::Inspect",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "Global tracing semantics should stay unified via marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_ui_layout_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("src/active_highlight.rs");
    let headless_controllable = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../ui-headless/src/presence.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "#[cfg(feature = \"component-scroll_shadow\")]",
        "pub mod scroll_shadow;",
        "pub use root::UiRoot;",
        "pub use scroll_shadow::ScrollShadow;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib entry should keep stable export/gate marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-layout lib entry should not expose internal/detail marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-scroll_shadow\")]\n    out.push_str(crate::scroll_shadow::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry should keep feature-gated component aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "data-slot=\"ui-root\"",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight entry should keep shared style/motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn ScrollShadow(", "ui-scroll-shadow"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain shared utility without component business token `{forbidden}`."
        );
    }

    assert!(
        manifest_dir.join("src/active_highlight.rs").exists(),
        "ui-layout should keep shared `src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-layout should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-layout should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-layout should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
    );

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless layer should keep canonical primitive entry marker `{needle}`."
        );
    }

    for required in [
        "- [x] `ui-layout` 固定入口文件落点正确。",
        "`crates/ui-layout/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-layout/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-layout/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-layout/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-layout/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-layout/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-layout/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            check2_source.contains(required),
            "ScrollShadow checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_component_directory_standard_files_are_present_and_layered_without_render_spec_drift()
 {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2_source = load_source("src/scroll_shadow/check2.md");

    let mod_path = manifest_dir.join("src/scroll_shadow/mod.rs");
    let logic_path = manifest_dir.join("src/scroll_shadow/logic.rs");
    let styles_path = manifest_dir.join("src/scroll_shadow/styles.rs");
    let view_path = manifest_dir.join("src/scroll_shadow/view.rs");
    let motion_path = manifest_dir.join("src/scroll_shadow/motion.rs");
    let render_path = manifest_dir.join("src/scroll_shadow/render.rs");
    let spec_path = manifest_dir.join("src/scroll_shadow/spec.rs");

    assert!(mod_path.exists(), "ScrollShadow should keep `mod.rs`.");
    assert!(logic_path.exists(), "ScrollShadow should keep `logic.rs`.");
    assert!(
        styles_path.exists(),
        "ScrollShadow should keep `styles.rs`."
    );
    assert!(view_path.exists(), "ScrollShadow should keep `view.rs`.");
    assert!(
        !render_path.exists(),
        "ScrollShadow should not drift rendering entry to `render.rs`."
    );
    assert!(
        !spec_path.exists(),
        "ScrollShadow should not introduce `spec.rs` for simple component scope."
    );
    assert!(
        !motion_path.exists(),
        "ScrollShadow currently keeps motion mapping as N/A and should not define `motion.rs`."
    );

    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ScrollShadowEdges, compute_scroll_shadow_edges};",
        "pub use view::ScrollShadow;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable exports via `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub mod motion",
        "mod render",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not over-export implementation detail `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "web_sys", "ResizeObserver", "style="] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay normalization-only and avoid `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-scroll-shadow-max-h)",
        "var(--ui-bg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep static token-first css marker `{needle}`."
        );
    }

    for forbidden in ["format!(", "view! {", "web_sys", "ResizeObserver"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should stay static and avoid `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn ScrollShadow(",
        "view! {",
        "on:scroll=on_scroll",
        "logic::resolve_state(ScrollShadowStateInput {",
        "logic::resolve_semantic_state(ScrollShadowSemanticInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep render + logic mount marker `{needle}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "pub use ui_state_primitives::scroll_shadow::{",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not absorb style/primitive-definition token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            check2_source.contains(required),
            "ScrollShadow checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn scroll_shadow_agent_contract_markers_are_schema_like_machine_readable_and_whitelist_safe() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_shadow.rs");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let styles_source = load_source("src/scroll_shadow/styles.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitive_source}");
    let render_chain = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "data-state/data-scrollable/data-shadow-top/data-shadow-bottom/data-max-height/data-custom-class",
        "ScrollShadowEdgeState/ScrollShadowSemanticState",
        "scroll_shadow_agent_contract_markers_are_schema_like_machine_readable_and_whitelist_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "ScrollShadow checklist should keep agent-contract governance marker `{required}`."
        );
    }

    for marker in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollShadow should expose machine-readable Agent marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum ScrollShadowEdgeState",
        "pub struct ScrollShadowSemanticInput",
        "pub struct ScrollShadowSemanticState",
        "pub fn resolve_semantic_state(input: ScrollShadowSemanticInput) -> ScrollShadowSemanticState",
        "pub fn as_str(self) -> &'static str",
    ] {
        assert!(
            combined.contains(typed_source),
            "ScrollShadow Agent fields should remain type-derived via `{typed_source}`."
        );
    }

    // Non-complex component: `data-ui-schema` is optional and should not be faked via free-form strings.
    for forbidden in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "data-state=move || format!(",
        "data-scrollable=move || format!(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow should avoid free-form schema/string-splicing token `{forbidden}`."
        );
    }

    for required in [
        "on:scroll=on_scroll",
        "let edges = compute_scroll_shadow_edges(scroll_top, client_height, scroll_height);",
        "let next_state = logic::resolve_edge_state(edges.top, edges.bottom);",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollShadow intent/action/state traceability marker should exist `{required}`."
        );
    }

    for forbidden in [
        "<script",
        "javascript:",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !render_chain.contains(forbidden),
            "ScrollShadow render chain should remain whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "scroll_shadow_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            check2_source.contains(required),
            "ScrollShadow checklist should keep streaming-definition marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "fallback=snapshot",
        "data-ui-stream-",
        "data-ui-output-status",
        "data-stream",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow should stay non-LLM-streaming container and avoid protocol token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "scroll_shadow_check2_marks_snapshot_baseline_capability_complete",
        "scroll_shadow_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep snapshot-baseline completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));
    let combined = format!("{view_source}\n{logic_source}\n{docs_scroll_shadow_section}");

    for required in [
        "children: Children",
        "{children()}",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] max_height_px: Option<u32>",
        "logic::resolve_state(ScrollShadowStateInput {",
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Default Scrollable\" code_signal=default_code>",
        "<Playground title=\"Custom Height + Class\" code_signal=custom_class_code>",
    ] {
        assert!(
            combined.contains(required),
            "ScrollShadow should keep snapshot-baseline render marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-",
        "data-stream",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow snapshot baseline should remain protocol-agnostic without `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_streaming_requirement_by_component_scope_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "scroll_shadow_check2_marks_streaming_requirement_by_component_scope_complete",
        "scroll_shadow_streaming_requirement_is_optional_with_snapshot_fallback_and_semantic_continuity",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep streaming-requirement completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_streaming_requirement_is_optional_with_snapshot_fallback_and_semantic_continuity()
{
    let view_source = load_source("src/scroll_shadow/view.rs");
    let logic_source = load_source("src/scroll_shadow/logic.rs");
    let mod_source = load_source("src/scroll_shadow/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let docs_scroll_shadow_section = docs_source
        .split("pub(super) fn scroll_shadow() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout docs page should define scroll_shadow() section"));
    let combined =
        format!("{view_source}\n{logic_source}\n{mod_source}\n{docs_scroll_shadow_section}");

    for required in [
        "data-state=move || semantic_state.get().edge_state_attr",
        "data-scrollable=move || semantic_state.get().scrollable_attr",
        "data-shadow-top=move || semantic_state.get().shadow_top_attr",
        "data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr",
        "data-max-height=state.max_height_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "on:scroll=on_scroll",
        "let next_state = logic::resolve_edge_state(edges.top, edges.bottom);",
    ] {
        assert!(
            view_source.contains(required) || combined.contains(required),
            "ScrollShadow should keep semantic continuity marker `{required}` in snapshot fallback mode."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-",
        "data-stream",
        "data-output-status",
        "data-draft",
        "data-verified",
        "data-submittable",
        "retry",
        "reconnect",
        "fallback=snapshot",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollShadow should keep streaming-optional boundary clean without `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-layout` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "scroll_shadow_state_primitives_are_sourced_from_ui_state_primitives",
        "scroll_shadow_anti_pattern_ui_headless_remains_visual_and_motion_free",
        "scroll_shadow_motion_contract_avoids_component_local_motion_engine_and_relies_on_global_stub",
        "scroll_shadow_theme_contract_is_token_first_and_avoids_hardcoded_visual_constants",
        "scroll_shadow_component_file_responsibilities_are_layered_correctly",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep architecture-layer completion marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_a11y_i18n_l10n_contract_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let view_source = load_source("src/scroll_shadow/view.rs");

    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot",
        "scroll_shadow_docs_are_beginner_friendly_with_default_then_advanced_path",
        "scroll_shadow_agent_contract_markers_are_schema_like_machine_readable_and_whitelist_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep a11y/i18n completion marker `{needle}`."
        );
    }

    for forbidden in ["aria-label=\"", "title=\"", "placeholder=\""] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollShadow view should avoid hardcoded user-facing text token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_visual_desire_baseline_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "scroll_shadow_theme_contract_is_token_first_and_avoids_hardcoded_visual_constants",
        "scroll_shadow_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep visual-desire completion marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_repeatable_e2e_flow_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "scroll_shadow_playground_acceptance_surface_is_repeatable_via_docs_coverage_e2e",
        "scroll_shadow_e2e_contract_uses_semantic_selectors_and_stable_wasm_wait_strategy",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep repeatable-e2e completion marker `{needle}`."
        );
    }

    assert!(
        e2e_source.contains("await page.goto(`/#/components/${slug}`);"),
        "docs-app coverage e2e should keep repeatable component-route flow."
    );
}

#[test]
fn scroll_shadow_check2_marks_docs_examples_and_matrices_synced_complete() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "scroll_shadow_docs_page_covers_primary_playgrounds",
        "scroll_shadow_docs_playgrounds_lock_state_matrix_contract_values",
        "scroll_shadow_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep docs-sync completion marker `{needle}`."
        );
    }
}

#[test]
fn scroll_shadow_check2_marks_final_merge_gates_complete_with_full_gate_done() {
    let check2_source = load_source("src/scroll_shadow/check2.md");

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
        "scroll_shadow_state_primitives_are_sourced_from_ui_state_primitives",
        "scroll_shadow_discrete_state_axes_are_enum_typed_and_closed",
        "scroll_shadow_semantic_tests_prioritize_contract_over_visual_snapshot",
        "scroll_shadow_theme_contract_is_token_first_and_avoids_hardcoded_visual_constants",
        "scroll_shadow_check2_marks_semantic_contract_first_testing_complete",
        "scroll_shadow_api_naming_contract_is_consistent_without_alias_drift",
        "scroll_shadow_type_system_and_semantic_markers_form_machine_readable_contract",
        "scroll_shadow_state_observability_contract_uses_stable_closed_markers",
        "scroll_shadow_reduced_motion_ssr_wasm_contract_is_consistent",
        "scroll_shadow_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
        "$HOME/.cargo/bin/rustfmt crates/ui-layout/tests/scroll_shadow_semantics.rs",
        "$HOME/.cargo/bin/cargo test -p ui-layout --test scroll_shadow_semantics --no-default-features --features component-scroll_shadow,inject-css",
        "$HOME/.cargo/bin/cargo clippy -p ui-layout --test scroll_shadow_semantics --no-default-features --features component-scroll_shadow,inject-css -- -D warnings",
        "bash scripts/smoke-csr.sh apps/web-demo \"body:not(:has(#boot))\"",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollShadow checklist should keep final-merge-gate evidence marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。"),
        "ScrollShadow checklist should keep full-gate item complete after full validation chain."
    );
}
