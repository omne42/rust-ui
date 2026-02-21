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
fn checkbox_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/checkbox_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CheckboxField internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_uses_logic_state_model() {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let view_source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "pub struct CheckboxFieldStateInput",
        "pub struct CheckboxFieldState",
        "CheckboxFieldStatus",
    ] {
        assert!(
            mod_source.contains(needle),
            "CheckboxField module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub use ui_state_primitives::checkbox::{",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "resolve_checked_axis",
        "resolve_checked_change_handler_source",
        "resolve_status",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxField logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let resolved_content = logic::resolve_content(logic::CheckboxFieldContentInput {",
        "let render_state = Memo::new(move |_| {",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxField view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_composes_checkbox_with_label_slot() {
    let source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "<Checkbox",
        "is_checked=Some(checked)",
        "on_checked_change=on_checked_change",
        "is_disabled=Some(disabled)",
        "variant=checkbox_affordance.variant",
        "class_name=checkbox_affordance.class_name",
        "data-slot=\"checkbox-field-label\"",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxField should compose Checkbox with stable contracts (`{needle}`)."
        );
    }
}

#[test]
fn checkbox_field_api_naming_contract_prefers_is_on_default_prefixes_with_alias_migration() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let readme_source = load_source("src/checkbox_field/README.md");

    for needle in [
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] set_checked: Option<WriteSignal<bool>>",
        "let resolved_content = logic::resolve_content(logic::CheckboxFieldContentInput {",
        "let checked_control = logic::resolve_checked_control(",
        "let checkbox_affordance = logic::resolve_checkbox_affordance(",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxField API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct CheckboxFieldContentInput {",
        "pub struct CheckboxFieldContent {",
        "pub fn resolve_content(",
        "CheckboxControlMode",
        "CheckboxCheckedAxisInput",
        "CheckboxCheckedValueSource",
        "CheckboxChangeHandlerSource",
        "CheckboxFieldStatus",
        "resolve_status",
        "pub struct CheckboxFieldRenderStateInput {",
        "pub struct CheckboxFieldRenderState {",
        "pub struct CheckboxFieldAffordance {",
        "pub fn resolve_checkbox_affordance(",
        "pub fn resolve_render_state(",
        "pub fn resolve_checked_control(",
        "pub fn normalize_is_disabled(",
        "pub fn normalize_is_invalid(",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxField logic should centralize naming normalization via `{needle}`."
        );
    }

    assert!(
        readme_source.contains("## 命名兼容与迁移"),
        "CheckboxField README should document naming compatibility and migration path."
    );
    assert!(
        !view_source.contains("unwrap_or_default()"),
        "CheckboxField view should not contain fallback default branches; defaults must be centralized in logic.rs."
    );
    assert!(
        !view_source.contains("logic::resolve_state(CheckboxFieldStateInput {"),
        "CheckboxField view should not rebuild state-machine mapping outside logic.rs."
    );
    assert!(
        !logic_source.contains("pub enum CheckedControlMode {"),
        "CheckboxField logic should consume control mode from ui-state-primitives instead of duplicating it."
    );
}

#[test]
fn checkbox_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/checkbox_field/view.rs");

    for attr in [
        "data-slot=\"checkbox-field\"",
        "data-state=move || render_state.get().state.state_attr",
        "data-tone=move || render_state.get().state.tone_attr",
        "data-indicator-placement=move || render_state.get().state.indicator_placement_attr",
        "data-description=move || render_state.get().state.description_attr",
        "data-label-source=move || render_state.get().state.label_source_attr",
        "data-aria-source=move || render_state.get().state.aria_source_attr",
        "data-custom-class=move || render_state.get().state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"checkbox-field-description\"",
    ] {
        assert!(
            source.contains(attr),
            "CheckboxField should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn checkbox_field_mounts_group_a11y_via_ui_headless_contract() {
    let source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let group_a11y = StoredValue::new(labeled_group_attrs(",
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
    ] {
        assert!(
            source.contains(needle),
            "CheckboxField should mount group semantics via ui-headless contract; missing `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_mounts_motion_contract_via_motion_module() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");

    for needle in [
        "let motion = motion::sanitize_motion(motion);",
        "let motion_source = motion::source_attr(motion);",
        "let style_vars = StoredValue::new(motion::attach_motion(None, motion));",
        "style=move || style_vars.get_value()",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxField view should mount motion contract via motion module; missing `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "CheckboxField motion module should provide `{needle}` as stable contract."
        );
    }
}

#[test]
fn checkbox_field_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let motion_unit_test_source = load_source("../../components/checkbox-field/test/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "pub struct CheckboxFieldMotion {",
        "pub enabled: bool,",
        "pub transition_ms: u16,",
        "pub indicator_scale_pct: u16,",
        "default_text_field_motion_tokens()",
        "pub fn sanitize_motion(",
        "transition_ms.min(1200)",
        "indicator_scale_pct: motion.indicator_scale_pct.clamp(80, 140)",
        "pub fn resolve_effective_motion(",
        "if !motion.enabled || prefers_reduced_motion {",
        "transition_ms: 1,",
        "indicator_scale_pct: 100,",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-checkbox-field-transition-ms",
        "--ui-checkbox-field-indicator-scale",
    ] {
        assert!(
            motion_source.contains(needle),
            "checkbox-field motion contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn default_motion_matches_contract()",
        "fn sanitize_motion_clamps_invalid_values()",
        "fn resolve_effective_motion_respects_reduced_motion_contract()",
        "fn attach_motion_emits_css_custom_properties()",
    ] {
        assert!(
            motion_unit_test_source.contains(needle),
            "checkbox-field component-local motion regression should keep `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm stub/no-op contract should keep marker `{needle}`."
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "checkbox_field_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-field checklist should keep motion-contract evidence `{required}`."
        );
    }
}

#[test]
fn checkbox_field_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox_field\")]",
        "out.push_str(crate::checkbox_field::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should keep cascade-layer contract marker `{needle}`."
        );
    }

    for needle in [
        "let style_vars = StoredValue::new(motion::attach_motion(None, motion));",
        "style=move || style_vars.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field view should mount runtime style via css-variable payload `{needle}`."
        );
    }

    for needle in [
        "--ui-checkbox-field-transition-ms",
        "--ui-checkbox-field-indicator-scale",
        "pub fn attach_motion(",
        "pub fn compose_style_vars(",
    ] {
        assert!(
            motion_source.contains(needle),
            "checkbox-field motion runtime style should keep css-variable marker `{needle}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "top:",
        "left:",
        "right:",
        "bottom:",
        "position:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox-field motion runtime style should avoid plain inline style token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "checkbox_field_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-field checklist should keep cascade-layer/runtime-style evidence `{required}`."
        );
    }
}

