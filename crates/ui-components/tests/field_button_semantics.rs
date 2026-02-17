use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn field_button_has_no_compat_module_and_is_reexported_from_button_field() {
    let source = load_source("src/lib.rs");

    for needle in ["pub use button::field::FieldButton;", "FieldButton"] {
        assert!(
            source.contains(needle),
            "crate re-exports should include `{needle}` from button/field."
        );
    }

    assert!(
        !source.contains("pub mod field_button;"),
        "compat module `src/field_button.rs` should not be reintroduced."
    );
}

#[test]
fn field_button_implementation_lives_under_button_field_module() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "pub mod styles;",
        "mod logic;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub fn FieldButton(",
    ] {
        assert!(
            module_source.contains(needle),
            "button/field module should define `{needle}` as canonical FieldButton contract."
        );
    }

    assert!(
        logic_source.contains("pub const DEFAULT_ARIA_LABEL: &str = \"FieldButton\";"),
        "button/field/logic.rs should own the canonical FieldButton aria label default."
    );
}

#[test]
fn field_button_api_naming_uses_is_prefix_only() {
    let source = load_source("src/button/field/mod.rs");

    for needle in [
        "#[prop(optional)] is_quiet: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_active: bool",
        "#[prop(optional)] on_press: Option<OnPress>",
    ] {
        assert!(
            source.contains(needle),
            "FieldButton API naming should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] quiet: bool",
        "#[prop(optional)] invalid: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] on_click:",
        "#[prop(optional)] on_action:",
    ] {
        assert!(
            !source.contains(forbidden),
            "FieldButton should not expose legacy boolean alias `{forbidden}`."
        );
    }
}

#[test]
fn field_button_new_params_must_follow_naming_and_contract_pipeline() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "#[prop(optional)] is_quiet: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_active: bool",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] button_type: ButtonType",
        "#[prop(optional)] on_press: Option<OnPress>",
        "pub struct FieldButtonResolveInput {",
        "pub is_quiet: bool,",
        "pub is_invalid: bool,",
        "pub is_disabled: bool,",
        "pub is_active: bool,",
        "pub aria_label: Option<String>,",
        "pub class_name: Option<String>,",
        "pub button_type: ButtonType,",
        "pub on_press: Option<OnPress>,",
        "tone: if input.is_quiet {",
        "validation: if input.is_invalid {",
        "is_disabled: input.is_disabled,",
        "is_active: input.is_active,",
    ] {
        assert!(
            module_source.contains(needle) || logic_source.contains(needle),
            "FieldButton parameters should stay in naming+normalization contract via `{needle}`."
        );
    }

    for needle in [
        "is_quiet=true",
        "is_invalid=true",
        "is_active=true",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs should expose the same contract parameter `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] quiet: bool",
        "#[prop(optional)] invalid: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] active: bool",
        "#[prop(optional)] on_click:",
        "#[prop(optional)] on_action:",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should reject uncontracted parameter alias `{forbidden}`."
        );
    }
}

#[test]
fn field_button_composes_base_button_instead_of_reimplementing_hooks() {
    let source = load_source("src/button/field/mod.rs");

    for needle in [
        "use super::{Button, ButtonSize, ButtonType};",
        "let resolved = logic::resolve_props(logic::FieldButtonResolveInput {",
        "<Button",
        "variant=resolved.variant",
        "color=resolved.color",
        "size=ButtonSize::S",
        "button_type=resolved.button_type",
        "on_press=resolved.on_press",
    ] {
        assert!(
            source.contains(needle),
            "FieldButton should compose shared Button behavior via `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "motion::attach_motion(",
    ] {
        assert!(
            !source.contains(forbidden),
            "FieldButton should not reimplement button interaction hook `{forbidden}`."
        );
    }
}

#[test]
fn field_button_avoids_temporary_patches_and_keeps_button_contract_consistency() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "<Button",
        "variant=resolved.variant",
        "color=resolved.color",
        "size=ButtonSize::S",
        "is_disabled=resolved.is_disabled",
        "class_name=resolved.class_name",
        "button_type=resolved.button_type",
        "aria_label=resolved.aria_label",
        "on_press=resolved.on_press",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton should keep Button contract consistency via `{needle}`."
        );
    }

    for forbidden in [
        "legacy",
        "compat",
        "temporary",
        "workaround",
        "hotfix",
        "TODO",
        "FIXME",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not keep temporary patch marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not keep temporary patch marker `{forbidden}`."
        );
    }
}

#[test]
fn field_button_maps_field_state_to_button_variant_and_css_markers() {
    let source = load_source("src/button/field/logic.rs");

    for needle in [
        "pub enum FieldButtonTone {",
        "pub enum FieldButtonValidation {",
        "pub fn resolve_state(input: FieldButtonStateInput) -> FieldButtonState {",
        "FieldButtonTone::Default => ButtonVariant::Default",
        "FieldButtonTone::Quiet => ButtonVariant::Ghost",
        "ButtonVariant::Ghost",
        "ButtonVariant::Default",
        "FieldButtonValidation::Default => ButtonColor::Default",
        "FieldButtonValidation::Invalid => ButtonColor::Danger",
        "ButtonColor::Danger",
        "ButtonColor::Default",
        "\"ui-field-button\"",
        "\"ui-field-button--quiet\"",
        "\"ui-field-button--invalid\"",
        "\"ui-field-button--active\"",
        "\"ui-field-button--disabled\"",
        "\"ui-field-button--custom-handler\"",
        "\"ui-field-button--custom-aria-label\"",
        "\"ui-field-button--custom-class\"",
    ] {
        assert!(
            source.contains(needle),
            "FieldButton should keep field-specific state mapping via `{needle}`."
        );
    }
}

