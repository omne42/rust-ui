use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    let source = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));

    if rel_path.ends_with("apps/docs-app/src/pages/components/pages/forms_extra.rs") {
        let forms_extra_dir = path
            .parent()
            .unwrap_or_else(|| panic!("forms_extra.rs should have parent: {path:?}"))
            .join("forms_extra");
        let mut combined = source.clone();

        for line in source.lines() {
            let trimmed = line.trim();
            let Some(rest) = trimmed.strip_prefix("#[path = \"forms_extra/") else {
                continue;
            };
            let Some(rel_end) = rest.find("\"]") else {
                continue;
            };
            let child_rel = &rest[..rel_end];
            let child_path = forms_extra_dir.join(child_rel);
            let child_source = fs::read_to_string(&child_path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
            let compat_child = child_source.replace("pub(crate) fn ", "pub(super) fn ");
            combined.push('\n');
            combined.push_str(&compat_child);
        }

        return combined;
    }

    source
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
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            source.contains(needle),
            "Calendar module should include `{needle}` contract."
        );
    }
}

#[test]
fn calendar_component_standard_file_layout_and_public_boundary_are_stable() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_rs_files: BTreeSet<String> = fs::read_dir(manifest_dir.join("src"))
        .unwrap_or_else(|e| panic!("read_dir failed for src/: {e}"))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                return None;
            }
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_string())
        })
        .collect();
    let expected_source_rs_files: BTreeSet<String> =
        ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"]
            .into_iter()
            .map(|name| name.to_string())
            .collect();
    assert_eq!(
        source_rs_files, expected_source_rs_files,
        "Calendar src/ should keep strict file-placement discipline for source files."
    );

    for required_file in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            manifest_dir.join(required_file).exists(),
            "Calendar should keep standard component file `{required_file}`."
        );
    }

    for forbidden_file in [
        "src/render.rs",
        "src/spec.rs",
        "src/debug.rs",
        "src/protocol.rs",
        "src/README.md",
        "src/check2.md",
    ] {
        assert!(
            !manifest_dir.join(forbidden_file).exists(),
            "Calendar should not introduce optional/legacy file `{forbidden_file}` in standard layout."
        );
    }

    let mod_source = load_source("src/mod.rs");
    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::CalendarMotion;",
        "pub use view::Calendar;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Calendar mod.rs should keep minimal stable export boundary via `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod debug;",
        "pub mod protocol;",
        "pub use debug::",
        "pub use protocol::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Calendar mod.rs should not over-export internals via `{forbidden}`."
        );
    }
}

