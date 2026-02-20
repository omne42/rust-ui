use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn calendar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Calendar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn calendar_module_exposes_motion_contracts() {
    let source = load_source("src/mod.rs");

    for needle in [
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::CalendarMotion;",
        "pub use view::Calendar;",
    ] {
        assert!(
            source.contains(needle),
            "Calendar module should include `{needle}` contract."
        );
    }
}

#[test]
fn calendar_logic_delegates_state_primitives() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub use ui_logic_calendar::calendar::{",
        "CalendarFirstWeekday",
        "CalendarGridCell",
        "CalendarState",
        "CalendarStateInput",
        "CalendarTone",
        "DEFAULT_ARIA_LABEL",
        "build_month_grid",
        "normalize_month",
        "normalize_selected_day",
        "resolve_state",
        "weekday_labels",
        "pub fn compose_class_name(",
        "pub struct CalendarAgentContract",
        "pub fn resolve_agent_contract(state: CalendarState)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Calendar logic should include `{needle}` for state-primitive delegation."
        );
    }

    for forbidden in [
        "pub enum CalendarTone {",
        "pub enum CalendarFirstWeekday {",
        "pub struct CalendarGridCell {",
        "pub struct CalendarStateInput {",
        "pub struct CalendarState {",
        "pub fn normalize_aria_label(",
        "pub fn normalize_month(",
        "pub fn normalize_selected_day(",
        "pub fn weekday_index(",
        "pub fn build_month_grid(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Calendar logic should not reimplement state primitives after migration: `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_month(month)",
        "logic::normalize_selected_day(selected_day, year, normalized_month)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(CalendarStateInput {",
        "logic::resolve_agent_contract(state)",
        "logic::compose_class_name(class_name, state)",
        "logic::build_month_grid(",
    ] {
        assert!(
            view_source.contains(needle),
            "Calendar view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn calendar_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"calendar\"",
        "data-tone=state.tone_attr",
        "data-first-weekday=state.first_weekday_attr",
        "data-state=state.data_state_attr",
        "data-show-outside-days=state.show_outside_days.then_some(\"true\")",
        "data-selected-day=state.selected_day.map(|day| day.to_string())",
        "data-year=state.year.to_string()",
        "data-month=state.month.to_string()",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-motion-source=motion_source",
        "data-custom-motion=(motion_source == \"custom\").then_some(\"true\")",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-state=agent_contract.state.as_attr()",
        "data-ui-source=agent_contract.source.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
        "data-slot=\"calendar-header\"",
        "data-slot=\"calendar-title\"",
        "data-slot=\"calendar-weekdays\"",
        "data-slot=\"calendar-weekday\"",
        "data-slot=\"calendar-grid\"",
        "data-slot=\"calendar-day\"",
        "data-slot=\"calendar-day-empty\"",
        "role=\"group\"",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "Calendar should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn calendar_styles_include_tone_weekday_and_selection_markers() {
    let source = load_source("src/styles.rs");

    for selector in [
        "--ui-calendar-motion-duration",
        ".ui-calendar--tone-default",
        ".ui-calendar[data-tone=\"default\"]",
        ".ui-calendar--tone-quiet",
        ".ui-calendar--tone-strong",
        ".ui-calendar--weekday-sunday",
        ".ui-calendar[data-first-weekday=\"sunday\"]",
        ".ui-calendar--weekday-monday",
        ".ui-calendar[data-first-weekday=\"monday\"]",
        ".ui-calendar--outside-days",
        ".ui-calendar[data-show-outside-days=\"true\"]",
        ".ui-calendar--has-selection",
        ".ui-calendar[data-state=\"selected\"]",
        ".ui-calendar--custom-class",
        ".ui-calendar[data-custom-class=\"true\"]",
        ".ui-calendar__day--selected",
        ".ui-calendar__day[data-selected=\"true\"]",
        ".ui-calendar__day--outside",
        ".ui-calendar__day[data-month-source=\"outside\"]",
        ".ui-calendar__day:active",
    ] {
        assert!(
            source.contains(selector),
            "Calendar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn calendar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn calendar() -> AnyView",
        "title=\"Calendar\"",
        "slug=\"calendar\"",
        "title=\"Default + Outside Days\"",
        "title=\"Monday First + Strong Tone\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "\"Source-first / Copy-Paste Ready\"",
    ] {
        assert!(
            source.contains(needle),
            "calendar docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn calendar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "<Playground title=\"Default + Outside Days\" code_signal=code>",
        "year=2026",
        "month=1",
        "selected_day=Some(6)",
        "tone=CalendarTone::Default",
        "first_weekday=CalendarFirstWeekday::Sunday",
        "show_outside_days=true",
        "<Playground title=\"Monday First + Strong Tone\" code_signal=states_code>",
        "month=2",
        "selected_day=Some(14)",
        "tone=CalendarTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "show_outside_days=false",
        "class_name=\"docs-calendar-custom\".to_string()",
        "let (interactive_month, set_interactive_month) = signal(3_u8);",
        "data-slot=\"calendar-interactive-controls\"",
        "data-action=\"prev-month\"",
        "data-action=\"next-month\"",
        "data-action=\"toggle-weekday\"",
        "data-action=\"toggle-tone\"",
        "data-action=\"toggle-outside-days\"",
        "data-action=\"clear-selection\"",
        "data-slot=\"calendar-interactive-summary\"",
        "class_name=\"docs-calendar-interactive\".to_string()",
        "data-slot=\"calendar-source-first\"",
        "class_name=\"docs-calendar-source-copy\".to_string()",
        "\"components/calendar/src/motion.rs\"",
        "\"component-calendar\"",
        "\"inject-css\"",
    ] {
        assert!(
            source.contains(needle),
            "calendar docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn calendar_motion_contract_exposes_sanitization_and_style_vars() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub struct CalendarMotion",
        "pub fn sanitize_motion(motion: CalendarMotion) -> CalendarMotion",
        "pub fn source_attr(motion: CalendarMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: CalendarMotion) -> String",
        "--ui-calendar-motion-duration",
    ] {
        assert!(
            source.contains(needle),
            "Calendar motion should include `{needle}`."
        );
    }
}

#[test]
fn calendar_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_calendar_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "#/components/calendar",
        "[data-slot=\"calendar\"]",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "[data-action=\"next-month\"]",
        "[data-action=\"clear-selection\"]",
    ] {
        assert!(
            source.contains(needle),
            "calendar e2e should include `{needle}` semantic contract selector/wait."
        );
    }
}

#[test]
fn calendar_check2_marks_component_governance_complete() {
    let check2_source = load_source("src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui-components` 定义",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "N/A：`Calendar` 无远程请求与异步状态轴",
        "Streaming Optional",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar/check2.md should pin completion marker `{needle}`."
        );
    }
}

#[test]
fn calendar_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Calendar check2.md should not keep unchecked checklist items after completion."
    );
}

#[test]
fn calendar_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        "### Calendar 同步记录（2026-02-19）",
        "`Calendar` 继续保持月视图基元定位",
        "component_doc!(\"Calendar\", \"calendar\", \"Forms\", forms_extra::calendar)",
        "`#/components/calendar` 可索引访问",
        "Interactive Playground (State + Source Markers)",
        "Source-first / Copy-Paste Ready",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI strategy doc should keep calendar sync token `{needle}`."
        );
    }

    for needle in ["\"Calendar\"", "\"calendar\"", "forms_extra::calendar"] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep calendar docs entry `{needle}`."
        );
    }
}
