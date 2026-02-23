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

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn chart_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/chart/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Chart internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn chart_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/chart/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Chart;"),
        "chart module should export `Chart`."
    );
    assert!(
        module_source.contains("pub use motion::ChartMotion;"),
        "chart module should expose `ChartMotion` from motion layer."
    );
    assert!(
        crate_source.contains("pub use chart::{Chart, ChartKind, ChartMotion, ChartPoint};"),
        "crate root should re-export chart contracts."
    );
}

#[test]
fn chart_feature_depends_on_ui_chart_for_minimal_build() {
    let cargo_toml = load_source("Cargo.toml");
    assert!(
        cargo_toml.contains("component-chart = [\"dep:ui-chart\"]"),
        "component-chart feature should explicitly depend on dep:ui-chart."
    );
}

#[test]
fn chart_state_primitives_are_sourced_from_ui_state_primitives() {
    let primitive_lib = load_source("../ui-state-primitives/src/lib.rs");
    let primitive_chart = load_source("../ui-state-primitives/src/chart.rs");
    let component_logic = load_source("../../components/chart/src/logic.rs");

    assert!(
        primitive_lib.contains("pub mod chart;"),
        "ui-state-primitives should export chart module."
    );

    for needle in [
        "pub enum ChartKind",
        "pub struct ChartPoint",
        "pub struct ChartStateInput",
        "pub struct ChartState",
        "pub fn normalize_points(",
        "pub fn value_domain(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn next_index_for_key(",
    ] {
        assert!(
            primitive_chart.contains(needle),
            "ui-state-primitives chart module should define `{needle}`."
        );
    }

    assert!(
        component_logic.contains("pub use ui_state_primitives::chart::{"),
        "chart component logic should only re-export from ui-state-primitives."
    );
}

#[test]
fn chart_headless_contract_is_exported_and_consumed_by_view() {
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_chart = load_source("../ui-headless/src/chart.rs");
    let view_source = load_source("../../components/chart/src/view.rs");

    assert!(
        headless_lib.contains("pub mod chart;"),
        "ui-headless should export chart module."
    );
    assert!(
        headless_lib.contains("pub use chart::{"),
        "ui-headless should re-export chart headless contracts."
    );

    for needle in [
        "pub struct ChartAttrs",
        "pub enum ChartKeyAction",
        "pub struct ChartHandlers",
        "pub struct ChartContract",
        "pub struct ChartOptions",
        "pub fn use_chart(options: ChartOptions) -> ChartContract",
    ] {
        assert!(
            headless_chart.contains(needle),
            "ui-headless chart module should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "ChartKeyAction",
        "ChartOptions",
        "use_chart",
        "use_controllable_state(",
        "use_chart(ChartOptions {",
        "handlers.on_key_down(",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view should consume headless contract via `{needle}`."
        );
    }
}

#[test]
fn chart_motion_contract_lives_in_chart_motion_rs() {
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
    let view_source = load_source("../../components/chart/src/view.rs");

    assert!(
        mod_source.contains("pub mod motion;"),
        "chart module should expose motion.rs."
    );
    assert!(
        motion_source.contains(
            "pub type ChartMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;"
        ),
        "chart motion should map to active highlight motion contract."
    );

    for needle in ["sanitize_motion(motion)", "attach_motion("] {
        assert!(
            view_source.contains(needle),
            "chart view should call motion contract `{needle}`."
        );
    }
}

#[test]
fn chart_supports_controlled_and_uncontrolled_active_index() {
    let source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "active_index: Option<Signal<usize>>",
        "default_active_index: Option<usize>",
        "on_active_index_change: Option<Callback<usize>>",
        "let is_controlled = active_index.is_some();",
        "use_controllable_state(",
        "Some(default_active_index),",
        "on_active_index_change,",
        "let request_active_index_change = active_state.request_change;",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
    ] {
        assert!(
            source.contains(needle),
            "Chart should support `{needle}` for active-index control flow."
        );
    }
}

#[test]
fn chart_default_active_index_priority_is_readable_and_not_reimplemented_in_view() {
    let source = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");

    assert!(
        source.contains(
            "let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {"
        ) && source.contains("let default_active_index = normalized.default_active_index;"),
        "Chart view should read default-active-index from logic input normalization only."
    );
    assert!(
        logic.contains("pub fn normalize_input_boundary(")
            && logic.contains("default_active_index(point_count, input.default_active_index)"),
        "Chart logic should normalize default_active_index priority in one boundary function."
    );

    for forbidden in [
        "default_active_index.unwrap_or",
        "active_index.unwrap_or",
        "if default_active_index.is_none()",
        "match default_active_index",
    ] {
        assert!(
            !source.contains(forbidden),
            "Chart view must not add local default fallback branch `{forbidden}`."
        );
    }
}

#[test]
fn chart_state_normalization_is_concentrated_in_logic() {
    let source = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let styles = load_source("../../components/chart/src/styles.rs");

    for needle in [
        "pub struct ChartInputBoundary",
        "pub fn normalize_input_boundary(",
        "pub struct ChartStateBoundary",
        "pub fn derive_state_from_boundary(",
        "pub fn normalize_interaction_index(",
    ] {
        assert!(
            logic.contains(needle),
            "Chart logic should expose centralized normalization entry `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_input_boundary(logic::ChartInputBoundary {",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "logic::normalize_interaction_index(index, point_count, is_disabled)",
    ] {
        assert!(
            source.contains(needle),
            "Chart view should consume logic normalization helper `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_points(points).into()",
        "logic::value_domain(points.as_ref())",
        "logic::resolve_state(ChartStateInput {",
    ] {
        assert!(
            !source.contains(forbidden),
            "Chart view should not rebuild state normalization branch `{forbidden}`."
        );
    }

    for forbidden in [
        "normalize_input_boundary(",
        "derive_state_from_boundary(",
        "normalize_interaction_index(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "Chart styles should not own state normalization `{forbidden}`."
        );
    }
}

#[test]
fn chart_discrete_state_axes_use_chart_kind_enum() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let primitive = load_source("../ui-state-primitives/src/chart.rs");

    for needle in [
        "pub enum ChartKind",
        "Bar,",
        "Line,",
        "#[prop(optional)] kind: ChartKind,",
        "state.get().kind == ChartKind::Line",
    ] {
        assert!(
            primitive.contains(needle) || view.contains(needle),
            "chart discrete state should be type-constrained via `{needle}`"
        );
    }

    assert!(
        logic.contains("pub use ui_state_primitives::chart::{") && logic.contains("ChartKind,"),
        "chart logic should consume typed enum axis from state primitives."
    );

    for forbidden in [
        "kind: String",
        "kind: Option<String>",
        "mode: String",
        "status: String",
        "variant: String",
    ] {
        assert!(
            !view.contains(forbidden),
            "chart view should not accept free-form discrete axis `{forbidden}`."
        );
    }
}

#[test]
fn chart_accepts_is_disabled_and_locale_contract_inputs() {
    let source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "is_disabled: bool",
        "is_show_grid: bool",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "is_disabled,",
        "is_show_grid,",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            source.contains(needle),
            "Chart should include normalized disabled/locale contract via `{needle}`."
        );
    }
}

#[test]
fn chart_has_no_async_interaction_contract_and_no_busy_mapping() {
    let source = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");

    for forbidden in [
        "async fn ",
        "Future<",
        "Resource<",
        "use_async_action",
        "spawn_local",
        "spawn(",
        "is_loading",
        "on_retry",
        "error:",
        "aria-busy",
    ] {
        assert!(
            !source.contains(forbidden) && !logic.contains(forbidden),
            "chart should stay N/A for async contract and must not contain `{forbidden}`."
        );
    }
}

#[test]
fn chart_dx_paradox_default_usage_stays_short_and_without_internal_state_wiring() {
    let view = load_source("../../components/chart/src/view.rs");
    let readme = load_source("../../components/chart/src/README.md");
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        view.contains("points: Vec<ChartPoint>,"),
        "chart should keep points as the only required input."
    );

    for forbidden in ["#[prop(optional)] state:", "state: Signal<", "state="] {
        assert!(
            !view.contains(forbidden),
            "chart should not expose internal state wiring `{forbidden}`."
        );
    }

    let hello_world_one_line = "<Chart points=vec![ChartPoint::new(\"jan\", \"Jan\", 12.0), ChartPoint::new(\"feb\", \"Feb\", 18.5), ChartPoint::new(\"mar\", \"Mar\", 17.2)] />";
    assert!(
        readme.contains(hello_world_one_line),
        "README should keep a <=5-line hello-world snippet."
    );
    assert!(
        docs.contains("title=\"Hello World\"") && docs.contains(hello_world_one_line),
        "docs should keep an obvious short hello-world default path."
    );
}

#[test]
fn chart_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
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
fn chart_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "CHART_WORKBENCH_STORAGE_KEY",
        "fn load_chart_workbench_state() -> Option<ChartWorkbenchState>",
        "fn save_chart_workbench_state(state: ChartWorkbenchState)",
        "fn clear_chart_workbench_state()",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "test_css_source=chart_test_css_source",
        "test_config_signal=workbench_config",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "Effect::new(move |_| {",
        "save_chart_workbench_state(ChartWorkbenchState {",
        "clear_chart_workbench_state();",
        "data-slot=\"chart-workbench\"",
        "data-slot=\"chart-workbench-canvas\"",
    ] {
        assert!(
            source.contains(needle),
            "chart workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "chart workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn chart_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn chart_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
    let checklist_source = load_source("../../components/chart/check2.md");

    assert!(
        !manifest_dir
            .join("../../components/chart/src/spec.rs")
            .exists(),
        "chart should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-chart = [\"dep:ui-chart\"]"),
        "chart feature should stay minimal without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-chart = [\"dep:serde\"")
            && !cargo_source.contains("component-chart = [\"dep:serde_json\""),
        "chart should not opt into serde/spec migration dependencies without explicit schema contract."
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
            "chart engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
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
            "chart checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn chart_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("../../components/chart/src/mod.rs"),
        load_source("../../components/chart/src/logic.rs"),
        load_source("../../components/chart/src/view.rs"),
        load_source("../../components/chart/src/styles.rs"),
        load_source("../../components/chart/src/motion.rs"),
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
        !cargo_source.contains("chart-wasm-debug"),
        "chart should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::chart::",
        "const CHART_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn chart_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ] {
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
                "chart engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "chart public module boundary should not leak web_sys types."
    );
}

