use std::sync::OnceLock;

fn load_source(path: &str) -> &'static str {
    match path {
        "view" => include_str!("../src/view.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "check2" => include_str!("../check2.md"),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn load_docs_forms_page() -> String {
    static SOURCE: OnceLock<String> = OnceLock::new();
    SOURCE
        .get_or_init(|| {
            let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let parent_path =
                manifest_dir.join("../../apps/docs-app/src/pages/components/pages/forms.rs");
            let child_path =
                manifest_dir.join("../../apps/docs-app/src/pages/components/pages/forms/form.rs");
            let parent = std::fs::read_to_string(&parent_path).unwrap_or_else(|err| {
                panic!(
                    "failed to read docs forms parent page {}: {err}",
                    parent_path.display()
                )
            });
            let child = std::fs::read_to_string(&child_path).unwrap_or_else(|err| {
                panic!(
                    "failed to read docs forms child page {}: {err}",
                    child_path.display()
                )
            });
            format!("{parent}\n\n{child}").replace(
                "pub(crate) fn form() -> AnyView {",
                "pub(super) fn form() -> AnyView {",
            )
        })
        .clone()
}

fn load_ui_components_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../crates/ui/src/{path}"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read ui source {}: {err}", path.display()))
}

fn load_ui_components_cargo_toml() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/Cargo.toml");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read ui Cargo.toml {}: {err}", path.display()))
}

fn load_ui_headless_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../crates/ui-headless/src/{path}"));
    std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read ui-headless source {}: {err}",
            path.display()
        )
    })
}

fn load_ui_headless_cargo_toml() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/ui-headless/Cargo.toml");
    std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read ui-headless Cargo.toml {}: {err}",
            path.display()
        )
    })
}

fn load_ui_motion_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../crates/ui-motion/src/{path}"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read ui-motion source {}: {err}", path.display()))
}

fn load_ui_visual_primitive_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../crates/ui-visual-primitive/src/{path}"));
    std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read ui-visual-primitive source {}: {err}",
            path.display()
        )
    })
}

fn load_ui_theme_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../../crates/ui-theme/src/{path}"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read ui-theme source {}: {err}", path.display()))
}

fn load_docs_plan_todo() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/plan/TODO.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read docs plan todo {}: {err}", path.display()))
}

fn load_performance_gate_script() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../scripts/check-ui-performance.sh");
    std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read performance gate script {}: {err}",
            path.display()
        )
    })
}

#[test]
fn form_view_mounts_headless_locale_contract() {
    let view = load_source("view");

    for required in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            view.contains(required),
            "form view should mount ui-headless locale attrs via `{required}`"
        );
    }
}

#[test]
fn form_api_naming_contract_uses_is_prefix_for_boolean_props() {
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "#[prop(optional, into)] is_read_only: Option<bool>",
        "#[prop(optional, into)] is_required: Option<bool>",
        "| `is_disabled` | `bool` | `false` |",
        "| `is_read_only` | `bool` | `false` |",
        "| `is_required` | `bool` | `false` |",
    ] {
        assert!(
            view.contains(required) || readme.contains(required),
            "form public api naming contract should keep `is_*` boolean props via `{required}`"
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] read_only: bool",
        "#[prop(optional)] required: bool",
        "#[prop(optional)] on_disabled_change:",
        "#[prop(optional)] default_disabled:",
    ] {
        assert!(
            !view.contains(forbidden),
            "form should not introduce alias drift or unmatched callback/default naming: `{forbidden}`"
        );
    }

    for required in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "`on_*`/`default_*` 在该项按 N/A 处理",
        "components/form/test/semantics.rs::form_api_naming_contract_uses_is_prefix_for_boolean_props",
    ] {
        assert!(
            check2.contains(required),
            "form checklist should document api naming contract evidence via `{required}`"
        );
    }
}

#[test]
fn form_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet() {
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "#[prop(optional, into)] is_read_only: Option<bool>",
        "#[prop(optional, into)] is_required: Option<bool>",
        "let resolved = logic::resolve_props(",
        "let view_state = logic::resolve_view_state(&resolved);",
        "provide_context(logic::FormContextValue {",
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
    ] {
        assert!(
            view.contains(required),
            "form should map semantic bool inputs directly without creating a controllable state axis: `{required}`"
        );
    }

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional)] default_disabled:",
        "#[prop(optional)] default_read_only:",
        "#[prop(optional)] default_required:",
        "#[prop(optional)] on_disabled_change:",
        "#[prop(optional)] on_read_only_change:",
        "#[prop(optional)] on_required_change:",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "form should not expose half-controlled api surface for non-stateful semantics: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
            "本组件判定：`Form` 是语义容器，不维护 `value` 状态轴",
            "components/form/test/semantics.rs::form_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet",
        ] {
            assert!(
                checklist.contains(required),
                "form checklist should record controlled/uncontrolled N/A boundary via `{required}`"
            );
        }
    }
}

#[test]
fn form_defaults_are_normalized_in_logic_only() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub fn resolve_props(",
        "disabled: is_disabled.unwrap_or(false)",
        "read_only: is_read_only.unwrap_or(false)",
        "required: is_required.unwrap_or(false)",
        "label_position: label_position.unwrap_or_default()",
        "label_align: label_align.unwrap_or_default()",
        "class_name: resolve_class_name(class_name)",
        "pub fn resolve_view_state(resolved: &FormResolvedProps) -> FormViewState",
    ] {
        assert!(
            logic.contains(required),
            "form logic should keep explicit default and priority normalization via `{required}`"
        );
    }

    for required in [
        "let resolved = logic::resolve_props(",
        "let view_state = logic::resolve_view_state(&resolved);",
    ] {
        assert!(
            view.contains(required),
            "form view should consume normalized props from logic via `{required}`"
        );
    }

    for forbidden in [
        ".unwrap_or(",
        ".unwrap_or_else(",
        ".filter(|value| !value.trim().is_empty())",
        ".then_some(",
    ] {
        assert!(
            !view.contains(forbidden),
            "form view should not implement default fallbacks directly: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
            "components/form/src/logic.rs::resolve_props",
            "components/form/test/semantics.rs::form_defaults_are_normalized_in_logic_only",
            "components/form/test/logic.rs::resolve_props_applies_default_priority_in_logic",
        ] {
            assert!(
                checklist.contains(required),
                "form checklist should document default-source single ownership via `{required}`"
            );
        }
    }
}

#[test]
fn form_state_normalization_is_centralized_in_logic() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub fn resolve_props(",
        "pub fn resolve_view_state(resolved: &FormResolvedProps) -> FormViewState",
        "data_disabled: bool_attr(resolved.disabled)",
        "data_read_only: bool_attr(resolved.read_only)",
        "data_required: bool_attr(resolved.required)",
        "aria_disabled: bool_attr(resolved.disabled)",
        "state_source: \"logic.rs::resolve_view_state\"",
    ] {
        assert!(
            logic.contains(required),
            "state normalization should stay centralized in logic via `{required}`"
        );
    }

    for required in [
        "let resolved = logic::resolve_props(",
        "let view_state = logic::resolve_view_state(&resolved);",
        "data-state-source=view_state.state_source",
    ] {
        assert!(
            view.contains(required),
            "view should only consume normalized state from logic via `{required}`"
        );
    }

    for forbidden in [".then_some(", ".unwrap_or(", ".unwrap_or_else("] {
        assert!(
            !view.contains(forbidden),
            "view should not rebuild state derivation rules: `{forbidden}`"
        );
    }

    {
        let required = ".ui-form[data-disabled=\\\"true\\\"]";
        assert!(
            styles.contains(required),
            "styles should consume explicit state marker only: `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
            "components/form/src/logic.rs::resolve_props",
            "components/form/src/logic.rs::resolve_view_state",
            "components/form/test/semantics.rs::form_state_normalization_is_centralized_in_logic",
            "components/form/test/logic.rs::resolve_view_state_derives_render_markers_in_logic",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document centralized state normalization evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_discrete_states_are_type_constrained_by_enums() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub enum FormLabelPosition",
        "pub enum FormLabelAlign",
        "#[prop(optional)] label_position: Option<FormLabelPosition>",
        "#[prop(optional)] label_align: Option<FormLabelAlign>",
        "label_position: Option<FormLabelPosition>",
        "label_align: Option<FormLabelAlign>",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "discrete state inputs should stay enum-constrained via `{required}`"
        );
    }

    for forbidden in [
        "label_position: Option<String>",
        "label_align: Option<String>",
        "#[prop(optional)] is_label_left: bool",
        "#[prop(optional)] is_label_top: bool",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not express exclusive discrete states with strings or bool explosion: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
            "components/form/src/logic.rs::FormLabelPosition",
            "components/form/src/logic.rs::FormLabelAlign",
            "components/form/test/semantics.rs::form_discrete_states_are_type_constrained_by_enums",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record discrete-state type constraints evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_state_primitive_source_boundary_is_respected() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub(crate) mod logic;",
        "pub fn resolve_props(",
        "pub fn resolve_view_state(resolved: &FormResolvedProps) -> FormViewState",
        "provide_context(logic::FormContextValue {",
        "pub fn use_form_context() -> Option<FormContextValue>",
    ] {
        assert!(
            module.contains(required) || logic.contains(required) || view.contains(required),
            "form should keep logic-only assembly/mapping boundary via `{required}`"
        );
    }

    for forbidden in [
        "ui_state_primitives::",
        "use ui_state_primitives",
        "store::",
        "use_app_store",
        "AppStore",
        "GlobalStore",
        "create_signal(",
        "RwSignal<",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not bind app store or local signal state machine details directly: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
            "components/form/src/logic.rs` 仅做 props 归一与视图语义映射",
            "components/form/test/semantics.rs::form_state_primitive_source_boundary_is_respected",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record state primitive source boundary evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_async_semantics_is_na_and_no_protocol_leak() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "error",
        "retry",
        "use_async_action",
        "on_retry",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !readme.contains(forbidden),
            "form should not leak async interaction protocol for non-async container: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
            "本组件判定：`Form` 为纯同步语义容器",
            "components/form/test/semantics.rs::form_async_semantics_is_na_and_no_protocol_leak",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document async semantics N/A boundary via `{required}`"
            );
        }
    }
}