#[test]
fn calendar_logic_delegates_state_primitives() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub use ui_state_primitives::calendar::{",
        "CalendarFirstWeekday",
        "CalendarGridCell",
        "CalendarState",
        "CalendarStateInput",
        "CalendarTone",
        "CalendarSelectedDayMode",
        "CalendarSelectedDaySource",
        "DEFAULT_ARIA_LABEL",
        "build_month_grid",
        "normalize_month",
        "normalize_is_show_outside_days",
        "normalize_selected_day_axis",
        "resolve_effective_selected_day",
        "resolve_selected_day_press_update",
        "normalize_selected_day",
        "resolve_state",
        "weekday_labels",
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
        "pub fn normalize_is_show_outside_days(",
        "pub fn normalize_month(",
        "pub fn normalize_selected_day(",
        "pub enum CalendarSelectedDayMode {",
        "pub struct CalendarSelectedDayAxis {",
        "pub enum CalendarSelectedDaySource {",
        "pub fn normalize_selected_day_axis(",
        "pub fn resolve_effective_selected_day(",
        "pub struct CalendarSelectedDayPressUpdate {",
        "pub fn resolve_selected_day_press_update(",
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
        "logic::normalize_is_show_outside_days(is_show_outside_days, show_outside_days)",
        "logic::normalize_selected_day_axis(",
        "logic::resolve_effective_selected_day(selected_day_axis, uncontrolled_selected_day.get())",
        "logic::resolve_selected_day_press_update(",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(CalendarStateInput {",
        "logic::resolve_agent_contract(state.get())",
        "fn compose_class_name(base_class_name: Option<String>, state: logic::CalendarState)",
        "compose_class_name(class_name.get_value(), state.get())",
        "logic::build_month_grid(",
        "use_calendar_root(CalendarRootOptions {",
        "use_calendar_day(",
    ] {
        assert!(
            view_source.contains(needle),
            "Calendar view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn calendar_api_uses_is_prefix_for_outside_days_bool_axis() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("README.md");

    for needle in [
        "#[prop(optional)] is_show_outside_days: Option<bool>",
        "#[prop(optional)] show_outside_days: Option<bool>",
        "#[prop(optional)] default_selected_day: Option<u8>",
        "#[prop(default = None)] on_selected_day_change: Option<Callback<Option<u8>>>",
        "logic::normalize_is_show_outside_days(is_show_outside_days, show_outside_days)",
        "logic::normalize_selected_day_axis(",
        "is_show_outside_days=true",
        "is_show_outside_days=false",
        "旧参数 `show_outside_days` 仅保留兼容",
    ] {
        assert!(
            view_source.contains(needle) || readme_source.contains(needle),
            "Calendar API naming contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] show_outside_days: bool"),
        "Calendar bool axis should not expose legacy bare bool prop signature."
    );
}

#[test]
fn calendar_default_priority_is_normalized_in_logic_only() {
    let logic_source = load_source("src/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/calendar.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "normalize_is_show_outside_days",
        "normalize_selected_day_axis",
        "CalendarSelectedDaySource",
        "resolve_selected_day_press_update",
    ] {
        assert!(
            logic_source.contains(needle),
            "Calendar logic should re-export default-priority helpers from state-primitives; missing `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_is_show_outside_days(",
        "pub fn normalize_selected_day_axis(",
        "pub enum CalendarSelectedDaySource",
        "source: CalendarSelectedDaySource::SelectedDay",
        "source: if has_default_selected_day {",
    ] {
        assert!(
            primitives_source.contains(needle),
            "Calendar default-priority rule should live in ui-state-primitives; missing `{needle}`."
        );
    }

    for forbidden in [
        "unwrap_or(",
        ".or(default_selected_day)",
        "selected_day.or(",
        "set_uncontrolled_selected_day.set(Some(day))",
        "selected_day_source.set(\"interaction\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar view.rs should not perform default fallback branching: `{forbidden}`."
        );
    }
}

#[test]
fn calendar_non_test_sources_follow_rust_hygiene_contract() {
    let source_files = [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ];

    for rel_path in source_files {
        let source = load_source(rel_path);
        assert!(
            !source.contains(".unwrap(")
                && !source.contains(".unwrap_err(")
                && !source.contains(".expect("),
            "Calendar non-test source should not use unwrap/expect; found in `{rel_path}`."
        );

        assert!(
            !source.contains("String::from(") && !source.contains(".to_owned()"),
            "Calendar should avoid string clone hotspot patterns in `{rel_path}`."
        );

        for (line_index, line) in source.lines().enumerate() {
            assert!(
                !line.trim_start().starts_with("let _ ="),
                "Calendar non-test source should not swallow results via `let _ = ...`; found in `{rel_path}`:{}.",
                line_index + 1
            );
        }
    }

    let view_source = load_source("src/view.rs");
    for marker in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-calendar\")",
        "Cow::Borrowed(\"ui-calendar--outside-days\")",
        "Cow::Borrowed(\"ui-calendar--has-selection\")",
        "Cow::Borrowed(\"ui-calendar--custom-class\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            view_source.contains(marker),
            "Calendar class-name composition should use Cow-based string contract; missing `{marker}`."
        );
    }

    for forbidden in [
        "\"ui-calendar\".to_string()",
        "\"ui-calendar--outside-days\".to_string()",
        "\"ui-calendar--has-selection\".to_string()",
        "\"ui-calendar--custom-class\".to_string()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar class-name composition should avoid clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn calendar_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"calendar\"",
        "data-tone=move || state.get().tone_attr",
        "data-first-weekday=move || state.get().first_weekday_attr",
        "data-state=move || state.get().data_state_attr",
        "data-show-outside-days=move || state.get().show_outside_days.then_some(\"true\")",
        "data-selected-day=move || state.get().selected_day.map(|day| day.to_string())",
        "data-selected-day-mode=selected_day_mode.as_attr()",
        "data-selected-day-source=move || selected_day_source.get().as_attr()",
        "data-year=move || state.get().year.to_string()",
        "data-month=move || state.get().month.to_string()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source",
        "data-custom-motion=(motion_source == \"custom\").then_some(\"true\")",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
        "data-slot=\"calendar-header\"",
        "data-slot=\"calendar-title\"",
        "data-slot=\"calendar-weekdays\"",
        "data-slot=\"calendar-weekday\"",
        "data-slot=\"calendar-grid\"",
        "data-slot=\"calendar-day\"",
        "data-slot=\"calendar-day-empty\"",
        "role=root.attrs.role",
        "lang=root.attrs.lang.clone()",
        "dir=root.attrs.dir",
        "aria-disabled=day_contract.attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "Calendar should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn calendar_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/calendar.rbi");
    let check2_source = load_source("check2.md");

    for marker in [
        "pub enum CalendarAgentAction",
        "pub enum CalendarAgentState",
        "pub enum CalendarAgentSource",
        "pub enum CalendarAgentStreamSupport",
        "pub enum CalendarAgentStreamFallback",
        "pub enum CalendarAgentOutputStatus",
        "pub struct CalendarAgentContract",
        "pub fn resolve_agent_contract(state: CalendarState) -> CalendarAgentContract",
        "schema_attr: \"ui.calendar\"",
        "intent_attr: \"date-selection\"",
        "action: if has_selected_day {",
        "state: if has_selected_day {",
        "source: if has_selected_day {",
    ] {
        assert!(
            logic_source.contains(marker),
            "Calendar logic should keep typed agent-contract marker `{marker}`."
        );
    }

    for forbidden in [
        "pub action: String",
        "pub state: String",
        "pub source: String",
        "data-ui-action=\"",
        "data-ui-state=\"",
        "data-ui-source=\"",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Calendar should avoid free-form string protocol drift: `{forbidden}`."
        );
    }

    for marker in [
        "let agent_contract = Signal::derive(move || logic::resolve_agent_contract(state.get()));",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Calendar view should mount typed/traceable agent marker `{marker}`."
        );
    }

    for marker in [
        "name = \"agent_contract_schema_markers\"",
        "name = \"semantic-markers\"",
        "data-ui-*",
    ] {
        assert!(
            manifest_source.contains(marker),
            "Calendar Component.toml should project agent-contract marker `{marker}`."
        );
    }

    for marker in [
        "pub enum CalendarAgentAction",
        "pub enum CalendarAgentState",
        "pub enum CalendarAgentSource",
        "pub struct CalendarAgentContract",
        "pub fn Calendar(",
    ] {
        assert!(
            rbi_source.contains(marker),
            "calendar.rbi should keep agent-contract signature marker `{marker}`."
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Calendar render chain should keep whitelist boundary; found `{forbidden}`."
        );
    }

    for marker in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "calendar_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered",
    ] {
        assert!(
            check2_source.contains(marker),
            "Calendar check2 should keep agent-contract schema marker `{marker}`."
        );
    }
}

