use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn auto_height_wraps_children_in_content_container() {
    let source = load_source("src/auto_height/view.rs");

    assert!(
        source.contains("ui-auto-height__content"),
        "AutoHeight should wrap children in a `.ui-auto-height__content` element for measurement and motion."
    );
}

#[test]
fn auto_height_attaches_motion_driver() {
    let source = load_source("src/auto_height/view.rs");

    assert!(
        source.contains("attach_motion"),
        "AutoHeight should attach its motion driver rather than ignoring the motion contract."
    );
}

#[test]
fn auto_height_defines_height_css_variable() {
    let source = load_source("src/auto_height/styles.rs");

    assert!(
        source.contains("--ui-auto-height-height"),
        "AutoHeight styles should use `--ui-auto-height-height` so motion updates only touch CSS variables."
    );
}

#[test]
fn auto_height_motion_uses_resize_observer_and_spring() {
    let source = load_source("src/auto_height/motion.rs");

    assert!(
        source.contains("ResizeObserver"),
        "AutoHeight motion should observe content size changes via ResizeObserver."
    );

    assert!(
        source.contains("SpringAnimator"),
        "AutoHeight motion should animate height changes via a spring."
    );
}
