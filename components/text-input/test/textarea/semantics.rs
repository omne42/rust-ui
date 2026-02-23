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
fn textarea_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/textarea/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Textarea internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn textarea_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/text_input/textarea/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Textarea;"),
        "textarea module should export `Textarea`.",
    );
    assert!(
        crate_source.contains("pub use textarea::Textarea;"),
        "crate root should re-export `Textarea`.",
    );
}

#[test]
fn textarea_module_exposes_motion_contract() {
    let source = load_source("src/text_input/textarea/mod.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in ["pub mod motion;", "pub use motion::TextareaMotion;"] {
        assert!(
            source.contains(needle),
            "Textarea module should include `{needle}`.",
        );
    }

    for needle in [
        "pub struct TextareaMotion",
        "pub fn sanitize_motion(motion: TextareaMotion) -> TextareaMotion",
        "pub fn motion_style_vars(motion: TextareaMotion) -> String",
        "pub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea motion should include `{needle}`.",
        );
    }
}

#[test]
fn textarea_module_does_not_define_local_state_primitives() {
    let source = load_source("src/text_input/textarea/mod.rs");

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub struct TextareaStateInput",
        "pub struct TextareaState",
    ] {
        assert!(
            !source.contains(needle),
            "Textarea module should not define local state primitive `{needle}`.",
        );
    }
}

#[test]
fn textarea_logic_exposes_state_helpers() {
    let source = load_source("src/text_input/textarea/logic.rs");

    for needle in [
        "pub use ui_state_primitives::button::normalize_optional_text;",
        "pub use ui_state_primitives::textarea::{",
        "TextareaSourceAttr",
        "TextareaState",
        "TextareaStateInput",
        "resolve_label",
        "resolve_label_with_fallback",
        "resolve_state",
        "pub fn normalize_default_value(default_value: Option<String>) -> String",
        "pub struct ValueAxisInput",
        "pub struct ValueAxisState",
        "pub enum ValueControlModeAttr",
        "pub enum ValueChangeSourceAttr",
        "pub const fn as_str(self) -> &'static str",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub has_controlled_value: bool",
        "pub has_on_value_change: bool",
        "let default_value = normalize_default_value(input.default_value);",
        "ValueControlModeAttr::Controlled",
        "ValueControlModeAttr::Uncontrolled",
        "TextareaSourceAttr::Custom",
        "TextareaSourceAttr::Default",
        "ValueChangeSourceAttr::OnValueChange",
        "ValueChangeSourceAttr::None",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput)",
        "is_disabled: input.is_disabled.unwrap_or(false)",
        "is_read_only: input.is_read_only.unwrap_or(false)",
        "pub fn compose_class_name(class_name: Option<String>, state: TextareaState)",
    ] {
        assert!(
            source.contains(needle),
            "Textarea logic should include `{needle}` for primitive-consumer contracts.",
        );
    }
}

