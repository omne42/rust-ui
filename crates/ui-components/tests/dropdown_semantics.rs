use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropdown_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/dropdown/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Dropdown internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn dropdown_uses_logic_state_model() {
    let view_source = load_source("src/dropdown/view.rs");
    let logic_source = load_source("src/dropdown/logic.rs");

    for needle in [
        "pub enum DropdownOpenFocusStrategy",
        "pub fn focus_strategy_for_open_key(",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_disabled(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Dropdown logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count)",
        "overlay_open::use_controllable_open_state_traced(",
        "logic::resolve_state(crate::dropdown::DropdownStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Dropdown view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn dropdown_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/dropdown/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "motion: DropdownMotion",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn dropdown_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/dropdown/view.rs");

    for attr in [
        "data-slot=\"dropdown\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "data-motion-source=if motion == DropdownMotion::default()",
        "data-custom-motion=(motion != DropdownMotion::default()).then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Dropdown should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn dropdown_composes_button_popover_and_menu() {
    let source = load_source("src/dropdown/view.rs");

    for needle in [
        "<Button",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "<Popover",
        "motion=motion.popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown should compose overlay/menu flow through `{needle}`."
        );
    }
}

#[test]
fn dropdown_styles_include_persistent_and_disabled_markers() {
    let source = load_source("src/dropdown/styles.rs");

    for selector in [
        ".ui-dropdown--disabled",
        ".ui-dropdown[data-disabled=\"true\"]",
        ".ui-dropdown--persistent",
        ".ui-dropdown[data-keep-open-on-action=\"true\"]",
        ".ui-dropdown--custom-class",
        ".ui-dropdown[data-custom-class=\"true\"]",
        ".ui-dropdown[data-motion-source=\"custom\"]",
        ".ui-dropdown[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dropdown styles should include `{selector}` as stable visual-state contracts."
        );
    }
}

#[test]
fn dropdown_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/dropdown/mod.rs");
    let motion_source = load_source("src/dropdown/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::DropdownMotion;",
        "pub struct DropdownMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Dropdown motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn dropdown_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/dropdown/motion.rs");
    let view_source = load_source("src/dropdown/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DropdownMotion) -> DropdownMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Dropdown motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::dropdown::motion::sanitize_motion(motion);"),
        "Dropdown view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn dropdown_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn dropdown() -> AnyView",
        "title=\"Dropdown\"",
        "slug=\"dropdown\"",
        "description=\"Spectrum/HeroUI-style dropdown trigger primitive with centralized state/source contracts, controllable open state, and spring-tuned popover motion.\"",
        "<Playground title=\"Default\" code_signal=code>",
        "<Playground title=\"Controlled + Persistent + Motion\" code_signal=states_code>",
        "<Dropdown",
        "open=open_signal",
        "close_on_action=false",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for dropdown coverage.",
        );
    }
}

#[test]
fn dropdown_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "id_base=\"docs-dropdown-default\".to_string()",
        "items=items",
        "on_action=on_action",
        "\"Open actions\"",
        "\"last action: \"",
        "let (open_raw, set_open_raw) = signal(false);",
        "let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());",
        "let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));",
        "id_base=\"docs-dropdown-controlled\".to_string()",
        "items=controlled_items",
        "disabled_indices=vec![1]",
        "item_kinds=vec![",
        "MenuItemKind::Action",
        "class_name=\"docs-dropdown-custom\".to_string()",
        "initial_scale: 0.94",
        "offset_y_px: 12.0",
        "\"Controlled dropdown\"",
        "\"open: \"",
    ] {
        assert!(
            source.contains(needle),
            "dropdown docs playgrounds should contain `{needle}`.",
        );
    }
}
