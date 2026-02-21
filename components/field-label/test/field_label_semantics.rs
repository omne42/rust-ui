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
fn field_label_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/field_form/field_label/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "FieldLabel internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn field_label_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/field_form/field_label/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::FieldLabel;"),
        "field_label module should export `FieldLabel`."
    );
    assert!(
        crate_source.contains("pub use field_form::field_label::{FieldLabel, FieldLabelTone};"),
        "crate root should re-export FieldLabel contract."
    );
}

#[test]
fn field_label_uses_primitives_and_headless_contract_model() {
    let logic_source = load_source("src/field_form/field_label/logic.rs");
    let view_source = load_source("src/field_form/field_label/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/field_label.rs");
    let headless_source = load_source("../ui-headless/src/field_label.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "pub use ui_state_primitives::field_label::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_REQUIRED_INDICATOR",
        "DEFAULT_TEXT",
        "FieldLabelState",
        "FieldLabelStateInput",
        "FieldLabelTone",
        "normalize_aria_label",
        "normalize_optional_text",
        "normalize_required_indicator",
        "normalize_text",
        "normalize_props",
        "FieldLabelLogicInput",
        "FieldLabelViewModel",
        "derive_view_model",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "FieldLabel logic should bridge primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum FieldLabelTone",
        "pub struct FieldLabelStateInput",
        "pub struct FieldLabelState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_text(",
        "pub fn normalize_required_indicator(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "FieldLabel state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub struct FieldLabelOptions",
        "pub struct FieldLabelAttrs",
        "pub struct FieldLabelContract",
        "pub fn use_field_label(options: FieldLabelOptions) -> FieldLabelContract",
        "locale_attrs(options.lang, options.dir)",
    ] {
        assert!(
            headless_source.contains(needle),
            "FieldLabel headless contract should include `{needle}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod field_label;"),
        "ui-state-primitives should export `field_label` module."
    );
    assert!(
        headless_lib_source.contains("pub mod field_label;"),
        "ui-headless should export `field_label` module."
    );
    assert!(
        headless_lib_source.contains("use_field_label"),
        "ui-headless should re-export `use_field_label` contract."
    );

    for needle in [
        "logic::derive_view_model(",
        "FieldLabelLogicInput {",
        "let logic::FieldLabelViewModel {",
        "logic::compose_class_name(class_name.get_value(), state.get_value())",
        "use_field_label(FieldLabelOptions {",
        "state: state.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "FieldLabel view should compose logic/headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn field_label_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/field_form/field_label/view.rs");

    for attr in [
        "<label",
        "for=for_id",
        "data-ui-schema=logic::FIELD_LABEL_AGENT_SCHEMA",
        "data-ui-intent=logic::FieldLabelAgentIntent::Label.as_attr()",
        "data-ui-action=logic::FieldLabelAgentAction::SnapshotRender.as_attr()",
        "data-ui-streaming=logic::FieldLabelAgentStreaming::Optional.as_attr()",
        "data-ui-fallback=logic::FieldLabelAgentFallback::Snapshot.as_attr()",
        "data-ui-output-state=logic::FieldLabelAgentOutputState::Verified.as_attr()",
        "data-slot=\"field-label\"",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-required=move || semantics.get().attrs.data_required",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-has-for=move || semantics.get().attrs.data_has_for",
        "data-text-source=move || semantics.get().attrs.data_text_source",
        "data-indicator-source=move || semantics.get().attrs.data_indicator_source",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-disabled=move || semantics.get().attrs.aria_disabled",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-slot=\"field-label-text\"",
        "data-slot=\"field-label-required\"",
    ] {
        assert!(
            source.contains(attr),
            "FieldLabel should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn field_label_manifest_declares_agent_contract_schema_and_whitelist() {
    let source = load_source("../../components/field-label/src/Component.toml");

    for needle in [
        "name = \"agent_contract_schema_markers\"",
        "schema = \"field_label.v1\"",
        "intent = \"label\"",
        "action = \"snapshot_render\"",
        "streaming = \"optional\"",
        "fallback = \"snapshot\"",
        "output_state = \"verified\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-streaming\"",
        "attr = \"data-ui-fallback\"",
        "attr = \"data-ui-output-state\"",
        "name = \"render_path\"",
        "inner_html",
        "<script",
        "retry",
        "reconnect",
        "transport_validation",
    ] {
        assert!(
            source.contains(needle),
            "field_label manifest should include agent contract evidence `{needle}`."
        );
    }
}

#[test]
fn field_label_styles_include_tone_state_and_markers() {
    let source = load_source("src/field_form/field_label/styles.rs");

    for selector in [
        ".ui-field-label {",
        ".ui-field-label--tone-default",
        ".ui-field-label[data-tone=\"default\"]",
        ".ui-field-label--tone-muted",
        ".ui-field-label[data-tone=\"muted\"]",
        ".ui-field-label--tone-strong",
        ".ui-field-label[data-tone=\"strong\"]",
        ".ui-field-label--required",
        ".ui-field-label[data-required=\"true\"]",
        ".ui-field-label--disabled",
        ".ui-field-label[data-disabled=\"true\"]",
        ".ui-field-label--for",
        ".ui-field-label[data-has-for=\"true\"]",
        ".ui-field-label--text-custom",
        ".ui-field-label[data-text-source=\"custom\"]",
        ".ui-field-label--indicator-custom",
        ".ui-field-label[data-indicator-source=\"custom\"]",
        ".ui-field-label--aria-custom",
        ".ui-field-label[data-aria-source=\"custom\"]",
        ".ui-field-label--custom-class",
        ".ui-field-label[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "FieldLabel styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn field_label_styles_use_defensive_variable_fallback_chain() {
    let source = load_source("src/field_form/field_label/styles.rs");

    for required in [
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "color: var(--ui-danger, var(--ui-fallback-danger));",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
    ] {
        assert!(
            source.contains(required),
            "FieldLabel styles should use defensive fallback-chain token `{required}`."
        );
    }

    for forbidden in ["14px", "20px", "0.85em"] {
        assert!(
            !source.contains(forbidden),
            "FieldLabel styles should not keep hard-coded terminal size `{forbidden}`."
        );
    }
}

#[test]
fn field_label_non_test_sources_follow_rust_hygiene_contract() {
    let logic_source = load_source("src/field_form/field_label/logic.rs");
    let view_source = load_source("src/field_form/field_label/view.rs");

    for source in [&logic_source, &view_source] {
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "field_label non-test source should forbid `{forbidden}`."
            );
        }
    }

    assert!(
        logic_source.contains("use std::borrow::Cow;"),
        "field_label logic should use Cow for string clone hotspot control."
    );
    assert!(
        logic_source.contains("Vec<Cow<'static, str>>"),
        "field_label class composition should use Vec<Cow<'static, str>>."
    );
}

#[test]
fn field_label_css_is_aggregated_in_ui_layer_and_view_avoids_inline_style() {
    let css_source = load_source("src/css.rs");
    let view_source = load_source("src/field_form/field_label/view.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "out.push_str(crate::field_form::field_label::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregation should keep `{required}` for field-label layer contract."
        );
    }

    for forbidden in ["style=\"", "style=move ||"] {
        assert!(
            !view_source.contains(forbidden),
            "field_label view should avoid plain inline style contract `{forbidden}`."
        );
    }
}

#[test]
fn field_label_ui_components_entry_files_follow_architecture_contract() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "pub mod root;",
        "feature = \"component-field_label\"",
        "pub mod field_form {",
        "pub use field_form::field_label::{FieldLabel, FieldLabelTone};",
        "#[doc(hidden)]",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components lib entry should keep `{required}` for stable public contract."
        );
    }

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-field_label\")]",
        "out.push_str(crate::field_form::field_label::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css entry should keep `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            root_source.contains(required),
            "ui-components root entry should keep `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "ui-visual-primitive active_highlight should keep shared motion capability `{required}`."
        );
    }

    assert!(
        !path_exists("src/overlay_open.rs"),
        "ui-components should not reintroduce overlay_open.rs; use ui-headless controllable_state."
    );
    assert!(
        !path_exists("src/presence.rs"),
        "ui-components should not reintroduce presence.rs; use ui-headless presence."
    );
    assert!(
        !path_exists("src/a11y.rs"),
        "ui-components should not reintroduce a11y.rs; use ui-headless a11y helpers."
    );
}

