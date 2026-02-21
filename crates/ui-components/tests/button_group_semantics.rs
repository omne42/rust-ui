use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/button/") {
        let migrated = manifest_dir
            .join("../../components/button/src")
            .join(suffix);
        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ButtonGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn button_group_uses_logic_state_model() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "pub struct ButtonGroupState",
        "pub fn normalize_button_group_aria_label(",
        "pub fn resolve_button_group_state(",
        "pub fn compose_button_group_class_name(",
        "pub is_attached: bool",
        "pub has_explicit_label: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "ButtonGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let (aria_label, has_explicit_label) = logic::normalize_button_group_aria_label(aria_label);",
        "let group_a11y = labeled_group_attrs(aria_label, lang, dir);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let state = Memo::new(move |_|",
        "logic::resolve_button_group_state(orientation, is_attached, has_explicit_label)",
        "let class = logic::compose_button_group_class_name(class_name, orientation, is_attached);",
    ] {
        assert!(
            view_source.contains(needle),
            "ButtonGroup view should derive root state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn button_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "data-slot=\"button-group\"",
        "data-orientation=orientation.data_orientation()",
        "data-horizontal=move || state.get().is_horizontal.then_some(\"true\")",
        "data-vertical=move || state.get().is_vertical.then_some(\"true\")",
        "data-attached=move || state.get().is_attached.then_some(\"true\")",
        "data-detached=move || state.get().is_detached.then_some(\"true\")",
        "data-has-explicit-label=move || state.get().has_explicit_label.then_some(\"true\")",
        "data-has-fallback-label=move || state.get().has_fallback_label.then_some(\"true\")",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label.clone()",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            source.contains(attr),
            "ButtonGroup should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn button_group_defaults_accessible_group_label() {
    let source = load_source("src/button/logic.rs");

    assert!(
        source.contains("\"Button group\".to_string()"),
        "ButtonGroup should provide a stable fallback aria label when none is passed."
    );
}

#[test]
fn button_group_styles_define_attached_overlap_rule() {
    let source = load_source("src/button/styles.rs");

    assert!(
        source.contains("--ui-button-group-border-overlap"),
        "ButtonGroup styles should define overlap CSS vars for attached layout behavior."
    );
}

#[test]
fn button_group_docs_page_covers_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_group() -> AnyView",
        "title=\"ButtonGroup\"",
        "slug=\"button-group\"",
        "description=\"Groups Buttons with baseline-style root state attrs for orientation, attachment, and accessible labeling.\"",
        "<Playground title=\"Attached horizontal\" code_signal=code>",
        "<Playground title=\"Vertical + detached\" code_signal=states_code>",
        "<ButtonGroup",
        "is_attached=true",
        "is_attached=false",
        "orientation=ButtonGroupOrientation::Vertical",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for button-group coverage.",
        );
    }
}

#[test]
fn button_group_docs_attached_and_vertical_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<ButtonGroup is_attached=true>",
        "<Button variant=ButtonVariant::Secondary>\"Left\"</Button>",
        "<Button variant=ButtonVariant::Secondary>\"Middle\"</Button>",
        "<Button variant=ButtonVariant::Secondary>\"Right\"</Button>",
        "\"left/middle/right clicks: \"",
        "orientation=ButtonGroupOrientation::Horizontal",
        "aria_label=\"Document actions\".to_string()",
        "<Button variant=ButtonVariant::Outline is_disabled=true>",
        "\"top/bottom clicks: \"",
        "{move || format!(\"{}/{}\", top_count.get(), bottom_count.get())}",
    ] {
        assert!(
            source.contains(needle),
            "button-group docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn button_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_group() -> AnyView",
        "title=\"ButtonGroup\"",
        "slug=\"button-group\"",
        "title=\"Attached horizontal\"",
        "title=\"Vertical + detached\"",
    ] {
        assert!(
            source.contains(needle),
            "button-group docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Attached horizontal\" code_signal=code>",
        "<ButtonGroup is_attached=true orientation=ButtonGroupOrientation::Horizontal>",
        "\"left/middle/right clicks: \"",
        "<Playground title=\"Vertical + detached\" code_signal=states_code>",
        "is_attached=false",
        "orientation=ButtonGroupOrientation::Vertical",
        "aria_label=\"Document actions\".to_string()",
        "<Button variant=ButtonVariant::Outline is_disabled=true>",
        "\"top/bottom clicks: \"",
    ] {
        assert!(
            source.contains(needle),
            "button-group docs playground should contain `{needle}`.",
        );
    }
}
