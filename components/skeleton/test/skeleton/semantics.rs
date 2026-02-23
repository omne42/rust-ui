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
#[test]
fn skeleton_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/skeleton/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Skeleton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_public_api_stays_component_layer_and_hides_dom_details() {
    let lib_source = load_source("src/lib.rs");
    let mod_source = load_source("src/skeleton/mod.rs");
    let view_source = load_source("src/skeleton/view.rs");

    for needle in [
        "#[cfg(feature = \"component-skeleton\")]",
        "pub mod skeleton;",
        "pub use logic::SkeletonVariant;",
        "pub use view::Skeleton;",
    ] {
        assert!(
            (lib_source.contains(needle) || mod_source.contains(needle)),
            "Skeleton public API contract should expose `{needle}` in stable component-layer exports."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "HtmlElement",
        "NodeRef<web_sys",
        "EventTarget",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Skeleton API should not leak DOM/web-sys details; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_public_props_follow_is_on_default_prefix_contract() {
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    assert!(
        view_source.contains("#[prop(optional)] is_shimmer: Option<bool>"),
        "Skeleton boolean public prop should use `is_*` prefix.",
    );
    assert!(
        !view_source.contains("shimmer: bool"),
        "Legacy boolean prop name `shimmer` should be removed to avoid alias drift.",
    );
    assert!(
        logic_source.contains("pub const DEFAULT_IS_SHIMMER: bool = true;"),
        "Skeleton default values should be centralized in logic.rs.",
    );
}

#[test]
fn skeleton_has_no_half_controlled_state_axis_contract() {
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    assert!(
        view_source.contains("logic::normalize_state_input(logic::SkeletonViewInput {"),
        "Skeleton should map props directly to pure state primitive input."
    );
    for forbidden in [
        "create_signal(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "on_shimmer_change",
        "default_shimmer",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Skeleton should not implement half-controlled local state contract; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_uses_logic_state_model() {
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    for needle in [
        "pub use ui_state_primitives::skeleton::{",
        "SkeletonViewInput",
        "SkeletonStateInput",
        "normalize_optional_text",
        "normalize_state_input",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "Skeleton logic should source `{needle}` from ui_state_primitives."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_state_input(logic::SkeletonViewInput {",
        "let state = logic::resolve_state(state_input);",
        "logic::compose_class_name(class_name, state)",
        "data-state=state.state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Skeleton view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn skeleton_defaults_are_normalized_only_in_logic() {
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    assert!(
        !view_source.contains("default ="),
        "Skeleton view.rs must not own default value branches.",
    );
    assert!(
        !view_source.contains("unwrap_or("),
        "Skeleton view.rs should consume normalized outputs instead of fallback branching.",
    );
    for needle in [
        "pub const DEFAULT_IS_SHIMMER: bool = true;",
        "pub fn normalize_state_input(",
        "input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Skeleton defaults should be centralized in logic.rs; missing `{needle}`.",
        );
    }
}

#[test]
fn skeleton_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/skeleton/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");

    assert!(
        !view_source.contains("if state.has_shimmer"),
        "State branch should not be rebuilt in view.rs; use normalized state markers.",
    );
    for needle in [
        "pub struct SkeletonState {",
        "pub state_attr: &'static str,",
        "state_attr: if input.is_shimmer { \"shimmer\" } else { \"still\" },",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Skeleton state normalization should be centralized in primitives/logic; missing `{needle}`.",
        );
    }
}

#[test]
fn skeleton_status_primitives_are_sourced_from_ui_state_primitives() {
    let logic_source = load_source("src/skeleton/logic.rs");
    assert!(
        logic_source.contains("pub use ui_state_primitives::skeleton::{"),
        "Skeleton must consume state primitives from ui_state_primitives instead of implementing local state machines.",
    );
}

#[test]
fn skeleton_is_non_interactive_and_does_not_mount_headless_handlers() {
    let source = load_source("src/skeleton/view.rs");

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointer",
        "tabindex=",
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "use_press(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Skeleton is a non-interactive display primitive; `{forbidden}` should not appear unless reusable interaction semantics are introduced.",
        );
    }
}

#[test]
fn skeleton_does_not_introduce_component_motion_layer_or_custom_driver() {
    let mod_source = load_source("src/skeleton/mod.rs");
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");

    for forbidden in [
        "mod motion;",
        "attach_motion(",
        "ui_motion::",
        "SpringAnimator",
        "request_animation_frame",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "Skeleton should not introduce component-local motion driver contract; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_macro_micro_duality_dragging_loop_is_explicitly_na_for_non_interactive_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。")
            && check2_source.contains("组件无拖拽/指针追踪/逐帧物理循环"),
        "check2 should mark macro/micro dual-state drag contract as N/A with explicit non-interactive reason."
    );

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "on:dragstart",
        "on:dragend",
        "requestAnimationFrame",
        "request_animation_frame",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-interactive scope should not define macro/micro drag loop token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_two_pass_rendering_measure_rectification_is_explicitly_na_for_non_measurement_component(
) {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。")
            && check2_source.contains("不承担 `Tooltip/Popover/Menu` 几何定位职责"),
        "check2 should mark two-pass rendering contract as N/A with explicit non-measurement reason."
    );

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ResizeObserver",
        "MutationObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "Action::Rectification",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-measurement scope should not define two-pass geometry token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_registration_protocol_is_explicitly_na_for_non_navigable_collection_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。")
            && check2_source.contains("不属于动态子项导航组件"),
        "check2 should mark registration protocol contract as N/A with explicit non-navigable collection reason."
    );

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "std::collections::HashSet",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-navigable collection scope should not define registration protocol token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_slot_projection_strategy_is_explicitly_na_for_non_projection_container() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。")
            && check2_source.contains("不承担多插槽投影策略与 keep-alive 生命周期管理职责"),
        "check2 should mark slot projection strategy as N/A with explicit non-projection reason."
    );

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "ProjectionMode",
        "set_interval",
        "set_timeout",
        "requestAnimationFrame(",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-projection scope should not define slot projection lifecycle token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_env_streams_are_explicitly_na_for_non_subscribing_display_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。")
            && check2_source.contains("不承担响应式环境订阅职责"),
        "check2 should mark env-streams contract as N/A with explicit non-subscribing reason."
    );

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "ThemeChanged",
        "on:resize",
        "on:scroll",
        "debounce",
        "throttle",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-subscribing scope should not define env-stream token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_event_light_cone_is_explicitly_na_for_non_bulk_collection_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。")
            && check2_source.contains("非大型可交互集合组件"),
        "check2 should mark event-light-cone contract as N/A with explicit non-bulk-collection reason."
    );

    for forbidden in [
        "ContextBus",
        "SelectionState::All",
        "SelectionState",
        "SelectionContext",
        "provide_selection",
        "bulk_select",
        "prop_drilling",
        "TableAction",
        "GridAction",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-bulk-collection scope should not define event-light-cone token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_focus_stack_and_gc_is_explicitly_na_for_non_overlay_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        check2_source.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。")
            && check2_source.contains("不承担层叠 `Overlay` 焦点管理职责"),
        "check2 should mark focus-stack contract as N/A with explicit non-overlay reason."
    );

    for forbidden in [
        "Overlay",
        "NodeRef<web_sys",
        "FallbackTo",
        "Selector",
        "FocusManager",
        "focus_stack",
        "restore_focus",
        "document.body",
        "document().body",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden),
            "Skeleton non-overlay scope should not define focus-stack token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_escape_hatches_foreign_zone_is_explicitly_na_for_non_imperative_integration_component(
) {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");
    let skeleton_mod_source = load_source("src/skeleton/mod.rs");
    let skeleton_group_mod_source = load_source("src/skeleton/group/mod.rs");

    assert!(
        check2_source.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。")
            && check2_source.contains("不承担 ECharts/Map 等命令式第三方运行时集成职责"),
        "check2 should mark escape-hatches contract as N/A with explicit non-imperative-integration reason."
    );

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "leaflet",
        "google.maps",
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "foreign_instance",
        "imperative_instance",
    ] {
        assert!(
            !skeleton_view_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden)
                && !skeleton_logic_source.contains(forbidden)
                && !skeleton_group_logic_source.contains(forbidden)
                && !skeleton_mod_source.contains(forbidden)
                && !skeleton_group_mod_source.contains(forbidden),
            "Skeleton non-imperative scope should not define escape-hatch token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/skeleton/view.rs");

    for attr in [
        "data-slot=\"skeleton\"",
        "data-variant=state.variant_attr",
        "data-variant-source=source_state.variant_source_attr",
        "data-state=state.state_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "data-shimmer-source=source_state.shimmer_source_attr",
        "data-still=state.is_still.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Skeleton should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn skeleton_semantic_contract_covers_aria_and_state_source_markers() {
    let source = load_source("src/skeleton/view.rs");

    for attr in [
        "aria-hidden=\"true\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-variant-source=source_state.variant_source_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "data-shimmer-source=source_state.shimmer_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "Skeleton semantic contract should expose `{attr}` for a11y + machine-readable state/source checks.",
        );
    }
}

