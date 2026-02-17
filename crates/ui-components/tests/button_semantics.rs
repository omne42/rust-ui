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

fn collect_spec_files(root: &Path, base: &Path, out: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_spec_files(&path, base, out);
                continue;
            }

            if path.file_name().and_then(|name| name.to_str()) == Some("spec.rs")
                && let Ok(rel) = path.strip_prefix(base)
            {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

#[test]
fn button_does_not_expose_logic_module() {
    let source = load_source("src/button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Button's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Button should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "data-slot=SLOT_BUTTON",
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-hovered",
        "data-pressed",
        "data-loading",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
        "data-loading-placement",
        "data-motion-source=if state.has_custom_motion",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-ui-schema=schema_json",
    ] {
        assert!(
            source.contains(attr),
            "Button should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn button_exposes_agent_capabilities_for_machine_consumers() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "ButtonAgentContract",
        "ButtonAgentCapabilities",
        "resolve_agent_contract(state, has_popup_trigger)",
        "data-ui-agent-schema=agent_contract.schema_name",
        "data-ui-agent-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-capability-press=agent_contract.capabilities.can_press.then_some(\"true\")",
        "data-ui-capability-focus=agent_contract.capabilities.can_focus.then_some(\"true\")",
        "data-ui-capability-hover=agent_contract.capabilities.can_hover.then_some(\"true\")",
        "data-ui-capability-popup-trigger=agent_contract",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Button should expose typed agent capability contract marker `{needle}`."
        );
    }
}

#[test]
fn button_forwards_headless_button_semantics() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Button should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn button_mounts_popup_a11y_contract_from_ui_headless() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "popup_trigger_attrs(",
        "aria-haspopup=popup_a11y.aria_haspopup",
        "aria-controls=move || popup_a11y.aria_controls.get()",
        "aria-expanded=move || popup_a11y.aria_expanded.get()",
        "lang=popup_a11y.lang.clone()",
        "dir=popup_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Button should mount headless popup a11y contract via `{needle}`."
        );
    }
}

#[test]
fn button_a11y_and_i18n_entrypoints_are_wired_via_headless_contracts() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "icon_only_fallback_aria_label: Some(common_strings.icon_button_aria_label.to_string())",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "lang=popup_a11y.lang.clone()",
        "dir=popup_a11y.dir",
        "popup_trigger_attrs(",
    ] {
        assert!(
            view_source.contains(needle),
            "Button should expose a11y/i18n integration point via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("input.icon_only_fallback_aria_label"),
        "Button logic should consume i18n-provided fallback aria label through typed normalization input.",
    );
}

#[test]
fn button_loading_forces_disabled_and_sets_aria_busy() {
    let source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    assert!(
        source.contains("resolve_view_state(") && logic_source.contains("pub fn resolve_state("),
        "Button should derive view data via `resolve_view_state` while keeping `resolve_state` testable in logic.rs."
    );

    for needle in [
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Button should wire loading/disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn button_async_contract_is_externalized_and_has_no_internal_retry_protocol() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");

    for required in [
        "#[prop(optional)] is_loading: bool",
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "is_loading: input.is_loading,",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "Button async mapping should include `{required}`."
        );
    }

    for forbidden in [
        "is_error",
        "error_message",
        "on_retry",
        "retry_count",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !spec_source.contains(forbidden),
            "Button should not define component-local async error/retry protocol `{forbidden}`.",
        );
    }
}

#[test]
fn button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button/styles.rs");
    let motion = load_source("src/button/motion.rs");

    for needle in [
        "--ui-button-scale",
        "transform: scale(var(--ui-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "Button styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("--ui-button-scale"),
        "Button motion should write `--ui-button-scale` to drive interaction feedback without triggering rerenders."
    );
}

#[test]
fn button_spinner_respects_reduced_motion() {
    let styles = load_source("src/button/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "Button spinner should disable its CSS animation under reduced-motion via `{needle}`."
        );
    }
}

#[test]
fn button_styles_consume_theme_layout_variables() {
    let styles = load_source("src/button/styles.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");

    for var_name in [
        "--ui-button-min-width",
        "--ui-button-font-size",
        "--ui-button-spinner-size",
        "--ui-button-spinner-border",
        "--ui-button-spinner-duration",
        "--ui-button-focus-outline-width",
        "--ui-button-focus-outline-offset",
        "--ui-button-radius-full",
        "--ui-button-size-xs-height",
        "--ui-button-size-s-height",
        "--ui-button-size-m-height",
        "--ui-button-size-l-height",
        "--ui-button-size-xl-height",
    ] {
        assert!(
            styles.contains(var_name),
            "Button styles should consume ui-theme variable `{var_name}` instead of hard-coded layout values."
        );
        assert!(
            theme_css.contains(var_name),
            "ui-theme css emitter should export `{var_name}` for button layout tokens."
        );
    }

    for legacy_literal in [
        "min-width: 80px;",
        "font-size: 14px;",
        "width: 16px;",
        "height: 24px;",
        "height: 28px;",
        "height: 32px;",
        "height: 36px;",
        "height: 40px;",
        "outline: 3px solid var(--ui-focus-ring);",
        "outline-offset: 2px;",
    ] {
        assert!(
            !styles.contains(legacy_literal),
            "Button styles should not keep legacy literal `{legacy_literal}` after token downshift.",
        );
    }
}

