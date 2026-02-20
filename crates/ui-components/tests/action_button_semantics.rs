use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/button/") {
        let migrated = manifest_dir
            .join("../../components/button/src")
            .join(suffix);
        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_button_has_no_compat_module_and_is_reexported_from_button_action() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub use button::action::{",
        "ActionButton",
        "ActionButtonLoadingPlacement",
        "ActionButtonMotion",
        "ActionButtonSize",
        "ActionButtonType",
    ] {
        assert!(
            source.contains(needle),
            "crate re-exports should include `{needle}` from button/action."
        );
    }

    assert!(
        !source.contains("pub mod action_button;"),
        "compat module `src/action_button.rs` should not be reintroduced."
    );
}

#[test]
fn action_button_implementation_lives_under_button_action_module() {
    let mod_source = load_source("src/button/action/mod.rs");
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        "pub type ActionButtonSize = ButtonSize;",
        "pub type ActionButtonLoadingPlacement = ButtonLoadingPlacement;",
        "pub type ActionButtonMotion = ButtonMotion;",
        "pub type ActionButtonType = ButtonType;",
    ] {
        assert!(
            mod_source.contains(needle),
            "button/action module should define `{needle}` as the canonical ActionButton contract."
        );
    }

    assert!(
        view_source.contains("pub fn ActionButton("),
        "ActionButton view should live in `src/button/action/view.rs`."
    );
}

#[test]
fn action_button_spec_boundary_reuses_button_spec_without_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_mod_source = load_source("src/button/mod.rs");
    let action_mod_source = load_source("src/button/action/mod.rs");
    let button_spec = manifest_dir.join("src/button/spec.rs");
    let migrated_button_spec = manifest_dir.join("../../components/button/src/spec.rs");
    let action_spec = manifest_dir.join("src/button/action/spec.rs");
    let migrated_action_spec = manifest_dir.join("../../components/button/src/action/spec.rs");

    assert!(
        button_spec.exists() || migrated_button_spec.exists(),
        "Button should keep the canonical spec.rs boundary for complex schema contract."
    );
    assert!(
        !action_spec.exists() && !migrated_action_spec.exists(),
        "ActionButton should not introduce a parallel spec.rs file."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec contract export `{needle}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "ButtonSpec", "ButtonSchema"] {
        assert!(
            !action_mod_source.contains(forbidden),
            "ActionButton module should stay lightweight and avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_inherits_group_context_when_feature_enabled() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "use_action_button_group_context()",
        "let inherited_disabled = group.map(|ctx| ctx.is_disabled);",
        "let inherited_size = group.map(|ctx| ctx.size);",
        "let inherited_quiet = group.map(|ctx| ctx.is_quiet);",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should inherit group contract via `{needle}` when grouped."
        );
    }
}

#[test]
fn action_button_uses_button_state_machine_and_headless_hooks() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "let view_state = button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "let state = view_state.state;",
        "let render = view_state.render;",
        "let aria = use_button(ButtonOptions {",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should be wired through shared button logic/headless hooks via `{needle}`."
        );
    }
}

#[test]
fn action_button_mounts_headless_contract_in_view_not_logic_layer() {
    let view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/action/logic.rs");

    for needle in [
        "use ui_headless::{",
        "ButtonOptions",
        "FocusRingOptions",
        "HoverOptions",
        "use_button",
        "use_focus_ring",
        "use_hover",
        "let aria = use_button(ButtonOptions {",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButton should mount headless semantic contract in view via `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "ButtonOptions {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionButton logic should not host headless hook wiring token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_motion_reuses_button_contract_and_group_motion_stays_mapping_only() {
    let mod_source = load_source("src/button/action/mod.rs");
    let view_source = load_source("src/button/action/view.rs");
    let action_motion_source = load_source("src/button/action/motion.rs");

    for needle in [
        "pub type ActionButtonMotion = ButtonMotion;",
        "#[prop(optional)] motion: ActionButtonMotion,",
        "button_motion::attach_motion(",
    ] {
        assert!(
            mod_source.contains(needle) || view_source.contains(needle),
            "ActionButton motion should reuse shared Button motion contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct ActionButtonGroupMotion",
        "pub fn sanitize_motion(motion: ActionButtonGroupMotion) -> ActionButtonGroupMotion",
        "pub fn attach_motion(motion: ActionButtonGroupMotion) -> String",
        "\"--ui-action-button-group-motion-duration: {}ms;\"",
    ] {
        assert!(
            action_motion_source.contains(needle),
            "ActionButtonGroup motion should stay as mapping-only contract via `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "request_animation_frame",
        "KeyframeEffect",
        "SpringSolver",
        "unsafe",
    ] {
        assert!(
            !action_motion_source.contains(forbidden),
            "Action motion module should not implement motion engine internals `{forbidden}`."
        );
    }
}

#[test]
fn action_button_theme_contract_reuses_ui_theme_tokens_and_button_surface() {
    let view_source = load_source("src/button/action/view.rs");
    let styles_source = load_source("src/button/action/styles.rs");
    let theme_tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "let view_state = button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "let class = view_state.class_name;",
        "class:ui-button--focus-visible",
        "data-color=state.color_attr",
        "data-radius=state.radius_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButton should reuse Button theme surface contract via `{needle}`."
        );
    }

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-accent)",
        "var(--ui-accent-soft)",
        "var(--ui-border)",
        "var(--ui-bg)",
        "var(--ui-radius-sm)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Action button-family styles should consume ui-theme token variable `{needle}`."
        );
    }

    for needle in [
        "pub struct SemanticColorTokens",
        "pub struct ThemeContext",
        "fn resolve_tokens(ctx: ThemeContext) -> ThemeTokens {",
        "--ui-system:",
        "--ui-color:",
        "--ui-scale:",
    ] {
        assert!(
            theme_tokens_source.contains(needle)
                || theme_theme_source.contains(needle)
                || theme_css_source.contains(needle),
            "ui-theme should remain the source of theme-axis/token contract `{needle}`."
        );
    }
}