#[test]
fn field_button_defaults_are_centralized_in_logic() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    assert!(
        module_source
            .contains("let resolved = logic::resolve_props(logic::FieldButtonResolveInput {"),
        "FieldButton view should consume a single resolved logic payload."
    );

    for forbidden in [
        "normalize_optional_text(",
        "unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string())",
        "Callback::new(|_| {})",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view should not own fallback/default logic `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_input(input: FieldButtonResolveInput) -> FieldButtonNormalizedInput {",
        "let normalized_aria_label = normalize_optional_text(input.aria_label);",
        "let normalized_class_name = normalize_optional_text(input.class_name);",
        "tone: if input.is_quiet {",
        "validation: if input.is_invalid {",
        "pub fn resolve_state(input: FieldButtonStateInput) -> FieldButtonState {",
        "pub fn compose_class_name(state: FieldButtonState, custom_class_name: Option<&str>) -> String {",
        "aria_label: normalized",
        "on_press: normalized.on_press.unwrap_or_else(|| Callback::new(|_| {})),",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton logic should centralize defaults/normalization via `{needle}`."
        );
    }
}

#[test]
fn field_button_state_normalization_is_not_scattered_in_view() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for forbidden in [
        "if is_quiet {",
        "if is_invalid {",
        "if is_active {",
        "if is_disabled {",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view should not scatter state branching `{forbidden}`."
        );
    }

    for needle in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "let state = resolve_state(FieldButtonStateInput {",
        "let core = resolve_state_core(ButtonStateCoreInput {",
        "class_name: compose_class_name(state, normalized.class_name.as_deref()),",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton state derivation should be centralized in logic via `{needle}`."
        );
    }
}

#[test]
fn field_button_view_layer_only_consumes_resolved_state_payload() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    assert!(
        module_source
            .contains("let resolved = logic::resolve_props(logic::FieldButtonResolveInput {"),
        "FieldButton view should only consume the resolved payload from logic."
    );

    for forbidden in [
        "if is_quiet {",
        "if is_invalid {",
        "if is_disabled {",
        "if is_active {",
        "match is_quiet",
        "match is_invalid",
        "match is_disabled",
        "match is_active",
        "unwrap_or(",
        "unwrap_or_else(",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view should not own state-decision token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_input(input: FieldButtonResolveInput) -> FieldButtonNormalizedInput {",
        "pub fn resolve_state(input: FieldButtonStateInput) -> FieldButtonState {",
        "pub fn compose_class_name(state: FieldButtonState, custom_class_name: Option<&str>) -> String {",
        "pub fn resolve_props(input: FieldButtonResolveInput) -> FieldButtonResolved {",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton should keep state decision in logic via `{needle}`."
        );
    }
}

#[test]
fn field_button_styles_include_quiet_invalid_and_active_markers() {
    let source = load_source("src/button/field/styles.rs");

    for selector in [
        ".ui-field-button--quiet",
        ".ui-field-button[data-quiet=\"true\"]",
        ".ui-field-button--invalid",
        ".ui-field-button[data-invalid=\"true\"]",
        ".ui-field-button.is-hovered",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button.is-active",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button--disabled",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button--focus-visible",
        ".ui-field-button.ui-button--focus-visible",
        ".ui-field-button--custom-class",
        ".ui-field-button[data-custom-class=\"true\"]",
        ".ui-field-button .ui-button__label",
    ] {
        assert!(
            source.contains(selector),
            "FieldButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn field_button() -> AnyView",
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "description=\"baseline-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior.\"",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
        "<FieldButton",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra docs page should include `{needle}` for field_button primary playground coverage.",
        );
    }
}

#[test]
fn field_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Default + Quiet\"",
        "aria_label=\"Open options\".to_string()",
        "\"Options\"",
        "is_quiet=true",
        "aria_label=\"Open calendar\".to_string()",
        "\"📅\"",
        "title=\"Invalid + Active + Disabled\"",
        "is_invalid=true",
        "is_active=true",
        "aria_label=\"Invalid trigger\".to_string()",
        "class_name=\"docs-field-button-custom\".to_string()",
        "\"Needs fix\"",
        "is_disabled=true",
        "aria_label=\"Disabled trigger\".to_string()",
        "\"Disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "field_button docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn field_button_docs_examples_are_synced_with_current_api_and_state_matrix() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let module_source = load_source("src/button/field/mod.rs");

    for needle in [
        "pub(super) fn field_button() -> AnyView",
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
        "is_quiet=true",
        "is_invalid=true",
        "is_active=true",
        "is_disabled=true",
        "aria_label=\"Open options\".to_string()",
        "aria_label=\"Open calendar\".to_string()",
        "aria_label=\"Invalid trigger\".to_string()",
        "aria_label=\"Disabled trigger\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs matrix should stay synced via `{needle}`."
        );
    }

    for forbidden in [
        "<FieldButton quiet=true",
        "<FieldButton invalid=true",
        "<FieldButton disabled=true",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "FieldButton docs should not drift to legacy alias usage `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_quiet: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_active: bool",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton public API should match docs matrix token `{needle}`."
        );
    }
}