#[test]
fn checkbox_field_theme_contract_is_token_first_and_ui_theme_backed() {
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");
    let css_source = load_source("../ui-theme/src/css.rs");
    let styling_spec_source = load_source("../docs/spec/styling.md");
    let token_baseline_test_source = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let wcag_test_source = load_source("../ui-theme/tests/wcag_contrast.rs");

    for needle in [
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default))",
        "var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
    ] {
        assert!(
            styles_source.contains(needle),
            "CheckboxField styles should consume ui-theme variables via `{needle}`."
        );
    }

    for forbidden in ["Theme::", "theme.get()", "resolve_tokens("] {
        assert!(
            !styles_source.contains(forbidden),
            "CheckboxField styles should not rebuild theme pipeline via `{forbidden}`."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
    ] {
        assert!(
            theme_source.contains(needle),
            "ui-theme should own three-axis context contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct ThemeTokens",
        "pub struct SemanticColorTokens",
        "pub struct LayoutTokens",
        "pub struct TypographyTokens",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme token taxonomy should be defined in tokens.rs via `{needle}`."
        );
    }

    for needle in [
        "--ui-space-2xs",
        "--ui-fg",
        "--ui-fg-muted",
        "--ui-danger",
        "--ui-accent",
        "--ui-text-field-motion-duration",
        "--ui-text-field-motion-easing",
        "--ui-fallback-text-field-motion-duration",
        "--ui-fallback-text-field-motion-easing",
        "--ui-fallback-checkbox-gap",
        "--ui-fallback-checkbox-size-default",
        "--ui-fallback-checkbox-disabled-opacity",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css emitter should output variable `{needle}`."
        );
    }

    assert!(
        styling_spec_source.contains("Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量"),
        "styling spec should document ui-theme SSOT path."
    );
    assert!(
        token_baseline_test_source.contains("fn token_scale_baselines_are_regression_testable()"),
        "ui-theme should keep token scale baseline regression test."
    );
    assert!(
        wcag_test_source.contains("fn semantic_colors_meet_wcag_21_aa_for_text_pairs()"),
        "ui-theme should keep WCAG contrast regression guard."
    );
}

#[test]
fn checkbox_field_styles_use_defensive_variable_fallback_chain() {
    let source = load_source("src/checkbox_field/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default))",
        "var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
    ] {
        assert!(
            source.contains(required),
            "CheckboxField styles should keep defensive variable fallback chain `{required}`."
        );
    }

    for forbidden in [
        "180ms",
        "cubic-bezier(0.2, 0, 0, 1)",
        "calc(20px + 10px)",
        "font-size: var(--ui-font-size-100, 12px);",
        "line-height: var(--ui-line-height-100, 16px);",
        "outline: 1px solid",
        "outline-offset: 2px;",
    ] {
        assert!(
            !source.contains(forbidden),
            "CheckboxField styles should avoid hardcoded terminal fallback `{forbidden}`."
        );
    }

    for required in [
        "--ui-fallback-text-field-motion-duration",
        "--ui-fallback-text-field-motion-easing",
        "--ui-fallback-checkbox-gap",
        "--ui-fallback-checkbox-size-default",
        "--ui-fallback-checkbox-disabled-opacity",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css output should provide checkbox-field fallback variable `{required}`."
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "checkbox_field_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-field checklist should keep defensive-variables evidence `{required}`."
        );
    }
}

#[test]
fn checkbox_field_styles_include_state_marker_contracts() {
    let source = load_source("src/checkbox_field/styles.rs");

    for selector in [
        ".ui-checkbox-field--indicator-end",
        ".ui-checkbox-field[data-indicator-placement=\"end\"]",
        ".ui-checkbox-field--tone-quiet",
        ".ui-checkbox-field[data-tone=\"default\"]",
        ".ui-checkbox-field--invalid .ui-checkbox-field__description",
        ".ui-checkbox-field[data-disabled=\"true\"]",
        ".ui-checkbox-field--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "CheckboxField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn checkbox_field_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "const CHECKBOX_FIELD_DOC_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "title=\"Hello World（默认路径）\"",
        "code_signal=hello_code",
        "code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()",
        "<CheckboxField label=\"Accept terms of service\".to_string() />",
        "title=\"Controlled + Description\"",
        "code_signal=code",
        "id_base=\"docs-checkbox-field-newsletter\".to_string()",
        "label=\"Subscribe to product updates\".to_string()",
        "description=\"Receive release notes and occasional best-practice tips.\".to_string()",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "code_signal=states_code",
        "id_base=\"docs-checkbox-field-terms\".to_string()",
        "indicator_placement=CheckboxFieldIndicatorPlacement::End",
        "tone=CheckboxFieldTone::Quiet",
        "is_invalid=Some(true)",
        "class_name=\"docs-checkbox-field-custom\".to_string()",
        "id_base=\"docs-checkbox-field-read-only\".to_string()",
        "is_disabled=Some(true)",
        "aria_label=\"Maintenance alerts (read only)\".to_string()",
        "data-slot=\"checkbox-field-state-matrix-note\"",
        "title=\"Controlled vs Default (Comparison)\"",
        "code_signal=comparison_code",
        "id_base=\"docs-checkbox-field-controlled\".to_string()",
        "id_base=\"docs-checkbox-field-uncontrolled\".to_string()",
        "default_checked=Some(true)",
        "data-slot=\"checkbox-field-controlled-uncontrolled-note\"",
        "data-slot=\"checkbox-field-streaming-policy\"",
        "data-slot=\"checkbox-field-streaming-modes\"",
        "data-slot=\"checkbox-field-copy-ready\"",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
        "data-slot=\"checkbox-field-state-matrix-note\"",
        "data-slot=\"checkbox-field-controlled-uncontrolled-note\"",
        "data-slot=\"checkbox-field-streaming-policy\"",
        "data-slot=\"checkbox-field-streaming-modes\"",
        "data-slot=\"checkbox-field-copy-ready\"",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for checkbox-field semantics.",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui_components::*.",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs should keep docs-product marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "compose_copy_ready_code",
        "missing_import_lines",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy-ready pipeline should keep import-completion marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "checkbox_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 should keep docs-product evidence `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        script_source.contains(script_needle),
        "dx gate script should include `{script_needle}`."
    );
}

