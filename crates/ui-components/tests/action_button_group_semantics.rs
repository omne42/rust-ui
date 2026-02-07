use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_button_group_does_not_expose_logic_module() {
    let source = load_source("src/action_button_group/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "ActionButtonGroup's `logic` module should stay non-public to avoid leaking implementation details into the public API."
    );
}

#[test]
fn action_button_group_uses_logic_state_model() {
    let view_source = load_source("src/action_button_group/view.rs");
    let logic_source = load_source("src/action_button_group/logic.rs");

    for needle in [
        "pub struct ActionButtonGroupState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionButtonGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let class_name = logic::normalize_optional_text(class_name);",
        "let (aria_label, has_explicit_label) = logic::normalize_aria_label(aria_label);",
        "let state = logic::resolve_state(",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionButtonGroup view should derive root state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_button_group_provides_context_for_child_buttons() {
    let source = load_source("src/action_button_group/view.rs");

    for needle in ["provide_context", "ActionButtonGroupContextValue"] {
        assert!(
            source.contains(needle),
            "ActionButtonGroup should provide a context value via `{needle}` so child ActionButton instances can inherit group config."
        );
    }
}

#[test]
fn action_button_group_emits_toolbar_semantics_and_state_attributes() {
    let source = load_source("src/action_button_group/view.rs");

    for needle in [
        "data-slot=\"action-button-group\"",
        "data-state=if state.is_disabled { \"disabled\" } else { \"ready\" }",
        "data-orientation=state.orientation_attr",
        "data-density=state.density_attr",
        "data-horizontal=state.is_horizontal.then_some(\"true\")",
        "data-vertical=state.is_vertical.then_some(\"true\")",
        "data-regular=state.is_regular.then_some(\"true\")",
        "data-compact=state.is_compact.then_some(\"true\")",
        "data-justified=state.is_justified.then_some(\"true\")",
        "data-not-justified=state.is_not_justified.then_some(\"true\")",
        "data-quiet=state.is_quiet.then_some(\"true\")",
        "data-filled=state.is_filled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-has-explicit-label=state.has_explicit_label.then_some(\"true\")",
        "data-has-fallback-label=state.has_fallback_label.then_some(\"true\")",
        "role=\"toolbar\"",
        "aria-orientation=state.orientation.aria_orientation()",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionButtonGroup should set `{needle}` to align with Spectrum toolbar semantics and enable state-driven styling."
        );
    }
}

#[test]
fn action_button_group_defaults_accessible_toolbar_label() {
    let source = load_source("src/action_button_group/logic.rs");

    assert!(
        source.contains("\"Action button group\".to_string()"),
        "ActionButtonGroup should provide a stable fallback aria label when none is passed."
    );
}

#[test]
fn action_button_group_styles_include_density_and_disabled_markers() {
    let source = load_source("src/action_button_group/styles.rs");

    for needle in [
        ".ui-action-button-group--density-regular",
        ".ui-action-button-group--density-compact",
        ".ui-action-button-group--disabled",
    ] {
        assert!(
            source.contains(needle),
            "ActionButtonGroup styles should include `{needle}` for stable visual state contracts."
        );
    }
}