#[test]
fn calendar_streaming_definition_is_llm_scoped_and_limited_to_streaming_snapshot_modes() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/calendar.rbi");
    let check2_source = load_source("check2.md");

    for marker in [
        "pub enum CalendarAgentStreamSupport",
        "Unsupported",
        "pub enum CalendarAgentStreamFallback",
        "Snapshot",
        "stream_support: CalendarAgentStreamSupport::Unsupported",
        "stream_fallback: CalendarAgentStreamFallback::Snapshot",
    ] {
        assert!(
            logic_source.contains(marker),
            "Calendar logic should keep stream definition marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-stream-mode=\"snapshot\"",
    ] {
        assert!(
            view_source.contains(marker),
            "Calendar view should expose stream definition marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-stream-mode=\"incremental\"",
        "data-ui-stream-mode=\"partial\"",
        "data-ui-stream-mode=\"chunk\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar stream display modes should stay constrained; found `{forbidden}`."
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "name = \"semantic-markers\"",
        "data-ui-*",
    ] {
        assert!(
            manifest_source.contains(marker),
            "Calendar Component.toml should project stream/snapshot definition marker `{marker}`."
        );
    }

    for marker in [
        "pub enum CalendarAgentStreamSupport",
        "pub enum CalendarAgentStreamFallback",
        "Snapshot",
    ] {
        assert!(
            rbi_source.contains(marker),
            "calendar.rbi should keep stream/snapshot signature marker `{marker}`."
        );
    }

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "calendar_streaming_definition_is_llm_scoped_and_limited_to_streaming_snapshot_modes",
    ] {
        assert!(
            check2_source.contains(marker),
            "Calendar check2 should keep stream-definition marker `{marker}`."
        );
    }
}

#[test]
fn calendar_snapshot_is_baseline_capability_for_complete_outputs() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/calendar.rbi");
    let check2_source = load_source("check2.md");

    for marker in [
        "pub enum CalendarAgentStreamFallback",
        "Snapshot",
        "stream_fallback: CalendarAgentStreamFallback::Snapshot",
        "pub enum CalendarAgentOutputStatus",
        "Verified",
        "output_status: CalendarAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(marker),
            "Calendar logic should keep snapshot baseline marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Calendar view should expose snapshot baseline marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-stream-mode=move ||",
        "data-ui-stream-mode=\"streaming\"",
        "data-ui-stream-mode=\"draft\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar snapshot baseline should avoid unstable stream-mode marker `{forbidden}`."
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "name = \"semantic-markers\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "Calendar Component.toml should include snapshot capability marker `{marker}`."
        );
    }

    for marker in ["pub enum CalendarAgentStreamFallback", "Snapshot"] {
        assert!(
            rbi_source.contains(marker),
            "calendar.rbi should include snapshot signature marker `{marker}`."
        );
    }

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "calendar_snapshot_is_baseline_capability_for_complete_outputs",
    ] {
        assert!(
            check2_source.contains(marker),
            "Calendar check2 should keep snapshot baseline marker `{marker}`."
        );
    }
}

#[test]
fn calendar_streaming_policy_is_optional_snapshot_with_status_markers_and_upper_layer_retry_boundary()
 {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");

    for marker in [
        "pub enum CalendarAgentStreamSupport",
        "Unsupported",
        "pub enum CalendarAgentStreamFallback",
        "Snapshot",
        "stream_support: CalendarAgentStreamSupport::Unsupported",
        "stream_fallback: CalendarAgentStreamFallback::Snapshot",
        "output_status: CalendarAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(marker),
            "Calendar logic should keep streaming-optional marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
        "role=root.attrs.role",
        "aria-label=root.attrs.aria_label.clone()",
        "data-slot=\"calendar\"",
    ] {
        assert!(
            view_source.contains(marker),
            "Calendar view should keep continuous status/a11y markers via `{marker}`."
        );
    }

    for forbidden in [
        "retry",
        "reconnect",
        "data-retry",
        "data-error",
        "on_retry",
        "on_reconnect",
        "aria-busy",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Calendar should keep retry/recovery policy in upper layer; found `{forbidden}`."
        );
    }

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "Streaming Optional",
        "fallback=snapshot",
        "calendar_streaming_policy_is_optional_snapshot_with_status_markers_and_upper_layer_retry_boundary",
    ] {
        assert!(
            check2_source.contains(marker),
            "Calendar check2 should keep streaming policy marker `{marker}`."
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
        "@media (prefers-reduced-motion: reduce)",
        "--ui-calendar-motion-duration: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));",
    ] {
        assert!(
            source.contains(selector),
            "Calendar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn calendar_styles_are_token_first_and_theme_driven() {
    let source = load_source("src/styles.rs");

    for token_var in [
        "--ui-text-field-motion-duration",
        "--ui-text-field-motion-easing",
        "--ui-space-xs",
        "--ui-space-sm",
        "--ui-radius-lg",
        "--ui-bg",
        "--ui-fg",
        "--ui-border",
        "--ui-heading-h6-font-size",
        "--ui-heading-h6-line-height",
        "--ui-font-size-100",
        "--ui-line-height-100",
        "--ui-component-height-100",
    ] {
        assert!(
            source.contains(token_var),
            "Calendar styles should consume theme token variable `{token_var}`."
        );
    }

    for forbidden_hex_literal in ["#000", "#fff", "#FFF", "#FFFFFF", "#000000"] {
        assert!(
            !source.contains(forbidden_hex_literal),
            "Calendar styles should avoid private hex literal `{forbidden_hex_literal}` and consume theme tokens."
        );
    }

    for defensive_var in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
    ] {
        assert!(
            source.contains(defensive_var),
            "Calendar styles should use defensive token fallback chain `{defensive_var}`."
        );
    }

    for forbidden_bare_token in [
        "var(--ui-space-xs)",
        "var(--ui-space-sm)",
        "var(--ui-component-height-100)",
        "var(--ui-border-width)",
        "var(--ui-border)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-accent)",
        "var(--ui-accent-soft)",
        "var(--ui-fg-muted)",
    ] {
        assert!(
            !source.contains(forbidden_bare_token),
            "Calendar styles should not use bare theme token `{forbidden_bare_token}` without fallback."
        );
    }

    for forbidden_terminal_unit in ["px;", "rem;", "ms;"] {
        assert!(
            !source.contains(forbidden_terminal_unit),
            "Calendar styles should avoid hardcoded terminal unit literals (`{forbidden_terminal_unit}`) in component CSS."
        );
    }
}

#[test]
fn calendar_css_layering_and_runtime_style_contract_stay_ui_scoped() {
    let components_css_source = load_source("../../crates/ui/src/css.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-calendar\")]",
        "out.push_str(crate::calendar::styles::CSS);",
    ] {
        assert!(
            components_css_source.contains(needle),
            "Calendar CSS should be aggregated into `@layer ui` via `{needle}`."
        );
    }

    for needle in [
        "let panel_vars = crate::motion::attach_motion(None, motion);",
        "style=panel_vars",
    ] {
        assert!(
            view_source.contains(needle),
            "Calendar runtime style should only mount css vars through `{needle}`."
        );
    }

    for forbidden in ["style=\"", "style:top=", "style:left=", "style:transform="] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar view should not use regular inline style path `{forbidden}`."
        );
    }

    for needle in [
        "style.push_str(&format!(",
        "\" --ui-calendar-motion-duration: {}ms;\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "Calendar motion attach should emit css custom properties via `{needle}`."
        );
    }
}

