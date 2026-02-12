use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_slider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color_slider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorSlider internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_slider_uses_logic_state_model() {
    let logic_source = load_source("src/color_slider/logic.rs");
    let view_source = load_source("src/color_slider/view.rs");

    for needle in [
        "pub enum ColorSliderChannel",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn sanitize_bounds(",
        "pub fn sanitize_step(",
        "pub fn sanitize_value(",
        "pub fn sanitize_track_color(",
        "pub fn compose_inline_style(",
        "pub fn format_channel_value(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorSlider logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "slider_motion::attach_motion(root_ref, visual_percent, motion)",
        "logic::resolve_state(ColorSliderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSlider view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_slider_exposes_spectrum_style_data_markers() {
    let source = load_source("src/color_slider/view.rs");

    for attr in [
        "data-slot=\"color-slider\"",
        "data-state=move || state.get().data_state_attr",
        "data-channel=move || state.get().channel_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-track-source=move || state.get().track_source_attr",
        "data-slot=\"color-slider-label\"",
        "data-slot=\"color-slider-value\"",
        "data-slot=\"color-slider-input\"",
        "data-slot=\"color-slider-track\"",
        "data-slot=\"color-slider-fill\"",
        "data-slot=\"color-slider-thumb\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorSlider should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn color_slider_styles_include_channel_and_custom_contracts() {
    let source = load_source("src/color_slider/styles.rs");

    for selector in [
        ".ui-color-slider",
        ".ui-color-slider__track",
        ".ui-color-slider__thumb",
        ".ui-color-slider--channel-hue",
        ".ui-color-slider[data-channel=\"alpha\"] .ui-color-slider__track::before",
        ".ui-color-slider--disabled",
        ".ui-color-slider[data-disabled=\"true\"]",
        ".ui-color-slider--track-custom",
        ".ui-color-slider[data-track-source=\"custom\"]",
        ".ui-color-slider--custom-class",
        ".ui-color-slider[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorSlider styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_slider_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_slider() -> AnyView",
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "title=\"Controlled Hue Channel\"",
        "title=\"Disabled Alpha + Custom Track + Reduced Motion\"",
    ] {
        assert!(
            source.contains(needle),
            "color-slider docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_slider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Controlled Hue Channel\" code=basic_code>",
        "id_base=\"docs-color-slider-hue\".to_string()",
        "channel=ColorSliderChannel::Hue",
        "value=hue.into()",
        "on_value_change=on_hue_change",
        "<Playground title=\"Disabled Alpha + Custom Track + Reduced Motion\" code=states_code>",
        "id_base=\"docs-color-slider-alpha\".to_string()",
        "channel=ColorSliderChannel::Alpha",
        "disabled=true",
        "id_base=\"docs-color-slider-custom\".to_string()",
        "channel=ColorSliderChannel::Blue",
        "track_start_color=\"#0f172a\".to_string()",
        "track_end_color=\"#38bdf8\".to_string()",
        "motion=reduced_motion",
        "class_name=\"docs-color-slider-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-slider docs playground should contain `{needle}`.",
        );
    }
}
