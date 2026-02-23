use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_circular_progress_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir
        .join("components/circular-progress")
        .join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_circular_progress_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-circular_progress\")]")
            && lib_source.contains("pub use ui_circular_progress as circular_progress;"),
        "ui should re-export the external ui-circular-progress crate as `circular_progress`.",
    );
    assert!(
        cargo_source.contains("component-circular_progress = [\"dep:ui-circular-progress\"]"),
        "component-circular_progress feature should depend on dep:ui-circular-progress after extraction.",
    );
    assert!(
        cargo_source
            .contains("ui-circular-progress = { path = \"../../components/circular-progress\", optional = true }"),
        "ui Cargo.toml should include the optional ui-circular-progress dependency.",
    );
}

#[test]
fn circular_progress_does_not_expose_logic_or_view_modules() {
    let source = load_circular_progress_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CircularProgress internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn circular_progress_uses_logic_state_model() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");

    for needle in [
        "pub use ui_state_primitives::circular_progress::{",
        "CircularProgressState",
        "CircularProgressStateInput",
        "DEFAULT_ARIA_LABEL",
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state",
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CircularProgress logic should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub struct CircularProgressStateInput",
        "pub struct CircularProgressState",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Loading\";",
        "pub fn normalize_optional_text(",
        "pub fn resolve_aria_label(value: Option<String>, default_aria_label: &str)",
        "pub fn sanitize_dimension(",
        "pub fn resolve_state(",
        "pub size_px: Option<f64>",
        "pub thickness_px: Option<f64>",
        "size_source_attr",
        "thickness_source_attr",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "CircularProgress primitives should include `{needle}` for centralized state derivation."
        );
    }
    for forbidden in ["pub fn compose_style_vars(", "pub fn compose_class_name("] {
        assert!(
            !primitive_source.contains(forbidden),
            "CircularProgress primitives should stay style-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let logic::CircularProgressLogicOutput {",
        "} = logic::resolve_component_contract(CircularProgressLogicInput {",
        "default_aria_label: common.loading_aria_label.as_ref(),",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "style=style_vars",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn circular_progress_emits_baseline_style_state_data_attributes() {
    let source = load_circular_progress_component_source("src/view.rs");

    for attr in [
        "data-slot=\"circular-progress\"",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-custom-size=semantics.attrs.data_custom_size",
        "data-custom-thickness=semantics.attrs.data_custom_thickness",
        "data-custom-aria-label=semantics.attrs.data_custom_aria_label",
        "data-custom-class=semantics.attrs.data_custom_class",
        "data-class-source=semantics.attrs.data_class_source",
        "role=semantics.attrs.role",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "CircularProgress should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn circular_progress_consumes_ui_headless_a11y_contract() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");

    for needle in [
        "use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress view should mount ui-headless contract `{needle}`."
        );
    }

    for needle in [
        "pub struct CircularProgressAttrs",
        "pub struct CircularProgressSemanticState",
        "pub struct CircularProgressContract",
        "pub struct CircularProgressOptions",
        "pub fn use_circular_progress(options: CircularProgressOptions) -> CircularProgressContract",
        "role: \"progressbar\"",
        "aria_valuemin: \"0\"",
        "aria_valuemax: \"100\"",
        "locale_attrs(options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless circular-progress contract should contain `{needle}`."
        );
    }
}

#[test]
fn circular_progress_styles_include_state_marker_contracts() {
    let source = load_circular_progress_component_source("src/styles.rs");

    for selector in [
        ".ui-circular-progress--state-indeterminate",
        ".ui-circular-progress[data-motion=\"spin\"]",
        ".ui-circular-progress--size-custom",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress--thickness-custom",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress--label-custom",
        ".ui-circular-progress[data-label-source=\"custom\"]",
        ".ui-circular-progress--custom-class",
        ".ui-circular-progress[data-custom-class=\"true\"]",
        "--ui-cp-rotation-duration",
        "--ui-button-spinner-duration",
        "prefers-reduced-motion: reduce",
        "animation-duration: 1ms;",
    ] {
        assert!(
            source.contains(selector),
            "CircularProgress styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn circular_progress_motion_contract_stays_token_driven_and_runtime_free() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let ui_motion_source = load_ui_components_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "animation: ui-circular-progress-spin",
        "var(--ui-cp-rotation-duration,",
        "var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))",
        "@media (prefers-reduced-motion: reduce)",
        "animation-duration: 1ms;",
    ] {
        assert!(
            styles_source.contains(needle),
            "CircularProgress styles should keep token-driven motion contract `{needle}`."
        );
    }

    for forbidden in [
        "0.9s linear infinite",
        "use ui_motion",
        "attach_motion",
        "MotionOptions",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "CircularProgress should avoid component-local motion runtime wiring `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm predictable no-op contract `{needle}`."
        );
    }
}

#[test]
fn circular_progress_ui_theme_layer_uses_shared_token_pipeline_without_rebuilding_theme() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let check2_source = load_circular_progress_component_source("check2.md");
    let ui_theme_tokens_source = load_ui_components_source("../ui-theme/src/tokens.rs");
    let ui_theme_theme_source = load_ui_components_source("../ui-theme/src/theme.rs");
    let ui_theme_css_source = load_ui_components_source("../ui-theme/src/css.rs");
    let token_baseline_source =
        load_ui_components_source("../ui-theme/tests/token_scale_baseline.rs");
    let styling_spec_source = load_ui_components_source("../../docs/spec/styling.md");

    for needle in [
        "single source of truth for token taxonomy and baselines",
        "pub enum TokenScale",
        "pub struct ThemeTokens",
    ] {
        assert!(
            ui_theme_tokens_source.contains(needle),
            "ui-theme token taxonomy should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "ThemeColor::Light",
        "ThemeColor::Dark",
        "ThemeColor::Oled",
    ] {
        assert!(
            ui_theme_theme_source.contains(needle),
            "ui-theme context mapping should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "--ui-system",
        "--ui-color",
        "--ui-scale",
    ] {
        assert!(
            ui_theme_css_source.contains(needle),
            "ui-theme css emitter should keep marker `{needle}`.",
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "CircularProgress styles should consume ui-theme variables via `var(--ui-*)`.",
    );

    for forbidden in ["--cp-", "--circular-progress-"] {
        assert!(
            !styles_source.contains(forbidden),
            "CircularProgress styles must not define parallel private token prefix `{forbidden}`."
        );
    }

    for forbidden in [
        "ThemeContext",
        "ThemeColor",
        "ThemeScale",
        "Theme::",
        "to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress component layer must not rebuild theme mapping via `{forbidden}`.",
        );
    }

    for needle in [
        "Token 统一基线落点固定",
        "crates/ui-theme/src/tokens.rs",
        "crates/ui-theme/src/theme.rs",
        "crates/ui-theme/src/css.rs",
        "WCAG 2.1 AA",
        "Light/Dark/OLED",
    ] {
        assert!(
            styling_spec_source.contains(needle),
            "styling spec should keep ui-theme contract marker `{needle}`.",
        );
    }

    for needle in [
        "fn token_scale_baselines_are_regression_testable()",
        "Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium)",
        "Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium)",
        "Theme::baseline_two(ThemeColor::Oled, ThemeScale::Medium)",
    ] {
        assert!(
            token_baseline_source.contains(needle),
            "ui-theme token baseline regression should keep marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "circular_progress_ui_theme_layer_uses_shared_token_pipeline_without_rebuilding_theme",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress checklist should keep ui-theme evidence marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_ui_components_layer_keeps_assembly_boundaries() {
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
        "#[cfg(test)]",
        "#[path = \"../test/semantics.rs\"]",
    ] {
        assert!(
            module_source.contains(needle),
            "CircularProgress module should keep ui assembly marker `{needle}`.",
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module_source.contains(forbidden),
            "CircularProgress internals should stay private; found `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::circular_progress::{"),
        "CircularProgress logic should consume state primitives instead of reimplementation.",
    );
    assert!(
        view_source.contains(
            "use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};"
        ),
        "CircularProgress view should mount headless semantic contract.",
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "CircularProgress styles should remain token-first static CSS.",
    );

    for forbidden in ["web_sys::", "web-sys", "HtmlElement", "NodeRef"] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "CircularProgress ui layer must not leak platform detail `{forbidden}`.",
        );
    }

    for needle in [
        "fn circular_progress_module_keeps_public_surface_stable()",
        "fn circular_progress_component_layer_assembles_primitives_headless_and_theme_consumption()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "circular-progress local semantics suite should contain `{needle}`.",
        );
    }

    for needle in [
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/circular-progress/test/semantics.rs",
        "circular_progress_ui_components_layer_keeps_assembly_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep ui evidence marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_api_naming_contract_is_stable_and_n_a_for_state_axis_prefixes() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress public API naming should include `{required}`."
        );
    }

    for forbidden in [
        "ariaLabel",
        "className",
        "#[prop(optional)] open:",
        "on_open_change",
        "default_open",
        "on_value_change",
        "default_value",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress should not drift to alias or unrelated state-axis naming `{forbidden}`."
        );
    }

    // CircularProgress exposes no boolean/callback/default public state axis.
    for forbidden in [
        "#[prop(optional)] is_",
        "#[prop(optional, into)] is_",
        "#[prop(optional)] on_",
        "#[prop(optional, into)] on_",
        "#[prop(optional)] default_",
        "#[prop(optional, into)] default_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress should keep `is_/on_/default_` prefixes N/A at prop declaration level; found `{forbidden}`."
        );
    }

    for required in [
        "<CircularProgress aria_label=\"Loading\".to_string() />",
        "size_px=24.0",
        "thickness_px=3.0",
        "class_name=\"docs-circular-progress-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "CircularProgress docs usage should keep canonical API name `{required}`."
        );
    }
}

#[test]
fn circular_progress_check2_marks_api_naming_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "circular_progress_api_naming_contract_is_stable_and_n_a_for_state_axis_prefixes",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep API naming marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_controlled_uncontrolled_pair_contract_is_explicitly_not_applicable() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");

    // CircularProgress is an indeterminate display component and intentionally has no controlled axis.
    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional, into)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional, into)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional, into)] on_value_change:",
        "#[prop(optional)] open:",
        "#[prop(optional, into)] open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional, into)] default_open:",
        "#[prop(optional)] on_open_change:",
        "#[prop(optional, into)] on_open_change:",
        "on_value_change",
        "default_value",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress must stay N/A for controlled/uncontrolled axis contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress should remain a display-only API surface with `{required}`."
        );
    }
}