#[test]
fn action_button_token_first_styles_are_gated_and_injected_via_ui_root() {
    let action_styles_source = load_source("src/button/action/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let action_view_source = load_source("src/button/action/view.rs");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-space-2xs)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-accent)",
        "var(--ui-accent-soft)",
        "var(--ui-border)",
        "var(--ui-bg)",
        "var(--ui-radius-sm)",
        "pub const ACTION_BUTTON_GROUP_CSS: &str = r#\"",
        "pub const ACTION_GROUP_CSS: &str = r#\"",
    ] {
        assert!(
            action_styles_source.contains(needle),
            "Action styles should stay token-first/static via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-action_button_group\")]",
        "out.push_str(crate::button::action::styles::ACTION_BUTTON_GROUP_CSS);",
        "#[cfg(feature = \"component-action_group\")]",
        "out.push_str(crate::button::action::styles::ACTION_GROUP_CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Action styles should be feature-gated in css aggregation via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] inject_components_css: bool,",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should own component css injection path via `{needle}`."
        );
    }

    for needle in [
        "let panel_vars = action_motion::attach_motion(motion);",
        "style=panel_vars",
    ] {
        assert!(
            action_view_source.contains(needle),
            "ActionButtonGroup runtime style should be limited to motion css vars via `{needle}`."
        );
    }

    for forbidden in ["class=\"flex", "class=\"grid", "tw-", "tailwind"] {
        assert!(
            !action_styles_source.contains(forbidden) && !action_view_source.contains(forbidden),
            "Action component contract should not depend on utility-first styling token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_family_a11y_and_i18n_fallbacks_are_context_driven() {
    let view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/action/logic.rs");
    let common_strings_source = load_source("../../crates/ui-headless/src/i18n/common.rs");

    for needle in [
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "icon_only_fallback_aria_label: Some(common_strings.icon_button_aria_label.to_string())",
        "common_strings.action_button_group_aria_label.as_ref()",
        "common_strings.action_group_aria_label.as_ref()",
        "role=\"toolbar\"",
        "aria-label=aria_label",
        "let aria = use_button(ButtonOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Action button family should route A11y/i18n fallback via context-driven contract `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_aria_label(",
        "fallback_aria_label: &str",
        "(fallback_aria_label.to_string(), false)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Action button-family logic should expose fallback-aware A11y normalization `{needle}`."
        );
    }

    for needle in [
        "pub action_button_group_aria_label: Arc<str>,",
        "pub action_group_aria_label: Arc<str>,",
    ] {
        assert!(
            common_strings_source.contains(needle),
            "ui-headless CommonStrings should expose action family i18n keys `{needle}`."
        );
    }

    for forbidden in ["\"Action button group\"", "\"Action group\""] {
        assert!(
            !view_source.contains(forbidden),
            "Action button-family view should not hardcode fallback copy `{forbidden}`."
        );
    }
}

#[test]
fn action_button_family_stays_in_ui_components_assembly_layer() {
    let view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/action/logic.rs");
    let motion_source = load_source("src/button/action/motion.rs");
    let styles_source = load_source("src/button/action/styles.rs");

    for needle in [
        "action_logic::action_button_logic::resolve_input(",
        "button_logic::normalize_input(",
        "button_logic::resolve_view_state(",
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
        "button_motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "Action button family view should stay as assembly layer via `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "on:pointerdown",
        "on:keydown",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Action button family logic should not host view/headless event wiring `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: ActionButtonGroupMotion) -> ActionButtonGroupMotion",
        "pub fn attach_motion(motion: ActionButtonGroupMotion) -> String",
    ] {
        assert!(
            motion_source.contains(needle),
            "Action button family motion module should stay contract-mapping via `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "unsafe",
        "request_animation_frame",
        "KeyframeEffect",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Action button family motion module should not embed engine/runtime internals `{forbidden}`."
        );
    }

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-fg)",
        "var(--ui-border)",
        "var(--ui-bg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Action button family styles should consume theme tokens via `{needle}`."
        );
    }
}

