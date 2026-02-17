use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_button_has_no_compat_module_and_is_reexported_from_button_action() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub use button::action::{",
        "ActionButton",
        "ActionButtonLoadingPlacement",
        "ActionButtonMotion",
        "ActionButtonSize",
        "ActionButtonType",
    ] {
        assert!(
            source.contains(needle),
            "crate re-exports should include `{needle}` from button/action."
        );
    }

    assert!(
        !source.contains("pub mod action_button;"),
        "compat module `src/action_button.rs` should not be reintroduced."
    );
}

#[test]
fn action_button_implementation_lives_under_button_action_module() {
    let mod_source = load_source("src/button/action/mod.rs");
    let view_source = load_source("src/button/action/view.rs");

    for needle in [
        "pub type ActionButtonSize = ButtonSize;",
        "pub type ActionButtonLoadingPlacement = ButtonLoadingPlacement;",
        "pub type ActionButtonMotion = ButtonMotion;",
        "pub type ActionButtonType = ButtonType;",
    ] {
        assert!(
            mod_source.contains(needle),
            "button/action module should define `{needle}` as the canonical ActionButton contract."
        );
    }

    assert!(
        view_source.contains("pub fn ActionButton("),
        "ActionButton view should live in `src/button/action/view.rs`."
    );
}

#[test]
fn action_button_inherits_group_context_when_feature_enabled() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "use_action_button_group_context()",
        "let inherited_disabled = group.map(|ctx| ctx.is_disabled);",
        "let inherited_size = group.map(|ctx| ctx.size);",
        "let inherited_quiet = group.map(|ctx| ctx.is_quiet);",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should inherit group contract via `{needle}` when grouped."
        );
    }
}

#[test]
fn action_button_uses_button_state_machine_and_headless_hooks() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "let state = button_logic::resolve_state(button_logic::ButtonStateInput {",
        "let aria = use_button(ButtonOptions {",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should be wired through shared button logic/headless hooks via `{needle}`."
        );
    }
}

#[test]
fn action_button_api_naming_uses_is_prefix_only() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "#[prop(optional)] is_loading: bool",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_quiet: Option<bool>",
        "#[prop(optional)] is_icon_only: bool",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton API naming should include `{needle}`."
        );
    }

    assert!(
        !source.contains("#[prop(optional)] disabled: Option<bool>"),
        "ActionButton should not expose legacy boolean alias `disabled`."
    );
}

#[test]
fn action_button_emits_semantic_slot_and_loading_attributes() {
    let source = load_source("src/button/action/view.rs");

    for needle in [
        "data-slot=\"action-button\"",
        "data-loading=state.is_loading.then_some(\"true\")",
        "data-loading-placement=state.loading_placement_attr",
        "data-quiet=is_quiet.then_some(\"true\")",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionButton should expose semantic/loading attrs via `{needle}`."
        );
    }
}

#[test]
fn action_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_button() -> AnyView",
        "title=\"ActionButton\"",
        "slug=\"action-button\"",
        "<ActionButton",
        "is_quiet=true",
        "is_loading=true",
        "loading_placement=ActionButtonLoadingPlacement::Center",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for action-button coverage."
        );
    }
}