#[test]
fn circular_progress_check2_marks_controlled_uncontrolled_pair_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "N/A-by-design",
        "circular_progress_controlled_uncontrolled_pair_contract_is_explicitly_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep controlled/uncontrolled marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_default_values_are_normalized_once_in_logic() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");

    for needle in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
        "pub fn resolve_component_contract(",
        "let default_aria_label = resolve_default_aria_label(input.default_aria_label);",
        "let (aria_label, has_custom_aria_label) = resolve_aria_label(input.aria_label, default_aria_label);",
    ] {
        assert!(
            logic_source.contains(needle),
            "CircularProgress logic should centralize default priority via `{needle}`.",
        );
    }

    for needle in [
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "default_aria_label: common.loading_aria_label.as_ref(),",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress view should consume logic-only default contract `{needle}`.",
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(lang)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(",
        "logic::resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress view should not run default fallbacks directly; found `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_single_default_source_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "circular_progress_default_values_are_normalized_once_in_logic",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep single-default-source marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_state_normalization_is_centralized_in_logic() {
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");

    for needle in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "let state = resolve_state(CircularProgressStateInput {",
        "let class = compose_class_name(class_name, &state);",
        "let style_vars = compose_style_vars(&state);",
    ] {
        assert!(
            logic_source.contains(needle),
            "CircularProgress logic should centralize state normalization via `{needle}`.",
        );
    }

    for needle in [
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress view should only consume normalized state markers `{needle}`.",
        );
    }

    for forbidden in [
        "logic::resolve_state(CircularProgressStateInput {",
        "logic::normalize_optional_text(",
        "logic::resolve_aria_label(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress view should not rebuild state normalization `{forbidden}`.",
        );
    }

    for selector in [
        ".ui-circular-progress--state-indeterminate",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress[data-label-source=\"custom\"]",
        ".ui-circular-progress[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "CircularProgress styles should only consume state markers via `{selector}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_state_normalization_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "circular_progress_state_normalization_is_centralized_in_logic",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep state-normalization marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_discrete_state_contract_is_n_a_without_free_string_or_boolean_explosion() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");

    for forbidden in [
        "Option<bool>",
        "#[prop(optional)] variant:",
        "#[prop(optional, into)] variant:",
        "#[prop(optional)] mode:",
        "#[prop(optional, into)] mode:",
        "#[prop(optional)] status:",
        "#[prop(optional, into)] status:",
        "#[prop(optional)] size:",
        "#[prop(optional, into)] size:",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "is_success",
        "is_warning",
        "is_error",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "CircularProgress should avoid free-form discrete state protocol `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "ui-circular-progress--state-indeterminate",
        "data-state=semantics.attrs.data_state",
        "pub struct CircularProgressStateInput",
        "pub struct CircularProgressState",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || primitive_source.contains(required),
            "CircularProgress should keep fixed state contract marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_discrete_state_type_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "N/A-by-design",
        "circular_progress_discrete_state_contract_is_n_a_without_free_string_or_boolean_explosion",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep discrete-state marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_state_primitives_source_stays_in_ui_state_primitives_without_business_store_binding()
 {
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");

    for required in [
        "pub use ui_state_primitives::circular_progress::{",
        "CircularProgressState",
        "CircularProgressStateInput",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state",
        "let state = resolve_state(CircularProgressStateInput {",
        "logic::resolve_component_contract(CircularProgressLogicInput {",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "CircularProgress should consume state primitives via `{required}`.",
        );
    }

    for forbidden in [
        "Signal<",
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
        "Store<",
        "store::",
        "redux",
        "zustand",
        "mobx",
        "Context<",
        "Arc<Mutex",
        "tokio::sync",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "CircularProgress component layer should not bind business/global store `{forbidden}`.",
        );
    }

    for forbidden in ["compose_style_vars", "compose_class_name"] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives must stay focused on state derivation, found `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_state_primitive_source_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "circular_progress_state_primitives_source_stays_in_ui_state_primitives_without_business_store_binding",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep state-primitives-source marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_async_interaction_contract_is_not_applicable_for_display_only_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");

    for forbidden in [
        "#[prop(optional)] is_loading:",
        "#[prop(optional, into)] is_loading:",
        "is_loading",
        "is_disabled",
        "on_retry",
        "retry",
        "error",
        "aria-busy",
        "aria_busy",
        "use_async_action",
        "tokio::spawn",
        "spawn_local",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "CircularProgress should not define async loading/error protocol `{forbidden}`.",
        );
    }

    for required in [
        "pub fn CircularProgress(",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "data-state=semantics.attrs.data_state",
        "let semantics = use_circular_progress(CircularProgressOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress should remain a display-only semantic contract with `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_async_contract_complete_for_non_async_component() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "N/A-by-design",
        "circular_progress_async_interaction_contract_is_not_applicable_for_display_only_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep async-contract marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_paradox_keeps_hello_world_simple_and_hides_internal_state_wiring() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(into)] state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress should not require internal state prop `{forbidden}` for basic API.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "let hello_world_code = Signal::derive(move || r#\"<CircularProgress />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<CircularProgress />",
    ] {
        assert!(
            section.contains(required),
            "CircularProgress docs should keep minimal hello-world DX path `{required}`.",
        );
    }

    for forbidden in ["ui_state_primitives", "use_circular_progress", "state="] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs hello-world path should hide internal plumbing `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_dx_paradox_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "circular_progress_dx_paradox_keeps_hello_world_simple_and_hides_internal_state_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep DX paradox marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_composite_parent_item_api_contract_is_not_applicable_for_single_node_component()
 {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop(optional)] children:",
        "#[prop(optional, into)] children:",
        "#[prop(optional)] items:",
        "#[prop(optional, into)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "ItemSpec",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress public API should not expose composite parent/item contract `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<CircularProgress />",
    ] {
        assert!(
            section.contains(required),
            "CircularProgress docs should keep single-node default path marker `{required}`.",
        );
    }

    for forbidden in [
        "labels=",
        "titles=",
        "panels=",
        "children=",
        "items=",
        "ItemSpec",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs section should not use composite parallel-slot conventions `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_composite_parent_item_api_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A-by-design",
        "circular_progress_composite_parent_item_api_contract_is_not_applicable_for_single_node_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep composite-parent-item marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_macro_micro_duality_contract_is_not_applicable_without_drag_interaction() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "cancelAnimationFrame",
        "raf",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "CircularProgress should not implement drag macro/micro loop token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Dragging",
        "DragEnd",
        "drag",
        "pointermove",
        "requestAnimationFrame",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe drag macro/micro flow token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_macro_micro_duality_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "N/A-by-design",
        "circular_progress_macro_micro_duality_contract_is_not_applicable_without_drag_interaction",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep macro/micro-duality marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_two_pass_rendering_contract_is_not_applicable_without_dom_measurement() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "getBoundingClientRect",
        "NodeRef",
        "Measure(",
        "Rectification(",
        "Intent -> Measure(view) -> Rectification(logic)",
        "measure_rect",
        "layout_rect",
        "overlay",
        "popover",
        "tooltip",
        "menu",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should not define two-pass measurement token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "getBoundingClientRect",
        "NodeRef",
        "Measure(",
        "Rectification(",
        "tooltip",
        "popover",
        "menu",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe two-pass geometry loop token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_two_pass_rendering_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A-by-design",
        "circular_progress_two_pass_rendering_contract_is_not_applicable_without_dom_measurement",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep two-pass-rendering marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_registration_protocol_contract_is_not_applicable_for_single_node_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "accordion",
        "tabs",
        "menu",
        "children",
        "items:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should not define collection registration protocol token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
        "children=",
        "items=",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe registration protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_registration_protocol_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "N/A-by-design",
        "circular_progress_registration_protocol_contract_is_not_applicable_for_single_node_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep registration-protocol marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_slot_projection_contract_is_not_applicable_for_single_node_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot projection",
        "children",
        "items:",
        "pause_animation_on_hidden",
        "pause_polling_on_hidden",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "CircularProgress should not define slot-projection protocol token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot",
        "children=",
        "items=",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe slot-projection token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_slot_projection_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "N/A-by-design",
        "circular_progress_slot_projection_contract_is_not_applicable_for_single_node_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep slot-projection marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_env_streams_contract_is_not_applicable_for_display_only_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "ThemeChanged",
        "BreakpointChanged",
        "Action::BreakpointChanged",
        "on:resize",
        "window.onresize",
        "match_media",
        "debounce",
        "throttle",
        "IntersectionChanged",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should not define env-stream token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "ThemeChanged",
        "BreakpointChanged",
        "debounce",
        "throttle",
        "resize",
        "intersection",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe env-stream token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_env_streams_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "N/A-by-design",
        "circular_progress_env_streams_contract_is_not_applicable_for_display_only_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep env-streams marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_event_light_cone_contract_is_not_applicable_for_single_node_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "BulkSelect",
        "row_selection",
        "select_all",
        "prop drilling",
        "event light cone",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should not define event-light-cone token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "BulkSelect",
        "row_selection",
        "select_all",
        "prop drilling",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe event-light-cone token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_event_light_cone_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "N/A-by-design",
        "circular_progress_event_light_cone_contract_is_not_applicable_for_single_node_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep event-light-cone marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_causality_bus_contract_is_not_applicable_for_single_node_component() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "CausalityBus",
        "cause_id",
        "event_bus",
        "bus broadcast",
        "subscriber",
        "derived command",
        "command bus",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should not define causality-bus token `{forbidden}`.",
        );
    }

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "CausalityBus",
        "event_bus",
        "bus broadcast",
        "subscriber",
        "derived command",
        "command bus",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not describe causality-bus token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_causality_bus_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "N/A-by-design",
        "circular_progress_causality_bus_contract_is_not_applicable_for_single_node_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep causality-bus marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_a11y_i18n_l10n_contract_is_mounted_via_headless_and_i18n_chain() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");
    let a11y_source = load_ui_components_source("../../crates/ui-headless/src/a11y.rs");
    let headless_test_source =
        load_ui_components_source("../../crates/ui-headless/src/test/circular_progress.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");

    for required in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "default_aria_label: common.loading_aria_label.as_ref(),",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress view should mount a11y+i18n contract via `{required}`.",
        );
    }

    for forbidden in ["aria-label=\"Loading\"", "Loading</", ">Loading<"] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress view should not hardcode visible copy token `{forbidden}`.",
        );
    }

    for required in [
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
        "let (aria_label, has_custom_aria_label) =",
        "resolve_aria_label(input.aria_label, default_aria_label);",
    ] {
        assert!(
            logic_source.contains(required),
            "CircularProgress logic should keep i18n-aware aria fallback chain `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Loading\";",
        "pub fn resolve_aria_label(value: Option<String>, default_aria_label: &str)",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives should keep fallback label contract `{required}`.",
        );
    }

    for required in [
        "pub fn use_circular_progress(options: CircularProgressOptions) -> CircularProgressContract",
        "role: \"progressbar\"",
        "aria_valuemin: \"0\"",
        "aria_valuemax: \"100\"",
        "locale_attrs(options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless circular-progress contract should include `{required}`.",
        );
    }

    for required in [
        "pub enum A11yDirection",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs",
    ] {
        assert!(
            a11y_source.contains(required),
            "ui-headless shared a11y utility should include `{required}`.",
        );
    }

    for required in [
        "fn use_circular_progress_maps_locale_and_custom_source_attrs()",
        "assert_eq!(contract.attrs.role, \"progressbar\");",
        "assert_eq!(contract.attrs.lang.as_deref(), Some(\"zh-CN\"));",
        "assert_eq!(contract.attrs.dir, Some(\"rtl\"));",
    ] {
        assert!(
            headless_test_source.contains(required),
            "ui-headless circular-progress tests should regression-cover `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_a11y_i18n_l10n_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "props aria_label > i18n 注入 > DEFAULT_ARIA_LABEL",
        "circular_progress_a11y_i18n_l10n_contract_is_mounted_via_headless_and_i18n_chain",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep a11y+i18n+l10n marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_state_observability_contract_uses_stable_data_and_aria_markers() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");

    for required in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-custom-size=semantics.attrs.data_custom_size",
        "data-custom-thickness=semantics.attrs.data_custom_thickness",
        "data-custom-aria-label=semantics.attrs.data_custom_aria_label",
        "data-custom-class=semantics.attrs.data_custom_class",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress view should expose stable observability marker `{required}`.",
        );
    }

    for required in [
        "pub struct CircularProgressAttrs",
        "pub data_state: &'static str",
        "pub data_motion: &'static str",
        "pub data_size_source: &'static str",
        "pub data_thickness_source: &'static str",
        "pub data_label_source: &'static str",
        "pub data_class_source: &'static str",
        "data_state: \"indeterminate\"",
        "data_motion: \"spin\"",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless contract should provide typed stable marker `{required}`.",
        );
    }

    for required in [
        "size_source_attr: if has_custom_size { \"custom\" } else { \"default\" }",
        "thickness_source_attr: if has_custom_thickness {",
        "label_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives should keep marker source as enumerable set via `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_state_observability_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "封闭集合 `custom/default`",
        "circular_progress_state_observability_contract_uses_stable_data_and_aria_markers",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep state-observability marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_style_contract_depends_on_explicit_state_markers_not_dom_structure_guessing() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");

    for required in [
        ".ui-circular-progress--state-indeterminate",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-motion=\"spin\"]",
        ".ui-circular-progress--size-custom",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress--thickness-custom",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress--label-custom",
        ".ui-circular-progress[data-label-source=\"custom\"]",
        ".ui-circular-progress--custom-class",
        ".ui-circular-progress[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "CircularProgress styles should use explicit semantic selector `{required}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        ":has(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "CircularProgress styles should not rely on fragile DOM-shape selector `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("style=style_vars"),
        "CircularProgress view should only pass runtime style vars via `style=style_vars`.",
    );

    for required in [
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            logic_source.contains(required),
            "CircularProgress logic should limit runtime style payload to CSS variables `{required}`.",
        );
    }

    for forbidden in ["border:", "border-top-color:", "box-shadow:", "animation:"] {
        assert!(
            !logic_source.contains(forbidden),
            "CircularProgress logic should not embed business/visual inline style rule `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_style_explicit_state_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "circular_progress_style_contract_depends_on_explicit_state_markers_not_dom_structure_guessing",
        "`--ui-cp-size`/`--ui-cp-thickness`",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep style-explicit-state marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots()
 {
    let workspace_semantics_source =
        load_ui_components_source("tests/circular_progress/semantics.rs");
    let component_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "fn circular_progress_emits_baseline_style_state_data_attributes()",
        "fn circular_progress_state_observability_contract_uses_stable_data_and_aria_markers()",
        "fn circular_progress_a11y_i18n_l10n_contract_is_mounted_via_headless_and_i18n_chain()",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            workspace_semantics_source.contains(required),
            "workspace semantics suite should keep semantic-contract assertion marker `{required}`.",
        );
    }

    for required in [
        "fn circular_progress_exposes_stable_observable_state_markers_for_selectors()",
        "fn circular_progress_has_a11y_i18n_l10n_contract_without_view_level_hardcoded_copy()",
    ] {
        assert!(
            component_semantics_source.contains(required),
            "component-local semantics suite should keep marker `{required}`.",
        );
    }

    for forbidden in ["assert_snapshot", "insta::", "to_match_snapshot"] {
        assert!(
            !workspace_semantics_source.contains(forbidden)
                && !component_semantics_source.contains(forbidden),
            "semantics suites should not rely on snapshot-only assertion token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_semantic_contract_testing_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "N/A-by-design",
        "circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep semantic-testing marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");
    let semantics_source = load_ui_components_source("tests/circular_progress/semantics.rs");
    let perf_script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "circular-progress semantic-priority contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "fn circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots()",
        "fn circular_progress_state_observability_contract_uses_stable_data_and_aria_markers()",
        "fn circular_progress_exposes_stable_observable_state_markers_for_selectors()",
        "for forbidden in [\"assert_snapshot\", \"insta::\", \"to_match_snapshot\"]",
        "semantics suites should not rely on snapshot-only assertion token",
    ] {
        assert!(
            local_semantics_source.contains(needle) || semantics_source.contains(needle),
            "circular-progress semantic-priority suite should keep marker `{needle}`.",
        );
    }

    for forbidden_snapshot in ["toHaveScreenshot(", "toMatchSnapshot(", "screenshot("] {
        assert!(
            !semantics_source.contains(forbidden_snapshot),
            "circular-progress semantics should avoid snapshot-only assertion `{forbidden_snapshot}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`.",
    );
}

#[test]
fn circular_progress_performance_script_covers_semantic_test_priority_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "echo \"[perf] contract: circular-progress semantic test priority\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should include circular-progress semantic-priority marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_semantic_test_priority_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots",
        "circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "circular_progress_performance_script_covers_semantic_test_priority_contract",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 semantic-test-priority section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_semantic_test_priority_item_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "circular-progress check2 should mark semantic-test-priority item complete.",
    );

    for needle in [
        "circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "circular_progress_performance_script_covers_semantic_test_priority_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks_locally",
        "scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should retain semantic-test-priority marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress checklist should keep e2e selector/stable-wait rule `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "page.goto(\"/#/components/circular-progress\")",
        "body:not(:has(#boot))",
        "[data-component=\"circular-progress\"]",
        "[data-slot=\"circular-progress\"]",
        "[data-ui-schema=\"ui.circular-progress.agent-contract\"]",
        "[data-ui-schema-version=\"v1\"]",
        "[data-ui-state=\"indeterminate\"]",
        "[role=\"progressbar\"]",
        "toBeVisible()",
        "toHaveAttribute(\"data-state\", \"indeterminate\")",
        "toHaveAttribute(\"data-motion\", \"spin\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "circular-progress e2e selector/stable-wait contract should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"circular-progress-streaming-policy\"",
        "data-slot=\"circular-progress-copy-ready-hint\"",
        "data-slot=\"circular-progress-source-first\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress docs source should keep semantic anchor `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "nth-child(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "circular-progress e2e selector contract should avoid flaky/snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_e2e_contract_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");

    for needle in [
        "async function gotoCircularProgressDocsAndWaitSettled(page)",
        "await page.locator(WASM_READY_SELECTOR).waitFor();",
        "data-ui-schema=\"ui.circular-progress.agent-contract\"",
        "data-ui-state=\"indeterminate\"",
        "data-motion=\"spin\"",
        "toHaveText(/fallback=snapshot/)",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"indeterminate\")",
        "toHaveAttribute(\"data-motion\", \"spin\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "circular-progress e2e ready/settled contract should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_key_flow_regression_collection_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress checklist should keep key-flow regression collection rule `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");

    for needle in [
        "test(\"docs-app circular-progress key flow regression uses semantic breakpoints for diagnosis\"",
        "await gotoCircularProgressDocsAndWaitSettled(page)",
        "test.step(\"open route reaches semantic ready breakpoint\"",
        "test.step(\"interaction keeps source markers diagnosable\"",
        "test.step(\"reopen/remount keeps settled breakpoint stable\"",
        "data-ui-action=\"render\"",
        "data-ui-source=\"state-primitives\"",
        "data-size-source=\"default\"",
        "data-size-source=\"custom\"",
        "await page.reload();",
        "data-motion=\"spin\"",
    ] {
        assert!(
            e2e_source.contains(needle),
            "circular-progress e2e key-flow regression contract should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source =
        load_ui_components_source("../../components/circular-progress/scripts/check-ui-e2e-circular-progress.sh");

    for needle in [
        "echo \"[e2e-circular-progress] contract: checklist e2e-selector/stable-wait governance\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_e2e_selector_and_stable_wait_rules",
        "echo \"[e2e-circular-progress] contract: semantic selectors + settled waits\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "echo \"[e2e-circular-progress] contract: animation path ready/settled semantic breakpoints\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "circular-progress e2e script should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_e2e_check_script_covers_key_flow_regression_contract() {
    let script_source =
        load_ui_components_source("../../components/circular-progress/scripts/check-ui-e2e-circular-progress.sh");

    for needle in [
        "echo \"[e2e-circular-progress] contract: key flow regression is repeatable and semantic-breakpoint diagnosable\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable",
    ] {
        assert!(
            script_source.contains(needle),
            "circular-progress e2e script should include key-flow regression gate `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "circular-progress check2 should mark e2e selector stability item complete.",
    );

    for needle in [
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
        "components/circular-progress/test/semantics.rs::circular_progress_e2e_selector_stability_prefers_semantic_markers_and_settled_waits_locally",
        "components/circular-progress/scripts/check-ui-e2e-circular-progress.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 e2e selector stability section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_key_flow_regression_collection_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "circular-progress check2 should mark key-flow regression collection item complete.",
    );

    for needle in [
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_check2_documents_key_flow_regression_collection_rules",
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable",
        "components/circular-progress/test/circular_progress/semantics.rs::circular_progress_e2e_check_script_covers_key_flow_regression_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_e2e_key_flow_regression_collection_is_repeatable_and_breakpoint_diagnosable_locally",
        "components/circular-progress/scripts/check-ui-e2e-circular-progress.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 key-flow regression section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_component_file_responsibility_contract_is_correct_with_motion_runtime_not_applicable()
 {
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal export boundary marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "pub mod motion",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not expose internal/runtime motion module `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "NodeRef",
        "getBoundingClientRect",
        "document()",
        "window()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain DOM operation token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for forbidden in ["#[component]", "use ui_headless", "use leptos"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not mix rendering/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "view! {",
        "data-state=semantics.attrs.data_state",
        "role=semantics.attrs.role",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mounting marker `{required}`.",
        );
    }

    for forbidden in ["@keyframes", ".ui-circular-progress {", "web_sys::"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not own static CSS/DOM API token `{forbidden}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let motion_path = workspace_dir
        .join("components/circular-progress")
        .join("src/motion.rs");
    assert!(
        !motion_path.exists(),
        "motion.rs should stay N/A-by-design for circular-progress runtime motion attach.",
    );
}

#[test]
fn circular_progress_check2_marks_component_file_responsibility_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "N/A-by-design",
        "circular_progress_component_file_responsibility_contract_is_correct_with_motion_runtime_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep component-file-responsibility marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_spec_rs_contract_is_not_applicable_for_simple_component() {
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "CircularProgressSpec",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "CircularProgress should not expose spec.rs contract token `{forbidden}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_dir
        .join("components/circular-progress")
        .join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple circular-progress component should not add `src/spec.rs`.",
    );

    let section_start = docs_source
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display.rs should contain circular_progress page section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display.rs should contain spinner page after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in ["Spec::new(", "CircularProgressSpec", "schema", "spec.rs"] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should not require spec builder/schema token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_spec_rs_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "N/A-by-design",
        "circular_progress_spec_rs_contract_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep spec-rs contract marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component()
{
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        ".render()",
        "CircularProgressSpec",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "CircularProgress should not expose Hyper-Structure Builder path token `{forbidden}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_dir
        .join("components/circular-progress")
        .join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple circular-progress component should keep Hyper-Structure spec.rs as N/A.",
    );
}

#[test]
fn circular_progress_component_files_check_script_covers_hyper_structure_builder_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_hyper_structure_builder_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design",
        "circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component",
        "circular_progress_component_files_check_script_covers_hyper_structure_builder_contract",
        "circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep Hyper-Structure Builder marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent() {
    let component_manifest_source = load_circular_progress_component_source("src/Component.toml");
    let component_rbi_source = load_circular_progress_component_source("src/circular_progress.rbi");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src_dir = workspace_dir.join("components/circular-progress/src");

    for required_file in ["Component.toml", "circular_progress.rbi"] {
        assert!(
            component_src_dir.join(required_file).exists(),
            "CircularProgress context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"CircularProgress\"",
        "crate = \"ui-circular-progress\"",
        "name = \"aria_label\"",
        "name = \"size_px\"",
        "name = \"thickness_px\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest_source.contains(required),
            "Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn CircularProgress(",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi_source.contains(required),
            "circular_progress.rbi should keep signature-projection marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_component_files_check_script_covers_context_compression_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_context_compression_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent",
        "circular_progress_component_files_check_script_covers_context_compression_contract",
        "circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep context-compression marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            check2_source.contains(required),
            "CircularProgress checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let manifest_source = load_circular_progress_component_source("src/Component.toml");
    let rbi_source = load_circular_progress_component_source("src/circular_progress.rbi");

    for typed_source in [
        "pub const CIRCULAR_PROGRESS_AGENT_SCHEMA: &str = \"ui.circular-progress.agent-contract\";",
        "pub enum CircularProgressAgentSchemaVersion",
        "pub enum CircularProgressAgentIntent",
        "pub enum CircularProgressAgentAction",
        "pub enum CircularProgressAgentState",
        "pub enum CircularProgressAgentSource",
        "pub struct CircularProgressAgentContract",
        "pub fn resolve_agent_contract(state: &CircularProgressState) -> CircularProgressAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "CircularProgress Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-size-source=agent_contract.size_source",
        "data-ui-thickness-source=agent_contract.thickness_source",
        "data-ui-label-source=agent_contract.label_source",
        "data-ui-class-source=agent_contract.class_source",
    ] {
        assert!(
            view_source.contains(marker),
            "CircularProgress view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent-contract-markers\"",
        "CIRCULAR_PROGRESS_AGENT_SCHEMA",
        "CircularProgressAgentContract",
    ] {
        assert!(
            manifest_source.contains(required) || rbi_source.contains(required),
            "Context compression assets should keep Agent Contract marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()
 {
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");

    assert!(
        logic_source.contains("schema_name: CIRCULAR_PROGRESS_AGENT_SCHEMA")
            && logic_source.contains("size_source: state.size_source_attr")
            && logic_source.contains("thickness_source: state.thickness_source_attr")
            && logic_source.contains("label_source: state.label_source_attr")
            && logic_source.contains("class_source: state.class_source_attr"),
        "Agent Contract fields should be derived from typed constants/state source attrs in logic.rs.",
    );

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "format!(\"ui.circular-progress",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should avoid free-form Agent Contract splicing token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "javascript:",
        "<script",
        "eval(",
        "onerror=",
        "onclick=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Agent Contract render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "circular_progress_check2_documents_agent_contract_schema_governance_rules",
        "circular_progress_agent_contract_is_schema_typed_and_machine_readable",
        "circular_progress_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "circular_progress_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "circular_progress_contract_hygiene_script_covers_agent_contract_schema_guards",
        "circular_progress_agent_contract_is_schema_typed_and_machine_readable_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep Agent Contract marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_token_first_static_style_contract_is_aggregated_and_injected_via_uiroot() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "--ui-cp-size",
        "--ui-cp-thickness",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for required in [
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should restrict runtime style payload to CSS variables `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui css aggregation should include circular-progress marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot CSS injection path should include `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_token_first_static_style_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "circular_progress_token_first_static_style_contract_is_aggregated_and_injected_via_uiroot",
        "circular_progress_token_first_style_contract_flows_from_styles_to_css_aggregation_and_uiroot",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep token-first static style contract marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_visual_desire_contract_is_guarded_by_theme_baseline_page_and_visual_regression()
 {
    let baseline_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs",
    );
    let docs_registry_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let visual_e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for required in [
        "pub(super) fn theme_visual_baseline() -> AnyView {",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.\"",
        "Playground",
        "title=\"Default Theme Visual Baseline\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>\"Primary Action\"</Button>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page_source.contains(required),
            "theme visual baseline docs page should keep marker `{required}`.",
        );
    }

    for required in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_registry_source.contains(required),
            "docs-app components registry should include theme visual baseline marker `{required}`.",
        );
    }

    for required in [
        "const visualMode = process.env.E2E_VISUAL_BASELINE ?? \"off\";",
        "test(\"docs-app: theme visual baseline renders button/input/overlay\"",
        "await page.goto(\"/#/components/theme-visual-baseline\");",
        "await expect(page.locator('[data-slot=\"theme-visual-baseline-button\"] [data-slot=\"button\"]').first()).toBeVisible();",
        "await expect(page.locator('[data-slot=\"theme-visual-baseline-input\"] [data-slot=\"input\"]').first()).toBeVisible();",
        "await expect(page.locator('[data-slot=\"overlay\"][data-state=\"open\"]').first()).toBeVisible();",
        "test(\"docs-app: theme visual baseline screenshots\"",
        "toHaveScreenshot(",
        "\"docs-app-theme-visual-baseline-page.png\"",
        "\"docs-app-theme-visual-baseline-button.png\"",
        "\"docs-app-theme-visual-baseline-input.png\"",
        "\"docs-app-theme-visual-baseline-overlay.png\"",
    ] {
        assert!(
            visual_e2e_source.contains(required),
            "visual regression spec should keep marker `{required}`.",
        );
    }

    for required in [
        "# HeroUI 参数设计风格对齐策略",
        "### Non-Goals",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_strategy_source.contains(required),
            "HeroUI strategy doc should keep non-API-copy marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_visual_desire_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "circular_progress_visual_desire_contract_is_guarded_by_theme_baseline_page_and_visual_regression",
        "circular_progress_visual_desire_baseline_is_backed_by_docs_and_playwright_screenshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep visual-desire marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes()
 {
    let cargo_source = load_ui_components_source("Cargo.toml");
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");

    for required in [
        "[features]",
        "default = [\"inject-css\", \"all-components\"]",
        "component-circular_progress = [\"dep:ui-circular-progress\"]",
        "web-demo-components = [",
        "\"component-circular_progress\"",
        "all-components = [",
    ] {
        assert!(
            cargo_source.contains(required),
            "ui Cargo features should keep tree-shaking marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]\npub use ui_circular_progress as circular_progress;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "mod web_demo_components {",
        "#[cfg(feature = \"all-components\")]",
        "mod all_components {",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]\npub use web_demo_components::*;",
        "#[cfg(feature = \"all-components\")]\npub use all_components::*;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib export surface should keep feature-gated marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(required),
            "ui css aggregation should keep prunable feature-gate marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_tree_shaking_contract_has_ci_budget_and_feature_tree_guards() {
    let tree_shaking_script =
        load_ui_components_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_ui_components_source("../../scripts/tree_shaking_budget.env");
    let ci_source = load_ui_components_source("../../.github/workflows/ci.yml");

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "source \"$BUDGET_FILE\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "if (( CURRENT_BYTES > MAX_BYTES )); then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking governance script should keep marker `{required}`.",
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(required),
            "tree-shaking budget file should keep marker `{required}`.",
        );
    }

    for required in [
        "- name: Tree Shaking Budget",
        "run: ./scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            ci_source.contains(required),
            "CI workflow should keep tree-shaking step marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script =
        load_ui_components_source("../../scripts/check-ui-tree-shaking.sh");

    for required in [
        "CIRCULAR_PROGRESS_MIN_FEATURES=\"component-circular_progress,inject-css\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CIRCULAR_PROGRESS_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p ui --no-default-features --features \"$CIRCULAR_PROGRESS_MIN_FEATURES\")\"",
        "if grep -q 'all-components' <<<\"$CIRCULAR_PROGRESS_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$CIRCULAR_PROGRESS_MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should keep circular-progress marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_tree_shaking_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes",
        "circular_progress_tree_shaking_contract_has_ci_budget_and_feature_tree_guards",
        "circular_progress_tree_shaking_contract_is_backed_by_feature_gates_and_budget_script",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep tree-shaking marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes",
        "circular_progress_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-circular_progress,inject-css",
        "scripts/check-ui-tree-shaking.sh",
        "components/circular-progress/test/semantics.rs::circular_progress_tree_shaking_feature_pruning_contract_is_gated_in_lib_css_and_script_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "circular-progress check2 should keep tree-shaking feature-pruning marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "pub struct CircularProgressStateInput",
        "pub struct CircularProgressState",
        "let state = resolve_state(CircularProgressStateInput {",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || primitive_source.contains(required),
            "circular-progress should keep typed input or semantic marker `{required}`.",
        );
    }

    for required in [
        "pub struct CircularProgressAttrs",
        "pub data_state: &'static str",
        "pub data_size_source: &'static str",
        "pub data_thickness_source: &'static str",
        "pub data_label_source: &'static str",
        "pub data_class_source: &'static str",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless contract should keep machine-readable typed attrs `{required}`.",
        );
    }

    for required in [
        "size_source_attr: if has_custom_size { \"custom\" } else { \"default\" }",
        "thickness_source_attr: if has_custom_thickness {",
        "label_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives should keep enumerable source normalization `{required}`.",
        );
    }

    for forbidden in [
        "Option<bool>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "circular-progress should avoid free-form/boolean-burst protocol `{forbidden}`.",
        );
    }

    for required in [
        "fn circular_progress_has_no_discrete_mutually_exclusive_state_axis()",
        "fn circular_progress_exposes_stable_observable_state_markers_for_selectors()",
    ] {
        assert!(
            local_semantics_source.contains(required),
            "component-local semantics suite should keep machine-readable contract marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_type_system_semantic_marker_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "circular_progress_type_system_and_semantic_markers_form_machine_readable_state_contract",
        "circular_progress_type_system_and_semantic_markers_keep_machine_readable_contract",
        "circular_progress_discrete_state_contract_is_n_a_without_free_string_or_boolean_explosion",
        "circular_progress_state_observability_contract_uses_stable_data_and_aria_markers",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep type-system+semantic-marker marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_focus_stack_contract_is_not_applicable_while_global_focus_manager_stays_in_headless()
 {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let headless_focus_trap_source =
        load_ui_components_source("../../crates/ui-headless/src/focus_trap.rs");

    for forbidden in [
        "NodeRef",
        "use_focus_trap",
        "FocusTrapOptions",
        "FOCUS_MANAGER_STACK",
        "RestorePolicy::",
        "document.body",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress component layer should not implement overlay focus-stack concern `{forbidden}`.",
        );
    }

    for required in [
        "pub enum RestorePolicy",
        "Selector(String)",
        "FallbackTo(String)",
        "FOCUS_MANAGER_STACK",
        "fn focus_manager_push_trap(",
        "fn focus_manager_pop_trap(",
        "fn restore_focus_chain(",
        "if let Some(body) = document.body() {",
    ] {
        assert!(
            headless_focus_trap_source.contains(required),
            "ui-headless global focus manager should keep marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_focus_stack_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A-by-design",
        "circular_progress_focus_stack_contract_is_not_applicable_while_global_focus_manager_stays_in_headless",
        "circular_progress_focus_stack_contract_is_n_a_and_global_focus_manager_remains_in_headless",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep focus-stack marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_escape_hatch_contract_is_not_applicable_without_imperative_third_party_integration()
 {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for forbidden in [
        "echarts",
        "ECharts",
        "mapbox",
        "leaflet",
        "amap",
        "google.maps",
        "ForeignZone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "JsValue",
        "wasm_bindgen",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !module_source.contains(forbidden),
            "CircularProgress should not embed imperative third-party escape-hatch concern `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub use ui_state_primitives::circular_progress::{",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "CircularProgress API/logic should remain pure semantic contract marker `{required}`.",
        );
    }

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_escape_hatch_contract_is_n_a_without_foreign_zone_integration()"
        ),
        "component-local semantics suite should keep escape-hatch N/A marker.",
    );
}

#[test]
fn circular_progress_check2_marks_escape_hatch_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A-by-design",
        "circular_progress_escape_hatch_contract_is_not_applicable_without_imperative_third_party_integration",
        "circular_progress_escape_hatch_contract_is_n_a_without_foreign_zone_integration",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep escape-hatch marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_hydration_discontinuity_contract_is_not_applicable_without_time_or_random_id_init()
 {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");
    let id_provider_source =
        load_ui_components_source("../../crates/ui-headless/src/id_provider.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime",
        "UNIX_EPOCH",
        "Uuid",
        "uuid::",
        "rand::",
        "thread_rng",
        "random(",
        "Math.random",
        "js_sys::Date",
        "performance.now",
        "nanoid",
        "id_base",
        "use_ui_id_provider(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !module_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "CircularProgress should not initialize runtime id/time/random source `{forbidden}`.",
        );
    }

    for required in [
        "pub struct UiIdProvider",
        "pub fn new(seed: u64) -> Self",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn next_prefixed_id(self, prefix: &str) -> String",
    ] {
        assert!(
            id_provider_source.contains(required),
            "ui-headless id provider should keep deterministic seed marker `{required}`.",
        );
    }

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_hydration_discontinuity_contract_is_n_a_without_time_random_or_uuid_init()",
        ),
        "component-local semantics suite should keep hydration-discontinuity N/A marker.",
    );
}

#[test]
fn circular_progress_check2_marks_hydration_discontinuity_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A-by-design",
        "circular_progress_hydration_discontinuity_contract_is_not_applicable_without_time_or_random_id_init",
        "circular_progress_hydration_discontinuity_contract_is_n_a_without_time_random_or_uuid_init",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep hydration-discontinuity marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_cross_platform_compile_contract_covers_default_native_ssr_and_wasm_paths() {
    let platform_script =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let headless_lib_source = load_ui_components_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_ui_components_source("../../crates/ui-headless/Cargo.toml");
    let motion_lib_source = load_ui_components_source("../../crates/ui-motion/src/lib.rs");
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "[platform] compile-only: default native path",
        "cargo check -p ui",
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile-only: circular-progress native path",
        "cargo check -p ui --no-default-features --features component-circular_progress,inject-css",
        "[platform] compile-only: circular-progress wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-circular_progress,inject-css",
        "[platform] source guard: non-wasm circular-progress files must not reference web_sys",
        "components/circular-progress/src/mod.rs",
        "components/circular-progress/src/logic.rs",
        "components/circular-progress/src/styles.rs",
        "components/circular-progress/src/view.rs",
    ] {
        assert!(
            platform_script.contains(required),
            "platform compile-only/source-guard script should keep marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "web-sys = { version = \"0.3.85\"",
    ] {
        assert!(
            headless_lib_source.contains(required) || headless_cargo_source.contains(required),
            "ui-headless platform feature guard should keep marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(required),
            "ui-motion platform stub contract should keep marker `{required}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "web-sys",
        "window()",
        "document()",
        "HtmlElement",
        "NodeRef",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "CircularProgress non-wasm source path should not rely on browser object `{forbidden}`.",
        );
    }

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_cross_platform_compile_contract_uses_explicit_cfg_and_feature_guards()",
        ),
        "component-local semantics suite should keep cross-platform compile contract marker.",
    );
}

#[test]
fn circular_progress_check2_marks_cross_platform_ssr_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "circular_progress_cross_platform_compile_contract_covers_default_native_ssr_and_wasm_paths",
        "circular_progress_cross_platform_compile_contract_uses_explicit_cfg_and_feature_guards",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep cross-platform SSR marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_ui_headless_web_ssr_mutex_contract_is_guarded_by_compile_error_and_platform_gate()
 {
    let headless_lib_source = load_ui_components_source("../../crates/ui-headless/src/lib.rs");
    let platform_script =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless should keep web/ssr mutex compile_error marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should keep ui-headless web/ssr mutex guard marker `{required}`.",
        );
    }

    assert!(
        view_source.contains(
            "use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};"
        ),
        "CircularProgress should keep ui-headless dependency mount marker in view.rs.",
    );

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_ui_headless_web_ssr_mutex_contract_is_compile_error_guarded_and_platform_checked()",
        ),
        "component-local semantics suite should keep ui-headless mutex contract marker.",
    );
}

