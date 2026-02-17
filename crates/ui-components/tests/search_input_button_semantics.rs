use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_input_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/search_input/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SearchInputButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn search_input_button_prioritizes_button_capability_reuse_over_reimplementation() {
    let view_source = load_source("src/button/search_input/view.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let state_primitives_source = load_source("../ui-state-primitives/src/button.rs");

    for needle in [
        "use super::super::{ButtonType, logic as button_logic};",
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "is_disabled,",
        "let button_type = logic::resolve_button_type(button_type);",
        "logic::resolve_effective_aria_label(normalized.aria_label, &view_state.placeholder);",
        "let class = logic::compose_class_name(normalized.class_name, state);",
        "let button_type = normalized.button_type.as_attr();",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton should reuse shared button normalization contract via `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};",
    ] {
        assert!(
            button_logic_source.contains(needle),
            "Button shared logic should expose normalization pipeline `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn resolve_aria_label(",
    ] {
        assert!(
            state_primitives_source.contains(needle),
            "Button normalization capability should be rooted in ui-state-primitives via `{needle}`."
        );
    }
}

#[test]
fn search_input_button_state_primitives_are_consumed_via_button_logic() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let state_primitives_source = load_source("../ui-state-primitives/src/button.rs");

    for needle in [
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "is_disabled,",
        "let state = logic::resolve_state(SearchInputButtonStateInput {",
        "is_disabled: normalized.is_disabled,",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should consume shared button primitive pipeline via `{needle}`."
        );
    }

    assert!(
        !logic_source.contains("input.is_disabled || input.disabled"),
        "SearchInputButton local logic should not reimplement disabled merge once shared button normalization is used."
    );

    for needle in [
        "pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
    ] {
        assert!(
            button_logic_source.contains(needle),
            "Button logic should expose primitive-backed normalization `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn resolve_aria_label(",
    ] {
        assert!(
            state_primitives_source.contains(needle),
            "ui-state-primitives should own reusable button normalization primitive `{needle}`."
        );
    }
}

#[test]
fn search_input_button_state_primitive_source_stays_in_status_primitives_and_no_business_store_binding()
 {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let state_primitives_source = load_source("../ui-state-primitives/src/button.rs");

    for needle in [
        "button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "button_logic::normalize_optional_text(value)",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "SearchInputButton should source reusable state primitives via shared button pipeline `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};",
        "resolve_state_core",
    ] {
        assert!(
            button_logic_source.contains(needle),
            "Button logic should bridge from ui-components to ui-state-primitives via `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state_core(input: ButtonStateCoreInput) -> ButtonStateCore",
    ] {
        assert!(
            state_primitives_source.contains(needle),
            "ui-state-primitives should remain the source of reusable button state primitive `{needle}`."
        );
    }

    for forbidden in [
        "use_context::<",
        "provide_context(",
        "leptos_store",
        "AppState",
        "GlobalState",
        "GlobalStore",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SearchInputButton should not bind component state directly to app/business store token `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_has_no_async_interaction_protocol_so_async_contract_is_not_applicable() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "error_message",
        "is_error",
        "loading_placement",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SearchInputButton should not define component-local async protocol token `{forbidden}` when no async axis exists."
        );
    }

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "let aria = use_button(ButtonOptions {",
        "is_disabled: state.is_disabled,",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton should keep simple press/disabled interaction contract via `{needle}`."
        );
    }
}

#[test]
fn search_input_button_uses_logic_state_model() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for needle in [
        "pub struct SearchInputButtonStateInput",
        "pub struct SearchInputButtonState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: SearchInputButtonStateInput)",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_button_type(",
        "pub fn resolve_effective_aria_label(",
        "pub state_attr: &'static str",
        "pub shortcut_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let placeholder = logic::normalize_optional_text(placeholder);",
        "let compact_placeholder = logic::normalize_optional_text(compact_placeholder);",
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "let state = logic::resolve_state(SearchInputButtonStateInput {",
        "let class = logic::compose_class_name(normalized.class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn search_input_button_api_naming_uses_is_prefix_only() {
    let view_source = load_source("src/button/search_input/view.rs");
    let docs_full_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let docs_start = docs_full_source
        .find("pub(super) fn search_input_button() -> AnyView")
        .expect("search_input_button docs section should exist.");
    let docs_tail = &docs_full_source[docs_start..];
    let docs_end = docs_tail
        .find("pub(super) fn button_copy() -> AnyView")
        .expect("search_input_button docs section should end before button_copy.");
    let docs_source = &docs_tail[..docs_end];

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "is_disabled=disabled",
        "is_disabled=true",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "SearchInputButton naming contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] disabled: bool"),
        "SearchInputButton should not expose legacy boolean prop alias `disabled`."
    );
    assert!(
        !docs_source.contains(" disabled=true"),
        "SearchInputButton docs should not use legacy boolean prop alias `disabled=true`."
    );
}

#[test]
fn search_input_button_has_no_controlled_state_axis_so_triplet_contract_is_not_applicable() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    let signature_start = view_source
        .find("pub fn SearchInputButton(")
        .expect("SearchInputButton signature should exist.");
    let signature_tail = &view_source[signature_start..];
    let signature_end = signature_tail
        .find(") -> impl IntoView {")
        .expect("SearchInputButton signature should close before function body.");
    let signature = &signature_tail[..signature_end];

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] on_press: Option<OnPress>",
    ] {
        assert!(
            signature.contains(needle),
            "SearchInputButton should keep trigger-only contract via `{needle}`."
        );
    }

    for forbidden in [
        "Signal<",
        "default_",
        "on_",
        "_change",
        "selected_",
        "open",
        "value",
        "checked",
    ] {
        let is_allowed_on_press = forbidden == "on_";
        if is_allowed_on_press {
            assert!(
                !signature.contains("on_open_change")
                    && !signature.contains("on_value_change")
                    && !signature.contains("on_selected")
                    && !signature.contains("on_checked_change"),
                "SearchInputButton should not expose controlled-state change handlers."
            );
            continue;
        }
        assert!(
            !signature.contains(forbidden),
            "SearchInputButton should not expose controlled-state axis token `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("use_controllable_state")
            && !logic_source.contains("use_controllable_state"),
        "SearchInputButton should not wire controllable-state primitive when no controlled axis exists."
    );
}

#[test]
fn search_input_button_dx_paradox_keeps_default_usage_simple_and_advanced_optional() {
    let view_source = load_source("src/button/search_input/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    let signature_start = view_source
        .find("pub fn SearchInputButton(")
        .expect("SearchInputButton signature should exist.");
    let signature_tail = &view_source[signature_start..];
    let signature_end = signature_tail
        .find(") -> impl IntoView {")
        .expect("SearchInputButton signature should close before function body.");
    let signature = &signature_tail[..signature_end];

    for needle in [
        "#[prop(optional, into)] placeholder: Option<String>",
        "#[prop(optional, into)] compact_placeholder: Option<String>",
        "#[prop(optional, into)] meta_key_label: Option<String>",
        "#[prop(optional, into)] key_label: Option<String>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] motion: SearchInputButtonMotion",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] button_type: Option<ButtonType>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] on_press: Option<OnPress>",
    ] {
        assert!(
            signature.contains(needle),
            "SearchInputButton should keep advanced controls optional via `{needle}`."
        );
    }

    for forbidden in ["state:", "use_controllable_state"] {
        assert!(
            !signature.contains(forbidden),
            "SearchInputButton baseline API should not require internal state wiring `{forbidden}`."
        );
    }

    let docs_start = docs_source
        .find("pub(super) fn search_input_button() -> AnyView")
        .expect("search_input_button docs section should exist.");
    let docs_tail = &docs_source[docs_start..];
    let docs_end = docs_tail
        .find("pub(super) fn button_copy() -> AnyView")
        .expect("search_input_button docs section should end before button_copy.");
    let docs_section = &docs_tail[..docs_end];

    for needle in [
        "let mut snippet = vec![\"<SearchInputButton\".to_string()];",
        "snippet.push(\"/>\".to_string());",
        "if placeholder != \"Search\" {",
        "if compact_placeholder != placeholder {",
        "if !meta_key_label.is_empty() {",
        "if !key_label.is_empty() {",
        "if disabled {",
        "if custom_aria_label {",
    ] {
        assert!(
            docs_section.contains(needle),
            "SearchInputButton docs should present minimal default path with optional advanced switches via `{needle}`."
        );
    }
}

