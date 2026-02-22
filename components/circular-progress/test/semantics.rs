fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "check2" => include_str!("../check2.md"),
        "check2.md" => include_str!("../check2.md"),
        "src/Component.toml" => include_str!("../src/Component.toml"),
        "src/circular_progress.rbi" => include_str!("../src/circular_progress.rbi"),
        "src/mod.rs" => include_str!("../src/mod.rs"),
        "src/logic.rs" => include_str!("../src/logic.rs"),
        "src/view.rs" => include_str!("../src/view.rs"),
        "src/styles.rs" => include_str!("../src/styles.rs"),
        "docs_display" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn circular_progress_module_keeps_public_surface_stable() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
        "#[cfg(test)]",
        "#[path = \"../test/semantics.rs\"]",
    ] {
        assert!(
            module.contains(required),
            "CircularProgress module should keep assembly boundary marker `{required}`.",
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "CircularProgress internals should stay private: `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_component_layer_assembles_primitives_headless_and_theme_consumption() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for needle in [
        "pub use ui_state_primitives::circular_progress::{",
        "CircularProgressState",
        "CircularProgressStateInput",
        "resolve_state",
        "resolve_aria_label",
    ] {
        assert!(
            logic.contains(needle),
            "CircularProgress logic should consume state primitives via `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view.contains(needle),
            "CircularProgress view should mount headless semantics via `{needle}`.",
        );
    }

    assert!(
        styles.contains("var(--ui-"),
        "CircularProgress styles should stay token-first and consume `--ui-*` variables.",
    );
}

#[test]
fn circular_progress_public_layer_does_not_expose_platform_dom_details() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    for forbidden in ["web_sys::", "web-sys", "HtmlElement", "NodeRef", "JsValue"] {
        assert!(
            !module.contains(forbidden),
            "CircularProgress module should not expose platform detail `{forbidden}`.",
        );
        assert!(
            !logic.contains(forbidden),
            "CircularProgress logic should not expose platform detail `{forbidden}`.",
        );
        assert!(
            !view.contains(forbidden),
            "CircularProgress view should not expose platform detail `{forbidden}`.",
        );
    }
}

#[test]
fn circular_progress_check2_marks_ui_components_boundary_complete() {
    let check2 = load_source("check2");
    for needle in [
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/circular-progress/test/semantics.rs",
    ] {
        assert!(
            check2.contains(needle),
            "CircularProgress check2 should keep ui marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_component_has_no_controlled_or_uncontrolled_state_axis() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional)] on_open_change:",
        "on_value_change",
        "default_value",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should keep controlled/uncontrolled axis as N/A; found `{forbidden}`."
        );
    }
}

#[test]
fn circular_progress_defaults_are_resolved_in_logic_only() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should own default resolution via `{required}`.",
        );
    }

    for required in [
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "default_aria_label: common.loading_aria_label.as_ref(),",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should consume logic-derived defaults via `{required}`.",
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(lang)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress view should not do local default fallback; found `{forbidden}`."
        );
    }

    assert!(
        check2.contains("默认值单一来源"),
        "CircularProgress checklist should keep the single default source contract entry.",
    );
}

#[test]
fn circular_progress_state_normalization_flows_from_logic_to_view_and_styles() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "let state = resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should own normalized state derivation via `{required}`.",
        );
    }

    for required in [
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should only mount normalized state via `{required}`.",
        );
    }

    for forbidden in [
        "logic::resolve_state(CircularProgressStateInput {",
        "logic::normalize_optional_text(",
        "logic::resolve_aria_label(",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress view must not rebuild state machine logic: `{forbidden}`.",
        );
    }

    for required in [
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress[data-label-source=\"custom\"]",
    ] {
        assert!(
            styles.contains(required),
            "CircularProgress styles should consume state markers via `{required}`.",
        );
    }

    assert!(
        check2.contains("状态归一化集中"),
        "CircularProgress checklist should keep the centralized state-normalization contract entry.",
    );
}

#[test]
fn circular_progress_has_no_discrete_mutually_exclusive_state_axis() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for forbidden in [
        "Option<bool>",
        "#[prop(optional)] variant:",
        "#[prop(optional, into)] variant:",
        "#[prop(optional)] mode:",
        "#[prop(optional, into)] mode:",
        "#[prop(optional)] status:",
        "#[prop(optional, into)] status:",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should keep discrete state axis as N/A; found `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "ui-circular-progress--state-indeterminate",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "CircularProgress should preserve fixed state marker `{required}`.",
        );
    }

    assert!(
        check2.contains("离散状态必须类型约束"),
        "CircularProgress checklist should keep the discrete-state type contract entry.",
    );
}

#[test]
fn circular_progress_consumes_state_primitives_without_business_store_coupling() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub use ui_state_primitives::circular_progress::{",
        "CircularProgressState",
        "CircularProgressStateInput",
        "resolve_state",
        "resolve_aria_label",
        "normalize_optional_text",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should consume ui-state-primitives contract `{required}`.",
        );
    }

    {
        let required = "logic::resolve_component_contract(CircularProgressLogicInput {";
        assert!(
            view.contains(required),
            "CircularProgress view should consume logic output via `{required}`.",
        );
    }

    for forbidden in [
        "Signal<",
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
        "Store<",
        "store::",
        "redux",
        "zustand",
        "mobx",
        "Context<",
        "Arc<Mutex",
        "tokio::sync",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "CircularProgress should not bind business/global store type `{forbidden}`."
        );
    }

    assert!(
        check2.contains("状态原语来源正确"),
        "CircularProgress checklist should keep the state-primitives-source contract entry.",
    );
}

#[test]
fn circular_progress_has_no_async_loading_error_retry_contract() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for forbidden in [
        "#[prop(optional)] is_loading:",
        "#[prop(optional, into)] is_loading:",
        "is_loading",
        "is_disabled",
        "on_retry",
        "retry",
        "error",
        "aria-busy",
        "aria_busy",
        "use_async_action",
        "tokio::spawn",
        "spawn_local",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not expose async loading/error contract `{forbidden}`."
        );
    }

    for required in [
        "pub fn CircularProgress(",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "data-state=semantics.attrs.data_state",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress should keep display-only semantics marker `{required}`.",
        );
    }

    assert!(
        check2.contains("如果无异步相关"),
        "CircularProgress checklist should keep the async-contract entry.",
    );
}

#[test]
fn circular_progress_dx_hello_world_is_minimal_and_does_not_require_internal_state_plumbing() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let docs = load_source("docs_display");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(into)] state:",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress should not require internal state prop `{forbidden}` for DX path."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "let hello_world_code = Signal::derive(move || r#\"<CircularProgress />\"#.to_string());",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<CircularProgress />",
    ] {
        assert!(
            section.contains(required),
            "CircularProgress docs should provide minimal hello-world path `{required}`.",
        );
    }

    assert!(
        check2.contains("API 易用性验收标准（DX Paradox）"),
        "CircularProgress checklist should keep DX paradox contract entry.",
    );
}

#[test]
fn circular_progress_is_not_composite_parent_item_api_surface() {
    let view = load_source("view");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "#[prop(optional)] children:",
        "#[prop(optional, into)] children:",
        "#[prop(optional)] items:",
        "#[prop(optional, into)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "ItemSpec",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress should not expose composite parent/item API token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for required in [
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<CircularProgress />",
    ] {
        assert!(
            section.contains(required),
            "CircularProgress docs should keep single-node path marker `{required}`.",
        );
    }

    for forbidden in [
        "labels=",
        "titles=",
        "panels=",
        "children=",
        "items=",
        "ItemSpec",
        "<Parent>",
        "<Item",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs section should avoid composite conventions `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("组合型组件主 API 必须“显示优于约定”"),
        "CircularProgress checklist should keep composite-parent-item contract entry.",
    );
}

