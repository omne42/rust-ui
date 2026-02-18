use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

fn select_docs_section() -> String {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    docs_source
        .split("pub(super) fn select() -> AnyView {")
        .nth(1)
        .map(|tail| {
            tail.split("\npub(super) fn ")
                .next()
                .unwrap_or(tail)
                .to_string()
        })
        .unwrap_or_else(|| panic!("collections docs page should define select() section"))
}

#[test]
fn select_does_not_expose_logic_module() {
    let source = load_source("src/select/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Select's `logic` module should stay private to avoid leaking internal behavior helpers into the public API."
    );
}

#[test]
fn select_uses_logic_state_model() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");

    for needle in [
        "pub use ui_state_primitives::select::{",
        "SelectStateInput",
        "SelectState",
        "resolve_state",
        "resolve_horizontal_nav_target",
        "find_typeahead_match",
        "typeahead_char",
    ] {
        assert!(
            logic_source.contains(needle),
            "Select logic should consume primitive exports via `{needle}`.",
        );
    }

    for needle in [
        "pub struct SelectStateInput",
        "pub struct SelectState",
        "pub fn normalize_id_base(",
        "pub fn resolve_placeholder(",
        "pub fn resolve_disabled_option_count(",
        "pub fn resolve_state(input: SelectStateInput)",
        "pub fn compose_class_name(",
        "pub class_source_attr: &'static str",
        "pub motion_source_attr: &'static str",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Select primitive layer should include `{needle}` for centralized state derivation.",
        );
    }

    for needle in [
        "let state = Signal::derive(move ||",
        "logic::resolve_state(logic::SelectStateInput {",
        "selected_index: selected_index.get()",
        "let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view should derive root state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn select_status_primitives_layer_is_pure_and_component_only_consumes() {
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for forbidden in [
        "leptos",
        "web_sys",
        "NodeRef",
        "view!",
        ".ui-select",
        "var(--ui-",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives select must stay DOM/style/framework free; found `{forbidden}`.",
        );
    }

    for forbidden in [
        "pub struct SelectStateInput {",
        "pub struct SelectState {",
        "pub fn resolve_state(",
        "pub fn find_typeahead_match(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "select logic layer should not redefine primitive `{forbidden}`.",
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod select;"),
        "ui-state-primitives should export `select` module.",
    );
    assert!(
        primitive_source.contains("#[cfg(test)]"),
        "ui-state-primitives select should own unit tests.",
    );

    for needle in [
        "logic::resolve_state(logic::SelectStateInput {",
        "logic::resolve_disabled_option_count(disabled_set.get_value().as_ref(), item_count);",
        "logic::resolve_horizontal_nav_target(",
        "logic::find_typeahead_match(",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should consume primitive-derived logic via `{needle}`.",
        );
    }
}

#[test]
fn select_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: Option<bool>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "motion: SelectMotion",
        "class_name: Option<String>",
    ] {
        assert!(
            source.contains(needle),
            "Select should accept `{needle}` to support controlled/uncontrolled open state."
        );
    }
}

#[test]
fn select_api_naming_uses_prefixed_boolean_props_with_legacy_alias_migration_path() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: Option<bool>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "select api naming should include `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool",
        "is_disabled.or(disabled).unwrap_or(false)",
    ] {
        assert!(
            logic_source.contains(needle),
            "select logic should centralize prefixed naming priority via `{needle}`.",
        );
    }

    for needle in [
        "let is_disabled = logic::normalize_is_disabled(is_disabled, disabled);",
        "disabled: is_disabled,",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should consume normalized disabled axis via `{needle}`.",
        );
    }

    for needle in [
        "id_base=\"docs-select-disabled\".to_string()",
        "is_disabled=true",
        "on_open_change=on_open_change",
    ] {
        assert!(
            docs_source.contains(needle),
            "select docs should demonstrate prefixed naming via `{needle}`.",
        );
    }
}

#[test]
fn select_headless_contracts_drive_interaction_and_a11y_boundaries() {
    let view_source = load_source("src/select/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "use ui_headless as overlay_open;",
        "use ui_headless::{",
        "A11yDirection",
        "PopoverPlacement",
        "locale_attrs",
        "use_presence",
        "overlay_open::use_controllable_open_state_traced(",
        "use_presence(open)",
        "ui_headless::aria_controls_when_open(open, listbox_id.get_value())",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "select should consume headless interaction/a11y contract `{needle}`.",
        );
    }

    for forbidden in ["query_selector(", "classList", "style.set_property("] {
        assert!(
            !view_source.contains(forbidden),
            "select view should not reimplement raw DOM semantic orchestration via `{forbidden}`.",
        );
    }

    for forbidden in [
        ".ui-select",
        "animation:",
        "transition:",
        "spring",
        "keyframe",
    ] {
        assert!(
            !headless_a11y_source.contains(forbidden),
            "ui-headless a11y layer must stay semantic-only; found `{forbidden}`.",
        );
    }

    for needle in [
        "pub mod a11y;",
        "pub mod controllable_state;",
        "pub mod presence;",
        "locale_attrs",
        "aria_controls_when_open",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless export surface should include `{needle}` for select contracts.",
        );
    }
}

#[test]
fn select_trigger_is_labeled_and_owns_a_listbox() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=input.trigger_id",
    ] {
        assert!(
            source.contains(needle),
            "Select should wire `{needle}` for baseline-style listbox trigger semantics."
        );
    }
}

#[test]
fn select_uses_presence_to_allow_exit_motion() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "use_presence(open)",
        "motion=input.motion.popover",
        "on_exit_complete=input.presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Select should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn select_exposes_root_state_and_slot_data_attributes() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "data-slot=SLOT_SELECT",
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-closed=move || state.get().is_closed.then_some(BOOL_TRUE)",
        "data-disabled=move || state.get().trigger_disabled.then_some(BOOL_TRUE)",
        "data-component-disabled=move || state.get().is_disabled.then_some(BOOL_TRUE)",
        "data-empty=move || state.get().is_empty.then_some(BOOL_TRUE)",
        "data-has-items=move || state.get().has_items.then_some(BOOL_TRUE)",
        "data-count=move || state.get().item_count.to_string()",
        "data-has-selection=move || state.get().has_selection.then_some(BOOL_TRUE)",
        "data-selection-empty=move || state.get().selection_empty.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(BOOL_TRUE)",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(BOOL_TRUE)",
        "data-slot=SLOT_SELECT_PANEL",
    ] {
        assert!(
            source.contains(needle),
            "Select should expose `{needle}` for baseline-style state styling and regression tests."
        );
    }
}

#[test]
fn select_styles_include_source_marker_selectors() {
    let source = load_source("src/select/styles.rs");

    for needle in [
        ".ui-select[data-class-source=\"custom\"]",
        ".ui-select[data-custom-class=\"true\"]",
        ".ui-select--custom-class",
        ".ui-select[data-motion-source=\"custom\"]",
        ".ui-select[data-custom-motion=\"true\"]",
        ".ui-select--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "Select styles should include `{needle}` for stable source-marker contracts."
        );
    }
}

#[test]
fn select_centralizes_trigger_disabled_logic() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");

    for needle in [
        "resolve_trigger_disabled",
        "disabled=trigger_disabled",
        "if trigger_disabled {",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view should centralize trigger disabled semantics via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("resolve_trigger_disabled"),
        "Select logic should re-export disabled/empty trigger helper from primitives."
    );
}