#[test]
fn textarea_view_has_textfield_a11y_and_state_contracts() {
    let source = load_source("src/text_input/textarea/view.rs");

    for needle in [
        "use_focus_ring",
        "use_text_field",
        "use_controllable_state",
        "TextareaMotion",
        "A11yDirection",
        "locale_attrs",
        "value: Option<Signal<String>>",
        "default_value: Option<String>",
        "on_value_change: Option<Callback<String>>",
        "is_disabled: Option<bool>",
        "is_read_only: Option<bool>",
        "is_required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "let value_axis = logic::normalize_value_axis(logic::ValueAxisInput {",
        "has_controlled_value: value.is_some()",
        "has_on_value_change: on_value_change.is_some()",
        "let value_state = use_controllable_state(",
        "value,",
        "Some(value_axis.default_value.clone()),",
        "on_value_change,",
        ");",
        "let request_value_change = value_state.request_change;",
        "let accessibility_state =",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let is_disabled = accessibility_state.is_disabled;",
        "let is_read_only = accessibility_state.is_read_only;",
        "let is_required_input = is_required;",
        "let is_invalid_input = is_invalid;",
        "let is_required = Signal::derive(move || match is_required_input {",
        "let is_invalid = Signal::derive(move || match is_invalid_input {",
        "logic::resolve_label_with_fallback(label, common.textarea_label.as_ref())",
        "logic::resolve_state(logic::TextareaStateInput {",
        "logic::compose_class_name(class_name.clone(), state.get())",
        "let motion = motion::sanitize_motion(motion);",
        "let inline_style = StoredValue::new(Some(motion::motion_style_vars(motion)));",
        "motion::attach_motion(root_ref, is_active, motion);",
        "let locale = locale_attrs(lang, dir);",
        "data-slot=\"textarea\"",
        "style=inline_style.get_value().unwrap_or_default()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-value-controlled=value_axis.is_controlled.then_some(\"true\")",
        "data-value-uncontrolled=(!value_axis.is_controlled).then_some(\"true\")",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "data-has-value-change=value_axis.has_value_change_handler.then_some(\"true\")",
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
    ] {
        assert!(
            source.contains(needle),
            "Textarea view should include `{needle}` to preserve stable contracts.",
        );
    }

    for forbidden in [
        "let is_controlled_value = value.is_some();",
        "let has_default_value = default_value.is_some();",
        "let has_on_value_change = on_value_change.is_some();",
        "let default_value = logic::normalize_default_value(default_value);",
        "let on_value_change = logic::normalize_on_value_change_handler(on_value_change);",
    ] {
        assert!(
            !source.contains(forbidden),
            "Textarea view should not own default-value priority logic; found `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_a11y_i18n_and_locale_contract_is_headless_driven() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let i18n_source = load_source("../ui-headless/src/i18n/common.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "use_ui_i18n",
        "locale_attrs",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "logic::resolve_label_with_fallback(label, common.textarea_label.as_ref())",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "pub textarea_label: Arc<str>,",
        "textarea_label: \"Textarea\".into(),",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || i18n_source.contains(needle)
                || a11y_source.contains(needle),
            "Textarea A11y/i18n/locale contract should include `{needle}`.",
        );
    }

    for forbidden in ["let label = \"Textarea\".to_string();", ">\"Textarea\"<"] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea view should not hardcode user-facing label copy: `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_state_markers_are_observable_queryable_and_source_explicit() {
    let view_source = load_source("src/text_input/textarea/view.rs");

    for needle in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-read-only=is_read_only.then_some(\"true\")",
        "data-required=move || is_required.get().then_some(\"true\")",
        "data-invalid=move || is_invalid.get().then_some(\"true\")",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-value-controlled=value_axis.is_controlled.then_some(\"true\")",
        "data-value-uncontrolled=(!value_axis.is_controlled).then_some(\"true\")",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea should expose stable observable/queryable marker `{needle}`.",
        );
    }
}

#[test]
fn textarea_styles_include_state_and_source_selectors() {
    let source = load_source("src/text_input/textarea/styles.rs");

    for selector in [
        ".ui-textarea[data-state=\"disabled\"]",
        ".ui-textarea[data-state=\"invalid\"]",
        ".ui-textarea[data-state=\"readonly\"]",
        ".ui-textarea[data-value=\"filled\"]",
        ".ui-textarea[data-requirement=\"required\"]",
        ".ui-textarea[data-label-source=\"custom\"]",
        ".ui-textarea[data-description-source=\"custom\"]",
        ".ui-textarea[data-error-source=\"custom\"]",
        ".ui-textarea[data-placeholder-source=\"custom\"]",
        ".ui-textarea[data-rows-source=\"custom\"]",
        ".ui-textarea__textarea:hover:not(:disabled):not([readonly])",
        ".ui-textarea__textarea:active:not(:disabled):not([readonly])",
        ".ui-textarea--custom-class",
        "--ui-textarea-control-bg-hover",
        "--ui-textarea-control-border-hover",
        "--ui-textarea-motion-duration",
        "--ui-textarea-motion-easing",
        "--ui-textarea-label-font-size",
        "--ui-textarea-meta-font-size",
        "prefers-reduced-motion: reduce",
        "--ui-textarea-motion-duration: 1ms;",
    ] {
        assert!(
            source.contains(selector),
            "Textarea styles should include `{selector}` selector.",
        );
    }
}

#[test]
fn textarea_styles_depend_on_semantic_selectors_not_structural_guesses() {
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in [
        ".ui-textarea[data-state=\"disabled\"]",
        ".ui-textarea[data-state=\"invalid\"]",
        ".ui-textarea[data-state=\"readonly\"]",
        ".ui-textarea[data-value=\"filled\"]",
        ".ui-textarea[data-requirement=\"required\"]",
        ".ui-textarea[data-label-source=\"custom\"]",
        ".ui-textarea[data-description-source=\"custom\"]",
        ".ui-textarea[data-error-source=\"custom\"]",
        ".ui-textarea[data-placeholder-source=\"custom\"]",
        ".ui-textarea[data-rows-source=\"custom\"]",
        ".ui-textarea[data-custom-class=\"true\"]",
        ".ui-textarea--focus-visible .ui-textarea__textarea",
        ".ui-textarea--invalid .ui-textarea__textarea",
    ] {
        assert!(
            styles_source.contains(needle),
            "Textarea styles should derive visual states from semantic marker selector `{needle}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        ":has(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Textarea styles should not depend on structural selector guess `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Textarea runtime style should be centralized to motion CSS variable payload."
    );
    for needle in [
        "--ui-textarea-motion-duration",
        "--ui-textarea-motion-easing",
        "pub fn motion_style_vars(motion: TextareaMotion) -> String",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea motion style runtime contract should include `{needle}`.",
        );
    }
}

#[test]
fn textarea_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::textarea::styles::CSS);"),
        "ui css aggregator should include textarea styles.",
    );
}

#[test]
fn textarea_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "title=\"Textarea\"",
        "slug=\"textarea\"",
        "State + Source Markers",
        "data-rows-source",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should contain `{needle}` for textarea.",
        );
    }
}

#[test]
fn textarea_docs_basic_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Basic Textarea\"",
        "id=\"docs-textarea-basic\".to_string()",
        "label=\"About\".to_string()",
        "default_value=\"Write your release summary\".to_string()",
        "placeholder=\"Write something…\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Textarea docs basic playground should contain `{needle}`.",
        );
    }
}

