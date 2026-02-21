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
fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path).is_some()
}
#[test]
fn segmented_control_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/segmented_control/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SegmentedControl internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn segmented_control_uses_headless_hooks() {
    let source = load_source("src/segmented_control/view.rs");

    for needle in ["use_radio", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "SegmentedControl should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn segmented_control_uses_logic_state_model() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/segmented_control.rs");

    for needle in [
        "pub enum SegmentedControlOrientation",
        "pub fn class_name(self) -> &'static str",
        "pub fn aria_orientation(self) -> &'static str",
        "pub fn is_vertical(self) -> bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl logic should keep assembly-oriented orientation contracts via `{needle}`."
        );
    }

    for needle in [
        "pub struct SegmentedControlStateInput",
        "pub struct SegmentedControlState",
        "pub fn resolve_state(input: SegmentedControlStateInput<'_>) -> SegmentedControlState",
        "pub item_count: usize",
        "pub has_disabled_options: bool",
        "pub selected_index: Option<usize>",
        "pub has_selection: bool",
    ] {
        assert!(
            primitive_source.contains(needle),
            "SegmentedControl state primitive should be owned by ui-state-primitives via `{needle}`."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "resolve_state(SegmentedControlStateInput {",
        "aria.state.selected_index.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should derive root state via ui-state-primitives; missing `{needle}`."
        );
    }

    for forbidden in ["pub struct SegmentedControlState", "pub fn resolve_state("] {
        assert!(
            !logic_source.contains(forbidden),
            "SegmentedControl logic should not carry state-machine primitive `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_attaches_indicator_motion_driver() {
    let source = load_source("src/segmented_control/view.rs");

    assert!(
        source.contains("attach_indicator_motion"),
        "SegmentedControl should attach a motion driver for the selection indicator (baseline-style feel)."
    );
}

#[test]
fn segmented_control_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/segmented_control/view.rs");

    for attr in [
        "const SLOT_ROOT: &str = \"segmented-control\";",
        "data-slot=SLOT_ROOT",
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selection-empty=move || state.get().selection_empty.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-orientation=orientation.data_orientation()",
        "data-horizontal=move || state.get().is_horizontal.then_some(\"true\")",
        "data-vertical=move || state.get().is_vertical.then_some(\"true\")",
        "data-has-label=move || state.get().has_label.then_some(\"true\")",
        "const SLOT_OPTION: &str = \"segmented-control-option\";",
        "data-slot=SLOT_OPTION",
        "data-index=index",
        "data-selected=move || is_selected().then_some(\"true\")",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "SegmentedControl should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn segmented_control_sets_aria_orientation_and_option_fallback_label() {
    let source = load_source("src/segmented_control/view.rs");

    for needle in [
        "aria-orientation=orientation.aria_orientation()",
        "format!(\"Option {}\", index + 1)",
    ] {
        assert!(
            source.contains(needle),
            "SegmentedControl should keep `{needle}` for robust ARIA semantics and predictable option labels."
        );
    }
}

#[test]
fn segmented_control_styles_define_indicator_css_vars() {
    let source = load_source("src/segmented_control/styles.rs");

    for var in [
        "--ui-segmented-control-indicator-x",
        "--ui-segmented-control-indicator-y",
        "--ui-segmented-control-indicator-w",
        "--ui-segmented-control-indicator-h",
        "--ui-segmented-control-indicator-o",
    ] {
        assert!(
            source.contains(var),
            "SegmentedControl styles should define `{var}` so motion can update the indicator without re-rendering."
        );
    }
}

#[test]
fn segmented_control_motion_uses_spring_animator() {
    let source = load_source("src/segmented_control/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SegmentedControl motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/segmented_control/motion.rs");
    let view_source = load_source("src/segmented_control/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SegmentedControlMotion) -> SegmentedControlMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    for needle in [
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let motion = crate::segmented_control::motion::sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle) || view_source.contains(needle),
            "SegmentedControl should include `{needle}` to sanitize motion at component and runtime boundaries.",
        );
    }
}

#[test]
fn segmented_control_headless_boundary_is_enforced() {
    let view_source = load_source("src/segmented_control/view.rs");
    let radio_source = load_source("../ui-headless/src/radio.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "use_radio(RadioOptions {",
        "group: RadioGroupOptions {",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should consume typed headless radio contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct RadioAttrs",
        "pub struct RadioState",
        "pub struct RadioContract",
        "pub lang: Option<String>",
        "pub dir: Option<&'static str>",
    ] {
        assert!(
            radio_source.contains(needle),
            "ui-headless radio contract should expose typed attrs + handlers + state via `{needle}`."
        );
    }

    for forbidden in [".ui-", "box-shadow", "transition:"] {
        assert!(
            !radio_source.contains(forbidden),
            "ui-headless radio contract should not include visual/CSS concerns; found `{forbidden}`."
        );
    }

    assert!(
        a11y_source.contains("pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs"),
        "ui-headless a11y should keep lang/dir locale entrypoint."
    );

    assert!(
        check2_source.contains("- [x] `ui-headless` 定义："),
        "segmented_control check2 should mark ui-headless gate as completed."
    );
}

#[test]
fn segmented_control_motion_boundary_is_enforced() {
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "SpringAnimator",
        "default_button_motion_tokens",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion should keep contract/runtime boundary via `{needle}`."
        );
    }

    for forbidden in ["aria-", "role=\"", "on:keydown", "use_radio"] {
        assert!(
            !motion_source.contains(forbidden),
            "SegmentedControl motion layer should not absorb a11y/interaction semantics; found `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-motion` 定义："),
        "segmented_control check2 should mark ui-motion gate as completed."
    );
}

#[test]
fn segmented_control_theme_tokens_are_consumed_in_styles() {
    let styles_source = load_source("src/segmented_control/styles.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "var(--ui-space-2xs)",
        "var(--ui-space-3xs)",
        "var(--ui-space-md)",
        "var(--ui-component-height-100)",
        "var(--ui-font-size-100)",
        "var(--ui-font-size-150)",
        "var(--ui-font-size-200)",
    ] {
        assert!(
            styles_source.contains(needle),
            "SegmentedControl styles should consume ui-theme tokens via `{needle}`."
        );
    }

    for forbidden in [
        "font-size: 13px",
        "font-size: 12px",
        "font-size: 14px",
        "height: 34px",
        "height: 30px",
        "height: 38px",
        "padding: 4px",
        "top: 4px",
        "left: 4px",
        "gap: 6px",
        "min-width: 44px",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "SegmentedControl styles should avoid hardcoded size constants; found `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-theme` 定义："),
        "segmented_control check2 should mark ui-theme gate as completed."
    );
}

