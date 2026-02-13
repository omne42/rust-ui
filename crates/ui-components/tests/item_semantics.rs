use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn item_does_not_expose_view_module() {
    let source = load_source("src/item/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Item internals should stay private; found `pub mod view`."
    );
}

#[test]
fn item_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/item/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::{"),
        "item module should export item primitive family."
    );
    assert!(
        crate_source.contains("pub use item::{"),
        "crate root should re-export item primitive contracts."
    );
}

#[test]
fn item_primitives_expose_slot_contracts() {
    let source = load_source("src/item/view.rs");

    for needle in [
        "pub fn ItemGroup(",
        "pub fn ItemSeparator(",
        "pub fn Item(",
        "pub fn ItemMedia(",
        "pub fn ItemContent(",
        "pub fn ItemTitle(",
        "pub fn ItemDescription(",
        "pub fn ItemActions(",
        "pub fn ItemHeader(",
        "pub fn ItemFooter(",
        "data-slot=\"item-group\"",
        "data-slot=\"item-separator\"",
        "data-slot=\"item\"",
        "data-slot=\"item-media\"",
        "data-slot=\"item-content\"",
        "data-slot=\"item-title\"",
        "data-slot=\"item-description\"",
        "data-slot=\"item-actions\"",
        "data-slot=\"item-header\"",
        "data-slot=\"item-footer\"",
    ] {
        assert!(
            source.contains(needle),
            "Item primitive family should include `{needle}`."
        );
    }
}

#[test]
fn item_group_and_item_keep_list_semantics() {
    let source = load_source("src/item/view.rs");

    for needle in [
        "<div class=class_name role=\"list\" data-slot=\"item-group\">",
        "role=\"listitem\"",
    ] {
        assert!(
            source.contains(needle),
            "Item primitives should preserve list semantics via `{needle}`."
        );
    }
}

#[test]
fn item_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in [
        "pub(super) fn item_primitives() -> AnyView",
        "title=\"Item\"",
        "slug=\"item\"",
        "<Item",
    ] {
        assert!(
            source.contains(needle),
            "collections_item_shadcn docs page should contain `{needle}`."
        );
    }
}

#[test]
fn item_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in [
        "pub(super) fn item_primitives() -> AnyView",
        "title=\"Item\"",
        "slug=\"item\"",
        "title=\"Media + Content + Actions\"",
        "title=\"Header + Footer Layout\"",
    ] {
        assert!(
            source.contains(needle),
            "collections_item_shadcn docs page should contain `{needle}` for Item.",
        );
    }
}

#[test]
fn item_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_item_shadcn.rs");

    for needle in [
        "title=\"Media + Content + Actions\"",
        "variant=variant size=size",
        "variant=ItemMediaVariant::Icon",
        "<ItemSeparator />",
        "<Item>",
        "variant=ItemMediaVariant::Image",
        "title=\"Header + Footer Layout\"",
        "variant=ItemVariant::Muted size=ItemSize::Sm",
        "\"Status: degraded\"",
    ] {
        assert!(
            source.contains(needle),
            "item docs playgrounds should contain `{needle}`.",
        );
    }
}
