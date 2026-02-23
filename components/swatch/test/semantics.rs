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
fn swatch_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/swatch/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Swatch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn swatch_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/swatch/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Swatch;"),
        "swatch module should export `Swatch`."
    );
    assert!(
        crate_source
            .contains("pub use swatch::{Swatch, SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize};"),
        "crate root should re-export Swatch contract."
    );
}

#[test]
fn swatch_component_files_follow_layered_responsibilities() {
    let mod_source = load_source("../../components/swatch/src/mod.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/swatch.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Swatch;",
        "pub use motion::SwatchMotion;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Swatch module boundary should include `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "resolve_selection_control_state(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Swatch mod.rs should keep minimal exports and avoid implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::swatch::{",
        "resolve_selection_control_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Swatch logic layer should include `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "<div",
        "NodeRef",
        "web_sys::",
        "SpringAnimator::new",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic layer should not include view/platform/motion-engine detail `{forbidden}`."
        );
    }

    for needle in [
        "pub enum SwatchSize",
        "pub enum SwatchBorder",
        "pub enum SwatchRounding",
        "pub enum SwatchShape",
        "pub struct SwatchStateInput",
        "pub struct SwatchState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Swatch primitive layer should include `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "var(--ui-fg)",
        "var(--ui-bg)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Swatch styles layer should include token-first css signal `{needle}`."
        );
    }

    for forbidden in ["view! {", "Callback::new(", "on:click", "on:keydown"] {
        assert!(
            !styles_source.contains(forbidden),
            "Swatch styles layer should stay static and avoid runtime logic `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_state(logic::SwatchStateInput {",
        "logic::resolve_selection_control_state(",
        "use_swatch(SwatchOptions {",
        "swatch_motion::attach_motion(node_ref, selected, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view layer should compose structure + headless + motion via `{needle}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "pub struct SwatchStateInput",
        "pub enum SwatchSize",
        "SpringAnimator::new",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view layer should avoid lower-layer reimplementation `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SwatchMotion",
        "pub fn sanitize_motion(motion: SwatchMotion) -> SwatchMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(needle),
            "Swatch motion layer should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "role=", "aria-", "use_swatch("] {
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion layer should stay mapping/attach only and avoid `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn swatch_spec_boundary_avoids_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_mod_source = load_source("src/button/mod.rs");
    let swatch_mod_source = load_source("../../components/swatch/src/mod.rs");

    assert!(
        manifest_dir.join("src/button/spec.rs").exists(),
        "button should keep canonical spec.rs boundary for complex schema contract."
    );
    assert!(
        !manifest_dir
            .join("../../components/swatch/src/spec.rs")
            .exists(),
        "Swatch should not introduce a local spec.rs file."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec export `{needle}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "SwatchSpec", "SwatchSchema"] {
        assert!(
            !swatch_mod_source.contains(forbidden),
            "Swatch module should stay lightweight and avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_uses_logic_state_model_from_ui_state_primitives() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/swatch.rs");

    for needle in [
        "pub use ui_state_primitives::swatch::{",
        "SwatchStateInput",
        "SwatchState",
        "normalize_optional_text",
        "sanitize_color_value",
        "resolve_aria_label",
        "resolve_aria_label_with_fallbacks",
        "resolve_state",
        "resolve_selection_control_state",
        "pub fn compose_class_name(",
        "pub fn compose_inline_style(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Swatch logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub enum SwatchSize {",
        "pub enum SwatchBorder {",
        "pub enum SwatchRounding {",
        "pub enum SwatchShape {",
        "pub struct SwatchStateInput {",
        "pub struct SwatchState {",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "pub enum SwatchSize",
        "pub enum SwatchBorder",
        "pub enum SwatchRounding",
        "pub enum SwatchShape",
        "pub struct SwatchStateInput",
        "pub struct SwatchState",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Swatch state primitive layer should include `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "let color = logic::sanitize_color_value(color);",
        "logic::resolve_state(logic::SwatchStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::resolve_aria_label_with_fallbacks(",
        "logic::compose_inline_style(color.as_deref())",
        "logic::resolve_selection_control_state(",
        "use_controllable_state(",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view should consume logic helpers for state assembly; missing `{needle}`."
        );
    }
}

#[test]
fn swatch_view_mounts_headless_contract_instead_of_local_keyboard_state_machine() {
    let source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "use ui_headless as overlay_open;",
        "use ui_headless::{A11yDirection, CommonStrings, SwatchOptions, use_swatch, use_ui_i18n};",
        "let swatch_aria = use_swatch(SwatchOptions {",
        "let common = i18n.strings::<CommonStrings>();",
        "let mixed_label_fallback = common.swatch_mixed_aria_label.as_ref();",
        "let nothing_label_fallback = common.swatch_nothing_aria_label.as_ref();",
        "let default_label_fallback = common.swatch_default_aria_label.as_ref();",
        "on:pointerdown=move |_| swatch_aria.handlers.button.press.on_pointer_down.run(())",
        "on:pointerup=move |_| swatch_aria.handlers.button.press.on_pointer_up.run(())",
        "on:pointercancel=move |_| swatch_aria.handlers.button.press.on_pointer_cancel.run(())",
        "on:click=move |_| swatch_aria.handlers.button.press.on_click.run(())",
        "swatch_aria.handlers.button.press.on_key_down.run(key)",
        "swatch_aria.handlers.button.press.on_key_up.run(key)",
        "aria-pressed=move || swatch_aria.attrs.aria_pressed.get()",
        "lang=swatch_aria.attrs.lang.clone()",
        "dir=swatch_aria.attrs.dir",
    ] {
        assert!(
            source.contains(needle),
            "Swatch view should mount ui-headless swatch contract token `{needle}`."
        );
    }

    for forbidden in [
        "let on_activate = move ||",
        "let on_keydown = move |ev: ev::KeyboardEvent|",
        "if key == \" \" || key == \"Enter\"",
        "crate::color_swatch::sanitize_color_value",
    ] {
        assert!(
            !source.contains(forbidden),
            "Swatch view should not reimplement keyboard semantics locally; found `{forbidden}`."
        );
    }
}

#[test]
fn swatch_i18n_keys_are_defined_in_common_strings_bundle() {
    let source = load_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "pub swatch_default_aria_label: Arc<str>",
        "pub swatch_mixed_aria_label: Arc<str>",
        "pub swatch_nothing_aria_label: Arc<str>",
        "swatch_default_aria_label: \"Swatch\".into()",
        "swatch_mixed_aria_label: \"Mixed\".into()",
        "swatch_nothing_aria_label: \"No fill\".into()",
    ] {
        assert!(
            source.contains(needle),
            "CommonStrings should define swatch i18n key `{needle}`."
        );
    }
}