#[test]
fn circular_progress_has_no_dragging_macro_micro_duality_loop() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "cancelAnimationFrame",
        "raf",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !styles.contains(forbidden),
            "CircularProgress should not contain drag macro/micro token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Dragging",
        "DragEnd",
        "drag",
        "pointermove",
        "requestAnimationFrame",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid drag macro/micro mention `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("宏观/微观双状态机（Macro/Micro Duality）"),
        "CircularProgress checklist should keep macro/micro duality contract entry.",
    );
}

#[test]
fn circular_progress_has_no_two_pass_measure_rectification_flow() {
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "getBoundingClientRect",
        "NodeRef",
        "Measure(",
        "Rectification(",
        "measure_rect",
        "layout_rect",
        "overlay",
        "popover",
        "tooltip",
        "menu",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not contain two-pass geometry token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "getBoundingClientRect",
        "NodeRef",
        "Measure(",
        "Rectification(",
        "tooltip",
        "popover",
        "menu",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid two-pass geometry token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("几何两段式渲染（Two-Pass Rendering）"),
        "CircularProgress checklist should keep two-pass rendering contract entry.",
    );
}

#[test]
fn circular_progress_has_no_registration_context_items_order_protocol() {
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "accordion",
        "tabs",
        "menu",
        "children",
        "items:",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not contain collection registration token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
        "children=",
        "items=",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid registration protocol token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("集合注册协议（Registration Protocol）"),
        "CircularProgress checklist should keep registration protocol contract entry.",
    );
}

#[test]
fn circular_progress_has_no_slot_projection_keepalive_notifyhidden_protocol() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot projection",
        "children",
        "items:",
        "pause_animation_on_hidden",
        "pause_polling_on_hidden",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !styles.contains(forbidden),
            "CircularProgress should not contain slot-projection token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "children=",
        "items=",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid slot-projection token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("插槽投影策略（Slot Projection）"),
        "CircularProgress checklist should keep slot-projection contract entry.",
    );
}

#[test]
fn circular_progress_has_no_env_streams_sampling_debounce_action_flow() {
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "ThemeChanged",
        "BreakpointChanged",
        "Action::BreakpointChanged",
        "on:resize",
        "window.onresize",
        "match_media",
        "debounce",
        "throttle",
        "IntersectionChanged",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not contain env-stream token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "ThemeChanged",
        "BreakpointChanged",
        "debounce",
        "throttle",
        "resize",
        "intersection",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid env-stream token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("环境订阅流（Env Streams）"),
        "CircularProgress checklist should keep env-streams contract entry.",
    );
}

#[test]
fn circular_progress_has_no_event_light_cone_context_bus_selector_flow() {
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "BulkSelect",
        "row_selection",
        "select_all",
        "prop drilling",
        "event light cone",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not contain event-light-cone token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "BulkSelect",
        "row_selection",
        "select_all",
        "prop drilling",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid event-light-cone token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("事件光锥（Event Light Cone）"),
        "CircularProgress checklist should keep event-light-cone contract entry.",
    );
}

#[test]
fn circular_progress_has_no_causality_bus_trace_id_propagation_flow() {
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "CausalityBus",
        "cause_id",
        "event_bus",
        "bus broadcast",
        "subscriber",
        "derived command",
        "command bus",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not contain causality-bus token `{forbidden}`."
        );
    }

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "CausalityBus",
        "event_bus",
        "bus broadcast",
        "subscriber",
        "derived command",
        "command bus",
    ] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid causality-bus token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("统一因果总线（Causality Bus）"),
        "CircularProgress checklist should keep causality-bus contract entry.",
    );
}

#[test]
fn circular_progress_has_a11y_i18n_l10n_contract_without_view_level_hardcoded_copy() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "default_aria_label: common.loading_aria_label.as_ref(),",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should mount a11y+i18n+l10n contract `{required}`.",
        );
    }

    for forbidden in ["aria-label=\"Loading\"", ">Loading<", "Loading</"] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress view should avoid hardcoded visible text `{forbidden}`.",
        );
    }

    for required in [
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
        "resolve_aria_label(input.aria_label, default_aria_label);",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should keep fallback chain marker `{required}`.",
        );
    }

    assert!(
        check2.contains("存在 A11y 实现、国际化与本地化实现"),
        "CircularProgress checklist should keep a11y+i18n+l10n contract entry.",
    );
}

#[test]
fn circular_progress_exposes_stable_observable_state_markers_for_selectors() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-custom-size=semantics.attrs.data_custom_size",
        "data-custom-thickness=semantics.attrs.data_custom_thickness",
        "data-custom-aria-label=semantics.attrs.data_custom_aria_label",
        "data-custom-class=semantics.attrs.data_custom_class",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress should expose stable state marker `{required}`."
        );
    }

    assert!(
        check2.contains("状态可观测、可检索、可验证"),
        "CircularProgress checklist should keep state observability contract entry.",
    );
}

#[test]
fn circular_progress_styles_rely_on_explicit_state_markers_not_dom_shape_guessing() {
    let styles = load_source("styles");
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        ".ui-circular-progress--state-indeterminate",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-motion=\"spin\"]",
        ".ui-circular-progress[data-size-source=\"custom\"]",
        ".ui-circular-progress[data-thickness-source=\"custom\"]",
        ".ui-circular-progress[data-label-source=\"custom\"]",
        ".ui-circular-progress[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "CircularProgress styles should use explicit semantic selector `{required}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        ":has(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "CircularProgress styles should avoid fragile DOM selector `{forbidden}`.",
        );
    }

    assert!(
        view.contains("style=style_vars"),
        "CircularProgress view should pass runtime style via `style=style_vars` only.",
    );

    for required in [
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should restrict runtime style to CSS vars `{required}`.",
        );
    }

    assert!(
        check2.contains("样式依赖显式状态（`data-*`/class）"),
        "CircularProgress checklist should keep style-explicit-state contract entry.",
    );
}

#[test]
fn circular_progress_semantic_tests_prioritize_contract_markers_over_visual_snapshots() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let self_source = include_str!("semantics.rs");

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should expose semantic contract marker `{required}`.",
        );
    }

    let snapshot_macro = ["assert", "_snapshot"].concat();
    let insta_prefix = ["insta", "::"].concat();
    let to_match_macro = ["to_match", "_snapshot"].concat();

    for forbidden in [&snapshot_macro, &insta_prefix, &to_match_macro] {
        assert!(
            !self_source.contains(forbidden.as_str()),
            "component semantics suite should not rely on snapshot token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("测试验证“语义契约”而不只验证视觉快照"),
        "CircularProgress checklist should keep semantic-contract-testing entry.",
    );
}

#[test]
fn circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks_locally()
 {
    let view = load_source("view");
    let local_semantics = include_str!("semantics.rs");
    let workspace_semantics =
        include_str!("../../../components/circular-progress/test/circular_progress_semantics.rs");
    let perf_script = include_str!("../../../scripts/check-ui-performance.sh");
    let check2 = load_source("check2");

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view.contains(required),
            "circular-progress semantic-priority contract should keep marker `{required}`.",
        );
    }

    for required in [
        "fn circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots()",
        "fn circular_progress_semantic_tests_prioritize_contract_markers_over_visual_snapshots()",
        "let snapshot_macro = [\"assert\", \"_snapshot\"].concat();",
    ] {
        assert!(
            local_semantics.contains(required) || workspace_semantics.contains(required),
            "circular-progress semantic-priority suite should keep marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`.",
    );

    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "CircularProgress checklist should keep semantic-test-priority entry.",
    );
}

#[test]
fn circular_progress_e2e_selector_stability_prefers_semantic_markers_and_settled_waits_locally() {
    let check2 = load_source("check2");
    let e2e_contract =
        include_str!("../../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");
    let e2e_script = include_str!(
        "../../../components/circular-progress/scripts/check-ui-e2e-circular-progress.sh"
    );

    for required in [
        "body:not(:has(#boot))",
        "[data-component=\"circular-progress\"]",
        "[data-slot=\"circular-progress\"]",
        "data-ui-schema=\"ui.circular-progress.agent-contract\"",
        "data-ui-schema-version=\"v1\"",
        "data-ui-state=\"indeterminate\"",
        "data-state=\"indeterminate\"",
        "data-motion=\"spin\"",
        "toHaveText(/fallback=snapshot/)",
    ] {
        assert!(
            e2e_contract.contains(required),
            "circular-progress e2e selector stability contract should keep marker `{required}`.",
        );
    }

    for forbidden in [
        "getByText(",
        "locator(\"div > div >",
        "nth-child(",
        "waitForTimeout(",
        "setTimeout(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_contract.contains(forbidden),
            "circular-progress e2e selector stability should avoid brittle/snapshot token `{forbidden}`.",
        );
    }

    for required in [
        "circular_progress_check2_documents_e2e_selector_and_stable_wait_rules",
        "circular_progress_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "circular_progress_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
    ] {
        assert!(
            e2e_script.contains(required),
            "circular-progress e2e script should gate `{required}`.",
        );
    }

    assert!(
        check2.contains("components/circular-progress/scripts/check-ui-e2e-circular-progress.sh"),
        "circular-progress checklist should reference e2e selector stability gate script.",
    );
}

