use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn modal_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/modal/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Modal internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn modal_is_exported_from_module_and_exposes_state_contracts() {
    let module_source = load_source("../../components/modal/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use view::Modal;",
        "pub enum ModalSlot",
        "pub struct ModalPartStateInput",
        "pub struct ModalPartState",
    ] {
        assert!(
            module_source.contains(needle),
            "modal module should include `{needle}` contracts."
        );
    }

    assert!(
        crate_source.contains("pub use modal::Modal;"),
        "crate root should re-export `Modal` contract."
    );
}

#[test]
fn modal_logic_exposes_state_helpers() {
    let source = load_source("../../components/modal/src/logic.rs");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-modal\";",
        "pub const DEFAULT_TITLE: &str = \"Modal\";",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: ModalPartStateInput)",
        "pub fn compose_class_name(",
        ") -> Cow<'static, str>",
    ] {
        assert!(
            source.contains(needle),
            "Modal logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn modal_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("../../components/modal/src/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(ModalPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-description=root_state.description_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=title_state.slot_attr",
        "data-title-source=title_state.title_source_attr",
        "data-slot=body_state.slot_attr",
        "motion=motion",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Modal view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn modal_only_sets_describedby_when_description_exists() {
    let source = load_source("../../components/modal/src/view.rs");

    assert!(
        source.contains(
            "let description: Option<Signal<String>> = content_state.description.map(Into::into);"
        ),
        "Modal should normalize description once and keep a single render path."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "description.as_ref().map(|_| description_id.clone())",
        "aria_describedby=dialog_aria_describedby",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Modal should wire description path contracts (`{needle}`)."
        );
    }

    assert_eq!(
        source.matches("<Overlay").count(),
        1,
        "Modal should avoid duplicated Overlay render branches for description/no-description cases."
    );
}

#[test]
fn modal_styles_include_state_and_source_markers() {
    let source = load_source("../../components/modal/src/styles.rs");

    for selector in [
        ".ui-modal[data-motion-source=\"custom\"]",
        ".ui-modal[data-custom-motion=\"true\"]",
        ".ui-modal--custom-motion",
        ".ui-modal--custom-id",
        ".ui-modal[data-id-source=\"custom\"]",
        ".ui-modal[data-custom-id=\"true\"]",
        ".ui-modal--custom-title",
        ".ui-modal[data-title-source=\"custom\"]",
        ".ui-modal[data-custom-title=\"true\"]",
        ".ui-modal--custom-description",
        ".ui-modal[data-description-source=\"custom\"]",
        ".ui-modal[data-custom-description=\"true\"]",
        ".ui-modal[data-class-source=\"custom\"]",
        ".ui-modal[data-exit-source=\"custom\"]",
        ".ui-modal[data-custom-exit=\"true\"]",
        ".ui-modal--with-description",
        ".ui-modal[data-state=\"with-description\"]",
        ".ui-modal--title-only",
        ".ui-modal[data-description=\"present\"]",
        ".ui-modal__title[data-slot=\"modal-title\"]",
        ".ui-modal__description[data-slot=\"modal-description\"]",
        ".ui-modal__body[data-slot=\"modal-body\"]",
    ] {
        assert!(
            source.contains(selector),
            "Modal styles should include `{selector}` as stable state/source contracts."
        );
    }
}

#[test]
fn modal_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::modal::styles::CSS);"),
        "ui-components css aggregator should include modal styles."
    );
}

#[test]
fn modal_tree_shaking_contract_is_feature_gated() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "web-demo-components = [",
        "component-modal = [\"component-overlay\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components feature table should keep tree-shaking contract marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-modal\")]",
        "#[path = \"../../../components/modal/src/mod.rs\"]",
        "pub mod modal;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib export should gate modal module by feature via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui-components css aggregation should stay feature-gated/no-op compatible via `{needle}`."
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "web-demo should consume ui-components in minimal feature mode."
    );
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not pull all-components implicitly."
    );
}

#[test]
fn modal_tree_shaking_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MODAL_MIN_FEATURES=\"component-modal,inject-css\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_tree_shaking_contract_is_feature_gated",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_tree_shaking_script_covers_feature_tree_wasm_and_budget",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "MODAL_TREE_OUTPUT",
        "if grep -q 'all-components' <<<\"$MODAL_TREE_OUTPUT\";",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "modal check2 should mark tree-shaking first-class ability item complete.",
    );
    assert!(
        source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "modal check2 should mark tree-shaking feature-pruning checklist item complete.",
    );

    for needle in [
        "modal_tree_shaking_contract_is_feature_gated",
        "modal_tree_shaking_script_covers_feature_tree_wasm_and_budget",
        "modal_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-modal,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 tree-shaking section should reference `{needle}`."
        );
    }
}

#[test]
fn modal_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "State + Source Markers",
        "data-id-source",
        "<Modal",
    ] {
        assert!(
            source.contains(needle),
            "modal docs page should contain `{needle}`."
        );
    }
}

#[test]
fn modal_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = OverlayMotion {",
        "initial_scale: 0.92",
        "initial_y_px: 18.0",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "motion=custom_motion",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-id-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "modal docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn modal_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "description=\"Overlay composition with centralized title/description/class state attrs and stable modal slots.\"",
        "<Playground title=\"Label + Description\" code_signal=semantic_code>",
        "title=\"State + Source Markers\"",
        "code_signal=custom_code",
        "<Modal",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for modal primary playground coverage.",
        );
    }
}

#[test]
fn modal_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Label + Description\"",
        "id_base=\"docs-modal-semantic\".to_string()",
        "title=\"Confirm\".to_string()",
        "description=\"Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.\".to_string()",
        "on_exit_complete=on_semantic_exit_complete",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "title=\"Title only\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "let custom_motion = OverlayMotion {",
        "initial_scale: 0.92",
        "initial_y_px: 18.0",
        "motion=custom_motion",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-id-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "modal docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn modal_docs_include_interactive_playground_contract_panels() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "test_source_path=\"components/modal/src/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn modal_readme_and_docs_shell_register_display_config_code_css_contract() {
    let readme_source = load_source("../../components/modal/src/README.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    assert!(
        readme_source.contains("## Playground 展示区（Display / Config / Code / CSS Test）"),
        "modal README should document display/config/code/css test playground layout.",
    );
    assert!(
        shell_source.contains("\"modal\" => Some(MODAL_README_MD)"),
        "docs shell should map modal slug to MODAL_README_MD.",
    );
}