#[test]
fn field_button_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn field_button() -> AnyView",
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "description=\"baseline-style field trigger button",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<FieldButton aria_label=\"Open options\".to_string()>",
        "\"Options\"",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs should keep beginner-oriented token `{needle}`."
        );
    }

    let default_index = docs_source
        .find("title=\"Default + Quiet\"")
        .expect("Default playground title should exist");
    let advanced_index = docs_source
        .find("title=\"Invalid + Active + Disabled\"")
        .expect("Advanced playground title should exist");
    assert!(
        default_index < advanced_index,
        "FieldButton docs should present default usage before advanced states."
    );

    for forbidden in [
        "use_state_primitives",
        "use_headless",
        "state=",
        "controller=",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "FieldButton docs should not require low-level wiring token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_docs_app_exposes_interactive_playground_with_live_code_signals() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "let default_code = Signal::derive(move || {",
        "let state_code = Signal::derive(move || {",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
        "<FieldButton aria_label=\"Open options\".to_string()>",
        "is_invalid=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs playground should keep live preview token `{needle}`."
        );
    }

    for needle in [
        "docs-app field-button keeps stable semantic selectors and settled contract states",
        "docs-app field-button key interaction flow is repeatable with semantic breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "FieldButton playground should stay covered by E2E acceptance token `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "FieldButton playground E2E should avoid fixed-sleep token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_docs_source_is_copy_paste_ready_with_imports_and_copy_control() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground source-first copy pipeline should keep token `{needle}`."
        );
    }

    for needle in [
        "docs-app field-button playground source is copy-paste ready",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui_components::*;\")",
        "toContainText(\"<FieldButton\")",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "FieldButton source-first docs contract should keep acceptance token `{needle}`."
        );
    }
}

#[test]
fn field_button_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "### FieldButton 同步记录（2026-02-17）",
        "FieldButton` 保持 `Button` 语义薄封装定位",
        "`is_quiet/is_invalid/is_active/is_disabled`",
        "component_doc!(\"FieldButton\", \"field-button\", \"Actions\", actions_extra::field_button)",
        "`#/components/field-button` 可索引访问",
        "`Default + Quiet` 与 `Invalid + Active + Disabled`",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI alignment strategy doc should keep FieldButton sync token `{needle}`."
        );
    }

    for needle in [
        "\"FieldButton\"",
        "\"field-button\"",
        "actions_extra::field_button",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs catalog entry should expose FieldButton token `{needle}`."
        );
    }

    assert!(
        docs_source.contains("slug=\"field-button\""),
        "FieldButton docs page should keep reachable docs slug."
    );
}

#[test]
fn field_button_keeps_status_primitives_free_of_dom_and_style_logic() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/button.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "pub struct ButtonStateCoreInput",
        "pub struct ButtonStateCore",
        "pub fn resolve_state_core(input: ButtonStateCoreInput) -> ButtonStateCore",
        "pub fn resolve_aria_label(",
    ] {
        assert!(
            primitives_source.contains(needle),
            "status-primitives button contract should keep pure state token `{needle}`."
        );
    }

    for forbidden in [
        "leptos::",
        "web_sys::",
        "wasm_bindgen::",
        "NodeRef",
        "HtmlElement",
        "data-slot",
        "aria-",
        ".ui-",
        "color-mix(",
    ] {
        assert!(
            !primitives_source.contains(forbidden),
            "status-primitives must not contain DOM/style token `{forbidden}`."
        );
    }

    for needle in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "let core = resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            field_logic_source.contains(needle),
            "FieldButton should consume pure primitive via `{needle}`."
        );
    }
}

#[test]
fn field_button_keeps_ui_headless_free_of_visual_and_motion_orchestration() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_button_source = load_source("../../crates/ui-headless/src/button.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "pub mod a11y;",
        "pub mod button;",
        "pub use a11y::{",
        "pub use button::{",
        "pub fn use_button(options: ButtonOptions) -> ButtonAria {",
        "pub fn popup_trigger_attrs(",
    ] {
        assert!(
            headless_lib_source.contains(needle)
                || headless_button_source.contains(needle)
                || headless_a11y_source.contains(needle),
            "ui-headless should stay semantic-contract focused via `{needle}`."
        );
    }

    for forbidden in [
        "styles::CSS",
        ".ui-",
        "--ui-",
        "color-mix(",
        "@media",
        "keyframes",
        "animation:",
        "ui_motion::",
        "SpringAnimator",
    ] {
        assert!(
            !headless_lib_source.contains(forbidden),
            "ui-headless lib should not contain visual/motion orchestration token `{forbidden}`."
        );
        assert!(
            !headless_button_source.contains(forbidden),
            "ui-headless button should not contain visual/motion orchestration token `{forbidden}`."
        );
        assert!(
            !headless_a11y_source.contains(forbidden),
            "ui-headless a11y should not contain visual/motion orchestration token `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless::OnPress;",
        "<Button",
        "on_press=resolved.on_press",
    ] {
        assert!(
            field_module_source.contains(needle) || field_logic_source.contains(needle),
            "FieldButton should consume headless semantic contract via `{needle}`."
        );
    }
}

#[test]
fn field_button_default_visual_baseline_is_token_driven_and_button_aligned() {
    let styles_source = load_source("src/button/field/styles.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        ".ui-field-button.is-hovered",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button.is-active",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button--focus-visible",
        ".ui-field-button[data-focus-visible=\"true\"]",
        "color-mix(in oklab, var(--ui-danger) 55%, var(--ui-border))",
        "color-mix(in oklab, var(--ui-bg-muted) 80%, var(--ui-accent) 20%)",
        "var(--ui-button-font-size)",
        "var(--ui-focus-ring)",
    ] {
        assert!(
            styles_source.contains(needle),
            "FieldButton visual baseline should keep token-driven feedback `{needle}`."
        );
    }

    for forbidden in ["#007bff", "btn-primary", "font-family: Arial", "bootstrap"] {
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should avoid legacy visual fallback token `{forbidden}`."
        );
    }

    for needle in [
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "description=\"baseline-style field trigger button",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs should keep default visual baseline proof `{needle}`."
        );
    }
}

#[test]
fn field_button_does_not_define_async_loading_protocol_surface() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for forbidden in ["is_loading", "aria-busy", "aria_busy", "retry", "error"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should not expose async protocol token `{forbidden}` in view API.",
        );
    }

    for forbidden in [
        "aria-busy",
        "aria_busy",
        "retry",
        "error",
        "use_async_action",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton should not implement async protocol token `{forbidden}` in logic.",
        );
    }

    assert!(
        logic_source.contains("is_loading: false,"),
        "FieldButton should explicitly lock loading branch to `is_loading: false`."
    );
}