#[test]
fn select_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/select/mod.rs");
    let motion_source = load_source("src/select/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::SelectMotion;",
        "pub struct SelectMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Select motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn select_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/select/motion.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Select motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::select::motion::sanitize_motion(motion);"),
        "Select view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn select_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn select() -> AnyView",
        "title=\"Select\"",
        "slug=\"select\"",
        "description=\"Select with controlled open state, listbox semantics, and baseline-style root state attrs.\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Controlled Open + Selection\" code_signal=code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
        "<Select",
        "id_base=\"docs-select-hello\".to_string()",
        "open=controlled_open",
        "is_disabled=true",
        "placeholder=\"No options\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for select coverage.",
        );
    }
}

#[test]
fn select_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-select-hello\".to_string()",
        "\"Start here: default Select wiring with only items + selected signals.\"",
        "let (selected, set_selected) = signal(None::<usize>);",
        "<Select id_base=\"select-hello\".to_string() items=vec![\"Apple\".to_string(), \"Banana\".to_string()] selected_index=selected set_selected_index=set_selected />",
        "id_base=\"docs-select-controlled\".to_string()",
        "disabled_indices=disabled_indices",
        "set_controlled_open_raw.update(|value| *value = !*value);",
        "\"Toggle open\"",
        "\"open: \"",
        "\"selected: \"",
        "\" · has selection: \"",
        "\" · disabled options: \"",
        "id_base=\"docs-select-disabled\".to_string()",
        "items=disabled_items",
        "\"disabled selected: \"",
        "id_base=\"docs-select-empty\".to_string()",
        "items=empty_items",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "select docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn select_docs_documentation_as_product_keeps_hello_world_first_and_default_path_first() {
    let source = select_docs_section();

    let hello = source
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .unwrap_or_else(|| panic!("select docs should include Hello World playground first."));
    let controlled = source
        .find("<Playground title=\"Controlled Open + Selection\" code_signal=code>")
        .unwrap_or_else(|| panic!("select docs should include controlled playground."));
    let states = source
        .find("<Playground title=\"Disabled + Empty\" code_signal=states_code>")
        .unwrap_or_else(|| panic!("select docs should include disabled/empty playground."));

    assert!(
        hello < controlled && controlled < states,
        "select docs should keep default path first, then advanced scenarios."
    );

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            !source.contains(forbidden),
            "select docs newcomer path should not require internal wiring marker `{forbidden}`."
        );
    }
}

#[test]
fn select_check2_marks_status_primitives_layer_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。"),
        "select check2 should mark status-primitives architecture item complete.",
    );
    assert!(
        source.contains("select_status_primitives_layer_is_pure_and_component_only_consumes"),
        "select check2 should reference executable regression evidence for status-primitives layering.",
    );
}

#[test]
fn select_check2_marks_ui_headless_layer_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。"),
        "select check2 should mark ui-headless architecture item complete.",
    );
    assert!(
        source.contains("select_headless_contracts_drive_interaction_and_a11y_boundaries"),
        "select check2 should reference executable regression evidence for ui-headless layering.",
    );
}

#[test]
fn select_ui_motion_layer_keeps_engine_vs_component_mapping_boundary() {
    let select_motion_source = load_source("src/select/motion.rs");
    let select_view_source = load_source("src/select/view.rs");
    let select_logic_source = load_source("src/select/logic.rs");
    let select_styles_source = load_source("src/select/styles.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");

    for needle in [
        "pub struct SelectMotion",
        "pub popover: PopoverMotion",
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "motion=input.motion.popover",
    ] {
        assert!(
            select_motion_source.contains(needle) || select_view_source.contains(needle),
            "select motion layer should keep component-state -> motion-contract mapping `{needle}`.",
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "request_animation_frame",
        "cancel_animation_frame",
        "MotionKeyframe",
        "pub fn animate(",
    ] {
        assert!(
            !select_motion_source.contains(forbidden),
            "select motion.rs must not re-implement shared motion engine via `{forbidden}`.",
        );
    }

    for forbidden in ["SpringConfig", "SpringAnimator", "request_animation_frame"] {
        assert!(
            !select_logic_source.contains(forbidden) && !select_styles_source.contains(forbidden),
            "select logic/styles should not host motion runtime internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub mod spring;",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub struct SpringConfig",
        "pub fn sanitize_config(",
        "pub struct SpringAnimator",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle) || ui_motion_spring_source.contains(needle),
            "shared ui-motion layer should host reusable runtime capability `{needle}`.",
        );
    }

    for forbidden in ["ui-select", "SelectMotion", "select-panel", "aria-"] {
        assert!(
            !ui_motion_lib_source.contains(forbidden)
                && !ui_motion_spring_source.contains(forbidden),
            "ui-motion must stay free of component/business/a11y semantics `{forbidden}`.",
        );
    }

    assert!(
        popover_motion_source.contains("pub fn attach_motion("),
        "component motion runtime attach should remain in component motion layer (popover).",
    );
}

#[test]
fn select_ui_motion_non_wasm_stub_contract_is_predictable() {
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let select_check2_source = load_source("src/select/check2.md");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion_source.contains(needle),
            "component motion runtime should define predictable non-wasm downgrade token `{needle}`.",
        );
    }

    assert!(
        select_check2_source
            .contains("非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。"),
        "select check2 should keep explicit non-wasm no-op/stub acceptance rule.",
    );
}

#[test]
fn select_check2_marks_ui_motion_layer_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。"),
        "select check2 should mark ui-motion architecture item complete.",
    );
    assert!(
        source.contains("select_ui_motion_layer_keeps_engine_vs_component_mapping_boundary"),
        "select check2 should reference executable regression evidence for ui-motion layering.",
    );
    assert!(
        source.contains("select_ui_motion_non_wasm_stub_contract_is_predictable"),
        "select check2 should reference executable non-wasm stub evidence for ui-motion layering.",
    );
}

#[test]
fn select_ui_theme_layer_uses_shared_token_pipeline_without_rebuilding_theme() {
    let select_styles_source = load_source("src/select/styles.rs");
    let select_view_source = load_source("src/select/view.rs");
    let select_logic_source = load_source("src/select/logic.rs");
    let ui_theme_tokens_source = load_source("../ui-theme/src/tokens.rs");
    let ui_theme_theme_source = load_source("../ui-theme/src/theme.rs");
    let ui_theme_css_source = load_source("../ui-theme/src/css.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");
    let token_baseline_source = load_source("../ui-theme/tests/token_scale_baseline.rs");

    for needle in [
        "pub struct OverlayLayoutTokens",
        "pub panel_min_width_px: u16",
        "pub overlay_layout: OverlayLayoutTokens",
    ] {
        assert!(
            ui_theme_tokens_source.contains(needle),
            "ui-theme tokens layer should define overlay sizing baseline via `{needle}`.",
        );
    }

    for needle in [
        "pub struct ThemeContext",
        "pub fn overlay_layout_tokens(ctx: ThemeContext) -> OverlayLayoutTokens",
        "ThemeColor::Light",
        "ThemeColor::Dark",
        "ThemeColor::Oled",
    ] {
        assert!(
            ui_theme_theme_source.contains(needle),
            "ui-theme theme mapping layer should carry tri-axis context via `{needle}`.",
        );
    }

    for needle in [
        "--ui-overlay-panel-min-width",
        "--ui-system:",
        "--ui-color:",
        "--ui-scale:",
    ] {
        assert!(
            ui_theme_css_source.contains(needle),
            "ui-theme css output should emit `{needle}` for component consumption.",
        );
    }

    assert!(
        select_styles_source.contains("min-width: var(--ui-overlay-panel-min-width);"),
        "select styles should consume overlay sizing token variable.",
    );
    assert!(
        !select_styles_source.contains("min-width: 240px;"),
        "select styles must not hardcode theme baseline constants.",
    );

    for forbidden in [
        "ThemeContext",
        "ThemeColor",
        "ThemeScale",
        "Theme::",
        "to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !select_view_source.contains(forbidden) && !select_logic_source.contains(forbidden),
            "select component layer must not rebuild theme mapping via `{forbidden}`.",
        );
    }

    for needle in [
        "Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-components/src/<component>/styles.rs` 消费。",
        "量化尺寸基准必须可回归",
    ] {
        assert!(
            styling_spec_source.contains(needle),
            "styling spec should keep traceable ui-theme contract `{needle}`.",
        );
    }

    for needle in [
        "fn token_scale_baselines_are_regression_testable()",
        "panel_min_width_px",
        "ThemeColor::Light",
        "ThemeScale::Large",
    ] {
        assert!(
            token_baseline_source.contains(needle),
            "ui-theme baseline regression test should include `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_ui_theme_layer_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。"),
        "select check2 should mark ui-theme architecture item complete.",
    );
    assert!(
        source
            .contains("select_ui_theme_layer_uses_shared_token_pipeline_without_rebuilding_theme"),
        "select check2 should reference executable regression evidence for ui-theme layering.",
    );
}

