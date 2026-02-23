use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn switch_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/switch/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Switch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn switch_uses_headless_hooks() {
    let source = load_source("src/switch/view.rs");
    let headless_source = load_source("../ui-headless/src/switch.rs");

    for needle in ["use_switch", "SwitchOptions"] {
        assert!(
            source.contains(needle),
            "Switch should use headless `{needle}` hooks."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, OnPress, SwitchOptions, use_controllable_state, use_switch};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "lang,",
        "dir,",
    ] {
        assert!(
            source.contains(needle),
            "Switch should expose and pass `{needle}` to headless locale contract."
        );
    }

    for needle in ["use_focus_ring", "use_hover"] {
        assert!(
            !source.contains(needle),
            "Switch view should not wire `{needle}` directly after semantic contract sink."
        );
    }

    for needle in [
        "use_hover(HoverOptions",
        "use_focus_ring(FocusRingOptions",
        "resolve_switch_state(SwitchStateInput",
        "locale_attrs(lang, dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "Switch headless contract should include `{needle}`."
        );
    }
}

#[test]
fn switch_uses_logic_state_model() {
    let view_source = load_source("src/switch/view.rs");
    let logic_source = load_source("src/switch/logic.rs");
    let headless_source = load_source("../ui-headless/src/switch.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");

    for needle in [
        "ui_theme::default_switch_layout_tokens()",
        "pub use ui_state_primitives::switch::{DEFAULT_CHECKED, SwitchCheckedControlMode};",
        "use ui_state_primitives::switch::{",
        "resolve_checked_axis(PrimitiveSwitchCheckedAxisInput {",
        "pub fn compose_class_name(class_name: Option<String>) -> String",
        "pub fn resolve_motion_markers(is_custom_motion: bool) -> (&'static str, Option<&'static str>)",
        "pub fn default_thumb_size_px() -> f64",
        "pub fn checked_thumb_x_px(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Switch logic should derive motion geometry from theme tokens; missing `{needle}`."
        );
    }

    for needle in ["pub const TRACK_WIDTH_PX", "pub const TRACK_PADDING_PX", "pub const THUMB_WIDTH_PX"] {
        assert!(
            !logic_source.contains(needle),
            "Switch logic should not keep hardcoded geometry token constants; found `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(",
        "SwitchStateInput",
        "resolve_switch_state",
    ] {
        assert!(
            !logic_source.contains(needle),
            "Switch logic should not re-implement state derivation in component layer; found `{needle}`."
        );
    }

    for needle in [
        "pub struct SwitchState {",
        "pub resolved: Memo<PrimitiveSwitchState>",
        "pub state: SwitchState",
        "resolve_switch_state(SwitchStateInput {",
    ] {
        assert!(
            headless_source.contains(needle),
            "Switch headless should expose typed attrs/handlers/state contract; missing `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_CHECKED: bool = false;",
        "pub enum SwitchCheckedControlMode",
        "pub struct SwitchCheckedAxisInput",
        "pub struct SwitchCheckedAxisState",
        "pub fn resolve_checked_axis(input: SwitchCheckedAxisInput) -> SwitchCheckedAxisState",
        "pub struct SwitchStateInput",
        "pub struct SwitchState",
        "pub fn resolve_state(input: SwitchStateInput) -> SwitchState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Switch primitive should define `{needle}`."
        );
    }

    {
        let needle = "pub mod switch;";
        assert!(
            primitive_lib_source.contains(needle),
            "ui-state-primitives should export `{needle}`."
        );
    }

    for needle in [
        "aria.state.resolved.get().data_state()",
        "let class = logic::compose_class_name(class_name);",
        "logic::resolve_motion_markers(motion != SwitchMotion::default());",
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
        "on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())",
        "on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "Switch view should mount headless semantics contract; missing `{needle}`."
        );
    }
}

#[test]
fn switch_attaches_thumb_motion_driver() {
    let source = load_source("src/switch/view.rs");

    assert!(
        source.contains("attach_thumb_motion"),
        "Switch should attach a motion driver for thumb micro-interactions."
    );
}

