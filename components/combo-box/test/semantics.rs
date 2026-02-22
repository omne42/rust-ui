use std::fs;
use std::path::Path;

fn read_source(path: &str) -> String {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let full = base.join(path);
    fs::read_to_string(&full).unwrap_or_else(|e| panic!("failed to read {full:?}: {e}"))
}

#[test]
fn combo_box_public_api_exports_are_minimal_and_dom_agnostic() {
    let source = read_source("src/mod.rs");

    for needle in ["pub use motion::ComboBoxMotion;", "pub use view::ComboBox;"] {
        assert!(
            source.contains(needle),
            "combo-box public API should expose `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !source.contains(forbidden),
            "combo-box public API must not expose internal or DOM detail `{forbidden}`."
        );
    }
}

#[test]
fn combo_box_layering_is_split_across_logic_view_styles_and_motion() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");
    let styles_source = read_source("src/styles.rs");
    let motion_source = read_source("src/motion.rs");

    assert!(
        logic_source.contains("pub fn normalize_root_state("),
        "logic.rs should own normalization/state derivation."
    );
    assert!(
        view_source.contains("logic::normalize_root_state(logic::RootStateInput {"),
        "view.rs should consume logic normalization output."
    );
    assert!(
        view_source.contains("use_combo_box(ComboBoxOptions {"),
        "view.rs should mount headless semantic contract."
    );

    for needle in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should consume ui-theme CSS variable `{needle}`."
        );
    }

    for needle in [
        "pub fn attach_popover_motion(",
        "ui_popover::motion::attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should map component semantics and delegate driver via `{needle}`."
        );
    }
}

#[test]
fn combo_box_component_layer_does_not_reimplement_primitives_or_headless_contracts() {
    let logic_source = read_source("src/logic.rs");
    let view_source = read_source("src/view.rs");

    for forbidden in [
        "pub fn normalize_disabled_indices(",
        "pub fn filter_indices(",
        "pub fn map_selected_to_filtered(",
        "pub fn map_filtered_to_original(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should consume ui-state-primitives and must not reimplement `{forbidden}`."
        );
    }

    for needle in [
        "let option_attrs = aria.option_attrs;",
        "role=move || option_attrs.run(filtered_index).role",
        "aria-selected=move || option_attrs.run(filtered_index).aria_selected",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount headless option attrs contract via `{needle}`."
        );
    }
}
