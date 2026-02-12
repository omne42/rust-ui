use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn iconset_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/iconset/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Iconset internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn iconset_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/iconset/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Iconset;"),
        "iconset module should export `Iconset`."
    );
    assert!(
        module_source.contains("pub struct IconsetGlyph"),
        "iconset module should expose `IconsetGlyph` data contract."
    );
    assert!(
        crate_source
            .contains("pub use iconset::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};"),
        "crate root should re-export Iconset contracts."
    );
}

#[test]
fn iconset_logic_exposes_state_helpers() {
    let source = load_source("src/iconset/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn parse_icon_reference(icon: &str)",
        "pub fn resolve_iconset_namespace(",
        "pub fn glyph_matches(candidate_name: &str, iconset: &str, icon_name: &str)",
        "pub fn resolve_registry_glyph(",
        "pub fn resolve_accessible_label(",
        "pub fn resolve_state(input: IconsetStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: IconsetState)",
        "DEFAULT_ICONSET_NAMESPACE",
    ] {
        assert!(
            source.contains(needle),
            "Iconset logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn iconset_view_uses_logic_state_contracts() {
    let source = load_source("src/iconset/view.rs");

    for needle in [
        "pub fn Iconset(",
        "logic::parse_icon_reference(&icon)",
        "logic::resolve_iconset_namespace(iconset_from_prop, iconset_from_icon)",
        "logic::resolve_registry_glyph(glyphs, &resolved_iconset, &icon_name)",
        "logic::resolve_state(IconsetStateInput {",
        "logic::resolve_accessible_label(decorative, custom_aria_label, registry_label, &icon_name)",
        "logic::compose_class_name(class_name, state)",
        "<Icon",
        "data-slot=\"iconset\"",
        "data-state=state.state_attr",
        "data-icon-source=state.icon_source_attr",
        "data-iconset-source=state.iconset_source_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "data-size-source=state.size_source_attr",
        "data-tone-source=state.tone_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Iconset view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn iconset_styles_include_state_and_source_markers() {
    let source = load_source("src/iconset/styles.rs");

    for selector in [
        ".ui-iconset {",
        ".ui-iconset[data-state=\"disabled\"]",
        ".ui-iconset[data-state=\"decorative\"]",
        ".ui-iconset[data-state=\"fallback\"]",
        ".ui-iconset[data-icon-source=\"registry\"]",
        ".ui-iconset[data-icon-source=\"fallback\"]",
        ".ui-iconset[data-iconset-source=\"prop\"]",
        ".ui-iconset[data-iconset-source=\"icon\"]",
        ".ui-iconset[data-iconset-source=\"default\"]",
        ".ui-iconset[data-label-source=\"custom\"]",
        ".ui-iconset[data-label-source=\"registry\"]",
        ".ui-iconset[data-label-source=\"fallback\"]",
        ".ui-iconset[data-class-source=\"custom\"]",
        ".ui-iconset[data-size-source=\"custom\"]",
        ".ui-iconset[data-tone-source=\"custom\"]",
        ".ui-iconset--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Iconset styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn iconset_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::iconset::styles::CSS);"),
        "ui-components css aggregator should include iconset styles."
    );
}

#[test]
fn iconset_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "pub(super) fn iconset() -> AnyView",
        "title=\"Iconset\"",
        "slug=\"iconset\"",
        "State + Source Markers",
        "data-label-source",
        "<Iconset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_iconset docs page should contain `{needle}`."
        );
    }
}

#[test]
fn iconset_docs_default_playgrounds_lock_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "<Playground title=\"Registry Namespace Resolution\" code=registry_code>",
        "icon=\"workflow:check\".to_string()",
        "icon=\"workflow:alert\".to_string()",
        "glyphs=workflow_glyphs.clone()",
        "size=IconsetSize::Md",
        "tone=IconsetTone::Accent",
        "tone=IconsetTone::Danger",
        "decorative=false",
        "<Playground title=\"Fallback + Source State\" code=fallback_code>",
        "icon=\"ui:unknown\".to_string()",
        "iconset=\"ui\".to_string()",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Muted",
        "class_name=\"docs-iconset-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs default playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn iconset_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "icon=\"workflow:check\".to_string()",
        "iconset=\"workflow\".to_string()",
        "glyphs=vec![",
        "IconsetGlyph::new(\"workflow:check\", \"✓\")",
        ".with_aria_label(\"Registry Check\")",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Danger",
        "aria_label=\"Explicit workflow check\".to_string()",
        "class_name=\"docs-iconset-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn iconset_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "pub(super) fn iconset() -> AnyView",
        "title=\"Iconset\"",
        "slug=\"iconset\"",
        "description=\"Spectrum-compatible Iconset registry wrapper for namespace + icon-name resolution, composed on Icon accessibility contracts with stable source markers.\"",
        "<Playground title=\"Registry Namespace Resolution\" code=registry_code>",
        "<Playground title=\"Fallback + Source State\" code=fallback_code>",
        "title=\"State + Source Markers\"",
        "<Iconset",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_iconset docs should include `{needle}` for iconset primary playground coverage.",
        );
    }
}

#[test]
fn iconset_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_iconset.rs");

    for needle in [
        "title=\"Registry Namespace Resolution\"",
        "icon=\"workflow:check\".to_string()",
        "icon=\"workflow:alert\".to_string()",
        "glyphs=workflow_glyphs.clone()",
        "size=IconsetSize::Md",
        "tone=IconsetTone::Accent",
        "tone=IconsetTone::Danger",
        "title=\"Fallback + Source State\"",
        "icon=\"ui:unknown\".to_string()",
        "iconset=\"ui\".to_string()",
        "size=IconsetSize::Lg",
        "tone=IconsetTone::Muted",
        "class_name=\"docs-iconset-custom\".to_string()",
        "title=\"State + Source Markers\"",
        "icon=\"workflow:check\".to_string()",
        "iconset=\"workflow\".to_string()",
        "IconsetGlyph::new(\"workflow:check\", \"✓\")",
        ".with_aria_label(\"Registry Check\")",
        "aria_label=\"Explicit workflow check\".to_string()",
        "class_name=\"docs-iconset-state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "iconset docs playgrounds should contain `{needle}`.",
        );
    }
}