#[test]
fn button_styles_flow_through_css_registry_and_ui_root_injection() {
    let css_registry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(needle),
            "Button style contract should be aggregated via css registry marker `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject component styles via `{needle}`."
        );
    }
}

#[test]
fn button_component_layer_avoids_utility_first_and_css_in_rust_defaults() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let motion_source = load_source("src/button/motion.rs");

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "class=\"w-",
        "class=\"h-",
        "class=\"gap-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button component should not use utility-first contract marker `{forbidden}`."
        );
    }

    for forbidden in ["tailwind", "tw!", "css!(", "style!(", "styled!(", "emotion"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Button component should not adopt CSS-in-Rust/utility-first default marker `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-button-scale\""),
        "Button runtime style path should only update semantic css variable `--ui-button-scale`."
    );
    assert_eq!(
        motion_source.matches("set_property(\"--ui-button-").count(),
        1,
        "Button runtime style path should remain minimal and only write one `--ui-button-*` variable.",
    );
}

#[test]
fn button_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/styles.rs");

    for selector in [
        ".ui-button[data-motion-source=\"custom\"]",
        ".ui-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Button styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_styles_use_semantic_state_markers_and_avoid_fragile_selectors() {
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");

    for required_selector in [
        ".ui-button[data-loading=\"true\"][data-loading-placement=\"center\"] .ui-button__label",
        ".ui-button[data-loading=\"true\"][data-loading-placement=\"start\"]:not([data-has-start=\"true\"])",
        ".ui-button__start[data-loading-start=\"true\"] .ui-button__start-content",
        ".ui-button[data-hovered=\"true\"]:not(:disabled).ui-button--variant-default",
    ] {
        assert!(
            styles_source.contains(required_selector),
            "Button styles should branch visual state via semantic selector `{required_selector}`.",
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", ":has("] {
        assert!(
            !styles_source.contains(forbidden),
            "Button styles should avoid fragile structural selector `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("style="),
        "Button view should not push business style logic through inline styles.",
    );
}

#[test]
fn button_semantic_contract_test_matrix_covers_required_branches() {
    let semantics_source = load_source("tests/button_semantics.rs");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "fn button_forwards_headless_button_semantics()",
        "fn button_emits_baseline_style_data_attributes()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_loading_forces_disabled_and_sets_aria_busy()",
        "fn button_has_no_half_controlled_state_axes()",
        "fn button_uses_headless_press_hover_and_focus_ring()",
        "fn ui_motion_stays_component_agnostic_and_non_wasm_safe()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic matrix should include branch coverage test `{required}`.",
        );
    }

    for required in [
        "on:pointerdown=on_pointer_down",
        "on:keydown=on_key_down",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=state.state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button view should expose semantic contract surface `{required}`.",
        );
    }
}

#[test]
fn button_semantics_tests_do_not_depend_on_visual_snapshot_assertions() {
    let semantics_source = load_source("tests/button_semantics.rs");
    let snapshot_tokens = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        [".", "snap"].concat(),
    ];

    for forbidden in snapshot_tokens {
        assert!(
            !semantics_source.contains(&forbidden),
            "Button semantic contract tests should not depend on visual snapshot marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/motion.rs");

    for needle in [
        "use ui_theme::default_button_motion_tokens;",
        "let tokens = default_button_motion_tokens();",
        "pub fn sanitize_motion(motion: ButtonMotion) -> ButtonMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "ui_motion::spring::SpringAnimator::new(",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            source.contains(needle),
            "Button motion should include `{needle}` so invalid custom motion values cannot leak into runtime animation behavior.",
        );
    }
}

#[test]
fn ui_motion_stays_component_agnostic_and_non_wasm_safe() {
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let motion_spring = load_source("../../crates/ui-motion/src/spring.rs");
    let motion_keyframes = load_source("../../crates/ui-motion/src/keyframes.rs");
    let motion_web = load_source("../../crates/ui-motion/src/web.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "pub struct SpringAnimator",
    ] {
        assert!(
            motion_lib.contains(needle)
                || motion_spring.contains(needle)
                || motion_keyframes.contains(needle)
                || motion_web.contains(needle),
            "ui-motion should keep runtime/math/no-op contracts via `{needle}`."
        );
    }

    for forbidden in ["ui-button", "ui-accordion", "aria-", "slot"] {
        assert!(
            !motion_lib.contains(forbidden)
                && !motion_spring.contains(forbidden)
                && !motion_keyframes.contains(forbidden)
                && !motion_web.contains(forbidden),
            "ui-motion must stay component-agnostic; found forbidden marker `{forbidden}`."
        );
    }
}

#[test]
fn button_docs_page_covers_button_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "description=\"Variants + sizes with spring hover/tap motion.\"",
        "<Playground",
        "title=\"Variants & sizes\"",
        "title=\"Colors\"",
        "<Button",
        "variant=variant",
        "size=size",
        "is_disabled=is_disabled",
        "is_loading=is_loading",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for button playground coverage.",
        );
    }
}

#[test]
fn button_dx_minimal_hello_world_path_is_documented_and_state_free() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/view.rs");

    for needle in [
        "Playground title=\"Hello world\" code_signal=hello_code",
        "let hello_code = Signal::derive(move || r#\"<Button>\"Button\"</Button>\"#.to_string());",
        "<Button>\"Button\"</Button>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Button docs should expose minimal DX path via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(into)] state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button should not require internal state object prop `{forbidden}` for basic usage.",
        );
    }
}