#[test]
fn field_button_exposes_no_controlled_uncontrolled_state_axis() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for forbidden in [
        "value:",
        "default_value:",
        "on_value_change:",
        "default_is_",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should not expose controlled/uncontrolled axis token `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not implement controlled/uncontrolled axis token `{forbidden}`.",
        );
    }

    assert!(
        module_source.contains("#[prop(optional)] on_press: Option<OnPress>"),
        "FieldButton should only expose the event callback `on_press` for trigger semantics."
    );
}

#[test]
fn field_button_dx_keeps_simple_default_path_without_state_wiring() {
    let module_source = load_source("src/button/field/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for forbidden in [
        "state:",
        "controller:",
        "selection:",
        "ui_state_primitives::",
        "use_button(",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton public API should not require manual state wiring token `{forbidden}`."
        );
    }

    for needle in [
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<FieldButton aria_label=\"Open options\".to_string()>",
        "  \"Options\"",
        "</FieldButton>",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton docs should present minimal default path snippet `{needle}`."
        );
    }
}

#[test]
fn field_button_api_is_explicit_composition_without_parallel_arrays() {
    let module_source = load_source("src/button/field/mod.rs");

    for needle in ["children: Children,", "{children()}", "<Button"] {
        assert!(
            module_source.contains(needle),
            "FieldButton composition should stay explicit via `{needle}`."
        );
    }

    for forbidden in ["labels:", "titles:", "panels:", "items: Vec", "item_specs:"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should not introduce implicit parallel-array API `{forbidden}`."
        );
    }
}

#[test]
fn field_button_a11y_and_i18n_entrypoints_delegate_to_button_contract() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "use super::{Button, ButtonSize, ButtonType};",
        "#[prop(optional, into)] aria_label: Option<String>",
        "aria_label=resolved.aria_label",
        "on_press=resolved.on_press",
        "<Button",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton should delegate A11y semantics via Button contract `{needle}`."
        );
    }

    for needle in [
        "let normalized_aria_label = normalize_optional_text(input.aria_label);",
        "pub const DEFAULT_ARIA_LABEL: &str = \"FieldButton\";",
        ".unwrap_or_else(|| DEFAULT_ARIA_LABEL.to_string())",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton should keep i18n/l10n aria label entrypoint and fallback via `{needle}`."
        );
    }

    assert!(
        docs_source.contains("aria_label=\"Open options\".to_string()"),
        "FieldButton docs should demonstrate caller-provided aria_label localization entrypoint."
    );
}

#[test]
fn field_button_state_markers_are_observable_and_source_distinguishable() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "is_disabled=resolved.is_disabled",
        "class_name=resolved.class_name",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton should forward observable state contract into Button via `{needle}`."
        );
    }

    for needle in [
        "has_custom_aria_label",
        "has_custom_class_name",
        "has_custom_press_handler",
        "\"ui-field-button--custom-handler\"",
        "\"ui-field-button--custom-aria-label\"",
        "\"ui-field-button--custom-class\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton logic should expose stable source markers via `{needle}`."
        );
    }

    for selector in [
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-focus-visible=\"true\"]",
        ".ui-field-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "FieldButton style contract should stay selectable via marker `{selector}`."
        );
    }
}

#[test]
fn field_button_styles_depend_on_explicit_markers_not_fragile_dom_structure() {
    let module_source = load_source("src/button/field/mod.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for selector in [
        ".ui-field-button[data-quiet=\"true\"]",
        ".ui-field-button[data-invalid=\"true\"]",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button[data-focus-visible=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "FieldButton styles should derive visual states from explicit marker `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", " > ", " + ", " ~ "] {
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should not rely on fragile DOM structure selector `{forbidden}`."
        );
    }

    assert!(
        !module_source.contains("style="),
        "FieldButton view should avoid inline style branching and delegate style contract to styles.rs."
    );
}

