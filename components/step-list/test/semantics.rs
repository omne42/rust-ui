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
fn step_list_check2_documents_architecture_and_api_first_eight_rules() {
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "`status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "`ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "`ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "`ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "`ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
    ] {
        assert!(
            checklist_source.contains(required),
            "StepList checklist should keep governance rule `{required}`."
        );
    }
}

#[test]
fn step_list_check2_tracks_platform_contract_items() {
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "`ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
    ] {
        assert!(
            checklist_source.contains(required),
            "StepList checklist should keep platform governance rule `{required}`."
        );
    }
}

#[test]
fn step_list_state_primitives_are_sunk_to_ui_state_primitives_and_exported() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");
    let primitives_lib_source = load_source("../../crates/ui-state-primitives/src/lib.rs");
    let mod_source = load_source("src/step_list/mod.rs");
    let logic_source = load_source("src/step_list/logic.rs");

    assert!(
        path_exists("../../crates/ui-state-primitives/src/step_list.rs"),
        "step-list primitive module should exist in ui-state-primitives."
    );
    assert!(
        primitives_lib_source.contains("pub mod step_list;"),
        "ui-state-primitives lib.rs should export step_list module."
    );

    for needle in [
        "pub enum StepListOrientation",
        "pub enum StepListSize",
        "pub struct StepListItem",
        "pub struct StepListStateInput",
        "pub struct StepListState",
        "pub struct StepListItemStateInput",
        "pub struct StepListItemState",
        "pub fn normalize_items(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn resolve_item_state(",
        "pub fn normalize_completed_indices(",
        "pub fn next_enabled_index(",
    ] {
        assert!(
            primitives_source.contains(needle),
            "step-list primitive should expose `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        mod_source.contains("pub use ui_state_primitives::step_list::"),
        "step-list mod.rs should re-export primitive types from ui-state-primitives."
    );
    for required in [
        "primitives::normalize_optional_text(value)",
        "primitives::normalize_items(items)",
        "primitives::normalize_aria_label(value)",
        "primitives::sanitize_index(index, item_count)",
        "primitives::resolve_selected_index(items, selected_index)",
        "primitives::normalize_completed_indices(item_count, completed_indices)",
        "primitives::first_enabled_index(items)",
        "primitives::resolve_state(input)",
        "primitives::resolve_item_state(input)",
    ] {
        assert!(
            logic_source.contains(required),
            "step-list logic.rs should forward state primitives through `{required}`."
        );
    }
}

#[test]
fn step_list_primitives_stay_pojo_and_framework_agnostic() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");

    for forbidden in [
        "leptos",
        "Signal",
        "ReadSignal",
        "WriteSignal",
        "StoredValue",
        "Callback",
        "view!",
        "web_sys",
        "Html",
        "on:click",
        "on:keydown",
        ".ui-step-list",
    ] {
        assert!(
            !primitives_source.contains(forbidden),
            "step-list state primitives must stay POJO and framework-agnostic; found `{forbidden}`."
        );
    }
}

#[test]
fn step_list_headless_contract_exists_and_view_consumes_it() {
    let headless_source = load_source("../../crates/ui-headless/src/step_list.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let view_source = load_source("src/step_list/view.rs");

    assert!(
        headless_lib_source.contains("pub mod step_list;"),
        "ui-headless lib.rs should export step_list module."
    );

    for needle in [
        "pub struct StepListRootA11yAttrs",
        "pub struct StepListItemA11yInput",
        "pub struct StepListItemA11yAttrs",
        "pub struct StepListItemSemanticState",
        "pub struct StepListItemContract",
        "pub fn step_list_root_a11y_attrs(",
        "pub fn step_list_item_contract(",
        "pub fn resolve_step_list_next_index(",
    ] {
        assert!(
            headless_source.contains(needle),
            "step-list headless contract should include `{needle}`."
        );
    }

    for forbidden in [
        ".ui-step-list",
        "color:",
        "background:",
        "transition:",
        "animation:",
        "@keyframes",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless step-list should not own style/motion orchestration token `{forbidden}`."
        );
    }

    for required in [
        "use ui_headless::{",
        "step_list_root_a11y_attrs",
        "step_list_item_contract",
        "resolve_step_list_next_index",
        "let root_a11y = step_list_root_a11y_attrs(aria_label, lang, dir);",
        "let item_contract = step_list_item_contract(ui_headless::StepListItemA11yInput {",
    ] {
        assert!(
            view_source.contains(required),
            "step-list view should consume headless contract marker `{required}`."
        );
    }

    for forbidden in [
        "match ev.key().as_str()",
        "\"ArrowRight\" if orientation == StepListOrientation::Horizontal",
        "logic::next_enabled_index(&items, index, 1)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list keyboard model should be normalized in ui-headless, not in view.rs (`{forbidden}`)."
        );
    }
}