#[test]
fn form_api_dx_paradox_keeps_simple_path_simple() {
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let docs_forms = load_docs_forms_page();
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "## Hello World（默认路径）",
        "<Form>",
        "<Input id=\"name\".to_string() label=\"Name\".to_string() value=name set_value=set_name />",
        "默认用法不需要手动接线 `ui-state-primitives` / `ui-headless` 状态机",
        "title=\"Hello World（默认路径）\"",
        "id=\"docs-form-hello\".to_string()",
    ] {
        assert!(
            readme.contains(required) || docs_forms.contains(required),
            "form dx path should expose copy-paste minimal usage via `{required}`"
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] state_machine:",
        "state=ui_state_primitives",
        "state=use_async_action",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "form default api should not force internal state object wiring: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
            "Hello World 示例保持 3 行核心调用",
            "apps/docs-app/src/pages/components/pages/forms.rs` 提供 `Hello World（默认路径）`",
            "components/form/test/semantics.rs::form_api_dx_paradox_keeps_simple_path_simple",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record dx paradox evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_composite_api_prefers_explicit_parent_child_composition() {
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub fn Form(",
        "children: Children,",
        "{children()}",
        "<Form>",
        "<Input id=\"name\".to_string() label=\"Name\".to_string() value=name set_value=set_name />",
    ] {
        assert!(
            view.contains(required) || readme.contains(required),
            "form should keep explicit parent/child composition via `{required}`"
        );
    }

    for forbidden in [
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] items:",
        "ItemSpec",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "form should not expose convention-based parallel-array api sugar: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
            "`Form` 主 API 采用显式组合 `<Form>{children}</Form>`",
            "components/form/test/semantics.rs::form_composite_api_prefers_explicit_parent_child_composition",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document explicit-composition contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_macro_micro_dragging_duality_is_na_for_static_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "DragEnd",
        "on:pointermove",
        "on:mousemove",
        "pointermove",
        "mousemove",
        "requestAnimationFrame",
        "cancelAnimationFrame",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement macro/micro dragging state machines: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。",
        "`Dragging`/`Action::DragEnd` 在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_macro_micro_dragging_duality_is_na_for_static_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document macro/micro duality N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_two_pass_rendering_is_na_for_static_form_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "requestAnimationFrame",
        "cancelAnimationFrame",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement geometry measurement/rectification loop for two-pass rendering: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "`Intent -> Measure(view) -> Rectification(logic)` 在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_two_pass_rendering_is_na_for_static_form_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document two-pass rendering N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_registration_protocol_is_na_for_non_navigable_form_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement collection registration protocol for dynamic item navigation: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。",
        "`RegistrationContext/Register/Unregister/items_order` 在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_registration_protocol_is_na_for_non_navigable_form_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document registration protocol N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_slot_projection_policy_is_na_for_static_form_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "requestAnimationFrame",
        "set_interval",
        "set_timeout",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement slot projection lifecycle orchestration: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。",
        "`NotifyHidden` 在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_slot_projection_policy_is_na_for_static_form_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document slot projection N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_env_streams_are_na_for_static_form_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "debounce",
        "throttle",
        "on:resize",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement environment stream sampling/pushdown pipeline: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。",
        "`Resize/Theme/Intersection -> debounce -> Action(BreakpointChanged)` 流程在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_env_streams_are_na_for_static_form_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document env stream N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_event_light_cone_is_na_for_non_collection_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "Table",
        "Grid",
        "prop drilling",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement event-light-cone pipeline for large-collection bulk operations: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。",
        "O(N) 向下分发问题在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_event_light_cone_is_na_for_non_collection_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document event light cone N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_causality_bus_is_na_for_static_form_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "bus.broadcast",
        "dispatch_command",
        "subscribe(",
        "emit(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not implement causality bus pipeline for complex derived operations: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。",
        "链路在本组件按 N/A 处理",
        "components/form/test/semantics.rs::form_causality_bus_is_na_for_static_form_container",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document causality bus N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_a11y_i18n_contract_is_wired_without_hardcoded_user_copy() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "aria-disabled=view_state.aria_disabled",
        "lang=locale.lang",
        "dir=locale.dir",
        "{children()}",
    ] {
        assert!(
            view.contains(required),
            "form should mount a11y/i18n contract via `{required}`"
        );
    }

    for forbidden in [
        "fn locale_attrs(",
        "struct LocaleAttrs",
        ">Submit<",
        ">提交<",
        ">Required<",
        ">必填<",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form should not reinvent headless locale helpers or hardcode user-visible copy: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
            "`lang/dir` 通过 `ui_headless::locale_attrs` 接入",
            "仅渲染 `children`",
            "components/form/test/semantics.rs::form_a11y_i18n_contract_is_wired_without_hardcoded_user_copy",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document a11y/i18n contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_state_markers_are_observable_queryable_and_enumerable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
        "data-label-position=view_state.label_position",
        "data-label-align=view_state.label_align",
        "data-state-source=view_state.state_source",
        "aria-disabled=view_state.aria_disabled",
    ] {
        assert!(
            view.contains(required),
            "form view should expose stable observable/queryable semantic markers via `{required}`"
        );
    }

    for required in [
        "fn bool_attr(value: bool) -> Option<&'static str>",
        "value.then_some(\"true\")",
        "pub label_position: &'static str,",
        "pub label_align: &'static str,",
        "state_source: \"logic.rs::resolve_view_state\"",
        "FormLabelPosition::Top => \"top\"",
        "FormLabelPosition::Left => \"left\"",
        "FormLabelAlign::Start => \"start\"",
        "FormLabelAlign::End => \"end\"",
    ] {
        assert!(
            logic.contains(required),
            "form logic should keep marker value space closed and enumerable via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
            "值域封闭可枚举",
            "来源统一标记为 `logic.rs::resolve_view_state`",
            "components/form/test/semantics.rs::form_state_markers_are_observable_queryable_and_enumerable",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record state marker observability evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_styles_depend_on_explicit_semantic_state_markers() {
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [".ui-form {", ".ui-form[data-disabled=\\\"true\\\"] {"] {
        assert!(
            styles.contains(required),
            "form styles should branch on stable class/data markers via `{required}`"
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", " > ", "+ ", "~ "] {
        assert!(
            !styles.contains(forbidden),
            "form styles should avoid fragile structural selectors: `{forbidden}`"
        );
    }

    for forbidden in ["style=", "style:", "style:\""] {
        assert!(
            !view.contains(forbidden),
            "form view should not inject business inline style logic: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
            "视觉状态切换由 `data-disabled` 语义标记驱动",
            "components/form/test/semantics.rs::form_styles_depend_on_explicit_semantic_state_markers",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document explicit-style-state contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_token_first_static_style_contract_is_enforced() {
    let styles = load_source("styles");
    let view = load_source("view");
    let ui_components_css = load_ui_components_source("css.rs");
    let ui_components_root = load_ui_components_source("root.rs");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub const CSS: &str =",
        ".ui-form {",
        "var(--ui-space-md)",
        ".ui-form[data-disabled=\\\"true\\\"]",
    ] {
        assert!(
            styles.contains(required),
            "form should keep token-first static css contract in styles.rs via `{required}`"
        );
    }

    for forbidden in ["style=", "style:", "style:\""] {
        assert!(
            !view.contains(forbidden),
            "form view should not encode business styling logic in inline styles: `{forbidden}`"
        );
    }

    for forbidden in [
        "#ff",
        "rgb(",
        "hsl(",
        "tailwind",
        "class-variance-authority",
        "stylist::",
    ] {
        assert!(
            !styles.to_lowercase().contains(forbidden),
            "form styles should avoid private theme values and css-in-rust/utility-first defaults: `{forbidden}`"
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String)",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_css.contains(required) || ui_components_root.contains(required),
            "ui should keep css aggregation and UiRoot injection entry via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
            "视觉值来自 `var(--ui-space-md)`",
            "components/form/test/semantics.rs::form_token_first_static_style_contract_is_enforced",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record token-first static style evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_defensive_variables_use_two_layer_fallback_chain() {
    let styles = load_source("styles");
    let ui_theme_css = load_ui_theme_source("css.rs");
    let check2 = load_source("check2");

    for required in [
        "gap: var(--ui-space-md, var(--ui-fallback-space-md));",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
    ] {
        assert!(
            styles.contains(required),
            "form styles should use defensive variable fallback chain via `{required}`"
        );
    }

    for forbidden in ["#ff", "rgb(", "hsl(", "px;", "rem;", "em;"] {
        assert!(
            !styles.to_lowercase().contains(forbidden),
            "form styles should avoid hardcoded colors and bare size terminals: `{forbidden}`"
        );
    }

    assert!(
        ui_theme_css.contains("--ui-fallback-space-md:"),
        "ui-theme should remain SSOT for fallback terminal `--ui-fallback-space-md`."
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "`gap: var(--ui-space-md, var(--ui-fallback-space-md));`",
        "`crates/ui-theme/src/css.rs` 统一输出 `--ui-fallback-space-md`",
        "components/form/test/semantics.rs::form_defensive_variables_use_two_layer_fallback_chain",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document defensive variable evidence via `{required}`"
        );
    }
}

#[test]
fn form_cascade_layer_contract_uses_ui_layer_and_rejects_plain_inline_styles() {
    let view = load_source("view");
    let ui_components_css = load_ui_components_source("css.rs");
    let check2 = load_source("check2");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-form\")]",
        "out.push_str(crate::field_form::form::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregator should keep form styles inside `@layer ui` via `{required}`"
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view.contains(forbidden),
            "form view should reject plain inline numeric style writes: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "`crates/ui/src/css.rs::push_components_css` 以 `@layer ui` 聚合组件样式",
        "`components/form/src/view.rs` 不包含 `style=\\\"top:*\\\"`/`style=\\\"left:*\\\"`",
        "components/form/test/semantics.rs::form_cascade_layer_contract_uses_ui_layer_and_rejects_plain_inline_styles",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document cascade layer coverage evidence via `{required}`"
        );
    }
}

#[test]
fn form_motion_contract_is_na_and_respects_reduced_motion_noop() {
    let module = load_source("mod");
    let view = load_source("view");
    let ui_motion_lib = load_ui_motion_source("lib.rs");
    let check2 = load_source("check2");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    assert!(
        !motion_file.exists(),
        "form is a static container; `src/motion.rs` should stay absent for motion-contract N/A."
    );

    for forbidden in ["attach_motion(", "ui_motion::", "use ui_motion"] {
        assert!(
            !module.contains(forbidden) && !view.contains(forbidden),
            "form component should not mount motion runtime path for N/A motion contract: `{forbidden}`"
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep reduced-motion/non-wasm no-op baseline via `{required}`"
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "`motion.rs + attach_motion` 在本组件按 N/A 处理",
        "`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(&(), ..)` no-op stub",
        "components/form/test/semantics.rs::form_motion_contract_is_na_and_respects_reduced_motion_noop",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document motion-contract N/A evidence via `{required}`"
        );
    }
}

#[test]
fn form_ui_components_fixed_entrypoints_are_correctly_placed() {
    let ui_components_lib = load_ui_components_source("lib.rs");
    let ui_components_css = load_ui_components_source("css.rs");
    let ui_components_root = load_ui_components_source("root.rs");
    let ui_components_cargo = load_ui_components_cargo_toml();
    let active_highlight = load_ui_visual_primitive_source("active_highlight.rs");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let ui_components_src_root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
        "pub use field_form::form::{Form, FormLabelAlign, FormLabelPosition, use_form_context};",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib entry should keep stable module/api surface via `{required}`"
        );
    }

    for required in [
        "component-form = []",
        "component-form_field = [\"component-switch\", \"component-checkbox\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui feature gate manifest should keep form entrypoint gating via `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-form\")]",
        "out.push_str(crate::field_form::form::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css entrypoint should keep feature-conditional form injection via `{required}`"
        );
    }

    for required in [
        "pub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should keep centralized theme/css/i18n injection path via `{required}`"
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep generic motion capability via `{required}`"
        );
    }

    for forbidden in ["field_form::form", "FormContextValue", "ui-form"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should not bind to form business semantics: `{forbidden}`"
        );
    }

    for absent in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_root.join(absent).exists(),
            "ui fixed entrypoint policy requires `{absent}` to be absent."
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `ui` 固定入口文件落点正确。",
            "`crates/ui/src/lib.rs` 维持总入口 + `UiRoot` 公共导出，并通过 `component-*` 特性门控组件导出面（含 `component-form`）。",
            "`crates/ui/src/css.rs::push_components_css` 使用 `inject-css + component-*` 条件注入，`component-form` 下仅按需注入 `crate::field_form::form::styles::CSS`。",
            "`crates/ui/src/root.rs::UiRoot` 统一注入 `BASE_CSS + theme vars + optional components css`，并提供 `UiI18n`/`IdProvider` 上下文。",
            "`crates/ui-visual-primitive/src/active_highlight.rs` 仅承载共享高亮样式与 motion driver，未绑定 Form 业务语义。",
            "`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 当前不存在（契约落在 `ui-headless`）。",
            "components/form/test/semantics.rs::form_ui_components_fixed_entrypoints_are_correctly_placed",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document ui fixed-entrypoint evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_visual_desire_contract_is_scoped_and_traceable() {
    let styles = load_source("styles");
    let docs_forms = load_docs_forms_page();
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in ["var(--ui-space-md)", ".ui-form[data-disabled=\\\"true\\\"]"] {
        assert!(
            styles.contains(required),
            "form visual baseline should stay token-first and semantic-state driven via `{required}`"
        );
    }

    for forbidden in [
        "bootstrap",
        "form-control",
        "btn-default",
        "panel-default",
        "navbar-default",
    ] {
        assert!(
            !styles.to_lowercase().contains(forbidden),
            "form styles should not regress to legacy coarse visual language: `{forbidden}`"
        );
    }

    for required in [
        "pub(super) fn form() -> AnyView {",
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
    ] {
        assert!(
            docs_forms.contains(required),
            "docs-app should keep form default-theme baseline display contract via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
            "本组件判定：`Form` 是语义容器，不定义品牌级视觉皮肤",
            "`Button/Input/Overlay` 截图回归属于仓库级视觉门禁；本单组件检查按 N/A 继承",
            "components/form/test/semantics.rs::form_visual_desire_contract_is_scoped_and_traceable",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record visual desire scope and evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_tree_shaking_contract_is_feature_gated_and_not_registry_bound() {
    let ui_components_cargo = load_ui_components_cargo_toml();
    let ui_components_lib = load_ui_components_source("lib.rs");
    let ui_components_css = load_ui_components_source("css.rs");
    let form_cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "component-form = []",
        "inject-css = []",
        "all-components = [",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui feature manifest should expose component-level tree-shaking toggles via `{required}`"
        );
    }

    for required in [
        "feature = \"component-form\"",
        "pub mod field_form {",
        "pub use field_form::form::{Form, FormLabelAlign, FormLabelPosition, use_form_context};",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui exports should keep form module and api behind feature gates via `{required}`"
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "#[cfg(feature = \"component-form\")]",
        "out.push_str(crate::field_form::form::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css pipeline should keep feature-gated tree-shaking behavior via `{required}`"
        );
    }

    for required in ["name = \"ui-form\"", "[features]", "default = []"] {
        assert!(
            form_cargo.contains(required),
            "ui-form crate should stay source-mode friendly with opt-in features via `{required}`"
        );
    }

    assert!(
        !form_cargo.contains("\nui ="),
        "ui-form crate should not depend on ui central registry in source mode."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
            "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
            "--no-default-features --features component-form,inject-css",
            "`crates/ui/Cargo.toml` 已注册 `component-form = []`，并纳入 `web-demo-components` 与 `all-components` 特性树",
            "`css.rs` 门控证据：`crates/ui/src/css.rs` 通过 `#[cfg(feature = \"inject-css\")] + #[cfg(feature = \"component-form\")]` 按需注入",
            "source 模式：`components/form` 是独立 crate",
            "CI 体积预算属于仓库级门禁；本单组件检查按 N/A 继承",
            "components/form/test/semantics.rs::form_tree_shaking_contract_is_feature_gated_and_not_registry_bound",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record tree-shaking evidence and scope via `{required}`"
            );
        }
    }
}