#[test]
fn field_button_semantics_suite_targets_contracts_not_visual_snapshots() {
    let test_source = load_source("tests/field_button_semantics.rs");

    for needle in [
        "data-disabled",
        "data-hovered",
        "data-active",
        "data-focus-visible",
        "custom-aria-label",
        "custom-handler",
        "aria_label",
    ] {
        assert!(
            test_source.contains(needle),
            "FieldButton semantic suite should keep contract assertion token `{needle}`."
        );
    }

    let forbidden_assert_snapshot = ["assert", "_", "snapshot", "!"].concat();
    let forbidden_insta = ["in", "sta", "::"].concat();
    let forbidden_match_snapshot = ["to", "_", "match", "_", "snapshot"].concat();

    for forbidden in [
        forbidden_assert_snapshot,
        forbidden_insta,
        forbidden_match_snapshot,
    ] {
        assert!(
            !test_source.contains(&forbidden),
            "FieldButton semantic suite should not rely on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_semantics_suite_covers_data_aria_role_and_source_contracts() {
    let test_source = load_source("tests/field_button_semantics.rs");
    let button_view_source = load_source("src/button/view.rs");

    for needle in [
        "field_button_a11y_and_i18n_entrypoints_delegate_to_button_contract",
        "field_button_state_markers_are_observable_and_source_distinguishable",
        "field_button_styles_depend_on_explicit_markers_not_fragile_dom_structure",
        "field_button_semantics_suite_targets_contracts_not_visual_snapshots",
        "data-disabled",
        "data-hovered",
        "data-active",
        "data-focus-visible",
        "custom-handler",
        "custom-aria-label",
        "aria_label",
    ] {
        assert!(
            test_source.contains(needle),
            "FieldButton semantics suite should cover contract token `{needle}`."
        );
    }

    for needle in ["role=aria.attrs.role", "aria-label=normalized_aria_label"] {
        assert!(
            button_view_source.contains(needle),
            "Button semantic mount should keep `{needle}` for delegated FieldButton role/aria coverage."
        );
    }

    let forbidden_assert_snapshot = ["assert", "_", "snapshot", "!"].concat();
    let forbidden_insta = ["in", "sta", "::"].concat();
    let forbidden_match_snapshot = ["to", "_", "match", "_", "snapshot"].concat();

    for forbidden in [
        forbidden_assert_snapshot,
        forbidden_insta,
        forbidden_match_snapshot,
    ] {
        assert!(
            !test_source.contains(&forbidden),
            "FieldButton semantics suite should stay contract-first and avoid `{forbidden}`."
        );
    }
}

#[test]
fn field_button_e2e_contract_uses_semantic_selectors_and_stable_waiting() {
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "page.goto(\"/#/components/field-button\")",
        "body:not(:has(#boot))",
        "[data-slot=\"field-button\"]",
        "section.playground",
        "[data-slot=\"button\"]",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(needle),
            "FieldButton E2E contract should use semantic selector/wait token `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "getByText(",
        "xpath=",
        ":nth-child",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "FieldButton E2E contract should avoid fragile selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_e2e_suite_contains_repeatable_key_interaction_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "key interaction flow is repeatable with semantic breakpoints",
        "await defaultButton.click();",
        "await expect(defaultButton).toBeFocused();",
        "await quietButton.click();",
        "await expect(quietButton).toBeFocused();",
        "await expect(root).toHaveAttribute(\"data-slot\", \"field-button\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "FieldButton E2E suite should keep repeatable key-flow token `{needle}`."
        );
    }

    let field_button_e2e_cases = e2e_source.matches("test(\"docs-app field-button").count();
    assert!(
        field_button_e2e_cases >= 2,
        "FieldButton E2E regression suite should keep at least two deterministic contract tests."
    );
}

#[test]
fn field_button_file_responsibilities_stay_minimal_and_button_reuse_focused() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "pub fn FieldButton(",
        "<Button",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton module boundary should keep minimal composition token `{needle}`."
        );
    }

    for forbidden in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub const CSS: &str",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton mod.rs should not absorb logic/style implementation token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_props(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton logic.rs should own derivation token `{needle}`."
        );
    }
    assert!(
        !logic_source.contains("view!"),
        "FieldButton logic.rs must not contain rendering markup."
    );

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "FieldButton styles.rs should own static CSS contract."
    );
    for forbidden in ["#[component]", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles.rs should not contain rendering token `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !manifest_dir.join("src/button/field/view.rs").exists(),
        "FieldButton should keep thin-wrapper layout and not introduce standalone view.rs."
    );
    assert!(
        !manifest_dir.join("src/button/field/motion.rs").exists(),
        "FieldButton should reuse Button motion contract and not introduce motion.rs."
    );
}

#[test]
fn field_button_does_not_introduce_spec_schema_layer() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        !manifest_dir.join("src/button/field/spec.rs").exists(),
        "FieldButton should not add spec.rs for a simple thin-wrapper component."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "spec::", "FieldButtonSpec"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not depend on spec layer token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not depend on spec layer token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_styles_use_token_first_values_and_reuse_button_vars() {
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "var(--ui-button-size-s-icon)",
        "var(--ui-button-size-s-height)",
        "var(--ui-button-font-size)",
        "var(--ui-button-focus-outline-width)",
        "var(--ui-button-focus-outline-offset)",
        "var(--ui-focus-ring)",
        "var(--ui-button-disabled-opacity, 0.5)",
    ] {
        assert!(
            styles_source.contains(needle),
            "FieldButton styles should be token-first and reuse Button/theme vars via `{needle}`."
        );
    }

    for forbidden in [
        "min-width: 2rem",
        "min-height: 2rem",
        "font-size: 0.875rem",
        "outline: 2px",
        "outline-offset: 2px",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should not keep hard-coded visual constant `{forbidden}`."
        );
    }
}

#[test]
fn field_button_theme_contract_comes_from_ui_theme_and_button_tokens() {
    let theme_lib_source = load_source("../../crates/ui-theme/src/lib.rs");
    let theme_tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_styles_source = load_source("src/button/field/styles.rs");

    for needle in ["pub mod css;", "pub mod theme;", "pub mod tokens;"] {
        assert!(
            theme_lib_source.contains(needle),
            "ui-theme entry should expose canonical module `{needle}`."
        );
    }

    for needle in [
        "pub struct ButtonLayoutTokens",
        "pub struct ButtonMotionTokens",
        "pub fn default_button_layout_tokens() -> ButtonLayoutTokens",
        "pub fn default_button_motion_tokens() -> ButtonMotionTokens",
        "  --ui-system:",
        "  --ui-color:",
        "  --ui-scale:",
    ] {
        assert!(
            theme_tokens_source.contains(needle)
                || theme_theme_source.contains(needle)
                || theme_css_source.contains(needle),
            "ui-theme should keep token/theme/css contract token `{needle}`."
        );
    }

    assert!(
        field_module_source.contains("<Button"),
        "FieldButton should consume themed button surface by composing `<Button>`."
    );

    for needle in [
        "var(--ui-button-size-s-icon)",
        "var(--ui-button-size-s-height)",
        "var(--ui-button-font-size)",
        "var(--ui-button-focus-outline-width)",
        "var(--ui-button-focus-outline-offset)",
        "var(--ui-focus-ring)",
        "var(--ui-danger)",
        "var(--ui-border)",
        "var(--ui-accent)",
    ] {
        assert!(
            field_styles_source.contains(needle),
            "FieldButton styles should consume theme variables via `{needle}`."
        );
    }
}

#[test]
fn field_button_type_system_and_markers_form_machine_readable_contract() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "pub enum FieldButtonTone {",
        "pub enum FieldButtonValidation {",
        "FieldButtonTone::Default => ButtonVariant::Default",
        "FieldButtonTone::Quiet => ButtonVariant::Ghost",
        "FieldButtonValidation::Default => ButtonColor::Default",
        "FieldButtonValidation::Invalid => ButtonColor::Danger",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton should keep type-constrained discrete states via `{needle}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "validation: Option<String>",
        "tone: String",
        "validation: String",
        "#[prop(optional, into)] tone: Option<String>",
        "#[prop(optional, into)] validation: Option<String>",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !module_source.contains(forbidden),
            "FieldButton should not expose stringly-typed state axis `{forbidden}`."
        );
    }

    for marker in [
        "data-quiet",
        "data-invalid",
        "data-hovered",
        "data-active",
        "data-pressed",
        "data-disabled",
        "data-focus-visible",
        "data-custom-class",
        "ui-field-button--custom-handler",
        "ui-field-button--custom-aria-label",
    ] {
        assert!(
            styles_source.contains(marker) || logic_source.contains(marker),
            "FieldButton should expose machine-readable state marker `{marker}`."
        );
    }
}

