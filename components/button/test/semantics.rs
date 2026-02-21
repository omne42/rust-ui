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
fn collect_spec_files(root: &Path, base: &Path, out: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_spec_files(&path, base, out);
                continue;
            }

            if path.file_name().and_then(|name| name.to_str()) == Some("spec.rs")
                && let Ok(rel) = path.strip_prefix(base)
            {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

fn collect_component_src_spec_files(out: &mut Vec<String>) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let components_root = workspace_dir.join("components");

    collect_spec_files(&components_root, &components_root, out);
    out.retain(|rel| rel.ends_with("/src/spec.rs"));
}

#[test]
fn button_does_not_expose_logic_module() {
    let source = load_source("src/button/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Button's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn button_uses_headless_press_hover_and_focus_ring() {
    let source = load_source("src/button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "Button should use headless `{needle}` hooks to align behavior with global focus-visible/modality providers."
        );
    }
}

#[test]
fn button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "data-slot=SLOT_BUTTON",
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-hovered",
        "data-pressed",
        "data-loading",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
        "data-loading-placement",
        "data-motion-source=if state.has_custom_motion",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-schema-payload=normalized_schema.schema_json",
        "data-ui-schema-input-source=normalized_schema.source.as_attr()",
    ] {
        assert!(
            source.contains(attr),
            "Button should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn button_exposes_agent_capabilities_for_machine_consumers() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "ButtonAgentContract",
        "ButtonAgentCapabilities",
        "ButtonAgentAction",
        "ButtonAgentSource",
        "ButtonSchemaInputSource",
        "resolve_agent_contract(state, has_popup_trigger)",
        "normalize_schema_json_input(schema_json)",
        "data-ui-agent-schema=agent_contract.schema_name",
        "data-ui-agent-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-capability-press=agent_contract.capabilities.can_press.then_some(\"true\")",
        "data-ui-capability-focus=agent_contract.capabilities.can_focus.then_some(\"true\")",
        "data-ui-capability-hover=agent_contract.capabilities.can_hover.then_some(\"true\")",
        "data-ui-capability-popup-trigger=agent_contract",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Button should expose typed agent capability contract marker `{needle}`."
        );
    }
}

#[test]
fn button_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered() {
    let check2 = load_source("../../components/button/check2.md");
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let component_manifest = load_source("../../components/button/src/Component.toml");
    let component_rbi = load_source("../../components/button/src/button.rbi");

    for typed_marker in [
        "pub const BUTTON_AGENT_SCHEMA: &str = \"ui.button.agent-contract\";",
        "pub enum ButtonAgentSchemaVersion",
        "pub enum ButtonAgentIntent",
        "pub enum ButtonAgentAction",
        "pub enum ButtonAgentStateAxis",
        "pub enum ButtonAgentSource",
        "pub struct ButtonAgentContract",
        "pub enum ButtonSchemaInputSource",
        "pub struct ButtonSchemaInputNormalization",
        "pub fn normalize_schema_json_input(",
        "ButtonSchema::from_json(&raw).and_then(|schema| schema.to_json_result())",
    ] {
        assert!(
            logic_source.contains(typed_marker),
            "button logic should keep typed Agent Contract marker `{typed_marker}`.",
        );
    }

    for marker in [
        "let normalized_schema = logic::normalize_schema_json_input(schema_json);",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-schema-payload=normalized_schema.schema_json",
        "data-ui-schema-input-source=normalized_schema.source.as_attr()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-loading-source=view_state.source.loading_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "button view should mount schema-typed Agent marker `{marker}`.",
        );
    }

    for marker in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"whitelist_render_policy_no_script_injection\"",
        "pub const BUTTON_AGENT_SCHEMA: &str;",
        "pub enum ButtonAgentAction {",
        "pub enum ButtonAgentSource {",
        "pub enum ButtonSchemaInputSource {",
    ] {
        assert!(
            component_manifest.contains(marker) || component_rbi.contains(marker),
            "button context-compression assets should keep agent marker `{marker}`.",
        );
    }

    for forbidden in [
        "data-ui-action=\"",
        "data-ui-state=\"",
        "format!(\"data-ui-",
        "inner_html=",
        "javascript:",
        "<script",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "button render path should keep whitelist-only boundary without `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("语义标记统一升级为 Agent Contract（Schema 化）"),
        "button check2 should keep Agent Contract governance entry.",
    );
}

#[test]
fn button_check2_marks_agent_contract_schema_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "components/button/src/logic.rs",
        "components/button/src/view.rs",
        "button_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered",
        "normalize_schema_json_input_enforces_typed_whitelist_boundary",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep agent-contract schema marker `{required}`.",
        );
    }
}

#[test]
fn button_forwards_headless_button_semantics() {
    let source = load_source("src/button/view.rs");

    for attr in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Button should forward headless attrs via `{attr}` for correct custom-element semantics."
        );
    }
}

#[test]
fn button_mounts_popup_a11y_contract_from_ui_headless() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "popup_trigger_attrs(",
        "aria-haspopup=popup_a11y.aria_haspopup",
        "aria-controls=move || popup_a11y.aria_controls.get()",
        "aria-expanded=move || popup_a11y.aria_expanded.get()",
        "lang=popup_a11y.lang.clone()",
        "dir=popup_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Button should mount headless popup a11y contract via `{needle}`."
        );
    }
}

#[test]
fn button_a11y_and_i18n_entrypoints_are_wired_via_headless_contracts() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "icon_only_fallback_aria_label: Some(common_strings.icon_button_aria_label.to_string())",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "lang=popup_a11y.lang.clone()",
        "dir=popup_a11y.dir",
        "popup_trigger_attrs(",
    ] {
        assert!(
            view_source.contains(needle),
            "Button should expose a11y/i18n integration point via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("input.icon_only_fallback_aria_label"),
        "Button logic should consume i18n-provided fallback aria label through typed normalization input.",
    );
}

#[test]
fn button_family_a11y_i18n_l10n_contract_is_complete_and_headless_sourced() {
    let root_view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let copy_view_source = load_source("src/button/copy/view.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for required in [
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-label=normalized_aria_label",
        "aria-haspopup=popup_a11y.aria_haspopup",
        "aria-controls=move || popup_a11y.aria_controls.get()",
        "aria-expanded=move || popup_a11y.aria_expanded.get()",
        "aria-live=\"polite\"",
        "use_ui_i18n();",
        "i18n.strings::<CommonStrings>();",
        "strings::<super::i18n::ButtonCopyStrings>();",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "lang=popup_a11y.lang.clone()",
        "dir=popup_a11y.dir",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
        "let locale = ui_headless::a11y::locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "labeled_group_attrs(",
        "labeled_toolbar_attrs(",
        "popup_trigger_attrs(",
    ] {
        assert!(
            root_view_source.contains(required)
                || action_view_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_button_view_source.contains(required)
                || copy_view_source.contains(required),
            "Button family should expose a11y/i18n/l10n contract marker `{required}`.",
        );
    }

    for required in [
        "pub fn locale_attrs(",
        "pub fn labeled_group_attrs(",
        "pub fn labeled_toolbar_attrs(",
        "pub fn popup_trigger_attrs(",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "Shared a11y primitives should stay in ui-headless marker `{required}`.",
        );
    }

    for forbidden in ["Action button group", "Action group"] {
        assert!(
            !root_view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden)
                && !copy_view_source.contains(forbidden),
            "view layer should not hardcode user-facing fallback text `{forbidden}`.",
        );
    }
}

#[test]
fn button_loading_forces_disabled_and_sets_aria_busy() {
    let source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    assert!(
        source.contains("resolve_view_state(") && logic_source.contains("pub fn resolve_state("),
        "Button should derive view data via `resolve_view_state` while keeping `resolve_state` testable in logic.rs."
    );

    for needle in [
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Button should wire loading/disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn button_async_contract_is_externalized_and_has_no_internal_retry_protocol() {
    let view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");

    for required in [
        "#[prop(optional)] is_loading: bool",
        "disabled=state.is_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "is_loading: input.is_loading,",
    ] {
        assert!(
            view_source.contains(required)
                || action_view_source.contains(required)
                || logic_source.contains(required),
            "Button async mapping should include `{required}`."
        );
    }

    for forbidden in [
        "is_error",
        "error_message",
        "on_retry",
        "retry_count",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !spec_source.contains(forbidden),
            "Button should not define component-local async error/retry protocol `{forbidden}`.",
        );
    }
}

#[test]
fn button_copy_async_failure_state_has_recoverable_path_and_semantic_markers() {
    let copy_view_source = load_source("src/button/copy/view.rs");
    let snippet_logic_source = load_source("../../components/snippet/src/logic.rs");
    let headless_snippet_source = load_source("../../crates/ui-headless/src/snippet.rs");

    for needle in [
        "is_loading=is_copying",
        "on_press=logic.copy",
        "data-copy-error=move || logic.has_copy_error.get().then_some(\"true\")",
        "data-copy-status=move || {",
        "aria-live=\"polite\"",
        "copy_failed_status_text.get_value()",
    ] {
        assert!(
            copy_view_source.contains(needle),
            "ButtonCopy should expose async error/recovery semantics marker `{needle}`."
        );
    }

    for needle in [
        "retry_copy: contract.handlers.on_retry",
        "has_copy_error: contract.state.has_error",
    ] {
        assert!(
            snippet_logic_source.contains(needle),
            "Snippet logic adapter should preserve async error/retry signal `{needle}`."
        );
    }

    for needle in [
        "let on_retry = on_copy;",
        "let aria_busy = Signal::derive(move || is_loading.get().then_some(\"true\"));",
    ] {
        assert!(
            headless_snippet_source.contains(needle),
            "Headless snippet primitive should keep unified async recovery contract `{needle}`."
        );
    }
}

#[test]
fn button_has_spring_driven_scale_css_variable() {
    let styles = load_source("src/button/styles.rs");
    let motion = load_source("src/button/motion.rs");

    for needle in [
        "--ui-button-scale",
        "transform: scale(var(--ui-button-scale",
    ] {
        assert!(
            styles.contains(needle),
            "Button styles should reference `{needle}` for spring-driven interaction scaling."
        );
    }

    assert!(
        motion.contains("--ui-button-scale"),
        "Button motion should write `--ui-button-scale` to drive interaction feedback without triggering rerenders."
    );
}

#[test]
fn button_spinner_respects_reduced_motion() {
    let styles = load_source("src/button/styles.rs");

    for needle in ["@media (prefers-reduced-motion: reduce)", "animation: none"] {
        assert!(
            styles.contains(needle),
            "Button spinner should disable its CSS animation under reduced-motion via `{needle}`."
        );
    }
}

#[test]
fn button_styles_consume_theme_layout_variables() {
    let styles = load_source("src/button/styles.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");

    for var_name in [
        "--ui-button-min-width",
        "--ui-button-font-size",
        "--ui-button-spinner-size",
        "--ui-button-spinner-border",
        "--ui-button-spinner-duration",
        "--ui-button-focus-outline-width",
        "--ui-button-focus-outline-offset",
        "--ui-button-radius-full",
        "--ui-button-size-xs-height",
        "--ui-button-size-xs-min-width",
        "--ui-button-size-xs-font-size",
        "--ui-button-size-xs-line-height",
        "--ui-button-size-s-height",
        "--ui-button-size-s-min-width",
        "--ui-button-size-s-font-size",
        "--ui-button-size-s-line-height",
        "--ui-button-size-m-height",
        "--ui-button-size-m-min-width",
        "--ui-button-size-m-font-size",
        "--ui-button-size-m-line-height",
        "--ui-button-size-l-height",
        "--ui-button-size-l-min-width",
        "--ui-button-size-l-font-size",
        "--ui-button-size-l-line-height",
        "--ui-button-size-xl-height",
        "--ui-button-size-xl-min-width",
        "--ui-button-size-xl-font-size",
        "--ui-button-size-xl-line-height",
    ] {
        assert!(
            styles.contains(var_name),
            "Button styles should consume ui-theme variable `{var_name}` instead of hard-coded layout values."
        );
        assert!(
            theme_css.contains(var_name),
            "ui-theme css emitter should export `{var_name}` for button layout tokens."
        );
    }

    for legacy_literal in [
        "min-width: 80px;",
        "font-size: 14px;",
        "width: 16px;",
        "height: 24px;",
        "height: 28px;",
        "height: 32px;",
        "height: 36px;",
        "height: 40px;",
        "outline: 3px solid var(--ui-focus-ring);",
        "outline-offset: 2px;",
    ] {
        assert!(
            !styles.contains(legacy_literal),
            "Button styles should not keep legacy literal `{legacy_literal}` after token downshift.",
        );
    }
}

#[test]
fn button_styles_use_defensive_variable_fallback_chain_locally() {
    let styles_source = load_source("src/button/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("../../components/button/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "var(--ui-button-size-m-min-width, var(--ui-button-min-width, var(--ui-component-height-100, var(--ui-fallback-component-height-100))))",
        "var(--ui-button-size-m-font-size, var(--ui-button-font-size, var(--ui-fallback-font-size-150)))",
        "var(--ui-button-size-m-line-height, var(--ui-line-height-150, var(--ui-fallback-line-height-150)))",
        "var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-fg, var(--ui-fallback-accent-fg))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border))",
        "var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
    ] {
        assert!(
            styles_source.contains(required),
            "button styles should keep defensive double-fallback token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-accent:",
        "--ui-fallback-accent-fg:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-bg:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-border:",
        "--ui-fallback-border-width:",
        "--ui-fallback-danger:",
        "--ui-fallback-danger-fg:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-shadow-sm:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-button-spinner-size:",
        "--ui-fallback-button-spinner-border:",
        "--ui-fallback-button-spinner-duration:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-button-focus-outline-offset:",
        "--ui-fallback-button-radius-full:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for button fallback token `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-button-spinner-size);",
        "var(--ui-button-spinner-border) solid currentColor;",
        "var(--ui-button-spinner-duration) linear infinite;",
        "outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);",
        "outline-offset: var(--ui-button-focus-outline-offset);",
        "border: 1px solid transparent;",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "button styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test button_semantics button_styles_use_defensive_variable_fallback_chain_locally";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "button_styles_use_defensive_variable_fallback_chain_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn button_styles_flow_through_css_registry_and_ui_root_injection() {
    let css_registry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(needle),
            "Button style contract should be aggregated via css registry marker `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject component styles via `{needle}`."
        );
    }
}

#[test]
fn button_family_text_metrics_use_tokenized_line_height_and_no_label_translate_nudges() {
    let style_files = [
        "src/button/styles.rs",
        "src/button/field/styles.rs",
        "src/button/clear_button/styles.rs",
        "src/button/infield_button/styles.rs",
        "src/button/logic_button/styles.rs",
        "src/button/toggle_button/styles.rs",
        "src/button/search_input/styles.rs",
    ];

    for style_file in style_files {
        let source = load_source(style_file);
        assert!(
            source.contains("line-height: var(--ui-"),
            "{style_file} should use tokenized line-height instead of local literal hacks."
        );
        assert!(
            !source.contains("line-height: 1;"),
            "{style_file} should not use `line-height: 1;` which tends to create optical vertical-centering issues."
        );
        assert!(
            !source.contains("translateY("),
            "{style_file} should not nudge text alignment via translateY hacks."
        );
    }
}

#[test]
fn button_component_layer_avoids_utility_first_and_css_in_rust_defaults() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let motion_source = load_source("src/button/motion.rs");

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "class=\"w-",
        "class=\"h-",
        "class=\"gap-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button component should not use utility-first contract marker `{forbidden}`."
        );
    }

    for forbidden in ["tailwind", "tw!", "css!(", "style!(", "styled!(", "emotion"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Button component should not adopt CSS-in-Rust/utility-first default marker `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-button-scale\""),
        "Button runtime style path should only update semantic css variable `--ui-button-scale`."
    );
    assert_eq!(
        motion_source.matches("set_property(\"--ui-button-").count(),
        1,
        "Button runtime style path should remain minimal and only write one `--ui-button-*` variable.",
    );
}

#[test]
fn button_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/styles.rs");

    for selector in [
        ".ui-button[data-motion-source=\"custom\"]",
        ".ui-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Button styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_styles_use_semantic_state_markers_and_avoid_fragile_selectors() {
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");

    for required_selector in [
        ".ui-button[data-loading=\"true\"][data-loading-placement=\"center\"] .ui-button__label",
        ".ui-button[data-loading=\"true\"][data-loading-placement=\"start\"]:not([data-has-start=\"true\"])",
        ".ui-button__start[data-loading-start=\"true\"] .ui-button__start-content",
        ".ui-button[data-hovered=\"true\"]:not(:disabled).ui-button--variant-default",
    ] {
        assert!(
            styles_source.contains(required_selector),
            "Button styles should branch visual state via semantic selector `{required_selector}`.",
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", ":has("] {
        assert!(
            !styles_source.contains(forbidden),
            "Button styles should avoid fragile structural selector `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("style="),
        "Button view should not push business style logic through inline styles.",
    );
}

