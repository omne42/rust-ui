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
