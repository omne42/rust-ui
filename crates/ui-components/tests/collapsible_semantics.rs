use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn collapsible_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/collapsible/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Collapsible internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn collapsible_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/collapsible/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Collapsible;"),
        "collapsible module should export `Collapsible`.",
    );
    assert!(
        module_source.contains("CollapsibleMotion"),
        "collapsible module should expose a motion contract alias.",
    );
    assert!(
        crate_source.contains("pub use collapsible::{Collapsible, CollapsibleMotion};"),
        "crate root should re-export `Collapsible` and `CollapsibleMotion`.",
    );
}

#[test]
fn collapsible_logic_exposes_state_helpers() {
    let source = load_source("src/collapsible/logic.rs");

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn resolve_title(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(input: CollapsibleStateInput)",
        "pub fn compose_class_name(class_name: Option<String>, state: CollapsibleState)",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible logic should include `{needle}` for centralized normalization/state contracts.",
        );
    }
}

#[test]
fn collapsible_view_uses_logic_state_and_motion_contracts() {
    let source = load_source("src/collapsible/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_title(title)",
        "logic::resolve_aria_label(&title, aria_label)",
        "logic::resolve_state(CollapsibleStateInput {",
        "logic::compose_class_name(normalized_class_name.get_value(), state.get())",
        "data-slot=\"collapsible\"",
        "data-state=move || state.get().state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Collapsible view should include `{needle}` for stable state/source/motion contracts.",
        );
    }
}

#[test]
fn collapsible_css_contains_state_mode_and_motion_markers() {
    let css = load_source("src/collapsible/styles.rs");

    for needle in [
        ".ui-collapsible {",
        ".ui-collapsible--state-open",
        ".ui-collapsible[data-state=\"disabled\"]",
        ".ui-collapsible[data-open-mode=\"controlled\"]",
        ".ui-collapsible[data-motion-source=\"custom\"]",
        ".ui-collapsible[data-custom-motion=\"true\"]",
        ".ui-collapsible--custom-class",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            css.contains(needle),
            "Collapsible CSS should include `{needle}` selector.",
        );
    }
}

#[test]
fn collapsible_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::collapsible::styles::CSS);"),
        "ui-components css aggregator should include collapsible styles.",
    );
}

#[test]
fn collapsible_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "pub(super) fn collapsible() -> AnyView",
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "State + Source Markers",
        "data-open-mode",
    ] {
        assert!(
            source.contains(needle),
            "collections_groups docs page should contain `{needle}` for Collapsible.",
        );
    }
}

#[test]
fn collapsible_docs_disabled_custom_motion_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "title=\"Disabled + Custom Motion\"",
        "id_base=\"docs-collapsible-disabled\".to_string()",
        "disabled=true",
        "class_name=\"docs-collapsible-custom\".to_string()",
        "let custom_motion = CollapsibleMotion {",
        "panel_offset_y_px: 6.0",
        "motion=custom_motion",
    ] {
        assert!(
            source.contains(needle),
            "collapsible disabled/custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn collapsible_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-collapsible-markers\".to_string()",
        "aria_label=\"Advanced settings panel\".to_string()",
        "class_name=\"docs-collapsible-state\".to_string()",
        "let marker_motion = CollapsibleMotion {",
        "panel_offset_y_px: 8.0",
        "motion=marker_motion",
        "Open mode, label source, class source, and motion source are explicit.",
    ] {
        assert!(
            source.contains(needle),
            "collapsible state/source playground should contain `{needle}`."
        );
    }
}

#[test]
fn collapsible_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "pub(super) fn collapsible() -> AnyView",
        "title=\"Collapsible\"",
        "slug=\"collapsible\"",
        "title=\"Controlled Collapsible\"",
        "title=\"Disabled + Custom Motion\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            source.contains(needle),
            "collapsible docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn collapsible_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "<Playground title=\"Controlled Collapsible\" code_signal=basic_code>",
        "id_base=\"docs-collapsible\".to_string()",
        "open=open.into()",
        "on_open_change=on_open_change",
        "<Playground title=\"Disabled + Custom Motion\" code_signal=states_code>",
        "id_base=\"docs-collapsible-disabled\".to_string()",
        "disabled=true",
        "class_name=\"docs-collapsible-custom\".to_string()",
        "motion=custom_motion",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-collapsible-markers\".to_string()",
        "aria_label=\"Advanced settings panel\".to_string()",
        "class_name=\"docs-collapsible-state\".to_string()",
        "motion=marker_motion",
    ] {
        assert!(
            source.contains(needle),
            "collapsible docs playground should contain `{needle}`.",
        );
    }
}
