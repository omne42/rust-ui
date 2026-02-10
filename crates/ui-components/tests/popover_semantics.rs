use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn popover_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/popover/view.rs");

    assert!(
        source.contains("default_prevented"),
        "Popover should not close on Escape when a child already called preventDefault (Spectrum parity for Escape-to-clear flows)."
    );
    assert!(
        source.contains("is_composing"),
        "Popover should ignore Escape while IME composition is active (matches React Spectrum's `useOverlay`)."
    );
    assert!(
        source.contains("stop_propagation()"),
        "Popover should stop Escape propagation when closing to avoid cascading dismiss handlers."
    );
}

#[test]
fn popover_emits_spectrum_root_state_and_motion_data_attributes() {
    let source = load_source("src/popover/view.rs");

    for attr in [
        "data-slot=\"popover\"",
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-placement=move || position.placement.get().as_str()",
        "data-motion-source=if motion == PopoverMotion::default()",
        "data-custom-motion=(motion != PopoverMotion::default()).then_some(\"true\")",
        "data-ui-overlay-portal=\"\"",
    ] {
        assert!(
            source.contains(attr),
            "Popover should expose `{attr}` for Spectrum-style root state and motion contract selectors."
        );
    }
}

#[test]
fn popover_styles_include_motion_and_open_state_markers() {
    let source = load_source("src/popover/styles.rs");

    for selector in [
        ".ui-popover[data-motion-source=\"custom\"]",
        ".ui-popover[data-custom-motion=\"true\"]",
        ".ui-popover[data-state=\"open\"]",
        ".ui-popover[data-open=\"true\"]",
        ".ui-popover[data-state=\"closed\"]",
        ".ui-popover[data-closed=\"true\"]",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
        ".ui-popover__panel[data-placement=\"bottom-end\"]",
        ".ui-popover__panel[data-placement=\"top-start\"]",
        ".ui-popover__panel[data-placement=\"top-end\"]",
    ] {
        assert!(
            source.contains(selector),
            "Popover styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn popover_motion_contract_exposes_default_and_placement_offset_helpers() {
    let mod_source = load_source("src/popover/mod.rs");
    let motion_source = load_source("src/popover/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::PopoverMotion;",
        "pub struct PopoverMotion",
        "fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64",
        "fn default_motion_matches_heroui_style_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Popover motion contract should include `{needle}` for HeroUI-level spring configuration and directional offsets."
        );
    }
}