#[test]
fn chart_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            script_source.contains(required),
            "engineering check script should enforce `{required}`."
        );
    }
}

#[test]
fn chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    let check2_source = load_source("../../components/chart/check2.md");
    let manifest_source = load_source("../../components/chart/Component.toml");
    let rbi_source = load_source("../../components/chart/Component.rbi");
    let protocol_source = load_source("../../components/chart/src/protocol.rs");
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");

    for needle in [
        "pub enum ChartComponentSchemaVersion {",
        "V1,",
        "pub struct ChartComponentSpec {",
        "pub schema_version: ChartComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "chart protocol should keep v1 schema marker `{needle}`.",
        );
    }

    for needle in [
        "schema = \"ui.chart.agent-contract/v1\"",
        "schema_version = \"data-ui-schema-version\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "chart Component.toml should keep v1 contract marker `{needle}`.",
        );
    }

    for needle in [
        "agent_contract_schema \"ui.chart.agent-contract/v1\"",
        "streaming_policy {",
        "fallback: \"snapshot\"",
    ] {
        assert!(
            rbi_source.contains(needle),
            "chart RBI should keep stable contract marker `{needle}`.",
        );
    }

    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
        protocol_source.as_str(),
        manifest_source.as_str(),
        rbi_source.as_str(),
    ]
    .join("\n");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "agent-contract/v2",
        "contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, chart should not introduce migration marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Chart` 变更未引入跨大版本 API 破坏升级",
        "schema = \"ui.chart.agent-contract/v1\"",
        "chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 version-migration section should include `{needle}`.",
        );
    }
}

#[test]
fn chart_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let marker = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`.",
    );
}

#[test]
fn chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "chart non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");

    for forbidden in [
        "common.chart_aria_label.to_string()",
        "String::from(\"Chart\")",
        "\"ui-chart\".to_string()",
        "\"ui-chart--line\".to_string()",
        "\"ui-chart--bar\".to_string()",
        "\"ui-chart--disabled\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "chart string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn chart_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering check script should enforce `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "chart_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "chart_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "chart_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "RUST_HYGIENE_SCOPE=\"components/chart\"",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 rust-hygiene section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_composite_contract_uses_typed_points_and_no_parallel_slots() {
    let view = load_source("../../components/chart/src/view.rs");
    let primitive = load_source("../ui-state-primitives/src/chart.rs");
    let readme = load_source("../../components/chart/src/README.md");

    assert!(
        view.contains("points: Vec<ChartPoint>,"),
        "chart should keep typed points collection as primary item input."
    );
    for required in [
        "pub struct ChartPoint {",
        "pub id: String,",
        "pub label: String,",
        "pub value: f64,",
    ] {
        assert!(
            primitive.contains(required),
            "chart item semantics should stay bound in ChartPoint via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "children: Children",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "chart should not expose parallel array/slot contract `{forbidden}`."
        );
    }
}

#[test]
fn chart_macro_micro_contract_is_na_without_dragging_pipeline() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let motion = load_source("../../components/chart/src/motion.rs");

    for required in [
        "on:pointerenter=on_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view.contains(required),
            "chart should expose non-dragging discrete interaction `{required}`."
        );
    }

    for forbidden in [
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "DragStart",
        "DragEnd",
        "Action::DragEnd",
        "Dragging",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "chart should not include dragging macro/micro pipeline token `{forbidden}`."
        );
    }
}

#[test]
fn chart_two_pass_rendering_contract_stays_na_for_logic_with_visual_measure_guard() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let motion = load_source("../../components/chart/src/motion.rs");
    let visual_driver = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for forbidden in [
        "getBoundingClientRect",
        "offset_top",
        "offset_height",
        "Rect",
        "Measure",
        "Rectification",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "chart logic/view should not own two-pass geometry token `{forbidden}`."
        );
    }

    for required in [
        "attach_active_highlight_motion(",
        "sync_measured_layout(",
        "let unchanged =",
        "if unchanged {",
    ] {
        assert!(
            motion.contains("attach_motion(") && visual_driver.contains(required),
            "visual driver should keep two-pass measurement with idempotent guard `{required}`."
        );
    }
}

#[test]
fn chart_registration_protocol_remains_na_without_dynamic_item_registry() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let headless_chart = load_source("../ui-headless/src/chart.rs");

    for required in [
        "points: Vec<ChartPoint>,",
        "let indices: StoredValue<Vec<usize>> = StoredValue::new((0..point_count).collect());",
    ] {
        assert!(
            view.contains(required),
            "chart should keep deterministic ordering from points vector via `{required}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register(",
        "Unregister(",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not include dynamic item registry token `{forbidden}`."
        );
    }
}

#[test]
fn chart_slot_projection_contract_remains_na_without_render_mode_or_hidden_hooks() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let motion = load_source("../../components/chart/src/motion.rs");

    for required in [
        "data-slot=\"chart\"",
        "data-slot=\"chart-plot\"",
        "data-slot=\"chart-legend\"",
    ] {
        assert!(
            view.contains(required),
            "chart should preserve single-tree slot markers via `{required}`."
        );
    }

    for forbidden in [
        "Children",
        "RenderMode",
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "NotifyVisible",
        "on_hidden",
        "on_visible",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "chart should not expose slot projection lifecycle token `{forbidden}`."
        );
    }
}

#[test]
fn chart_env_stream_contract_stays_na_for_logic_without_raw_env_flood() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let visual_driver = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "ColorSchemeChanged",
        "VisibilityChanged",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "chart logic/view should remain free from env stream token `{forbidden}`."
        );
    }

    for required in ["ResizeObserver", "sync_measured_layout("] {
        assert!(
            visual_driver.contains(required),
            "visual driver should keep isolated env sampling via `{required}`."
        );
    }
}

#[test]
fn chart_event_light_cone_contract_remains_na_without_context_bus_or_bulk_selector() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let headless_chart = load_source("../ui-headless/src/chart.rs");

    for required in [
        "let apply_headless_action = Callback::new(move |action: ChartKeyAction| match action {",
        "ChartKeyAction::MoveTo(next)",
        "ChartKeyAction::Activate(current)",
        "request_active_index_change.run(index);",
    ] {
        assert!(
            view.contains(required),
            "chart should keep single-point action flow `{required}`."
        );
    }

    for forbidden in [
        "ContextBus",
        "SelectorSubscription",
        "SelectionState::All",
        "select_all",
        "bulk_select",
        "broadcast(",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not expose bulk event-light-cone token `{forbidden}`."
        );
    }
}

#[test]
fn chart_causality_bus_contract_remains_na_without_trace_id_or_broadcast_chain() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let headless_chart = load_source("../ui-headless/src/chart.rs");

    for required in [
        "ChartKeyAction::MoveTo(next)",
        "ChartKeyAction::Activate(current)",
        "request_active_index_change.run(index);",
    ] {
        assert!(
            view.contains(required),
            "chart should preserve local causal flow `{required}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "broadcast(",
        "subscribe(",
        "SignalBus",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless_chart.contains(forbidden),
            "chart should not include cross-system causality token `{forbidden}`."
        );
    }
}

#[test]
fn chart_a11y_i18n_locale_contract_is_headless_driven_with_ui_root_i18n_entrypoint() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let headless_chart = load_source("../ui-headless/src/chart.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");
    let headless_i18n_common = load_source("../ui-headless/src/i18n/common.rs");
    let ui_root = load_source("src/root.rs");

    for needle in [
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "resolve_aria_label_with_fallback(",
        "common.chart_aria_label",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "handlers.on_key_down(",
    ] {
        assert!(
            view.contains(needle),
            "chart should keep a11y+i18n+locale contract `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_aria_label_with_fallback(",
        "normalize_aria_label(aria_label.or(i18n_aria_label))",
    ] {
        assert!(
            logic.contains(needle),
            "chart logic should centralize i18n fallback via `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, region_attrs};",
        "let region = region_attrs(options.aria_label, options.lang, options.dir);",
    ] {
        assert!(
            headless_chart.contains(needle),
            "chart headless contract should mount shared a11y mapping `{needle}`."
        );
    }

    for needle in [
        "pub fn region_attrs(",
        "pub fn locale_attrs(",
        "pub struct RegionA11yAttrs",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "chart should depend on shared a11y tool `{needle}`."
        );
    }

    for needle in [
        "pub chart_aria_label: Arc<str>,",
        "chart_aria_label: \"Chart\".into(),",
    ] {
        assert!(
            headless_i18n_common.contains(needle),
            "common i18n should include chart fallback text via `{needle}`."
        );
    }

    for needle in ["#[prop(optional)] i18n: UiI18n,", "provide_ui_i18n(i18n);"] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should keep global i18n injection entry `{needle}`."
        );
    }

    for forbidden in ["\"Quarterly growth line chart\"", "\"last action: \""] {
        assert!(
            !view.contains(forbidden),
            "chart view should not hardcode app/business text `{forbidden}`."
        );
    }
}