#[test]
fn switch_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/switch/view.rs");

    for attr in [
        "data-slot=\"switch\"",
        "data-state=move || aria.state.resolved.get().data_state()",
        "data-checked=move || aria.state.resolved.get().is_checked.then_some(\"true\")",
        "data-unchecked=move || aria.state.resolved.get().is_unchecked.then_some(\"true\")",
        "data-disabled=move || aria.state.resolved.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || aria.state.resolved.get().is_enabled.then_some(\"true\")",
        "data-pressed=move || aria.state.resolved.get().is_pressed.then_some(\"true\")",
        "data-hovered=move || aria.state.resolved.get().is_hovered.then_some(\"true\")",
        "data-focused=move || aria.state.resolved.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Switch should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn switch_styles_include_motion_marker_contracts() {
    let source = load_source("src/switch/styles.rs");

    for selector in [
        ".ui-switch[data-motion-source=\"custom\"]",
        ".ui-switch[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Switch styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn switch_styles_consume_theme_switch_layout_tokens() {
    let source = load_source("src/switch/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css/render/theme_to_css_variables.inc");
    let theme_tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");

    for needle in [
        "var(--ui-switch-gap, var(--ui-fallback-switch-gap))",
        "var(--ui-switch-track-width, var(--ui-fallback-switch-track-width))",
        "var(--ui-switch-track-height, var(--ui-fallback-switch-track-height))",
        "var(--ui-switch-track-padding, var(--ui-fallback-switch-track-padding))",
        "var(--ui-switch-thumb-size, var(--ui-fallback-switch-thumb-size))",
        "var(--ui-switch-thumb-checked-x, var(--ui-fallback-switch-thumb-checked-x))",
        "var(--ui-switch-focus-outline-width, var(--ui-fallback-switch-focus-outline-width))",
        "var(--ui-switch-focus-outline-offset, var(--ui-fallback-switch-focus-outline-offset))",
        "var(--ui-switch-disabled-opacity, var(--ui-fallback-switch-disabled-opacity))",
        "var(--ui-switch-hover-brightness, var(--ui-fallback-switch-hover-brightness))",
    ] {
        assert!(
            source.contains(needle),
            "Switch styles should consume theme-backed switch layout token `{needle}`."
        );
    }

    for needle in ["pub struct SwitchLayoutTokens", "pub const SWITCH_LAYOUT_TOKENS_MEDIUM"] {
        assert!(
            theme_tokens_source.contains(needle),
            "ui-theme tokens should define switch layout baseline `{needle}`."
        );
    }

    for needle in ["pub fn switch_layout_tokens(ctx: ThemeContext)", "pub fn default_switch_layout_tokens()"] {
        assert!(
            theme_source.contains(needle),
            "ui-theme theme mapping should expose switch layout token API `{needle}`."
        );
    }

    for needle in ["--ui-switch-gap", "--ui-switch-track-width", "--ui-switch-thumb-size"] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css variable rendering should emit `{needle}`."
        );
    }
}

#[test]
fn switch_motion_uses_spring_animator() {
    let source = load_source("src/switch/motion.rs");
    let motion_core_source = load_source("../ui-motion/src/spring.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Switch motion should be spring-driven to match the repo's motion spec."
    );

    for needle in ["ui_motion::spring::sanitize_config", "ui_motion::spring::SpringAnimator"] {
        assert!(
            source.contains(needle),
            "Switch motion should consume shared ui-motion spring contracts via `{needle}`."
        );
    }

    for needle in ["request_animation_frame", "cancel_animation_frame"] {
        assert!(
            !source.contains(needle),
            "Switch motion should not implement driver internals in component layer; found `{needle}`."
        );
        assert!(
            motion_core_source.contains(needle),
            "ui-motion spring backend should own `{needle}` driver internals."
        );
    }
}

#[test]
fn switch_view_uses_token_backed_pressed_width_default() {
    let source = load_source("src/switch/view.rs");

    assert!(
        source.contains("#[prop(optional, default = motion::default_pressed_width_px())]"),
        "Switch view should source default pressed width from theme-backed motion helper.",
    );
}