#[test]
fn step_list_motion_boundary_stays_component_na_and_no_runtime_engine_reimplementation() {
    let mod_source = load_source("src/step_list/mod.rs");
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");

    assert!(
        !path_exists("src/step_list/motion.rs"),
        "step-list should not add motion.rs when reusable motion contract is N/A."
    );

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "Spring",
        "keyframe",
        "request_animation_frame",
        "cancel_animation_frame",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "step-list should not re-implement motion engine token `{forbidden}` in component layer."
        );
    }
}

#[test]
fn step_list_platform_contract_depends_on_headless_mutual_exclusion_and_motion_noop_stub() {
    let headless_source = load_source("../../crates/ui-headless/src/lib.rs");
    let motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless should keep web/ssr mutual exclusion guard `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_source.contains(required),
            "ui-motion should expose predictable non-wasm no-op contract `{required}`."
        );
    }
}

#[test]
fn step_list_reduced_motion_ssr_wasm_contract_stays_semantically_stable() {
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let styles_source = load_source("src/step_list/styles.rs");

    assert!(
        !path_exists("src/step_list/motion.rs"),
        "step-list has no component-level motion adapter, so reduced-motion path should stay deterministic no-op."
    );

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::",
        "request_animation_frame",
        "window()",
        "web_sys::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "step-list semantics should not split across platform branches (`{forbidden}`)."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-status=item_state.status_attr",
        "aria-current=item_contract.attrs.aria_current",
        "aria-disabled=item_contract.attrs.aria_disabled",
    ] {
        assert!(
            view_source.contains(required),
            "step-list should expose stable semantic markers across SSR/wasm (`{required}`)."
        );
    }

    for forbidden in ["animation:", "transition:"] {
        assert!(
            !styles_source.contains(forbidden),
            "step-list styles should avoid runtime animation dependencies when motion adapter is N/A (`{forbidden}`)."
        );
    }
}

#[test]
fn step_list_performance_governance_contract_is_budgeted_repeatable_and_attributable() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    assert!(
        checklist_source.contains(
            "性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "step-list checklist should keep explicit performance governance gate."
    );

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-completed-count=move || state.get().completed_count.to_string()",
        "data-disabled-count=move || state.get().disabled_count.to_string()",
        "data-emphasis-source=move || state.get().emphasis_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "step-list should expose attributable performance marker `{needle}`."
        );
    }

    let view_memo_count = view_source.matches("Memo::new(").count();
    assert!(
        view_memo_count <= 3,
        "step-list reactive budget exceeded: expected <= 3 `Memo::new`, found {view_memo_count}."
    );

    let view_signal_derive_count = view_source.matches("Signal::derive(").count();
    assert_eq!(
        view_signal_derive_count, 0,
        "step-list should avoid ad-hoc derive chains in view; found {view_signal_derive_count}."
    );

    let view_effect_count = view_source.matches("Effect::new(").count();
    assert_eq!(
        view_effect_count, 0,
        "step-list view should avoid effect loops for baseline perf stability; found {view_effect_count}."
    );

    for forbidden in [
        "performance.now",
        "request_animation_frame",
        "set_interval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list should avoid timer-driven perf drift token `{forbidden}` in view path."
        );
    }

    assert!(
        todo_source.contains("render_count"),
        "performance governance should keep explicit render_count follow-up tracking in docs/plan/TODO.md."
    );
}

#[test]
fn step_list_inner_html_contract_blocks_untrusted_injection() {
    let checklist_source = load_source("src/step_list/check2.md");
    let mod_source = load_source("src/step_list/mod.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let view_source = load_source("src/step_list/view.rs");
    let styles_source = load_source("src/step_list/styles.rs");

    assert!(
        checklist_source.contains(
            "`inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"
        ),
        "step-list checklist should keep inner_html safety governance gate."
    );

    for forbidden in [
        "inner_html=",
        "inner_html =",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "step-list should block untrusted html-injection path token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let components_cargo_source = load_source("Cargo.toml");

    assert!(
        checklist_source.contains(
            "WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"
        ),
        "step-list checklist should keep wasm-debug governance gate."
    );

    for required in [
        "let trace = ui_headless::use_ui_trace();",
        "emit_selection_trace(",
        "ui_headless::UiTraceEventKind::Note",
        "\"intent=selection;source={source};index={index};prev={};next={}\"",
        "\"pointer\"",
        "\"keyboard\"",
    ] {
        assert!(
            view_source.contains(required),
            "step-list wasm debug contract should keep trace/replay marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
    ] {
        assert!(
            docs_lib_source.contains(required),
            "docs app should keep debug-trace bootstrap marker `{required}`."
        );
    }

    assert!(
        debug_overlay_source.contains("ui_headless::UiTraceEventKind::Note"),
        "debug overlay should render shared trace-note events for replay/inspection."
    );

    for forbidden in ["step_list-wasm-debug", "step-list-wasm-debug"] {
        assert!(
            !components_cargo_source.contains(forbidden),
            "step-list wasm debug switch must remain feature-isolated from public API (`{forbidden}`)."
        );
    }
}

#[test]
fn step_list_view_macro_complexity_is_split_into_semantic_helpers() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");

    assert!(
        checklist_source.contains(
            "`view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"
        ),
        "step-list checklist should keep view-macro complexity governance gate."
    );

    for required in [
        "fn render_step_list_items(",
        "fn render_step_list_item(",
        "render_step_list_items(",
    ] {
        assert!(
            view_source.contains(required),
            "step-list view should keep split render helper marker `{required}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 3,
        "step-list view macro blocks should stay bounded after semantic split; expected <= 3, found {view_macro_count}."
    );
}

#[test]
fn step_list_prefers_function_split_without_local_component_noise() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");

    assert!(
        checklist_source.contains(
            "函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"
        ),
        "step-list checklist should keep function-split governance gate."
    );

    for required in [
        "fn render_item_description(",
        "fn render_step_list_item(",
        "fn render_step_list_items(",
    ] {
        assert!(
            view_source.contains(required),
            "step-list view should keep plain function split marker `{required}`."
        );
    }

    let component_attr_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "step-list should expose only one component entry and keep helper fragments as plain functions."
    );
}

