use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_does_not_expose_logic_module() {
    let source = load_source("src/button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Button's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Button should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "data-slot=\"button\"",
        "data-hovered",
        "data-pressed",
        "data-loading",
        "data-loading-placement",
        "data-motion-source=if motion == ButtonMotion::default()",
        "data-custom-motion=(motion != ButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Button should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn button_forwards_headless_button_semantics() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Button should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn button_loading_forces_disabled_and_sets_aria_busy() {
    let source = load_source("src/button/view.rs");

    assert!(
        source.contains("resolve_state"),
        "Button should normalize `disabled`/`is_loading` via `resolve_state` to keep the contract testable and consistent."
    );

    for needle in [
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Button should wire loading/disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button/styles.rs");
    let motion = load_source("src/button/motion.rs");

    for needle in [
        "--ui-button-scale",
        "transform: scale(var(--ui-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "Button styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("--ui-button-scale"),
        "Button motion should write `--ui-button-scale` to drive interaction feedback without triggering rerenders."
    );
}

#[test]
fn button_spinner_respects_reduced_motion() {
    let styles = load_source("src/button/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "Button spinner should disable its CSS animation under reduced-motion via `{needle}`."
        );
    }
}

#[test]
fn button_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/styles.rs");

    for selector in [
        ".ui-button[data-motion-source=\"custom\"]",
        ".ui-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Button styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ButtonMotion) -> ButtonMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "Button motion should include `{needle}` so invalid custom motion values cannot leak into runtime animation behavior.",
        );
    }
}

#[test]
fn button_docs_page_covers_button_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "description=\"Variants + sizes with spring hover/tap motion.\"",
        "<Playground",
        "title=\"Variants & sizes\"",
        "<Button",
        "variant=variant",
        "size=size",
        "disabled=disabled",
        "is_loading=is_loading",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for button playground coverage.",
        );
    }
}

#[test]
fn button_docs_variants_and_controls_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Button variant=ButtonVariant::Default>\"Primary\"</Button>",
        "<Button variant=ButtonVariant::Outline>\"Outline\"</Button>",
        "<Button variant=ButtonVariant::Ghost>\"Ghost\"</Button>",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "{format!(\"{variant:?} · {size:?}\")}",
    ] {
        assert!(
            source.contains(needle),
            "button docs variants/controls playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "Playground",
        "title=\"Variants & sizes\"",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should contain `{needle}` for Button.",
        );
    }
}

#[test]
fn button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Variants & sizes\"",
        "<Button variant=ButtonVariant::Default>\"Primary\"</Button>",
        "<Button variant=ButtonVariant::Outline>\"Outline\"</Button>",
        "<Button variant=ButtonVariant::Ghost>\"Ghost\"</Button>",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "variant=variant",
        "size=size",
        "disabled=disabled",
        "is_loading=is_loading",
    ] {
        assert!(
            source.contains(needle),
            "button docs playground should contain `{needle}`.",
        );
    }
}
