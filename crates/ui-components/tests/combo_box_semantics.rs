use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn combo_box_escape_stops_propagation_when_open() {
    let source = load_source("src/combo_box/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "ComboBox should stop Escape from bubbling when its popup is open (so parent overlays don't close)."
    );
    assert!(
        source.contains("key == \"Escape\""),
        "ComboBox should conditionally stop propagation only for Escape."
    );
    assert!(
        source.contains("was_open"),
        "ComboBox should only stop propagation when it was open (so Escape still closes parent overlays when closed)."
    );
}

#[test]
fn combo_box_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("src/combo_box/view.rs");

    assert!(
        source.contains("<Portal>"),
        "ComboBox panel should render in a Portal to avoid overflow clipping (Spectrum parity)."
    );
    assert!(
        source.contains("use_popover_position"),
        "ComboBox panel should use headless popover positioning (flip/clamp) rather than absolute offsets."
    );
    assert!(
        source.contains("data-ui-overlay-portal"),
        "ComboBox panel portal root should be marked as an overlay portal so modal aria-hidden logic doesn't hide it."
    );
    assert!(
        source.contains("--ui-popover-top"),
        "ComboBox panel should set `--ui-popover-top/left/anchor-width` CSS vars for positioning."
    );
}

#[test]
fn combo_box_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("src/combo_box/styles.rs");

    assert!(
        source.contains("position: fixed;"),
        "ComboBox panel should use fixed positioning when portaled."
    );
    assert!(
        source.contains("var(--ui-popover-top"),
        "ComboBox panel should consume `--ui-popover-top` for viewport positioning."
    );
    assert!(
        source.contains("data-placement=\"bottom-start\""),
        "ComboBox panel styles should set transform origin based on `data-placement`."
    );
}

#[test]
fn combo_box_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/combo_box/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-required",
        "data-open",
        "data-empty",
    ] {
        assert!(
            source.contains(attr),
            "ComboBox should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}
