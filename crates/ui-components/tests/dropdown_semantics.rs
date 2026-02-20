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
fn dropdown_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/dropdown/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Dropdown internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn dropdown_uses_logic_state_model() {
    let view_source = load_source("src/menu/dropdown/view.rs");
    let logic_source = load_source("src/menu/dropdown/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/dropdown.rs");

    for needle in [
        "pub use ui_state_primitives::dropdown::{",
        "DropdownStateInput",
        "DropdownState",
        "normalize_optional_text",
        "normalize_id_base",
        "normalize_aria_label",
        "normalize_disabled_indices",
        "resolve_trigger_disabled",
        "resolve_state",
        "pub struct DisabledStateInput",
        "pub fn normalize_disabled_state(",
        "pub struct OpenStateInput",
        "pub fn normalize_open_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Dropdown logic should include `{needle}` while consuming ui-state-primitives."
        );
    }

    for needle in [
        "pub enum DropdownOpenFocusStrategy",
        "pub fn focus_strategy_for_open_key(",
        "pub struct DropdownStateInput",
        "pub struct DropdownState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_disabled(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Dropdown primitives should define `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_disabled_state(logic::DisabledStateInput {",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count)",
        "overlay_open::use_controllable_open_state_traced(",
        "logic::resolve_state(logic::DropdownStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Dropdown view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn dropdown_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menu/dropdown/view.rs");
    let logic_source = load_source("src/menu/dropdown/logic.rs");

    for needle in [
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "pub struct OpenStateInput",
        "pub fn normalize_open_state(",
        "motion: DropdownMotion",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "Dropdown should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn dropdown_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/menu/dropdown/view.rs");

    for attr in [
        "data-slot=\"dropdown\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "data-motion-source=if motion == DropdownMotion::default()",
        "data-custom-motion=(motion != DropdownMotion::default()).then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Dropdown should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn dropdown_composes_button_popover_and_menu() {
    let source = load_source("src/menu/dropdown/view.rs");

    for needle in [
        "<Button",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "<Popover",
        "motion=motion.popover",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown should compose overlay/menu flow through `{needle}`."
        );
    }
}

#[test]
fn dropdown_styles_include_persistent_and_disabled_markers() {
    let source = load_source("src/menu/dropdown/styles.rs");

    for selector in [
        ".ui-dropdown--disabled",
        ".ui-dropdown[data-disabled=\"true\"]",
        ".ui-dropdown--persistent",
        ".ui-dropdown[data-keep-open-on-action=\"true\"]",
        ".ui-dropdown--custom-class",
        ".ui-dropdown[data-custom-class=\"true\"]",
        ".ui-dropdown[data-motion-source=\"custom\"]",
        ".ui-dropdown[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dropdown styles should include `{selector}` as stable visual-state contracts."
        );
    }
}

#[test]
fn dropdown_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/menu/dropdown/mod.rs");
    let motion_source = load_source("src/menu/dropdown/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::DropdownMotion;",
        "pub struct DropdownMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Dropdown motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dropdown_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/menu/dropdown/motion.rs");
    let view_source = load_source("src/menu/dropdown/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DropdownMotion) -> DropdownMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Dropdown motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::dropdown::motion::sanitize_motion(motion);"),
        "Dropdown view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn dropdown_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn dropdown() -> AnyView",
        "title=\"Dropdown\"",
        "slug=\"dropdown\"",
        "description=\"baseline-style dropdown trigger primitive with centralized state/source contracts, controllable open state, and spring-tuned popover motion.\"",
        "<Playground title=\"Default\" code_signal=code>",
        "<Playground title=\"Controlled + Persistent + Motion\" code_signal=states_code>",
        "<Dropdown",
        "open=open_signal",
        "close_on_action=false",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for dropdown coverage.",
        );
    }
}

