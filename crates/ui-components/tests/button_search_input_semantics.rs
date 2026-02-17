use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_search_input_module_reexports_component_and_motion() {
    let source = load_source("src/button/search_input/mod.rs");

    for needle in [
        "pub use motion::SearchInputButtonMotion;",
        "pub use view::SearchInputButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_search_input module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_search_input_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button;",
        "pub use button::search_input::{SearchInputButton, SearchInputButtonMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_search_input compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_search_input_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"SearchInputButton\"",
        "slug=\"search-input-button\"",
        "<SearchInputButton",
        "placeholder=\"Find components\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for search-input-button coverage.",
        );
    }
}

#[test]
fn button_search_input_motion_contract_defaults_and_sanitize_paths_are_locked() {
    let source = load_source("src/button/search_input/motion.rs");

    for needle in [
        "pub struct SearchInputButtonMotion",
        "stiffness: 260.0",
        "damping: 16.0",
        "mass: 1.0",
        "hover_scale: 1.0",
        "tap_scale: 0.98",
        "pub fn sanitize_motion(motion: SearchInputButtonMotion) -> SearchInputButtonMotion",
        ".clamp(0.5, 2.0)",
        ".clamp(0.5, 1.5)",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "search input button motion should include `{needle}` for baseline-level spring contract stability."
        );
    }
}

#[test]
fn button_search_input_view_wires_motion_and_source_markers() {
    let source = load_source("src/button/search_input/view.rs");

    for needle in [
        "motion::attach_motion(",
        "data-motion-source=if motion == SearchInputButtonMotion::default()",
        "data-custom-motion=(motion != SearchInputButtonMotion::default()).then_some(\"true\")",
        "data-hovered=move || if hover.is_hovered.get() { Some(\"true\") } else { None }",
        "data-pressed=move || if aria.is_pressed.get() { Some(\"true\") } else { None }",
    ] {
        assert!(
            source.contains(needle),
            "search input button view should include `{needle}` for stable motion/source marker contracts."
        );
    }
}

#[test]
fn docs_actions_page_locks_search_input_motion_narrative() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "description=\"baseline-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs.\"",
        "title=\"Interactive + shortcut\"",
        "title=\"Custom Class + Aria Label\"",
        "let meta_key_options = vec![",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for search-input-button motion/docs stability."
        );
    }
}

#[test]
fn search_input_button_docs_interactive_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "id_base=\"docs-search-input-preset\".to_string()",
        "id_base=\"docs-search-input-meta-key\".to_string()",
        "id_base=\"docs-search-input-key\".to_string()",
        "let preset_options = vec![",
        "let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0)",
        "let meta_key_options = vec![",
        "let key_label_options = vec![",
        "if custom_aria_label {",
        "aria_label=\"Open command menu\".to_string()",
        "on_press=on_press",
        "\"presses: \" {move || press_count.get().to_string()}",
    ] {
        assert!(
            source.contains(needle),
            "search-input-button interactive playground should contain `{needle}`.",
        );
    }
}

#[test]
fn search_input_button_docs_state_and_custom_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Placeholder + disabled matrix\" code_signal=states_code>",
        "placeholder=\"Find components\".to_string()",
        "compact_placeholder=\"Find\".to_string()",
        "placeholder=\"Disabled search\".to_string() disabled=true",
        "placeholder=\"Forced disabled\".to_string()",
        "is_disabled=true",
        "<Playground title=\"Custom Class + Aria Label\" code_signal=custom_code>",
        "placeholder=\"Browse components\".to_string()",
        "compact_placeholder=\"Browse\".to_string()",
        "aria_label=\"Open component search\".to_string()",
        "placeholder=\"Search by keyword\".to_string()",
        "class_name=\"docs-search-input-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "search-input-button state/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn button_search_input_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"SearchInputButton\"",
        "slug=\"search-input-button\"",
        "title=\"Interactive + shortcut\"",
        "title=\"Placeholder + disabled matrix\"",
        "title=\"Custom Class + Aria Label\"",
    ] {
        assert!(
            source.contains(needle),
            "search-input-button docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_search_input_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "id_base=\"docs-search-input-preset\".to_string()",
        "id_base=\"docs-search-input-meta-key\".to_string()",
        "<Playground title=\"Placeholder + disabled matrix\" code_signal=states_code>",
        "placeholder=\"Disabled search\".to_string() disabled=true",
        "placeholder=\"Forced disabled\".to_string()",
        "is_disabled=true",
        "<Playground title=\"Custom Class + Aria Label\" code_signal=custom_code>",
        "class_name=\"docs-search-input-button-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "search-input-button docs playground should contain `{needle}`.",
        );
    }
}