#[test]
fn search_input_button_is_not_a_composite_component_and_avoids_parallel_array_api_shapes() {
    let view_source = load_source("src/button/search_input/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    let signature_start = view_source
        .find("pub fn SearchInputButton(")
        .expect("SearchInputButton signature should exist.");
    let signature_tail = &view_source[signature_start..];
    let signature_end = signature_tail
        .find(") -> impl IntoView {")
        .expect("SearchInputButton signature should close before function body.");
    let signature = &signature_tail[..signature_end];

    for forbidden in [
        "children:",
        "items:",
        "ItemSpec",
        "labels",
        "titles",
        "panels",
    ] {
        assert!(
            !signature.contains(forbidden),
            "SearchInputButton should not expose composite/parallel-array API token `{forbidden}`."
        );
    }

    let docs_start = docs_source
        .find("pub(super) fn search_input_button() -> AnyView")
        .expect("search_input_button docs section should exist.");
    let docs_tail = &docs_source[docs_start..];
    let docs_end = docs_tail
        .find("pub(super) fn button_copy() -> AnyView")
        .expect("search_input_button docs section should end before button_copy.");
    let docs_section = &docs_tail[..docs_end];

    for forbidden in [
        "labels=vec![",
        "titles=vec![",
        "panels=vec![",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !docs_section.contains(forbidden),
            "SearchInputButton docs should not suggest composite parallel-array contract `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button/search_input/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SearchInputButton should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn search_input_button_mounts_headless_contract_in_view_not_logic_layer() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

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
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton should mount headless semantic contract in view via `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "ButtonOptions {",
        "on:pointerdown",
        "on:keydown",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "SearchInputButton logic should not host headless/event wiring `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/search_input/view.rs");

    for attr in [
        "data-slot=\"search-input-button\"",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-shortcut=state.shortcut_attr",
        "data-placeholder=state.placeholder_source_attr",
        "data-compact-placeholder=state.compact_placeholder_source_attr",
        "data-aria-label-source=state.aria_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-hovered",
        "data-pressed",
        "data-motion-source=if motion == SearchInputButtonMotion::default()",
        "data-custom-motion=(motion != SearchInputButtonMotion::default()).then_some(\"true\")",
        "data-slot=\"search-input-button-icon\"",
        "data-slot=\"search-input-button-shortcut\"",
        "data-slot=\"search-input-button-key\"",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should set `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn search_input_button_forwards_headless_button_semantics() {
    let source = load_source("src/button/search_input/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn search_input_button_uses_fallback_aria_label_from_placeholder() {
    let source = load_source("src/button/search_input/view.rs");

    for needle in [
        "logic::resolve_effective_aria_label(normalized.aria_label, &view_state.placeholder);",
        "has_custom_aria_label: aria_label.has_custom_aria_label,",
        "let aria_label = StoredValue::new(aria_label.aria_label);",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton should normalize aria labeling using `{needle}`."
        );
    }
}

#[test]
fn search_input_button_default_priority_is_centralized_in_logic_module() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for needle in [
        "pub fn resolve_view_state(",
        ".unwrap_or(fallback_placeholder)",
        ".unwrap_or(placeholder.as_str())",
        "fallback_placeholder: &str,",
        "pub fn resolve_button_type(button_type: Option<ButtonType>) -> ButtonType",
        "pub fn resolve_effective_aria_label(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton default-priority rules should be centralized in logic via `{needle}`."
        );
    }

    for needle in [
        "let button_type = logic::resolve_button_type(button_type);",
        "logic::resolve_effective_aria_label(normalized.aria_label, &view_state.placeholder);",
        "common_strings.search_input_button_placeholder.as_ref()",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should consume centralized default resolution output `{needle}`."
        );
    }

    for forbidden in [
        ".unwrap_or_else(|| view_state.placeholder.clone())",
        "match button_type {",
        ".unwrap_or(\"Search\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SearchInputButton view should not keep local default fallback branch `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_a11y_and_i18n_fallbacks_are_context_driven() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");
    let common_strings_source = load_source("../../crates/ui-headless/src/i18n/common.rs");

    for needle in [
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "common_strings.search_input_button_placeholder.as_ref()",
        "locale_attrs(lang, dir)",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "let aria = use_button(ButtonOptions {",
        "role=aria.attrs.role",
        "aria-label=move || aria_label.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton should route A11y/i18n fallback via context-driven contract `{needle}`."
        );
    }

    for needle in [
        "fallback_placeholder: &str",
        ".unwrap_or(fallback_placeholder)",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton logic should expose fallback-aware i18n normalization `{needle}`."
        );
    }

    let needle = "pub search_input_button_placeholder: Arc<str>,";
    assert!(
        common_strings_source.contains(needle),
        "ui-headless CommonStrings should expose SearchInputButton i18n key `{needle}`."
    );

    assert!(
        !view_source.contains("\"Search\""),
        "SearchInputButton view should not hardcode visible fallback copy."
    );
}

#[test]
fn search_input_button_discrete_inputs_are_enum_constrained() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for needle in [
        "use super::super::{ButtonType, logic as button_logic};",
        "#[prop(optional)] button_type: Option<ButtonType>",
        "pub fn resolve_button_type(button_type: Option<ButtonType>) -> ButtonType",
        "button_type.unwrap_or_default()",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "SearchInputButton discrete contract should be enum-constrained via `{needle}`."
        );
    }

    for forbidden in [
        "button_type: Option<&'static str>",
        "button_type: Option<String>",
        "Some(\"submit\")",
        "Some(\"reset\")",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SearchInputButton should not model discrete state with string aliases `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_state_normalization_is_centralized_in_logic_module() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for needle in [
        "pub struct SearchInputButtonStateInput",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String> {",
        "button_logic::normalize_optional_text(value)",
        "pub fn resolve_view_state(",
        "pub fn resolve_state(input: SearchInputButtonStateInput) -> SearchInputButtonState",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton logic should centralize state normalization via `{needle}`."
        );
    }

    for needle in [
        "let placeholder = logic::normalize_optional_text(placeholder);",
        "let compact_placeholder = logic::normalize_optional_text(compact_placeholder);",
        "let meta_key_label = logic::normalize_optional_text(meta_key_label);",
        "let key_label = logic::normalize_optional_text(key_label);",
        "let view_state = logic::resolve_view_state(",
        "let state = logic::resolve_state(SearchInputButtonStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should consume centralized normalization output via `{needle}`."
        );
    }

    for forbidden in [
        "data-state=if ",
        "if is_disabled { \"disabled\" } else {",
        "if view_state.show_shortcut",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SearchInputButton view should not rebuild local state-machine branch `{forbidden}`."
        );
    }
}

#[test]
fn search_input_button_styles_include_state_marker_contracts() {
    let styles = load_source("src/button/search_input/styles.rs");

    for selector in [
        ".ui-search-input-button--enabled",
        ".ui-search-input-button[data-state=\"disabled\"]",
        ".ui-search-input-button--custom-placeholder",
        ".ui-search-input-button[data-compact-placeholder=\"custom\"] .ui-search-input-button__placeholder--compact",
        ".ui-search-input-button--with-shortcut .ui-search-input-button__shortcut",
        ".ui-search-input-button[data-shortcut=\"visible\"] .ui-search-input-button__shortcut",
        ".ui-search-input-button--custom-class",
        ".ui-search-input-button[data-motion-source=\"custom\"]",
        ".ui-search-input-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            styles.contains(selector),
            "SearchInputButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn search_input_button_theme_contract_reuses_ui_theme_tokens_and_button_surface() {
    let styles_source = load_source("src/button/search_input/styles.rs");
    let view_source = load_source("src/button/search_input/view.rs");
    let theme_tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "--ui-search-input-button-scale: var(--ui-button-scale, 1);",
        "var(--ui-fg-muted)",
        "var(--ui-space-sm)",
        "var(--ui-space-md)",
        "var(--ui-radius-md)",
        "var(--ui-border)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-shadow-sm)",
        "var(--ui-focus-ring)",
    ] {
        assert!(
            styles_source.contains(needle),
            "SearchInputButton styles should consume ui-theme/button token surface via `{needle}`."
        );
    }

    for needle in [
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "let class = logic::compose_class_name(normalized.class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should preserve shared button surface assembly via `{needle}`."
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
            "ui-theme should remain the source of token/theme-axis contract `{needle}`."
        );
    }
}

#[test]
fn search_input_button_stays_in_ui_components_assembly_layer() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");
    let motion_source = load_source("src/button/search_input/motion.rs");
    let styles_source = load_source("src/button/search_input/styles.rs");

    for needle in [
        "let placeholder = logic::normalize_optional_text(placeholder);",
        "let view_state = logic::resolve_view_state(",
        "let normalized = button_logic::normalize_input(button_logic::ButtonInputNormalizationInput {",
        "let state = logic::resolve_state(SearchInputButtonStateInput {",
        "let class = logic::compose_class_name(normalized.class_name, state);",
        "let aria = use_button(ButtonOptions {",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
        "motion::attach_motion(",
        "data-slot=\"search-input-button\"",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should remain assembly-only via `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "on:pointerdown",
        "on:keydown",
        "web_sys::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "SearchInputButton logic should not host view/headless/platform wiring `{forbidden}`."
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "let motion = as_button_motion(sanitize_motion(motion));",
        "crate::button::motion::attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "SearchInputButton motion should stay as semantic->shared button motion mapping via `{needle}`."
        );
    }

    for needle in [
        "var(--ui-fg-muted)",
        "var(--ui-space-sm)",
        "var(--ui-border)",
        "var(--ui-bg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "SearchInputButton styles should stay token-first via `{needle}`."
        );
    }
}