#[test]
fn field_label_tree_shaking_feature_contract_is_registered() {
    let cargo_toml_source = load_source("Cargo.toml");

    for required in [
        "component-field_label = [\"dep:ui-field-label\"]",
        "ui-field-label = { path = \"../../components/field-label\", optional = true }",
        "component-domain-field_form = [",
        "\"component-field_label\",",
    ] {
        assert!(
            cargo_toml_source.contains(required),
            "ui-components feature tree should keep `{required}` for field_label tree shaking."
        );
    }

    for forbidden in [
        "ui-field-label = { path = \"../../components/field-label\", optional = false }",
        "component-field_label = [\"ui-field-label\"]",
    ] {
        assert!(
            !cargo_toml_source.contains(forbidden),
            "ui-components should avoid non-gated field_label dependency contract `{forbidden}`."
        );
    }
}

#[test]
fn field_label_docs_page_exists_in_forms_extra() {
    let forms_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "pub(super) fn field_label() -> AnyView",
        "title=\"FieldLabel\"",
        "slug=\"field-label\"",
        "<FieldLabel",
    ] {
        assert!(
            forms_extra.contains(needle),
            "forms_extra docs page should contain `{needle}`."
        );
    }
}

#[test]
fn field_label_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "pub(super) fn field_label() -> AnyView",
        "const FIELD_LABEL_DOC_IMPORTS: &str =",
        "title=\"FieldLabel\"",
        "slug=\"field-label\"",
        "description=\"baseline-compatible field label primitive with centralized tone/required/source-state modeling and stable data contracts.\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Tone + Required\"",
        "title=\"Custom Indicator + Aria + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A for FieldLabel)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "<FieldLabel",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra_field_label docs page should include `{needle}` for field_label primary playground coverage.",
        );
    }
}