#[test]
fn button_non_composite_api_avoids_parallel_slot_conventions() {
    let button_view_source = load_source("src/button/view.rs");
    let action_button_source = load_source("src/button/action/view.rs");
    let button_spec_source = load_source("src/button/spec.rs");

    let required = "children: Children,";
    assert!(
        button_view_source.contains(required) && action_button_source.contains(required),
        "Button and ActionButton should use explicit child composition via `{required}`."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "ButtonItemSpec",
        "ActionButtonItemSpec",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !action_button_source.contains(forbidden)
                && !button_spec_source.contains(forbidden),
            "Button API should not expose parallel-array/item-spec sugar marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "Button is snapshot-only; forbidden streaming marker `{forbidden}` should not appear."
        );
    }
}

#[test]
fn button_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/button/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "Button 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only）",
    ] {
        assert!(
            check2_source.contains(needle),
            "button/check2.md should pin streaming contract marker `{needle}`."
        );
    }
}

#[test]
fn button_state_derivation_is_consumed_from_ui_state_primitives() {
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "pub use ui_state_primitives::button::{",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Button logic should consume state primitives via `{needle}`."
        );
    }
}

#[test]
fn button_api_naming_uses_is_prefix_only() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_full_width: bool",
        "pub struct ButtonInputNormalizationInput",
        "pub struct ButtonInputNormalization",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "let normalized = logic::normalize_input(logic::ButtonInputNormalizationInput {",
        "is_disabled: normalized.is_disabled,",
        "is_full_width: normalized.is_full_width,",
        "pub fn is_disabled(mut self, value: bool) -> Self",
        "pub fn is_full_width(mut self, value: bool) -> Self",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || spec_source.contains(needle),
            "Button API naming contract should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] full_width: Option<bool>",
        "disabled == Some(true)",
        "full_width == Some(true)",
        "pub fn disabled(self, value: bool) -> Self",
        "pub fn full_width(self, value: bool) -> Self",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !spec_source.contains(forbidden),
            "Button API naming contract should not keep legacy alias marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_default_priority_is_normalized_in_logic_only() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "let (is_disabled, disabled_input_source) = if input.is_disabled {",
        "let (is_full_width, full_width_input_source) = if input.is_full_width {",
        "let class_name = normalize_optional_text(input.class_name);",
        "let (aria_label, aria_label_source) = resolve_aria_label(",
        "input.icon_only_fallback_aria_label,",
        "let button_type = input.button_type;",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic.rs should centralize defaults/priority via `{required}`."
        );
    }

    for forbidden in [
        "disabled.unwrap_or(",
        "full_width.unwrap_or(",
        "normalize_optional_text(",
        "resolve_aria_label(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not apply fallback/default logic directly; found `{forbidden}`."
        );
    }
}

#[test]
fn button_state_observability_uses_closed_semantic_marker_sets() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub enum ButtonBooleanInputSource",
        "ButtonBooleanInputSource::IsProp => \"is-prop\"",
        "ButtonBooleanInputSource::Default => \"default\"",
        "disabled_source_attr: if state.is_loading {",
        "\"loading\"",
        "\"prop\"",
        "\"default\"",
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "Button observability contract should include `{required}`.",
        );
    }
}

#[test]
fn button_discrete_state_inputs_are_type_constrained() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub enum ButtonVariant",
        "pub enum ButtonSize",
        "pub enum ButtonLoadingPlacement",
        "pub enum ButtonType",
        "impl From<&str> for ButtonType",
        "#[prop(optional, into)] button_type: ButtonType",
        "pub button_type: ButtonType,",
        "type=normalized_button_type.as_attr()",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "Button discrete-state contract should include `{required}`."
        );
    }

    for forbidden in [
        "button_type: Option<&'static str>",
        "button_type.unwrap_or(\"button\")",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Button should not keep stringly discrete state marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let semantics_source = load_source("tests/button_semantics.rs");

    for required in [
        "pub enum ButtonVariant",
        "pub enum ButtonColor",
        "pub enum ButtonRadius",
        "pub enum ButtonSize",
        "pub enum ButtonLoadingPlacement",
        "pub enum ButtonType",
        "pub enum ButtonBooleanInputSource",
        "pub struct ButtonStateSource",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "pub fn resolve_view_state(input: ButtonLogicInput) -> ButtonViewState",
        "fn normalize_input_prefers_is_prefix_aliases_and_applies_defaults()",
        "fn normalize_input_uses_is_flags_without_legacy_aliases()",
        "fn resolve_view_state_centralizes_state_and_class_derivation()",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic type/normalization contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] variant: String",
        "#[prop(optional, into)] color: String",
        "#[prop(optional, into)] radius: String",
        "#[prop(optional, into)] size: String",
        "#[prop(optional, into)] loading_placement: String",
        "#[prop(optional, into)] button_type: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button view should not expose untyped string state axis `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional, into)] variant: ButtonVariant",
        "#[prop(optional, into)] color: ButtonColor",
        "#[prop(optional, into)] radius: ButtonRadius",
        "#[prop(optional, into)] size: ButtonSize",
        "#[prop(optional)] loading_placement: ButtonLoadingPlacement",
        "#[prop(optional, into)] button_type: ButtonType",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button view should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "fn button_discrete_state_inputs_are_type_constrained()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_default_priority_is_normalized_in_logic_only()",
        "fn button_semantic_contract_test_matrix_covers_required_branches()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic suite should keep feedback locator `{required}`."
        );
    }
}