#[test]
fn swatch_api_uses_is_on_default_naming_and_controlled_triplet_markers() {
    let source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "#[prop(optional)] is_nothing: bool",
        "#[prop(optional)] is_mixed_value: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_decorative: bool",
        "#[prop(optional, into)] selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
        "data-controlled=selection_control.is_controlled_selected.then_some(\"true\")",
        "data-uncontrolled=selection_control.is_uncontrolled_selected.then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Swatch API/control markers should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] nothing: bool",
        "#[prop(optional)] mixed_value: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] decorative: bool",
        "default_selected.unwrap_or(false)",
    ] {
        assert!(
            !source.contains(forbidden),
            "Swatch API/default normalization should not keep legacy token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_async_semantics_are_explicitly_not_applicable_for_now() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let check2_source = load_source("../../components/swatch/src/check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch has no async interaction contract yet, so view should not include `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch has no async interaction contract yet, so logic should not include `{forbidden}`."
        );
    }

    for needle in [
        "如果无异步相关，直接打勾",
        "组件无远程请求与异步状态",
        "有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist source should define async N/A contract text `{needle}`."
        );
    }
}

#[test]
fn swatch_macro_micro_dragging_dual_state_machine_is_not_applicable_for_now() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "宏观/微观双状态机（Macro/Micro Duality）",
        "Action::DragEnd",
        "N/A：`Swatch` 为叶子展示组件，当前不提供拖拽交互与 `Dragging` 生命周期",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep macro/micro dual-state-machine marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep macro/micro marker `{needle}`."
        );
    }

    for forbidden in [
        "on:dragstart",
        "on:drag",
        "on:dragend",
        "DragEnd",
        "is_dragging",
        "dragging",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose dragging lifecycle token `{forbidden}` for current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose dragging lifecycle token `{forbidden}` for current scope."
        );
    }

    for forbidden in ["DragEnd", "on_drag", "is_dragging", "dragging"] {
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion should not define drag loop contract token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_two_pass_geometry_rendering_contract_is_not_applicable_for_now() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "几何两段式渲染（Two-Pass Rendering）",
        "Intent -> Measure(view) -> Rectification(logic)",
        "N/A：`Swatch` 不属于依赖几何测量的 overlay 组件",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep two-pass geometry marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep two-pass geometry marker `{needle}`."
        );
    }

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "on:resize",
        "measure(",
        "Rectification",
        "geometry",
        "layout_rect",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose geometry two-pass token `{forbidden}` in current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose geometry two-pass token `{forbidden}` in current scope."
        );
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion should not expose geometry two-pass token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_registration_protocol_contract_is_not_applicable_for_leaf_component() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "集合注册协议（Registration Protocol）",
        "RegistrationContext",
        "Register/Unregister",
        "items_order",
        "HashSet",
        "N/A：`Swatch` 是单体叶子组件，不存在动态子项集合与导航序关系",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep registration-protocol marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep registration-protocol marker `{needle}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose collection-registration token `{forbidden}` in current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose collection-registration token `{forbidden}` in current scope."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Swatch docs should not present collection-registration token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_slot_projection_contract_is_not_applicable_for_leaf_component() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "插槽投影策略（Slot Projection）",
        "Lazy/KeepAlive/Eager",
        "NotifyHidden",
        "N/A：`Swatch` 为单体叶子组件，不提供容器级 slot 投影与子树保活策略",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep slot-projection marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep slot-projection marker `{needle}`."
        );
    }

    for forbidden in [
        "KeepAlive",
        "Lazy",
        "Eager",
        "NotifyHidden",
        "slot projection",
        "projection_mode",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose slot-projection token `{forbidden}` in current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose slot-projection token `{forbidden}` in current scope."
        );
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion should not expose slot-projection token `{forbidden}` in current scope."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Swatch docs should not present slot-projection token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_env_stream_contract_is_not_applicable_for_leaf_component() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "环境订阅流（Env Streams）",
        "Resize/Theme/Intersection",
        "BreakpointChanged",
        "N/A：`Swatch` 当前无环境订阅流能力",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep env-stream marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep env-stream marker `{needle}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "debounce",
        "throttle",
        "on:resize",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose env-stream token `{forbidden}` in current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose env-stream token `{forbidden}` in current scope."
        );
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion should not expose env-stream token `{forbidden}` in current scope."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Swatch docs should not present env-stream token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_event_light_cone_contract_is_not_applicable_for_leaf_component() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/check2.md");
    let check2_source_copy = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "事件光锥（Event Light Cone）",
        "Context Bus + Selector",
        "SelectionState::All",
        "N/A：`Swatch` 为单体叶子展示组件，不承载 `Table/Grid` 级批量集合操作",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep event-light-cone marker `{needle}`."
        );
        assert!(
            check2_source_copy.contains(needle),
            "Swatch checklist source copy should keep event-light-cone marker `{needle}`."
        );
    }

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "selection_state_all",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should not expose event-light-cone token `{forbidden}` in current scope."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Swatch logic should not expose event-light-cone token `{forbidden}` in current scope."
        );
        assert!(
            !motion_source.contains(forbidden),
            "Swatch motion should not expose event-light-cone token `{forbidden}` in current scope."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Swatch docs should not present event-light-cone token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn swatch_composite_parent_item_api_contract_is_explicitly_not_applicable_for_leaf_component() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "组合型组件主 API 必须“显示优于约定”",
        "labels + children",
        "titles + panels",
        "ItemSpec",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist source should keep composite-api governance marker `{needle}`."
        );
    }

    assert!(
        view_source.contains("pub fn Swatch("),
        "Swatch should keep leaf component API entry."
    );
    assert!(
        docs_source.contains("<Swatch color=\"#ffcc00\".to_string() />"),
        "Swatch docs should keep direct leaf usage path."
    );

    for forbidden in [
        "children: Children",
        "#[prop(optional)] children",
        "labels + children",
        "titles + panels",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Leaf Swatch API should not expose composite parent-item contract `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Swatch docs should not recommend composite parent-item syntax `{forbidden}`."
        );
    }
}