#[test]
fn field_label_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "const FIELD_LABEL_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui_components::{FieldLabel, FieldLabelTone};",
        "code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/field_form/field_label/styles.rs\".to_string()",
        "FieldLabel has no controllable value axis; parent passes a full snapshot props set each render.",
        "No value/on_change/default_value triad. Controlled/uncontrolled contrast is N/A.",
        "FieldLabel is snapshot-first; streaming stays optional with snapshot fallback.",
        "data-ui-streaming=optional data-ui-fallback=snapshot data-ui-output-state=verified",
    ] {
        assert!(
            source.contains(needle),
            "field_label docs should keep copy-ready and streaming/snapshot contract `{needle}`.",
        );
    }
}

#[test]
fn field_label_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "title=\"Tone + Required\"",
        "text=\"Email\".to_string()",
        "for_id=\"docs-field-label-email\".to_string()",
        "is_required=true",
        "placeholder=\"name@example.com\"",
        "text=\"Helper\".to_string()",
        "tone=FieldLabelTone::Muted",
        "text=\"Critical\".to_string()",
        "tone=FieldLabelTone::Strong",
        "title=\"Custom Indicator + Aria + Class\"",
        "text=\"Assignee\".to_string()",
        "for_id=\"docs-field-label-assignee\".to_string()",
        "required_indicator=\"(required)\".to_string()",
        "aria_label=\"Assignee field label\".to_string()",
        "class_name=\"docs-field-label-custom\".to_string()",
        "placeholder=\"Owner\"",
    ] {
        assert!(
            source.contains(needle),
            "field_label docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn field_label_docs_workbench_exposes_interactive_playground_controls() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs");

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"field-label-config-controls\"",
        "data-slot=\"field-label-config-summary\"",
        "data-action=\"cycle-tone-config\"",
        "data-action=\"toggle-required-config\"",
        "data-action=\"toggle-disabled-config\"",
        "data-action=\"toggle-for-config\"",
        "data-action=\"toggle-indicator-config\"",
        "data-action=\"toggle-aria-config\"",
        "data-action=\"toggle-class-config\"",
        "text=\"Workbench\".to_string()",
        "id=\"docs-field-label-workbench\"",
    ] {
        assert!(
            source.contains(needle),
            "field_label docs workbench should expose interactive control marker `{needle}`.",
        );
    }
}

#[test]
fn field_label_heroui_parameter_strategy_doc_is_synced_with_component_docs() {
    let source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "### FieldLabel 同步记录（2026-02-21）",
        "`FieldLabel` 维持 form primitive 定位",
        "`text/for_id/is_required/is_disabled/tone/required_indicator/aria_label/class_name/lang/dir`",
        "component_doc!(\"FieldLabel\", \"field-label\", \"Forms\", fxl::field_label)",
        "forms_extra_field_label.rs",
        "Source-first / Copy-Paste Ready",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            source.contains(needle),
            "field_label HeroUI strategy doc should include `{needle}`.",
        );
    }
}

#[test]
fn field_label_component_files_follow_expected_layout_and_no_spec_file() {
    let mod_source = load_source("src/field_form/field_label/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FieldLabel;",
    ] {
        assert!(
            mod_source.contains(needle),
            "field_label module should include `{needle}`."
        );
    }

    assert!(
        !path_exists("src/field_form/field_label/spec.rs"),
        "field_label should not introduce spec.rs for a simple form primitive."
    );
    assert!(
        !path_exists("src/field_form/field_label/render.rs"),
        "field_label should not introduce render.rs; rendering entry stays in view.rs."
    );
    assert!(
        !path_exists("src/field_form/field_label/motion.rs"),
        "field_label should not introduce motion.rs when no component motion semantic exists."
    );

    for forbidden in ["mod motion;", "pub use motion::"] {
        assert!(
            !mod_source.contains(forbidden),
            "field_label module should not expose motion contract marker `{forbidden}`."
        );
    }
}

#[test]
fn field_label_check2_marks_core_sections_complete() {
    let source = load_source("src/field_form/field_label/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] API 命名契约统一",
        "- [x] 状态归一化集中",
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "ui-state-primitives/src/field_label.rs",
        "ui-headless/src/field_label.rs",
        "components/field-label/test/field_label_semantics.rs",
    ] {
        assert!(
            source.contains(needle),
            "FieldLabel check2 should contain completion evidence `{needle}`."
        );
    }
}

#[test]
fn field_label_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/field_form/field_label/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "field_label check2 should not keep unchecked checklist items"
    );
}
