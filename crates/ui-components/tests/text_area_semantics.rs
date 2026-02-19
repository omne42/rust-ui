use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/text_area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TextArea internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn text_area_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/text_input/text_area/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::TextArea;"),
        "text_area module should export `TextArea`.",
    );
    assert!(
        module_source.contains("pub use motion::TextAreaMotion;"),
        "text_area module should export `TextAreaMotion`.",
    );
    assert!(
        crate_source.contains("pub use text_input::text_area::TextArea;"),
        "crate root should re-export `TextArea`.",
    );
}

#[test]
fn text_area_logic_exposes_state_helpers() {
    let logic_source = load_source("src/text_input/text_area/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/text_area.rs");

    for needle in [
        "pub use ui_state_primitives::text_area::{",
        "TextAreaState",
        "TextAreaStateInput",
        "normalize_optional_text",
        "resolve_label_with_fallback",
        "resolve_state",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
        "pub fn resolve_props(input: ResolvedTextAreaPropsInput) -> ResolvedTextAreaProps",
        "pub fn compose_class_name(class_name: Option<String>, state: TextAreaState)",
    ] {
        assert!(
            logic_source.contains(needle),
            "TextArea logic should include `{needle}` to consume shared state primitives.",
        );
    }

    for needle in [
        "pub struct TextAreaStateInput",
        "pub struct TextAreaState",
        "pub fn resolve_label(value: String)",
        "pub fn resolve_label_with_fallback(value: String, fallback_label: &str)",
        "pub fn resolve_state(input: TextAreaStateInput) -> TextAreaState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "TextArea state primitive layer should include `{needle}`.",
        );
    }
}

#[test]
fn text_area_view_uses_logic_state_and_a11y_contracts() {
    let source = load_source("src/text_input/text_area/view.rs");

    for needle in [
        "use_focus_ring",
        "use_text_field",
        "use_controllable_state",
        "use_ui_i18n",
        "A11yDirection",
        "locale_attrs",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "logic::resolve_props(logic::ResolvedTextAreaPropsInput {",
        "logic::resolve_state(TextAreaStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "motion::sanitize_motion(motion)",
        "motion::motion_style_vars(motion)",
        "motion::attach_motion(root_ref, is_active, motion)",
        "let locale = locale_attrs(lang, dir);",
        "data-slot=\"text-area\"",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-value-control-mode=value_axis.control_mode_attr",
        "data-default-value-source=value_axis.default_value_source_attr",
        "data-value-change-source=value_axis.value_change_source_attr",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-state=move || state.get().state_attr",
        "data-value=move || state.get().value_attr",
        "data-requirement=move || state.get().requirement_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-rows-source=move || state.get().rows_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            source.contains(needle),
            "TextArea view should include `{needle}` for stable marker + a11y contracts.",
        );
    }
}

#[test]
fn text_area_styles_include_state_source_and_reduced_motion_markers() {
    let source = load_source("src/text_input/text_area/styles.rs");

    for selector in [
        ".ui-text-area[data-state=\"disabled\"]",
        ".ui-text-area[data-state=\"invalid\"]",
        ".ui-text-area[data-state=\"readonly\"]",
        ".ui-text-area[data-value=\"filled\"]",
        ".ui-text-area[data-requirement=\"required\"]",
        ".ui-text-area[data-value-control-mode=\"controlled\"]",
        ".ui-text-area[data-default-value-source=\"custom\"]",
        ".ui-text-area[data-value-change-source=\"on_value_change\"]",
        ".ui-text-area[data-label-source=\"custom\"]",
        ".ui-text-area[data-description-source=\"custom\"]",
        ".ui-text-area[data-error-source=\"custom\"]",
        ".ui-text-area[data-placeholder-source=\"custom\"]",
        ".ui-text-area[data-rows-source=\"custom\"]",
        ".ui-text-area[data-class-source=\"custom\"]",
        ".ui-text-area--custom-class",
        "var(--ui-text-area-motion-duration)",
        "var(--ui-text-area-motion-easing)",
        "prefers-reduced-motion: reduce",
        "--ui-text-area-motion-duration: 1ms;",
    ] {
        assert!(
            source.contains(selector),
            "TextArea styles should include `{selector}` as stable selectors.",
        );
    }
}

#[test]
fn text_area_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::text_input::text_area::styles::CSS);"),
        "ui-components css aggregator should include text_area styles.",
    );
}

#[test]
fn text_area_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn text_area() -> AnyView",
        "title=\"TextArea\"",
        "slug=\"text-area\"",
        "State + Source Markers",
        "data-rows-source",
    ] {
        assert!(
            source.contains(needle),
            "forms docs page should contain `{needle}` for text-area.",
        );
    }
}

#[test]
fn text_area_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-text-area-markers\".to_string()",
        "label=\"Release notes\".to_string()",
        "default_value=\"Shipping notes\".to_string()",
        "is_required=Signal::derive(move || true)",
        "is_invalid=Signal::derive(move || invalid.get())",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Release notes are required\".to_string()",
        "placeholder=\"Write release notes…\".to_string()",
        "motion=TextAreaMotion::disabled()",
        "rows=6",
        "class_name=\"docs-text-area-state\".to_string()",
        "Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-value-control-mode`, `data-default-value-source`, `data-value-change-source`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-rows-source`.",
    ] {
        assert!(
            source.contains(needle),
            "TextArea docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn text_area_motion_module_uses_token_backed_contract() {
    let source = load_source("src/text_input/text_area/motion.rs");

    for needle in [
        "pub struct TextAreaMotion",
        "default_textarea_motion_tokens",
        "pub fn sanitize_motion(motion: TextAreaMotion) -> TextAreaMotion",
        "pub fn motion_style_vars(motion: TextAreaMotion) -> String",
        "--ui-text-area-motion-duration",
        "--ui-text-area-motion-easing",
        "pub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "TextArea motion module should include `{needle}`.",
        );
    }
}

#[test]
fn text_area_docs_page_covers_primary_playgrounds() {
    text_area_docs_page_contains_state_source_playground();
}

#[test]
fn text_area_docs_playgrounds_lock_state_matrix_contract_values() {
    text_area_docs_state_source_playground_locks_contract_values();
}