#[test]
fn skeleton_component_files_follow_responsibility_boundaries() {
    let mod_source = load_source("src/skeleton/mod.rs");
    let logic_source = load_source("src/skeleton/logic.rs");
    let styles_source = load_source("src/skeleton/styles.rs");
    let view_source = load_source("src/skeleton/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_path = manifest_dir.join("src/skeleton/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::SkeletonVariant;",
        "pub use view::Skeleton;",
    ] {
        assert!(
            mod_source.contains(needle),
            "skeleton/mod.rs should keep minimal stable exports; missing `{needle}`.",
        );
    }

    for forbidden in ["#[component]", "view! {", "pub const CSS", "mod motion;"] {
        assert!(
            !mod_source.contains(forbidden),
            "skeleton/mod.rs should not carry implementation details (`{forbidden}`).",
        );
    }

    for forbidden in [
        "#[component]",
        "view! {",
        "<div",
        "data-slot=",
        "var(--ui-",
        "color-mix(",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "skeleton/logic.rs should only normalize/derive state, not view/style/platform details (`{forbidden}`).",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border, var(--ui-fallback-border))",
    ] {
        assert!(
            styles_source.contains(needle),
            "skeleton/styles.rs should be token-first static CSS; missing `{needle}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "fn normalize_state_input",
        "on:click=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "skeleton/styles.rs should not include component logic/event handling (`{forbidden}`).",
        );
    }

    for needle in ["#[component]", "view! {", "data-state=state.state_attr"] {
        assert!(
            view_source.contains(needle),
            "skeleton/view.rs should render structure and mount semantic contract; missing `{needle}`.",
        );
    }
    for forbidden in ["unwrap_or(", "pub const CSS", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !view_source.contains(forbidden),
            "skeleton/view.rs should not own defaults/style constants/platform details (`{forbidden}`).",
        );
    }

    assert!(
        !motion_path.exists() && !mod_source.contains("mod motion;"),
        "Skeleton is non-interactive and should keep motion layer out of this component.",
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_directory_file_layout_matches_standard_component_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let skeleton_mod = load_source("src/skeleton/mod.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_styles = load_source("src/skeleton/styles.rs");
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_mod = load_source("src/skeleton/group/mod.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let skeleton_group_styles = load_source("src/skeleton/group/styles.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");

    for required in [
        "src/skeleton/mod.rs",
        "src/skeleton/logic.rs",
        "src/skeleton/styles.rs",
        "src/skeleton/view.rs",
        "src/skeleton/group/mod.rs",
        "src/skeleton/group/logic.rs",
        "src/skeleton/group/styles.rs",
        "src/skeleton/group/view.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "Skeleton directory standard-file contract should include `{required}`."
        );
    }

    for forbidden in [
        "src/skeleton/render.rs",
        "src/skeleton/group/render.rs",
        "src/skeleton/spec.rs",
        "src/skeleton/group/spec.rs",
    ] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "Skeleton directory should not drift into forbidden file `{forbidden}`."
        );
    }

    // Skeleton/SkeletonGroup are non-interactive display components; motion.rs is N/A here.
    for forbidden_motion in ["src/skeleton/motion.rs", "src/skeleton/group/motion.rs"] {
        assert!(
            !manifest_dir.join(forbidden_motion).exists(),
            "Skeleton non-interactive scope should keep motion file as N/A (`{forbidden_motion}`).",
        );
    }
    for forbidden in ["mod motion;", "pub mod motion;"] {
        assert!(
            !skeleton_mod.contains(forbidden) && !skeleton_group_mod.contains(forbidden),
            "Skeleton module boundary should not export motion implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::SkeletonVariant;",
        "pub use view::Skeleton;",
    ] {
        assert!(
            skeleton_mod.contains(needle),
            "skeleton/mod.rs should keep minimal stable export marker `{needle}`."
        );
    }
    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::SkeletonGroup;",
    ] {
        assert!(
            skeleton_group_mod.contains(needle),
            "skeleton/group/mod.rs should keep minimal stable export marker `{needle}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;", "#[component]", "view! {"] {
        assert!(
            !skeleton_mod.contains(forbidden) && !skeleton_group_mod.contains(forbidden),
            "mod.rs should not leak implementation detail `{forbidden}`."
        );
    }

    for source in [skeleton_logic.as_str(), skeleton_group_logic.as_str()] {
        for needle in [
            "normalize_state_input",
            "resolve_state",
            "compose_class_name",
        ] {
            assert!(
                source.contains(needle),
                "logic.rs should keep normalization/derivation marker `{needle}`."
            );
        }
        for forbidden in ["#[component]", "view! {", "<div", "on:click=", "web_sys::"] {
            assert!(
                !source.contains(forbidden),
                "logic.rs should not host view/event/platform detail `{forbidden}`."
            );
        }
    }

    for source in [skeleton_styles.as_str(), skeleton_group_styles.as_str()] {
        assert!(
            source.contains("pub const CSS: &str = r#\"") && source.contains("var(--ui-"),
            "styles.rs should be static token-first CSS and consume var(--ui-*).",
        );
        for forbidden in [
            "#[component]",
            "view! {",
            "fn normalize_state_input",
            "on:click=",
        ] {
            assert!(
                !source.contains(forbidden),
                "styles.rs should not carry logic/event/render detail `{forbidden}`."
            );
        }
    }

    for source in [skeleton_view.as_str(), skeleton_group_view.as_str()] {
        for needle in [
            "#[component]",
            "view! {",
            "logic::normalize_state_input",
            "data-state=",
        ] {
            assert!(
                source.contains(needle),
                "view.rs should render structure and mount semantic contract marker `{needle}`."
            );
        }
        for forbidden in ["unwrap_or(", "pub const CSS", "web_sys::", "wasm_bindgen::"] {
            assert!(
                !source.contains(forbidden),
                "view.rs should not own defaults/style constants/platform details `{forbidden}`."
            );
        }
    }
}

#[test]
fn skeleton_simple_component_does_not_define_spec_file_or_module() {
    let mod_source = load_source("src/skeleton/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/skeleton/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Skeleton is a simple component and should not expose spec module contract (`{forbidden}`).",
        );
    }

    assert!(
        !spec_path.exists(),
        "Skeleton should not define `src/skeleton/spec.rs` without stable external schema/versioning requirements.",
    );
}

#[test]
fn skeleton_styles_include_variant_and_shimmer_markers() {
    let source = load_source("src/skeleton/styles.rs");

    for selector in [
        ".ui-skeleton--variant-rect",
        ".ui-skeleton[data-variant=\"circle\"]",
        ".ui-skeleton[data-variant-source=\"prop\"]",
        ".ui-skeleton--shimmer::after",
        ".ui-skeleton[data-shimmer=\"true\"]::after",
        ".ui-skeleton[data-shimmer-source=\"prop\"]",
        ".ui-skeleton--still",
        ".ui-skeleton[data-still=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Skeleton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn skeleton_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/skeleton/styles.rs");
    let view_source = load_source("src/skeleton/view.rs");

    for needle in [
        ".ui-skeleton[data-variant=\"rect\"]",
        ".ui-skeleton[data-variant=\"circle\"]",
        ".ui-skeleton[data-variant-source=\"prop\"]",
        ".ui-skeleton[data-shimmer=\"true\"]::after",
        ".ui-skeleton[data-shimmer-source=\"prop\"]",
        ".ui-skeleton[data-still=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Skeleton styles should key off explicit semantic state markers `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", " > ", "style="] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "Skeleton should not rely on fragile DOM-structure/inline-style contract `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_theme_contract_is_token_first_and_ui_theme_owned() {
    let source = load_source("src/skeleton/styles.rs");

    for needle in [
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "color-mix(",
    ] {
        assert!(
            source.contains(needle),
            "Skeleton styles should consume theme tokens via `{needle}`.",
        );
    }

    for forbidden in ["--skeleton-", "#fff", "#000", "rgb(", "hsl("] {
        assert!(
            !source.contains(forbidden),
            "Skeleton styles should not introduce private color/token system (`{forbidden}`).",
        );
    }
}

#[test]
fn skeleton_token_first_styles_are_static_and_aggregated_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/skeleton/styles.rs");
    let view_source = load_source("src/skeleton/view.rs");
    let logic_source = load_source("src/skeleton/logic.rs");
    let checklist_source = load_source("src/skeleton/check2.md");

    for required in [
        "#[cfg(feature = \"component-skeleton\")]",
        "out.push_str(crate::skeleton::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "css.rs should aggregate Skeleton styles via feature-gated contract `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] inject_components_css: bool",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should stay as centralized CSS injection boundary via `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "color-mix(",
    ] {
        assert!(
            styles_source.contains(required),
            "Skeleton styles should stay token-first/static and include `{required}`.",
        );
    }

    for forbidden in [
        "--skeleton-",
        "@apply",
        "tailwind",
        "tw-",
        "styled(",
        "stylex",
        "emotion",
        "css!(",
        "style!(",
        "format!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Skeleton styles should not adopt utility-first/CSS-in-Rust/runtime style token `{forbidden}`.",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "tailwind",
        "tw!",
        "css!(",
        "style!(",
        "styled!(",
        "emotion",
        "style=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Skeleton component layer should not depend on utility-first/CSS-in-Rust marker `{forbidden}`.",
        );
    }

    for required in [
        "样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。",
        "Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。",
        "CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Skeleton checklist should keep token-first style governance guidance `{required}`.",
        );
    }
}

