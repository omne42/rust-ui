use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn step_list_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/step_list/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "StepList internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn step_list_uses_logic_state_model() {
    let logic_source = load_source("src/step_list/logic.rs");
    let view_source = load_source("src/step_list/view.rs");

    for needle in [
        "pub enum StepListOrientation",
        "pub enum StepListSize",
        "pub struct StepListItem",
        "pub fn normalize_items(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_selected_index(",
        "pub fn normalize_completed_indices(",
        "pub fn resolve_state(",
        "pub fn resolve_item_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "StepList logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_items(steps.get())",
        "overlay_open::use_controllable_state(",
        "logic::resolve_state(StepListStateInput {",
        "logic::resolve_item_state(StepListItemStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "StepList view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn step_list_exposes_spectrum_style_data_markers() {
    let source = load_source("src/step_list/view.rs");

    for attr in [
        "data-slot=\"step-list\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-size=move || state.get().size_attr",
        "data-state=move || state.get().data_state_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-completed-count=move || state.get().completed_count.to_string()",
        "data-disabled-count=move || state.get().disabled_count.to_string()",
        "data-slot=\"step-list-item\"",
        "data-status=item_state.status_attr",
        "data-slot=\"step-list-button\"",
        "data-slot=\"step-list-marker\"",
        "data-slot=\"step-list-label\"",
        "data-slot=\"step-list-connector\"",
    ] {
        assert!(
            source.contains(attr),
            "StepList should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn step_list_styles_include_orientation_status_and_size_contracts() {
    let source = load_source("src/step_list/styles.rs");

    for selector in [
        ".ui-step-list",
        ".ui-step-list--orientation-horizontal",
        ".ui-step-list[data-orientation=\"vertical\"]",
        ".ui-step-list__item",
        ".ui-step-list__item--current",
        ".ui-step-list__item[data-status=\"completed\"]",
        ".ui-step-list__item--disabled",
        ".ui-step-list--size-s",
        ".ui-step-list[data-size=\"xl\"]",
        ".ui-step-list--emphasized",
        ".ui-step-list--custom-class",
        ".ui-step-list[data-custom-class=\"true\"]",
        ".ui-step-list[data-empty=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "StepList styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn step_list_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn step_list() -> AnyView",
        "title=\"StepList\"",
        "slug=\"step-list\"",
        "description=\"Spectrum-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts.\"",
        "<Playground title=\"Controlled Selection\" code_signal=code>",
        "<Playground title=\"Vertical + Emphasized + Disabled\" code_signal=states_code>",
        "<StepList",
        "on_selected_change=on_selected_change",
        "orientation=StepListOrientation::Vertical",
        "size=StepListSize::L",
        "emphasized=true",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for step-list coverage.",
        );
    }
}

#[test]
fn step_list_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "StepListItem::new(\"account\", \"Account\").described(\"Create account and verify email\")",
        "StepListItem::new(\"shipping\", \"Shipping\").described(\"Choose shipping address\")",
        "StepListItem::new(\"payment\", \"Payment\").described(\"Add payment method\")",
        "StepListItem::new(\"review\", \"Review\").described(\"Confirm and place order\")",
        "StepListItem::new(\"plan\", \"Plan\").described(\"Pick your subscription tier\")",
        "StepListItem::new(\"profile\", \"Profile\").described(\"Fill organization details\")",
        "StepListItem::new(\"billing\", \"Billing\")",
        ".disabled(true)",
        "StepListItem::new(\"launch\", \"Launch\").described(\"Start using the workspace\")",
        "let (selected_index, set_selected_index) = signal(Some(1_usize));",
        "completed_indices=vec![0]",
        "default_selected_index=3",
        "class_name=\"docs-step-list-custom\".to_string()",
        "aria_label=\"Workspace setup steps\".to_string()",
        "\"selected index: \"",
    ] {
        assert!(
            source.contains(needle),
            "step-list docs playgrounds should contain `{needle}`.",
        );
    }
}