#[test]
fn button_status_primitives_boundary_blocks_business_store_bindings() {
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "pub use ui_state_primitives::button::ButtonLabelSource;",
        "pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};",
        "resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic should consume status primitives via `{required}`."
        );
    }

    for forbidden in [
        "use_context(",
        "provide_context(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "create_signal(",
        "leptos::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Button logic should stay store-agnostic and must not contain `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("ui_state_primitives::button::"),
        "Button view should not bypass logic and read status primitives directly.",
    );
}

#[test]
fn button_contract_consistency_has_no_temporary_patch_markers() {
    let sources = [
        load_source("src/button/mod.rs"),
        load_source("src/button/logic.rs"),
        load_source("src/button/view.rs"),
        load_source("src/button/styles.rs"),
        load_source("src/button/motion.rs"),
        load_source("src/button/spec.rs"),
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs"),
    ];

    let merged = sources.join("\n").to_ascii_lowercase();
    for forbidden in [
        "temporary patch",
        "temp patch",
        "quick fix",
        "hotfix",
        "compat shim",
        "remove after release",
    ] {
        assert!(
            !merged.contains(forbidden),
            "button contract should avoid temporary patch marker `{forbidden}`."
        );
    }
}

#[test]
fn button_has_no_half_controlled_state_axes() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    // Button has no component-owned controllable axis (like open/selected/value).
    // It consumes external state and emits `on_press` only.
    for forbidden in [
        "default_open",
        "default_value",
        "default_selected",
        "on_open_change",
        "on_value_change",
        "on_selected_change",
        "set_is_disabled",
        "set_is_loading",
        "set_is_full_width",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Button should not introduce half-controlled state marker `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_loading: bool",
        "#[prop(optional)] is_full_width: bool",
        "#[prop(optional)] on_press: Option<OnPress>",
        "let view_state = logic::resolve_view_state(",
    ] {
        assert!(
            view_source.contains(required),
            "Button should keep external-source-only state flow via `{required}`."
        );
    }
}

#[test]
fn button_layering_matches_ui_components_assembly_contract() {
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let styles_source = load_source("src/button/styles.rs");
    let motion_source = load_source("src/button/motion.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::Button;",
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "use ui_headless::{",
        "logic::resolve_view_state(",
        "motion::attach_motion(",
        "var(--ui-button-",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        let hit = mod_source.contains(needle)
            || logic_source.contains(needle)
            || view_source.contains(needle)
            || styles_source.contains(needle)
            || motion_source.contains(needle);
        assert!(
            hit,
            "Button layering contract evidence `{needle}` should exist in mod/logic/view/styles/motion."
        );
    }

    for forbidden in [
        "use ui_headless::",
        "ui_motion::",
        "leptos::web_sys",
        "on:pointer",
        "aria-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay as assembly/derivation only; found forbidden marker `{forbidden}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not bypass logic and consume primitives directly; found `{forbidden}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:keydown"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not carry view/headless semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn button_component_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");
    let motion_source = load_source("src/button/motion.rs");

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub mod spec;",
        "pub use view::Button;",
    ] {
        assert!(
            mod_source.contains(required),
            "button/mod.rs should keep export boundary marker `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "resolve_state_core(",
        "view! {",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "button/mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_input(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "button/logic.rs should expose derivation contract `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "on:pointer",
        "aria-",
        "data-slot",
        "leptos::web_sys",
        "HtmlElement",
        "style.set_property",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "button/logic.rs should not cross into DOM/view/style runtime logic via `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str", ".ui-button"] {
        assert!(
            styles_source.contains(required),
            "button/styles.rs should keep static CSS contract marker `{required}`."
        );
    }

    for forbidden in ["view! {", "on:pointer", "aria-"] {
        assert!(
            !styles_source.contains(forbidden),
            "button/styles.rs should not carry view/headless logic marker `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::resolve_view_state(",
        "use_button(",
        "motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "button/view.rs should include structure + headless mount marker `{required}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "button/view.rs should not bypass logic layer via `{forbidden}`."
        );
    }

    for required in [
        "pub fn attach_motion(",
        "sanitize_motion(",
        "SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(required),
            "button/motion.rs should include motion-contract attach marker `{required}`."
        );
    }

    for forbidden in ["view! {", "data-slot", "aria-", "on:pointer"] {
        assert!(
            !motion_source.contains(forbidden),
            "button/motion.rs should not include view/headless semantics marker `{forbidden}`."
        );
    }
}

#[test]
fn button_public_surface_does_not_export_web_sys_or_dom_types() {
    let mod_source = load_source("src/button/mod.rs");

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "wasm_bindgen",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Button public API should not expose internal web/DOM details via `{forbidden}`."
        );
    }
}

#[test]
fn button_spec_file_is_scoped_to_complex_schema_contract_and_versioned() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_root = manifest_dir.join("src");
    let mut spec_files = Vec::new();
    collect_spec_files(&src_root, &src_root, &mut spec_files);
    spec_files.sort();

    assert_eq!(
        spec_files,
        vec!["button/spec.rs".to_string()],
        "spec.rs should stay limited to complex components; simple components should not add spec.rs by default.",
    );

    let spec_source = load_source("src/button/spec.rs");
    for required in [
        "pub const BUTTON_SCHEMA_VERSION: u16 = 1;",
        "pub struct ButtonSchema {",
        "pub schema_version: u16,",
        "pub fn schema_version(mut self, value: u16) -> Self",
        "\\\"schema_version\\\":{},",
        "fn schema_json_is_machine_readable()",
        "fn schema_version_normalization_is_stable()",
    ] {
        assert!(
            spec_source.contains(required),
            "button/spec.rs should carry schema contract + version evolution evidence `{required}`."
        );
    }
}

#[test]
fn button_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "BUTTON_WORKBENCH_STORAGE_KEY",
        "fn load_button_workbench_state() -> Option<ButtonWorkbenchState>",
        "fn save_button_workbench_state(state: ButtonWorkbenchState)",
        "fn clear_button_workbench_state()",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "test_css_source=test_css_source",
        "test_config_signal=actual_config",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "Effect::new(move |_| {",
        "save_button_workbench_state(ButtonWorkbenchState {",
        "clear_button_workbench_state();",
        "data-slot=\"button-workbench\"",
        "data-slot=\"button-workbench-canvas\"",
    ] {
        assert!(
            source.contains(needle),
            "Button workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "Button workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn button_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_docs_variants_and_controls_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "code_signal=code",
        "let size_options = vec![",
        "let color_options = vec![",
        "\"primary\".to_string()",
        "\"danger\".to_string()",
        "let radius_options = vec![",
        "\"full\".to_string()",
        "\"none\".to_string()",
        "\"xs\".to_string()",
        "\"s\".to_string()",
        "\"m\".to_string()",
        "\"l\".to_string()",
        "\"xl\".to_string()",
        "size=\"s\"",
        "size=\"m\"",
        "0 => ButtonSize::Xs",
        "1 => ButtonSize::S",
        "2 => ButtonSize::M",
        "3 => ButtonSize::L",
        "_ => ButtonSize::Xl",
        "let loading_placement_options =",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-color\".to_string()",
        "id_base=\"docs-button-radius\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "id_base=\"docs-button-loading-placement\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button color\".to_string()",
        "aria_label=\"Button radius\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "aria_label=\"Button loading placement\".to_string()",
        "<Switch checked=is_disabled set_checked=set_is_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "<Switch checked=icon_only set_checked=set_icon_only>\"Icon only\"</Switch>",
        "<Switch checked=is_full_width set_checked=set_is_full_width>\"Full width\"</Switch>",
        "<Switch checked=show_start set_checked=set_show_start>\"Start slot\"</Switch>",
        "<Switch checked=show_end set_checked=set_show_end>\"End slot\"</Switch>",
    ] {
        assert!(
            source.contains(needle),
            "button docs variants/controls playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "Playground",
        "title=\"Variants & sizes\"",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should contain `{needle}` for Button.",
        );
    }
}

#[test]
fn button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Variants & sizes\"",
        "code_signal=code",
        "let color = color.get();",
        "let radius = radius.get();",
        "let size = size.get();",
        "let is_disabled = is_disabled.get();",
        "let is_loading = loading.get();",
        "let loading_placement = loading_placement.get();",
        "let icon_only = icon_only.get();",
        "let is_full_width = is_full_width.get();",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-color\".to_string()",
        "id_base=\"docs-button-radius\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button color\".to_string()",
        "aria_label=\"Button radius\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "<Switch checked=is_disabled set_checked=set_is_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "color=color",
        "variant=variant",
        "radius=radius",
        "size=size",
        "is_disabled=is_disabled",
        "is_loading=is_loading",
        "loading_placement=loading_placement",
        "is_icon_only=icon_only",
        "is_full_width=is_full_width",
        "let colors_code = Signal::derive(move || {",
        "<Button color=\"default\">\"Default\"</Button>",
        "<Button color=\"primary\">\"Primary\"</Button>",
        "<Button color=\"secondary\">\"Secondary\"</Button>",
        "<Button color=\"success\">\"Success\"</Button>",
        "<Button color=\"warning\">\"Warning\"</Button>",
        "<Button color=\"danger\">\"Danger\"</Button>",
        "title=\"Colors\"",
    ] {
        assert!(
            source.contains(needle),
            "button docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_heroui_strategy_doc_sync_tracks_button_params_and_docs_entrypoint() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_button_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "### Button 同步记录（2026-02-16）",
        "variant/color/radius/size",
        "is_disabled/is_loading/is_icon_only/is_full_width",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/actions.rs",
        "compose_copy_ready_code",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy document should include button sync marker `{needle}`."
        );
    }

    assert!(
        docs_registry_source
            .contains("component_doc!(\"Button\", \"button\", \"Actions\", actions::button)"),
        "docs component registry should expose button entrypoint.",
    );

    for needle in [
        "title=\"Button\"",
        "slug=\"button\"",
        "title=\"Colors\"",
        "title=\"Radius\"",
        "title=\"Sizes\"",
    ] {
        assert!(
            docs_button_page_source.contains(needle),
            "docs button page should keep synced example marker `{needle}`."
        );
    }
}

#[test]
fn button_visual_desire_has_docs_theme_baseline_page_with_key_components() {
    let page_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");

    for needle in [
        "mod theme_visual_baseline;",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            page_registry_source.contains(needle),
            "docs component registry should expose visual baseline route marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should contain `{needle}`."
        );
    }
}

#[test]
fn button_visual_desire_has_e2e_visual_regression_contract() {
    let e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "E2E_VISUAL_BASELINE",
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_source.contains(needle),
            "theme visual baseline e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn button_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "section.playground",
        "[data-slot=\"button-workbench\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"button-workbench-canvas\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button e2e contract should include semantic marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "button e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn button_e2e_key_flow_covers_keyboard_and_code_sync_path() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for needle in [
        "supports keyboard flow and code snapshot sync",
        "loadingSwitch.focus();",
        "page.keyboard.press(\"Space\")",
        "toContainText(\"is_loading=true\")",
        "not.toContainText(\"is_loading=true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button e2e key-flow contract should include `{needle}`."
        );
    }
}

#[test]
fn button_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-accordion = [",
        "component-button = [",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-button\")]\npub mod button;"),
        "lib.rs should feature-gate button module export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-button\")]")
            && css_source.contains("out.push_str(crate::button::styles::CSS);"),
        "css.rs should gate button CSS aggregation behind component-button feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn button_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
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
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn button_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free() {
    let motion_source = load_source("src/button/motion.rs");
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = button.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(needle),
            "button motion should keep explicit platform branch marker `{needle}`."
        );
    }

    let forbidden = "web_sys";
    assert!(
        !mod_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !spec_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden),
        "non-wasm button files should stay browser-object free; found `{forbidden}` outside motion.rs.",
    );
}

#[test]
fn button_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "crates/ui-components/src/button/view.rs",
        "crates/ui-components/src/button/motion.rs",
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn button_ui_headless_feature_mutex_compile_error_guard_is_present() {
    let headless_source = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless should keep feature mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn button_ui_motion_non_wasm_stub_contract_is_enforced() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm stub contract marker `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should enforce ui-motion portability via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "button motion should keep non-wasm safe downgrade marker `{needle}`."
        );
    }

    for forbidden in ["panic!(", ".unwrap()", ".expect("] {
        assert!(
            !button_motion_source.contains(forbidden),
            "button non-wasm motion downgrade path should avoid hard-failure marker `{forbidden}`."
        );
    }
}

