use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn select_does_not_expose_logic_module() {
    let source = load_source("src/select/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Select's `logic` module should stay private to avoid leaking internal behavior helpers into the public API."
    );
}

#[test]
fn select_uses_logic_state_model() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");

    for needle in [
        "pub struct SelectState",
        "pub fn resolve_state(",
        "pub item_count: usize",
        "pub has_selection: bool",
        "pub has_disabled_options: bool",
        "pub disabled_option_count: usize",
    ] {
        assert!(
            logic_source.contains(needle),
            "Select logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "selected_index.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }
}

#[test]
fn select_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "motion: SelectMotion",
    ] {
        assert!(
            source.contains(needle),
            "Select should accept `{needle}` to support controlled/uncontrolled open state."
        );
    }
}

#[test]
fn select_trigger_is_labeled_and_owns_a_listbox() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "aria_haspopup=\"listbox\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "Select should wire `{needle}` for Spectrum-style listbox trigger semantics."
        );
    }
}

#[test]
fn select_uses_presence_to_allow_exit_motion() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "use_presence(open)",
        "motion=motion.popover",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Select should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn select_exposes_root_state_and_slot_data_attributes() {
    let source = load_source("src/select/view.rs");

    for needle in [
        "data-slot=\"select\"",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-disabled=move || state.get().trigger_disabled.then_some(\"true\")",
        "data-component-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selection-empty=move || state.get().selection_empty.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-motion-source=if motion == SelectMotion::default()",
        "data-custom-motion=(motion != SelectMotion::default()).then_some(\"true\")",
        "data-slot=\"select-panel\"",
    ] {
        assert!(
            source.contains(needle),
            "Select should expose `{needle}` for Spectrum-style state styling and regression tests."
        );
    }
}

#[test]
fn select_centralizes_trigger_disabled_logic() {
    let view_source = load_source("src/select/view.rs");
    let logic_source = load_source("src/select/logic.rs");

    for needle in [
        "resolve_trigger_disabled",
        "disabled=trigger_disabled",
        "if trigger_disabled {",
    ] {
        assert!(
            view_source.contains(needle),
            "Select view should centralize trigger disabled semantics via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("pub fn resolve_trigger_disabled"),
        "Select logic should expose a dedicated helper for disabled/empty trigger semantics."
    );
}

#[test]
fn select_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/select/mod.rs");
    let motion_source = load_source("src/select/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::SelectMotion;",
        "pub struct SelectMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Select motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn select_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/select/motion.rs");
    let view_source = load_source("src/select/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SelectMotion) -> SelectMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Select motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::select::motion::sanitize_motion(motion);"),
        "Select view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn select_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn select() -> AnyView",
        "title=\"Select\"",
        "slug=\"select\"",
        "description=\"Select with controlled open state, listbox semantics, and Spectrum-style root state attrs.\"",
        "<Playground title=\"Controlled Open + Selection\" code=code>",
        "<Playground title=\"Disabled + Empty\" code=states_code>",
        "<Select",
        "open=controlled_open",
        "disabled=true",
        "placeholder=\"No options\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for select coverage.",
        );
    }
}

#[test]
fn select_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-select-controlled\".to_string()",
        "disabled_indices=disabled_indices",
        "set_controlled_open_raw.update(|value| *value = !*value);",
        "\"Toggle open\"",
        "\"open: \"",
        "\"selected: \"",
        "\" · has selection: \"",
        "\" · disabled options: \"",
        "id_base=\"docs-select-disabled\".to_string()",
        "items=disabled_items",
        "\"disabled selected: \"",
        "id_base=\"docs-select-empty\".to_string()",
        "items=empty_items",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "select docs playgrounds should contain `{needle}`.",
        );
    }
}