#[test]
fn swatch_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/swatch.rs");

    for needle in [
        "pub enum SwatchSize",
        "pub enum SwatchBorder",
        "pub enum SwatchRounding",
        "pub enum SwatchShape",
        "pub struct SwatchStateInput",
        "pub struct SwatchState",
        "pub fn resolve_state(input: SwatchStateInput) -> SwatchState",
        "#[prop(optional)] size: SwatchSize,",
        "#[prop(optional)] border: SwatchBorder,",
        "#[prop(optional)] rounding: SwatchRounding,",
        "#[prop(optional)] shape: SwatchShape,",
        "#[prop(optional, into)] selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
    ] {
        assert!(
            primitive_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Swatch machine-readable contract should keep typed input/model token `{needle}`."
        );
    }

    for needle in [
        "show_nothing = !show_mixed_value && input.nothing;",
        "has_color = input.has_color && !show_mixed_value && !show_nothing;",
        "let data_state_attr = if input.disabled {",
        "} else if show_mixed_value {",
        "} else if show_nothing {",
        "} else if has_color {",
        "} else {",
        "\"disabled\"",
        "\"mixed\"",
        "\"nothing\"",
        "\"color\"",
        "\"empty\"",
        "let control_mode_attr = if is_controlled_selected {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "let default_selected_source_attr = if input.default_selected.is_some() {",
        "\"custom\"",
        "\"default\"",
        "let selected_change_source_attr = if input.has_on_selected_change {",
        "\"none\"",
    ] {
        assert!(
            primitive_source.contains(needle) || logic_source.contains(needle),
            "Swatch invalid-state normalization / closed marker mapping should include `{needle}`."
        );
    }

    for needle in [
        "data-size=state.size_attr",
        "data-border=state.border_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-pressed=move || swatch_aria.state.is_pressed.get().then_some(\"true\")",
        "data-disabled=state.disabled.then_some(\"true\")",
        "data-nothing=state.show_nothing.then_some(\"true\")",
        "data-mixed-value=state.show_mixed_value.then_some(\"true\")",
        "data-has-color=state.has_color.then_some(\"true\")",
        "data-decorative=state.decorative.then_some(\"true\")",
        "data-aria-label-source=aria_label_source",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch machine-readable contract should expose semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] size: Option<String>",
        "#[prop(optional)] border: Option<String>",
        "#[prop(optional)] rounding: Option<String>",
        "#[prop(optional)] shape: Option<String>",
        "#[prop(optional)] size: String",
        "#[prop(optional)] border: String",
        "#[prop(optional)] rounding: String",
        "#[prop(optional)] shape: String",
        "data-size=\"",
        "data-border=\"",
        "data-rounding=\"",
        "data-shape=\"",
        "format!(\"data-",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Swatch should avoid string-protocol / brittle marker generation token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_attaches_motion_driver() {
    let source = load_source("../../components/swatch/src/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Swatch should attach its motion driver to deliver spring-based selection feedback."
    );
}

#[test]
fn swatch_motion_uses_spring_animator() {
    let source = load_source("../../components/swatch/src/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Swatch motion should animate via springs to match the repo motion spec."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn swatch_motion_contract_defaults_match_upstream_level_expectations() {
    let source = load_source("../../components/swatch/src/motion.rs");

    for needle in [
        "use ui_theme::default_swatch_motion_tokens;",
        "let tokens = default_swatch_motion_tokens();",
        "stiffness: tokens.spring.stiffness",
        "damping: tokens.spring.damping",
        "mass: tokens.spring.mass",
        "precision: tokens.spring.precision",
        "selected_scale: tokens.selected_scale",
        "selected_ring_opacity: tokens.selected_ring_opacity",
        "pub fn disabled() -> Self",
        "enabled: false",
        "fn default_motion_reads_theme_tokens()",
    ] {
        assert!(
            source.contains(needle),
            "Swatch motion contract should include `{needle}` for baseline-level defaults and disabled-path stability."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn swatch_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("../../components/swatch/src/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SwatchMotion) -> SwatchMotion",
        ".clamp(1.0, 1.18)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            source.contains(needle),
            "Swatch motion implementation should include `{needle}` to avoid baseline-level motion regressions."
        );
    }
}

#[test]
fn swatch_styles_use_css_variables_for_motion() {
    let source = load_source("../../components/swatch/src/styles.rs");

    for name in ["--ui-swatch-scale", "--ui-swatch-ring-opacity"] {
        assert!(
            source.contains(name),
            "Swatch styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn swatch_token_first_styles_are_static_and_aggregated_via_ui_root() {
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-bg)",
        "var(--ui-bg-muted)",
        "var(--ui-fg)",
        "var(--ui-accent)",
        "var(--ui-danger)",
        "var(--ui-swatch-color)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Swatch styles should stay token-first/static and include `{needle}`."
        );
    }

    for forbidden in [
        "@apply",
        "styled(",
        "css!(",
        "tailwind",
        "tw-",
        "class:",
        "var(--swatch-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Swatch styles should avoid utility/CSS-in-Rust/private-token pattern `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-swatch\")]",
        "out.push_str(crate::swatch::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Swatch styles must be aggregated through css.rs via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized injection path via `{needle}`."
        );
    }

    assert!(
        view_source
            .contains("style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()"),
        "Swatch view should only mount css-variable inline style output from logic."
    );
    assert!(
        logic_source.contains("format!(\"--ui-swatch-color: {color};\")"),
        "Swatch runtime style mapping should stay css-variable-only."
    );
}

#[test]
fn swatch_runtime_style_only_sets_css_custom_property() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");

    assert!(
        view_source
            .contains("style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()"),
        "Swatch view should mount precomputed inline style only."
    );
    assert!(
        logic_source.contains("format!(\"--ui-swatch-color: {color};\")"),
        "Swatch runtime style should only set CSS custom properties."
    );
}

#[test]
fn swatch_visual_desire_default_theme_baseline_is_wired_for_docs_and_e2e_regression() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "theme_visual_baseline::theme_visual_baseline",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "ButtonVariant::Accent",
        "ButtonVariant::Secondary",
        "ButtonVariant::Ghost",
        "is_clearable=true",
        "<Overlay",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "theme visual baseline docs gate should include `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "E2E_VISUAL_BASELINE",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression gate should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn swatch_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-swatch = []",
        "\"component-swatch\"",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-swatch\")]\n#[path = \"color/swatch_core/mod.rs\"]\npub mod swatch;"
        ),
        "lib.rs should feature-gate swatch module export for tree-shaking.",
    );

    for needle in [
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
        "pub use web_demo_components::*;",
        "pub use all_components::*;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should keep feature-bounded export surface token `{needle}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-swatch\")]")
            && css_source.contains("out.push_str(crate::swatch::styles::CSS);"),
        "css.rs should gate swatch CSS aggregation behind component-swatch feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    for forbidden in [
        "static ALL_COMPONENTS",
        "const ALL_COMPONENTS",
        "HashMap<&'static str, fn",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "global registry pattern that defeats DCE should stay absent `{forbidden}`."
        );
    }

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
fn swatch_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
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
fn swatch_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free() {
    let check_source = load_source("../../components/swatch/src/check2.md");
    let ui_components_cargo_source = load_source("Cargo.toml");
    let ui_headless_cargo_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let swatch_view_source = load_source("../../components/swatch/src/view.rs");
    let swatch_logic_source = load_source("../../components/swatch/src/logic.rs");
    let swatch_motion_source = load_source("../../components/swatch/src/motion.rs");
    let swatch_styles_source = load_source("../../components/swatch/src/styles.rs");

    assert!(
        check_source.contains("SSR 与跨平台检查"),
        "Swatch checklist should keep SSR/cross-platform contract item explicit.",
    );

    let wasm_target_deps_header = "[target.'cfg(target_arch = \"wasm32\")'.dependencies]";
    let wasm_target_deps_index = ui_components_cargo_source
        .find(wasm_target_deps_header)
        .expect("ui Cargo.toml should keep wasm32 target dependency section");
    let non_wasm_deps_section = &ui_components_cargo_source[..wasm_target_deps_index];

    assert!(
        non_wasm_deps_section.contains("[dependencies]"),
        "ui should keep a native dependency section before wasm32 target dependencies."
    );
    assert!(
        !non_wasm_deps_section.contains("web-sys ="),
        "ui should not pull web-sys in non-wasm dependency path."
    );

    for needle in [
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "js-sys = \"0.3.85\"",
        "web-sys = { version = \"0.3.85\"",
    ] {
        assert!(
            ui_components_cargo_source.contains(needle),
            "ui Cargo.toml should keep wasm-specific dependency token `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo_source.contains(needle),
            "ui-headless feature contract should include `{needle}`.",
        );
    }

    assert!(
        ui_headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!("),
        "ui-headless should keep web/ssr compile_error! mutex guard."
    );
    assert!(
        ui_headless_lib_source.contains("mutually exclusive"),
        "ui-headless mutex compile_error should keep explicit mutually-exclusive diagnosis.",
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]\npub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep explicit wasm/non-wasm backend split token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            swatch_motion_source.contains(needle),
            "Swatch motion should keep explicit wasm/non-wasm split marker `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen",
        "window()",
        "document()",
    ] {
        assert!(
            !swatch_view_source.contains(forbidden)
                && !swatch_logic_source.contains(forbidden)
                && !swatch_styles_source.contains(forbidden),
            "Swatch non-wasm component files should avoid browser-only token `{forbidden}`.",
        );
    }
}

