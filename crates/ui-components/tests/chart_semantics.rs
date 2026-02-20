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
        "use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "Chart should support `{needle}` for active-index control flow."
        );
    }
}

#[test]
fn chart_accepts_is_disabled_and_locale_contract_inputs() {
    let source = load_source("../../components/chart/src/view.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: bool",
        "let resolved_disabled = is_disabled.unwrap_or(disabled);",
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
fn chart_check2_marks_all_items_complete() {
    let source = load_source("../../components/chart/src/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "chart/check2.md should not keep unchecked checklist items after completion."
    );
}

#[test]
fn chart_check2_records_layering_and_verification_evidence() {
    let source = load_source("../../components/chart/src/check2.md");

    for needle in [
        "component-chart = [\"dep:ui-chart\"]",
        "crates/ui-state-primitives/src/chart.rs",
        "crates/ui-headless/src/chart.rs",
        "components/chart/src/motion.rs",
        "cargo test -p ui-components --test chart_semantics --no-default-features --features component-chart,inject-css",
    ] {
        assert!(
            source.contains(needle),
            "chart/check2.md should include completion evidence `{needle}`."
        );
    }
}
