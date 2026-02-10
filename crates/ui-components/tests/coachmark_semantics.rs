use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn coachmark_does_not_expose_view_module() {
    let source = load_source("src/coachmark/mod.rs");

    assert!(
        !source.contains("pub mod view"),
        "Coachmark internals should stay private; found `pub mod view`."
    );

    assert!(
        !source.contains("pub mod logic"),
        "Coachmark `logic` module should stay private to avoid leaking internal state helpers."
    );
}

#[test]
fn coachmark_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/coachmark/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Coachmark;"),
        "coachmark module should export `Coachmark`."
    );
    assert!(
        crate_source.contains("pub use coachmark::{"),
        "crate root should re-export `Coachmark` contracts."
    );
}

#[test]
fn coachmark_wraps_contextual_help_contract() {
    let source = load_source("src/coachmark/view.rs");

    for needle in [
        "pub fn Coachmark(",
        "<ContextualHelp",
        "logic::resolve_state(logic::CoachmarkStateInput {",
        "logic::compose_class_name(normalized_class_name, state)",
        "primary_cta: Option<String>",
        "asset_variant: Option<CoachmarkAssetVariant>",
        "footer=move || footer_view.get_value().run()",
        "data-slot=\"coachmark-content\"",
        "data-asset=state.asset_attr",
        "data-asset-source=state.asset_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark wrapper should preserve ContextualHelp contract marker `{needle}`."
        );
    }
}

#[test]
fn coachmark_logic_tracks_heading_steps_and_source_contracts() {
    let source = load_source("src/coachmark/logic.rs");

    for needle in [
        "pub const DEFAULT_TITLE: &str = \"Coachmark\";",
        "pub const DEFAULT_ASSET_LABEL: &str = \"Coachmark asset\";",
        "pub struct CoachmarkStateInput",
        "pub struct CoachmarkState",
        "pub fn compose_heading(",
        "pub fn compose_step_label(",
        "pub fn resolve_state(input: CoachmarkStateInput) -> CoachmarkState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: CoachmarkState)",
        "asset_source_attr",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Coachmark logic should include `{needle}` for centralized state/source normalization."
        );
    }
}

#[test]
fn coachmark_styles_include_variant_state_and_accessibility_markers() {
    let source = load_source("src/coachmark/styles.rs");

    for selector in [
        ".ui-coachmark--variant-help",
        ".ui-coachmark[data-variant=\"info\"]",
        ".ui-coachmark--state-disabled",
        ".ui-coachmark[data-state=\"enabled\"]",
        ".ui-coachmark[data-cta=\"none\"] .ui-coachmark__actions",
        ".ui-coachmark__actions-extra",
        ".ui-coachmark[data-motion-source=\"custom\"]",
        ".ui-coachmark[data-custom-motion=\"true\"]",
        "@media (forced-colors: active)",
    ] {
        assert!(
            source.contains(selector),
            "Coachmark styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn coachmark_docs_page_exists() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs");

    for needle in [
        "pub(super) fn coachmark() -> AnyView",
        "title=\"Coachmark\"",
        "slug=\"coachmark\"",
        "<Coachmark",
        "State + Source Markers",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs page should contain `{needle}` for Coachmark."
        );
    }
}
