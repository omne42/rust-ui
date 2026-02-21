use std::fs;
use std::path::{Path, PathBuf};

fn component_src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn load_source(rel_path: &str) -> String {
    let path = component_src_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn component_file_boundaries_are_explicit() {
    let mod_source = load_source("mod.rs");
    let logic_source = load_source("logic.rs");
    let view_source = load_source("view.rs");
    let styles_source = load_source("styles.rs");
    let motion_source = load_source("motion.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "pub mod motion;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Legend module boundary should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::legend::{",
        "pub fn normalize_required_state(",
        "pub fn normalize_accessibility_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Legend logic should stay as primitive mapping layer; missing `{needle}`."
        );
    }

    assert!(
        view_source.contains("let semantics = use_legend(LegendOptions {"),
        "Legend view should mount headless semantics contract."
    );
    assert!(
        view_source.contains("logic::normalize_component_state(logic::LegendNormalizeInput {"),
        "Legend view should consume centralized state normalization model from logic.rs."
    );
    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Legend styles should stay in styles.rs."
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "Legend motion contract should stay in motion.rs."
    );
}

#[test]
fn view_mounts_stable_semantic_markers() {
    let source = load_source("view.rs");

    for attr in [
        "data-slot=\"legend\"",
        "data-tone=legend_data_tone",
        "data-state=legend_data_state",
        "data-required=legend_data_required",
        "data-disabled=legend_data_disabled",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-action=logic::LegendUiAction::Idle.as_attr()",
        "aria-disabled=legend_aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Legend view should expose semantic marker `{attr}`.",
        );
    }
}

#[test]
fn public_sources_do_not_expose_platform_dom_types() {
    for file in ["mod.rs", "logic.rs", "view.rs"] {
        let source = load_source(file);
        for forbidden in ["web_sys", "wasm_bindgen"] {
            assert!(
                !source.contains(forbidden),
                "Legend component public layer should not expose `{forbidden}` in `{file}`.",
            );
        }
    }
}