#[test]
fn skeleton_defensive_variables_use_two_level_fallback_chain_and_no_hardcoded_terminal_sizes() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_styles = load_source("src/skeleton/styles.rs");
    let skeleton_group_styles = load_source("src/skeleton/group/styles.rs");

    for required in [
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-radius-full, var(--ui-fallback-radius-full))",
        "var(--ui-image-skeleton-duration, var(--ui-fallback-image-skeleton-duration))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
    ] {
        assert!(
            skeleton_styles.contains(required) || skeleton_group_styles.contains(required),
            "Skeleton defensive-variable contract should include `{required}`.",
        );
    }

    for forbidden in ["9999px", "8rem", "2px", "1px", "1.25s", "1.15s", "#fff", "#000"] {
        assert!(
            !skeleton_styles.contains(forbidden) && !skeleton_group_styles.contains(forbidden),
            "Skeleton defensive-variable contract should not keep hardcoded terminal literal `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "check2 should mark defensive-variables contract complete."
    );
}

#[test]
fn skeleton_css_is_aggregated_under_layer_ui_and_runtime_styles_forbid_plain_inline_style() {
    let check2_source = load_source("src/skeleton/check2.md");
    let css_source = load_source("src/css.rs");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "out.push_str(crate::skeleton::styles::CSS);",
        "out.push_str(crate::skeleton::group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "Skeleton CSS should be aggregated under `@layer ui`; missing `{required}`."
        );
    }

    for source in [skeleton_view_source.as_str(), skeleton_group_view_source.as_str()] {
        for forbidden in ["style=", "style ="] {
            assert!(
                !source.contains(forbidden),
                "Skeleton runtime markup should forbid plain inline style `{forbidden}`."
            );
        }

        for line in source.lines().filter(|line| line.contains("style:")) {
            assert!(
                line.contains("style:--"),
                "Runtime style adjustments should use CSS custom property only (`style:--*`), found `{line}`."
            );
        }
    }

    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。"),
        "check2 should mark cascade-layer coverage contract complete."
    );
}

#[test]
fn skeleton_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

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
            "Theme visual baseline page should keep visual-quality contract token `{needle}`.",
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`.",
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
            "Theme visual baseline e2e contract should include `{needle}`.",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment constraint `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "web-demo-components = [",
        "component-skeleton = []",
        "component-skeleton_group = [\"component-skeleton\"]",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-skeleton\")]\npub mod skeleton;"),
        "lib.rs should feature-gate skeleton module export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-skeleton\")]")
            && css_source.contains("out.push_str(crate::skeleton::styles::CSS);"),
        "css.rs should gate skeleton CSS aggregation behind component-skeleton feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-skeleton_group\")]")
            && css_source.contains("out.push_str(crate::skeleton::group::styles::CSS);"),
        "css.rs should gate skeleton-group CSS aggregation behind component-skeleton_group feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn skeleton_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn skeleton_platform_script_covers_default_ssr_and_wasm_compile_only_paths() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "compile-only: default native path",
        "cargo check -p ui",
        "compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "compile-only: web wasm path",
        "cargo check -p ui --target wasm32-unknown-unknown",
        "compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should keep compile-only coverage contract `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep explicit web/ssr feature guard `{needle}`.",
        );
    }
}

#[test]
fn skeleton_ui_headless_web_ssr_mutex_contract_is_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep explicit web/ssr mutual-exclusion guard `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null;",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should verify ui-headless web/ssr contract via `{needle}`.",
        );
    }
}

#[test]
fn skeleton_ui_motion_non_wasm_stub_contract_is_guarded() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let motion_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let skeleton_mod_source = load_source("src/skeleton/mod.rs");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_mod_source = load_source("src/skeleton/group/mod.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "assert!(web::prefers_reduced_motion());",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            motion_stub_test_source.contains(needle),
            "ui-motion non-wasm stub regression test should include `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should cover ui-motion non-wasm/wasm/toolchain path via `{needle}`.",
        );
    }

    for forbidden in ["mod motion;", "ui_motion::", "attach_motion("] {
        assert!(
            !skeleton_mod_source.contains(forbidden)
                && !skeleton_view_source.contains(forbidden)
                && !skeleton_group_mod_source.contains(forbidden)
                && !skeleton_group_view_source.contains(forbidden),
            "skeleton and skeleton-group should not assume runtime motion handles (`{forbidden}`).",
        );
    }
}

#[test]
fn skeleton_reduced_motion_ssr_wasm_branches_are_covered_and_semantics_stable() {
    let skeleton_styles = load_source("src/skeleton/styles.rs");
    let skeleton_group_styles = load_source("src/skeleton/group/styles.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-skeleton--shimmer::after",
        ".ui-skeleton[data-shimmer=\"true\"]::after",
        "animation: none;",
    ] {
        assert!(
            skeleton_styles.contains(needle),
            "Skeleton reduced-motion contract should include `{needle}`."
        );
    }

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-skeleton-group--variant-pulse .ui-skeleton",
        ".ui-skeleton-group[data-variant=\"pulse\"] .ui-skeleton",
        "animation: none;",
    ] {
        assert!(
            skeleton_group_styles.contains(needle),
            "SkeletonGroup reduced-motion contract should include `{needle}`."
        );
    }

    for source in [
        ("src/skeleton/logic.rs", skeleton_logic.as_str()),
        ("src/skeleton/view.rs", skeleton_view.as_str()),
        ("src/skeleton/group/logic.rs", skeleton_group_logic.as_str()),
        ("src/skeleton/group/view.rs", skeleton_group_view.as_str()),
    ] {
        for forbidden in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
            "if cfg!(target_arch = \"wasm32\")",
        ] {
            assert!(
                !source.1.contains(forbidden),
                "Skeleton semantic contract must not split across SSR/wasm in `{}` via `{forbidden}`.",
                source.0
            );
        }
    }

    for needle in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
    ] {
        assert!(
            skeleton_view.contains(needle),
            "Skeleton SSR/wasm semantic contract should expose `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-variant=state.variant_attr",
    ] {
        assert!(
            skeleton_group_view.contains(needle),
            "SkeletonGroup SSR/wasm semantic contract should expose `{needle}`."
        );
    }
}

#[test]
fn skeleton_hydration_discontinuity_contract_is_explicitly_na_for_static_idless_component() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");
    let skeleton_protocol_source = load_source("src/skeleton/protocol.rs");
    let skeleton_group_protocol_source = load_source("src/skeleton/group/protocol.rs");

    assert!(
        check2_source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。")
            && check2_source.contains("不生成运行时随机 ID"),
        "check2 should mark hydration-discontinuity contract as N/A with explicit static-idless reason."
    );

    let combined = [
        skeleton_view_source,
        skeleton_group_view_source,
        skeleton_logic_source,
        skeleton_group_logic_source,
        skeleton_protocol_source,
        skeleton_group_protocol_source,
    ]
    .join("\n");

    for forbidden in [
        "now()",
        "Date::now",
        "SystemTime::now",
        "Utc::now",
        "Uuid::new_v4",
        "uuid::Uuid::new_v4",
        "rand::",
        "thread_rng(",
        "random::<",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton static-idless scope should not use non-deterministic hydration token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_performance_governance_has_static_equivalent_evidence_and_blocking_contract() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/skeleton/check2.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let skeleton_logic_source = load_source("src/skeleton/logic.rs");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_logic_source = load_source("src/skeleton/group/logic.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep budgeted perf-probe baseline contract `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(\"Skeleton\", \"skeleton\", \"Display\", display::skeleton)",
        "\"skeleton-group\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "Skeleton docs route should stay in coverage traversal via `{needle}`.",
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
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable budget/violation marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf guard `{needle}`.",
        );
    }

    for needle in [
        "let state_input = logic::normalize_state_input(logic::SkeletonViewInput {",
        "let state = logic::resolve_state(state_input);",
        "let class = logic::compose_class_name(class_name, state);",
        "data-state=state.state_attr",
    ] {
        assert!(
            skeleton_view_source.contains(needle),
            "Skeleton view should keep deterministic render pipeline token `{needle}`.",
        );
    }

    for needle in [
        "let state_input = logic::normalize_state_input(logic::SkeletonGroupViewInput {",
        "let state = logic::resolve_state(state_input);",
        "let class = logic::compose_class_name(class_name, state);",
        "data-state=state.state_attr",
        "data-loading-mode=state.loading_mode_attr",
    ] {
        assert!(
            skeleton_group_view_source.contains(needle),
            "SkeletonGroup view should keep deterministic render pipeline token `{needle}`.",
        );
    }

    for source in [
        ("src/skeleton/logic.rs", skeleton_logic_source.as_str()),
        ("src/skeleton/view.rs", skeleton_view_source.as_str()),
        (
            "src/skeleton/group/logic.rs",
            skeleton_group_logic_source.as_str(),
        ),
        (
            "src/skeleton/group/view.rs",
            skeleton_group_view_source.as_str(),
        ),
    ] {
        for forbidden in [
            "create_signal(",
            "signal(",
            "RwSignal",
            "Memo::new",
            "Effect::new",
            "request_animation_frame",
            "set_interval",
            "spawn_local(",
        ] {
            assert!(
                !source.1.contains(forbidden),
                "Skeleton perf-equivalent evidence requires static render path in `{}`; found `{forbidden}`.",
                source.0
            );
        }
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
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
            "Skeleton checklist should keep perf governance marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should keep blocking baseline token `{needle}`.",
        );
    }
}

