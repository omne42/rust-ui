use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn flip_card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/flip_card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FlipCard internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn flip_card_module_exposes_slot_state_motion_contracts() {
    let source = load_source("src/flip_card/mod.rs");

    for needle in [
        "pub enum FlipCardSlot",
        "pub struct FlipCardPartStateInput",
        "pub struct FlipCardPartState",
        "pub use view::FlipCard;",
        "pub use motion::FlipCardMotion;",
        "DEFAULT_DISABLED",
        "DEFAULT_FLIPPED",
        "DEFAULT_HOVER_FLIP",
    ] {
        assert!(
            source.contains(needle),
            "flip_card module should include `{needle}` contracts."
        );
    }
}

#[test]
fn flip_card_is_exported_from_crate_root() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod flip_card;"),
        "crate root should expose `flip_card` module."
    );
    assert!(
        source.contains("pub use flip_card::{FlipCard, FlipCardMotion};"),
        "crate root should re-export FlipCard contracts."
    );
}

#[test]
fn flip_card_logic_exposes_state_and_source_helpers() {
    let source = load_source("src/flip_card/logic.rs");

    for needle in [
        "pub fn state_attr(is_flipped: bool)",
        "pub fn flip_mode_attr(flip_on_hover: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn resolve_part_state(input: FlipCardPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FlipCardPartState)",
        "pub fn should_toggle_key(key: &str, is_composing: bool)",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard logic should include `{needle}` for centralized contracts."
        );
    }
}

#[test]
fn flip_card_view_uses_motion_and_state_contracts() {
    let source = load_source("src/flip_card/view.rs");

    for needle in [
        "motion::attach_motion(root_ref, is_flipped, is_hovered, motion)",
        "logic::resolve_part_state(FlipCardPartStateInput",
        "logic::compose_class_name(class_name.clone(), root_state.get())",
        "logic::should_toggle_key(&ev.key(), is_composing)",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-flip-mode=move || root_state.get().flip_mode_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-slot=move || front_state.get().slot_attr",
        "data-slot=move || back_state.get().slot_attr",
        "data-slot=\"flip-card-inner\"",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard view should include `{needle}` for stable state/source contracts."
        );
    }
}

#[test]
fn flip_card_styles_include_state_source_and_face_markers() {
    let source = load_source("src/flip_card/styles.rs");

    for selector in [
        ".ui-flip-card {",
        ".ui-flip-card[data-disabled=\"true\"]",
        ".ui-flip-card[data-motion-source=\"custom\"]",
        ".ui-flip-card[data-class-source=\"custom\"]",
        ".ui-flip-card[data-id-source=\"custom\"]",
        ".ui-flip-card[data-flip-mode=\"hover\"]",
        ".ui-flip-card[data-flip-mode=\"toggle\"]",
        ".ui-flip-card__inner {",
        ".ui-flip-card__face {",
        ".ui-flip-card__front {",
        ".ui-flip-card__back {",
        ".ui-flip-card__face[data-visible=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FlipCard styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn flip_card_motion_contract_exposes_default_and_customization_tests() {
    let mod_source = load_source("src/flip_card/mod.rs");
    let motion_source = load_source("src/flip_card/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::FlipCardMotion;",
        "pub struct FlipCardMotion",
        "fn default_motion_uses_soft_spring_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "FlipCard motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn flip_card_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::flip_card::styles::CSS);"),
        "ui-components css aggregator should include flip_card styles."
    );
}

#[test]
fn flip_card_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
        "State + Source Markers",
        "data-flip-mode",
        "data-motion-source",
        "data-id-source",
        "data-visible",
        "<FlipCard",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs page should contain `{needle}`."
        );
    }
}

#[test]
fn flip_card_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/flip_card/motion.rs");
    let view_source = load_source("src/flip_card/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: FlipCardMotion) -> FlipCardMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "FlipCard motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::flip_card::motion::sanitize_motion(motion);"),
        "FlipCard view should sanitize motion before deriving state and attaching motion driver.",
    );
}

#[test]
fn flip_card_docs_default_and_disabled_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "<Playground title=\"Click + Keyboard Flip\" code_signal=basic_code>",
        "<div class=\"ui-flip-card__title\">\"Front\"</div>",
        "Click or press Enter/Space to flip.",
        "<div class=\"ui-flip-card__title\">\"Back\"</div>",
        "Back face stays keyboard reachable with the same button semantics.",
        "<Playground title=\"Disabled\" code_signal=disabled_code>",
        "disabled=true",
        "<div class=\"ui-flip-card__title\">\"Disabled front\"</div>",
        "No click/keyboard toggle while disabled.",
        "<div class=\"ui-flip-card__title\">\"Disabled back\"</div>",
        "aria-disabled and disabled markers remain consistent.",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs default/disabled playground should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_card_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-flip-card\".to_string()",
        "class_name=\"docs-flip-card-state\".to_string()",
        "flip_on_hover=true",
        "motion=FlipCardMotion {",
        "hover_scale: 1.03,",
        "hover_tilt_deg: 4.0,",
        "..FlipCardMotion::default()",
        "<div class=\"ui-flip-card__title\">\"Inspect markers (front)\"</div>",
        "Hover enters flipped mode source = custom.",
        "<div class=\"ui-flip-card__title\">\"Inspect markers (back)\"</div>",
        "Front/back visibility markers stay explicit for regression tests.",
    ] {
        assert!(
            source.contains(needle),
            "FlipCard docs marker playground should contain `{needle}`.",
        );
    }
}

#[test]
fn flip_card_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn flip_card() -> AnyView",
        "title=\"FlipCard\"",
        "slug=\"flip-card\"",
        "description=\"3D front/back card with baseline-style state/source markers and baseline-level spring motion for flip/hover interactions.\"",
        "<Playground title=\"Click + Keyboard Flip\" code_signal=basic_code>",
        "title=\"State + Source Markers\"",
        "<Playground title=\"Disabled\" code_signal=disabled_code>",
        "<FlipCard",
    ] {
        assert!(
            source.contains(needle),
            "display_extra flip_card docs page should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn flip_card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Click + Keyboard Flip\"",
        "<div class=\"ui-flip-card__title\">\"Front\"</div>",
        "Click or press Enter/Space to flip.",
        "<div class=\"ui-flip-card__title\">\"Back\"</div>",
        "Back face stays keyboard reachable with the same button semantics.",
        "title=\"State + Source Markers\"",
        "id=\"docs-flip-card\".to_string()",
        "class_name=\"docs-flip-card-state\".to_string()",
        "flip_on_hover=true",
        "hover_scale: 1.03",
        "hover_tilt_deg: 4.0",
        "title=\"Disabled\"",
        "disabled=true",
        "<div class=\"ui-flip-card__title\">\"Disabled front\"</div>",
        "<div class=\"ui-flip-card__title\">\"Disabled back\"</div>",
    ] {
        assert!(
            source.contains(needle),
            "flip_card docs playgrounds should contain `{needle}`.",
        );
    }
}