#[test]
fn circular_progress_check2_marks_ui_headless_web_ssr_mutex_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "circular_progress_ui_headless_web_ssr_mutex_contract_is_guarded_by_compile_error_and_platform_gate",
        "circular_progress_ui_headless_web_ssr_mutex_contract_is_compile_error_guarded_and_platform_checked",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep ui-headless web/ssr mutex marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe() {
    let motion_lib_source = load_ui_components_source("../../crates/ui-motion/src/lib.rs");
    let platform_script =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(required),
            "ui-motion non-wasm stub contract should keep marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ui-motion native path",
        "cargo check -p ui-motion",
        "[platform] compile-only: ui-motion wasm path",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "[platform] ui-motion non-wasm stub tests",
        "cargo test -p ui-motion --test non_wasm_stub",
        "[platform] ui-motion reduced-motion spring contract",
        "cargo test -p ui-motion --test spring",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should keep ui-motion compile/tooling guard marker `{required}`.",
        );
    }

    for forbidden in [
        "attach_motion(",
        "MotionOptions::default()",
        "web_sys::Element",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress component layer should not assume runtime motion handle `{forbidden}`.",
        );
    }

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe_locally()",
        ),
        "component-local semantics suite should keep ui-motion non-wasm stub marker.",
    );
}