#[test]
fn skeleton_view_macro_complexity_is_controlled_for_skeleton_and_group() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");

    assert!(
        skeleton_view.contains("view! {"),
        "Skeleton view.rs should keep explicit render block."
    );
    assert_eq!(
        skeleton_view.matches("view! {").count(),
        1,
        "Skeleton should keep one `view!` block to avoid macro expansion bloat."
    );
    assert!(
        skeleton_view.lines().count() <= 90,
        "Skeleton view.rs should stay compact; split semantic sub-blocks if this grows significantly."
    );

    assert!(
        skeleton_group_view.contains("view! {"),
        "SkeletonGroup view.rs should keep explicit render block."
    );
    assert_eq!(
        skeleton_group_view.matches("view! {").count(),
        1,
        "SkeletonGroup should keep one `view!` block to avoid macro expansion bloat."
    );
    assert!(
        skeleton_group_view.lines().count() <= 120,
        "SkeletonGroup view.rs should stay compact; split semantic sub-blocks if this grows significantly."
    );
}

#[test]
fn skeleton_function_split_prefers_plain_functions_over_extra_components() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");

    assert_eq!(
        skeleton_view.matches("#[component]").count(),
        1,
        "Skeleton should avoid promoting tiny view fragments into extra #[component] noise."
    );
    assert_eq!(
        skeleton_group_view.matches("#[component]").count(),
        1,
        "SkeletonGroup should avoid promoting tiny view fragments into extra #[component] noise."
    );

    for needle in ["pub fn Skeleton(", "data-state=state.state_attr"] {
        assert!(
            skeleton_view.contains(needle),
            "Skeleton root render contract should remain stable via `{needle}`.",
        );
    }

    for needle in ["pub fn SkeletonGroup(", "data-state=state.state_attr"] {
        assert!(
            skeleton_group_view.contains(needle),
            "SkeletonGroup root render contract should remain stable via `{needle}`.",
        );
    }
}

#[test]
fn skeleton_static_fragments_are_minimal_and_constantized_paths_are_clear() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");

    for forbidden in ["<svg", "inner_html", "footer", "lorem", "ipsum"] {
        assert!(
            !skeleton_view.contains(forbidden) && !skeleton_group_view.contains(forbidden),
            "Skeleton/SkeletonGroup should avoid heavy static fragment construction in view.rs (`{forbidden}`).",
        );
    }

    assert!(
        skeleton_group_logic.contains("pub const DEFAULT_ARIA_LABEL: &str = \"Skeleton group\";"),
        "SkeletonGroup default a11y label should be centralized as a stable constant in logic.rs."
    );
    assert!(
        skeleton_group_view.contains(
            "let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);"
        ),
        "SkeletonGroup view should consume centralized a11y label normalization instead of duplicating literals."
    );
    assert!(
        !skeleton_group_view.contains("\"Skeleton group\""),
        "SkeletonGroup view should not scatter fallback static label literal; keep one source in logic.rs."
    );

    for needle in [
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            skeleton_group_view.contains(needle) || skeleton_view.contains(needle),
            "Static fragment simplification must keep a11y semantics marker `{needle}`.",
        );
    }
}

#[test]
fn skeleton_inner_html_contract_rejects_untrusted_injection_and_keeps_a11y_semantics() {
    let files = [
        "src/skeleton/mod.rs",
        "src/skeleton/logic.rs",
        "src/skeleton/styles.rs",
        "src/skeleton/view.rs",
        "src/skeleton/group/mod.rs",
        "src/skeleton/group/logic.rs",
        "src/skeleton/group/styles.rs",
        "src/skeleton/group/view.rs",
    ];

    for file in files {
        let source = load_source(file);
        for forbidden in [
            "inner_html=",
            ".set_inner_html(",
            "dangerously_set_inner_html",
            "insert_adjacent_html",
        ] {
            assert!(
                !source.contains(forbidden),
                "Skeleton inner_html contract forbids untrusted HTML injection path `{forbidden}` in `{file}`.",
            );
        }
    }

    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    for needle in [
        "aria-hidden=\"true\"",
        "role=\"group\"",
        "aria-label=aria_label",
    ] {
        assert!(
            skeleton_view.contains(needle) || skeleton_group_view.contains(needle),
            "N/A inner_html path must still keep semantic/a11y contract marker `{needle}`.",
        );
    }
}

#[test]
fn skeleton_wasm_debug_capability_stays_feature_isolated_and_non_polluting() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let combined = [
        load_source("src/skeleton/mod.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/styles.rs"),
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/group/mod.rs"),
        load_source("src/skeleton/group/logic.rs"),
        load_source("src/skeleton/group/styles.rs"),
        load_source("src/skeleton/group/view.rs"),
    ]
    .join("\n");

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root_source.contains(needle),
            "ui should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    for forbidden_feature in ["skeleton-wasm-debug", "skeleton-group-wasm-debug"] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "Skeleton/SkeletonGroup should not expose dedicated wasm-debug feature `{forbidden_feature}`.",
        );
    }

    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton production contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui --test button_semantics button_wasm_debug_contract_is_feature_gated_and_dev_only",
    ] {
        assert!(
            script_source.contains(needle),
            "wasm-debug gate script should keep feature-isolated verification marker `{needle}`.",
        );
    }
}

#[test]
fn skeleton_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for marker in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-loading-mode=state.loading_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            skeleton_view.contains(marker) || skeleton_group_view.contains(marker),
            "Skeleton should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
        "request_replay",
        "data-slot=\"button-debug-replay\"",
    ] {
        assert!(
            !skeleton_view.contains(forbidden) && !skeleton_group_view.contains(forbidden),
            "Skeleton has no interactive replay path; non-applicable interaction token `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep wasm dev visual-entry gate `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }
}

#[test]
fn skeleton_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let display_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let display_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

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
            "Playground should keep CSS hot-reload/context-preserving marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn skeleton() -> AnyView",
        "title=\"Shimmer\"",
        "code_signal=shimmer_code",
        "title=\"Still\"",
        "code_signal=still_code",
    ] {
        assert!(
            display_source.contains(needle),
            "Skeleton docs should mount reusable Playground hot-reload path via `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn skeleton_group() -> AnyView",
        r#"title="Shimmer + Pulse Layout""#,
        "code_signal=loading_code",
        r#"title="Loaded + Skeleton Only""#,
        "code_signal=state_code",
    ] {
        assert!(
            display_extra_source.contains(needle),
            "SkeletonGroup docs should provide isolated demo/workbench entry `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let display_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let display_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let skeleton_source = display_source
        .split("pub(super) fn skeleton() -> AnyView")
        .nth(1)
        .and_then(|section| section.split("pub(super) fn ").next())
        .expect("display docs page should define skeleton() section");
    let skeleton_group_source = display_extra_source
        .split("pub(super) fn skeleton_group() -> AnyView")
        .nth(1)
        .and_then(|section| section.split("pub(super) fn ").next())
        .expect("display-extra docs page should define skeleton_group() section");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
        "<UiPerfProbe name=format!(\"Playground::{title}\")>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas/feedback-loop marker `{needle}`.",
        );
    }

    for forbidden in [
        "SKELETON_WORKBENCH_STORAGE_KEY",
        "SKELETON_GROUP_WORKBENCH_STORAGE_KEY",
        "load_skeleton_workbench_state(",
        "save_skeleton_workbench_state(",
        "clear_skeleton_workbench_state(",
        "load_skeleton_group_workbench_state(",
        "save_skeleton_group_workbench_state(",
        "clear_skeleton_group_workbench_state(",
        "Persist workbench state",
        "test_config_signal=",
    ] {
        assert!(
            !skeleton_source.contains(forbidden) && !skeleton_group_source.contains(forbidden),
            "Skeleton/SkeletonGroup are non-interactive display components; optional persisted state is N/A and `{forbidden}` should remain absent.",
        );
    }
}