#[test]
fn button_reduced_motion_and_ssr_wasm_semantics_contract_is_enforced() {
    let styles_source = load_source("src/button/styles.rs");
    let motion_web_source = load_source("../../crates/ui-motion/src/web.rs");
    let view_source = load_source("src/button/view.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-button__spinner",
        "animation: none",
    ] {
        assert!(
            styles_source.contains(needle),
            "button styles should keep reduced-motion downgrade marker `{needle}`."
        );
    }

    for needle in ["if prefers_reduced_motion() {", "return;"] {
        assert!(
            motion_web_source.contains(needle),
            "ui-motion wasm runtime should skip animation under reduced-motion via `{needle}`."
        );
    }

    for needle in [
        "data-slot=SLOT_BUTTON",
        "data-state=state.state_attr",
        "data-loading=state.is_loading.then_some(\"true\")",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "button SSR output should keep hydration-stable semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
        "window(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "button view semantics should not split by platform marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let element: leptos::web_sys::HtmlElement = button.unchecked_into();",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "button motion enhancement should stay wasm-only while non-wasm remains safe via `{needle}`."
        );
    }

    assert!(
        platform_script_source
            .contains("cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css"),
        "platform script should keep wasm compile-only coverage for button component path."
    );
}

#[test]
fn button_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget entry `{needle}`."
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
            "UiPerfProbe should expose regression budget marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep trace-based attribution marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance should keep render_count follow-up marker `{needle}`."
        );
    }
}