#[test]
fn field_button_source_keeps_platform_agnostic_contract() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen",
        "js_sys::",
        "window()",
        "document()",
        "HtmlElement",
        "cfg(target_arch = \"wasm32\")",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton mod.rs should stay platform-agnostic and avoid `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic.rs should stay platform-agnostic and avoid `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles.rs should stay platform-agnostic and avoid `{forbidden}`."
        );
    }
}

#[test]
fn field_button_respects_ui_headless_web_ssr_mutex_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutex guard `{needle}`."
        );
    }

    let needle = "use ui_headless::OnPress;";
    assert!(
        module_source.contains(needle),
        "FieldButton should consume headless contract via `{needle}` in mod.rs."
    );
    assert!(
        logic_source.contains(needle),
        "FieldButton should consume headless contract via `{needle}` in logic.rs."
    );
}

#[test]
fn field_button_reuses_button_motion_and_ui_motion_has_non_wasm_stub() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

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

    for forbidden in ["use ui_motion::", "ui_motion::", "attach_motion("] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton mod.rs should reuse Button motion and avoid direct token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic.rs should reuse Button motion and avoid direct token `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir.join("src/button/field/motion.rs").exists(),
        "FieldButton should not add motion.rs and should inherit Button motion behavior."
    );
}

#[test]
fn field_button_motion_runtime_is_delegated_to_button_motion_layer() {
    let button_motion_source = load_source("src/button/motion.rs");
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "use ui_theme::default_button_motion_tokens;",
        "pub struct ButtonMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "ui_motion::spring::SpringAnimator::new",
        "pub fn attach_motion(",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "Button motion layer should own runtime token `{needle}`."
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringConfig",
        "ui_motion::spring::SpringAnimator::new",
        "attach_motion(",
        "default_button_motion_tokens",
    ] {
        assert!(
            !field_module_source.contains(forbidden),
            "FieldButton module should delegate motion runtime and avoid `{forbidden}`."
        );
        assert!(
            !field_logic_source.contains(forbidden),
            "FieldButton logic should delegate motion runtime and avoid `{forbidden}`."
        );
    }
}

#[test]
fn field_button_inherits_button_reduced_motion_ssr_and_wasm_branches() {
    let button_styles_source = load_source("src/button/styles.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let field_module_source = load_source("src/button/field/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-button__spinner {",
        "animation: none;",
    ] {
        assert!(
            button_styles_source.contains(needle),
            "Button styles should expose reduced-motion fallback via `{needle}`."
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

    assert!(
        field_module_source.contains("<Button"),
        "FieldButton should inherit branch behavior by composing Button."
    );
    assert!(
        !manifest_dir.join("src/button/field/motion.rs").exists(),
        "FieldButton should not fork motion branch handling into its own motion.rs."
    );
}

#[test]
fn field_button_perf_budget_inherits_button_without_extra_reactive_work() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "let resolved = logic::resolve_props(logic::FieldButtonResolveInput {",
        "<Button",
        "size=ButtonSize::S",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton should keep thin delegation path via `{needle}`."
        );
    }

    for forbidden in [
        "signal(",
        "create_signal(",
        "Signal::derive(",
        "Effect::new(",
        "create_memo(",
        "StoredValue::new(",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should not add local reactive/render loops `{forbidden}` in view layer."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should stay pure derivation and avoid `{forbidden}`."
        );
    }
}

#[test]
fn field_button_view_macro_complexity_stays_small_and_delegated() {
    let module_source = load_source("src/button/field/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert_eq!(
        module_source.matches("view!").count(),
        1,
        "FieldButton should keep a single small `view!` block."
    );
    assert_eq!(
        module_source.matches("<Button").count(),
        1,
        "FieldButton should delegate rendering through one Button node."
    );

    for forbidden in [
        "<div", "<section", "<header", "<footer", "<ul", "<li", "<For",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view macro should avoid deep layout expansion token `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir.join("src/button/field/view.rs").exists(),
        "FieldButton should not split into a separate view.rs for this thin wrapper."
    );
}

#[test]
fn field_button_prefers_plain_functions_over_extra_component_noise() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    assert_eq!(
        module_source.matches("#[component]").count(),
        1,
        "FieldButton should expose one component boundary only."
    );
    assert!(
        module_source.contains("pub fn FieldButton("),
        "FieldButton module should keep only the primary component entry."
    );

    for forbidden in [
        "#[component] fn",
        "#[component]\nfn",
        "pub fn FieldButtonItem(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should avoid promoting local fragments into components `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_props(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton keeps non-view work in plain functions via `{needle}`."
        );
    }
}

