use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn disclosure_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/disclosure/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Disclosure internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn disclosure_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/disclosure/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Disclosure should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn disclosure_uses_headless_hooks() {
    let source = load_source("src/disclosure/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Disclosure should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn disclosure_attaches_motion_drivers() {
    let source = load_source("src/disclosure/view.rs");

    for needle in ["attach_indicator_motion", "attach_panel_motion"] {
        assert!(
            source.contains(needle),
            "Disclosure should attach `{needle}` for HeroUI-style spring motion."
        );
    }
}

#[test]
fn disclosure_emits_spectrum_style_data_attributes() {
    let source = load_source("src/disclosure/view.rs");

    for attr in [
        "data-slot=\"disclosure\"",
        "data-slot=\"disclosure-trigger\"",
        "data-slot=\"disclosure-label\"",
        "data-slot=\"disclosure-indicator\"",
        "data-slot=\"disclosure-panel\"",
        "data-slot=\"disclosure-panel-surface\"",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-open=move || if open.get() { Some(\"true\") } else { None }",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-hovered",
        "data-pressed",
        "data-disabled=disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Disclosure should set `{attr}` to support Spectrum-style styling and regression testing."
        );
    }
}

#[test]
fn disclosure_uses_logic_state_model() {
    let view_source = load_source("src/disclosure/view.rs");
    let logic_source = load_source("src/disclosure/logic.rs");

    for needle in [
        "pub struct DisclosureState",
        "pub fn resolve_state(",
        "pub is_open: bool",
        "pub is_closed: bool",
        "pub is_disabled: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Disclosure logic should include `{needle}` for centralized state derivation."
        );
    }

    assert!(
        view_source.contains("logic::resolve_state(open.get(), disabled)"),
        "Disclosure view should derive root state through resolve_state."
    );
}

#[test]
fn disclosure_ids_and_aria_contract_are_wired() {
    let view_source = load_source("src/disclosure/view.rs");
    let logic_source = load_source("src/disclosure/logic.rs");

    for needle in [
        "pub struct DisclosureIds",
        "trigger_id: format!(\"{id_base}-trigger\")",
        "panel_id: format!(\"{id_base}-panel\")",
    ] {
        assert!(
            logic_source.contains(needle),
            "Disclosure logic should define `{needle}` for stable id generation."
        );
    }

    for needle in [
        "aria-expanded",
        "aria-controls",
        "role=\"region\"",
        "aria-labelledby=trigger_id",
    ] {
        assert!(
            view_source.contains(needle),
            "Disclosure should wire `{needle}` for accessible disclosure semantics."
        );
    }
}

#[test]
fn disclosure_styles_include_motion_marker_contracts() {
    let source = load_source("src/disclosure/styles.rs");

    for selector in [
        ".ui-disclosure[data-motion-source=\"custom\"]",
        ".ui-disclosure[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Disclosure styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn disclosure_styles_define_motion_css_vars() {
    let source = load_source("src/disclosure/styles.rs");

    for var in [
        "--ui-disclosure-indicator-rotation",
        "--ui-disclosure-panel-height",
        "--ui-disclosure-panel-opacity",
        "--ui-disclosure-panel-y",
    ] {
        assert!(
            source.contains(var),
            "Disclosure styles should define `{var}` so motion can update without re-rendering."
        );
    }
}

#[test]
fn disclosure_motion_is_spring_driven() {
    let source = load_source("src/disclosure/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Disclosure motion should use SpringAnimator to match the motion spec."
    );
}

#[test]
fn disclosure_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/disclosure/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DisclosureMotion) -> DisclosureMotion",
        "fn sanitize_spring(value: SpringConfig) -> SpringConfig",
        "closed_rotation_deg:",
        "open_rotation_deg:",
        "panel_offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "Disclosure motion should include `{needle}` so invalid custom values cannot leak into runtime animation state.",
        );
    }
}

#[test]
fn disclosure_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn disclosure() -> AnyView",
        "title=\"Disclosure\"",
        "slug=\"disclosure\"",
        "description=\"Single disclosure panel with HeroUI-level spring motion and Spectrum-style root state attrs.\"",
        "<Playground title=\"Controlled\" code_signal=code>",
        "<Playground title=\"Disabled\" code_signal=states_code>",
        "<Disclosure",
        "on_open_change=on_open_change",
        "default_open=false",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for disclosure coverage.",
        );
    }
}

#[test]
fn disclosure_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let (open, set_open) = signal(true);",
        "let on_open_change = Callback::new(move |next: bool| set_open.set(next));",
        "id_base=\"docs-disclosure\".to_string()",
        "label=\"Details\".to_string()",
        "\"Hidden content\"",
        "\"Uses the same open-state contract as overlays.\"",
        "\"open: \"",
        "id_base=\"docs-disclosure-disabled\".to_string()",
        "label=\"Disabled details\".to_string()",
        "\"Disabled content\"",
        "\"Disabled disclosure keeps trigger non-interactive.\"",
        "\"disabled: true\"",
    ] {
        assert!(
            source.contains(needle),
            "disclosure docs playgrounds should contain `{needle}`.",
        );
    }
}