#[test]
fn textarea_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-textarea-marker\".to_string()",
        "label=\"Summary\".to_string()",
        "let on_value_change = Callback::new(move |next: String| {",
        "set_value.set(next);",
        "is_required=true",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
        "on_value_change=on_value_change",
        "on_value_change=on_marker_value_change",
        "description=\"Inspect source/state marker contracts\".to_string()",
        "error=\"Summary must include at least 20 characters.\".to_string()",
        "placeholder=\"Write a summary\".to_string()",
        "rows=5",
        "class_name=\"docs-textarea-state\".to_string()",
        "Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-rows-source`.",
    ] {
        assert!(
            source.contains(needle),
            "Textarea docs state/source playground should contain `{needle}`.",
        );
    }

    assert!(
        !source.contains(
            "<Textarea\n  id=\"summary\".to_string()\n  label=\"Summary\".to_string()\n  value=value\n  set_value=set_value"
        ),
        "Textarea docs should prefer `on_value_change` naming and avoid legacy `set_value` in textarea examples.",
    );
    assert!(
        !source.contains("set_value=set_value_marker"),
        "Textarea docs should prefer `on_value_change` naming and avoid legacy `set_value` in playground usage.",
    );
}

#[test]
fn textarea_docs_page_covers_primary_playgrounds() {
    textarea_docs_page_contains_state_source_playground();
}

#[test]
fn textarea_has_no_async_loading_protocol_and_keeps_sync_input_contract() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea view should stay sync-only and must not contain async protocol token `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "Textarea logic should stay sync-only and must not contain async protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let hello_world = r#"<Textarea id="about".to_string()
  label="About".to_string()
  default_value="Write your release summary".to_string()
  placeholder="Write something…".to_string()
/>"#;

    assert!(
        source.contains(hello_world),
        "Textarea docs should expose a minimal hello-world snippet using default props.",
    );

    let hello_world_line_count = hello_world.lines().count();
    assert!(
        hello_world_line_count <= 5,
        "Textarea hello-world snippet should be <= 5 lines; got {hello_world_line_count} lines.",
    );

    assert!(
        !source.contains(
            "<Textarea id=\"about\".to_string()\n  label=\"About\".to_string()\n  value=value",
        ),
        "Textarea hello-world docs should not require controlled signal wiring.",
    );
}

#[test]
fn textarea_is_leaf_input_without_parent_item_parallel_slot_api() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for forbidden in [
        "children: Children",
        "children: Option<Children>",
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea should stay a leaf input API and must not expose composite-slot token `{forbidden}`.",
        );
        assert!(
            !docs_source.contains(forbidden),
            "Textarea docs should not recommend composite-slot token `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_docs_playgrounds_lock_state_matrix_contract_values() {
    textarea_docs_basic_playground_locks_contract_values();
    textarea_docs_state_source_playground_locks_contract_values();
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn textarea_semantic_contract_matrix_covers_state_paths_and_platform_branches() {
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in [
        "fn normalize_value_axis_centralizes_default_priority_and_sources()",
        "fn normalize_value_axis_tracks_on_value_change_source()",
        "fn normalize_value_axis_uses_closed_enumerated_source_markers()",
        "fn normalize_accessibility_state_prefers_is_prefixed_inputs()",
        "fn normalize_accessibility_state_uses_defaults_when_values_are_absent()",
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            logic_source.contains(needle)
                || view_source.contains(needle)
                || motion_source.contains(needle),
            "Textarea semantic-contract matrix should cover `{needle}`.",
        );
    }

    for forbidden in [
        "assert_snapshot!",
        "insta::assert_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Textarea contract implementation should not rely on visual snapshot primitive `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_component_files_keep_responsibility_boundaries() {
    let mod_source = load_source("src/text_input/textarea/mod.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::TextareaMotion;",
        "pub use view::Textarea;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Textarea mod.rs should keep minimal export boundary token `{needle}`.",
        );
    }
    for forbidden in ["#[component]", "view! {", "pub struct "] {
        assert!(
            !mod_source.contains(forbidden),
            "Textarea mod.rs should not contain implementation detail token `{forbidden}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::textarea::{",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
        "pub fn compose_class_name(class_name: Option<String>, state: TextareaState)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Textarea logic.rs should include normalization/derivation token `{needle}`.",
        );
    }
    for forbidden in [
        "view! {",
        "#[component]",
        "NodeRef<",
        "ui_motion::",
        "<textarea",
        "<div",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Textarea logic.rs should not include rendering/DOM/motion token `{forbidden}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-textarea[data-state=\"disabled\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Textarea styles.rs should include token-first static css token `{needle}`.",
        );
    }
    for forbidden in [
        "use leptos",
        "#[component]",
        "Signal<",
        "Callback<",
        "NodeRef<",
        "ui_headless::",
        "ui_motion::",
        "on:input=",
        "aria-invalid=move ||",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Textarea styles.rs should not include logic/render token `{forbidden}`.",
        );
    }

    for needle in [
        "#[component]",
        "use ui_headless::{",
        "use_controllable_state",
        "use_focus_ring",
        "use_text_field",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "let state = Signal::derive(move || {",
        "data-state=move || state.get().state_attr.as_str()",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea view.rs should include structure + headless-mount token `{needle}`.",
        );
    }
    for forbidden in [
        "pub const CSS: &str",
        "MotionKeyframe::new()",
        "ui_motion::web::animate(",
        "pub fn sanitize_duration_ms(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea view.rs should not include style-engine/motion-engine token `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct TextareaMotion",
        "pub fn sanitize_motion(motion: TextareaMotion) -> TextareaMotion",
        "pub fn motion_style_vars(motion: TextareaMotion) -> String",
        "pub fn attach_motion(",
        "ui_motion::web::animate(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea motion.rs should include motion-contract token `{needle}`.",
        );
    }
    for forbidden in [
        "use_text_field(",
        "use_focus_ring(",
        "aria-invalid=",
        "data-state=",
        "normalize_value_axis(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Textarea motion.rs should not include a11y/state-logic/render token `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_component_directory_has_standard_file_layout() {
    for required in [
        "src/text_input/textarea/mod.rs",
        "src/text_input/textarea/logic.rs",
        "src/text_input/textarea/styles.rs",
        "src/text_input/textarea/view.rs",
        "src/text_input/textarea/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "textarea component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/text_input/textarea/render.rs"),
        "textarea component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
    assert!(
        !path_exists("src/text_input/textarea/spec.rs"),
        "Textarea is a simple component and should not introduce `src/text_input/textarea/spec.rs`."
    );
}

#[test]
fn textarea_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/text_input/textarea/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::TextareaMotion;",
        "pub use view::Textarea;",
    ] {
        assert!(
            mod_source.contains(needle),
            "textarea/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use logic::",
        "pub use self::logic::",
        "pub use styles::",
        "pub use self::styles::",
        "pub mod spec;",
        "pub use spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "textarea/mod.rs should not over-export internal marker `{forbidden}`."
        );
    }
}