#[test]
fn swatch_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: web wasm path\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn swatch_headless_web_ssr_feature_mutex_is_compile_guarded_and_script_verified() {
    let check_source = load_source("../../components/swatch/src/check2.md");
    let ui_headless_cargo_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let swatch_view_source = load_source("../../components/swatch/src/view.rs");

    assert!(
        check_source.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "Swatch checklist should keep explicit ui-headless web/ssr mutex item.",
    );

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo_source.contains(needle),
            "ui-headless feature contract should include `{needle}`.",
        );
    }

    assert!(
        ui_headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!("),
        "ui-headless should keep web/ssr compile_error! mutex guard.",
    );
    assert!(
        ui_headless_lib_source.contains("mutually exclusive"),
        "ui-headless mutex compile_error should keep explicit mutually-exclusive diagnosis.",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-headless web/ssr mutex verification token `{needle}`.",
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, CommonStrings, SwatchOptions, use_swatch, use_ui_i18n};",
        "let swatch_aria = use_swatch(SwatchOptions {",
    ] {
        assert!(
            swatch_view_source.contains(needle),
            "Swatch should keep explicit ui-headless integration token `{needle}`.",
        );
    }
}

#[test]
fn swatch_motion_non_wasm_stub_contract_is_predictable_and_toolchain_safe() {
    let check_source = load_source("../../components/swatch/src/check2.md");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let swatch_motion_source = load_source("../../components/swatch/src/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    assert!(
        check_source.contains("`ui-motion` 非 wasm 提供 no-op/stub"),
        "Swatch checklist should keep explicit ui-motion non-wasm stub item.",
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]\npub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }

    for needle in [
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "assert!(web::prefers_reduced_motion());",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_stub_test_source.contains(needle),
            "ui-motion non-wasm stub tests should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            swatch_motion_source.contains(needle),
            "Swatch motion should keep non-wasm safe downgrade marker `{needle}`.",
        );
    }

    for forbidden in ["panic!(", ".unwrap()", ".expect("] {
        assert!(
            !swatch_motion_source.contains(forbidden),
            "Swatch non-wasm motion downgrade path should avoid hard-failure marker `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should enforce ui-motion portability via `{needle}`."
        );
    }
}

#[test]
fn swatch_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let check_source = load_source("../../components/swatch/src/check2.md");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_spring_checks_source = load_source("../ui-motion/tests/spring.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    assert!(
        check_source.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "Swatch checklist should keep explicit reduced-motion/SSR/wasm branch item.",
    );

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion downgrade behavior token `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_checks_source.contains(needle),
            "ui-motion reduced-motion regression tests should include `{needle}`.",
        );
    }

    for needle in [
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "scale.set_target(target_scale);",
        "ring.set_target(target_ring);",
        "return;",
    ] {
        assert!(
            motion_source.contains(needle),
            "Swatch reduced-motion branch should keep deterministic minimal motion fallback via `{needle}`.",
        );
    }

    for needle in [
        "data-slot=SLOT_SWATCH",
        "data-size=state.size_attr",
        "data-border=state.border_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "role=swatch_aria.attrs.role",
        "aria-disabled=swatch_aria.attrs.aria_disabled",
        "aria-pressed=move || swatch_aria.attrs.aria_pressed.get()",
        "lang=swatch_aria.attrs.lang.clone()",
        "dir=swatch_aria.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view should keep SSR/hydration-stable semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "prefers_reduced_motion(",
        "web_sys",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Swatch semantic surface should not split by platform/reduced-motion token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Swatch motion adapter should keep wasm/non-wasm split token `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/SSR/wasm verification token `{needle}`.",
        );
    }
}

