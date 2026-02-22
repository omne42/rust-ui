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
fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path).is_some()
}
#[test]
fn color_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color/area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorArea internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_area_uses_state_primitives_headless_logic_layers() {
    let logic_source = load_source("src/color/area/logic.rs");
    let view_source = load_source("src/color/area/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_area.rs");
    let headless_source = load_source("../ui-headless/src/color_area.rs");

    for needle in [
        "pub use ui_state_primitives::color_area::{",
        "pub struct ColorAreaDisableInput",
        "pub struct ColorAreaValueAxis",
        "pub struct ColorAreaRootInput",
        "pub struct ColorAreaRootState",
        "pub fn normalize_disable_state(",
        "pub fn normalize_value_axis(",
        "pub fn normalize_default_value(",
        "pub fn normalize_root_state(",
        "pub fn compose_class_name(",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorArea logic should include `{needle}` for centralized assembly."
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub const DEFAULT_X_AXIS_LABEL",
        "pub const DEFAULT_Y_AXIS_LABEL",
        "pub struct ColorAreaStateInput",
        "pub struct ColorAreaState",
        "pub fn sanitize_step(",
        "pub fn sanitize_grid_size(",
        "pub fn clamp_value(",
        "pub fn sanitize_preview_color(",
        "pub fn value_from_cell(",
        "pub fn move_value_by_delta(",
        "pub fn parse_axis_percent(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives color_area should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ColorAreaOptions",
        "pub struct ColorAreaRootAttrs",
        "pub struct ColorAreaGridAttrs",
        "pub struct ColorAreaAxisAttrs",
        "pub struct ColorAreaHandlers",
        "pub struct ColorAreaContract",
        "pub fn use_color_area(options: ColorAreaOptions) -> ColorAreaContract",
        "labeled_group_attrs",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless color_area should include `{needle}`."
        );
    }

    for needle in [
        "use_controllable_state",
        "let value_axis = logic::normalize_value_axis(value.is_some());",
        "let default_value = logic::normalize_default_value(default_value);",
        "logic::normalize_root_state(ColorAreaRootInput {",
        "use_color_area(ColorAreaOptions",
        "data-value-control-mode=value_axis.control_mode.as_attr()",
        "data-value-source=value_axis.value_source.as_attr()",
        "data-ui-schema=move || logic::resolve_agent_contract(root.get().state, value_axis).schema_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorArea view should integrate logic+headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn color_area_headless_and_primitives_are_exported_and_boundary_safe() {
    let primitive_lib = load_source("../ui-state-primitives/src/lib.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_area.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_source = load_source("../ui-headless/src/color_area.rs");

    assert!(
        primitive_lib.contains("pub mod color_area;"),
        "ui-state-primitives should export color_area module."
    );
    assert!(
        headless_lib.contains("pub mod color_area;"),
        "ui-headless should export color_area module."
    );
    assert!(
        headless_lib.contains("use_color_area"),
        "ui-headless should re-export color_area contract."
    );

    for forbidden in ["use leptos", "web_sys", "NodeRef", "view! {", "on:click"] {
        assert!(
            !primitive_source.contains(forbidden),
            "color_area primitive must stay framework/DOM-free; found `{forbidden}`."
        );
    }

    for forbidden in ["ui-color-area", "view! {", "NodeRef<", "style.set_property"] {
        assert!(
            !headless_source.contains(forbidden),
            "color_area headless layer should stay semantic-only; found `{forbidden}`."
        );
    }
}

#[test]
fn color_area_exposes_baseline_style_and_agent_data_markers() {
    let source = load_source("src/color/area/view.rs");

    for attr in [
        "const SLOT_COLOR_AREA: &str = \"color-area\";",
        "const SLOT_COLOR_AREA_GRID: &str = \"color-area-grid\";",
        "const SLOT_COLOR_AREA_CELL: &str = \"color-area-cell\";",
        "const SLOT_COLOR_AREA_AXIS_X: &str = \"color-area-axis-x\";",
        "const SLOT_COLOR_AREA_AXIS_Y: &str = \"color-area-axis-y\";",
        "data-slot=SLOT_COLOR_AREA",
        "data-state=move ||",
        "data-disabled-source=move || root.get().disabled_source_attr.as_attr()",
        "data-grid-size=move ||",
        "data-value-x=move ||",
        "data-value-y=move ||",
        "data-selected-col=move ||",
        "data-selected-row=move ||",
        "data-label-source=move ||",
        "data-aria-source=move ||",
        "data-class-source=move ||",
        "data-x-axis-source=move ||",
        "data-y-axis-source=move ||",
        "data-value-control-mode=value_axis.control_mode.as_attr()",
        "data-value-source=value_axis.value_source.as_attr()",
        "data-ui-schema=move || logic::resolve_agent_contract(root.get().state, value_axis).schema_attr",
        "data-ui-stream-support=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_support_attr",
        "data-ui-stream-fallback=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_fallback_attr",
        "data-ui-stream-mode=move || logic::resolve_agent_contract(root.get().state, value_axis).stream_mode_attr",
        "data-ui-output-status=move || logic::resolve_agent_contract(root.get().state, value_axis).output_status_attr",
        "data-ui-intent=move || logic::resolve_agent_contract(root.get().state, value_axis).intent_attr",
        "data-ui-action=move || logic::resolve_agent_contract(root.get().state, value_axis).action_attr",
        "data-ui-state=move || logic::resolve_agent_contract(root.get().state, value_axis).state_attr",
        "data-ui-source=move || logic::resolve_agent_contract(root.get().state, value_axis).source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ColorArea should expose `{attr}` as stable semantic contract markers."
        );
    }
}