#[test]
fn textarea_component_file_responsibilities_remain_scoped() {
    textarea_component_files_keep_responsibility_boundaries();
}

#[test]
fn textarea_keeps_simple_surface_without_spec_module_sprawl() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/text_input/textarea/spec.rs");
    let mod_source = load_source("src/text_input/textarea/mod.rs");

    assert!(
        !spec_path.exists(),
        "Textarea is a simple component and should not introduce `src/text_input/textarea/spec.rs`."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "pub use self::spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Textarea module boundary should not expose spec module token `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_component_files_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_directory_has_standard_file_layout",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_uses_token_first_static_css_pipeline_without_utility_or_css_in_rust() {
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "--ui-textarea-label-font-size: var(--ui-font-size-150);",
        "--ui-textarea-control-bg: var(--ui-bg);",
        "--ui-textarea-control-border: var(--ui-border);",
        "gap: var(--ui-space-xs);",
        "padding: var(--ui-space-sm) var(--ui-space-md);",
        "border: 1px solid var(--ui-textarea-control-border);",
        "border-radius: var(--ui-radius-md);",
        "background: var(--ui-textarea-control-bg);",
        "color: var(--ui-fg);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Textarea styles should stay token-first and include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-textarea\")]",
        "out.push_str(crate::textarea::styles::CSS);",
        "crate::css::push_components_css(&mut out);",
        "if inject_components_css.get_value() {",
    ] {
        assert!(
            css_source.contains(needle) || root_source.contains(needle),
            "Textarea css contract should include stable injection pipeline token `{needle}`.",
        );
    }

    assert!(
        view_source.contains("style=inline_style.get_value().unwrap_or_default()"),
        "Textarea runtime styling should pass only compact CSS variable payload."
    );
    for needle in [
        "--ui-textarea-motion-duration",
        "--ui-textarea-motion-easing",
        "pub fn motion_style_vars(motion: TextareaMotion) -> String",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea runtime style payload should stay CSS-variable-only and include `{needle}`.",
        );
    }

    for forbidden in [
        "@apply",
        "tailwind",
        "class=\"flex",
        "class_name=\"flex",
        "style!(",
        "css!(",
        "stylist::",
        "styled_components",
        "emotion::",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Textarea component should not depend on utility-first/css-in-rust token `{forbidden}`.",
        );
    }
}

#[test]
fn textarea_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "--ui-textarea-label-font-size",
        "--ui-textarea-meta-font-size",
        "gap: var(--ui-space-xs);",
        "font-weight: 500;",
        ".ui-textarea__textarea:hover:not(:disabled):not([readonly])",
        ".ui-textarea__textarea:active:not(:disabled):not([readonly])",
        ".ui-textarea--focus-visible .ui-textarea__textarea",
    ] {
        assert!(
            styles_source.contains(needle),
            "Textarea default-theme visual quality should include style contract token `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline page should keep visual-quality contract token `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment contract token `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn textarea_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "inject-css = []",
        "component-textarea",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-textarea\")]\n#[path = \"text_input/textarea/mod.rs\"]\npub mod textarea;"
        ),
        "lib.rs should feature-gate textarea module export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-textarea\")]")
            && css_source.contains("out.push_str(crate::textarea::styles::CSS);"),
        "css.rs should gate textarea CSS aggregation behind component-textarea feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn textarea_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn textarea_type_system_and_machine_readable_markers_stay_in_sync() {
    let primitive_source = load_source("../ui-state-primitives/src/textarea.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");

    for needle in [
        "pub enum TextareaVisualStateAttr",
        "pub enum TextareaValueAttr",
        "pub enum TextareaRequirementAttr",
        "pub enum TextareaSourceAttr",
        "pub const fn as_str(self) -> &'static str",
        "state_attr: TextareaVisualStateAttr",
        "value_attr: TextareaValueAttr",
        "requirement_attr: TextareaRequirementAttr",
        "label_source_attr: TextareaSourceAttr",
        "description_source_attr: TextareaSourceAttr",
        "error_source_attr: TextareaSourceAttr",
        "placeholder_source_attr: TextareaSourceAttr",
        "rows_source_attr: TextareaSourceAttr",
        "class_source_attr: TextareaSourceAttr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Textarea primitive should keep typed state/source contract token `{needle}`.",
        );
    }

    for needle in [
        "pub enum ValueControlModeAttr",
        "pub enum ValueChangeSourceAttr",
        "pub control_mode_attr: ValueControlModeAttr",
        "pub default_value_source_attr: TextareaSourceAttr",
        "pub value_change_source_attr: ValueChangeSourceAttr",
        "ValueControlModeAttr::Controlled",
        "ValueControlModeAttr::Uncontrolled",
        "ValueChangeSourceAttr::OnValueChange",
        "ValueChangeSourceAttr::None",
    ] {
        assert!(
            logic_source.contains(needle),
            "Textarea logic should keep typed value-axis contract token `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea view should expose machine-readable marker mapping token `{needle}`.",
        );
    }
}

