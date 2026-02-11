use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tabs_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tabs/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Tabs's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
    assert!(
        !source.contains("pub mod view"),
        "Tabs's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn tabs_uses_headless_hooks() {
    let source = load_source("src/tabs/view.rs");

    for needle in [
        "use_roving_tabindex",
        "use_focus_ring",
        "use_hover",
        "use_press",
    ] {
        assert!(
            source.contains(needle),
            "Tabs should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn tabs_attaches_indicator_motion_driver() {
    let source = load_source("src/tabs/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Tabs should attach a motion driver for the selection indicator (HeroUI-style feel)."
    );
}

#[test]
fn tabs_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/tabs/view.rs");

    for attr in [
        "data-slot=\"tabs\"",
        "data-slot=\"tabs-list\"",
        "data-slot=\"tabs-indicator\"",
        "data-slot=\"tabs-tab\"",
        "data-slot=\"tabs-panel\"",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selection-empty=move || state.get().selected_index.is_none().then_some(\"true\")",
        "data-has-disabled-tabs=move || state.get().has_disabled_tabs.then_some(\"true\")",
        "data-keyboard-activation=match keyboard_activation",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-index=index",
        "data-selected",
        "data-hovered",
        "data-pressed",
        "data-disabled",
        "data-focused",
        "data-focus-visible",
    ] {
        assert!(
            source.contains(attr),
            "Tabs should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn tabs_uses_logic_state_model() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");

    for needle in [
        "pub struct TabsState",
        "pub fn resolve_tabs_state(",
        "pub selected_index: Option<usize>",
        "pub has_disabled_tabs: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tabs logic should include `{needle}` for centralized root-state derivation."
        );
    }

    assert!(
        view_source.contains("resolve_tabs_state(item_count, selected.get(), has_disabled_tabs)"),
        "Tabs view should derive root state through resolve_tabs_state."
    );
}

#[test]
fn tabs_styles_include_motion_marker_contracts() {
    let source = load_source("src/tabs/styles.rs");

    for selector in [
        ".ui-tabs[data-motion-source=\"custom\"]",
        ".ui-tabs[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tabs styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn tabs_styles_define_indicator_css_vars() {
    let source = load_source("src/tabs/styles.rs");

    for var in [
        "--ui-tabs-indicator-x",
        "--ui-tabs-indicator-w",
        "--ui-tabs-indicator-o",
    ] {
        assert!(
            source.contains(var),
            "Tabs styles should define `{var}` so motion can update the indicator without re-rendering."
        );
    }
}

#[test]
fn tabs_motion_uses_spring_animator() {
    let source = load_source("src/tabs/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Tabs motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn tabs_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/tabs/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TabsMotion) -> TabsMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_spring_values()",
        "fn sanitize_motion_keeps_valid_custom_spring_values()",
    ] {
        assert!(
            source.contains(needle),
            "Tabs motion should include `{needle}` so invalid custom spring contracts cannot leak into runtime animation state.",
        );
    }
}
