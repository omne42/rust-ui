use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn underlay_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/underlay/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Underlay internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn underlay_module_exposes_slot_and_part_state_contracts() {
    let source = load_source("src/underlay/mod.rs");

    for needle in [
        "pub use view::Underlay;",
        "pub enum UnderlaySlot",
        "pub struct UnderlayPartStateInput",
        "pub struct UnderlayPartState",
        "DEFAULT_TRANSPARENT",
        "DEFAULT_DISABLED",
    ] {
        assert!(
            source.contains(needle),
            "Underlay module should include `{needle}` as stable state contracts."
        );
    }
}

#[test]
fn underlay_logic_models_state_helpers_and_contracts() {
    let source = load_source("src/underlay/logic.rs");

    for needle in [
        "pub const DEFAULT_OPEN: bool = false;",
        "pub const DEFAULT_TRANSPARENT: bool = false;",
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn state_attr(is_open: bool, is_disabled: bool)",
        "pub fn tone_attr(is_transparent: bool)",
        "pub fn close_mode_attr(is_interactive: bool)",
        "pub fn resolve_state(input: UnderlayPartStateInput) -> UnderlayPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Underlay logic should include `{needle}` for centralized source-state derivation."
        );
    }
}

#[test]
fn underlay_view_uses_logic_state_model() {
    let source = load_source("src/underlay/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(UnderlayPartStateInput {",
        "slot: UnderlaySlot::Root",
        "has_custom_transparent",
        "has_custom_disabled",
        "has_custom_close_handler: has_on_close",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "if !state.is_interactive",
        "on_close.run(())",
        "on:click=on_click",
    ] {
        assert!(
            source.contains(needle),
            "Underlay view should derive behavior via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn underlay_exposes_spectrum_style_state_and_source_markers() {
    let source = load_source("src/underlay/view.rs");

    for attr in [
        "data-slot=move || state.get().slot_attr",
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open_attr",
        "data-transparent=move || state.get().transparent_attr",
        "data-disabled=move || state.get().disabled_attr",
        "data-interactive=move || state.get().interactive_attr",
        "data-tone=move || state.get().tone_attr",
        "data-close-mode=move || state.get().close_mode_attr",
        "data-transparent-source=move || state.get().transparent_source_attr",
        "data-disabled-source=move || state.get().disabled_source_attr",
        "data-close-source=move || state.get().close_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-transparent=move || state.get().has_custom_transparent.then_some(\"true\")",
        "data-custom-disabled=move || state.get().has_custom_disabled.then_some(\"true\")",
        "data-custom-close=move || state.get().has_custom_close_handler.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Underlay should expose `{attr}` for Spectrum-style styling and source inspection."
        );
    }
}

#[test]
fn underlay_styles_include_state_and_source_contracts() {
    let source = load_source("src/underlay/styles.rs");

    for selector in [
        ".ui-underlay",
        ".ui-underlay--open",
        ".ui-underlay[data-open=\"true\"]",
        ".ui-underlay[data-state=\"open\"]",
        ".ui-underlay--transparent",
        ".ui-underlay[data-transparent=\"true\"]",
        ".ui-underlay[data-tone=\"transparent\"]",
        ".ui-underlay--interactive",
        ".ui-underlay[data-interactive=\"true\"]",
        ".ui-underlay[data-close-mode=\"interactive\"]",
        ".ui-underlay--disabled",
        ".ui-underlay[data-disabled=\"true\"]",
        ".ui-underlay[data-state=\"disabled\"]",
        ".ui-underlay[data-transparent-source=\"custom\"]",
        ".ui-underlay[data-custom-transparent=\"true\"]",
        ".ui-underlay[data-disabled-source=\"custom\"]",
        ".ui-underlay[data-custom-disabled=\"true\"]",
        ".ui-underlay[data-close-source=\"custom\"]",
        ".ui-underlay[data-custom-close=\"true\"]",
        ".ui-underlay[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Underlay styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn underlay_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::underlay::styles::CSS);"),
        "ui-components css aggregator should include underlay styles."
    );
}

#[test]
fn underlay_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn underlay() -> AnyView",
        "title=\"Underlay\"",
        "slug=\"underlay\"",
        "State + Source Markers",
        "data-transparent-source",
        "data-disabled-source",
        "data-close-source",
        "data-class-source",
        "<Underlay",
    ] {
        assert!(
            source.contains(needle),
            "underlay docs page should contain `{needle}`."
        );
    }
}
