use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_clears_on_escape_when_clearable_and_not_empty() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("key != \"Escape\""),
        "Input should treat Escape as a clear shortcut when `is_clearable` is enabled."
    );
    assert!(
        source.contains("!is_clearable"),
        "Input Escape handling should gate on `is_clearable` to avoid surprising clears."
    );
    assert!(
        source.contains("is_empty.get_untracked()"),
        "Input should only clear on Escape when a value is present (otherwise let Escape propagate)."
    );
    assert!(
        source.contains("set_value.set(String::new())"),
        "Input should clear its value on Escape."
    );
}

#[test]
fn input_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "Input clear button should be excluded from tab order to avoid extra Tab stops."
    );
}

#[test]
fn input_clear_button_is_presence_safe() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("data-visible"),
        "Input should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || view_state.get().show_clear"),
        "Input should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn input_attaches_clear_motion_driver() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("attach_clear_button_motion"),
        "Input should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn input_styles_define_clear_motion_css_vars() {
    let source = load_source("src/input/styles.rs");

    assert!(
        source.contains("--ui-input-clear-opacity"),
        "Input styles should define `--ui-input-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-input-clear-scale"),
        "Input styles should define `--ui-input-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn input_motion_uses_spring_animator() {
    let source = load_source("src/input/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Input motion should be spring-driven to match the repo's motion spec."
    );
}