#[test]
fn action_button_state_primitives_are_consumed_via_button_logic() {
    let action_view_source = load_source("src/button/action/view.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let state_primitives_source = load_source("../ui-state-primitives/src/button.rs");

    for needle in [
        "let view_state = button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "let state = view_state.state;",
    ] {
        assert!(
            action_view_source.contains(needle),
            "ActionButton should consume shared button state pipeline via `{needle}`."
        );
    }

    for needle in [
        "use ui_state_primitives::button::{",
        "ButtonStateCoreInput",
        "resolve_state_core",
        "let core = resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            button_logic_source.contains(needle),
            "Shared button logic should source state primitive contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct ButtonStateCoreInput",
        "pub fn resolve_state_core(input: ButtonStateCoreInput) -> ButtonStateCore",
    ] {
        assert!(
            state_primitives_source.contains(needle),
            "ui-state-primitives should expose button state primitive via `{needle}`."
        );
    }
}

#[test]
fn action_button_styles_depend_on_explicit_state_markers_not_dom_shape() {
    let styles_source = load_source("src/button/action/styles.rs");
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        ".ui-action-group[data-tone=\"default\"]",
        ".ui-action-group[data-tone=\"quiet\"]",
        ".ui-action-group[data-tone=\"strong\"]",
        ".ui-action-group[data-disabled=\"true\"]",
        ".ui-action-group[data-has-selection=\"true\"]",
        ".ui-action-group[data-selection-mode=\"single\"] .ui-action-group__item",
        ".ui-action-group[data-selection-mode=\"multiple\"] .ui-action-group__item",
        ".ui-action-group[data-selection-mode=\"none\"] .ui-action-group__item",
        ".ui-action-group__item[data-selected=\"true\"]",
        ".ui-action-group__item[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Action button-family styles should key off explicit state markers via `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-loading=state.is_loading.then_some(\"true\")",
        "data-loading-placement=state.loading_placement_attr",
        "data-quiet=is_quiet.then_some(\"true\")",
        "data-selection-mode=move || state.get().selection_mode_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-selected=is_selected.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Action button-family view should expose marker-driven state contract `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Action button-family styles should avoid brittle DOM-structure selector `{forbidden}`."
        );
    }
}