#[test]
fn segmented_control_ui_components_layer_boundary_is_enforced() {
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "pub use logic::{SegmentedControlOrientation, SegmentedControlSize};",
        "pub use motion::SegmentedControlMotion;",
        "pub use view::SegmentedControl;",
    ] {
        assert!(
            mod_source.contains(needle),
            "SegmentedControl mod export boundary should stay minimal via `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "web_sys", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden),
            "SegmentedControl public API boundary should not expose internals/platform details; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum SegmentedControlOrientation",
        "pub enum SegmentedControlSize",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl logic layer should own prop-facing typed normalization helpers via `{needle}`."
        );
    }

    for forbidden in [
        "view!",
        "NodeRef<",
        "web_sys",
        "use_radio(",
        "resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "SegmentedControl logic layer should stay assembly-only and avoid view/headless/primitive implementation leakage; found `{forbidden}`."
        );
    }

    for needle in [
        "use_radio(RadioOptions {",
        "resolve_state(SegmentedControlStateInput {",
        "motion::attach_indicator_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view layer should mount headless + primitive + motion contracts via `{needle}`."
        );
    }

    for forbidden in [".ui-segmented-control", "box-shadow:", "SpringAnimator"] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl view layer should avoid inline style/motion engine implementation; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "SegmentedControl styles should remain token-first static CSS."
    );
    for forbidden in ["on:click", "use_radio(", "resolve_state("] {
        assert!(
            !styles_source.contains(forbidden),
            "SegmentedControl styles layer should not include behavior/state logic; found `{forbidden}`."
        );
    }

    for needle in ["pub fn attach_indicator_motion(", "SpringAnimator"] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion layer should provide attach mapping and use shared motion runtime via `{needle}`."
        );
    }
    for forbidden in ["data-slot", "aria-", "use_radio("] {
        assert!(
            !motion_source.contains(forbidden),
            "SegmentedControl motion layer should not absorb view/headless semantics; found `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-components` 定义："),
        "segmented_control check2 should mark ui-components gate as completed."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_component_directory_has_standard_file_layout_and_scoped_responsibilities() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for required in [
        "src/segmented_control/mod.rs",
        "src/segmented_control/logic.rs",
        "src/segmented_control/styles.rs",
        "src/segmented_control/view.rs",
        "src/segmented_control/motion.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "SegmentedControl should keep standard component file `{required}`."
        );
    }
    for forbidden in [
        "src/segmented_control/render.rs",
        "src/segmented_control/spec.rs",
    ] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "SegmentedControl should avoid non-standard/simple-scope file `{forbidden}`."
        );
    }

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use logic::{SegmentedControlOrientation, SegmentedControlSize};",
        "pub use motion::SegmentedControlMotion;",
        "pub use view::SegmentedControl;",
    ] {
        assert!(
            mod_source.contains(required),
            "segmented_control/mod.rs should keep minimal stable exports via `{required}`."
        );
    }
    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "segmented_control/mod.rs should not over-export internals `{forbidden}`."
        );
    }

    for required in [
        "pub enum SegmentedControlOrientation",
        "pub enum SegmentedControlSize",
        "pub enum SegmentedControlControlMode",
        "pub enum SegmentedControlSelectionSource",
        "pub enum SegmentedControlSelectionOrigin",
    ] {
        assert!(
            logic_source.contains(required),
            "segmented_control/logic.rs should own typed normalization/source markers `{required}`."
        );
    }
    for forbidden in [
        "view! {",
        "NodeRef<",
        "web_sys",
        "use_radio(",
        "resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "segmented_control/logic.rs should avoid view/dom/headless/primitive impl detail `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "segmented_control/styles.rs should remain token-first static CSS `{required}`."
        );
    }
    for forbidden in [
        "use leptos",
        "view! {",
        "on:click",
        "on:keydown",
        "use_radio(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "segmented_control/styles.rs should avoid behavior/view logic `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn SegmentedControl(",
        "use_radio(RadioOptions {",
        "resolve_state(SegmentedControlStateInput {",
        "motion::attach_indicator_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "segmented_control/view.rs should render structure and mount contracts via `{required}`."
        );
    }
    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "pub fn attach_indicator_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "segmented_control/view.rs should not absorb motion engine/driver implementation `{forbidden}`."
        );
    }

    for required in [
        "pub struct SegmentedControlMotion",
        "pub fn attach_indicator_motion(",
        "pub fn sanitize_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "segmented_control/motion.rs should keep motion contract + attach boundary `{required}`."
        );
    }
    for forbidden in [
        "role=\"",
        "aria-",
        "data-slot",
        "use_radio(",
        "resolve_state(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "segmented_control/motion.rs should avoid view/headless/state semantics `{forbidden}`."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
        "segmented_control_component_directory_has_standard_file_layout_and_scoped_responsibilities",
    ] {
        assert!(
            check2_source.contains(required),
            "SegmentedControl checklist should keep component-directory governance marker `{required}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            check2_source.contains(required),
            "SegmentedControl checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");

    for needle in [
        "pub enum SegmentedControlAgentSchemaVersion",
        "pub enum SegmentedControlAgentIntent",
        "pub enum SegmentedControlAgentActionModel",
        "pub struct SegmentedControlAgentContract",
        "pub fn segmented_control_agent_contract() -> SegmentedControlAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = segmented_control_agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn segmented_control_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing()
 {
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");

    for needle in [
        "schema_version_attr: SegmentedControlAgentSchemaVersion::V1.as_attr()",
        "intent_attr: SegmentedControlAgentIntent::SingleChoiceSelection.as_attr()",
        "action_model_attr: SegmentedControlAgentActionModel::NavigateAndSelect.as_attr()",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl agent contract should derive `{needle}` from typed enums."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-schema-version=format!(",
        "data-ui-intent=format!(",
        "data-ui-action-model=format!(",
        "data-ui-state-axis=format!(",
        "data-ui-source-axis=format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl view should not build agent contract marker via free-form splice `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{docs_source}"
    );

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
            "SegmentedControl Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn segmented_control() -> AnyView",
        "title=\"SegmentedControl\"",
        "slug=\"segmented-control\"",
        "description=\"Segmented control with baseline-level indicator motion and baseline-style root state attrs.\"",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "<Playground title=\"Vertical + Disabled + Empty\" code_signal=states_code>",
        "<SegmentedControl",
        "orientation=SegmentedControlOrientation::Vertical",
        "size=SegmentedControlSize::Sm",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "forms docs page should include `{needle}` for segmented-control coverage.",
        );
    }
}