#[test]
fn modal_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/modal/check2.md");
    let view_source = load_source("../../components/modal/src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget baseline token `{needle}`."
        );
    }

    let needle = "component_doc!(\"Modal\", \"modal\", \"Overlays\", overlays::modal),";
    assert!(
        pages_source.contains(needle),
        "docs pages registry should keep modal route token `{needle}`."
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose perf regression marker `{needle}`."
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
            "coverage e2e should keep perf assertion `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Modal view should expose perf triage attribution marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "UiPerfBudget::mount_only(120.0)",
        "data-perf-violation != true",
        "trace.emit",
        "render_count",
        "N/A（精确 `render_count` 自动计数）",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 performance evidence should include `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("../../components/modal/src/view.rs");
    let overlay_view_source = load_source("../../components/overlay/src/view.rs");
    let local_semantics_source = load_source("../../components/modal/test/semantics.rs");
    let semantics_source = load_source("tests/modal_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "data-state=root_state.state_attr",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "aria_labelledby=dialog_aria_labelledby",
        "aria_describedby=dialog_aria_describedby",
    ] {
        assert!(
            view_source.contains(marker),
            "Modal semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev: ev::PointerEvent| ev.stop_propagation()",
    ] {
        assert!(
            overlay_view_source.contains(marker),
            "Overlay semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "fn modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only()",
        "modal_exposes_observable_and_enumerable_state_markers",
        "for forbidden in [",
        "\"assert_snapshot\"",
        "\"to_match_snapshot\"",
        "\"snapshot!\"",
        "Modal tests should not rely on visual snapshot assertion token",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "Modal local semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    for marker in [
        "fn modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "fn modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only()",
    ] {
        assert!(
            semantics_source.contains(marker),
            "Modal workspace semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include modal semantic-priority gate `{script_needle}`.",
    );
}

#[test]
fn modal_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "echo \"[perf] contract: modal semantic test priority\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include modal semantic-priority marker `{marker}`."
        );
    }
}

#[test]
fn modal_view_macro_complexity_is_bounded_with_semantic_subblocks() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "fn render_modal_title(",
        "fn render_modal_description(",
        "fn render_modal_body(",
        "fn render_modal_sections(",
        "{render_modal_title(",
        "{render_modal_body(",
        "let description_view = description.map(",
    ] {
        assert!(
            view_source.contains(needle),
            "Modal view should keep semantic sub-block split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("<Overlay").count(),
        1,
        "Modal should keep a single Overlay render path and avoid duplicated description/no-description branches."
    );

    assert!(
        view_source.matches("view! {").count() <= 6,
        "Modal should keep view! macro expansion bounded after sub-block extraction."
    );

    for forbidden in [
        "if let Some(description) = description",
        "else {\n        view! {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Modal view should not retain duplicated branch macro pattern `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控：",
        "render_modal_title",
        "render_modal_sections",
        "modal_view_macro_complexity_is_bounded_with_semantic_subblocks",
    ] {
        assert!(
            check2_source.contains(needle),
            "Modal checklist should keep view-macro complexity evidence `{needle}`."
        );
    }
}

#[test]
fn modal_prefers_functional_subviews_over_local_component_sprawl() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "fn render_modal_title(",
        "fn render_modal_description(",
        "fn render_modal_body(",
        "fn render_modal_sections(",
        ") -> AnyView {",
        "pub fn Modal(",
        "data-slot=title_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
        "data-slot=body_state.slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Modal view should keep functional subview split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Modal view should keep a single component entrypoint and avoid local component sprawl."
    );

    for forbidden in [
        "#[component]\nfn render_modal_title(",
        "#[component]\nfn render_modal_description(",
        "#[component]\nfn render_modal_body(",
        "#[component]\nfn render_modal_sections(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Modal view should keep helper fragment as plain Rust fn, not nested component `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：",
        "render_modal_title",
        "render_modal_sections",
        "modal_prefers_functional_subviews_over_local_component_sprawl",
    ] {
        assert!(
            check2_source.contains(needle),
            "Modal checklist should keep functional split evidence `{needle}`."
        );
    }
}

#[test]
fn modal_static_fragments_are_constantized_or_absent_for_simple_overlay_layout() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "view! {",
        "<Overlay",
        "aria_labelledby=dialog_aria_labelledby",
        "aria_describedby=dialog_aria_describedby",
        "{section_view}",
        "pub const CSS: &str = r#\"",
    ] {
        assert!(
            view_source.contains(needle) || styles_source.contains(needle),
            "Modal should keep minimal static fragment path via `{needle}`."
        );
    }

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "const LONG_COPY",
        "include_str!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Modal should avoid heavy inline static-fragment expansion marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "静态片段缺省即通过（absent）",
        "components/modal/src/styles.rs::CSS",
        "modal_static_fragments_are_constantized_or_absent_for_simple_overlay_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "Modal checklist should keep static-fragment constantization evidence `{needle}`."
        );
    }
}

#[test]
fn modal_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "../../components/modal/src/mod.rs",
        "../../components/modal/src/logic.rs",
        "../../components/modal/src/styles.rs",
        "../../components/modal/src/view.rs",
        "../../components/modal/src/motion.rs",
        "../../components/modal/src/README.md",
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
                "Modal source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Modal docs examples must not contain raw-html injection token `{forbidden}`."
        );
    }

    let check2_source = load_source("../../components/modal/check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：",
        "零注入面",
        "modal_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2_source.contains(needle),
            "Modal checklist should keep inner_html security evidence `{needle}`."
        );
    }
}