#[test]
fn chart_state_markers_are_observable_queryable_and_enumerated() {
    let view = load_source("../../components/chart/src/view.rs");
    let logic = load_source("../../components/chart/src/logic.rs");

    for needle in [
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-disabled=is_disabled.then_some(\"true\")",
        "aria-pressed=move || {",
        "data-index=index",
    ] {
        assert!(
            view.contains(needle),
            "chart should expose stable semantic/query marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ChartActiveValueSource",
        "pub enum ChartInteractionSource",
        "pub const fn as_attr(self) -> &'static str",
        "Self::Default => \"default\"",
        "Self::External => \"external\"",
        "Self::Interaction => \"interaction\"",
        "Self::None => \"none\"",
        "Self::Focus => \"focus\"",
        "Self::Pointer => \"pointer\"",
        "Self::Keyboard => \"keyboard\"",
        "pub const fn initial_active_value_source(is_controlled: bool) -> ChartActiveValueSource",
        "pub const fn interaction_active_value_source(",
    ] {
        assert!(
            logic.contains(needle),
            "chart marker values should stay in closed enum set via `{needle}`."
        );
    }
}

#[test]
fn chart_emits_baseline_state_data_attributes() {
    let source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "data-slot=\"chart\"",
        "data-kind=move || semantics.get().attrs.data_kind",
        "data-state=move || semantics.get().attrs.data_state",
        "data-empty=move || semantics.get().attrs.data_empty",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
    ] {
        assert!(
            source.contains(needle),
            "Chart should expose `{needle}` for stable styling/testing contracts."
        );
    }
}

#[test]
fn chart_styles_include_plot_and_legend_markers() {
    let source = load_source("../../components/chart/src/styles.rs");

    for needle in [
        ".ui-chart {",
        ".ui-chart__plot-wrap",
        ".ui-chart__line",
        ".ui-chart__bar",
        ".ui-chart__dot",
        ".ui-chart__legend-highlight",
        ".ui-chart--line",
        ".ui-chart--disabled",
        ".ui-chart--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "Chart styles should include `{needle}` marker contracts."
        );
    }
}

#[test]
fn chart_styles_consume_ui_theme_variables_without_private_theme_forks() {
    let source = load_source("../../components/chart/src/styles.rs");

    for needle in [
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-bg, var(--ui-fallback-bg-muted))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-radius-md",
        "var(--ui-radius-sm",
    ] {
        assert!(
            source.contains(needle),
            "chart styles should consume ui-theme tokens via `{needle}`."
        );
    }

    for forbidden in [
        "--ui-bg-surface",
        "--ui-bg-canvas",
        "--ui-accent-solid",
        "--ui-accent-emphasis",
        "--ui-border-subtle",
    ] {
        assert!(
            !source.contains(forbidden),
            "chart styles should not depend on undefined/private token `{forbidden}`."
        );
    }
}

#[test]
fn chart_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-group-motion-duration, var(--ui-fallback-checkbox-group-motion-duration))",
        "var(--ui-checkbox-group-motion-easing, var(--ui-fallback-checkbox-group-motion-easing))",
    ] {
        assert!(
            styles_source.contains(needle),
            "chart styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-accent:",
        "--ui-fallback-checkbox-group-motion-duration:",
        "--ui-fallback-checkbox-group-motion-easing:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`."
        );
    }

    for forbidden in ["0.75rem", "0.5rem", "14rem", "160ms", "2px solid", "#"] {
        assert!(
            !styles_source.contains(forbidden),
            "chart styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn chart_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_defensive_variables_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "chart check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "chart_styles_use_defensive_variable_fallback_chain",
        "chart_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "components/chart/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 defensive-variables section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_token_first_styles_are_aggregated_via_ui_root_without_utility_or_css_in_rust_defaults() {
    let styles = load_source("../../components/chart/src/styles.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let css_registry = load_source("src/css.rs");
    let ui_root = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str =",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles.contains(needle),
            "chart styles should stay token-first/static via `{needle}`."
        );
    }

    assert!(
        css_registry.contains("out.push_str(crate::chart::styles::CSS);"),
        "ui css registry should aggregate chart styles through styles.rs."
    );
    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should provide centralized css injection path `{needle}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "tailwind",
        "tw-",
        "style=",
        "styled(",
        "css!(",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "chart component should not adopt utility-first/CSS-in-Rust default token `{forbidden}`."
        );
    }
}

#[test]
fn chart_visual_desire_contract_uses_theme_baseline_page_snapshot_and_heroui_alignment() {
    let docs_pages = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let theme_baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let theme_baseline_e2e = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
    ] {
        assert!(
            docs_pages.contains(needle),
            "docs-app should expose theme visual baseline route via `{needle}`."
        );
    }

    for needle in [
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "title=\"Default Theme Visual Baseline\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            theme_baseline_page.contains(needle),
            "theme visual baseline page should keep visual-quality proof `{needle}`."
        );
    }

    for needle in [
        "const visualMode = process.env.E2E_VISUAL_BASELINE ?? \"off\";",
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "docs-app: theme visual baseline renders button/input/overlay",
        "docs-app: theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_baseline_e2e.contains(needle),
            "visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in [
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "输出统一参数分层规范",
        "不是只能改根节点 class",
        "HeroUI 参数设计风格对齐策略",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "heroui strategy should preserve visual-language alignment boundary `{needle}`."
        );
    }
}

#[test]
fn chart_tree_shaking_contract_keeps_component_feature_gates_and_budget_ci_pipeline() {
    let chart_module = load_source("../../components/chart/src/mod.rs");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");
    let ci_workflow = load_source("../../.github/workflows/ci.yml");

    assert!(
        ui_components_cargo.contains("component-chart = [\"dep:ui-chart\"]"),
        "component-chart should stay as an explicit package-level feature gate."
    );

    let chart_export = ui_components_lib
        .find("pub use ui_chart as chart;")
        .expect("ui lib should export chart module.");
    let chart_export_cfg = ui_components_lib[..chart_export]
        .rfind("#[cfg(feature = \"component-chart\")]")
        .expect("chart export should be cfg-gated in lib.rs.");
    assert!(
        chart_export - chart_export_cfg < 80,
        "chart export should stay immediately behind component-chart cfg gate."
    );

    let chart_css = ui_components_css
        .find("out.push_str(crate::chart::styles::CSS);")
        .expect("css registry should include chart styles aggregation.");
    let chart_css_cfg = ui_components_css[..chart_css]
        .rfind("#[cfg(feature = \"component-chart\")]")
        .expect("chart css aggregation should be cfg-gated in css.rs.");
    assert!(
        chart_css - chart_css_cfg < 80,
        "chart css aggregation should stay behind component-chart cfg gate."
    );

    for needle in [
        "CHART_MIN_FEATURES=\"component-chart,inject-css\"",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CHART_TREE_OUTPUT",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "if ! grep -q 'feature \"component-chart\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$CHART_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking budget script should retain gate `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(needle),
            "tree-shaking budget env should define `{needle}`."
        );
    }

    for needle in [
        "- name: Tree Shaking Budget",
        "run: ./scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            ci_workflow.contains(needle),
            "ci workflow should enforce tree-shaking budget via `{needle}`."
        );
    }

    for forbidden in [
        "ComponentRegistry",
        "register_component(",
        "all_components::",
    ] {
        assert!(
            !chart_module.contains(forbidden),
            "chart source mode should not require central registry token `{forbidden}`."
        );
    }
}

#[test]
fn chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let chart_module = load_source("../../components/chart/src/mod.rs");

    assert!(
        ui_components_cargo.contains("component-chart = [\"dep:ui-chart\"]"),
        "component-chart should stay as an explicit package-level feature gate."
    );

    let lib_lines: Vec<&str> = ui_components_lib.lines().collect();
    assert!(
        lib_lines.windows(2).any(|window| {
            window[0].trim() == "#[cfg(feature = \"component-chart\")]"
                && window[1].trim() == "pub use ui_chart as chart;"
        }),
        "chart export should stay cfg-gated in ui lib.rs.",
    );

    let css_lines: Vec<&str> = ui_components_css.lines().collect();
    assert!(
        css_lines.windows(2).any(|window| {
            window[0].trim() == "#[cfg(feature = \"component-chart\")]"
                && window[1].trim() == "out.push_str(crate::chart::styles::CSS);"
        }),
        "chart css aggregation should stay cfg-gated in ui css.rs.",
    );

    for forbidden in [
        "ComponentRegistry",
        "register_component(",
        "all_components::",
    ] {
        assert!(
            !chart_module.contains(forbidden),
            "chart source mode should not require central registry token `{forbidden}`.",
        );
    }
}

#[test]
fn chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "CHART_MIN_FEATURES=\"component-chart,inject-css\"",
        "chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CHART_TREE_OUTPUT",
        "if ! grep -q 'feature \"component-chart\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$CHART_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$CHART_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$CHART_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should enforce `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(needle),
            "tree-shaking budget env should define `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-chart = [\"dep:ui-chart\"]",
        "#[cfg(feature = \"component-chart\")]",
        "pub use ui_chart as chart;",
        "out.push_str(crate::chart::styles::CSS);",
        "chart_tree_shaking_contract_keeps_component_feature_gates_and_budget_ci_pipeline",
        "chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_locally",
        "chart_check2_marks_tree_shaking_feature_pruning_contract_complete_locally",
        "components/chart/test/chart/semantics.rs::chart_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "components/chart/test/chart/semantics.rs::chart_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "components/chart/test/chart/semantics.rs::chart_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-chart,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_type_system_and_semantic_markers_form_machine_readable_contract_feedback_loop() {
    let logic = load_source("../../components/chart/src/logic.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let local_semantics = load_source("../../components/chart/test/semantics.rs");
    let repo_semantics = load_source("tests/chart/semantics.rs");

    for needle in [
        "pub enum ChartKind",
        "pub enum ChartActiveValueSource",
        "pub enum ChartInteractionSource",
        "pub fn normalize_input_boundary(",
        "pub fn derive_state_from_boundary(",
        "pub fn normalize_interaction_index(",
    ] {
        assert!(
            logic.contains(needle),
            "type-safe and normalized logic contract should contain `{needle}`."
        );
    }

    for forbidden in ["match kind.as_str()", "from_str(", ".parse::<ChartKind>()"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "chart should avoid string protocol for discrete state token `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] kind: ChartKind,",
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(needle),
            "machine-readable semantic marker contract should expose `{needle}`."
        );
    }

    for needle in [
        "fn chart_discrete_state_axes_are_type_constrained()",
        "fn chart_state_markers_are_observable_queryable_and_enumerated()",
    ] {
        assert!(
            local_semantics.contains(needle) && repo_semantics.contains(needle),
            "feedback loop should keep mirrored semantic regressions via `{needle}`."
        );
    }
}

