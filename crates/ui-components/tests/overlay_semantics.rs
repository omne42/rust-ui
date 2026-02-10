use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn overlay_escape_respects_default_prevented_and_composition() {
    let source = load_source("src/overlay/view.rs");

    assert!(
        source.contains("default_prevented"),
        "Overlay should not close on Escape when a child already called preventDefault (Spectrum parity for Escape-to-clear flows)."
    );
    assert!(
        source.contains("is_composing"),
        "Overlay should ignore Escape while IME composition is active (matches React Spectrum's `useOverlay`)."
    );
    assert!(
        source.contains("stop_propagation()"),
        "Overlay should stop Escape propagation when closing to avoid cascading dismiss handlers."
    );
}

#[test]
fn overlay_supports_dismissable_and_keyboard_dismiss_flags() {
    let source = load_source("src/overlay/view.rs");

    for needle in [
        "is_dismissable",
        "is_keyboard_dismiss_disabled",
        "if is_dismissable",
        "!is_keyboard_dismiss_disabled",
    ] {
        assert!(
            source.contains(needle),
            "Overlay should support Spectrum-style dismiss control flags (`{needle}`)."
        );
    }
}

#[test]
fn overlay_emits_root_state_and_motion_data_attributes() {
    let source = load_source("src/overlay/view.rs");

    for needle in [
        "data-slot=\"overlay\"",
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-dismissable=is_dismissable.then_some(\"true\")",
        "data-keyboard-dismiss-disabled=is_keyboard_dismiss_disabled.then_some(\"true\")",
        "data-motion-source=if motion == OverlayMotion::default()",
        "data-custom-motion=(motion != OverlayMotion::default()).then_some(\"true\")",
        "data-ui-overlay-portal=\"\"",
    ] {
        assert!(
            source.contains(needle),
            "Overlay should expose `{needle}` for Spectrum-style root state and motion contract selectors."
        );
    }
}

#[test]
fn overlay_styles_include_motion_and_dismiss_markers() {
    let source = load_source("src/overlay/styles.rs");

    for selector in [
        ".ui-overlay[data-motion-source=\"custom\"]",
        ".ui-overlay[data-custom-motion=\"true\"]",
        ".ui-overlay[data-state=\"open\"]",
        ".ui-overlay[data-open=\"true\"]",
        ".ui-overlay[data-state=\"closed\"]",
        ".ui-overlay[data-closed=\"true\"]",
        ".ui-overlay[data-dismissable=\"true\"] .ui-overlay__backdrop",
        ".ui-overlay[data-keyboard-dismiss-disabled=\"true\"] .ui-overlay__panel",
    ] {
        assert!(
            source.contains(selector),
            "Overlay styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn overlay_motion_contract_exposes_default_and_customization_tests() {
    let mod_source = load_source("src/overlay/mod.rs");
    let motion_source = load_source("src/overlay/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::OverlayMotion;",
        "pub struct OverlayMotion",
        "fn default_motion_uses_flip3d_spring_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Overlay motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}