#[test]
fn modal_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let modal_view_source = load_source("../../components/modal/src/view.rs");
    let modal_logic_source = load_source("../../components/modal/src/logic.rs");
    let docs_overlays_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("modal-wasm-debug"),
        "Modal should not define a component-local wasm-debug feature that leaks into production API surface."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`."
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
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_state_source.contains(needle),
            "ui-headless controllable state should emit open-change trace event via `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, overlay_dialog_attrs, use_controllable_open_state_traced};",
        "let open_state = use_controllable_open_state_traced(",
        "\"modal\",",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            modal_view_source.contains(needle),
            "Modal should expose reproducible interaction/state markers for debug tracing via `{needle}`."
        );
    }

    for needle in [
        "title=\"State + Source Markers\"",
        "<Button on_press=open_custom_modal>",
        "open: \" {move || open_custom_raw.get()}",
        "on_open_change=on_controlled_open_change",
        "Inspect data-state / data-open-mode / data-*-source markers.",
    ] {
        assert!(
            docs_overlays_source.contains(needle),
            "Modal docs playground should keep minimal replay path marker `{needle}`."
        );
    }

    let combined = format!("{modal_view_source}\n{modal_logic_source}");
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
            "Modal component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "use_controllable_open_state_traced(\"modal\"",
        "modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "Modal checklist should keep wasm-debug evidence `{needle}`."
        );
    }
}

#[test]
fn modal_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

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
        "pub(super) fn modal() -> AnyView",
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_source_path=\"components/modal/src/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "Open interactive modal",
    ] {
        assert!(
            docs_source.contains(needle),
            "Modal docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-controls\"",
        "class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (interactive_open_raw, set_interactive_open_raw) = signal(false);",
        "let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());",
        "let open_interactive_modal: OnPress =",
        "let close_interactive_modal: OnPress =",
        "\"open: \" {move || interactive_open_raw.get()}",
        "<Modal",
        "is_open=interactive_open",
        "Inspect root markers in DevTools while toggling config.",
    ] {
        assert!(
            docs_source.contains(needle),
            "Modal docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "MODAL_WORKBENCH_STORAGE_KEY",
        "load_modal_workbench_state(",
        "save_modal_workbench_state(",
        "clear_modal_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Modal keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
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
            "Modal checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn modal_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "const MODAL_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui_components::{Modal, OnPress, OverlayMotion};",
        "code_imports=MODAL_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "requested mode:",
        "requested output status:",
        "effective component status: data-ui-output-status=verified",
        "data-slot=\"modal-source-first\"",
        "data-slot=\"modal-source-paths\"",
        "component-modal",
        "inject-css",
        "compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal docs should keep copy-ready + streaming/snapshot contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "modal check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-modal\";",
        "pub const DEFAULT_TITLE: &str = \"Modal\";",
        "pub const DEFAULT_OPEN: bool = false;",
        "pub struct ModalOpenStateInput {",
        "pub is_open: Option<Signal<bool>>",
        "pub default_open: Option<bool>",
        "pub on_open_change: Option<Callback<bool>>",
        "default_open: input.default_open.unwrap_or(DEFAULT_OPEN)",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "modal API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"modal-state-matrix\"",
        "data-slot=\"modal-controlled-uncontrolled\"",
        "is_open=state_matrix_open",
        "default_open=state_matrix_default_open.get()",
        "on_open_change=on_state_matrix_open_change",
        "is_open=compare_controlled_open",
        "default_open=true",
        "on_open_change=on_compare_uncontrolled_open_change",
        "data-slot=\"modal-defaults-contract\"",
        "components/modal/src/logic.rs",
        "id_base=\\\"ui-modal\\\"",
        "title=\\\"Modal\\\"",
        "default_open=false",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal docs should keep synced example/matrix/default marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays.rs::modal",
        "modal_check2_documents_docs_sync_and_state_matrix_rules",
        "modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/modal/check2.md should keep docs-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn modal_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: modal docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn modal_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce `{needle}`.",
    );
}

#[test]
fn modal_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "modal check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/overlays.rs::modal",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"modal-defaults-contract\"",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_OPEN",
        "modal_check2_documents_docs_sync_and_state_matrix_rules",
        "modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "modal_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 docs-sync/state-matrix section should reference `{needle}`."
        );
    }
}

#[test]
fn modal_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 documentation-as-product section should include `{needle}`.",
        );
    }
}

#[test]
fn modal_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/modal/src/README.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "# Modal",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先用 `default_open + id_base + title + on_close`",
        "进阶控制：按需启用 `is_open + default_open + on_open_change`",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(needle),
            "modal README should include beginner-first marker `{needle}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("modal README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("modal README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("modal README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("modal README should include controlled advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "modal README should keep beginner-first progression order (hello -> beginner -> common -> advanced).",
    );

    for needle in [
        "component_doc!(\"Modal\", \"modal\", \"Overlays\", overlays::modal),",
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            pages_source.contains(needle) || docs_source.contains(needle),
            "modal docs entry should include `{needle}`.",
        );
    }
}

#[test]
fn modal_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: modal documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "modal check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/modal/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "modal_check2_documents_documentation_as_product_rules",
        "modal_documentation_entry_exists_with_beginner_first_progression",
        "modal_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 documentation-as-product section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 interactive-playground section should include `{needle}`.",
        );
    }
}