#[test]
fn calendar_ui_components_entry_points_follow_layered_architecture_contract() {
    let components_lib_source = load_source("../../crates/ui/src/lib.rs");
    let components_css_source = load_source("../../crates/ui/src/css.rs");
    let components_root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-calendar\")]",
        "pub use ui_calendar as calendar;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            components_lib_source.contains(needle),
            "ui lib.rs entry contract should contain `{needle}`."
        );
    }

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen::",
        "pub use leptos::web_sys",
    ] {
        assert!(
            !components_lib_source.contains(forbidden),
            "ui public API should not expose platform-specific DOM/web-sys detail `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-calendar\")]",
        "out.push_str(crate::calendar::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            components_css_source.contains(needle),
            "ui css.rs feature-gated aggregation contract should contain `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            components_root_source.contains(needle),
            "UiRoot root.rs should centralize theme/css/i18n injection via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight primitive should expose shared style/motion contract `{needle}`."
        );
    }

    for forbidden in ["ui_chart::", "ui_calendar::", "ui_tabs::"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should not embed component-specific business semantic `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden_file in [
        "../../crates/ui/src/overlay_open.rs",
        "../../crates/ui/src/presence.rs",
        "../../crates/ui/src/a11y.rs",
    ] {
        let path = manifest_dir.join(forbidden_file);
        assert!(
            !path.exists(),
            "ui fixed entry contract forbids file `{forbidden_file}`."
        );
    }
}

#[test]
fn calendar_tree_shaking_feature_pruning_is_gated_in_ui_components() {
    let ui_components_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib_source = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css_source = load_source("../../crates/ui/src/css.rs");
    let tree_shaking_script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget_source = load_source("../../scripts/tree_shaking_budget.env");
    let check2_source = load_source("check2.md");

    for needle in [
        "component-calendar = [\"dep:ui-calendar\"]",
        "ui-calendar = { path = \"../../components/calendar\", optional = true }",
        "all-components = [",
        "\"component-calendar\",",
        "default = [\"inject-css\", \"all-components\"]",
    ] {
        assert!(
            ui_components_cargo_source.contains(needle),
            "ui Cargo feature tree should contain `{needle}`."
        );
    }

    let calendar_export_signature =
        "#[cfg(feature = \"component-calendar\")]\npub use ui_calendar as calendar;";
    assert!(
        ui_components_lib_source.contains(calendar_export_signature),
        "ui lib.rs should gate calendar export by `component-calendar`."
    );
    assert_eq!(
        ui_components_lib_source
            .matches("pub use ui_calendar as calendar;")
            .count(),
        1,
        "calendar module export should stay single and feature-gated in ui lib.rs."
    );

    let calendar_css_signature =
        "#[cfg(feature = \"component-calendar\")]\n    out.push_str(crate::calendar::styles::CSS);";
    assert!(
        ui_components_css_source.contains(calendar_css_signature),
        "ui css.rs should gate calendar CSS aggregation by `component-calendar`."
    );
    assert_eq!(
        ui_components_css_source
            .matches("out.push_str(crate::calendar::styles::CSS);")
            .count(),
        1,
        "calendar CSS aggregation should stay single and feature-gated in ui css.rs."
    );

    for forbidden in [
        "pub use ui_calendar as calendar;\npub use ui_calendar as calendar;",
        "out.push_str(crate::calendar::styles::CSS);\nout.push_str(crate::calendar::styles::CSS);",
    ] {
        assert!(
            !ui_components_lib_source.contains(forbidden)
                && !ui_components_css_source.contains(forbidden),
            "ui tree-shaking path should avoid duplicated unconditional registration `{forbidden}`."
        );
    }

    for needle in [
        "cargo tree -e features -i ui -p ui --no-default-features",
        "if grep -q 'all-components' <<<\"$",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
    ] {
        assert!(
            tree_shaking_script_source.contains(needle),
            "tree-shaking CI script should enforce feature pruning contract via `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget_source.contains(needle),
            "tree-shaking budget baseline should define `{needle}`."
        );
    }

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "cargo tree -e features -p ui --no-default-features --features component-calendar,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "calendar_tree_shaking_feature_pruning_is_gated_in_ui_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should lock tree-shaking gate marker `{needle}`."
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
        "title=\"Hello World\"",
        "title=\"Default + Outside Days\"",
        "title=\"Monday First + Strong Tone\"",
        "title=\"State Matrix (Outside Days / Weekday / Tone)\"",
        "title=\"Controlled vs Uncontrolled (selected_day axis)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
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
        "const CALENDAR_WORKBENCH_STORAGE_KEY: &str = \"docs:calendar:workbench:v1\";",
        "struct CalendarWorkbenchState {",
        "fn load_calendar_workbench_state() -> Option<CalendarWorkbenchState>",
        "fn save_calendar_workbench_state(state: CalendarWorkbenchState)",
        "fn clear_calendar_workbench_state()",
        "let hello_world_code = Signal::derive(move || {",
        "let persisted_workbench_state = load_calendar_workbench_state();",
        "let has_persisted_workbench_state = persisted_workbench_state.is_some();",
        "let initial_workbench_state = persisted_workbench_state.unwrap_or_default();",
        "let calendar_imports =",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "let (controlled_selected_day, set_controlled_selected_day) = signal(Some(12_u8));",
        "let on_controlled_selected_day_change =",
        "save_calendar_workbench_state(state);",
        "clear_calendar_workbench_state();",
        "<Playground",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "code_imports=calendar_imports.clone()",
        "<Calendar year=2026 month=3 />",
        "<Playground title=\"Default + Outside Days\" code_signal=code>",
        "year=2026",
        "month=1",
        "selected_day=Some(6)",
        "tone=CalendarTone::Default",
        "first_weekday=CalendarFirstWeekday::Sunday",
        "is_show_outside_days=true",
        "<Playground title=\"Monday First + Strong Tone\" code_signal=states_code>",
        "month=2",
        "selected_day=Some(14)",
        "tone=CalendarTone::Strong",
        "first_weekday=CalendarFirstWeekday::Monday",
        "is_show_outside_days=false",
        "class_name=\"docs-calendar-custom\".to_string()",
        "<Playground",
        "title=\"State Matrix (Outside Days / Weekday / Tone)\"",
        "code_signal=state_matrix_code",
        "data-slot=\"calendar-state-matrix\"",
        "<Playground",
        "title=\"Controlled vs Uncontrolled (selected_day axis)\"",
        "code_signal=controlled_uncontrolled_code",
        "data-slot=\"calendar-controlled-uncontrolled\"",
        "default_selected_day=Some(12)",
        "selected_day=controlled_selected_day.get()",
        "on_selected_day_change=Some(on_controlled_selected_day_change)",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_signal=stream_snapshot_code",
        "data-slot=\"calendar-streaming-snapshot\"",
        "// Snapshot: render final calendar result in one shot.",
        "// Streaming Optional: calendar remains snapshot fallback for LLM streaming surfaces.",
        "let (interactive_month, set_interactive_month) = signal(initial_workbench_state.month);",
        "data-slot=\"calendar-interactive-controls\"",
        "data-action=\"prev-month\"",
        "data-action=\"next-month\"",
        "data-action=\"toggle-weekday\"",
        "data-action=\"toggle-tone\"",
        "data-action=\"toggle-outside-days\"",
        "data-action=\"clear-selection\"",
        "Switch checked=workbench_persist_state set_checked=set_workbench_persist_state",
        "\"Persist workbench state\"",
        "data-slot=\"calendar-interactive-summary\"",
        "persist={}",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "class_name=\"docs-calendar-interactive\".to_string()",
        "data-slot=\"calendar-source-first\"",
        "class_name=\"docs-calendar-source-copy\".to_string()",
        "use ui::{Calendar, CalendarFirstWeekday, CalendarTone};",
        "compose_copy_ready_code",
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
fn calendar_documentation_entry_is_newcomer_friendly_and_progressive() {
    let readme_source = load_source("README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "# Calendar",
        "## 快速开始（先用起来）",
        "最小可用示例（Hello World）",
        "<Calendar",
        "year=2026",
        "month=3",
        "#/components/calendar",
        "## 常见用法（基础）",
        "Controlled vs Uncontrolled",
        "default_selected_day",
        "selected_day + on_selected_day_change",
        "## 进阶（需要时再看）",
        "### Config 区",
        "### Source-first / Copy-Paste Ready",
        "### WASM 调试入口（feature gate）",
    ] {
        assert!(
            readme_source.contains(needle),
            "calendar README newcomer path should contain `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn calendar() -> AnyView",
        "title=\"Calendar\"",
        "title=\"Hello World\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "data-slot=\"calendar-source-first\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "calendar docs entry should include `{needle}`."
        );
    }

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "calendar_documentation_entry_is_newcomer_friendly_and_progressive",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should include documentation-product marker `{needle}`."
        );
    }
}

