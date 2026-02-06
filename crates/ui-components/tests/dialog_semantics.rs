use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn dialog_does_not_expose_logic_module() {
    let source = load_source("src/dialog/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Dialog's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn dialog_composes_overlay_and_labels_it_via_title_id() {
    let source = load_source("src/dialog/view.rs");

    assert!(
        source.contains("<Overlay"),
        "Dialog should compose `Overlay` rather than re-implementing modal semantics."
    );

    for needle in [
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
        "id=move || title_id_attr.get()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should wire `aria-labelledby` via a stable title id (`{needle}`)."
        );
    }
}

#[test]
fn dialog_only_sets_aria_describedby_when_description_is_present() {
    let source = load_source("src/dialog/view.rs");

    assert!(
        source.contains("if view_state.show_description"),
        "Dialog should branch on description presence so `aria-describedby` is only set when it has real content."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "id=move || description_id_attr.get()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should wire `aria-describedby` via a stable description id (`{needle}`)."
        );
    }
}

#[test]
fn dialog_close_button_uses_icon_button_with_aria_label() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "data-slot=\"dialog-close\"",
        "<IconButton",
        "aria_label=close_label",
    ] {
        assert!(
            source.contains(needle),
            "Dialog close button should be accessible and stable (`{needle}`)."
        );
    }
}

#[test]
fn dialog_size_class_is_derived_from_dialog_size_enum() {
    let source = load_source("src/dialog/view.rs");

    assert!(
        source.contains("size.class_name()"),
        "Dialog should map `DialogSize` to a CSS class via `DialogSize::class_name()`."
    );
}