#[test]
fn modal_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: toggle source contracts and inspect actual normalized config.\"",
        "code_signal=interactive_code",
        "test_css_source=interactive_test_css",
        "test_source_path=\"components/modal/src/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "Switch",
        "checked=interactive_with_description",
        "set_checked=set_interactive_with_description",
        "checked=interactive_custom_motion",
        "set_checked=set_interactive_custom_motion",
        "checked=interactive_custom_exit",
        "set_checked=set_interactive_custom_exit",
        "data-slot=\"modal-interactive-controls\"",
        "data-slot=\"modal-interactive-open\"",
        "data-slot=\"modal-interactive-close\"",
        "\"open: \" {move || interactive_open_raw.get()}",
        "ModalActualConfig {",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn modal_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_modal_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "docs-app modal interactive playground replays open-close flow with stable semantic anchors",
        "[data-slot=\"modal-interactive-controls\"]",
        "[data-slot=\"modal-interactive-open\"]",
        "[data-slot=\"modal-interactive-close\"]",
        "for (const cycle of [1, 2]) {",
        "modal interactive playground cycle ${cycle}",
        "data-id-source=\"custom\"",
        "data-title-source=\"custom\"",
        "data-open-mode=\"controlled\"",
        "toHaveAttribute(\"aria-modal\", \"true\")",
        "toHaveAttribute(\"data-description\", \"present\")",
        "toHaveAttribute(\"data-open-prop-source\", \"is_open\")",
        "toHaveCount(0)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "modal interactive e2e flow should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"modal-interactive-controls\"",
        "data-slot=\"modal-interactive-open\"",
        "data-slot=\"modal-interactive-close\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal docs should expose stable interactive anchor `{needle}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn modal_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: modal interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "modal check2 should mark interactive-playground item complete.",
    );

    for needle in [
        "title=\"Interactive Playground\"",
        "data-slot=\"modal-interactive-controls\"",
        "data-slot=\"modal-interactive-open\"",
        "ModalActualConfig {",
        "N/A：`Modal` 非 AI Spec 组件",
        "modal_check2_documents_interactive_playground_rules",
        "modal_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "modal_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "modal_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 interactive-playground section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_dx_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "modal check2 should mark DX gate complete.",
    );

    for needle in [
        "playground.rs",
        "compose_scoped_css",
        "Interactive Playground",
        "optional persisted workbench state as N/A",
        "scripts/check-ui-components-dx.sh",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "components/modal/test/semantics.rs::modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "components/modal/test/semantics.rs::modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "components/modal/test/modal_semantics.rs::modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "components/modal/test/modal_semantics.rs::modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 DX section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World (Minimal Path)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "MODAL_DOC_IMPORTS",
        "compose_copy_ready_code",
        "modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "modal_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"modal-source-first\"",
        "data-slot=\"modal-source-paths\"",
        "<code>\"Show code\"</code>",
        "MODAL_DOC_IMPORTS",
        "compose_copy_ready_code",
        "component-modal",
        "inject-css",
        "components/modal/src/mod.rs",
        "components/modal/src/logic.rs",
        "components/modal/src/view.rs",
        "components/modal/src/styles.rs",
        "components/modal/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "modal source-first docs should include `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should include `{needle}`.",
        );
    }
}

#[test]
fn modal_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: modal source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include source-first marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "modal check2 should mark source-first copy-paste-ready item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/overlays.rs::modal",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "modal_check2_documents_source_first_copy_paste_ready_rules",
        "modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "modal_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_kernel_shell_layer_boundary_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 kernel-shell architecture section should include `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_kernel_shell_layer_boundary_items_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "modal_consumes_headless_open_primitive_and_avoids_store_coupling",
        "modal_reuses_headless_a11y_contract_and_exposes_locale_entrypoints",
        "modal_motion_non_wasm_noop_stub_contract_is_predictable",
        "styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "modal_component_files_keep_layer_responsibilities",
        "scripts/check-ui-components-component-files.sh",
        "scripts/check-ui-components-entrypoints.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 kernel-shell architecture section should retain evidence marker `{needle}`.",
        );
    }
}

#[test]
fn modal_engineering_script_covers_kernel_shell_layer_boundary_check2_completion() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_marks_kernel_shell_layer_boundary_items_complete";
    assert!(
        script_source.contains(needle),
        "engineering check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let readme_source = load_source("../../components/modal/src/README.md");

    for needle in [
        "### Modal 同步记录（2026-02-20）",
        "参数模型同步：`Modal` 参数主轴保持 `is_open/default_open/on_open_change`",
        "component_doc!(\"Modal\", \"modal\", \"Overlays\", overlays::modal)",
        "`apps/docs-app/src/pages/components/pages/overlays.rs::modal()`",
        "`components/modal/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include modal synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Modal\"",
        "\"modal\"",
        "overlays::modal",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose modal entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn modal() -> AnyView {",
        "title=\"Modal\"",
        "slug=\"modal\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app modal page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in ["# Modal", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(needle),
            "modal README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn modal_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: modal heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "modal_check2_documents_heroui_benchmark_docs_sync_rules",
        "modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "modal_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let protocol_source = load_source("../../components/modal/src/protocol.rs");

    assert!(
        mod_source.contains("pub mod protocol;"),
        "modal module should expose `protocol` for schema migration contract discoverability."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum ModalComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct ModalComponentSpec",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(default)]",
        "pub schema_version: ModalComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "modal protocol should keep serde/schema contract marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "from_json(",
        "to_json_result(",
        "SchemaError",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "modal protocol should avoid ad-hoc serde drift token `{forbidden}`."
        );
    }
}

#[test]
fn modal_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/modal/src/Component.toml");
    let rbi_source = load_source("../../components/modal/src/modal.rbi");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let protocol_source = load_source("../../components/modal/src/protocol.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Modal\"",
        "crate = \"ui-modal\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "modal manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Modal(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "modal RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
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
            "modal should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Modal` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "modal_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn modal_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let marker = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("../../components/modal/src/mod.rs"),
        load_source("../../components/modal/src/logic.rs"),
        load_source("../../components/modal/src/view.rs"),
        load_source("../../components/modal/src/styles.rs"),
        load_source("../../components/modal/src/motion.rs"),
        load_source("../../components/modal/src/protocol.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("modal-wasm-debug")
            && !cargo_source.contains("modal_wasm_debug")
            && !cargo_source.contains("component-modal-wasm-debug"),
        "modal should not define component-local tracing feature aliases."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::modal::",
        "const MODAL_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "modal should avoid tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let protocol_source = load_source("../../components/modal/src/protocol.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
        &protocol_source,
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
                "modal engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "modal public module boundary should not leak web_sys types."
    );
}

#[test]
fn modal_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn modal_check2_marks_engineering_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "modal check2 should mark engineering gate complete.",
    );

    for needle in [
        "components/modal/src/protocol.rs",
        "ModalComponentSchemaVersion",
        "ModalComponentSpec",
        "button-wasm-debug",
        "target: \"ui_components::button::state_change\"",
        "scripts/check-ui-components-engineering.sh",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "components/modal/test/semantics.rs::engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "components/modal/test/semantics.rs::engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "components/modal/test/semantics.rs::engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "components/modal/test/modal_semantics.rs::modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "components/modal/test/modal_semantics.rs::modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "components/modal/test/modal_semantics.rs::modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 engineering section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for required in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "--ui-overlay-panel-min-width,",
        "var(--ui-fallback-overlay-panel-min-width)",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "--ui-overlay-viewport-inset,",
        "var(--ui-fallback-overlay-viewport-inset)",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "--ui-heading-h5-font-size,",
        "var(--ui-fallback-heading-h5-font-size)",
        "--ui-heading-h5-line-height,",
        "var(--ui-fallback-heading-h5-line-height)",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "--ui-text-field-motion-duration,",
        "var(--ui-fallback-text-field-motion-duration)",
    ] {
        assert!(
            styles_source.contains(required),
            "modal styles should keep defensive fallback chain marker `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-md:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-space-3xs:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-border-width:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-heading-h5-font-size:",
        "--ui-fallback-heading-h5-line-height:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-text-field-motion-duration:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-space-md);",
        "var(--ui-space-lg);",
        "var(--ui-overlay-panel-min-width);",
        "var(--ui-overlay-viewport-inset);",
        "var(--ui-fg);",
        "var(--ui-fg-muted);",
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "translateY(1px)",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "modal styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn modal_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_defensive_variables_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "modal check2 should mark defensive-variables gate complete.",
    );

    for needle in [
        "modal_styles_use_defensive_variable_fallback_chain",
        "modal_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/modal/src/styles.rs",
        "crates/ui-theme/src/css.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 defensive-variables section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "modal view should not embed plain inline style assignments."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "modal view should not include fragile inline style token `{forbidden}`."
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "modal runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for needle in ["pub const CSS: &str", ".ui-modal", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "modal styles should remain static token css contract `{needle}`."
        );
    }
}

#[test]
fn modal_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "modal check2 should mark cascade-layer gate complete.",
    );

    for needle in [
        "modal_cascade_layer_and_runtime_style_contract_is_enforced",
        "modal_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/modal/src/view.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 cascade-layer section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let modal_motion_source = load_source("../../components/modal/src/motion.rs");
    let modal_view_source = load_source("../../components/modal/src/view.rs");
    let overlay_view_source = load_source("../../components/overlay/src/view.rs");
    let overlay_motion_source = load_source("../../components/overlay/src/motion.rs");
    let ui_motion_spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "MODAL_MOTION_CONTRACT_STIFFNESS",
        "MODAL_MOTION_CONTRACT_DAMPING",
        "MODAL_MOTION_CONTRACT_MASS",
        "MODAL_MOTION_CONTRACT_PRECISION",
        "pub fn default_motion_contract() -> OverlayMotion",
        "spring: ui_motion::spring::SpringConfig {",
        "stiffness: MODAL_MOTION_CONTRACT_STIFFNESS",
        "damping: MODAL_MOTION_CONTRACT_DAMPING",
        "if sanitized == OverlayMotion::default()",
        "overlay_motion::sanitize_motion(default_motion_contract())",
    ] {
        assert!(
            modal_motion_source.contains(needle),
            "modal motion contract should define typed spring parameters via `{needle}`."
        );
    }

    for needle in [
        "let motion = motion_contract::normalize_motion(motion);",
        "motion=motion",
    ] {
        assert!(
            modal_view_source.contains(needle),
            "modal view should pass normalized motion contract via `{needle}`."
        );
    }

    let attach_needle = "motion::attach_motion(root_ref, open, on_exit_complete, motion);";
    assert!(
        overlay_view_source.contains(attach_needle),
        "overlay view should attach motion contract via `{attach_needle}`."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "finish_exit.run(());",
    ] {
        assert!(
            overlay_motion_source.contains(needle),
            "overlay motion bridge should keep wasm/non-wasm safe attach path `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should honor reduced-motion via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should stay predictable no-op via `{needle}`."
        );
    }
}