#[test]
fn circular_progress_check2_marks_ui_motion_non_wasm_stub_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "circular_progress_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe",
        "circular_progress_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep ui-motion non-wasm stub marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let headless_source =
        load_ui_components_source("../../crates/ui-headless/src/circular_progress.rs");
    let platform_script =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");

    for required in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-motion=\"spin\"]",
        "animation-duration: 1ms;",
        "animation-iteration-count: 1;",
    ] {
        assert!(
            styles_source.contains(required),
            "reduced-motion branch should keep marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: circular-progress wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-circular_progress,inject-css",
        "[platform] compile-only: ui-motion wasm path",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "[platform] circular-progress reduced-motion/ssr/wasm contract",
        "cargo test -p ui --test circular_progress_semantics circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            platform_script.contains(required),
            "platform branch coverage should keep marker `{required}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"ssr\")]",
        "#[cfg(feature = \"web\")]",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "semantic contract should not split across wasm/ssr branch by `{forbidden}`.",
        );
    }

    for required in [
        "role: \"progressbar\"",
        "aria_valuemin: \"0\"",
        "aria_valuemax: \"100\"",
        "data_state: \"indeterminate\"",
        "data_motion: \"spin\"",
    ] {
        assert!(
            headless_source.contains(required),
            "ssr/wasm semantic parity should keep headless marker `{required}`.",
        );
    }

    assert!(
        local_semantics_source.contains(
            "fn circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent_locally()",
        ),
        "component-local semantics suite should keep reduced-motion/ssr/wasm contract marker.",
    );
}