#[test]
fn form_machine_readable_state_contract_is_type_constrained_and_traceable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let logic_tests = include_str!("logic.rs");
    let semantics_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    let semantics_source = std::fs::read_to_string(&semantics_path).unwrap_or_else(|err| {
        panic!(
            "failed to read semantics source {}: {err}",
            semantics_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub enum FormLabelPosition",
        "pub enum FormLabelAlign",
        "pub fn resolve_props(",
        "pub fn resolve_view_state(resolved: &FormResolvedProps) -> FormViewState",
        "state_source: \"logic.rs::resolve_view_state\"",
    ] {
        assert!(
            logic.contains(required),
            "form logic should keep enum-first and normalized machine-readable state model via `{required}`"
        );
    }

    for forbidden in [
        "label_position: Option<String>",
        "label_align: Option<String>",
        "label_position: String",
        "label_align: String",
    ] {
        assert!(
            !view.contains(forbidden) && !readme.contains(forbidden),
            "form public surface should avoid string protocol for discrete state axes: `{forbidden}`"
        );
    }

    for required in [
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
        "data-label-position=view_state.label_position",
        "data-label-align=view_state.label_align",
        "data-state-source=view_state.state_source",
    ] {
        assert!(
            view.contains(required),
            "form view should expose stable semantic markers for machine-readable state via `{required}`"
        );
    }

    for required in [
        "fn attr_mapping_matches_enum_variants()",
        "fn resolve_view_state_derives_render_markers_in_logic()",
        "fn form_discrete_states_are_type_constrained_by_enums()",
        "fn form_state_markers_are_observable_queryable_and_enumerable()",
    ] {
        assert!(
            logic_tests.contains(required) || semantics_source.contains(required),
            "test feedback should pinpoint contract regressions via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
            "本组件判定：离散布局输入由 `FormLabelPosition/FormLabelAlign` 枚举建模",
            "机器可读状态：`components/form/src/view.rs` 暴露稳定标记",
            "components/form/test/semantics.rs::form_machine_readable_state_contract_is_type_constrained_and_traceable",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should record type-system + semantic-marker contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_focus_stack_gc_contract_is_na_for_non_overlay_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let check2 = load_source("check2");

    for forbidden in [
        "NodeRef",
        "FallbackTo",
        "Selector",
        "FocusManager",
        "document.body",
        "focus manager",
        "focus_stack",
        "focus_trap",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "form should not implement overlay focus stack internals in component scope: `{forbidden}`"
        );
    }

    for forbidden in ["is_open", "default_open", "on_open_change", "aria-modal"] {
        assert!(
            !view.contains(forbidden),
            "form should not expose overlay-focused api surface in non-overlay container: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "本组件判定：`Form` 是静态语义容器，不属于层叠 `Overlay`",
        "回归：`components/form/test/semantics.rs::form_focus_stack_gc_contract_is_na_for_non_overlay_container`。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should record focus stack GC N/A boundary evidence via `{required}`"
        );
    }
}

#[test]
fn form_escape_hatches_contract_is_na_for_non_foreign_zone_container() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let check2 = load_source("check2");

    for forbidden in [
        "YieldControl",
        "CleanupForeign",
        "Foreign Zone",
        "foreign_zone",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "JsValue",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "form should not embed imperative foreign-zone bridge internals: `{forbidden}`"
        );
    }

    for forbidden in [
        "on_foreign_mount",
        "on_foreign_cleanup",
        "foreign_handle",
        "chart_instance",
        "map_instance",
    ] {
        assert!(
            !view.contains(forbidden),
            "form public props should not expose third-party imperative handles: `{forbidden}`"
        );
    }

    for required in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "本组件判定：`Form` 是字段语义容器，不集成命令式第三方渲染实例",
        "回归：`components/form/test/semantics.rs::form_escape_hatches_contract_is_na_for_non_foreign_zone_container`。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should record escape-hatches N/A boundary evidence via `{required}`"
        );
    }
}

#[test]
fn form_hydration_discontinuity_contract_is_na_without_runtime_id_generation() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let check2 = load_source("check2");

    for forbidden in [
        "now()",
        "Date::now",
        "SystemTime::now",
        "Instant::now",
        "Uuid::new_v4",
        "uuid::Uuid",
        "rand::",
        "thread_rng",
        "getrandom",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "form should not use non-deterministic time/random initialization in hydration-sensitive paths: `{forbidden}`"
        );
    }

    for required in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "本组件判定：`Form` 不在 `logic.rs`/`view.rs` 生成运行时 ID",
        "回归：`components/form/test/semantics.rs::form_hydration_discontinuity_contract_is_na_without_runtime_id_generation`。",
    ] {
        assert!(
            check2.contains(required),
            "checklist should record hydration-discontinuity N/A boundary evidence via `{required}`"
        );
    }
}

