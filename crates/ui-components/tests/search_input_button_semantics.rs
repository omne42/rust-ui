use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_input_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button_search_input/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SearchInputButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn search_input_button_uses_logic_state_model() {
    let view_source = load_source("src/button_search_input/view.rs");
    let logic_source = load_source("src/button_search_input/logic.rs");

    for needle in [
        "pub struct SearchInputButtonStateInput",
        "pub struct SearchInputButtonState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_state(input: SearchInputButtonStateInput)",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
        "pub state_attr: &'static str",
        "pub shortcut_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "SearchInputButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let placeholder = logic::normalize_optional_text(placeholder);",
        "let compact_placeholder = logic::normalize_optional_text(compact_placeholder);",
        "let state = logic::resolve_state(SearchInputButtonStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "SearchInputButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn search_input_button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button_search_input/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SearchInputButton should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn search_input_button_emits_spectrum_style_data_attributes() {
    let source = load_source("src/button_search_input/view.rs");

    for attr in [
        "data-slot=\"search-input-button\"",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-shortcut=state.shortcut_attr",
        "data-placeholder=state.placeholder_source_attr",
        "data-compact-placeholder=state.compact_placeholder_source_attr",
        "data-aria-label-source=state.aria_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-hovered",
        "data-pressed",
        "data-motion-source=if motion == SearchInputButtonMotion::default()",
        "data-custom-motion=(motion != SearchInputButtonMotion::default()).then_some(\"true\")",
        "data-slot=\"search-input-button-icon\"",
        "data-slot=\"search-input-button-shortcut\"",
        "data-slot=\"search-input-button-key\"",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should set `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn search_input_button_forwards_headless_button_semantics() {
    let source = load_source("src/button_search_input/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "SearchInputButton should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn search_input_button_uses_fallback_aria_label_from_placeholder() {
    let source = load_source("src/button_search_input/view.rs");

    for needle in [
        "let aria_label = aria_label.unwrap_or_else(|| view_state.placeholder.clone());",
        "let aria_label = StoredValue::new(aria_label);",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton should normalize aria labeling using `{needle}`."
        );
    }
}

#[test]
fn search_input_button_styles_include_state_marker_contracts() {
    let styles = load_source("src/button_search_input/styles.rs");

    for selector in [
        ".ui-search-input-button--enabled",
        ".ui-search-input-button[data-state=\"disabled\"]",
        ".ui-search-input-button--custom-placeholder",
        ".ui-search-input-button[data-compact-placeholder=\"custom\"] .ui-search-input-button__placeholder--compact",
        ".ui-search-input-button--with-shortcut .ui-search-input-button__shortcut",
        ".ui-search-input-button[data-shortcut=\"visible\"] .ui-search-input-button__shortcut",
        ".ui-search-input-button--custom-class",
        ".ui-search-input-button[data-motion-source=\"custom\"]",
        ".ui-search-input-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            styles.contains(selector),
            "SearchInputButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn search_input_button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button_search_input/styles.rs");
    let motion = load_source("src/button_search_input/motion.rs");

    for needle in [
        "--ui-search-input-button-scale",
        "transform: scale(var(--ui-search-input-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "SearchInputButton styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("set_property(\"--ui-search-input-button-scale\""),
        "SearchInputButton motion should write `--ui-search-input-button-scale` to drive interaction feedback without triggering rerenders."
    );

    assert!(
        motion.contains("if is_disabled {\n        return;\n    }"),
        "SearchInputButton motion should short-circuit when disabled to avoid unnecessary work and keep disabled visuals stable."
    );
}

#[test]
fn search_input_button_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button_search_input/motion.rs");

    for needle in [
        "pub struct SearchInputButtonMotion",
        "fn default_motion_matches_search_input_button_spring_contract()",
        "fn supports_custom_search_input_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton motion module should include `{needle}` for HeroUI-level motion contract coverage."
        );
    }
}

#[test]
fn search_input_button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button_search_input/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SearchInputButtonMotion) -> SearchInputButtonMotion",
        "fn sanitize_spring(",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "SearchInputButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn search_input_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn search_input_button() -> AnyView",
        "title=\"SearchInputButton\"",
        "slug=\"search-input-button\"",
        "description=\"HeroUI-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs.\"",
        "<Playground",
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "id_base=\"docs-search-input-preset\".to_string()",
        "id_base=\"docs-search-input-meta-key\".to_string()",
        "id_base=\"docs-search-input-key\".to_string()",
        "aria_label=\"Search input preset\".to_string()",
        "aria_label=\"Search input meta key\".to_string()",
        "aria_label=\"Search input shortcut key\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=custom_aria_label set_checked=set_custom_aria_label>",
        "<Playground title=\"Placeholder + disabled matrix\" code=states_code>",
        "<Playground title=\"Custom Class + Aria Label\" code=custom_code>",
        "<SearchInputButton",
    ] {
        assert!(
            source.contains(needle),
            "actions docs should include `{needle}` for search-input-button primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"button-search-input\" => &[\"search-input-button\"]"),
        "components mod mapping should keep `button-search-input` mapped to `search-input-button` slug.",
    );
}

#[test]
fn search_input_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Interactive + shortcut\"",
        "code_signal=code",
        "let preset_options = vec![",
        "let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0)",
        "let meta_key_options = vec![\"⌘\".to_string(), \"Ctrl\".to_string(), \"Alt\".to_string()];",
        "let key_label_options = vec![\"K\".to_string(), \"F\".to_string()];",
        "placeholder=placeholder",
        "compact_placeholder=compact_placeholder",
        "meta_key_label=meta_key_label",
        "key_label=key_label",
        "if custom_aria_label {",
        "aria_label=\"Open command menu\".to_string()",
        "on_press=on_press",
        "\"presses: \"",
        "title=\"Placeholder + disabled matrix\"",
        "placeholder=\"Find components\".to_string()",
        "compact_placeholder=\"Find\".to_string()",
        "placeholder=\"Disabled search\".to_string() disabled=true",
        "placeholder=\"Forced disabled\".to_string()",
        "is_disabled=true",
        "title=\"Custom Class + Aria Label\"",
        "placeholder=\"Browse components\".to_string()",
        "compact_placeholder=\"Browse\".to_string()",
        "aria_label=\"Open component search\".to_string()",
        "class_name=\"docs-search-input-button-custom\".to_string()",
        "placeholder=\"Search by keyword\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "actions docs playgrounds should contain `{needle}` for search-input-button contracts.",
        );
    }
}