#[test]
fn circular_progress_e2e_key_flow_regression_collection_is_repeatable_and_breakpoint_diagnosable_locally()
 {
    let check2 = load_source("check2");
    let e2e_contract =
        include_str!("../../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");
    let e2e_script = include_str!(
        "../../../components/circular-progress/scripts/check-ui-e2e-circular-progress.sh"
    );

    for required in [
        "docs-app circular-progress key flow regression uses semantic breakpoints for diagnosis",
        "test.step(\"open route reaches semantic ready breakpoint\"",
        "test.step(\"interaction keeps source markers diagnosable\"",
        "test.step(\"reopen/remount keeps settled breakpoint stable\"",
        "data-ui-action=\"render\"",
        "data-ui-source=\"state-primitives\"",
        "data-size-source=\"default\"",
        "data-size-source=\"custom\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_contract.contains(required),
            "circular-progress e2e key-flow contract should keep marker `{required}`.",
        );
    }

    for required in [
        "circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable",
        "key flow regression is repeatable and semantic-breakpoint diagnosable",
    ] {
        assert!(
            e2e_script.contains(required),
            "circular-progress e2e script should gate key-flow marker `{required}`.",
        );
    }

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "circular_progress_e2e_key_flow_regression_is_repeatable_and_breakpoint_diagnosable",
    ] {
        assert!(
            check2.contains(required),
            "circular-progress checklist should keep key-flow regression marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_component_files_keep_clear_responsibility_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal export boundary marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "pub mod motion",
    ] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not expose internal/runtime motion module `{forbidden}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "NodeRef",
        "getBoundingClientRect",
        "document()",
        "window()",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain DOM operation token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for forbidden in ["#[component]", "use ui_headless", "use leptos"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not mix rendering/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "view! {",
        "data-state=semantics.attrs.data_state",
        "role=semantics.attrs.role",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep render + headless mounting marker `{required}`.",
        );
    }

    for forbidden in ["@keyframes", ".ui-circular-progress {", "web_sys::"] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not own static CSS/DOM API token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("组件文件职责正确"),
        "CircularProgress checklist should keep component-file-responsibility entry.",
    );
}

#[test]
fn circular_progress_has_no_spec_rs_builder_schema_contract_surface() {
    let module = load_source("mod");
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "CircularProgressSpec",
    ] {
        assert!(
            !module.contains(forbidden) && !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should not expose spec.rs token `{forbidden}`."
        );
    }

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple circular-progress component should not add `src/spec.rs`.",
    );

    let section_start = docs
        .find("pub(super) fn circular_progress() -> AnyView {")
        .unwrap_or_else(|| panic!("display docs should contain circular_progress section"));
    let section_tail = &docs[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn spinner() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display docs should contain spinner section after circular_progress")
        });
    let section = &section_tail[..section_end_rel];

    for forbidden in ["Spec::new(", "CircularProgressSpec", "schema", "spec.rs"] {
        assert!(
            !section.contains(forbidden),
            "CircularProgress docs should avoid spec builder/schema token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("`spec.rs` 只用于少数复杂组件"),
        "CircularProgress checklist should keep spec-rs contract entry.",
    );
}

#[test]
fn circular_progress_hyper_structure_builder_spec_contract_is_not_applicable_for_simple_component_locally()
 {
    let module = load_source("mod");
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let check2 = load_source("check2");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        ".render()",
        "CircularProgressSpec",
    ] {
        assert!(
            !module.contains(forbidden)
                && !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !docs.contains(forbidden),
            "CircularProgress should not expose Hyper-Structure Builder token `{forbidden}`.",
        );
    }

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple circular-progress component should keep Hyper-Structure spec.rs as N/A.",
    );

    assert!(
        check2.contains("Hyper-Structure Builder（`spec.rs`）"),
        "CircularProgress checklist should keep Hyper-Structure Builder entry.",
    );
}

#[test]
fn circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent_locally() {
    let check2 = load_source("check2");
    let script = include_str!("../../../scripts/check-ui-component-files.sh");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/circular_progress.rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "circular_progress.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "CircularProgress context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"CircularProgress\"",
        "crate = \"ui-circular-progress\"",
        "name = \"aria_label\"",
        "name = \"size_px\"",
        "name = \"thickness_px\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn CircularProgress(",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "circular_progress.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_context_compression_manifest_and_rbi_are_present_and_consistent";
    assert!(
        script.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("上下文压缩协议（Manifest + RBI）"),
        "CircularProgress checklist should keep context-compression entry.",
    );
}

#[test]
fn circular_progress_agent_contract_is_schema_typed_and_machine_readable_locally() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/circular_progress.rbi");

    for typed_source in [
        "pub const CIRCULAR_PROGRESS_AGENT_SCHEMA: &str = \"ui.circular-progress.agent-contract\";",
        "pub enum CircularProgressAgentSchemaVersion",
        "pub enum CircularProgressAgentIntent",
        "pub enum CircularProgressAgentAction",
        "pub enum CircularProgressAgentState",
        "pub enum CircularProgressAgentSource",
        "pub struct CircularProgressAgentContract",
        "pub fn resolve_agent_contract(state: &CircularProgressState) -> CircularProgressAgentContract",
    ] {
        assert!(
            logic.contains(typed_source),
            "CircularProgress Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
        "data-ui-size-source=agent_contract.size_source",
        "data-ui-thickness-source=agent_contract.thickness_source",
        "data-ui-label-source=agent_contract.label_source",
        "data-ui-class-source=agent_contract.class_source",
    ] {
        assert!(
            view.contains(marker),
            "CircularProgress view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "CIRCULAR_PROGRESS_AGENT_SCHEMA",
        "CircularProgressAgentContract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "Context compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "inner_html",
        "javascript:",
        "<script",
        "eval(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress Agent Contract path should avoid unsafe/free-form token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("语义标记统一升级为 Agent Contract（Schema 化）"),
        "CircularProgress checklist should keep Agent Contract governance entry.",
    );
}

#[test]
fn circular_progress_token_first_style_contract_flows_from_styles_to_css_aggregation_and_uiroot() {
    let styles = load_source("styles");
    let logic = load_source("logic");
    let css = include_str!("../../../crates/ui/src/css.rs");
    let root = include_str!("../../../crates/ui/src/root.rs");
    let check2 = load_source("check2");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "--ui-cp-size",
        "--ui-cp-thickness",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for required in [
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should limit runtime style payload to CSS vars `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
    ] {
        assert!(
            css.contains(required),
            "ui css aggregation should include circular-progress marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root.contains(required),
            "UiRoot CSS injection path should include `{required}`.",
        );
    }

    assert!(
        check2.contains("组件层遵循 token-first 静态样式契约"),
        "CircularProgress checklist should keep token-first static style contract entry.",
    );
}

#[test]
fn circular_progress_visual_desire_baseline_is_backed_by_docs_and_playwright_screenshots() {
    let check2 = load_source("check2");
    let baseline_page =
        include_str!("../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let docs_registry = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let visual_e2e = include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");

    for required in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "title=\"Default Theme Visual Baseline\"",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page.contains(required),
            "theme visual baseline docs page should keep marker `{required}`.",
        );
    }

    for required in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_registry.contains(required),
            "docs registry should keep theme baseline marker `{required}`.",
        );
    }

    for required in [
        "test(\"docs-app: theme visual baseline renders button/input/overlay\"",
        "test(\"docs-app: theme visual baseline screenshots\"",
        "\"docs-app-theme-visual-baseline-page.png\"",
        "\"docs-app-theme-visual-baseline-button.png\"",
        "\"docs-app-theme-visual-baseline-input.png\"",
        "\"docs-app-theme-visual-baseline-overlay.png\"",
    ] {
        assert!(
            visual_e2e.contains(required),
            "theme visual baseline playwright spec should keep marker `{required}`.",
        );
    }

    for required in [
        "# HeroUI 参数设计风格对齐策略",
        "### Non-Goals",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_strategy.contains(required),
            "HeroUI strategy should keep non-API-copy marker `{required}`.",
        );
    }

    assert!(
        check2.contains("默认主题美学质量达标（Visual Desire）"),
        "CircularProgress checklist should keep visual-desire contract entry.",
    );
}

#[test]
fn circular_progress_tree_shaking_contract_is_backed_by_feature_gates_and_budget_script() {
    let check2 = load_source("check2");
    let cargo = include_str!("../../../crates/ui/Cargo.toml");
    let lib = include_str!("../../../crates/ui/src/lib.rs");
    let css = include_str!("../../../crates/ui/src/css.rs");
    let tree_shaking_script = include_str!("../../../scripts/check-ui-tree-shaking.sh");
    let budget = include_str!("../../../scripts/tree_shaking_budget.env");
    let ci = include_str!("../../../.github/workflows/ci.yml");

    for required in [
        "[features]",
        "component-circular_progress = [\"dep:ui-circular-progress\"]",
        "web-demo-components = [",
        "\"component-circular_progress\"",
        "all-components = [",
    ] {
        assert!(
            cargo.contains(required),
            "ui Cargo should keep tree-shaking feature marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]\npub use ui_circular_progress as circular_progress;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]\nmod all_components {",
    ] {
        assert!(
            lib.contains(required),
            "ui lib should keep feature-gated export marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css.contains(required),
            "ui css should keep feature-gated aggregation marker `{required}`.",
        );
    }

    for required in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "if (( CURRENT_BYTES > MAX_BYTES )); then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should keep marker `{required}`.",
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget.contains(required),
            "tree-shaking budget file should keep marker `{required}`.",
        );
    }

    for required in [
        "- name: Tree Shaking Budget",
        "run: ./scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            ci.contains(required),
            "CI workflow should keep tree-shaking step marker `{required}`.",
        );
    }

    assert!(
        check2.contains("Tree Shaking 是一等能力"),
        "CircularProgress checklist should keep tree-shaking contract entry.",
    );
}

