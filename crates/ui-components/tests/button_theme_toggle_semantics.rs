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
    let view_source = load_source("src/button_theme_toggle/view.rs");
    let styles_source = load_source("src/button_theme_toggle/styles.rs");

    for needle in [
        "motion::attach_motion(icon_ref, mode.into(), motion)",
        "let has_custom_motion = motion != ThemeToggleMotion::default();",
        "let motion_source_attr = if has_custom_motion {",
        "data-slot=\"theme-toggle-button\"",
        "data-slot=\"theme-toggle-icon\"",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "button_theme_toggle view should include `{needle}` for stable motion/source contracts."
        );
    }

    for needle in ["--ui-theme-toggle-rotate", "--ui-theme-toggle-scale"] {
        assert!(
            styles_source.contains(needle),
            "button_theme_toggle styles should include `{needle}` for css-variable motion contracts."
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
        "title=\"Default cycle\"",
        "code_signal=code",
        "let mode_options = vec![\"Light\".to_string(), \"Dark\".to_string(), \"OLED\".to_string()];",
        "id_base=\"docs-theme-toggle-mode\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=two_mode_cycle set_checked=set_two_mode_cycle>",
        "<Switch checked=custom_aria_label set_checked=set_custom_aria_label>",
        "modes=modes",
        "if custom_aria_label {",
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
        "<Playground title=\"Custom modes + disabled\" code_signal=states_code>",
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

#[test]
fn button_theme_toggle_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn theme_toggle_button() -> AnyView",
        "title=\"ThemeToggleButton\"",
        "slug=\"theme-toggle-button\"",
        "title=\"Default cycle\"",
        "title=\"Custom modes + disabled\"",
    ] {
        assert!(
            source.contains(needle),
            "theme-toggle docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_theme_toggle_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Default cycle\"",
        "code_signal=code",
        "let modes = if two_mode_cycle {",
        "<Playground title=\"Custom modes + disabled\" code_signal=states_code>",
        "let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];",
        "modes=custom_modes.clone()",
        "aria_label=\"Switch UI mode\".to_string()",
        "<ThemeToggleButton mode=mode set_mode=set_mode disabled=true />",
        "\"disabled toggle should remain inert\"",
    ] {
        assert!(
            source.contains(needle),
            "theme-toggle docs playground should contain `{needle}`.",
        );
    }
}