#[test]
fn search_input_button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button/search_input/styles.rs");
    let motion = load_source("src/button/search_input/motion.rs");

    for needle in [
        "--ui-search-input-button-scale",
        "transform: scale(var(--ui-search-input-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "SearchInputButton styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("crate::button::motion::attach_motion("),
        "SearchInputButton motion should delegate spring runtime to Button motion so interaction feedback uses the shared motion engine."
    );

    assert!(
        motion.contains("let motion = as_button_motion(sanitize_motion(motion));"),
        "SearchInputButton motion should sanitize and convert to ButtonMotion before delegation."
    );
}

#[test]
fn search_input_button_inherits_button_reduced_motion_ssr_and_wasm_branches() {
    let button_styles_source = load_source("src/button/styles.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let search_input_motion_source = load_source("src/button/search_input/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-button__spinner {",
        "animation: none;",
    ] {
        assert!(
            button_styles_source.contains(needle),
            "Button reduced-motion style contract should expose `{needle}` for SearchInputButton reuse."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "Button motion should keep wasm/non-wasm split contract via `{needle}`."
        );
    }

    for needle in [
        "let motion = as_button_motion(sanitize_motion(motion));",
        "crate::button::motion::attach_motion(node_ref, is_hovered, is_pressed, is_disabled, motion);",
    ] {
        assert!(
            search_input_motion_source.contains(needle),
            "SearchInputButton should inherit reduced-motion/SSR/wasm branches by delegating to Button motion via `{needle}`."
        );
    }

    assert!(
        ui_motion_source.contains("pub fn prefers_reduced_motion() -> bool {"),
        "ui-motion should keep reduced-motion capability for delegated Button/SearchInputButton motion paths."
    );
}

#[test]
fn search_input_button_perf_budget_inherits_button_runtime_without_extra_motion_engine() {
    let view_source = load_source("src/button/search_input/view.rs");
    let motion_source = load_source("src/button/search_input/motion.rs");
    let button_motion_source = load_source("src/button/motion.rs");

    for needle in [
        "motion::attach_motion(",
        "let motion = as_button_motion(sanitize_motion(motion));",
        "crate::button::motion::attach_motion(node_ref, is_hovered, is_pressed, is_disabled, motion);",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "SearchInputButton should keep thin perf path by reusing button motion capability `{needle}`."
        );
    }

    for forbidden in ["Effect::new(", "SpringAnimator::new"] {
        assert!(
            !motion_source.contains(forbidden),
            "SearchInputButton motion should avoid spawning a separate motion engine token `{forbidden}`."
        );
    }

    for needle in ["Effect::new(", "SpringAnimator::new"] {
        assert!(
            button_motion_source.contains(needle),
            "Button motion runtime should remain the single owner of heavy motion work token `{needle}`."
        );
    }
}