#[test]
fn chart_focus_stack_overlay_contract_is_na_for_non_overlay_component() {
    let logic = load_source("../../components/chart/src/logic.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");

    for needle in [
        "let legend_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "attach_motion(ChartMotionAttach {",
    ] {
        assert!(
            view.contains(needle) || motion.contains(needle),
            "chart node refs should remain scoped to visual highlight path `{needle}`."
        );
    }

    for forbidden in [
        "<Overlay",
        "role=\"dialog\"",
        "FallbackTo",
        "Selector",
        "FocusStack",
        "restore_focus",
        "focus_restore",
        "document.body",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "chart should not carry overlay focus-stack token `{forbidden}`."
        );
    }
}

#[test]
fn chart_foreign_zone_escape_hatch_contract_is_na_without_imperative_third_party_runtime() {
    let module = load_source("../../components/chart/src/mod.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "leaflet",
        "google.maps",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "imperative_runtime",
        "extern \"C\"",
        "wasm_bindgen",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden),
            "chart should not contain foreign-zone integration token `{forbidden}`."
        );
    }

    for needle in [
        "pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};",
        "pub use motion::ChartMotion;",
        "pub use view::Chart;",
    ] {
        assert!(
            module.contains(needle),
            "chart public api should stay pure and not expose third-party runtime via `{needle}`."
        );
    }
}

#[test]
fn chart_styles_depend_on_explicit_state_markers_without_fragile_dom_or_inline_business_style() {
    let styles = load_source("../../components/chart/src/styles.rs");
    let view = load_source("../../components/chart/src/view.rs");

    for needle in [
        ".ui-chart__bar[data-active=\"true\"]",
        ".ui-chart__dot[data-active=\"true\"]",
        ".ui-chart[data-disabled=\"true\"]",
        ".ui-chart[data-empty=\"true\"]",
        ".ui-chart[data-custom-class=\"true\"]",
        ".ui-chart--disabled",
        ".ui-chart--empty",
        ".ui-chart--custom-class",
    ] {
        assert!(
            styles.contains(needle),
            "chart styles should branch on explicit semantic marker `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":first-child",
        ":last-child",
        ":only-child",
        ":has(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "chart styles should not rely on fragile dom selector `{forbidden}`."
        );
    }

    assert!(
        !view.contains("style="),
        "chart view should not push business styling through inline style attributes."
    );
}

#[test]
fn chart_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-chart\")]",
        "out.push_str(crate::chart::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`."
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

    for forbidden in [
        " style=",
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart view should not include plain inline style token `{forbidden}`."
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
                "chart runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }
}

#[test]
fn chart_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "chart check2 should mark cascade-layer gate complete."
    );

    for needle in [
        "chart_cascade_layer_and_runtime_style_contract_is_enforced",
        "chart_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "components/chart/src/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 cascade-layer section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()
 {
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");
    let visual_driver = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let component_semantics = load_source("../../components/chart/test/semantics.rs");

    for needle in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "on:keydown=on_key_down",
        "on:pointerenter=on_enter",
        "on:click=on_click",
    ] {
        assert!(
            view.contains(needle),
            "chart semantic matrix should include branch marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "chart motion should keep wasm/ssr split via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(needle),
            "visual driver should keep platform split contract `{needle}`."
        );
    }

    let forbidden_tokens = vec![
        ["assert", "snapshot"].join("_"),
        "insta::".to_string(),
        "to_match_image_snapshot".to_string(),
    ];
    for token in forbidden_tokens {
        assert!(
            !component_semantics.contains(&token),
            "chart semantic contract tests should not rely on visual snapshot token `{token}`."
        );
    }
}

#[test]
fn chart_component_file_responsibilities_stay_layered_and_non_overlapping() {
    let module = load_source("../../components/chart/src/mod.rs");
    let logic = load_source("../../components/chart/src/logic.rs");
    let styles = load_source("../../components/chart/src/styles.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ChartKind, ChartPoint, DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE};",
        "pub use motion::ChartMotion;",
        "pub use view::Chart;",
    ] {
        assert!(
            module.contains(needle),
            "mod.rs should keep minimal export boundary `{needle}`."
        );
    }

    for forbidden in ["view!", "NodeRef", "on:click", "on:keydown", ".ui-chart__"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain view/dom/style detail `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str =", "var(--ui-"] {
        assert!(
            styles.contains(needle),
            "styles.rs should remain token-first static css via `{needle}`."
        );
    }
    for forbidden in ["on:click", "on:keydown", "view!", "use_chart("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not include interaction/render logic `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_input_boundary(",
        "logic::derive_state_from_boundary(",
        "use_chart(ChartOptions {",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should assemble logic/headless/motion via `{needle}`."
        );
    }
    for forbidden in [
        "pub struct ChartStateInput",
        "pub struct ChartState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not redefine primitive state type `{forbidden}`."
        );
    }

    for needle in [
        "pub type ChartMotion =",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::sanitize_config",
    ] {
        assert!(
            motion.contains(needle),
            "motion.rs should map semantic state to shared motion contract `{needle}`."
        );
    }
    for forbidden in [
        "role=",
        "aria-",
        "on:click",
        "on:keydown",
        "SpringAnimator::new(",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not own a11y/events/engine internals `{forbidden}`."
        );
    }

    for required in [
        "../../components/chart/src/mod.rs",
        "../../components/chart/src/logic.rs",
        "../../components/chart/src/styles.rs",
        "../../components/chart/src/view.rs",
        "../../components/chart/src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "chart component directory should contain required file `{required}`."
        );
    }

    for forbidden in [
        "../../components/chart/src/render.rs",
        "../../components/chart/src/spec.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "chart component directory should keep `{forbidden}` absent for this checklist scope."
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_responsibility_contract() {
    let source = load_source("../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_component_file_responsibilities_stay_layered_and_non_overlapping";
    assert!(
        source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_component_file_responsibility_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] 组件目录标准文件落点正确。"),
        "chart check2 should mark component-file-responsibility gate complete."
    );

    for needle in [
        "chart_component_file_responsibilities_stay_layered_and_non_overlapping",
        "chart_component_files_check_script_covers_responsibility_contract",
        "scripts/check-ui-component-files.sh",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 component-files section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_file_placement_discipline_is_strict_for_struct_first_scope() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/chart/src");
    let mut files: Vec<String> = fs::read_dir(&src_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {src_dir:?}: {e}"))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("dir entry read failed: {e}"));
            entry.file_name().to_string_lossy().into_owned()
        })
        .collect();
    files.sort();

    assert_eq!(
        files,
        vec![
            "logic.rs",
            "mod.rs",
            "motion.rs",
            "protocol.rs",
            "styles.rs",
            "view.rs",
        ],
        "chart src should keep strict file-placement discipline and only include layered core files plus protocol.rs."
    );

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !files.iter().any(|name| name == forbidden),
            "chart src should keep `{forbidden}` absent unless checklist scope changes."
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_file_placement_discipline() {
    let source = load_source("../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_file_placement_discipline_is_strict_for_struct_first_scope";
    assert!(
        source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_file_placement_discipline_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "chart check2 should mark file-placement-discipline gate complete."
    );

    for needle in [
        "chart_file_placement_discipline_is_strict_for_struct_first_scope",
        "chart_component_files_check_script_covers_file_placement_discipline",
        "scripts/check-ui-component-files.sh",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
        "components/chart/src/protocol.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 file-placement section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_docs_page_exists_in_display_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Chart\"",
        "slug=\"chart\"",
        "<Chart",
    ] {
        assert!(
            docs.contains(needle),
            "Chart docs page should contain `{needle}`."
        );
    }
}

#[test]
fn chart_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Chart\"",
        "slug=\"chart\"",
        "title=\"Hello World\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "title=\"Bar + Hover/Keyboard + Action\"",
        "title=\"Controlled Line + Active Index\"",
    ] {
        assert!(
            source.contains(needle),
            "chart docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn chart_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_css_source=chart_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/chart/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "code_signal=matrix_code",
        "id_base=\"docs-chart-matrix-bar\".to_string()",
        "id_base=\"docs-chart-matrix-line\".to_string()",
        "id_base=\"docs-chart-matrix-disabled\".to_string()",
        "id_base=\"docs-chart-matrix-empty\".to_string()",
        "is_disabled=true points=vec![...]",
        "<Playground title=\"Bar + Hover/Keyboard + Action\" code_signal=bar_code>",
        "id_base=\"docs-chart-bar\".to_string()",
        "kind=ChartKind::Bar",
        "on_action=on_action",
        "\"last action: \"",
        "<Playground title=\"Controlled Line + Active Index\" code_signal=line_code>",
        "id_base=\"docs-chart-line\".to_string()",
        "kind=ChartKind::Line",
        "active_index=controlled_active",
        "on_active_index_change=on_controlled_active_change",
        "is_show_grid=is_show_grid",
        "aria_label=\"Quarterly growth line chart\".to_string()",
        "class_name=\"docs-chart-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "chart docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let chart_imports =",
        "use ui::{Chart, ChartKind, ChartPoint};",
        "code_imports=chart_imports.clone()",
        "data-slot=\"chart-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
    ] {
        assert!(
            source.contains(needle),
            "chart docs-product copy-paste-ready contract should contain `{needle}`.",
        );
    }
}

#[test]
fn chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"chart-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"chart-copy-ready-hint\"",
        "data-slot=\"chart-source-paths\"",
        "component-chart",
        "inject-css",
        "UiRoot",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/view.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce chart docs-product guard `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_docs_product_copy_paste_ready_rules() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World / Comparison Matrix / Controlled vs Uncontrolled Contrast / Streaming / Snapshot Contract / Source-first Starter (Copy-Paste Ready)",
        "compose_copy_ready_code",
        "component-chart",
        "chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "chart_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "chart_check2_documents_docs_product_copy_paste_ready_rules",
        "chart_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "chart_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "chart_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "components/chart/test/semantics.rs::chart_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 should keep docs-product completion marker `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "chart checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/chart.rs");

    chart_docs_page_covers_primary_playgrounds();
    chart_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Comparison Matrix (Bar / Line / Disabled / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "data-slot=\"chart-parameter-matrix\"",
        "Parameter Matrix (API Names + Defaults)",
        "data-slot=\"chart-state-matrix-summary\"",
        "Size/variant: N/A for Chart",
        "kind=ChartKind::Line",
        "active_index=controlled_active.clone()",
        "on_active_index_change=on_controlled_active_change.clone()",
        "is_disabled=true",
        "id_base=\"docs-chart-matrix-disabled\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart docs examples should keep state-matrix/API sync marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] kind: ChartKind,",
        "#[prop(optional)] default_active_index: Option<usize>,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional, default = true)] is_show_grid: bool,",
        "#[prop(optional, into)] id_base: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view public API should keep `{needle}` for docs/runtime sync.",
        );
    }

    for needle in [
        "pub fn normalize_input_boundary(input: ChartInputBoundary) -> ChartNormalizedInput",
        "let default_active_index = default_active_index(point_count, input.default_active_index);",
        "pub const DEFAULT_ID_BASE: &str = \"ui-chart\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Chart\";",
        "pub fn default_active_index(point_count: usize, requested: Option<usize>) -> usize",
        "requested.unwrap_or(0)",
    ] {
        assert!(
            logic_source.contains(needle) || primitive_source.contains(needle),
            "chart logic defaults should keep `{needle}` for docs consistency.",
        );
    }

    for forbidden in [
        "default_kind=",
        "on_active_change=",
        "is_open=",
        "default_is_open",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "chart docs should avoid stale/aliased API token `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_docs_sync_and_state_matrix_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce chart docs-sync/state-matrix command `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_docs_sync_and_state_matrix_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "chart_check2_documents_docs_sync_and_state_matrix_rules",
        "chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "chart_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "components/chart/test/semantics.rs::chart_check2_documents_docs_sync_and_state_matrix_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_docs_sync_and_state_matrix_contract_complete_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 docs-sync/state-matrix section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "chart checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn chart_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/chart/src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "# Chart",
        "## Hello World（最小可用）",
        "<Chart points=vec![ChartPoint::new(\"jan\", \"Jan\", 12.0), ChartPoint::new(\"feb\", \"Feb\", 18.5), ChartPoint::new(\"mar\", \"Mar\", 17.2)] />",
        "先传 `points` 即可运行，后续再按需开启受控、动作、动效等高级参数。",
        "## 常见用法",
        "## 再进阶（受控 + 语义 + 动效）",
        "apps/docs-app/src/pages/components/pages/display_extra.rs",
    ] {
        assert!(
            readme_source.contains(needle),
            "chart README should keep beginner-first documentation marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn chart() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Controlled vs Uncontrolled Contrast\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart docs-app entry should include `{needle}`.",
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("chart docs should include Hello World playground");
    let controlled_pos = docs_source
        .find("title=\"Controlled vs Uncontrolled Contrast\"")
        .expect("chart docs should include controlled/uncontrolled playground");
    assert!(
        hello_pos < controlled_pos,
        "chart docs should keep beginner Hello World path before advanced controlled path",
    );

    for forbidden in [
        "必须先理解 ui-state-primitives",
        "必须先理解 ui-headless",
        "Only source code is provided",
    ] {
        assert!(
            !readme_source.contains(forbidden) && !docs_source.contains(forbidden),
            "chart documentation should avoid architecture-first barrier `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_documentation_entry_exists_with_beginner_first_progression",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_documentation_as_product_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce chart documentation-as-product command `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_documentation_as_product_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "chart_check2_documents_documentation_as_product_rules",
        "chart_documentation_entry_exists_with_beginner_first_progression",
        "chart_dx_check_script_covers_documentation_as_product_contract",
        "components/chart/test/semantics.rs::chart_check2_documents_documentation_as_product_rules_locally",
        "components/chart/test/semantics.rs::chart_documentation_entry_exists_with_beginner_first_progression_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_documentation_as_product_contract_complete_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 documentation-as-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "chart checklist should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn chart_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "id_base=\"docs-chart-kind\".to_string()",
        "id_base=\"docs-chart-dataset\".to_string()",
        "<Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>",
        "<Switch checked=workbench_is_show_grid set_checked=set_workbench_is_show_grid>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_lang set_checked=set_workbench_lang>",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "data-slot=\"chart-workbench\"",
        "data-slot=\"chart-workbench-canvas\"",
        "\"workbench last action: \"",
        "test_config_signal=workbench_config",
        "test_css_source=chart_test_css_source",
        "code_signal=workbench_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn chart_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_chart_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "docs-app chart key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "[data-slot=\"chart-workbench-canvas\"] [data-slot=\"chart\"]",
        "chart-workbench-toggle-disabled",
        "await page.keyboard.press(\"ArrowRight\");",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "chart interactive playground e2e flow should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"chart-workbench-canvas\"",
        "data-slot=\"chart-workbench-toggle-disabled\"",
        "data-slot=\"chart-e2e-controlled-line\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart docs should expose interactive/e2e semantic anchor `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "chart interactive e2e flow should avoid brittle wait `{forbidden}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce chart interactive-playground command `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_interactive_playground_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "chart_check2_documents_interactive_playground_rules",
        "chart_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "chart_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "chart_dx_check_script_covers_interactive_playground_contract",
        "components/chart/test/semantics.rs::chart_check2_documents_interactive_playground_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_app_provides_interactive_playground_for_props_state_and_preview_locally",
        "components/chart/test/semantics.rs::chart_interactive_playground_reuses_repeatable_semantic_e2e_flow_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_interactive_playground_contract_complete_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 interactive-playground section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "chart checklist should keep source-first rule `{required}`.",
        );
    }
}

#[test]
fn chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"chart-source-first\"",
        "data-slot=\"chart-copy-ready-hint\"",
        "data-slot=\"chart-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "component-chart",
        "inject-css",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/view.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "chart source-first docs should include `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should include `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep copy button marker `{needle}`.",
        );
    }
}