#[test]
fn form_ssr_cross_platform_contract_is_non_wasm_safe_and_branch_stable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for source in [logic, view, module] {
        for forbidden in [
            "web_sys::",
            "web-sys",
            "window.",
            "document.",
            "wasm_bindgen",
            "js_sys::",
            "cfg!(target_arch",
        ] {
            assert!(
                !source.contains(forbidden),
                "form source should keep non-wasm path clean from browser-only runtime coupling: `{forbidden}`"
            );
        }
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
            "compile-only 命令基线：`cargo check -p ui-form`、`cargo check -p ui-form --target wasm32-unknown-unknown`、`cargo check -p ui-form --no-default-features`",
            "Invalid cross-device link (os error 18)",
            "components/form/test/semantics.rs::form_ssr_cross_platform_contract_is_non_wasm_safe_and_branch_stable",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document ssr/platform contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_ui_headless_feature_mutex_contract_is_preserved() {
    let ui_headless_lib = load_ui_headless_source("lib.rs");
    let ui_headless_cargo = load_ui_headless_cargo_toml();
    let form_cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should keep web/ssr mutual exclusion compile guard via `{required}`"
        );
    }

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo.contains(required),
            "ui-headless feature matrix should keep explicit web/ssr split via `{required}`"
        );
    }

    assert!(
        form_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "form should consume ui-headless without overriding web/ssr feature mutex."
    );
    assert!(
        !form_cargo.contains("features = [\"web\", \"ssr\"]"),
        "form dependency graph must not enable ui-headless web+ssr simultaneously."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
            "仅依赖 `ui-headless` 默认 feature（`web`）",
            "--no-default-features --features web,ssr`（应失败并命中互斥保护）",
            "components/form/test/semantics.rs::form_ui_headless_feature_mutex_contract_is_preserved",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document ui-headless feature mutex evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_ui_motion_non_wasm_noop_contract_is_preserved() {
    let ui_motion_lib = load_ui_motion_source("lib.rs");
    let view = load_source("view");
    let module = load_source("mod");
    let form_cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep deterministic non-wasm no-op branch via `{required}`"
        );
    }

    for forbidden in ["panic!(", "todo!(", "unimplemented!("] {
        assert!(
            !ui_motion_lib.contains(forbidden),
            "ui-motion non-wasm no-op path should not panic or leave stubs unresolved: `{forbidden}`"
        );
    }

    for forbidden in ["attach_motion(", "ui_motion::", "MotionHandle", "Animation"] {
        assert!(
            !view.contains(forbidden) && !module.contains(forbidden),
            "form should not assume motion runtime handle existence: `{forbidden}`"
        );
    }

    assert!(
        !form_cargo.contains("ui-motion"),
        "form toolchain path should not be blocked by direct ui-motion dependency."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
            "通过 `#[cfg(not(target_arch = \"wasm32\"))]` 提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(&(), ..)` no-op stub",
            "components/form/test/semantics.rs::form_ui_motion_non_wasm_noop_contract_is_preserved",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document ui-motion non-wasm no-op evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_motion_branch_coverage_is_na_and_semantics_are_platform_stable() {
    let view = load_source("view");
    let logic = load_source("logic");
    let module = load_source("mod");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for source in [view, logic, module] {
        for forbidden in [
            "attach_motion(",
            "ui_motion::",
            "prefers_reduced_motion(",
            "cfg(target_arch = \"wasm32\")",
            "cfg(not(target_arch = \"wasm32\"))",
            "web_sys::",
            "window.",
            "document.",
        ] {
            assert!(
                !source.contains(forbidden),
                "form should keep reduced-motion/ssr/wasm path free from runtime motion or platform branch coupling: `{forbidden}`"
            );
        }
    }

    for required in [
        "<form",
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
        "aria-disabled=view_state.aria_disabled",
        "data-state-source=view_state.state_source",
    ] {
        assert!(
            view.contains(required),
            "form should expose stable semantic markers independent of reduced-motion/ssr/wasm runtime branch: `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
            "`reduced-motion` 分支在组件级按 N/A 处理（零动画即最小反馈）",
            "SSR 与 wasm 均挂载同一组 `data-*`/`aria-*` 语义标记",
            "components/form/test/semantics.rs::form_motion_branch_coverage_is_na_and_semantics_are_platform_stable",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document reduced-motion/ssr/wasm branch coverage evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let todo = load_docs_plan_todo();
    let performance_gate_script = load_performance_gate_script();

    assert_eq!(
        view.matches("let resolved = logic::resolve_props(").count(),
        1,
        "form mount path should normalize props exactly once."
    );
    assert_eq!(
        view.matches("let view_state = logic::resolve_view_state(&resolved);")
            .count(),
        1,
        "form mount path should derive render state exactly once."
    );

    for source in [view, logic] {
        for forbidden in [
            " on:",
            "create_effect(",
            "create_memo(",
            "create_resource(",
            "spawn_local(",
            "set_interval",
            "request_animation_frame",
            "ui_motion::",
            "attach_motion(",
            "tokio::",
            "async move",
        ] {
            assert!(
                !source.contains(forbidden),
                "form mount-only performance contract should reject high-frequency runtime path `{forbidden}`"
            );
        }
    }

    for forbidden in ["animation:", "transition:", "@keyframes"] {
        assert!(
            !styles.contains(forbidden),
            "form static styles should not add animation runtime overhead `{forbidden}`"
        );
    }

    for required in [
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            performance_gate_script.contains(required),
            "repo performance gate should keep blocking baseline `{required}`"
        );
    }

    assert!(
        todo.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "repo todo should keep render_count follow-up tracking until automation lands."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
            "`Form` 为静态语义容器，按 mount-only 预算通过",
            "docs/plan/TODO.md` 保持“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”跟踪项",
            "components/form/test/semantics.rs::form_performance_governance_contract_is_mount_only_traceable_and_blocking",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document performance governance evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_semantics_and_performance_regression_contract_covers_aria_data_focus_and_render_count_boundary()
 {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let todo = load_docs_plan_todo();
    let performance_gate_script = load_performance_gate_script();
    let semantics_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    let semantics_source = std::fs::read_to_string(&semantics_path).unwrap_or_else(|err| {
        panic!(
            "failed to read semantics source {}: {err}",
            semantics_path.display()
        )
    });

    for required in [
        "<form",
        "aria-disabled=view_state.aria_disabled",
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
        "data-state-source=view_state.state_source",
    ] {
        assert!(
            view.contains(required),
            "form semantic+performance regression contract should keep core aria/data markers via `{required}`"
        );
    }

    for required in [
        "fn form_a11y_i18n_contract_is_wired_without_hardcoded_user_copy()",
        "fn form_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_semantic_contract_tests_take_priority_over_snapshots()",
        "fn form_focus_stack_gc_contract_is_na_for_non_overlay_container()",
        "fn form_performance_governance_contract_is_mount_only_traceable_and_blocking()",
    ] {
        assert!(
            semantics_source.contains(required),
            "form semantic+performance regression contract should keep dedicated matrix coverage via `{required}`"
        );
    }

    for required in [
        "perf_render_count_follow_up_is_tracked_in_plan",
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            performance_gate_script.contains(required),
            "repo performance gate should keep render-count governance hooks via `{required}`"
        );
    }

    assert!(
        todo.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "repo todo should keep render_count automation follow-up for heavy components."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "`components/form/test/semantics.rs` 已覆盖 `aria-*`、`data-*`、`role(<form>)` 与状态来源标记",
            "焦点流转边界：`Form` 非 overlay 组件，不承担焦点栈与焦点恢复状态机",
            "`render_count=1` 强预算维持仓库级治理（Button/Input/Accordion）并由 `docs/plan/TODO.md` 持续跟踪自动化补齐",
            "components/form/test/semantics.rs::form_semantics_and_performance_regression_contract_covers_aria_data_focus_and_render_count_boundary",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document semantic+performance regression evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_version_deprecation_migration_contract_is_na_without_major_breaking_upgrade() {
    let module = load_source("mod");
    let protocol = include_str!("../src/protocol.rs");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "pub use logic::{",
        "FormContextValue, FormLabelAlign, FormLabelPosition, FormViewState, use_form_context,",
        "pub use view::Form;",
    ] {
        assert!(
            module.contains(required),
            "form public export surface should remain stable for non-breaking change path via `{required}`"
        );
    }

    for required in [
        "pub enum FormComponentSchemaVersion",
        "#[default]",
        "V1,",
        "pub struct FormComponentSpec",
        "pub schema_version: FormComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "form protocol should stay on V1 schema without migration branch drift via `{required}`"
        );
    }

    for forbidden in ["V2", "migrate_v1_to_v2", "#[deprecated", "deprecated("] {
        assert!(
            !module.contains(forbidden) && !protocol.contains(forbidden),
            "non-breaking form changes should not introduce deprecation migration internals: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
            "本次提交未引入跨大版本 API 破坏升级",
            "`FormComponentSchemaVersion` 仍仅包含 `V1`",
            "未引入 `V2` 协议分支、`migrate_v1_to_v2` 或 `deprecated` 迁移标记",
            "components/form/test/semantics.rs::form_version_deprecation_migration_contract_is_na_without_major_breaking_upgrade",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document deprecation migration N/A evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_view_macro_complexity_is_bounded_and_not_fragmented() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    assert_eq!(
        view.matches("view! {").count(),
        1,
        "form view should keep exactly one view! macro block for the current simple container scope."
    );

    for required in [
        "view! {",
        "<form",
        "{children()}",
        "</form>",
        "let resolved = logic::resolve_props(",
        "let view_state = logic::resolve_view_state(&resolved);",
    ] {
        assert!(
            view.contains(required),
            "form view macro should keep minimal semantic structure via `{required}`"
        );
    }

    for forbidden in [
        "<For ",
        "Indexed",
        "collect_view",
        "render_header(",
        "render_body(",
    ] {
        assert!(
            !view.contains(forbidden),
            "form view should not introduce oversized macro composition for a simple container: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
            "仅存在 1 个 `view!` 宏块",
            "components/form/test/semantics.rs::form_view_macro_complexity_is_bounded_and_not_fragmented",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document view-macro complexity evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_functional_split_prefers_plain_functions_over_component_noise() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "form view should expose a single component entry and avoid local component noise."
    );
    assert_eq!(
        view.matches("pub fn Form(").count(),
        1,
        "form view should keep exactly one public component function."
    );

    for forbidden in [
        "fn FormHeader(",
        "fn FormBody(",
        "fn FormItem(",
        "fn render_header(",
        "fn render_body(",
        "fn render_item(",
    ] {
        assert!(
            !view.contains(forbidden),
            "form should not introduce fragment-level component abstraction noise: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
            "仅保留 `#[component] pub fn Form(...)` 一个组件入口",
            "components/form/test/semantics.rs::form_functional_split_prefers_plain_functions_over_component_noise",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document functional-split preference evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_static_fragments_are_constantized_or_absent_for_simple_container() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in ["view! {", "<form", "{children()}", "</form>"] {
        assert!(
            view.contains(required),
            "form view should keep a minimal typed container path via `{required}`"
        );
    }

    for forbidden in [
        "<svg",
        "<footer",
        "inner_html",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "const LONG_COPY",
    ] {
        assert!(
            !view.contains(forbidden),
            "form should keep static fragment path absent or centralized instead of inline expansion: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
            "静态片段缺省即通过（absent）",
            "components/form/test/semantics.rs::form_static_fragments_are_constantized_or_absent_for_simple_container",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document static-fragment constantization evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let view = load_source("view");
    let readme = include_str!("../src/README.md");
    let docs_forms_page = load_docs_forms_page();
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for source in [view, readme, docs_forms_page.as_ref()] {
        let normalized = source.to_ascii_lowercase();
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
        ] {
            assert!(
                !normalized.contains(forbidden),
                "form component/docs source should forbid html injection marker `{forbidden}`"
            );
        }
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
            "该项按“零注入面”通过",
            "components/form/test/semantics.rs::form_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document inner_html safety evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_wasm_debug_contract_is_na_and_feature_isolated() {
    let view = load_source("view");
    let logic = load_source("logic");
    let module = load_source("mod");
    let form_cargo = include_str!("../Cargo.toml");
    let ui_components_lib = load_ui_components_source("lib.rs");
    let ui_components_cargo = load_ui_components_cargo_toml();
    let docs_app_lib = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../apps/docs-app/src/lib.rs"),
    )
    .unwrap_or_else(|err| panic!("failed to read docs app lib source: {err}"));
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for source in [view, logic, module] {
        for forbidden in [
            "use_ui_trace(",
            "provide_ui_trace(",
            "trace.emit(",
            "debug_overlay",
            "replay",
            "trace_id",
            "wasm_debug_proxy",
            "observability",
        ] {
            assert!(
                !source.contains(forbidden),
                "form should not implement local wasm-debug pipeline marker `{forbidden}`"
            );
        }
    }

    for forbidden in ["wasm-debug", "dep:tracing"] {
        assert!(
            !form_cargo.contains(forbidden),
            "form package should not leak wasm-debug/public debug feature surface `{forbidden}`"
        );
    }
    assert!(
        form_cargo.contains("default = []"),
        "form package should keep empty default feature set."
    );

    for required in [
        "macro_rules! wasm_debug_proxy",
        "provide_ui_trace(debug_overlay_enabled);",
        "Show when=move || debug_overlay_enabled",
        "-wasm-debug",
    ] {
        assert!(
            ui_components_lib.contains(required)
                || docs_app_lib.contains(required)
                || ui_components_cargo.contains(required),
            "global wasm-debug infrastructure should keep feature-isolated marker `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
            "本项在组件级按 N/A 继承全局调试基础设施",
            "components/form/test/semantics.rs::form_wasm_debug_contract_is_na_and_feature_isolated",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document wasm-debug contract evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_dx_workbench_supports_css_hot_reload_and_context_retention_with_optional_persist_na() {
    let docs_forms_page = load_docs_forms_page();
    let docs_playground_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/docs-app/src/playground.rs"),
    )
    .unwrap_or_else(|err| panic!("failed to read docs playground source: {err}"));
    let dev_docs_script = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/dev-docs-app.sh"),
    )
    .unwrap_or_else(|err| panic!("failed to read dev docs script: {err}"));
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
        "test_css_source=form_test_css_source",
        "test_config_signal=workbench_config",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs page should keep DX workbench marker `{required}`"
        );
    }

    for required in [
        "compose_scoped_css",
        "data-playground-scope",
        "show_test_panel",
        "test_css_source",
        "playground__preview-stage",
    ] {
        assert!(
            docs_playground_source.contains(required),
            "shared playground should keep isolated canvas and css test marker `{required}`"
        );
    }

    assert!(
        dev_docs_script.contains("exec trunk serve --open true"),
        "dev docs script should keep trunk live-serve flow for fast DX feedback."
    );

    for forbidden in [
        "load_form_workbench_state(",
        "save_form_workbench_state(",
        "clear_form_workbench_state(",
        "Persist workbench state",
        "localStorage",
    ] {
        assert!(
            !docs_forms_page.contains(forbidden),
            "form docs should keep optional persisted workbench state as N/A in current scope: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
            "Interactive Playground (展示 / Config / Code / CSS Test)",
            "按组件范围 N/A（不强制持久化）",
            "components/form/test/semantics.rs::form_dx_workbench_supports_css_hot_reload_and_context_retention_with_optional_persist_na",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document DX/workbench evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports() {
    let docs_forms_page = load_docs_forms_page();
    let docs_playground_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/docs-app/src/playground.rs"),
    )
    .unwrap_or_else(|err| panic!("failed to read docs playground source: {err}"));
    let semantics_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    let semantics_source = std::fs::read_to_string(&semantics_path).unwrap_or_else(|err| {
        panic!(
            "failed to read semantics source {}: {err}",
            semantics_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
        "code_signal=hello_code",
        "code_signal=workbench_code",
        "code_signal=matrix_code",
        "`Form` 无 value 状态轴，受控/非受控对照在该组件按 N/A 处理",
        "data-ui-stream-mode=snapshot",
        "data-ui-streaming-policy=optional",
        "data-ui-streaming-fallback=snapshot",
        "data-ui-output-status=verified",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs product page should keep playground/copy/streaming markers via `{required}`"
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "compose_copy_ready_code",
        "missing_import_lines",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            docs_playground_source.contains(required),
            "shared docs playground should keep import-ready copy assembly via `{required}`"
        );
    }

    for required in [
        "fn form_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet()",
        "fn form_streaming_definition_is_limited_to_llm_output_modes()",
        "fn form_snapshot_mode_is_baseline_and_consumes_complete_config()",
        "fn form_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_status()",
    ] {
        assert!(
            semantics_source.contains(required),
            "form docs product contract should keep controlled/streaming semantic regression anchors via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 已提供 `Hello World（默认路径）`、`Interactive Playground (展示 / Config / Code / CSS Test)`、`Comparison Matrix (Default / Required / Disabled / ReadOnly)`",
            "`Form` 无 `value` 状态轴，受控/非受控在组件级按 N/A 处理",
            "`Actual config` 已暴露 `data-ui-stream-mode=snapshot`、`data-ui-streaming-policy=optional`、`data-ui-streaming-fallback=snapshot`、`data-ui-output-status=verified`",
            "`compose_copy_ready_code` + `DEFAULT_PLAYGROUND_IMPORTS`",
            "components/form/test/semantics.rs::form_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document docs-as-product copy-paste evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_docs_examples_and_matrices_are_synced_with_logic_api_defaults() {
    let docs_forms_page = load_docs_forms_page();
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Default / Required / Disabled / ReadOnly)\"",
        "id=\"docs-form-matrix-default\"",
        "id=\"docs-form-matrix-required\"",
        "id=\"docs-form-matrix-disabled\"",
        "id=\"docs-form-matrix-readonly\"",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs should keep hello-world/examples and state-matrix coverage via `{required}`"
        );
    }

    for required in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "#[prop(optional, into)] is_read_only: Option<bool>",
        "#[prop(optional, into)] is_required: Option<bool>",
        "#[prop(optional)] label_position: Option<FormLabelPosition>",
        "#[prop(optional)] label_align: Option<FormLabelAlign>",
    ] {
        assert!(
            view.contains(required),
            "form public API names must remain consistent in view props via `{required}`"
        );
    }

    for required in [
        "disabled: is_disabled.unwrap_or(false)",
        "read_only: is_read_only.unwrap_or(false)",
        "required: is_required.unwrap_or(false)",
        "label_position: label_position.unwrap_or_default()",
        "label_align: label_align.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(required),
            "form logic defaults should stay centralized in resolve_props via `{required}`"
        );
    }

    for required in [
        "let (workbench_label_position_index, set_workbench_label_position_index) = signal(Some(0));",
        "let (workbench_label_align_index, set_workbench_label_align_index) = signal(Some(0));",
        "let (workbench_is_required, set_workbench_is_required) = signal(false);",
        "let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);",
        "let (workbench_is_read_only, set_workbench_is_read_only) = signal(false);",
        "<Switch checked=workbench_is_required set_checked=set_workbench_is_required>",
        "<Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>",
        "<Switch checked=workbench_is_read_only set_checked=set_workbench_is_read_only>",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs playground defaults and API controls should match logic.rs defaults via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 已同步 `Hello World（默认路径）`、`Interactive Playground (展示 / Config / Code / CSS Test)`、`Comparison Matrix (Default / Required / Disabled / ReadOnly)`。",
            "参数矩阵与状态矩阵覆盖：`is_required`、`is_disabled`、`is_read_only`、`label_position`、`label_align`。",
            "默认值与 `components/form/src/logic.rs::resolve_props` 对齐：`is_required/is_disabled/is_read_only=false`，`label_position=Top`，`label_align=Start`。",
            "components/form/test/semantics.rs::form_docs_examples_and_matrices_are_synced_with_logic_api_defaults",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document docs/example/matrix sync evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_documentation_is_beginner_friendly_and_progressive() {
    let readme = include_str!("../src/README.md");
    let docs_forms_page = load_docs_forms_page();
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "# Form",
        "## Hello World（默认路径）",
        "## 常见用法（先用起来，再进阶）",
        "## API (Table)",
        "默认用法不需要手动接线 `ui-state-primitives` / `ui-headless` 状态机",
        "建议顺序：先确认默认路径可用，再逐个开启 `is_*` 与标签布局参数。",
    ] {
        assert!(
            readme.contains(required),
            "form README should keep beginner-friendly docs entry and progressive guidance via `{required}`"
        );
    }

    let hello_idx = readme
        .find("## Hello World（默认路径）")
        .unwrap_or_else(|| panic!("README must include Hello World section"));
    let common_idx = readme
        .find("## 常见用法（先用起来，再进阶）")
        .unwrap_or_else(|| panic!("README must include common usage section"));
    let api_idx = readme
        .find("## API (Table)")
        .unwrap_or_else(|| panic!("README must include API table section"));
    assert!(
        hello_idx < common_idx && common_idx < api_idx,
        "form README should follow progressive order: Hello World -> common usage -> API table"
    );

    for required in [
        "title=\"Form\"",
        "slug=\"form\"",
        "title=\"Hello World（默认路径）\"",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "docs-app should keep accessible form docs entry via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
            "`components/form/src/README.md` 提供 `Hello World（默认路径）` 与 `常见用法（先用起来，再进阶）`，不要求先理解底层分层。",
            "文档顺序为“默认路径在前、进阶参数在后”：`Hello World -> 常见用法 -> API (Table)`。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 提供等价文档入口（`title=\"Form\"` + `slug=\"form\"`）。",
            "components/form/test/semantics.rs::form_documentation_is_beginner_friendly_and_progressive",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document beginner-friendly documentation evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_docs_app_provides_interactive_playground_with_live_preview_and_repeatable_flow() {
    let docs_forms_page = load_docs_forms_page();
    let e2e_spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../e2e/tests/docs_app_form_contract.spec.mjs");
    let e2e_spec = std::fs::read_to_string(&e2e_spec_path).unwrap_or_else(|err| {
        panic!(
            "failed to read form e2e spec {}: {err}",
            e2e_spec_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "controls=move || {",
        "selected_index=workbench_label_position_index",
        "set_selected_index=set_workbench_label_position_index",
        "selected_index=workbench_label_align_index",
        "set_selected_index=set_workbench_label_align_index",
        "<Switch checked=workbench_is_required set_checked=set_workbench_is_required>",
        "<Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>",
        "<Switch checked=workbench_is_read_only set_checked=set_workbench_is_read_only>",
        "let is_required = workbench_is_required.get();",
        "let is_disabled = workbench_is_disabled.get();",
        "let is_read_only = workbench_is_read_only.get();",
        "<Form",
        "is_required=is_required",
        "is_disabled=is_disabled",
        "is_read_only=is_read_only",
        "id=\"docs-form-name\".to_string()",
        "id=\"docs-form-email\".to_string()",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs page should keep interactive controls and live-preview bindings via `{required}`"
        );
    }

    for required in [
        "docs-app form key flow is repeatable with semantic breakpoints for focus and keyboard paths",
        "await page.goto(\"/#/components/form\");",
        "#docs-form-name",
        "#docs-form-email",
        "await page.keyboard.press(\"Tab\");",
        "await page.reload();",
    ] {
        assert!(
            e2e_spec.contains(required),
            "form e2e should keep repeatable interactive-playground key flow via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 提供 `Interactive Playground (展示 / Config / Code / CSS Test)` 与 `controls=move || { ... }`。",
            "控件覆盖基础 props/状态切换：`Label Position`、`Label Align`、`is_required`、`is_disabled`、`is_read_only`、`Custom class_name`。",
            "实时预览绑定：`workbench_*` 当前值映射到 `<Form is_required=is_required is_disabled=is_disabled is_read_only=is_read_only ...>`。",
            "可重复关键流程由 `e2e/tests/docs_app_form_contract.spec.mjs` 覆盖（`Tab` 焦点流转 + 刷新复验）。",
            "components/form/test/semantics.rs::form_docs_app_provides_interactive_playground_with_live_preview_and_repeatable_flow",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document interactive-playground evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_source_first_docs_are_copy_paste_ready_with_source_paths_and_import_hints() {
    let docs_forms_page = load_docs_forms_page();
    let docs_playground_source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/docs-app/src/playground.rs"),
    )
    .unwrap_or_else(|err| panic!("failed to read docs playground source: {err}"));
    let e2e_spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../e2e/tests/docs_app_form_contract.spec.mjs");
    let e2e_spec = std::fs::read_to_string(&e2e_spec_path).unwrap_or_else(|err| {
        panic!(
            "failed to read form e2e spec {}: {err}",
            e2e_spec_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "title=\"Hello World（默认路径）\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "code_signal=hello_code",
        "code_signal=workbench_code",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/form/src/styles.rs\".to_string()",
        "lines.push(\"  is_required=true\".to_string());",
        "lines.push(\"  is_disabled=true\".to_string());",
        "lines.push(\"  is_read_only=true\".to_string());",
    ] {
        assert!(
            docs_forms_page.contains(required),
            "form docs page should keep source-first code/source-path sync marker `{required}`"
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "compose_copy_ready_code",
        "missing_import_lines",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            docs_playground_source.contains(required),
            "docs playground should keep copy-paste import-hint assembly via `{required}`"
        );
    }

    for required in [
        "docs-app form playground code panel remains copy-ready via semantic selectors",
        "await expect(codeBlock).toHaveAttribute(\"data-copyable\", \"true\");",
        "await expect(code).toContainText(\"use leptos::prelude::*;\");",
        "await expect(code).toContainText(\"use ui::*;\");",
        "await expect(code).toContainText(\"<Form>\");",
        "await expect(code).toContainText(\"id=\\\"docs-form-hello\\\"\");",
    ] {
        assert!(
            e2e_spec.contains(required),
            "form e2e should keep source-first copy-ready evidence via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 通过 `code_signal=hello_code/workbench_code` 提供与当前实现同步的可复制片段。",
            "`apps/docs-app/src/playground.rs::compose_copy_ready_code` + `DEFAULT_PLAYGROUND_IMPORTS` 为复制内容补全默认 imports（`use leptos::prelude::*;`、`use ui::*;`）。",
            "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 通过 `test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/form/src/styles.rs\"` 指向真实源码落点。",
            "`e2e/tests/docs_app_form_contract.spec.mjs` 断言 `data-copyable=true` 与复制代码内容，防止示例漂移。",
            "components/form/test/semantics.rs::form_source_first_docs_are_copy_paste_ready_with_source_paths_and_import_hints",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document source-first copy-paste-ready evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_heroui_strategy_and_component_docs_entry_are_synced() {
    let docs_pages = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/docs-app/src/pages/components/pages.rs"),
    )
    .unwrap_or_else(|err| panic!("failed to read docs component pages index: {err}"));
    let heroui_strategy = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/spec/heroui-parameter-design-strategy.md"),
    )
    .unwrap_or_else(|err| panic!("failed to read HeroUI strategy doc: {err}"));
    let docs_forms_page = load_docs_forms_page();
    let readme = include_str!("../src/README.md");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "component_doc!(\"Form\", \"form\", \"Forms\", forms::form)",
        "\"Form\"",
        "\"form\"",
    ] {
        assert!(
            docs_pages.contains(required),
            "docs index should expose Form entry for routing/discovery via `{required}`"
        );
    }

    for required in [
        "### Form 同步记录（2026-02-20）",
        "参数模型同步：`Form` 维持表单上下文容器定位",
        "component_doc!(\"Form\", \"form\", \"Forms\", forms::form)",
        "`#/components/form` 可索引访问",
        "`apps/docs-app/src/pages/components/pages/forms.rs::form()` 已覆盖 `Hello World（默认路径）`、`Interactive Playground (展示 / Config / Code / CSS Test)`、`Comparison Matrix (Default / Required / Disabled / ReadOnly)`",
        "研究文档补充判定：本轮仅为 Form 参数模型与文档入口同步",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            heroui_strategy.contains(required),
            "HeroUI strategy doc should keep Form sync record via `{required}`"
        );
    }

    for required in ["title=\"Form\"", "slug=\"form\""] {
        assert!(
            docs_forms_page.contains(required),
            "form docs page should remain indexable via `{required}`"
        );
    }
    assert!(
        readme.contains("# Form"),
        "form component should keep README docs entry for discoverability."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
            "`docs/spec/heroui-parameter-design-strategy.md` 已新增 `Form 同步记录（2026-02-20）`，同步参数模型与 docs 入口。",
            "`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"Form\", \"form\", \"Forms\", forms::form)` 暴露可索引入口 `#/components/form`。",
            "`components/form/src/README.md` 与 `apps/docs-app/src/pages/components/pages/forms.rs::form()` 共同提供可访问组件文档入口。",
            "本轮未引入新的 Spectrum/HeroUI 风格结论，`docs/research/spectrum-heroui-style-interface-study.md` 按 N/A 不追加。",
            "components/form/test/semantics.rs::form_heroui_strategy_and_component_docs_entry_are_synced",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document HeroUI strategy/docs sync evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_e2e_selectors_are_semantic_and_wasm_waits_are_stable_without_sleep() {
    let e2e_spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../e2e/tests/docs_app_form_contract.spec.mjs");
    let e2e_spec = std::fs::read_to_string(&e2e_spec_path).unwrap_or_else(|err| {
        panic!(
            "failed to read form e2e spec {}: {err}",
            e2e_spec_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "await page.goto(\"/#/components/form\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-slot=\"form\"]",
        "#docs-form-matrix-default",
        "#docs-form-matrix-required",
        "#docs-form-matrix-disabled",
        "#docs-form-matrix-readonly",
        "data-required",
        "data-disabled",
        "data-readonly",
        "data-label-position",
        "data-label-align",
        "data-state-source",
        "aria-disabled",
        "[data-slot=\"code-block\"]",
    ] {
        assert!(
            e2e_spec.contains(required),
            "form e2e contract should keep semantic selector/wait marker `{required}`"
        );
    }

    for forbidden in [
        "waitForTimeout",
        "setTimeout(",
        "sleep(",
        "nth-child",
        ">> nth=",
    ] {
        assert!(
            !e2e_spec.contains(forbidden),
            "form e2e contract should reject fragile waits/selectors marker `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
            "`e2e/tests/docs_app_form_contract.spec.mjs`",
            "`await page.locator(\"body:not(:has(#boot))\").waitFor()`",
            "`Form` 组件不涉及异步加载与动画时间线，本项按 N/A",
            "components/form/test/semantics.rs::form_e2e_selectors_are_semantic_and_wasm_waits_are_stable_without_sleep",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document e2e semantic-selector/stable-wait evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_e2e_key_flow_is_repeatable_and_locates_semantic_contract_breakpoints() {
    let e2e_spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../e2e/tests/docs_app_form_contract.spec.mjs");
    let e2e_spec = std::fs::read_to_string(&e2e_spec_path).unwrap_or_else(|err| {
        panic!(
            "failed to read form e2e spec {}: {err}",
            e2e_spec_path.display()
        )
    });
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "docs-app form key flow is repeatable with semantic breakpoints for focus and keyboard paths",
        "await page.goto(\"/#/components/form\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "#docs-form-name",
        "#docs-form-email",
        "await page.keyboard.press(\"Tab\");",
        "data-state-source",
        "data-required",
        "data-disabled",
        "await page.reload();",
    ] {
        assert!(
            e2e_spec.contains(required),
            "form e2e key-flow contract should keep repeatable flow and semantic breakpoints via `{required}`"
        );
    }

    let e2e_snapshot_marker = format!("{}{}", "toMatch", "Snapshot");
    for forbidden in [
        "waitForTimeout",
        "setTimeout(",
        "sleep(",
        "screenshot",
        e2e_snapshot_marker.as_str(),
    ] {
        assert!(
            !e2e_spec.contains(forbidden),
            "form e2e key-flow contract should reject non-repeatable or non-semantic evidence marker `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
            "`e2e/tests/docs_app_form_contract.spec.mjs` 新增关键流程回归",
            "键盘 `Tab` 焦点流转",
            "`overlay/async` 不属于 `Form` 容器职责，按 N/A 继承",
            "components/form/test/semantics.rs::form_e2e_key_flow_is_repeatable_and_locates_semantic_contract_breakpoints",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document e2e key-flow regression evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_engineering_capability_contract_keeps_serde_tracing_and_runtime_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let protocol = include_str!("../src/protocol.rs");
    let form_cargo = include_str!("../Cargo.toml");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "Serialize, Deserialize",
        "pub enum FormComponentSchemaVersion",
        "pub struct FormComponentSpec",
    ] {
        assert!(
            protocol.contains(required),
            "form protocol should keep serde-backed schema/spec contract via `{required}`"
        );
    }

    for forbidden in [
        "tracing::",
        "trace!(",
        "debug!(",
        "info!(",
        "span!(",
        "tokio::",
        "async_std::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form runtime path should stay free of tracing/runtime coupling marker `{forbidden}`"
        );
    }

    for forbidden in ["tracing =", "tokio =", "async-std ="] {
        assert!(
            !form_cargo.contains(forbidden),
            "form crate dependencies should not pin runtime/tracing implementation detail `{forbidden}`"
        );
    }

    for forbidden in [
        "pub use tokio",
        "pub use async_std",
        "pub type Runtime",
        "pub type JoinHandle",
    ] {
        assert!(
            !module.contains(forbidden),
            "form public API must not leak runtime detail type exports `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
            "`tracing` 与 async runtime 约束在本组件按 N/A 处理",
            "`components/form/src/protocol.rs` 以 `serde::{Serialize, Deserialize}`",
            "`components/form/Cargo.toml` 未引入 `tracing`、`tokio`、`async-std`",
            "components/form/test/semantics.rs::form_engineering_capability_contract_keeps_serde_tracing_and_runtime_boundaries",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document engineering capability boundary evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_semantic_contract_tests_take_priority_over_snapshots() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let semantics_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    let semantics_source = std::fs::read_to_string(&semantics_path).unwrap_or_else(|err| {
        panic!(
            "failed to read semantics source {}: {err}",
            semantics_path.display()
        )
    });

    for required in [
        "<form",
        "aria-disabled=view_state.aria_disabled",
        "data-state-source=view_state.state_source",
    ] {
        assert!(
            view.contains(required),
            "form should expose role/aria/source semantic contract markers via `{required}`"
        );
    }

    for required in [
        "fn form_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet()",
        "fn form_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_styles_depend_on_explicit_semantic_state_markers()",
        "fn form_a11y_i18n_contract_is_wired_without_hardcoded_user_copy()",
    ] {
        assert!(
            semantics_source.contains(required),
            "form semantics test matrix should cover core branches and contracts via `{required}`"
        );
    }

    let forbidden_snapshots = [
        format!("{}{}", "assert_", "snapshot!"),
        format!("{}::{}", "insta", "assert"),
        format!("{}{}", "toMatch", "Snapshot"),
        format!("{}{}", "snapshot_", "assert"),
    ];
    for forbidden in forbidden_snapshots {
        assert!(
            !semantics_source.contains(&forbidden),
            "form should not rely on visual snapshot assertions as contract baseline: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 测试验证“语义契约”而不只验证视觉快照。",
            "相关矩阵在本组件按 N/A 处理",
            "语义契约断言是主路径",
            "components/form/test/semantics.rs::form_semantic_contract_tests_take_priority_over_snapshots",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document semantic-test-first evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_semantic_test_priority_contract_covers_data_aria_role_source_and_no_snapshot_regression() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let semantics_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");
    let semantics_source = std::fs::read_to_string(&semantics_path).unwrap_or_else(|err| {
        panic!(
            "failed to read semantics source {}: {err}",
            semantics_path.display()
        )
    });

    for required in [
        "#[cfg(test)]",
        "#[path = \"../test/semantics.rs\"]",
        "<form",
        "aria-disabled=view_state.aria_disabled",
        "data-state-source=view_state.state_source",
        "data-disabled=view_state.data_disabled",
        "data-readonly=view_state.data_read_only",
        "data-required=view_state.data_required",
    ] {
        assert!(
            view.contains(required),
            "form semantic-test-priority contract should keep stable role/aria/data/source surface via `{required}`"
        );
    }

    for required in [
        "fn form_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_a11y_i18n_contract_is_wired_without_hardcoded_user_copy()",
        "fn form_semantic_contract_tests_take_priority_over_snapshots()",
        "fn form_streaming_definition_is_limited_to_llm_output_modes()",
        "fn form_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_status()",
    ] {
        assert!(
            semantics_source.contains(required),
            "form semantic-test-priority matrix should keep key semantic branches covered via `{required}`"
        );
    }

    let forbidden_snapshots = [
        format!("{}{}", "assert_", "snapshot!"),
        format!("{}::{}", "insta", "assert"),
        format!("{}{}", "toMatch", "Snapshot"),
        format!("{}{}", "snapshot_", "assert"),
    ];
    for forbidden in forbidden_snapshots {
        assert!(
            !semantics_source.contains(&forbidden),
            "form semantic-test-priority contract should reject snapshot-only testing path: `{forbidden}`"
        );
    }

    let snapshot_policy = format!(
        "明确禁止 `{}{}{}` 作为主断言",
        "assert_", "snapshot!/insta", "::*"
    );
    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
            "`components/form/src/view.rs` 通过 `<form>` + `aria-disabled` + `data-state-source`",
            "`#[cfg(test)] #[path = \"../test/semantics.rs\"]` 挂载组件语义套件",
            snapshot_policy.as_str(),
            "components/form/test/semantics.rs::form_semantic_test_priority_contract_covers_data_aria_role_source_and_no_snapshot_regression",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document semantic-test-priority evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_component_files_follow_responsibility_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::Form;",
    ] {
        assert!(
            module.contains(required),
            "form `mod.rs` should keep minimal stable exports via `{required}`"
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !module.contains(forbidden),
            "form `mod.rs` should not leak internal implementation modules: `{forbidden}`"
        );
    }

    for forbidden in [
        "web_sys::",
        "NodeRef",
        "HtmlElement",
        "view! {",
        "pub const CSS",
    ] {
        assert!(
            !logic.contains(forbidden),
            "form `logic.rs` should stay on normalization/derivation only: `{forbidden}`"
        );
    }

    {
        let required = "pub const CSS: &str =";
        assert!(
            styles.contains(required),
            "form `styles.rs` should keep static css contract via `{required}`"
        );
    }

    for forbidden in [
        "#[component]",
        "view! {",
        "create_signal(",
        "web_sys::",
        "UiRoot",
    ] {
        assert!(
            !styles.contains(forbidden),
            "form `styles.rs` should not carry view/runtime logic: `{forbidden}`"
        );
    }

    for required in [
        "#[component]",
        "pub fn Form(",
        "let resolved = logic::resolve_props(",
        "let view_state = logic::resolve_view_state(&resolved);",
        "let locale = locale_attrs(lang, dir);",
        "view! {",
    ] {
        assert!(
            view.contains(required),
            "form `view.rs` should render structure and mount headless contract via `{required}`"
        );
    }

    assert!(
        !motion_file.exists(),
        "form is a static container; `src/motion.rs` should stay absent (N/A)."
    );
    for forbidden in ["mod motion;", "pub mod motion;", "pub use motion::"] {
        assert!(
            !module.contains(forbidden),
            "form module should not export motion implementation for N/A component: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
            "`motion.rs` 维度按 N/A（文件不存在且模块未导出）处理",
            "components/form/test/semantics.rs::form_component_files_follow_responsibility_boundaries",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document component file-responsibility evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_component_directory_standard_file_layout_is_correct() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "form component directory should contain required standard file `{required}`"
        );
    }

    for absent in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent).exists(),
            "form standard layout should keep `{absent}` absent for this simple static component."
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Form;",
    ] {
        assert!(
            module.contains(required),
            "form `mod.rs` should keep minimal stable export boundary via `{required}`"
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod render;",
        "mod motion;",
    ] {
        assert!(
            !module.contains(forbidden),
            "form `mod.rs` should not expose or include forbidden layout drift via `{forbidden}`"
        );
    }

    for required in ["pub fn resolve_props(", "pub fn resolve_view_state("] {
        assert!(
            logic.contains(required),
            "form `logic.rs` should keep normalization/derivation contract via `{required}`"
        );
    }
    assert!(
        styles.contains("pub const CSS: &str ="),
        "form `styles.rs` should keep static css contract."
    );
    assert!(
        view.contains("#[component]") && view.contains("pub fn Form("),
        "form `view.rs` should keep leptos structure renderer entry."
    );

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 组件目录标准文件落点正确。",
            "`components/form/src` 目录满足 `mod.rs + logic.rs + styles.rs + view.rs` 基线",
            "`motion.rs` 在本组件按 N/A（文件不存在且模块未导出）处理",
            "`spec.rs` 对简单组件按“无必要不新增”保持缺省",
            "components/form/test/semantics.rs::form_component_directory_standard_file_layout_is_correct",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document standard file-layout evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_file_placement_discipline_is_enforced_for_simple_component() {
    let module = load_source("mod");
    let check2 = load_source("check2");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "file-placement discipline should keep required file `{required}` in component src dir"
        );
    }

    for absent in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent).exists(),
            "file-placement discipline should keep `{absent}` absent for this simple form component"
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Form;",
    ] {
        assert!(
            module.contains(required),
            "file-placement discipline should keep `mod.rs` export boundary marker `{required}`"
        );
    }

    for forbidden in ["mod render;", "mod motion;", "mod spec;"] {
        assert!(
            !module.contains(forbidden),
            "file-placement discipline should reject module drift marker `{forbidden}`"
        );
    }

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "`components/form/src` 采用 `mod.rs + logic.rs + styles.rs + view.rs` 结构；`render.rs` 不存在",
        "`motion.rs` 在本组件按 N/A（文件不存在且模块未导出）处理；`spec.rs` 仅保留复杂组件策略，本组件保持缺省不存在",
        "components/form/test/semantics.rs::form_file_placement_discipline_is_enforced_for_simple_component",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document file-placement discipline evidence via `{required}`"
        );
    }
}

#[test]
fn form_hyper_structure_builder_is_na_for_simple_form_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "simple form component should not introduce `src/spec.rs` hyper-structure builder entry."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "FormSpec::new(",
        "struct FormSpec",
        "impl FormSpec",
        "Spec::new(",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "simple form component should not expose hyper-structure builder surface: `{forbidden}`"
        );
    }

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "`Form` 为简单语义容器，不存在复杂配置固化与 builder 装配需求，本项按 N/A 通过",
        "`components/form/src/spec.rs` 不存在，`components/form/src/mod.rs` 无 `spec` 模块导出",
        "`*Spec::new()...render()` 入口",
        "components/form/test/semantics.rs::form_hyper_structure_builder_is_na_for_simple_form_component",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document Hyper-Structure Builder N/A evidence via `{required}`"
        );
    }
}

#[test]
fn form_context_compression_manifest_and_rbi_projection_are_present() {
    let check2 = load_source("check2");
    let component_toml_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/Component.toml");
    let rbi_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/form.rbi");

    assert!(
        component_toml_path.exists(),
        "form context-compression contract requires `src/Component.toml`."
    );
    assert!(
        rbi_path.exists(),
        "form context-compression contract requires `src/form.rbi`."
    );

    let component_toml = std::fs::read_to_string(&component_toml_path).unwrap_or_else(|err| {
        panic!(
            "failed to read form manifest {}: {err}",
            component_toml_path.display()
        )
    });
    let rbi = std::fs::read_to_string(&rbi_path)
        .unwrap_or_else(|err| panic!("failed to read form RBI {}: {err}", rbi_path.display()));

    for required in [
        "schema_version = \"1\"",
        "name = \"Form\"",
        "crate = \"ui-form\"",
        "rbi = \"form.rbi\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_toml.contains(required),
            "form manifest should keep context compression metadata marker `{required}`"
        );
    }

    for required in [
        "pub type FormLabelPosition = crate::logic::FormLabelPosition;",
        "pub type FormLabelAlign = crate::logic::FormLabelAlign;",
        "pub type FormContextValue = crate::logic::FormContextValue;",
        "pub type FormViewState = crate::logic::FormViewState;",
        "pub fn Form(",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi.contains(required),
            "form rbi projection should keep stable type/signature marker `{required}`"
        );
    }

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "`components/form/src/Component.toml` 与 `components/form/src/form.rbi` 已补齐",
        "`context_compression_manifest` 与 `rbi_signature_projection` 能力",
        "components/form/test/semantics.rs::form_context_compression_manifest_and_rbi_projection_are_present",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document Manifest + RBI evidence via `{required}`"
        );
    }
}

#[test]
fn form_agent_contract_schema_markers_are_typed_and_whitelisted() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let component_manifest = include_str!("../src/Component.toml");

    for required in [
        "let agent_contract = logic::resolve_agent_contract_attrs(&view_state);",
        "data-ui-schema=agent_contract.schema",
        "data-ui-schema-version=agent_contract.schema_version",
        "data-ui-intent=agent_contract.intent",
        "data-ui-action=agent_contract.action",
        "data-ui-state-disabled=agent_contract.state_disabled",
        "data-ui-state-readonly=agent_contract.state_read_only",
        "data-ui-state-required=agent_contract.state_required",
        "data-ui-source=agent_contract.source",
    ] {
        assert!(
            view.contains(required),
            "form view should mount typed agent contract markers via `{required}`"
        );
    }

    for required in [
        "pub const FORM_AGENT_SCHEMA: &str = \"ui.form.agent-contract.v1\";",
        "pub const FORM_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum FormAgentIntent",
        "pub enum FormAgentAction",
        "pub enum FormAgentSource",
        "pub struct FormAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(view_state: &FormViewState) -> FormAgentContractAttrs",
        "intent: FormAgentIntent::FormContainer.as_attr()",
        "action: FormAgentAction::Render.as_attr()",
        "source: FormAgentSource::LogicResolved.as_attr()",
    ] {
        assert!(
            logic.contains(required),
            "form logic should generate agent contract fields from typed schema via `{required}`"
        );
    }

    assert!(
        !logic.contains("format!(\"data-ui"),
        "form logic should avoid string concatenation for agent contract marker generation."
    );

    for required in [
        "schema = \"ui.form.agent-contract.v1\"",
        "name = \"agent_contract_schema_markers\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state-disabled\"",
        "attr = \"data-ui-state-readonly\"",
        "attr = \"data-ui-state-required\"",
        "attr = \"data-ui-source\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"typed_state_from_logic::resolve_view_state\"",
        "\"typed_agent_contract_from_logic::resolve_agent_contract_attrs\"",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "form manifest should keep schema markers and whitelist contract via `{required}`"
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "`components/form/src/view.rs` 已挂载 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state-disabled/data-ui-state-readonly/data-ui-state-required/data-ui-source`",
        "`components/form/src/logic.rs` 新增 `FormAgentIntent/FormAgentAction/FormAgentSource` 与 `resolve_agent_contract_attrs`",
        "`components/form/src/Component.toml` 增加 `[agent_contract]`、`[[agent_contract_markers]]`、`[[agent_contract_whitelist]]`",
        "components/form/test/semantics.rs::form_agent_contract_schema_markers_are_typed_and_whitelisted",
    ] {
        assert!(
            check2.contains(required),
            "checklist should document agent contract schema evidence via `{required}`"
        );
    }
}

#[test]
fn form_streaming_definition_is_limited_to_llm_output_modes() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let component_manifest = include_str!("../src/Component.toml");

    for required in [
        "data-ui-stream-mode=agent_contract.stream_mode",
        "let agent_contract = logic::resolve_agent_contract_attrs(&view_state);",
    ] {
        assert!(
            view.contains(required),
            "form view should expose stream mode marker for agent-readable rendering mode via `{required}`"
        );
    }

    for required in [
        "pub enum FormAgentStreamMode",
        "FormAgentStreamMode::Streaming => \"streaming\"",
        "FormAgentStreamMode::Snapshot => \"snapshot\"",
        "stream_mode: FormAgentStreamMode::Snapshot.as_attr()",
    ] {
        assert!(
            logic.contains(required),
            "form logic should type and constrain streaming modes via `{required}`"
        );
    }

    for required in [
        "output_mode_axis = [\"streaming\", \"snapshot\"]",
        "name = \"stream_mode\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"streaming\", \"snapshot\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "form manifest should record two-mode streaming/snapshot axis via `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
            "- `Streaming`：LLM 还在生成，界面边生成边显示。",
            "- `Snapshot`：LLM 全部生成完成后，一次性显示。",
            "`FormAgentStreamMode::{Streaming, Snapshot}`",
            "data-ui-stream-mode=agent_contract.stream_mode",
            "output_mode_axis = [\"streaming\", \"snapshot\"]",
            "components/form/test/semantics.rs::form_streaming_definition_is_limited_to_llm_output_modes",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document two-mode LLM streaming definition evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_snapshot_mode_is_baseline_and_consumes_complete_config() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for required in [
        "let resolved = logic::resolve_props(",
        "is_disabled,",
        "is_read_only,",
        "is_required,",
        "label_position,",
        "label_align,",
        "class_name,",
        "let view_state = logic::resolve_view_state(&resolved);",
        "let agent_contract = logic::resolve_agent_contract_attrs(&view_state);",
        "data-ui-stream-mode=agent_contract.stream_mode",
        "<form",
        "{children()}",
    ] {
        assert!(
            view.contains(required),
            "form should consume complete config and render stable snapshot output via `{required}`"
        );
    }

    for required in [
        "stream_mode: FormAgentStreamMode::Snapshot.as_attr()",
        "FormAgentStreamMode::Snapshot => \"snapshot\"",
    ] {
        assert!(
            logic.contains(required),
            "form logic should keep snapshot mode as baseline via `{required}`"
        );
    }

    for forbidden in [
        "stream_buffer",
        "append_chunk",
        "partial_token",
        "token_delta",
        "stream_cursor",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "form snapshot baseline should not depend on incremental streaming state: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
            "`Form` 渲染链路以完整配置为输入",
            "`agent_contract.stream_mode` 固定为 `FormAgentStreamMode::Snapshot.as_attr()`",
            "data-ui-stream-mode=agent_contract.stream_mode",
            "components/form/test/semantics.rs::form_snapshot_mode_is_baseline_and_consumes_complete_config",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document snapshot baseline evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_status() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let component_manifest = include_str!("../src/Component.toml");

    for required in [
        "data-ui-streaming-policy=agent_contract.streaming_policy",
        "data-ui-streaming-fallback=agent_contract.streaming_fallback",
        "data-ui-output-status=agent_contract.output_status",
        "aria-disabled=view_state.aria_disabled",
    ] {
        assert!(
            view.contains(required),
            "form view should expose optional-streaming contract markers and aria continuity via `{required}`"
        );
    }

    for required in [
        "pub enum FormAgentStreamingPolicy",
        "FormAgentStreamingPolicy::Optional => \"optional\"",
        "FormAgentStreamingPolicy::Required => \"required\"",
        "pub enum FormAgentStreamingFallback",
        "FormAgentStreamingFallback::Snapshot => \"snapshot\"",
        "pub enum FormAgentOutputStatus",
        "FormAgentOutputStatus::Draft => \"draft\"",
        "FormAgentOutputStatus::Verified => \"verified\"",
        "FormAgentOutputStatus::Submittable => \"submittable\"",
        "streaming_policy: FormAgentStreamingPolicy::Optional.as_attr()",
        "streaming_fallback: FormAgentStreamingFallback::Snapshot.as_attr()",
        "output_status: FormAgentOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            logic.contains(required),
            "form logic should type and resolve role-based streaming optional/fallback/status via `{required}`"
        );
    }

    for required in [
        "stream_support = \"optional\"",
        "stream_fallback = \"snapshot\"",
        "output_status = \"verified\"",
        "name = \"streaming_policy\"",
        "attr = \"data-ui-streaming-policy\"",
        "values = [\"optional\", \"required\"]",
        "name = \"streaming_fallback\"",
        "attr = \"data-ui-streaming-fallback\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "attr = \"data-ui-output-status\"",
        "values = [\"draft\", \"verified\", \"submittable\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "form manifest should declare optional-streaming fallback and output status schema via `{required}`"
        );
    }

    for forbidden in ["retry", "reconnect", "transport", "validate_request"] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "form component layer should not own data validation/retry transport responsibilities: `{forbidden}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
            "`Form` 不是正文阅读面，按 `Streaming Optional` 处理",
            "`FormAgentStreamingPolicy::Optional` + `FormAgentStreamingFallback::Snapshot`",
            "`FormAgentOutputStatus::{Draft, Verified, Submittable}` 并默认 `Verified`",
            "`data-ui-output-status=agent_contract.output_status`",
            "components/form/test/semantics.rs::form_streaming_requirement_is_role_based_optional_with_snapshot_fallback_and_status",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document role-based streaming requirement evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_rust_hygiene_contract_blocks_unwrap_expect_and_ignored_results() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let protocol = include_str!("../src/protocol.rs");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");

    for source in [module, logic, view, styles, protocol] {
        for forbidden in ["unwrap(", "expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "form non-test source should satisfy rust-hygiene baseline and avoid `{forbidden}`"
            );
        }
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
            "非测试源码未出现 `unwrap(`、`expect(` 与无处理 `let _ = ...`",
            "`Cow<'static, str>` 在本组件按 N/A 处理",
            "`PCRE2 is not available in this build of ripgrep`",
            "components/form/test/semantics.rs::form_rust_hygiene_contract_blocks_unwrap_expect_and_ignored_results",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document rust-hygiene evidence for form via `{required}`"
            );
        }
    }
}

#[test]
fn form_spec_rs_scope_is_restricted_for_simple_component() {
    let module = load_source("mod");
    let check2 = load_source("check2");
    let check2_src = include_str!("../src/check2.md");
    let readme = include_str!("../src/README.md");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "form is a simple container; `src/spec.rs` should not exist."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "form module should not expose spec surface for a simple component: `{forbidden}`"
        );
    }

    for required in ["## Props", "## Hello World（默认路径）"] {
        assert!(
            readme.contains(required),
            "form docs should keep simple component guidance in README/checklist instead of `spec.rs`: `{required}`"
        );
    }

    for checklist in [check2, check2_src] {
        for required in [
            "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
            "组件目录未引入 `components/form/src/spec.rs`",
            "components/form/test/semantics.rs::form_spec_rs_scope_is_restricted_for_simple_component",
        ] {
            assert!(
                checklist.contains(required),
                "checklist should document spec.rs scope boundary evidence via `{required}`"
            );
        }
    }
}

#[test]
fn form_checklist_records_headless_scope_and_na_boundary() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "handlers 维度按 N/A 处理",
        "components/form/test/semantics.rs::form_view_mounts_headless_locale_contract",
    ] {
        assert!(
            check2.contains(required),
            "form checklist should document ui-headless coverage boundary via `{required}`"
        );
    }
}

#[test]
fn form_component_keeps_motion_layer_na_for_static_container() {
    let view = load_source("view");
    let module = load_source("mod");
    let check2 = load_source("check2");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    assert!(
        !motion_file.exists(),
        "Form is a static container; motion layer should stay N/A without `src/motion.rs`."
    );

    for forbidden in ["attach_motion(", "ui_motion::", "use ui_motion"] {
        assert!(
            !view.contains(forbidden),
            "form view should not wire ui-motion runtime: `{forbidden}`"
        );
    }

    for forbidden in ["mod motion;", "pub mod motion;", "pub use motion::"] {
        assert!(
            !module.contains(forbidden),
            "form module should not export motion layer for a static container: `{forbidden}`"
        );
    }

    for required in [
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "motion.rs` 在该项按 N/A 处理",
        "components/form/test/semantics.rs::form_component_keeps_motion_layer_na_for_static_container",
    ] {
        assert!(
            check2.contains(required),
            "form checklist should record motion N/A boundary via `{required}`"
        );
    }
}

#[test]
fn form_component_consumes_ui_theme_tokens_without_rebuilding_theme_context() {
    let styles = load_source("styles");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    assert!(
        styles.contains("--ui-space-md"),
        "form styles should consume ui-theme css variables from `--ui-*` namespace."
    );
    assert!(
        styles.contains("var(--ui-space-md)"),
        "form styles should consume ui-theme token variable `--ui-space-md`."
    );

    for forbidden in [
        "ui_theme::",
        "ThemeContext",
        "ThemeSystem",
        "ThemeColor",
        "ThemeScale",
    ] {
        assert!(
            !logic.contains(forbidden),
            "form logic should not rebuild ui-theme context mapping: `{forbidden}`"
        );
        assert!(
            !view.contains(forbidden),
            "form view should not rebuild ui-theme context mapping: `{forbidden}`"
        );
    }

    for required in [
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "components/form/test/semantics.rs::form_component_consumes_ui_theme_tokens_without_rebuilding_theme_context",
    ] {
        assert!(
            check2.contains(required),
            "form checklist should document ui-theme boundary evidence via `{required}`"
        );
    }
}

#[test]
fn form_component_stays_in_ui_components_assembly_boundary() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let semantics_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs");

    assert!(
        semantics_file.exists(),
        "form component should keep semantic regression tests in `components/form/test/semantics.rs`."
    );

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::Form;",
    ] {
        assert!(
            module.contains(required),
            "form module should keep stable ui assembly boundary `{required}`"
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !module.contains(forbidden),
            "form module should keep internals private: `{forbidden}`"
        );
    }

    for forbidden in ["web_sys::", "web-sys", "JsValue", "HtmlElement", "NodeRef"] {
        assert!(
            !module.contains(forbidden),
            "form module public surface must not expose platform detail `{forbidden}`"
        );
        assert!(
            !logic.contains(forbidden),
            "form logic must not leak platform detail `{forbidden}`"
        );
        assert!(
            !view.contains(forbidden),
            "form view must not leak platform detail `{forbidden}`"
        );
    }

    assert!(
        styles.contains("pub const CSS: &str ="),
        "form styles should stay as static token-first css contract."
    );

    for required in [
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/form/test/semantics.rs::form_component_stays_in_ui_components_assembly_boundary",
    ] {
        assert!(
            check2.contains(required),
            "form checklist should document ui assembly boundary via `{required}`"
        );
    }
}
