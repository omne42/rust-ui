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

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Link internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn link_uses_logic_state_model() {
    let view_source = load_source("src/link/view.rs");
    let logic_source = load_source("src/link/logic.rs");

    for needle in [
        "pub struct LinkStateInput",
        "pub struct LinkState",
        "pub fn normalize_href(",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: LinkStateInput)",
        "pub fn resolve_rel(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Link logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let href = logic::normalize_href(href);",
        "let rel = logic::normalize_optional_text(rel);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let state = logic::resolve_state(LinkStateInput {",
        "let rel = logic::resolve_rel(target, rel);",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "Link view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
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
        "href=if state.is_enabled { href } else { None }",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "tabindex=state.is_disabled.then_some(-1)",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-missing-href=(!state.has_href).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Link should wire disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn link_emits_baseline_style_data_attributes() {
    let source = load_source("src/link/view.rs");

    for needle in [
        "data-slot=\"link\"",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-target=state.target_kind",
        "data-rel=state.rel_source_attr",
        "data-aria-label=if state.has_aria_label { \"custom\" } else { \"none\" }",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "class:ui-link--focus-visible=move || focus_ring.is_focus_visible.get()",
    ] {
        assert!(
            source.contains(needle),
            "Link should include `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn link_styles_include_state_marker_contracts() {
    let source = load_source("src/link/styles.rs");

    for selector in [
        ".ui-link--enabled",
        ".ui-link[data-state=\"enabled\"]",
        ".ui-link[data-external=\"true\"]",
        ".ui-link[data-state=\"disabled\"]",
        ".ui-link[data-state=\"missing-href\"]",
        ".ui-link--rel-provided",
        ".ui-link[data-rel=\"provided\"]",
        ".ui-link--custom-class",
        ".ui-link[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Link styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for forbidden in [":hover", ":focus-visible"] {
        assert!(
            !source.contains(forbidden),
            "Link styles should not rely on `{forbidden}`; use headless-driven state attributes instead."
        );
    }
}

#[test]
fn link_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn link() -> AnyView",
        "title=\"Link\"",
        "slug=\"link\"",
        "Playground title=\"State Matrix\"",
        "Playground title=\"Custom Rel + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Link.",
        );
    }
}

#[test]
fn link_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"State Matrix\"",
        "<Link href=\"#/docs/welcome\".to_string()>\"Internal docs link\"</Link>",
        "<Link href=\"https://example.com\".to_string() target=\"_blank\">",
        "<Link href=\"#/docs/welcome\".to_string() disabled=true>\"Disabled\"</Link>",
        "<Link href=\"   \".to_string()>\"Missing href\"</Link>",
        "title=\"Custom Rel + Class\"",
        "rel=\"sponsored\".to_string()",
        "aria_label=\"Open partner documentation\".to_string()",
        "class_name=\"docs-link-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "link docs playgrounds should contain `{needle}`.",
        );
    }
}
