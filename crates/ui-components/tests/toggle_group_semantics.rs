use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn toggle_group_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/toggle_group/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ToggleGroup;"),
        "toggle_group module should export `ToggleGroup`."
    );
    assert!(
        module_source.contains("ToggleGroupOrientation")
            && module_source.contains("ToggleGroupSelectionMode"),
        "toggle_group module should export orientation and selection mode contracts."
    );
    assert!(
        crate_source.contains("pub use toggle_group::{")
            && crate_source.contains("ToggleGroup")
            && crate_source.contains("ToggleGroupItem")
            && crate_source.contains("ToggleGroupOrientation")
            && crate_source.contains("ToggleGroupSelectionMode"),
        "crate root should re-export toggle group contracts."
    );
}

#[test]
fn toggle_group_uses_logic_state_model() {
    let view_source = load_source("src/toggle_group/view.rs");
    let logic_source = load_source("src/toggle_group/logic.rs");

    for needle in [
        "pub enum ToggleGroupOrientation",
        "pub enum ToggleGroupSelectionMode",
        "pub fn normalize_items(",
        "pub fn sanitize_selected_ids(",
        "pub fn toggle_selected_id(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ToggleGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let selected_state = overlay_open::use_controllable_state(",
        "let selected_ids = selected_state.value;",
        "let request_selected_ids_change = selected_state.request_change;",
        "let resolved_selected_ids = Signal::derive(move ||",
        "logic::resolve_state(ToggleGroupStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::toggle_selected_id(",
    ] {
        assert!(
            view_source.contains(needle),
            "ToggleGroup view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn toggle_group_supports_controlled_and_uncontrolled_selection_contracts() {
    let source = load_source("src/toggle_group/view.rs");

    for needle in [
        "selected_ids: Option<Signal<BTreeSet<String>>>",
        "default_selected_ids: Option<BTreeSet<String>>",
        "on_selected_ids_change: Option<Callback<BTreeSet<String>>>",
        "selection_mode: ToggleGroupSelectionMode",
        "orientation: ToggleGroupOrientation",
        "on_change=on_item_change",
    ] {
        assert!(
            source.contains(needle),
            "ToggleGroup should include `{needle}` for controlled/uncontrolled selection behavior."
        );
    }
}

#[test]
fn toggle_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/toggle_group/view.rs");

    for needle in [
        "data-slot=\"toggle-group\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-selection-mode=move || state.get().selection_mode_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-attached=move || state.get().is_attached.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-item-count=move || state.get().item_count.to_string()",
        "data-selected-count=move || state.get().selected_count.to_string()",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-disabled-item-count=move || state.get().disabled_item_count.to_string()",
        "data-slot=\"toggle-group-items\"",
    ] {
        assert!(
            source.contains(needle),
            "ToggleGroup should set `{needle}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn toggle_group_styles_define_orientation_and_attached_layout_rules() {
    let source = load_source("src/toggle_group/styles.rs");

    for needle in [
        ".ui-toggle-group {",
        ".ui-toggle-group__items {",
        ".ui-toggle-group--horizontal .ui-toggle-group__items",
        ".ui-toggle-group--vertical .ui-toggle-group__items",
        ".ui-toggle-group--attached .ui-toggle-group__items",
        ".ui-toggle-group--attached.ui-toggle-group--horizontal",
        ".ui-toggle-group--attached.ui-toggle-group--vertical",
    ] {
        assert!(
            source.contains(needle),
            "ToggleGroup styles should include `{needle}` for stable attached/orientation behavior."
        );
    }
}

#[test]
fn toggle_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn toggle_group() -> AnyView",
        "title=\"ToggleGroup\"",
        "slug=\"toggle-group\"",
        "description=\"Shadcn-compatible grouped toggle primitive with controlled selection modes and Spectrum-style root state contracts.\"",
        "<Playground title=\"Multiple + Attached\" code_signal=code>",
        "<Playground title=\"Single + Vertical + Disabled Item\" code_signal=states_code>",
        "<ToggleGroup",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra toggle_group docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn toggle_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Multiple + Attached\"",
        "id_base=\"docs-toggle-group-formatting\".to_string()",
        "selection_mode=ToggleGroupSelectionMode::Multiple",
        "attached=true",
        "title=\"Single + Vertical + Disabled Item\"",
        "id_base=\"docs-toggle-group-alignment\".to_string()",
        "selection_mode=ToggleGroupSelectionMode::Single",
        "orientation=ToggleGroupOrientation::Vertical",
        "attached=false",
        "aria_label=\"Alignment controls\".to_string()",
        "class_name=\"docs-toggle-group-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions_extra toggle_group docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