#[test]
fn textarea_platform_guards_keep_non_wasm_files_web_sys_free() {
    let mod_source = load_source("src/text_input/textarea/mod.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "window()",
        "document()",
        "js_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "non-wasm Textarea files should stay browser-object free; found `{forbidden}`."
        );
    }
}

#[test]
fn textarea_motion_covers_wasm_and_non_wasm_contract_paths() {
    let motion_source = load_source("src/text_input/textarea/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea motion should keep explicit platform contract token `{needle}`."
        );
    }
}

#[test]
fn textarea_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: textarea native path\"",
        "cargo check -p ui --no-default-features --features component-textarea,inject-css",
        "echo \"[platform] compile-only: textarea wasm path\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-textarea,inject-css",
        "echo \"[platform] source guard: non-wasm textarea files must not reference web_sys\"",
        "echo \"[platform] source guard: textarea motion must keep explicit wasm/non-wasm branches\"",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}` for native/ssr/wasm textarea evidence."
        );
    }
}

#[test]
fn textarea_ui_headless_web_ssr_mutex_is_compile_error_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn textarea_platform_script_enforces_ui_headless_web_ssr_mutex() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex via `{needle}`."
        );
    }
}

#[test]
fn textarea_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }
}

#[test]
fn textarea_platform_script_covers_ui_motion_native_wasm_and_stub_paths() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should cover ui-motion stub/compile path token `{needle}`."
        );
    }
}

#[test]
fn textarea_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_spring_checks_source = load_source("../ui-motion/tests/spring.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion downgrade behavior token `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_checks_source.contains(needle),
            "ui-motion reduced-motion regression tests should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"textarea\"",
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea should keep hydration-stable semantic marker `{needle}` across SSR/wasm paths.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "prefers_reduced_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Textarea semantic surface should not split by platform/reduced-motion token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Textarea motion adapter should keep wasm/non-wasm split token `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-textarea,inject-css",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/SSR/wasm verification token `{needle}`.",
        );
    }
}

#[test]
fn textarea_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/text_input/textarea/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/text_input/textarea/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget contract token `{needle}`."
        );
    }

    {
        let needle = "component_doc!(\"Textarea\", \"textarea\", \"Forms\", forms_extra::textarea)";
        assert!(
            pages_source.contains(needle),
            "Textarea docs page should remain in coverage traversal via `{needle}`."
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
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf regression marker `{needle}`."
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
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep explicit render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Textarea checklist should keep perf budget/follow-up governance token `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea view should expose attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn textarea_view_macro_complexity_is_bounded_with_semantic_subblocks() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "Textarea should keep an explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "Textarea should keep one primary view block with two local semantic subblocks."
    );
    assert!(
        view_source.lines().count() <= 240,
        "Textarea view.rs should stay bounded; split semantic subrenders if this grows significantly."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        ".fold(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea view should avoid loop-heavy/expansion-heavy rendering token `{forbidden}`."
        );
    }

    for needle in [
        "fn render_description_block(",
        "fn render_error_block(",
        "{description_view}",
        "{error_view}",
        "<Show when=move || is_invalid.get()>",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea view should keep semantic subblock marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_macro_complexity_is_bounded_with_semantic_subblocks";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn textarea_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Textarea should keep a single public component boundary for current layout."
    );

    for needle in [
        "fn render_description_block(description: Option<String>, description_id: String) -> AnyView",
        "fn render_error_block(",
        "match description {",
        "match error {",
        "None => ().into_any(),",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea view should prefer plain function split marker `{needle}`."
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\nfn textarea_"] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea should not introduce local component abstraction noise `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn textarea_static_fragments_are_constantized_or_absent_for_simple_input_layout() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/textarea.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("src/text_input/textarea/check2.md");

    for forbidden in [
        "inner_html=",
        "<svg",
        "<path",
        "<footer",
        "<article",
        "<aside",
        "let markdown",
        "let long_text",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea view should avoid heavy inline static fragments for simple input layout `{forbidden}`."
        );
    }

    for needle in [
        "data-slot=\"textarea\"",
        "data-slot=\"textarea-label\"",
        "data-slot=\"textarea-input\"",
        "data-slot=\"textarea-description\"",
        "data-slot=\"textarea-error\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Textarea should keep stable static slot marker `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Textarea\";",
        "logic::resolve_label_with_fallback(label, common.textarea_label.as_ref())",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            primitives_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Textarea static/a11y fragment path should stay centralized and traceable via `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_static_fragments_are_constantized_or_absent_for_simple_input_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
        "textarea_static_fragments_are_constantized_or_absent_for_simple_input_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "Textarea checklist should keep static-fragment constantization completion evidence `{needle}`."
        );
    }
}

#[test]
fn textarea_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/text_input/textarea/mod.rs",
        "src/text_input/textarea/logic.rs",
        "src/text_input/textarea/styles.rs",
        "src/text_input/textarea/view.rs",
        "src/text_input/textarea/motion.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "Textarea source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Textarea docs examples must not contain raw-html injection token `{forbidden}`."
        );
    }
}

#[test]
fn textarea_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    let needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn textarea_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let textarea_view_source = load_source("src/text_input/textarea/view.rs");
    let textarea_logic_source = load_source("src/text_input/textarea/logic.rs");
    let docs_textarea_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("textarea-wasm-debug"),
        "Textarea should not define a component-local wasm-debug feature that leaks into production API surface."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`."
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
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep visual/temporal trace marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep typed timestamp/source event marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
    ] {
        assert!(
            textarea_view_source.contains(needle),
            "Textarea should expose reproducible interaction/state markers for debug tracing via `{needle}`."
        );
    }

    for needle in [
        "let on_marker_value_change = Callback::new(move |next: String| set_value_marker.set(next));",
        "set_marker_invalid.update(|value| *value = !*value)",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_textarea_source.contains(needle),
            "Textarea docs playground should keep minimal replay path marker `{needle}`."
        );
    }

    let combined = format!("{textarea_view_source}\n{textarea_logic_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Textarea component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }
}