#[test]
fn button_family_styles_depend_on_semantic_markers_and_runtime_styles_stay_css_var_only() {
    let button_styles = load_source("src/button/styles.rs");
    let toggle_styles = load_source("src/button/toggle/styles.rs");
    let toggle_button_styles = load_source("src/button/toggle_button/styles.rs");
    let action_styles = load_source("src/button/action/styles.rs");
    let action_view = load_source("src/button/action/view.rs");
    let toggle_button_view = load_source("src/button/toggle_button/view.rs");
    let action_motion = load_source("src/button/action/motion.rs");
    let toggle_button_motion = load_source("src/button/toggle_button/motion.rs");

    for required in [
        ".ui-button[data-loading=\"true\"][data-loading-placement=\"center\"] .ui-button__label",
        ".ui-button[data-hovered=\"true\"]:not(:disabled).ui-button--variant-default",
        ".ui-toggle[data-state=\"selected\"]",
        ".ui-toggle[data-interaction=\"focus-visible\"]",
        ".ui-toggle-button[data-selected=\"true\"].ui-toggle-button--variant-default",
        ".ui-action-group[data-tone=\"default\"]",
        ".ui-action-group[data-disabled=\"true\"]",
        ".ui-action-group__item[data-selected=\"true\"]",
    ] {
        assert!(
            button_styles.contains(required)
                || toggle_styles.contains(required)
                || toggle_button_styles.contains(required)
                || action_styles.contains(required),
            "Button family style branching should use semantic selector `{required}`.",
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", ":has("] {
        assert!(
            !button_styles.contains(forbidden)
                && !toggle_styles.contains(forbidden)
                && !toggle_button_styles.contains(forbidden)
                && !action_styles.contains(forbidden),
            "Button family styles should avoid fragile structural state selector `{forbidden}`.",
        );
    }

    for required in ["style=panel_vars", "style=style_vars"] {
        assert!(
            action_view.contains(required) || toggle_button_view.contains(required),
            "Button family runtime style binding should stay on motion-css-vars marker `{required}`.",
        );
    }

    for required in [
        "--ui-action-button-group-motion-duration",
        "--ui-toggle-button-group-motion-duration",
    ] {
        assert!(
            action_motion.contains(required) || toggle_button_motion.contains(required),
            "Button family motion attach should emit css-var contract `{required}`.",
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !action_view.contains(forbidden) && !toggle_button_view.contains(forbidden),
            "Button family runtime styles should not embed business inline property `{forbidden}`.",
        );
    }
}

#[test]
fn button_cascade_layer_and_runtime_style_contract_is_enforced_locally() {
    let css_source = load_source("src/css.rs");
    let button_view = load_source("src/button/view.rs");
    let action_view = load_source("src/button/action/view.rs");
    let toggle_button_view = load_source("src/button/toggle_button/view.rs");
    let button_motion = load_source("src/button/motion.rs");
    let check2_source = load_source("../../components/button/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css registry should keep cascade-layer marker `{required}`.",
        );
    }

    assert!(
        !button_view.contains("style="),
        "button root view should not use inline style attributes for business layout.",
    );

    for required in ["style=panel_vars", "style=style_vars"] {
        assert!(
            action_view.contains(required) || toggle_button_view.contains(required),
            "button family runtime style binding should stay on css-variable marker `{required}`.",
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=move || format!(\"top:",
        "style=move || format!(\"left:",
        "style=move || format!(\"width:",
        "style=move || format!(\"height:",
    ] {
        assert!(
            !button_view.contains(forbidden)
                && !action_view.contains(forbidden)
                && !toggle_button_view.contains(forbidden),
            "button family views should not embed non-css-variable inline style `{forbidden}`.",
        );
    }

    assert!(
        button_motion.contains("set_property(\"--ui-button-scale\""),
        "button motion runtime style writes should stay on css custom property `--ui-button-scale`.",
    );
    for forbidden in [
        "set_property(\"top\"",
        "set_property(\"left\"",
        "set_property(\"width\"",
        "set_property(\"height\"",
        "set_property(\"background\"",
        "set_property(\"color\"",
    ] {
        assert!(
            !button_motion.contains(forbidden),
            "button motion should not write non-variable business style `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test button_semantics button_cascade_layer_and_runtime_style_contract_is_enforced_locally";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "button_cascade_layer_and_runtime_style_contract_is_enforced_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn button_semantic_contract_test_matrix_covers_required_branches() {
    let semantics_source = load_source("../../components/button/test/semantics.rs");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "fn button_forwards_headless_button_semantics()",
        "fn button_emits_baseline_style_data_attributes()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_loading_forces_disabled_and_sets_aria_busy()",
        "fn button_has_no_half_controlled_state_axes()",
        "fn button_machine_readable_type_and_marker_contract_has_non_ignored_feedback_loop()",
        "fn button_family_styles_depend_on_semantic_markers_and_runtime_styles_stay_css_var_only()",
        "fn button_cascade_layer_and_runtime_style_contract_is_enforced_locally()",
        "fn button_styles_use_defensive_variable_fallback_chain_locally()",
        "fn button_focus_stack_gc_stays_headless_owned_and_overlay_scope_is_not_applicable()",
        "fn button_escape_hatches_foreign_zone_is_not_applicable_without_imperative_third_party_instances()",
        "fn button_hydration_discontinuity_contract_avoids_nondeterministic_init_and_keeps_id_flow_deterministic()",
        "fn button_uses_headless_press_hover_and_focus_ring()",
        "fn ui_motion_stays_component_agnostic_and_non_wasm_safe()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic matrix should include branch coverage test `{required}`.",
        );
    }

    for required in [
        "on:pointerdown=on_pointer_down",
        "on:keydown=on_key_down",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=state.state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button view should expose semantic contract surface `{required}`.",
        );
    }
}

#[test]
fn button_semantics_checks_do_not_depend_on_visual_snapshot_assertions() {
    let semantics_source = load_source("../../components/button/test/semantics.rs");
    let snapshot_tokens = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        [".", "snap"].concat(),
    ];

    for forbidden in snapshot_tokens {
        assert!(
            !semantics_source.contains(&forbidden),
            "Button semantic contract tests should not depend on visual snapshot marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_semantic_contract_is_primary_and_visual_snapshot_is_only_supplementary() {
    let semantics_source = load_source("../../components/button/test/semantics.rs");
    let view_source = load_source("src/button/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for required in [
        "fn button_forwards_headless_button_semantics()",
        "fn button_emits_baseline_style_data_attributes()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_family_controllable_axes_are_value_on_change_default_triplets()",
        "fn button_loading_forces_disabled_and_sets_aria_busy()",
        "fn button_e2e_selector_contract_uses_semantic_markers_and_settled_waits()",
        "fn button_e2e_key_flow_covers_keyboard_and_code_sync_path()",
        "fn button_reduced_motion_and_ssr_wasm_semantics_contract_is_enforced()",
        "fn button_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic contract suite should include required branch coverage test `{required}`.",
        );
    }

    for required in [
        "on:pointerdown=on_pointer_down",
        "on:click=on_click",
        "on:keydown=on_key_down",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button semantic surface should expose stable marker `{required}`.",
        );
    }

    for required in [
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
        "page.keyboard.press(\"Space\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "Button e2e semantic flow should include marker `{required}`.",
        );
    }

    for forbidden in [
        "expect(page).toHaveScreenshot(",
        "toMatchSnapshot(",
        "assert_snapshot",
        "waitForTimeout(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Button semantic e2e flow should not rely on visual snapshot/fixed wait marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/motion.rs");

    for needle in [
        "use ui_theme::default_button_motion_tokens;",
        "let tokens = default_button_motion_tokens();",
        "pub fn sanitize_motion(motion: ButtonMotion) -> ButtonMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "ui_motion::spring::SpringAnimator::new(",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "if ui_motion::web::prefers_reduced_motion() {",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "Button motion should include `{needle}` so invalid custom motion values cannot leak into runtime animation behavior.",
        );
    }
}

#[test]
fn ui_motion_stays_component_agnostic_and_non_wasm_safe() {
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let motion_spring = load_source("../../crates/ui-motion/src/spring.rs");
    let motion_keyframes = load_source("../../crates/ui-motion/src/keyframes.rs");
    let motion_web = load_source("../../crates/ui-motion/src/web.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "pub struct SpringAnimator",
    ] {
        assert!(
            motion_lib.contains(needle)
                || motion_spring.contains(needle)
                || motion_keyframes.contains(needle)
                || motion_web.contains(needle),
            "ui-motion should keep runtime/math/no-op contracts via `{needle}`."
        );
    }

    for forbidden in ["ui-button", "ui-accordion", "aria-", "slot"] {
        assert!(
            !motion_lib.contains(forbidden)
                && !motion_spring.contains(forbidden)
                && !motion_keyframes.contains(forbidden)
                && !motion_web.contains(forbidden),
            "ui-motion must stay component-agnostic; found forbidden marker `{forbidden}`."
        );
    }
}

#[test]
fn button_docs_page_covers_button_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "description=\"Variants + sizes with spring hover/tap motion.\"",
        "<Playground",
        "title=\"Variants & sizes\"",
        "title=\"Colors\"",
        "<Button",
        "variant=variant",
        "size=size",
        "is_disabled=is_disabled",
        "is_loading=is_loading",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for button playground coverage.",
        );
    }
}

#[test]
fn button_dx_minimal_hello_world_path_is_documented_and_state_free() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/view.rs");

    for needle in [
        "Playground title=\"Hello world\" code_signal=hello_code",
        "let hello_code = Signal::derive(move || r#\"<Button>\"Button\"</Button>\"#.to_string());",
        "<Button>\"Button\"</Button>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Button docs should expose minimal DX path via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(into)] state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button should not require internal state object prop `{forbidden}` for basic usage.",
        );
    }
}