#[test]
fn calendar_docs_parameter_matrix_syncs_api_names_and_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/calendar.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "data-slot=\"calendar-parameter-matrix\"",
        "data-slot=\"calendar-parameter-matrix-grid\"",
        "data-prop=\"tone\"",
        "data-prop=\"first_weekday\"",
        "data-prop=\"is_show_outside_days\"",
        "data-prop=\"show_outside_days\"",
        "data-prop=\"selected-day-axis\"",
        "data-prop=\"aria-label\"",
        "CalendarTone::Default",
        "CalendarFirstWeekday::Sunday",
        "normalize_is_show_outside_days(is_show_outside_days, show_outside_days)",
        "normalize_selected_day_axis(selected_day, default_selected_day, year, normalize_month(month))",
        "DEFAULT_ARIA_LABEL",
        "\"\\\"Calendar\\\"\"",
        "data-slot=\"calendar-state-matrix-note\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "calendar docs parameter matrix should contain `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] tone: CalendarTone,",
        "#[prop(optional)] first_weekday: CalendarFirstWeekday,",
        "#[prop(optional)] is_show_outside_days: Option<bool>,",
        "#[prop(optional)] show_outside_days: Option<bool>,",
        "#[prop(default = None)] selected_day: Option<u8>,",
        "#[prop(optional)] default_selected_day: Option<u8>,",
        "#[prop(default = None)] on_selected_day_change: Option<Callback<Option<u8>>>",
        "#[prop(optional, into)] aria_label: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "calendar view api should include docs-synced prop marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CalendarTone",
        "Default,",
        "pub enum CalendarFirstWeekday",
        "Sunday,",
        "pub fn normalize_is_show_outside_days(",
        "is_show_outside_days.or(show_outside_days).unwrap_or(false)",
        "pub fn normalize_selected_day_axis(",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Calendar\";",
    ] {
        assert!(
            primitives_source.contains(needle),
            "calendar primitives should include default/source marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "calendar_docs_parameter_matrix_syncs_api_names_and_logic_defaults",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should include docs-sync marker `{needle}`."
        );
    }
}

#[test]
fn calendar_docs_interactive_playground_supports_live_state_preview_and_repeatable_paths() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_calendar_contract.spec.mjs");
    let check2_source = load_source("check2.md");

    for needle in [
        "title=\"Interactive Playground (State + Source Markers)\"",
        "data-slot=\"calendar-interactive-controls\"",
        "data-slot=\"calendar-interactive-summary\"",
        "data-action=\"prev-month\"",
        "data-action=\"next-month\"",
        "data-action=\"toggle-weekday\"",
        "data-action=\"toggle-tone\"",
        "data-action=\"toggle-outside-days\"",
        "data-action=\"clear-selection\"",
        "selected_day=interactive_selected_day.get()",
        "on_selected_day_change=Some(Callback::new(move |next| {",
        "set_interactive_selected_day.set(next);",
        "format!(",
        "\"month={} selected_day={:?} weekday={} tone={} outside_days={} persist={}\",",
    ] {
        assert!(
            docs_source.contains(needle),
            "calendar interactive playground contract should include `{needle}`."
        );
    }

    for needle in [
        "test(\"docs-app calendar key flow is repeatable with semantic contract breakpoints\"",
        "await page.locator('[data-action=\"next-month\"]').click();",
        "await page.locator('[data-action=\"clear-selection\"]').click();",
        "await page.reload();",
        "await expect(interactiveAfterReload).toHaveAttribute(\"data-ui-state\", \"selected\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "calendar interactive e2e should include repeatable path marker `{needle}`."
        );
    }

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "calendar_docs_interactive_playground_supports_live_state_preview_and_repeatable_paths",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should include interactive-playground marker `{needle}`."
        );
    }
}

#[test]
fn calendar_motion_contract_exposes_sanitization_and_style_vars() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub struct CalendarMotion",
        "pub struct EffectiveCalendarMotion",
        "pub spring: ui_motion::spring::SpringConfig",
        "pub fn sanitize_motion(motion: CalendarMotion) -> CalendarMotion",
        "pub fn resolve_effective_motion(",
        "pub fn source_attr(motion: CalendarMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: CalendarMotion) -> String",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-calendar-motion-duration",
        "--ui-calendar-motion-stiffness",
        "--ui-calendar-motion-damping",
        "--ui-calendar-motion-mass",
        "--ui-calendar-motion-precision",
        "--ui-calendar-motion-reduced",
    ] {
        assert!(
            source.contains(needle),
            "Calendar motion should include `{needle}`."
        );
    }
}

#[test]
fn calendar_view_macro_complexity_is_split_by_semantic_fragments() {
    let source = load_source("src/view.rs");

    for needle in [
        "fn render_header(title: String) -> impl IntoView",
        "fn render_weekday(index: usize, label: String) -> impl IntoView",
        "fn render_weekdays(weekdays: Vec<String>) -> impl IntoView",
        "fn compose_day_class(cell: logic::CalendarGridCell) -> String",
        "struct CalendarDayRenderInput",
        "fn render_day(input: CalendarDayRenderInput) -> AnyView",
        "fn render_empty_day(index: usize) -> AnyView",
        "{render_header(title)}",
        "{render_weekdays(",
        "render_day(CalendarDayRenderInput {",
    ] {
        assert!(
            source.contains(needle),
            "Calendar view complexity should stay split by semantic helper fragments via `{needle}`."
        );
    }

    let component_count = source.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "Calendar view should stay function-split and expose only one component entry point."
    );
}

#[test]
fn calendar_static_fragments_are_centralized_and_not_scattered() {
    let view_source = load_source("src/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/calendar.rs");

    for needle in [
        "logic::month_title(year, normalized_month)",
        "logic::weekday_labels(first_weekday)",
        "fn render_header(title: String) -> impl IntoView",
        "fn render_weekday(index: usize, label: String) -> impl IntoView",
        "fn render_empty_day(index: usize) -> AnyView",
    ] {
        assert!(
            view_source.contains(needle),
            "Calendar static fragment path should stay centralized; missing `{needle}`."
        );
    }

    for forbidden in ["<svg", "inner_html", "\"January\"", "\"February\""] {
        assert!(
            !view_source.contains(forbidden),
            "Calendar view should avoid scattering complex static fragments: found `{forbidden}`."
        );
    }

    assert_eq!(
        view_source
            .matches("data-slot=\"calendar-day-empty\"")
            .count(),
        1,
        "Calendar empty-day static template should stay centralized in a single helper."
    );

    for needle in [
        "pub fn weekday_labels(first_weekday: CalendarFirstWeekday) -> [&'static str; 7]",
        "pub fn month_name(month: u8) -> &'static str",
        "pub fn month_title(year: i32, month: u8) -> String",
    ] {
        assert!(
            primitives_source.contains(needle),
            "Calendar static text should stay centralized in ui-state-primitives via `{needle}`."
        );
    }
}

#[test]
fn calendar_inner_html_contract_disallows_untrusted_html_injection() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "README.md",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            ".set_html(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Calendar should forbid unsafe html injection token `{forbidden}` in `{rel_path}`."
            );
        }
    }
}

#[test]
fn calendar_wasm_debug_contract_is_feature_gated_traceable_and_replayable() {
    let calendar_cargo_source = load_source("Cargo.toml");
    let ui_components_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("README.md");

    for needle in [
        "default = [\"web\"]",
        "wasm-debug = [\"dep:tracing\"]",
        "tracing = { version = \"0.1\", optional = true }",
        "#[cfg(feature = \"wasm-debug\")]",
        "mod debug_trace {",
    ] {
        assert!(
            calendar_cargo_source.contains(needle)
                || mod_source.contains(needle)
                || view_source.contains(needle),
            "calendar wasm-debug contract should include `{needle}`."
        );
    }

    for forbidden in ["mod debug;", "pub mod debug;", "pub use debug::"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "calendar wasm-debug internals must stay private: `{forbidden}`."
        );
    }

    for needle in [
        "calendar-wasm-debug = [\"component-calendar\", \"ui-calendar/wasm-debug\"]",
        "data-slot=\"calendar-debug\"",
        "data-action=\"replay-last-debug-event\"",
        "data-slot=\"calendar-debug-events\"",
        "data-slot=\"calendar-debug-event\"",
        "data-trace-id",
        "data-tick",
        "data-prev-selected-day",
        "data-next-selected-day",
        "data-prev-source",
        "data-next-source",
        "CalendarDebugInteraction::DayPress",
        "CalendarDebugInteraction::ReplayLast",
        "关键状态追踪：`trace_id/tick` + `prev_selected_day/next_selected_day` + `prev_source/next_source`",
        "关键交互回放：`data-action=\"replay-last-debug-event\"` 按钮重放最近一次交互链路",
    ] {
        assert!(
            ui_components_cargo_source.contains(needle)
                || view_source.contains(needle)
                || readme_source.contains(needle),
            "calendar wasm-debug contract should keep `{needle}`."
        );
    }
}