#[test]
fn field_button_static_fragments_are_constantized_or_absent() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    assert!(
        logic_source.contains("pub const DEFAULT_ARIA_LABEL: &str = \"FieldButton\";"),
        "FieldButton should keep static fallback label in a constant."
    );
    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "FieldButton should keep static style fragment in CSS constant."
    );

    for forbidden in ["<svg", "<path", "inner_html", "dangerously_set_inner_html"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton should avoid inline complex static fragment token `{forbidden}` in view."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton should avoid inline complex static fragment token `{forbidden}` in logic."
        );
    }
}

#[test]
fn field_button_forbids_inner_html_injection_paths() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "raw_html",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view should not expose HTML injection token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not expose HTML injection token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should not expose HTML injection token `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "FieldButton docs examples should not encourage HTML injection token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_wasm_debug_contract_is_feature_gated_and_inherited_from_button() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "button-wasm-debug = [",
        "\"component-button\"",
        "\"dep:tracing\"",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components should keep wasm debug feature gating token `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("field-wasm-debug"),
        "FieldButton should not introduce a parallel debug feature; reuse Button debug feature."
    );

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "record_transition(",
        "render_debug_panel(",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button view should provide wasm debug trace/replay contract `{needle}`."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "record_transition(",
        "render_debug_panel(",
    ] {
        assert!(
            !field_module_source.contains(forbidden),
            "FieldButton module should not leak debug runtime token `{forbidden}`."
        );
        assert!(
            !field_logic_source.contains(forbidden),
            "FieldButton logic should not leak debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_dx_has_playground_canvas_and_state_preserving_usage_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let module_source = load_source("src/button/field/mod.rs");

    for needle in [
        "pub(super) fn field_button() -> AnyView",
        "title=\"FieldButton\"",
        "slug=\"field-button\"",
        "<Playground title=\"Default + Quiet\" code_signal=default_code>",
        "<Playground title=\"Invalid + Active + Disabled\" code_signal=state_code>",
        "let default_code = Signal::derive(move || {",
        "let state_code = Signal::derive(move || {",
    ] {
        assert!(
            docs_source.contains(needle),
            "FieldButton should provide docs playground/workbench signal `{needle}`."
        );
    }

    for forbidden in ["signal(", "create_signal("] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton view should avoid local state loops `{forbidden}` to keep debug context stable."
        );
    }
}

#[test]
fn field_button_engineering_contract_stays_runtime_agnostic_and_spec_free() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "tracing::",
        "tokio::",
        "async_std::",
        "runtime",
        "spec::",
        "FieldButtonSpec",
        "async fn",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not leak engineering/runtime binding token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not leak engineering/runtime binding token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should not leak engineering/runtime binding token `{forbidden}`."
        );
    }

    assert!(
        module_source.contains("#[prop(optional)] on_press: Option<OnPress>"),
        "FieldButton should expose runtime-agnostic callback surface via OnPress."
    );
}

#[test]
fn ui_components_entry_files_keep_expected_boundary_layout() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("src/active_highlight.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "pub mod button;",
        "#[cfg(feature = \"component-button\")]",
        "mod css;",
        "pub mod root;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib.rs should keep entry boundary token `{needle}`."
        );
    }

    assert!(
        css_source.contains("pub fn push_components_css(out: &mut String)"),
        "ui-components css.rs should keep CSS aggregation entrypoint."
    );
    assert!(
        root_source.contains("pub fn UiRoot("),
        "ui-components root.rs should keep UiRoot injection entrypoint."
    );
    assert!(
        active_highlight_source.contains("pub const CSS: &str"),
        "ui-components active_highlight.rs should keep shared highlight style capability."
    );

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui-components should not define forbidden root-level file `{forbidden}`."
        );
    }
}

#[test]
fn field_button_tree_shaking_is_anchored_to_component_button_feature_only() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let button_mod_source = load_source("src/button/mod.rs");

    for needle in [
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "#[cfg(feature = \"component-button\")]",
        "pub mod button;",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
        "out.push_str(crate::button::field::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Tree-shaking contract should keep token `{needle}`."
        );
    }

    assert!(
        button_mod_source.contains("pub mod field;"),
        "FieldButton should stay under button module so it follows component-button feature gate."
    );

    for forbidden in [
        "component-field_button",
        "component-action_button =",
        "pub mod field_button;",
        "cfg(feature = \"component-field_button\")",
    ] {
        assert!(
            !cargo_source.contains(forbidden)
                && !lib_source.contains(forbidden)
                && !css_source.contains(forbidden),
            "Tree-shaking should not keep legacy alias token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_component_directory_layout_stays_minimal_and_button_delegated() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for required in [
        "src/button/field/mod.rs",
        "src/button/field/logic.rs",
        "src/button/field/styles.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "FieldButton component directory should keep required file `{required}`."
        );
    }

    for forbidden in [
        "src/button/field/render.rs",
        "src/button/field/spec.rs",
        "src/button/field/view.rs",
        "src/button/field/motion.rs",
    ] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "FieldButton should not duplicate Button implementation file `{forbidden}`."
        );
    }

    for delegated in ["src/button/view.rs", "src/button/motion.rs"] {
        assert!(
            manifest_dir.join(delegated).exists(),
            "Button capability file `{delegated}` should exist for FieldButton delegation."
        );
    }

    for needle in [
        "pub mod styles;",
        "mod logic;",
        "pub fn FieldButton(",
        "let resolved = logic::resolve_props(logic::FieldButtonResolveInput {",
        "<Button",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton module should keep minimal boundary token `{needle}`."
        );
    }

    for forbidden in ["pub mod view;", "pub mod motion;", "pub mod spec;"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not export duplicated layer `{forbidden}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen::", "unsafe"] {
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should stay platform-agnostic without token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should stay platform-agnostic without token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_consumes_state_primitives_instead_of_reimplementing_state_machine() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "let core = resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton should consume state primitive contract via `{needle}`."
        );
    }

    for forbidden in [
        "create_signal(",
        "RwSignal<",
        "Signal::derive(",
        "set_timeout(",
        "request_animation_frame",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not host local reactive state-machine token `{forbidden}`."
        );
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not host local reactive state-machine token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_delegates_headless_semantics_to_button_view_layer() {
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");
    let button_view_source = load_source("src/button/view.rs");

    for needle in [
        "use ui_headless::OnPress;",
        "<Button",
        "on_press=resolved.on_press",
    ] {
        assert!(
            field_module_source.contains(needle) || field_logic_source.contains(needle),
            "FieldButton should consume headless contract via delegation token `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "ButtonOptions {",
        "FocusRingOptions {",
        "HoverOptions {",
    ] {
        assert!(
            !field_module_source.contains(forbidden),
            "FieldButton module should not implement headless semantics token `{forbidden}`."
        );
        assert!(
            !field_logic_source.contains(forbidden),
            "FieldButton logic should not implement headless semantics token `{forbidden}`."
        );
    }

    for needle in [
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
        "ui_headless::ButtonAria",
        "ui_headless::PressHandlers",
        "ui_headless::FocusRingHandlers",
        "ui_headless::HoverHandlers",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button view should remain the canonical headless semantic attachment point `{needle}`."
        );
    }
}

