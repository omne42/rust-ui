use std::fs;
use std::path::Path;

#[path = "../../../components/color-picker/test/semantics.rs"]
mod color_picker_local_semantics;

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
fn color_picker_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-picker/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorPicker internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_picker_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub struct ColorPickerIds",
        "pub struct ColorPickerDerivedStateInput",
        "pub fn sanitize_selected_color(",
        "pub fn resolve_default_selected_color(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_selected_color_axis<",
        "pub fn resolve_selected_change_axis<",
        "pub fn resolve_derived_state(",
        "pub fn resolve_ids(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorPicker logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "overlay_open::use_controllable_open_state_traced(",
        "use_presence(open)",
        "logic::resolve_derived_state(logic::ColorPickerDerivedStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<Popover",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_picker_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-picker/src/view.rs");

    for attr in [
        "data-slot=\"color-picker\"",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-motion-source=if motion == ColorPickerMotion::default()",
        "data-custom-motion=move || (motion != ColorPickerMotion::default()).then_some(\"true\")",
        "data-slot=\"color-picker-trigger\"",
        "data-slot=\"color-picker-swatch\"",
        "data-slot=\"color-picker-label\"",
        "data-slot=\"color-picker-value\"",
        "data-slot=\"color-picker-panel\"",
        "data-slot=\"color-picker-content\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorPicker should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_picker_styles_include_open_disabled_and_custom_contracts() {
    let source = load_source("../../components/color-picker/src/styles.rs");

    for selector in [
        ".ui-color-picker",
        ".ui-color-picker__trigger",
        ".ui-color-picker__panel",
        ".ui-color-picker__content",
        ".ui-color-picker--open .ui-color-picker__trigger",
        ".ui-color-picker[data-open=\"true\"] .ui-color-picker__trigger",
        ".ui-color-picker--disabled",
        ".ui-color-picker[data-disabled=\"true\"]",
        ".ui-color-picker--custom-class",
        ".ui-color-picker[data-motion-source=\"custom\"]",
        ".ui-color-picker[data-custom-motion=\"true\"]",
        ".ui-color-picker[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorPicker styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_picker_styles_consume_ui_theme_variables_without_private_token_namespace() {
    let source = load_source("../../components/color-picker/src/styles.rs");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-font-size-150",
        "var(--ui-line-height-150",
        "var(--ui-overlay-panel-min-width, 240px)",
        "var(--ui-radius-md)",
        "var(--ui-shadow-md)",
    ] {
        assert!(
            source.contains(needle),
            "ColorPicker styles should consume ui-theme variables via `{needle}`."
        );
    }

    assert!(
        !source.contains("--ui-color-picker-custom-motion"),
        "ColorPicker should not introduce a component-private token namespace in styles.rs."
    );
}

#[test]
fn color_picker_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ColorPickerMotion;",
        "pub struct ColorPickerMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ColorPicker motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn color_picker_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ColorPickerMotion) -> ColorPickerMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorPicker motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::color_picker::motion::sanitize_motion(motion);"),
        "ColorPicker view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn color_picker_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled Color + Controlled Open\"",
        "title=\"Disabled + Default Open + Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "color-picker docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_picker_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "id_base=\"docs-color-picker-hello\".to_string()",
        "<ColorPicker id_base=\"docs-color-picker-hello\".to_string()>",
        "<Playground title=\"Controlled Color + Controlled Open\" code_signal=basic_code>",
        "id_base=\"docs-color-picker-basic\".to_string()",
        "selected_color=selected_color_signal",
        "on_selected_change=on_selected_change",
        "open=open_signal",
        "on_open_change=on_open_change",
        "<ColorSwatchPicker",
        "<Playground title=\"Disabled + Default Open + Custom Class\" code_signal=states_code>",
        "id_base=\"docs-color-picker-disabled\".to_string()",
        "default_selected_color=\"#0ea5e9\".to_string()",
        "is_disabled=true",
        "class_name=\"docs-color-picker-custom\".to_string()",
        "id_base=\"docs-color-picker-open\".to_string()",
        "default_selected_color=\"#8b5cf6\".to_string()",
        "default_open=true",
    ] {
        assert!(
            source.contains(needle),
            "color-picker docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_picker_feature_dependency_chain_supports_minimal_component_builds() {
    let cargo_toml = load_source("Cargo.toml");

    for needle in [
        "component-color_picker = [\"component-color_swatch\", \"component-popover\"]",
        "component-color_swatch = [\"component-illustrated_message\"]",
    ] {
        assert!(
            cargo_toml.contains(needle),
            "ColorPicker feature dependency chain should include `{needle}` for minimal-feature builds."
        );
    }
}

#[test]
fn color_picker_view_mounts_locale_and_headless_a11y_contracts() {
    let source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "popup_trigger_attrs(",
        "overlay_dialog_attrs(",
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
        "trigger_aria.handlers.press.on_key_down.run(key)",
        "trigger_aria.handlers.press.on_key_up.run(key)",
        "ui_headless::aria_controls_when_open(open, panel_id.get_value())",
        "aria-haspopup=\"dialog\"",
        "role=\"dialog\"",
    ] {
        assert!(
            source.contains(needle),
            "ColorPicker view should include `{needle}` for locale/a11y contract coverage."
        );
    }
}