#[test]
fn segmented_control_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "id_base=\"seg\".to_string()",
        "options=options",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "disabled_indices=vec![2]",
        "id_base=\"docs-segments\".to_string()",
        "\"selected: \"",
        "\" · has selection: \"",
        "\" · disabled options: 1\"",
        "id_base=\"docs-segments-vertical\".to_string()",
        "disabled_indices=vertical_disabled_indices",
        "id_base=\"docs-segments-empty\".to_string()",
        "aria_label=\"No options\".to_string()",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "segmented-control docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn segmented_control_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_docs_examples_sync_with_logic_api_names_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/segmented_control.rs");

    segmented_control_docs_page_covers_primary_playgrounds();
    segmented_control_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn segmented_control() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Selection + Root State\"",
        "title=\"Vertical + Disabled + Empty\"",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "disabled_indices=vec![2]",
        "orientation=SegmentedControlOrientation::Vertical",
        "size=SegmentedControlSize::Sm",
        "disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs should keep API/default/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] disabled_indices: Vec<usize>",
        "#[prop(optional)] orientation: SegmentedControlOrientation",
        "#[prop(optional)] size: SegmentedControlSize",
        "pub enum SegmentedControlOrientation",
        "pub enum SegmentedControlSize",
        "pub struct SegmentedControlStateInput",
        "pub fn resolve_state(input: SegmentedControlStateInput<'_>) -> SegmentedControlState",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "SegmentedControl public/default contract should keep `{needle}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep documentation-as-product rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/segmented_control/README.md");
    let has_docs_page = path_exists("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        has_readme || has_docs_page,
        "SegmentedControl must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn segmented_control() -> AnyView"),
        "Equivalent docs entry should expose segmented_control page function."
    );
    for needle in [
        "\"SegmentedControl\"",
        "\"segmented-control\"",
        "forms::segmented_control",
    ] {
        assert!(
            pages_registry.contains(needle),
            "docs page registry should index segmented control route token `{needle}`."
        );
    }
}

#[test]
fn segmented_control_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "SegmentedControl checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"SegmentedControl\"",
        "slug=\"segmented-control\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "<Playground title=\"Interactive Playground (Props + State)\"",
        "<Playground title=\"Vertical + Disabled + Empty\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .expect("segmented_control docs should include hello-world playground");
    let base_pos = docs_source
        .find("<Playground title=\"Selection + Root State\" code_signal=code>")
        .expect("segmented_control docs should include baseline playground");
    let interactive_pos = docs_source
        .find("<Playground title=\"Interactive Playground (Props + State)\"")
        .expect("segmented_control docs should include interactive playground");
    let matrix_pos = docs_source
        .find("<Playground title=\"Vertical + Disabled + Empty\" code_signal=states_code>")
        .expect("segmented_control docs should include state-matrix playground");

    assert!(
        hello_pos < base_pos && base_pos < interactive_pos && interactive_pos < matrix_pos,
        "SegmentedControl docs should present default usage before advanced controls."
    );
}

#[test]
fn segmented_control_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let section_start = docs_source
        .find("pub(super) fn segmented_control() -> AnyView")
        .expect("forms docs should define segmented_control page function");
    let section = &docs_source[section_start..];
    let hello_start = section
        .find("let hello_code = Signal::derive(move || {")
        .expect("segmented_control docs should define hello_code");
    let hello_end = section[hello_start..]
        .find("let code = Signal::derive(move || {")
        .map(|offset| hello_start + offset)
        .expect("segmented_control docs should define baseline code block after hello_code");
    let hello_block = &section[hello_start..hello_end];

    let snippet_start = hello_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("hello snippet should be embedded as raw string");
    let snippet_end = hello_block[snippet_start..]
        .find("\"#")
        .map(|offset| snippet_start + offset)
        .expect("hello snippet should terminate raw string");
    let hello_snippet = &hello_block[snippet_start..snippet_end];
    let meaningful_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "SegmentedControl Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{hello_snippet}"
    );

    for forbidden in [
        "ui_state_primitives",
        "ui-headless",
        "ui_headless",
        "state=",
        "controller=",
        "Signal<",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "SegmentedControl Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "segmented_control_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "segmented_control_docs_are_beginner_friendly_with_default_then_advanced_path",
        "segmented_control_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "SegmentedControl checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn segmented_control() -> AnyView",
        "<Playground title=\"Interactive Playground (Props + State)\"",
        "let (interactive_vertical, set_interactive_vertical) = signal(false);",
        "let (interactive_small, set_interactive_small) = signal(false);",
        "let (interactive_disabled, set_interactive_disabled) = signal(false);",
        "let (interactive_disable_last, set_interactive_disable_last) = signal(true);",
        "let (interactive_selected, set_interactive_selected) = signal(Some(0_usize));",
        "id_base=\"docs-segments-interactive\".to_string()",
        "orientation=if interactive_vertical.get() {",
        "size=if interactive_small.get() {",
        "disabled=interactive_disabled.get()",
        "disabled_indices=if interactive_disable_last.get() {",
        "data-slot=\"segmented-control-marker-controls\"",
        "data-slot=\"segmented-control-toggle-orientation\"",
        "data-slot=\"segmented-control-toggle-size\"",
        "data-slot=\"segmented-control-toggle-disabled\"",
        "data-slot=\"segmented-control-toggle-disabled-last\"",
        "data-slot=\"segmented-control-reset-selection\"",
        "data-slot=\"segmented-control-marker-summary\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs should provide interactive playground marker `{needle}`."
        );
    }

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app Playground should keep interactive preview contract `{needle}`."
        );
    }
}