#[test]
fn circular_progress_tree_shaking_feature_pruning_contract_is_gated_in_lib_css_and_script_locally()
{
    let check2 = load_source("check2");
    let cargo = include_str!("../../../crates/ui/Cargo.toml");
    let lib = include_str!("../../../crates/ui/src/lib.rs");
    let css = include_str!("../../../crates/ui/src/css.rs");
    let tree_shaking_script = include_str!("../../../scripts/check-ui-tree-shaking.sh");

    for required in [
        "component-circular_progress = [\"dep:ui-circular-progress\"]",
        "#[cfg(feature = \"component-circular_progress\")]\npub use ui_circular_progress as circular_progress;",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
    ] {
        assert!(
            cargo.contains(required) || lib.contains(required) || css.contains(required),
            "tree-shaking feature-pruning should keep circular-progress marker `{required}`.",
        );
    }

    for required in [
        "CIRCULAR_PROGRESS_MIN_FEATURES=\"component-circular_progress,inject-css\"",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_contract_is_feature_gated_and_prunable_for_package_and_source_modes",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$CIRCULAR_PROGRESS_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$CIRCULAR_PROGRESS_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should keep circular-progress feature-pruning marker `{required}`.",
        );
    }

    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "CircularProgress checklist should mark tree-shaking feature-pruning item complete.",
    );
    assert!(
        check2.contains("components/circular-progress/test/circular_progress_semantics.rs::circular_progress_check2_marks_tree_shaking_feature_pruning_contract_complete"),
        "check2 should reference the ui tree-shaking feature-pruning regression test.",
    );
}

#[test]
fn circular_progress_type_system_and_semantic_markers_keep_machine_readable_contract() {
    let view = load_source("view");
    let logic = load_source("logic");
    let primitive = include_str!("../../../crates/ui-state-primitives/src/circular_progress.rs");
    let headless = include_str!("../../../crates/ui-headless/src/circular_progress.rs");
    let check2 = load_source("check2");
    let self_source = include_str!("semantics.rs");

    for required in [
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "pub struct CircularProgressStateInput",
        "pub struct CircularProgressState",
        "let state = resolve_state(CircularProgressStateInput {",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view.contains(required) || logic.contains(required) || primitive.contains(required),
            "CircularProgress should keep typed input or semantic marker `{required}`.",
        );
    }

    for required in [
        "pub struct CircularProgressAttrs",
        "pub data_state: &'static str",
        "pub data_size_source: &'static str",
        "pub data_thickness_source: &'static str",
        "pub data_label_source: &'static str",
        "pub data_class_source: &'static str",
    ] {
        assert!(
            headless.contains(required),
            "ui-headless contract should keep machine-readable typed attrs `{required}`.",
        );
    }

    for required in [
        "size_source_attr: if has_custom_size { \"custom\" } else { \"default\" }",
        "thickness_source_attr: if has_custom_thickness {",
        "label_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            primitive.contains(required),
            "ui-state-primitives should keep enumerable source normalization `{required}`.",
        );
    }

    for forbidden in [
        "Option<bool>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !primitive.contains(forbidden),
            "CircularProgress should avoid free-form/boolean-burst protocol `{forbidden}`.",
        );
    }

    for required in [
        "fn circular_progress_has_no_discrete_mutually_exclusive_state_axis()",
        "fn circular_progress_exposes_stable_observable_state_markers_for_selectors()",
    ] {
        assert!(
            self_source.contains(required),
            "component-local semantics suite should keep machine-readable contract marker `{required}`.",
        );
    }

    assert!(
        check2.contains("类型系统 + 语义标记共同提供机器可读状态"),
        "CircularProgress checklist should keep type-system+semantic-marker contract entry.",
    );
}

#[test]
fn circular_progress_focus_stack_contract_is_n_a_and_global_focus_manager_remains_in_headless() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let focus_trap = include_str!("../../../crates/ui-headless/src/focus_trap.rs");

    for forbidden in [
        "NodeRef",
        "use_focus_trap",
        "FocusTrapOptions",
        "FOCUS_MANAGER_STACK",
        "RestorePolicy::",
        "document.body",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress component layer should not implement overlay focus-stack concern `{forbidden}`.",
        );
    }

    for required in [
        "pub enum RestorePolicy",
        "Selector(String)",
        "FallbackTo(String)",
        "FOCUS_MANAGER_STACK",
        "fn focus_manager_push_trap(",
        "fn focus_manager_pop_trap(",
        "fn restore_focus_chain(",
        "if let Some(body) = document.body() {",
    ] {
        assert!(
            focus_trap.contains(required),
            "ui-headless global focus manager should keep marker `{required}`.",
        );
    }

    assert!(
        check2.contains("焦点全局栈（Focus Stack & GC）"),
        "CircularProgress checklist should keep focus-stack contract entry.",
    );
}

#[test]
fn circular_progress_escape_hatch_contract_is_n_a_without_foreign_zone_integration() {
    let view = load_source("view");
    let logic = load_source("logic");
    let module = load_source("mod");
    let check2 = load_source("check2");

    for forbidden in [
        "echarts",
        "ECharts",
        "mapbox",
        "leaflet",
        "amap",
        "google.maps",
        "ForeignZone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "JsValue",
        "wasm_bindgen",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !module.contains(forbidden),
            "CircularProgress should not embed imperative third-party escape-hatch concern `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional)] size_px: Option<f64>",
        "#[prop(optional)] thickness_px: Option<f64>",
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub use ui_state_primitives::circular_progress::{",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "CircularProgress API/logic should remain pure semantic contract marker `{required}`.",
        );
    }

    assert!(
        check2.contains("受控外交特区（Escape Hatches）"),
        "CircularProgress checklist should keep escape-hatch contract entry.",
    );
}

#[test]
fn circular_progress_hydration_discontinuity_contract_is_n_a_without_time_random_or_uuid_init() {
    let view = load_source("view");
    let logic = load_source("logic");
    let module = load_source("mod");
    let primitive = include_str!("../../../crates/ui-state-primitives/src/circular_progress.rs");
    let id_provider = include_str!("../../../crates/ui-headless/src/id_provider.rs");
    let check2 = load_source("check2");

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime",
        "UNIX_EPOCH",
        "Uuid",
        "uuid::",
        "rand::",
        "thread_rng",
        "random(",
        "Math.random",
        "js_sys::Date",
        "performance.now",
        "nanoid",
        "id_base",
        "use_ui_id_provider(",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !module.contains(forbidden)
                && !primitive.contains(forbidden),
            "CircularProgress should not initialize runtime id/time/random source `{forbidden}`.",
        );
    }

    for required in [
        "pub struct UiIdProvider",
        "pub fn new(seed: u64) -> Self",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn next_prefixed_id(self, prefix: &str) -> String",
    ] {
        assert!(
            id_provider.contains(required),
            "ui-headless id provider should keep deterministic seed marker `{required}`.",
        );
    }

    assert!(
        check2.contains("SSR 时空断裂治理（Hydration Discontinuity）"),
        "CircularProgress checklist should keep hydration-discontinuity contract entry.",
    );
}

#[test]
fn circular_progress_cross_platform_compile_contract_uses_explicit_cfg_and_feature_guards() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let platform_script = include_str!("../../../scripts/check-ui-platforms.sh");
    let headless_lib = include_str!("../../../crates/ui-headless/src/lib.rs");
    let motion_lib = include_str!("../../../crates/ui-motion/src/lib.rs");

    for required in [
        "[platform] compile-only: default native path",
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile-only: circular-progress native path",
        "cargo check -p ui --no-default-features --features component-circular_progress,inject-css",
        "[platform] compile-only: circular-progress wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-circular_progress,inject-css",
        "[platform] source guard: non-wasm circular-progress files must not reference web_sys",
    ] {
        assert!(
            platform_script.contains(required),
            "platform compile-only script should keep marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep explicit web/ssr mutual exclusion marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep explicit wasm/non-wasm stub marker `{required}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "web-sys",
        "window()",
        "document()",
        "HtmlElement",
        "NodeRef",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !styles.contains(forbidden)
                && !view.contains(forbidden),
            "CircularProgress non-wasm source path should avoid browser object token `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "CircularProgress checklist should keep cross-platform compile contract entry.",
    );
}