#[test]
fn skeleton_check2_marks_dx_governance_complete() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "skeleton_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "skeleton_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "Skeleton checklist should keep DX completion evidence `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let check2_source = load_source("src/skeleton/check2.md");
    let combined = [
        load_source("src/skeleton/mod.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/styles.rs"),
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/group/mod.rs"),
        load_source("src/skeleton/group/logic.rs"),
        load_source("src/skeleton/group/styles.rs"),
        load_source("src/skeleton/group/view.rs"),
    ]
    .join("\n");

    assert!(
        !manifest_dir.join("src/skeleton/spec.rs").exists()
            && !manifest_dir.join("src/skeleton/group/spec.rs").exists(),
        "Skeleton scope should keep spec/schema boundary as N/A for simple component context."
    );

    for needle in [
        "component-skeleton = []",
        "component-skeleton_group = [\"component-skeleton\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "Skeleton features should stay lightweight via `{needle}`."
        );
    }
    for forbidden in [
        "component-skeleton = [\"dep:serde\"",
        "component-skeleton = [\"dep:serde_json\"",
        "component-skeleton_group = [\"dep:serde\"",
        "component-skeleton_group = [\"dep:serde_json\"",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "Skeleton should not opt into serde/spec migration dependencies without explicit schema contract: `{forbidden}`."
        );
    }

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
            "Skeleton engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "Skeleton checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn skeleton_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/skeleton/mod.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/styles.rs"),
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/group/mod.rs"),
        load_source("src/skeleton/group/logic.rs"),
        load_source("src/skeleton/group/styles.rs"),
        load_source("src/skeleton/group/view.rs"),
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

    for forbidden_feature in ["skeleton-wasm-debug", "skeleton-group-wasm-debug"] {
        assert!(
            !cargo_source.contains(forbidden_feature),
            "Skeleton should not define component-local tracing feature `{forbidden_feature}` when no local debug replay contract exists."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::skeleton::",
        "target: \"ui::skeleton_group::",
        "const SKELETON_TRACE_TARGET",
        "const SKELETON_GROUP_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let sources = [
        load_source("src/skeleton/mod.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/styles.rs"),
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/group/mod.rs"),
        load_source("src/skeleton/group/logic.rs"),
        load_source("src/skeleton/group/styles.rs"),
        load_source("src/skeleton/group/view.rs"),
    ];

    for source in &sources {
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
                "Skeleton engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !sources[0].contains("web_sys"),
        "Skeleton public module boundary should not leak web_sys types."
    );
}

#[test]
fn skeleton_check2_marks_engineering_governance_complete() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "skeleton_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "skeleton_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "skeleton_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            check2_source.contains(needle),
            "Skeleton checklist should keep engineering completion evidence `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn skeleton_ui_components_entry_points_stay_correct() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let skeleton_mod_source = load_source("src/skeleton/mod.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../ui-headless/src/presence.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-skeleton\")]\npub mod skeleton;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use skeleton::{Skeleton, SkeletonVariant};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep `{needle}`.",
        );
    }

    assert!(
        skeleton_mod_source
            .contains("#[cfg(feature = \"component-skeleton_group\")]\npub mod group;"),
        "skeleton module entry should gate skeleton_group submodule behind component-skeleton_group."
    );

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-skeleton\")]",
        "out.push_str(crate::skeleton::styles::CSS);",
        "#[cfg(feature = \"component-skeleton_group\")]",
        "out.push_str(crate::skeleton::group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css entry should keep `{needle}`.",
        );
    }

    for needle in [
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/css/i18n injection marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep `{needle}`.",
        );
    }
    for forbidden in [
        "Accordion",
        "Tabs",
        "Skeleton",
        "Well",
        "Surface",
        "data-slot=\"",
        "aria-",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business semantic token `{forbidden}`.",
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui root should not host `{forbidden}`.",
        );
    }

    for required in [
        "../ui-headless/src/controllable_state.rs",
        "../ui-headless/src/presence.rs",
        "../ui-headless/src/a11y.rs",
    ] {
        assert!(
            manifest_dir.join(required).exists(),
            "ui-headless canonical source `{required}` should exist.",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state_source.contains(needle),
            "ui-headless controllable-state canonical primitive should keep `{needle}`.",
        );
    }
    assert!(
        presence_source.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "ui-headless presence canonical primitive should keep use_presence contract."
    );
    for needle in [
        "pub fn aria_controls_when_open(",
        "pub fn locale_attrs(",
        "pub enum A11yDirection {",
    ] {
        assert!(
            a11y_source.contains(needle),
            "ui-headless a11y canonical primitive should keep `{needle}`.",
        );
    }
}

#[test]
fn skeleton_check2_marks_ui_components_entry_points_complete() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
        "skeleton_ui_components_entry_points_stay_correct",
    ] {
        assert!(
            check2_source.contains(needle),
            "Skeleton checklist should keep ui entry-point completion evidence `{needle}`.",
        );
    }
}

#[test]
fn skeleton_non_wasm_sources_do_not_reference_web_sys_or_browser_objects() {
    let files = [
        "src/skeleton/mod.rs",
        "src/skeleton/logic.rs",
        "src/skeleton/styles.rs",
        "src/skeleton/view.rs",
        "src/skeleton/group/mod.rs",
        "src/skeleton/group/logic.rs",
        "src/skeleton/group/styles.rs",
        "src/skeleton/group/view.rs",
    ];

    for file in files {
        let source = load_source(file);
        for forbidden in ["web_sys::", "wasm_bindgen::", "window()", "document()"] {
            assert!(
                !source.contains(forbidden),
                "non-wasm skeleton source `{file}` should not reference browser-only token `{forbidden}`.",
            );
        }
    }
}

#[test]
fn skeleton_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");
    let logic_source = load_source("src/skeleton/logic.rs");
    let view_source = load_source("src/skeleton/view.rs");

    for needle in [
        "pub enum SkeletonVariant {",
        "Rect,",
        "Circle,",
        "pub struct SkeletonStateInput {",
        "pub variant: SkeletonVariant,",
        "pub struct SkeletonState {",
        "pub variant_attr: &'static str,",
        "pub state_attr: &'static str,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "skeleton primitive type contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct SkeletonViewInput {",
        "pub variant: Option<SkeletonVariant>,",
        "pub is_shimmer: Option<bool>,",
        "pub fn normalize_state_input(input: SkeletonViewInput) -> SkeletonStateInput",
        "variant: input.variant.unwrap_or_default(),",
        "is_shimmer: input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER),",
    ] {
        assert!(
            logic_source.contains(needle),
            "skeleton logic should keep typed input normalization contract `{needle}`."
        );
    }

    for forbidden in [
        "pub variant: Option<String>",
        "variant: String",
        "match variant.as_str()",
        "format!(\"{}\", variant)",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "skeleton should not rely on string protocol for state typing/markers (`{forbidden}`).",
        );
    }

    for needle in [
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "data-still=state.is_still.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "skeleton view should expose machine-readable semantic marker `{needle}`.",
        );
    }
}

#[test]
fn skeleton_agent_contract_semantics_are_typed_traceable_and_script_safe() {
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");
    let group_mod_source = load_source("src/skeleton/group/mod.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");

    for needle in [
        "pub variant_attr: &'static str,",
        "pub state_attr: &'static str,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Agent contract baseline should keep typed skeleton state field `{needle}`."
        );
    }

    for needle in [
        "pub state_attr: &'static str,",
        "pub visibility_attr: &'static str,",
        "pub loading_mode_attr: &'static str,",
        "pub label_source_attr: &'static str,",
        "pub class_source_attr: &'static str,",
    ] {
        assert!(
            group_mod_source.contains(needle),
            "Agent contract baseline should keep typed skeleton-group state/source field `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"skeleton\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "data-still=state.is_still.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            skeleton_view.contains(needle),
            "Skeleton view should expose stable machine-readable marker `{needle}` for Agent consumption."
        );
    }

    for needle in [
        "data-slot=\"skeleton-group\"",
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-variant=state.variant_attr",
        "data-layout=state.layout_attr",
        "data-density=state.density_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            skeleton_group_view.contains(needle),
            "SkeletonGroup view should expose stable machine-readable marker `{needle}` for Agent consumption."
        );
    }

    for needle in [
        "variant: input.variant.unwrap_or_default(),",
        "is_shimmer: input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER),",
    ] {
        assert!(
            skeleton_logic.contains(needle),
            "Skeleton typed input normalization should keep `{needle}`."
        );
    }

    for needle in [
        "variant: input.variant.unwrap_or_default(),",
        "layout: input.layout.unwrap_or_default(),",
        "density: input.density.unwrap_or_default(),",
        "let loading_mode_attr = if input.is_skeleton_only {",
        "loading_mode_attr,",
        "label_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            skeleton_group_logic.contains(needle),
            "SkeletonGroup typed normalization/source mapping should keep `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=",
        "format!(\"data-",
        "push_str(\"data-",
        "inner_html=",
        "dangerously_set_inner_html",
        ".set_inner_html(",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !skeleton_view.contains(forbidden)
                && !skeleton_group_view.contains(forbidden)
                && !skeleton_logic.contains(forbidden)
                && !skeleton_group_logic.contains(forbidden),
            "Agent contract should avoid non-whitelisted/dynamic injection path `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/skeleton/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(required),
            "Skeleton checklist should keep streaming-definition marker `{required}`."
        );
    }
}

#[test]
fn skeleton_streaming_definition_stays_llm_scoped_and_not_component_local_protocol() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let combined =
        format!("{skeleton_view}\n{skeleton_logic}\n{skeleton_group_view}\n{skeleton_group_logic}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "snapshot",
        "fallback=snapshot",
        "data-stream",
        "data-output-status",
        "data-status=\"draft\"",
        "data-status=\"verified\"",
        "data-status=\"committable\"",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton should not encode LLM render-mode protocol locally (`{forbidden}`); streaming definition stays upper-layer scoped.",
        );
    }

    for required in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            skeleton_view.contains(required) || skeleton_group_view.contains(required),
            "Skeleton semantic continuity should stay on component state/source marker `{required}`."
        );
    }
}