#[test]
fn field_button_stays_as_ui_components_assembly_layer_only() {
    let field_module_source = load_source("src/button/field/mod.rs");
    let field_logic_source = load_source("src/button/field/logic.rs");
    let field_styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "let resolved = logic::resolve_props(logic::FieldButtonResolveInput {",
        "<Button",
        "class_name=resolved.class_name",
        "on_press=resolved.on_press",
    ] {
        assert!(
            field_module_source.contains(needle),
            "FieldButton should keep ui-components assembly token `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_hover(",
        "use_focus_ring(",
        "motion::attach_motion(",
        "ui_motion::",
        "web_sys::",
        "leptos::web_sys::",
    ] {
        assert!(
            !field_module_source.contains(forbidden),
            "FieldButton module should not reimplement lower-layer token `{forbidden}`."
        );
        assert!(
            !field_logic_source.contains(forbidden),
            "FieldButton logic should not reimplement lower-layer token `{forbidden}`."
        );
        assert!(
            !field_styles_source.contains(forbidden),
            "FieldButton styles should not reimplement lower-layer token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_agent_contract_uses_machine_readable_markers_without_dom_guessing() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "class_name=resolved.class_name",
        "\"ui-field-button--custom-handler\"",
        "\"ui-field-button--custom-aria-label\"",
        "\"ui-field-button--custom-class\"",
        ".ui-field-button[data-quiet=\"true\"]",
        ".ui-field-button[data-invalid=\"true\"]",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-pressed=\"true\"]",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button[data-focus-visible=\"true\"]",
        ".ui-field-button[data-custom-class=\"true\"]",
    ] {
        assert!(
            module_source.contains(needle)
                || logic_source.contains(needle)
                || styles_source.contains(needle),
            "FieldButton Agent-contract path should keep machine-readable marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "eval(",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should keep whitelist rendering boundary and avoid `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should keep whitelist rendering boundary and avoid `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "FieldButton styles should keep whitelist rendering boundary and avoid `{forbidden}`."
        );
    }
}

#[test]
fn field_button_streaming_scope_is_not_applicable_and_keeps_plain_render_contract() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in ["pub fn FieldButton(", "<Button", "{children()}"] {
        assert!(
            module_source.contains(needle),
            "FieldButton should remain a plain render component via `{needle}`."
        );
    }

    for forbidden in [
        "stream",
        "streaming",
        "snapshot",
        "token_delta",
        "draft",
        "verified",
        "fallback=snapshot",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not define LLM streaming protocol token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not define LLM streaming protocol token `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "FieldButton docs should not claim LLM streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_snapshot_mode_consumes_complete_input_and_renders_in_one_pass() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");

    for needle in [
        "let resolved = logic::resolve_props(logic::FieldButtonResolveInput {",
        "variant=resolved.variant",
        "color=resolved.color",
        "is_disabled=resolved.is_disabled",
        "class_name=resolved.class_name",
        "button_type=resolved.button_type",
        "aria_label=resolved.aria_label",
        "on_press=resolved.on_press",
        "{children()}",
    ] {
        assert!(
            module_source.contains(needle),
            "FieldButton snapshot path should keep complete one-pass render token `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_input(",
        "pub fn resolve_state(",
        "pub fn resolve_props(",
        "class_name: compose_class_name(state, normalized.class_name.as_deref()),",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldButton logic should deterministically resolve full input via `{needle}`."
        );
    }

    for forbidden in ["stream", "streaming", "token_delta", "incremental"] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not require incremental protocol token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not require incremental protocol token `{forbidden}`."
        );
    }
}

#[test]
fn field_button_streaming_policy_is_optional_and_delegated_to_upper_layer() {
    let module_source = load_source("src/button/field/mod.rs");
    let logic_source = load_source("src/button/field/logic.rs");
    let styles_source = load_source("src/button/field/styles.rs");

    for needle in [
        "aria_label=resolved.aria_label",
        "class_name=resolved.class_name",
        "on_press=resolved.on_press",
        ".ui-field-button[data-disabled=\"true\"]",
        ".ui-field-button[data-hovered=\"true\"]",
        ".ui-field-button[data-active=\"true\"]",
        ".ui-field-button[data-focus-visible=\"true\"]",
    ] {
        assert!(
            module_source.contains(needle)
                || logic_source.contains(needle)
                || styles_source.contains(needle),
            "FieldButton should keep continuous semantic markers via `{needle}`."
        );
    }

    for forbidden in [
        "streaming",
        "token_delta",
        "fallback=snapshot",
        "draft",
        "verified",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "FieldButton module should not own upper-layer streaming policy token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "FieldButton logic should not own upper-layer streaming policy token `{forbidden}`."
        );
    }
}
