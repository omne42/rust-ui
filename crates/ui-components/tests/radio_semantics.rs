use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn radio_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/radio/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Radio internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn radio_group_uses_headless_roving_and_interaction_hooks() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "use_radio_group",
        "use_focus_ring",
        "use_hover",
        "use_press",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn radio_group_uses_logic_state_model() {
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");

    for needle in [
        "pub struct RadioGroupState",
        "pub fn resolve_state(",
        "pub item_count: usize",
        "pub has_disabled_options: bool",
        "pub selected_index: Option<usize>",
        "pub has_selection: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Radio logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "aria.selected_index.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "RadioGroup view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn radio_group_supports_accessible_name_resolution() {
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_labelledby: Option<String>",
        "resolve_accessible_name",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "RadioGroup should wire `{needle}` for baseline-style accessible naming."
        );
    }

    assert!(
        logic_source.contains("aria_label: Some(\"Radio group\".to_string())"),
        "RadioGroup logic should provide a fallback accessible label when no labels are supplied."
    );
}

#[test]
fn radio_group_exposes_state_and_orientation_data_attributes() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "data-slot=\"radio-group\"",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selection-empty=move || state.get().selection_empty.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-orientation=orientation.data_orientation()",
        "data-horizontal=move || state.get().is_horizontal.then_some(\"true\")",
        "data-vertical=move || state.get().is_vertical.then_some(\"true\")",
        "data-has-label=move || state.get().has_label.then_some(\"true\")",
        "data-slot=\"radio\"",
        "data-index=index",
        "data-active=move || (aria.active_index.get() == index).then_some(\"true\")",
        "data-checked",
        "data-focus-visible",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should expose `{needle}` for baseline-style state styling and inspection."
        );
    }
}

#[test]
fn radio_group_sets_aria_orientation_and_option_label_fallback() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "aria-orientation=orientation.aria_orientation()",
        "format!(\"Option {}\", index + 1)",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should keep `{needle}` for robust ARIA semantics and predictable labels."
        );
    }
}

#[test]
fn radio_attaches_motion_driver() {
    let source = load_source("src/radio/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Radio should attach motion via `radio::motion::attach_motion`."
    );
}

#[test]
fn radio_styles_include_motion_marker_contracts() {
    let source = load_source("src/radio/styles.rs");

    for selector in [
        ".ui-radio[data-motion-source=\"custom\"]",
        ".ui-radio[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Radio styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn radio_motion_uses_spring_animator() {
    let source = load_source("src/radio/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Radio motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn radio_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/radio/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: RadioMotion) -> RadioMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "Radio motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn radio_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn radio_group() -> AnyView",
        "title=\"RadioGroup\"",
        "slug=\"radio-group\"",
        "description=\"Roving tabindex radiogroup with baseline-level spring motion and baseline-style root state attrs.\"",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "<Playground title=\"Horizontal + Disabled + Empty\" code_signal=states_code>",
        "pub(super) fn radio() -> AnyView",
        "title=\"Radio\"",
        "slug=\"radio\"",
        "<Playground title=\"Standalone\" code_signal=code>",
        "<RadioGroup",
        "<Radio",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for radio primary playground coverage.",
        );
    }
}

#[test]
fn radio_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Selection + Root State\"",
        "id_base=\"docs-radio-group\".to_string()",
        "label=\"Size\".to_string()",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "title=\"Horizontal + Disabled + Empty\"",
        "id_base=\"docs-radio-group-billing\".to_string()",
        "orientation=RadioGroupOrientation::Horizontal",
        "disabled_indices=billing_disabled_indices",
        "aria_labelledby=external_label_id.clone()",
        "selected_index=billing_selected",
        "set_selected_index=set_billing_selected",
        "id_base=\"docs-radio-group-empty\".to_string()",
        "options=empty_options",
        "disabled=true",
        "aria_label=\"No options available\".to_string()",
        "selected_index=empty_selected",
        "set_selected_index=set_empty_selected",
        "title=\"Standalone\"",
        "id=\"docs-radio\".to_string()",
        "label=\"Standalone\".to_string()",
        "checked=checked.into()",
        "on_change=on_change",
    ] {
        assert!(
            source.contains(needle),
            "forms docs playgrounds should contain `{needle}` for radio contracts.",
        );
    }
}
