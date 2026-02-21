fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn color_area_module_keeps_layered_boundaries() {
    let module = load_source("mod");

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorAreaMotion;",
        "pub use ui_headless::A11yDirection;",
        "pub use view::ColorArea;",
        "#[path = \"../test/semantics.rs\"]",
    ] {
        assert!(
            module.contains(required),
            "color-area module should keep layered export contract `{required}`."
        );
    }
}

#[test]
fn color_area_logic_and_view_consume_primitives_and_headless_contracts() {
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        logic.contains("pub use ui_state_primitives::color_area::{"),
        "color-area logic should consume ui-state-primitives instead of redefining state machines."
    );

    for required in [
        "use_color_area(ColorAreaOptions {",
        "use_controllable_state(value, Some(default_value), on_value_change)",
        ".handlers",
        ".on_key_down",
        ".parse_axis_input",
        ".resolve_cell",
    ] {
        assert!(
            view.contains(required),
            "color-area view should mount ui-headless contract via `{required}`."
        );
    }
}

#[test]
fn color_area_public_api_does_not_expose_dom_types() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "NodeRef<",
    ] {
        assert!(
            !module.contains(forbidden),
            "color-area module public surface must not expose `{forbidden}`."
        );
        assert!(
            !logic.contains(forbidden),
            "color-area logic must not expose `{forbidden}`."
        );
        assert!(
            !view.contains(forbidden),
            "color-area view should avoid leaking `{forbidden}` in public API surface."
        );
    }
}

#[test]
fn color_area_styles_and_motion_stay_component_shell_only() {
    let styles = load_source("styles");
    let motion = load_source("motion");

    assert!(
        styles.contains("pub const CSS: &str ="),
        "color-area styles should expose static css contract."
    );
    assert!(
        styles.contains("var(--ui-"),
        "color-area styles should consume ui-theme tokens."
    );

    for forbidden in [
        "SpringAnimator",
        "MotionKeyframe",
        "request_animation_frame",
        "web_sys::",
    ] {
        assert!(
            !motion.contains(forbidden),
            "color-area motion should not implement engine details `{forbidden}`."
        );
    }
    assert!(
        motion.contains("pub fn attach_motion("),
        "color-area motion should only provide semantic-to-runtime attach mapping."
    );
}

#[test]
fn color_area_api_naming_contract_uses_is_on_default_prefixes() {
    let view = load_source("view");

    for required in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] default_value: Option<(f32, f32)>",
        "#[prop(optional)] on_value_change: Option<Callback<(f32, f32)>>",
    ] {
        assert!(
            view.contains(required),
            "color-area public props should keep naming contract `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: Option<bool>",
        "on_change",
        "default_disabled",
    ] {
        assert!(
            !view.contains(forbidden),
            "color-area should avoid naming drift alias `{forbidden}`."
        );
    }
}

#[test]
fn color_area_controlled_uncontrolled_axis_is_paired_and_observable() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "#[prop(optional)] value: Option<Signal<(f32, f32)>>",
        "#[prop(optional)] default_value: Option<(f32, f32)>",
        "#[prop(optional)] on_value_change: Option<Callback<(f32, f32)>>",
        "let default_value = logic::normalize_default_value(default_value);",
        "let value_axis = logic::normalize_value_axis(value.is_some());",
        "let controllable = use_controllable_state(value, Some(default_value), on_value_change);",
        "data-value-control-mode=value_axis.control_mode.as_attr()",
        "data-value-source=value_axis.value_source.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "color-area should keep controlled/uncontrolled paired contract `{required}`."
        );
    }

    for required in [
        "pub fn normalize_value_axis(is_controlled: bool) -> ColorAreaValueAxis",
        "ColorAreaValueControlMode::Controlled",
        "ColorAreaValueControlMode::Uncontrolled",
        "ColorAreaValueSourceAttr::External",
        "ColorAreaValueSourceAttr::Default",
    ] {
        assert!(
            logic.contains(required),
            "color-area logic should keep stable control-mode mapping `{required}`."
        );
    }
}

#[test]
fn color_area_view_uses_logic_as_single_default_source() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "let default_value = logic::normalize_default_value(default_value);",
        "let step = logic::normalize_step(step);",
        "let grid_size = logic::normalize_grid_size(grid_size);",
    ] {
        assert!(
            view.contains(required),
            "color-area view should consume centralized default normalization `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, default = logic::DEFAULT_STEP)] step: f32",
        "#[prop(optional, default = logic::DEFAULT_GRID_SIZE)] grid_size: usize",
        "unwrap_or(",
    ] {
        assert!(
            !view.contains(forbidden),
            "color-area view must not keep local default fallback `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_default_value(default_value: Option<(f32, f32)>) -> (f32, f32)",
        "pub fn normalize_step(step: Option<f32>) -> f32",
        "pub fn normalize_grid_size(grid_size: Option<usize>) -> usize",
    ] {
        assert!(
            logic.contains(required),
            "color-area logic should be the single default source via `{required}`."
        );
    }
}

#[test]
fn color_area_event_handlers_delegate_state_rules_to_logic() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "logic::reduce_axis_input(",
        "logic::ColorAreaAxis::X",
        "logic::ColorAreaAxis::Y",
        "logic::reduce_cell_select(",
        "logic::reduce_keyboard_result(",
    ] {
        assert!(
            view.contains(required),
            "color-area view event handlers should delegate state rules via `{required}`."
        );
    }

    for forbidden in [
        "if root.get_untracked().state.is_disabled {",
        "let current = logic::clamp_value(value.get_untracked());",
    ] {
        assert!(
            !view.contains(forbidden),
            "color-area view should avoid rebuilding state rules in handlers `{forbidden}`."
        );
    }

    for required in [
        "pub enum ColorAreaAxis",
        "pub struct ColorAreaEventOutcome",
        "pub fn reduce_axis_input(",
        "pub fn reduce_cell_select(",
        "pub fn reduce_keyboard_result(",
    ] {
        assert!(
            logic.contains(required),
            "color-area logic should centralize typed event normalization `{required}`."
        );
    }
}