#[test]
fn select_ui_components_layer_keeps_assembly_boundaries_and_public_api_clean() {
    let mod_source = load_source("src/select/mod.rs");
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");
    let styles_source = load_source("src/select/styles.rs");
    let motion_source = load_source("src/select/motion.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::SelectMotion;",
        "pub use view::Select;",
    ] {
        assert!(
            mod_source.contains(needle),
            "select module boundary should keep stable export layout via `{needle}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::select::{",
        "SelectStateInput",
        "resolve_state",
        "resolve_horizontal_nav_target",
        "find_typeahead_match",
    ] {
        assert!(
            logic_source.contains(needle),
            "select logic.rs should stay an assembly bridge to primitives via `{needle}`.",
        );
    }

    for forbidden in ["web_sys", "NodeRef", "view!", ".ui-select"] {
        assert!(
            !logic_source.contains(forbidden),
            "select logic.rs must not absorb view/dom/style concerns `{forbidden}`.",
        );
    }

    for needle in [
        "logic::resolve_state(logic::SelectStateInput {",
        "overlay_open::use_controllable_open_state_traced(",
        "use_presence(open)",
        "ui_headless::aria_controls_when_open(",
        "let locale = locale_attrs(lang, dir);",
        "<Popover",
        "<List",
    ] {
        assert!(
            view_source.contains(needle),
            "select view.rs should render structure and mount contracts via `{needle}`.",
        );
    }

    for forbidden in ["pub struct SelectStateInput", "pub fn resolve_state("] {
        assert!(
            !view_source.contains(forbidden),
            "select view.rs must not redefine primitive state machines `{forbidden}`.",
        );
    }

    assert!(
        styles_source.contains("min-width: var(--ui-overlay-panel-min-width);"),
        "select styles.rs should remain token-first and consume ui-theme variables.",
    );
    for forbidden in ["ThemeContext", "ThemeColor", "ThemeScale"] {
        assert!(
            !styles_source.contains(forbidden),
            "select styles.rs must not rebuild theme mapping `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct SelectMotion",
        "pub popover: PopoverMotion",
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion_source.contains(needle),
            "select motion.rs should stay as mapping/attach contract layer via `{needle}`.",
        );
    }

    for forbidden in ["use web_sys", "pub use web_sys", "WebSys"] {
        assert!(
            !mod_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !lib_source.contains(forbidden),
            "ui-components public API surface must not leak DOM/web-sys detail `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-select\")]",
        "pub mod select;",
        "pub use select::{Select, SelectMotion};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components should expose select via feature-gated stable public API `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_ui_components_layer_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。"),
        "select check2 should mark ui-components architecture item complete.",
    );
    assert!(
        source
            .contains("select_ui_components_layer_keeps_assembly_boundaries_and_public_api_clean"),
        "select check2 should reference executable regression evidence for ui-components layering.",
    );
}

#[test]
fn select_check2_marks_api_naming_contract_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。"),
        "select check2 should mark api naming contract item complete.",
    );
    assert!(
        source.contains(
            "select_api_naming_uses_prefixed_boolean_props_with_legacy_alias_migration_path"
        ),
        "select check2 should reference executable regression evidence for api naming contract.",
    );
}

#[test]
fn select_semantics_contract_tests_cover_state_and_interaction_matrix() {
    let select_view_source = load_source("src/select/view.rs");
    let list_view_source = load_source("src/list/view.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "role=\"option\"",
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-disabled=move || state.get().trigger_disabled.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            select_view_source.contains(needle) || list_view_source.contains(needle),
            "semantic contract coverage should include `{needle}`.",
        );
    }

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "is_disabled: Option<bool>",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            select_view_source.contains(needle),
            "controlled/uncontrolled + disabled branches should include `{needle}`.",
        );
    }

    for needle in [
        "on:keydown=on_key_down",
        "on:keyup=on_key_up",
        "\"ArrowDown\"",
        "\"ArrowUp\"",
        "KEY_ARROW_LEFT | KEY_ARROW_RIGHT",
        "Enter",
        "typeahead_char",
    ] {
        assert!(
            select_view_source.contains(needle),
            "keyboard path semantic coverage should include `{needle}`.",
        );
    }

    for needle in [
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "aria.handlers.on_option_click.run(index);",
    ] {
        assert!(
            list_view_source.contains(needle),
            "pointer path semantic coverage should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
    ] {
        assert!(
            ui_motion_source.contains(needle) || popover_motion_source.contains(needle),
            "SSR/wasm branch coverage should include `{needle}`.",
        );
    }

    for forbidden in ["assert_snapshot!", "insta::", "to_match_snapshot"] {
        assert!(
            !select_view_source.contains(forbidden) && !list_view_source.contains(forbidden),
            "select semantic contract implementation should not depend on visual snapshots via `{forbidden}`.",
        );
    }
}

#[test]
fn select_component_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("src/select/mod.rs");
    let logic_source = load_source("src/select/logic.rs");
    let styles_source = load_source("src/select/styles.rs");
    let view_source = load_source("src/select/view.rs");
    let motion_source = load_source("src/select/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::SelectMotion;",
        "pub use view::Select;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable exports via `{needle}`.",
        );
    }

    for forbidden in ["NodeRef", "view!", ".ui-select", "animation:"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry view/style runtime concern `{forbidden}`.",
        );
    }
    assert!(
        logic_source.contains("pub use ui_state_primitives::select::{"),
        "logic.rs should remain normalization/derivation bridge from ui-state-primitives.",
    );

    for needle in [".ui-select", "var(--ui-overlay-panel-min-width)"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should host static token-first CSS contract `{needle}`.",
        );
    }
    for forbidden in ["ThemeContext", "UiRoot", "Button", "Popover"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not hold logic/view concern `{forbidden}`.",
        );
    }

    for needle in ["#[component]", "view! {", "<Popover", "use_presence(open)"] {
        assert!(
            view_source.contains(needle),
            "view.rs should render structure and mount headless semantics `{needle}`.",
        );
    }
    for forbidden in ["pub struct SelectStateInput", "SpringAnimator::new("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not redefine primitives/motion engine `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct SelectMotion",
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep semantic->motion mapping via `{needle}`.",
        );
    }
    for forbidden in [
        "request_animation_frame",
        "cancel_animation_frame",
        "web_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not reimplement shared motion backend `{forbidden}`.",
        );
    }
}

#[test]
fn select_check2_marks_semantics_and_file_responsibility_items_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "select check2 should mark semantic-contract testing item complete.",
    );
    assert!(
        source.contains("select_semantics_contract_tests_cover_state_and_interaction_matrix"),
        "select check2 should reference semantic-matrix regression evidence.",
    );
    assert!(
        source.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。"),
        "select check2 should mark component-file-responsibility item complete.",
    );
    assert!(
        source.contains("select_component_files_keep_single_responsibility_boundaries"),
        "select check2 should reference file-responsibility regression evidence.",
    );
}

#[test]
fn select_spec_file_is_not_introduced_for_simple_component() {
    let mod_source = load_source("src/select/mod.rs");
    let check2_source = load_source("src/select/check2.md");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/select/spec.rs");

    assert!(
        !spec_path.exists(),
        "select should not add `spec.rs` unless there is a stable external schema contract.",
    );

    for forbidden in ["mod spec", "pub mod spec", "spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "select module boundary should not expose spec module via `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("简单组件不得为了“形式统一”新增 `spec.rs`"),
        "select check2 should keep explicit no-spec-for-simple-component constraint.",
    );
}