#[test]
fn color_area_api_naming_prefers_is_disabled_and_keeps_compatibility() {
    let source = load_source("src/color/area/view.rs");
    let logic_source = load_source("src/color/area/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "ColorAreaDisableInput",
        "normalize_disable_state",
        "ColorAreaDisabledSourceAttr",
    ] {
        assert!(
            logic_source.contains(needle) || source.contains(needle),
            "color_area should keep migration-safe disabled naming contract `{needle}`."
        );
    }

    assert!(
        docs_source.contains("is_disabled=true"),
        "docs color-area example should prefer `is_disabled` naming."
    );
}

#[test]
fn color_area_default_value_and_state_normalization_are_centralized_in_logic() {
    let source = load_source("src/color/area/view.rs");
    let logic_source = load_source("src/color/area/logic.rs");

    for needle in [
        "pub fn normalize_default_value(default_value: Option<(f32, f32)>) -> (f32, f32)",
        "pub fn normalize_root_state(",
        "pub fn normalize_label_with_fallback(",
        "pub fn normalize_aria_label_with_fallback(",
        "pub fn normalize_axis_label_with_fallback(",
    ] {
        assert!(
            logic_source.contains(needle),
            "color_area logic should define `{needle}` as the single normalization source."
        );
    }

    assert!(
        source.contains("let default_value = logic::normalize_default_value(default_value);"),
        "view should consume normalized default value from logic layer."
    );
    assert!(
        !source.contains("default_value.unwrap_or("),
        "view should not keep scattered default-value fallback branches."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn color_area_a11y_i18n_and_locale_are_headless_driven() {
    let source = load_source("src/color/area/view.rs");
    let i18n_source = load_source("../ui-headless/src/i18n/common.rs");
    let headless_source = load_source("../ui-headless/src/color_area.rs");

    for needle in [
        "use_ui_i18n",
        "CommonStrings",
        "fallback_label: common.color_area_label.as_ref().into()",
        "fallback_aria_label: common.color_area_aria_label.as_ref().into()",
        "fallback_x_axis_label: common.color_area_x_axis_label.as_ref().into()",
        "fallback_y_axis_label: common.color_area_y_axis_label.as_ref().into()",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<crate::color::area::A11yDirection>",
    ] {
        assert!(
            source.contains(needle),
            "color_area view should include headless/i18n locale hookup `{needle}`."
        );
    }

    for field in [
        "pub color_area_label: Arc<str>",
        "pub color_area_aria_label: Arc<str>",
        "pub color_area_x_axis_label: Arc<str>",
        "pub color_area_y_axis_label: Arc<str>",
        "color_area_label: \"Color area\".into(),",
        "color_area_aria_label: \"Color area\".into(),",
        "color_area_x_axis_label: \"Saturation\".into(),",
        "color_area_y_axis_label: \"Lightness\".into(),",
    ] {
        assert!(
            i18n_source.contains(field),
            "common i18n bundle should include color-area string `{field}`."
        );
    }

    assert!(
        headless_source.contains("labeled_group_attrs"),
        "color_area headless contract should use labeled_group_attrs for A11y role/locale mapping."
    );
}

#[test]
fn color_area_styles_and_motion_use_explicit_state_contracts() {
    let style_source = load_source("src/color/area/styles.rs");
    let motion_source = load_source("src/color/area/motion.rs");

    for selector in [
        ".ui-color-area",
        ".ui-color-area__grid",
        ".ui-color-area__cell",
        ".ui-color-area__cell[data-selected=\"true\"] .ui-color-area__thumb",
        ".ui-color-area--with-preview",
        ".ui-color-area[data-has-preview=\"true\"]",
        ".ui-color-area--disabled",
        ".ui-color-area[data-disabled=\"true\"]",
        ".ui-color-area--custom-class",
        ".ui-color-area[data-custom-class=\"true\"]",
        "--ui-color-area-preview-color",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            style_source.contains(selector),
            "ColorArea styles should include `{selector}` as explicit state contract."
        );
    }

    for needle in [
        "pub struct ColorAreaMotion",
        "pub fn sanitize_motion(motion: ColorAreaMotion) -> ColorAreaMotion",
        "pub fn source_attr(motion: ColorAreaMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: ColorAreaMotion) -> String",
        "--ui-color-area-motion-duration",
    ] {
        assert!(
            motion_source.contains(needle),
            "color_area motion module should include `{needle}`."
        );
    }

    for forbidden in ["ui_motion::spring::SpringAnimator", "MotionKeyframe::new()"] {
        assert!(
            !motion_source.contains(forbidden),
            "color_area motion should avoid local runtime engine `{forbidden}`."
        );
    }
}

#[test]
fn color_area_docs_page_covers_playgrounds_and_state_matrix() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_area() -> AnyView",
        "title=\"ColorArea\"",
        "slug=\"color-area\"",
        "title=\"Controlled Grid Selection\"",
        "title=\"Disabled + Custom Grid + Custom Class\"",
        "id_base=\"docs-color-area-basic\".to_string()",
        "value=value.into()",
        "on_value_change=on_value_change",
        "id_base=\"docs-color-area-disabled\".to_string()",
        "grid_size=15",
        "step=0.05",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "color-area docs page should include `{needle}` for state matrix coverage."
        );
    }
}

