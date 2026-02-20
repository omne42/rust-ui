use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn color_swatch_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-swatch/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorSwatch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_swatch_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");

    for needle in [
        "pub enum ColorSwatchSize",
        "pub enum ColorSwatchRounding",
        "pub enum ColorSwatchShape",
        "pub enum ColorSwatchAlpha",
        "pub fn sanitize_color_value(",
        "pub fn resolve_alpha(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn compose_inline_style(",
        "pub fn normalize_is_bordered(",
        "pub fn normalize_is_decorative(",
        "pub enum ColorSwatchBoolSource",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorSwatch logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::sanitize_color_value(color)",
        "logic::resolve_alpha(color.as_deref())",
        "logic::normalize_aria_label(aria_label, color_name, color.as_deref(), alpha)",
        "logic::normalize_is_bordered(is_bordered)",
        "logic::normalize_is_decorative(is_decorative)",
        "logic::resolve_state(ColorSwatchStateInput {",
        "logic::compose_class_name(class_name, state)",
        "let locale = locale_attrs(lang, dir);",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_swatch_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/color-swatch/src/view.rs");

    for attr in [
        "data-slot=\"color-swatch\"",
        "data-size=state.size_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-alpha=state.alpha_attr",
        "data-state=state.data_state_attr",
        "data-has-color=state.has_color.then_some(\"true\")",
        "data-bordered=state.is_bordered.then_some(\"true\")",
        "data-bordered-source=bordered_source.as_attr()",
        "data-decorative-source=decorative_source.as_attr()",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
        "data-ui-schema-version=\"1\"",
        "data-ui-intent=\"color-preview\"",
        "data-ui-action=\"render\"",
        "data-ui-state=state.data_state_attr",
        "data-ui-source=state.aria_source_attr",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
        "data-slot=\"color-swatch-checker\"",
        "data-slot=\"color-swatch-sample\"",
        "data-slot=\"color-swatch-slash\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorSwatch should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_swatch_styles_include_size_shape_and_alpha_markers() {
    let source = load_source("../../components/color-swatch/src/styles.rs");

    for selector in [
        ".ui-color-swatch--size-xs",
        ".ui-color-swatch[data-size=\"lg\"]",
        ".ui-color-swatch--rounding-default",
        ".ui-color-swatch[data-rounding=\"full\"]",
        ".ui-color-swatch--shape-wide",
        ".ui-color-swatch[data-shape=\"square\"]",
        ".ui-color-swatch--bordered",
        ".ui-color-swatch[data-bordered=\"true\"]",
        ".ui-color-swatch--alpha-translucent .ui-color-swatch__checker",
        ".ui-color-swatch[data-alpha=\"transparent\"] .ui-color-swatch__sample",
        ".ui-color-swatch--alpha-none .ui-color-swatch__slash",
        ".ui-color-swatch--custom-class",
        ".ui-color-swatch[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorSwatch styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_swatch_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn color_swatch() -> AnyView",
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Rounded Large + Custom Label/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch docs page should contain `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn color_swatch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_source_path=\"components/color-swatch/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "size=ColorSwatchSize::Xs",
        "size=ColorSwatchSize::Sm",
        "ColorSwatchSize::Md",
        "size=ColorSwatchSize::Lg",
        "rounding=ColorSwatchRounding::Full",
        "color_name=\"Brand blue / 35%\".to_string()",
        "shape=ColorSwatchShape::Wide",
        "rounding=ColorSwatchRounding::Default",
        "class_name=\"docs-color-swatch-custom\".to_string()",
        "color_name=\"No fill\".to_string()",
        "is_bordered=true",
        "is_decorative=is_decorative",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch docs playground should contain `{needle}`.",
        );
    }
}
