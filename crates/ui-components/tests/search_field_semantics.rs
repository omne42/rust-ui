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
fn search_field_clears_on_escape_when_not_empty() {
    let headless_source = load_source("../ui-headless/src/search_field.rs");

    assert!(
        headless_source.contains("use_clearable_text_field"),
        "SearchField should delegate Escape clear semantics to the shared headless clearable-text-field contract."
    );
    assert!(
        headless_source.contains("if clearable.handlers.on_key_down.run(key)"),
        "SearchField headless contract should route keydown through clearable-text-field handlers."
    );
    assert!(
        headless_source.contains("SearchFieldKeyDownResult::Cleared"),
        "SearchField should expose a typed `Cleared` result when Escape triggers value clear."
    );
}

#[test]
fn search_field_escape_clear_stops_propagation() {
    let source = load_source("src/text_input/search_field/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "SearchField should stop Escape propagation when clearing (baseline parity: Escape clears without dismissing parent overlays)."
    );
}

#[test]
fn search_field_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/text_input/search_field/view.rs");

    assert!(
        source.contains("exclude_from_tab_order=true"),
        "SearchField clear button should be excluded from tab order like UI Baseline."
    );
}

#[test]
fn search_field_clear_button_is_presence_safe() {
    let source = load_source("src/text_input/search_field/view.rs");

    assert!(
        source.contains(
            "is_visible=Signal::derive(move || search_field_contract.state.can_clear.get())"
        ),
        "SearchField should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || state.show_clear_button.get()"),
        "SearchField should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn search_field_attaches_clear_motion_driver() {
    let source = load_source("src/text_input/search_field/view.rs");

    assert!(
        source.contains("attach_clear_motion"),
        "SearchField should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn search_field_styles_define_clear_motion_css_vars() {
    let source = load_source("src/text_input/search_field/styles.rs");

    assert!(
        source.contains("--ui-search-field-clear-opacity"),
        "SearchField styles should define `--ui-search-field-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-search-field-clear-scale"),
        "SearchField styles should define `--ui-search-field-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn search_field_motion_uses_spring_animator() {
    let source = load_source("src/text_input/search_field/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SearchField motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn search_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/search_field/view.rs");

    for attr in [
        "data-state",
        "data-value",
        "data-requirement",
        "data-value-control-mode",
        "data-default-value-source",
        "data-value-change-source",
        "data-class-source",
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-read-only",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "SearchField should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn search_field_view_supports_locale_and_i18n_wiring() {
    let source = load_source("src/text_input/search_field/view.rs");

    for needle in [
        "use_ui_i18n",
        "lang=move || search_field_contract.attrs.lang.clone()",
        "dir=move || search_field_contract.attrs.dir",
        "clear_button_aria_label",
        "i18n_clear_aria_label",
        "data-clear-label-source",
    ] {
        assert!(
            source.contains(needle),
            "SearchField should include `{needle}` for locale/i18n and clear-label source wiring.",
        );
    }
}

#[test]
fn search_field_styles_respect_prefers_reduced_motion() {
    let source = load_source("src/text_input/search_field/styles.rs");

    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "SearchField styles should respect prefers-reduced-motion to avoid forced transitions."
    );
    assert!(
        source.contains("transition: none;"),
        "SearchField styles should disable transitions under prefers-reduced-motion."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn search_field_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/text_input/search_field/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SearchFieldMotion) -> SearchFieldMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hidden_scale:",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "SearchField motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn search_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn search_field() -> AnyView",
        "title=\"SearchField\"",
        "slug=\"search-field\"",
        "title=\"Hello World\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "<SearchField",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for search-field primary playground coverage.",
        );
    }
}

#[test]
fn search_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let start = source
        .find("pub(super) fn search_field() -> AnyView")
        .expect("forms docs should contain search_field function");
    let end = source[start..]
        .find("pub(super) fn number_field() -> AnyView")
        .map(|offset| start + offset)
        .unwrap_or(source.len());
    let section = &source[start..end];

    for needle in [
        "id=\"docs-search-field-hello\".to_string()",
        "id=\"docs-search-field-markers\".to_string()",
        "label=\"Search\".to_string()",
        "value=marker_value",
        "on_value_change=on_marker_value_change",
        "default_value=\"rust ui\".to_string()",
        "is_read_only=marker_read_only.get()",
        "is_disabled=marker_disabled.get()",
        "class_name=\"docs-search-field-state\".to_string()",
    ] {
        assert!(
            section.contains(needle),
            "forms docs playground should contain `{needle}` for search-field contracts.",
        );
    }

    assert!(
        !section.contains("set_value=set_value"),
        "SearchField docs should prefer `on_value_change` over legacy `set_value=` examples."
    );
}