#[test]
fn modal_motion_contract_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_motion_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "modal check2 should mark motion-contract gate complete.",
    );

    for needle in [
        "MODAL_MOTION_CONTRACT_STIFFNESS",
        "MODAL_MOTION_CONTRACT_DAMPING",
        "overlay motion::attach_motion(root_ref, open, on_exit_complete, motion)",
        "if crate::web::prefers_reduced_motion() {",
        "components/modal/test/semantics.rs::motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "components/modal/test/modal_semantics.rs::modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 motion-contract section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib_source = load_source("src/lib.rs");
    let ui_components_css_source = load_source("src/css.rs");
    let ui_components_root_source = load_source("src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_components_src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let ui_headless_src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui-headless/src");

    for needle in [
        "#[cfg(feature = \"component-modal\")]",
        "#[path = \"../../../components/modal/src/mod.rs\"]",
        "pub mod modal;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui-components lib entry should keep feature-gated modal/root surface via `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "HtmlElement"] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform details in public surface `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui-components css entry should stay feature-gated and no-op safe via `{needle}`."
        );
    }

    for needle in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "UiRoot should centralize theme/i18n/css injection via `{needle}`."
        );
    }

    for needle in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep generic motion contract via `{needle}`."
        );
    }

    for forbidden in ["Accordion", "Modal", "Popover", "Tooltip", "MenuItem"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`."
        );
    }

    for absent in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = ui_components_src_dir.join(absent);
        assert!(
            !path.exists(),
            "ui-components should not add forbidden entrypoint file `{}`.",
            path.display()
        );
    }

    for present in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        let path = ui_headless_src_dir.join(present);
        assert!(
            path.exists(),
            "ui-headless canonical primitive entrypoint should exist `{}`.",
            path.display()
        );
    }
}

#[test]
fn modal_entrypoints_check_script_covers_fixed_entry_files_gate() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "modal check2 should mark ui-components fixed-entry gate complete.",
    );

    for needle in [
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "modal_ui_components_fixed_entry_files_follow_layered_boundaries",
        "scripts/check-ui-components-entrypoints.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 fixed-entry section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_component_directory_standard_files_follow_contract_and_na_paths() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/modal/src");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "modal component standard file should exist `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "modal simple component should not add `{}`.",
            path.display()
        );
    }

    assert!(
        mod_source.contains("pub use view::Modal;"),
        "mod.rs should keep minimal stable export surface through `pub use view::Modal;`."
    );
    for forbidden in ["pub mod logic", "pub mod view", "pub mod motion"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not over-export internal implementation module `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_open_state(",
        "pub fn resolve_open_contract(",
        "pub fn resolve_content_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "NodeRef", "HtmlElement", "style="] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry render/platform/style detail `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should stay token-first static css via `{needle}`."
        );
    }
    for forbidden in ["view! {", "on:click=", "Signal<", "Callback<"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry runtime/view/event logic `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, overlay_dialog_attrs, use_controllable_open_state_traced};",
        "<Overlay",
        "data-open-mode=open_contract.mode.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount leptos structure + headless semantics via `{needle}`."
        );
    }
    for forbidden in ["overlay_motion::sanitize_motion(", "pub const CSS: &str"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not absorb motion-engine/style implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub fn default_motion_contract() -> OverlayMotion",
        "pub fn normalize_motion(motion: OverlayMotion) -> OverlayMotion",
        "overlay_motion::sanitize_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep semantic->motion contract mapping marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "on:click=", "requestAnimationFrame"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not carry view/event/driver detail `{forbidden}`."
        );
    }
}

#[test]
fn modal_component_files_check_script_covers_standard_layout_gate() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_component_directory_standard_files_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 组件目录标准文件落点正确。"),
        "modal check2 should mark component-directory-standard-files gate complete.",
    );

    for needle in [
        "components/modal/src/mod.rs",
        "components/modal/src/logic.rs",
        "components/modal/src/styles.rs",
        "components/modal/src/view.rs",
        "components/modal/src/motion.rs",
        "render.rs",
        "spec.rs",
        "modal_component_directory_standard_files_follow_contract_and_na_paths",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 standard-file section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_file_placement_discipline_is_strict_for_component_scope() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/modal/src");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let protocol_source = load_source("../../components/modal/src/protocol.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "modal file-placement discipline requires core file `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "modal should not introduce forbidden placement file `{}`.",
            path.display()
        );
    }

    assert!(
        component_src_dir.join("protocol.rs").exists(),
        "modal keeps versioned schema in protocol.rs as explicit repository-level exception."
    );
    for needle in [
        "pub enum ModalComponentSchemaVersion",
        "pub struct ModalComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should stay schema-only via `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for needle in [
        "pub use view::Modal;",
        "pub fn normalize_open_state(",
        "pub const CSS: &str",
        "view! {",
        "pub fn normalize_motion(motion: OverlayMotion) -> OverlayMotion",
    ] {
        assert!(
            combined.contains(needle),
            "file-placement discipline should keep core-layer marker `{needle}`."
        );
    }
}