#[test]
fn circular_progress_check2_marks_reduced_motion_ssr_wasm_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep reduced-motion/ssr/wasm marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_performance_governance_budget_is_defined_and_blocking() {
    let shell_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_ui_components_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source =
        load_ui_components_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_circular_progress_component_source("check2.md");
    let todo_source = load_ui_components_source("../../docs/plan/TODO.md");
    let script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"circular-progress\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget token `{needle}`.",
        );
    }

    for needle in [
        "\"CircularProgress\"",
        "\"circular-progress\"",
        "display::circular_progress",
    ] {
        assert!(
            pages_source.contains(needle),
            "CircularProgress docs page should stay in coverage traversal via `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should keep repeatable perf marker `{needle}`.",
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
            "docs coverage e2e should keep perf guard marker `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "CircularProgress check2 should keep performance-governance marker `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_component_contract(",
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || styles_source.contains(needle),
            "state/render/style/motion attribution path should keep marker `{needle}`.",
        );
    }

    let needle = "cargo test -p ui --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(needle),
        "performance gate script should include `{needle}`.",
    );
}

#[test]
fn circular_progress_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance governance script should keep marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_performance_governance_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "circular_progress_performance_governance_budget_is_defined_and_blocking",
        "circular_progress_performance_check_script_covers_budget_and_follow_up_gates",
        "circular_progress_performance_governance_budget_is_defined_and_blocking_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep performance-governance marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics_source = load_circular_progress_component_source("test/semantics.rs");
    let aggregated_semantics_source =
        load_ui_components_source("tests/circular_progress/semantics.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let focus_trap_source = load_ui_components_source("../../crates/ui-headless/src/focus_trap.rs");
    let todo_source = load_ui_components_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots()",
        "fn circular_progress_focus_stack_contract_is_not_applicable_while_global_focus_manager_stays_in_headless()",
        "fn circular_progress_performance_governance_budget_is_defined_and_blocking()",
        "fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            aggregated_semantics_source.contains(required_test),
            "aggregated semantic/performance suite should include `{required_test}`.",
        );
    }

    for required_test in [
        "fn circular_progress_semantic_tests_prioritize_contract_markers_over_visual_snapshots()",
        "fn circular_progress_focus_stack_contract_is_n_a_and_global_focus_manager_remains_in_headless()",
        "fn circular_progress_performance_governance_budget_is_defined_and_blocking_locally()",
        "fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()",
    ] {
        assert!(
            local_semantics_source.contains(required_test),
            "component-local semantic/performance suite should include `{required_test}`.",
        );
    }

    for marker in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(marker),
            "CircularProgress view should expose semantic marker `{marker}`.",
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless global focus manager should keep focus-flow marker `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }
}