#[test]
fn step_list_static_fragments_are_constantized_and_attached_via_stable_markers() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");

    assert!(
        checklist_source.contains(
            "静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"
        ),
        "step-list checklist should keep static-fragment constantization governance gate."
    );

    for required in [
        "const SLOT_ROOT: &str = \"step-list\";",
        "const SLOT_DESCRIPTION: &str = \"step-list-description\";",
        "const CLASS_BUTTON: &str = \"ui-step-list__button\";",
        "const CLASS_DESCRIPTION: &str = \"ui-step-list__description\";",
        "data-slot=SLOT_ROOT",
        "data-slot=SLOT_DESCRIPTION",
        "class=CLASS_BUTTON",
        "class=CLASS_DESCRIPTION",
    ] {
        assert!(
            view_source.contains(required),
            "step-list static fragment contract should keep marker `{required}`."
        );
    }

    for forbidden in [
        "data-slot=\"step-list",
        "class=\"ui-step-list__button\"",
        "class=\"ui-step-list__description\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list should avoid scattered inline static fragment literal `{forbidden}`."
        );
    }
}

#[test]
fn step_list_styles_are_token_first_and_theme_consuming() {
    let styles_source = load_source("src/step_list/styles.rs");

    for required in [
        "var(--ui-space-sm)",
        "var(--ui-fg-muted)",
        "var(--ui-fg)",
        "var(--ui-accent)",
        "var(--ui-success)",
        "var(--ui-bg)",
        "var(--ui-radius-sm)",
    ] {
        assert!(
            styles_source.contains(required),
            "step-list styles should consume theme token `{required}`."
        );
    }

    for forbidden in ["rgb(", "hsl(", "styled(", "tailwind", "@apply"] {
        assert!(
            !styles_source.contains(forbidden),
            "step-list styles should avoid private hardcoded token/style system `{forbidden}`."
        );
    }
}

#[test]
fn step_list_component_layer_is_assembly_only_and_public_api_hides_platform_details() {
    let mod_source = load_source("src/step_list/mod.rs");
    let view_source = load_source("src/step_list/view.rs");

    for forbidden in ["web_sys", "HtmlElement", "JsValue", "Window", "Document"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "step-list public component API/layer should not leak platform detail type `{forbidden}`."
        );
    }

    for required in [
        "logic::resolve_state(StepListStateInput {",
        "logic::resolve_item_state(StepListItemStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(required),
            "step-list view should consume normalized logic output marker `{required}`."
        );
    }
}