#[test]
fn calendar_engineering_capability_contract_uses_serde_tracing_and_runtime_agnostic_api() {
    let forms_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let docs_cargo_source = load_source("../../apps/docs-app/Cargo.toml");
    let calendar_cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/view.rs");
    let mod_source = load_source("src/mod.rs");

    for needle in [
        "const CALENDAR_WORKBENCH_STORAGE_VERSION: u8 = 1;",
        "#[cfg_attr(target_arch = \"wasm32\", derive(serde::Serialize, serde::Deserialize))]",
        "struct CalendarWorkbenchStorage {",
        "version: CALENDAR_WORKBENCH_STORAGE_VERSION,",
        "serde_json::to_string(&CalendarWorkbenchStorage {",
        "serde_json::from_str(raw).map_err(CalendarWorkbenchStorageError::Deserialize)?;",
        "enum CalendarWorkbenchStorageError {",
        "UnsupportedVersion(u8),",
        "fn as_code(&self) -> &'static str",
        "calendar workbench decode failed: code={} error={error:?}",
        "calendar workbench encode failed: code={} error={error:?}",
    ] {
        assert!(
            forms_source.contains(needle),
            "Calendar engineering serialization contract should contain `{needle}`."
        );
    }

    for needle in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "serde_json = \"1.0\"",
    ] {
        assert!(
            docs_cargo_source.contains(needle),
            "docs-app should include unified serde stack via `{needle}`."
        );
    }

    for needle in [
        "wasm-debug = [\"dep:tracing\"]",
        "tracing = { version = \"0.1\", optional = true }",
        "tracing::info_span!(",
        "target: \"ui.calendar\"",
        "calendar_interaction",
        "interaction = debug_trace::CalendarDebugInteraction::DayPress.as_attr()",
        "interaction = debug_trace::CalendarDebugInteraction::ReplayLast.as_attr()",
        "tracing::info!(",
        "calendar debug event recorded",
        "calendar replay event recorded",
    ] {
        assert!(
            calendar_cargo_source.contains(needle) || view_source.contains(needle),
            "Calendar tracing contract should include `{needle}`."
        );
    }

    for forbidden in ["tokio::", "async_std::", "tokio =", "async-std"] {
        assert!(
            !calendar_cargo_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "Calendar runtime boundary should not leak specific async runtime `{forbidden}`."
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
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            source.contains(needle),
            "calendar e2e should include `{needle}` semantic contract selector/wait."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !source.contains(forbidden),
            "calendar e2e should avoid fixed-time waits `{forbidden}` and rely on semantic readiness."
        );
    }
}

