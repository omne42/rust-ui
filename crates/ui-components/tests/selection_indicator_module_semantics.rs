use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn selection_indicator_compat_module_is_removed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/selection_indicator/mod.rs");
    assert!(!path.exists(), "compat module should not exist.");
}

#[test]
fn crate_root_does_not_register_selection_indicator_compat_module() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("pub mod selection_indicator;"),
        "crate root should not include legacy.",
    );
}

#[test]
fn selection_indicator_check2_includes_status_primitives_gate_and_sinking_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层",
        "已核验：`selection_indicator` 当前仅保留治理清单 `check2.md`",
        "选择状态原语统一来自 `crates/ui-state-primitives/src/selection.rs`",
        "并由 `crates/ui-state-primitives/src/lib.rs` 导出",
        "`status-primitives` 定义：纯状态原语层",
        "所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。",
        "下沉判定依据是“稳定状态不变量”",
        "处理方式固定：先下沉到 `ui-state-primitives/src/<capability>.rs`",
        "下沉后的原语必须有 `ui-state-primitives` 单元测试",
        "桥接规范：`ui-state-primitives` 结构体必须是 POJO",
        "消费规范：`ui-headless` 或组件 `logic.rs` 负责解包 `Signal` 当前值传入 primitive 方法",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep status-primitives governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_component_surface_absent_blocks_local_state_machine() {
    assert!(
        path_exists("src/selection_indicator/check2.md"),
        "selection_indicator should keep checklist entry for governance."
    );

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator compatibility component file must stay absent: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_status_primitives_live_in_ui_state_primitives_and_remain_pure_rust() {
    let primitives_lib = load_source("../ui-state-primitives/src/lib.rs");
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");

    assert!(
        primitives_lib.contains("pub mod selection;"),
        "ui-state-primitives should export `selection` primitive module."
    );

    for needle in [
        "pub struct SingleSelectionState",
        "pub struct MultipleSelectionState",
        "pub struct SingleSelectionStateOptions",
        "pub struct MultipleSelectionStateOptions",
        "pub fn use_single_selection_state(",
        "pub fn use_multiple_selection_state(",
        "#[cfg(test)]",
    ] {
        assert!(
            selection_source.contains(needle),
            "selection primitive should include `{needle}`."
        );
    }

    for forbidden in ["leptos", "web_sys", "wasm_bindgen", "view! {", "NodeRef"] {
        assert!(
            !selection_source.contains(forbidden),
            "ui-state-primitives selection module must stay framework/DOM-free; found `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_ui_headless_gate_and_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `ui-headless` 定义：交互与 A11y 原语层",
        "已核验：`selection_indicator` 当前无本地组件实现文件",
        "共享 A11y 契约落点保持在 `crates/ui-headless/src/a11y.rs`",
        "并通过 `locale_attrs + A11yDirection` 提供 `lang/dir` 接入",
        "输入边界：消费 `status-primitives` 状态 + 用户输入事件（keyboard/pointer/focus）+ 环境能力（web/ssr）。",
        "输出边界：只输出语义契约（attrs/handlers/state）；组件层只负责挂载与组合，不得把语义判断塞回 `view.rs`。",
        "A11y 契约与共享工具落点固定在 `crates/ui-headless/src/a11y.rs`；组件只在 `view.rs` 挂载，不在组件层重写。",
        "语义契约必须提供 `lang` / `dir`（LTR/RTL）接入能力；headless 不硬编码用户可见文本，文案由 i18n/l10n 层提供。",
        "禁止放在 `ui-headless`：视觉 class 选择、CSS 规则、组件 slot 布局、组件专属动效编排、业务文案。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep ui-headless governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_ui_headless_a11y_contract_remains_centralized_and_typed() {
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "pub mod a11y;",
        "pub use a11y::{",
        "A11yDirection",
        "locale_attrs",
        "aria_controls_when_open",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless crate root should expose centralized a11y contract marker `{needle}`."
        );
    }

    for needle in [
        "pub enum A11yDirection",
        "pub struct A11yLocaleAttrs",
        "pub fn locale_attrs(",
        "pub fn aria_controls_when_open(",
        "pub struct DisclosureTriggerA11yAttrs",
        "pub struct PopupTriggerA11yAttrs",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "ui-headless a11y module should keep typed contract marker `{needle}`."
        );
    }

    for forbidden in ["class:", "class_name", "@keyframes", "animation:"] {
        assert!(
            !headless_a11y.contains(forbidden),
            "ui-headless a11y module must stay semantics-only; found `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_ui_motion_gate_and_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `ui-motion` 定义：动效能力与契约执行层",
        "已核验：通用动效数学与执行后端位于 `crates/ui-motion/src/{spring,keyframes,web}.rs`",
        "在 non-wasm 提供 `web::animate` no-op/stub",
        "`selection_indicator` 当前无本地 `motion.rs`",
        "不存在组件语义状态到动效契约的私有重实现",
        "放在 `crates/ui-motion`：通用动画数学与执行后端",
        "放在 `crates/ui-components/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract",
        "禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。",
        "禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。",
        "非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep ui-motion governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_ui_motion_contract_remains_engine_only_with_non_wasm_noop() {
    let motion_lib = load_source("../ui-motion/src/lib.rs");
    let motion_spring = load_source("../ui-motion/src/spring.rs");
    let motion_web = load_source("../ui-motion/src/web.rs");

    for needle in [
        "pub mod keyframes;",
        "pub mod options;",
        "pub mod presets;",
        "pub mod spring;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion crate root should keep engine/noop contract marker `{needle}`."
        );
    }

    for needle in [
        "pub struct SpringConfig",
        "pub fn sanitize_config(",
        "pub struct SpringAnimator",
        "if crate::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_spring.contains(needle),
            "ui-motion spring engine should keep runtime contract marker `{needle}`."
        );
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool",
        "\"(prefers-reduced-motion: reduce)\"",
        "pub fn animate(element: &web_sys::Element, keyframes: &[MotionKeyframe], options: MotionOptions)",
    ] {
        assert!(
            motion_web.contains(needle),
            "ui-motion web backend should keep marker `{needle}`."
        );
    }

    for forbidden in ["aria_", "role:", "slot", "keyboard", "on_click"] {
        assert!(
            !motion_lib.contains(forbidden),
            "ui-motion crate root must avoid component semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_ui_theme_gate_and_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层",
        "已核验：`ui-theme` 的 token/映射/变量输出落点分别位于 `crates/ui-theme/src/{tokens,theme,css}.rs`",
        "三轴上下文由 `ThemeContext { system, color, scale }` 在 `theme.rs` 定义",
        "尺寸与主题回归由 `crates/ui-theme/tests/token_scale_baseline.rs` 覆盖",
        "`selection_indicator` 目录无样式实现文件，无法重建主题",
        "Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量",
        "三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。",
        "Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。",
        "量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`",
        "主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep ui-theme governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_ui_theme_contract_remains_token_theme_css_only() {
    let theme_tokens = load_source("../ui-theme/src/tokens.rs");
    let theme_mapping = load_source("../ui-theme/src/theme.rs");
    let theme_css = load_source("../ui-theme/src/css.rs");
    let theme_baseline_test = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "single source of truth for token taxonomy and baselines",
        "Theme mapping happens in `theme.rs`; CSS variable emission happens in `css.rs`",
        "pub struct ThemeTokens",
        "pub struct SliderLayoutTokens",
    ] {
        assert!(
            theme_tokens.contains(needle),
            "ui-theme tokens source should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub fn slider_layout_tokens(",
        "ThemeTokens {",
    ] {
        assert!(
            theme_mapping.contains(needle),
            "ui-theme mapping layer should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub const BASE_CSS: &str",
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "--ui-system:",
        "--ui-color:",
        "--ui-scale:",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css layer should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn token_scale_baselines_are_regression_testable()",
        "medium.tokens.typography.font_size_100_px",
        "large.tokens.typography.font_size_200_px",
        "medium.tokens.slider_layout.max_width_px",
        "fn css_variables_emit_theme_axes()",
    ] {
        assert!(
            theme_baseline_test.contains(needle),
            "ui-theme baseline regression test should keep marker `{needle}`."
        );
    }

    for needle in [
        "Token 统一基线落点固定",
        "三轴上下文（`system/color/scale`）在 `theme.rs` 定义",
        "量化尺寸基准必须可回归",
        "主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线",
        "主题层只输出 `theme/tokens/base css` 与变量",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should keep ui-theme contract marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "#[component]", "on:click", "NodeRef"] {
        assert!(
            !theme_tokens.contains(forbidden),
            "ui-theme tokens layer must stay framework-agnostic; found `{forbidden}`."
        );
        assert!(
            !theme_mapping.contains(forbidden),
            "ui-theme mapping layer must stay framework-agnostic; found `{forbidden}`."
        );
        assert!(
            !theme_css.contains(forbidden),
            "ui-theme css layer must stay framework-agnostic; found `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_ui_components_gate_and_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层",
        "已核验：`crates/ui-components/Cargo.toml` 依赖 `ui-state-primitives/ui-headless/ui-motion/ui-theme`",
        "`crates/ui-components/src/lib.rs` 通过 `component-*` feature gate 暴露组件入口",
        "`selection_indicator` 当前仅治理清单，无 `logic.rs/view.rs/styles.rs/motion.rs`",
        "不存在本地状态机重写与 `web-sys`/DOM 公共 API 泄露",
        "`logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。",
        "组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。",
        "对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep ui-components governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_ui_components_boundary_remains_feature_gated_and_layered() {
    let cargo_toml = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "ui-headless = { path = \"../ui-headless\" }",
        "ui-motion = { path = \"../ui-motion\" }",
        "ui-state-primitives = { path = \"../ui-state-primitives\" }",
        "ui-theme = { path = \"../ui-theme\" }",
    ] {
        assert!(
            cargo_toml.contains(needle),
            "ui-components dependency graph should include layered dependency `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-accordion\")]",
        "pub use ui_accordion as accordion;",
        "#[cfg(feature = \"component-slider\")]",
        "pub mod slider;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
        "pub use ui_theme::Theme;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components crate root should keep feature-gated/public boundary marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use web_sys",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components public root must avoid leaking low-level web details marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator component surface should remain absent: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_api_naming_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀",
        "已核验（N/A-无公共 API 面）：`selection_indicator` 当前未导出组件模块",
        "不存在新增 props/回调命名与同义别名漂移",
        "迁移路径保持“无兼容别名、无新命名引入”",
        "布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。",
        "同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。",
        "公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep api naming governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_has_no_public_api_naming_surface_to_drift() {
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should not expose public API surface marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator implementation file must stay absent for N/A API naming scope: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_controlled_uncontrolled_pair_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`",
        "已核验（N/A-组件无状态轴）：`selection_indicator` 当前未导出组件与 props，不存在可控状态入口",
        "`controlled.rs` 的 `value/default_value/on_change`",
        "`selection.rs` 的 `selected_key/default_selected_key/on_selection_change`",
        "组件侧无“半受控”二次实现",
        "受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。",
        "非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。",
        "受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep controlled/uncontrolled governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_controlled_uncontrolled_contract_is_primitive_backed() {
    let controlled_source = load_source("../ui-state-primitives/src/controlled.rs");
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "pub struct ControlledStateOptions<T>",
        "pub value: Option<T>",
        "pub default_value: Option<T>",
        "pub on_change: Option<ControlledOnChange<T>>",
        "let is_controlled = value.is_some();",
        "pub fn sync_controlled(&mut self, value: Option<T>)",
        "if !self.is_controlled {",
    ] {
        assert!(
            controlled_source.contains(needle),
            "ui-state-primitives controlled primitive should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub struct SingleSelectionStateOptions",
        "pub selected_key: Option<SelectedKey>",
        "pub default_selected_key: Option<SelectedKey>",
        "pub on_selection_change: Option<OnSingleSelectionChange>",
        "single_controlled_calls_on_change_but_does_not_update_internal",
        "multiple_controlled_calls_on_change_but_does_not_update_internal",
    ] {
        assert!(
            selection_source.contains(needle),
            "ui-state-primitives selection primitive should keep controlled/uncontrolled marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep zero public controlled/uncontrolled API surface marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local state axis implementation: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_single_default_source_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "已核验（N/A-组件无逻辑视图实现）：`selection_indicator` 当前不存在 `logic.rs/view.rs`",
        "默认值优先级由 `ui-state-primitives` 统一原语集中定义",
        "`use_controlled_state` 内部以 `value -> default_value -> initial` 单一路径归一",
        "由 `selection.rs` 通过 `ControlledStateOptions` 复用",
        "默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。",
        "`view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。",
        "一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep default-source governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_default_priority_is_centralized_in_primitives() {
    let controlled_source = load_source("../ui-state-primitives/src/controlled.rs");
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");

    for needle in [
        "let value = value.clone().or(default_value.clone()).unwrap_or(initial);",
        "let default_value = default_value.unwrap_or_else(|| value.clone());",
        "pub struct ControlledStateOptions<T>",
        "pub value: Option<T>",
        "pub default_value: Option<T>",
    ] {
        assert!(
            controlled_source.contains(needle),
            "controlled primitive should keep centralized default-priority marker `{needle}`."
        );
    }

    for needle in [
        "ControlledStateOptions {",
        "value: options.selected_key,",
        "default_value: options.default_selected_key,",
        "value: options.selected_keys,",
        "default_value: options.default_selected_keys,",
    ] {
        assert!(
            selection_source.contains(needle),
            "selection primitive should route defaults through shared controlled options marker `{needle}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep zero local default branches: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_state_normalization_centralization_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "已核验（N/A-组件无状态渲染路径）：`selection_indicator` 当前无 `logic.rs/view.rs/styles.rs`",
        "状态归一化与来源标记由 `ui-state-primitives` 原语集中处理",
        "`selection.rs` 的 `use_*_selection_state + is_controlled/default_*`",
        "组件层无事件回调与样式分支可重建规则",
        "输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。",
        "事件处理器只触发状态变更，不重建状态机规则。",
        "样式层只消费状态标记，不承担状态判定职责。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep state-normalization governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_state_normalization_stays_in_primitives_not_component_surface() {
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");

    for needle in [
        "pub fn use_single_selection_state(",
        "pub fn use_multiple_selection_state(",
        "pub fn is_controlled(&self) -> bool",
        "pub fn default_selected_key(&self) -> &SelectedKey",
        "pub fn default_selected_keys(&self) -> &BTreeSet<Key>",
    ] {
        assert!(
            selection_source.contains(needle),
            "selection primitive should keep centralized normalization/state marker `{needle}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local normalization surface file: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_discrete_state_typing_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "已核验（N/A-组件无离散输入面）：`selection_indicator` 当前不导出 props/状态参数",
        "离散互斥状态由 `ui-state-primitives` 以类型化枚举建模",
        "`selection.rs` 的 `SelectedKey::{None, Key}`",
        "未出现 `Option<bool>` 组合表达单一状态机",
        "字符串输入需先映射到枚举后再进入原语逻辑",
        "互斥状态优先用 `enum` 建模，利用编译器封住无效组合。",
        "字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。",
        "布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep discrete-state typing governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_discrete_state_typing_is_enum_backed_in_primitives() {
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "pub enum SelectedKey",
        "None,",
        "Key(Key),",
        "pub struct SingleSelectionStateOptions",
        "pub selected_key: Option<SelectedKey>",
    ] {
        assert!(
            selection_source.contains(needle),
            "selection primitive should keep typed-discrete marker `{needle}`."
        );
    }

    assert!(
        !selection_source.contains("Option<bool>"),
        "selection primitive should avoid bool-explosion for discrete selection state."
    );

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep zero public discrete-input API surface marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local discrete-state implementation file: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_state_primitive_source_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力",
        "已核验（N/A-组件无接入面）：`selection_indicator` 当前无组件实现与业务 store 绑定入口",
        "状态能力仅来自 `ui-state-primitives`",
        "`lib.rs` 导出 `selection`",
        "`selection.rs` 提供受控/非受控选择原语",
        "组件层不存在可复用状态机二次实现",
        "组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。",
        "组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。",
        "`logic.rs` 仅做装配与映射，不重新实现状态原语。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep primitive-source governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_state_primitive_source_stays_in_ui_state_primitives() {
    let primitives_lib = load_source("../ui-state-primitives/src/lib.rs");
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");
    let lib_source = load_source("src/lib.rs");

    assert!(
        primitives_lib.contains("pub mod selection;"),
        "ui-state-primitives crate root should export selection primitive."
    );

    for needle in [
        "pub struct SingleSelectionState",
        "pub struct MultipleSelectionState",
        "pub fn use_single_selection_state(",
        "pub fn use_multiple_selection_state(",
    ] {
        assert!(
            selection_source.contains(needle),
            "selection primitive source should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep zero public module/export marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local reusable state-machine surface: `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_async_semantics_rules_with_na_reason() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "已核验（N/A-无异步交互面）：`selection_indicator` 当前无组件实现与事件链路，不涉及远程请求或异步状态",
        "不存在 `is_loading/aria-busy/retry` 语义分歧风险，也无组件私有加载/错误协议",
        "无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。",
        "有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。",
        "异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep async-semantics governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_has_no_async_interaction_surface_or_private_async_protocol() {
    let lib_source = load_source("src/lib.rs");
    let selection_source = load_source("../ui-state-primitives/src/selection.rs");

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep zero public async API surface marker `{forbidden}`."
        );
    }

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local async interaction file: `{forbidden}`."
        );
    }

    for forbidden in ["is_loading", "aria-busy", "retry", "use_async_action"] {
        assert!(
            !selection_source.contains(forbidden),
            "selection primitive should stay free of component-level async protocol marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_composite_api_explicitness_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "已核验（N/A-无独立组合 API 面）：`selection_indicator` 当前不导出独立组件",
        "语义由宿主组件 `list-item/menu-item` 显式承载",
        "\"selection-indicator\" => &[\"list-item\", \"menu-item\"]",
        "实现侧使用类型化枚举（`ListItemSelectionIndicator` / `MenuItemSelectionIndicator`）表达 indicator 语义",
        "不存在 `labels + children`、`titles + panels` 这类并行数组默认 API",
        "每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定",
        "`labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。",
        "不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep composite-api governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_semantics_are_hosted_by_typed_item_components_not_parallel_arrays() {
    let docs_component_mod = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let lib_source = load_source("src/lib.rs");

    assert!(
        docs_component_mod.contains("\"selection-indicator\" => &[\"list-item\", \"menu-item\"]"),
        "docs-app should keep selection-indicator mapped to explicit host components."
    );

    for needle in [
        "pub use item::ListItemSelectionIndicator;",
        "pub enum ListItemSelectionIndicator",
        "pub fn resolve_selection_indicator(",
    ] {
        assert!(
            list_logic.contains(needle),
            "list host component should keep typed selection-indicator marker `{needle}`."
        );
    }

    for needle in [
        "pub enum MenuItemSelectionIndicator",
        "pub fn resolve_selection_indicator(kind: MenuItemKind) -> MenuItemSelectionIndicator",
        "MenuItemKind::Checkbox { .. } => MenuItemSelectionIndicator::Checkbox",
        "MenuItemKind::Radio { .. } => MenuItemSelectionIndicator::Radio",
    ] {
        assert!(
            menu_item_logic.contains(needle),
            "menu-item host component should keep typed selection-indicator marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should not expose standalone composite API marker `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels"] {
        assert!(
            !list_logic.contains(forbidden),
            "list host component should avoid parallel-array contract marker `{forbidden}`."
        );
        assert!(
            !menu_item_logic.contains(forbidden),
            "menu-item host component should avoid parallel-array contract marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_a11y_i18n_l10n_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "已核验（N/A-无独立渲染入口）：`selection_indicator` 当前不导出独立组件且无 `view.rs`",
        "A11y/locale 契约由 `crates/ui-headless/src/a11y.rs` 统一提供",
        "`A11yDirection`/`locale_attrs`/`aria_controls_when_open`",
        "i18n 注入由 `crates/ui-components/src/root.rs` 通过 `UiRoot` 的 `i18n: UiI18n` + `provide_ui_i18n(i18n)` 完成",
        "实际 `selection_indicator` 语义由宿主 `list-item/menu-item` 挂载 `role/aria-*` 并经 `normalize_aria_label` 走“外部输入优先、组件兜底”路径",
        "交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。",
        "用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。",
        "组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。",
        "共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep a11y/i18n governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_a11y_i18n_contract_is_headless_backed_and_root_injected() {
    let lib_source = load_source("src/lib.rs");
    let root_source = load_source("src/root.rs");
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");
    let headless_i18n = load_source("../ui-headless/src/i18n/mod.rs");

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local a11y/i18n rendering surface file: `{forbidden}`."
        );
    }

    for needle in [
        "pub mod a11y;",
        "pub use a11y::{",
        "A11yDirection",
        "aria_controls_when_open",
        "locale_attrs",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless crate root should expose shared a11y contract marker `{needle}`."
        );
    }

    for needle in [
        "pub enum A11yDirection",
        "pub struct A11yLocaleAttrs",
        "pub fn locale_attrs(",
        "pub fn aria_controls_when_open(",
        "pub fn disclosure_trigger_attrs(",
        "pub fn popup_trigger_attrs(",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "ui-headless a11y module should keep typed a11y/i18n marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiI18n",
        "pub fn provide_ui_i18n(i18n: UiI18n)",
        "pub fn use_ui_i18n() -> UiI18n",
    ] {
        assert!(
            headless_i18n.contains(needle),
            "ui-headless i18n module should keep injection contract marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "#[prop(optional)] i18n: UiI18n,",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep i18n injection marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-direction\")]",
        "pub mod direction;",
        "pub use direction::{DirectionMode, DirectionProvider};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components public surface should keep lang/dir integration marker `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub(crate) const DEFAULT_ARIA_LABEL: &str = \"Listbox item\"",
        "pub struct ListAccessibleName",
        "pub aria_label: Option<String>",
        "pub aria_labelledby: Option<String>",
    ] {
        assert!(
            list_logic.contains(needle),
            "list host component should keep role/aria label normalization marker `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Menu item\"",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "role_attr: input.kind.role(),",
        "pub fn resolve_aria_checked(kind: MenuItemKind) -> Option<&'static str>",
    ] {
        assert!(
            menu_item_logic.contains(needle),
            "menu-item host component should keep role/aria contract marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_observable_semantic_marker_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "已核验（N/A-无独立状态面）：`selection_indicator` 不导出独立组件",
        "状态可观测性由宿主 `list-item/menu-item` 统一暴露",
        "`crates/ui-components/src/list/view.rs` 与 `crates/ui-components/src/menu/item/view.rs` 挂载稳定 `data-state/data-selected/data-focused/data-disabled/data-selection-indicator/data-aria-source/data-class-source` 与 `aria-*`",
        "对应 `logic.rs` 以类型化枚举和固定 attr 集合输出（如 `selection_indicator_attr/kind_attr/data_state_attr`）",
        "标记值为封闭集合且可枚举，不依赖 DOM 结构猜测",
        "稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。",
        "状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。",
        "自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。",
        "标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep observable-state governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_observable_markers_are_stable_and_closed_set_in_host_components() {
    let list_view = load_source("src/list/view.rs");
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-selected=selected.then_some(\"true\")",
        "aria-disabled=disabled.then_some(\"true\")",
        "role=\"option\"",
    ] {
        assert!(
            list_view.contains(needle),
            "list host view should keep stable semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selection-indicator=selection_indicator.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-checked=move || logic::resolve_aria_checked(kind)",
        "aria-disabled=disabled.then_some(\"true\")",
        "role=move || state.get().role_attr",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item host view should keep stable semantic marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ListItemSelectionIndicator",
        "pub fn as_attr(self) -> &'static str",
        "ListItemSelectionIndicator::Hidden => \"hidden\"",
        "ListItemSelectionIndicator::Checkmark => \"checkmark\"",
        "pub data_state_attr: &'static str,",
        "pub selection_indicator_attr: &'static str,",
        "pub aria_source_attr: &'static str,",
        "pub class_source_attr: &'static str,",
        "\"disabled-selected\"",
        "\"disabled\"",
        "\"focused-selected\"",
        "\"focused\"",
        "\"selected\"",
        "\"idle\"",
    ] {
        assert!(
            list_logic.contains(needle),
            "list host logic should keep closed semantic state/source set marker `{needle}`."
        );
    }

    for needle in [
        "pub enum MenuItemSelectionIndicator",
        "pub fn as_attr(self) -> &'static str",
        "MenuItemSelectionIndicator::Hidden => \"hidden\"",
        "MenuItemSelectionIndicator::Checkbox => \"checkbox\"",
        "MenuItemSelectionIndicator::Radio => \"radio\"",
        "pub fn resolve_kind_attr(kind: MenuItemKind) -> &'static str",
        "pub fn resolve_state(input: MenuItemStateInput) -> MenuItemState",
        "\"disabled\"",
        "\"focused-checked\"",
        "\"focused\"",
        "\"checked\"",
        "\"idle\"",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            menu_item_logic.contains(needle),
            "menu-item host logic should keep closed semantic state/source set marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_style_state_selector_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "已核验（N/A-无独立样式面）：`selection_indicator` 无独立 `styles.rs`",
        "样式契约由宿主 `crates/ui-components/src/list/styles.rs` 与 `crates/ui-components/src/menu/item/styles.rs` 承担",
        "状态分支选择器使用稳定 class + `data-*`",
        "`[data-selected]`/`[data-focused]`/`[data-disabled]`/`[data-show-selection-indicator]`/`[data-kind]`",
        "未使用 `:nth-child` 等结构猜测选择器",
        "运行时在宿主 `view.rs` 仅挂载语义 `data-*`/`aria-*`，未内联业务样式逻辑（无 `style=` 注入）",
        "视觉切换由语义标记直接驱动",
        "`styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。",
        "运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。",
        "视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep style-state selector governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let list_styles = load_source("src/list/styles.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_styles = load_source("src/menu/item/styles.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        ".ui-listbox-item--selected,",
        ".ui-listbox-item[data-selected=\"true\"]",
        ".ui-listbox-item--focused,",
        ".ui-listbox-item[data-focused=\"true\"]",
        ".ui-listbox-item--disabled,",
        ".ui-listbox-item[data-disabled=\"true\"]",
        ".ui-listbox-item--selection-indicator,",
        ".ui-listbox-item[data-show-selection-indicator=\"true\"]",
        ".ui-listbox-item--divider,",
        ".ui-listbox-item[data-has-divider=\"true\"]",
    ] {
        assert!(
            list_styles.contains(needle),
            "list styles should keep explicit state selector marker `{needle}`."
        );
    }

    for needle in [
        ".ui-menu-item--kind-action,",
        ".ui-menu-item[data-kind=\"action\"]",
        ".ui-menu-item--kind-checkbox,",
        ".ui-menu-item[data-kind=\"checkbox\"]",
        ".ui-menu-item--kind-radio,",
        ".ui-menu-item[data-kind=\"radio\"]",
        ".ui-menu-item--checked,",
        ".ui-menu-item[data-checked=\"true\"]",
        ".ui-menu-item--focused,",
        ".ui-menu-item[data-focused=\"true\"]",
        ".ui-menu-item--disabled,",
        ".ui-menu-item[data-disabled=\"true\"]",
    ] {
        assert!(
            menu_item_styles.contains(needle),
            "menu-item styles should keep explicit state selector marker `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !list_styles.contains(forbidden),
            "list styles should avoid DOM-guessing/inline-style marker `{forbidden}`."
        );
        assert!(
            !menu_item_styles.contains(forbidden),
            "menu-item styles should avoid DOM-guessing/inline-style marker `{forbidden}`."
        );
        assert!(
            !list_view.contains(forbidden),
            "list view should avoid inline-style/DOM-guessing marker `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item view should avoid inline-style/DOM-guessing marker `{forbidden}`."
        );
    }

    for needle in [
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-show-selection-indicator=move || {",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
    ] {
        assert!(
            list_view.contains(needle),
            "list view should expose explicit style-driving semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-kind=move || state.get().kind_attr",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selection-indicator=selection_indicator.as_attr()",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item view should expose explicit style-driving semantic marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_semantic_contract_testing_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "已核验（N/A-无独立测试面）：`selection_indicator` 无独立组件实现",
        "回归测试集中在 `crates/ui-components/tests/selection_indicator_module_semantics.rs`",
        "宿主 `list/menu item` 与 `ui-state-primitives::selection` 的语义契约作为证据源",
        "覆盖 `role/aria/data-state/source`、受控/非受控",
        "disabled、键盘/指针路径（`use_listbox + on:keydown + on:pointermove/on:click`）等关键分支",
        "`SSR/wasm` 差异在该模块按适用范围为 N/A（无 `web-sys`/无平台分支代码）",
        "测试策略以语义断言为主，未以视觉快照作为合并依据",
        "至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。",
        "测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。",
        "视觉快照只能作为补充，不得替代语义契约断言。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep semantic-testing governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_semantic_contract_tests_cover_matrix_without_snapshot_dependency() {
    let selection_module_test_source = load_source("tests/selection_indicator_module_semantics.rs");
    let menu_item_semantics_test_source = load_source("tests/menu_item_semantics.rs");
    let list_module_semantics_test_source = load_source("tests/list_module_semantics.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");
    let selection_primitive = load_source("../ui-state-primitives/src/selection.rs");

    for needle in [
        "fn selection_indicator_observable_markers_are_stable_and_closed_set_in_host_components()",
        "fn selection_indicator_a11y_i18n_contract_is_headless_backed_and_root_injected()",
        "fn selection_indicator_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn selection_indicator_check2_includes_observable_semantic_marker_rules()",
    ] {
        assert!(
            selection_module_test_source.contains(needle),
            "selection_indicator module semantics test should keep semantic-contract coverage marker `{needle}`."
        );
    }

    for needle in [
        "single_controlled_calls_on_change_but_does_not_update_internal",
        "multiple_controlled_calls_on_change_but_does_not_update_internal",
        "pub fn use_single_selection_state(",
        "pub fn use_multiple_selection_state(",
    ] {
        assert!(
            selection_primitive.contains(needle),
            "selection primitive should keep controlled/uncontrolled regression marker `{needle}`."
        );
    }

    for needle in [
        "use_listbox(ListBoxOptions {",
        "let on_key_down = move |ev: leptos::ev::KeyboardEvent| {",
        "if aria.handlers.on_key_down.run(ev.key()) {",
        "on:keydown=on_key_down",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
    ] {
        assert!(
            list_view.contains(needle),
            "list host view should keep keyboard/pointer contract marker `{needle}`."
        );
    }

    for needle in [
        "on:pointermove=move |_| {",
        "on:click=move |_| {",
        "if disabled {",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-disabled=disabled.then_some(\"true\")",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item host view should keep disabled/pointer semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "role=move || state.get().role_attr",
        "aria-checked=move || logic::resolve_aria_checked(kind)",
    ] {
        assert!(
            menu_item_semantics_test_source.contains(needle),
            "menu_item semantics tests should keep role/aria/data/source marker `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "#[cfg(target_arch = \"wasm32\")]",
    ] {
        assert!(
            !list_view.contains(forbidden),
            "list host view should keep platform-neutral semantic surface; found `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item host view should keep platform-neutral semantic surface; found `{forbidden}`."
        );
    }

    for forbidden in [
        "assert_snapshot!",
        "insta::assert_snapshot!",
        "to_match_snapshot",
    ] {
        assert!(
            !menu_item_semantics_test_source.contains(forbidden),
            "menu-item contract tests should not depend on visual snapshot marker `{forbidden}`."
        );
        assert!(
            !list_module_semantics_test_source.contains(forbidden),
            "list-module contract tests should not depend on visual snapshot marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_component_file_responsibility_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "已核验（N/A-无独立组件文件面）：`selection_indicator` 当前目录仅保留治理清单 `check2.md`",
        "不存在 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs`，因此不存在职责错位风险",
        "未在 `crates/ui-components/src/lib.rs` 导出 `selection_indicator` 模块",
        "语义职责由宿主 `list/menu item` 组件按既有分层文件承担",
        "`selection_indicator` 本身仅维护契约治理",
        "`mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。",
        "`logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。",
        "`styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。",
        "`view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。",
        "`motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep component-file responsibility marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_has_no_local_component_file_surface_and_no_public_export() {
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/motion.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local component responsibility file: `{forbidden}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep no public export surface marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_spec_file_scarcity_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "已核验（N/A-无独立 Schema 面）：`selection_indicator` 当前无独立组件实现与外部 Schema 契约需求",
        "目录下不存在 `spec.rs`",
        "`crates/ui-components/src/button/spec.rs` 作为少数正例承载",
        "`selection_indicator` 说明与治理保留在 `check2.md`",
        "仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。",
        "简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。",
        "新增 `spec.rs` 必须同步给出契约测试与版本演进说明。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep spec-scarcity governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_does_not_add_spec_file_and_button_remains_complex_schema_example() {
    let button_mod = load_source("src/button/mod.rs");
    let button_spec = load_source("src/button/spec.rs");
    let button_semantics = load_source("tests/button_semantics.rs");
    let lib_source = load_source("src/lib.rs");

    assert!(
        !path_exists("src/selection_indicator/spec.rs"),
        "selection_indicator should not introduce spec.rs without complex external schema need."
    );
    assert!(
        path_exists("src/button/spec.rs"),
        "button/spec.rs should remain present as the complex-schema positive example."
    );

    for needle in [
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
        "pub struct ButtonSchema {",
        "pub schema_version: u16,",
        "pub fn schema_version(mut self, value: u16) -> Self",
        "pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>",
        "pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>",
    ] {
        assert!(
            button_mod.contains(needle) || button_spec.contains(needle),
            "button should keep versioned schema/spec contract marker `{needle}`."
        );
    }

    for needle in [
        "vec![\"button/spec.rs\".to_string()]",
        "spec.rs should stay limited to complex components; simple components should not add spec.rs by default.",
        "spec.rs should stay scarce; only button/spec.rs is allowed in ui-components/src.",
    ] {
        assert!(
            button_semantics.contains(needle),
            "button semantics should keep spec-scarcity regression marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::spec",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should not expose spec/module export marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_token_first_static_style_contract_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "已核验（N/A-无独立样式实现）：`selection_indicator` 目录无 `styles.rs`",
        "`styles.rs` 提供，并通过 `crates/ui-components/src/css.rs::push_components_css` 聚合",
        "在 `crates/ui-components/src/root.rs` 由 `UiRoot` 注入",
        "宿主样式视觉值使用 `var(--ui-*)`",
        "运行时 `view.rs` 未注入业务 `style=`",
        "未引入 Utility-First 或 CSS-in-Rust 作为组件库默认范式",
        "样式规则统一落在 `styles.rs`，由 `crates/ui-components/src/css.rs` 聚合并通过 `UiRoot` 注入。",
        "颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。",
        "Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。",
        "CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep token-first static-style governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_token_first_styles_are_aggregated_and_theme_variable_driven() {
    let css_aggregate = load_source("src/css.rs");
    let ui_root = load_source("src/root.rs");
    let list_styles = load_source("src/list/styles.rs");
    let menu_item_styles = load_source("src/menu/item/styles.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
    ] {
        assert!(
            css_aggregate.contains(needle),
            "component css aggregation should keep token-first marker `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should keep component-css injection marker `{needle}`."
        );
    }

    for needle in [
        "var(--ui-space-sm)",
        "var(--ui-border)",
        "var(--ui-radius-lg)",
        "var(--ui-bg)",
        "var(--ui-shadow-sm)",
        "var(--ui-focus-ring)",
        "var(--ui-accent)",
    ] {
        assert!(
            list_styles.contains(needle) || menu_item_styles.contains(needle),
            "host styles should keep theme-token variable marker `{needle}`."
        );
    }

    for forbidden in ["@apply", "tailwind", "tw-", "css!(", "stylex", "styled("] {
        assert!(
            !list_styles.contains(forbidden),
            "list styles should avoid utility-first/CSS-in-Rust default marker `{forbidden}`."
        );
        assert!(
            !menu_item_styles.contains(forbidden),
            "menu-item styles should avoid utility-first/CSS-in-Rust default marker `{forbidden}`."
        );
    }

    let forbidden = "style=";
    assert!(
        !list_view.contains(forbidden),
        "list view should not inject business inline style marker `{forbidden}`."
    );
    assert!(
        !menu_item_view.contains(forbidden),
        "menu-item view should not inject business inline style marker `{forbidden}`."
    );
}

#[test]
fn selection_indicator_check2_includes_visual_desire_theme_quality_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "已核验（N/A-无独立视觉主题面）：`selection_indicator` 无独立主题/样式实现",
        "docs-app `theme-visual-baseline` 页面（`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs`）",
        "`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 锁定 page/button/input/overlay 四张截图基线",
        "`docs/spec/heroui-parameter-design-strategy.md` 约束为“体验质量对齐而非 API 表层复制”",
        "默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。",
        "docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。",
        "禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。",
        "HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep visual-desire governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_visual_desire_reuses_theme_baseline_and_avoids_bootstrap_regression() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let list_styles = load_source("src/list/styles.rs");
    let menu_item_styles = load_source("src/menu/item/styles.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "theme visual baseline page should include `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e.contains(needle),
            "theme visual baseline e2e should include snapshot marker `{needle}`."
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "HeroUI strategy should include quality-alignment marker `{needle}`."
        );
    }

    for forbidden in [
        "Bootstrap",
        "btn-default",
        "panel-default",
        "form-control",
        "well well-",
    ] {
        assert!(
            !list_styles.contains(forbidden),
            "list host styles should avoid visual-regression marker `{forbidden}`."
        );
        assert!(
            !menu_item_styles.contains(forbidden),
            "menu-item host styles should avoid visual-regression marker `{forbidden}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "SelectionIndicatorTheme",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should not expose standalone theme surface marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_dx_paradox_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "已核验（N/A-无独立公共 API 面）：`selection_indicator` 未在 `crates/ui-components/src/lib.rs` 导出独立组件",
        "\"selection-indicator\" => &[\"list-item\", \"menu-item\"]",
        "不手动接线 `ui-state-primitives/ui-headless`、不要求 `state=...` 必填参数",
        "基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。",
        "简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。",
        "docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep DX paradox governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_dx_paradox_uses_host_defaults_without_required_state_wiring() {
    let lib_source = load_source("src/lib.rs");
    let component_mod = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    let collections_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use selection_indicator::",
        "selection_indicator::SelectionIndicator",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "selection_indicator should keep no standalone export marker `{forbidden}`."
        );
    }

    assert!(
        component_mod.contains("\"selection-indicator\" => &[\"list-item\", \"menu-item\"]"),
        "docs component mapping should route selection-indicator to list-item/menu-item host pages."
    );

    for needle in [
        "pub(super) fn list_item() -> AnyView",
        "slug=\"list-item\"",
        "<Playground title=\"Selectable Option\" code_signal=code>",
        "show_selection_indicator=true",
        "pub(super) fn menu_item() -> AnyView",
        "slug=\"menu-item\"",
        "<Playground title=\"Action + Checkbox\" code_signal=code>",
        "kind=MenuItemKind::Action",
    ] {
        assert!(
            collections_extra.contains(needle),
            "selection-indicator host docs should keep simple default-path marker `{needle}`."
        );
    }

    assert!(
        !collections_extra.contains("\n  state="),
        "selection-indicator host docs should not require explicit state object wiring in baseline usage."
    );
}

#[test]
fn selection_indicator_check2_includes_tree_shaking_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "已核验：`ui-components` 保持组件级 feature 边界",
        "Tree-shaking 证据命令已执行：`/root/.cargo/bin/cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-accordion,inject-css`",
        "仅出现命令行特性 `component-accordion` 与 `inject-css`",
        "`/root/.cargo/bin/cargo tree -e features -i ui-components -p web-demo` 出现 `web-demo-components` 且无 `all-components`",
        "CI 预算/阻断由 `scripts/check-ui-components-tree-shaking.sh` + `scripts/tree_shaking_budget.env`",
        "验证命令（反向依赖）：`cargo tree -e features -i ui-components -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep tree-shaking governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_tree_shaking_boundaries_and_budget_guards_exist() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-list = [\"component-active_highlight\", \"component-illustrated_message\"]",
        "component-menu_item = [\"component-menu\"]",
        "#[cfg(feature = \"component-list\")]",
    ] {
        assert!(
            cargo_source.contains(needle) || lib_source.contains(needle),
            "tree-shaking boundary should keep component feature marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "component css aggregation should keep feature-sliced marker `{needle}`."
        );
    }

    assert!(
        web_demo_cargo.contains(
            "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ),
        "web-demo should keep source-like dependency path with default-features disabled + web-demo-components."
    );
    assert!(
        docs_app_cargo.contains(
            "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"all-components\"] }"
        ),
        "docs-app should keep all-components as full acceptance surface."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking script/budget should keep gating marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_machine_readable_type_marker_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "已核验（N/A-无独立输入面）：`selection_indicator` 无独立 props/API",
        "`crates/ui-state-primitives/src/selection.rs` 的 `SelectedKey`",
        "`crates/ui-components/src/list/logic.rs` 的 `ListItemSelectionIndicator`",
        "`crates/ui-components/src/menu/item/logic.rs` 的 `MenuItemSelectionIndicator`",
        "`data-state/data-selection-indicator/data-aria-source/data-class-source/aria-*`",
        "并由 `crates/ui-components/tests/selection_indicator_module_semantics.rs` 持续断言。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep machine-readable contract marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_machine_readable_contract_is_typed_and_semantic() {
    let primitive_selection = load_source("../ui-state-primitives/src/selection.rs");
    let list_logic = load_source("src/list/logic.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "pub enum SelectedKey",
        "pub enum ListItemSelectionIndicator",
        "pub enum MenuItemSelectionIndicator",
        "data-state=move || state.get().data_state_attr",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-selection-indicator=selection_indicator.as_attr()",
    ] {
        assert!(
            primitive_selection.contains(needle)
                || list_logic.contains(needle)
                || list_view.contains(needle)
                || menu_item_logic.contains(needle)
                || menu_item_view.contains(needle),
            "typed/semantic machine-readable marker `{needle}` should remain present."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_platform_and_motion_branch_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "已执行 `/root/.cargo/bin/cargo check -p ui-components`（默认 native）",
        "`/root/.cargo/bin/cargo check -p ui-headless --no-default-features --features ssr`（ssr native）",
        "`/root/.cargo/bin/cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`（web wasm）并通过",
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "web,ssr` 已实测失败并命中 “mutually exclusive” 错误（退出码 `101`）",
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "`/root/.cargo/bin/cargo test -p ui-motion --test non_wasm_stub` 均通过",
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "`crates/ui-components/src/active_highlight.rs` 显式 `#[cfg(target_arch = \"wasm32\")] / #[cfg(not(target_arch = \"wasm32\"))]`",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep platform/motion governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_platform_and_motion_branch_guards_are_centralized() {
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_cargo = load_source("../ui-headless/Cargo.toml");
    let motion_lib = load_source("../ui-motion/src/lib.rs");
    let motion_spring = load_source("../ui-motion/src/spring.rs");
    let motion_stub_test = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let active_highlight = load_source("src/active_highlight.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "mutually exclusive",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform script should keep compile-only/mutex/noop marker `{needle}`."
        );
    }

    assert!(
        headless_lib.contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless must keep compile_error mutex guard for web/ssr."
    );
    assert!(
        headless_cargo.contains("default = [\"web\"]")
            && headless_cargo.contains("web = [\"leptos/csr\"]")
            && headless_cargo.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless Cargo features should keep explicit web/ssr split."
    );

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "if crate::web::prefers_reduced_motion()",
        "non_wasm_web_backend_animate_is_safe_noop",
        "non_wasm_web_backend_prefers_reduced_motion",
    ] {
        assert!(
            motion_lib.contains(needle)
                || motion_spring.contains(needle)
                || motion_stub_test.contains(needle),
            "ui-motion should keep non-wasm/reduced-motion marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active-highlight motion should keep wasm/non-wasm split marker `{needle}`."
        );
    }

    assert!(
        !path_exists("src/selection_indicator/motion.rs"),
        "selection_indicator should keep no local motion.rs; branch handling stays in shared host/motion layers."
    );

    for forbidden in ["web_sys::", "wasm_bindgen::"] {
        assert!(
            !list_view.contains(forbidden),
            "list view non-wasm path should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item view non-wasm path should avoid browser-only marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_performance_governance_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "已核验（共享性能门禁 + N/A-无独立渲染面）：性能预算与阻断由公共链路承担",
        "`component_page_perf_budget` + `UiPerfProbe` 提供 `data-perf-*` 指标",
        "`e2e/tests/docs_app_components_coverage.spec.mjs` 持续断言预算属性与无 `data-perf-violation=true`",
        "`scripts/check-ui-components-performance.sh` 覆盖组件性能契约并包含 `perf_render_count_follow_up_is_tracked_in_plan` 跟踪项",
        "已执行 `/root/.cargo/bin/cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages`",
        "`docs/plan/TODO.md` 保留 `render_count` 自动化补齐任务。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep performance-governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_shared_perf_budget_and_render_count_follow_up_are_guarded() {
    let docs_shell = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_e2e = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_script = load_source("../../scripts/check-ui-components-performance.sh");
    let plan_todo = load_source("../../docs/plan/TODO.md");

    for needle in [
        "use ui_headless::{UiPerfBudget, UiPerfProbe};",
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell.contains(needle),
            "docs shell should keep perf budget probe marker `{needle}`."
        );
    }

    for needle in [
        "[data-slot=\"ui-perf-probe\"]",
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
    ] {
        assert!(
            coverage_e2e.contains(needle),
            "components coverage e2e should keep performance assertion marker `{needle}`."
        );
    }

    for needle in [
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(needle),
            "performance script should keep blocking/follow-up marker `{needle}`."
        );
    }

    for needle in [
        "profiling workbench",
        "建立 `render_count` 自动化回归（Button/Input/Accordion）",
    ] {
        assert!(
            plan_todo.contains(needle),
            "engineering TODO should keep explicit perf follow-up marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_view_macro_complexity_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "已核验（N/A-无独立 `selection_indicator/view.rs`）：`selection_indicator` 当前不导出独立组件视图层",
        "`crates/ui-components/src/list/view.rs` 已按 `List` / `ListItem` / `ListSection` 分块",
        "`crates/ui-components/src/menu/item/view.rs` 保持独立 `MenuItem` 组件",
        "`selection_indicator` 仅以稳定语义标记与插槽挂载（如 `data-selection-indicator`），未引入巨型单块 `view!`",
        "复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。",
        "`view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。",
        "编译时间/产物体积异常增长时，优先排查宏展开体量。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep view-macro complexity marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_view_macro_surface_is_host_split_not_local_monolith() {
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    assert!(
        !path_exists("src/selection_indicator/view.rs"),
        "selection_indicator should keep no local view.rs surface."
    );

    for needle in [
        "#[component]\npub fn List(",
        "#[component]\npub fn ListItem(",
        "#[component]\npub fn ListSection(",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
    ] {
        assert!(
            list_view.contains(needle),
            "list host view should keep semantic split marker `{needle}`."
        );
    }

    for needle in [
        "#[component]\npub fn MenuItem(",
        "data-selection-indicator=selection_indicator.as_attr()",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item host view should keep semantic split marker `{needle}`."
        );
    }

    let list_view_blocks = list_view.matches("view! {").count();
    let menu_item_view_blocks = menu_item_view.matches("view! {").count();
    assert!(
        (3..=8).contains(&list_view_blocks),
        "list host view should stay in bounded macro-block range; got `{list_view_blocks}`."
    );
    assert!(
        (1..=4).contains(&menu_item_view_blocks),
        "menu-item host view should stay in bounded macro-block range; got `{menu_item_view_blocks}`."
    );
}

#[test]
fn selection_indicator_check2_includes_functional_split_preference_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "已核验（N/A-无独立 `selection_indicator/view.rs`）：`selection_indicator` 当前无本地视图组件定义",
        "`crates/ui-components/src/list/view.rs` 仅保留 `List` / `ListItem` / `ListSection` 三个具备独立 props 语义的组件",
        "`crates/ui-components/src/menu/item/view.rs` 仅保留 `MenuItem` 主组件",
        "未把局部片段继续升格为额外 `#[component]`",
        "轻量片段通过局部闭包与 `Show` fallback 挂载，语义标记（含 `data-selection-indicator`）保持稳定",
        "纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。",
        "禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。",
        "拆分后语义标记与测试定位仍需稳定。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep functional-split marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_functional_split_keeps_host_component_surface_minimal() {
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    assert!(
        !path_exists("src/selection_indicator/view.rs"),
        "selection_indicator should keep no local view.rs surface."
    );

    let list_component_count = list_view.matches("#[component]").count();
    let menu_item_component_count = menu_item_view.matches("#[component]").count();
    assert_eq!(
        list_component_count, 3,
        "list host view should keep only List/ListItem/ListSection component boundaries."
    );
    assert_eq!(
        menu_item_component_count, 1,
        "menu-item host view should keep only MenuItem component boundary."
    );

    for needle in [
        "let indicator_text = move || selection_indicator.marker(state.get().is_selected);",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
    ] {
        assert!(
            list_view.contains(needle),
            "list host view should keep lightweight local fragment marker `{needle}`."
        );
    }

    for needle in [
        "let indicator_text = move || selection_indicator.marker(state.get().is_checked);",
        "fallback=move || view! {",
        "data-selection-indicator=selection_indicator.as_attr()",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item host view should keep lightweight local fragment marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_static_fragment_const_guidance_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "已核验（N/A-无独立 `selection_indicator/view.rs`）：`selection_indicator` 当前无本地静态片段模板面",
        "`crates/ui-components/src/list/view.rs` 与 `crates/ui-components/src/menu/item/view.rs` 不存在复杂 SVG、长说明文本或 `inner_html` 注入",
        "`\"selected\"` / `\"not selected\"`",
        "并集中在宿主视图固定插槽（`*-selection-sr`）内，变更路径单一可追踪",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep static-fragment guidance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_static_fragments_stay_small_accessible_and_without_inner_html() {
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    assert!(
        !path_exists("src/selection_indicator/view.rs"),
        "selection_indicator should keep no local static fragment view surface."
    );

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !list_view.contains(forbidden),
            "list host view should avoid complex static/unsafe html marker `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item host view should avoid complex static/unsafe html marker `{forbidden}`."
        );
    }

    for needle in [
        "data-slot=\"listbox-item-selection-sr\"",
        "{move || if state.get().is_selected { \"selected\" } else { \"not selected\" }}",
        "role=\"option\"",
        "aria-label=aria_label",
    ] {
        assert!(
            list_view.contains(needle),
            "list host view should keep small accessible static-fragment marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"menu-item-selection-sr\"",
        "{move || if state.get().is_checked { \"selected\" } else { \"not selected\" }}",
        "role=move || state.get().role_attr",
        "aria-label=aria_label",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item host view should keep small accessible static-fragment marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_inner_html_safety_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "已核验（N/A-无独立 `selection_indicator/view.rs` 且宿主无注入点）：`selection_indicator` 当前无本地 `inner_html` 使用",
        "`crates/ui-components/src/list/view.rs` 与 `crates/ui-components/src/menu/item/view.rs` 未出现 `inner_html`/`dangerously_set_inner_html`/`set_inner_html`",
        "不存在用户输入、远端返回或未清洗模板被注入 DOM 的路径",
        "语义回归由 `crates/ui-components/tests/selection_indicator_module_semantics.rs` 持续断言",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep inner_html safety marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_inner_html_injection_surface_remains_absent() {
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    assert!(
        !path_exists("src/selection_indicator/view.rs"),
        "selection_indicator should keep no local view.rs injection surface."
    );

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "set_inner_html",
        "html=",
        "<script",
    ] {
        assert!(
            !list_view.contains(forbidden),
            "list host view should avoid inner-html injection marker `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item host view should avoid inner-html injection marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_wasm_debug_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "已核验（N/A-无独立 `selection_indicator` 组件调试面）：调试能力复用全局链路而非组件私有实现",
        "`crates/ui-headless/src/trace.rs` 以 `UiTraceEvent { ts_ms, component, kind }` 记录时间戳与来源",
        "`crates/ui-headless/src/controllable_state.rs::use_controllable_open_state_traced` 输出 `UiTraceEventKind::OpenChange`",
        "`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` 并挂载 `<debug_overlay::UiDebugOverlay enabled=true />`",
        "`apps/docs-app/src/debug_overlay.rs` 提供 inspect snapshot + event timeline（含 `format!(\"{ts_ms}ms\")`）",
        "`selection_indicator` 宿主 `list/menu item` 通过 `data-*-source` + `on:keydown/on:pointermove/on:click` 暴露可追踪/可回放链路",
        "仅 `accordion-wasm-debug`/`button-wasm-debug`",
        "未引入 `selection_indicator` 私有 debug feature",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep wasm-debug governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let controllable_source = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let list_view_source = load_source("src/list/view.rs");
    let menu_item_view_source = load_source("src/menu/item/view.rs");
    let wasm_debug_script = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components should keep wasm-debug capability isolated with `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("selection_indicator-wasm-debug")
            && !cargo_source.contains("selection-indicator-wasm-debug")
            && !cargo_source.contains("component-selection_indicator-wasm-debug"),
        "selection_indicator should not expose a dedicated wasm-debug feature and must reuse global trace/debug overlay."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug")
            && !all_components_block.contains("accordion-wasm-debug"),
        "wasm debug features must not leak into all-components production path."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep debug visual-entry gated by debug_assertions via `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
        "ui_headless::UiTraceEventKind::Inspect",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "trace.emit(",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace snapshot/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "OpenChange {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source markers `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_source.contains(needle),
            "controllable-state bridge should keep traced open-change marker `{needle}`."
        );
    }

    let host_views = format!("{list_view_source}\n{menu_item_view_source}");
    for marker in [
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "on:keydown=on_key_down",
        "on:pointermove=move |_|",
        "on:click=move |_|",
    ] {
        assert!(
            host_views.contains(marker),
            "selection_indicator host views should keep replay/source marker `{marker}`."
        );
    }

    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !host_views.contains(forbidden),
            "selection_indicator host production path should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only",
    ] {
        assert!(
            wasm_debug_script.contains(needle),
            "wasm-debug check script should keep shared contract command `{needle}`."
        );
    }

    assert!(
        !wasm_debug_script.contains("selection_indicator_module_semantics"),
        "selection_indicator should reuse shared wasm-debug gate; script should not require a dedicated selection_indicator debug command."
    );
}

#[test]
fn selection_indicator_check2_includes_dx_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "已核验（N/A-无独立 `selection_indicator` workbench）：样式热重载与隔离画布复用 docs 通用 `Playground`",
        "scoped CSS 注入 `<style>{move || compose_scoped_css(..)}</style>`",
        "测试面板输入 `on:input=...set_test_css...`",
        "`\"Restore original CSS\"`",
        "`list-item/menu-item` 通过本地 `signal` + `on_press` 回调保持当前交互上下文",
        "`data-playground-scope` + `playground__preview-stage` + `playground-controls`",
        "可选状态保留在 `selection_indicator` 当前范围标记为 N/A",
        "沿用共享 `scripts/check-ui-components-dx.sh` 门禁",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep DX governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn list_item() -> AnyView",
        "pub(super) fn menu_item() -> AnyView",
        "<Playground title=\"Selectable Option\" code_signal=code>",
        "<Playground title=\"Focused + Divider + Disabled\" code_signal=states_code>",
        "<Playground title=\"Action + Checkbox\" code_signal=code>",
        "<Playground title=\"Radio + Submenu + Disabled\" code_signal=states_code>",
        "let (selected_default, set_selected_default) = signal(true);",
        "let (selected_states, set_selected_states) = signal(true);",
        "let (radio_selected, set_radio_selected) = signal(true);",
        "on_press=toggle_default",
        "on_press=toggle_states",
        "on_press=toggle_radio",
    ] {
        assert!(
            docs_source.contains(needle),
            "selection_indicator host docs should expose interactive context marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_dx_scope_keeps_isolated_canvas_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let dx_script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for forbidden in [
        "SELECTION_INDICATOR_WORKBENCH_STORAGE_KEY",
        "load_selection_indicator_workbench_state(",
        "save_selection_indicator_workbench_state(",
        "clear_selection_indicator_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "selection_indicator scope keeps optional persisted-state as N/A; `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "[dx] contract: playground css hot-reload path",
        "cargo test -p ui-components --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
    ] {
        assert!(
            dx_script_source.contains(needle),
            "DX script should keep shared contract gate marker `{needle}`."
        );
    }

    assert!(
        !dx_script_source.contains("selection_indicator_module_semantics"),
        "selection_indicator should reuse shared DX gate; script should not require dedicated selection_indicator DX command."
    );
}

#[test]
fn selection_indicator_check2_includes_engineering_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "已核验（N/A-无独立 `selection_indicator` spec/config 面）：`selection_indicator` 当前不导出组件模块",
        "不存在 `spec.rs` 与组件级 serde 序列化/版本迁移实现",
        "`crates/ui-components/src/button/spec.rs`",
        "`scripts/check-ui-components-engineering.sh` 中 `button_engineering_contract_uses_serde_schema_and_structured_migration_errors`",
        "tracing 语义沿用全库统一基线",
        "`target: \"ui_components::button::state_change\"`",
        "`selection_indicator` 宿主 `list/menu item` 无组件私有 tracing 词汇漂移",
        "无 `tokio/async-std/runtime::Handle` 泄露",
        "复用共享 engineering 门禁脚本",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep engineering governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_engineering_contract_marks_spec_serde_path_as_na_for_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let list_mod = load_source("src/list/mod.rs");
    let list_logic = load_source("src/list/logic.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_mod = load_source("src/menu/item/mod.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    assert!(
        !manifest_dir
            .join("src/selection_indicator/spec.rs")
            .exists(),
        "selection_indicator should keep spec/schema boundary as N/A for current component scope."
    );
    assert!(
        !cargo_source.contains("component-selection_indicator"),
        "selection_indicator should not create standalone feature fan-out when implemented through list/menu-item hosts."
    );
    assert!(
        !crate_root_source.contains("pub mod selection_indicator;"),
        "crate root should stay free from a legacy selection_indicator export."
    );

    let host_combined = format!(
        "{list_mod}\n{list_logic}\n{list_view}\n{menu_item_mod}\n{menu_item_logic}\n{menu_item_view}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !host_combined.contains(forbidden),
            "selection_indicator host contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let headless_trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let host_combined = [
        load_source("src/list/mod.rs"),
        load_source("src/list/logic.rs"),
        load_source("src/list/view.rs"),
        load_source("src/menu/item/mod.rs"),
        load_source("src/menu/item/logic.rs"),
        load_source("src/menu/item/view.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
        "pub enum UiTraceEventKind {",
    ] {
        assert!(
            cargo_source.contains(required)
                || button_view_source.contains(required)
                || headless_trace_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("selection_indicator-wasm-debug")
            && !cargo_source.contains("selection-indicator-wasm-debug")
            && !cargo_source.contains("component-selection_indicator-wasm-debug"),
        "selection_indicator should not define component-local tracing/debug feature."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::selection_indicator::",
        "const SELECTION_INDICATOR_TRACE_TARGET",
    ] {
        assert!(
            !host_combined.contains(forbidden),
            "selection_indicator hosts should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let sources = [
        load_source("src/lib.rs"),
        load_source("src/list/mod.rs"),
        load_source("src/list/logic.rs"),
        load_source("src/list/view.rs"),
        load_source("src/list/styles.rs"),
        load_source("src/menu/item/mod.rs"),
        load_source("src/menu/item/logic.rs"),
        load_source("src/menu/item/view.rs"),
        load_source("src/menu/item/styles.rs"),
    ];

    for source in &sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
            "JoinHandle",
        ] {
            assert!(
                !source.contains(forbidden),
                "selection_indicator host contract should not leak runtime marker `{forbidden}`."
            );
        }
    }
}

#[test]
fn selection_indicator_engineering_check_script_covers_shared_contract_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api",
        "cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should keep shared contract gate marker `{needle}`."
        );
    }

    assert!(
        !script_source.contains("selection_indicator_module_semantics"),
        "selection_indicator should reuse shared engineering gate; script should not require dedicated selection_indicator engineering command."
    );
}

#[test]
fn selection_indicator_check2_includes_ui_components_entrypoint_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "已核验（N/A-无独立 `selection_indicator` 入口文件面）",
        "`crates/ui-components/src/lib.rs` 保持总入口与 feature gate 导出边界",
        "`#[cfg(feature = \"component-list\")]` / `#[cfg(feature = \"component-menu_item\")]`",
        "`lib.rs` 通过 `#[cfg(feature = \"inject-css\")] pub fn push_components_css(...)`",
        "`crates/ui-components/src/css.rs` 以 `push_components_css` 聚合组件样式并按 `component-*` 条件注入",
        "`crates/ui-components/src/root.rs` 的 `UiRoot` 集中注入 base css + theme vars + 可选 component css",
        "`provide_ui_i18n` 提供全局 i18n 上下文",
        "`crates/ui-components/src/active_highlight.rs` 仅承载共享高亮样式与 motion driver",
        "`crates/ui-components/src/overlay_open.rs` / `presence.rs` / `a11y.rs` 在组件层不存在",
        "`crates/ui-headless/src/{controllable_state,presence,a11y}.rs`",
        "`scripts/check-ui-components-entrypoints.sh` 共享入口契约检查",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-components/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep ui-components entrypoint governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("src/active_highlight.rs");
    let headless_controllable = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../ui-headless/src/presence.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "#[cfg(feature = \"component-list\")]",
        "pub mod list;",
        "#[cfg(feature = \"component-menu\")]",
        "pub mod menu;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable export/gate marker `{needle}`."
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
            "ui-components lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css entry should keep feature-gated aggregation marker `{needle}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight should keep shared motion primitive marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_components::list",
        "ui_components::menu",
        "MenuItem",
        "business",
        "role=",
        "aria-",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay component-agnostic and semantics-free; found `{forbidden}`."
        );
    }

    for forbidden_path in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden_path),
            "ui-components forbidden entrypoint file should remain absent: `{forbidden_path}`."
        );
    }

    for headless_path in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(headless_path),
            "headless canonical entrypoint should exist: `{headless_path}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub struct ControllableOpenState",
    ] {
        assert!(
            headless_controllable.contains(needle),
            "headless controllable-state canonical primitive should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub struct Presence",
    ] {
        assert!(
            headless_presence.contains(needle),
            "headless presence canonical primitive should keep marker `{needle}`."
        );
    }

    for needle in ["pub enum A11yDirection", "pub fn aria_controls_when_open("] {
        assert!(
            headless_a11y.contains(needle),
            "headless a11y canonical primitive should keep marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_entrypoint_check_script_covers_shared_contract_gate() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "cargo test -p ui-components --test button_semantics ui_components_css_registry_remains_feature_gated_and_non_global",
        "cargo test -p ui-components --test button_semantics ui_root_centralizes_theme_injection_and_i18n_context",
        "cargo test -p ui-components --test button_semantics active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "cargo test -p ui-components --test button_semantics ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoint check script should keep shared contract gate marker `{needle}`."
        );
    }

    assert!(
        !script_source.contains("selection_indicator_module_semantics"),
        "selection_indicator should reuse shared entrypoint gate; script should not require dedicated selection_indicator entrypoint command."
    );
}

#[test]
fn selection_indicator_check2_includes_component_directory_standard_file_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "已核验（N/A-无独立 `selection_indicator` 组件目录面）",
        "`crates/ui-components/src/selection_indicator/` 当前仅保留治理清单 `check2.md`",
        "不存在 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs/spec.rs`",
        "不存在目录职责错位与 `render.rs` 漂移风险",
        "`crates/ui-components/src/list/{mod,logic,styles,view,motion}.rs`",
        "`crates/ui-components/src/menu/item/{mod,logic,styles,view}.rs`",
        "`scripts/check-ui-components-component-files.sh` 门禁",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep component-directory governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_component_directory_standard_files_are_na_and_hosted_without_render_drift() {
    for forbidden in [
        "src/selection_indicator/mod.rs",
        "src/selection_indicator/logic.rs",
        "src/selection_indicator/styles.rs",
        "src/selection_indicator/view.rs",
        "src/selection_indicator/motion.rs",
        "src/selection_indicator/spec.rs",
        "src/selection_indicator/render.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator should keep no local component-directory standard file `{forbidden}`."
        );
    }

    for required in [
        "src/list/mod.rs",
        "src/list/logic.rs",
        "src/list/styles.rs",
        "src/list/view.rs",
        "src/list/motion.rs",
        "src/menu/item/mod.rs",
        "src/menu/item/logic.rs",
        "src/menu/item/styles.rs",
        "src/menu/item/view.rs",
    ] {
        assert!(
            path_exists(required),
            "selection_indicator host component file should exist: `{required}`."
        );
    }

    for forbidden in ["src/list/render.rs", "src/menu/item/render.rs"] {
        assert!(
            !path_exists(forbidden),
            "selection_indicator host component should avoid render.rs drift file `{forbidden}`."
        );
    }

    let list_mod = load_source("src/list/mod.rs");
    let list_logic = load_source("src/list/logic.rs");
    let list_styles = load_source("src/list/styles.rs");
    let list_view = load_source("src/list/view.rs");
    let list_motion = load_source("src/list/motion.rs");
    let menu_item_mod = load_source("src/menu/item/mod.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let menu_item_styles = load_source("src/menu/item/styles.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "pub use view::{List, ListItem, ListSection};",
        "pub mod styles;",
        "pub mod motion;",
    ] {
        assert!(
            list_mod.contains(needle),
            "list mod.rs should keep minimal export boundary marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(",
        "pub(crate) mod item",
        "pub(crate) mod section",
    ] {
        assert!(
            list_logic.contains(needle),
            "list logic.rs should keep normalization/state markers `{needle}`."
        );
    }

    for needle in ["var(--ui-", ".ui-listbox", ".ui-listbox-item"] {
        assert!(
            list_styles.contains(needle),
            "list styles.rs should keep token-first static style marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "on:click", "event_target", "web_sys::"] {
        assert!(
            !list_styles.contains(forbidden),
            "list styles.rs should stay style-only and avoid `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn ListItem(",
        "data-selection-indicator",
        "on:click=move |_|",
    ] {
        assert!(
            list_view.contains(needle),
            "list view.rs should keep render + semantic mount marker `{needle}`."
        );
    }

    for needle in [
        "pub type ListMotion = ActiveHighlightMotion;",
        "pub fn sanitize_motion(motion: ListMotion) -> ListMotion",
        "pub fn attach_section_motion(",
        "ui_motion::",
    ] {
        assert!(
            list_motion.contains(needle),
            "list motion.rs should keep semantic-to-motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn List("] {
        assert!(
            !list_motion.contains(forbidden),
            "list motion.rs should avoid view responsibility marker `{forbidden}`."
        );
    }

    for needle in ["pub use view::MenuItem;", "pub mod styles;", "mod view;"] {
        assert!(
            menu_item_mod.contains(needle),
            "menu-item mod.rs should keep minimal export boundary marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(",
        "pub fn resolve_selection_indicator(",
    ] {
        assert!(
            menu_item_logic.contains(needle),
            "menu-item logic.rs should keep normalization/state markers `{needle}`."
        );
    }

    for needle in ["var(--ui-", ".ui-menu-item"] {
        assert!(
            menu_item_styles.contains(needle),
            "menu-item styles.rs should keep token-first static style marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "on:click", "event_target", "web_sys::"] {
        assert!(
            !menu_item_styles.contains(forbidden),
            "menu-item styles.rs should stay style-only and avoid `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn MenuItem(",
        "data-selection-indicator",
        "on:click=move |_|",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item view.rs should keep render + semantic mount marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_component_files_check_script_covers_shared_contract_gate() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test button_semantics button_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test button_semantics button_component_file_responsibilities_remain_scoped",
        "cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_component_directory_has_standard_file_layout",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should keep shared contract gate marker `{needle}`."
        );
    }

    assert!(
        !script_source.contains("selection_indicator_module_semantics"),
        "selection_indicator should reuse shared component-files gate; script should not require dedicated selection_indicator component-files command."
    );
}

#[test]
fn selection_indicator_check2_includes_agent_contract_schema_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "已核验（N/A-无独立 `selection_indicator` schema 文件）",
        "`crates/ui-components/src/list/view.rs` 与 `crates/ui-components/src/menu/item/view.rs` 挂载 `data-selection-indicator` + `data-state` + `data-aria-source` + `data-class-source`",
        "`crates/ui-components/src/list/logic.rs` 的 `ListItemSelectionIndicator/ListItemState`",
        "`crates/ui-components/src/menu/item/logic.rs` 的 `MenuItemSelectionIndicator/resolve_state`",
        "渲染链路未开放任意脚本注入入口",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep Agent Contract governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_host_views_expose_machine_readable_agent_contract_markers() {
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-selection-indicator=move || state.get().selection_indicator_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            list_view.contains(needle),
            "list view should keep machine-readable Agent Contract marker `{needle}`."
        );
    }

    for needle in [
        "data-kind=move || state.get().kind_attr",
        "data-state=move || state.get().data_state_attr",
        "data-selection-indicator=selection_indicator.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item view should keep machine-readable Agent Contract marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_agent_contract_fields_remain_typed_and_script_safe() {
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");

    for needle in [
        "pub enum ListItemSelectionIndicator",
        "pub struct ListItemState",
        "pub fn resolve_selection_indicator(",
        "pub fn resolve_state(input: ListItemStateInput) -> ListItemState",
        "selection_indicator_attr: selection_indicator.as_attr()",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            list_logic.contains(needle),
            "list logic should keep typed Agent Contract derivation marker `{needle}`."
        );
    }

    for needle in [
        "pub enum MenuItemSelectionIndicator",
        "pub fn resolve_kind_attr(kind: MenuItemKind) -> &'static str",
        "pub fn resolve_state(input: MenuItemStateInput) -> MenuItemState",
        "kind_attr: resolve_kind_attr(input.kind)",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            menu_item_logic.contains(needle),
            "menu-item logic should keep typed Agent Contract derivation marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "set_inner_html",
        "<script",
        "eval(",
    ] {
        assert!(
            !list_view.contains(forbidden),
            "list view should keep Agent Contract render path script-safe; found `{forbidden}`."
        );
        assert!(
            !menu_item_view.contains(forbidden),
            "menu-item view should keep Agent Contract render path script-safe; found `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_streaming_scope_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "已核验（N/A-`selection_indicator` 非正文阅读面）",
        "本项“流式”边界保持为上层输出协议定义",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep streaming-scope governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_snapshot_baseline_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "已核验：`selection_indicator` 当前通过宿主 `crates/ui-components/src/list/view.rs` 与 `crates/ui-components/src/menu/item/view.rs` 消费完整输入并渲染稳定状态",
        "`crates/ui-components/src/list/logic.rs::resolve_state` 与 `crates/ui-components/src/menu/item/logic.rs::resolve_state` 一次派生",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep snapshot baseline governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_streaming_requiredness_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "已核验（N/A-Streaming Optional + `fallback=snapshot`）",
        "默认仅消费 snapshot",
        "data-state/data-selection-indicator/data-aria-source/data-class-source",
        "断线恢复/重试/校验仍由上层承担",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep streaming-requiredness governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_hosts_remain_snapshot_first_with_continuous_semantic_markers() {
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let list_view = load_source("src/list/view.rs");
    let menu_item_view = load_source("src/menu/item/view.rs");
    let host_sources = format!("{list_logic}\n{menu_item_logic}\n{list_view}\n{menu_item_view}");

    for needle in [
        "pub fn resolve_state(input: ListItemStateInput) -> ListItemState",
        "pub fn resolve_state(input: MenuItemStateInput) -> MenuItemState",
        "role=\"option\"",
        "role=move || state.get().role_attr",
        "data-state=move || state.get().data_state_attr",
        "data-selection-indicator",
        "data-aria-source",
        "data-class-source",
    ] {
        assert!(
            host_sources.contains(needle),
            "selection_indicator host path should keep snapshot + semantic continuity marker `{needle}`."
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "ReadableStream",
        "on_message_chunk",
        "data-stream-mode",
        "streaming-delta",
    ] {
        assert!(
            !host_sources.contains(forbidden),
            "selection_indicator host path should not embed component-local streaming transport marker `{forbidden}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_testing_and_docs_closure_rules() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "回归测试集中在 `crates/ui-components/tests/selection_indicator_module_semantics.rs`",
        "`e2e/tests/docs_app_components_coverage.spec.mjs`",
        "`apps/docs-app/src/pages/components/pages/collections_extra.rs`",
        "`apps/docs-app/src/playground.rs` 提供 `compose_copy_ready_code(raw, imports)`",
        "\"selection-indicator\" => &[\"list-item\", \"menu-item\"]",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep testing/docs closure governance marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_docs_entry_state_matrix_and_playground_paths_are_present() {
    let components_mod = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let collections_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    let needle = "\"selection-indicator\" => &[\"list-item\", \"menu-item\"]";
    assert!(
        components_mod.contains(needle),
        "docs component mapping should keep selection-indicator host entry `{needle}`."
    );

    for needle in [
        "component_doc!(",
        "\"ListItem\"",
        "\"list-item\"",
        "collections_extra::list_item",
        "\"MenuItem\"",
        "\"menu-item\"",
        "collections_extra::menu_item",
    ] {
        assert!(
            pages_registry.contains(needle),
            "docs page registry should keep selection-indicator host doc marker `{needle}`."
        );
    }

    for needle in [
        "slug=\"list-item\"",
        "slug=\"menu-item\"",
        "<Playground title=\"Selectable Option\" code_signal=code>",
        "<Playground title=\"Focused + Divider + Disabled\" code_signal=states_code>",
        "<Playground title=\"Action + Checkbox\" code_signal=code>",
        "<Playground title=\"Radio + Submenu + Disabled\" code_signal=states_code>",
        "show_selection_indicator=true",
        "focused=true",
        "disabled=true",
        "kind=checkbox_kind",
        "kind=radio_kind",
    ] {
        assert!(
            collections_extra.contains(needle),
            "selection_indicator host docs should keep state-matrix/playground marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_docs_copy_ready_pipeline_and_beginner_flow_are_kept() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let components_mod = load_source("../../apps/docs-app/src/pages/components/mod.rs");
    let collections_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        ".map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))",
        "let section_class = \"docs-card playground\";",
        "pub fn Playground(",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep copy-ready beginner path marker `{needle}`."
        );
    }

    assert!(
        components_mod.contains("fn every_component_doc_page_renders_at_least_one_playground()"),
        "docs app should keep beginner-facing guarantee that each component page has a playground."
    );

    for needle in [
        "description=\"baseline-style list option primitive",
        "description=\"baseline-style menu row primitive",
    ] {
        assert!(
            collections_extra.contains(needle),
            "selection_indicator host docs should keep beginner-oriented component description marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_e2e_contracts_use_semantic_selectors_and_ready_waits() {
    let coverage_e2e = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let time_field_e2e_script = load_source("../../scripts/check-ui-components-e2e-time-field.sh");

    for needle in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "await expect(perfProbe).toHaveAttribute(\"data-perf-observability\", /mount/);",
    ] {
        assert!(
            coverage_e2e.contains(needle),
            "components-coverage e2e should keep semantic selector/ready wait marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !coverage_e2e.contains(forbidden),
            "components-coverage e2e should avoid brittle fixed-delay wait marker `{forbidden}`."
        );
    }

    for needle in [
        "contract: semantic selectors + settled waits",
        "contract: repeatable key flow with semantic breakpoints",
        "contract: checklist repeatable-flow governance + high-risk path coverage",
    ] {
        assert!(
            time_field_e2e_script.contains(needle),
            "shared e2e contract script should keep semantic selector/settled flow gate marker `{needle}`."
        );
    }
}

#[test]
fn selection_indicator_check2_includes_forbidden_anti_pattern_and_merge_gate_completion() {
    let check2 = load_source("src/selection_indicator/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
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
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "已核验（按 `selection_indicator` 责任范围）",
    ] {
        assert!(
            check2.contains(needle),
            "selection_indicator check2 should keep anti-pattern/merge-gate completion marker `{needle}`."
        );
    }

    assert!(
        !check2.contains("- [ ]"),
        "selection_indicator check2 should not keep unchecked checklist items after completion sweep."
    );
}

#[test]
fn selection_indicator_forbidden_anti_patterns_remain_absent_in_host_layers() {
    let selection_primitive = load_source("../ui-state-primitives/src/selection.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");
    let list_logic = load_source("src/list/logic.rs");
    let menu_item_logic = load_source("src/menu/item/logic.rs");
    let components_lib = load_source("src/lib.rs");
    let host_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for forbidden in ["leptos", "web_sys", "view! {", "class=", "var(--ui-"] {
        assert!(
            !selection_primitive.contains(forbidden),
            "status-primitives selection module should stay DOM/style-free; found `{forbidden}`."
        );
    }

    for forbidden in ["var(--ui-", "class=", "animation", "@keyframes"] {
        assert!(
            !headless_a11y.contains(forbidden),
            "ui-headless a11y contract should stay non-visual; found `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_state(input: ListItemStateInput) -> ListItemState",
        "pub fn resolve_state(input: MenuItemStateInput) -> MenuItemState",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            list_logic.contains(needle) || menu_item_logic.contains(needle),
            "host logic should keep centralized state-derivation marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod selection_indicator;",
        "pub use web_sys::",
        "pub use leptos::web_sys",
    ] {
        assert!(
            !components_lib.contains(forbidden),
            "ui-components public API should not leak selection-indicator compat/platform detail `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels"] {
        assert!(
            !host_docs.contains(forbidden),
            "selection_indicator host docs should avoid implicit parallel-array API marker `{forbidden}`."
        );
    }
}