#[test]
fn chart_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_source_first_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce chart source-first command `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "chart_check2_documents_source_first_copy_paste_ready_rules",
        "chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "chart_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "components/chart/test/semantics.rs::chart_check2_documents_source_first_copy_paste_ready_rules_locally",
        "components/chart/test/semantics.rs::chart_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_source_first_copy_paste_ready_contract_complete_locally",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 source-first section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_heroui_benchmark_docs_sync_rules() {
    let checklist_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "chart checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("../../components/chart/src/README.md");

    for needle in [
        "### Chart 同步记录（2026-02-20）",
        "参数模型同步：`Chart` 维持 display data-visual primitive 定位",
        "component_doc!(\"Chart\", \"chart\", \"Display\", display_extra::chart)",
        "#/components/chart",
        "`components/chart/src/README.md` 提供等价组件文档入口",
        "display_extra.rs::chart()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include chart synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Chart\"",
        "\"chart\"",
        "display_extra::chart",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose chart entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn chart() -> AnyView {",
        "title=\"Chart\"",
        "slug=\"chart\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app chart page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# Chart"),
        "chart README should remain an equivalent component doc entry.",
    );
}

#[test]
fn chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "chart_check2_documents_heroui_benchmark_docs_sync_rules",
        "chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/chart/test/semantics.rs::chart_check2_documents_heroui_benchmark_docs_sync_rules_locally",
        "components/chart/test/semantics.rs::chart_heroui_strategy_and_component_docs_are_synchronized_and_indexable_locally",
        "components/chart/test/semantics.rs::chart_dx_check_script_covers_heroui_benchmark_docs_sync_contract_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_heroui_benchmark_docs_sync_contract_complete_locally",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "chart check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn chart_readme_exists_and_is_copy_paste_ready() {
    let source = load_source("../../components/chart/src/README.md");

    for needle in [
        "# Chart",
        "## 展示区（Display）",
        "## Config 展示区",
        "## Code 展示区",
        "## CSS Test 展示区",
        "## Hello World（最小可用）",
        "<Chart",
        "ChartPoint::new",
        "components/chart/src/view.rs",
        "apps/docs-app/src/pages/components/pages/display_extra.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart README should contain `{needle}`.",
        );
    }
}

#[test]
fn chart_hydration_discontinuity_uses_deterministic_id_provider_seed_for_generated_ids() {
    let logic = load_source("../../components/chart/src/logic.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let root = load_source("src/root.rs");
    let id_provider = load_source("../ui-headless/src/id_provider.rs");

    for needle in [
        "use_ui_id_provider,",
        "let generated_id_base = use_ui_id_provider()",
        "next_prefixed_id(logic::DEFAULT_ID_BASE)",
        "let id_base = logic::resolve_id_base(id_base, generated_id_base);",
        "id_base: Some(id_base),",
        "pub fn resolve_id_base(id_base: Option<String>, generated_id_base: String) -> String",
        "normalize_optional_text(id_base).unwrap_or(generated_id_base)",
    ] {
        assert!(
            view.contains(needle) || logic.contains(needle),
            "Chart SSR/hydration deterministic id contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "now()",
        "rand::",
        "thread_rng",
        "Uuid::new_v4",
        "uuid::Uuid",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "Chart should not include hydration-unstable entropy source `{forbidden}`.",
        );
    }

    assert!(
        root.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should wire deterministic id seed via provide_ui_id_provider(id_seed).",
    );

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            id_provider.contains(needle),
            "ui-headless id provider contract should expose `{needle}`.",
        );
    }
}

#[test]
fn chart_platform_contract_covers_default_ssr_wasm_compile_paths_and_non_wasm_source_guards() {
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "cargo check -p ui",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --no-default-features --features component-chart,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-chart,inject-css",
        "components/chart/src/mod.rs",
        "components/chart/src/logic.rs",
        "components/chart/src/styles.rs",
        "components/chart/src/view.rs",
        "components/chart/src/motion.rs",
        "chart motion must keep explicit wasm/non-wasm branches",
        "'#[cfg(target_arch = \"wasm32\")]' components/chart/src/motion.rs",
        "'#[cfg(not(target_arch = \"wasm32\"))]' components/chart/src/motion.rs",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform gate script should include chart compile/source guard `{needle}`.",
        );
    }

    for forbidden in ["web_sys", "web-sys"] {
        assert!(
            !view.contains(forbidden) && !motion.contains(forbidden),
            "Chart non-wasm path should not depend on browser API token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion.contains(needle),
            "Chart motion should keep explicit platform cfg branch `{needle}`.",
        );
    }
}

#[test]
fn chart_ui_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let view = load_source("../../components/chart/src/view.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless feature mutex should be guarded via `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\"",
        "cargo check -p ui --no-default-features --features component-chart,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-chart,inject-css",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform gate should enforce ui-headless mutex and chart compile paths via `{needle}`.",
        );
    }

    assert!(
        view.contains("use ui_headless::{"),
        "chart should keep consuming ui-headless contract under mutex discipline.",
    );
}