#[test]
fn action_button_normalization_reuses_button_contract() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "icon_only_fallback_aria_label: Some(common_strings.icon_button_aria_label.to_string())",
        "button_type: button_type.unwrap_or_default()",
        "let button_type = normalized.button_type;",
        "let class = view_state.class_name;",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should reuse shared Button normalization path via `{needle}`."
        );
    }
}

#[test]
fn action_button_default_priority_is_centralized_in_logic_module() {
    let view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/action/logic.rs");

    for needle in [
        "pub(crate) mod action_button_logic {",
        "pub struct ActionButtonInputResolutionInput",
        "pub struct ActionButtonResolvedInput",
        "pub fn resolve_input(input: ActionButtonInputResolutionInput) -> ActionButtonResolvedInput",
        "let is_disabled = input",
        ".is_disabled",
        ".or(input.inherited_disabled)",
        ".unwrap_or(false);",
        "let size = input.size.or(input.inherited_size).unwrap_or_default();",
        "let is_quiet = input.is_quiet.or(input.inherited_quiet).unwrap_or(false);",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionButton logic should centralize default-priority rules via `{needle}`."
        );
    }

    for needle in [
        "let resolved = action_logic::action_button_logic::resolve_input(",
        "ActionButtonInputResolutionInput {",
        "is_disabled: resolved.is_disabled",
        "size: resolved.size",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButton view should consume centralized resolution output `{needle}`."
        );
    }
}

#[test]
fn action_button_loading_render_rules_reuse_button_render_state() {
    let source = load_source("src/button/action/view.rs");
    let button_source = load_source("src/button/view.rs");

    for needle in [
        "let render = view_state.render;",
        "{button_view::render_button_content(state, render, start_content, end_content, children)}",
        "let button_type = normalized.button_type;",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should reuse shared Button render rules via `{needle}`."
        );
    }

    assert!(
        button_source.contains("pub(crate) fn render_button_content("),
        "Button view should expose shared render helper for button-family reuse."
    );
}

#[test]
fn action_button_loading_semantics_reuse_button_contract_without_custom_async_protocol() {
    let view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/action/logic.rs");

    for needle in [
        "#[prop(optional)] is_loading: bool",
        "let view_state = button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "is_loading,",
        "data-loading=state.is_loading.then_some(\"true\")",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "is_disabled: state.is_disabled,",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButton loading semantics should reuse shared Button contract via `{needle}`."
        );
    }

    for forbidden in ["use_async_action", "on_retry", "is_error", "error_message"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ActionButton should not define a local async protocol field `{forbidden}`."
        );
    }
}

#[test]
fn action_button_api_naming_uses_is_prefix_only() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "#[prop(optional)] is_loading: bool",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_quiet: Option<bool>",
        "#[prop(optional)] is_icon_only: bool",
        "pub fn ActionButtonGroup(",
        "pub fn ActionGroup(",
        "#[prop(optional)] is_disabled: bool",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton API naming should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
    ] {
        assert!(
            !source.contains(forbidden),
            "Action button family should not expose legacy boolean alias `{forbidden}`."
        );
    }
}