#[test]
fn segmented_control_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_segmented_control_contract.spec.mjs");

    for needle in [
        "docs-app segmented-control key flow is repeatable with semantic ready/settled breakpoints",
        "await page.goto(SEGMENTED_CONTROL_PAGE);",
        "body:not(:has(#boot))",
        "#docs-segments-interactive-radio-0",
        "await page.keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-selection-origin\", \"keyboard\")",
        "await page.reload();",
        "toHaveAttribute(\"data-selection-origin\", \"programmatic\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SegmentedControl interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code-block/src/view.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");

    for needle in [
        "pub(super) fn segmented_control() -> AnyView",
        "data-slot=\"segmented-control-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<ui_components::Snippet",
        "copyable=true",
        "class_name=\"docs-segmented-control-source-copy\".to_string()",
        "data-slot=\"segmented-control-source-paths\"",
        "\"crates/ui-components/src/segmented_control/mod.rs\"",
        "\"crates/ui-components/src/segmented_control/logic.rs\"",
        "\"crates/ui-components/src/segmented_control/view.rs\"",
        "\"crates/ui-components/src/segmented_control/styles.rs\"",
        "\"crates/ui-components/src/segmented_control/motion.rs\"",
        "data-slot=\"segmented-control-source-prerequisites\"",
        "\"component-segmented_control\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs should keep copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let resolved_code = Signal::derive(move || {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app playground should keep copy-paste pipeline marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep one-click copy marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] disabled_indices: Vec<usize>",
        "#[prop(optional)] orientation: SegmentedControlOrientation",
        "#[prop(optional)] size: SegmentedControlSize",
        "pub enum SegmentedControlOrientation",
        "pub enum SegmentedControlSize",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "SegmentedControl docs copy-ready snippets should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn segmented_control_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "segmented_control_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "SegmentedControl checklist should keep HeroUI/doc sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let view_source = load_source("src/segmented_control/view.rs");

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "- 一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment guard token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn segmented_control() -> AnyView",
        "title=\"SegmentedControl\"",
        "slug=\"segmented-control\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs entry should remain accessible via `{needle}`."
        );
    }

    for needle in [
        "\"SegmentedControl\"",
        "\"segmented-control\"",
        "forms::segmented_control",
    ] {
        assert!(
            pages_registry.contains(needle),
            "SegmentedControl docs route should stay indexed via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] disabled_indices: Vec<usize>",
        "#[prop(optional)] orientation: SegmentedControlOrientation",
        "#[prop(optional)] size: SegmentedControlSize",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl parameter model marker `{needle}` should remain traceable for HeroUI/doc sync."
        );
    }
}

#[test]
fn segmented_control_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_segmented_control_contract.spec.mjs");

    for needle in [
        "SEGMENTED_CONTROL_PAGE = \"/#/components/segmented-control\"",
        "body:not(:has(#boot))",
        "[data-slot=\"segmented-control\"][data-control-mode=\"controlled\"]",
        "#docs-segments-interactive-radio-0",
        "#docs-segments-interactive-radio-1",
        "toHaveAttribute(\"data-ui-schema\", \"ui.segmented-control\")",
        "toHaveAttribute(\"data-selection-origin\", \"programmatic\")",
        "toHaveAttribute(\"data-selection-origin\", \"pointer\")",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"aria-checked\", \"true\")",
        "toHaveAttribute(\"style\",",
        "--ui-segmented-control-indicator-o:\\\\s*1",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SegmentedControl e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "section.playground",
        "xpath=",
        "getByText(",
        "locator(\"text=",
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "SegmentedControl e2e selector contract should avoid unstable/non-semantic token `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep repeatable-flow rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_segmented_control_contract.spec.mjs");

    for needle in [
        "docs-app segmented-control key flow is repeatable with semantic ready/settled breakpoints",
        "await option0.focus();",
        "await page.keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-selection-origin\", \"keyboard\")",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "await page.reload();",
        "toHaveAttribute(\"data-selection-origin\", \"programmatic\")",
        "toHaveAttribute(\"data-selected-index\", \"0\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SegmentedControl e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "SegmentedControl e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_segmented_control_contract.spec.mjs");

    for needle in [
        "await option0.focus();",
        "await expect(option0).toBeFocused();",
        "await page.keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-selection-origin\", \"keyboard\")",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"style\",",
        "--ui-segmented-control-indicator-o:\\\\s*1",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "SegmentedControl e2e high-risk path contract should include `{needle}`."
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
            "SegmentedControl high-risk e2e path should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-segmented-control.sh");

    for needle in [
        "cargo test -p ui-components --test segmented_control_semantics --no-default-features --features component-segmented_control,inject-css segmented_control_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test segmented_control_semantics --no-default-features --features component-segmented_control,inject-css segmented_control_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test segmented_control_semantics --no-default-features --features component-segmented_control,inject-css segmented_control_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test segmented_control_semantics --no-default-features --features component-segmented_control,inject-css segmented_control_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test segmented_control_semantics --no-default-features --features component-segmented_control,inject-css segmented_control_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "segmented_control e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn segmented_control_status_primitive_boundary_is_enforced() {
    let primitive_source = load_source("../ui-state-primitives/src/segmented_control.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for forbidden in ["leptos", "web_sys", "view!", "data-slot", "NodeRef<"] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives segmented_control should stay POJO-only; found `{forbidden}`."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod segmented_control;"),
        "ui-state-primitives lib.rs should export segmented_control primitive module."
    );

    for needle in [
        "use ui_state_primitives::segmented_control::{SegmentedControlStateInput, resolve_state};",
        "resolve_state(SegmentedControlStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should consume state primitive via `{needle}`."
        );
    }

    for forbidden in ["pub struct SegmentedControlState", "pub fn resolve_state("] {
        assert!(
            !logic_source.contains(forbidden),
            "SegmentedControl logic should not retain local primitive `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `status-primitives` 定义："),
        "segmented_control check2 should mark status-primitives gate as completed."
    );
}

#[test]
fn segmented_control_state_primitive_source_is_correct() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "use ui_state_primitives::segmented_control::{SegmentedControlStateInput, resolve_state};",
        "resolve_state(SegmentedControlStateInput {",
        "use_radio(RadioOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl should consume state primitives/headless contracts via `{needle}`."
        );
    }

    for forbidden in [
        "RwSignal<",
        "store::",
        "global_store",
        "redux",
        "zustand",
        "mobx",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SegmentedControl should not bind business store directly; found `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 状态原语来源正确："),
        "segmented_control check2 should mark state-primitive-source gate as completed."
    );
}

#[test]
fn segmented_control_async_contract_is_explicit_na() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "retry",
        "use_async_action",
        "async fn",
        "spawn_local",
        "tokio::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SegmentedControl has no async interaction contract and should not include `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 如果无异步相关，直接打勾。"),
        "segmented_control check2 should mark async-contract gate as completed."
    );
    assert!(
        check2_source.contains("N/A：组件仅处理本地选择切换（radio roving + state primitive 映射），无远程请求与异步状态。"),
        "segmented_control check2 should document explicit async N/A reason."
    );
}