#[test]
fn select_check2_marks_spec_file_scope_item_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "select check2 should mark spec.rs scope item complete.",
    );
    assert!(
        source.contains("select_spec_file_is_not_introduced_for_simple_component"),
        "select check2 should reference executable regression evidence for spec.rs scope.",
    );
}

#[test]
fn select_token_first_static_style_contract_is_aggregated_and_injected_via_ui_root() {
    let select_styles_source = load_source("src/select/styles.rs");
    let select_view_source = load_source("src/select/view.rs");
    let components_css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        ".ui-select",
        ".ui-select__panel",
        "min-width: var(--ui-overlay-panel-min-width);",
    ] {
        assert!(
            select_styles_source.contains(needle),
            "select styles.rs should keep token-first static css contract `{needle}`.",
        );
    }

    for forbidden in ["@apply", "tw-", "class=\"flex", "class=\"grid", "css!("] {
        assert!(
            !select_styles_source.contains(forbidden),
            "select styles.rs should not depend on utility-first/css-in-rust default path `{forbidden}`.",
        );
    }

    for forbidden in ["style=", "style:background", "style:padding", "style:color"] {
        assert!(
            !select_view_source.contains(forbidden),
            "select view should not encode business style logic inline via `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-select\")]",
        "out.push_str(crate::select::styles::CSS);",
    ] {
        assert!(
            components_css_source.contains(needle),
            "ui-components css aggregation should include `{needle}` for select style contract.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject base/theme/components css through `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_token_first_static_style_contract_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "select check2 should mark token-first static style contract item complete.",
    );
    assert!(
        source.contains(
            "select_token_first_static_style_contract_is_aggregated_and_injected_via_ui_root"
        ),
        "select check2 should reference executable regression evidence for token-first static styles.",
    );
}

#[test]
fn select_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let select_styles_source = load_source("src/select/styles.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        ".ui-select--open .ui-select__panel",
        ".ui-select[data-open=\"true\"] .ui-select__panel",
        ".ui-select[data-empty=\"true\"]",
        ".ui-select[data-has-disabled-options=\"true\"] .ui-select__listbox",
        "min-width: var(--ui-overlay-panel-min-width);",
    ] {
        assert!(
            select_styles_source.contains(needle),
            "select default styles should keep interaction and hierarchy markers `{needle}`.",
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "first-impression quality",
        "clear hierarchy, natural contrast, and explicit interaction feedback",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should keep visual-desire contract `{needle}`.",
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e gate should include `{needle}`.",
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc_source.contains(needle),
            "HeroUI strategy doc should keep alignment marker `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_visual_desire_gate_complete() {
    let source = load_source("src/select/check2.md");
    assert!(
        source.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "select check2 should mark visual desire gate complete.",
    );
    assert!(
        source.contains("select_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts"),
        "select check2 should reference executable visual-desire regression evidence.",
    );
}

#[test]
fn select_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-select = [\"component-button\", \"component-list\", \"component-popover\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo tree-shaking contract should include `{needle}`.",
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-select\")]\npub mod select;"),
        "lib.rs should feature-gate select module export for tree-shaking.",
    );
    assert!(
        lib_source.contains("pub use select::{Select, SelectMotion};"),
        "lib.rs should expose select API from feature-gated module.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-select\")]")
            && css_source.contains("out.push_str(crate::select::styles::CSS);"),
        "css.rs should gate select CSS aggregation behind component-select feature.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection.",
    );

    for forbidden in ["component_registry", "ALL_COMPONENTS_MAP", "lazy_static!"] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "tree-shaking boundary should avoid global keep-alive registries `{forbidden}`.",
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components.",
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of relying on implicit defaults.",
    );
}

#[test]
fn select_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking gate script should include `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_tree_shaking_contract_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "select check2 should mark tree-shaking item complete.",
    );

    for needle in [
        "select_tree_shaking_keeps_component_feature_and_css_boundaries",
        "select_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-select,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-select,inject-css",
        "bash ./scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            source.contains(needle),
            "select check2 tree-shaking section should reference executable evidence `{needle}`.",
        );
    }
}

#[test]
fn select_open_state_contract_requires_value_default_callback_triplet() {
    let view_source = load_source("src/select/view.rs");
    let headless_state_source = load_source("../ui-headless/src/controllable_state.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"select\",",
        "open,",
        "default_open,",
        "on_open_change,",
    ] {
        assert!(
            view_source.contains(needle),
            "select open axis must include controlled/uncontrolled triplet token `{needle}`.",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
    ] {
        assert!(
            headless_state_source.contains(needle),
            "headless controllable-state primitive should enforce controlled/uncontrolled semantics via `{needle}`.",
        );
    }
}

#[test]
fn select_defaults_are_not_rewritten_in_view_layer() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");

    for needle in [
        "let placeholder = logic::resolve_placeholder(placeholder);",
        "let id_base = logic::normalize_id_base(id_base);",
        "let motion = crate::select::motion::sanitize_motion(motion);",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should delegate defaults/priority to logic or headless primitive via `{needle}`.",
        );
    }

    for forbidden in [
        "default_open.unwrap_or",
        "placeholder.unwrap_or",
        "id_base.unwrap_or",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs must not rewrite default priority locally via `{forbidden}`.",
        );
    }

    for needle in [
        "resolve_placeholder",
        "normalize_id_base",
        "normalize_optional_text",
    ] {
        assert!(
            logic_source.contains(needle) || primitive_source.contains(needle),
            "default normalization should stay in logic/primitives via `{needle}`.",
        );
    }
}

#[test]
fn select_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");
    let styles_source = load_source("src/select/styles.rs");

    for needle in [
        "logic::resolve_state(logic::SelectStateInput {",
        "let state = Signal::derive(move ||",
        "logic::resolve_horizontal_nav_target(",
        "logic::find_typeahead_match(",
    ] {
        assert!(
            view_source.contains(needle),
            "view layer should consume centralized normalization result via `{needle}`.",
        );
    }

    for forbidden in [
        "pub struct SelectStateInput",
        "pub struct SelectState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not fork primitive normalization implementation `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct SelectStateInput",
        "pub struct SelectState",
        "pub fn resolve_state(input: SelectStateInput)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state normalization primitive should stay in ui-state-primitives via `{needle}`.",
        );
    }

    for forbidden in ["if data-open", "match data-state"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should consume semantic markers rather than implement state machine logic `{forbidden}`.",
        );
    }
}