#[test]
fn button_non_composite_api_avoids_parallel_slot_conventions() {
    let button_view_source = load_source("src/button/view.rs");
    let action_button_source = load_source("src/button/action/view.rs");
    let button_spec_source = load_source("src/button/spec.rs");

    let required = "children: Children,";
    assert!(
        button_view_source.contains(required) && action_button_source.contains(required),
        "Button and ActionButton should use explicit child composition via `{required}`."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "ButtonItemSpec",
        "ActionButtonItemSpec",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !action_button_source.contains(forbidden)
                && !button_spec_source.contains(forbidden),
            "Button API should not expose parallel-array/item-spec sugar marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_collection_registration_protocol_is_not_applicable_without_dynamic_child_registry() {
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");

    for required in [
        "items: Vec<ActionGroupItem>,",
        "items.into_iter().enumerate()",
        "items: Vec<super::ToggleGroupItem>,",
        "each=move || items.get_value()",
        "key=|item| item.id.clone()",
        "use std::collections::BTreeSet;",
    ] {
        assert!(
            action_view_source.contains(required)
                || toggle_view_source.contains(required)
                || action_logic_source.contains(required)
                || toggle_logic_source.contains(required),
            "Button collection flow should keep deterministic ordered item traversal marker `{required}`.",
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register(",
        "Unregister(",
        "items_order",
        "HashSet<",
    ] {
        assert!(
            !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !action_logic_source.contains(forbidden)
                && !toggle_logic_source.contains(forbidden),
            "Button collection flow should not introduce dynamic registration protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn button_slot_projection_strategy_is_not_applicable_without_keep_alive_container() {
    let button_view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let motion_source = load_source("src/button/motion.rs");

    for required in [
        "{children()}",
        "render_button_content(",
        "items.into_iter().enumerate()",
        "each=move || items.get_value()",
    ] {
        assert!(
            button_view_source.contains(required)
                || action_view_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_button_view_source.contains(required),
            "Button slots should stay on direct eager render flow marker `{required}`.",
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "ProjectionMode",
        "keep_alive",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Button should not introduce slot projection lifecycle token `{forbidden}`.",
        );
    }
}

#[test]
fn button_env_stream_subscription_flow_is_not_applicable_in_current_scope() {
    let button_view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let theme_toggle_view_source = load_source("src/button/theme_toggle/view.rs");
    let button_logic_source = load_source("src/button/logic.rs");

    for required in [
        "let view_state = logic::resolve_view_state(",
        "let state = Memo::new(move |_|",
        "use_button(ButtonOptions",
        "use_focus_ring(FocusRingOptions",
        "use_hover(HoverOptions",
    ] {
        assert!(
            button_view_source.contains(required)
                || action_view_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_button_view_source.contains(required)
                || theme_toggle_view_source.contains(required)
                || button_logic_source.contains(required),
            "Button flow should remain interaction/state driven without env stream protocol marker `{required}`.",
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "on:resize",
        "add_event_listener(\"resize\"",
        "add_event_listener(\"scroll\"",
        "NotifyEnv",
        "env_stream",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden)
                && !theme_toggle_view_source.contains(forbidden)
                && !button_logic_source.contains(forbidden),
            "Button should not introduce raw env subscription stream token `{forbidden}`.",
        );
    }
}

#[test]
fn button_event_light_cone_is_not_applicable_without_large_collection_batch_bus() {
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");

    for required in [
        "struct ActionGroupRenderContext {",
        "let render_context = ActionGroupRenderContext {",
        "items: Vec<ActionGroupItem>,",
        "render_action_group_items(",
        "data-item-count=move || state.get().item_count.to_string()",
    ] {
        assert!(
            action_view_source.contains(required)
                || action_logic_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_logic_source.contains(required),
            "Button event flow should stay on bounded local collection semantics marker `{required}`.",
        );
    }

    for forbidden in [
        "SelectionState::All",
        "ContextBus",
        "Selector",
        "select_all",
        "batch_select",
        "prop drilling",
        "GridSelection",
    ] {
        assert!(
            !action_view_source.contains(forbidden)
                && !action_logic_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_logic_source.contains(forbidden),
            "Button should not introduce large-collection event-light-cone token `{forbidden}`.",
        );
    }
}

#[test]
fn button_causality_bus_trace_id_contract_is_not_applicable_without_derived_bus_flow() {
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");

    for required in [
        "let selected_state = use_controllable_state(",
        "render_context.request_selected_change.run(next);",
        "on_action.run(item_id_for_action.clone());",
        "on:click=on_click",
        "overlay_open::use_controllable_state(is_pressed, default_pressed, on_pressed_change);",
    ] {
        assert!(
            action_view_source.contains(required)
                || action_logic_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_logic_source.contains(required),
            "Button event causality should remain local callback flow marker `{required}`.",
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast",
        "subscriber",
        "publish(",
        "emit(",
    ] {
        assert!(
            !action_view_source.contains(forbidden)
                && !action_logic_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_logic_source.contains(forbidden),
            "Button should not introduce derived causality-bus protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn button_escape_hatches_foreign_zone_is_not_applicable_without_imperative_third_party_instances() {
    let button_mod_source = load_source("src/button/mod.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let button_view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let theme_toggle_view_source = load_source("src/button/theme_toggle/view.rs");
    let docs_actions_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for forbidden in [
        "echarts",
        "ECharts",
        "mapbox",
        "leaflet",
        "google.maps",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_zone",
        "ForeignInstance",
    ] {
        assert!(
            !button_mod_source.contains(forbidden)
                && !button_logic_source.contains(forbidden)
                && !button_view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden)
                && !theme_toggle_view_source.contains(forbidden)
                && !docs_actions_source.contains(forbidden),
            "Button scope should not integrate imperative third-party foreign-zone token `{forbidden}`.",
        );
    }

    for required in [
        "pub use view::Button;",
        "let view_state = logic::resolve_view_state(",
        "use_button(ButtonOptions",
        "on:click=on_click",
        "data-ui-agent-schema=agent_contract.schema_name",
    ] {
        assert!(
            button_mod_source.contains(required)
                || button_logic_source.contains(required)
                || button_view_source.contains(required)
                || action_view_source.contains(required),
            "Button should keep internal state+headless assembly contract marker `{required}` instead of foreign imperative escape-hatch coupling.",
        );
    }
}

#[test]
fn button_hydration_discontinuity_contract_avoids_nondeterministic_init_and_keeps_id_flow_deterministic()
 {
    let button_view_source = load_source("src/button/view.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");
    let headless_id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");

    for forbidden in [
        "Uuid::new_v4",
        "uuid::Uuid",
        "rand::random",
        "thread_rng",
        "SystemTime::now",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !button_logic_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !action_logic_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_logic_source.contains(forbidden),
            "Button hydration path should avoid nondeterministic init token `{forbidden}`.",
        );
    }

    for required in [
        "mod wasm_debug {",
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "target_arch = \"wasm32\"",
        "let timestamp_ms = js_sys::Date::now();",
    ] {
        assert!(
            button_view_source.contains(required),
            "Button runtime timestamp marker `{required}` should stay debug+wasm-gated and out of SSR init path.",
        );
    }

    for required in [
        "id: Option<String>,",
        "id=id",
        "id_base: String,",
        "id=move || id_base.get_value()",
        "let item_node_id = format!(\"{}-item-{}\", render_context.id_base.get_value(), index + 1);",
    ] {
        assert!(
            button_view_source.contains(required)
                || action_view_source.contains(required)
                || toggle_view_source.contains(required),
            "Button id flow should be input-derived and deterministic marker `{required}`.",
        );
    }

    for required in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
        "pub use id_provider::{UiIdProvider, provide_ui_id_provider, use_ui_id_provider};",
    ] {
        assert!(
            headless_id_provider_source.contains(required)
                || headless_lib_source.contains(required),
            "Deterministic ID provider contract should remain available via ui-headless marker `{required}`.",
        );
    }
}

#[test]
fn button_focus_stack_gc_stays_headless_owned_and_overlay_scope_is_not_applicable() {
    let button_view_source = load_source("src/button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let theme_toggle_view_source = load_source("src/button/theme_toggle/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");

    for forbidden in [
        "use_focus_trap(",
        "FocusTrapOptions",
        "FocusTrapFrame",
        "RestorePolicy",
        "FallbackTo(",
        "Selector(",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap(",
        "document.body()",
    ] {
        assert!(
            !button_view_source.contains(forbidden)
                && !action_view_source.contains(forbidden)
                && !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden)
                && !theme_toggle_view_source.contains(forbidden),
            "Button scope should not private-implement overlay focus-stack restore token `{forbidden}`.",
        );
    }

    for required in [
        "thread_local! {",
        "FOCUS_MANAGER_STACK",
        "FOCUS_MANAGER_NEXT_ID",
        "pub enum RestorePolicy",
        "Selector(String)",
        "FallbackTo(String)",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap(",
        "restore_focus_chain(",
        "if let Some(body) = document.body()",
    ] {
        assert!(
            focus_trap_source.contains(required),
            "Global overlay focus-stack manager should stay in ui-headless focus_trap via `{required}`.",
        );
    }

    for required in [
        "pub mod focus_trap;",
        "FocusTrapFrame, FocusTrapHandlers, FocusTrapOptions, RestorePolicy, use_focus_trap",
    ] {
        assert!(
            headless_lib_source.contains(required),
            "ui-headless public surface should export focus manager contract marker `{required}`.",
        );
    }
}

#[test]
fn button_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "Button is snapshot-only; forbidden streaming marker `{forbidden}` should not appear."
        );
    }
}

#[test]
fn button_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/button/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "button_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "button/check2.md should pin streaming contract marker `{needle}`."
        );
    }
}

#[test]
fn button_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("src/button/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "button_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "button/check2.md should pin snapshot-baseline marker `{needle}`."
        );
    }
}

#[test]
fn button_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/button/view.rs");
    let spec_source = load_source("src/button/spec.rs");
    let logic_source = load_source("src/button/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for required in [
        "#[component]",
        "pub fn Button(",
        "#[prop(optional, into)] schema_json: Option<String>,",
        "let view_state = logic::resolve_view_state(logic::ButtonLogicInput {",
        "data-slot=SLOT_BUTTON",
        "data-state=state.state_attr",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-output-status=output_status.as_attr()",
        "pub fn render(self) -> impl IntoView {",
        "schema_json=schema_json",
    ] {
        assert!(
            view_source.contains(required)
                || spec_source.contains(required)
                || logic_source.contains(required),
            "button snapshot baseline should keep complete-result render marker `{required}`.",
        );
    }

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "button snapshot baseline should stay stable without streaming-only marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("src/button/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Button 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only）",
        "fallback=snapshot",
        "button_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "button_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep streaming required/optional marker `{required}`.",
        );
    }
}

#[test]
fn button_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/button/view.rs");

    for required in [
        "role=aria.attrs.role",
        "tabindex=aria.attrs.tabindex",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "aria-haspopup=popup_a11y.aria_haspopup",
        "aria-controls=move || popup_a11y.aria_controls.get()",
        "aria-expanded=move || popup_a11y.aria_expanded.get()",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-ui-output-status=output_status.as_attr()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "button should keep continuous role/aria/data semantics via `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "button optional-streaming scope should stay snapshot-only without `{forbidden}`.",
        );
    }
}

#[test]
fn button_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let motion_source = load_source("src/button/motion.rs");
    let styles_source = load_source("src/button/styles.rs");
    let check2_source = load_source("src/button/check2.md");
    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{motion_source}\n{styles_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "button should keep validation/retry/resilience out of component layer; found `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。"),
        "button check2 should keep upstream-boundary statement for validation/retry/resilience.",
    );
}

#[test]
fn button_state_derivation_is_consumed_from_ui_state_primitives() {
    let logic_source = load_source("src/button/logic.rs");

    for needle in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "pub use ui_state_primitives::button::{",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Button logic should consume state primitives via `{needle}`."
        );
    }
}