#[test]
fn segmented_control_state_markers_are_observable_and_have_closed_source_sets() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "SegmentedControlSelectionSource::from_indices(",
        "data-selection-origin=move || selection_origin.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should expose stable state/source marker `{needle}`."
        );
    }

    for needle in [
        "pub enum SegmentedControlControlMode",
        "pub enum SegmentedControlSelectionSource",
        "pub enum SegmentedControlSelectionOrigin",
        "\"controlled\"",
        "\"external-none\"",
        "\"external-selected\"",
        "\"external-out-of-range\"",
        "\"programmatic\"",
        "\"keyboard\"",
        "\"pointer\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl logic should constrain source marker values via `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 状态可观测、可检索、可验证："),
        "segmented_control check2 should mark observability/state-source gate as completed."
    );
}

#[test]
fn segmented_control_styles_depend_on_explicit_state_selectors() {
    let styles_source = load_source("src/segmented_control/styles.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        ".ui-segmented-control__option[data-hovered=\"true\"]",
        ".ui-segmented-control__option--focus-visible",
        ".ui-segmented-control__option:disabled",
    ] {
        assert!(
            styles_source.contains(needle),
            "SegmentedControl styles should branch by explicit state markers/selectors via `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "SegmentedControl should avoid fragile selector/runtime style anti-pattern `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
        "segmented_control check2 should mark explicit-state-style gate as completed."
    );
}

#[test]
fn segmented_control_semantics_checks_focus_on_role_aria_data_contracts() {
    let semantics_source = load_source("tests/segmented_control_semantics.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "segmented_control_emits_baseline_style_state_data_attributes",
        "segmented_control_sets_aria_orientation_and_option_fallback_label",
        "segmented_control_state_markers_are_observable_and_have_closed_source_sets",
    ] {
        assert!(
            semantics_source.contains(needle),
            "SegmentedControl semantics suite should assert contract marker `{needle}`."
        );
    }

    let forbidden_snapshot = ["assert", "_snapshot("].concat();
    assert!(
        !semantics_source.contains(&forbidden_snapshot),
        "SegmentedControl semantics suite should not rely on snapshot-only assertions."
    );

    let forbidden_insta = ["insta", "::assert_"].concat();
    assert!(
        !semantics_source.contains(&forbidden_insta),
        "SegmentedControl semantics suite should not rely on insta snapshot assertions."
    );

    assert!(
        check2_source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "segmented_control check2 should mark semantic-contract-test gate as completed."
    );
}

#[test]
fn segmented_control_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/segmented_control/view.rs");
    let semantics_source = load_source("tests/segmented_control_semantics.rs");

    for marker in [
        "role=aria.attrs.role",
        "aria-label=aria_label",
        "aria-orientation=orientation.aria_orientation()",
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(marker),
            "SegmentedControl view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "SegmentedControl semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn segmented_control_spec_file_is_not_introduced_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/segmented_control/spec.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    assert!(
        !spec_path.exists(),
        "SegmentedControl should not introduce `spec.rs` for a simple component."
    );

    assert!(
        check2_source.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "segmented_control check2 should mark spec.rs-scope gate as completed."
    );
}

#[test]
fn segmented_control_token_first_styles_are_feature_gated_in_css_aggregation() {
    let styles_source = load_source("src/segmented_control/styles.rs");
    let css_source = load_source("src/css.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "SegmentedControl should keep static CSS contract in styles.rs."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "SegmentedControl styles should remain token-first via ui-theme CSS vars."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-segmented_control\")]")
            && css_source.contains("out.push_str(crate::segmented_control::styles::CSS);"),
        "ui-components css aggregation should feature-gate segmented_control CSS injection."
    );

    assert!(
        check2_source.contains("- [x] 组件层遵循 token-first 静态样式契约："),
        "segmented_control check2 should mark token-first static-style gate as completed."
    );
}

#[test]
fn segmented_control_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let styles_source = load_source("src/segmented_control/styles.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        ".ui-segmented-control__label {",
        "font-size: var(--ui-font-size-100);",
        "font-weight: 600;",
        ".ui-segmented-control__options {",
        "background: var(--ui-bg-muted);",
        "border: 1px solid var(--ui-border);",
        "box-shadow: var(--ui-shadow-sm);",
        ".ui-segmented-control__option[data-hovered=\"true\"]:not(:disabled)",
        ".ui-segmented-control__option--focus-visible",
    ] {
        assert!(
            styles_source.contains(needle),
            "SegmentedControl default-theme visual contract should include `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline docs page should keep token `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs registry should expose theme visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "- 一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment guard token `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 默认主题美学质量达标（Visual Desire）："),
        "segmented_control check2 should mark visual desire gate as completed."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "inject-css = []",
        "web-demo-components = [",
        "all-components = [",
        "component-segmented_control = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo feature graph should include `{needle}`."
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-segmented_control\")]\npub mod segmented_control;"
        ),
        "lib.rs should feature-gate segmented_control module export."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-segmented_control\")]")
            && css_source.contains("out.push_str(crate::segmented_control::styles::CSS);"),
        "css.rs should feature-gate segmented_control CSS aggregation."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components when needed."
    );

    assert!(
        check2_source.contains("- [x] Tree Shaking 是一等能力："),
        "segmented_control check2 should mark tree-shaking gate as completed."
    );
}