#[test]
fn chart_ui_motion_non_wasm_stub_contract_keeps_ssr_and_tooling_paths_compilable() {
    let chart_motion = load_source("../../components/chart/src/motion.rs");
    let ui_motion_lib = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_stub_tests = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion non-wasm no-op contract should contain `{needle}`.",
        );
    }

    for needle in [
        "non_wasm_web_backend_prefers_reduced_motion",
        "non_wasm_web_backend_animate_is_safe_noop",
        "#![cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            ui_motion_non_wasm_stub_tests.contains(needle),
            "ui-motion non-wasm regression suite should contain `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            chart_motion.contains(needle),
            "Chart motion should keep predictable non-wasm downgrade via `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform gate should enforce ui-motion non-wasm/wasm compile-test path `{needle}`.",
        );
    }
}

#[test]
fn chart_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view = load_source("../../components/chart/src/view.rs");
    let motion = load_source("../../components/chart/src/motion.rs");
    let visual_driver = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let ui_motion_spring = load_source("../ui-motion/src/spring.rs");
    let ui_motion_web = load_source("../ui-motion/src/web.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "spring runtime should keep reduced-motion immediate-settle contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool {",
        "if prefers_reduced_motion() {",
    ] {
        assert!(
            ui_motion_web.contains(needle),
            "wasm web backend should branch on reduced-motion via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "Chart motion should keep explicit ssr/wasm split and safe downgrade `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(needle),
            "active-highlight driver should keep ssr/wasm split `{needle}`.",
        );
    }

    for needle in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-disabled=move || semantics.get().attrs.data_disabled",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-uncontrolled=move || semantics.get().attrs.data_uncontrolled",
    ] {
        assert!(
            view.contains(needle),
            "chart semantic markers should remain stable across ssr/wasm branches `{needle}`.",
        );
    }
    assert!(
        !view.contains("cfg(target_arch"),
        "chart view should avoid splitting semantic output by target_arch to keep hydration consistency.",
    );

    assert!(
        platform_script
            .contains("chart_reduced_motion_ssr_wasm_branches_keep_semantics_consistent"),
        "platform script should execute chart reduced-motion/ssr/wasm regression gate.",
    );
}

#[test]
fn chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion = load_source("../../components/chart/src/motion.rs");
    let motion_unit_test_source = load_source("../../components/chart/test/motion.rs");
    let view = load_source("../../components/chart/src/view.rs");
    let ui_motion_spring = load_source("../ui-motion/src/spring.rs");
    let ui_motion_lib = load_source("../ui-motion/src/lib.rs");
    let visual_driver = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let check2_source = load_source("../../components/chart/check2.md");

    for needle in [
        "pub type ChartMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn sanitize_motion(motion: ChartMotion) -> ChartMotion {",
        "ui_motion::spring::sanitize_config(",
        "ui_motion::presets::spring_slide()",
        "pub fn attach_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "chart motion contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "fn sanitize_motion_preserves_contract()",
        "fn sanitize_motion_falls_back_for_invalid_spring_values()",
        "stiffness: f64::NAN,",
        "damping: -1.0,",
    ] {
        assert!(
            motion_unit_test_source.contains(needle),
            "chart motion unit tests should keep `{needle}`.",
        );
    }

    for needle in [
        "let motion = motion::sanitize_motion(motion);",
        "motion::attach_motion(",
    ] {
        assert!(
            view.contains(needle),
            "chart view should sanitize and attach motion contract via `{needle}`.",
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should keep reduced-motion immediate-settle branch `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion non-wasm no-op contract should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            visual_driver.contains(needle),
            "active-highlight shared driver should keep platform split marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart checklist should keep motion contractualization evidence `{needle}`.",
        );
    }
}

#[test]
fn chart_motion_contract_platform_script_covers_guard() {
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platform_script.contains(needle),
        "platform check script should enforce `{needle}`.",
    );
}

#[test]
fn chart_check2_marks_motion_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "chart_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "chart_motion_contract_platform_script_covers_guard",
        "scripts/check-ui-platforms.sh",
        "components/chart/src/motion.rs",
        "components/chart/src/view.rs",
        "crates/ui-motion/src/spring.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 motion section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../ui-headless/src/presence.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "#[cfg(feature = \"component-chart\")]",
        "pub use ui_chart as chart;",
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

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-chart\")]",
        "out.push_str(crate::chart::styles::CSS);",
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

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
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
        "ComboBox",
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
            "ui forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
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
fn chart_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] `ui` 固定入口文件落点正确。"),
        "chart check2 should mark fixed-entry-files gate complete."
    );

    for needle in [
        "chart_ui_components_fixed_entry_files_follow_layered_boundaries",
        "chart_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "scripts/check-ui-entrypoints.sh",
        "crates/ui/src/lib.rs",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 fixed-entry-files section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/chart/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "\"chart\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "chart docs shell should keep explicit performance budget `{needle}`.",
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
            "UiPerfProbe should expose budget/violation observability marker `{needle}`.",
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
            "docs coverage e2e should include blocking perf assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance follow-up plan should keep `{needle}`.",
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
            "chart check2 should keep performance governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );

    for needle in [
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "motion::attach_motion(",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view should expose attribution marker `{needle}` for perf triage.",
        );
    }
}

#[test]
fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let local_semantics = load_source("../../components/chart/test/semantics.rs");
    let aggregated_semantics = load_source("tests/chart/semantics.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "chart local semantics/performance suite should include `{required_test}`.",
        );
    }

    for required_test in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "fn chart_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            aggregated_semantics.contains(required_test),
            "chart aggregated semantics/performance suite should include `{required_test}`.",
        );
    }

    for marker in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "on:focus=on_focus",
        "on:pointerenter=on_pointer_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
        "Self::Focus => \"focus\"",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "chart semantic/focus-flow regression should include marker `{marker}`.",
        );
    }

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
fn chart_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`.",
        );
    }
}

#[test]
fn chart_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance",
        "chart_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally",
        "components/chart/test/chart/semantics.rs::chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "chart check2 semantic/performance section should include `{marker}`.",
        );
    }
}

#[test]
fn chart_semantics_tests_prioritize_data_aria_role_and_state_source_over_visual_snapshot() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let local_semantics = load_source("../../components/chart/test/semantics.rs");
    let aggregated_semantics = load_source("tests/chart/semantics.rs");

    for marker in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-controlled=move || semantics.get().attrs.data_controlled",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "on:focus=on_focus",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(marker),
            "chart semantic-first contract should expose `{marker}`.",
        );
    }

    for marker in [
        "fn chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance()",
        "chart semantic contract tests should not rely on visual snapshot token",
    ] {
        assert!(
            local_semantics.contains(marker) && aggregated_semantics.contains(marker),
            "chart local/aggregated semantic suites should keep `{marker}`.",
        );
    }
}

#[test]
fn chart_semantics_priority_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_semantics_tests_priority_rules",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include semantic-first command `{marker}`.",
        );
    }
}

#[test]
fn chart_check2_documents_semantics_tests_priority_rules() {
    let source = load_source("../../components/chart/check2.md");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "chart_semantic_contract_matrix_covers_state_interaction_and_platform_paths_without_snapshot_reliance",
        "chart_semantics_tests_prioritize_data_aria_role_and_state_source_over_visual_snapshot",
        "components/chart/test/semantics.rs::chart_semantics_priority_contract_is_documented_and_scripted_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_semantics_tests_priority_rules",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(marker),
            "chart check2 semantic-first section should reference `{marker}`.",
        );
    }
}

#[test]
fn chart_check2_marks_semantics_tests_priority_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "chart_semantics_tests_prioritize_data_aria_role_and_state_source_over_visual_snapshot",
        "chart_semantics_priority_script_covers_contract",
        "chart_check2_documents_semantics_tests_priority_rules",
        "components/chart/test/semantics.rs::chart_semantics_priority_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            source.contains(marker),
            "chart check2 should keep semantic-first completion marker `{marker}`.",
        );
    }
}

#[test]
fn chart_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 e2e-selector section should include `{required}`.",
        );
    }
}

#[test]
fn chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_contract_source = load_source("../../e2e/tests/docs_app_chart_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for required in [
        "docs-app chart uses semantic selectors with wasm-stable ready waits",
        "docs-app chart interaction uses semantic ready and settled breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"chart-e2e-controlled-line\"] [data-slot=\"chart\"]",
        "[data-slot=\"chart-e2e-state-disabled\"] [data-slot=\"chart\"]",
        "[data-slot=\"chart-workbench-canvas\"]",
        "[data-slot=\"chart-workbench-toggle-lang\"] [data-slot=\"switch\"]",
    ] {
        assert!(
            e2e_contract_source.contains(required),
            "chart e2e selector contract should include marker `{required}`.",
        );
    }

    for required in [
        "data-slot=\"chart-e2e-controlled-line\"",
        "data-slot=\"chart-e2e-state-disabled\"",
        "data-slot=\"chart-workbench-canvas\"",
        "data-slot=\"chart-workbench-toggle-lang\"",
    ] {
        assert!(
            docs_source.contains(required),
            "chart docs should expose stable semantic selector anchor `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
    ] {
        assert!(
            !e2e_contract_source.contains(forbidden),
            "chart e2e selector contract should avoid fragile wait/selector `{forbidden}`.",
        );
    }
}