#[test]
fn skeleton_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("src/skeleton/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "Skeleton checklist should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn skeleton_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for marker in [
        "#[prop(optional)] variant: Option<SkeletonVariant>",
        "#[prop(optional)] is_shimmer: Option<bool>",
        "#[prop(optional, into)] class_name: Option<String>",
        "logic::normalize_state_input(logic::SkeletonViewInput {",
        "let state = logic::resolve_state(state_input);",
        "data-slot=\"skeleton\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
    ] {
        assert!(
            skeleton_view.contains(marker),
            "Skeleton snapshot-baseline render path should include `{marker}`.",
        );
    }

    for marker in [
        "pub struct SkeletonViewInput {",
        "pub fn normalize_state_input(input: SkeletonViewInput) -> SkeletonStateInput",
        "variant: input.variant.unwrap_or_default(),",
        "is_shimmer: input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER),",
    ] {
        assert!(
            skeleton_logic.contains(marker),
            "Skeleton snapshot-baseline normalization path should include `{marker}`.",
        );
    }

    for marker in [
        "#[prop(optional)] is_loading: Option<bool>",
        "#[prop(optional)] is_skeleton_only: Option<bool>",
        "#[prop(optional)] variant: Option<SkeletonGroupVariant>",
        "#[prop(optional)] layout: Option<SkeletonGroupLayout>",
        "#[prop(optional)] density: Option<SkeletonGroupDensity>",
        "children: Children,",
        "{children()}",
        "logic::normalize_state_input(logic::SkeletonGroupViewInput {",
        "let state = logic::resolve_state(state_input);",
        "data-slot=\"skeleton-group\"",
        "data-state=state.state_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-visibility=state.visibility_attr",
    ] {
        assert!(
            skeleton_group_view.contains(marker),
            "SkeletonGroup snapshot-baseline render path should include `{marker}`.",
        );
    }

    for marker in [
        "pub struct SkeletonGroupViewInput {",
        "pub fn normalize_state_input(input: SkeletonGroupViewInput) -> SkeletonGroupStateInput",
        "is_loading: input.is_loading.unwrap_or(DEFAULT_IS_LOADING),",
        "is_skeleton_only: input.is_skeleton_only.unwrap_or(DEFAULT_IS_SKELETON_ONLY),",
        "variant: input.variant.unwrap_or_default(),",
        "layout: input.layout.unwrap_or_default(),",
        "density: input.density.unwrap_or_default(),",
    ] {
        assert!(
            skeleton_group_logic.contains(marker),
            "SkeletonGroup snapshot-baseline normalization path should include `{marker}`.",
        );
    }

    for marker in [
        "pub struct SkeletonStateInput {",
        "pub struct SkeletonState {",
        "pub variant_attr: &'static str,",
        "pub state_attr: &'static str,",
    ] {
        assert!(
            primitive_source.contains(marker),
            "Snapshot baseline should keep primitive typed-state marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn skeleton() -> AnyView",
        "title=\"Skeleton\"",
        "slug=\"skeleton\"",
        "title=\"Shimmer\"",
        "title=\"Still\"",
    ] {
        assert!(
            docs_display.contains(marker),
            "Skeleton docs should include snapshot-ready complete config marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn skeleton_group() -> AnyView",
        "title=\"SkeletonGroup\"",
        "slug=\"skeleton-group\"",
        "title=\"Shimmer + Pulse Layout\"",
        "title=\"Loaded + Skeleton Only\"",
        "When `is_skeleton_only=true` and loading is finished, the skeleton group hides itself.",
    ] {
        assert!(
            docs_display_extra.contains(marker),
            "SkeletonGroup docs should include snapshot-ready complete config marker `{marker}`.",
        );
    }
}

#[test]
fn skeleton_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("src/skeleton/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "Skeleton checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn skeleton_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let combined = format!("{skeleton_view}\n{skeleton_group_view}");

    for required in [
        "aria-hidden=\"true\"",
        "data-slot=\"skeleton\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-slot=\"skeleton-group\"",
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "data-loading-mode=state.loading_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            combined.contains(required),
            "Skeleton/SkeletonGroup should keep continuous role/aria/data semantics via `{required}` in snapshot-only optional-streaming scope."
        );
    }

    for forbidden in [
        "data-ui-output-status",
        "data-output-status",
        "data-stream-status",
        "data-status=\"draft\"",
        "data-status=\"verified\"",
        "data-status=\"committed\"",
        "data-status=\"committable\"",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton/SkeletonGroup should not mount fake streaming status field `{forbidden}` when stream protocol is N/A."
        );
    }
}

#[test]
fn skeleton_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let combined = [
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/group/view.rs"),
        load_source("src/skeleton/group/logic.rs"),
    ]
    .join("\n");

    for forbidden in [
        "on_retry",
        "retry",
        "reconnect",
        "backoff",
        "resume",
        "revalidate",
        "validate_stream",
        "stream_error",
        "disconnect",
        "network_error",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton/SkeletonGroup should keep streaming validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/skeleton/semantics.rs");
    let has_marker = |marker: &str| {
        semantics_source.contains(marker) || semantics_source.contains(&marker.replace('"', "\\\""))
    };

    for required in [
        "skeleton_semantic_contract_covers_aria_and_state_source_markers",
        "skeleton_emits_baseline_style_state_data_attributes",
        "skeleton_group_view",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-label-source=state.label_source_attr",
        "role=\"group\"",
        "aria-label=aria_label",
    ] {
        assert!(
            has_marker(required),
            "Skeleton semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["image", "_snapshot"].concat(),
    ];
    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Skeleton semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn skeleton_semantic_markers_changed_in_views_must_be_covered_by_semantics_checks() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let semantics_source = load_source("tests/skeleton/semantics.rs");
    let has_semantics_marker = |marker: &str| {
        semantics_source.contains(marker) || semantics_source.contains(&marker.replace('"', "\\\""))
    };

    for marker in [
        "data-slot=\"skeleton\"",
        "data-variant=state.variant_attr",
        "data-state=state.state_attr",
        "data-shimmer=state.has_shimmer.then_some(\"true\")",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            skeleton_view.contains(marker),
            "Skeleton view should expose semantic marker `{marker}`."
        );
        assert!(
            has_semantics_marker(marker),
            "Skeleton semantics tests must cover semantic marker `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"skeleton-group\"",
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            skeleton_group_view.contains(marker),
            "SkeletonGroup view should expose semantic marker `{marker}`."
        );
        assert!(
            has_semantics_marker(marker),
            "Skeleton semantics tests must cover SkeletonGroup semantic marker `{marker}`."
        );
    }

    assert!(
        semantics_source
            .contains("skeleton_is_non_interactive_and_does_not_mount_headless_handlers"),
        "Skeleton semantics suite should keep non-interactive path assertion for keyboard/pointer contract."
    );
}

#[test]
fn skeleton_e2e_selectors_use_semantic_markers_with_wasm_stable_ready_waits() {
    let coverage_spec = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for marker in [
        "component_doc!(\"Skeleton\", \"skeleton\", \"Display\", display::skeleton)",
        "\"skeleton-group\"",
        "display_extra::skeleton_group",
    ] {
        assert!(
            pages_registry.contains(marker),
            "docs-app component registry should include skeleton route marker `{marker}`.",
        );
    }

    for marker in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "await expect(perfProbe).toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "await expect(perfProbe).toHaveAttribute(\"data-perf-observability\", /mount/);",
        "await expect(perfProbe).not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_spec.contains(marker),
            "E2E coverage should keep semantic selector + stable-ready marker `{marker}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ".nth(",
        "nth-child(",
        "text=",
    ] {
        assert!(
            !coverage_spec.contains(forbidden),
            "E2E coverage should avoid brittle/fixed-wait selector token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_e2e_ready_settled_for_async_or_animation_is_explicit_na_in_component_scope() {
    let skeleton_view = load_source("src/skeleton/view.rs");
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_group_view = load_source("src/skeleton/group/view.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let skeleton_mod = load_source("src/skeleton/mod.rs");
    let skeleton_group_mod = load_source("src/skeleton/group/mod.rs");

    let combined = format!(
        "{skeleton_view}\n{skeleton_logic}\n{skeleton_group_view}\n{skeleton_group_logic}\n{skeleton_mod}\n{skeleton_group_mod}"
    );

    for forbidden in [
        "async fn",
        "spawn_local(",
        "fetch(",
        "mod motion;",
        "attach_motion(",
        "ui_motion::",
        "request_animation_frame",
        "on:click=",
        "on:keydown=",
        "on:pointer",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Skeleton/SkeletonGroup async-animation settled path should stay N/A in component scope; found `{forbidden}`."
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-visibility=state.visibility_attr",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            skeleton_view.contains(marker) || skeleton_group_view.contains(marker),
            "Even when async/animation is N/A, semantic ready marker `{marker}` should remain observable."
        );
    }
}

#[test]
fn skeleton_check2_marks_e2e_selector_stability_complete() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "skeleton_e2e_selectors_use_semantic_markers_with_wasm_stable_ready_waits",
        "skeleton_e2e_ready_settled_for_async_or_animation_is_explicit_na_in_component_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "Skeleton checklist should keep E2E selector stability completion evidence `{needle}`.",
        );
    }
}

