use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn contextual_help_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/contextual_help/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ContextualHelp internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn contextual_help_uses_logic_state_model() {
    let view_source = load_source("src/contextual_help/view.rs");
    let logic_source = load_source("src/contextual_help/logic.rs");

    for needle in [
        "pub struct ContextualHelpStateInput",
        "pub struct ContextualHelpState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_trigger_aria_label(",
        "pub fn resolve_id(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ContextualHelp logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(heading)",
        "logic::resolve_trigger_aria_label(variant, aria_label)",
        "logic::resolve_id(id, format!(\"ui-contextual-help-{}\", next_id()))",
        "logic::resolve_state(ContextualHelpStateInput {",
        "logic::compose_class_name(class_name, state)",
        "motion: ContextualHelpMotion",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "ContextualHelp view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn contextual_help_uses_controllable_open_and_presence() {
    let source = load_source("src/contextual_help/view.rs");

    for needle in [
        "overlay_open::use_controllable_open_state",
        "use_presence(open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ContextualHelp should keep open/presence contracts (`{needle}`)."
        );
    }
}

#[test]
fn contextual_help_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/contextual_help/view.rs");

    for attr in [
        "data-slot=\"contextual-help\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-placement=state.placement_attr",
        "data-heading=state.heading_attr",
        "data-footer=state.footer_attr",
        "data-open-mode=state.open_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-id-source=state.id_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=if motion == ContextualHelpMotion::default()",
        "data-custom-motion=(motion != ContextualHelpMotion::default()).then_some(\"true\")",
        "data-slot=\"contextual-help-panel\"",
        "data-slot=\"contextual-help-content\"",
    ] {
        assert!(
            source.contains(attr),
            "ContextualHelp should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn contextual_help_panel_preserves_non_modal_dialog_semantics() {
    let source = load_source("src/contextual_help/view.rs");

    for needle in [
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "aria-label=panel_aria_label.get_value()",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "is_modal=false",
    ] {
        assert!(
            source.contains(needle),
            "ContextualHelp panel should preserve dialog semantics (`{needle}`)."
        );
    }
}

#[test]
fn contextual_help_styles_include_state_marker_contracts() {
    let source = load_source("src/contextual_help/styles.rs");

    for selector in [
        ".ui-contextual-help--variant-info",
        ".ui-contextual-help[data-variant=\"help\"]",
        ".ui-contextual-help--placement-top-end",
        ".ui-contextual-help[data-state=\"disabled\"]",
        ".ui-contextual-help[data-heading=\"absent\"]",
        ".ui-contextual-help[data-footer=\"present\"]",
        ".ui-contextual-help--custom-class",
        ".ui-contextual-help[data-motion-source=\"custom\"]",
        ".ui-contextual-help[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ContextualHelp styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn contextual_help_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/contextual_help/mod.rs");
    let motion_source = load_source("src/contextual_help/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ContextualHelpMotion;",
        "pub struct ContextualHelpMotion",
        "pub popover: crate::popover::PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ContextualHelp motion contract should include `{needle}` for HeroUI-style spring customization."
        );
    }
}

#[test]
fn contextual_help_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/contextual_help/motion.rs");
    let view_source = load_source("src/contextual_help/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ContextualHelpMotion) -> ContextualHelpMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ContextualHelp motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source
            .contains("let motion = crate::contextual_help::motion::sanitize_motion(motion);"),
        "ContextualHelp view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn contextual_help_docs_page_covers_primary_playgrounds() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn contextual_help() -> AnyView",
        "title=\"ContextualHelp\"",
        "slug=\"contextual-help\"",
        "description=\"Non-modal popover help trigger with centralized variant/placement/heading/footer state attrs.\"",
        "<Playground title=\"Help Variant + Slots\" code=semantic_code>",
        "<Playground title=\"Info Variant + Controlled\" code=controlled_code>",
        "<ContextualHelp",
        "ContextualHelpVariant::Info",
        "open=controlled_open",
        "on_open_change=on_controlled_open_change",
    ] {
        assert!(
            docs.contains(needle),
            "overlays docs page should include `{needle}` for contextual_help primary coverage.",
        );
    }
}

#[test]
fn contextual_help_docs_playgrounds_lock_state_matrix_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "heading=\"Contextual help\".to_string()",
        "footer=move || view! { \"Popover-based\" }",
        "\"Uses Button + Popover + spring motion.\"",
        "\"Works in Light/Dark/OLED via tokens.\"",
        "let (controlled_open_raw, set_controlled_open_raw) = signal(false);",
        "let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());",
        "variant=ContextualHelpVariant::Info",
        "open=controlled_open",
        "on_open_change=on_controlled_open_change",
        "aria_label=\"More info\".to_string()",
        "class_name=\"docs-contextual-help-custom\".to_string()",
        "\"Toggle controlled help\"",
        "\"open: \"",
        "\"Controlled mode keeps parent state as the source of truth.\"",
    ] {
        assert!(
            docs.contains(needle),
            "contextual_help docs playgrounds should contain `{needle}`.",
        );
    }
}