#[test]
fn segmented_control_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "size regression",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            script_source.contains(needle) || budget_source.contains(needle),
            "tree-shaking script/budget contracts should include `{needle}`."
        );
    }

    for needle in [
        "component-segmented_control,inject-css",
        "MIN_TREE_ALL_COMPONENTS=NO",
        "WEB_DEMO_ALL_COMPONENTS=NO",
        "CURRENT_BYTES=2171866",
        "MAX_BYTES=3806222",
        "BUDGET_STATUS=PASS",
    ] {
        assert!(
            check2_source.contains(needle),
            "segmented_control check2 tree-shaking evidence should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "#[cfg(feature = \"component-segmented_control\")]\npub mod segmented_control;",
        "pub use root::UiRoot;",
        "pub use segmented_control::{",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib.rs should keep segmented-control/public-surface boundary `{needle}`."
        );
    }

    for forbidden in ["pub use web_sys::", "pub use leptos::web_sys::"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib.rs should not leak platform detail marker `{forbidden}`."
        );
    }

    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-segmented_control\")]\n    out.push_str(crate::segmented_control::styles::CSS);"
        ),
        "ui-components css.rs should feature-gate segmented_control CSS aggregation."
    );

    for needle in [
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme=move || state.get().theme_scheme_attr",
        "data-theme-color=move || state.get().theme_color_attr",
        "data-theme-system=move || state.get().theme_system_attr",
        "data-theme-scale=move || state.get().theme_scale_attr",
    ] {
        assert!(
            root_source.contains(needle),
            "ui-components root.rs should keep centralized theme/i18n injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight should keep shared style/motion primitive marker `{needle}`."
        );
    }

    for forbidden in [
        "segmented_control",
        "accordion",
        "tabs",
        "menu",
        "aria-",
        "data-slot",
        "use_radio(",
        "resolve_state(",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should avoid component business/a11y semantics token `{forbidden}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui-components forbidden entry file should remain absent: `{forbidden}`."
        );
    }

    for required in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "headless canonical file should exist for shared primitive boundary `{required}`."
        );
    }

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
        "segmented_control_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "SegmentedControl checklist should keep fixed-entrypoint governance marker `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/segmented_control.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "pub enum SegmentedControlControlMode",
        "pub enum SegmentedControlSelectionSource",
        "pub enum SegmentedControlSelectionOrigin",
        "SegmentedControlSelectionSource::from_indices(",
        "SegmentedControlSelectionSource::OutOfRange",
    ] {
        assert!(
            logic_source.contains(needle),
            "SegmentedControl should model discrete state axes via typed enums/contracts `{needle}`."
        );
    }

    for needle in [
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        ".as_attr()",
        "data-selection-origin=move || selection_origin.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl should expose machine-readable semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "data-selection-source=move || format!(",
        "data-selection-origin=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl semantic marker values should be closed sets, not free-form strings `{forbidden}`."
        );
    }

    for needle in [
        "let selected_index = input",
        ".selected_index",
        ".filter(|index| *index < input.item_count);",
    ] {
        assert!(
            primitive_source.contains(needle),
            "SegmentedControl primitive should normalize invalid state in `logic.rs` boundary via `{needle}`."
        );
    }

    assert!(
        check2_source
            .contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "segmented_control check2 should mark type-system + semantic-marker gate as completed."
    );
}

#[test]
fn segmented_control_platform_paths_are_cfg_gated_and_non_wasm_safe() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/segmented_control/view.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    assert!(
        cargo_source.contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]")
            && cargo_source.contains("web-sys = { version = "),
        "ui-components should keep browser dependencies behind wasm32 target dependency gating."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn focus_option(option_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize)",
        "fn focus_option(_option_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should keep explicit platform branch gating via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_indicator_motion(",
        "drop(sanitize_motion(motion));",
        "leptos::web_sys::ResizeObserver",
    ] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion should keep wasm/non-wasm split and deterministic non-wasm fallback via `{needle}`."
        );
    }

    for forbidden in ["use web_sys", "web_sys::"] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl non-wasm-safe view path should not directly bind browser-only API `{forbidden}`."
        );
    }

    assert!(
        check2_source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "segmented_control check2 should mark SSR/cross-platform gate as completed."
    );
}