#[test]
fn dropdown_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "id_base=\"docs-dropdown-default\".to_string()",
        "items=items",
        "on_action=on_action",
        "\"Open actions\"",
        "\"last action: \"",
        "let (open_raw, set_open_raw) = signal(false);",
        "let open_signal: Signal<bool> = Signal::derive(move || open_raw.get());",
        "let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));",
        "id_base=\"docs-dropdown-controlled\".to_string()",
        "items=controlled_items",
        "disabled_indices=vec![1]",
        "item_kinds=vec![",
        "MenuItemKind::Action",
        "class_name=\"docs-dropdown-custom\".to_string()",
        "initial_scale: 0.94",
        "offset_y_px: 12.0",
        "\"Controlled dropdown\"",
        "\"open: \"",
    ] {
        assert!(
            source.contains(needle),
            "dropdown docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn dropdown_supports_is_prefixed_boolean_props_with_legacy_aliases() {
    let view_source = load_source("src/menu/dropdown/view.rs");
    let logic_source = load_source("src/menu/dropdown/logic.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "pub struct DisabledStateInput",
        "pub fn normalize_disabled_state(",
        "pub struct OpenStateInput",
        "pub fn normalize_open_state(",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Dropdown API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "let is_disabled = logic::normalize_disabled_state(logic::DisabledStateInput {",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "disabled: logic::resolve_trigger_disabled(is_disabled, item_count)",
    ] {
        assert!(
            view_source.contains(needle),
            "Dropdown view should consume normalized accessibility/open state via `{needle}`.",
        );
    }
}

#[test]
fn dropdown_wires_open_triplet_into_headless_state() {
    let source = load_source("src/menu/dropdown/view.rs");

    for needle in [
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"dropdown\",",
        "normalized_open_state.open,",
        "normalized_open_state.default_open,",
        "normalized_open_state.on_open_change,",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown open axis should wire `{needle}` for stable controlled/uncontrolled semantics."
        );
    }
}

#[test]
fn dropdown_view_does_not_inline_default_fallback_rules() {
    let source = load_source("src/menu/dropdown/view.rs");

    for forbidden in ["is_disabled.unwrap_or(disabled)", "is_open.or(open)"] {
        assert!(
            !source.contains(forbidden),
            "Dropdown view.rs should avoid owning fallback/priority rule `{forbidden}`."
        );
    }
}

#[test]
fn dropdown_styles_depend_on_explicit_state_markers_only() {
    let source = load_source("src/menu/dropdown/styles.rs");

    for required in [
        ".ui-dropdown--disabled",
        ".ui-dropdown[data-disabled=\"true\"]",
        ".ui-dropdown--persistent",
        ".ui-dropdown[data-keep-open-on-action=\"true\"]",
        ".ui-dropdown[data-motion-source=\"custom\"]",
    ] {
        assert!(
            source.contains(required),
            "Dropdown styles should include explicit marker selector `{required}`."
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", " > * > * > "] {
        assert!(
            !source.contains(forbidden),
            "Dropdown styles should avoid brittle structural selector `{forbidden}`."
        );
    }
}

#[test]
fn dropdown_component_files_are_layered_and_spec_file_is_absent() {
    let module_source = load_source("src/menu/dropdown/mod.rs");
    let logic_source = load_source("src/menu/dropdown/logic.rs");
    let styles_source = load_source("src/menu/dropdown/styles.rs");
    let view_source = load_source("src/menu/dropdown/view.rs");
    let motion_source = load_source("src/menu/dropdown/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::DropdownMotion;",
        "pub use view::Dropdown;",
    ] {
        assert!(
            module_source.contains(needle),
            "Dropdown mod.rs should keep layered export marker `{needle}`.",
        );
    }
    assert!(
        logic_source.contains("pub use ui_state_primitives::dropdown::{"),
        "Dropdown logic should consume state primitives from ui-state-primitives."
    );
    assert!(styles_source.contains("pub const CSS: &str"));
    assert!(view_source.contains("#[component]\npub fn Dropdown("));
    assert!(motion_source.contains("pub struct DropdownMotion"));
    assert!(
        !Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/menu/dropdown/spec.rs")
            .exists(),
        "Dropdown should not add spec.rs for this component scope."
    );
}

#[test]
fn dropdown_docs_page_includes_hello_world_entrypoint() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "data-slot=\"dropdown-hello-world\"",
        "id_base=\"docs-dropdown-hello\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown docs should include zero-threshold marker `{needle}`."
        );
    }
}