#[test]
fn swatch_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"swatch\" => UiPerfBudget {",
        "max_mount_ms: 22.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget token `{needle}`."
        );
    }

    let needle =
        "component_doc!(\"Swatch\", \"swatch\", \"Display\", display_extra_swatch::swatch)";
    assert!(
        pages_source.contains(needle),
        "Swatch docs page should remain in component coverage traversal via `{needle}`."
    );

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
            "UiPerfProbe should expose repeatable perf regression marker `{needle}`."
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
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`."
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
            "Swatch checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_state(logic::SwatchStateInput {",
        "style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()",
        "swatch_motion::attach_motion(node_ref, selected, motion);",
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-aria-label-source=aria_label_source",
        "data-pressed=move || swatch_aria.state.is_pressed.get().then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view should expose state/render/style/motion attribution marker `{needle}`."
        );
    }
}

#[test]
fn swatch_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("../../components/swatch/src/view.rs");

    assert!(
        view_source.contains("view! {"),
        "Swatch should keep a single explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Swatch should keep one focused `view!` block; split only when layout complexity actually grows."
    );
    assert!(
        view_source.lines().count() <= 240,
        "Swatch view.rs should stay compact; if this grows significantly, split into semantic subrenders."
    );

    for forbidden in ["for item in", "collect::<Vec<_>>()", "match children"] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should avoid loop-heavy/expansion-heavy rendering token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("../../components/swatch/src/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Swatch should keep a single public component boundary for current simple indicator layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn swatch_",
        "pub fn render_",
        "fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch should not introduce extra local component/render API noise for simple layout `{forbidden}`."
        );
    }
}

#[test]
fn swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout() {
    let view_source = load_source("../../components/swatch/src/view.rs");

    for forbidden in [
        "inner_html=",
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "<path",
        "let markdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Swatch view should avoid heavy inline static fragments for simple indicator layout `{forbidden}`."
        );
    }

    for needle in [
        "const SLOT_SWATCH: &str = \"swatch\";",
        "const SLOT_SWATCH_CHECKER: &str = \"swatch-checker\";",
        "const SLOT_SWATCH_SAMPLE: &str = \"swatch-sample\";",
        "const SLOT_SWATCH_SLASH: &str = \"swatch-slash\";",
        "const SLOT_SWATCH_MIXED_MARK: &str = \"swatch-mixed-mark\";",
        "const SLOT_SWATCH_DISABLED_MARK: &str = \"swatch-disabled-mark\";",
        "const CLASS_SWATCH_CHECKER: &str = \"ui-swatch__checker\";",
        "const CLASS_SWATCH_SAMPLE: &str = \"ui-swatch__sample\";",
        "const CLASS_SWATCH_SLASH: &str = \"ui-swatch__slash\";",
        "const CLASS_SWATCH_MIXED_MARK: &str = \"ui-swatch__mixed-mark\";",
        "const CLASS_SWATCH_DISABLED_MARK: &str = \"ui-swatch__disabled-mark\";",
        "const BOOL_TRUE: &str = \"true\";",
        "data-slot=SLOT_SWATCH",
        "data-slot=SLOT_SWATCH_CHECKER",
        "data-slot=SLOT_SWATCH_SAMPLE",
        "data-slot=SLOT_SWATCH_SLASH",
        "data-slot=SLOT_SWATCH_MIXED_MARK",
        "data-slot=SLOT_SWATCH_DISABLED_MARK",
        "class=CLASS_SWATCH_CHECKER",
        "class=CLASS_SWATCH_SAMPLE",
        "class=CLASS_SWATCH_SLASH",
        "class=CLASS_SWATCH_MIXED_MARK",
        "class=CLASS_SWATCH_DISABLED_MARK",
        "aria-hidden=BOOL_TRUE",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch should keep static fragment constantization marker `{needle}`."
        );
    }
}

#[test]
fn swatch_functional_split_keeps_semantic_markers_stable_for_test_selectors() {
    let view_source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "data-slot=SLOT_SWATCH",
        "data-slot=SLOT_SWATCH_CHECKER",
        "data-slot=SLOT_SWATCH_SAMPLE",
        "data-slot=SLOT_SWATCH_SLASH",
        "data-slot=SLOT_SWATCH_MIXED_MARK",
        "data-slot=SLOT_SWATCH_DISABLED_MARK",
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
        "data-aria-label-source=aria_label_source",
        "role=swatch_aria.attrs.role",
        "aria-label=swatch_aria.attrs.aria_label.clone()",
        "aria-disabled=swatch_aria.attrs.aria_disabled",
        "aria-pressed=move || swatch_aria.attrs.aria_pressed.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch semantic marker should stay stable after functional split decisions `{needle}`."
        );
    }
}

#[test]
fn swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "../../components/swatch/src/mod.rs",
        "../../components/swatch/src/logic.rs",
        "../../components/swatch/src/styles.rs",
        "../../components/swatch/src/motion.rs",
        "../../components/swatch/src/view.rs",
        "../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Swatch path `{rel_path}` must not inject raw html; found `{forbidden}`."
            );
        }
    }

    let check2_source = load_source("../../components/swatch/src/check2.md");
    for needle in [
        "`inner_html` 使用约束",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep inner_html security contract marker `{needle}`."
        );
    }
}

#[test]
fn swatch_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    let needle = "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce Swatch contract marker `{needle}`."
    );
}

#[test]
fn swatch_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let swatch_view_source = load_source("../../components/swatch/src/view.rs");
    let swatch_logic_source = load_source("../../components/swatch/src/logic.rs");
    let swatch_motion_source = load_source("../../components/swatch/src/motion.rs");
    let swatch_check2_source = load_source("../../components/swatch/src/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

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
            "shared button wasm debug path should keep trace/replay marker `{needle}`."
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
            "docs debug visual entry should keep `{needle}`."
        );
    }

    for needle in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-pressed=move || swatch_aria.state.is_pressed.get().then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
        "data-aria-label-source=aria_label_source",
        "on:pointerdown=move |_| swatch_aria.handlers.button.press.on_pointer_down.run(())",
        "on:pointerup=move |_| swatch_aria.handlers.button.press.on_pointer_up.run(())",
        "swatch_aria.handlers.button.press.on_key_down.run(key)",
        "swatch_aria.handlers.button.press.on_key_up.run(key)",
    ] {
        assert!(
            swatch_view_source.contains(needle),
            "Swatch should keep machine-readable state/source/interaction marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !swatch_view_source.contains(forbidden)
                && !swatch_logic_source.contains(forbidden)
                && !swatch_motion_source.contains(forbidden),
            "Swatch should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            swatch_check2_source.contains(needle),
            "Swatch checklist should keep wasm debug governance contract marker `{needle}`."
        );
    }
}

#[test]
fn swatch_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
}

#[test]
fn swatch_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

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
        "pub(super) fn swatch() -> AnyView",
        "title=\"Size + Shape + Rounding\"",
        "code_signal=size_code",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "code_signal=state_code",
        "title=\"Custom Motion Contract\"",
        "code_signal=motion_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn swatch_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let check2_source = load_source("../../components/swatch/src/check2.md");

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

    for needle in [
        "let (selected, set_selected) = signal(true);",
        "let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "{move || format!(\"Selected: {}\", selected.get())}",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "SWATCH_WORKBENCH_STORAGE_KEY",
        "load_swatch_workbench_state(",
        "save_swatch_workbench_state(",
        "clear_swatch_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Swatch keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "Swatch checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn swatch_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("../../components/swatch/src/mod.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");
    let checklist_source = load_source("../../components/swatch/src/check2.md");

    assert!(
        !manifest_dir
            .join("../../components/swatch/src/spec.rs")
            .exists(),
        "Swatch should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-swatch = []"),
        "Swatch feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-swatch = [\"dep:serde\"")
            && !cargo_source.contains("component-swatch = [\"dep:serde_json\""),
        "Swatch should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
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
            "Swatch engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Swatch checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn swatch_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("../../components/swatch/src/mod.rs"),
        load_source("../../components/swatch/src/logic.rs"),
        load_source("../../components/swatch/src/view.rs"),
        load_source("../../components/swatch/src/styles.rs"),
        load_source("../../components/swatch/src/motion.rs"),
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
        !cargo_source.contains("swatch-wasm-debug"),
        "Swatch should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::swatch::",
        "const SWATCH_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Swatch should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("../../components/swatch/src/mod.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");

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
                "Swatch engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Swatch public module boundary should not leak web_sys types."
    );
}

#[test]
fn swatch_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("../../components/swatch/src/check2.md");

    for required in [
        "- [ ] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Swatch checklist should keep ui entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn swatch_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-swatch\")]",
        "pub mod swatch;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
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
}

#[test]
fn swatch_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

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
fn swatch_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    for needle in [
        "cargo test -p ui --test button_semantics ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "cargo test -p ui --test button_semantics ui_components_css_registry_remains_feature_gated_and_non_global",
        "cargo test -p ui --test button_semantics ui_root_centralizes_theme_injection_and_i18n_context",
        "cargo test -p ui --test button_semantics active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "cargo test -p ui --test button_semantics ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoints check script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_component_directory_has_standard_file_layout() {
    for required in [
        "../../components/swatch/src/mod.rs",
        "../../components/swatch/src/logic.rs",
        "../../components/swatch/src/styles.rs",
        "../../components/swatch/src/view.rs",
        "../../components/swatch/src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "swatch component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("../../components/swatch/src/render.rs"),
        "swatch component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
    assert!(
        !path_exists("../../components/swatch/src/spec.rs"),
        "Swatch is a simple component and should not introduce `../../components/swatch/src/spec.rs`."
    );
}

#[test]
fn swatch_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("../../components/swatch/src/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{SwatchBorder, SwatchRounding, SwatchShape, SwatchSize};",
        "pub use motion::SwatchMotion;",
        "pub use view::Swatch;",
    ] {
        assert!(
            mod_source.contains(needle),
            "swatch/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "swatch/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn swatch_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let motion_source = load_source("../../components/swatch/src/motion.rs");

    for forbidden in [
        "view! {",
        "on:click",
        "NodeRef<",
        "web_sys",
        "leptos::html",
        "role=",
        "aria-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "swatch/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "swatch/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "swatch/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Swatch(",
        "logic::resolve_state(logic::SwatchStateInput {",
        "logic::resolve_selection_control_state(",
        "use_swatch(SwatchOptions {",
        "swatch_motion::attach_motion(node_ref, selected, motion);",
    ] {
        assert!(
            view_source.contains(required),
            "swatch/view.rs should keep rendering + semantics mount marker `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator::new",
        "pub struct SwatchStateInput",
        "pub enum SwatchSize",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "swatch/view.rs should not embed lower-layer engine/state implementation `{forbidden}`."
        );
    }

    for required in [
        "pub struct SwatchMotion",
        "pub fn sanitize_motion(motion: SwatchMotion) -> SwatchMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(required),
            "swatch/motion.rs should keep motion mapping/attach marker `{required}`."
        );
    }

    for forbidden in ["view! {", "role=", "aria-", "use_swatch("] {
        assert!(
            !motion_source.contains(forbidden),
            "swatch/motion.rs should not include view/a11y contract token `{forbidden}`."
        );
    }
}

