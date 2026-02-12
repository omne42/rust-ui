use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn layout_module_reexports_flex_and_grid_contracts() {
    let source = load_source("src/layout/mod.rs");

    for needle in [
        "pub use crate::flex::{Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap};",
        "pub use crate::grid::{Grid, GridAlign, GridColumns, GridGap, GridJustify, GridRows};",
    ] {
        assert!(
            source.contains(needle),
            "layout module should expose `{needle}` for react-spectrum layout compatibility."
        );
    }
}

#[test]
fn crate_root_registers_layout_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod layout;"),
        "crate root should include `pub mod layout;` for @react-spectrum/layout compatibility."
    );
}

#[test]
fn layout_compatibility_reuses_flex_and_grid_docs_playgrounds() {
    let layout_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let layout_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in ["title=\"Flex\"", "slug=\"flex\"", "<Flex"] {
        assert!(
            layout_source.contains(needle),
            "layout docs should contain `{needle}` for Flex compatibility coverage."
        );
    }

    for needle in ["title=\"Grid\"", "slug=\"grid\"", "<Grid"] {
        assert!(
            layout_extra_source.contains(needle),
            "layout_extra docs should contain `{needle}` for Grid compatibility coverage."
        );
    }
}

#[test]
fn layout_module_docs_page_covers_primary_playgrounds() {
    let layout_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let layout_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn flex() -> AnyView",
        "title=\"Flex\"",
        "slug=\"flex\"",
        "description=\"Spectrum-style flex layout primitive with centralized direction/wrap/alignment/gap normalization and stable data-state contracts.\"",
        "<Playground title=\"Direction + Wrap + Gap\" code=matrix_code>",
        "<Playground title=\"Inline + Distribution\" code=inline_code>",
        "<Flex",
    ] {
        assert!(
            layout_source.contains(needle),
            "layout docs should include `{needle}` for layout module Flex primary playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn grid() -> AnyView",
        "title=\"Grid\"",
        "slug=\"grid\"",
        "description=\"Spectrum-style grid layout primitive with centralized columns/rows/gap/alignment normalization and stable state-marker contracts.\"",
        "<Playground title=\"Columns + Gap\" code=columns_code>",
        "<Playground title=\"AutoFit + Dense + Equal Rows\" code=adaptive_code>",
        "<Grid",
    ] {
        assert!(
            layout_extra_source.contains(needle),
            "layout_extra docs should include `{needle}` for layout module Grid primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"layout\" => &[\"flex\", \"grid\"]"),
        "components mod mapping should keep `layout` mapped to `flex` and `grid` slugs.",
    );
}

#[test]
fn layout_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let layout_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let layout_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Direction + Wrap + Gap\"",
        "direction=FlexDirection::Row",
        "wrap=FlexWrap::Wrap",
        "gap=FlexGap::Md",
        "align=FlexAlign::Center",
        "aria_label=\"Tag cloud layout\".to_string()",
        "direction=FlexDirection::Column",
        "class_name=\"docs-flex-column\".to_string()",
        "title=\"Inline + Distribution\"",
        "inline=true",
        "justify=FlexJustify::SpaceBetween",
        "align=FlexAlign::Baseline",
        "gap=FlexGap::Lg",
        "class_name=\"docs-flex-inline\".to_string()",
    ] {
        assert!(
            layout_source.contains(needle),
            "layout docs playgrounds should contain `{needle}` for layout module Flex contracts.",
        );
    }

    for needle in [
        "title=\"Columns + Gap\"",
        "columns=GridColumns::Three",
        "gap=GridGap::Md",
        "aria_label=\"Overview cards grid\".to_string()",
        "title=\"AutoFit + Dense + Equal Rows\"",
        "columns=GridColumns::AutoFit",
        "rows=GridRows::Equal",
        "gap=GridGap::Lg",
        "justify=GridJustify::Stretch",
        "align=GridAlign::Stretch",
        "dense=true",
        "class_name=\"docs-grid-adaptive\".to_string()",
    ] {
        assert!(
            layout_extra_source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for layout module Grid contracts.",
        );
    }
}
