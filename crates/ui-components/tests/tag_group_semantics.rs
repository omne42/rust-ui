use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tag_group_supports_spectrum_aria_contract() {
    let source = load_source("src/tag_group/view.rs");

    for attr in [
        "aria-labelledby",
        "aria-describedby",
        "aria-invalid",
        "aria-required",
        "data-slot=\"tag-group\"",
        "data-slot=\"tag-group-list\"",
        "data-slot=\"tag-group-item\"",
        "data-slot=\"tag-group-description\"",
        "data-slot=\"tag-group-error\"",
    ] {
        assert!(
            source.contains(attr),
            "TagGroup should provide `{attr}` for Spectrum-style semantics."
        );
    }
}

#[test]
fn tag_group_merges_external_and_internal_describedby_ids() {
    let source = load_source("src/tag_group/view.rs");
    assert!(
        source.contains("merge_describedby_ids"),
        "TagGroup should merge external aria-describedby with description/error ids."
    );
}

#[test]
fn tag_group_styles_include_description_and_error_states() {
    let source = load_source("src/tag_group/styles.rs");

    for selector in [
        ".ui-tag-group__description",
        ".ui-tag-group__error",
        ".ui-tag-group[data-invalid=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "TagGroup styles should include `{selector}`"
        );
    }
}