#[test]
fn dropdown_docs_playground_exposes_semantic_selector_anchors() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    for needle in [
        "data-slot=\"dropdown-default-playground\"",
        "data-slot=\"dropdown-last-action\"",
        "data-slot=\"dropdown-controlled-playground\"",
        "data-slot=\"dropdown-controlled-open\"",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown docs should expose semantic selector anchor `{needle}`."
        );
    }
}

#[test]
fn dropdown_docs_page_includes_interactive_playground_sections() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "<SegmentedControl",
        "<Switch checked=workbench_controlled",
        "data-slot=\"dropdown-workbench-preview\"",
        "<Playground title=\"State Matrix Compare\" code_signal=matrix_code>",
        "data-slot=\"dropdown-state-matrix\"",
        "id_base=\"docs-dropdown-compare-default\".to_string()",
        "id_base=\"docs-dropdown-compare-controlled\".to_string()",
        "id_base=\"docs-dropdown-compare-disabled\".to_string()",
        "id_base=\"docs-dropdown-compare-empty\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Dropdown docs interactive playground should include `{needle}`."
        );
    }
}

#[test]
fn dropdown_docs_entry_has_readme_streaming_policy_and_source_paths() {
    let readme = load_source("src/menu/dropdown/README.md");

    for needle in [
        "# Dropdown",
        "## Streaming 策略",
        "Snapshot",
        "Streaming Optional",
        "fallback=snapshot",
        "## Hello World",
        "## Source-first",
        "crates/ui-components/src/menu/dropdown/{mod,logic,view,styles,motion}.rs",
        "crates/ui-state-primitives/src/dropdown.rs",
    ] {
        assert!(
            readme.contains(needle),
            "Dropdown README should include `{needle}`."
        );
    }
}

#[test]
fn dropdown_readme_documents_display_config_code_and_css_test_sections() {
    let readme = load_source("src/menu/dropdown/README.md");

    for needle in [
        "## docs-app 展示区（展示 / config / code / css test）",
        "展示区（Display）",
        "Config 区",
        "Code 区",
        "CSS Test 区",
        "Interactive Playground",
        "## 多场景对比展示",
        "State Matrix Compare",
        "`Default`",
        "`Controlled + Persistent`",
        "`Disabled`",
        "`Empty`",
    ] {
        assert!(
            readme.contains(needle),
            "Dropdown README should document `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dropdown_tree_shaking_feature_gates_are_explicit() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-dropdown = [\"component-button\", \"component-menu\", \"component-popover\"]",
        "#[cfg(feature = \"component-dropdown\")]\n#[path = \"menu/dropdown/mod.rs\"]\npub mod dropdown;",
        "#[cfg(feature = \"component-dropdown\")]\n    out.push_str(crate::dropdown::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Dropdown tree-shaking contract should include `{needle}`.",
        );
    }
}

#[test]
fn dropdown_platform_contract_preserves_headless_mutex_and_motion_stub_references() {
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let motion_lib = load_source("../ui-motion/src/lib.rs");
    let view_source = load_source("src/menu/dropdown/view.rs");

    for needle in [
        "feature = \"web\"",
        "feature = \"ssr\"",
        "compile_error!(",
        "features `web` and `ssr` are mutually exclusive; enable exactly one",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex marker `{needle}`."
        );
    }
    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion should expose non-wasm fallback marker `{needle}`."
        );
    }
    for forbidden in ["web_sys::", "window()", "document()"] {
        assert!(
            !view_source.contains(forbidden),
            "Dropdown view should not directly bind browser-only api `{forbidden}`.",
        );
    }
}

