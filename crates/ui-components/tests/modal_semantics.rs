use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn modal_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/modal/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Modal internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn modal_is_exported_from_module_and_exposes_state_contracts() {
    let module_source = load_source("src/modal/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use view::Modal;",
        "pub enum ModalSlot",
        "pub struct ModalPartStateInput",
        "pub struct ModalPartState",
    ] {
        assert!(
            module_source.contains(needle),
            "modal module should include `{needle}` contracts."
        );
    }

    assert!(
        crate_source.contains("pub use modal::Modal;"),
        "crate root should re-export `Modal` contract."
    );
}

#[test]
fn modal_logic_exposes_state_helpers() {
    let source = load_source("src/modal/logic.rs");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-modal\";",
        "pub const DEFAULT_TITLE: &str = \"Modal\";",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: ModalPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ModalPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Modal logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn modal_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/modal/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(ModalPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-description=root_state.description_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=title_state.slot_attr",
        "data-title-source=title_state.title_source_attr",
        "data-slot=body_state.slot_attr",
        "motion=motion",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Modal view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn modal_only_sets_describedby_when_description_exists() {
    let source = load_source("src/modal/view.rs");

    assert!(
        source.contains("if let Some(description) = description"),
        "Modal should branch on description presence rather than emitting empty aria-describedby."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Modal should wire description path contracts (`{needle}`)."
        );
    }
}

#[test]
fn modal_styles_include_state_and_source_markers() {
    let source = load_source("src/modal/styles.rs");

    for selector in [
        ".ui-modal[data-motion-source=\"custom\"]",
        ".ui-modal[data-custom-motion=\"true\"]",
        ".ui-modal--custom-motion",
        ".ui-modal--custom-id",
        ".ui-modal[data-id-source=\"custom\"]",
        ".ui-modal[data-custom-id=\"true\"]",
        ".ui-modal--custom-title",
        ".ui-modal[data-title-source=\"custom\"]",
        ".ui-modal[data-custom-title=\"true\"]",
        ".ui-modal--custom-description",
        ".ui-modal[data-description-source=\"custom\"]",
        ".ui-modal[data-custom-description=\"true\"]",
        ".ui-modal[data-class-source=\"custom\"]",
        ".ui-modal[data-exit-source=\"custom\"]",
        ".ui-modal[data-custom-exit=\"true\"]",
        ".ui-modal--with-description",
        ".ui-modal[data-state=\"with-description\"]",
        ".ui-modal--title-only",
        ".ui-modal[data-description=\"present\"]",
        ".ui-modal__title[data-slot=\"modal-title\"]",
        ".ui-modal__description[data-slot=\"modal-description\"]",
        ".ui-modal__body[data-slot=\"modal-body\"]",
    ] {
        assert!(
            source.contains(selector),
            "Modal styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn modal_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::modal::styles::CSS);"),
        "ui-components css aggregator should include modal styles."
    );
}

#[test]
fn modal_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "State + Source Markers",
        "data-id-source",
        "<Modal",
    ] {
        assert!(
            source.contains(needle),
            "modal docs page should contain `{needle}`."
        );
    }
}

#[test]
fn modal_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = OverlayMotion {",
        "initial_scale: 0.92",
        "initial_y_px: 18.0",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "motion=custom_motion",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-id-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "modal docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn modal_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "description=\"Overlay composition with centralized title/description/class state attrs and stable modal slots.\"",
        "<Playground title=\"Label + Description\" code=semantic_code>",
        "title=\"State + Source Markers\"",
        "code=custom_code",
        "<Modal",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for modal primary playground coverage.",
        );
    }
}

#[test]
fn modal_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Label + Description\"",
        "id_base=\"docs-modal-semantic\".to_string()",
        "title=\"Confirm\".to_string()",
        "description=\"Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.\".to_string()",
        "on_exit_complete=on_semantic_exit_complete",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "title=\"Title only\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "let custom_motion = OverlayMotion {",
        "initial_scale: 0.92",
        "initial_y_px: 18.0",
        "motion=custom_motion",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-id-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "modal docs playgrounds should contain `{needle}`.",
        );
    }
}
