use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn command_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/command/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Command;"),
        "command module should export `Command`."
    );
    assert!(
        module_source.contains("CommandMotion"),
        "command module should expose a motion alias."
    );
    assert!(
        crate_source
            .contains("pub use command::{Command, CommandGroup, CommandItem, CommandMotion};"),
        "crate root should re-export command contracts."
    );
}

#[test]
fn command_view_has_keyboard_filter_and_slot_contracts() {
    let source = load_source("src/command/view.rs");

    for needle in [
        "use_listbox(ListBoxOptions",
        "attach_active_highlight_motion",
        "data-slot=\"command\"",
        "data-slot=\"command-input\"",
        "data-slot=\"command-list\"",
        "data-slot=\"command-item\"",
        "role=\"combobox\"",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
    ] {
        assert!(
            source.contains(needle),
            "Command view should include `{needle}` for stable behavior contracts."
        );
    }
}

#[test]
fn command_css_contains_expected_selectors() {
    let css = load_source("src/command/styles.rs");

    for needle in [
        ".ui-command {",
        ".ui-command__input {",
        ".ui-command__option[data-focused=\"true\"]",
        ".ui-command__empty {",
    ] {
        assert!(
            css.contains(needle),
            "Command CSS should include `{needle}` selector."
        );
    }
}