#[test]
fn search_input_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button/search_input/motion.rs");

    for needle in [
        "pub struct SearchInputButtonMotion",
        "fn default_motion_matches_search_input_button_spring_contract()",
        "fn supports_custom_search_input_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn search_input_button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/search_input/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SearchInputButtonMotion) -> SearchInputButtonMotion",
        "fn sanitize_spring(",
        "fn as_button_motion(",
        "hover_scale:",
        "tap_scale:",
        "let motion = as_button_motion(sanitize_motion(motion));",
        "crate::button::motion::attach_motion(",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn search_input_button_does_not_use_inner_html_in_render_path() {
    let view_source = load_source("src/button/search_input/view.rs");
    let logic_source = load_source("src/button/search_input/logic.rs");

    for forbidden in ["inner_html", "set_inner_html"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SearchInputButton should avoid `{forbidden}` and keep render path on typed/escaped content."
        );
    }
}

#[test]
fn search_input_button_view_is_split_into_plain_functions_with_static_icon_fragments() {
    let view_source = load_source("src/button/search_input/view.rs");

    for needle in [
        "const SEARCH_ICON_VIEW_BOX: &str = \"0 0 20 20\";",
        "const SEARCH_ICON_PATH: &str = \"M13.5 13.5l3 3\";",
        "fn render_search_icon() -> impl IntoView {",
        "fn render_placeholders(",
        "fn render_shortcut(",
        "{render_search_icon()}",
        "{render_placeholders(placeholder, compact_placeholder)}",
        "{render_shortcut(show_shortcut, meta_key_label, key_label)}",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view complexity should stay split and static-fragment friendly via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("fn render_search_icon() -> View"),
        "SearchInputButton helper rendering should remain lightweight plain function returns (`impl IntoView`) and avoid unnecessary concrete view typing."
    );
}