#[test]
fn color_area_tree_shaking_feature_gates_keep_css_and_module_conditional() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let cargo_source = load_source("Cargo.toml");

    let grouped_gate = "#[cfg(any(\n    feature = \"component-color_area\",";
    assert!(
        lib_source.contains(grouped_gate),
        "ui lib should gate grouped color domain by feature `{grouped_gate}`."
    );
    assert!(
        !lib_source.contains("pub mod color_area;"),
        "no-compat migration should avoid flat `color_area` module export."
    );
    assert!(
        lib_source.contains("pub use color::area::ColorArea;"),
        "ui all-components export should re-export grouped color-area API."
    );

    assert!(
        css_source.contains("#[cfg(feature = \"component-color_area\")]\n    out.push_str(crate::color::area::styles::CSS);"),
        "css aggregation should gate color-area css by feature."
    );
    assert!(
        cargo_source.contains("component-color_area = [\"dep:ui-color-area\"]"),
        "Cargo features should expose component-color_area minimal feature."
    );
}

#[test]
fn color_area_forbids_inner_html_and_platform_type_leakage() {
    let view_source = load_source("src/color/area/view.rs");
    let logic_source = load_source("src/color/area/logic.rs");

    for forbidden in ["inner_html", "web_sys", "wasm_bindgen", "NodeRef<"] {
        assert!(
            !view_source.contains(forbidden),
            "color_area view should not leak unsafe/platform-only path `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "color_area logic should stay platform-agnostic; found `{forbidden}`."
        );
    }
}

#[test]
fn color_area_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2 = load_source("src/color/area/check2.md");

    for needle in [
        "### 7. 测试与文档（验证闭环）",
        "E2E 选择器稳定",
        "WASM 场景有稳定等待策略",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should document e2e selector/stable-wait governance `{needle}`."
        );
    }
}

#[test]
fn color_area_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_area_contract.spec.mjs");

    for needle in [
        "const COLOR_AREA_PAGE = \"/#/components/color-area\";",
        "[data-component=\"color-area\"] [data-slot=\"color-area\"]",
        "body:not(:has(#boot))",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-area e2e should include selector/wait contract `{needle}`."
        );
    }
}

#[test]
fn color_area_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2 = load_source("src/color/area/check2.md");

    for needle in ["关键流程纳入可重复回归集合", "高风险路径", "键盘"] {
        assert!(
            check2.contains(needle),
            "check2 should include repeatable-flow governance `{needle}`."
        );
    }
}

#[test]
fn color_area_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_area_contract.spec.mjs");

    for needle in [
        "ArrowRight",
        "ArrowUp",
        "await page.reload();",
        "data-value-x",
        "data-selected-col",
        "data-selected-row",
        "data-ui-output-status",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-area e2e should cover repeatable key flow contract `{needle}`."
        );
    }
}

#[test]
fn color_area_e2e_high_risk_paths_cover_keyboard_and_disabled_branches() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_area_contract.spec.mjs");

    for needle in [
        "data-disabled=\"true\"",
        "aria-disabled",
        "await root.focus();",
        "page.keyboard.press",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-area e2e should include high-risk path marker `{needle}`."
        );
    }
}

#[test]
fn color_area_has_e2e_script_and_spec_files() {
    assert!(
        path_exists("../../e2e/tests/docs_app_color_area_contract.spec.mjs"),
        "color-area e2e spec should exist."
    );
    assert!(
        path_exists("../../components/color-area/scripts/check-ui-e2e-color-area.sh"),
        "color-area e2e check script should exist."
    );
}