#[test]
fn button_api_naming_uses_is_prefix_only() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_full_width: bool",
        "pub struct ButtonInputNormalizationInput",
        "pub struct ButtonInputNormalization",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "let normalized = logic::normalize_input(logic::ButtonInputNormalizationInput {",
        "is_disabled: normalized.is_disabled,",
        "is_full_width: normalized.is_full_width,",
        "pub fn is_disabled(mut self, value: bool) -> Self",
        "pub fn is_full_width(mut self, value: bool) -> Self",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || spec_source.contains(needle),
            "Button API naming contract should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] full_width: Option<bool>",
        "disabled == Some(true)",
        "full_width == Some(true)",
        "pub fn disabled(self, value: bool) -> Self",
        "pub fn full_width(self, value: bool) -> Self",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !spec_source.contains(forbidden),
            "Button API naming contract should not keep legacy alias marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_family_api_naming_contract_uses_is_and_on_prefixes() {
    let root_view_source = load_source("src/button/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let theme_toggle_view_source = load_source("src/button/theme_toggle/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let merged = [
        root_view_source.as_str(),
        toggle_view_source.as_str(),
        toggle_button_view_source.as_str(),
        theme_toggle_view_source.as_str(),
        action_view_source.as_str(),
    ]
    .join("\n");

    for required in [
        "#[prop(optional)] is_attached: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] on_pressed_change: Option<Callback<bool>>",
        "#[prop(optional)] on_selected_ids_change: Option<Callback<BTreeSet<String>>>",
    ] {
        assert!(
            merged.contains(required),
            "Button family API naming contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] attached: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] on_change: Option<Callback<bool>>",
        "#[prop(optional)] on_selected_change: Option<Callback<BTreeSet<String>>>",
    ] {
        assert!(
            !merged.contains(forbidden),
            "Button family API naming contract should not keep legacy alias marker `{forbidden}`."
        );
    }
}

#[test]
fn button_family_controllable_axes_are_value_on_change_default_triplets() {
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");

    for required in [
        "#[prop(optional)] is_pressed: Option<Signal<bool>>",
        "#[prop(optional)] default_pressed: Option<bool>",
        "#[prop(optional)] on_pressed_change: Option<Callback<bool>>",
        "overlay_open::use_controllable_state(is_pressed, default_pressed, on_pressed_change)",
        "selected_ids: Option<Signal<BTreeSet<String>>>",
        "default_selected_ids: Option<BTreeSet<String>>",
        "on_selected_ids_change: Option<Callback<BTreeSet<String>>>",
    ] {
        assert!(
            toggle_view_source.contains(required)
                || toggle_button_view_source.contains(required)
                || action_view_source.contains(required),
            "Button family controllable axes should include `{required}`."
        );
    }

    for forbidden in [
        "set_pressed: WriteSignal<bool>",
        "set_selected: WriteSignal<bool>",
    ] {
        assert!(
            !toggle_view_source.contains(forbidden)
                && !toggle_button_view_source.contains(forbidden),
            "Button family controllable axes should not keep controlled-only marker `{forbidden}`."
        );
    }
}

#[test]
fn button_default_priority_is_normalized_in_logic_only() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "let (is_disabled, disabled_input_source) = if input.is_disabled {",
        "let (is_full_width, full_width_input_source) = if input.is_full_width {",
        "let class_name = normalize_optional_text(input.class_name);",
        "let (aria_label, aria_label_source) = resolve_aria_label(",
        "input.icon_only_fallback_aria_label,",
        "let button_type = input.button_type;",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic.rs should centralize defaults/priority via `{required}`."
        );
    }

    for forbidden in [
        "disabled.unwrap_or(",
        "full_width.unwrap_or(",
        "normalize_optional_text(",
        "resolve_aria_label(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not apply fallback/default logic directly; found `{forbidden}`."
        );
    }
}

#[test]
fn button_family_default_priority_is_centralized_in_logic_only() {
    let button_group_view = load_source("src/button/view.rs");
    let toggle_view = load_source("src/button/toggle/view.rs");
    let toggle_button_view = load_source("src/button/toggle_button/view.rs");
    let theme_toggle_view = load_source("src/button/theme_toggle/view.rs");
    let action_view = load_source("src/button/action/view.rs");
    let clear_button_view = load_source("src/button/clear_button/view.rs");
    let search_input_view = load_source("src/button/search_input/view.rs");
    let picker_button_view = load_source("src/button/picker_button/view.rs");
    let button_copy_view = load_source("src/button/copy/view.rs");

    let logic_sources = [
        load_source("src/button/logic.rs"),
        load_source("src/button/toggle/logic.rs"),
        load_source("src/button/toggle_button/logic.rs"),
        load_source("src/button/theme_toggle/logic.rs"),
        load_source("src/button/action/logic.rs"),
        load_source("src/button/clear_button/logic.rs"),
        load_source("src/button/search_input/logic.rs"),
        load_source("src/button/picker_button/logic.rs"),
        load_source("src/button/copy/logic.rs"),
    ]
    .join("\n");

    for required in [
        "pub fn compose_button_group_class_name(",
        "pub fn normalize_toggle_group_default_selected_ids(",
        "pub fn compose_class_name(",
        "pub fn compose_toggle_button_group_class_name(",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn resolve_button_type(button_type: Option<ActionButtonType>) -> ActionButtonType",
        "pub fn normalize_default_selected_ids(",
        "pub fn resolve_visibility_signals(",
        "pub fn resolve_shortcut_labels(",
        "pub fn resolve_inner_class_name(value: Option<String>) -> String",
        "pub fn resolve_text_contract_with_defaults(",
        "pub fn resolve_copy_failed_status_text(",
    ] {
        assert!(
            logic_sources.contains(required),
            "Button family logic should centralize default-priority helpers via `{required}`."
        );
    }

    for view_source in [
        button_group_view.as_str(),
        toggle_view.as_str(),
        toggle_button_view.as_str(),
        theme_toggle_view.as_str(),
        action_view.as_str(),
        clear_button_view.as_str(),
        search_input_view.as_str(),
        picker_button_view.as_str(),
        button_copy_view.as_str(),
    ] {
        for forbidden in [
            "unwrap_or(",
            "unwrap_or_default(",
            "unwrap_or_else(",
            ".or(",
        ] {
            assert!(
                !view_source.contains(forbidden),
                "button view layer should not host default-priority fallback marker `{forbidden}`."
            );
        }
    }
}

#[test]
fn button_state_normalization_is_centralized_in_logic_only() {
    let action_view_source = load_source("src/button/action/view.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");

    for required in [
        "pub fn resolve_selected_ids(",
        "pub fn resolve_item_render_state(",
        "pub fn resolve_next_selected_ids(",
        "pub fn resolve_toggle_group_state(",
        "pub fn toggle_toggle_group_selected_id(",
    ] {
        assert!(
            action_logic_source.contains(required) || toggle_logic_source.contains(required),
            "state normalization helpers should live in logic.rs via `{required}`."
        );
    }

    for required in [
        "action_logic::action_group_logic::resolve_item_render_state(",
        "action_logic::action_group_logic::resolve_next_selected_ids(",
        "logic::toggle_toggle_group_selected_id(",
    ] {
        assert!(
            action_view_source.contains(required) || toggle_view_source.contains(required),
            "view.rs should consume typed logic helpers via `{required}`."
        );
    }

    for forbidden in [
        "ACTION_GROUP_ITEM_CLASS_SELECTED",
        "ACTION_GROUP_ITEM_CLASS_DISABLED",
        "action_logic::action_group_logic::toggle_selected_id(",
    ] {
        assert!(
            !action_view_source.contains(forbidden),
            "action view.rs should not rebuild selection state machine details directly; found `{forbidden}`.",
        );
    }
}

#[test]
fn button_state_primitives_are_sourced_from_ui_state_primitives() {
    let logic_sources = [
        load_source("src/button/logic.rs"),
        load_source("src/button/action/logic.rs"),
        load_source("src/button/toggle/logic.rs"),
        load_source("src/button/copy/logic.rs"),
        load_source("src/button/share/logic.rs"),
        load_source("src/button/flip/logic.rs"),
        load_source("src/button/clear_button/logic.rs"),
        load_source("src/button/infield_button/logic.rs"),
        load_source("src/button/close_button/logic.rs"),
        load_source("src/button/link_button/logic.rs"),
        load_source("src/button/picker_button/logic.rs"),
        load_source("src/button/theme_toggle/logic.rs"),
        load_source("src/button/toggle_button/logic.rs"),
        load_source("src/button/logic_button/logic.rs"),
    ]
    .join("\n");

    for required in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "use ui_state_primitives::action_group as action_group_state;",
        "use ui_state_primitives::toggle_button as toggle_group_state;",
        "use ui_state_primitives::button_copy::{",
        "pub use ui_state_primitives::share_button::{",
        "use ui_state_primitives::button_flip::{FlipButtonStateCoreInput, resolve_state_core};",
        "ui_state_primitives::button::normalize_optional_text(value)",
    ] {
        assert!(
            logic_sources.contains(required),
            "button state primitive source should come from ui-state-primitives via `{required}`.",
        );
    }

    for forbidden in [
        "use crate::store",
        "use crate::app_state",
        "use leptos_redux",
        "use yewdux",
    ] {
        assert!(
            !logic_sources.contains(forbidden),
            "button component logic should not bind app-level store directly: `{forbidden}`.",
        );
    }
}

#[test]
fn button_state_observability_uses_closed_semantic_marker_sets() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub enum ButtonBooleanInputSource",
        "ButtonBooleanInputSource::IsProp => \"is-prop\"",
        "ButtonBooleanInputSource::Default => \"default\"",
        "disabled_source_attr: if state.is_loading {",
        "\"loading\"",
        "\"prop\"",
        "\"default\"",
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "Button observability contract should include `{required}`.",
        );
    }
}

#[test]
fn button_family_observability_markers_cover_key_axes_and_closed_source_values() {
    let button_view_source = load_source("src/button/view.rs");
    let toggle_view_source = load_source("src/button/toggle/view.rs");
    let toggle_button_view_source = load_source("src/button/toggle_button/view.rs");
    let action_view_source = load_source("src/button/action/view.rs");
    let button_logic_source = load_source("src/button/logic.rs");
    let toggle_logic_source = load_source("src/button/toggle/logic.rs");
    let action_logic_source = load_source("src/button/action/logic.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for required in [
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-loading=state.is_loading.then_some(\"true\")",
        "aria-expanded=move || popup_a11y.aria_expanded.get()",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "aria-pressed=move || if state.get().is_selected { \"true\" } else { \"false\" }",
        "data-selection-source=move || state.get().selection_source_attr",
    ] {
        assert!(
            button_view_source.contains(required)
                || toggle_view_source.contains(required)
                || toggle_button_view_source.contains(required)
                || action_view_source.contains(required),
            "Button family key observability axis should include semantic marker `{required}`.",
        );
    }

    for required in [
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
        "data-variant-source=move || state.get().variant_source_attr",
        "data-size-source=move || state.get().size_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-handler-source=move || state.get().handler_source_attr",
        "data-selection-source=move || state.get().selection_source_attr",
    ] {
        assert!(
            button_view_source.contains(required)
                || toggle_view_source.contains(required)
                || action_view_source.contains(required),
            "Button family observability should expose source marker `{required}`.",
        );
    }

    for required in [
        "ButtonBooleanInputSource::IsProp => \"is-prop\"",
        "ButtonBooleanInputSource::Default => \"default\"",
        "loading_source_attr: if state.is_loading { \"prop\" } else { \"default\" }",
        "pub fn state_attr_for_selected(selected: bool) -> &'static str",
        "if selected { \"selected\" } else { \"unselected\" }",
        "if disabled { \"disabled\" } else if pressed { \"pressed\" }",
        "else if hovered { \"hovered\" } else if focus_visible { \"focus-visible\" }",
        "else if focused { \"focused\" } else { \"idle\" }",
        "let selection_source_attr = if input.is_selection_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "let data_state_attr = if input.is_disabled {",
        "\"empty\"",
        "\"selected\"",
        "\"default\"",
    ] {
        assert!(
            button_logic_source.contains(required)
                || toggle_logic_source.contains(required)
                || action_logic_source.contains(required),
            "Button family observability values should stay in closed enumerable set marker `{required}`.",
        );
    }

    for required in [
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"data-loading-source\", \"prop\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "E2E selectors should assert semantic marker `{required}` instead of DOM depth.",
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", "locator(\"button\").nth("] {
        assert!(
            !e2e_source.contains(forbidden),
            "E2E selector contract should avoid fragile DOM-position marker `{forbidden}`.",
        );
    }
}

#[test]
fn button_discrete_state_inputs_are_type_constrained() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    for required in [
        "pub enum ButtonVariant",
        "pub enum ButtonSize",
        "pub enum ButtonLoadingPlacement",
        "pub enum ButtonType",
        "impl From<&str> for ButtonType",
        "#[prop(optional, into)] button_type: ButtonType",
        "pub button_type: ButtonType,",
        "type=normalized_button_type.as_attr()",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "Button discrete-state contract should include `{required}`."
        );
    }

    for forbidden in [
        "button_type: Option<&'static str>",
        "button_type.unwrap_or(\"button\")",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Button should not keep stringly discrete state marker `{forbidden}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn button_type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let semantics_source = load_source("../../components/button/test/semantics.rs");

    for required in [
        "pub enum ButtonVariant",
        "pub enum ButtonColor",
        "pub enum ButtonRadius",
        "pub enum ButtonSize",
        "pub enum ButtonLoadingPlacement",
        "pub enum ButtonType",
        "pub enum ButtonBooleanInputSource",
        "pub struct ButtonStateSource",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "pub fn resolve_view_state(input: ButtonLogicInput) -> ButtonViewState",
        "fn normalize_input_prefers_is_prefix_aliases_and_applies_defaults()",
        "fn normalize_input_uses_is_flags_without_legacy_aliases()",
        "fn resolve_view_state_centralizes_state_and_class_derivation()",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic type/normalization contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] variant: String",
        "#[prop(optional, into)] color: String",
        "#[prop(optional, into)] radius: String",
        "#[prop(optional, into)] size: String",
        "#[prop(optional, into)] loading_placement: String",
        "#[prop(optional, into)] button_type: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button view should not expose untyped string state axis `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional, into)] variant: ButtonVariant",
        "#[prop(optional, into)] color: ButtonColor",
        "#[prop(optional, into)] radius: ButtonRadius",
        "#[prop(optional, into)] size: ButtonSize",
        "#[prop(optional)] loading_placement: ButtonLoadingPlacement",
        "#[prop(optional, into)] button_type: ButtonType",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button view should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "fn button_discrete_state_inputs_are_type_constrained()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_default_priority_is_normalized_in_logic_only()",
        "fn button_semantic_contract_test_matrix_covers_required_branches()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic suite should keep feedback locator `{required}`."
        );
    }
}