#[test]
fn search_input_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"SearchInputButton\"",
        "slug=\"search-input-button\"",
        "description=\"baseline-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs.\"",
        "<Playground",
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "id_base=\"docs-search-input-preset\".to_string()",
        "id_base=\"docs-search-input-meta-key\".to_string()",
        "id_base=\"docs-search-input-key\".to_string()",
        "aria_label=\"Search input preset\".to_string()",
        "aria_label=\"Search input meta key\".to_string()",
        "aria_label=\"Search input shortcut key\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=custom_aria_label set_checked=set_custom_aria_label>",
        "<Switch checked=persist_workbench_state set_checked=set_persist_workbench_state>",
        "<Playground title=\"Placeholder + disabled matrix\" code_signal=states_code>",
        "<Playground title=\"Custom Class + Aria Label\" code_signal=custom_code>",
        "<SearchInputButton",
    ] {
        assert!(
            source.contains(needle),
            "actions docs should include `{needle}` for search-input-button primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"button-search-input\" => &[\"search-input-button\"]"),
        "components mod mapping should keep `button-search-input` mapped to `search-input-button` slug.",
    );
}

#[test]
fn search_input_button_docs_workbench_supports_optional_state_persistence() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "struct SearchInputButtonWorkbenchState {",
        "fn load_search_input_button_workbench_state() -> Option<SearchInputButtonWorkbenchState>",
        "fn save_search_input_button_workbench_state(state: SearchInputButtonWorkbenchState)",
        "fn clear_search_input_button_workbench_state()",
        "let persisted_workbench_state = load_search_input_button_workbench_state();",
        "let (persist_workbench_state, set_persist_workbench_state) =",
        "save_search_input_button_workbench_state(state);",
        "clear_search_input_button_workbench_state();",
        "\"Persist workbench state\"",
    ] {
        assert!(
            source.contains(needle),
            "search_input_button docs workbench persistence contract should include `{needle}`."
        );
    }
}

#[test]
fn search_input_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "let preset_options = vec![",
        "let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0)",
        "let meta_key_options = vec![",
        "let key_label_options = vec![",
        "placeholder=placeholder",
        "compact_placeholder=compact_placeholder",
        "meta_key_label=meta_key_label",
        "key_label=key_label",
        "if custom_aria_label {",
        "aria_label=\"Open command menu\".to_string()",
        "on_press=on_press",
        "\"presses: \"",
        "title=\"Placeholder + disabled matrix\"",
        "placeholder=\"Find components\".to_string()",
        "compact_placeholder=\"Find\".to_string()",
        "placeholder=\"Disabled search\".to_string()",
        "is_disabled=true",
        "placeholder=\"Forced disabled\".to_string()",
        "is_disabled=true",
        "title=\"Custom Class + Aria Label\"",
        "placeholder=\"Browse components\".to_string()",
        "compact_placeholder=\"Browse\".to_string()",
        "aria_label=\"Open component search\".to_string()",
        "class_name=\"docs-search-input-button-custom\".to_string()",
        "placeholder=\"Search by keyword\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs playgrounds should contain `{needle}` for search-input-button contracts.",
        );
    }
}