#[test]
fn circular_progress_ui_headless_web_ssr_mutex_contract_is_compile_error_guarded_and_platform_checked()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let headless_lib = include_str!("../../../crates/ui-headless/src/lib.rs");
    let platform_script = include_str!("../../../scripts/check-ui-platforms.sh");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep web/ssr mutex compile_error marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should keep ui-headless web/ssr mutex guard marker `{required}`.",
        );
    }

    assert!(
        view.contains(
            "use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};"
        ),
        "CircularProgress view should keep ui-headless dependency mount marker.",
    );

    assert!(
        check2.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "CircularProgress checklist should keep ui-headless web/ssr mutex entry.",
    );
}

#[test]
fn circular_progress_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let motion_lib = include_str!("../../../crates/ui-motion/src/lib.rs");
    let platform_script = include_str!("../../../scripts/check-ui-platforms.sh");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion non-wasm stub contract should keep marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ui-motion native path",
        "cargo check -p ui-motion",
        "[platform] compile-only: ui-motion wasm path",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "[platform] ui-motion non-wasm stub tests",
        "cargo test -p ui-motion --test non_wasm_stub",
        "[platform] ui-motion reduced-motion spring contract",
        "cargo test -p ui-motion --test spring",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should keep ui-motion compile/tooling guard marker `{required}`.",
        );
    }

    for forbidden in [
        "attach_motion(",
        "MotionOptions::default()",
        "web_sys::Element",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress component should not assume runtime motion handle `{forbidden}`.",
        );
    }

    assert!(
        check2.contains("`ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`）"),
        "CircularProgress checklist should keep ui-motion non-wasm stub entry.",
    );
}

#[test]
fn circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent_locally() {
    let check2 = load_source("check2");
    let styles = load_source("styles");
    let view = load_source("view");
    let logic = load_source("logic");
    let headless = include_str!("../../../crates/ui-headless/src/circular_progress.rs");
    let platform_script = include_str!("../../../scripts/check-ui-platforms.sh");

    for required in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
        ".ui-circular-progress[data-motion=\"spin\"]",
        "animation-duration: 1ms;",
        "animation-iteration-count: 1;",
    ] {
        assert!(
            styles.contains(required),
            "reduced-motion style contract should keep marker `{required}`.",
        );
    }

    for required in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: circular-progress wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-circular_progress,inject-css",
        "[platform] compile-only: ui-motion wasm path",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "[platform] circular-progress reduced-motion/ssr/wasm contract",
        "cargo test -p ui --test circular_progress_semantics circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            platform_script.contains(required),
            "platform script should keep reduced-motion/ssr/wasm marker `{required}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"ssr\")]",
        "#[cfg(feature = \"web\")]",
    ] {
        assert!(
            !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !headless.contains(forbidden),
            "semantic contract should not fork by runtime branch marker `{forbidden}`.",
        );
    }

    for required in [
        "role: \"progressbar\"",
        "aria_valuemin: \"0\"",
        "aria_valuemax: \"100\"",
        "data_state: \"indeterminate\"",
        "data_motion: \"spin\"",
    ] {
        assert!(
            headless.contains(required),
            "headless semantic contract should keep SSR/wasm parity marker `{required}`.",
        );
    }

    assert!(
        check2.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "CircularProgress checklist should keep reduced-motion/ssr/wasm entry.",
    );
}

#[test]
fn circular_progress_performance_governance_budget_is_defined_and_blocking_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let shell = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let pages = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let coverage = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script = include_str!("../../../scripts/check-ui-performance.sh");
    let todo = include_str!("../../../docs/plan/TODO.md");

    for required in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"circular-progress\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell.contains(required),
            "docs shell should keep performance budget marker `{required}`.",
        );
    }

    for required in [
        "\"CircularProgress\"",
        "\"circular-progress\"",
        "display::circular_progress",
    ] {
        assert!(
            pages.contains(required),
            "CircularProgress docs page should stay in coverage traversal via `{required}`.",
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe.contains(required),
            "UiPerfProbe should keep perf observability marker `{required}`.",
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage.contains(required),
            "docs coverage e2e should keep perf guard marker `{required}`.",
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress checklist should keep performance governance marker `{required}`.",
        );
    }

    assert!(
        todo.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "performance follow-up plan should keep render_count automation marker.",
    );

    for required in [
        "pub fn resolve_component_contract(",
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            view.contains(required) || logic.contains(required) || styles.contains(required),
            "state/render/style/motion attribution should keep marker `{required}`.",
        );
    }

    let needle = "cargo test -p ui --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script.contains(needle),
        "performance governance script should include `{needle}`.",
    );
}

#[test]
fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let local_semantics = include_str!("semantics.rs");
    let aggregated_semantics =
        include_str!("../../../components/circular-progress/test/circular_progress_semantics.rs");
    let focus_trap = include_str!("../../../crates/ui-headless/src/focus_trap.rs");
    let todo = include_str!("../../../docs/plan/TODO.md");
    let script = include_str!("../../../scripts/check-ui-performance.sh");

    for required_test in [
        "fn circular_progress_semantic_tests_prioritize_contract_markers_over_visual_snapshots()",
        "fn circular_progress_focus_stack_contract_is_n_a_and_global_focus_manager_remains_in_headless()",
        "fn circular_progress_performance_governance_budget_is_defined_and_blocking_locally()",
        "fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "component-local semantic/performance suite should include `{required_test}`.",
        );
    }

    for required_test in [
        "fn circular_progress_semantic_contract_test_suite_prioritizes_contract_markers_over_visual_snapshots()",
        "fn circular_progress_focus_stack_contract_is_not_applicable_while_global_focus_manager_stays_in_headless()",
        "fn circular_progress_performance_governance_budget_is_defined_and_blocking()",
        "fn circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            aggregated_semantics.contains(required_test),
            "aggregated semantic/performance suite should include `{required_test}`.",
        );
    }

    for marker in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view.contains(marker),
            "CircularProgress view should expose semantic marker `{marker}`.",
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap.contains(marker),
            "ui-headless global focus manager should keep focus-flow marker `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test circular_progress_semantics circular_progress_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            script.contains(script_needle),
            "performance check script should include `{script_needle}`.",
        );
    }

    assert!(
        check2.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "CircularProgress checklist should mark semantic/performance regression item complete.",
    );
}

#[test]
fn circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = include_str!("../../../scripts/check-ui-view-macro.sh");

    assert!(
        view.contains("view! {"),
        "CircularProgress should keep one explicit render block in view.rs.",
    );
    assert_eq!(
        view.matches("view! {").count(),
        1,
        "CircularProgress should keep one compact `view!` block for current simple layout.",
    );
    assert!(
        view.lines().count() <= 120,
        "CircularProgress view.rs should stay compact; split semantic subrenders only when complexity grows.",
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        "match children",
        "#[component]\nfn render_",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress view should avoid loop-heavy/macro-heavy rendering token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("`view!` 宏复杂度受控"),
        "CircularProgress checklist should keep the view-macro complexity contract entry.",
    );
}

#[test]
fn circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = include_str!("../../../scripts/check-ui-view-macro.sh");

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "CircularProgress should keep one public component boundary for current simple layout.",
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn circular_progress_",
        "pub fn render_",
        "fn render_",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress should avoid local component/render abstraction noise `{forbidden}`.",
        );
    }

    for required in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress should keep stable semantic markers `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("函数式拆分优先"),
        "CircularProgress checklist should keep the functional split contract entry.",
    );
}

#[test]
fn circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = include_str!("../../../scripts/check-ui-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "<svg",
        "<path",
        "let markdown",
        "let description_text",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress should avoid heavy static fragment construction token `{forbidden}`.",
        );
    }

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress should keep stable a11y/state markers `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_static_fragments_are_constantized_or_absent_for_simple_indicator_layout";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("静态片段常量化"),
        "CircularProgress checklist should keep the static fragment contract entry.",
    );
}

#[test]
fn circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally() {
    let check2 = load_source("check2");
    let script = include_str!("../../../scripts/check-ui-inner-html.sh");
    let docs_source = include_str!("../../../apps/docs-app/src/pages/components/pages/display.rs");

    for (rel_path, source) in [
        ("mod", load_source("mod")),
        ("logic", load_source("logic")),
        ("styles", load_source("styles")),
        ("view", load_source("view")),
    ] {
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
                "CircularProgress `{rel_path}` should not contain raw-html injection token `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "CircularProgress docs should not contain raw-html injection token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("`inner_html` 使用约束"),
        "CircularProgress checklist should keep the inner-html contract entry.",
    );
}