#[test]
fn textarea_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn textarea_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn textarea_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("src/text_input/textarea/check2.md");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (value_marker, set_value_marker) = signal(\"Pending review\".to_string());",
        "let on_marker_value_change = Callback::new(move |next: String| set_value_marker.set(next));",
        "let (marker_invalid, set_marker_invalid) = signal(false);",
        "on_value_change=on_marker_value_change",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
        "set_marker_invalid.update(|value| *value = !*value)",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "TEXTAREA_WORKBENCH_STORAGE_KEY",
        "load_textarea_workbench_state(",
        "save_textarea_workbench_state(",
        "clear_textarea_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Textarea keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "Textarea checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn textarea_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/text_input/textarea/mod.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    assert!(
        !manifest_dir
            .join("src/text_input/textarea/spec.rs")
            .exists(),
        "Textarea should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-textarea = []"),
        "Textarea feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-textarea = [\"dep:serde\"")
            && !cargo_source.contains("component-textarea = [\"dep:serde_json\""),
        "Textarea should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Textarea engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
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
            "Textarea checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn textarea_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/text_input/textarea/mod.rs"),
        load_source("src/text_input/textarea/logic.rs"),
        load_source("src/text_input/textarea/view.rs"),
        load_source("src/text_input/textarea/styles.rs"),
        load_source("src/text_input/textarea/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("textarea-wasm-debug"),
        "Textarea should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::textarea::",
        "const TEXTAREA_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Textarea should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn textarea_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/text_input/textarea/mod.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");

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
                "Textarea engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Textarea public module boundary should not leak web_sys types."
    );
}

#[test]
fn textarea_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep ui entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn textarea_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-textarea\")]",
        "pub mod textarea;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep marker `{needle}`."
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
            "ui lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn textarea_ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-textarea\")]",
        "out.push_str(crate::textarea::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn textarea_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
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
}

#[test]
fn textarea_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn textarea_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui forbidden entrypoint file should not exist: `{forbidden}`."
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

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

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
fn textarea_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn textarea_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn textarea_agent_contract_markers_are_schema_like_and_machine_readable() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/textarea.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for marker in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-read-only=is_read_only.then_some(\"true\")",
        "data-required=move || is_required.get().then_some(\"true\")",
        "data-invalid=move || is_invalid.get().then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(marker),
            "Textarea should expose agent-readable machine marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum ValueControlModeAttr",
        "pub enum ValueChangeSourceAttr",
        "pub struct ValueAxisInput",
        "pub struct ValueAxisState",
        "pub enum TextareaVisualStateAttr",
        "pub enum TextareaValueAttr",
        "pub enum TextareaRequirementAttr",
        "pub enum TextareaSourceAttr",
        "pub struct TextareaState",
        "pub fn resolve_state(input: TextareaStateInput) -> TextareaState",
        "pub const fn as_str(self) -> &'static str",
    ] {
        assert!(
            combined.contains(typed_source),
            "Textarea Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }
}

#[test]
fn textarea_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/textarea.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for forbidden in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "intent=\"",
        "action=\"",
        "format!(\"data-",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Textarea should avoid free-form/fake schema field token `{forbidden}`."
        );
    }

    for required_interaction in [
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(required_interaction),
            "Textarea interactive intent/action path should remain explicit via `{required_interaction}`."
        );
    }
}

