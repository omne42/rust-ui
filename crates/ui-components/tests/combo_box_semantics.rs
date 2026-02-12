use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn combo_box_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/combo_box/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ComboBox internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn combo_box_uses_logic_state_model() {
    let view_source = load_source("src/combo_box/view.rs");
    let logic_source = load_source("src/combo_box/logic.rs");

    for needle in [
        "pub struct ComboBoxStateInput",
        "pub struct ComboBoxState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let state = logic::resolve_state(logic::ComboBoxStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn combo_box_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/combo_box/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn combo_box_normalizes_label_placeholder_and_id_base() {
    let view_source = load_source("src/combo_box/view.rs");
    let logic_source = load_source("src/combo_box/logic.rs");

    for needle in [
        "logic::normalize_label",
        "logic::resolve_placeholder",
        "logic::normalize_id_base",
    ] {
        assert!(
            view_source.contains(needle),
            "ComboBox view should use `{needle}` to keep text and id semantics stable."
        );
    }

    for needle in ["\"Options\".to_string()", "\"combo-box\".to_string()"] {
        assert!(
            logic_source.contains(needle),
            "ComboBox logic should provide fallback semantics via `{needle}`."
        );
    }
}

#[test]
fn combo_box_escape_stops_propagation_when_open() {
    let source = load_source("src/combo_box/view.rs");

    for needle in ["stop_propagation()", "key == \"Escape\"", "was_open"] {
        assert!(
            source.contains(needle),
            "ComboBox should handle Escape bubbling with `{needle}` to avoid closing parent overlays unexpectedly."
        );
    }
}

#[test]
fn combo_box_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("src/combo_box/view.rs");

    for needle in [
        "<Portal>",
        "use_popover_position",
        "data-ui-overlay-portal",
        "--ui-popover-top",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox panel should include `{needle}` for Spectrum-style popover behavior."
        );
    }
}

#[test]
fn combo_box_panel_exposes_option_and_empty_state_slots() {
    let source = load_source("src/combo_box/view.rs");

    for needle in [
        "data-slot=\"combo-box-listbox\"",
        "data-empty=move || filtered_indices.get().is_empty().then_some(\"true\")",
        "data-slot=\"combo-box-option\"",
        "data-focused=move || (active_index.get() == filtered_index).then_some(\"true\")",
        "data-slot=\"combo-box-empty\"",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox panel should expose `{needle}` for deterministic style/test hooks."
        );
    }
}

#[test]
fn combo_box_uses_presence_for_motion_safe_unmounting() {
    let source = load_source("src/combo_box/view.rs");

    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox should use `{needle}` so popover exit motion can finish before unmount."
        );
    }
}

#[test]
fn combo_box_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/combo_box/view.rs");

    for attr in [
        "data-slot=\"combo-box\"",
        "data-state=move ||",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=move || (filtered_count.get() == 0).then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-filtered-items=move || (filtered_count.get() > 0).then_some(\"true\")",
        "data-selection-empty=move || selected_index.get().is_none().then_some(\"true\")",
        "data-has-selection=move || selected_index.get().is_some().then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-valid=move || (!invalid.get()).then_some(\"true\")",
        "data-required=move || required.get().then_some(\"true\")",
        "data-optional=move || (!required.get()).then_some(\"true\")",
        "data-has-description=state.has_description.then_some(\"true\")",
        "data-has-error=state.has_error.then_some(\"true\")",
        "data-has-disabled-options=state.has_disabled_options.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-motion-source=if motion == ComboBoxMotion::default()",
        "data-custom-motion=(motion != ComboBoxMotion::default()).then_some(\"true\")",
        "data-typed=move || has_typed.get().then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-filtered-count=move || filtered_count.get().to_string()",
        "data-disabled-option-count=state.disabled_option_count.to_string()",
        "data-slot=\"combo-box-trigger\"",
    ] {
        assert!(
            source.contains(attr),
            "ComboBox should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn combo_box_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("src/combo_box/styles.rs");

    for needle in [
        "position: fixed;",
        "var(--ui-popover-top",
        "data-placement=\"bottom-start\"",
        ".ui-combo-box__empty",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox styles should include `{needle}` for popover layout and empty-state rendering."
        );
    }
}

#[test]
fn combo_box_styles_include_controlled_and_disabled_option_markers() {
    let source = load_source("src/combo_box/styles.rs");

    for needle in [
        ".ui-combo-box--controlled",
        ".ui-combo-box--has-disabled-options",
        ".ui-combo-box--empty",
        ".ui-combo-box[data-motion-source=\"custom\"]",
        ".ui-combo-box[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "ComboBox styles should include `{needle}` for stable state-marker contracts."
        );
    }
}

#[test]
fn combo_box_motion_contract_exposes_popover_and_highlight_customization() {
    let mod_source = load_source("src/combo_box/mod.rs");
    let motion_source = load_source("src/combo_box/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ComboBoxMotion;",
        "pub struct ComboBoxMotion",
        "pub popover: PopoverMotion",
        "pub highlight: ActiveHighlightMotion",
        "fn default_motion_uses_default_popover_and_highlight_motion()",
        "fn supports_custom_popover_and_highlight_motion_contracts()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ComboBox motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn combo_box_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/combo_box/motion.rs");
    let view_source = load_source("src/combo_box/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ComboBoxMotion) -> ComboBoxMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "highlight: sanitize_highlight(motion.highlight)",
        "fn sanitize_motion_falls_back_for_invalid_nested_values()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ComboBox motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::combo_box::motion::sanitize_motion(motion);"),
        "ComboBox view should sanitize motion before attaching popover and active-highlight motion.",
    );
}

#[test]
fn combo_box_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn combo_box() -> AnyView",
        "title=\"ComboBox\"",
        "slug=\"combo-box\"",
        "description=\"Combobox with input + listbox + popover, Spectrum-style root attrs, and HeroUI-level panel/highlight motion.\"",
        "<Playground title=\"Selection + Validation\" code=code>",
        "<Playground title=\"Controlled Open State\" code=controlled_code>",
        "<Playground title=\"Disabled + Empty\" code=states_code>",
        "<ComboBox",
        "open=controlled_open",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for combo-box coverage.",
        );
    }
}

#[test]
fn combo_box_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-combo-box\".to_string()",
        "label=\"Language\".to_string()",
        "disabled_indices=vec![4]",
        "description=\"Pick one runtime language\".to_string()",
        "error=\"Language is required\".to_string()",
        "on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))",
        "\"selected: \"",
        "id_base=\"docs-combo-box-controlled\".to_string()",
        "on_open_change=on_open_change",
        "description=\"Open state is externally controlled\".to_string()",
        "\"open: \"",
        "id_base=\"docs-combo-box-disabled\".to_string()",
        "id_base=\"docs-combo-box-empty\".to_string()",
        "placeholder=\"No options\".to_string()",
        "\"disabled selected: \"",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "combo-box docs playgrounds should contain `{needle}`.",
        );
    }
}