#[test]
fn circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let cargo = include_str!("../../../crates/ui/Cargo.toml");
    let crate_root = include_str!("../../../crates/ui/src/lib.rs");
    let docs_app = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let trace = include_str!("../../../crates/ui-headless/src/trace.rs");
    let script = include_str!("../../../scripts/check-ui-wasm-debug.sh");

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo.contains(required),
            "ui Cargo should keep shared wasm-debug feature marker `{required}`.",
        );
    }

    for forbidden in [
        "circular-progress-wasm-debug",
        "circular_progress-wasm-debug",
        "component-circular_progress-wasm-debug",
    ] {
        assert!(
            !cargo.contains(forbidden),
            "CircularProgress should not expose local wasm-debug feature `{forbidden}`.",
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root.contains(required),
            "ui root should keep shared wasm-debug isolation marker `{required}`.",
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app.contains(required),
            "docs app should keep wasm-debug visual entry marker `{required}`.",
        );
    }

    for required in [
        "ui_headless::UiTraceEventKind::Inspect",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            debug_overlay.contains(required) || trace.contains(required),
            "global trace/debug overlay should keep marker `{required}`.",
        );
    }

    for required in [
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-class-source=semantics.attrs.data_class_source",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress should keep stable state/source marker `{required}`.",
        );
    }

    for forbidden in [
        "on:click=",
        "on:input=",
        "on:pointerdown=",
        "on:pointerup=",
        "on:keydown=",
        "on:keyup=",
        "request_replay",
        "emit_selection_trace(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress replay path should remain N/A-by-design without interaction token `{forbidden}`.",
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("WASM 调试要求"),
        "CircularProgress checklist should keep the wasm-debug contract entry.",
    );
}

#[test]
fn circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let docs_display = load_source("docs_display");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let script = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep DX hot-reload marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "title=\"Size + Thickness Matrix\"",
        "code_signal=matrix_code",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
        "slug=\"circular-progress\"",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs should keep isolated demo entry `{required}`.",
        );
    }

    for forbidden in [
        "WORKBENCH_STORAGE_KEY",
        "load_circular_progress_workbench_",
        "save_circular_progress_workbench_",
        "clear_circular_progress_workbench_",
        "Persist workbench state",
    ] {
        assert!(
            !docs_display.contains(forbidden),
            "CircularProgress persists no interactive workbench state in current scope; `{forbidden}` should remain absent.",
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
    ] {
        assert!(
            !view.contains(forbidden),
            "CircularProgress should stay non-interactive display primitive; `{forbidden}` is not expected.",
        );
    }

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            script.contains(needle),
            "DX gate script should include `{needle}`.",
        );
    }

    assert!(
        check2.contains("DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "CircularProgress checklist should keep DX contract entry.",
    );
}

#[test]
fn circular_progress_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally() {
    let docs_display = load_source("docs_display");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view = include_str!("../../../components/code-block/src/view.rs");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Size + Thickness Matrix\"",
        "title=\"Custom Label + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::CircularProgress;\"",
        "data-slot=\"circular-progress-streaming-policy\"",
        "data-slot=\"circular-progress-source-first\"",
        "data-slot=\"circular-progress-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs product surface should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock one-click copy affordance should keep `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_docs_product_copy_paste_ready_rules",
    ] {
        assert!(
            script.contains(required),
            "DX gate script should include docs-product command `{required}`.",
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "circular_progress_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "circular_progress_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "circular_progress_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "circular_progress_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 should keep docs-product marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_source_first_copy_paste_ready_contract_is_documented_and_scripted_locally() {
    let docs_display = load_source("docs_display");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view = include_str!("../../../components/code-block/src/view.rs");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"circular-progress-source-first\"",
        "data-slot=\"circular-progress-source-first-contract\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"circular-progress-source-prerequisites\"",
        "component-circular_progress",
        "inject-css",
        "UiRoot",
        "Copy circular-progress starter",
        "docs-circular-progress-source-copy",
        "data-slot=\"circular-progress-source-paths\"",
        "components/circular-progress/src/mod.rs",
        "components/circular-progress/src/logic.rs",
        "components/circular-progress/src/view.rs",
        "components/circular-progress/src/styles.rs",
        "data-slot=\"circular-progress-source-sync-note\"",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress source-first docs should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock one-click copy affordance should keep `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script.contains(required),
            "DX gate script should include source-first command `{required}`.",
        );
    }

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "circular_progress_check2_documents_source_first_copy_paste_ready_rules",
        "circular_progress_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "circular_progress_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "circular_progress_source_first_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 should keep source-first marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_heroui_benchmark_docs_sync_contract_is_documented_and_scripted_locally() {
    let docs_pages = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display = load_source("docs_display");
    let readme = include_str!("../src/README.md");
    let heroui_strategy = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "### CircularProgress 同步记录（2026-02-20）",
        "`CircularProgress` 参数主轴保持 `aria_label/size_px/thickness_px/class_name/lang/dir`",
        "component_doc!(\"CircularProgress\", \"circular-progress\", \"Display\", display::circular_progress)",
        "display.rs::circular_progress()` 维持 `title=\"CircularProgress\"` 与 `slug=\"circular-progress\"` 可索引访问。",
        "已覆盖 `Hello World`、`Size + Thickness Matrix`、`Custom Label + Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Playground (Props / State / Preview)` 与 `Source-first Starter (Copy-Paste Ready)`",
        "component-circular_progress + inject-css + UiRoot",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            heroui_strategy.contains(required),
            "CircularProgress HeroUI strategy docs should include `{required}`.",
        );
    }

    for required in [
        "\"CircularProgress\"",
        "\"circular-progress\"",
        "display::circular_progress",
    ] {
        assert!(
            docs_pages.contains(required),
            "CircularProgress docs registry should include `{required}`.",
        );
    }

    for required in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"CircularProgress\"",
        "slug=\"circular-progress\"",
        "data-slot=\"circular-progress-source-first\"",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs display should include `{required}`.",
        );
    }

    for required in [
        "# CircularProgress",
        "## Hello World（先用起来）",
        "## docs-app 入口",
        "/#/components/circular-progress",
    ] {
        assert!(
            readme.contains(required),
            "CircularProgress README should include `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script.contains(required),
            "DX gate script should include heroui benchmark command `{required}`.",
        );
    }

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "circular_progress_check2_documents_heroui_benchmark_docs_sync_rules",
        "circular_progress_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "circular_progress_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "circular_progress_heroui_benchmark_docs_sync_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 should keep heroui benchmark marker `{required}`.",
        );
    }
}

#[test]
fn circular_progress_docs_sync_and_state_matrix_contract_is_documented_and_scripted_locally() {
    let docs_display = load_source("docs_display");
    let logic = load_source("logic");
    let view = load_source("view");
    let primitive = include_str!("../../../crates/ui-state-primitives/src/circular_progress.rs");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "title=\"Size + Thickness Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "data-slot=\"circular-progress-docs-sync-matrix\"",
        "<h3>\"State Matrix\"</h3>",
        "<h3>\"Parameter Matrix\"</h3>",
        "data-size-source / data-thickness-source / data-label-source / data-class-source",
        "default = None；`logic.rs::resolve_component_contract`",
        "size_px / thickness_px: Option&lt;f64&gt;",
        "finite 且 > 0",
        "class_name / lang: Option&lt;String&gt;",
        "normalize_optional_text",
        "dir: Option&lt;A11yDirection&gt;",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs-sync/state-matrix docs should keep `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional)] size_px: Option<f64>,",
        "#[prop(optional)] thickness_px: Option<f64>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "default_aria_label: common.loading_aria_label.as_ref(),",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress public API/default injection should keep `{required}`.",
        );
    }

    for required in [
        "fn resolve_default_aria_label(default_aria_label: &str) -> &str",
        "resolve_aria_label(input.aria_label, default_aria_label);",
        "let lang = normalize_optional_text(input.lang);",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic defaults/normalization should keep `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Loading\";",
        "pub fn sanitize_dimension(value: Option<f64>) -> Option<f64> {",
        "size_source_attr: if has_custom_size { \"custom\" } else { \"default\" },",
    ] {
        assert!(
            primitive.contains(required),
            "CircularProgress primitive defaults/source contract should keep `{required}`.",
        );
    }

    for required in [
        "circular_progress_check2_documents_docs_sync_and_state_matrix_rules",
        "circular_progress_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script.contains(required),
            "DX script should include docs-sync/state-matrix gate `{required}`.",
        );
    }

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "circular_progress_docs_sync_and_state_matrix_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 docs-sync/state-matrix evidence should keep `{required}`.",
        );
    }
}