#[test]
fn button_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn button_view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "fn render_spinner() -> impl IntoView",
        "fn render_start_slot(",
        "fn render_end_slot(",
        "fn render_button_content(",
        "{render_button_content(state, render, start_content, end_content, children)}",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep macro complexity split marker `{needle}`."
        );
    }

    let spinner_occurrences = source.matches("\"ui-button__spinner\"").count();
    assert_eq!(
        spinner_occurrences, 1,
        "button spinner class literal should have a single source of truth via constants."
    );

    for forbidden in [
        "expect(\"checked start_content\")",
        "expect(\"checked end_content\")",
    ] {
        assert!(
            !source.contains(forbidden),
            "button view should avoid fragile inline content assertions `{forbidden}` after split."
        );
    }
}

#[test]
fn button_view_macro_check_script_covers_split_contract() {
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_view_macro_complexity_is_split_into_semantic_subrenders",
        "cargo test -p ui-components --test button_semantics button_view_functional_split_prefers_plain_functions_over_local_components",
        "cargo test -p ui-components --test button_semantics button_static_fragments_are_constantized_with_stable_a11y_semantics",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro check script should enforce split contract marker `{needle}`."
        );
    }
}

#[test]
fn button_view_functional_split_prefers_plain_functions_over_local_components() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "fn render_spinner() -> impl IntoView",
        "fn render_start_slot(",
        "-> AnyView {",
        "fn render_end_slot(",
        "fn render_button_content(",
        "pub fn Button(",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep function-first split marker `{needle}`."
        );
    }

    let component_attr_count = source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "button view should keep only one `#[component]` (Button root), and keep local fragments as plain Rust functions."
    );

    for forbidden in ["#[component]\nfn render_", "#[component]\r\nfn render_"] {
        assert!(
            !source.contains(forbidden),
            "button view should not promote local render fragments to components via `{forbidden}`."
        );
    }

    for semantic_marker in [
        "data-slot=SLOT_BUTTON_SPINNER",
        "data-slot=SLOT_BUTTON_START",
        "data-slot=SLOT_BUTTON_END",
        "data-slot=SLOT_BUTTON_LABEL",
    ] {
        assert!(
            source.contains(semantic_marker),
            "button functional split should keep semantic marker `{semantic_marker}` stable."
        );
    }
}