#[test]
fn button_machine_readable_type_and_marker_contract_has_non_ignored_feedback_loop() {
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let semantics_source = load_source("../../components/button/test/semantics.rs");

    for required in [
        "pub enum ButtonVariant",
        "pub enum ButtonColor",
        "pub enum ButtonRadius",
        "pub enum ButtonSize",
        "pub enum ButtonLoadingPlacement",
        "pub enum ButtonType",
        "pub enum ButtonBooleanInputSource",
        "pub fn normalize_input(input: ButtonInputNormalizationInput) -> ButtonInputNormalization",
        "pub fn resolve_view_state(input: ButtonLogicInput) -> ButtonViewState",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic typed state contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] variant: String",
        "#[prop(optional, into)] color: String",
        "#[prop(optional, into)] radius: String",
        "#[prop(optional, into)] size: String",
        "#[prop(optional, into)] loading_placement: String",
        "#[prop(optional, into)] button_type: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Button view should not expose untyped machine-state axis `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] variant: ButtonVariant",
        "#[prop(optional, into)] color: ButtonColor",
        "#[prop(optional, into)] radius: ButtonRadius",
        "#[prop(optional, into)] size: ButtonSize",
        "#[prop(optional)] loading_placement: ButtonLoadingPlacement",
        "#[prop(optional, into)] button_type: ButtonType",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Button view machine-readable marker contract should include `{required}`."
        );
    }

    for required in [
        "fn button_discrete_state_inputs_are_type_constrained()",
        "fn button_state_observability_uses_closed_semantic_marker_sets()",
        "fn button_default_priority_is_normalized_in_logic_only()",
        "fn button_semantic_contract_test_matrix_covers_required_branches()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Button semantic feedback loop should include locator `{required}`."
        );
    }
}

#[test]
fn button_status_primitives_boundary_blocks_business_store_bindings() {
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "pub use ui_state_primitives::button::ButtonLabelSource;",
        "pub use ui_state_primitives::button::{normalize_optional_text, resolve_aria_label};",
        "resolve_state_core(ButtonStateCoreInput {",
    ] {
        assert!(
            logic_source.contains(required),
            "Button logic should consume status primitives via `{required}`."
        );
    }

    for forbidden in [
        "use_context(",
        "provide_context(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "create_signal(",
        "leptos::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Button logic should stay store-agnostic and must not contain `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("ui_state_primitives::button::"),
        "Button view should not bypass logic and read status primitives directly.",
    );
}

#[test]
fn button_contract_consistency_has_no_temporary_patch_markers() {
    let sources = [
        load_source("src/button/mod.rs"),
        load_source("src/button/logic.rs"),
        load_source("src/button/view.rs"),
        load_source("src/button/styles.rs"),
        load_source("src/button/motion.rs"),
        load_source("src/button/spec.rs"),
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs"),
    ];

    let merged = sources.join("\n").to_ascii_lowercase();
    for forbidden in [
        "temporary patch",
        "temp patch",
        "quick fix",
        "hotfix",
        "compat shim",
        "remove after release",
    ] {
        assert!(
            !merged.contains(forbidden),
            "button contract should avoid temporary patch marker `{forbidden}`."
        );
    }
}

#[test]
fn button_has_no_half_controlled_state_axes() {
    let view_source = load_source("src/button/view.rs");
    let logic_source = load_source("src/button/logic.rs");

    // Button has no component-owned controllable axis (like open/selected/value).
    // It consumes external state and emits `on_press` only.
    for forbidden in [
        "default_open",
        "default_value",
        "default_selected",
        "on_open_change",
        "on_value_change",
        "on_selected_change",
        "set_is_disabled",
        "set_is_loading",
        "set_is_full_width",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Button should not introduce half-controlled state marker `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_loading: bool",
        "#[prop(optional)] is_full_width: bool",
        "#[prop(optional)] on_press: Option<OnPress>",
        "let view_state = logic::resolve_view_state(",
    ] {
        assert!(
            view_source.contains(required),
            "Button should keep external-source-only state flow via `{required}`."
        );
    }
}

#[test]
fn button_layering_matches_ui_components_assembly_contract() {
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let view_source = load_source("src/button/view.rs");
    let styles_source = load_source("src/button/styles.rs");
    let motion_source = load_source("src/button/motion.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::Button;",
        "use ui_state_primitives::button::{ButtonStateCoreInput, resolve_state_core};",
        "use ui_headless::{",
        "logic::resolve_view_state(",
        "motion::attach_motion(",
        "var(--ui-button-",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        let hit = mod_source.contains(needle)
            || logic_source.contains(needle)
            || view_source.contains(needle)
            || styles_source.contains(needle)
            || motion_source.contains(needle);
        assert!(
            hit,
            "Button layering contract evidence `{needle}` should exist in mod/logic/view/styles/motion."
        );
    }

    for forbidden in [
        "use ui_headless::",
        "ui_motion::",
        "leptos::web_sys",
        "on:pointer",
        "aria-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay as assembly/derivation only; found forbidden marker `{forbidden}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not bypass logic and consume primitives directly; found `{forbidden}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:keydown"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not carry view/headless semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn button_component_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");
    let motion_source = load_source("src/button/motion.rs");

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub mod spec;",
        "pub use view::Button;",
    ] {
        assert!(
            mod_source.contains(required),
            "button/mod.rs should keep export boundary marker `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "resolve_state_core(",
        "view! {",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "button/mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_input(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "button/logic.rs should expose derivation contract `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "on:pointer",
        "aria-",
        "data-slot",
        "leptos::web_sys",
        "HtmlElement",
        "style.set_property",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "button/logic.rs should not cross into DOM/view/style runtime logic via `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str", ".ui-button"] {
        assert!(
            styles_source.contains(required),
            "button/styles.rs should keep static CSS contract marker `{required}`."
        );
    }

    for forbidden in ["view! {", "on:pointer", "aria-"] {
        assert!(
            !styles_source.contains(forbidden),
            "button/styles.rs should not carry view/headless logic marker `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::resolve_view_state(",
        "use_button(",
        "motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "button/view.rs should include structure + headless mount marker `{required}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "button/view.rs should not bypass logic layer via `{forbidden}`."
        );
    }

    for required in [
        "pub fn attach_motion(",
        "sanitize_motion(",
        "SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(required),
            "button/motion.rs should include motion-contract attach marker `{required}`."
        );
    }

    for forbidden in ["view! {", "data-slot", "aria-", "on:pointer"] {
        assert!(
            !motion_source.contains(forbidden),
            "button/motion.rs should not include view/headless semantics marker `{forbidden}`."
        );
    }
}

#[test]
fn button_public_surface_does_not_export_web_sys_or_dom_types() {
    let mod_source = load_source("src/button/mod.rs");

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "wasm_bindgen",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Button public API should not expose internal web/DOM details via `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn button_spec_file_is_scoped_to_complex_schema_contract_and_versioned() {
    let mut spec_files = Vec::new();
    collect_component_src_spec_files(&mut spec_files);
    spec_files.sort();

    assert_eq!(
        spec_files,
        vec!["button/src/spec.rs".to_string()],
        "spec.rs should stay limited to complex components; simple components should not add spec.rs by default.",
    );

    let spec_source = load_source("src/button/spec.rs");
    for required in [
        "pub const BUTTON_SCHEMA_VERSION: u16 = 1;",
        "pub struct ButtonSchema {",
        "pub schema_version: u16,",
        "pub fn schema_version(mut self, value: u16) -> Self",
        "\\\"schema_version\\\":{},",
        "fn schema_json_is_machine_readable()",
        "fn schema_version_normalization_is_stable()",
    ] {
        assert!(
            spec_source.contains(required),
            "button/spec.rs should carry schema contract + version evolution evidence `{required}`."
        );
    }
}

#[test]
fn button_spec_file_contract_is_scarce_and_has_versioned_regression_coverage() {
    let mut spec_files = Vec::new();
    collect_component_src_spec_files(&mut spec_files);
    spec_files.sort();

    assert_eq!(
        spec_files,
        vec!["button/src/spec.rs".to_string()],
        "spec.rs should stay scarce; only components/button/src/spec.rs is allowed under components/*/src."
    );

    let spec_source = load_source("src/button/spec.rs");
    for required in [
        "pub const BUTTON_SCHEMA_VERSION: u16 = 1;",
        "pub struct ButtonSchema {",
        "pub fn schema_version(mut self, value: u16) -> Self",
        "if schema_version != BUTTON_SCHEMA_VERSION {",
        "ButtonSchemaError::unsupported_version(schema_version)",
        "supported_schema_version: BUTTON_SCHEMA_VERSION",
    ] {
        assert!(
            spec_source.contains(required),
            "button/spec.rs should keep stable schema/version contract marker `{required}`."
        );
    }

    let spec_test_source = load_source("../../components/button/test/spec.rs");
    for required in [
        "fn schema_json_is_machine_readable()",
        "fn schema_version_normalization_is_stable()",
        "fn schema_to_json_result_and_from_json_roundtrip()",
        "fn schema_from_json_rejects_missing_or_zero_version()",
        "fn schema_from_json_reports_structured_error_for_unsupported_version()",
    ] {
        assert!(
            spec_test_source.contains(required),
            "button spec regression coverage should include `{required}`."
        );
    }
}

#[test]
fn button_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "BUTTON_WORKBENCH_STORAGE_KEY",
        "fn load_button_workbench_state() -> Option<ButtonWorkbenchState>",
        "fn save_button_workbench_state(state: ButtonWorkbenchState)",
        "fn clear_button_workbench_state()",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "test_css_source=test_css_source",
        "test_config_signal=actual_config",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "Effect::new(move |_| {",
        "save_button_workbench_state(ButtonWorkbenchState {",
        "clear_button_workbench_state();",
        "data-slot=\"button-workbench\"",
        "data-slot=\"button-workbench-canvas\"",
    ] {
        assert!(
            source.contains(needle),
            "Button workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "Button workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn button_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test button_semantics button_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_docs_variants_and_controls_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "code_signal=code",
        "let size_options = vec![",
        "let color_options = vec![",
        "\"primary\".to_string()",
        "\"danger\".to_string()",
        "let radius_options = vec![",
        "\"full\".to_string()",
        "\"none\".to_string()",
        "\"xs\".to_string()",
        "\"s\".to_string()",
        "\"m\".to_string()",
        "\"l\".to_string()",
        "\"xl\".to_string()",
        "size=\"s\"",
        "size=\"m\"",
        "0 => ButtonSize::Xs",
        "1 => ButtonSize::S",
        "2 => ButtonSize::M",
        "3 => ButtonSize::L",
        "_ => ButtonSize::Xl",
        "let loading_placement_options =",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-color\".to_string()",
        "id_base=\"docs-button-radius\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "id_base=\"docs-button-loading-placement\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button color\".to_string()",
        "aria_label=\"Button radius\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "aria_label=\"Button loading placement\".to_string()",
        "<Switch checked=is_disabled set_checked=set_is_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "<Switch checked=icon_only set_checked=set_icon_only>\"Icon only\"</Switch>",
        "<Switch checked=is_full_width set_checked=set_is_full_width>\"Full width\"</Switch>",
        "<Switch checked=show_start set_checked=set_show_start>\"Start slot\"</Switch>",
        "<Switch checked=show_end set_checked=set_show_end>\"End slot\"</Switch>",
    ] {
        assert!(
            source.contains(needle),
            "button docs variants/controls playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button() -> AnyView",
        "title=\"Button\"",
        "slug=\"button\"",
        "Playground",
        "title=\"Variants & sizes\"",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should contain `{needle}` for Button.",
        );
    }
}

#[test]
fn button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Variants & sizes\"",
        "code_signal=code",
        "let color = color.get();",
        "let radius = radius.get();",
        "let size = size.get();",
        "let is_disabled = is_disabled.get();",
        "let is_loading = loading.get();",
        "let loading_placement = loading_placement.get();",
        "let icon_only = icon_only.get();",
        "let is_full_width = is_full_width.get();",
        "id_base=\"docs-button-variant\".to_string()",
        "id_base=\"docs-button-color\".to_string()",
        "id_base=\"docs-button-radius\".to_string()",
        "id_base=\"docs-button-size\".to_string()",
        "aria_label=\"Button variant\".to_string()",
        "aria_label=\"Button color\".to_string()",
        "aria_label=\"Button radius\".to_string()",
        "aria_label=\"Button size\".to_string()",
        "<Switch checked=is_disabled set_checked=set_is_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "color=color",
        "variant=variant",
        "radius=radius",
        "size=size",
        "is_disabled=is_disabled",
        "is_loading=is_loading",
        "loading_placement=loading_placement",
        "is_icon_only=icon_only",
        "is_full_width=is_full_width",
        "let colors_code = Signal::derive(move || {",
        "<Button color=\"default\">\"Default\"</Button>",
        "<Button color=\"primary\">\"Primary\"</Button>",
        "<Button color=\"secondary\">\"Secondary\"</Button>",
        "<Button color=\"success\">\"Success\"</Button>",
        "<Button color=\"warning\">\"Warning\"</Button>",
        "<Button color=\"danger\">\"Danger\"</Button>",
        "title=\"Colors\"",
    ] {
        assert!(
            source.contains(needle),
            "button docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    let section_start = docs_source
        .find("pub(super) fn button() -> AnyView {")
        .unwrap_or_else(|| panic!("actions docs should contain button section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn action_button() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("actions docs should contain action_button section after button")
        });
    let section = &section_tail[..section_end_rel];

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "id=\"docs-button-matrix-idle\".to_string()",
        "id=\"docs-button-matrix-loading\".to_string()",
        "id=\"docs-button-matrix-disabled\".to_string()",
        "id=\"docs-button-matrix-icon-only\".to_string()",
        "id=\"docs-button-controlled-like\".to_string()",
        "id=\"docs-button-uncontrolled-like\".to_string()",
        "id=\"docs-button-snapshot\".to_string()",
        "id=\"docs-button-source-first\".to_string()",
        "data-slot=\"button-output-mode\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "code_imports=button_imports.clone()",
        "data-slot=\"button-source-first-contract\"",
        "data-slot=\"button-source-paths\"",
        "component-button",
        "inject-css",
    ] {
        assert!(
            section.contains(needle),
            "button docs product contract should include `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should include `{needle}`.",
        );
    }
}

#[test]
fn button_check2_documents_docs_product_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Controlled vs Uncontrolled (N/A)",
        "button_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "button_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "compose_copy_ready_code",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 docs-as-product evidence should include `{required}`.",
        );
    }
}

