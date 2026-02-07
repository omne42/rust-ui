use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn autocomplete_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/autocomplete/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Autocomplete internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn autocomplete_normalizes_label_text_and_placeholder() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "logic::normalize_label",
        "logic::resolve_placeholder",
        "logic::normalize_optional_text",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should use `{needle}` to keep text and labeling semantics stable."
        );
    }

    assert!(
        logic_source.contains("\"Options\".to_string()"),
        "Autocomplete logic should provide a stable fallback label for blank input labels."
    );
}

#[test]
fn autocomplete_escape_stops_propagation_when_open() {
    let source = load_source("src/autocomplete/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "Autocomplete should stop Escape from bubbling when its popup is open (so parent overlays don't close)."
    );
    assert!(
        source.contains("key == \"Escape\""),
        "Autocomplete should conditionally stop propagation only for Escape."
    );
    assert!(
        source.contains("was_open"),
        "Autocomplete should only stop propagation when it was open (so Escape still closes parent overlays when closed)."
    );
}

#[test]
fn autocomplete_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("src/autocomplete/view.rs");

    assert!(
        source.contains("<Portal>"),
        "Autocomplete panel should render in a Portal to avoid overflow clipping (Spectrum parity)."
    );
    assert!(
        source.contains("use_popover_position"),
        "Autocomplete panel should use headless popover positioning (flip/clamp) rather than absolute offsets."
    );
    assert!(
        source.contains("data-ui-overlay-portal"),
        "Autocomplete panel portal root should be marked as an overlay portal so modal aria-hidden logic doesn't hide it."
    );
    assert!(
        source.contains("--ui-popover-top"),
        "Autocomplete panel should set `--ui-popover-top/left/anchor-width` CSS vars for positioning."
    );
}

#[test]
fn autocomplete_panel_exposes_option_and_empty_state_slots() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "data-slot=\"autocomplete-listbox\"",
        "data-empty=move || filtered_indices.get().is_empty().then_some(\"true\")",
        "data-slot=\"autocomplete-option\"",
        "data-focused=move || (active_index.get() == filtered_index).then_some(\"true\")",
        "data-slot=\"autocomplete-empty\"",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should expose `{needle}` for Spectrum-style state styling and deterministic tests."
        );
    }
}

#[test]
fn autocomplete_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("src/autocomplete/styles.rs");

    assert!(
        source.contains("position: fixed;"),
        "Autocomplete panel should use fixed positioning when portaled."
    );
    assert!(
        source.contains("var(--ui-popover-top"),
        "Autocomplete panel should consume `--ui-popover-top` for viewport positioning."
    );
    assert!(
        source.contains("data-placement=\"bottom-start\""),
        "Autocomplete panel styles should set transform origin based on `data-placement`."
    );
    assert!(
        source.contains(".ui-autocomplete__empty"),
        "Autocomplete styles should define an explicit empty state presentation."
    );
}

#[test]
fn autocomplete_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/autocomplete/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-required",
        "data-open",
        "data-empty",
        "data-has-description",
        "data-has-error",
    ] {
        assert!(
            source.contains(attr),
            "Autocomplete should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}