#[test]
fn action_button_discrete_inputs_are_enum_constrained() {
    let mod_source = load_source("src/button/action/mod.rs");
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        "pub type ActionButtonSize = ButtonSize;",
        "pub type ActionButtonLoadingPlacement = ButtonLoadingPlacement;",
        "pub type ActionButtonType = ButtonType;",
    ] {
        assert!(
            mod_source.contains(needle),
            "ActionButton discrete contract should be typed via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] size: Option<ActionButtonSize>",
        "#[prop(optional)] loading_placement: ActionButtonLoadingPlacement",
        "#[prop(optional, into)] button_type: Option<ActionButtonType>",
        "#[prop(optional)] density: ActionButtonGroupDensity",
        "#[prop(optional)] orientation: ActionButtonGroupOrientation",
        "#[prop(optional)] tone: ActionGroupTone",
        "#[prop(optional)] selection_mode: ActionGroupSelectionMode",
    ] {
        assert!(
            view_source.contains(needle),
            "Action button-family discrete inputs should use enums via `{needle}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "loading_placement: Option<String>",
        "density: Option<String>",
        "orientation: Option<String>",
        "tone: Option<String>",
        "selection_mode: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Action button-family should not model discrete state with strings: `{forbidden}`."
        );
    }
}

#[test]
fn action_button_machine_readable_state_contract_is_typed_and_marker_driven() {
    let mod_source = load_source("src/button/action/mod.rs");
    let logic_source = load_source("src/button/action/logic.rs");
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        "pub type ActionButtonSize = ButtonSize;",
        "pub type ActionButtonLoadingPlacement = ButtonLoadingPlacement;",
        "pub type ActionButtonType = ButtonType;",
        "pub enum ActionGroupTone",
        "pub enum ActionGroupSelectionMode",
    ] {
        assert!(
            mod_source.contains(needle) || logic_source.contains(needle),
            "Action button-family should keep key input/state axes typed via `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-loading=state.is_loading.then_some(\"true\")",
        "data-label-source=aria_label_source.as_attr()",
        "data-selection-mode=move || state.get().selection_mode_attr",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Action button-family should expose machine-readable semantic marker `{needle}`."
        );
    }

    for needle in [
        "pub fn as_attr(self) -> &'static str {",
        "ActionGroupSelectionMode::Single => \"single\"",
        "ActionGroupSelectionMode::Multiple => \"multiple\"",
        "ActionGroupSelectionMode::None => \"none\"",
        "ActionGroupTone::Default => \"default\"",
        "ActionGroupTone::Quiet => \"quiet\"",
        "ActionGroupTone::Strong => \"strong\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "Action button-family marker values should come from closed enum mappings `{needle}`."
        );
    }
}

#[test]
fn action_button_emits_semantic_slot_and_loading_attributes() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "data-slot=\"action-button\"",
        "data-loading=state.is_loading.then_some(\"true\")",
        "data-loading-placement=state.loading_placement_attr",
        "data-quiet=is_quiet.then_some(\"true\")",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should expose semantic/loading attrs via `{needle}`."
        );
    }
}

