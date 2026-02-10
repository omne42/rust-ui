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

#[test]
fn dialog_emits_spectrum_style_state_and_motion_markers() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "data-slot=\"dialog\"",
        "data-state=move || if open.get() { \"open\" } else { \"closed\" }",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-with-description=view_state.show_description.then_some(\"true\")",
        "data-with-footer=view_state.show_footer.then_some(\"true\")",
        "data-close-visible=view_state.show_close_button.then_some(\"true\")",
        "data-motion-source=if motion == DialogMotion::default()",
        "data-custom-motion=(motion != DialogMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should expose `{needle}` for stable state/motion marker contracts."
        );
    }
}

#[test]
fn dialog_styles_include_motion_marker_selectors() {
    let source = load_source("src/dialog/styles.rs");

    for selector in [
        ".ui-dialog[data-motion-source=\"custom\"]",
        ".ui-dialog[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dialog styles should include `{selector}` as stable motion-marker contracts."
        );
    }
}

#[test]
fn dialog_motion_contract_exposes_default_and_custom_overlay_tests() {
    let source = load_source("src/dialog/motion.rs");

    for needle in [
        "pub struct DialogMotion",
        "pub overlay: crate::overlay::OverlayMotion",
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog motion module should include `{needle}` for HeroUI-level contract coverage."
        );
    }
}
