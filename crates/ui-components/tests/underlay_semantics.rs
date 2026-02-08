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
fn underlay_uses_logic_state_model() {
    let logic_source = load_source("src/underlay/logic.rs");
    let view_source = load_source("src/underlay/view.rs");

    for needle in [
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn normalize_optional_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Underlay logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_state(UnderlayStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "on:click=on_click",
    ] {
        assert!(
            view_source.contains(needle),
            "Underlay view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn underlay_exposes_spectrum_style_data_markers() {
    let source = load_source("src/underlay/view.rs");

    for attr in [
        "data-slot=\"underlay\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-transparent=move || state.get().is_transparent.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-interactive=move || state.get().is_interactive.then_some(\"true\")",
        "data-tone=move || state.get().tone_attr",
        "data-close-mode=move || state.get().close_mode_attr",
    ] {
        assert!(
            source.contains(attr),
            "Underlay should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn underlay_styles_include_open_transparent_disabled_and_custom_contracts() {
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
        ".ui-underlay--custom-class",
        ".ui-underlay[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Underlay styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