#[test]
fn select_discrete_axes_are_modeled_with_enums() {
    let view_source = load_source("src/select/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");

    for needle in [
        "placement: PopoverPlacement",
        "dir: Option<A11yDirection>",
        "logic::SelectOpenFocusStrategy::Selected",
        "logic::SelectOpenFocusStrategy::First",
        "logic::SelectOpenFocusStrategy::Last",
        "logic::SelectHorizontalNav::Previous",
        "logic::SelectHorizontalNav::Next",
    ] {
        assert!(
            view_source.contains(needle),
            "select discrete axes should use typed enums via `{needle}`.",
        );
    }

    for needle in [
        "pub enum SelectOpenFocusStrategy",
        "pub enum SelectHorizontalNav",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives should constrain select discrete state space via `{needle}`.",
        );
    }
}

#[test]
fn select_only_consumes_state_primitives_without_business_store_binding() {
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::select::{"),
        "select logic should consume state primitives as the only reusable state source.",
    );

    for forbidden in ["redux", "mobx", "zustand", "pinia", "Store<", "GlobalStore"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "select component must not bind directly to business store types `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn resolve_state(input: SelectStateInput) -> SelectState",
        "pub fn resolve_horizontal_nav_target(",
        "pub fn find_typeahead_match(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "reusable state machine and derivation should remain in ui-state-primitives via `{needle}`.",
        );
    }
}

#[test]
fn select_async_semantics_are_not_applicable() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");
    let check2_source = load_source("src/select/check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "Future",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "select should stay synchronous and not introduce async interaction protocol `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("N/A：`Select` 组件无远程请求与异步状态"),
        "check2 should document explicit async N/A rationale for select.",
    );
}

#[test]
fn select_check2_marks_state_management_small_skeleton_items_complete() {
    let source = load_source("src/select/check2.md");

    for needle in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should mark item complete with exact contract line `{needle}`.",
        );
    }

    assert!(
        source.contains("N/A：`Select` 组件无远程请求与异步状态"),
        "select check2 should record explicit async N/A rationale for select.",
    );

    for needle in [
        "select_open_state_contract_requires_value_default_callback_triplet",
        "select_defaults_are_not_rewritten_in_view_layer",
        "select_state_normalization_is_centralized_in_logic",
        "select_discrete_axes_are_modeled_with_enums",
        "select_only_consumes_state_primitives_without_business_store_binding",
        "select_async_semantics_are_not_applicable",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should reference executable evidence `{needle}`.",
        );
    }
}

#[test]
fn select_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../ui-state-primitives/src/select.rs");
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "pub enum SelectOpenFocusStrategy",
        "pub enum SelectHorizontalNav",
        "pub struct SelectStateInput",
        "pub struct SelectState",
        "selected_index: Option<usize>",
        "selected_index = input\n        .selected_index\n        .filter(|index| *index < input.item_count);",
        "resolve_disabled_option_count(",
        ".filter(|index| **index < item_count)",
        "current = current.filter(|&index| index < item_count);",
    ] {
        assert!(
            primitive_source.contains(needle),
            "type system / normalization contract should include `{needle}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::select::{",
        "SelectStateInput",
        "resolve_state",
        "resolve_horizontal_nav_target",
        "find_typeahead_match",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic layer should consume typed primitives via `{needle}`.",
        );
    }

    for needle in [
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-closed=move || state.get().is_closed.then_some(BOOL_TRUE)",
        "data-disabled=move || state.get().trigger_disabled.then_some(BOOL_TRUE)",
        "data-component-disabled=move || state.get().is_disabled.then_some(BOOL_TRUE)",
        "data-empty=move || state.get().is_empty.then_some(BOOL_TRUE)",
        "data-has-items=move || state.get().has_items.then_some(BOOL_TRUE)",
        "data-has-selection=move || state.get().has_selection.then_some(BOOL_TRUE)",
        "data-selection-empty=move || state.get().selection_empty.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(BOOL_TRUE)",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "machine-readable semantic markers should include `{needle}`.",
        );
    }

    for needle in [
        "class_source_attr: if input.has_custom_class_name {\n            \"custom\"\n        } else {\n            \"default\"\n        },",
        "motion_source_attr: if input.has_custom_motion {\n            \"custom\"\n        } else {\n            \"default\"\n        },",
        "#[cfg(test)]",
        "fn resolve_state_tracks_empty_and_disabled()",
        "fn resolve_state_normalizes_selection_and_markers()",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state-contract feedback loop should include `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_type_system_and_semantic_marker_contract_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "select check2 should mark type-system + semantic-marker contract item complete.",
    );

    for needle in [
        "select_type_system_and_semantic_markers_form_machine_readable_contract",
        "select_discrete_axes_are_modeled_with_enums",
        "select_state_normalization_is_centralized_in_logic",
        "select_exposes_root_state_and_slot_data_attributes",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should reference executable evidence `{needle}`.",
        );
    }
}

#[test]
fn select_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_web_sys_free() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_headless_cargo = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib = load_source("../ui-headless/src/lib.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let select_mod_source = load_source("src/select/mod.rs");
    let select_logic_source = load_source("src/select/logic.rs");
    let select_styles_source = load_source("src/select/styles.rs");
    let select_view_source = load_source("src/select/view.rs");
    let select_motion_source = load_source("src/select/motion.rs");

    for needle in [
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "web-sys = { version = \"0.3.85\"",
        "js-sys = \"0.3.85\"",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components should keep explicit wasm target dependency split via `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_cargo.contains(needle) || ui_headless_lib.contains(needle),
            "ui-headless should manage web/ssr platform branches explicitly via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            popover_motion_source.contains(needle),
            "select motion backend path should stay explicit in popover motion via `{needle}`.",
        );
    }

    let forbidden = "web_sys";
    assert!(
        !select_mod_source.contains(forbidden)
            && !select_logic_source.contains(forbidden)
            && !select_styles_source.contains(forbidden)
            && !select_view_source.contains(forbidden)
            && !select_motion_source.contains(forbidden),
        "non-wasm select source paths must not reference browser-specific `{forbidden}`.",
    );
}

#[test]
fn select_check2_marks_ssr_cross_platform_item_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "select check2 should mark SSR/cross-platform item complete.",
    );

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-select,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components",
        "select_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_web_sys_free",
    ] {
        assert!(
            source.contains(needle),
            "select check2 SSR/cross-platform section should reference evidence `{needle}`.",
        );
    }
}

#[test]
fn select_ui_headless_web_ssr_mutex_contract_is_explicit_and_component_safe() {
    let ui_headless_cargo = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib = load_source("../ui-headless/src/lib.rs");
    let select_view_source = load_source("src/select/view.rs");

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_cargo.contains(needle) || ui_headless_lib.contains(needle),
            "ui-headless web/ssr mutex guard contract should include `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless as overlay_open;",
        "overlay_open::use_controllable_open_state_traced(",
        "use ui_headless::{",
        "use_presence(open)",
    ] {
        assert!(
            select_view_source.contains(needle),
            "select component should consume headless contracts safely via `{needle}`.",
        );
    }
}

#[test]
fn select_ui_headless_platform_script_enforces_mutex_failure_and_dual_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
        "expected ui-headless web+ssr to fail",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce headless web/ssr mutex contract `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_ui_headless_web_ssr_mutex_item_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "select check2 should mark ui-headless web/ssr mutex item complete.",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "mutually exclusive",
        "select_ui_headless_web_ssr_mutex_contract_is_explicit_and_component_safe",
        "select_ui_headless_platform_script_enforces_mutex_failure_and_dual_compile_paths",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should reference ui-headless mutex evidence `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_ui_motion_non_wasm_stub_item_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "select check2 should mark ui-motion non-wasm stub item complete.",
    );

    for needle in [
        "cargo check -p ui-motion",
        "cargo test -p ui-motion --test non_wasm_stub",
        "cargo check -p ui-components --no-default-features --features component-select,inject-css",
        "select_ui_motion_non_wasm_stub_contract_is_predictable",
        "select_check2_marks_ui_motion_non_wasm_stub_item_complete",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should reference ui-motion non-wasm stub evidence `{needle}`.",
        );
    }
}

