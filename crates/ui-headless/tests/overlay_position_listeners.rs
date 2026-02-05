use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn assert_captured_scroll_listener(source: &str, rel_path: &str) {
    assert!(
        !source.contains("add_event_listener_with_callback(\"scroll\""),
        "{rel_path}: expected scroll listeners to use capture=true so overlays reposition inside scroll containers."
    );

    let idx = source
        .find("add_event_listener_with_callback_and_bool")
        .unwrap_or_else(|| {
            panic!(
                "{rel_path}: expected `add_event_listener_with_callback_and_bool` to be used for the scroll listener."
            )
        });

    let end = (idx + 400).min(source.len());
    let snippet = &source[idx..end];
    assert!(
        snippet.contains("\"scroll\""),
        "{rel_path}: expected captured listener to be registered for the `scroll` event."
    );
    assert!(
        snippet.contains("true"),
        "{rel_path}: expected scroll listener to pass `true` for capture."
    );
}

#[test]
fn overlay_position_scroll_listeners_capture_scroll_container_events() {
    let popover = load_source("src/popover_position.rs");
    assert_captured_scroll_listener(&popover, "src/popover_position.rs");

    let tooltip = load_source("src/tooltip_position.rs");
    assert_captured_scroll_listener(&tooltip, "src/tooltip_position.rs");
}