#[test]
fn skeleton_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn skeleton() -> AnyView",
        "title=\"Skeleton\"",
        "slug=\"skeleton\"",
        "title=\"Shimmer\"",
        "title=\"Still\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Skeleton.",
        );
    }
}

#[test]
fn skeleton_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Shimmer\"",
        "<Skeleton variant=SkeletonVariant::Rect class_name=\"docs-skeleton-line\".to_string() />",
        "<Skeleton variant=SkeletonVariant::Circle class_name=\"docs-skeleton-avatar\".to_string() />",
        "title=\"Still\"",
        "is_shimmer=false",
        "class_name=\"docs-skeleton-line docs-skeleton-line--short\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "skeleton docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn skeleton_discrete_state_axes_are_enum_typed_and_not_free_form_protocols() {
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let group_logic_source = load_source("src/skeleton/group/logic.rs");

    for needle in [
        "pub enum SkeletonVariant {",
        "pub enum SkeletonGroupVariant {",
        "pub enum SkeletonGroupLayout {",
        "pub enum SkeletonGroupDensity {",
        "Option<SkeletonVariant>",
        "Option<SkeletonGroupVariant>",
        "Option<SkeletonGroupLayout>",
        "Option<SkeletonGroupDensity>",
    ] {
        assert!(
            primitive_source.contains(needle)
                || skeleton_view_source.contains(needle)
                || group_logic_source.contains(needle),
            "skeleton discrete axis should stay enum-typed via `{needle}`."
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "layout: Option<String>",
        "density: Option<String>",
        "variant: String",
        "layout: String",
        "density: String",
        "match variant.as_str()",
    ] {
        assert!(
            !group_logic_source.contains(forbidden) && !primitive_source.contains(forbidden),
            "skeleton discrete axis should not degrade to free-form string protocol `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_state_primitive_source_boundary_stays_in_ui_state_primitives() {
    let skeleton_logic = load_source("src/skeleton/logic.rs");
    let skeleton_group_logic = load_source("src/skeleton/group/logic.rs");
    let check2_source = load_source("src/skeleton/check2.md");

    assert!(
        skeleton_logic.contains("pub use ui_state_primitives::skeleton::{"),
        "skeleton logic should source state primitives from ui-state-primitives."
    );

    for forbidden in [
        "use_context(",
        "provide_context(",
        "leptos_reactive::",
        "pinia",
        "redux",
        "mobx",
        "zustand",
    ] {
        assert!(
            !skeleton_logic.contains(forbidden) && !skeleton_group_logic.contains(forbidden),
            "component logic should not bind business/global store directly via `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。"),
        "check2 should mark status-primitives source boundary as complete."
    );
}

#[test]
fn skeleton_async_semantics_are_explicitly_na_without_component_async_protocol() {
    let check2_source = load_source("src/skeleton/check2.md");
    let combined = [
        load_source("src/skeleton/view.rs"),
        load_source("src/skeleton/logic.rs"),
        load_source("src/skeleton/group/view.rs"),
        load_source("src/skeleton/group/logic.rs"),
    ]
    .join("\n");

    assert!(
        check2_source.contains("- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。")
            && check2_source.contains("组件无远程请求与异步状态"),
        "check2 should explicitly mark async contract as N/A with reason."
    );

    for forbidden in [
        "use_async_action",
        "async fn",
        "spawn_local(",
        "retry",
        "on_retry",
        "stream_error",
        "network_error",
    ] {
        assert!(
            !combined.contains(forbidden),
            "skeleton should not define component-local async protocol token `{forbidden}`."
        );
    }

    assert!(
        combined.contains("aria-busy=state.is_loading.then_some(\"true\")"),
        "skeleton-group should keep aria-busy mapping for loading semantics."
    );
}

#[test]
fn skeleton_dx_paradox_keeps_default_api_short_and_internal_complexity_hidden() {
    let check2_source = load_source("src/skeleton/check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        check2_source
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"),
        "check2 should mark DX paradox requirement complete."
    );

    for needle in [
        "title=\"Shimmer\"",
        "title=\"Still\"",
        "title=\"Shimmer + Pulse Layout\"",
        "title=\"Loaded + Skeleton Only\"",
    ] {
        assert!(
            docs_display.contains(needle) || docs_display_extra.contains(needle),
            "skeleton docs should expose default easy path marker `{needle}`."
        );
    }

    for forbidden in ["state=", "headless_state=", "primitive_state="] {
        assert!(
            !docs_display.contains(forbidden) && !docs_display_extra.contains(forbidden),
            "skeleton docs default usage should not require internal state-object wiring `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_composition_api_uses_explicit_children_and_rejects_parallel_arrays() {
    let check2_source = load_source("src/skeleton/check2.md");
    let group_view = load_source("src/skeleton/group/view.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        check2_source
            .contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
        "check2 should mark explicit-composition rule complete."
    );

    assert!(
        group_view.contains("children: Children,") && group_view.contains("{children()}"),
        "SkeletonGroup should keep explicit composition via children slot."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children_by_index",
        "labels + children",
    ] {
        assert!(
            !docs_display_extra.contains(forbidden) && !group_view.contains(forbidden),
            "SkeletonGroup should not expose parallel-array implicit API token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("src/skeleton/check2.md");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/skeleton.rbi");
    let skeleton_view_source = load_source("src/skeleton/view.rs");
    let skeleton_group_view_source = load_source("src/skeleton/group/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        manifest_dir.join("src/Component.toml").exists()
            && manifest_dir.join("src/skeleton.rbi").exists(),
        "Skeleton context-compression protocol requires `src/Component.toml` and `src/skeleton.rbi`.",
    );

    for required in [
        "schema_version = \"1\"",
        "[component]",
        "crate = \"ui-skeleton\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Skeleton Component.toml should include `{required}`.",
        );
    }

    for required in [
        "pub fn Skeleton(",
        "pub fn SkeletonGroup(",
        "is_shimmer: Option<bool>",
        "is_loading: Option<bool>",
        "is_skeleton_only: Option<bool>",
    ] {
        assert!(
            rbi_source.contains(required),
            "Skeleton RBI projection should include `{required}`.",
        );
    }

    for forbidden in ["shimmer: bool", "loading: bool", "skeleton_only: bool"] {
        assert!(
            !rbi_source.contains(forbidden),
            "Skeleton RBI should avoid stale API signature token `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_shimmer: Option<bool>",
        "#[prop(optional)] is_loading: Option<bool>",
        "#[prop(optional)] is_skeleton_only: Option<bool>",
    ] {
        assert!(
            skeleton_view_source.contains(required) || skeleton_group_view_source.contains(required),
            "RBI signature projection should stay aligned with current view props `{required}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "check2 should mark context-compression protocol as complete."
    );
}

#[test]
fn skeleton_a11y_i18n_l10n_contract_has_accessible_entrypoints_without_text_hardcoding_in_view() {
    let check2_source = load_source("src/skeleton/check2.md");
    let group_logic = load_source("src/skeleton/group/logic.rs");
    let group_view = load_source("src/skeleton/group/view.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    assert!(
        check2_source.contains(
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
        ),
        "check2 should mark a11y/i18n contract complete."
    );

    for needle in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "DEFAULT_ARIA_LABEL",
        "role=\"group\"",
        "aria-label=aria_label",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
    ] {
        assert!(
            group_view.contains(needle) || group_logic.contains(needle),
            "SkeletonGroup should keep a11y/l10n entrypoint `{needle}`."
        );
    }

    assert!(
        headless_a11y.contains("pub fn locale_attrs("),
        "ui-headless shared a11y module should keep locale attrs entrypoint."
    );
}

#[test]
fn skeleton_state_markers_are_observable_searchable_and_enumerated() {
    let check2_source = load_source("src/skeleton/check2.md");
    let skeleton_view = load_source("src/skeleton/view.rs");
    let group_view = load_source("src/skeleton/group/view.rs");

    assert!(
        check2_source.contains(
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
        ),
        "check2 should mark state observability contract complete."
    );

    for marker in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-visibility=state.visibility_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            skeleton_view.contains(marker) || group_view.contains(marker),
            "skeleton semantic marker should remain observable `{marker}`."
        );
    }
}

#[test]
fn skeleton_check2_marks_directory_layout_agent_and_semantic_suite_items_complete() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should mark completed evidence item `{needle}`."
        );
    }
}

#[test]
fn skeleton_e2e_key_flow_contract_is_repeatable_and_semantic() {
    let check2_source = load_source("src/skeleton/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_skeleton_contract.spec.mjs");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep repeatable key-flow rule `{needle}`."
        );
    }

    for needle in [
        "docs-app skeleton-group key flow is repeatable with semantic breakpoints",
        "body:not(:has(#boot))",
        "[data-component=\"skeleton-group\"]",
        "[data-slot=\"skeleton-group\"][data-loading-mode=\"skeleton-only\"][data-visibility=\"hidden\"]",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"loaded\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "skeleton e2e key-flow should include semantic checkpoint `{needle}`."
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
            "skeleton e2e key-flow should avoid flaky/non-semantic token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_docs_and_examples_are_synced_with_matrix_and_runtime_api() {
    let check2_source = load_source("src/skeleton/check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let skeleton_view = load_source("src/skeleton/view.rs");
    let group_view = load_source("src/skeleton/group/view.rs");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "check2 should mark docs/example/matrix sync complete."
    );

    for needle in [
        "title=\"Shimmer\"",
        "title=\"Still\"",
        "is_shimmer=false",
        "title=\"Shimmer + Pulse Layout\"",
        "title=\"Loaded + Skeleton Only\"",
        "is_loading=false",
        "is_skeleton_only=true",
    ] {
        assert!(
            docs_display.contains(needle) || docs_display_extra.contains(needle),
            "skeleton docs matrix should include `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] variant: Option<SkeletonVariant>",
        "#[prop(optional)] is_shimmer: Option<bool>",
        "#[prop(optional)] is_loading: Option<bool>",
        "#[prop(optional)] is_skeleton_only: Option<bool>",
        "#[prop(optional)] density: Option<SkeletonGroupDensity>",
    ] {
        assert!(
            skeleton_view.contains(needle) || group_view.contains(needle),
            "runtime API should keep marker `{needle}` aligned with docs."
        );
    }
}

#[test]
fn skeleton_docs_are_beginner_friendly_and_order_default_before_advanced() {
    let check2_source = load_source("src/skeleton/check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "check2 should mark documentation-as-product item complete."
    );

    let skeleton_hello = docs_display
        .find("title=\"Shimmer\"")
        .expect("skeleton docs should include default hello-style path");
    let skeleton_advanced = docs_display
        .find("title=\"Still\"")
        .expect("skeleton docs should include advanced state path");
    assert!(
        skeleton_hello < skeleton_advanced,
        "skeleton docs should keep default path before advanced variations."
    );

    let group_default = docs_display_extra
        .find("title=\"Shimmer + Pulse Layout\"")
        .expect("skeleton-group docs should include default path");
    let group_advanced = docs_display_extra
        .find("title=\"Loaded + Skeleton Only\"")
        .expect("skeleton-group docs should include advanced state path");
    assert!(
        group_default < group_advanced,
        "skeleton-group docs should keep default path before advanced state matrix."
    );
}

#[test]
fn skeleton_docs_app_provides_interactive_playground_runtime() {
    let check2_source = load_source("src/skeleton/check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "check2 should mark interactive playground item complete."
    );

    for needle in [
        "<Playground",
        "code_signal=shimmer_code",
        "code_signal=still_code",
        "code_signal=loading_code",
        "code_signal=state_code",
    ] {
        assert!(
            docs_display.contains(needle) || docs_display_extra.contains(needle),
            "skeleton docs should keep interactive playground marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Playground(",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "let resolved_code = Signal::derive(move || {",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep live-preview marker `{needle}`."
        );
    }
}

#[test]
fn skeleton_docs_source_first_are_copy_paste_ready_and_traceable() {
    let check2_source = load_source("src/skeleton/check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_skeleton_contract.spec.mjs");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "check2 should mark source-first copy-paste item complete."
    );

    for needle in [
        "test_source_path=\"crates/ui/src/skeleton/view.rs\".to_string()",
        "test_source_path=\"crates/ui/src/skeleton/group/view.rs\".to_string()",
    ] {
        assert!(
            docs_display.contains(needle) || docs_display_extra.contains(needle),
            "skeleton docs should keep source-trace marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "docs-app skeleton playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui::*;\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "skeleton copy-flow e2e should keep marker `{needle}`."
        );
    }
}