#[test]
fn action_button_dx_paradox_keeps_default_usage_simple_and_advanced_optional() {
    let view_source = load_source("src/button/action/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    let signature_start = view_source
        .find("pub fn ActionButton(")
        .expect("ActionButton signature should exist.");
    let signature_tail = &view_source[signature_start..];
    let signature_end = signature_tail
        .find("children: Children,")
        .expect("ActionButton signature should include children parameter.");
    let signature = &signature_tail[..signature_end + "children: Children,".len()];

    for needle in [
        "#[prop(optional)] is_loading: bool",
        "#[prop(optional, into)] start_content: Option<ViewFn>",
        "#[prop(optional, into)] end_content: Option<ViewFn>",
        "#[prop(optional)] motion: ActionButtonMotion",
        "#[prop(optional)] loading_placement: ActionButtonLoadingPlacement",
    ] {
        assert!(
            signature.contains(needle),
            "ActionButton default API should keep advanced control optional via `{needle}`."
        );
    }

    assert!(
        !signature.contains("state:"),
        "ActionButton baseline usage must not require internal state object wiring."
    );

    let docs_section_start = docs_source
        .find("pub(super) fn action_button() -> AnyView")
        .expect("docs should contain action_button page section.");
    let docs_section = &docs_source[docs_section_start..];
    let code_block_start = docs_section
        .find("let code = Signal::derive(move || {")
        .expect("docs should define minimal ActionButton code snippet.");
    let code_block_end = docs_section
        .find("let states_code = Signal::derive(move || {")
        .expect("docs should define advanced states snippet.");
    let code_block = &docs_section[code_block_start..code_block_end];

    let snippet_start = code_block
        .find("r#\"<ActionButton")
        .expect("minimal ActionButton snippet should start with ActionButton.");
    let snippet_tail = &code_block[snippet_start..];
    let snippet_end = snippet_tail
        .find("</ActionButton>\"#")
        .expect("minimal ActionButton snippet should close ActionButton.");
    let snippet = &snippet_tail[..snippet_end + "</ActionButton>\"#".len()];

    let non_empty_lines = snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        non_empty_lines <= 5,
        "ActionButton Hello World snippet should stay within 5 non-empty lines, got {non_empty_lines}."
    );

    for needle in [
        "<ActionButton",
        "on_press=Callback::new(move |_| {})",
        "\"Action\"",
        "</ActionButton>",
    ] {
        assert!(
            snippet.contains(needle),
            "ActionButton docs minimal path should include `{needle}`."
        );
    }
}

#[test]
fn action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_button() -> AnyView",
        "title=\"ActionButton Workbench\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/code/personal/omne/rust-ui/crates/ui-components/src/button/styles.rs\".to_string()",
        "ui_components::button::styles::CSS",
        "scoped css live-edit",
    ] {
        assert!(
            source.contains(needle),
            "action-button docs workbench should keep CSS hot-reload marker `{needle}`."
        );
    }
}

#[test]
fn action_button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "const ACTION_BUTTON_WORKBENCH_STORAGE_KEY: &str = \"docs:action-button:workbench:state\";",
        "fn load_action_button_workbench_state() -> Option<ActionButtonWorkbenchState>",
        "fn save_action_button_workbench_state(state: ActionButtonWorkbenchState)",
        "fn clear_action_button_workbench_state()",
        "save_action_button_workbench_state(ActionButtonWorkbenchState {",
        "clear_action_button_workbench_state();",
        "data-slot=\"action-button-workbench-controls\"",
        "data-slot=\"action-button-workbench\"",
        "data-slot=\"action-button-workbench-canvas\"",
        "\"Persist workbench state\"",
    ] {
        assert!(
            source.contains(needle),
            "action-button dx workbench should include `{needle}`."
        );
    }
}

#[test]
fn action_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_button() -> AnyView",
        "title=\"ActionButton\"",
        "slug=\"action-button\"",
        "<ActionButton",
        "is_quiet=true",
        "is_loading=true",
        "loading_placement=ActionButtonLoadingPlacement::Center",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for action-button coverage."
        );
    }
}

#[test]
fn action_button_implementation_covers_reduced_motion_ssr_and_wasm_paths() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_web = load_source("../../crates/ui-motion/src/web.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let action_view_source = load_source("src/button/action/view.rs");

    for needle in [
        "pub fn prefers_reduced_motion() -> bool",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            ui_motion_web.contains(needle),
            "ui-motion web backend should include reduced-motion downgrade `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should expose explicit wasm/non-wasm branch `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "button motion should keep wasm enhancement and non-wasm safe fallback `{needle}`."
        );
    }

    assert!(
        action_view_source.contains("button_motion::attach_motion("),
        "ActionButton should reuse shared Button motion branch handling instead of reimplementing platform branches."
    );
}