#[test]
fn button_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn button_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "pub(super) fn button() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code",
        "title=\"Variants & sizes\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "id=\"docs-button-matrix-idle\".to_string()",
        "id=\"docs-button-matrix-loading\".to_string()",
        "id=\"docs-button-matrix-disabled\".to_string()",
        "id=\"docs-button-matrix-icon-only\".to_string()",
        "id=\"docs-button-controlled-like\".to_string()",
        "id=\"docs-button-uncontrolled-like\".to_string()",
        "<Switch checked=is_disabled set_checked=set_is_disabled>\"Disabled\"</Switch>",
        "<Switch checked=loading set_checked=set_loading>\"Loading\"</Switch>",
        "if color != ButtonColor::Primary {",
        "if variant != ButtonVariant::Solid {",
        "if radius != ButtonRadius::Md {",
        "if size != ButtonSize::M {",
        "if loading_placement != ButtonLoadingPlacement::Start {",
    ] {
        assert!(
            docs_source.contains(required),
            "button docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_loading: bool,",
        "#[prop(optional, into)] variant: ButtonVariant,",
        "#[prop(optional, into)] color: ButtonColor,",
        "#[prop(optional, into)] radius: ButtonRadius,",
        "#[prop(optional, into)] size: ButtonSize,",
        "#[prop(optional)] is_icon_only: bool,",
        "#[prop(optional)] is_full_width: bool,",
        "#[prop(optional)] loading_placement: ButtonLoadingPlacement,",
    ] {
        assert!(
            view_source.contains(required),
            "button public API contract should keep `{required}` for docs sync.",
        );
    }
}

#[test]
fn button_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: button docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include docs-sync/state-matrix marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "button check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/actions.rs::button",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "is_disabled/is_loading/variant/color/radius/size/loading_placement",
        "components/button/src/view.rs",
        "components/button/test/semantics.rs::button_check2_documents_docs_sync_and_state_matrix_rules",
        "components/button/test/semantics.rs::button_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/button/test/semantics.rs::button_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }
}

#[test]
fn button_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn button_documentation_entry_exists_with_beginner_first_progression() {
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/README.md");

    assert!(
        readme_path.exists()
            || docs_registry_source
                .contains("component_doc!(\"Button\", \"button\", \"Actions\", actions::button)"),
        "button should provide README or an equivalent docs-app entrypoint.",
    );

    let section_start = docs_source
        .find("pub(super) fn button() -> AnyView {")
        .unwrap_or_else(|| panic!("actions docs should contain button section"));
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn action_button() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("actions docs should contain action_button section after button")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"Button\"",
        "slug=\"button\"",
        "title=\"Hello World\"",
        "title=\"Variants & sizes\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "<Button>\"Button\"</Button>",
    ] {
        assert!(
            section.contains(required),
            "button docs should keep beginner-first documentation marker `{required}`.",
        );
    }

    let hello_pos = section
        .find("title=\"Hello World\"")
        .expect("button docs should include Hello World playground.");
    let common_pos = section
        .find("title=\"Variants & sizes\"")
        .expect("button docs should include common variants/sizes playground.");
    let matrix_pos = section
        .find("title=\"State Matrix\"")
        .expect("button docs should include state matrix playground.");
    let advanced_pos = section
        .find("title=\"Controlled vs Uncontrolled (N/A)\"")
        .expect("button docs should include controlled/uncontrolled explanation.");
    let source_first_pos = section
        .find("title=\"Source-first Starter (Copy-Paste Ready)\"")
        .expect("button docs should include source-first advanced playground.");
    assert!(
        hello_pos < common_pos
            && common_pos < matrix_pos
            && matrix_pos < advanced_pos
            && advanced_pos < source_first_pos,
        "button docs should present default path before advanced path.",
    );
}

#[test]
fn button_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: button documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include documentation-as-product marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "button check2 should mark documentation-as-product item complete."
    );

    for required in [
        "components/button/src/README.md（当前不存在，按“README 或等价入口”规则走 docs-app 入口）",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/actions.rs::button",
        "title=\"Hello World\"",
        "title=\"Variants & sizes\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "button_check2_documents_documentation_as_product_rules",
        "button_documentation_entry_exists_with_beginner_first_progression",
        "button_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 documentation-as-product section should retain marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 interactive-playground section should include `{required}`.",
        );
    }
}

#[test]
fn button_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for required in [
        "title=\"Variants & sizes\"",
        "controls=move || view! {",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=loading set_checked=set_loading",
        "Switch checked=spec_schema_enabled set_checked=set_spec_schema_enabled",
        "Switch checked=spec_requires_confirmation set_checked=set_spec_requires_confirmation",
        "\"Use AI spec payload\"",
        "\"Spec requires confirmation\"",
        "ButtonSchema::new(\"docs-button-workbench\", ButtonIntent::Primary, \"button.press\")",
        ".requires_confirmation(spec_requires_confirmation.get())",
        "data-slot=\"button-workbench\"",
        "data-slot=\"button-workbench-canvas\"",
        "data-slot=\"button-interactive-spec-preview\"",
        "data-slot=\"button-interactive-spec-input\"",
        "data-slot=\"button-interactive-spec-json\"",
        "schema_json=schema_json.clone()",
        "test_config_signal=actual_config",
        "schema_json: {schema_json:?}",
    ] {
        assert!(
            docs_source.contains(required),
            "button docs interactive playground should include `{required}`.",
        );
    }
}

#[test]
fn button_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for required in [
        "docs-app button workbench uses semantic selectors with settled loading/disabled states",
        "docs-app button workbench supports keyboard flow and code snapshot sync",
        "[data-slot=\"button-workbench\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"button-workbench-canvas\"] [data-slot=\"button\"]",
        "await loadingSwitch.click();",
        "await expect(button).toHaveAttribute(\"data-loading\", \"true\");",
        "await loadingSwitch.focus();",
        "await page.keyboard.press(\"Space\");",
        "await codeToggle.click();",
    ] {
        assert!(
            e2e_source.contains(required),
            "button interactive e2e flow should include `{required}`.",
        );
    }

    let replay_count = e2e_source
        .matches("await page.goto(\"/#/components/button\");")
        .count();
    assert!(
        replay_count >= 2,
        "button interactive acceptance should be repeatable; expected >=2 flows, got {replay_count}."
    );

    for required in [
        "data-slot=\"button-workbench\"",
        "data-slot=\"button-workbench-canvas\"",
        "data-slot=\"button-interactive-spec-input\"",
        "data-slot=\"button-interactive-spec-json\"",
    ] {
        assert!(
            docs_source.contains(required),
            "button docs should expose stable interactive anchor `{required}`.",
        );
    }
}

#[test]
fn button_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: button interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include interactive-playground marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "button check2 should mark interactive-playground item complete."
    );

    for required in [
        "title=\"Variants & sizes\"",
        "data-slot=\"button-workbench\"",
        "data-slot=\"button-workbench-canvas\"",
        "data-slot=\"button-interactive-spec-input\"",
        "data-slot=\"button-interactive-spec-json\"",
        "ButtonSchema::new(\"docs-button-workbench\", ButtonIntent::Primary, \"button.press\")",
        "button_check2_documents_interactive_playground_rules",
        "button_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "button_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "button_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 interactive-playground section should retain marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 source-first section should include `{required}`.",
        );
    }
}

#[test]
fn button_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_signal=source_first_code",
        "code_imports=button_imports.clone()",
        "id=\"docs-button-source-first\".to_string()",
        "data-slot=\"button-source-first-contract\"",
        "data-slot=\"button-source-paths\"",
        "component-button + inject-css",
        "components/button/src/view.rs and crates/ui-components/src/button/view.rs.",
    ] {
        assert!(
            docs_source.contains(required),
            "button source-first docs should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "playground copy pipeline should include `{required}`.",
        );
    }
}

#[test]
fn button_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: button source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include source-first marker `{required}`.",
        );
    }
}

#[test]
fn button_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "button check2 should mark source-first copy-paste-ready item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/actions.rs::button",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "button_check2_documents_source_first_copy_paste_ready_rules",
        "button_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "button_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 source-first section should retain marker `{required}`.",
        );
    }
}

#[test]
fn button_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-product marker `{needle}`.",
        );
    }
}

#[test]
fn button_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "button check2 heroui docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn button_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_button_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "### Button 同步记录（2026-02-16）",
        "variant/color/radius/size",
        "is_disabled/is_loading/is_icon_only/is_full_width",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/actions.rs",
        "compose_copy_ready_code",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy document should include button sync marker `{needle}`."
        );
    }

    assert!(
        docs_registry_source
            .contains("component_doc!(\"Button\", \"button\", \"Actions\", actions::button)"),
        "docs component registry should expose button entrypoint.",
    );

    for needle in [
        "title=\"Button\"",
        "slug=\"button\"",
        "title=\"Colors\"",
        "title=\"Radius\"",
        "title=\"Sizes\"",
    ] {
        assert!(
            docs_button_page_source.contains(needle),
            "docs button page should keep synced example marker `{needle}`."
        );
    }
}

#[test]
fn button_heroui_strategy_doc_sync_tracks_button_params_and_docs_entrypoint() {
    button_heroui_strategy_and_component_docs_are_synchronized_and_indexable();
}

#[test]
fn button_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: button heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include heroui docs-sync marker `{needle}`.",
        );
    }
}

#[test]
fn button_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "button check2 should mark heroui docs-sync item complete."
    );

    for needle in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "### Button 同步记录（2026-02-16）",
        "component_doc!(\"Button\", \"button\", \"Actions\", actions::button)",
        "apps/docs-app/src/pages/components/pages/actions.rs::button",
        "button_check2_documents_heroui_benchmark_docs_sync_rules",
        "button_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "button_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "button check2 heroui docs-sync section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn button_visual_desire_has_docs_theme_baseline_page_with_key_components() {
    let page_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");

    for needle in [
        "mod theme_visual_baseline;",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            page_registry_source.contains(needle),
            "docs component registry should expose visual baseline route marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should contain `{needle}`."
        );
    }
}

#[test]
fn button_visual_desire_has_e2e_visual_regression_contract() {
    let e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "E2E_VISUAL_BASELINE",
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_source.contains(needle),
            "theme visual baseline e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn button_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "section.playground",
        "[data-slot=\"button-workbench\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"button-workbench-canvas\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button e2e contract should include semantic marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "button e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn button_e2e_key_flow_covers_keyboard_and_code_sync_path() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for needle in [
        "supports keyboard flow and code snapshot sync",
        "loadingSwitch.focus();",
        "page.keyboard.press(\"Space\")",
        "toContainText(\"is_loading=true\")",
        "not.toContainText(\"is_loading=true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button e2e key-flow contract should include `{needle}`."
        );
    }
}

#[test]
fn button_check2_documents_e2e_selector_stability_rules() {
    let check2_source = load_source("../../components/button/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "button_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "button_e2e_key_flow_covers_keyboard_and_code_sync_path",
        "scripts/check-ui-components-e2e-button.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep e2e selector-stability marker `{required}`.",
        );
    }

    for required in [
        "body:not(:has(#boot))",
        "[data-slot=\"button-workbench\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"button-workbench-canvas\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "button e2e selector stability should keep semantic marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        ":nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "button e2e selector stability should avoid fragile selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn button_e2e_flow_is_in_repeatable_regression_set() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-button.sh");

    for required in [
        "test(\"docs-app button workbench uses semantic selectors with settled loading/disabled states\"",
        "test(\"docs-app button workbench supports keyboard flow and code snapshot sync\"",
        "await page.goto(\"/#/components/button\");",
        "await loadingSwitch.click();",
        "await disabledSwitch.click();",
        "await page.keyboard.press(\"Space\");",
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "button repeatable e2e flow should include semantic breakpoint marker `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "button repeatable e2e flow should avoid unstable wait token `{forbidden}`.",
        );
    }

    for required in [
        "cargo test -p ui-components --test button_semantics button_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test button_semantics button_e2e_key_flow_covers_keyboard_and_code_sync_path",
        "cargo test -p ui-components --test button_semantics button_e2e_flow_is_in_repeatable_regression_set",
    ] {
        assert!(
            script_source.contains(required),
            "button e2e check script should include repeatable-regression command `{required}`.",
        );
    }
}