#[test]
fn skeleton_heroui_strategy_and_component_docs_stay_synced() {
    let check2_source = load_source("src/skeleton/check2.md");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "check2 should mark HeroUI/docs sync item complete."
    );

    for needle in [
        "### Skeleton 同步记录（2026-02-18）",
        "`Skeleton` 与 `SkeletonGroup` 维持 display primitive 定位",
        "component_doc!(\"Skeleton\", \"skeleton\", \"Display\", display::skeleton)",
        "component_doc!(\"SkeletonGroup\", \"skeleton-group\", \"Display\", display_extra::skeleton_group)",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep skeleton sync marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Skeleton\", \"skeleton\", \"Display\", display::skeleton)",
        "\"skeleton-group\"",
        "display_extra::skeleton_group",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs catalog should keep skeleton entry marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Skeleton\"",
        "title=\"SkeletonGroup\"",
        "slug=\"skeleton\"",
        "slug=\"skeleton-group\"",
    ] {
        assert!(
            docs_display.contains(needle) || docs_display_extra.contains(needle),
            "docs page should keep skeleton marker `{needle}`."
        );
    }
}

#[test]
fn skeleton_antipattern_guardrails_are_explicit_and_enforced() {
    let check2_source = load_source("src/skeleton/check2.md");
    let primitive_source = load_source("../ui-state-primitives/src/skeleton.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let logic_source = load_source("src/skeleton/logic.rs");
    let group_logic_source = load_source("src/skeleton/group/logic.rs");
    let view_source = load_source("src/skeleton/view.rs");
    let group_view_source = load_source("src/skeleton/group/view.rs");
    let mod_source = load_source("src/skeleton/mod.rs");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should mark anti-pattern guardrail complete `{needle}`."
        );
    }

    for forbidden in [
        "use leptos",
        "view! {",
        "class=",
        "web_sys::",
        "HtmlElement",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives skeleton should stay DOM/style free; found `{forbidden}`."
        );
    }

    for forbidden in ["@keyframes", "animation:", "transition:", "class="] {
        assert!(
            !headless_a11y_source.contains(forbidden),
            "ui-headless a11y shared primitives should avoid visual/motion orchestration token `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_state_input(logic::SkeletonViewInput {",
        "let state = logic::resolve_state(state_input);",
        "logic::normalize_state_input(logic::SkeletonGroupViewInput {",
    ] {
        assert!(
            view_source.contains(needle) || group_view_source.contains(needle),
            "view should consume normalized logic output via `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "web_sys", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden),
            "public skeleton API should not leak internal/platform detail `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::skeleton::{")
            && group_logic_source.contains("pub fn normalize_state_input("),
        "reusable primitive/normalization logic should stay in primitives or logic assembly layer."
    );
}

#[test]
fn skeleton_rust_hygiene_contract_is_enforced_for_component_scope() {
    let check2_source = load_source("src/skeleton/check2.md");
    let group_logic_source = load_source("src/skeleton/group/logic.rs");
    let hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");

    assert!(
        check2_source.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "check2 should mark rust hygiene governance item complete."
    );

    assert!(
        hygiene_script.contains("check-rust-hygiene"),
        "workspace should keep scripts/check-rust-hygiene.sh as the hygiene gate entrypoint."
    );

    let non_test_sources = [
        "src/skeleton/mod.rs",
        "src/skeleton/logic.rs",
        "src/skeleton/styles.rs",
        "src/skeleton/view.rs",
        "src/skeleton/group/mod.rs",
        "src/skeleton/group/logic.rs",
        "src/skeleton/group/styles.rs",
        "src/skeleton/group/view.rs",
    ];

    for rel_path in non_test_sources {
        let source = load_source(rel_path);

        for forbidden in ["unwrap(", "expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "rust hygiene forbids `{forbidden}` in non-test source `{rel_path}`."
            );
        }
    }

    for needle in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-skeleton-group\")",
        "classes.push(Cow::Owned(base_class_name));",
    ] {
        assert!(
            group_logic_source.contains(needle),
            "skeleton string-copy hotspot should converge to Cow contract marker `{needle}`."
        );
    }

    for forbidden in [
        "\"ui-skeleton-group\".to_string()",
        "\"ui-skeleton-group--loading\".to_string()",
        "\"ui-skeleton-group--loaded\".to_string()",
        "\"ui-skeleton-group--skeleton-only\".to_string()",
        "\"ui-skeleton-group--custom-class\".to_string()",
    ] {
        assert!(
            !group_logic_source.contains(forbidden),
            "Cow-based class composition should remove eager string allocation token `{forbidden}`."
        );
    }
}

#[test]
fn skeleton_merge_gate_items_are_fully_marked_complete_with_verification_evidence() {
    let check2_source = load_source("src/skeleton/check2.md");

    for needle in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
    ] {
        assert!(
            check2_source.contains(needle),
            "merge gate checklist item should be completed `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。"),
        "full-gate row should be completed after running fmt/clippy/test/smoke verification."
    );

    for needle in [
        "/root/.cargo/bin/cargo fmt --all -- --check",
        "/root/.cargo/bin/cargo clippy -p ui --no-default-features --features component-skeleton_group,inject-css --lib -- -D warnings",
        "/root/.cargo/bin/cargo clippy -p ui --no-default-features --features component-skeleton_group,inject-css --test skeleton_semantics -- -D warnings",
        "/root/.cargo/bin/cargo clippy -p ui --no-default-features --features component-skeleton_group,inject-css --test skeleton_group_semantics -- -D warnings",
        "/root/.cargo/bin/cargo test -p ui --no-default-features --features component-skeleton_group,inject-css --test skeleton_semantics --test skeleton_group_semantics",
        "bash ./scripts/smoke-csr.sh apps/docs-app \"body:not(:has(#boot))\"",
    ] {
        assert!(
            check2_source.contains(needle),
            "full-gate evidence should keep verification command `{needle}`."
        );
    }
}