#[test]
fn select_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_spring_test_source = load_source("../ui-motion/tests/spring.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let select_view_source = load_source("src/select/view.rs");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion fast path token `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_test_source.contains(needle),
            "ui-motion reduced-motion regression coverage should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "opacity.set_target(1.0);",
        "scale.set_target(1.0);",
        "scale.set_on_rest(move || on_exit_complete.run(()));",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion_source.contains(needle),
            "popover motion should preserve reduced-motion/SSR/wasm split token `{needle}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !select_view_source.contains(forbidden),
            "select view semantic surface should not split by platform token `{forbidden}`.",
        );
    }

    for needle in [
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-disabled=move || state.get().trigger_disabled.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
    ] {
        assert!(
            select_view_source.contains(needle),
            "select semantics should remain stable across SSR/wasm branches via `{needle}`.",
        );
    }
}

#[test]
fn select_check2_marks_reduced_motion_ssr_wasm_item_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "select check2 should mark reduced-motion/SSR/wasm item complete.",
    );

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-select,inject-css",
        "cargo check -p ui-components --no-default-features --features component-select,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo test -p ui-motion --test spring",
        "select_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should reference reduced-motion/SSR/wasm evidence `{needle}`.",
        );
    }
}

#[test]
fn select_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("src/select/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_select_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "N/A：`Select` 暂未接入精确 `render_count` 自动化计数",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Select checklist should include performance governance evidence token `{needle}`.",
        );
    }

    assert!(
        pages_source.contains(
            "component_doc!(\"Select\", \"select\", \"Collections\", collections::select)"
        ),
        "Select docs page should stay in component coverage traversal.",
    );

    for needle in ["title=\"Select\"", "slug=\"select\"", "<ComponentPage"] {
        assert!(
            docs_select_page_source.contains(needle),
            "Select docs page should mount through ComponentPage contract `{needle}`.",
        );
    }

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep mount-only fallback/perf probe wiring via `{needle}`.",
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
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable perf observability marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance should keep explicit render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Select should expose state attribution marker `{needle}` for perf triage.",
        );
    }
}

#[test]
fn select_check2_marks_performance_governance_item_complete() {
    let source = load_source("src/select/check2.md");

    assert!(
        source.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "select check2 should mark performance-governance item complete.",
    );

    for needle in [
        "select_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test select_semantics --no-default-features --features component-select,inject-css select_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            source.contains(needle),
            "select check2 performance section should reference executable evidence `{needle}`.",
        );
    }
}

#[test]
fn select_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/select/view.rs");

    assert!(
        view_source.contains("view! {"),
        "Select should keep explicit render blocks in view.rs.",
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        4,
        "Select should keep one main render block and three semantic subrender blocks.",
    );
    assert!(
        view_source.lines().count() <= 460,
        "Select view.rs should stay bounded; split further if this grows significantly.",
    );

    for needle in [
        "fn render_select_trigger(",
        "fn render_select_list(input: SelectListRenderInput) -> impl IntoView",
        "fn render_select_panel(input: SelectPanelRenderInput) -> impl IntoView",
        "fn resolve_list_focus_plan(",
        "let trigger_view = render_select_trigger(",
        "let panel_view = render_select_panel(",
        "{trigger_view}",
        "{panel_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view macro split contract should include `{needle}`.",
        );
    }
}

#[test]
fn select_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/select/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Select should keep a single public component boundary for current layout.",
    );

    for needle in [
        "fn render_select_trigger(",
        "fn render_select_list(input: SelectListRenderInput) -> impl IntoView",
        "fn render_select_panel(input: SelectPanelRenderInput) -> impl IntoView",
        "fn resolve_list_focus_plan(",
        "pub fn Select(",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view should prefer plain function split marker `{needle}`.",
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\nfn select_"] {
        assert!(
            !view_source.contains(forbidden),
            "Select should not introduce local component abstraction noise `{forbidden}`.",
        );
    }
}

#[test]
fn select_static_fragments_are_constantized_or_absent_for_select_layout() {
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "const SLOT_SELECT: &str = \"select\";",
        "const SLOT_SELECT_PANEL: &str = \"select-panel\";",
        "const CLASS_SELECT_PANEL: &str = \"ui-select__panel\";",
        "const CLASS_SELECT_LISTBOX: &str = \"ui-select__listbox\";",
        "const BOOL_TRUE: &str = \"true\";",
        "data-slot=SLOT_SELECT",
        "data-slot=SLOT_SELECT_PANEL",
        "class=CLASS_SELECT_PANEL",
        "class_name=CLASS_SELECT_LISTBOX",
    ] {
        assert!(
            view_source.contains(needle),
            "Select static fragment contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "<svg",
        "<path",
        "<footer",
        "let markdown",
        "let long_text",
        "inner_html=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Select view should avoid heavy inline static fragments token `{forbidden}`.",
        );
    }
}

#[test]
fn select_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/select/mod.rs",
        "src/select/logic.rs",
        "src/select/styles.rs",
        "src/select/view.rs",
        "src/select/motion.rs",
    ] {
        let source = load_source(rel_path);
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
                "Select source `{rel_path}` must not contain raw-html injection token `{forbidden}`.",
            );
        }
    }

    let docs_select = select_docs_section();
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_select.contains(forbidden),
            "Select docs examples must not contain raw-html injection token `{forbidden}`.",
        );
    }
}

#[test]
fn select_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let select_view_source = load_source("src/select/view.rs");
    let select_logic_source = load_source("src/select/logic.rs");
    let select_motion_source = load_source("src/select/motion.rs");
    let list_view_source = load_source("src/list/view.rs");
    let docs_select_source = select_docs_section();

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep shared wasm-debug marker `{needle}`.",
        );
    }
    for forbidden in [
        "select-wasm-debug",
        "component-select-wasm-debug",
        "select_wasm_debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "Select should not define a component-local wasm-debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`.",
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
            "docs-app should expose dev-only wasm debug entry via `{needle}`.",
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
        "ts_ms: now_ms(),",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"select\",",
        "data-open=move || state.get().is_open.then_some(BOOL_TRUE)",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "on:keydown=on_key_down",
        "on:keyup=on_key_up",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "title=\"Controlled Open + Selection\"",
        "title=\"Disabled + Empty\"",
    ] {
        assert!(
            select_view_source.contains(needle)
                || list_view_source.contains(needle)
                || docs_select_source.contains(needle),
            "Select should keep reproducible trace/replay marker `{needle}`.",
        );
    }

    let combined = format!("{select_view_source}\n{select_logic_source}\n{select_motion_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Select component contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }
}

#[test]
fn select_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_source = select_docs_section();

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`.",
        );
    }

    assert!(
        docs_page_source.contains("pub(super) fn select() -> AnyView {"),
        "Select docs page should keep a dedicated `select()` playground entry function.",
    );

    for needle in [
        "<Playground title=\"Controlled Open + Selection\" code_signal=code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Select docs should mount reusable Playground hot-reload path via `{needle}`.",
        );
    }
}