#[test]
fn button_static_fragments_are_constantized_with_stable_a11y_semantics() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "const SLOT_BUTTON: &str = \"button\";",
        "const SLOT_BUTTON_SPINNER: &str = \"button-spinner\";",
        "const SLOT_BUTTON_START: &str = \"button-start\";",
        "const SLOT_BUTTON_START_CONTENT: &str = \"button-start-content\";",
        "const SLOT_BUTTON_LABEL: &str = \"button-label\";",
        "const SLOT_BUTTON_END: &str = \"button-end\";",
        "const CLASS_BUTTON_SPINNER: &str = \"ui-button__spinner\";",
        "const CLASS_BUTTON_START: &str = \"ui-button__start\";",
        "const CLASS_BUTTON_START_CONTENT: &str = \"ui-button__start-content\";",
        "const CLASS_BUTTON_LABEL: &str = \"ui-button__label\";",
        "const CLASS_BUTTON_END: &str = \"ui-button__end\";",
        "class=CLASS_BUTTON_SPINNER data-slot=SLOT_BUTTON_SPINNER aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep static fragment constantization marker `{needle}`."
        );
    }

    for literal in [
        "\"button-spinner\"",
        "\"button-start\"",
        "\"button-start-content\"",
        "\"button-end\"",
        "\"button-label\"",
        "\"ui-button__spinner\"",
        "\"ui-button__start\"",
        "\"ui-button__start-content\"",
        "\"ui-button__end\"",
        "\"ui-button__label\"",
    ] {
        let count = source.matches(literal).count();
        assert_eq!(
            count, 1,
            "static fragment literal `{literal}` should be centralized to one constant source."
        );
    }
}

#[test]
fn button_inner_html_is_disallowed_in_button_runtime_paths() {
    for rel_path in [
        "src/button/view.rs",
        "src/button/logic.rs",
        "src/button/motion.rs",
        "src/button/styles.rs",
        "src/button/mod.rs",
        "src/button/spec.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Button runtime path must not inject raw HTML; found `{forbidden}` in `{rel_path}`."
            );
        }
    }
}

#[test]
fn docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let markdown_page_source = load_source("../../apps/docs-app/src/pages/docs/markdown_page.rs");

    for needle in [
        "const ACCORDION_README_MD: &str =",
        "include_str!(\"../../../../../crates/ui-components/src/accordion/README.md\")",
        "fn component_readme_markdown(slug: &str) -> Option<&'static str> {",
        "\"accordion\" => Some(ACCORDION_README_MD),",
        "_ => None,",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "<section class=\"docs-card docs-prose\" data-slot=\"component-readme\" inner_html=html></section>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep trusted inner_html whitelist marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=format!(",
        "inner_html=slug",
        "inner_html=description",
    ] {
        assert!(
            !shell_source.contains(forbidden),
            "docs component shell must not pipe dynamic text directly to inner_html via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn MarkdownPage(markdown: &'static str) -> impl IntoView",
        "let crate::markdown::MarkdownDoc {",
        "html: rendered_html,",
        "} = crate::markdown::render_markdown(markdown);",
        "let html = StoredValue::new(rendered_html);",
        "<div node_ref=container_ref inner_html=move || html.get_value()></div>",
    ] {
        assert!(
            markdown_page_source.contains(needle),
            "docs markdown page should keep trusted static markdown-to-html flow marker `{needle}`."
        );
    }

    for forbidden in ["inner_html=markdown", "inner_html=move || markdown"] {
        assert!(
            !markdown_page_source.contains(forbidden),
            "docs markdown page must not directly inject markdown source via `{forbidden}`."
        );
    }
}

#[test]
fn button_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_inner_html_is_disallowed_in_button_runtime_paths",
        "cargo test -p ui-components --test button_semantics docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources",
    ] {
        assert!(
            script_source.contains(needle),
            "inner-html check script should enforce security contract marker `{needle}`."
        );
    }
}

#[test]
fn button_wasm_debug_contract_is_feature_gated_and_dev_only() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/button/view.rs");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "Button wasm debug should be opt-in and tied to component-button feature."
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
        !all_components_block.contains("button-wasm-debug"),
        "button-wasm-debug must not be pulled into all-components production path."
    );

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "target_arch = \"wasm32\"",
        "Button Debug (wasm dev)",
        "data-slot=\"button-debug-entry\"",
        "data-slot=\"button-debug-event\"",
        "data-slot=\"button-debug-replay\"",
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "request_replay.run(event.source)",
        "target: \"ui_components::button::state_change\"",
        "debug_store.record(source, before, after);",
    ] {
        assert!(
            view_source.contains(needle),
            "Button wasm debug contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] debug"),
        "Button public API should not leak debug props."
    );
}

