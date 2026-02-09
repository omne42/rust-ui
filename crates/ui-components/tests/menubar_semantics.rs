use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menubar_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/menubar/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Menubar;"),
        "menubar module should export `Menubar`."
    );
    assert!(
        module_source.contains("pub use logic::MenubarMenu;"),
        "menubar module should export `MenubarMenu`."
    );
    assert!(
        module_source.contains("MenubarMotion"),
        "menubar module should expose a motion alias."
    );
    assert!(
        crate_source.contains("pub use menubar::{Menubar, MenubarMenu, MenubarMotion};"),
        "crate root should re-export menubar contracts."
    );
}

#[test]
fn menubar_uses_logic_state_model() {
    let view_source = load_source("src/menubar/view.rs");
    let logic_source = load_source("src/menubar/logic.rs");

    for needle in [
        "pub struct MenubarMenu",
        "pub struct MenubarMenuResolved",
        "pub struct MenubarStateInput",
        "pub struct MenubarState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_menus(",
        "pub fn sanitize_open_index_for_menus(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn next_enabled_menu_index(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Menubar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let menus = logic::resolve_menus(&id_base.get_value(), menus);",
        "let open_state = overlay_open::use_controllable_state(",
        "let state = Signal::derive(move ||",
        "logic::resolve_state(logic::MenubarStateInput {",
        "let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "Menubar view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn menubar_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "open_index: Option<Signal<Option<usize>>>",
        "default_open_index: Option<usize>",
        "on_open_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should accept `{needle}` for controlled/uncontrolled open index state."
        );
    }
}

#[test]
fn menubar_exposes_keyboard_and_trigger_contracts() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "role=\"menubar\"",
        "role=\"menuitem\"",
        "on:keydown=on_key_down",
        "on:pointerenter=on_pointer_enter",
        "if let Some(focus_strategy) = logic::focus_strategy_for_open_key(&key)",
        "logic::next_enabled_menu_index(menus.get_value().as_ref(), index, 1)",
        "logic::next_enabled_menu_index(menus.get_value().as_ref(), index, -1)",
        "focus_trigger(&trigger_refs, next_index);",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should wire `{needle}` to match menubar keyboard + pointer semantics."
        );
    }
}

#[test]
fn menubar_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "data-slot=\"menubar\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().has_open_menu.then_some(\"true\")",
        "data-closed=move || (!state.get().has_open_menu).then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-menus=move || state.get().has_menus.then_some(\"true\")",
        "data-open-index=move || state.get().open_index.map(|index| index.to_string())",
        "data-menu-count=move || state.get().menu_count.to_string()",
        "data-has-disabled-menus=move || state.get().has_disabled_menus.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(\"true\")",
        "data-placement=move || state.get().placement_attr",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should set `{needle}` so it can be styled/tested with Spectrum-compatible selectors."
        );
    }
}

#[test]
fn menubar_renders_menu_in_popover_with_presence_and_motion() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "motion=motion.popover",
        "is_modal=false",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn menubar_styles_include_open_disabled_and_empty_markers() {
    let source = load_source("src/menubar/styles.rs");

    for needle in [
        ".ui-menubar {",
        ".ui-menubar__trigger {",
        ".ui-menubar--open",
        ".ui-menubar--empty",
        ".ui-menubar__menu[data-disabled=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "Menubar styles should include `{needle}` for stable visual state contracts."
        );
    }
}
