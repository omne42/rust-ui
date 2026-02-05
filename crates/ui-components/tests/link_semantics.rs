use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn link_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/link/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Link's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
    assert!(
        !source.contains("pub mod view"),
        "Link's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn link_uses_headless_hover_and_focus_ring() {
    let source = load_source("src/link/view.rs");

    for needle in ["use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Link should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn link_supports_disabled_semantics_without_navigation() {
    let source = load_source("src/link/view.rs");

    for needle in [
        "href=(!disabled).then_some(href)",
        "aria-disabled=disabled.then_some(\"true\")",
        "tabindex=disabled.then_some(-1)",
        "data-disabled=disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Link should wire disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn link_emits_spectrum_style_data_attributes() {
    let source = load_source("src/link/view.rs");

    for needle in [
        "data-slot=\"link\"",
        "data-hovered",
        "data-focused",
        "data-focus-visible",
        "ui-link--focus-visible",
    ] {
        assert!(
            source.contains(needle),
            "Link should include `{needle}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn link_styles_use_state_data_attributes_instead_of_pseudo_classes() {
    let source = load_source("src/link/styles.rs");

    for needle in ["data-hovered=\"true\"", "ui-link--focus-visible"] {
        assert!(
            source.contains(needle),
            "Link styles should include `{needle}` to match the component's state contracts."
        );
    }

    for forbidden in [":hover", ":focus-visible"] {
        assert!(
            !source.contains(forbidden),
            "Link styles should not rely on `{forbidden}`; use headless-driven state attributes instead."
        );
    }
}
