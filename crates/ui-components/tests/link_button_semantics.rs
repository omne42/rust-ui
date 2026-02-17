use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn link_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/link_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "LinkButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn link_button_uses_logic_state_model() {
    let view_source = load_source("src/link_button/view.rs");
    let logic_source = load_source("src/link_button/logic.rs");

    for needle in [
        "pub struct LinkButtonState",
        "pub target_kind: &'static str",
        "pub fn normalize_href(",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(",
        "pub fn resolve_rel(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "LinkButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let href = logic::normalize_href(href);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let state = logic::resolve_state(",
        "let rel = logic::resolve_rel(target, rel);",
        "let class = logic::compose_class_name(variant, size, class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "LinkButton view should derive state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn link_button_uses_headless_hover_and_focus_ring() {
    let source = load_source("src/link_button/view.rs");

    for needle in ["use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "LinkButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn link_button_supports_disabled_semantics_without_navigation() {
    let source = load_source("src/link_button/view.rs");

    for needle in [
        "href=if state.is_enabled { href } else { None }",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "tabindex=state.is_disabled.then_some(-1)",
        "data-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "LinkButton should wire disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn link_button_composes_button_variant_and_size_classes() {
    let source = load_source("src/link_button/logic.rs");

    for needle in [
        "variant.class_name().to_string()",
        "size.class_name().to_string()",
        "ui-link-button--disabled",
        "ui-link-button--enabled",
    ] {
        assert!(
            source.contains(needle),
            "LinkButton class composition should include `{needle}`."
        );
    }
}

#[test]
fn link_button_emits_baseline_style_data_attributes() {
    let source = load_source("src/link_button/view.rs");

    for needle in [
        "data-slot=\"link-button\"",
        "data-state=if state.is_disabled { \"disabled\" } else { \"enabled\" }",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-hovered",
        "data-target=state.target_kind",
        "data-rel=if state.has_explicit_rel",
    ] {
        assert!(
            source.contains(needle),
            "LinkButton should include `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn link_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn link_button() -> AnyView",
        "title=\"LinkButton\"",
        "slug=\"link-button\"",
        "description=\"Button styling on anchors with baseline-style disabled semantics and secure rel handling for external targets.\"",
        "<Playground",
        "title=\"External target + rel hardening\"",
        "code_signal=code",
        "<Playground title=\"Variant + size + disabled matrix\" code_signal=states_code>",
        "id_base=\"docs-link-button-variant\".to_string()",
        "id_base=\"docs-link-button-size\".to_string()",
        "aria_label=\"LinkButton variant\".to_string()",
        "aria_label=\"LinkButton size\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=open_in_new_tab set_checked=set_open_in_new_tab>",
        "\"Open in new tab (_blank)\"",
        "<Switch checked=sponsored_rel set_checked=set_sponsored_rel>",
        "\"Add sponsored rel\"",
        "<LinkButton",
        "href=\"https://example.com/docs\".to_string()",
        "target=\"_blank\"",
        "rel=rel.clone()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for link-button coverage.",
        );
    }
}

#[test]
fn link_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "code_signal=code",
        "if open_in_new_tab.get() {",
        "target=Some(\\\"_blank\\\")",
        "if sponsored_rel.get() {",
        "rel=Some(\\\"sponsored\\\".to_string())",
        "aria_label=\"Open docs in a new tab\".to_string()",
        "<LinkButton href=\"https://example.com/changelog\".to_string()>",
        "<LinkButton href=\"   \".to_string() variant=ButtonVariant::Ghost>",
        "_blank links auto-append noopener+noreferrer; blank href is normalized as non-navigable.",
        "href=\"https://example.com/xs\".to_string() size=ButtonSize::Xs",
        "href=\"https://example.com/s\".to_string() size=ButtonSize::S",
        "href=\"https://example.com/m\".to_string()",
        "size=ButtonSize::M",
        "href=\"https://example.com/l\".to_string()",
        "size=ButtonSize::L",
        "href=\"https://example.com/xl\".to_string()",
        "size=ButtonSize::Xl",
        "variant=ButtonVariant::Secondary",
        "href=\"https://example.com/disabled\".to_string() disabled=true",
        "href=\"https://example.com/disabled-ghost\".to_string()",
        "\"Disabled ghost\"",
    ] {
        assert!(
            source.contains(needle),
            "link-button docs playgrounds should contain `{needle}`.",
        );
    }
}