#[test]
fn checkbox_field_readme_hello_world_keeps_uncontrolled_default_path() {
    let source = load_source("src/checkbox_field/README.md");

    for needle in [
        "## Hello World（最小可用）",
        "<CheckboxField label=\"Accept terms\".to_string() />",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field README should include default-path hello world `{needle}`.",
        );
    }

    assert!(
        !source.contains("let (checked, set_checked) = signal(false);"),
        "checkbox-field README hello world should not require manual signal wiring.",
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn checkbox_field_minimal_feature_gate_keeps_checkbox_dependency_wired() {
    let cargo_toml = load_source("Cargo.toml");
    let view_source = load_source("src/checkbox_field/view.rs");

    assert!(
        cargo_toml.contains("component-checkbox_field = [\"component-checkbox\"]"),
        "component-checkbox_field must depend on component-checkbox to keep minimal feature builds valid."
    );

    assert!(
        view_source.contains("use crate::checkbox::{Checkbox, CheckboxVariant};"),
        "checkbox_field view should import checkbox types from crate::checkbox module, not root re-exports."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn checkbox_field_breaking_migration_removes_nested_checkbox_domain() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let legacy_dir = manifest_dir.join("src/checkbox_field/checkbox");
    let lib_source = load_source("src/lib.rs");

    assert!(
        !legacy_dir.exists(),
        "breaking migration should remove legacy nested checkbox domain at `{}`.",
        legacy_dir.display()
    );
    assert!(
        lib_source.contains("pub use ui_checkbox as checkbox;"),
        "crate root should re-export top-level checkbox domain from ui-checkbox crate."
    );
    assert!(
        lib_source.contains("pub mod checkbox_field;"),
        "crate root should keep checkbox_field as separate domain after split."
    );
}

#[test]
fn checkbox_field_docs_include_interactive_playground_contract_panels() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"crates/ui-components/src/checkbox_field/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_component_local_semantics_suite_exists_and_is_migrated() {
    let source = load_source("../../components/checkbox-field/test/semantics.rs");

    for needle in [
        "fn checkbox_field_component_keeps_ui_components_layer_file_layout()",
        "fn checkbox_field_logic_consumes_state_primitives_without_reimplementation()",
        "fn checkbox_field_view_mounts_headless_and_motion_contracts()",
        "fn checkbox_field_public_surface_does_not_expose_platform_dom_types()",
    ] {
        assert!(
            source.contains(needle),
            "checkbox-field should keep component-local semantics suite marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("src/checkbox_field/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "checkbox-field README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"checkbox-field\" => Some(CHECKBOX_FIELD_README_MD)"),
        "docs shell should map checkbox-field slug to CHECKBOX_FIELD_README_MD.",
    );
}

#[test]
fn checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`CheckboxField` 暂未接入精确 `render_count` 自动化计数",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field checklist should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "\"checkbox-field\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep checkbox-field budget/probe marker `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose perf marker `{needle}`.",
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should keep perf guard `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || render_state.get().state.state_attr",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "data-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field view should expose attribution marker `{needle}` for perf triage.",
        );
    }
}

#[test]
fn checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = load_source("../../components/checkbox-field/test/semantics.rs");
    let aggregated_semantics = load_source("tests/checkbox_field_semantics.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let checkbox_view_source = load_source("../../components/checkbox/src/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn checkbox_field_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency_locally(",
        "fn checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally(",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "checkbox-field local semantic/performance suite should include `{required_test}`.",
        );
    }

    for required_test in [
        "fn checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement(",
    ] {
        assert!(
            aggregated_semantics.contains(required_test),
            "checkbox-field aggregated semantic/performance suite should include `{required_test}`.",
        );
    }

    for marker in [
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())",
        "aria-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "aria-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "data-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "checkbox-field view should expose aria/data semantic marker `{marker}`.",
        );
    }

    for marker in [
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
        "on:focus=move |_| aria.handlers.focus_ring.on_focus.run(())",
        "on:blur=move |_| {",
        "data-focused=move || render_state.get().state.is_focused.then_some(\"true\")",
        "data-focus-visible=move || render_state.get().state.is_focus_visible.then_some(\"true\")",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            checkbox_view_source.contains(marker),
            "checkbox-field focus-flow contract should remain delegated to checkbox semantics via `{marker}`.",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !aggregated_semantics.contains(&snapshot_macro)
            && !aggregated_semantics.contains(&insta_snapshot),
        "checkbox-field semantic/performance regression should not degrade to snapshot-only checks.",
    );

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }
}

#[test]
fn checkbox_field_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "checkbox_field_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency_locally",
        "checkbox_field_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "checkbox-field check2 semantic/performance section should include `{marker}`.",
        );
    }
}

#[test]
fn checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/checkbox_field/view.rs");
    let local_semantics_source = load_source("../../components/checkbox-field/test/semantics.rs");
    let semantics_source = load_source("tests/checkbox_field_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())",
        "aria-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "aria-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "checkbox-field semantic-priority contract should keep marker `{marker}`.",
        );
    }

    for marker in [
        "fn checkbox_field_component_keeps_ui_components_layer_file_layout()",
        "fn checkbox_field_logic_consumes_state_primitives_without_reimplementation()",
        "fn checkbox_field_view_mounts_headless_and_motion_contracts()",
        "fn checkbox_field_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency_locally()",
        "fn checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks_locally()",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "checkbox-field local semantics suite should keep marker `{marker}`.",
        );
    }

    for marker in [
        "fn checkbox_field_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement(",
        "fn checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks(",
    ] {
        assert!(
            semantics_source.contains(marker),
            "checkbox-field aggregated semantics suite should keep marker `{marker}`.",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !local_semantics_source.contains(&snapshot_macro)
            && !semantics_source.contains(&snapshot_macro)
            && !local_semantics_source.contains(&insta_snapshot)
            && !semantics_source.contains(&insta_snapshot),
        "checkbox-field semantic-priority path should avoid snapshot-only assertions.",
    );

    let script_needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`.",
    );
}

