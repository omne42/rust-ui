use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sidenav_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/sidenav/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sidenav internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sidenav_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/sidenav/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Sidenav;"),
        "sidenav module should export `Sidenav`."
    );
    assert!(
        crate_source.contains("pub use sidenav::Sidenav;"),
        "crate root should re-export `Sidenav`."
    );
}

#[test]
fn sidenav_logic_exposes_state_helpers() {
    let source = load_source("src/sidenav/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_trigger_label(",
        "pub fn normalize_shortcut_key(value: Option<String>, enable_shortcut: bool)",
        "pub fn resolve_state(input: SidenavStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: SidenavState)",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_TRIGGER_LABEL",
    ] {
        assert!(
            source.contains(needle),
            "Sidenav logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn sidenav_view_uses_logic_state_contracts() {
    let source = load_source("src/sidenav/view.rs");

    for needle in [
        "pub fn Sidenav(",
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_trigger_label(trigger_label)",
        "logic::normalize_shortcut_key(shortcut_key, enable_shortcut)",
        "logic::resolve_state(SidenavStateInput {",
        "logic::compose_class_name(class_name, state)",
        "<Sidebar",
        "on_open_change: Option<Callback<bool>>",
        "data-slot=\"sidenav\"",
        "data-state=state.state_attr",
        "data-open-mode=state.open_mode_attr",
        "data-initial-open=state.initial_open_attr",
        "data-trigger-mode=state.trigger_mode_attr",
        "data-shortcut-mode=state.shortcut_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-trigger-source=state.trigger_source_attr",
        "data-shortcut-source=state.shortcut_source_attr",
        "data-class-source=state.class_source_attr",
        "data-handler-source=state.handler_source_attr",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=(!state.is_controlled).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Sidenav view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn sidenav_styles_include_state_and_source_markers() {
    let source = load_source("src/sidenav/styles.rs");

    for selector in [
        ".ui-sidenav {",
        ".ui-sidenav[data-state=\"disabled\"]",
        ".ui-sidenav[data-open-mode=\"controlled\"]",
        ".ui-sidenav[data-open-mode=\"uncontrolled\"]",
        ".ui-sidenav[data-initial-open=\"open\"]",
        ".ui-sidenav[data-initial-open=\"closed\"]",
        ".ui-sidenav[data-trigger-mode=\"visible\"]",
        ".ui-sidenav[data-trigger-mode=\"hidden\"]",
        ".ui-sidenav[data-shortcut-mode=\"enabled\"]",
        ".ui-sidenav[data-shortcut-mode=\"disabled\"]",
        ".ui-sidenav[data-label-source=\"custom\"]",
        ".ui-sidenav[data-trigger-source=\"custom\"]",
        ".ui-sidenav[data-shortcut-source=\"custom\"]",
        ".ui-sidenav[data-class-source=\"custom\"]",
        ".ui-sidenav[data-handler-source=\"custom\"]",
        ".ui-sidenav--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "Sidenav styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn sidenav_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::sidenav::styles::CSS);"),
        "ui-components css aggregator should include sidenav styles."
    );
}

#[test]
fn sidenav_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidenav.rs");

    for needle in [
        "pub(super) fn sidenav() -> AnyView",
        "title=\"Sidenav\"",
        "slug=\"sidenav\"",
        "State + Source Markers",
        "data-handler-source",
        "<Sidenav",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_sidenav docs page should contain `{needle}`."
        );
    }
}

#[test]
fn sidenav_docs_controlled_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidenav.rs");

    for needle in [
        "title=\"Controlled + Floating\"",
        "open=Signal::derive(move || open.get())",
        "on_open_change=on_open_change",
        "side=SidebarSide::Right",
        "variant=SidebarVariant::Floating",
        "trigger_label=\"Toggle nav\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Sidenav docs controlled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn sidenav_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_sidenav.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "open=marker_open_signal",
        "on_open_change=marker_on_open_change",
        "default_open=false",
        "show_trigger=false",
        "enable_shortcut=true",
        "shortcut_key=\"n\".to_string()",
        "trigger_label=\"Toggle markers nav\".to_string()",
        "aria_label=\"Markers navigation\".to_string()",
        "class_name=\"docs-sidenav-state\".to_string()",
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::Offcanvas",
        "Inspect wrapper markers like `data-state`, `data-open-mode`, `data-initial-open`, `data-trigger-mode`, `data-shortcut-mode`, `data-label-source`, `data-trigger-source`, `data-shortcut-source`, `data-class-source`, and `data-handler-source`.",
    ] {
        assert!(
            source.contains(needle),
            "Sidenav docs state/source playground should contain `{needle}`.",
        );
    }
}
