use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_barrel_module_is_removed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/color/mod.rs");
    assert!(
        !path.exists(),
        "color barrel module `src/color/mod.rs` should not exist."
    );
}

#[test]
fn crate_root_does_not_register_color_barrel_module() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("pub mod color;"),
        "crate root should not include `pub mod color;`."
    );
}

#[test]
fn color_compatibility_reuses_forms_and_display_color_docs_playgrounds() {
    let forms_color_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let display_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"ColorField\"",
        "slug=\"color-field\"",
        "title=\"ColorArea\"",
        "slug=\"color-area\"",
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "title=\"ColorWheel\"",
        "slug=\"color-wheel\"",
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
    ] {
        assert!(
            forms_color_source.contains(needle),
            "forms_color docs should contain `{needle}` for color compatibility coverage."
        );
    }

    for needle in [
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
    ] {
        assert!(
            display_extra_source.contains(needle),
            "display_extra docs should contain `{needle}` for color compatibility coverage."
        );
    }
}

#[test]
fn color_module_docs_page_covers_primary_playgrounds() {
    let forms_color_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let display_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn color_field() -> AnyView",
        "pub(super) fn color_area() -> AnyView",
        "pub(super) fn color_slider() -> AnyView",
        "pub(super) fn color_wheel() -> AnyView",
        "pub(super) fn color_picker() -> AnyView",
        "pub(super) fn color_editor() -> AnyView",
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
    ] {
        assert!(
            forms_color_source.contains(needle),
            "forms_color docs should contain `{needle}` for color module coverage.",
        );
    }

    for needle in [
        "pub(super) fn color_swatch() -> AnyView",
        "pub(super) fn color_swatch_picker() -> AnyView",
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"ColorSwatchPicker\"",
        "slug=\"color-swatch-picker\"",
    ] {
        assert!(
            display_extra_source.contains(needle),
            "display docs should contain `{needle}` for color module coverage.",
        );
    }
}

#[test]
fn color_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let forms_color_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let display_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Controlled Grid Selection\"",
        "title=\"Disabled Alpha + Custom Track + Reduced Motion\"",
        "title=\"Controlled Color + Controlled Open\"",
        "title=\"Controlled Color + Controlled Format\"",
        "title=\"Invalid + Disabled + Custom Class\"",
    ] {
        assert!(
            forms_color_source.contains(needle),
            "forms_color playground coverage should contain `{needle}`.",
        );
    }

    for needle in [
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Rounded Large + Custom Label/Class\"",
        "title=\"Basic Selection\"",
        "title=\"Transparency + Disabled + Custom Class\"",
    ] {
        assert!(
            display_extra_source.contains(needle),
            "display color playground coverage should contain `{needle}`.",
        );
    }
}