#[test]
fn checkbox_field_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "echo \"[perf] contract: checkbox-field semantic test priority\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should include checkbox-field semantic-priority marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_semantic_test_priority_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "checkbox-field check2 should mark semantic-test-priority item complete.",
    );

    for needle in [
        "components/checkbox-field/test/semantics.rs",
        "checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks_locally",
        "checkbox_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "checkbox_field_performance_script_covers_semantic_test_priority_contract",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 semantic-test-priority section should reference `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_view_macro_complexity_is_bounded_and_semantic() {
    let view_source = load_source("src/checkbox_field/view.rs");

    assert!(
        view_source.lines().count() <= 220,
        "checkbox-field view.rs should stay compact; split when layout grows."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "checkbox-field should keep bounded macro count: root + lightweight function subrenders."
    );
    assert_eq!(
        view_source
            .matches("data-slot=\"checkbox-field-description\"")
            .count(),
        1,
        "description fragment should stay single-source to avoid duplicated markup branches."
    );

    for needle in [
        "fn render_checkbox_field_label(",
        "fn render_checkbox_field_description(",
        "{render_checkbox_field_label(label)}",
        "{render_checkbox_field_description(",
        "let render_state = Memo::new(move |_| {",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
        "<Checkbox",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field view macro should keep semantic assembly marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_prefers_functional_subrenders_over_extra_components() {
    let view_source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "fn render_checkbox_field_label(",
        "fn render_checkbox_field_description(",
        "-> impl IntoView",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field should keep lightweight subrender function marker `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_checkbox_field_label",
        "#[component]\nfn render_checkbox_field_description",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-field subfragments should stay plain functions, not components: `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_component_directory_has_standard_file_layout() {
    for required in [
        "src/checkbox_field/mod.rs",
        "src/checkbox_field/logic.rs",
        "src/checkbox_field/styles.rs",
        "src/checkbox_field/view.rs",
        "src/checkbox_field/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "checkbox-field component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/checkbox_field/render.rs"),
        "checkbox-field should not drift into `render.rs`; rendering must stay in `view.rs`."
    );
    assert!(
        !path_exists("src/checkbox_field/spec.rs"),
        "checkbox-field is a simple component and should not introduce `src/checkbox_field/spec.rs`."
    );

    assert!(
        !path_exists("src/checkbox_field/protocol.rs"),
        "checkbox-field file-placement discipline should not keep `src/checkbox_field/protocol.rs`."
    );

    let component_src_dir = resolve_source_path("src/checkbox_field/mod.rs")
        .and_then(|path| path.parent().map(Path::to_path_buf))
        .expect("checkbox-field src directory should be discoverable from mod.rs");
    let mut rust_files: Vec<String> = fs::read_dir(&component_src_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {component_src_dir:?}: {e}"))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        })
        .collect();
    rust_files.sort();

    assert_eq!(
        rust_files,
        vec!["logic.rs", "mod.rs", "motion.rs", "styles.rs", "view.rs"],
        "checkbox-field src should keep strict file-placement discipline with only standard component files."
    );
}

#[test]
fn checkbox_field_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/checkbox_field/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CheckboxFieldMotion;",
        "pub use view::CheckboxField;",
    ] {
        assert!(
            mod_source.contains(needle),
            "checkbox_field/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use styles::",
        "pub use self::styles::",
        "pub mod spec;",
        "pub use spec::",
        "pub use self::spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "checkbox_field/mod.rs should not over-export internal marker `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");

    for needle in [
        "pub use ui_state_primitives::checkbox::{",
        "pub use ui_state_primitives::checkbox_field::{",
        "pub fn resolve_content(input: CheckboxFieldContentInput) -> CheckboxFieldContent",
        "pub fn resolve_checked_control(",
        "pub fn resolve_render_state(input: CheckboxFieldRenderStateInput) -> CheckboxFieldRenderState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "checkbox-field logic.rs should include normalization/derivation marker `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "#[component]",
        "data-slot=",
        "aria-label=",
        "<Checkbox",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "checkbox-field logic.rs should avoid view-layer marker `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-checkbox-field[data-indicator-placement=\"end\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "checkbox-field styles.rs should include token-first static css marker `{needle}`."
        );
    }

    for forbidden in [
        "use leptos",
        "Signal<",
        "Callback<",
        "NodeRef<",
        "ui_headless::",
        "ui_motion::",
        "on:click=",
        "aria-invalid=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "checkbox-field styles.rs should avoid logic/render marker `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "let group_a11y = StoredValue::new(labeled_group_attrs(",
        "logic::resolve_checked_control(",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
        "data-slot=\"checkbox-field\"",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field view.rs should include structure + headless mount marker `{needle}`."
        );
    }

    for forbidden in [
        "pub const CSS: &str =",
        "ui_motion::web::animate(",
        "pub fn sanitize_motion(",
        "pub struct CheckboxFieldMotion",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-field view.rs should avoid style-engine/motion-engine marker `{forbidden}`."
        );
    }

    for needle in [
        "pub struct CheckboxFieldMotion",
        "pub fn sanitize_motion(motion: CheckboxFieldMotion) -> CheckboxFieldMotion",
        "pub fn source_attr(motion: CheckboxFieldMotion) -> &'static str",
        "pub fn resolve_effective_motion(",
        "pub fn attach_motion(base_vars: Option<String>, motion: CheckboxFieldMotion) -> String",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "checkbox-field motion.rs should include motion-contract marker `{needle}`."
        );
    }

    for forbidden in [
        "use ui_headless::",
        "labeled_group_attrs(",
        "aria-label=",
        "data-slot=",
        "<Checkbox",
        "view! {",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox-field motion.rs should avoid a11y/view-layer marker `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_component_file_responsibilities_remain_scoped",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_file_placement_discipline_is_strict_and_protocol_free",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_spec_file_is_not_introduced_for_simple_component",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_component_directory_rules() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
        "checkbox_field_component_directory_has_standard_file_layout",
        "checkbox_field_mod_rs_keeps_minimal_stable_exports",
        "checkbox_field_component_file_responsibilities_remain_scoped",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn checkbox_field_file_placement_discipline_is_strict_and_protocol_free() {
    checkbox_field_component_directory_has_standard_file_layout();
}

#[test]
fn checkbox_field_check2_documents_file_placement_discipline_rules() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "无 `protocol.rs/render.rs/spec.rs` 额外实现文件",
        "checkbox_field_file_placement_discipline_is_strict_and_protocol_free",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep file-placement discipline rule `{required}`."
        );
    }
}

#[test]
fn checkbox_field_spec_file_is_not_introduced_for_simple_component() {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/checkbox_field/spec.rs");
    let readme_source = load_source("../../components/checkbox-field/src/README.md");

    assert!(
        !spec_path.exists(),
        "checkbox-field should not add `spec.rs` unless there is a stable external schema contract."
    );

    for forbidden in ["mod spec", "pub mod spec", "spec::", "CheckboxFieldSpec"] {
        assert!(
            !mod_source.contains(forbidden),
            "checkbox-field module boundary should not expose spec module via `{forbidden}`."
        );
    }

    for forbidden in ["Spec::new(", ".render()", "schema_version", "spec.rs"] {
        assert!(
            !readme_source.contains(forbidden),
            "checkbox-field docs should not force Hyper-Structure builder token `{forbidden}` for simple component scope."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`checkbox-field` 为简单字段组件"),
        "checkbox-field check2 should keep explicit no-spec-for-simple-component constraint."
    );
}

#[test]
fn checkbox_field_check2_marks_hyper_structure_builder_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design：`checkbox-field` 为简单字段组件",
        "checkbox_field_spec_file_is_not_introduced_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 should keep Hyper-Structure builder marker `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src_dir = manifest_dir.join("../../components/checkbox-field/src");
    let manifest_path = component_src_dir.join("Component.toml");
    let rbi_path = component_src_dir.join("checkbox_field.rbi");

    assert!(
        manifest_path.exists(),
        "checkbox-field should provide Component.toml for context compression."
    );
    assert!(
        rbi_path.exists(),
        "checkbox-field should provide checkbox_field.rbi for API signature projection."
    );

    let manifest_source = load_source("../../components/checkbox-field/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-field/src/checkbox_field.rbi");
    let view_source = load_source("src/checkbox_field/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CheckboxField\"",
        "crate = \"ui-checkbox-field\"",
        "name = \"is_checked\"",
        "name = \"checked\"",
        "name = \"on_checked_change\"",
        "name = \"set_checked\"",
        "name = \"default_checked\"",
        "name = \"is_disabled\"",
        "name = \"is_invalid\"",
        "name = \"id_base\"",
        "name = \"label\"",
        "name = \"description\"",
        "name = \"aria_label\"",
        "name = \"dir\"",
        "name = \"tone\"",
        "name = \"indicator_placement\"",
        "name = \"motion\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "checkbox-field Component.toml should include `{needle}`."
        );
    }

    for needle in [
        "pub type CheckboxFieldTone = ui_state_primitives::checkbox_field::CheckboxFieldTone;",
        "pub type CheckboxFieldIndicatorPlacement = ui_state_primitives::checkbox_field::CheckboxFieldIndicatorPlacement;",
        "pub struct CheckboxFieldMotion {",
        "pub fn CheckboxField(",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "checked: Option<leptos::prelude::ReadSignal<bool>>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
        "set_checked: Option<leptos::prelude::WriteSignal<bool>>",
        "default_checked: Option<bool>",
        "is_disabled: Option<bool>",
        "is_invalid: Option<bool>",
        "dir: Option<ui_headless::A11yDirection>",
        "tone: CheckboxFieldTone",
        "indicator_placement: CheckboxFieldIndicatorPlacement",
        "motion: CheckboxFieldMotion",
    ] {
        assert!(
            rbi_source.contains(needle),
            "checkbox_field.rbi should include signature projection marker `{needle}`."
        );
    }

    for needle in [
        "pub fn CheckboxField(",
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] set_checked: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_invalid: Option<bool>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] tone: CheckboxFieldTone",
        "#[prop(optional)] indicator_placement: CheckboxFieldIndicatorPlacement",
        "#[prop(optional)] motion: CheckboxFieldMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field view API should include `{needle}` for manifest/RBI alignment."
        );
    }
}