#[test]
fn action_button_respects_ui_headless_web_ssr_mutex_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let action_view_source = load_source("src/button/action/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutex compile contract `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "ButtonOptions",
        "FocusRingOptions",
        "HoverOptions",
        "use_button",
        "use_focus_ring",
        "use_hover",
    ] {
        assert!(
            action_view_source.contains(needle),
            "ActionButton should consume ui-headless contract via `{needle}`."
        );
    }
}

#[test]
fn action_button_reuses_button_motion_and_ui_motion_has_non_wasm_stub() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let action_group_motion_source = load_source("src/button/action/motion.rs");

    for needle in [
        "//! - Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm no-op/stub contract `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "Button motion should keep wasm/non-wasm safe split contract `{needle}`."
        );
    }

    assert!(
        action_view_source.contains("button_motion::attach_motion("),
        "ActionButton should reuse Button motion attach path to inherit non-wasm stub behavior."
    );

    for forbidden in ["web_sys::", "request_animation_frame", "KeyframeEffect"] {
        assert!(
            !action_group_motion_source.contains(forbidden),
            "ActionButton group motion mapping should avoid runtime motion engine token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_ssr_and_cross_platform_compile_paths_are_covered() {
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let action_motion_source = load_source("src/button/action/motion.rs");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "ui-headless web+ssr must fail",
        "mutually exclusive",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform check script should keep compile-only and mutex guard contract `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "window(",
        "document(",
        "js_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !action_view_source.contains(forbidden),
            "ActionButton view should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
        assert!(
            !action_logic_source.contains(forbidden),
            "ActionButton logic should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
        assert!(
            !action_motion_source.contains(forbidden),
            "ActionButton motion mapping should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_wasm_debug_contract_reuses_button_debug_and_keeps_feature_isolated() {
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let button_view_source = load_source("src/button/view.rs");
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "default = [\"inject-css\", \"all-components\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components feature contract should keep `{needle}`."
        );
    }

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button wasm debug contract should keep `{needle}` for source/time/before/after and replay."
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
            "docs visual debug entry should keep `{needle}`."
        );
    }

    for needle in [
        "button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "button_motion::attach_motion(",
        "button_view::render_button_content(",
    ] {
        assert!(
            action_view_source.contains(needle),
            "ActionButton should reuse shared Button capabilities via `{needle}`."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "record_transition(",
        "render_debug_panel(",
        "data-debug-source",
    ] {
        assert!(
            !action_view_source.contains(forbidden),
            "ActionButton view should not duplicate button debug runtime token `{forbidden}`."
        );
        assert!(
            !action_logic_source.contains(forbidden),
            "ActionButton logic should not duplicate button debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn action_button_view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "fn render_action_group_items(",
        "fn render_action_group_item(",
        "render_action_group_items(",
        "render_action_group_item(",
        ".collect_view()",
    ] {
        assert!(
            source.contains(needle),
            "Action button family view should keep macro complexity split marker `{needle}`."
        );
    }
}

#[test]
fn action_button_functional_split_prefers_plain_functions_over_extra_components() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "fn render_action_group_items(",
        "fn render_action_group_item(",
        ") -> impl IntoView {",
        "data-slot=\"action-group-node\"",
        "data-slot=\"action-group-item\"",
    ] {
        assert!(
            source.contains(needle),
            "ActionGroup view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_action_group_items(",
        "#[component]\nfn render_action_group_item(",
        "#[component]\npub fn ActionGroupItem(",
    ] {
        assert!(
            !source.contains(forbidden),
            "ActionGroup local fragments should stay plain functions, not extra components `{forbidden}`."
        );
    }
}

#[test]
fn action_button_static_fragments_are_constantized_for_action_group_items() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "const ACTION_GROUP_ITEM_CLASS_BASE: &str = \"ui-action-group__item\";",
        "const ACTION_GROUP_ITEM_CLASS_SELECTED: &str = \" ui-action-group__item--selected\";",
        "const ACTION_GROUP_ITEM_CLASS_DISABLED: &str = \" ui-action-group__item--disabled\";",
        "{ACTION_GROUP_ITEM_CLASS_BASE}{}{}",
        "ACTION_GROUP_ITEM_CLASS_SELECTED",
        "ACTION_GROUP_ITEM_CLASS_DISABLED",
    ] {
        assert!(
            source.contains(needle),
            "ActionGroup item static fragment should keep constantized marker `{needle}`."
        );
    }
}

#[test]
fn action_button_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let view_source = load_source("src/button/action/view.rs");
    let docs_actions_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let docs_actions_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    assert!(
        !view_source.contains("inner_html"),
        "Action button family components should not use `inner_html`; keep rendering explicit and safe."
    );
    assert!(
        !docs_actions_source.contains("inner_html"),
        "Action docs examples should not demonstrate `inner_html` injection in actions.rs."
    );
    assert!(
        !docs_actions_extra_source.contains("inner_html"),
        "Action docs examples should not demonstrate `inner_html` injection in actions_extra.rs."
    );
}

#[test]
fn action_button_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks() {
    let cargo_source = load_source("Cargo.toml");
    let action_mod_source = load_source("src/button/action/mod.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let action_motion_source = load_source("src/button/action/motion.rs");
    let button_mod_source = load_source("src/button/mod.rs");
    let button_view_source = load_source("src/button/view.rs");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "action engineering contract should keep feature boundary marker `{needle}`."
        );
    }

    for needle in [
        "button_logic::resolve_view_state(button_logic::ButtonLogicInput {",
        "button_motion::attach_motion(",
        "button_view::render_button_content(",
    ] {
        assert!(
            action_view_source.contains(needle),
            "ActionButton should reuse Button engineering capability marker `{needle}`."
        );
    }

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "ActionButton should rely on canonical Button spec boundary marker `{needle}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "ButtonSpec", "ButtonSchema"] {
        assert!(
            !action_mod_source.contains(forbidden),
            "ActionButton should avoid local spec/serde boundary token `{forbidden}`."
        );
    }

    for needle in [
        "target: \"ui_components::button::state_change\"",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button tracing/debug contract should provide `{needle}` for ActionButton reuse path."
        );
    }

    for source in [
        &action_mod_source,
        &action_view_source,
        &action_logic_source,
        &action_motion_source,
    ] {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async-std",
            "async_std::",
            "runtime::Handle",
        ] {
            assert!(
                !source.contains(forbidden),
                "ActionButton engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !action_mod_source.contains("web_sys"),
        "ActionButton public module boundary should not leak web_sys types."
    );
}