#[test]
fn search_field_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("src/text_input/search_field/logic.rs");
    let view_source = load_source("src/text_input/search_field/view.rs");

    for needle in [
        "pub enum SearchFieldAgentSchemaVersion",
        "pub enum SearchFieldAgentIntent",
        "pub enum SearchFieldAgentActionModel",
        "pub struct SearchFieldAgentContract",
        "pub fn search_field_agent_contract() -> SearchFieldAgentContract",
        "schema_attr: \"ui.search-field\"",
        "schema_version_attr: SearchFieldAgentSchemaVersion::V1.as_attr()",
        "intent_attr: SearchFieldAgentIntent::FormSearchInput.as_attr()",
        "action_model_attr: SearchFieldAgentActionModel::InputSubmitClear.as_attr()",
        "let agent_contract = logic::search_field_agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "SearchField should keep typed Agent Contract marker `{needle}`."
        );
    }

    let combined = format!("{logic_source}\n{view_source}");
    for forbidden in ["<script", "inner_html=", "javascript:"] {
        assert!(
            !combined.contains(forbidden),
            "SearchField Agent Contract path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn search_field_streaming_policy_is_optional_with_snapshot_fallback() {
    let readme_source = load_source("src/text_input/search_field/README.md");
    let view_source = load_source("src/text_input/search_field/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in ["Snapshot", "Streaming Optional", "fallback=snapshot"] {
        assert!(
            readme_source.contains(needle),
            "SearchField README should define non-LLM streaming policy marker `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{docs_source}");
    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "streaming",
    ] {
        assert!(
            !combined.contains(forbidden),
            "SearchField is not an LLM renderer and should not expose streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn search_field_docs_page_syncs_api_matrix_state_matrix_and_source_first_contracts() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"search-field-api-matrix\"",
        "data-slot=\"search-field-state-matrix\"",
        "data-slot=\"search-field-source-first\"",
        "data-slot=\"search-field-marker-controls\"",
        "compose_copy_ready_code",
        "copyable=true",
        "component-search_field",
        "inject-css",
    ] {
        assert!(
            docs_source.contains(needle),
            "forms docs should include matrix/source-first marker `{needle}`."
        );
    }

    assert!(
        playground_source
            .contains("fn compose_copy_ready_code(raw: &str, imports: &str) -> String"),
        "docs playground should keep copy-ready import composition contract."
    );
}

#[test]
fn search_field_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_search_field_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "body:not(:has(#boot))",
        "[data-slot=\"search-field\"]",
        ".filter({ has: page.locator(\"#docs-search-field-markers\") })",
        "[data-slot=\"search-field-marker-controls\"]",
        "[data-slot=\"search-field-toggle-invalid\"] [data-slot=\"button\"]",
        "[data-slot=\"search-field-toggle-readonly\"] [data-slot=\"button\"]",
        "[data-slot=\"search-field-toggle-disabled\"] [data-slot=\"button\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.search-field\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SearchField e2e selector/wait contract should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"search-field-marker-controls\"",
        "data-slot=\"search-field-toggle-invalid\"",
        "data-slot=\"search-field-toggle-readonly\"",
        "data-slot=\"search-field-toggle-disabled\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "SearchField docs controls should expose semantic selector anchor `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "locator(\"."] {
        assert!(
            !e2e_source.contains(forbidden),
            "SearchField e2e should avoid brittle selector/wait API `{forbidden}`."
        );
    }
}

#[test]
fn search_field_e2e_ready_settled_flow_covers_keyboard_pointer_and_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_search_field_contract.spec.mjs");

    for needle in [
        "docs-app search-field covers ready-settled keyboard and pointer flow via semantic markers",
        "await input.fill(\"release\")",
        "await input.press(\"Escape\")",
        "await toggleInvalid.click()",
        "await page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-state\", \"invalid\")",
        "toHaveAttribute(\"data-state\", \"readonly\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SearchField e2e ready/settled flow should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn search_field_tree_shaking_and_feature_gate_contract_are_explicit() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-search_field = [\"component-clear_button\"]",
        "#[cfg(feature = \"component-search_field\")]\n#[path = \"text_input/search_field/mod.rs\"]\npub mod search_field;",
        "#[cfg(feature = \"component-search_field\")]\n    out.push_str(crate::search_field::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "SearchField tree-shaking contract should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn search_field_engineering_contract_is_runtime_agnostic_and_spec_free() {
    let cargo_source = load_source("Cargo.toml");
    let logic_source = load_source("src/text_input/search_field/logic.rs");
    let view_source = load_source("src/text_input/search_field/view.rs");
    let motion_source = load_source("src/text_input/search_field/motion.rs");

    assert!(
        !cargo_source.contains("search_field-wasm-debug"),
        "SearchField should not add component-specific wasm debug feature; reuse global capability."
    );

    let combined = format!("{logic_source}\n{view_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json",
        "tokio::",
        "async_std::",
        "tracing::span",
    ] {
        assert!(
            !combined.contains(forbidden),
            "SearchField component implementation should avoid local engineering coupling `{forbidden}`."
        );
    }
}

#[test]
fn search_field_heroui_strategy_and_docs_entry_stay_in_sync() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### SearchField 同步记录（2026-02-18）",
        "forms::search_field",
        "component_doc!(\"SearchField\", \"search-field\", \"Forms\", forms::search_field)",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_catalog_source.contains(needle),
            "SearchField HeroUI/doc sync should include `{needle}`."
        );
    }
}

#[test]
fn search_field_checklist_has_no_unchecked_items() {
    let checklist = load_source("src/text_input/search_field/check2.md");
    assert!(
        !checklist.contains("- [ ]"),
        "SearchField checklist should be fully checked after scoped verification."
    );
}