#[test]
fn select_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = select_docs_section();

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`.",
        );
    }

    for needle in [
        "let (selected, set_selected) = signal(Some(1_usize));",
        "let (controlled_open_raw, set_controlled_open_raw) = signal(false);",
        "let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));",
        "set_controlled_open_raw.update(|value| *value = !*value);",
        "open=controlled_open",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "\"open: \"",
        "\"selected: \"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Select docs should keep context-preserving interactive marker `{needle}`.",
        );
    }

    for forbidden in [
        "SELECT_WORKBENCH_STORAGE_KEY",
        "load_select_workbench_state(",
        "save_select_workbench_state(",
        "clear_select_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Select keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }
}

#[test]
fn select_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let mod_source = load_source("src/select/mod.rs");
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");
    let styles_source = load_source("src/select/styles.rs");
    let motion_source = load_source("src/select/motion.rs");

    assert!(
        !path_exists("src/select/spec.rs"),
        "Select should keep spec/serde migration path N/A for current simple scope.",
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Select engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`.",
        );
    }
}

#[test]
fn select_check2_marks_view_dx_engineering_and_inner_html_items_complete() {
    let source = load_source("src/select/check2.md");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should mark view/dx/engineering item complete: `{needle}`."
        );
    }

    for evidence in [
        "select_view_macro_complexity_is_split_into_semantic_subrenders",
        "select_view_functional_split_prefers_plain_functions_over_local_components",
        "select_static_fragments_are_constantized_or_absent_for_select_layout",
        "select_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "select_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        "select_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "select_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "select_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "select_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "select_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            source.contains(evidence),
            "select check2 should include executable evidence `{evidence}`."
        );
    }
}

#[test]
fn select_check2_marks_entry_files_agent_contract_and_streaming_items_complete() {
    let source = load_source("src/select/check2.md");

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "- [x] 组件目录标准文件落点正确。",
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should mark entry/agent/streaming item complete: `{needle}`."
        );
    }

    for evidence in [
        "select_ui_components_fixed_entry_files_follow_layered_boundaries",
        "select_component_directory_has_standard_file_layout_and_boundaries",
        "select_agent_contract_is_schema_typed_and_machine_readable_in_view",
        "select_streaming_contract_is_optional_with_snapshot_fallback_and_explicit_output_status",
        "select_snapshot_mode_is_default_and_stable_for_non_text_component",
    ] {
        assert!(
            source.contains(evidence),
            "select check2 should include executable evidence `{evidence}`."
        );
    }
}

#[test]
fn select_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/select/mod.rs"),
        load_source("src/select/logic.rs"),
        load_source("src/select/view.rs"),
        load_source("src/select/styles.rs"),
        load_source("src/select/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    assert!(
        !cargo_source.contains("select-wasm-debug") && !cargo_source.contains("select_wasm_debug"),
        "Select should not define component-local tracing feature when no local debug event/replay contract exists.",
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::select::",
        "const SELECT_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Select should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn select_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/select/mod.rs");
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");
    let styles_source = load_source("src/select/styles.rs");
    let motion_source = load_source("src/select/motion.rs");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ] {
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
                "Select engineering contract should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Select public module boundary should not leak web_sys types.",
    );
}

#[test]
fn select_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("src/active_highlight.rs");
    let headless_controllable = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../ui-headless/src/presence.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "#[cfg(feature = \"component-select\")]",
        "pub mod select;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable export/gate marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-select\")]",
        "out.push_str(crate::select::styles::CSS);",
        "out.push_str(\"\\n@layer ui {\\n\");",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css entry should keep marker `{needle}`.",
        );
    }

    for needle in [
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`.",
        );
    }

    for forbidden in ["Accordion", "Button", "aria-", "on:click"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component-business marker `{forbidden}`.",
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless canonical primitive should keep marker `{needle}`.",
        );
    }
}

#[test]
fn select_component_directory_has_standard_file_layout_and_boundaries() {
    for required in [
        "src/select/mod.rs",
        "src/select/logic.rs",
        "src/select/styles.rs",
        "src/select/view.rs",
        "src/select/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "select component directory should include required file `{required}`.",
        );
    }
    for forbidden in ["src/select/render.rs", "src/select/spec.rs"] {
        assert!(
            !path_exists(forbidden),
            "select component directory should not include `{forbidden}`.",
        );
    }

    let mod_source = load_source("src/select/mod.rs");
    let logic_source = load_source("src/select/logic.rs");
    let styles_source = load_source("src/select/styles.rs");
    let view_source = load_source("src/select/view.rs");
    let motion_source = load_source("src/select/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::SelectMotion;",
        "pub use view::Select;",
    ] {
        assert!(
            mod_source.contains(needle),
            "select/mod.rs should keep stable export marker `{needle}`.",
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;", "web_sys"] {
        assert!(
            !mod_source.contains(forbidden),
            "select/mod.rs should not leak internal/platform marker `{forbidden}`.",
        );
    }

    for forbidden in ["view! {", "on:pointer", "on:keydown", "NodeRef<", "web_sys"] {
        assert!(
            !logic_source.contains(forbidden),
            "select/logic.rs should stay normalization-only; found `{forbidden}`.",
        );
    }
    assert!(
        logic_source.contains("pub use ui_state_primitives::select::{"),
        "select/logic.rs should remain bridge to ui-state-primitives.",
    );

    for needle in ["pub const CSS: &str =", "var(--ui-overlay-panel-min-width)"] {
        assert!(
            styles_source.contains(needle),
            "select/styles.rs should keep token-first CSS marker `{needle}`.",
        );
    }
    for forbidden in ["#[component]", "on:click", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "select/styles.rs should stay static style contract; found `{forbidden}`.",
        );
    }

    for needle in [
        "#[component]",
        "pub fn Select(",
        "overlay_open::use_controllable_open_state_traced(",
        "use_presence(open)",
        "render_select_trigger(",
        "render_select_panel(",
    ] {
        assert!(
            view_source.contains(needle),
            "select/view.rs should keep render+headless mount marker `{needle}`.",
        );
    }
    for forbidden in ["pub fn resolve_state(", "SpringAnimator::new("] {
        assert!(
            !view_source.contains(forbidden),
            "select/view.rs should not host primitive/motion engine concern `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct SelectMotion",
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion_source.contains(needle),
            "select/motion.rs should keep semantic->motion mapping marker `{needle}`.",
        );
    }
}

#[test]
fn select_agent_contract_is_schema_typed_and_machine_readable_in_view() {
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "pub enum SelectAgentSchema",
        "pub enum SelectAgentSchemaVersion",
        "pub enum SelectStreamSupport",
        "pub enum SelectStreamFallback",
        "pub enum SelectStreamMode",
        "pub enum SelectOutputStatus",
        "pub enum SelectAgentIntent",
        "pub enum SelectAgentAction",
        "pub enum SelectAgentState",
        "pub struct SelectAgentContract",
        "pub fn resolve_agent_contract(state: SelectState) -> SelectAgentContract",
        "schema_attr: SelectAgentSchema::V1.as_attr()",
        "schema_version_attr: SelectAgentSchemaVersion::V1.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "select agent contract logic should include `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should expose machine-readable agent contract marker `{needle}`.",
        );
    }
}

#[test]
fn select_streaming_contract_is_optional_with_snapshot_fallback_and_explicit_output_status() {
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "SelectStreamSupport::Optional.as_attr()",
        "SelectStreamFallback::Snapshot.as_attr()",
        "SelectStreamMode::Snapshot.as_attr()",
        "SelectOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "select streaming contract logic should include `{needle}`.",
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should expose explicit stream/output marker `{needle}`.",
        );
    }
}

#[test]
fn select_snapshot_mode_is_default_and_stable_for_non_text_component() {
    let logic_source = load_source("src/select/logic.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "pub enum SelectStreamMode",
        "Self::Snapshot => \"snapshot\"",
        "pub enum SelectStreamFallback",
        "Self::Snapshot => \"snapshot\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "select snapshot baseline contract should include `{needle}`.",
        );
    }

    for needle in [
        "data-ui-stream-mode=move || agent_contract.get().stream_mode_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "select snapshot baseline should stay visible via `{needle}`.",
        );
    }
}

#[test]
fn select_api_dx_paradox_keeps_simple_usage_and_hides_internal_wiring() {
    let view_source = load_source("src/select/view.rs");
    let docs_source = select_docs_section();

    for needle in [
        "pub fn Select(",
        "items: Vec<String>",
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "select api should keep simple user-facing marker `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Controlled Open + Selection\" code_signal=code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
        "<Select",
        "open=controlled_open",
        "set_selected_index=set_selected",
    ] {
        assert!(
            docs_source.contains(needle),
            "select docs should present direct, default-first usage marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "SelectStateInput",
        "state=",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "select docs basic usage should hide internal wiring `{forbidden}`."
        );
    }
}