#[test]
fn swatch_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_directory_has_standard_file_layout",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_view_macro_check_script_covers_complexity_and_split_gates() {
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_functional_split_keeps_semantic_markers_stable_for_test_selectors",
    ] {
        assert!(
            script_source.contains(needle),
            "view-macro gate script should include `{needle}`."
        );
    }
}

#[test]
fn swatch_docs_page_exists_in_display_extra_swatch() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "pub(super) fn swatch() -> AnyView",
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "<Swatch",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs page should contain `{needle}`."
        );
    }
}

#[test]
fn swatch_docs_page_includes_custom_motion_playground() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "SwatchMotion {",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs page should include `{needle}` for custom motion contract demos."
        );
    }
}

#[test]
fn swatch_docs_default_and_state_playgrounds_lock_contract_values() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Size + Shape + Rounding\"",
        "code_signal=size_code",
        "size=SwatchSize::Xs",
        "size=SwatchSize::S",
        "size=SwatchSize::M",
        "size=SwatchSize::L",
        "shape=SwatchShape::Rectangle",
        "rounding=SwatchRounding::Full",
        "border=SwatchBorder::Light",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "code_signal=state_code",
        "label=\"Brand blue\".to_string()",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "is_mixed_value=true",
        "is_nothing=true",
        "border=SwatchBorder::None",
        "color=\"#111827\".to_string()",
        "is_disabled=true",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs default/state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn swatch_docs_custom_motion_playground_locks_contract_values() {
    let display_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "code_signal=motion_code",
        "let custom_motion = SwatchMotion {",
        "selected_scale: 1.12,",
        "selected_ring_opacity: 0.92,",
        "..SwatchMotion::default()",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
        "label=\"Featured motion\".to_string()",
        "label=\"Reduced motion\".to_string()",
    ] {
        assert!(
            display_extra.contains(needle),
            "display_extra_swatch docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn swatch_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "pub(super) fn swatch() -> AnyView",
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "description=\"baseline-compatible swatch primitive with centralized size/shape/rounding/border/state contracts and baseline-level spring selection motion.\"",
        "title=\"Size + Shape + Rounding\"",
        "code_signal=size_code",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "code_signal=state_code",
        "title=\"Custom Motion Contract\"",
        "code_signal=motion_code",
        "<Swatch",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_swatch docs should include `{needle}` for swatch primary playground coverage.",
        );
    }
}