#[test]
fn chart_e2e_contract_covers_ready_and_settled_conditions_for_chart_interaction() {
    let e2e_contract_source = load_source("../../e2e/tests/docs_app_chart_contract.spec.mjs");

    for required in [
        "toHaveAttribute(\"data-state\", \"ready\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"pointer\")",
        "toHaveAttribute(\"data-ui-state\", \"ready\")",
        "await langSwitch.click();",
    ] {
        assert!(
            e2e_contract_source.contains(required),
            "chart e2e contract should keep ready/settled semantic breakpoint `{required}`.",
        );
    }
}

#[test]
fn chart_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../components/chart/scripts/check-ui-e2e-chart.sh");

    for required in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_contract_covers_ready_and_settled_conditions_for_chart_interaction",
    ] {
        assert!(
            script_source.contains(required),
            "chart e2e check script should include `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/chart/check2.md");
    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "chart check2 should mark e2e selector stability item complete.",
    );

    for required in [
        "components/chart/test/semantics.rs::chart_check2_documents_e2e_selector_and_stable_wait_rules_locally",
        "components/chart/test/semantics.rs::chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/chart/test/chart/semantics.rs::chart_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "e2e/tests/docs_app_chart_contract.spec.mjs",
        "components/chart/scripts/check-ui-e2e-chart.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 e2e selector stability section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/chart/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 repeatable-key-flow section should include `{required}`.",
        );
    }
}

#[test]
fn chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_contract = load_source("../../e2e/tests/docs_app_chart_contract.spec.mjs");

    for required in [
        "docs-app chart key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "chart-workbench-toggle-disabled",
        "toHaveAttribute(\"data-active-index\", \"2\")",
        "toHaveAttribute(\"data-active-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-disabled\", \"false\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "await page.keyboard.press(\"ArrowRight\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "chart repeatable key-flow contract should include marker `{required}`.",
        );
    }
}

#[test]
fn chart_e2e_check_script_covers_repeatable_key_flow_contract() {
    let script_source = load_source("../../components/chart/scripts/check-ui-e2e-chart.sh");

    for required in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(required),
            "chart e2e check script should include repeatable-key-flow marker `{required}`.",
        );
    }
}

#[test]
fn chart_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2_source = load_source("../../components/chart/check2.md");
    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "chart check2 should mark repeatable e2e key-flow item complete.",
    );

    for required in [
        "components/chart/test/semantics.rs::chart_check2_documents_e2e_repeatable_key_flow_rules_locally",
        "components/chart/test/semantics.rs::chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic_locally",
        "components/chart/test/semantics.rs::chart_check2_marks_e2e_repeatable_key_flow_item_complete_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_e2e_repeatable_key_flow_rules",
        "components/chart/test/chart/semantics.rs::chart_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/chart/test/chart/semantics.rs::chart_check2_marks_e2e_repeatable_key_flow_item_complete",
        "components/chart/scripts/check-ui-e2e-chart.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "chart check2 repeatable key-flow section should reference `{required}`.",
        );
    }
}

#[test]
fn chart_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_chart_plot(",
        "fn render_chart_legend(",
        "let plot = render_chart_plot(",
        "let legend = render_chart_legend(",
        "{plot}",
        "{legend}",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view macro split should include `{needle}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "chart should keep a single public component boundary.",
    );

    assert!(
        view_source.matches("view! {").count() <= 6,
        "chart view should keep macro count bounded after semantic subrender split.",
    );

    assert!(
        view_source.lines().count() <= 520,
        "chart view.rs should stay bounded; split further if this grows significantly.",
    );

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn chart_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_chart_plot(",
        "fn render_chart_legend(",
        ") -> impl IntoView {",
        "pub fn Chart(",
    ] {
        assert!(
            view_source.contains(needle),
            "chart functional split should include `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_chart_plot(",
        "#[component]\nfn render_chart_legend(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart local fragments should stay plain functions, not extra components `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "chart should keep one public component boundary.",
    );

    for needle in [
        "data-slot=\"chart\"",
        "data-slot=\"chart-plot\"",
        "data-slot=\"chart-legend\"",
        "data-slot=\"chart-legend-item\"",
    ] {
        assert!(
            view_source.contains(needle),
            "chart semantic markers should stay stable after function split `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn chart_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "const CHART_PLOT_VIEWBOX: &str = \"0 0 100 56\";",
        "const CHART_GRID_LINE_CLASS: &str = \"ui-chart__grid-line\";",
        "const CHART_GRID_LINES: [(&str, &str, &str, &str); 4] = [",
        "fn render_chart_grid_lines() -> impl IntoView",
        "viewBox=CHART_PLOT_VIEWBOX",
        "{render_chart_grid_lines()}",
    ] {
        assert!(
            view_source.contains(needle),
            "chart static fragments should be constantized via `{needle}`.",
        );
    }

    for needle in [
        "role=\"img\"",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-slot=\"chart-grid\"",
        "data-slot=\"chart-plot\"",
    ] {
        assert!(
            view_source.contains(needle),
            "chart static fragment refactor should keep a11y/semantic marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn chart_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let styles_source = load_source("../../components/chart/src/styles.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let docs_chart_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let check2_source = load_source("../../components/chart/check2.md");
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_chart_source.contains(forbidden),
            "chart component/docs page should stay free from html injection token `{forbidden}`.",
        );
    }

    assert!(
        docs_shell_source.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "shared docs shell should keep the single trusted inner_html mount for readme rendering.",
    );
    assert!(
        !docs_shell_source.contains("\"chart\" => Some("),
        "chart should stay out of readme inner_html whitelist in shared shell.",
    );
    for needle in [
        "const ACCORDION_README_MD: &str = include_str!(",
        "const CHECKBOX_README_MD: &str = include_str!(",
        "const MODAL_README_MD: &str = include_str!(",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell trusted markdown whitelist should include static source marker `{needle}`.",
        );
    }

    for needle in [
        "`inner_html` 使用约束：仅允许注入受信任静态常量",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart checklist should keep inner_html safety governance marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );
}

#[test]
fn chart_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
    let check2_source = load_source("../../components/chart/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm debug isolation marker `{needle}`.",
        );
    }

    for needle in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`.",
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
            "docs debug visual entry should keep `{needle}`.",
        );
    }

    for needle in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
        "trace.emit(",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`.",
        );
    }

    for needle in [
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-motion-source=motion_source",
        "on:pointerenter=on_enter",
        "on:click=on_click",
        "on:keydown=on_key_down",
        "apply_headless_action.run(action);",
        "pub enum ChartInteractionSource",
        "Self::Focus => \"focus\"",
        "Self::Pointer => \"pointer\"",
        "Self::Keyboard => \"keyboard\"",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "chart should keep machine-readable state/source/interaction marker `{needle}` for debug attribution.",
        );
    }

    for forbidden in [
        "chart-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "chart should not duplicate shared wasm debug runtime token `{forbidden}`.",
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart checklist should keep wasm-debug governance contract marker `{needle}`.",
        );
    }
}

#[test]
fn chart_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`.",
    );
}

#[test]
fn chart_does_not_introduce_spec_rs_without_stable_schema_contract_need() {
    let module = load_source("../../components/chart/src/mod.rs");
    let check2 = load_source("../../components/chart/check2.md");
    let readme = load_source("../../components/chart/src/README.md");
    let spec_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/chart/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "chart should not add spec.rs without stable external schema contract."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "chart module should not export spec layer token `{forbidden}`."
        );
    }

    for needle in [
        "`spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "components/chart/src/README.md",
        "components/chart/check2.md",
    ] {
        assert!(
            check2.contains(needle),
            "chart/check2.md should record spec-discipline evidence `{needle}`.",
        );
    }

    assert!(
        readme.contains("# Chart"),
        "chart should keep component docs in README instead of introducing spec.rs."
    );
}

#[test]
fn chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let module = load_source("../../components/chart/src/mod.rs");
    let check2 = load_source("../../components/chart/check2.md");
    let readme = load_source("../../components/chart/src/README.md");
    let spec_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/chart/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "chart should keep `spec.rs` absent because this component has no complex external schema contract."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ChartSpec::new(",
        ".render()",
    ] {
        assert!(
            !module.contains(forbidden),
            "chart module should not expose hyper-structure builder token `{forbidden}` for simple-component scope."
        );
    }

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A：chart 当前不属于“复杂组件（稳定外部 schema/版本化 spec 契约）”范围",
        "components/chart/src/README.md",
    ] {
        assert!(
            check2.contains(needle),
            "chart/check2.md should keep hyper-structure N/A marker `{needle}`."
        );
    }

    assert!(
        readme.contains("# Chart"),
        "chart should keep component docs in README instead of introducing spec builder layer."
    );
}

#[test]
fn chart_component_files_check_script_covers_hyper_structure_builder_spec_na() {
    let source = load_source("../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_hyper_structure_builder_spec_na_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "chart check2 should mark hyper-structure builder item complete with explicit N/A rationale."
    );

    for needle in [
        "chart_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "chart_component_files_check_script_covers_hyper_structure_builder_spec_na",
        "scripts/check-ui-component-files.sh",
        "components/chart/src/spec.rs（不存在）",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 hyper-structure section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/chart/Component.toml");
    let rbi_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/chart/Component.rbi");
    assert!(
        manifest_path.exists(),
        "chart context-compression manifest should exist at `{}`.",
        manifest_path.display()
    );
    assert!(
        rbi_path.exists(),
        "chart RBI signature projection should exist at `{}`.",
        rbi_path.display()
    );

    let manifest_source = load_source("../../components/chart/Component.toml");
    let rbi_source = load_source("../../components/chart/Component.rbi");

    for needle in [
        "id = \"ui-chart\"",
        "name = \"Chart\"",
        "kind = \"snapshot\"",
        "manifest_version = 1",
        "rbi = \"Component.rbi\"",
        "mod_rs = \"src/mod.rs\"",
        "logic_rs = \"src/logic.rs\"",
        "styles_rs = \"src/styles.rs\"",
        "view_rs = \"src/view.rs\"",
        "motion_rs = \"src/motion.rs\"",
        "protocol_rs = \"src/protocol.rs\"",
        "context_compression_manifest = true",
        "rbi_signature_projection = true",
        "schema = \"ui.chart.agent-contract/v1\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "chart Component.toml should keep context-compression marker `{needle}`."
        );
    }

    for needle in [
        "component \"ui-chart\"",
        "signature Chart(",
        "mode: \"snapshot\"",
        "fallback: \"snapshot\"",
        "agent_contract_schema \"ui.chart.agent-contract/v1\"",
        "\"data-active-index\"",
        "\"data-active-value-source\"",
        "\"data-active-interaction-source\"",
        "\"data-motion-source\"",
    ] {
        assert!(
            rbi_source.contains(needle),
            "chart RBI projection should include interface signature marker `{needle}`."
        );
    }
}