#[test]
fn circular_progress_documentation_as_product_contract_is_documented_and_scripted_locally() {
    let readme = include_str!("../src/README.md");
    let docs_display = load_source("docs_display");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "# CircularProgress",
        "## Hello World（先用起来）",
        "## 常见用法",
        "## 进阶用法（需要时再看）",
        "## docs-app 入口",
        "<CircularProgress />",
        "size_px=24.0",
        "class_name=\"docs-circular-progress-custom\".to_string()",
        "dir=A11yDirection::Rtl",
        "/#/components/circular-progress",
    ] {
        assert!(
            readme.contains(required),
            "CircularProgress README should include beginner-first marker `{required}`.",
        );
    }

    let hello_index = readme
        .find("## Hello World（先用起来）")
        .unwrap_or_else(|| panic!("README should contain Hello World section."));
    let common_index = readme
        .find("## 常见用法")
        .unwrap_or_else(|| panic!("README should contain common-usage section."));
    let advanced_index = readme
        .find("## 进阶用法（需要时再看）")
        .unwrap_or_else(|| panic!("README should contain advanced section."));
    assert!(
        hello_index < common_index && common_index < advanced_index,
        "CircularProgress README should keep beginner-first order: Hello -> common -> advanced.",
    );

    for required in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Size + Thickness Matrix\"",
        "title=\"Custom Label + Class\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs entry should include `{required}`.",
        );
    }

    for required in [
        "circular_progress_check2_documents_documentation_as_product_rules",
        "circular_progress_documentation_entry_exists_with_beginner_first_progression",
        "circular_progress_dx_check_script_covers_documentation_as_product_contract",
    ] {
        assert!(
            script.contains(required),
            "DX script should include documentation-as-product gate `{required}`.",
        );
    }

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "components/circular-progress/src/README.md",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "circular_progress_documentation_as_product_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 documentation-as-product evidence should include `{required}`.",
        );
    }
}

#[test]
fn circular_progress_interactive_playground_contract_is_documented_and_scripted_locally() {
    let docs_display = load_source("docs_display");
    let e2e_source =
        include_str!("../../../e2e/tests/docs_app_circular_progress_contract.spec.mjs");
    let script = include_str!("../../../scripts/check-ui-dx.sh");
    let check2 = load_source("check2");

    for required in [
        "title=\"Interactive Playground (Props / State / Preview)\"",
        "data-slot=\"circular-progress-workbench-controls\"",
        "data-slot=\"circular-progress-workbench-preview\"",
        "data-slot=\"circular-progress-workbench-state\"",
        "data-slot=\"circular-progress-workbench-size-24\"",
        "data-slot=\"circular-progress-workbench-thickness-3\"",
        "data-slot=\"circular-progress-workbench-label-custom\"",
        "data-slot=\"circular-progress-workbench-class-custom\"",
        "data-slot=\"circular-progress-workbench-dir-rtl\"",
    ] {
        assert!(
            docs_display.contains(required),
            "CircularProgress docs interactive playground should include `{required}`.",
        );
    }

    for required in [
        "docs-app circular-progress interactive playground updates props and semantic markers",
        "adjust size/thickness props and observe custom source markers",
        "toggle label/class source and observe semantic marker updates",
        "toggle direction and verify semantic locale attrs",
        "replay flow after remount remains deterministic",
        "circular-progress-workbench-size-default",
    ] {
        assert!(
            e2e_source.contains(required),
            "CircularProgress interactive e2e flow should include `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "toHaveScreenshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "CircularProgress interactive e2e flow should avoid `{forbidden}`.",
        );
    }

    for required in [
        "circular_progress_check2_documents_interactive_playground_rules",
        "circular_progress_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "circular_progress_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "circular_progress_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script.contains(required),
            "DX script should include interactive-playground gate `{required}`.",
        );
    }

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "apps/docs-app/src/pages/components/pages/display.rs::circular_progress",
        "e2e/tests/docs_app_circular_progress_contract.spec.mjs::docs-app circular-progress interactive playground updates props and semantic markers",
        "circular_progress_interactive_playground_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress check2 interactive-playground evidence should include `{required}`.",
        );
    }
}

#[test]
fn circular_progress_engineering_contract_is_spec_free_tracing_aligned_and_runtime_agnostic_locally()
 {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let cargo = include_str!("../../../crates/ui/Cargo.toml");
    let button_view = include_str!("../../../components/button/src/view.rs");
    let script = include_str!("../../../scripts/check-ui-engineering.sh");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "CircularProgress should keep spec/schema boundary as N/A for simple component scope.",
    );
    assert!(
        cargo.contains("component-circular_progress = [\"dep:ui-circular-progress\"]"),
        "CircularProgress feature should stay lightweight without serde/spec dependency fan-out.",
    );
    assert!(
        !cargo.contains("component-circular_progress = [\"dep:serde\"")
            && !cargo.contains("component-circular_progress = [\"dep:serde_json\""),
        "CircularProgress should not opt into serde/spec migration dependencies without explicit schema contract.",
    );

    let combined = [module, logic, view, styles].join("\n");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "CircularProgress engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`.",
        );
    }

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo.contains(required) || button_view.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`.",
        );
    }

    for forbidden in [
        "circular-progress-wasm-debug",
        "circular_progress-wasm-debug",
        "component-circular_progress-wasm-debug",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::circular_progress::",
        "const CIRCULAR_PROGRESS_TRACE_TARGET",
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
            !combined.contains(forbidden) && !cargo.contains(forbidden),
            "CircularProgress engineering contract should avoid tracing/runtime leak marker `{forbidden}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script.contains(needle),
            "engineering gate script should include `{needle}`.",
        );
    }

    assert!(
        check2.contains("工程能力统一"),
        "CircularProgress checklist should keep engineering contract entry.",
    );
}

#[test]
fn circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally() {
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/circular_progress.rbi");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let check2_source = load_source("check2.md");
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for needle in [
        "schema_version = \"1\"",
        "name = \"CircularProgress\"",
        "crate = \"ui-circular-progress\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "circular-progress manifest should keep stable v1 schema marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn CircularProgress(",
        "pub enum CircularProgressAgentSchemaVersion {",
        "V1,",
        "pub schema_version: CircularProgressAgentSchemaVersion,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "circular-progress RBI should keep stable public API marker `{needle}`.",
        );
    }

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");
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
            "circular-progress should not introduce major-version migration marker `{forbidden}` in current scope.",
        );
    }

    let marker = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `CircularProgress` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "circular_progress_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "circular-progress/check2.md should keep version-migration governance marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_styles_use_defensive_variable_fallback_chain_locally() {
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let theme_css = include_str!("../../../crates/ui-theme/src/css.rs");
    let script = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-cp-size, var(--ui-button-spinner-size, var(--ui-fallback-button-spinner-size)))",
        "var(--ui-cp-thickness, var(--ui-button-spinner-border, var(--ui-fallback-button-spinner-border)))",
        "--ui-cp-rotation-duration,",
        "var(--ui-button-spinner-duration, var(--ui-fallback-button-spinner-duration))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
    ] {
        assert!(
            styles.contains(required),
            "CircularProgress styles should keep defensive fallback chain `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-button-spinner-size:",
        "--ui-fallback-button-spinner-border:",
        "--ui-fallback-button-spinner-duration:",
        "--ui-fallback-button-radius-full:",
        "--ui-fallback-border:",
        "--ui-fallback-accent:",
        "--ui-fallback-fg:",
        "--ui-fallback-border-width:",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme css should remain SSOT for fallback token `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-button-spinner-size, 16px)",
        "var(--ui-button-spinner-border, 2px)",
        "var(--ui-button-spinner-duration, 800ms)",
        "border-radius: 9999px;",
    ] {
        assert!(
            !styles.contains(forbidden),
            "CircularProgress styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_styles_use_defensive_variable_fallback_chain";
    assert!(
        script.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("样式孤岛防御（Defensive Variables）"),
        "CircularProgress checklist should keep defensive-variables contract entry.",
    );
}

#[test]
fn circular_progress_cascade_layer_and_runtime_style_contract_is_enforced_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let script = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css registry should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "style=style_vars",
        "pub fn compose_style_vars(state: &CircularProgressState) -> Option<String>",
        "vars.push(format!(\"--ui-cp-size: {size_px}px;\"));",
        "vars.push(format!(\"--ui-cp-thickness: {thickness_px}px;\"));",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "CircularProgress runtime style path should stay css-variable-only via `{required}`.",
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should avoid plain inline style token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("级联层覆盖（`@layer ui`）"),
        "CircularProgress checklist should keep cascade-layer contract entry.",
    );
}

#[test]
fn circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards_locally()
 {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let ui_motion = include_str!("../../../crates/ui-motion/src/lib.rs");
    let script = include_str!("../../../scripts/check-ui-platforms.sh");
    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    assert!(
        !motion_path.exists(),
        "CircularProgress motion.rs should stay N/A-by-design for display-only runtime-attach path.",
    );

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "attach_motion(",
        "MotionOptions",
        "spring",
        "stiffness",
        "damping",
        "ui_motion::web::animate(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "CircularProgress should avoid runtime motion-attach token `{forbidden}`.",
        );
    }

    for required in [
        "@media (prefers-reduced-motion: reduce)",
        "animation-duration: 1ms;",
        "animation-iteration-count: 1;",
        "data-motion=\"spin\"",
    ] {
        assert!(
            styles.contains(required),
            "CircularProgress styles should keep reduced-motion marker `{required}`.",
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion.contains(required),
            "ui-motion should keep non-wasm no-op marker `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test circular_progress_semantics circular_progress_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "cargo test -p ui --test circular_progress_semantics circular_progress_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
    ] {
        assert!(
            script.contains(needle),
            "platform gate script should include `{needle}`.",
        );
    }

    assert!(
        check2.contains("Motion 合同化"),
        "CircularProgress checklist should keep motion-contractualization entry.",
    );
}

