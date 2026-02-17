use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn aspect_ratio_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/aspect_ratio/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AspectRatio internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn aspect_ratio_uses_logic_state_model() {
    let logic_source = load_source("src/aspect_ratio/logic.rs");
    let view_source = load_source("src/aspect_ratio/view.rs");

    for needle in [
        "pub enum AspectRatioPreset",
        "pub enum AspectRatioRadius",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "AspectRatio logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(AspectRatioStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "AspectRatio view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn aspect_ratio_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/aspect_ratio/view.rs");

    for attr in [
        "data-slot=\"aspect-ratio\"",
        "data-ratio=move || state.get().ratio_attr",
        "data-radius=move || state.get().radius_attr",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-fill=move || state.get().is_fill.then_some(\"true\")",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"aspect-ratio-inner\"",
    ] {
        assert!(
            source.contains(attr),
            "AspectRatio should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn aspect_ratio_styles_include_ratio_and_frame_markers() {
    let source = load_source("src/aspect_ratio/styles.rs");

    for selector in [
        ".ui-aspect-ratio--ratio-square",
        ".ui-aspect-ratio[data-ratio=\"video\"]",
        ".ui-aspect-ratio--ratio-ultra-wide",
        ".ui-aspect-ratio--radius-md",
        ".ui-aspect-ratio[data-radius=\"full\"]",
        ".ui-aspect-ratio--bordered",
        ".ui-aspect-ratio[data-bordered=\"true\"]",
        ".ui-aspect-ratio--fill .ui-aspect-ratio__inner",
        ".ui-aspect-ratio[data-fill=\"true\"] .ui-aspect-ratio__inner > *",
        ".ui-aspect-ratio--custom-class",
        ".ui-aspect-ratio[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "AspectRatio styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn aspect_ratio_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn aspect_ratio() -> AnyView",
        "title=\"AspectRatio\"",
        "slug=\"aspect-ratio\"",
        "Playground title=\"Ratio Presets\"",
        "Playground title=\"Bordered + Fill + Custom Aria/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout-extra docs page should contain `{needle}` for AspectRatio.",
        );
    }
}

#[test]
fn aspect_ratio_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Ratio Presets\"",
        "ratio=AspectRatioPreset::Square",
        "ratio=AspectRatioPreset::Video",
        "ratio=AspectRatioPreset::Portrait",
        "radius=AspectRatioRadius::Sm",
        "radius=AspectRatioRadius::Md",
        "fill=true",
        "title=\"Bordered + Fill + Custom Aria/Class\"",
        "ratio=AspectRatioPreset::UltraWide",
        "radius=AspectRatioRadius::Lg",
        "bordered=true",
        "aria_label=\"Release trailer preview\".to_string()",
        "class_name=\"docs-aspect-ratio-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "aspect-ratio docs playgrounds should contain `{needle}`.",
        );
    }
}