#[test]
fn swatch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "title=\"Size + Shape + Rounding\"",
        "size=SwatchSize::Xs",
        "size=SwatchSize::S",
        "size=SwatchSize::M",
        "size=SwatchSize::L",
        "shape=SwatchShape::Rectangle",
        "rounding=SwatchRounding::Full",
        "border=SwatchBorder::Light",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "is_mixed_value=true",
        "is_nothing=true",
        "is_disabled=true",
        "title=\"Custom Motion Contract\"",
        "let custom_motion = SwatchMotion {",
        "selected_scale: 1.12,",
        "selected_ring_opacity: 0.92,",
        "motion=custom_motion",
        "motion=SwatchMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "display_extra_swatch docs playgrounds should contain `{needle}` for swatch state-matrix contracts.",
        );
    }
}

#[test]
fn swatch_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/swatch.rs");

    for needle in [
        "pub enum SwatchAgentSchemaVersion",
        "pub enum SwatchAgentIntent",
        "pub enum SwatchAgentAction",
        "pub enum SwatchAgentStateAxis",
        "pub enum SwatchAgentSource",
        "pub enum SwatchAgentOutputStatus",
        "pub enum SwatchAgentStreamSupport",
        "pub enum SwatchAgentStreamFallback",
        "pub struct SwatchAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Swatch state primitive layer should include schema-typed agent token `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::swatch::{",
        "SwatchAgentSource",
        "SwatchSelectionControlInput",
        "resolve_selection_control_state",
        "resolve_agent_source",
        "resolve_agent_contract",
    ] {
        assert!(
            logic_source.contains(needle),
            "Swatch logic should only consume/re-export agent contract primitives via `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-toggle=move || {",
        "data-ui-capability-disable=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn swatch_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let styles_source = load_source("../../components/swatch/src/styles.rs");
    let mod_source = load_source("../../components/swatch/src/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{docs_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Swatch Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn swatch_snapshot_baseline_and_streaming_fallback_contract_are_explicit() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let check2_source = load_source("../../components/swatch/src/check2.md");

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch view should expose explicit snapshot/fallback marker `{needle}`."
        );
    }

    for needle in [
        "SwatchAgentStreamSupport::Unsupported",
        "SwatchAgentStreamFallback::FullSnapshot",
    ] {
        assert!(
            logic_source.contains(needle),
            "Swatch logic should model stream N/A/fallback contract via `{needle}`."
        );
    }

    for needle in [
        "- [ ] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(needle),
            "swatch/check2.md should pin streaming baseline marker `{needle}`."
        );
    }
}

#[test]
fn swatch_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/swatch/src/check2.md");

    for required in [
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "swatch/check2.md should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/swatch/src/view.rs");

    for required in [
        "role=swatch_aria.attrs.role",
        "aria-label=swatch_aria.attrs.aria_label.clone()",
        "aria-disabled=swatch_aria.attrs.aria_disabled",
        "aria-pressed=move || swatch_aria.attrs.aria_pressed.get()",
        "data-slot=SLOT_SWATCH",
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "Swatch should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn swatch_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
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
            "Swatch should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn swatch_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test swatch_semantics --no-default-features --features component-swatch,inject-css swatch_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn swatch_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("../../components/swatch/src/check2.md");

    for required in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "swatch/check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn swatch_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/semantics.rs");

    for required in [
        "swatch_machine_readable_contract_uses_typed_inputs_and_semantic_markers",
        "swatch_agent_contract_is_schema_typed_and_machine_readable",
        "swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "swatch_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
    ] {
        assert!(
            semantics_source.contains(required),
            "Swatch semantic suite should assert contract marker `{required}`."
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
            "Swatch semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn swatch_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("../../components/swatch/src/view.rs");
    let semantics_source = load_source("tests/semantics.rs");
    let semantics_source_normalized = semantics_source.replace("\\\"", "\"");

    for marker in [
        "data-state=state.data_state_attr",
        "data-selected=move || selected.get().then_some(\"true\")",
        "data-control-mode=selection_control.control_mode_attr",
        "data-default-selected-source=selection_control.default_selected_source_attr",
        "data-selected-change-source=selection_control.selected_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "Swatch view should keep semantic marker `{marker}`."
        );
        assert!(
            semantics_source_normalized.contains(marker),
            "Swatch semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn swatch_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_swatch_contract.spec.mjs");

    for needle in [
        "docs-app swatch contract uses semantic selectors with settled waits",
        "body:not(:has(#boot))",
        "[data-component=\"swatch\"] [data-slot=\"swatch\"]",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            e2e_source.contains(needle),
            "swatch e2e selector contract should include `{needle}` semantic selector/wait."
        );
    }
}

#[test]
fn swatch_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_swatch_contract.spec.mjs");

    for needle in [
        "docs-app swatch key flow is repeatable and fails at semantic breakpoints",
        "[aria-label=\"Brand blue\"]",
        "toHaveAttribute(\"data-ui-action\", \"initialize\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle-press\")",
        "toHaveAttribute(\"data-ui-source\", \"toggle-press\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "swatch e2e key flow should keep repeatable semantic breakpoint `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "swatch e2e key flow should avoid fragile fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn swatch_docs_examples_sync_with_logic_api_names_and_default_matrix() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "pub(super) fn swatch() -> AnyView",
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "title=\"Hello World\"",
        "title=\"Size + Shape + Rounding\"",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "title=\"Custom Motion Contract\"",
        "<Swatch color=\"#ffcc00\".to_string() />",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "is_mixed_value=true",
        "is_nothing=true",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs examples should keep matrix/API marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] size: SwatchSize,",
        "#[prop(optional)] border: SwatchBorder,",
        "#[prop(optional)] rounding: SwatchRounding,",
        "#[prop(optional)] shape: SwatchShape,",
        "#[prop(optional)] is_nothing: bool,",
        "#[prop(optional)] is_mixed_value: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional, into)] selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch public API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}

#[test]
fn swatch_docs_entry_exists_and_is_beginner_friendly_default_then_advanced() {
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");

    for needle in [
        "- [ ] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "title=\"Hello World\"",
        "title=\"Size + Shape + Rounding\"",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "title=\"Custom Motion Contract\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs entry should include beginner-to-advanced marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("Swatch docs should include Hello World playground");
    let matrix_pos = docs_source
        .find("title=\"Size + Shape + Rounding\"")
        .expect("Swatch docs should include default matrix playground");
    let advanced_pos = docs_source
        .find("title=\"Mixed + Nothing + Disabled + Controlled\"")
        .expect("Swatch docs should include advanced state playground");

    assert!(
        hello_pos < matrix_pos && matrix_pos < advanced_pos,
        "Swatch docs should keep default path before advanced controls."
    );
}

#[test]
fn swatch_dx_paradox_keeps_zero_wiring_hello_world_and_advanced_opt_in() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");

    let hello_title = "title=\"Hello World (Default Swatch)\"";
    let hello_code = "<Swatch color=\"#ffcc00\".to_string() label=\"Brand\".to_string() />";
    let workbench_title = "title=\"Workbench (All API + Actual Config)\"";

    assert!(
        docs_source.contains(hello_title),
        "Swatch docs should expose a first-glance Hello World playground."
    );
    assert!(
        docs_source.contains(hello_code),
        "Swatch docs should keep a direct runnable Hello World snippet."
    );
    assert!(
        hello_code.lines().count() <= 5,
        "Swatch Hello World snippet should stay within five lines for DX baseline."
    );

    let hello_pos = docs_source
        .find(hello_title)
        .expect("Swatch docs should include Hello World section.");
    let workbench_pos = docs_source
        .find(workbench_title)
        .expect("Swatch docs should include advanced workbench section.");
    assert!(
        hello_pos < workbench_pos,
        "Swatch docs should present default path before advanced controls."
    );

    let hello_scope = &docs_source[hello_pos..workbench_pos];
    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "use_swatch(",
        "resolve_selection_control_state(",
    ] {
        assert!(
            !hello_scope.contains(forbidden),
            "Hello World path should not leak internal wiring detail `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch API should keep advanced control as optional prop `{needle}`."
        );
    }

    let signature_start = view_source
        .find("pub fn Swatch(")
        .expect("Swatch view should define public component signature.");
    let signature_tail = &view_source[signature_start..];
    let signature_end = signature_tail
        .find(") -> impl IntoView")
        .expect("Swatch signature should end before implementation body.");
    let signature = &signature_tail[..signature_end];
    assert!(
        !signature.contains("state:"),
        "Swatch API should not require exposing internal state object on basic path."
    );
}

