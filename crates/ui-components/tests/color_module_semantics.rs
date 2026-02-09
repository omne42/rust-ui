use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_module_reexports_react_spectrum_color_family() {
    let source = load_source("src/color/mod.rs");

    for needle in [
        "pub use crate::color_area::ColorArea;",
        "pub use crate::color_wheel::ColorWheel;",
        "pub use crate::color_slider::ColorSlider;",
        "pub use crate::color_field::ColorField;",
        "pub use crate::color_swatch::ColorSwatch;",
        "pub use crate::color_picker::ColorPicker;",
        "pub use crate::color_editor::ColorEditor;",
        "pub use crate::color_swatch_picker::ColorSwatchPicker;",
    ] {
        assert!(
            source.contains(needle),
            "color module should expose `{needle}` for react-spectrum color compatibility."
        );
    }
}

#[test]
fn crate_root_registers_color_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod color;"),
        "crate root should include `pub mod color;` for @react-spectrum/color compatibility."
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
