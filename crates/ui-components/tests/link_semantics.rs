use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn link_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/link/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Link internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn link_uses_logic_state_model() {
    let view_source = load_source("../../components/link/src/view.rs");
    let logic_source = load_source("../../components/link/src/logic.rs");

    for needle in [
        "pub use ui_state_primitives::link::",
        "LinkStateInput",
        "LinkState",
        "LinkTargetKind",
        "LinkVisualState",
        "LinkRelSource",
        "normalize_href",
        "normalize_optional_text",
        "normalize_is_disabled",
        "LinkDisabledSource",
        "resolve_state",
        "resolve_target_kind",
        "resolve_rel",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Link logic should consume state primitives and include `{needle}`."
        );
    }

    for needle in [
        "let href = logic::normalize_href(href);",
        "let (is_disabled, disabled_source) = logic::normalize_is_disabled(is_disabled);",
        "let rel = logic::normalize_optional_text(rel);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let locale = locale_attrs(lang, dir);",
        "let target_kind = logic::resolve_target_kind(target);",
        "let state = logic::resolve_state(LinkStateInput {",
        "let rel = logic::resolve_rel(target_kind, rel);",
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
    let source = load_source("../../components/link/src/view.rs");

    for needle in ["use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Link should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn link_supports_disabled_semantics_without_navigation() {
    let source = load_source("../../components/link/src/view.rs");

    for needle in [
        "href=if state.is_enabled { href } else { None }",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "tabindex=state.is_disabled.then_some(-1)",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-disabled-source=disabled_source.as_attr()",
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
    let source = load_source("../../components/link/src/view.rs");

    for needle in [
        "data-slot=\"link\"",
        "data-state=state.state.as_attr()",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-target=state.target_kind.as_attr()",
        "data-rel=state.rel_source.as_attr()",
        "data-aria-label=if state.has_aria_label { \"custom\" } else { \"none\" }",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-ui-schema=\"ui.link.agent-contract\"",
        "data-ui-schema-version=\"1\"",
        "data-ui-intent=\"navigation\"",
        "data-ui-action=\"navigate\"",
        "data-ui-state=state.state.as_attr()",
        "data-ui-source=disabled_source.as_attr()",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
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
    let source = load_source("../../components/link/src/styles.rs");

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
        "title=\"Hello World (Default API)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Internal / External / Disabled / Missing)\"",
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
        "title=\"Hello World (Default API)\"",
        "<Link href=\"#/docs/welcome\".to_string()>\"Read docs\"</Link>",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_source_path=\"components/link/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "title=\"Comparison Matrix (Internal / External / Disabled / Missing)\"",
        "<Link href=\"#/docs/welcome\".to_string()>\"Internal docs link\"</Link>",
        "<Link href=\"https://example.com\".to_string() target=\"_blank\">",
        "<Link href=\"#/docs/welcome\".to_string() is_disabled=true>",
        "<Link href=\"   \".to_string()>\"Missing href\"</Link>",
        "Some(\"sponsored\".to_string())",
        "\"docs-link-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "link docs playgrounds should contain `{needle}`.",
        );
    }
}