#[test]
fn checkbox_field_check2_marks_context_compression_manifest_rbi_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/checkbox-field/src/Component.toml",
        "components/checkbox-field/src/checkbox_field.rbi",
        "checkbox_field_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 should keep context-compression marker `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "checkbox_field_agent_contract_is_schema_typed_and_machine_readable",
        "checkbox_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "checkbox_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn checkbox_field_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");

    for needle in [
        "pub enum CheckboxFieldAgentSchemaVersion",
        "pub enum CheckboxFieldAgentIntent",
        "pub enum CheckboxFieldAgentAction",
        "pub enum CheckboxFieldAgentStateAxis",
        "pub enum CheckboxFieldAgentSource",
        "pub enum CheckboxFieldAgentOutputStatus",
        "pub struct CheckboxFieldAgentContract",
        "pub struct CheckboxFieldAgentContractInput",
        "pub fn resolve_agent_contract(",
        "schema_name: \"ui.checkbox-field.agent-contract\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "CheckboxField agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::CheckboxFieldAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "CheckboxField view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()
{
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");

    for typed_source in [
        "CheckboxFieldAgentSchemaVersion::V1",
        "CheckboxFieldAgentIntent::BooleanField",
        "resolve_agent_action(input)",
        "resolve_agent_state_axis(input.status)",
        "resolve_agent_source(input)",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "CheckboxField Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "CheckboxField Agent Contract should avoid free-form schema string splicing `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "CheckboxField Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "checkbox-field 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
        "checkbox_field_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn checkbox_field_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "data-stream",
        "data-output-status",
        "data-draft",
        "data-verified",
        "data-commit-ready",
        "retry",
        "fallback=snapshot",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-field is snapshot-only; forbidden streaming marker `{forbidden}` should not appear."
        );
    }
}

#[test]
fn checkbox_field_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "`CheckboxField` 不直接渲染 LLM 正文，但可稳定消费上层完整配置快照（snapshot）并完成语义渲染。",
        "checkbox_field_check2_documents_snapshot_as_default_baseline_capability",
        "checkbox_field_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn checkbox_field_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for marker in [
        "let resolved_content = logic::resolve_content(logic::CheckboxFieldContentInput {",
        "let checked_control = logic::resolve_checked_control(",
        "let render_state = Memo::new(move |_| {",
        "logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {",
        "<Checkbox",
        "{render_checkbox_field_label(label)}",
        "{render_checkbox_field_description(",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "checkbox-field snapshot render path should keep marker `{marker}`."
        );
    }

    for marker in [
        "pub fn resolve_content(input: CheckboxFieldContentInput) -> CheckboxFieldContent",
        "pub fn resolve_checked_control(",
        "pub fn resolve_render_state(input: CheckboxFieldRenderStateInput) -> CheckboxFieldRenderState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(marker),
            "checkbox-field logic should keep snapshot baseline derivation marker `{marker}`."
        );
    }

    for marker in [
        "slug=\"checkbox-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs checkbox-field page should keep complete snapshot consumption marker `{marker}`."
        );
    }
}

#[test]
fn checkbox_field_streaming_check_script_covers_snapshot_baseline_guard() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "checkbox-field 归类为 `Streaming Optional`；当前实现为 snapshot-only，并显式声明 `fallback=snapshot`，输出 `data-ui-output-status`。",
        "checkbox_field_check2_documents_streaming_required_optional_classification_rules",
        "checkbox_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "checkbox_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep streaming required/optional marker `{required}`."
        );
    }
}

#[test]
fn checkbox_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/checkbox_field/view.rs");

    for marker in [
        "role=group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())",
        "aria-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "aria-invalid=move || render_state.get().state.is_invalid.then_some(\"true\")",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
        "data-state=move || render_state.get().state.state_attr",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "checkbox-field should keep continuous role/aria/data semantics marker `{marker}` in optional-streaming scope."
        );
    }
}

#[test]
fn checkbox_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-field should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_streaming_check_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
 {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-field non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let view_source = load_source("src/checkbox_field/view.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-checkbox-field\")",
        "Cow::Borrowed(\"ui-checkbox-field--custom-class\")",
        "Cow::Owned(base_class_name)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox-field logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-checkbox-field\".to_string()",
        "\"ui-checkbox-field--checked\".to_string()",
        "\"ui-checkbox-field--unchecked\".to_string()",
        "\"ui-checkbox-field--invalid\".to_string()",
        "\"ui-checkbox-field--disabled\".to_string()",
        "\"ui-checkbox-field--with-description\".to_string()",
        "\"ui-checkbox-field--no-description\".to_string()",
        "\"ui-checkbox-field--custom-class\".to_string()",
        "String::from(\"ui-checkbox-field\")",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "checkbox-field string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "checkbox_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "checkbox_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "checkbox_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_tree_shaking_contract_is_feature_gated_in_ui_components_lib_and_css() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-checkbox_field = [\"component-checkbox\", \"dep:ui-checkbox-field\"]",
        "ui-checkbox-field = { path = \"../../components/checkbox-field\", optional = true }",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo feature tree should keep `{needle}` for checkbox-field tree shaking.",
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-checkbox_field\")]\npub use ui_checkbox_field as checkbox_field;",
        ),
        "ui-components lib.rs should gate checkbox-field export behind component feature."
    );

    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-checkbox_field\")]\n    out.push_str(crate::checkbox_field::styles::CSS);",
        ),
        "ui-components css.rs should gate checkbox-field CSS aggregation behind component feature."
    );
}

#[test]
fn checkbox_field_tree_shaking_script_enforces_component_minimal_feature_tree_and_web_demo_reverse_dependency()
 {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "CHECKBOX_FIELD_MIN_FEATURES=\"component-checkbox_field,inject-css\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_tree_shaking_contract_is_feature_gated_in_ui_components_lib_and_css",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_tree_shaking_script_enforces_component_minimal_feature_tree_and_web_demo_reverse_dependency",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CHECKBOX_FIELD_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$CHECKBOX_FIELD_MIN_FEATURES\")\"",
        "feature \"component-checkbox_field\" (command-line)",
        "feature \"inject-css\" (command-line)",
        "checkbox-field minimal feature tree should not pull all-components",
        "WEB_DEMO_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p web-demo)\"",
        "web-demo should not pull all-components",
        "web-demo should pull web-demo-components feature bundle",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should enforce `{needle}` for checkbox-field.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-checkbox_field",
        "pub use ui_checkbox_field as checkbox_field;",
        "out.push_str(crate::checkbox_field::styles::CSS);",
        "cargo tree -e features -p ui-components --no-default-features --features component-checkbox_field,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "checkbox_field_tree_shaking_contract_is_feature_gated_in_ui_components_lib_and_css",
        "checkbox_field_tree_shaking_script_enforces_component_minimal_feature_tree_and_web_demo_reverse_dependency",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field checklist should keep tree-shaking contract marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_inner_html_contract_is_absent_and_security_guarded() {
    let view_source = load_source("src/checkbox_field/view.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "raw_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox-field view should not introduce unsafe html injection surface `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`checkbox-field` 当前无 `inner_html` 使用点",
        "checkbox_field_inner_html_contract_is_absent_and_security_guarded",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field checklist should keep inner_html security contract marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let checkbox_field_manifest = load_source("../../components/checkbox-field/Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "ui-components should keep shared wasm debug feature gate `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ndev-all-components")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before dev-all-components declaration");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "shared wasm debug feature must not be pulled into all-components production path."
    );

    for forbidden_feature in [
        "checkbox-field-wasm-debug",
        "checkbox_field-wasm-debug",
        "component-checkbox_field-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden_feature)
                && !checkbox_field_manifest.contains(forbidden_feature),
            "checkbox-field should not leak production-facing wasm debug feature `{forbidden_feature}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs app should keep shared dev-only wasm debug visual entry `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "shared ui-headless trace contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "format!(\"{ts_ms}ms\")",
        ".take(40)",
        "trace.emit(",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "shared debug overlay should keep trace/replay marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || render_state.get().state.state_attr",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-checked-mode=checked_mode_attr",
        "data-checked-prop-source=checked_prop_source_attr",
        "data-checked-change-source=checked_change_source_attr",
        "data-checked-default-source=checked_default_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox-field should expose machine-readable state/source marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "tracing::",
        "trace!(",
        "debug!(",
        "provide_ui_trace(",
        "use_ui_trace(",
        "trace.emit(",
        "UiDebugOverlay",
        "cfg(debug_assertions)",
        "replay",
        "timeline",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "checkbox-field should not duplicate component-local wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A：`checkbox-field` 不新增组件私有 wasm 调试事件协议",
        "checkbox_field_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field checklist should keep wasm debug governance marker `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
}

#[test]
fn checkbox_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-style/isolated-canvas marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "let (interactive_checked, set_interactive_checked) = signal(true);",
        "is_checked=Some(interactive_checked)",
        "on_checked_change=Some(set_interactive_checked)",
        "\"checked: \" {move || interactive_checked.get()}",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs should keep interactive workbench/context marker `{needle}`."
        );
    }

    for forbidden in [
        "CHECKBOX_FIELD_WORKBENCH_STORAGE_KEY",
        "load_checkbox_field_workbench_state(",
        "save_checkbox_field_workbench_state(",
        "clear_checkbox_field_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "checkbox-field keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "checkbox_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox-field checklist should keep DX governance marker `{required}`."
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";

    assert!(
        script_source.contains(needle),
        "DX check script should enforce `{needle}`."
    );
}

#[test]
fn checkbox_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        resolve_source_path("src/checkbox_field/spec.rs").is_none(),
        "checkbox-field should keep spec/schema boundary as N/A for simple component scope."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-field engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn checkbox_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/checkbox_field/mod.rs"),
        load_source("src/checkbox_field/logic.rs"),
        load_source("src/checkbox_field/view.rs"),
        load_source("src/checkbox_field/styles.rs"),
        load_source("src/checkbox_field/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    for forbidden_feature in [
        "checkbox-field-wasm-debug",
        "checkbox_field-wasm-debug",
        "component-checkbox_field-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "checkbox-field should not define component-local tracing feature `{forbidden_feature}` when no local debug event/replay contract exists."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::checkbox_field::",
        "const CHECKBOX_FIELD_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox-field should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/checkbox_field/mod.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");
    let view_source = load_source("src/checkbox_field/view.rs");
    let styles_source = load_source("src/checkbox_field/styles.rs");
    let motion_source = load_source("src/checkbox_field/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "checkbox-field engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "checkbox-field public module boundary should not leak web_sys types."
    );
}