#[test]
fn modal_file_placement_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_file_placement_discipline_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "modal check2 should mark file-placement-discipline gate complete.",
    );

    for needle in [
        "components/modal/src/mod.rs",
        "components/modal/src/logic.rs",
        "components/modal/src/styles.rs",
        "components/modal/src/view.rs",
        "components/modal/src/motion.rs",
        "components/modal/src/protocol.rs",
        "render.rs",
        "spec.rs",
        "modal_file_placement_discipline_is_strict_for_component_scope",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 file-placement section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/modal/src");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let protocol_source = load_source("../../components/modal/src/protocol.rs");
    let check2_source = load_source("../../components/modal/check2.md");

    let spec_path = component_src_dir.join("spec.rs");
    assert!(
        !spec_path.exists(),
        "modal is not a complex spec-first component; spec.rs should remain absent."
    );

    assert!(
        component_src_dir.join("protocol.rs").exists(),
        "modal keeps versioned schema in protocol.rs instead of introducing spec.rs."
    );
    for needle in [
        "pub enum ModalComponentSchemaVersion",
        "pub struct ModalComponentSpec",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep versioned schema marker `{needle}`."
        );
    }

    let combined = format!("{mod_source}\n{view_source}\n{logic_source}");
    for forbidden in [
        "Spec::new()",
        ".render()",
        "pub struct ModalSpec",
        "impl ModalSpec",
    ] {
        assert!(
            !combined.contains(forbidden),
            "modal should not expose complex builder API token `{forbidden}`."
        );
    }

    for needle in [
        "Hyper-Structure Builder（`spec.rs`）",
        "N/A（`modal` 非复杂 builder/spec 组件",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 should document hyper-structure-builder applicability via `{needle}`."
        );
    }
}

#[test]
fn modal_hyper_structure_builder_check_script_covers_na_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn modal_check2_marks_hyper_structure_builder_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "modal check2 should mark hyper-structure-builder gate complete.",
    );

    for needle in [
        "components/modal/src/spec.rs",
        "components/modal/src/protocol.rs",
        "modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 hyper-structure-builder section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "../../components/modal/src/Component.toml",
        "../../components/modal/src/modal.rbi",
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(required_file);
        assert!(
            path.exists(),
            "modal context-compression artifact should exist: `{required_file}`."
        );
    }

    let manifest_source = load_source("../../components/modal/src/Component.toml");
    let rbi_source = load_source("../../components/modal/src/modal.rbi");
    let view_source = load_source("../../components/modal/src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Modal\"",
        "crate = \"ui-modal\"",
        "name = \"is_open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"id_base\"",
        "name = \"title\"",
        "name = \"on_close\"",
        "name = \"description\"",
        "name = \"motion\"",
        "name = \"on_exit_complete\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"class_name\"",
        "name = \"children\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "modal Component.toml should include context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ModalSlot {",
        "pub enum ModalDescriptionState {",
        "pub const DEFAULT_ID_BASE: &str;",
        "pub const DEFAULT_TITLE: &str;",
        "pub const DEFAULT_OPEN: bool;",
        "pub struct ModalOpenContract {",
        "pub struct ModalComponentSpec {",
        "pub fn Modal(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "id_base: String",
        "title: String",
        "on_close: crate::OnPress",
        "description: Option<String>",
        "motion: crate::overlay::OverlayMotion",
        "on_exit_complete: Option<leptos::prelude::Callback<()>>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "class_name: Option<String>",
        "children: leptos::children::ChildrenFn",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "modal RBI projection should keep signature marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Modal(",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional)] motion: OverlayMotion",
        "#[prop(optional)] on_exit_complete: Option<Callback<()>>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: ChildrenFn,",
    ] {
        assert!(
            view_source.contains(needle),
            "modal view signature should include `{needle}` for manifest/rbi drift detection."
        );
    }
}

#[test]
fn modal_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "echo \"[component-files] contract: modal context-compression manifest + rbi projection\"",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce modal context-compression gate `{needle}`."
        );
    }
}