#[test]
fn step_list_api_naming_uses_is_on_default_prefixes_and_no_alias_drift() {
    let view_source = load_source("src/step_list/view.rs");

    for required in [
        "#[prop(optional)] is_emphasized: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            view_source.contains(required),
            "step-list API naming should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] emphasized: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] on_selected_change:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list should remove naming alias drift token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_controlled_uncontrolled_axis_is_paired_and_half_controlled_behavior_is_blocked() {
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");

    for required in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "headless::use_controllable_state(",
        "selection_axis.selected_index",
        "Some(selection_axis.default_selected_index)",
        "selection_axis.on_selected_index_change",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "step-list controlled/uncontrolled axis should keep paired contract marker `{required}`."
        );
    }

    assert!(
        !view_source.contains("set_selected_index")
            && !view_source.contains("signal(default_selected_index"),
        "step-list should not secretly write local state in controlled mode path."
    );
}

#[test]
fn step_list_default_values_are_normalized_in_logic_and_view_does_not_scatter_fallbacks() {
    let logic_source = load_source("src/step_list/logic.rs");
    let view_source = load_source("src/step_list/view.rs");

    for required in [
        "pub fn normalize_id_base(id_base: Option<String>) -> String",
        "pub fn normalize_selection_axis(input: StepListSelectionAxisInput) -> StepListSelectionAxis",
        "let default_selected_index = sanitize_index(input.default_selected_index, input.item_count);",
    ] {
        assert!(
            logic_source.contains(required),
            "step-list logic should centralize default normalization marker `{required}`."
        );
    }

    for required in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let selection_axis = logic::normalize_selection_axis(logic::StepListSelectionAxisInput {",
    ] {
        assert!(
            view_source.contains(required),
            "step-list view should consume logic-normalized defaults marker `{required}`."
        );
    }

    for forbidden in [
        "unwrap_or_else(|| \"ui-step-list\".to_string())",
        "sanitize_index(default_selected_index",
        "default_selected_index.unwrap_or",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list view should not host default fallback decision token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn step_list() -> AnyView",
        "title=\"StepList\"",
        "slug=\"step-list\"",
        "description=\"baseline-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts.\"",
        "<Playground title=\"Controlled Selection\" code_signal=code>",
        "<Playground title=\"Vertical + Emphasized + Disabled\" code_signal=states_code>",
        "<StepList",
        "on_selected_index_change=on_selected_index_change",
        "orientation=StepListOrientation::Vertical",
        "size=StepListSize::L",
        "is_emphasized=true",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for step-list coverage.",
        );
    }
}