#[test]
fn button_check2_documents_repeatable_e2e_regression_rules() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "button_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "button_e2e_key_flow_covers_keyboard_and_code_sync_path",
        "button_e2e_flow_is_in_repeatable_regression_set",
        "scripts/check-ui-components-e2e-button.sh",
        "overlay 路径在 Button 单组件范围内为 N/A",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep repeatable e2e regression marker `{required}`.",
        );
    }
}

#[test]
fn button_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-accordion = [",
        "component-button = [",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-button\")]\npub mod button;"),
        "lib.rs should feature-gate button module export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-button\")]")
            && css_source.contains("out.push_str(crate::button::styles::CSS);"),
        "css.rs should gate button CSS aggregation behind component-button feature."
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
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn button_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
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
fn button_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "BUTTON_MIN_FEATURES=\"component-button,inject-css\"",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_tree_shaking_check_script_covers_feature_tree_wasm_and_budget",
        "cargo test -p ui-components --test button_semantics --no-default-features --features component-button,inject-css button_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "BUTTON_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$BUTTON_MIN_FEATURES\")\"",
        "feature \"component-button\" (command-line)",
        "feature \"inject-css\" (command-line)",
        "button minimal feature tree should not pull all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$BUTTON_MIN_FEATURES\"",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking script should include `{needle}` for button minimal feature and budget contract."
        );
    }
}

#[test]
fn button_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-button",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-button,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "scripts/check-ui-components-tree-shaking.sh",
        "button_tree_shaking_keeps_component_feature_and_css_boundaries",
        "button_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep tree-shaking completion marker `{required}`."
        );
    }
}

#[test]
fn button_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free() {
    let motion_source = load_source("src/button/motion.rs");
    let mod_source = load_source("src/button/mod.rs");
    let logic_source = load_source("src/button/logic.rs");
    let spec_source = load_source("src/button/spec.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = button.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(needle),
            "button motion should keep explicit platform branch marker `{needle}`."
        );
    }

    let forbidden = "web_sys";
    assert!(
        !mod_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !spec_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden),
        "non-wasm button files should stay browser-object free; found `{forbidden}` outside motion.rs.",
    );
}

#[test]
fn button_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "crates/ui-components/src/button/view.rs",
        "crates/ui-components/src/button/motion.rs",
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn button_ui_headless_feature_mutex_compile_error_guard_is_present() {
    let headless_source = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless should keep feature mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn button_ui_motion_non_wasm_stub_contract_is_enforced() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm stub contract marker `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should enforce ui-motion portability via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "button motion should keep non-wasm safe downgrade marker `{needle}`."
        );
    }

    for forbidden in ["panic!(", ".unwrap()", ".expect("] {
        assert!(
            !button_motion_source.contains(forbidden),
            "button non-wasm motion downgrade path should avoid hard-failure marker `{forbidden}`."
        );
    }
}

#[test]
fn button_reduced_motion_and_ssr_wasm_semantics_contract_is_enforced() {
    let styles_source = load_source("src/button/styles.rs");
    let motion_web_source = load_source("../../crates/ui-motion/src/web.rs");
    let view_source = load_source("src/button/view.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-button__spinner",
        "animation: none",
    ] {
        assert!(
            styles_source.contains(needle),
            "button styles should keep reduced-motion downgrade marker `{needle}`."
        );
    }

    for needle in ["if prefers_reduced_motion() {", "return;"] {
        assert!(
            motion_web_source.contains(needle),
            "ui-motion wasm runtime should skip animation under reduced-motion via `{needle}`."
        );
    }

    for needle in [
        "data-slot=SLOT_BUTTON",
        "data-state=state.state_attr",
        "data-loading=state.is_loading.then_some(\"true\")",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "button SSR output should keep hydration-stable semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
        "window(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "button view semantics should not split by platform marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "let element: leptos::web_sys::HtmlElement = button.unchecked_into();",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "button motion enhancement should stay wasm-only while non-wasm remains safe via `{needle}`."
        );
    }

    assert!(
        platform_script_source
            .contains("cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css"),
        "platform script should keep wasm compile-only coverage for button component path."
    );
}

#[test]
fn button_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget entry `{needle}`."
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
            "UiPerfProbe should expose regression budget marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep trace-based attribution marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance should keep render_count follow-up marker `{needle}`."
        );
    }
}

#[test]
fn button_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
{
    let view_source = load_source("src/button/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_button_contract.spec.mjs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let semantics_source = load_source("../../components/button/test/semantics.rs");

    for needle in [
        "use_focus_ring(FocusRingOptions",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "data-state=state.state_attr",
        "data-focus-visible",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "button semantic contract should keep aria/data/focus marker `{needle}`."
        );
    }

    for needle in [
        "loadingSwitch.focus();",
        "page.keyboard.press(\"Space\")",
        "toHaveAttribute(\"data-loading\", \"true\")",
        "toHaveAttribute(\"aria-busy\", \"true\")",
        "toHaveAttribute(\"data-disabled-source\", \"prop\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button e2e flow should keep focus+semantic assertion `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "perf probe should keep measurable regression marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "render_count follow-up evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "fn button_semantics_checks_do_not_depend_on_visual_snapshot_assertions()",
        "fn button_focus_stack_gc_stays_headless_owned_and_overlay_scope_is_not_applicable()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "button semantics suite should keep non-snapshot/focus-flow contract test `{needle}`."
        );
    }
}

#[test]
fn button_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/button/view.rs");
    let semantics_source = load_source("../../components/button/test/semantics.rs");
    let check2_source = load_source("../../components/button/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for required in [
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "data-state=state.state_attr",
        "data-loading-source=view_state.source.loading_source_attr",
        "data-disabled-source=view_state.source.disabled_source_attr",
        "data-disabled-input-source=view_state.source.disabled_input_source_attr",
        "data-full-width-source=view_state.source.full_width_input_source_attr",
        "data-label-source=normalized_aria_label_source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "button semantic-priority contract should include `{required}`."
        );
    }

    for required in [
        "fn button_forwards_headless_button_semantics()",
        "fn button_semantics_checks_do_not_depend_on_visual_snapshot_assertions()",
        "fn button_e2e_key_flow_covers_keyboard_and_code_sync_path()",
    ] {
        assert!(
            semantics_source.contains(required),
            "button semantic suite should keep contract-first regression `{required}`."
        );
    }

    for forbidden in ["assert_snapshot!", "assert_json_snapshot!", "insta::assert"] {
        assert!(
            !semantics_source.contains(forbidden),
            "button semantic-priority contract should not rely on snapshot-only check `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test button_semantics button_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "components/button/test/semantics.rs::button_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            check2_source.contains(required),
            "button checklist should keep semantic-test-priority marker `{required}`."
        );
    }
}

#[test]
fn button_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test button_semantics button_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn button_check2_marks_semantics_and_performance_regression_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "button_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "button_semantics_checks_do_not_depend_on_visual_snapshot_assertions",
        "scripts/check-ui-components-performance.sh",
        "docs/plan/TODO.md",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep semantics/performance completion marker `{required}`."
        );
    }
}

#[test]
fn button_view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "fn render_spinner() -> impl IntoView",
        "fn render_start_slot(",
        "fn render_end_slot(",
        "fn render_button_content(",
        "{render_button_content(state, render, start_content, end_content, children)}",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep macro complexity split marker `{needle}`."
        );
    }

    let spinner_occurrences = source.matches("\"ui-button__spinner\"").count();
    assert_eq!(
        spinner_occurrences, 1,
        "button spinner class literal should have a single source of truth via constants."
    );

    for forbidden in [
        "expect(\"checked start_content\")",
        "expect(\"checked end_content\")",
    ] {
        assert!(
            !source.contains(forbidden),
            "button view should avoid fragile inline content assertions `{forbidden}` after split."
        );
    }
}

#[test]
fn button_view_macro_check_script_covers_split_contract() {
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_view_macro_complexity_is_split_into_semantic_subrenders",
        "cargo test -p ui-components --test button_semantics button_view_functional_split_prefers_plain_functions_over_local_components",
        "cargo test -p ui-components --test button_semantics button_static_fragments_are_constantized_with_stable_a11y_semantics",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro check script should enforce split contract marker `{needle}`."
        );
    }
}

#[test]
fn button_view_functional_split_prefers_plain_functions_over_local_components() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "fn render_spinner() -> impl IntoView",
        "fn render_start_slot(",
        "-> AnyView {",
        "fn render_end_slot(",
        "fn render_button_content(",
        "pub fn Button(",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep function-first split marker `{needle}`."
        );
    }

    let component_attr_count = source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "button view should keep only one `#[component]` (Button root), and keep local fragments as plain Rust functions."
    );

    for forbidden in ["#[component]\nfn render_", "#[component]\r\nfn render_"] {
        assert!(
            !source.contains(forbidden),
            "button view should not promote local render fragments to components via `{forbidden}`."
        );
    }

    for semantic_marker in [
        "data-slot=SLOT_BUTTON_SPINNER",
        "data-slot=SLOT_BUTTON_START",
        "data-slot=SLOT_BUTTON_END",
        "data-slot=SLOT_BUTTON_LABEL",
    ] {
        assert!(
            source.contains(semantic_marker),
            "button functional split should keep semantic marker `{semantic_marker}` stable."
        );
    }
}

#[test]
fn button_static_fragments_are_constantized_with_stable_a11y_semantics() {
    let source = load_source("src/button/view.rs");

    for needle in [
        "const SLOT_BUTTON: &str = \"button\";",
        "const SLOT_BUTTON_SPINNER: &str = \"button-spinner\";",
        "const SLOT_BUTTON_START: &str = \"button-start\";",
        "const SLOT_BUTTON_START_CONTENT: &str = \"button-start-content\";",
        "const SLOT_BUTTON_LABEL: &str = \"button-label\";",
        "const SLOT_BUTTON_END: &str = \"button-end\";",
        "const CLASS_BUTTON_SPINNER: &str = \"ui-button__spinner\";",
        "const CLASS_BUTTON_START: &str = \"ui-button__start\";",
        "const CLASS_BUTTON_START_CONTENT: &str = \"ui-button__start-content\";",
        "const CLASS_BUTTON_LABEL: &str = \"ui-button__label\";",
        "const CLASS_BUTTON_END: &str = \"ui-button__end\";",
        "class=CLASS_BUTTON_SPINNER data-slot=SLOT_BUTTON_SPINNER aria-hidden=\"true\"",
    ] {
        assert!(
            source.contains(needle),
            "button view should keep static fragment constantization marker `{needle}`."
        );
    }

    for literal in [
        "\"button-spinner\"",
        "\"button-start\"",
        "\"button-start-content\"",
        "\"button-end\"",
        "\"button-label\"",
        "\"ui-button__spinner\"",
        "\"ui-button__start\"",
        "\"ui-button__start-content\"",
        "\"ui-button__end\"",
        "\"ui-button__label\"",
    ] {
        let count = source.matches(literal).count();
        assert_eq!(
            count, 1,
            "static fragment literal `{literal}` should be centralized to one constant source."
        );
    }
}

#[test]
fn button_inner_html_is_disallowed_in_button_runtime_paths() {
    for rel_path in [
        "src/button/view.rs",
        "src/button/logic.rs",
        "src/button/motion.rs",
        "src/button/styles.rs",
        "src/button/mod.rs",
        "src/button/spec.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Button runtime path must not inject raw HTML; found `{forbidden}` in `{rel_path}`."
            );
        }
    }
}

#[test]
fn docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let markdown_page_source = load_source("../../apps/docs-app/src/pages/docs/markdown_page.rs");

    for needle in [
        "const ACCORDION_README_MD: &str =",
        "include_str!(\"../../../../../components/accordion/src/README.md\")",
        "const DATE_PICKER_README_MD: &str =",
        "include_str!(\"../../../../../components/text-input/src/date_picker/README.md\")",
        "fn component_readme_markdown(slug: &str) -> Option<&'static str> {",
        "\"accordion\" => Some(ACCORDION_README_MD),",
        "\"date-picker\" => Some(DATE_PICKER_README_MD),",
        "_ => None,",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "<div data-slot=\"component-readme\" inner_html=html></div>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep trusted inner_html whitelist marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=format!(",
        "inner_html=slug",
        "inner_html=description",
    ] {
        assert!(
            !shell_source.contains(forbidden),
            "docs component shell must not pipe dynamic text directly to inner_html via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn MarkdownPage(markdown: &'static str) -> impl IntoView",
        "let crate::markdown::MarkdownDoc {",
        "html: rendered_html,",
        "} = crate::markdown::render_markdown(markdown);",
        "let html = StoredValue::new(rendered_html);",
        "<div node_ref=container_ref inner_html=move || html.get_value()></div>",
    ] {
        assert!(
            markdown_page_source.contains(needle),
            "docs markdown page should keep trusted static markdown-to-html flow marker `{needle}`."
        );
    }

    for forbidden in ["inner_html=markdown", "inner_html=move || markdown"] {
        assert!(
            !markdown_page_source.contains(forbidden),
            "docs markdown page must not directly inject markdown source via `{forbidden}`."
        );
    }
}