#[test]
fn modal_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "modal check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/modal/src/Component.toml",
        "components/modal/src/modal.rbi",
        "modal_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "modal_component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/modal/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "modal_agent_contract_is_schema_typed_and_machine_readable",
        "modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "modal checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn modal_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let view_source = load_source("../../components/modal/src/view.rs");

    for needle in [
        "pub const MODAL_AGENT_SCHEMA: &str = \"ui.modal.agent-contract\";",
        "pub enum ModalAgentSchemaVersion",
        "pub enum ModalAgentIntent",
        "pub enum ModalAgentAction",
        "pub enum ModalAgentState",
        "pub enum ModalAgentSource",
        "pub enum ModalAgentConfigPolicy",
        "pub enum ModalAgentOutputStatus",
        "pub struct ModalAgentCapabilities",
        "pub struct ModalAgentContractInput",
        "pub struct ModalAgentContract",
        "pub fn resolve_agent_contract(input: ModalAgentContractInput) -> ModalAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "modal logic should keep typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::ModalAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-capability-description=move || {",
        "data-ui-capability-open=move || {",
        "data-ui-capability-close=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "modal view should mount schemaized agent marker `{needle}`."
        );
    }
}

#[test]
fn modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let view_source = load_source("../../components/modal/src/view.rs");

    for typed_source in [
        "schema_name: MODAL_AGENT_SCHEMA,",
        "schema_version: ModalAgentSchemaVersion::V1,",
        "intent: ModalAgentIntent::OverlayDialog,",
        "ModalAgentAction::Open",
        "ModalAgentAction::Close",
        "ModalAgentState::Open",
        "ModalAgentState::Closed",
        "ModalAgentSource::Controlled",
        "ModalAgentSource::Uncontrolled",
        "config_policy: ModalAgentConfigPolicy::Whitelist,",
        "output_status: ModalAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "modal agent fields should stay type-derived via `{typed_source}`."
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
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "modal agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
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
            "modal render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn modal_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "modal_check2_documents_agent_contract_schema_governance_rules",
        "modal_agent_contract_is_schema_typed_and_machine_readable",
        "modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "modal_contract_hygiene_script_covers_agent_contract_schema_guards",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 should keep Agent Contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/modal/check2.md");
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`Modal` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "modal check2 should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "modal runtime path should not embed LLM streaming protocol marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn modal_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    let needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn modal_check2_marks_streaming_two_mode_definition_complete() {
    let source = load_source("../../components/modal/check2.md");

    assert!(
        source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "modal check2 should mark streaming two-mode definition gate complete.",
    );

    for needle in [
        "modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "modal_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 streaming section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`Modal` 不直接渲染 LLM 正文",
        "modal_check2_documents_snapshot_as_default_baseline_capability",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 should keep snapshot-baseline marker `{needle}`.",
        );
    }
}

#[test]
fn modal_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let check2_source = load_source("../../components/modal/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "let open_state = logic::normalize_open_state(logic::ModalOpenStateInput {",
        "let open_contract = logic::resolve_open_contract(&open_state);",
        "let content_state = logic::resolve_content_state(logic::ModalContentStateInput {",
        "let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "modal snapshot baseline should keep stable complete-result render marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_open_state(input: ModalOpenStateInput) -> ModalOpenState",
        "pub fn resolve_open_contract(state: &ModalOpenState) -> ModalOpenContract",
        "pub fn resolve_content_state(input: ModalContentStateInput) -> ModalContentState",
        "pub fn resolve_state(input: ModalPartStateInput) -> ModalPartState",
    ] {
        assert!(
            logic_source.contains(needle),
            "modal logic should keep snapshot-baseline normalization marker `{needle}`.",
        );
    }

    for forbidden in [
        "streaming_chunk",
        "token_delta",
        "partial token",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "modal snapshot baseline should avoid incremental streaming marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "modal_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 snapshot section should reference `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn modal_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_snapshot_baseline_capability_complete() {
    let source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "modal_check2_documents_snapshot_as_default_baseline_capability",
        "modal_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "modal_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 snapshot-baseline section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/modal/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`Modal` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 should keep streaming required/optional rule `{needle}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let overlay_view_source = load_source("../../components/overlay/src/view.rs");

    for needle in [
        "<Overlay",
        "aria_labelledby=dialog_aria_labelledby",
        "aria_describedby=dialog_aria_describedby",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "modal optional-streaming scope should keep semantic continuity marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum ModalAgentOutputStatus",
        "ModalAgentOutputStatus::Verified",
        "data-output-status",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "modal optional-streaming scope should expose explicit output-status marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-role=role",
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
    ] {
        assert!(
            overlay_view_source.contains(needle),
            "overlay should keep role/aria/data continuity marker `{needle}` for modal optional-streaming path.",
        );
    }
}

#[test]
fn modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/modal/src/view.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "modal should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn modal_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "modal_check2_documents_streaming_required_optional_classification_rules",
        "modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "modal_streaming_script_covers_required_optional_classification_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "modal check2 should keep required/optional classification evidence marker `{needle}`.",
        );
    }
}

