use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_flip_module_reexports_flip_button_contracts() {
    let source = load_source("src/button_flip/mod.rs");

    for needle in [
        "pub use logic::FlipDirection;",
        "pub use motion::FlipButtonMotion;",
        "pub use view::FlipButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_flip module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_flip_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_flip;",
        "pub use button_flip::{FlipButton, FlipButtonMotion, FlipDirection};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_flip compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_flip_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "<FlipButton",
        "FlipDirection::Top",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip-button coverage.",
        );
    }
}

#[test]
fn button_flip_logic_tracks_class_and_motion_source_markers() {
    let source = load_source("src/button_flip/logic.rs");

    for needle in [
        "pub struct FlipButtonStateInput",
        "pub struct FlipButtonState",
        "pub has_custom_motion: bool",
        "pub class_source_attr: &'static str",
        "pub motion_source_attr: &'static str",
        "class_source_attr: if input.has_custom_class_name {",
        "motion_source_attr: if input.has_custom_motion {",
        "ui-flip-button--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "flip button logic should include `{needle}` for stable source-marker derivation.",
        );
    }
}

#[test]
fn button_flip_motion_contract_defaults_and_sanitize_paths_are_locked() {
    let source = load_source("src/button_flip/motion.rs");

    for needle in [
        "pub struct FlipButtonMotion",
        "stiffness: 260.0",
        "damping: 18.0",
        "mass: 1.0",
        "pub fn sanitize_motion(motion: FlipButtonMotion) -> FlipButtonMotion",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "flip button motion should include `{needle}` for HeroUI-level spring contract stability."
        );
    }
}

#[test]
fn button_flip_view_wires_motion_and_source_markers() {
    let source = load_source("src/button_flip/view.rs");

    for needle in [
        "let motion = crate::button_flip::motion::sanitize_motion(motion);",
        "let state = Signal::derive(move || {",
        "has_custom_motion,",
        "motion::attach_motion(node_ref, is_active, from, motion)",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-slot=\"flip-button-front\"",
        "data-slot=\"flip-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "flip button view should include `{needle}` for stable motion/source marker contracts."
        );
    }
}

#[test]
fn button_flip_styles_include_source_marker_selectors() {
    let source = load_source("src/button_flip/styles.rs");

    for needle in [
        ".ui-flip-button[data-class-source=\"custom\"]",
        ".ui-flip-button--custom-class",
        ".ui-flip-button[data-custom-class=\"true\"]",
        ".ui-flip-button[data-motion-source=\"custom\"]",
        ".ui-flip-button--custom-motion",
        ".ui-flip-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "flip button styles should include `{needle}` for stable source-marker selectors."
        );
    }
}

#[test]
fn docs_actions_page_locks_flip_button_motion_narrative() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "description=\"HeroUI-level spring flip surface with centralized direction/interaction/class-source state attrs.\"",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for flip-button motion/docs stability."
        );
    }
}

#[test]
fn button_flip_docs_top_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "<Playground title=\"Top flip\" code_signal=code>",
        "from=FlipDirection::Top",
        "<Button variant=ButtonVariant::Secondary>\"Front\"</Button>",
        "<Button variant=ButtonVariant::Accent>\"Back\"</Button>",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs top playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_direction_matrix_and_custom_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "from=FlipDirection::Bottom",
        "<Button variant=ButtonVariant::Secondary>\"Bottom\"</Button>",
        "from=FlipDirection::Left",
        "<Button variant=ButtonVariant::Secondary>\"Left\"</Button>",
        "from=FlipDirection::Right",
        "<Button variant=ButtonVariant::Secondary>\"Right\"</Button>",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "class_name=\"docs-flip-button-custom\".to_string()",
        "<Button variant=ButtonVariant::Outline>\"Inspect\"</Button>",
        "<Button variant=ButtonVariant::Accent>\"Inspecting\"</Button>",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs matrix/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn flip_button() -> AnyView",
        "title=\"FlipButton\"",
        "slug=\"flip-button\"",
        "Top flip",
        "Direction matrix",
        "Custom Class",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_flip_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Top flip\" code_signal=code>",
        "from=FlipDirection::Top",
        "<Playground title=\"Direction matrix\" code_signal=states_code>",
        "from=FlipDirection::Bottom",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
        "<Playground title=\"Custom Class\" code_signal=custom_code>",
        "class_name=\"docs-flip-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "flip-button docs playground should contain `{needle}`.",
        );
    }
}