#[test]
fn step_list_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "StepListItem::new(\"account\", \"Account\").described(\"Create account and verify email\")",
        "StepListItem::new(\"shipping\", \"Shipping\").described(\"Choose shipping address\")",
        "StepListItem::new(\"payment\", \"Payment\").described(\"Add payment method\")",
        "StepListItem::new(\"review\", \"Review\").described(\"Confirm and place order\")",
        "StepListItem::new(\"plan\", \"Plan\").described(\"Pick your subscription tier\")",
        "StepListItem::new(\"profile\", \"Profile\").described(\"Fill organization details\")",
        "StepListItem::new(\"billing\", \"Billing\")",
        ".disabled(true)",
        "StepListItem::new(\"launch\", \"Launch\").described(\"Start using the workspace\")",
        "let (selected_index, set_selected_index) = signal(Some(1_usize));",
        "completed_indices=vec![0]",
        "default_selected_index=3",
        "on_selected_index_change=on_selected_index_change",
        "class_name=\"docs-step-list-custom\".to_string()",
        "aria_label=\"Workspace setup steps\".to_string()",
        "\"selected index: \"",
    ] {
        assert!(
            source.contains(needle),
            "step-list docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn step_list_check2_documents_state_and_contract_rules_batch() {
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "测试验证“语义契约”而不只验证视觉快照。",
        "组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "`spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
    ] {
        assert!(
            checklist_source.contains(required),
            "step-list checklist should keep governance rule `{required}`."
        );
    }
}

#[test]
fn step_list_state_normalization_and_type_constraints_are_centralized() {
    let logic_source = load_source("src/step_list/logic.rs");
    let view_source = load_source("src/step_list/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");

    for required in [
        "pub fn normalize_selection_axis(input: StepListSelectionAxisInput) -> StepListSelectionAxis",
        "pub fn resolve_state(input: StepListStateInput) -> StepListState",
        "pub fn resolve_item_state(input: StepListItemStateInput) -> StepListItemState",
        "logic::normalize_selection_axis(logic::StepListSelectionAxisInput {",
        "logic::resolve_state(StepListStateInput {",
        "logic::resolve_item_state(StepListItemStateInput {",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "step-list state normalization should keep marker `{required}`."
        );
    }

    for required in [
        "pub enum StepListOrientation",
        "pub enum StepListSize",
        "pub struct StepListState",
        "pub struct StepListItemState",
    ] {
        assert!(
            primitives_source.contains(required),
            "step-list discrete/state axes should stay type constrained via `{required}`."
        );
    }

    for forbidden in [
        "match state.get().",
        "if selected_index.is_none() {",
        "if default_selected_index.is_some() {",
        "status =",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list view should avoid scattered state-machine reconstruction token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_async_contract_is_explicitly_na_for_component_scope() {
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "on_retry",
        "use_async_action",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "step-list async contract is N/A (no remote async workflow); forbidden token `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn step_list_dx_paradox_docs_keep_default_path_simple_and_no_internal_state_required() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for required in [
        "pub(super) fn step_list() -> AnyView",
        "<Playground title=\"Controlled Selection\" code_signal=code>",
        "<Playground title=\"Vertical + Emphasized + Disabled\" code_signal=states_code>",
        "<StepList",
        "completed_indices=vec![0]",
    ] {
        assert!(
            docs_source.contains(required),
            "step-list DX docs should keep simple default/advanced path marker `{required}`."
        );
    }

    for forbidden in [
        "state=",
        "state: StepList",
        "use ui_state_primitives::step_list",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "step-list docs should not require wiring internal primitive state object `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_composition_contract_uses_typed_item_spec_without_parallel_arrays() {
    let view_source = load_source("src/step_list/view.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");

    for required in [
        "steps: ReadSignal<Vec<StepListItem>>",
        "pub struct StepListItem",
        "StepListItem::new(",
    ] {
        assert!(
            view_source.contains(required) || primitives_source.contains(required),
            "step-list should keep typed item-spec composition marker `{required}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children=labels",
        "labels + children",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "step-list should avoid implicit parallel-array composition token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_a11y_i18n_l10n_contract_is_headless_driven() {
    let view_source = load_source("src/step_list/view.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for required in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let root_a11y = step_list_root_a11y_attrs(aria_label, lang, dir);",
        "role=root_role",
        "aria-label=root_aria_label",
        "lang=root_lang.clone()",
        "dir=root_dir",
    ] {
        assert!(
            view_source.contains(required),
            "step-list should mount a11y/i18n contract marker `{required}`."
        );
    }

    assert!(
        headless_a11y_source.contains("pub fn locale_attrs("),
        "ui-headless should keep shared locale helper for lang/dir semantics."
    );
}

#[test]
fn step_list_semantic_markers_and_styles_contract_are_explicit_and_stable() {
    let view_source = load_source("src/step_list/view.rs");
    let styles_source = load_source("src/step_list/styles.rs");

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-emphasis-source=move || state.get().emphasis_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-status=item_state.status_attr",
    ] {
        assert!(
            view_source.contains(required),
            "step-list should expose observable semantic marker `{required}`."
        );
    }

    for required in [
        ".ui-step-list__item[data-status=\"current\"]",
        ".ui-step-list__item[data-status=\"completed\"]",
        ".ui-step-list[data-orientation=\"vertical\"]",
        ".ui-step-list[data-size=\"xl\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "step-list styles should depend on explicit semantic marker `{required}`."
        );
    }

    for forbidden in [":nth-child(", " > * > * > ", ":has("] {
        assert!(
            !styles_source.contains(forbidden),
            "step-list styles should avoid fragile DOM-guess selector `{forbidden}`."
        );
    }
}

#[test]
fn step_list_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/semantics.rs");

    for required in [
        "step_list_headless_contract_exists_and_view_consumes_it",
        "step_list_performance_governance_contract_is_budgeted_repeatable_and_attributable",
        "step_list_inner_html_contract_blocks_untrusted_injection",
        "step_list_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated",
    ] {
        assert!(
            semantics_source.contains(required),
            "step-list semantic suite should keep contract-first assertion `{required}`."
        );
    }

    for forbidden in [
        ["assert", "_snapshot"].concat(),
        ["image", "_snapshot"].concat(),
        [".toMatch", "Snapshot("].concat(),
    ] {
        assert!(
            !semantics_source.contains(&forbidden),
            "step-list semantic suite should not rely on visual snapshot primary signal `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_component_directory_and_file_responsibility_contract_hold() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/step_list");
    let mod_source = load_source("src/step_list/mod.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let styles_source = load_source("src/step_list/styles.rs");
    let view_source = load_source("src/step_list/view.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "check2.md"] {
        assert!(
            component_dir.join(required).exists(),
            "step-list component directory should include `{required}`."
        );
    }

    for forbidden in ["motion.rs", "render.rs", "spec.rs"] {
        assert!(
            !component_dir.join(forbidden).exists(),
            "step-list should not introduce `{forbidden}` in current scope."
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::StepList;",
    ] {
        assert!(
            mod_source.contains(required),
            "step-list mod.rs should keep stable export marker `{required}`."
        );
    }

    for forbidden in ["on:click", "on:keydown", "view! {", ".ui-step-list"] {
        assert!(
            !logic_source.contains(forbidden),
            "step-list logic.rs should stay normalization-only without `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "step-list styles.rs should keep static css contract."
    );

    for forbidden in ["pub const CSS", "unwrap_or(", "sanitize_index("] {
        assert!(
            !view_source.contains(forbidden),
            "step-list view.rs should avoid logic/style drift token `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_tree_shaking_feature_gates_and_css_aggregation_contract_hold() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for required in [
        "component-step_list = []",
        "#[cfg(feature = \"component-step_list\")]",
        "pub mod step_list;",
        "#[cfg(feature = \"component-step_list\")]",
        "out.push_str(crate::step_list::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(required)
                || lib_source.contains(required)
                || css_source.contains(required),
            "step-list tree-shaking contract should keep marker `{required}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-step_list\")]\npub mod step_list;")
            || lib_source
                .contains("#[cfg(feature = \"component-step_list\")]\r\npub mod step_list;"),
        "step-list module export should remain feature-gated in lib.rs."
    );
}

#[test]
fn step_list_type_system_and_machine_markers_form_closed_contract() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");
    let view_source = load_source("src/step_list/view.rs");

    for required in [
        "pub enum StepListOrientation",
        "pub enum StepListSize",
        "pub struct StepListState",
        "pub struct StepListItemState",
        "pub fn resolve_state(input: StepListStateInput) -> StepListState",
    ] {
        assert!(
            primitives_source.contains(required),
            "step-list type system should keep closed-state marker `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-status=item_state.status_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
    ] {
        assert!(
            view_source.contains(required),
            "step-list machine-readable marker should include `{required}`."
        );
    }
}

#[test]
fn step_list_check2_documents_late_stage_governance_rules() {
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "`ui` 固定入口文件落点正确。",
        "组件目录标准文件落点正确。",
        "语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "`apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "在 `ui-headless` 写视觉和动画编排。",
        "在 `view` 层隐藏关键状态决策。",
        "新增参数但不纳入统一命名与契约。",
        "用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "公共 API 泄露底层实现细节类型。",
        "用临时补丁破坏跨组件一致性。",
        "明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            checklist_source.contains(required),
            "step-list checklist should keep governance rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_ui_components_entrypoint_files_and_forbidden_files_contract_hold() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "#[cfg(feature = \"component-step_list\")]",
        "pub mod step_list;",
        "pub use root::UiRoot;",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(crate::step_list::styles::CSS);",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "crate::css::push_components_css(&mut out);",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            lib_source.contains(required)
                || css_source.contains(required)
                || root_source.contains(required)
                || active_highlight_source.contains(required),
            "entrypoint contract should keep marker `{required}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_dx_playground_supports_hot_reload_and_isolated_canvas_with_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(required),
            "playground DX contract should keep marker `{required}`."
        );
    }

    for required in [
        "title=\"Controlled Selection\"",
        "title=\"Vertical + Emphasized + Disabled\"",
    ] {
        assert!(
            docs_source.contains(required),
            "step-list docs should keep isolated playground scenario `{required}`."
        );
    }

    for forbidden in [
        "STEP_LIST_WORKBENCH_STORAGE_KEY",
        "load_step_list_workbench_state(",
        "save_step_list_workbench_state(",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "step-list keeps optional persisted workbench state as N/A in current scope (`{forbidden}` absent)."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn step_list_engineering_contract_is_na_for_serde_spec_and_has_no_runtime_leaks() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/step_list/mod.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let view_source = load_source("src/step_list/view.rs");
    let styles_source = load_source("src/step_list/styles.rs");
    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");

    assert!(
        !manifest_dir.join("src/step_list/spec.rs").exists(),
        "step-list should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-step_list = []"),
        "step-list feature should stay lightweight."
    );

    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "spec::",
        "tokio::",
        "async_std::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "step-list engineering contract should avoid `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "HtmlElement", "JsValue", "Window", "Document"] {
        assert!(
            !combined.contains(forbidden),
            "step-list public engineering surface should avoid runtime leak token `{forbidden}`."
        );
    }
}

#[test]
fn step_list_agent_contract_markers_are_machine_readable_and_whitelist_safe() {
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-emphasis-source=move || state.get().emphasis_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-status=item_state.status_attr",
        "emit_selection_trace(",
        "\"intent=selection;source={source};index={index};prev={};next={}\"",
    ] {
        assert!(
            combined.contains(required),
            "step-list agent-readable contract should keep marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=",
        "data-ui-intent=",
        "data-ui-action=",
        "format!(\"data-",
        "<script",
        "javascript:",
        "inner_html",
        "set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "step-list contract should avoid free-form/injection marker `{forbidden}`."
        );
    }
}

#[test]
fn step_list_streaming_snapshot_contract_is_optional_and_snapshot_only_na() {
    let checklist_source = load_source("src/step_list/check2.md");
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    assert!(
        checklist_source.contains("fallback=snapshot"),
        "step-list streaming optional scope should explicitly declare `fallback=snapshot`."
    );

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "data-stream",
        "data-output-status",
        "data-draft",
        "data-verified",
        "retry",
    ] {
        assert!(
            !combined.contains(forbidden),
            "step-list is snapshot-only in current scope; forbidden streaming token `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn step_list_check2_documents_e2e_and_docs_rules() {
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "`apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
    ] {
        assert!(
            checklist_source.contains(required),
            "step-list checklist should keep e2e/docs governance rule `{required}`."
        );
    }
}

#[test]
fn step_list_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_step_list_contract.spec.mjs");

    for required in [
        "/#/components/step-list",
        "body:not(:has(#boot))",
        "[data-component=\"step-list\"]",
        "[data-slot=\"step-list\"]",
        "[data-slot=\"step-list-item\"][data-index=\"1\"]",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"data-status\", \"current\")",
        "toHaveAttribute(\"aria-current\", \"step\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "step-list e2e selector contract should keep marker `{required}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "step-list e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn step_list_e2e_key_flow_is_repeatable_and_coverage_suite_contains_playground_path() {
    let e2e_source = load_source("../../e2e/tests/docs_app_step_list_contract.spec.mjs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for required in [
        "key flow is repeatable with semantic state breakpoints",
        "await page.keyboard.press(\"ArrowRight\");",
        "await page.keyboard.press(\"ArrowLeft\");",
        "toHaveAttribute(\"data-selected-index\", \"2\")",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "step-list key-flow e2e should keep marker `{required}`."
        );
    }

    for required in [
        "docs-app components pages render playgrounds (sample)",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
    ] {
        assert!(
            coverage_source.contains(required),
            "docs components coverage suite should keep repeatable playground marker `{required}`."
        );
    }
}

#[test]
fn step_list_documentation_as_product_and_interactive_playground_contract_hold() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for required in [
        "title=\"StepList\"",
        "slug=\"step-list\"",
        "description=\"baseline-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts.\"",
        "<Playground title=\"Controlled Selection\" code_signal=code>",
        "<Playground title=\"Vertical + Emphasized + Disabled\" code_signal=states_code>",
        "\"selected index: \"",
    ] {
        assert!(
            docs_source.contains(required),
            "step-list docs-as-product contract should keep marker `{required}`."
        );
    }

    for required in [
        "\"StepList\",",
        "\"step-list\",",
        "collections_extra::step_list",
    ] {
        assert!(
            pages_source.contains(required),
            "step-list docs entry/index should keep marker `{required}`."
        );
    }
}

#[test]
fn step_list_source_first_copy_paste_ready_contract_holds() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_step_list_contract.spec.mjs");

    for required in [
        "let code = Signal::derive(move || {",
        "let states_code = Signal::derive(move || {",
        "r#\"let (selected_index, set_selected_index) = signal(Some(1_usize));",
        "on_selected_index_change=on_selected_index_change",
        "class_name=\"docs-step-list-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "step-list source-first docs should keep marker `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "CodeBlock code=resolved_code.get()",
        "DEFAULT_PLAYGROUND_IMPORTS",
    ] {
        assert!(
            playground_source.contains(required),
            "playground copy-ready pipeline should keep marker `{required}`."
        );
    }

    for required in [
        "playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toContainText(\"use leptos::prelude::*;\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "step-list copy-ready e2e should keep marker `{required}`."
        );
    }
}

#[test]
fn step_list_heroui_doc_sync_contract_holds_when_no_parameter_change() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_entry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let view_source = load_source("src/step_list/view.rs");

    for required in [
        "# HeroUI 参数设计风格对齐策略",
        "参数语义若变更，必须先同步本策略文档",
    ] {
        assert!(
            heroui_source.contains(required),
            "HeroUI strategy doc should keep synchronization policy marker `{required}`."
        );
    }

    for required in [
        "\"StepList\",",
        "\"step-list\",",
        "collections_extra::step_list",
    ] {
        assert!(
            docs_entry_source.contains(required),
            "step-list docs entry should remain indexable via `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] is_emphasized: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] default_selected_index: Option<usize>",
    ] {
        assert!(
            view_source.contains(required),
            "step-list parameter model should keep stable marker `{required}`."
        );
    }
}

#[test]
fn step_list_antipattern_guards_remain_blocked() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/step_list.rs");
    let headless_source = load_source("../../crates/ui-headless/src/step_list.rs");
    let view_source = load_source("src/step_list/view.rs");
    let logic_source = load_source("src/step_list/logic.rs");
    let mod_source = load_source("src/step_list/mod.rs");

    for forbidden in ["view!", "on:click", "on:keydown", ".ui-step-list", "color:"] {
        assert!(
            !primitives_source.contains(forbidden),
            "ui-state-primitives step-list should reject DOM/style antipattern token `{forbidden}`."
        );
    }

    for forbidden in [".ui-step-list", "@keyframes", "animation:", "transition:"] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless step-list should reject visual/motion antipattern token `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "HtmlElement", "JsValue", "Window", "Document"] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "step-list public API should reject platform leak antipattern token `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "on_selected_change"] {
        assert!(
            !view_source.contains(forbidden),
            "step-list should reject naming/implicit-structure antipattern token `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("primitives::resolve_state(input)")
            && logic_source.contains("primitives::resolve_item_state(input)"),
        "step-list should keep reusable primitive ownership in ui-state-primitives."
    );
}