#[test]
fn textarea_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let styles_source = load_source("src/text_input/textarea/styles.rs");
    let mod_source = load_source("src/text_input/textarea/mod.rs");
    let motion_source = load_source("src/text_input/textarea/motion.rs");
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
            "Textarea Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_markers_are_schema_like_and_machine_readable",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn textarea_streaming_check_script_covers_llm_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn textarea_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn textarea_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/textarea.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for marker in [
        "#[component]",
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "let value_state = use_controllable_state(",
        "Some(value_axis.default_value.clone()),",
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "Textarea snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub struct ValueAxisInput",
        "pub struct ValueAxisState",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub struct TextareaStateInput",
        "pub struct TextareaState",
        "pub fn resolve_state(input: TextareaStateInput) -> TextareaState",
    ] {
        assert!(
            logic_source.contains(marker) || primitives_source.contains(marker),
            "Textarea snapshot baseline should keep stable normalization/state marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn textarea() -> AnyView",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "id=\"docs-textarea-basic\".to_string()",
        "default_value=\"Write your release summary\".to_string()",
        "placeholder=\"Write something…\".to_string()",
        "title=\"State + Source Markers\"",
        "id=\"docs-textarea-marker\".to_string()",
        "description=\"Inspect source/state marker contracts\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "Textarea docs should include complete snapshot result marker `{marker}`."
        );
    }
}

#[test]
fn textarea_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_snapshot_as_default_baseline_capability";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn textarea_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Textarea` 归类为 `Streaming Optional` 且当前实现为 `N/A`",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn textarea_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/text_input/textarea/view.rs");

    for required in [
        "data-slot=\"textarea\"",
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-class-source=move || state.get().class_source_attr.as_str()",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
    ] {
        assert!(
            view_source.contains(required),
            "Textarea should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }

    for forbidden in [
        "data-ui-output-status",
        "data-output-status",
        "data-stream-status",
        "data-status=\"draft\"",
        "data-status=\"verified\"",
        "data-status=\"committed\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Textarea should not mount fake streaming status field `{forbidden}` when stream protocol is N/A."
        );
    }
}

