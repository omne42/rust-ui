use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_drop_zone_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/drop-zone").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_drop_zone_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-drop_zone\")]")
            && lib_source.contains("pub use ui_drop_zone as drop_zone;"),
        "ui-components should re-export the external ui-drop-zone crate as `drop_zone`.",
    );
    assert!(
        cargo_source.contains("component-drop_zone = [\"dep:ui-drop-zone\"]"),
        "component-drop_zone feature should depend on dep:ui-drop-zone after extraction.",
    );
    assert!(
        cargo_source
            .contains("ui-drop-zone = { path = \"../../components/drop-zone\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-drop-zone dependency.",
    );
}

#[test]
fn drop_zone_does_not_expose_logic_module() {
    let source = load_drop_zone_component_source("src/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "DropZone's `logic` module should stay private to avoid leaking DOM/web-sys details into the public API."
    );
}

#[test]
fn drop_zone_uses_headless_hover_and_focus_ring() {
    let source = load_drop_zone_component_source("src/view.rs");

    assert!(
        source.contains("use_focus_ring"),
        "DropZone should use `ui_headless::use_focus_ring` (not `:focus-visible`) to align focus-visible behavior with the global modality provider."
    );

    assert!(
        source.contains("use_hover"),
        "DropZone should use `ui_headless::use_hover` for hovered state."
    );
}

#[test]
fn drop_zone_emits_baseline_style_data_attributes() {
    let source = load_drop_zone_component_source("src/view.rs");

    for attr in [
        "data-slot=\"drop-zone\"",
        "data-motion-source=if motion == DropZoneMotion::default()",
        "data-custom-motion=(motion != DropZoneMotion::default()).then_some(\"true\")",
        "data-hovered",
        "data-focused",
        "data-focus-visible",
        "data-drop-target",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "DropZone should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn drop_zone_has_hidden_focus_button_and_paste_handler() {
    let source = load_drop_zone_component_source("src/view.rs");

    assert!(
        source.contains("class=\"ui-drop-zone__button\""),
        "DropZone should render a visually hidden focusable button to receive focus and clipboard events."
    );

    assert!(
        source.contains("on:paste"),
        "DropZone should support pasting files via `on:paste` on the hidden focus button."
    );
}

#[test]
fn drop_zone_has_spring_driven_highlight_css_var() {
    let source = load_drop_zone_component_source("src/styles.rs");

    assert!(
        source.contains("--ui-drop-zone-highlight"),
        "DropZone styles should define `--ui-drop-zone-highlight` for spring-driven hover/drop-target feedback."
    );
}

#[test]
fn drop_zone_styles_include_motion_marker_selectors() {
    let source = load_drop_zone_component_source("src/styles.rs");

    for selector in [
        ".ui-drop-zone[data-motion-source=\"custom\"]",
        ".ui-drop-zone[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "DropZone styles should include `{selector}` for stable motion marker contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn drop_zone_motion_contract_exposes_default_and_custom_checks() {
    let source = load_drop_zone_component_source("src/motion.rs");

    for needle in [
        "pub struct DropZoneMotion",
        "fn default_motion_uses_expected_drop_zone_contract()",
        "fn supports_custom_drop_zone_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "DropZone motion module should include `{needle}` for baseline-level regression coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn drop_zone_motion_sanitizes_custom_contract_values() {
    let motion_source = load_drop_zone_component_source("src/motion.rs");
    let view_source = load_drop_zone_component_source("src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DropZoneMotion) -> DropZoneMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "DropZone motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "DropZone view should sanitize motion before forwarding to runtime attachment and data markers.",
    );
}

#[test]
fn drop_zone_docs_page_covers_primary_playgrounds() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "pub(super) fn drop_zone() -> AnyView",
        "title=\"DropZone\"",
        "slug=\"drop-zone\"",
        "description=\"Drag-and-drop + paste file ingestion with focus handling.\"",
        "<Playground title=\"Drop / paste\" code_signal=code>",
        "<Playground title=\"Drop / paste with custom motion\" code_signal=motion_code>",
        "<DropZone",
        "label=\"Upload\".to_string()",
        "label=\"Upload (custom motion)\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "files docs page should include `{needle}` for drop_zone primary playground coverage.",
        );
    }
}

#[test]
fn drop_zone_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/files.rs");

    for needle in [
        "let (files, set_files) = signal(Vec::<DroppedFile>::new());",
        "let on_drop_files = Callback::new(move |next: Vec<DroppedFile>| set_files.set(next));",
        "title=\"Drop / paste\"",
        "label=\"Upload\".to_string()",
        "on_drop_files=on_drop_files",
        "\"No files received.\"",
        "title=\"Drop / paste with custom motion\"",
        "label=\"Upload (custom motion)\".to_string()",
        "motion=DropZoneMotion {",
        "hover_scale: 1.015",
        "drop_scale: 1.03",
        "hover_highlight: 0.42",
        "..DropZoneMotion::default()",
        "\"Custom spring scale + highlight tuning.\"",
    ] {
        assert!(
            source.contains(needle),
            "drop_zone docs playgrounds should contain `{needle}`.",
        );
    }
}