#[test]
fn checkbox_field_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/checkbox-field/src/Component.toml");
    let rbi_source = load_source("../../components/checkbox-field/src/checkbox_field.rbi");
    let mod_source = load_source("../../components/checkbox-field/src/mod.rs");
    let logic_source = load_source("../../components/checkbox-field/src/logic.rs");
    let view_source = load_source("../../components/checkbox-field/src/view.rs");
    let styles_source = load_source("../../components/checkbox-field/src/styles.rs");
    let motion_source = load_source("../../components/checkbox-field/src/motion.rs");
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CheckboxField\"",
        "crate = \"ui-checkbox-field\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "checkbox-field manifest should keep stable v1 marker `{needle}`."
        );
    }

    for needle in [
        "pub fn CheckboxField(",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "default_checked: Option<bool>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "checkbox-field RBI should keep stable API marker `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "checkbox-field should not introduce migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CheckboxField` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "checkbox_field_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let marker = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_version_deprecation_migration_is_na_without_major_breaking_upgrade";

    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn checkbox_field_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("../../components/checkbox-field/check2.md");

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "checkbox-field checklist should keep ui-components entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn checkbox_field_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-checkbox_field\")]",
        "pub use ui_checkbox_field as checkbox_field;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox_field\")]",
        "out.push_str(crate::checkbox_field::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "CheckboxField",
        "Accordion",
        "Button",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn checkbox_field_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_ui_components_fixed_entry_files_follow_layered_boundaries";

    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn checkbox_field_platform_check_script_covers_motion_reduced_and_non_wasm_contract() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let needle = "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";

    assert!(
        script_source.contains(needle),
        "platform check script should enforce `{needle}`."
    );
}