#[test]
fn step_list_merge_gate_items_have_contract_evidence() {
    let view_source = load_source("src/step_list/view.rs");
    let styles_source = load_source("src/step_list/styles.rs");
    let semantics_source = load_source("tests/semantics.rs");
    let checklist_source = load_source("src/step_list/check2.md");

    for required in [
        "架构正确（边界不破）。",
        "行为正确（状态与交互语义成立）。",
        "可访问性达标（默认可用）。",
        "默认主题美学质量达标（与可访问性同级门禁）。",
        "可测试（契约可断言）。",
        "可维护（命名和模式一致）。",
        "可解释（人和自动化都能读懂）。",
        "改动在正确层。",
        "命名与全库一致。",
        "无效状态被限制或归一化。",
        "暴露必要语义标记。",
        "覆盖 reduced-motion / SSR / wasm 分支。",
        "文档与示例同步更新。",
    ] {
        assert!(
            checklist_source.contains(required),
            "step-list merge-gate checklist should keep item `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "aria-current=item_contract.attrs.aria_current",
        "aria-disabled=item_contract.attrs.aria_disabled",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "step-list merge-gate evidence should include semantic marker `{required}`."
        );
    }

    for required in [
        "var(--ui-fg)",
        "var(--ui-accent)",
        "var(--ui-success)",
        ".ui-step-list[data-orientation=\"vertical\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "step-list merge-gate evidence should include style contract marker `{required}`."
        );
    }

    for required in [
        "step_list_headless_contract_exists_and_view_consumes_it",
        "step_list_reduced_motion_ssr_wasm_contract_stays_semantically_stable",
        "step_list_docs_page_covers_primary_playgrounds",
        "step_list_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
    ] {
        assert!(
            semantics_source.contains(required),
            "step-list merge-gate evidence should include test marker `{required}`."
        );
    }
}