#[test]
fn switch_checked_axis_supports_controlled_and_uncontrolled_contract() {
    let source = load_source("src/switch/view.rs");
    let logic_source = load_source("src/switch/logic.rs");
    let headless_source = load_source("../ui-headless/src/controllable_state.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch.rs");

    assert!(
        source.contains("#[prop(optional)] is_disabled: bool,"),
        "Switch should use `is_disabled` boolean prop naming contract.",
    );
    assert!(
        !source.contains("#[prop(optional)] disabled: bool,"),
        "Switch should not keep legacy boolean prop alias `disabled`.",
    );
    assert!(
        source.contains("#[prop(optional, into)] checked: Option<Signal<bool>>"),
        "Switch should expose `checked` as optional controlled value axis.",
    );
    assert!(
        source.contains("#[prop(optional)] default_checked: Option<bool>"),
        "Switch should expose `default_checked` for uncontrolled initialization.",
    );
    assert!(
        source.contains("#[prop(optional)] on_checked_change: Option<Callback<bool>>"),
        "Switch should expose `on_checked_change` callback axis.",
    );
    assert!(
        source.contains("let checked_state = use_controllable_state("),
        "Switch should route checked axis through `use_controllable_state` to avoid half-controlled behavior.",
    );
    assert!(
        source.contains("data-checked-control-mode=checked_control_mode_attr"),
        "Switch should expose stable control-mode markers for checked axis.",
    );
    assert!(
        logic_source.contains("pub fn normalize_checked_axis(input: CheckedAxisInput) -> CheckedAxisState"),
        "Switch logic should centralize checked-axis normalization in `logic.rs`.",
    );
    assert!(
        headless_source.contains("pub fn use_controllable_state<T>("),
        "Switch should rely on shared headless controllable primitive contract.",
    );
    assert!(
        source.contains("#[prop(optional)] set_checked: Option<WriteSignal<bool>>"),
        "Switch keeps `set_checked` as compatibility handler while canonical callback remains `on_checked_change`.",
    );
    assert!(
        !source.contains("#[prop(optional)] on_change: Option<Callback<bool>>"),
        "Switch should not keep legacy callback alias `on_change`.",
    );
    assert!(
        primitive_source.contains("on_checked_change+set_checked"),
        "Switch checked-axis merged handler source marker should be defined by state primitive.",
    );
    assert!(
        logic_source.contains("pub use ui_state_primitives::switch::{DEFAULT_CHECKED, SwitchCheckedControlMode};"),
        "Switch logic should consume checked-axis constants/types from ui-state-primitives.",
    );
    assert!(
        logic_source.contains("resolve_checked_axis(PrimitiveSwitchCheckedAxisInput {"),
        "Switch logic should derive checked-axis control markers via ui-state-primitives resolver.",
    );
    assert!(
        logic_source.contains("pub const fn next_checked(is_checked: bool) -> bool"),
        "Switch checked-transition rule should be centralized in `logic.rs`.",
    );
    assert!(
        source.contains("let next = logic::next_checked(checked.get_untracked());"),
        "Switch event callback should trigger change from logic-derived next state.",
    );
    assert!(
        !source.contains("let next = !checked.get_untracked();"),
        "Switch event callback should not inline transition rules in `view.rs`.",
    );
}

#[test]
fn switch_default_checked_is_normalized_only_in_logic_layer() {
    let view_source = load_source("src/switch/view.rs");
    let logic_source = load_source("src/switch/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch.rs");

    assert!(
        logic_source.contains("default_checked: input.default_checked.unwrap_or(DEFAULT_CHECKED)"),
        "Switch checked default priority should be normalized in `logic.rs`.",
    );
    assert!(
        primitive_source.contains("pub const DEFAULT_CHECKED: bool = false;"),
        "Switch should keep checked-default constant in state primitives.",
    );
    assert!(
        view_source.contains("Some(checked_axis.default_checked)"),
        "Switch view should consume normalized default from logic output.",
    );
    assert!(
        !view_source.contains("default_checked.unwrap_or"),
        "Switch view must not perform fallback branching for checked default.",
    );
}

#[test]
fn switch_discrete_mode_contract_uses_typed_enum() {
    let logic_source = load_source("src/switch/logic.rs");
    let view_source = load_source("src/switch/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/switch.rs");

    for needle in [
        "pub enum SwitchCheckedControlMode",
        "pub control_mode: SwitchCheckedControlMode",
        "let checked_control_mode_attr = checked_axis.control_mode.data_attr();",
    ] {
        assert!(
            logic_source.contains(needle)
                || view_source.contains(needle)
                || primitive_source.contains(needle),
            "Switch control mode should remain enum-typed and derived via logic; missing `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] mode: Option<String>",
        "#[prop(optional)] mode: Option<bool>",
        "#[prop(optional)] status: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Switch should not expose string/bool-union discrete mode props; found `{forbidden}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn switch_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/switch/motion.rs");

    for needle in [
        "use ui_theme::default_switch_motion_tokens;",
        "pub fn default_pressed_width_px() -> f64",
        "ui_motion::spring::sanitize_config",
        "pub fn sanitize_motion(motion: SwitchMotion) -> SwitchMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "fn sanitize_pressed_width_px(value: f64) -> f64",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let pressed_width_px = sanitize_pressed_width_px(pressed_width_px);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_pressed_width_clamps_and_uses_fallback()",
    ] {
        assert!(
            source.contains(needle),
            "Switch motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn switch_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn switch() -> AnyView",
        "title=\"Switch\"",
        "slug=\"switch\"",
        "description=\"Switch toggle with baseline-level spring thumb motion and baseline-style root state attrs.\"",
        "<Playground title=\"Controlled + on_checked_change\" code_signal=code>",
        "<Playground title=\"State matrix\" code_signal=states_code>",
        "<Switch",
    ] {
        assert!(
            source.contains(needle),
            "forms switch docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn switch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Controlled + on_checked_change\"",
        "checked=checked",
        "set_checked=set_checked",
        "on_checked_change=on_system_checked_change",
        "title=\"State matrix\"",
        "<Switch checked=system_enabled set_checked=set_system_enabled>",
        "<Switch checked=disabled_checked set_checked=set_disabled_checked is_disabled=true>",
        "<Switch checked=disabled_unchecked set_checked=set_disabled_unchecked is_disabled=true>",
        "\"last on_checked_change: \"",
    ] {
        assert!(
            source.contains(needle),
            "forms switch docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