#[test]
fn select_composition_api_avoids_parallel_arrays_and_implicit_pairing() {
    let view_source = load_source("src/select/view.rs");
    let docs_source = select_docs_section();

    assert!(
        view_source.contains("items: Vec<String>"),
        "select should keep single explicit item collection input."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children: Vec<",
        "ItemSpec",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "select api should avoid implicit parallel-structure contract `{forbidden}`."
        );
    }
}

#[test]
fn select_a11y_i18n_observability_and_style_contracts_are_explicit() {
    let view_source = load_source("src/select/view.rs");
    let styles_source = load_source("src/select/styles.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=input.trigger_id",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "select view should keep explicit a11y/i18n/observability marker `{needle}`."
        );
    }

    for needle in [
        ".ui-select[data-class-source=\"custom\"]",
        ".ui-select[data-motion-source=\"custom\"]",
        ".ui-select[data-open=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "select styles should depend on explicit state marker `{needle}`."
        );
    }

    for forbidden in [":nth-child(", "style=", "style:"] {
        assert!(
            !styles_source.contains(forbidden),
            "select styles should avoid fragile structure/inline style marker `{forbidden}`."
        );
    }
}

#[test]
fn select_docs_and_e2e_contracts_use_semantic_selectors_and_repeatable_flow() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_select_contract.spec.mjs");

    for needle in [
        "pub(super) fn select() -> AnyView",
        "title=\"Select\"",
        "slug=\"select\"",
        "<Playground title=\"Controlled Open + Selection\" code_signal=code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
        "id_base=\"docs-select-controlled\".to_string()",
        "id_base=\"docs-select-disabled\".to_string()",
        "id_base=\"docs-select-empty\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "select docs should keep synced docs/demo matrix marker `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
        "let section_class = \"docs-card playground\";",
        "data-playground-scope=scope_id.clone()",
        "code_signal: Option<Signal<String>>",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground should keep source-first/copy-paste-ready marker `{needle}`."
        );
    }

    for needle in [
        "docs-app select contract uses semantic selectors with settled waits",
        "docs-app select key flow is repeatable with semantic breakpoints",
        "docs-app select controlled playground code panel exposes copy-ready snippet",
        "/#/components/select",
        "body:not(:has(#boot))",
        "xpath=ancestor::*[@data-slot=\"select\" and @data-has-items][1]",
        "await expect(root).toHaveAttribute(\"data-closed\", \"true\")",
        "await expect(root).toHaveAttribute(\"data-selected-index\", \"2\")",
        "await page.keyboard.press(\"ArrowRight\")",
        "await page.keyboard.press(\"ArrowLeft\")",
        "await page.reload()",
        ".filter({ has: page.locator(\"#docs-select-controlled-trigger\") })",
        "await expect(codeBlock).toHaveAttribute(\"data-copyable\", \"true\")",
        ".ui-code-block__copy-button",
    ] {
        assert!(
            e2e_source.contains(needle),
            "select e2e should keep semantic-selector + settled-flow marker `{needle}`."
        );
    }
}

#[test]
fn select_source_first_docs_are_copy_paste_ready_via_playground_and_code_block() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("src/code_block/view.rs");
    let docs_source = select_docs_section();
    let check2_source = load_source("src/select/check2.md");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_signal: Option<Signal<String>>",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground should keep import-ready copy composition marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, default = true)] copyable: bool",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_source.contains(needle),
            "code block should expose one-click copy marker `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Controlled Open + Selection\" code_signal=code>",
        "<Playground title=\"Disabled + Empty\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "select docs should wire playground code signals for copy-ready snippets `{needle}`."
        );
    }

    assert!(
        check2_source.contains("AI Spec 联动示例对 `Select` 属 N/A"),
        "select check2 should explicitly mark source-first spec-linkage as N/A for non-spec component scope.",
    );
}

#[test]
fn select_heroui_strategy_docs_keep_select_contract_traceable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let research_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");

    for needle in [
        "HeroUI Select Docs: https://www.heroui.com/docs/components/select",
        "is_disabled",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            strategy_source.contains(needle) || research_source.contains(needle),
            "heroui strategy docs should keep select-aligned contract marker `{needle}`."
        );
    }

    for needle in [
        "### Select 同步记录（2026-02-18）",
        "component_doc!(\"Select\", \"select\", \"Collections\", collections::select)",
        "`#/components/select` 可索引访问。",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should keep explicit select sync marker `{needle}`."
        );
    }
}

#[test]
fn select_check2_marks_api_a11y_docs_e2e_antipattern_and_merge_gate_items_complete() {
    let source = load_source("src/select/check2.md");

    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
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
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "select check2 should mark item complete: `{needle}`."
        );
    }

    for evidence in [
        "select_api_dx_paradox_keeps_simple_usage_and_hides_internal_wiring",
        "select_composition_api_avoids_parallel_arrays_and_implicit_pairing",
        "select_a11y_i18n_observability_and_style_contracts_are_explicit",
        "select_semantics_contract_tests_cover_state_and_interaction_matrix",
        "select_docs_and_e2e_contracts_use_semantic_selectors_and_repeatable_flow",
        "select_source_first_docs_are_copy_paste_ready_via_playground_and_code_block",
        "select_docs_page_covers_primary_playgrounds",
        "select_docs_playgrounds_lock_state_matrix_contract_values",
        "select_docs_documentation_as_product_keeps_hello_world_first_and_default_path_first",
        "select_heroui_strategy_docs_keep_select_contract_traceable",
        "select_status_primitives_layer_is_pure_and_component_only_consumes",
        "select_headless_contracts_drive_interaction_and_a11y_boundaries",
        "select_defaults_are_not_rewritten_in_view_layer",
        "select_api_naming_uses_prefixed_boolean_props_with_legacy_alias_migration_path",
        "select_only_consumes_state_primitives_without_business_store_binding",
        "select_ui_components_layer_keeps_assembly_boundaries_and_public_api_clean",
        "/root/.cargo/bin/cargo fmt --all -- --check",
        "/root/.cargo/bin/cargo clippy -p ui-components --test select_semantics --no-default-features --features component-select,inject-css -- -D warnings",
        "/root/.cargo/bin/cargo test -p ui-components --test select_semantics --no-default-features --features component-select,inject-css",
        "npx playwright test tests/docs_app_select_contract.spec.mjs --reporter=list",
    ] {
        assert!(
            source.contains(evidence),
            "select check2 should include executable evidence `{evidence}`."
        );
    }
}
