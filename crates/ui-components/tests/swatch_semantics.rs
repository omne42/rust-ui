use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn swatch_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/swatch/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Swatch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn swatch_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/swatch/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Swatch;"),
        "swatch module should export `Swatch`."
    );
    assert!(
        crate_source
            .contains("pub use swatch::{Swatch, SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize};"),
        "crate root should re-export Swatch contract."
    );
}

#[test]
fn swatch_attaches_motion_driver() {
    let source = load_source("src/swatch/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Swatch should attach its motion driver to deliver spring-based selection feedback."
    );
}

#[test]
fn swatch_motion_uses_spring_animator() {
    let source = load_source("src/swatch/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Swatch motion should animate via springs to match the repo motion spec."
    );
}

#[test]
fn swatch_motion_contract_defaults_match_heroui_level_expectations() {
    let source = load_source("src/swatch/motion.rs");

    for needle in [
        "stiffness: 280.0",
        "damping: 20.0",
        "mass: 1.0",
        "selected_scale: 1.06",
        "selected_ring_opacity: 1.0",
        "pub fn disabled() -> Self",
        "enabled: false",
    ] {
        assert!(
            source.contains(needle),
            "Swatch motion contract should include `{needle}` for HeroUI-level defaults and disabled-path stability."
        );
    }
}

#[test]
fn swatch_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/swatch/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SwatchMotion) -> SwatchMotion",
        ".clamp(1.0, 1.18)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            source.contains(needle),
            "Swatch motion implementation should include `{needle}` to avoid HeroUI-level motion regressions."
        );
    }
}

#[test]
fn swatch_styles_use_css_variables_for_motion() {
    let source = load_source("src/swatch/styles.rs");

    for name in ["--ui-swatch-scale", "--ui-swatch-ring-opacity"] {
        assert!(
            source.contains(name),
            "Swatch styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn swatch_docs_page_exists_in_display_extra_swatch() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "pub(super) fn swatch() -> AnyView",
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "<Swatch",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs page should contain `{needle}`."
        );
    }
}

#[test]
fn swatch_docs_page_includes_custom_motion_playground() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "SwatchMotion {",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs page should include `{needle}` for custom motion contract demos."
        );
    }
}

#[test]
fn swatch_docs_default_and_state_playgrounds_lock_contract_values() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "<Playground title=\"Size + Shape + Rounding\" code=size_code>",
        "size=SwatchSize::Xs",
        "size=SwatchSize::S",
        "size=SwatchSize::M",
        "size=SwatchSize::L",
        "shape=SwatchShape::Rectangle",
        "rounding=SwatchRounding::Full",
        "border=SwatchBorder::Light",
        "<Playground title=\"Mixed + Nothing + Disabled + Controlled\" code=state_code>",
        "label=\"Brand blue\".to_string()",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "mixed_value=true",
        "nothing=true",
        "border=SwatchBorder::None",
        "color=\"#111827\".to_string()",
        "disabled=true",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs default/state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn swatch_docs_custom_motion_playground_locks_contract_values() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "<Playground title=\"Custom Motion Contract\" code=motion_code>",
        "let custom_motion = SwatchMotion {",
        "selected_scale: 1.12,",
        "selected_ring_opacity: 0.92,",
        "..SwatchMotion::default()",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
        "label=\"Hero motion\".to_string()",
        "label=\"Reduced motion\".to_string()",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn swatch_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "pub(super) fn swatch() -> AnyView",
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "description=\"Spectrum-compatible swatch primitive with centralized size/shape/rounding/border/state contracts and HeroUI-grade spring selection motion.\"",
        "<Playground title=\"Size + Shape + Rounding\" code=size_code>",
        "<Playground title=\"Mixed + Nothing + Disabled + Controlled\" code=state_code>",
        "<Playground title=\"Custom Motion Contract\" code=motion_code>",
        "<Swatch",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_swatch docs should include `{needle}` for swatch primary playground coverage.",
        );
    }
}

#[test]
fn swatch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Size + Shape + Rounding\"",
        "size=SwatchSize::Xs",
        "size=SwatchSize::S",
        "size=SwatchSize::M",
        "size=SwatchSize::L",
        "shape=SwatchShape::Rectangle",
        "rounding=SwatchRounding::Full",
        "border=SwatchBorder::Light",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "mixed_value=true",
        "nothing=true",
        "disabled=true",
        "title=\"Custom Motion Contract\"",
        "let custom_motion = SwatchMotion {",
        "selected_scale: 1.12,",
        "selected_ring_opacity: 0.92,",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_swatch docs playgrounds should contain `{needle}` for swatch state-matrix contracts.",
        );
    }
}