#[test]
fn textarea_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "revalidate",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Textarea should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn textarea_streaming_check_script_covers_streaming_responsibility_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn textarea_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/textarea/semantics.rs");

    for required in [
        "textarea_view_has_textfield_a11y_and_state_contracts",
        "textarea_state_markers_are_observable_queryable_and_source_explicit",
        "textarea_semantic_contract_matrix_covers_state_paths_and_platform_branches",
        "data-state=move || state.get().state_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
    ] {
        assert!(
            semantics_source.contains(required),
            "Textarea semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden = [
        ["assert", "_snapshot!("].concat(),
        ["insta::assert", "_snapshot!("].concat(),
        ["to_match", "_snapshot("].concat(),
        ["image", "_snapshot("].concat(),
    ];

    for forbidden in forbidden {
        assert!(
            !semantics_source.contains(&forbidden),
            "Textarea semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn textarea_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/text_input/textarea/view.rs");
    let semantics_source = load_source("tests/textarea/semantics.rs");

    for marker in [
        "data-state=move || state.get().state_attr.as_str()",
        "data-value=move || state.get().value_attr.as_str()",
        "data-requirement=move || state.get().requirement_attr.as_str()",
        "data-label-source=move || state.get().label_source_attr.as_str()",
        "data-description-source=move || state.get().description_source_attr.as_str()",
        "data-error-source=move || state.get().error_source_attr.as_str()",
        "data-placeholder-source=move || state.get().placeholder_source_attr.as_str()",
        "data-rows-source=move || state.get().rows_source_attr.as_str()",
        "data-value-control-mode=value_axis.control_mode_attr.as_str()",
        "data-default-value-source=value_axis.default_value_source_attr.as_str()",
        "data-value-change-source=value_axis.value_change_source_attr.as_str()",
        "aria-describedby=move || aria.input.aria_describedby.get()",
        "aria-invalid=move || aria.input.aria_invalid.get()",
        "aria-required=move || aria.input.aria_required.get()",
        "on:input=move |ev| request_value_change.run(event_target_value(&ev))",
    ] {
        assert!(
            view_source.contains(marker),
            "Textarea view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Textarea semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_semantics_first_contract_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn textarea_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_textarea_contract.spec.mjs");

    for needle in [
        "/#/components/textarea",
        "body:not(:has(#boot))",
        "[data-component=\"textarea\"] section.playground",
        "#docs-textarea-marker",
        "[data-slot=\"textarea\"]",
        "[data-slot=\"button\"]",
        "toHaveAttribute(\"data-value-control-mode\", \"controlled\")",
        "toHaveAttribute(\"data-default-value-source\", \"default\")",
        "toHaveAttribute(\"data-value-change-source\", \"on_value_change\")",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-invalid\", \"true\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "[data-slot=\"textarea-error\"]",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Textarea e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Textarea e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn textarea_e2e_check_script_covers_selector_contract() {
    let script_source = load_source("../../components/text-input/scripts/check-ui-e2e-textarea.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
    ] {
        assert!(
            script_source.contains(needle),
            "textarea e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep repeatable-key-flow rule `{required}`."
        );
    }
}

#[test]
fn textarea_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_textarea_contract.spec.mjs");

    for needle in [
        "docs-app textarea key flow is repeatable with semantic breakpoints",
        "await input.focus();",
        "await expect(input).toBeFocused();",
        "await page.keyboard.type(\" semantic-key-flow\");",
        "await toggleInvalid.focus();",
        "await expect(toggleInvalid).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-invalid\", \"true\")",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"ready\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Textarea e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Textarea e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn textarea_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../components/text-input/scripts/check-ui-e2e-textarea.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "textarea e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn textarea_docs_examples_sync_with_logic_api_names_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/textarea.rs");

    textarea_docs_page_covers_primary_playgrounds();
    textarea_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "title=\"Basic Textarea\"",
        "title=\"State + Source Markers\"",
        "default_value=\"Write your release summary\".to_string()",
        "value=value_marker",
        "on_value_change=on_marker_value_change",
        "is_required=true",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should keep API/default/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub enum ValueControlModeAttr",
        "pub enum ValueChangeSourceAttr",
        "pub struct TextareaStateInput",
        "pub struct TextareaState",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "Textarea public/default contract should keep `{needle}`."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_examples_sync_with_logic_api_names_and_state_matrix",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/text_input/textarea/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    assert!(
        has_readme || has_docs_page,
        "Textarea must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn textarea() -> AnyView"),
        "Equivalent docs entry should expose textarea page function."
    );
}

#[test]
fn textarea_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("src/text_input/textarea/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Textarea checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Textarea\"",
        "slug=\"textarea\"",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "<Playground",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let basic_pos = docs_source
        .find("<Playground title=\"Basic Textarea\" code_signal=basic_code>")
        .expect("textarea docs should include basic playground");
    let advanced_pos = docs_source
        .find("title=\"State + Source Markers\"")
        .expect("textarea docs should include state/source playground");

    assert!(
        basic_pos < advanced_pos,
        "Textarea docs should present default usage before advanced controls."
    );
}

#[test]
fn textarea_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let start = docs_source
        .find("let basic_code = Signal::derive(move || {")
        .expect("textarea docs should define basic_code");
    let end = docs_source[start..]
        .find("let markers_code = Signal::derive(move || {")
        .map(|offset| start + offset)
        .expect("textarea docs should define markers_code after basic_code");
    let basic_block = &docs_source[start..end];

    let snippet_start = basic_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("basic snippet should be embedded as raw string");
    let snippet_end = basic_block[snippet_start..]
        .find("\"#")
        .map(|offset| snippet_start + offset)
        .expect("basic snippet should terminate raw string");
    let basic_snippet = &basic_block[snippet_start..snippet_end];
    let meaningful_lines = basic_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "Textarea Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{basic_snippet}"
    );

    for forbidden in [
        "ui_state_primitives",
        "ui-headless",
        "ui_headless",
        "state=",
        "controller=",
        "Signal<",
    ] {
        assert!(
            !basic_snippet.contains(forbidden),
            "Textarea Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn textarea_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/text_input/textarea/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "textarea_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "textarea_docs_are_beginner_friendly_with_default_then_advanced_path",
        "textarea_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "Textarea checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_documentation_as_product_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn textarea_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "title=\"State + Source Markers\"",
        "let (value_marker, set_value_marker) = signal(\"Pending review\".to_string());",
        "let on_marker_value_change = Callback::new(move |next: String| set_value_marker.set(next));",
        "let (marker_invalid, set_marker_invalid) = signal(false);",
        "on_value_change=on_marker_value_change",
        "is_invalid=Signal::derive(move || marker_invalid.get())",
        "set_marker_invalid.update(|value| *value = !*value)",
        "if marker_invalid.get() { \"Clear marker invalid\" } else { \"Mark marker invalid\" }",
        "description=\"Inspect source/state marker contracts\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should provide interactive playground marker `{needle}`."
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
            "docs-app Playground should keep interactive preview contract `{needle}`."
        );
    }
}

#[test]
fn textarea_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_textarea_contract.spec.mjs");

    for needle in [
        "docs-app textarea key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/textarea\");",
        "[data-component=\"textarea\"] section.playground",
        "#docs-textarea-marker",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"ready\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Textarea interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn textarea_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("src/text_input/textarea/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Textarea checklist should keep source-first copy-ready rule `{required}`."
        );
    }
}

#[test]
fn textarea_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code-block/src/view.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "let basic_code = Signal::derive(move || {",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "let markers_code = Signal::derive(move || {",
        "title=\"State + Source Markers\"",
        "code_signal=markers_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "Textarea docs should keep copy-ready snippet anchor `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let resolved_code = Signal::derive(move || {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app playground should keep copy-paste pipeline marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep one-click copy marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub enum ValueControlModeAttr",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Textarea docs copy-ready snippets should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn textarea_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/text_input/textarea/view.rs");
    let logic_source = load_source("src/text_input/textarea/logic.rs");

    for needle in [
        "### Textarea 同步记录（2026-02-17）",
        "value + on_value_change + default_value",
        "is_required/is_invalid/is_disabled/is_read_only",
        "component_doc!(\"Textarea\", \"textarea\", \"Forms\", forms_extra::textarea)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs",
        "`Basic Textarea` 与 `State + Source Markers`",
        "compose_copy_ready_code",
        "参数语义若变更，必须先更新本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_index_source.contains(needle),
            "Textarea HeroUI/doc sync record should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn textarea() -> AnyView",
        "slug=\"textarea\"",
        "title=\"Textarea\"",
        "<Playground title=\"Basic Textarea\" code_signal=basic_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "Textarea docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<String>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<String>>",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Textarea parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn textarea_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/text_input/textarea/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "textarea_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "Textarea checklist should keep HeroUI/doc sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn textarea_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui --test textarea_semantics --no-default-features --features component-textarea,inject-css textarea_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}