#[test]
fn button_wasm_debug_check_script_covers_feature_and_replay_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only",
    ] {
        assert!(
            script_source.contains(needle),
            "wasm-debug check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_uses_serde_schema_and_structured_migration_errors() {
    let cargo_source = load_source("Cargo.toml");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "serde = { version = \"1.0\", features = [\"derive\"], optional = true }",
        "serde_json = { version = \"1.0\", optional = true }",
    ] {
        assert!(
            cargo_source.contains(needle),
            "button engineering contract should keep serde feature gate marker `{needle}`."
        );
    }

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]",
        "pub enum ButtonSchemaErrorKind {",
        "pub struct ButtonSchemaError {",
        "pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>",
        "pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>",
        "button_schema_unsupported_version",
        "Unsupported button schema_version=",
        "schema_version: Option<u16>",
        "if schema_version != BUTTON_SCHEMA_VERSION {",
    ] {
        assert!(
            spec_source.contains(needle),
            "button spec should keep structured serde/migration marker `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_uses_consistent_tracing_targets() {
    let view_source = load_source("src/button/view.rs");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "target: \"ui_components::button::state_change\"",
        "const BUTTON_SPEC_TRACE_TARGET: &str = \"ui_components::button::spec\";",
        "trace_button_spec_event(",
        "\"button.schema.serialize\"",
        "\"button.schema.deserialize\"",
        "status",
        "error_code",
    ] {
        assert!(
            view_source.contains(needle) || spec_source.contains(needle),
            "button tracing contract should include `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_avoids_runtime_leaks_in_public_api() {
    let sources = [
        load_source("src/button/mod.rs"),
        load_source("src/button/logic.rs"),
        load_source("src/button/view.rs"),
        load_source("src/button/spec.rs"),
        load_source("src/button/motion.rs"),
    ];

    for source in &sources {
        for forbidden in ["tokio", "async_std", "async-std", "tokio::", "async_std::"] {
            assert!(
                !source.contains(forbidden),
                "button engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    let mod_source = load_source("src/button/mod.rs");
    assert!(
        !mod_source.contains("web_sys"),
        "button public module boundary should not leak web_sys types."
    );
}

#[test]
fn button_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
        "#[cfg(feature = \"component-button\")]",
        "pub mod button;",
        "#[cfg(feature = \"component-overlay\")]",
        "pub mod overlay;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(crate::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn ui_components_entrypoints_check_script_covers_contract() {
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
            "entrypoints check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_component_directory_has_standard_file_layout() {
    for required in [
        "src/button/mod.rs",
        "src/button/logic.rs",
        "src/button/styles.rs",
        "src/button/view.rs",
        "src/button/motion.rs",
        "src/button/spec.rs",
    ] {
        assert!(
            path_exists(required),
            "button component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/button/render.rs"),
        "button component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
}

#[test]
fn button_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/button/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod spec;",
        "pub mod styles;",
        "pub use view::Button;",
        "pub use motion::ButtonMotion;",
        "pub use logic::ButtonVariant;",
    ] {
        assert!(
            mod_source.contains(needle),
            "button/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "button/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn button_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");
    let motion_source = load_source("src/button/motion.rs");
    let spec_source = load_source("src/button/spec.rs");

    for forbidden in [
        "view!",
        "on:pointer",
        "on:keydown",
        "aria-",
        "data-slot",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "button/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "button/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "button/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Button(",
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "render_button_content(",
    ] {
        assert!(
            view_source.contains(required),
            "button/view.rs should keep rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "button/view.rs should not bypass logic layer; found `{forbidden}`."
        );
    }

    for required in [
        "pub struct ButtonMotion",
        "pub fn attach_motion(",
        "sanitize_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "button/motion.rs should keep motion-contract marker `{required}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "button/motion.rs should not carry view semantics; found `{forbidden}`."
        );
    }

    for required in [
        "pub struct ButtonSchema",
        "pub struct ButtonSpec",
        "pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>",
        "pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>",
    ] {
        assert!(
            spec_source.contains(required),
            "button/spec.rs should keep schema-contract marker `{required}`."
        );
    }

    let mut spec_files = Vec::new();
    collect_spec_files(Path::new("src"), Path::new("src"), &mut spec_files);
    assert_eq!(
        spec_files,
        vec!["button/spec.rs".to_string()],
        "spec.rs should stay scarce; only button/spec.rs is allowed in ui-components/src."
    );
}

#[test]
fn button_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test button_semantics button_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test button_semantics button_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "cargo test -p ui-components --test button_semantics button_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-button.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test button_semantics button_e2e_key_flow_covers_keyboard_and_code_sync_path",
    ] {
        assert!(
            script_source.contains(needle),
            "button e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_contract_hygiene_check_script_covers_no_temp_patch_rule() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    assert!(
        script_source.contains(
            "cargo test -p ui-components --test button_semantics button_contract_consistency_has_no_temporary_patch_markers"
        ),
        "contract hygiene check script should enforce no temporary-patch rule for button."
    );
}
