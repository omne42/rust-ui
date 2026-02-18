use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dropdown_menu_does_not_expose_logic_module() {
    let source = load_source("src/dropdown_menu/mod.rs");

    for needle in ["pub mod logic", "pub use logic"] {
        assert!(
            !source.contains(needle),
            "DropdownMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn dropdown_menu_uses_logic_state_model() {
    let view_source = load_source("src/dropdown_menu/view.rs");
    let logic_source = load_source("src/dropdown_menu/logic.rs");

    for needle in [
        "pub struct DropdownMenuStateInput",
        "pub struct DropdownMenuState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DropdownMenu logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let state = logic::resolve_state(logic::DropdownMenuStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn dropdown_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "motion: DropdownMenuMotion",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn dropdown_menu_trigger_wires_overlay_aria_contract() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
        "disabled=state.is_trigger_disabled",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should wire `{needle}` to match baseline overlay trigger semantics."
        );
    }
}

#[test]
fn dropdown_menu_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "data-slot=\"dropdown-menu\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-disabled=state.is_trigger_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-placement=state.placement_attr",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "data-motion-source=if motion == DropdownMenuMotion::default()",
        "data-custom-motion=(motion != DropdownMenuMotion::default()).then_some(\"true\")",
        "on:keydown=on_key_down",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should set `{needle}` so it can be styled/tested with baseline-compatible root state selectors."
        );
    }
}

#[test]
fn dropdown_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "placement=state.placement",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
        "motion=motion.popover",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn dropdown_menu_uses_logic_for_disabled_trigger_and_open_keys() {
    let view_source = load_source("src/dropdown_menu/view.rs");
    let logic_source = load_source("src/dropdown_menu/logic.rs");

    for needle in [
        "resolve_trigger_disabled",
        "focus_strategy_for_open_key",
        "MenuOpenFocusStrategy",
    ] {
        assert!(
            logic_source.contains(needle),
            "DropdownMenu logic should centralize `{needle}` semantics."
        );
    }

    for needle in [
        "if trigger_disabled.get_value()",
        "if let Some(strategy) = logic::focus_strategy_for_open_key(&key)",
        "set_open_focus.set(strategy);",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should consume `{needle}` to keep trigger behavior and keyboard-open semantics consistent."
        );
    }
}

#[test]
fn dropdown_menu_styles_include_disabled_and_persistent_markers() {
    let source = load_source("src/dropdown_menu/styles.rs");

    for needle in [
        ".ui-dropdown-menu--persistent",
        ".ui-dropdown-menu--disabled",
        ".ui-dropdown-menu[data-motion-source=\"custom\"]",
        ".ui-dropdown-menu[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "DropdownMenu styles should include `{needle}` for stable visual state contracts."
        );
    }
}

#[test]
fn dropdown_menu_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/dropdown_menu/mod.rs");
    let motion_source = load_source("src/dropdown_menu/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::DropdownMenuMotion;",
        "pub struct DropdownMenuMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "DropdownMenu motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn dropdown_menu_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/dropdown_menu/motion.rs");
    let view_source = load_source("src/dropdown_menu/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DropdownMenuMotion) -> DropdownMenuMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "DropdownMenu motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::dropdown_menu::motion::sanitize_motion(motion);"),
        "DropdownMenu view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn dropdown_menu_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn dropdown_menu() -> AnyView",
        "title=\"DropdownMenu\"",
        "slug=\"dropdown-menu\"",
        "description=\"Button trigger that opens a Menu in a Popover with baseline-style root attrs, controlled/uncontrolled state, and persistent-open action handling.\"",
        "<Playground title=\"Default\" code_signal=code>",
        "<Playground title=\"Controlled + Persistent Open\" code_signal=controlled_code>",
        "<Playground title=\"Disabled + Empty\" code_signal=disabled_code>",
        "<DropdownMenu",
        "open=controlled_open",
        "disabled=true",
        "close_on_action=false",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for dropdown-menu coverage.",
        );
    }
}

#[test]
fn dropdown_menu_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-dropdown\".to_string()",
        "items=default_items",
        "item_kinds=vec![",
        "on_action=on_action",
        "\"last: \"",
        "id_base=\"docs-dropdown-controlled\".to_string()",
        "items=controlled_items",
        "on_open_change=on_open_change",
        "disabled_indices=vec![1]",
        "\"open: \"",
        "\"close_on_action: false (select keeps popover open)\"",
        "id_base=\"docs-dropdown-disabled\".to_string()",
        "items=disabled_items",
        "id_base=\"docs-dropdown-empty\".to_string()",
        "items=empty_items",
    ] {
        assert!(
            source.contains(needle),
            "dropdown-menu docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn dropdown_menu_minimal_feature_gate_keeps_dependency_chain_and_module_paths_wired() {
    let cargo_toml = load_source("Cargo.toml");
    let view_source = load_source("src/dropdown_menu/view.rs");
    let menu_motion_source = load_source("src/menu/motion.rs");

    assert!(
        cargo_toml.contains(
            "component-dropdown_menu = [\"component-button\", \"component-menu\", \"component-popover\"]"
        ),
        "component-dropdown_menu must depend on button/menu/popover for minimal-feature compilation."
    );

    for needle in [
        "use crate::button::{Button, ButtonSize, ButtonVariant};",
        "use crate::menu::Menu;",
        "use crate::popover::Popover;",
    ] {
        assert!(
            view_source.contains(needle),
            "DropdownMenu view should use module imports (`{needle}`) instead of fragile root re-exports."
        );
    }

    assert!(
        menu_motion_source.contains("use crate::active_highlight::ActiveHighlightMotion;"),
        "menu motion should import ActiveHighlightMotion from module path to keep feature-gated builds stable."
    );
}

#[test]
fn dropdown_menu_docs_include_interactive_playground_contract_panels() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"crates/ui-components/src/dropdown_menu/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "dropdown-menu docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn dropdown_menu_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("src/dropdown_menu/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "dropdown-menu README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"dropdown-menu\" => Some(DROPDOWN_MENU_README_MD)"),
        "docs shell should map dropdown-menu slug to DROPDOWN_MENU_README_MD.",
    );
}
