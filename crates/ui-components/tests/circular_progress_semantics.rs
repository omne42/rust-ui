use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn circular_progress_is_an_indeterminate_progressbar() {
    let source = load_source("src/circular_progress/view.rs");

    assert!(
        source.contains("role=\"progressbar\""),
        "CircularProgress should use `role=progressbar` to match Spectrum-style loading indicators."
    );
    assert!(
        source.contains("aria-valuemin=\"0\""),
        "CircularProgress should set `aria-valuemin` for progressbar semantics."
    );
    assert!(
        source.contains("aria-valuemax=\"100\""),
        "CircularProgress should set `aria-valuemax` for progressbar semantics."
    );
    assert!(
        !source.contains("role=\"status\""),
        "CircularProgress should not use `role=status` (live region) for a progress indicator."
    );
}

#[test]
fn circular_progress_sanitizes_custom_size_vars() {
    let source = load_source("src/circular_progress/view.rs");

    assert!(
        source.contains("is_finite()"),
        "CircularProgress should ignore non-finite `size_px`/`thickness_px` values to avoid emitting NaN/Infinity into CSS variables."
    );
    assert!(
        source.contains("--ui-cp-size"),
        "CircularProgress should set custom sizing via `--ui-cp-size` CSS variables."
    );
    assert!(
        source.contains("--ui-cp-thickness"),
        "CircularProgress should set custom thickness via `--ui-cp-thickness` CSS variables."
    );
}