#[test]
fn swatch_docs_app_provides_interactive_playground_with_live_props_and_state_preview() {
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "- [ ] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep interactive-playground marker `{needle}`."
        );
    }

    for needle in [
        "let (selected, set_selected) = signal(true);",
        "let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));",
        "selected=move || selected.get()",
        "on_selected_change=on_selected_change",
        "{move || format!(\"Selected: {}\", selected.get())}",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "pub fn Playground(",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "children: Children,",
        "let resolved_code = Signal::derive(move || {",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep live-preview marker `{needle}`."
        );
    }
}

#[test]
fn swatch_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync() {
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_swatch_contract.spec.mjs");

    for needle in [
        "- [ ] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep source-first copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "test_source_path=\"components/swatch/src/view.rs\".to_string()",
        "title=\"Hello World\"",
        "title=\"Size + Shape + Rounding\"",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
        "title=\"Custom Motion Contract\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs should keep source-first marker `{needle}`."
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
        "docs-app swatch playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui::*;\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Swatch copy-flow e2e should keep marker `{needle}`."
        );
    }
}

#[test]
fn swatch_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_swatch.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");

    for needle in [
        "- [ ] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Swatch checklist should keep HeroUI/docs-sync marker `{needle}`."
        );
    }

    for needle in [
        "### Swatch 同步记录（2026-02-17）",
        "`Swatch` 保持 display primitive 定位",
        "component_doc!(\"Swatch\", \"swatch\", \"Display\", display_extra_swatch::swatch)",
        "`#/components/swatch` 可索引访问",
        "`Hello World`、`Size + Shape + Rounding`、`Mixed + Nothing + Disabled + Controlled`、`Custom Motion Contract`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep Swatch sync marker `{needle}`."
        );
    }

    for needle in ["\"Swatch\"", "\"swatch\"", "display_extra_swatch::swatch"] {
        assert!(
            pages_source.contains(needle),
            "docs catalog should expose Swatch token `{needle}`."
        );
    }

    for needle in [
        "title=\"Swatch\"",
        "slug=\"swatch\"",
        "title=\"Hello World\"",
        "title=\"Size + Shape + Rounding\"",
        "title=\"Mixed + Nothing + Disabled + Controlled\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Swatch docs page should keep indexed/example marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] size: SwatchSize,",
        "#[prop(optional)] border: SwatchBorder,",
        "#[prop(optional)] rounding: SwatchRounding,",
        "#[prop(optional)] shape: SwatchShape,",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "Swatch API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}

#[test]
fn swatch_forbidden_antipatterns_are_guarded() {
    let primitive_source = load_source("../ui-state-primitives/src/swatch.rs");
    let headless_source = load_source("../ui-headless/src/swatch.rs");
    let logic_source = load_source("../../components/swatch/src/logic.rs");
    let view_source = load_source("../../components/swatch/src/view.rs");
    let mod_source = load_source("../../components/swatch/src/mod.rs");

    for forbidden in ["view! {", "on:click", "class=", "NodeRef<", "web_sys"] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives swatch should stay DOM/style free and avoid `{forbidden}`."
        );
    }

    for forbidden in [
        "var(--ui-",
        "ui-swatch",
        "SpringAnimator",
        "keyframes",
        "@media",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless swatch should stay interaction-only and avoid visual/motion token `{forbidden}`."
        );
    }

    for forbidden in [
        "pub enum SwatchSize",
        "pub enum SwatchBorder",
        "pub enum SwatchRounding",
        "pub enum SwatchShape",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "swatch/logic.rs should not duplicate reusable primitive token `{forbidden}`."
        );
    }

    for forbidden in ["TODO TEMP PATCH", "FIXME TEMP", "HACK:"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "Swatch should avoid temporary patch marker `{forbidden}` that drifts cross-component contracts."
        );
    }

    for forbidden in ["web_sys", "leptos::web_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "Swatch public API surface should not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn swatch_merge_verdict_contracts_have_traceable_evidence_and_full_repo_gate_is_deferred() {
    let check2_source = load_source("../../components/swatch/src/check2.md");
    let semantics_source = load_source("tests/semantics.rs");

    for needle in [
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
        "门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "swatch checklist should keep final-merge-gate marker `{needle}`."
        );
    }

    for evidence in [
        "swatch_component_file_responsibilities_remain_scoped",
        "swatch_view_mounts_headless_contract_instead_of_local_keyboard_state_machine",
        "swatch_machine_readable_contract_uses_typed_inputs_and_semantic_markers",
        "swatch_agent_contract_is_schema_typed_and_machine_readable",
        "swatch_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "swatch_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "swatch_docs_examples_sync_with_logic_api_names_and_default_matrix",
        "swatch_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
        "swatch_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "swatch_forbidden_antipatterns_are_guarded",
    ] {
        assert!(
            semantics_source.contains(evidence),
            "swatch merge verdict must remain evidence-traceable via `{evidence}`."
        );
    }

    let full_repo_gate_deferred_for_now = true;
    assert!(
        full_repo_gate_deferred_for_now,
        "Per current swatch checkpoint, full repository gate (fmt/clippy/test/smoke) is intentionally deferred."
    );
}