#[test]
fn calendar_e2e_key_flow_regression_is_repeatable_and_contract_breakpointed() {
    let source = load_source("../../e2e/tests/docs_app_calendar_contract.spec.mjs");
    let check2_source = load_source("check2.md");

    for needle in [
        "test(\"docs-app calendar key flow is repeatable with semantic contract breakpoints\"",
        "await page.goto(\"/#/components/calendar\");",
        "await expect(interactive).toHaveAttribute(\"data-ui-state\", \"selected\");",
        "await expect(interactive).toHaveAttribute(\"data-ui-source\", \"props-selected-day\");",
        "await firstPressableDay.focus();",
        "await expect(firstPressableDay).toBeFocused();",
        "await firstPressableDay.press(\"Enter\");",
        "await expect(interactive).toHaveAttribute(\"data-ui-action\", \"select-day\");",
        "await page.locator('[data-action=\"next-month\"]').click();",
        "await page.locator('[data-action=\"clear-selection\"]').click();",
        "await expect(interactive).toHaveAttribute(\"data-ui-state\", \"default\");",
        "await expect(interactive).toHaveAttribute(\"data-ui-source\", \"implicit-default\");",
        "await page.reload();",
        "await expect(interactiveAfterReload).toHaveAttribute(\"data-ui-state\", \"selected\");",
        "await expect(interactiveAfterReload).toHaveAttribute(\"data-ui-source\", \"props-selected-day\");",
    ] {
        assert!(
            source.contains(needle),
            "calendar repeatable key-flow regression should include semantic breakpoint `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "expect(page).toHaveScreenshot("] {
        assert!(
            !source.contains(forbidden),
            "calendar key-flow regression should not degrade to snapshot-only breakpoint `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "calendar_e2e_key_flow_regression_is_repeatable_and_contract_breakpointed",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should keep key-flow regression marker `{needle}`."
        );
    }
}

#[test]
fn calendar_semantics_and_perf_regression_cover_aria_data_focus_and_render_measurement() {
    let view_source = load_source("src/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_calendar_contract.spec.mjs");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "role=root.attrs.role",
        "aria-label=root.attrs.aria_label.clone()",
        "aria-selected=day_contract.attrs.aria_selected",
        "aria-disabled=day_contract.attrs.aria_disabled",
        "data-slot=\"calendar\"",
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-selected-day-source=move || selected_day_source.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "calendar semantics regression should keep aria/data marker `{needle}`."
        );
    }

    for needle in [
        "[data-slot=\"calendar-day\"][data-pressable=\"true\"]",
        "await firstPressableDay.focus();",
        "await expect(firstPressableDay).toBeFocused();",
        "await firstPressableDay.press(\"Enter\");",
        "await expect(interactive).toHaveAttribute(\"data-ui-action\", \"select-day\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "calendar e2e regression should cover focus/keyboard semantic flow `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "expect(page).toHaveScreenshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "calendar key-path regression should not rely on visual snapshot-only assertion `{forbidden}`."
        );
    }

    for needle in [
        "\"calendar\" => UiPerfBudget {",
        "max_mount_ms: 32.0",
        "max_update_ms: Some(10.0)",
        "max_heap_kb: Some(576.0)",
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
    ] {
        assert!(
            shell_source.contains(needle) || probe_source.contains(needle),
            "calendar performance regression should keep render measurement evidence `{needle}`."
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "toBeFocused",
        "render_count",
        "mount/update/heap 预算测量",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar check2 should keep semantics/perf gate marker `{needle}`."
        );
    }
}

#[test]
fn calendar_performance_governance_budget_is_defined_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "\"calendar\" => UiPerfBudget {",
        "max_mount_ms: 32.0",
        "max_update_ms: Some(10.0)",
        "max_heap_kb: Some(576.0)",
        "let perf_budget = component_page_perf_budget(slug);",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "Calendar performance governance should keep docs budget/probe token `{needle}`."
        );
    }

    for needle in [
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            probe_source.contains(needle),
            "UiPerfProbe should expose blocking budget marker `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\")",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should assert performance regression marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-calendar calendar_performance_governance_budget_is_defined_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "Performance check script should gate calendar perf governance via `{script_needle}`."
    );

    for needle in ["render_count", "替换当前 mount-only 等价证据"] {
        assert!(
            todo_source.contains(needle),
            "Performance governance should keep render_count follow-up marker `{needle}`."
        );
    }
}