#[test]
fn color_picker_value_axis_exposes_canonical_triplet_with_legacy_alias_fallbacks() {
    let source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "let selected_color = logic::resolve_selected_color_axis(value, selected_color);",
        "logic::resolve_default_selected_color(default_value, default_selected_color);",
        "let on_selected_change = logic::resolve_selected_change_axis(on_value_change, on_selected_change);",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
        "logic::resolve_derived_state(logic::ColorPickerDerivedStateInput {",
    ] {
        assert!(
            source.contains(needle),
            "ColorPicker value axis contract should include `{needle}`.",
        );
    }

    assert!(
        !source.contains("default_value.or(default_selected_color)"),
        "ColorPicker should keep default value priority normalization in logic.rs.",
    );
    assert!(
        !source.contains("has_selection: selected_color.get().is_some()"),
        "ColorPicker should keep state derivation in logic.rs.",
    );
    assert!(
        !source.contains("#[prop(optional)] state:"),
        "ColorPicker should not require an internal state object prop for baseline usage.",
    );
}

#[test]
fn color_picker_discrete_axes_are_type_constrained_with_enums() {
    let source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "#[prop(optional)] placement: PopoverPlacement",
        "#[prop(optional)] swatch_size: ColorSwatchSize",
        "#[prop(optional)] swatch_rounding: ColorSwatchRounding",
        "#[prop(optional)] swatch_shape: ColorSwatchShape",
    ] {
        assert!(
            source.contains(needle),
            "ColorPicker discrete axis should use enum type `{needle}`.",
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !source.contains(forbidden),
            "ColorPicker should not expose free-form discrete string axis `{forbidden}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn color_picker_tree_shaking_boundaries_stay_feature_gated() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"component-color_picker\")]",
        "#[path = \"color/picker/mod.rs\"]\npub mod color_picker;",
        "pub use color_picker::{ColorPicker, ColorPickerMotion};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib boundary should include `{needle}` for ColorPicker feature gating."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_picker\")]",
        "out.push_str(crate::color::picker::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css boundary should include `{needle}` for ColorPicker feature gating."
        );
    }
}

#[test]
fn color_picker_e2e_contract_uses_semantic_selectors_and_settled_waits() {
    let rel = "../../e2e/tests/docs_app_color_picker_contract.spec.mjs";
    assert!(
        path_exists(rel),
        "color-picker E2E contract file should exist at `{rel}`."
    );

    let source = load_source(rel);
    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"color-picker\"]",
        "#docs-color-picker-basic",
        "data-slot=\"color-picker\"",
        "data-slot=\"color-picker-trigger\"",
        "data-slot=\"color-swatch-picker-option\"",
    ] {
        assert!(
            source.contains(needle),
            "color-picker E2E contract should include semantic selector/wait marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source() {
    let source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");

    for needle in [
        "page.keyboard.press(\"Enter\")",
        "page.keyboard.press(\"Escape\")",
        "await page.reload();",
        "Show code|Hide code",
        "data-copyable",
        "Copy to clipboard",
    ] {
        assert!(
            source.contains(needle),
            "color-picker E2E contract should include `{needle}` for key-flow and source-copy coverage.",
        );
    }
}

#[test]
fn color_picker_check2_marks_component_governance_complete() {
    let check2_source = load_source("../../components/color-picker/src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui` 定义",
        "- [x] API 命名契约统一",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] 语义测试优先",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`ColorPicker` 当前仅包含同步状态轴",
        "`ColorPicker` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_picker/check2.md should pin completion marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("../../components/color-picker/src/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_picker/check2.md should mark anti-pattern guard `{needle}` as complete.",
        );
    }
}

#[test]
fn color_picker_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("../../components/color-picker/src/check2.md");

    for needle in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_picker/check2.md should keep final merge-gate marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("../../components/color-picker/src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "ColorPicker check2.md should not keep unchecked checklist items after completion."
    );
}