#[test]
fn action_button_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("src/button/action/check2.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "\"action-button\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(9.0),",
        "max_heap_kb: Some(448.0),",
        "\"action-button-group\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
        "\"action-group\" => UiPerfBudget {",
        "max_mount_ms: 38.0,",
        "max_update_ms: Some(14.0),",
        "max_heap_kb: Some(768.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "action family page should keep performance budget contract `{needle}`."
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
            "docs e2e coverage should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"ActionButton\", \"action-button\", \"Actions\", a::action_button)",
        "\"action-button-group\"",
        "\"action-group\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "action family docs pages should remain in coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "action performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    let needle = "cargo test -p ui-components --test action_button_semantics action_button_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(needle),
        "performance gate script should include `{needle}`."
    );
}

#[test]
fn action_button_check2_is_marked_complete() {
    let source = load_source("src/button/action/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "button/action/check2.md should not keep unchecked checklist items after completion."
    );
}

#[test]
fn action_button_check2_explicitly_records_button_based_design() {
    let source = load_source("src/button/action/check2.md");

    for needle in [
        "button组件的扩展组件",
        "它应该使用button组件的能力而非重新实现",
        "button_logic::resolve_view_state",
        "pub type ActionButtonMotion = ButtonMotion",
    ] {
        assert!(
            source.contains(needle),
            "button/action/check2.md should include button-based contract marker `{needle}`."
        );
    }
}
