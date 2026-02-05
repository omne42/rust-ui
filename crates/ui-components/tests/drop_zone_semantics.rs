use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn drop_zone_does_not_expose_logic_module() {
    let source = load_source("src/drop_zone/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "DropZone's `logic` module should stay private to avoid leaking DOM/web-sys details into the public API."
    );
}

#[test]
fn drop_zone_uses_headless_hover_and_focus_ring() {
    let source = load_source("src/drop_zone/view.rs");

    assert!(
        source.contains("use_focus_ring"),
        "DropZone should use `ui_headless::use_focus_ring` (not `:focus-visible`) to align focus-visible behavior with the global modality provider."
    );

    assert!(
        source.contains("use_hover"),
        "DropZone should use `ui_headless::use_hover` for hovered state."
    );
}

#[test]
fn drop_zone_emits_spectrum_style_data_attributes() {
    let source = load_source("src/drop_zone/view.rs");

    for attr in [
        "data-hovered",
        "data-focused",
        "data-focus-visible",
        "data-drop-target",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "DropZone should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn drop_zone_has_hidden_focus_button_and_paste_handler() {
    let source = load_source("src/drop_zone/view.rs");

    assert!(
        source.contains("class=\"ui-drop-zone__button\""),
        "DropZone should render a visually hidden focusable button to receive focus and clipboard events."
    );

    assert!(
        source.contains("on:paste"),
        "DropZone should support pasting files via `on:paste` on the hidden focus button."
    );
}

#[test]
fn drop_zone_has_spring_driven_highlight_css_var() {
    let source = load_source("src/drop_zone/styles.rs");

    assert!(
        source.contains("--ui-drop-zone-highlight"),
        "DropZone styles should define `--ui-drop-zone-highlight` for spring-driven hover/drop-target feedback."
    );
}
