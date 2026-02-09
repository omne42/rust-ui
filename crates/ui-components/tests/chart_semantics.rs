use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn chart_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/chart/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Chart internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn chart_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/chart/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Chart;"),
        "chart module should export `Chart`."
    );
    assert!(
        module_source.contains("ChartMotion"),
        "chart module should expose a motion alias."
    );
    assert!(
        crate_source.contains("pub use chart::{Chart, ChartKind, ChartMotion, ChartPoint};"),
        "crate root should re-export chart contracts."
    );
}

#[test]
fn chart_uses_logic_state_model() {
    let logic_source = load_source("src/chart/logic.rs");
    let view_source = load_source("src/chart/view.rs");

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
            logic_source.contains(needle),
            "Chart logic should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_points(points)",
        "logic::value_domain(points.as_ref())",
        "logic::resolve_state(ChartStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::next_index_for_key(",
        "logic::polyline_points(",
    ] {
        assert!(
            view_source.contains(needle),
            "Chart view should derive behavior via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn chart_supports_controlled_and_uncontrolled_active_index() {
    let source = load_source("src/chart/view.rs");

    for needle in [
        "active_index: Option<Signal<usize>>",
        "default_active_index: Option<usize>",
        "on_active_index_change: Option<Callback<usize>>",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "Chart should support `{needle}` for active-index control flow."
        );
    }
}

#[test]
fn chart_composes_active_highlight_motion_for_legend() {
    let source = load_source("src/chart/view.rs");

    for needle in [
        "attach_active_highlight_motion(",
        "node_ref=legend_ref",
        "node_ref=highlight_ref",
        "data-slot=\"chart-legend-highlight\"",
    ] {
        assert!(
            source.contains(needle),
            "Chart should compose active-highlight motion via `{needle}`."
        );
    }
}

#[test]
fn chart_emits_spectrum_state_data_attributes() {
    let source = load_source("src/chart/view.rs");

    for needle in [
        "data-slot=\"chart\"",
        "data-kind=move || state.get().kind_attr",
        "data-state=move || state.get().state_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(\"true\")",
        "data-active-index=move || state.get().active_index.to_string()",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Chart should expose `{needle}` for stable styling/testing contracts."
        );
    }
}

#[test]
fn chart_styles_include_plot_and_legend_markers() {
    let source = load_source("src/chart/styles.rs");

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