#[test]
fn circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries_locally() {
    let check2 = load_source("check2");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let ui_components_root = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let script = include_str!("../../../scripts/check-ui-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-circular_progress\")]",
        "pub use ui_circular_progress as circular_progress;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["web_sys::", "web-sys", "HtmlElement", "NodeRef", "JsValue"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-circular_progress\")]",
        "out.push_str(crate::circular_progress::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui root.rs should keep centralized injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should contain `{required}`.",
        );
    }

    for forbidden in ["CircularProgress", "aria-", "data-state"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`.",
        );
    }
    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence.contains(required),
            "ui-headless presence canonical path should contain `{required}`.",
        );
    }
    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y.contains(required),
            "ui-headless a11y canonical path should contain `{required}`.",
        );
    }

    let ui_components_src_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("`ui` 固定入口文件落点正确"),
        "CircularProgress checklist should keep fixed-entry-files contract entry.",
    );
}

#[test]
fn circular_progress_component_directory_standard_files_follow_contract_and_na_paths_locally() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let script = include_str!("../../../scripts/check-ui-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "CircularProgress component directory should include `{required_file}`.",
        );
    }
    for absent_file in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent_file).exists(),
            "CircularProgress component directory should keep `{absent_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::DEFAULT_ARIA_LABEL;",
        "pub use view::CircularProgress;",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "mod spec;",
        "mod render;",
    ] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalized state derivation marker `{required}`.",
        );
    }
    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "NodeRef",
        "HtmlElement",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should stay free of DOM/platform token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-circular-progress[data-state=\"indeterminate\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should avoid render/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "view! {",
        "data-state=semantics.attrs.data_state",
        "role=semantics.attrs.role",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in [
        "resolve_state(CircularProgressStateInput {",
        "logic::resolve_aria_label(",
        "@keyframes",
        ".ui-circular-progress {",
        "web_sys::",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should avoid hidden state/styling/platform token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    assert!(
        check2.contains("组件目录标准文件落点正确"),
        "CircularProgress checklist should keep component-directory-standard entry.",
    );
}

#[test]
fn circular_progress_check2_marks_file_placement_discipline_contract_complete_locally() {
    let check2 = load_source("check2");

    for needle in [
        "文件落点纪律",
        "N/A-by-design",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths",
        "circular_progress_component_files_check_script_covers_standard_directory_contract",
        "circular_progress_component_directory_standard_files_follow_contract_and_na_paths_locally",
    ] {
        assert!(
            check2.contains(needle),
            "CircularProgress check2 should keep file-placement-discipline marker `{needle}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_definition_contract_is_snapshot_only_and_protocol_free_locally() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let script = include_str!("../../../scripts/check-ui-streaming.sh");

    for required in [
        "流式在这里仅指 LLM 输出渲染",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-output-status",
        "project_streaming_",
        "streaming",
        "Streaming",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "CircularProgress should stay non-streaming in component scope and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn circular_progress_snapshot_baseline_consumes_complete_result_and_renders_stably_locally() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let docs = load_source("docs_display");
    let script = include_str!("../../../scripts/check-ui-streaming.sh");

    for required in [
        "`Snapshot` 是所有组件的基础能力（默认必须支持）",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress checklist should keep snapshot-baseline marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional)] size_px: Option<f64>,",
        "#[prop(optional)] thickness_px: Option<f64>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::resolve_component_contract(CircularProgressLogicInput {",
        "default_aria_label: common.loading_aria_label.as_ref(),",
        "let semantics = use_circular_progress(CircularProgressOptions {",
        "data-state=semantics.attrs.data_state",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-ui-schema=agent_contract.schema_name",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should keep complete snapshot-input render marker `{required}`.",
        );
    }

    for required in [
        "pub struct CircularProgressLogicInput<'a>",
        "pub struct CircularProgressLogicOutput",
        "pub fn resolve_component_contract(",
        "let lang = normalize_optional_text(input.lang);",
        "let class_name = normalize_optional_text(input.class_name);",
        "let default_aria_label = resolve_default_aria_label(input.default_aria_label);",
        "let state = resolve_state(CircularProgressStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should keep deterministic snapshot normalization marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn circular_progress() -> AnyView",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "title=\"Size + Thickness Matrix\"",
        "code_signal=matrix_code",
        "<Playground title=\"Custom Label + Class\" code_signal=custom_code>",
        "<CircularProgress />",
        "aria_label=\"Background refresh\".to_string()",
        "size_px=28.0",
        "thickness_px=3.5",
        "class_name=\"docs-circular-progress-custom\".to_string()",
    ] {
        assert!(
            docs.contains(required),
            "CircularProgress docs should keep complete snapshot configuration marker `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn circular_progress_streaming_required_optional_contract_is_snapshot_fallback_with_semantic_continuity_locally()
 {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let script = include_str!("../../../scripts/check-ui-streaming.sh");

    for required in [
        "`Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "输出状态（草稿/已验证/可提交）由上层 LLM 容器决策并透传",
        "数据校验、断线恢复、重试策略保持在上层，不下沉到组件。",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress checklist should keep streaming required/optional marker `{required}`.",
        );
    }

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "aria-valuemin=semantics.attrs.aria_valuemin",
        "aria-valuemax=semantics.attrs.aria_valuemax",
        "data-slot=\"circular-progress\"",
        "data-state=semantics.attrs.data_state",
        "data-motion=semantics.attrs.data_motion",
        "data-size-source=semantics.attrs.data_size_source",
        "data-thickness-source=semantics.attrs.data_thickness_source",
        "data-label-source=semantics.attrs.data_label_source",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-state=agent_contract.state.as_str()",
    ] {
        assert!(
            view.contains(required),
            "CircularProgress view should keep semantic continuity marker `{required}`.",
        );
    }

    for forbidden in [
        "retry",
        "backoff",
        "reconnect",
        "断线恢复",
        "is_loading",
        "error",
        "data-ui-output-status",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "CircularProgress should keep retry/resilience boundary outside component layer `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test circular_progress_semantics --no-default-features --features component-circular_progress,inject-css circular_progress_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn circular_progress_rust_hygiene_contract_forbids_unwrap_expect_let_underscore_and_converges_hotspots_to_cow_locally()
 {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../scripts/check-ui-engineering.sh");

    for source in [module, logic, view, styles] {
        for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "CircularProgress non-test sources should not contain `{forbidden}`.",
            );
        }
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-circular-progress\")",
        "Cow::Borrowed(\"ui-circular-progress--state-indeterminate\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic.contains(required),
            "CircularProgress logic should keep Cow-based string hotspot marker `{required}`.",
        );
    }

    for required in [
        "forbidden unwrap/expect in non-test code",
        "forbidden let _ = in non-test code",
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
        "[rust-hygiene] OK",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene script should keep marker `{required}`.",
        );
    }

    for required in [
        "circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering script should include circular-progress rust-hygiene gate `{required}`.",
        );
    }

    for required in [
        "代码卫生（Rust Hygiene）",
        "./scripts/check-rust-hygiene.sh",
        "Vec<Cow<'static, str>>",
        "circular_progress_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "circular_progress_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "circular_progress_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "circular_progress_rust_hygiene_contract_forbids_unwrap_expect_let_underscore_and_converges_hotspots_to_cow_locally",
    ] {
        assert!(
            check2.contains(required),
            "CircularProgress checklist should keep rust-hygiene evidence marker `{required}`.",
        );
    }
}
