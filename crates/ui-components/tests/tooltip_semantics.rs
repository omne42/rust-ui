use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tooltip_does_not_expose_view_or_motion_modules() {
    let source = load_source("src/tooltip/mod.rs");

    for needle in ["pub mod motion", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tooltip's internal modules should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tooltip_uses_headless_trigger_and_position_hooks() {
    let source = load_source("src/tooltip/view.rs");

    for needle in ["use_tooltip_trigger", "use_tooltip_position"] {
        assert!(
            source.contains(needle),
            "Tooltip should use headless `{needle}` hooks for Spectrum-style behavior and positioning."
        );
    }
}

#[test]
fn tooltip_uses_presence_for_exit_motion_unmounting() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "use_presence(open)",
        "presence.finish_exit",
        "motion::attach_motion",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn tooltip_emits_spectrum_style_data_slots_and_portal_marker() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "data-slot=\"tooltip\"",
        "data-slot=\"tooltip-panel\"",
        "data-ui-overlay-portal=\"\"",
        "data-placement",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should set `{needle}` to support Spectrum-style styling and regression testing."
        );
    }
}

#[test]
fn tooltip_emits_spectrum_style_state_and_motion_markers() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-disabled=disabled.then_some(\"true\")",
        "data-enabled=(!disabled).then_some(\"true\")",
        "data-motion-source=if motion == TooltipMotion::default()",
        "data-custom-motion=(motion != TooltipMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should expose `{needle}` for stable state/motion styling hooks."
        );
    }
}

#[test]
fn tooltip_panel_has_role_and_css_variable_positioning() {
    let source = load_source("src/tooltip/view.rs");

    for needle in ["role=\"tooltip\"", "--ui-tooltip-top", "--ui-tooltip-left"] {
        assert!(
            source.contains(needle),
            "Tooltip panel should wire `{needle}` for accessible tooltip semantics and CSS-variable positioning."
        );
    }
}

#[test]
fn tooltip_manages_aria_describedby_on_the_focused_element() {
    let source = load_source("src/tooltip/view.rs");

    for needle in [
        "aria-describedby",
        "set_attribute(\"aria-describedby\"",
        "remove_attribute(\"aria-describedby\"",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should manage `aria-describedby` on the focused element via `{needle}` (Spectrum parity)."
        );
    }
}

#[test]
fn tooltip_motion_drives_opacity_scale_and_offset_via_css_variables() {
    let styles = load_source("src/tooltip/styles.rs");
    let motion = load_source("src/tooltip/motion.rs");

    for needle in [
        "--ui-tooltip-opacity",
        "--ui-tooltip-scale",
        "--ui-tooltip-y",
        "pointer-events: none",
    ] {
        assert!(
            styles.contains(needle),
            "Tooltip styles should reference `{needle}` for motion and interaction behavior."
        );
    }

    for needle in [
        "--ui-tooltip-opacity",
        "--ui-tooltip-scale",
        "--ui-tooltip-y",
        "SpringAnimator",
    ] {
        assert!(
            motion.contains(needle),
            "Tooltip motion should update `{needle}` to provide HeroUI-style spring behavior without rerenders."
        );
    }
}

#[test]
fn tooltip_styles_include_root_state_and_motion_marker_contracts() {
    let source = load_source("src/tooltip/styles.rs");

    for needle in [
        ".ui-tooltip[data-motion-source=\"custom\"]",
        ".ui-tooltip[data-custom-motion=\"true\"]",
        ".ui-tooltip[data-disabled=\"true\"]",
        ".ui-tooltip[data-state=\"open\"]",
        ".ui-tooltip[data-state=\"closed\"]",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip styles should include `{needle}` for motion/state selector contracts."
        );
    }
}

#[test]
fn tooltip_motion_contract_exposes_default_and_custom_test_coverage() {
    let source = load_source("src/tooltip/motion.rs");

    for needle in [
        "pub struct TooltipMotion",
        "fn default_motion_uses_soft_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip motion contract should include `{needle}` for HeroUI-style regression coverage."
        );
    }
}