#[test]
fn segmented_control_headless_web_ssr_mutual_exclusion_is_compile_error_guarded() {
    let view_source = load_source("src/segmented_control/view.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should enforce web/ssr mutual exclusion via `{needle}`."
        );
    }

    assert!(
        view_source.contains("use_radio(RadioOptions {"),
        "SegmentedControl should keep consuming ui-headless contracts while respecting headless feature constraints."
    );

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "--no-default-features --features web",
        "--no-default-features --features ssr",
        "--no-default-features --features web,ssr",
        "mutually exclusive; enable exactly one",
    ] {
        assert!(
            check2_source.contains(needle),
            "segmented_control check2 should preserve headless web/ssr mutual exclusion evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_motion_non_wasm_noop_contract_is_enforced() {
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_checks = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should provide non-wasm no-op web backend contract `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_indicator_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion should safely degrade on non-wasm path via `{needle}`."
        );
    }

    assert!(
        !motion_source.contains("panic!("),
        "SegmentedControl non-wasm motion path should not panic."
    );

    for needle in [
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_checks.contains(needle),
            "ui-motion non-wasm stub test contract should include `{needle}`."
        );
    }

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "cargo check -p ui-motion --no-default-features",
        "cargo check -p ui-motion --target wasm32-unknown-unknown --no-default-features",
        "cargo test -p ui-motion --no-default-features",
        "cargo clippy -p ui-motion --all-targets --no-default-features -- -D warnings",
        "non_wasm_web_backend_animate_is_safe_noop",
    ] {
        assert!(
            check2_source.contains(needle),
            "segmented_control check2 should preserve ui-motion non-wasm no-op evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_reduced_motion_ssr_and_wasm_branches_keep_semantic_contract() {
    let spring_source = load_source("../ui-motion/src/spring.rs");
    let web_motion_source = load_source("../ui-motion/src/web.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let ui_motion_spring_checks = load_source("../ui-motion/tests/spring.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            spring_source.contains(needle) || web_motion_source.contains(needle),
            "ui-motion should keep reduced-motion downgrade path via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_indicator_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "SegmentedControl motion should keep wasm enhancement + non-wasm fallback split via `{needle}`."
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_checks.contains(needle),
            "ui-motion spring tests should lock reduced-motion behavior via `{needle}`."
        );
    }

    for needle in [
        "role=aria.attrs.role",
        "aria-orientation=orientation.aria_orientation()",
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-slot=SLOT_ROOT",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl semantic attrs should remain stable across SSR/wasm branches via `{needle}`."
        );
    }

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "cargo test -p ui-motion --test spring --no-default-features",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-segmented_control,inject-css",
        "cargo check -p ui-components --no-default-features --features component-segmented_control,inject-css",
    ] {
        assert!(
            check2_source.contains(needle),
            "segmented_control check2 should preserve reduced-motion/SSR/wasm evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_performance_governance_budget_is_defined_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let forms_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/segmented_control/view.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep shared performance budget/probe contract `{needle}`."
        );
    }

    for needle in ["title=\"SegmentedControl\"", "slug=\"segmented-control\""] {
        assert!(
            forms_source.contains(needle),
            "SegmentedControl docs page should stay wired into shared perf probe path via `{needle}`."
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs coverage e2e should keep perf regression guard `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should keep shared blocking/follow-up guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "resolve_state(SegmentedControlStateInput {",
        "motion::attach_indicator_motion(",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should expose state/render/motion attribution contract `{needle}`."
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "关键交互组件预算采用 docs-app 共享 `UiPerfProbe`",
        "`render_count` 自动化暂未具备通用基建",
        "当前采用可重复等价证据并持续跟踪",
        "渲染次数预算为 `1`",
        "Button",
        "Input",
    ] {
        assert!(
            check2_source.contains(needle),
            "segmented_control check2 should preserve performance-governance evidence `{needle}`."
        );
    }
}

#[test]
fn segmented_control_view_macro_complexity_is_split_into_semantic_helpers() {
    let check2_source = load_source("src/segmented_control/check2.md");
    let view_source = load_source("src/segmented_control/view.rs");

    assert!(
        check2_source.contains("- [x] `view!` 宏复杂度受控："),
        "SegmentedControl check2 should mark `view!` macro complexity gate as completed."
    );

    for needle in [
        "fn option_label_for_index(",
        "fn render_option_button(",
        "fn render_label(",
        "render_option_button(",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should keep semantic split helper `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 3,
        "SegmentedControl should keep `view!` blocks bounded after semantic split; expected <= 3, found {view_macro_count}."
    );
}

#[test]
fn segmented_control_prefers_function_split_without_local_component_noise() {
    let check2_source = load_source("src/segmented_control/check2.md");
    let view_source = load_source("src/segmented_control/view.rs");

    assert!(
        check2_source.contains("- [x] 函数式拆分优先："),
        "SegmentedControl check2 should mark function-split gate as completed."
    );

    for needle in [
        "fn option_label_for_index(",
        "fn render_option_button(",
        "fn render_label(",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl view should keep plain function split marker `{needle}`."
        );
    }

    let component_attr_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "SegmentedControl should expose one public component entry and keep sub-fragments as plain functions."
    );
}

#[test]
fn segmented_control_static_fragments_are_constantized_and_attached_via_stable_markers() {
    let check2_source = load_source("src/segmented_control/check2.md");
    let view_source = load_source("src/segmented_control/view.rs");

    assert!(
        check2_source.contains("- [x] 静态片段常量化："),
        "SegmentedControl check2 should mark static-fragment constantization gate as completed."
    );

    for needle in [
        "const ROOT_CLASS: &str = \"ui-segmented-control\";",
        "const LABEL_CLASS: &str = \"ui-segmented-control__label\";",
        "const OPTIONS_CLASS: &str = \"ui-segmented-control__options\";",
        "const OPTION_CLASS: &str = \"ui-segmented-control__option\";",
        "const OPTION_LABEL_CLASS: &str = \"ui-segmented-control__option-label\";",
        "const SLOT_ROOT: &str = \"segmented-control\";",
        "const SLOT_LABEL: &str = \"segmented-control-label\";",
        "const SLOT_OPTIONS: &str = \"segmented-control-options\";",
        "const SLOT_OPTION: &str = \"segmented-control-option\";",
        "const SLOT_INDICATOR: &str = \"segmented-control-indicator\";",
        "class=OPTION_CLASS",
        "class=OPTION_LABEL_CLASS",
        "class=OPTIONS_CLASS",
        "data-slot=SLOT_ROOT",
        "data-slot=SLOT_LABEL",
        "data-slot=SLOT_OPTIONS",
        "data-slot=SLOT_OPTION",
        "data-slot=SLOT_INDICATOR",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl static fragment contract should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "data-slot=\"segmented-control\"",
        "data-slot=\"segmented-control-label\"",
        "data-slot=\"segmented-control-options\"",
        "data-slot=\"segmented-control-option\"",
        "data-slot=\"segmented-control-indicator\"",
        "class=\"ui-segmented-control__option\"",
        "class=\"ui-segmented-control__label\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl should avoid scattered inline static fragment literal `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "SegmentedControl should not use html injection path `{forbidden}` in component/docs paths.",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            check2_source.contains(needle),
            "SegmentedControl check2 should preserve inner_html governance marker `{needle}`."
        );
    }
}

#[test]
fn segmented_control_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let components_lib_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            components_lib_source.contains(needle),
            "ui-components should keep shared wasm debug isolation entrypoint `{needle}`."
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
            "docs app should keep debug visual entry marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "UiTraceEventKind::Inspect",
        "UiTraceEventKind::Note",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "shared trace timeline contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "on:click=move |_| {",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(needle),
            "SegmentedControl should keep machine-readable state/interaction marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "segmented_control-wasm-debug",
        "segmented-control-wasm-debug",
        "wasm_debug_proxy!",
        "trace.emit(",
        "request_replay.run(",
        "#[prop(optional)] debug",
        "data-debug-source",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !cargo_source.contains(forbidden),
            "SegmentedControl should not introduce component-private wasm debug runtime marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "SegmentedControl check2 should preserve wasm debug governance marker `{needle}`."
        );
    }
}

