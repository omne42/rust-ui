use std::fs;
use std::path::PathBuf;

fn load_source(rel: &str) -> String {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = base.join(rel);
    fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("failed to read `{}`: {err}", path.display());
    })
}

#[test]
fn docs_perf_probe_budgets_are_wired_for_component_pages() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "use crate::perf_probe::{UiPerfBudget, UiPerfProbe};",
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"accordion\" => UiPerfBudget {",
        "UiPerfBudget::mount_only(120.0)",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should define budgeted perf probe via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
    ] {
        assert!(
            coverage_source.contains(needle),
            "e2e coverage should assert perf contract marker `{needle}`."
        );
    }
}

#[test]
fn perf_render_count_follow_up_is_tracked_in_plan() {
    let source = load_source("../../docs/plan/TODO.md");
    assert!(
        source.contains("render_count"),
        "perf governance should keep explicit follow-up task for render_count automation."
    );
}
