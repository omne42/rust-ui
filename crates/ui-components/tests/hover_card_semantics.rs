use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn tag_name_for_data_slot(source: &str, slot: &str) -> String {
    let needle = format!("data-slot=\"{slot}\"");
    let idx = source
        .find(&needle)
        .unwrap_or_else(|| panic!("expected `{needle}` in source"));
    let before = &source[..idx];
    let tag_start = before
        .rfind('<')
        .unwrap_or_else(|| panic!("expected a tag before `{needle}`"));
    let after_lt = &before[tag_start + 1..];
    after_lt
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .collect()
}

#[test]
fn hover_card_trigger_does_not_wrap_children_in_button() {
    let source = load_source("src/hover_card/view.rs");

    assert!(
        !source.contains("aria-describedby="),
        "HoverCard should not bind `aria-describedby` directly in markup; attach it to the focused element dynamically."
    );

    let tag = tag_name_for_data_slot(&source, "hover-card-trigger");
    assert_eq!(
        tag, "span",
        "HoverCard trigger wrapper must be a non-interactive <span> to avoid nested interactive elements."
    );
}

#[test]
fn hover_card_escape_stops_propagation_when_open() {
    let source = load_source("src/hover_card/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "HoverCard should stop Escape propagation while open so it doesn't dismiss parent overlays."
    );
    assert!(
        source.contains("open_signal.get_untracked()"),
        "HoverCard should only intercept Escape when open (otherwise allow Escape to bubble to parent overlays)."
    );
    assert!(
        source.contains("is_composing"),
        "HoverCard should ignore Escape while IME composition is active."
    );
}

#[test]
fn hover_card_emits_root_state_and_motion_data_attributes() {
    let source = load_source("src/hover_card/view.rs");

    for attr in [
        "data-slot=\"hover-card\"",
        "data-state=move || if open_signal.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
        "data-disabled=disabled.then_some(\"true\")",
        "data-enabled=(!disabled).then_some(\"true\")",
        "data-motion-source=if motion == HoverCardMotion::default()",
        "data-custom-motion=(motion != HoverCardMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "HoverCard should expose `{attr}` for Spectrum-style state and motion selectors."
        );
    }
}

#[test]
fn hover_card_styles_include_motion_and_state_markers() {
    let source = load_source("src/hover_card/styles.rs");

    for selector in [
        ".ui-hover-card[data-motion-source=\"custom\"]",
        ".ui-hover-card[data-custom-motion=\"true\"]",
        ".ui-hover-card[data-disabled=\"true\"]",
        ".ui-hover-card[data-state=\"open\"]",
        ".ui-hover-card[data-open=\"true\"]",
        ".ui-hover-card[data-state=\"closed\"]",
        ".ui-hover-card[data-closed=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "HoverCard styles should include `{selector}` as stable marker contracts."
        );
    }
}

#[test]
fn hover_card_motion_contract_exposes_default_and_customization_tests() {
    let mod_source = load_source("src/hover_card/mod.rs");
    let motion_source = load_source("src/hover_card/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::HoverCardMotion;",
        "pub struct HoverCardMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "HoverCard motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}