#[test]
fn calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
{
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/calendar.rbi");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Calendar\"",
        "crate = \"ui-calendar\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "calendar manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Calendar(",
        "selected_day: Option<u8>",
        "default_selected_day: Option<u8>",
        "on_selected_day_change: Option<leptos::prelude::Callback<Option<u8>>>",
        "on_day_press: Option<leptos::prelude::Callback<u8>>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "calendar RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
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
            "calendar should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Calendar` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "calendar/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn calendar_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let marker = "cargo test -p ui-calendar calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn calendar_check2_marks_component_governance_complete() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] `ui-theme` 定义",
        "- [x] `ui` 定义",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "### 8. 合并前门禁死命令（最终执行）",
        "- `cargo fmt --all -- --check`",
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
fn calendar_check2_tracks_file_placement_discipline_completion() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "通过依据（N/A）：`Calendar` 当前定位为月视图基础组件",
        "复杂组件的 Builder 基线由 `components/button/src/spec.rs` 承担（`ButtonSpec::new()...render()`）",
        "`debug.rs` 与 `protocol.rs` 已移除并回收实现到 `view.rs` 的 `#[cfg(feature = \"wasm-debug\")] mod debug_trace` 私有模块",
    ] {
        assert!(
            check2_source.contains(needle),
            "Calendar check2 file-placement discipline marker should include `{needle}`."
        );
    }
}

#[test]
fn calendar_context_compression_manifest_and_rbi_are_present_and_consistent_locally() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for required_file in ["src/Component.toml", "src/calendar.rbi"] {
        assert!(
            manifest_dir.join(required_file).exists(),
            "Calendar context-compression file should exist: `{required_file}`."
        );
    }

    let component_manifest = load_source("src/Component.toml");
    let component_rbi = load_source("src/calendar.rbi");
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");

    for required in [
        "schema_version = \"1\"",
        "name = \"Calendar\"",
        "crate = \"ui-calendar\"",
        "name = \"year\"",
        "name = \"month\"",
        "name = \"tone\"",
        "name = \"first_weekday\"",
        "name = \"is_show_outside_days\"",
        "name = \"show_outside_days\"",
        "name = \"selected_day\"",
        "name = \"default_selected_day\"",
        "name = \"on_selected_day_change\"",
        "name = \"on_day_press\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"ui-headless\"",
        "name = \"ui-state-primitives\"",
        "name = \"ui-motion\"",
        "name = \"ui-theme\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Calendar Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub type CalendarFirstWeekday = ui_state_primitives::calendar::CalendarFirstWeekday;",
        "pub type CalendarTone = ui_state_primitives::calendar::CalendarTone;",
        "pub struct CalendarMotion",
        "pub struct CalendarAgentContract",
        "pub fn Calendar(",
        "selected_day: Option<u8>",
        "default_selected_day: Option<u8>",
        "on_selected_day_change: Option<leptos::prelude::Callback<Option<u8>>>",
        "on_day_press: Option<leptos::prelude::Callback<u8>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "calendar.rbi should keep signature-projection marker `{required}`."
        );
    }

    for required in [
        "pub fn Calendar(",
        "year: i32,",
        "month: u8,",
        "#[prop(optional)] tone: CalendarTone,",
        "#[prop(optional)] first_weekday: CalendarFirstWeekday,",
        "#[prop(optional)] is_show_outside_days: Option<bool>,",
        "#[prop(optional)] show_outside_days: Option<bool>,",
        "#[prop(default = None)] selected_day: Option<u8>,",
        "#[prop(optional)] default_selected_day: Option<u8>,",
        "#[prop(default = None)] on_selected_day_change: Option<Callback<Option<u8>>>",
        "#[prop(default = None)] on_day_press: Option<Callback<u8>>",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional)] motion: CalendarMotion,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            view_source.contains(required),
            "Calendar view API should keep manifest/RBI sync marker `{required}`."
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/calendar/src/Component.toml",
        "components/calendar/src/calendar.rbi",
        "calendar_context_compression_manifest_and_rbi_are_present_and_consistent_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "calendar check2 should keep context-compression marker `{required}`."
        );
    }
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