#[test]
fn checkbox_field_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "components/checkbox-field/test/semantics.rs::checkbox_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_field_paths",
        "scripts/check-ui-components-e2e-checkbox-field.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 e2e-selector/stable-wait section should reference `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "/#/components/checkbox-field",
        "const WASM_READY = \"body:not(:has(#boot))\";",
        "await page.locator(WASM_READY).waitFor();",
        "#docs-checkbox-field-newsletter[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-terms[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-read-only[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-controlled[data-slot=\"checkbox-field\"]",
        "#docs-checkbox-field-uncontrolled[data-slot=\"checkbox-field\"]",
        "[data-slot=\"checkbox\"][role=\"checkbox\"]",
        "toHaveAttribute(\"data-checked-mode\", \"controlled\")",
        "toHaveAttribute(\"data-checked-mode\", \"uncontrolled\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e selector/stable-wait contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        ":nth-child(",
        ":nth-of-type(",
        "locator(\"div > div >",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-field e2e selector contract should avoid flaky selector/wait token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_field_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "await controlledCheckbox.click();",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"checked\");",
        "await uncontrolledCheckbox.click();",
        "await expect(uncontrolled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await page.reload();",
        "await expect(reloadedUncontrolled).toHaveAttribute(\"data-state\", \"checked\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e flow should keep ready/settled semantic breakpoint `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-checkbox-field.sh");

    for needle in [
        "echo \"[e2e-checkbox-field] contract: checklist e2e-selector/stable-wait governance\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "echo \"[e2e-checkbox-field] contract: semantic selectors + settled waits\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "echo \"[e2e-checkbox-field] contract: ready/settled semantic breakpoints\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_field_paths",
    ] {
        assert!(
            script_source.contains(needle),
            "checkbox-field e2e check script should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "checkbox-field check2 should mark e2e-selector-stability checklist item complete.",
    );

    for needle in [
        "docs_app_checkbox_field_contract.spec.mjs",
        "body:not(:has(#boot))",
        "checkbox_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits",
        "checkbox_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "checkbox_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "checkbox_field_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_field_paths",
        "scripts/check-ui-components-e2e-checkbox-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 e2e-selector-stability section should reference `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 repeatable-key-flow section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "body:not(:has(#boot))",
        "#docs-checkbox-field-controlled[data-slot=\"checkbox-field\"]",
        "const controlledCheckbox = controlled.locator('[data-slot=\"checkbox\"][role=\"checkbox\"]').first();",
        "await controlledCheckbox.focus();",
        "await expect(controlledCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"checked\");",
        "#docs-checkbox-field-uncontrolled[data-slot=\"checkbox-field\"]",
        "await uncontrolledCheckbox.focus();",
        "await expect(uncontrolledCheckbox).toBeFocused();",
        "await expect(uncontrolled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await page.reload();",
        "await expect(reloadedUncontrolled).toHaveAttribute(\"data-state\", \"checked\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e repeatable key-flow contract should include `{needle}`.",
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox-field repeatable key flow should avoid non-semantic/flaky token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_field_e2e_check_script_covers_repeatable_key_flow_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-checkbox-field.sh");

    for needle in [
        "echo \"[e2e-checkbox-field] contract: checklist repeatable-key-flow governance\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_e2e_repeatable_key_flow_rules",
        "echo \"[e2e-checkbox-field] contract: repeatable key flow with semantic ready/settled breakpoints\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "checkbox-field e2e script should include repeatable-key-flow marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "checkbox-field check2 should mark repeatable-key-flow item complete.",
    );

    for needle in [
        "docs_app_checkbox_field_contract.spec.mjs",
        "docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "checkbox_field_e2e_repeatable_key_flow_uses_focus_keyboard_and_semantic_breakpoints",
        "checkbox_field_check2_documents_e2e_repeatable_key_flow_rules",
        "checkbox_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "scripts/check-ui-components-e2e-checkbox-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 repeatable-key-flow section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 docs-sync/state-matrix section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let logic_source = load_source("src/checkbox_field/logic.rs");

    for needle in [
        "<Playground\n                title=\"Hello World（默认路径）\"",
        "<Playground\n                title=\"Controlled + Description\"",
        "<Playground\n                title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "<Playground\n                title=\"Controlled vs Default (Comparison)\"",
        "data-slot=\"checkbox-field-state-matrix-note\"",
        "data-slot=\"checkbox-field-controlled-uncontrolled-note\"",
        "is_checked=Some(newsletter)",
        "on_checked_change=Some(set_newsletter)",
        "is_checked=Some(terms)",
        "on_checked_change=Some(set_terms)",
        "default_checked=Some(true)",
        "is_disabled=Some(true)",
        "is_invalid=Some(true)",
        "tone=CheckboxFieldTone::Quiet",
        "indicator_placement=CheckboxFieldIndicatorPlacement::End",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs matrix/examples should include `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool {",
        "is_disabled.unwrap_or(disabled)",
        "pub fn normalize_is_invalid(is_invalid: Option<bool>, invalid: bool) -> bool {",
        "is_invalid.unwrap_or(invalid)",
        "pub fn resolve_checked_control(",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "default_checked,",
        "checked_default_source_attr = if default_checked.is_some() {",
        "\"default_checked\"",
        "\"implicit-default\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "checkbox-field logic should keep API/default normalization marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: checkbox-field docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include docs-sync/state-matrix gate `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checkbox-field check2 should mark docs-sync/state-matrix item complete.",
    );

    for needle in [
        "components/checkbox-field/test/semantics.rs::checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/checkbox-field/test/semantics.rs::checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_check2_documents_docs_sync_and_state_matrix_rules",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "crates/ui-components/tests/checkbox_field_semantics.rs::checkbox_field_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 docs-sync/state-matrix section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 documentation-as-product section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/checkbox-field/src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");

    for needle in [
        "# CheckboxField",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<CheckboxField label=... />`，只传 `label` 也能直接工作。",
        "进阶控制：按需启用 `is_checked + default_checked + on_checked_change`。",
    ] {
        assert!(
            readme_source.contains(needle),
            "checkbox-field README should include beginner marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Controlled + Description\"",
        "title=\"Indicator End + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs entry should include `{needle}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("checkbox-field README should include Hello World section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("checkbox-field README should include common-usage section");
    let readme_progressive = readme_source
        .find("## 先用起来，再进阶")
        .expect("checkbox-field README should include beginner-to-advanced section");
    assert!(
        readme_hello < readme_common && readme_common < readme_progressive,
        "checkbox-field README should keep beginner-first order before advanced guidance.",
    );
}

#[test]
fn checkbox_field_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: checkbox-field documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include documentation-as-product gate `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "checkbox-field check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/checkbox-field/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field",
        "checkbox_field_check2_documents_documentation_as_product_rules",
        "checkbox_field_documentation_entry_exists_with_beginner_first_progression",
        "checkbox_field_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 documentation-as-product section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 interactive-playground section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit props and inspect actual config/state contracts.\"",
        "code_signal=interactive_code",
        "test_css_source=interactive_test_css",
        "test_source_path=\"crates/ui-components/src/checkbox_field/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "Switch checked=interactive_checked set_checked=set_interactive_checked",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "Switch checked=interactive_invalid set_checked=set_interactive_invalid",
        "checked=interactive_show_description",
        "checked=interactive_custom_class",
        "is_checked=Some(interactive_checked)",
        "on_checked_change=Some(set_interactive_checked)",
        "\"checked: \" {move || interactive_checked.get()}",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field docs should provide interactive marker `{needle}`.",
        );
    }

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app playground should keep interactive preview marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "docs-app checkbox-field key flow is repeatable with semantic breakpoints",
        "await page.goto(CHECKBOX_FIELD_PAGE);",
        "body:not(:has(#boot))",
        "await controlledCheckbox.focus();",
        "await expect(controlledCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"unchecked\");",
        "await expect(controlled).toHaveAttribute(\"data-state\", \"checked\");",
        "await uncontrolledCheckbox.focus();",
        "await expect(uncontrolledCheckbox).toBeFocused();",
        "await page.reload();",
        "await expect(reloadedUncontrolled).toHaveAttribute(\"data-state\", \"checked\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field interactive playground should keep repeatable e2e marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: checkbox-field interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include interactive-playground gate `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_interactive_playground_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "checkbox-field check2 should mark interactive-playground item complete.",
    );

    for needle in [
        "title=\"Interactive Playground\"",
        "forms_groups_extra.rs::checkbox_field",
        "docs_app_checkbox_field_contract.spec.mjs",
        "checkbox_field_check2_documents_interactive_playground_rules",
        "checkbox_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "checkbox_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "checkbox_field_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "AI Spec 相关联动示例：N/A（`checkbox-field` 非 Spec 构建器组件）",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 interactive-playground section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_checkbox_field_contract.spec.mjs");

    for needle in [
        "data-slot=\"checkbox-field-copy-ready\"",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui_components::*.",
        "Source paths: components/checkbox-field/src/mod.rs, components/checkbox-field/src/logic.rs, components/checkbox-field/src/view.rs, components/checkbox-field/src/styles.rs.",
        "Feature prerequisites: component-checkbox_field (inject-css optional for runtime style injection).",
        "title=\"Controlled + Description\"",
        "title=\"Interactive Playground\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox-field source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "docs-app checkbox-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui_components::*;",
        "data-slot=\"checkbox-field-source-paths\"",
        "data-slot=\"checkbox-field-source-prerequisites\"",
        "toContainText(\"components/checkbox-field/src/mod.rs\")",
        "toContainText(\"component-checkbox_field\")",
        "toContainText(\"inject-css\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "checkbox-field e2e source-first contract should contain `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: checkbox-field source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "dx script should include source-first copy-paste-ready gate `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "checkbox-field check2 should mark source-first copy-paste-ready item complete.",
    );

    for needle in [
        "forms_groups_extra.rs::checkbox_field",
        "docs_app_checkbox_field_contract.spec.mjs::docs-app checkbox-field playground source is copy-paste ready",
        "checkbox_field_check2_documents_source_first_copy_paste_ready_rules",
        "checkbox_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "checkbox_field_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs");
    let readme_source = load_source("../../components/checkbox-field/src/README.md");

    for needle in [
        "### CheckboxField 同步记录（2026-02-20）",
        "参数模型同步：`CheckboxField` 参数主轴保持 `is_checked/default_checked/on_checked_change`",
        "component_doc!(\"CheckboxField\", \"checkbox-field\", \"Forms\", forms_groups_extra::checkbox_field)",
        "`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field()`",
        "`components/checkbox-field/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include checkbox-field synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"CheckboxField\"",
        "\"checkbox-field\"",
        "forms_groups_extra::checkbox_field",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose checkbox-field entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn checkbox_field() -> AnyView",
        "title=\"CheckboxField\"",
        "slug=\"checkbox-field\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app checkbox-field page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in [
        "# CheckboxField",
        "## docs-app 入口",
        "forms_groups_extra.rs::checkbox_field()",
        "#/components/checkbox-field",
    ] {
        assert!(
            readme_source.contains(needle),
            "checkbox-field README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: checkbox-field heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test checkbox_field_semantics --no-default-features --features component-checkbox_field,inject-css checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn checkbox_field_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/checkbox-field/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "checkbox_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "checkbox_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "checkbox_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox-field check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}