#[test]
fn button_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_inner_html_is_disallowed_in_button_runtime_paths",
        "cargo test -p ui-components --test button_semantics docs_inner_html_is_restricted_to_trusted_whitelisted_markdown_sources",
    ] {
        assert!(
            script_source.contains(needle),
            "inner-html check script should enforce security contract marker `{needle}`."
        );
    }
}

#[test]
fn button_wasm_debug_contract_is_feature_gated_and_dev_only() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/button/view.rs");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "Button wasm debug should be opt-in and tied to component-button feature."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "button-wasm-debug must not be pulled into all-components production path."
    );

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "target_arch = \"wasm32\"",
        "Button Debug (wasm dev)",
        "data-slot=\"button-debug-entry\"",
        "data-slot=\"button-debug-event\"",
        "data-slot=\"button-debug-replay\"",
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "request_replay.run(event.source)",
        "target: \"ui_components::button::state_change\"",
        "debug_store.record(source, before, after);",
    ] {
        assert!(
            view_source.contains(needle),
            "Button wasm debug contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] debug"),
        "Button public API should not leak debug props."
    );
}

#[test]
fn button_wasm_debug_check_script_covers_feature_and_replay_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only",
    ] {
        assert!(
            script_source.contains(needle),
            "wasm-debug check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_uses_serde_schema_and_structured_migration_errors() {
    let cargo_source = load_source("Cargo.toml");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "serde = { version = \"1.0\", features = [\"derive\"], optional = true }",
        "serde_json = { version = \"1.0\", optional = true }",
    ] {
        assert!(
            cargo_source.contains(needle),
            "button engineering contract should keep serde feature gate marker `{needle}`."
        );
    }

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]",
        "pub enum ButtonSchemaErrorKind {",
        "pub struct ButtonSchemaError {",
        "pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>",
        "pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>",
        "button_schema_unsupported_version",
        "Unsupported button schema_version=",
        "schema_version: Option<u16>",
        "if schema_version != BUTTON_SCHEMA_VERSION {",
    ] {
        assert!(
            spec_source.contains(needle),
            "button spec should keep structured serde/migration marker `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_uses_consistent_tracing_targets() {
    let view_source = load_source("src/button/view.rs");
    let spec_source = load_source("src/button/spec.rs");

    for needle in [
        "target: \"ui_components::button::state_change\"",
        "const BUTTON_SPEC_TRACE_TARGET: &str = \"ui_components::button::spec\";",
        "trace_button_spec_event(",
        "\"button.schema.serialize\"",
        "\"button.schema.deserialize\"",
        "status",
        "error_code",
    ] {
        assert!(
            view_source.contains(needle) || spec_source.contains(needle),
            "button tracing contract should include `{needle}`."
        );
    }
}

#[test]
fn button_engineering_contract_avoids_runtime_leaks_in_public_api() {
    let sources = [
        load_source("src/button/mod.rs"),
        load_source("src/button/logic.rs"),
        load_source("src/button/view.rs"),
        load_source("src/button/spec.rs"),
        load_source("src/button/motion.rs"),
    ];

    for source in &sources {
        for forbidden in ["tokio", "async_std", "async-std", "tokio::", "async_std::"] {
            assert!(
                !source.contains(forbidden),
                "button engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    let mod_source = load_source("src/button/mod.rs");
    assert!(
        !mod_source.contains("web_sys"),
        "button public module boundary should not leak web_sys types."
    );
}

#[test]
fn button_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let check2_source = load_source("../../components/button/check2.md");
    let spec_source = load_source("src/button/spec.rs");
    let migration_spec_source = load_source("../../docs/spec/compile_time_evolution_migration.md");

    for required in [
        "pub const BUTTON_SCHEMA_VERSION: u16 = 1;",
        "pub struct ButtonSchema {",
        "if schema_version != BUTTON_SCHEMA_VERSION {",
        "button_schema_unsupported_version",
    ] {
        assert!(
            spec_source.contains(required),
            "button spec should keep non-breaking v1 schema marker `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2(",
        "migrate_v2_to_v3(",
        "BUTTON_SCHEMA_VERSION: u16 = 2",
        "BUTTON_SCHEMA_VERSION: u16 = 3",
    ] {
        assert!(
            !spec_source.contains(forbidden),
            "button spec should not include major-version migration marker `{forbidden}` in current scope."
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 button 改动未发生跨大版本 API 破坏升级",
        "BUTTON_SCHEMA_VERSION` 维持 `1`",
        "未来若升级到 v2，将按 `docs/spec/compile_time_evolution_migration.md` 注册 Schema Registry 窗口并提供 `migrate_v1_to_v2` 纯函数迁移层",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep explicit N/A migration governance marker `{required}`."
        );
    }

    for required in [
        "Schema Registry",
        "fn migrate_v1_to_v2(old: ProtocolV1) -> ProtocolV2",
    ] {
        assert!(
            migration_spec_source.contains(required),
            "compile-time evolution spec should keep migration governance baseline `{required}`."
        );
    }
}

#[test]
fn button_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_serde_schema_and_structured_migration_errors",
        "cargo test -p ui-components --test button_semantics button_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_uses_consistent_tracing_targets",
        "cargo test -p ui-components --test button_semantics button_engineering_contract_avoids_runtime_leaks_in_public_api",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
        "#[cfg(feature = \"component-button\")]",
        "pub mod button;",
        "#[cfg(feature = \"component-overlay\")]",
        "pub mod overlay;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
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
}

#[test]
fn ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-button\")]",
        "out.push_str(crate::button::styles::CSS);",
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
}

#[test]
fn ui_root_centralizes_theme_injection_and_i18n_context() {
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
fn active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
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
fn ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
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
fn ui_components_entrypoints_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "cargo test -p ui-components --test button_semantics ui_components_css_registry_remains_feature_gated_and_non_global",
        "cargo test -p ui-components --test button_semantics ui_root_centralizes_theme_injection_and_i18n_context",
        "cargo test -p ui-components --test button_semantics active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "cargo test -p ui-components --test button_semantics ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoints check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_component_directory_has_standard_file_layout() {
    for required in [
        "src/button/mod.rs",
        "src/button/logic.rs",
        "src/button/styles.rs",
        "src/button/view.rs",
        "src/button/motion.rs",
        "src/button/spec.rs",
    ] {
        assert!(
            path_exists(required),
            "button component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/button/render.rs"),
        "button component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
}

#[test]
fn button_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/button/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod spec;",
        "pub mod styles;",
        "pub use view::Button;",
        "pub use motion::ButtonMotion;",
        "pub use logic::ButtonVariant;",
    ] {
        assert!(
            mod_source.contains(needle),
            "button/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "button/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn button_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/button/logic.rs");
    let styles_source = load_source("src/button/styles.rs");
    let view_source = load_source("src/button/view.rs");
    let motion_source = load_source("src/button/motion.rs");
    let spec_source = load_source("src/button/spec.rs");

    for forbidden in [
        "view!",
        "on:pointer",
        "on:keydown",
        "aria-",
        "data-slot",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "button/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "button/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "button/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Button(",
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "render_button_content(",
    ] {
        assert!(
            view_source.contains(required),
            "button/view.rs should keep rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in ["resolve_state_core(", "ui_state_primitives::button::"] {
        assert!(
            !view_source.contains(forbidden),
            "button/view.rs should not bypass logic layer; found `{forbidden}`."
        );
    }

    for required in [
        "pub struct ButtonMotion",
        "pub fn attach_motion(",
        "sanitize_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "button/motion.rs should keep motion-contract marker `{required}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "button/motion.rs should not carry view semantics; found `{forbidden}`."
        );
    }

    for required in [
        "pub struct ButtonSchema",
        "pub struct ButtonSpec",
        "pub fn to_json_result(&self) -> Result<String, ButtonSchemaError>",
        "pub fn from_json(raw: &str) -> Result<Self, ButtonSchemaError>",
    ] {
        assert!(
            spec_source.contains(required),
            "button/spec.rs should keep schema-contract marker `{required}`."
        );
    }

    let mut spec_files = Vec::new();
    collect_component_src_spec_files(&mut spec_files);
    spec_files.sort();
    assert_eq!(
        spec_files,
        vec!["button/src/spec.rs".to_string()],
        "spec.rs should stay scarce; only components/button/src/spec.rs is allowed in components/*/src."
    );
}

#[test]
fn button_hyper_structure_builder_spec_contract_is_available_for_complex_component() {
    let spec_source = load_source("src/button/spec.rs");

    for required in [
        "pub struct ButtonSpec",
        "impl ButtonSpec {",
        "pub fn new() -> Self {",
        "pub fn intent(mut self, value: ButtonIntent) -> Self {",
        "pub fn size(mut self, value: ButtonSize) -> Self {",
        "pub fn motion(mut self, value: ButtonMotion) -> Self {",
        "pub fn schema(mut self, value: ButtonSchema) -> Self {",
        "pub fn render(self) -> impl IntoView {",
    ] {
        assert!(
            spec_source.contains(required),
            "button spec builder should keep Hyper-Structure marker `{required}`."
        );
    }
}

#[test]
fn button_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src_dir = workspace_dir.join("components/button/src");

    for required_file in ["Component.toml", "button.rbi"] {
        assert!(
            component_src_dir.join(required_file).exists(),
            "button context-compression file should exist: `{required_file}`.",
        );
    }

    let manifest_source = load_source("../../components/button/src/Component.toml");
    let rbi_source = load_source("../../components/button/src/button.rbi");
    let view_source = load_source("src/button/view.rs");

    for required in [
        "schema_version = \"1\"",
        "name = \"Button\"",
        "crate = \"ui-components\"",
        "name = \"is_disabled\"",
        "name = \"is_loading\"",
        "name = \"variant\"",
        "name = \"color\"",
        "name = \"radius\"",
        "name = \"size\"",
        "name = \"motion\"",
        "name = \"loading_placement\"",
        "name = \"schema_json\"",
        "name = \"on_press\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "button Component.toml should include context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub struct ButtonSpec;",
        "pub fn new() -> Self;",
        "pub fn render(self) -> impl leptos::prelude::IntoView;",
        "pub fn Button(",
        "is_disabled: bool,",
        "is_loading: bool,",
        "variant: ButtonVariant,",
        "color: ButtonColor,",
        "radius: ButtonRadius,",
        "size: ButtonSize,",
        "motion: ButtonMotion,",
        "loading_placement: ButtonLoadingPlacement,",
        "schema_json: Option<String>,",
        "button_type: ButtonType,",
        "aria_label: Option<String>,",
        "dir: Option<ui_headless::A11yDirection>,",
        "on_press: Option<ui_headless::OnPress>,",
    ] {
        assert!(
            rbi_source.contains(required),
            "button.rbi should include signature-projection marker `{required}`.",
        );
    }

    for required in [
        "pub fn Button(",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_loading: bool,",
        "#[prop(optional, into)] variant: ButtonVariant,",
        "#[prop(optional, into)] color: ButtonColor,",
        "#[prop(optional, into)] radius: ButtonRadius,",
        "#[prop(optional, into)] size: ButtonSize,",
        "#[prop(optional)] motion: ButtonMotion,",
        "#[prop(optional)] loading_placement: ButtonLoadingPlacement,",
        "#[prop(optional, into)] schema_json: Option<String>,",
        "#[prop(optional, into)] button_type: ButtonType,",
        "#[prop(optional)] on_press: Option<OnPress>,",
    ] {
        assert!(
            view_source.contains(required),
            "button view API should include `{required}` for manifest/RBI alignment.",
        );
    }
}

#[test]
fn button_check2_marks_context_compression_manifest_rbi_item_complete() {
    let check2_source = load_source("../../components/button/check2.md");

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/button/src/Component.toml",
        "components/button/src/button.rbi",
        "button_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "button check2 should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn button_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test button_semantics button_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test button_semantics button_component_file_responsibilities_remain_scoped",
        "cargo test -p ui-components --test button_semantics button_spec_file_contract_is_scarce_and_has_versioned_regression_coverage",
        "cargo test -p ui-components --test button_semantics button_hyper_structure_builder_spec_contract_is_available_for_complex_component",
        "cargo test -p ui-components --test button_semantics button_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "cargo test -p ui-components --test button_semantics button_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-components --test button_semantics button_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test button_semantics button_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "cargo test -p ui-components --test button_semantics button_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test button_semantics button_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test button_semantics button_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-button.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test button_semantics button_e2e_key_flow_covers_keyboard_and_code_sync_path",
        "cargo test -p ui-components --test button_semantics button_check2_documents_e2e_selector_stability_rules",
        "cargo test -p ui-components --test button_semantics button_e2e_flow_is_in_repeatable_regression_set",
        "cargo test -p ui-components --test button_semantics button_check2_documents_repeatable_e2e_regression_rules",
    ] {
        assert!(
            script_source.contains(needle),
            "button e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn button_contract_hygiene_check_script_covers_no_temp_patch_rule() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test button_semantics button_contract_consistency_has_no_temporary_patch_markers",
        "cargo test -p ui-components --test button_semantics button_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered",
        "cargo test -p ui-components --test button_semantics button_check2_marks_agent_contract_schema_item_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract hygiene check script should enforce `{needle}` for button."
        );
    }
}