#[test]
fn circular_progress_semantics_and_performance_script_covers_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots",
        "circular_progress_focus_stack_contract_is_not_applicable_while_global_focus_manager_stays_in_headless",
        "circular_progress_performance_governance_budget_is_defined_and_blocking",
        "circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "components/circular-progress/test/semantics.rs::circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "circular-progress check2 semantic/performance section should include `{marker}`.",
        );
    }
}

#[test]
fn circular_progress_docs_page_covers_primary_playgrounds() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"CircularProgress\"",
        "slug=\"circular-progress\"",
        "Playground title=\"Hello World\"",
        "let hello_world_code = Signal::derive(move || r#\"<CircularProgress />\"#.to_string());",
        "Playground title=\"Size + Thickness Matrix\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for CircularProgress.",
        );
    }
}

#[test]
fn circular_progress_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Size + Thickness Matrix\"",
        "<CircularProgress aria_label=\"Loading\".to_string() />",
        "<CircularProgress aria_label=\"Syncing mail\".to_string() size_px=24.0 />",
        "<CircularProgress aria_label=\"Syncing mail\".to_string() thickness_px=3.0 />",
        "size_px=30.0",
        "thickness_px=4.0",
        "title=\"Custom Label + Class\"",
        "aria_label=\"Background refresh\".to_string()",
        "size_px=28.0",
        "thickness_px=3.5",
        "aria_label=\"   \".to_string()",
        "class_name=\"docs-circular-progress-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "circular-progress docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot()
 {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Size + Thickness Matrix\"",
        "title=\"Custom Label + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::CircularProgress;\"",
        "data-slot=\"circular-progress-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
        "data-slot=\"circular-progress-copy-ready-hint\"",
    ] {
        assert!(
            source.contains(needle),
            "circular-progress docs should keep docs-product copy-paste-ready marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source =
        load_ui_components_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"circular-progress-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"circular-progress-source-first-contract\"",
        "data-slot=\"circular-progress-source-prerequisites\"",
        "component-circular_progress",
        "inject-css",
        "UiRoot",
        "Copy circular-progress starter",
        "docs-circular-progress-source-copy",
        "data-slot=\"circular-progress-source-paths\"",
        "components/circular-progress/src/mod.rs",
        "components/circular-progress/src/logic.rs",
        "components/circular-progress/src/view.rs",
        "components/circular-progress/src/styles.rs",
        "data-slot=\"circular-progress-source-sync-note\"",
        "source_first_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_docs_product_copy_paste_ready_rules",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce circular-progress docs-product guard `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_docs_product_copy_paste_ready_rules() {
    let source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World / Size + Thickness Matrix / Custom Label + Class / Controlled vs Uncontrolled (N/A) / Streaming Optional / Snapshot / Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
        "component-circular_progress",
        "circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "circular_progress_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "circular-progress check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "circular_progress_check2_documents_docs_product_copy_paste_ready_rules",
        "circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "circular_progress_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            source.contains(needle),
            "circular-progress check2 should keep docs-product completion marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source =
        load_ui_components_source("../../components/code-block/src/view.rs");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"circular-progress-source-first\"",
        "data-slot=\"circular-progress-source-first-contract\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"circular-progress-source-prerequisites\"",
        "component-circular_progress",
        "inject-css",
        "UiRoot",
        "Copy circular-progress starter",
        "docs-circular-progress-source-copy",
        "data-slot=\"circular-progress-source-paths\"",
        "components/circular-progress/src/mod.rs",
        "components/circular-progress/src/logic.rs",
        "components/circular-progress/src/view.rs",
        "components/circular-progress/src/styles.rs",
        "data-slot=\"circular-progress-source-sync-note\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: circular-progress source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include source-first marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "circular-progress check2 should mark source-first copy-paste-ready item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "circular_progress_check2_documents_source_first_copy_paste_ready_rules",
        "circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "circular_progress_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_source_first_copy_paste_ready_contract_is_documented_and_scripted_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 heroui benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let readme_source = load_circular_progress_component_source("src/README.md");

    for needle in [
        "### CircularProgress 同步记录（2026-02-20）",
        "`CircularProgress` 参数主轴保持 `aria_label/size_px/thickness_px/class_name/lang/dir`",
        "component_doc!(\"CircularProgress\", \"circular-progress\", \"Display\", display::circular_progress)",
        "display.rs::circular_progress() 已覆盖 `Hello World`、`Size + Thickness Matrix`、`Custom Label + Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Playground (Props / State / Preview)` 与 `Source-first Starter (Copy-Paste Ready)`",
        "data-slot=\"circular-progress-source-first\"",
        "component-circular_progress + inject-css + UiRoot",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "circular-progress HeroUI strategy docs should contain `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"CircularProgress\"",
        "\"circular-progress\"",
        "display::circular_progress",
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"CircularProgress\"",
        "slug=\"circular-progress\"",
    ] {
        assert!(
            docs_registry_source.contains(needle) || docs_display_source.contains(needle),
            "circular-progress docs entry should keep indexable marker `{needle}`.",
        );
    }

    for needle in [
        "# CircularProgress",
        "## Hello World（先用起来）",
        "## docs-app 入口",
        "/#/components/circular-progress",
    ] {
        assert!(
            readme_source.contains(needle),
            "circular-progress README should keep docs-product marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: circular-progress heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include heroui benchmark docs-sync marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"
        ),
        "circular-progress check2 should mark heroui benchmark docs-sync item complete.",
    );

    for needle in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "components/circular-progress/src/README.md",
        "circular_progress_check2_documents_heroui_benchmark_docs_sync_rules",
        "circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_heroui_benchmark_docs_sync_contract_is_documented_and_scripted_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 heroui benchmark docs-sync section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "circular-progress check2 should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/circular_progress.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional)] size_px: Option<f64>,",
        "#[prop(optional)] thickness_px: Option<f64>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "default_aria_label: common.loading_aria_label.as_ref(),",
    ] {
        assert!(
            view_source.contains(needle),
            "circular-progress view API contract should keep `{needle}` for docs sync.",
        );
    }

    for needle in [
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
        "resolve_aria_label(input.aria_label, default_aria_label);",
        "let lang = normalize_optional_text(input.lang);",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic_source.contains(needle),
            "circular-progress logic default/normalization contract should keep `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Loading\";",
        "pub fn sanitize_dimension(value: Option<f64>) -> Option<f64> {",
        "size_source_attr: if has_custom_size { \"custom\" } else { \"default\" },",
        "thickness_source_attr: if has_custom_thickness {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "circular-progress primitive default/source contract should keep `{needle}`.",
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"Size + Thickness Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "data-slot=\"circular-progress-docs-sync-matrix\"",
        "<h3>\"State Matrix\"</h3>",
        "<h3>\"Parameter Matrix\"</h3>",
        "data-size-source / data-thickness-source / data-label-source / data-class-source",
        "default = None；`logic.rs::resolve_component_contract`",
        "DEFAULT_ARIA_LABEL",
        "size_px / thickness_px: Option&lt;f64&gt;",
        "finite 且 > 0",
        "class_name / lang: Option&lt;String&gt;",
        "normalize_optional_text",
        "dir: Option&lt;A11yDirection&gt;",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress docs should keep synced example/matrix marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 docs-sync evidence should include `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: circular-progress docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include circular-progress docs-sync/state-matrix marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    assert!(
        checklist_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "circular-progress check2 should mark docs-sync/state-matrix checklist item complete.",
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "data-slot=\"circular-progress-docs-sync-matrix\"",
        "circular_progress_check2_documents_docs_sync_and_state_matrix_rules",
        "circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "circular_progress_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_docs_sync_and_state_matrix_contract_is_documented_and_scripted_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "circular-progress check2 docs-sync/state-matrix section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "circular-progress check2 should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_circular_progress_component_source("src/README.md");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "# CircularProgress",
        "## Hello World（先用起来）",
        "## 常见用法",
        "## 进阶用法（需要时再看）",
        "## docs-app 入口",
        "<CircularProgress />",
        "size_px=24.0",
        "thickness_px=3.0",
        "class_name=\"docs-circular-progress-custom\".to_string()",
        "dir=A11yDirection::Rtl",
        "pages/components/pages/display.rs` 的 `circular_progress()`",
        "/#/components/circular-progress",
    ] {
        assert!(
            readme_source.contains(needle),
            "circular-progress README should include beginner-first documentation marker `{needle}`.",
        );
    }

    let hello_index = readme_source
        .find("## Hello World（先用起来）")
        .unwrap_or_else(|| panic!("README should contain Hello World section."));
    let common_index = readme_source
        .find("## 常见用法")
        .unwrap_or_else(|| panic!("README should contain common-usage section."));
    let advanced_index = readme_source
        .find("## 进阶用法（需要时再看）")
        .unwrap_or_else(|| panic!("README should contain advanced section."));
    assert!(
        hello_index < common_index && common_index < advanced_index,
        "circular-progress README should keep beginner-first order: Hello -> common -> advanced.",
    );

    for needle in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Size + Thickness Matrix\"",
        "title=\"Custom Label + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress docs-app entry should include `{needle}`.",
        );
    }

    assert!(
        check2_source.contains("components/circular-progress/src/README.md")
            && check2_source
                .contains("apps/docs-app/src/pages/components/pages/display.rs::circular_progress"),
        "circular-progress check2 documentation-as-product evidence should reference README and docs entry path.",
    );
}