#[test]
fn dropdown_inner_html_path_is_absent() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        load_source("src/menu/dropdown/logic.rs"),
        load_source("src/menu/dropdown/view.rs"),
        load_source("src/menu/dropdown/styles.rs"),
        load_source("src/menu/dropdown/motion.rs")
    );

    for forbidden in ["inner_html", "<script", "javascript:"] {
        assert!(
            !combined.contains(forbidden),
            "Dropdown should avoid unsafe html injection marker `{forbidden}`."
        );
    }
}

#[test]
fn dropdown_semantics_suite_is_contract_first_not_snapshot_only() {
    let source = load_source("tests/dropdown_semantics.rs");
    let has_rust_snapshot_macro = source
        .lines()
        .any(|line| line.trim_start().starts_with("assert_snapshot!("));
    let has_js_snapshot_matcher = source
        .lines()
        .any(|line| line.trim_start().starts_with("toMatchSnapshot("));
    assert!(
        !has_rust_snapshot_macro && !has_js_snapshot_matcher,
        "Dropdown semantics suite should stay contract-first and avoid snapshot-only assertions."
    );
}

#[test]
fn dropdown_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dropdown_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "body:not(:has(#boot))",
        "[data-slot=\"dropdown-default-playground\"]",
        "[data-slot=\"dropdown-last-action\"]",
        "[data-slot=\"dropdown-controlled-open\"]",
        "#docs-dropdown-controlled-trigger",
        "toHaveAttribute(\"data-controlled\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle) || docs_source.contains(needle),
            "Dropdown e2e/docs contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Dropdown e2e should avoid brittle wait primitive `{forbidden}`."
        );
    }
}

#[test]
fn dropdown_e2e_key_flow_is_repeatable_with_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dropdown_contract.spec.mjs");
    for needle in [
        "docs-app dropdown key flow is repeatable with semantic contract breakpoints",
        "await trigger.focus()",
        "await page.keyboard.press(\"ArrowDown\")",
        "await option.click()",
        "await page.reload()",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Dropdown e2e repeatable flow should include `{needle}`."
        );
    }
}

#[test]
fn dropdown_e2e_interactive_playground_covers_display_config_code_and_css_test() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dropdown_contract.spec.mjs");

    for needle in [
        "docs-app dropdown interactive playground exposes display config code and css-test panels",
        "Interactive Playground",
        "[data-slot=\"dropdown-workbench-preview\"]",
        "Show settings",
        "[data-slot=\"playground-controls\"]",
        "#docs-dropdown-workbench-placement-trigger",
        "Show code",
        "[data-slot=\"playground-code\"]",
        "Show test",
        "[data-slot=\"playground-test\"]",
        ".playground__test-editor",
        "Actual config",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Dropdown e2e interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn dropdown_e2e_state_matrix_compare_covers_key_variants() {
    let e2e_source = load_source("../../e2e/tests/docs_app_dropdown_contract.spec.mjs");

    for needle in [
        "docs-app dropdown state matrix compare keeps key variants visible",
        "[data-slot=\"dropdown-state-matrix\"]",
        "#docs-dropdown-compare-default-trigger",
        "#docs-dropdown-compare-controlled-trigger",
        "#docs-dropdown-compare-disabled-trigger",
        "#docs-dropdown-compare-empty-trigger",
        "toHaveAttribute(\"data-controlled\", \"true\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-empty\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Dropdown e2e state matrix compare should include `{needle}`.",
        );
    }
}

#[test]
fn dropdown_heroui_strategy_and_docs_entry_stay_in_sync() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### Dropdown 同步记录（2026-02-18）",
        "component_doc!(\"Dropdown\", \"dropdown\", \"Collections\", collections_extra::dropdown)",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_catalog_source.contains(needle),
            "Dropdown HeroUI/doc sync should include `{needle}`."
        );
    }
}

#[test]
fn dropdown_check2_has_no_unchecked_checklist_items() {
    let checklist = load_source("src/menu/dropdown/check2.md");
    assert!(
        !checklist.contains("- [ ]"),
        "Dropdown checklist should be fully checked after scoped verification."
    );
}