#[test]
fn segmented_control_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

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
        "pub(super) fn segmented_control() -> AnyView {",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "<Playground title=\"Vertical + Disabled + Empty\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "SegmentedControl docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");
    let check2_source = load_source("src/segmented_control/check2.md");
    let dev_docs_script = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script = load_source("../../scripts/dev-web-demo.sh");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    let docs_segmented_section = docs_source
        .split("pub(super) fn segmented_control() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("forms docs page should define segmented_control() section"));

    for needle in [
        "title=\"Selection + Root State\"",
        "title=\"Vertical + Disabled + Empty\"",
        "orientation=SegmentedControlOrientation::Vertical",
        "size=SegmentedControlSize::Sm",
        "disabled=true",
        "disabled_indices=vec![2]",
    ] {
        assert!(
            docs_segmented_section.contains(needle),
            "SegmentedControl docs should keep context-visible state-matrix marker `{needle}`."
        );
    }

    for forbidden in [
        "SEGMENTED_CONTROL_WORKBENCH_STORAGE_KEY",
        "load_segmented_control_workbench_state(",
        "save_segmented_control_workbench_state(",
        "clear_segmented_control_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_segmented_section.contains(forbidden),
            "SegmentedControl keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "compose_scoped_css + data-playground-scope + Show test + Restore original CSS",
        "可选状态保留在本组件文档场景按 N/A 处理",
    ] {
        assert!(
            check2_source.contains(required),
            "SegmentedControl checklist should keep DX governance marker `{required}`."
        );
    }

    for needle in ["#!/usr/bin/env bash", "trunk serve --open true"] {
        assert!(
            dev_docs_script.contains(needle) && dev_web_script.contains(needle),
            "dev scripts should keep fast local iteration entry `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn segmented_control_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");
    let check2_source = load_source("src/segmented_control/check2.md");

    assert!(
        !manifest_dir.join("src/segmented_control/spec.rs").exists(),
        "SegmentedControl should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-segmented_control = []"),
        "SegmentedControl feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-segmented_control = [\"dep:serde\"")
            && !cargo_source.contains("component-segmented_control = [\"dep:serde_json\""),
        "SegmentedControl should not opt into serde/spec migration dependencies without explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "spec_schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "SegmentedControl engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "`SegmentedControl` 为简单选择组件，spec/serde 迁移路径按 N/A 管理",
        "segmented_control_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "SegmentedControl checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn segmented_control_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/segmented_control/mod.rs"),
        load_source("src/segmented_control/logic.rs"),
        load_source("src/segmented_control/view.rs"),
        load_source("src/segmented_control/styles.rs"),
        load_source("src/segmented_control/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("segmented_control-wasm-debug")
            && !cargo_source.contains("segmented-control-wasm-debug"),
        "SegmentedControl should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::segmented_control::",
        "const SEGMENTED_CONTROL_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "SegmentedControl should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn segmented_control_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/segmented_control/mod.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let view_source = load_source("src/segmented_control/view.rs");
    let styles_source = load_source("src/segmented_control/styles.rs");
    let motion_source = load_source("src/segmented_control/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
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
                "SegmentedControl engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "SegmentedControl public module boundary should not leak web_sys types."
    );
}

#[test]
fn segmented_control_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn segmented_control_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let primitives_source =
        load_source("../../crates/ui-state-primitives/src/segmented_control.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for marker in [
        "#[component]",
        "pub fn SegmentedControl(",
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "resolve_state(SegmentedControlStateInput {",
        "role=aria.attrs.role",
        "aria-label=aria_label",
        "aria-orientation=orientation.aria_orientation()",
        "data-slot=SLOT_ROOT",
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
    ] {
        assert!(
            view_source.contains(marker),
            "SegmentedControl snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub struct SegmentedControlStateInput",
        "pub struct SegmentedControlState",
        "pub fn resolve_state(input: SegmentedControlStateInput<'_>) -> SegmentedControlState",
        "pub enum SegmentedControlSelectionSource",
        "pub enum SegmentedControlSelectionOrigin",
    ] {
        assert!(
            logic_source.contains(marker) || primitives_source.contains(marker),
            "SegmentedControl snapshot baseline should keep normalization/state marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn segmented_control() -> AnyView",
        "title=\"SegmentedControl\"",
        "slug=\"segmented-control\"",
        "<Playground title=\"Selection + Root State\" code_signal=code>",
        "id_base=\"docs-segments\".to_string()",
        "<Playground title=\"Vertical + Disabled + Empty\" code_signal=states_code>",
        "id_base=\"docs-segments-vertical\".to_string()",
        "id_base=\"docs-segments-empty\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "SegmentedControl docs should keep complete snapshot result marker `{marker}`."
        );
    }
}

#[test]
fn segmented_control_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/segmented_control/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "SegmentedControl 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
    ] {
        assert!(
            checklist_source.contains(required),
            "SegmentedControl checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn segmented_control_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/segmented_control/view.rs");

    for required in [
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-label=aria_label",
        "aria-labelledby=label_id",
        "aria-orientation=orientation.aria_orientation()",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
        "data-control-mode=SegmentedControlControlMode::Controlled.as_attr()",
        "data-selection-source=move || {",
        "data-selection-origin=move || selection_origin.get().as_attr()",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
    ] {
        assert!(
            view_source.contains(required),
            "SegmentedControl should keep continuous role/aria/data semantics via `{required}` in snapshot-only optional-streaming scope."
        );
    }

    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
        "data-output-status",
        "data-stream-status",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SegmentedControl should not mount fake streaming status field `{forbidden}` when stream protocol is N/A."
        );
    }
}

#[test]
fn segmented_control_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()
{
    let view_source = load_source("src/segmented_control/view.rs");
    let logic_source = load_source("src/segmented_control/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "SegmentedControl should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}
