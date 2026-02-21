use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_empty_state_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/empty-state").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_empty_state_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-empty_state\")]")
            && lib_source.contains("pub use ui_empty_state as empty_state;"),
        "ui-components should re-export the external ui-empty-state crate as `empty_state`.",
    );
    assert!(
        cargo_source.contains("component-empty_state = [\"dep:ui-empty-state\"]"),
        "component-empty_state feature should depend on dep:ui-empty-state after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-empty-state = { path = \"../../components/empty-state\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-empty-state dependency.",
    );
}

#[test]
fn empty_state_does_not_expose_logic_or_view_modules() {
    let source = load_empty_state_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "EmptyState internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_state_uses_logic_state_model() {
    let logic_source = load_empty_state_component_source("src/logic.rs");
    let view_source = load_empty_state_component_source("src/view.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/empty_state.rs");

    for needle in [
        "pub use ui_state_primitives::empty_state::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_DESCRIPTION",
        "DEFAULT_TITLE",
        "EmptyStateAlign",
        "EmptyStateState",
        "EmptyStateStateInput",
        "EmptyStateTone",
        "compose_class_name",
        "normalize_aria_label",
        "normalize_description",
        "normalize_optional_text",
        "normalize_title",
        "resolve_defaults",
        "EmptyStateResolvedDefaults",
        "resolve_render_state",
        "EmptyStateRenderStateInput",
        "EmptyStateRenderState",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "EmptyState logic should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub enum EmptyStateTone",
        "pub enum EmptyStateAlign",
        "pub struct EmptyStateStateInput",
        "pub struct EmptyStateState",
        "pub const DEFAULT_TITLE: &str = \"Nothing to show\";",
        "pub const DEFAULT_DESCRIPTION: &str = \"Try adjusting filters or refreshing data.\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Empty state\";",
        "pub fn normalize_optional_text(",
        "pub fn normalize_title(",
        "pub fn normalize_description(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "title_source_attr",
        "description_source_attr",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "EmptyState primitives should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<EmptyStateStrings>()",
        "live_region_attrs(LiveRegionPriority::Polite)",
        "logic::resolve_defaults(",
        "strings.default_title.as_ref()",
        "strings.default_description.as_ref()",
        "strings.default_aria_label.as_ref()",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "logic::resolve_render_state(logic::EmptyStateRenderStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get().state)",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "let motion = motion::sanitize_motion(motion);",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "EmptyState view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn empty_state_emits_baseline_style_state_data_attributes() {
    let source = load_empty_state_component_source("src/view.rs");

    for attr in [
        "data-slot=\"empty-state\"",
        "data-slot=\"empty-state-icon\"",
        "data-slot=\"empty-state-title\"",
        "data-slot=\"empty-state-description\"",
        "data-slot=\"empty-state-actions\"",
        "data-tone=move || state.get().state.tone_attr",
        "data-align=move || state.get().state.align_attr",
        "data-state=move || state.get().state.data_state_attr",
        "data-compact=move || state.get().state.is_compact.then_some(\"true\")",
        "data-bordered=move || state.get().state.is_bordered.then_some(\"true\")",
        "data-icon=move || state.get().state.has_icon.then_some(\"true\")",
        "data-actions=move || state.get().state.has_actions.then_some(\"true\")",
        "data-title-source=move || state.get().state.title_source_attr",
        "data-description-source=move || state.get().state.description_source_attr",
        "data-aria-source=move || state.get().state.aria_source_attr",
        "data-custom-class=move || state.get().state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().state.class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-label=aria_label",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "EmptyState should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn empty_state_styles_include_tone_align_and_markers() {
    let source = load_empty_state_component_source("src/styles.rs");

    for selector in [
        ".ui-empty-state--tone-default",
        ".ui-empty-state[data-tone=\"default\"]",
        ".ui-empty-state--tone-muted",
        ".ui-empty-state[data-tone=\"muted\"]",
        ".ui-empty-state--tone-accent",
        ".ui-empty-state[data-tone=\"accent\"]",
        ".ui-empty-state--align-start",
        ".ui-empty-state[data-align=\"start\"]",
        ".ui-empty-state--align-center",
        ".ui-empty-state[data-align=\"center\"]",
        ".ui-empty-state--compact",
        ".ui-empty-state[data-compact=\"true\"]",
        ".ui-empty-state--bordered",
        ".ui-empty-state[data-bordered=\"true\"]",
        ".ui-empty-state--custom-class",
        ".ui-empty-state[data-custom-class=\"true\"]",
        ".ui-empty-state[data-motion-source=\"custom\"]",
        ".ui-empty-state[data-custom-motion=\"true\"]",
        "--ui-empty-state-enter",
        "var(--ui-component-height-100)",
        "var(--ui-space-sm)",
        "var(--ui-radius-lg)",
        "var(--ui-heading-h5-font-size)",
        "var(--ui-heading-h5-line-height)",
        "var(--ui-font-size-150)",
        "var(--ui-line-height-150)",
        "var(--ui-space-3xs)",
        "prefers-reduced-motion: reduce",
    ] {
        assert!(
            source.contains(selector),
            "EmptyState styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn empty_state_styles_do_not_use_local_typography_fallback_constants() {
    let source = load_empty_state_component_source("src/styles.rs");

    for forbidden in [
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
    ] {
        assert!(
            !source.contains(forbidden),
            "EmptyState styles should consume theme typography tokens directly; found `{forbidden}`."
        );
    }
}

#[test]
fn empty_state_docs_page_covers_primary_playgrounds() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );

    for needle in [
        "pub(super) fn empty_state() -> AnyView",
        "title=\"EmptyState\"",
        "slug=\"empty-state\"",
        "description=\"baseline-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers.\"",
        "let empty_state_imports =",
        "<Playground title=\"Hello World (Default Path)\" code_signal=hello_code>",
        "<Playground\n                title=\"State Matrix\"",
        "<Playground title=\"Tone + Alignment + Actions\" code_signal=tone_code>",
        "<Playground title=\"Compact + Bordered + Custom Class\" code_signal=state_code>",
        "<Playground\n                title=\"Controlled vs Uncontrolled (N/A)\"",
        "<Playground\n                title=\"Streaming Optional / Snapshot\"",
        "<Playground\n                title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
        "code_imports=empty_state_imports.clone()",
        "<EmptyState",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs page should include `{needle}` for empty_state primary playground coverage.",
        );
    }
}

#[test]
fn empty_state_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<EmptyState />\"#.to_string());",
        "let state_matrix_code = Signal::derive(move || {",
        "title=\"Hello World (Default Path)\"",
        "<EmptyState />",
        "title=\"State Matrix\"",
        "title=\"Nothing matched\".to_string()",
        "description=\"Try a different query or clear filters.\".to_string()",
        "tone=EmptyStateTone::Muted",
        "align=EmptyStateAlign::Center",
        "title=\"Tone + Alignment + Actions\"",
        "title=\"No projects yet\".to_string()",
        "description=\"Create your first project to unlock dashboards and team workflows.\".to_string()",
        "tone=EmptyStateTone::Default",
        "icon=move || view! { <span>\"📁\"</span> }",
        "tone=EmptyStateTone::Muted",
        "align=EmptyStateAlign::Center",
        "title=\"Compact + Bordered + Custom Class\"",
        "title=\"Deployments paused\".to_string()",
        "description=\"Approvals are required before resuming this environment.\".to_string()",
        "tone=EmptyStateTone::Accent",
        "is_compact=true",
        "is_bordered=true",
        "class_name=\"docs-empty-state-custom\".to_string()",
        "icon=move || view! { <span>\"⏸\"</span> }",
        "variant=ui_components::ButtonVariant::Secondary",
        "\"Review approvals\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "EmptyState has no controlled/uncontrolled runtime axis",
        "title=\"Streaming Optional / Snapshot\"",
        "streaming is optional and falls back to snapshot rendering",
        "title=\"Snapshot baseline\".to_string()",
        "title=\"Streaming optional fallback\".to_string()",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
        "code_imports=empty_state_imports",
    ] {
        assert!(
            source.contains(needle),
            "empty_state docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_state_docs_source_first_contract_points_to_copy_ready_runtime_and_real_paths() {
    let docs_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"empty-state-source-first-contract\"",
        "data-slot=\"empty-state-source-first-paths\"",
        "data-slot=\"empty-state-source-first-deps\"",
        "components/empty-state/src/mod.rs",
        "components/empty-state/src/logic.rs",
        "components/empty-state/src/view.rs",
        "components/empty-state/src/styles.rs",
        "components/empty-state/src/motion.rs",
        "component-empty_state",
        "inject-css",
        "UiRoot",
    ] {
        assert!(
            docs_source.contains(needle),
            "empty_state source-first docs contract should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "Show code",
        "Hide code",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy runtime should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_state_heroui_strategy_sync_entry_is_present_and_indexable() {
    let strategy_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### EmptyState 同步记录（2026-02-20）",
        "`EmptyState` 维持 display primitive 定位",
        "component_doc!(\"EmptyState\", \"empty-state\", \"Display\", display_extra::empty_state)",
        "`#/components/empty-state` 可索引访问",
        "display_extra.rs::empty_state()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "empty_state heroui strategy sync contract should contain `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(\"EmptyState\", \"empty-state\", \"Display\", display_extra::empty_state)",
        "component_doc!(\"ErrorView\", \"error-view\", \"Display\", display_extra::error_view)",
    ] {
        assert!(
            docs_index_source.contains(needle),
            "docs index should contain empty_state discoverable entry `{needle}`.",
        );
    }
}

#[test]
fn empty_state_docs_include_interactive_playground_contract() {
    let docs_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_empty_state_contract.spec.mjs");

    for needle in [
        "title=\"Interactive Playground\"",
        "Interactive acceptance canvas: tune props/state and verify semantic markers in real time.",
        "let workbench_code = Signal::derive(move || {",
        "let workbench_actual_config = Signal::derive(move || {",
        "test_source_path=\"components/empty-state/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"empty-state-workbench-controls\"",
        "id_base=\"docs-empty-state-workbench-tone\".to_string()",
        "id_base=\"docs-empty-state-workbench-align\".to_string()",
        "data-slot=\"empty-state-workbench\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "empty_state interactive docs contract should contain `{needle}`.",
        );
    }

    for needle in [
        "docs-app empty-state interactive playground keeps live preview in sync",
        "async function runInteractiveWorkbenchFlow(docsRoot) {",
        "await page.reload();",
        "data-state\", \"rich\"",
        "data-state\", \"plain\"",
    ] {
        assert!(
            e2e_source.contains(needle),
            "empty_state interactive e2e contract should contain `{needle}`.",
        );
    }
}

#[test]
fn empty_state_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_ui_components_source("../../components/empty-state/check2.md");
    let shell_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_ui_components_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source =
        load_ui_components_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_ui_components_source("../../docs/plan/TODO.md");
    let script_source =
        load_ui_components_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_empty_state_component_source("src/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`EmptyState` 为展示型非交互组件",
        "render_count",
        "等价证据",
        "渲染次数预算为 `1`",
    ] {
        assert!(
            check2_source.contains(needle),
            "EmptyState checklist should keep performance-governance evidence marker `{needle}`.",
        );
    }

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep mount-only perf budget/probe contract `{needle}`.",
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
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable performance marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf regression assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up should keep marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test empty_state_semantics --no-default-features --features component-empty_state,inject-css empty_state_performance_governance_contract_is_mount_only_traceable_and_blocking",
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
        "data-state=move || state.get().state.data_state_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "EmptyState view should expose attribution marker `{needle}` for perf triage.",
        );
    }
}
