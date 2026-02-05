use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn progress_sets_value_text_for_screen_readers() {
    let source = load_source("src/progress/view.rs");

    assert!(
        source.contains("aria-valuetext"),
        "Progress should set `aria-valuetext` for determinate values (React Spectrum parity via useProgressBar)."
    );
    assert!(
        source.contains("value_label_text"),
        "Progress should derive a stable `value_label_text` signal for `aria-valuetext`."
    );
}

#[test]
fn progress_filters_non_finite_values() {
    let source = load_source("src/progress/logic.rs");

    assert!(
        source.contains("!value.is_finite()"),
        "Progress clamp logic should treat non-finite inputs as min to avoid emitting NaN into motion CSS variables."
    );
}