#[test]
fn circular_progress_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: circular-progress documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_documentation_entry_exists_with_beginner_first_progression",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_documentation_as_product_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_documentation_as_product_item_complete() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    assert!(
        checklist_source.contains(
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"
        ),
        "circular-progress check2 should mark documentation-as-product checklist item complete.",
    );

    for needle in [
        "components/circular-progress/src/README.md",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "circular_progress_check2_documents_documentation_as_product_rules",
        "circular_progress_documentation_entry_exists_with_beginner_first_progression",
        "circular_progress_dx_check_script_covers_documentation_as_product_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_documentation_as_product_contract_is_documented_and_scripted_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "circular-progress check2 documentation-as-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_interactive_playground_rules() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "circular-progress check2 should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Interactive Playground (Props / State / Preview)\"",
        "data-slot=\"circular-progress-workbench-controls\"",
        "data-slot=\"circular-progress-workbench-preview\"",
        "data-slot=\"circular-progress-workbench-state\"",
        "data-slot=\"circular-progress-workbench-size-24\"",
        "data-slot=\"circular-progress-workbench-thickness-3\"",
        "data-slot=\"circular-progress-workbench-label-custom\"",
        "data-slot=\"circular-progress-workbench-class-custom\"",
        "data-slot=\"circular-progress-workbench-dir-rtl\"",
        "size_source={size_source}; thickness_source={thickness_source}; label_source={label_source}; class_source={class_source}; dir={dir_label}",
        "test_config_signal=workbench_config",
    ] {
        assert!(
            docs_source.contains(needle),
            "circular-progress docs interactive playground should keep marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");

    for needle in [
        "docs-app circular-progress interactive playground updates props and semantic markers",
        "data-slot=\"circular-progress-workbench-controls\"",
        "data-slot=\"circular-progress-workbench-preview\"",
        "circular-progress-workbench-size-24",
        "circular-progress-workbench-thickness-3",
        "circular-progress-workbench-label-custom",
        "circular-progress-workbench-class-custom",
        "circular-progress-workbench-dir-rtl",
        "replay flow after remount remains deterministic",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "circular-progress interactive e2e flow should include `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "toHaveScreenshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "circular-progress interactive e2e flow should avoid flaky token `{forbidden}`.",
        );
    }

    assert!(
        docs_source.contains("data-slot=\"circular-progress-workbench-controls\""),
        "interactive playground e2e flow should be anchored on docs-app semantic slots.",
    );
}

#[test]
fn circular_progress_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: circular-progress interactive playground docs acceptance surface\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_interactive_playground_item_complete() {
    let checklist_source = load_circular_progress_component_source("check2.md");

    assert!(
        checklist_source.contains(
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"
        ),
        "circular-progress check2 should mark interactive-playground checklist item complete.",
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "data-slot=\"circular-progress-workbench-controls\"",
        "e2e/tests/docs_app_circular_progress_contract.spec.mjs::docs-app circular-progress interactive playground updates props and semantic markers",
        "circular_progress_check2_documents_interactive_playground_rules",
        "circular_progress_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "circular_progress_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "circular_progress_dx_check_script_covers_interactive_playground_contract",
        "components/circular-progress/test/semantics.rs::circular_progress_interactive_playground_contract_is_documented_and_scripted_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "circular-progress check2 interactive-playground section should reference `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "CircularProgress should keep a single explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "CircularProgress should keep one compact `view!` block for current simple indicator layout.",
    );
    assert!(
        view_source.lines().count() <= 120,
        "CircularProgress view.rs should stay compact; split semantic subrenders only if layout complexity grows.",
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        "match children",
        "#[component]\nfn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress view should avoid loop-heavy or macro-expansion-heavy pattern `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn circular_progress_view_macro_check_script_covers_complexity_gate() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");
    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(needle),
        "view-macro gate script should include `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_view_macro_complexity_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
        "circular_progress_view_macro_check_script_covers_complexity_gate",
        "circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep view-macro-complexity marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "CircularProgress should keep a single public component boundary for current simple layout.",
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn circular_progress_",
        "pub fn render_",
        "fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress should not introduce local component/render API noise `{forbidden}`.",
        );
    }

    for required in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress view should keep stable semantic markers after functional split decisions `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn circular_progress_view_macro_check_script_covers_functional_split_gate() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");
    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script_source.contains(needle),
        "view-macro gate script should include `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_view_functional_split_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout",
        "circular_progress_view_macro_check_script_covers_functional_split_gate",
        "circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep view-functional-split marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "<svg",
        "<path",
        "let markdown",
        "let description_text",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress should avoid heavy static fragment construction token `{forbidden}`.",
        );
    }

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress should keep stable a11y/state markers while static fragments stay absent `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn circular_progress_view_macro_check_script_covers_static_fragment_gate() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-view-macro.sh");
    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout";
    assert!(
        script_source.contains(needle),
        "view-macro gate script should include `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_static_fragment_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout",
        "circular_progress_view_macro_check_script_covers_static_fragment_gate",
        "circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep static-fragment marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        let source = load_circular_progress_component_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "CircularProgress source `{rel_path}` must not contain raw-html injection token `{forbidden}`.",
            );
        }
    }

    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "CircularProgress docs examples must not contain raw-html injection token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_inner_html_check_script_covers_security_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-inner-html.sh");
    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_inner_html_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "circular_progress_inner_html_check_script_covers_security_contract",
        "circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep inner-html marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let cargo_source = load_ui_components_source("Cargo.toml");
    let crate_root_source = load_ui_components_source("src/lib.rs");
    let docs_app_source = load_ui_components_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source =
        load_ui_components_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_ui_components_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`.",
        );
    }

    for forbidden in [
        "circular-progress-wasm-debug",
        "circular_progress-wasm-debug",
        "component-circular_progress-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "CircularProgress should not expose component-local wasm-debug feature `{forbidden}`.",
        );
    }

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ndev-all-components")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before dev-all-components declaration");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm-debug feature must stay out of all-components production path.",
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep shared wasm-debug isolation marker `{needle}`.",
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs app should keep wasm-debug visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace timeline marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "Note {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep typed timestamp/source marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "CircularProgress should keep machine-readable state/source marker `{needle}` for wasm-debug traceability.",
        );
    }

    // Non-interactive display component: replay path is N/A-by-design.
    for forbidden in [
        "on:click=",
        "on:input=",
        "on:pointerdown=",
        "on:pointerup=",
        "on:keydown=",
        "on:keyup=",
        "request_replay",
        "emit_selection_trace(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress replay path should stay N/A-by-design without interaction handler `{forbidden}`.",
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress component runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_wasm_debug_check_script_covers_shared_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-wasm-debug.sh");
    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_wasm_debug_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
        "circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "circular_progress_wasm_debug_check_script_covers_shared_contract",
        "circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep wasm-debug marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_ui_components_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na() {
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "pub(super) fn circular_progress() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Size + Thickness Matrix\" code_signal=matrix_code>",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
        "slug=\"circular-progress\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "CircularProgress docs should provide isolated playground/demo entry `{needle}`.",
        );
    }

    for forbidden in [
        "WORKBENCH_STORAGE_KEY",
        "load_circular_progress_workbench_",
        "save_circular_progress_workbench_",
        "clear_circular_progress_workbench_",
        "Persist workbench state",
        "test_config_signal=",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "CircularProgress has no interactive state-machine context to persist; workbench persistence is N/A and `{forbidden}` should remain absent.",
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "CircularProgress view should remain non-interactive display primitive; context-persist interaction token `{forbidden}` is N/A.",
        );
    }

    for needle in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "CircularProgress checklist should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_dx_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "circular_progress_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
        "circular_progress_dx_check_script_covers_hot_reload_and_isolated_canvas_contract",
        "circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep DX marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let cargo_source = load_ui_components_source("Cargo.toml");
    let mod_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let checklist_source = load_circular_progress_component_source("check2.md");

    assert!(
        !workspace_dir
            .join("components/circular-progress/src/spec.rs")
            .exists(),
        "CircularProgress should keep spec/schema boundary as N/A for simple component scope.",
    );
    assert!(
        cargo_source.contains("component-circular_progress = [\"dep:ui-circular-progress\"]"),
        "CircularProgress feature should stay lightweight without serde/spec dependency fan-out.",
    );
    assert!(
        !cargo_source.contains("component-circular_progress = [\"dep:serde\"")
            && !cargo_source.contains("component-circular_progress = [\"dep:serde_json\""),
        "CircularProgress should not opt into serde/spec migration dependencies without an explicit schema contract.",
    );

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");
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
            !combined.contains(forbidden),
            "CircularProgress engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "CircularProgress checklist should keep engineering governance rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_ui_components_source("Cargo.toml");
    let button_view_source = load_ui_components_source("src/button/view.rs");
    let combined = [
        load_circular_progress_component_source("src/mod.rs"),
        load_circular_progress_component_source("src/logic.rs"),
        load_circular_progress_component_source("src/view.rs"),
        load_circular_progress_component_source("src/styles.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    for forbidden in [
        "circular-progress-wasm-debug",
        "circular_progress-wasm-debug",
        "component-circular_progress-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "CircularProgress should not define component-local tracing feature `{forbidden}`.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::circular_progress::",
        "const CIRCULAR_PROGRESS_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "CircularProgress should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "CircularProgress engineering contract should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "CircularProgress public module boundary should not leak web_sys types.",
    );
}

#[test]
fn circular_progress_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_circular_progress_component_source("src/Component.toml");
    let rbi_source = load_circular_progress_component_source("src/circular_progress.rbi");
    let mod_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CircularProgress\"",
        "crate = \"ui-circular-progress\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "circular-progress manifest should keep stable v1 schema marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn CircularProgress(",
        "pub enum CircularProgressAgentSchemaVersion {",
        "V1,",
        "pub schema_version: CircularProgressAgentSchemaVersion,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "circular-progress RBI should keep stable public API marker `{needle}`.",
        );
    }

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "circular-progress should not introduce major-version migration marker `{forbidden}` in current scope.",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CircularProgress` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress/check2.md should keep version-migration governance marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-engineering.sh");

    let marker = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`.",
    );
}

#[test]
fn circular_progress_check2_marks_version_deprecation_migration_item_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for marker in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "circular_progress_version_deprecation_migration_script_covers_engineering_gate",
        "components/circular-progress/test/semantics.rs::circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally",
    ] {
        assert!(
            check2_source.contains(marker),
            "circular-progress check2 version-migration section should include `{marker}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_engineering_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "circular_progress_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "circular_progress_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "circular_progress_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "circular_progress_engineering_check_script_covers_serde_tracing_and_runtime_boundaries",
        "circular_progress_engineering_contract_is_spec_free_tracing_aligned_and_runtime_agnostic_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep engineering marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let theme_css_source = load_ui_components_source("../ui-theme/src/css.rs");

    for required in [
        "var(--ui-cp-size, var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size)))",
        "var(--ui-cp-thickness, var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border)))",
        "var(--ui-cp-rotation-duration,",
        "var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
    ] {
        assert!(
            styles_source.contains(required),
            "CircularProgress styles should keep defensive token fallback chain `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-button-spinner-size:",
        "--ui-fallback-button-spinner-border:",
        "--ui-fallback-button-spinner-duration:",
        "--ui-fallback-button-radius-full:",
        "--ui-fallback-border:",
        "--ui-fallback-accent:",
        "--ui-fallback-fg:",
        "--ui-fallback-border-width:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css output should provide CircularProgress fallback variable `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-button-spinner-size, 16px)",
        "var(--ui-button-spinner-border, 2px)",
        "var(--ui-button-spinner-duration, 800ms)",
        "border-radius: 9999px;",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "CircularProgress styles should not keep raw terminal token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_defensive_variables_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "circular_progress_styles_use_defensive_variable_fallback_chain",
        "circular_progress_defensive_variables_check_script_covers_style_fallback_contract",
        "circular_progress_styles_use_defensive_variable_fallback_chain_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep defensive-variables marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_ui_components_source("src/css.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should keep cascade-layer contract marker `{needle}`.",
        );
    }

    for needle in [
        "style=style_vars",
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "CircularProgress runtime style path should stay css-variable-only via `{needle}`.",
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should avoid plain inline style token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "circular_progress_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "circular-progress checklist should keep cascade-layer/runtime-style evidence `{required}`.",
        );
    }
}

