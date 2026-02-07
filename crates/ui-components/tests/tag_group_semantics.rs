use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tag_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tag_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TagGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tag_group_uses_logic_state_model() {
    let view_source = load_source("src/tag_group/view.rs");
    let logic_source = load_source("src/tag_group/logic.rs");

    for needle in [
        "pub struct TagGroupState",
        "pub fn resolve_state(",
        "pub item_count: usize",
        "pub has_disabled_tags: bool",
        "pub has_removable_tags: bool",
        "pub is_invalid: bool",
        "pub is_required: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "TagGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "resolve_state(",
        "has_remove_callback",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
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
fn tag_group_emits_root_and_item_state_attrs() {
    let source = load_source("src/tag_group/view.rs");

    for attr in [
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-disabled-tags=move || state.get().has_disabled_tags.then_some(\"true\")",
        "data-has-removable-tags=move || state.get().has_removable_tags.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-index=index",
        "data-tag-id=tag_id_for_attr",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-removable=is_removable.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "TagGroup should expose `{attr}` for Spectrum-style styling and regression checks."
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