#[test]
fn modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("../../components/modal/src/mod.rs");
    let logic_source = load_source("../../components/modal/src/logic.rs");
    let styles_source = load_source("../../components/modal/src/styles.rs");
    let view_source = load_source("../../components/modal/src/view.rs");
    let motion_source = load_source("../../components/modal/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "modal non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/modal/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(state.base_class)",
        "Cow::Borrowed(\"ui-modal--custom-class\")",
        "Cow::Owned(base_class_name)",
        ") -> Cow<'static, str>",
    ] {
        assert!(
            logic_source.contains(required),
            "modal logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-modal--with-description\".to_string()",
        "\"ui-modal--title-only\".to_string()",
        "\"ui-modal--custom-id\".to_string()",
        "\"ui-modal--custom-title\".to_string()",
        "\"ui-modal--custom-description\".to_string()",
        "\"ui-modal--custom-motion\".to_string()",
        "\"ui-modal--custom-exit\".to_string()",
        "\"ui-modal--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "modal logic should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn modal_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
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
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn modal_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "modal_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "modal check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let local_semantics = include_str!("../../../components/modal/test/semantics.rs");
    let aggregated_semantics = load_source("tests/modal_semantics.rs");
    let modal_view_source = load_source("../../components/modal/src/view.rs");
    let overlay_view_source = load_source("../../components/overlay/src/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only()",
        "fn modal_focus_restore_delegates_to_overlay_focus_manager_stack()",
        "fn modal_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "data-state=root_state.state_attr",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "aria_labelledby=dialog_aria_labelledby",
        "aria_describedby=dialog_aria_describedby",
    ] {
        assert!(
            modal_view_source.contains(marker),
            "Modal view should expose semantic/data marker `{marker}`."
        );
    }

    for marker in [
        "role=role",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev: ev::PointerEvent| ev.stop_propagation()",
    ] {
        assert!(
            overlay_view_source.contains(marker),
            "Overlay view should expose aria/focus interaction marker `{marker}`."
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless focus manager stack should expose focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`."
        );
    }
}

#[test]
fn modal_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn modal_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only",
        "modal_focus_restore_delegates_to_overlay_focus_manager_stack",
        "modal_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "modal check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn modal_check2_marks_semantic_test_priority_item_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "modal check2 should mark semantic-test-priority item complete."
    );

    for marker in [
        "components/modal/test/semantics.rs::modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only",
        "components/modal/test/semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "components/modal/test/modal_semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "modal check2 semantic-test-priority section should include `{marker}`."
        );
    }
}

#[test]
fn modal_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/modal/check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(marker),
            "modal check2 should keep e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn modal_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_modal_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for marker in [
        "page.goto(\"/#/components/modal\")",
        "body:not(:has(#boot))",
        "[data-component=\"modal\"]",
        "[data-slot=\"modal-e2e-described-controls\"]",
        "[data-slot=\"modal-e2e-open-described\"]",
        "[data-slot=\"overlay-panel\"][role=\"dialog\"][aria-labelledby=\"docs-modal-semantic-title\"]",
        "[data-slot=\"modal\"]",
        "toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "toHaveAttribute(\"data-open-source\", \"controlled\")",
        "toHaveAttribute(\"data-open-change-source\", \"none\")",
        "toHaveAttribute(\"data-open-prop-source\", \"is_open\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "modal e2e selector contract should keep semantic marker `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"modal-e2e-described-controls\"",
        "data-slot=\"modal-e2e-open-described\"",
        "data-slot=\"modal-e2e-custom-controls\"",
        "data-slot=\"modal-e2e-open-custom\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "modal docs source should keep e2e semantic anchor `{marker}`."
        );
    }

    for forbidden in [
        "getByText(",
        "nth-child(",
        "waitForTimeout(",
        "setTimeout(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "modal e2e selector contract should avoid brittle/snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal() {
    let e2e_source = load_source("../../e2e/tests/docs_app_modal_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for marker in [
        "[data-slot=\"modal-e2e-custom-controls\"]",
        "[data-slot=\"modal-e2e-open-custom\"]",
        "[data-slot=\"overlay-panel\"][role=\"dialog\"][aria-labelledby=\"docs-modal-custom-title\"]",
        "[data-slot=\"overlay\"]",
        "[data-slot=\"overlay-backdrop\"]",
        "await customBackdrop.click();",
        "await describedPanel.press(\"Escape\");",
        "await expect(describedPanel).toHaveCount(0);",
        "await expect(customPanel).toHaveCount(0);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "modal e2e ready/settled contract should include `{marker}`."
        );
    }

    assert!(
        docs_source.contains("on_close=close_semantic"),
        "modal docs should keep controllable close callback wiring for Escape settled path."
    );
    assert!(
        docs_source.contains("on_close=close_custom"),
        "modal docs should keep controllable close callback wiring for backdrop settled path."
    );
}

#[test]
fn modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_modal_contract.spec.mjs");

    for marker in [
        "docs-app modal critical flow is replayable with overlay focus and keyboard checkpoints",
        "for (const cycle of [1, 2]) {",
        "modal critical flow cycle ${cycle}",
        "await openDescribed.focus();",
        "await expect(openDescribed).toBeFocused();",
        "await expect(describedPanel).toHaveAttribute(\"aria-modal\", \"true\");",
        "await expect(describedModal).toHaveAttribute(\"data-open-mode\", \"controlled\");",
        "await expect(describedModal).toHaveAttribute(\"data-open-source\", \"controlled\");",
        "await expect(describedModal).toHaveAttribute(\"data-open-prop-source\", \"is_open\");",
        "await expectFocusInsidePanel(describedPanel);",
        "await page.keyboard.press(\"Tab\");",
        "await describedPanel.press(\"Escape\");",
        "await expect(describedPanel).toHaveCount(0);",
        "await expect(openDescribed).toBeFocused();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "modal e2e regression flow should keep semantic breakpoint marker `{marker}`."
        );
    }
}

#[test]
fn modal_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-modal.sh");

    for marker in [
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_uses_semantic_selectors_and_stable_waits",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal",
        "cargo test -p ui-components --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "modal e2e check script should include `{marker}`."
        );
    }
}

#[test]
fn modal_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "modal check2 should mark e2e selector stability item complete."
    );

    for marker in [
        "components/modal/test/semantics.rs::modal_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/modal/test/semantics.rs::modal_e2e_contract_uses_semantic_selectors_and_stable_waits",
        "components/modal/test/semantics.rs::modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal",
        "components/modal/test/modal_semantics.rs::modal_e2e_contract_uses_semantic_selectors_and_stable_waits",
        "scripts/check-ui-components-e2e-modal.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "modal check2 e2e selector stability section should include `{marker}`."
        );
    }
}

#[test]
fn modal_check2_marks_replayable_e2e_critical_flow_item_complete() {
    let check2_source = load_source("../../components/modal/check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "modal check2 should mark replayable e2e critical-flow item complete."
    );

    for marker in [
        "docs-app modal critical flow is replayable with overlay focus and keyboard checkpoints",
        "for (const cycle of [1, 2])",
        "await expect(describedPanel).toHaveAttribute(\"aria-modal\", \"true\")",
        "await expect(describedModal).toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "await expect(openDescribed).toBeFocused()",
        "modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints",
        "scripts/check-ui-components-e2e-modal.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "modal check2 replayable e2e critical-flow section should include `{marker}`."
        );
    }
}
