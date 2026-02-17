use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn surface_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/surface/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Surface internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn surface_uses_logic_state_model() {
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");

    for needle in [
        "pub enum SurfaceTone",
        "pub enum SurfaceElevation",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Surface logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(SurfaceStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn surface_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/surface/view.rs");

    for attr in [
        "data-slot=\"surface\"",
        "data-tone=move || state.get().tone_attr",
        "data-elevation=move || state.get().elevation_attr",
        "data-state=move || state.get().data_state_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-padded=move || state.get().is_padded.then_some(\"true\")",
        "data-plain=move || state.get().is_plain.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Surface should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn surface_styles_include_state_markers() {
    let source = load_source("src/surface/styles.rs");

    for selector in [
        ".ui-surface--tone-default",
        ".ui-surface[data-tone=\"subtle\"]",
        ".ui-surface--elevation-raised",
        ".ui-surface[data-elevation=\"floating\"]",
        ".ui-surface--bordered",
        ".ui-surface[data-bordered=\"true\"]",
        ".ui-surface--padded",
        ".ui-surface[data-state=\"framed\"]",
        ".ui-surface--custom-class",
        ".ui-surface[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Surface styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn surface_docs_page_covers_primary_playgrounds() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    assert!(
        layout_extra.contains("pub(super) fn surface() -> AnyView"),
        "layout_extra should expose surface route entry.",
    );

    for needle in [
        "pub(super) fn surface() -> AnyView",
        "title=\"Surface\"",
        "slug=\"surface\"",
        "description=\"baseline-style foundational container surface with centralized tone/elevation/frame/source contracts and stable data markers.\"",
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
        "<Surface",
    ] {
        assert!(
            docs.contains(needle),
            "layout_extra_surface docs should include `{needle}` for surface primary playground coverage.",
        );
    }
}

#[test]
fn surface_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    for needle in [
        "title=\"Tone + Elevation + Frame\"",
        "tone=SurfaceTone::Default",
        "elevation=SurfaceElevation::Raised",
        "tone=SurfaceTone::Subtle",
        "elevation=SurfaceElevation::Flat",
        "bordered=true",
        "tone=SurfaceTone::Strong",
        "elevation=SurfaceElevation::Floating",
        "padded=false",
        "title=\"Custom Aria + Class\"",
        "aria_label=\"Deployment summary\".to_string()",
        "class_name=\"docs-surface-custom\".to_string()",
        "Verifies custom aria source + custom class merge contracts.",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra_surface docs playgrounds should contain `{needle}` for surface contracts.",
        );
    }
}
