use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_field_clears_on_escape_when_not_empty() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("key != \"Escape\""),
        "SearchField should treat Escape as a clear shortcut (Spectrum parity)."
    );
    assert!(
        source.contains("value.get_untracked().is_empty()"),
        "SearchField should only clear on Escape when a value is present (otherwise let Escape propagate)."
    );
    assert!(
        source.contains("set_value.set(String::new())"),
        "SearchField should clear its value on Escape."
    );
}

#[test]
fn search_field_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "SearchField clear button should be excluded from tab order like React Spectrum."
    );
}

#[test]
fn search_field_clear_button_is_presence_safe() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("data-visible"),
        "SearchField should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || state.show_clear_button.get()"),
        "SearchField should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn search_field_attaches_clear_motion_driver() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("attach_clear_motion"),
        "SearchField should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn search_field_styles_define_clear_motion_css_vars() {
    let source = load_source("src/search_field/styles.rs");

    assert!(
        source.contains("--ui-search-field-clear-opacity"),
        "SearchField styles should define `--ui-search-field-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-search-field-clear-scale"),
        "SearchField styles should define `--ui-search-field-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn search_field_motion_uses_spring_animator() {
    let source = load_source("src/search_field/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SearchField motion should be spring-driven to match the repo's motion spec."
    );
}