#[test]
fn circular_progress_cascade_layer_check_script_covers_layer_and_inline_style_guard() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_cascade_layer_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "circular_progress_cascade_layer_and_runtime_style_contract_is_enforced",
        "circular_progress_cascade_layer_check_script_covers_layer_and_inline_style_guard",
        "circular_progress_cascade_layer_and_runtime_style_contract_is_enforced_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep cascade-layer marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards()
 {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let motion_path = workspace_dir.join("components/circular-progress/src/motion.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let ui_motion_source = load_ui_components_source("../../crates/ui-motion/src/lib.rs");

    assert!(
        !motion_path.exists(),
        "CircularProgress motion.rs should stay N/A-by-design for display-only runtime-attach path.",
    );

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "attach_motion(",
        "MotionOptions",
        "spring",
        "stiffness",
        "damping",
        "ui_motion::web::animate(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CircularProgress should avoid runtime motion-attach token `{forbidden}`.",
        );
    }

    for required in [
        "@media (prefers-reduced-motion: reduce)",
        "animation-duration: 1ms;",
        "animation-iteration-count: 1;",
        "data-motion=\"spin\"",
    ] {
        assert!(
            styles_source.contains(required),
            "CircularProgress styles should keep reduced-motion contract marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should keep non-wasm no-op contract marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_motion_contract_check_script_covers_reduced_motion_and_noop_guards() {
    let script_source = load_ui_components_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "cargo test -p ui --test circular_progress_semantics circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_motion_contractualization_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A-by-design",
        "circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
        "circular_progress_motion_contract_check_script_covers_reduced_motion_and_noop_guards",
        "circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep motion-contractualization marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");
    let active_highlight_source =
        load_ui_components_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_ui_components_source("../ui-headless/src/controllable_state.rs");
    let presence_source = load_ui_components_source("../ui-headless/src/presence.rs");
    let a11y_source = load_ui_components_source("../ui-headless/src/a11y.rs");
    let ui_components_src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]",
        "pub use ui_circular_progress as circular_progress;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["web_sys::", "web-sys", "HtmlElement", "NodeRef", "JsValue"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "ui root.rs should keep centralized injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight shared primitive should contain `{required}`.",
        );
    }

    for forbidden in ["CircularProgress", "aria-", "data-state"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`.",
        );
    }

    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state_source.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`.",
        );
    }
    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence_source.contains(required),
            "ui-headless presence canonical path should contain `{required}`.",
        );
    }
    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y_source.contains(required),
            "ui-headless a11y canonical path should contain `{required}`.",
        );
    }
}

#[test]
fn circular_progress_entrypoints_check_script_covers_fixed_entry_files_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries",
        "circular_progress_entrypoints_check_script_covers_fixed_entry_files_contract",
        "circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep fixed-entry-files marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src = workspace_dir.join("components/circular-progress/src");

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src.join(required_file).exists(),
            "CircularProgress component directory should include `{required_file}`.",
        );
    }
    for absent_file in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !component_src.join(absent_file).exists(),
            "CircularProgress component directory should keep `{absent_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "mod spec;",
        "mod render;",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalized state derivation marker `{required}`.",
        );
    }
    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "NodeRef",
        "HtmlElement",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay free of DOM/platform token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid render/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "view! {",
        "data-state=semantics.attrs.data_state",
        "role=semantics.attrs.role",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in [
        "resolve_state(CircularProgressStateInput {",
        "logic::resolve_aria_label(",
        "@keyframes",
        ".ui-circular-progress {",
        "web_sys::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid hidden state/styling/platform token `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_component_files_check_script_covers_standard_directory_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_marks_component_directory_standard_files_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "N/A-by-design",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths",
        "circular_progress_component_files_check_script_covers_standard_directory_contract",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep component-directory-standard marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_file_placement_discipline_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "N/A-by-design",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths",
        "circular_progress_component_files_check_script_covers_standard_directory_contract",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress check2 should keep file-placement-discipline marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(required),
            "CircularProgress checklist should keep streaming-definition rule `{required}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_definition_contract_is_snapshot_only_and_protocol_free() {
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");

    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-output-status",
        "project_streaming_",
        "streaming",
        "Streaming",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "CircularProgress should stay non-streaming in component scope and avoid `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_check_script_covers_two_mode_definition_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn circular_progress_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_circular_progress_component_source("check2.md");
    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "CircularProgress checklist should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_circular_progress_component_source("src/view.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let docs_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for marker in [
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional)] size_px: Option<f64>,",
        "#[prop(optional)] thickness_px: Option<f64>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "aria_label,",
        "size_px,",
        "thickness_px,",
        "class_name,",
        "lang,",
        "default_aria_label: common.loading_aria_label.as_ref(),",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "style=style_vars",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-ui-schema=agent_contract.schema_name",
    ] {
        assert!(
            view_source.contains(marker),
            "CircularProgress snapshot baseline should keep complete-input render marker `{marker}`.",
        );
    }

    for marker in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "let lang = normalize_optional_text(input.lang);",
        "let class_name = normalize_optional_text(input.class_name);",
        "let default_aria_label = resolve_default_aria_label(input.default_aria_label);",
        "let (aria_label, has_custom_aria_label) =",
        "let state = resolve_state(CircularProgressStateInput {",
        "let class = compose_class_name(class_name, &state);",
        "let style_vars = compose_style_vars(&state);",
    ] {
        assert!(
            logic_source.contains(marker),
            "CircularProgress logic should keep deterministic snapshot normalization marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn circular_progress() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Size + Thickness Matrix\" code_signal=matrix_code>",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
        "<CircularProgress />",
        "aria_label=\"Background refresh\".to_string()",
        "size_px=28.0",
        "thickness_px=3.5",
        "class_name=\"docs-circular-progress-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "CircularProgress docs should keep complete snapshot configuration marker `{marker}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "输出状态（草稿/已验证/可提交）由上层 LLM 容器决策并透传",
        "数据校验、断线恢复、重试策略保持在上层，不下沉到组件。",
    ] {
        assert!(
            check2_source.contains(required),
            "CircularProgress checklist should keep streaming responsibility marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_circular_progress_component_source("src/view.rs");

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-slot=\"circular-progress\"",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-state=agent_contract.state.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "CircularProgress view should keep semantic continuity marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()
{
    let module_source = load_circular_progress_component_source("src/mod.rs");
    let logic_source = load_circular_progress_component_source("src/logic.rs");
    let view_source = load_circular_progress_component_source("src/view.rs");
    let styles_source = load_circular_progress_component_source("src/styles.rs");
    let check2_source = load_circular_progress_component_source("check2.md");

    for forbidden in [
        "retry",
        "backoff",
        "reconnect",
        "断线恢复",
        "is_loading",
        "error",
        "data-ui-output-status",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "CircularProgress component layer should not own streaming/retry boundary token `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("数据校验、断线恢复、重试策略保持在上层，不下沉到组件。"),
        "CircularProgress checklist should keep upper-layer retry/resilience boundary statement.",
    );
}

#[test]
fn circular_progress_streaming_check_script_covers_required_optional_classification_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
 {
    for rel_path in ["src/mod.rs", "src/logic.rs", "src/view.rs", "src/styles.rs"] {
        let source = load_circular_progress_component_source(rel_path);
        for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "CircularProgress non-test source `{rel_path}` should not contain `{forbidden}`.",
            );
        }
    }
}

#[test]
fn circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_circular_progress_component_source("src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-circular-progress\")",
        "Cow::Borrowed(\"ui-circular-progress--state-indeterminate\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "CircularProgress logic should keep Cow-based class composition marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-circular-progress\".to_string()",
        "\"ui-circular-progress--state-indeterminate\".to_string()",
        "\"ui-circular-progress--size-custom\".to_string()",
        "\"ui-circular-progress--thickness-custom\".to_string()",
        "\"ui-circular-progress--label-custom\".to_string()",
        "\"ui-circular-progress--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "CircularProgress logic should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let hygiene_script = load_ui_components_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script =
        load_ui_components_source("../../scripts/check-ui-engineering.sh");

    for required in [
        "forbidden unwrap/expect in non-test code",
        "forbidden let _ = in non-test code",
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
        "[rust-hygiene] OK",
    ] {
        assert!(
            hygiene_script.contains(required),
            "rust-hygiene script should keep marker `{required}`.",
        );
    }

    for required in [
        "circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering script should enforce circular-progress rust-hygiene gate `{required}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_circular_progress_component_source("check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "components/circular-progress/test/semantics.rs::circular_progress_rust_hygiene_contract_forbids_unwrap_expect_let_underscore_and_converges_hotspots_to_cow_locally",
        "./scripts/check-rust-hygiene.sh",
        "Vec<Cow<'static, str>>",
    ] {
        assert!(
            check2_source.contains(required),
            "CircularProgress checklist should keep rust-hygiene marker `{required}`.",
        );
    }
}
