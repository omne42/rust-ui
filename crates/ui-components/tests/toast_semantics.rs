use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toast_viewport_marks_portaled_content_as_overlay_portal() {
    let source = load_source("src/toast/view.rs");

    assert!(
        source.contains("<Portal>"),
        "ToastViewport should render in a Portal by default."
    );
    assert!(
        source.contains("data-ui-overlay-portal"),
        "ToastViewport portal root should be marked as an overlay portal so modal aria-hidden logic doesn't hide it."
    );
}
