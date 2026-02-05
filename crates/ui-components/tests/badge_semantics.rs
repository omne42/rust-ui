use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn badge_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/badge/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Badge's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
    assert!(
        !source.contains("pub mod view"),
        "Badge's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn badge_emits_spectrum_style_data_attributes() {
    let source = load_source("src/badge/view.rs");

    assert!(
        source.contains("data-slot=\"badge\""),
        "Badge should set `data-slot=\"badge\"` for Spectrum-style styling and inspection."
    );
}

#[test]
fn badge_uses_variant_class_names() {
    let source = load_source("src/badge/view.rs");

    for needle in ["ui-badge", "variant.class_name()"] {
        assert!(
            source.contains(needle),
            "Badge should compose styles via `{needle}`."
        );
    }
}