#[test]
fn chart_component_files_check_script_covers_context_compression_manifest_and_rbi() {
    let source = load_source("../../scripts/check-ui-component-files.sh");
    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn chart_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "chart check2 should mark context-compression manifest/rbi item complete."
    );

    for needle in [
        "components/chart/Component.toml",
        "components/chart/Component.rbi",
        "chart_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "chart_component_files_check_script_covers_context_compression_manifest_and_rbi",
        "scripts/check-ui-component-files.sh",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 context-compression section should reference `{needle}`."
        );
    }
}

#[test]
fn chart_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");
    let manifest_source = load_source("../../components/chart/Component.toml");

    for needle in [
        "pub const CHART_AGENT_SCHEMA: &str = \"ui.chart.agent-contract\";",
        "pub enum ChartAgentSchemaVersion",
        "pub enum ChartAgentIntent",
        "pub enum ChartAgentAction",
        "pub enum ChartAgentKind",
        "pub enum ChartAgentState",
        "pub enum ChartAgentSource",
        "pub enum ChartAgentStreamSupport",
        "pub enum ChartAgentStreamFallback",
        "pub enum ChartAgentOutputStatus",
        "pub struct ChartAgentContract",
        "pub struct ChartAgentContractInput",
        "pub const fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "chart logic should keep typed agent-contract marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view should expose machine-readable agent-contract marker `{needle}`."
        );
    }

    for needle in [
        "schema = \"ui.chart.agent-contract/v1\"",
        "schema = \"data-ui-schema\"",
        "intent = \"data-ui-intent\"",
        "action = \"data-ui-action\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "chart Component.toml should keep agent-contract schema marker `{needle}`."
        );
    }
}

#[test]
fn chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let view_source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "kind: match input.state.kind {",
        "state: match input.state.state_attr {",
        "source: ChartAgentSource::StatePrimitives,",
        "stream_support: ChartAgentStreamSupport::Optional,",
        "stream_fallback: ChartAgentStreamFallback::Snapshot,",
        "output_status: ChartAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(needle),
            "chart agent contract should derive field from typed state/source axis via `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"ui.chart.agent-contract\"",
        "data-ui-intent=\"chart.interaction\"",
        "data-ui-action=\"navigate.activate\"",
        "format!(\"ui.chart",
        "schema_version.to_string()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "chart view should avoid free-form agent-contract string splicing token `{forbidden}`."
        );
    }
}

#[test]
fn chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let manifest_source = load_source("../../components/chart/Component.toml");
    let view_source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "[agent_contract_whitelist]",
        "logic::normalize_input_boundary(...)",
        "logic::derive_state_from_boundary(...)",
        "logic::resolve_agent_contract(...)",
        "use_chart(...)",
        "motion::attach_motion(...)",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            manifest_source.contains(needle) || view_source.contains(needle),
            "chart should keep whitelist-safe render path marker `{needle}`."
        );
    }

    for forbidden in ["inner_html=", "<script", "javascript:", "eval("] {
        assert!(
            !view_source.contains(forbidden),
            "chart view must reject script-injection-prone token `{forbidden}`."
        );
    }
}

#[test]
fn chart_contract_hygiene_script_covers_agent_contract_schema_contract() {
    let source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn chart_check2_documents_agent_contract_schema_governance_rules() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "chart_check2_documents_agent_contract_schema_governance_rules",
        "chart_agent_contract_is_schema_typed_and_machine_readable",
        "chart_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "chart_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 should keep agent-contract governance marker `{needle}`."
        );
    }
}

#[test]
fn chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/chart/check2.md");
    let view_source = load_source("../../components/chart/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`Chart` 不是 LLM 正文渲染组件，定义仅用于与上层 LLM 输出协议对齐。",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 should keep streaming-definition marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            view_source.contains(needle),
            "chart view should expose snapshot-compatible stream marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn chart_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn chart_check2_marks_streaming_two_mode_definition_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "chart check2 should mark streaming two-mode definition gate complete.",
    );

    for needle in [
        "chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes_locally",
        "chart_streaming_script_covers_two_mode_definition_contract_locally",
        "chart_check2_marks_streaming_two_mode_definition_complete_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "components/chart/test/chart/semantics.rs::chart_streaming_script_covers_two_mode_definition_contract",
        "components/chart/test/chart/semantics.rs::chart_check2_marks_streaming_two_mode_definition_complete",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 streaming section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/chart/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`Chart` 不直接渲染 LLM 正文；组件侧能力定义为“消费完整配置并一次性稳定渲染”。",
        "chart_check2_documents_snapshot_as_default_baseline_capability_locally",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 should keep snapshot-baseline marker `{needle}`.",
        );
    }
}

#[test]
fn chart_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let check2_source = load_source("../../components/chart/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}");

    for needle in [
        "let normalized = logic::normalize_input_boundary(logic::ChartInputBoundary {",
        "let state = Signal::derive(move || {",
        "logic::derive_state_from_boundary(logic::ChartStateBoundary {",
        "let semantics = Signal::derive(move || {",
        "use_chart(ChartOptions {",
        "data-state=move || semantics.get().attrs.data_state",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-active-value-source=move || active_value_source.get().as_attr()",
        "data-active-interaction-source=move || active_interaction_source.get().as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            view_source.contains(needle),
            "chart snapshot baseline should keep stable complete-result render marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_input_boundary(input: ChartInputBoundary) -> ChartNormalizedInput",
        "pub fn derive_state_from_boundary(input: ChartStateBoundary) -> ChartState",
        "pub fn normalize_interaction_index(",
        "stream_fallback: ChartAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "chart logic should keep snapshot-baseline normalization marker `{needle}`.",
        );
    }

    for forbidden in [
        "streaming_chunk",
        "token_delta",
        "partial token",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "chart snapshot baseline should avoid incremental streaming marker `{forbidden}`.",
        );
    }

    for needle in [
        "chart_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "data-ui-stream-mode=\"snapshot\"",
        "data-active-index=move || state.get().active_index.to_string()",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 snapshot section should reference `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn chart_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("../../components/chart/check2.md");

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "chart check2 should mark snapshot baseline gate complete.",
    );

    for needle in [
        "chart_check2_documents_snapshot_as_default_baseline_capability_locally",
        "chart_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "chart_streaming_script_covers_snapshot_baseline_contract_locally",
        "chart_check2_marks_snapshot_baseline_capability_complete_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_snapshot_as_default_baseline_capability",
        "components/chart/test/chart/semantics.rs::chart_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "components/chart/test/chart/semantics.rs::chart_streaming_script_covers_snapshot_baseline_contract",
        "components/chart/test/chart/semantics.rs::chart_check2_marks_snapshot_baseline_capability_complete",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 snapshot section should reference `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/chart/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "N/A：`Chart` 归类为 `Streaming Optional`，仅消费 `Snapshot`，并固定 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(needle),
            "chart check2 should keep required/optional classification marker `{needle}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");

    for needle in [
        "role=move || semantics.get().attrs.role",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-source=move || semantics.get().attrs.data_source",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "chart optional-streaming scope should keep semantic continuity marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum ChartAgentOutputStatus",
        "ChartAgentOutputStatus::Verified",
        "output_status: ChartAgentOutputStatus,",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "chart optional-streaming scope should expose explicit output-status marker `{needle}`.",
        );
    }
}

#[test]
fn chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/chart/src/view.rs");
    let logic_source = load_source("../../components/chart/src/logic.rs");
    let mod_source = load_source("../../components/chart/src/mod.rs");
    let motion_source = load_source("../../components/chart/src/motion.rs");
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
            "chart should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn chart_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_source("../../components/chart/check2.md");

    assert!(
        source.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "chart check2 should mark required/optional classification gate complete.",
    );

    for needle in [
        "chart_check2_documents_streaming_required_optional_classification_rules_locally",
        "chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous_locally",
        "chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer_locally",
        "chart_streaming_script_covers_required_optional_classification_contract_locally",
        "chart_check2_marks_streaming_required_optional_classification_complete_locally",
        "components/chart/test/chart/semantics.rs::chart_check2_documents_streaming_required_optional_classification_rules",
        "components/chart/test/chart/semantics.rs::chart_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "components/chart/test/chart/semantics.rs::chart_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "components/chart/test/chart/semantics.rs::chart_streaming_script_covers_required_optional_classification_contract",
        "components/chart/test/chart/semantics.rs::chart_check2_marks_streaming_required_optional_classification_complete",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            source.contains(needle),
            "chart check2 should keep required/optional classification evidence marker `{needle}`.",
        );
    }
}

#[test]
fn chart_check2_marks_all_items_complete() {
    let source = load_source("../../components/chart/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "chart/check2.md should not keep unchecked checklist items after completion."
    );
}

#[test]
fn chart_check2_records_layering_and_verification_evidence() {
    let source = load_source("../../components/chart/check2.md");

    for needle in [
        "component-chart = [\"dep:ui-chart\"]",
        "crates/ui-state-primitives/src/chart.rs",
        "crates/ui-headless/src/chart.rs",
        "components/chart/src/motion.rs",
        "cargo test -p ui --test chart_semantics --no-default-features --features component-chart,inject-css",
    ] {
        assert!(
            source.contains(needle),
            "chart/check2.md should include completion evidence `{needle}`."
        );
    }
}
