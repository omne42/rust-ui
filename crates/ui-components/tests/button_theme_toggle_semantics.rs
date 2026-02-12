use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_theme_toggle_module_reexports_component_motion_and_logic_contracts() {
    let source = load_source("src/button_theme_toggle/mod.rs");

    for needle in [
        "pub use logic::{ThemeMode, ThemeToggleViewState, resolve_view_state};",
        "pub use motion::ThemeToggleMotion;",
        "pub use view::ThemeToggleButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_theme_toggle module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_theme_toggle_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button_theme_toggle;",
        "pub use button_theme_toggle::{ThemeMode, ThemeToggleButton, ThemeToggleMotion};",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_theme_toggle compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_theme_toggle_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn theme_toggle_button() -> AnyView",
        "title=\"ThemeToggleButton\"",
        "slug=\"theme-toggle-button\"",
        "<ThemeToggleButton",
        "ThemeMode::Light",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for theme-toggle-button coverage.",
        );
    }
}

#[test]
fn button_theme_toggle_motion_contract_defaults_and_sanitization_are_locked() {
    let source = load_source("src/button_theme_toggle/motion.rs");

    for needle in [
        "pub struct ThemeToggleMotion",
        "spring: ui_motion::presets::spring_soft()",
        "rotate_deg: 180.0",
        "scale_down: 0.92",
        "scale_settle_delay_ms: 40",
        "pub fn sanitize_motion(motion: ThemeToggleMotion) -> ThemeToggleMotion",
        ".clamp(-MAX_ROTATE_DEG, MAX_ROTATE_DEG)",
        ".clamp(MIN_SCALE_DOWN, MAX_SCALE_DOWN)",
        "scale_settle_delay_ms: motion.scale_settle_delay_ms.min(MAX_SETTLE_DELAY_MS)",
        "fn sanitize_motion_falls_back_and_clamps_values()",
        "fn sanitize_motion_keeps_valid_values()",
    ] {
        assert!(
            source.contains(needle),
            "button_theme_toggle motion should include `{needle}` for HeroUI-level spring contract stability."
        );
    }
}

#[test]
fn button_theme_toggle_view_wires_motion_and_source_markers() {
    let source = load_source("src/button_theme_toggle/view.rs");

    for needle in [
        "motion::attach_motion(icon_ref, mode.into(), motion)",
        "data-motion-source=if motion == ThemeToggleMotion::default()",
        "data-custom-motion=(motion != ThemeToggleMotion::default()).then_some(\"true\")",
        "--ui-theme-toggle-rotate",
        "--ui-theme-toggle-scale",
    ] {
        assert!(
            source.contains(needle)
                || load_source("src/button_theme_toggle/motion.rs").contains(needle),
            "button_theme_toggle should include `{needle}` for stable motion/source contracts."
        );
    }
}

#[test]
fn docs_actions_page_locks_theme_toggle_motion_narrative() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "description=\"Icon-only theme toggle with HeroUI-level spring motion and Spectrum-style mode state attrs.\"",
        "title=\"Custom modes + disabled\"",
        "disabled toggle should remain inert",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for theme-toggle motion/docs stability."
        );
    }
}

#[test]
fn theme_toggle_docs_default_cycle_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn theme_toggle_button() -> AnyView",
        "<Playground title=\"Default cycle\" code=code>",
        "let (mode, set_mode) = signal(ThemeMode::Light);",
        "<ThemeToggleButton mode=mode set_mode=set_mode />",
        "\"mode: \" {move || format!(\"{:?}\", mode.get())}",
    ] {
        assert!(
            source.contains(needle),
            "theme-toggle docs default-cycle playground should contain `{needle}`.",
        );
    }
}

#[test]
fn theme_toggle_docs_custom_modes_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Custom modes + disabled\" code=states_code>",
        "let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];",
        "mode=custom_mode",
        "set_mode=set_custom_mode",
        "modes=custom_modes.clone()",
        "aria_label=\"Switch UI mode\".to_string()",
        "\"custom mode: \" {move || format!(\"{:?}\", custom_mode.get())}",
        "<ThemeToggleButton mode=mode set_mode=set_mode disabled=true />",
        "\"disabled toggle should remain inert\"",
    ] {
        assert!(
            source.contains(needle),
            "theme-toggle docs custom-modes playground should contain `{needle}`.",
        );
    }
}
