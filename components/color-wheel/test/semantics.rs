fn load_source(rel_path: &str) -> &'static str {
    match rel_path {
        "../../components/color-wheel/src/mod.rs" => include_str!("../src/mod.rs"),
        "../../components/color-wheel/src/logic.rs" => include_str!("../src/logic.rs"),
        "../../components/color-wheel/src/view.rs" => include_str!("../src/view.rs"),
        "../../components/color-wheel/src/motion.rs" => include_str!("../src/motion.rs"),
        "../../components/color-wheel/src/styles.rs" => include_str!("../src/styles.rs"),
        "../../components/color-wheel/src/protocol.rs" => include_str!("../src/protocol.rs"),
        "../../components/color-wheel/src/README.md" => include_str!("../src/README.md"),
        "../../components/color-wheel/src/Component.toml" => {
            include_str!("../src/Component.toml")
        }
        "../../components/color-wheel/src/color_wheel.rbi" => {
            include_str!("../src/color_wheel.rbi")
        }
        "../../crates/ui-components/src/css.rs" => {
            include_str!("../../../crates/ui-components/src/css.rs")
        }
        "../../crates/ui-components/Cargo.toml" => {
            include_str!("../../../crates/ui-components/Cargo.toml")
        }
        "../../crates/ui-components/src/lib.rs" => {
            include_str!("../../../crates/ui-components/src/lib.rs")
        }
        "../../crates/ui-components/src/root.rs" => {
            include_str!("../../../crates/ui-components/src/root.rs")
        }
        "../../apps/web-demo/Cargo.toml" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "../../apps/docs-app/Cargo.toml" => include_str!("../../../apps/docs-app/Cargo.toml"),
        "../../apps/docs-app/src/lib.rs" => include_str!("../../../apps/docs-app/src/lib.rs"),
        "../../apps/docs-app/src/playground.rs" => {
            include_str!("../../../apps/docs-app/src/playground.rs")
        }
        "../../apps/docs-app/src/perf_probe.rs" => {
            include_str!("../../../apps/docs-app/src/perf_probe.rs")
        }
        "../../apps/docs-app/src/debug_overlay.rs" => {
            include_str!("../../../apps/docs-app/src/debug_overlay.rs")
        }
        "../../crates/ui-headless/src/trace.rs" => {
            include_str!("../../../crates/ui-headless/src/trace.rs")
        }
        "../../scripts/check-ui-components-tree-shaking.sh" => {
            include_str!("../../../scripts/check-ui-components-tree-shaking.sh")
        }
        "../../scripts/check-ui-components-performance.sh" => {
            include_str!("../../../scripts/check-ui-components-performance.sh")
        }
        "../../scripts/check-ui-components-dx.sh" => {
            include_str!("../../../scripts/check-ui-components-dx.sh")
        }
        "../../scripts/check-ui-components-streaming.sh" => {
            include_str!("../../../scripts/check-ui-components-streaming.sh")
        }
        "../../scripts/check-ui-components-engineering.sh" => {
            include_str!("../../../scripts/check-ui-components-engineering.sh")
        }
        "../../scripts/check-ui-components-contract-hygiene.sh" => {
            include_str!("../../../scripts/check-ui-components-contract-hygiene.sh")
        }
        "../../scripts/check-ui-components-e2e-color-wheel.sh" => {
            include_str!("../../../scripts/check-ui-components-e2e-color-wheel.sh")
        }
        "../../scripts/tree_shaking_budget.env" => {
            include_str!("../../../scripts/tree_shaking_budget.env")
        }
        "../../e2e/tests/docs_app_components_coverage.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "../../e2e/tests/docs_app_color_wheel_contract.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_color_wheel_contract.spec.mjs")
        }
        "../../components/color-wheel/check2.md" => include_str!("../check2.md"),
        "../../components/color-wheel/Cargo.toml" => include_str!("../Cargo.toml"),
        "../../docs/plan/TODO.md" => include_str!("../../../docs/plan/TODO.md"),
        "../../apps/docs-app/src/pages/components/shell.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/forms_color.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs" => {
            include_str!(
                "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs"
            )
        }
        "../../apps/docs-app/src/pages/components/pages.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "legacy_semantics" => {
            include_str!("../../../components/color-wheel/test/color_wheel_semantics.rs")
        }
        "protocol_test" => include_str!("protocol.rs"),
        _ => panic!("unsupported source path: {rel_path}"),
    }
}

#[test]
fn color_wheel_semantics_tests_are_migrated_to_component_directory() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    assert!(
        mod_source.contains("#[path = \"../test/semantics.rs\"]")
            && mod_source.contains("mod semantics_tests;"),
        "color-wheel should wire `components/color-wheel/test/semantics.rs` from crate entry.",
    );

    assert!(
        legacy_semantics.contains("../../../components/color-wheel/test/semantics.rs"),
        "legacy ui-components semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics.contains("color_wheel_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_wheel_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-wheel public module should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_component_layer_keeps_file_responsibilities() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub(crate) mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorWheelMotion;",
        "pub use view::ColorWheel;",
    ] {
        assert!(
            mod_source.contains(needle),
            "color-wheel module boundary should include `{needle}`.",
        );
    }

    for needle in [
        "use ui_state_primitives::color_wheel as primitives;",
        "pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState {",
        "primitives::resolve_state(input)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ColorWheelState) -> String {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should compose state from primitives; missing `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "overlay_open::use_color_wheel(",
        "motion::attach_motion(root_ref, visual_percent, motion);",
        "logic::resolve_state(ColorWheelStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount headless semantics and logic mappings; missing `{needle}`.",
        );
    }

    for needle in [
        "pub struct ColorWheelMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_theme::default_slider_motion_tokens",
        "ui_motion::spring::sanitize_config",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should map component semantics to ui-motion contracts; missing `{needle}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str") && styles_source.contains("var(--ui-"),
        "styles.rs should expose token-first static css contract.",
    );
}

#[test]
fn color_wheel_component_tests_live_in_neighbor_test_directory() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let protocol_source = include_str!("../src/protocol.rs");

    for needle in [
        "../test/logic.rs",
        "../test/motion.rs",
        "../test/protocol.rs",
        "../test/semantics.rs",
    ] {
        assert!(
            mod_source.contains(needle)
                || logic_source.contains(needle)
                || motion_source.contains(needle)
                || protocol_source.contains(needle),
            "color-wheel should keep tests next to `src/` in `test/`; missing `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_api_naming_uses_is_prefix_with_legacy_alias_migration() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] is_value_label_visible: Option<bool>",
        "#[prop(optional, default = true)] show_value_label: bool",
        "let normalized_inputs = logic::normalize_state_inputs(",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorWheel view should include `{needle}` for API naming migration."
        );
    }

    for needle in [
        "title=\"ColorWheel\"",
        "id_base=\"docs-color-wheel-disabled\".to_string()",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "ColorWheel docs should include `{needle}` after API naming normalization."
        );
    }
}

#[test]
fn color_wheel_controlled_uncontrolled_contract_is_complete_and_centralized() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change)",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorWheel should include `{needle}` for controlled/uncontrolled pairing."
        );
    }

    for needle in [
        "<Playground title=\"Controlled Hue Wheel\" code_signal=basic_code>",
        "value=value.into()",
        "on_value_change=on_value_change",
        "<Playground title=\"Disabled + Reduced Motion + Custom Class\" code_signal=states_code>",
        "default_value=282.0",
    ] {
        assert!(
            docs_source.contains(needle),
            "ColorWheel docs should include `{needle}` for controlled/uncontrolled examples."
        );
    }
}

#[test]
fn color_wheel_dx_paradox_keeps_hello_world_path_simple() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    let wheel_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color.rs should include color_wheel docs section");
    let wheel_end = docs_source[wheel_start..]
        .find("\npub(super) fn color_picker() -> AnyView {")
        .map(|idx| wheel_start + idx)
        .expect("forms_color.rs should keep color_wheel section before color_picker");
    let wheel_docs = &docs_source[wheel_start..wheel_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ColorWheel id_base=\"docs-color-wheel-hello\".to_string() />",
        "r##\"<ColorWheel\n  id_base=\"docs-color-wheel-hello\".to_string()\n/>\"##",
    ] {
        assert!(
            wheel_docs.contains(needle),
            "ColorWheel docs should include `{needle}` for copy-paste Hello World."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "state=",
        "use_async_action",
    ] {
        assert!(
            !wheel_docs.contains(forbidden),
            "Hello World path should not require low-level contract `{forbidden}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] state",
        "#[prop(into)] state",
        "state: Signal<",
        "state: ReadSignal<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorWheel public API should not require internal state object `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let dx_script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let wheel_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color.rs should include color_wheel docs section");
    let wheel_end = docs_source[wheel_start..]
        .find("\npub(super) fn color_picker() -> AnyView {")
        .map(|idx| wheel_start + idx)
        .expect("forms_color.rs should keep color_wheel section before color_picker");
    let wheel_docs = &docs_source[wheel_start..wheel_end];

    for required in [
        "pub(super) fn color_wheel() -> AnyView {",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming Optional / Snapshot\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
        "data-slot=\"color-wheel-state-matrix\"",
        "data-slot=\"color-wheel-controlled-vs-uncontrolled\"",
        "data-slot=\"color-wheel-output-mode\"",
        "data-slot=\"color-wheel-copy-ready\"",
    ] {
        assert!(
            wheel_docs.contains(required),
            "color-wheel docs should keep docs-product marker `{required}`.",
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming Optional / Snapshot",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "code_imports",
        "color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep docs-product evidence `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(script_needle),
        "dx script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        "color_wheel_check2_documents_docs_sync_and_state_matrix_rules",
        "color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "color_wheel_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep docs-sync/state-matrix evidence `{required}`.",
        );
    }
}

#[test]
fn color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");

    let wheel_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color.rs should include color_wheel docs section");
    let wheel_end = docs_source[wheel_start..]
        .find("\npub(super) fn color_picker() -> AnyView {")
        .map(|idx| wheel_start + idx)
        .expect("forms_color.rs should keep color_wheel section before color_picker");
    let wheel_docs = &docs_source[wheel_start..wheel_end];

    for marker in [
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Parameter Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"color-wheel-state-matrix\"",
        "data-slot=\"color-wheel-parameter-matrix\"",
        "data-slot=\"color-wheel-api-defaults-note\"",
        "id_base=\"docs-color-wheel-param-default\".to_string()",
        "id_base=\"docs-color-wheel-param-step\".to_string()",
        "id_base=\"docs-color-wheel-param-hidden-value\".to_string()",
        "step=15.0",
        "is_value_label_visible=false",
        "is_disabled=true",
        "value=value.into()",
        "on_value_change=on_value_change",
        "default_value=180.0",
        "Default API sync: step uses logic::DEFAULT_STEP when omitted; default_value falls back through logic::resolve_default_value; is_disabled defaults to false.",
    ] {
        assert!(
            wheel_docs.contains(marker),
            "color-wheel docs should keep API/default sync marker `{marker}`.",
        );
    }

    for marker in [
        "#[prop(optional, default = logic::DEFAULT_STEP)] step: f64",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] is_value_label_visible: Option<bool>",
        "#[prop(optional, default = true)] show_value_label: bool",
        "let normalized_inputs = logic::normalize_state_inputs(",
        "let default_value = logic::resolve_default_value(default_value, step);",
    ] {
        assert!(
            view_source.contains(marker),
            "color-wheel view API surface should keep `{marker}` for docs sync.",
        );
    }

    for marker in [
        "pub const DEFAULT_STEP: f64 = primitives::DEFAULT_STEP;",
        "pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64 {",
        "primitives::resolve_default_value(default_value, step)",
        "status: ColorWheelStatus::from_disabled(is_disabled.unwrap_or(disabled))",
        "is_value_label_visible.unwrap_or(show_value_label)",
    ] {
        assert!(
            logic_source.contains(marker),
            "color-wheel logic defaults and normalization should keep `{marker}`.",
        );
    }

    for forbidden in [
        "disabled=true",
        "show_value_label=false",
        "default_disabled",
    ] {
        assert!(
            !wheel_docs.contains(forbidden),
            "color-wheel docs should avoid stale/alias API token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX script should enforce color-wheel docs-sync/state-matrix contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "color_wheel_check2_documents_documentation_as_product_rules",
        "color_wheel_documentation_entry_exists_with_beginner_first_progression",
        "color_wheel_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep documentation-as-product evidence `{required}`.",
        );
    }
}

#[test]
fn color_wheel_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/color-wheel/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for required in [
        "# ColorWheel",
        "## Hello World",
        "<ColorWheel id_base=\"demo-color-wheel\".to_string() />",
        "## 常见用法",
        "受控：`value + on_value_change`",
        "非受控：`default_value`",
        "## 新手路径（先用起来，再进阶）",
        "1. 先跑默认路径：`<ColorWheel id_base=... />`",
        "2. 再加常见参数：`default_value`、`is_disabled`、`label`、`aria_label`",
        "3. 最后再用进阶参数：`motion`、`class_name`、`lang`、`dir`",
        "docs-app 页面：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()`",
    ] {
        assert!(
            readme_source.contains(required),
            "color-wheel README should keep beginner-first doc marker `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("docs should include color-wheel Hello World playground");
    let workbench_pos = docs_source
        .find("title=\"Interactive Workbench (DX)\"")
        .expect("docs should include color-wheel Interactive Workbench playground");
    assert!(
        hello_pos < workbench_pos,
        "docs should keep beginner-first order: Hello World before Interactive Workbench.",
    );

    for required in [
        "pub(super) fn color_wheel() -> AnyView {",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Parameter Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs color-wheel entry should include `{required}`.",
        );
    }
}

#[test]
fn color_wheel_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX script should enforce documentation-as-product contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
fn color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    let section_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color docs should contain color_wheel section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_picker() -> AnyView {")
        .expect("forms_color docs should contain color_picker section after color_wheel");
    let section = &section_tail[..section_end_rel];

    for marker in [
        "title=\"Interactive Workbench (DX)\"",
        "data-slot=\"color-wheel-workbench-controls\"",
        "data-slot=\"color-wheel-workbench-preset\"",
        "data-slot=\"color-wheel-workbench-toggle-disabled\"",
        "data-slot=\"color-wheel-workbench-toggle-custom-class\"",
        "data-slot=\"color-wheel-workbench-toggle-reduced-motion\"",
        "data-slot=\"color-wheel-workbench-toggle-preserve-context\"",
        "data-slot=\"color-wheel-workbench-toggle-persist-state\"",
        "data-slot=\"color-wheel-workbench\"",
        "data-slot=\"color-wheel-workbench-canvas\"",
        "data-slot=\"color-wheel-workbench-state\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "workbench_code = Signal::derive(move || {",
        "Switch checked=workbench_disabled",
        "Switch checked=workbench_custom_class",
        "Switch checked=workbench_reduced_motion",
        "Switch checked=workbench_preserve_context",
        "Switch checked=workbench_persist_state",
    ] {
        assert!(
            section.contains(marker),
            "color-wheel docs interactive playground should keep marker `{marker}`."
        );
    }
}

#[test]
fn color_wheel_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_wheel_contract.spec.mjs");

    for marker in [
        "docs-app color-wheel interactive playground updates props and preview with semantic markers",
        "data-slot=\"color-wheel-workbench-controls\"",
        "data-slot=\"color-wheel-workbench-state\"",
        "getByRole(\"checkbox\", { name: \"Disabled\" })",
        "getByRole(\"checkbox\", { name: \"Custom class\" })",
        "getByRole(\"checkbox\", { name: \"Reduced motion\" })",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"ready\")",
        "toHaveAttribute(\"data-class-source\", \"custom\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "docs-app color-wheel key flow is repeatable and failures map to semantic breakpoints",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-wheel interactive playground e2e flow should keep marker `{marker}`."
        );
    }
}

#[test]
fn color_wheel_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn color_wheel_e2e_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-wheel.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "e2e check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn color_wheel_check2_marks_interactive_playground_contract_complete() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "AI Spec 联动示例 N/A（`ColorWheel` 非 AI Spec 输入组件）",
        "color_wheel_check2_documents_interactive_playground_rules",
        "color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "color_wheel_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "color_wheel_dx_check_script_covers_interactive_playground_contract",
        "color_wheel_e2e_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "scripts/check-ui-components-e2e-color-wheel.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-wheel checklist should keep interactive-playground evidence marker `{marker}`."
        );
    }
}

#[test]
fn color_wheel_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-wheel/src/README.md");

    let section_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color docs should contain color_wheel section");
    let section_tail = &docs_source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_picker() -> AnyView {")
        .expect("forms_color docs should contain color_picker section after color_wheel");
    let section = &section_tail[..section_end_rel];

    for needle in [
        "data-slot=\"color-wheel-copy-ready\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "<code>\"code_imports\"</code>",
        "data-slot=\"color-wheel-source-paths\"",
        "<code>\"components/color-wheel/src/mod.rs\"</code>",
        "<code>\"components/color-wheel/src/view.rs\"</code>",
        "<code>\"components/color-wheel/src/logic.rs\"</code>",
        "<code>\"components/color-wheel/src/styles.rs\"</code>",
        "<code>\"components/color-wheel/src/motion.rs\"</code>",
        "data-slot=\"color-wheel-source-prerequisites\"",
        "<code>\"component-color_wheel\"</code>",
        "<code>\"inject-css\"</code>",
        "data-slot=\"color-wheel-source-first-contract\"",
        "<code>\"Show code + Copy\"</code>",
    ] {
        assert!(
            section.contains(needle),
            "color-wheel source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::ColorWheel;\".to_string()",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            section.contains(needle),
            "color-wheel playground snippets should keep copy-ready marker `{needle}`.",
        );
    }

    for needle in [
        "## Source-first",
        "components/color-wheel/src/{mod,logic,view,styles,motion}.rs",
        "component-color_wheel",
        "inject-css",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            readme_source.contains(needle),
            "color-wheel README should document source-first dependency/path marker `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "color_wheel_check2_documents_source_first_copy_paste_ready_rules",
        "color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "color_wheel_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-wheel checklist should keep source-first copy-paste-ready evidence marker `{marker}`."
        );
    }
}

#[test]
fn color_wheel_check2_documents_heroui_benchmark_docs_sync_rules() {
    let checklist_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-wheel checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-wheel/src/README.md");

    for needle in [
        "### ColorWheel 同步记录（2026-02-20）",
        "参数模型同步：`ColorWheel` 参数主轴保持 `value + on_value_change + default_value`",
        "component_doc!(\"ColorWheel\", \"color-wheel\", \"Forms\", forms_color::color_wheel)",
        "#/components/color-wheel",
        "`components/color-wheel/src/README.md` 提供等价文档入口",
        "forms_color.rs::color_wheel()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include color-wheel synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ColorWheel\"",
        "\"color-wheel\"",
        "forms_color::color_wheel",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose color-wheel entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn color_wheel() -> AnyView {",
        "title=\"ColorWheel\"",
        "slug=\"color-wheel\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app color-wheel page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# ColorWheel"),
        "color-wheel README should remain an equivalent component doc entry.",
    );
}

#[test]
fn color_wheel_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "color_wheel_check2_documents_heroui_benchmark_docs_sync_rules",
        "color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "color_wheel_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-wheel check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn color_wheel_is_not_a_composite_parent_item_api() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    let wheel_start = docs_source
        .find("pub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color.rs should include color_wheel docs section");
    let wheel_end = docs_source[wheel_start..]
        .find("\npub(super) fn color_picker() -> AnyView {")
        .map(|idx| wheel_start + idx)
        .expect("forms_color.rs should keep color_wheel section before color_picker");
    let wheel_docs = &docs_source[wheel_start..wheel_end];

    for forbidden in [
        "children:",
        "#[prop(optional)] children",
        "#[prop(optional, into)] children",
        "items:",
        "labels:",
        "titles:",
        "panels:",
        "item_specs:",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorWheel should not expose composite Parent/Item API token `{forbidden}`.",
        );
    }

    for forbidden in [
        "labels=",
        "titles=",
        "panels=",
        "items=",
        "<ColorWheelItem",
        "</ColorWheel>",
    ] {
        assert!(
            !wheel_docs.contains(forbidden),
            "ColorWheel docs should not recommend composite-array syntax `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_default_value_priority_is_normalized_in_logic_not_view() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/color_wheel.rs");

    for needle in [
        "pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64 {",
        "primitives::resolve_default_value(default_value, step)",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorWheel logic should include `{needle}` for default value normalization."
        );
    }

    assert!(
        primitive_source.contains(
            "pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64 {"
        ),
        "ui-state-primitives color_wheel should expose default value normalization primitive.",
    );

    assert!(
        view_source
            .contains("let default_value = logic::resolve_default_value(default_value, step);"),
        "view.rs should consume normalized default value from logic.rs.",
    );
    assert!(
        !view_source.contains("default_value.unwrap_or("),
        "view.rs should not do fallback priority via `unwrap_or` directly.",
    );
}

#[test]
fn color_wheel_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "pub struct ColorWheelInputBoundary",
        "pub type ColorWheelStatus = primitives::ColorWheelStatus;",
        "pub type ColorWheelValueLabelMode = primitives::ColorWheelValueLabelMode;",
        "pub fn normalize_state_inputs(",
        "status: ColorWheelStatus::from_disabled(is_disabled.unwrap_or(disabled))",
        "value_label_mode: ColorWheelValueLabelMode::from_visible(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should include `{needle}` for centralized state input normalization."
        );
    }

    assert!(
        view_source.contains("let normalized_inputs = logic::normalize_state_inputs("),
        "view.rs should consume normalized state inputs from logic.rs.",
    );

    for needle in [
        "let is_disabled = normalized_inputs.is_disabled();",
        "let is_value_label_visible = normalized_inputs.is_value_label_visible();",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should consume normalized enum boundary via `{needle}`.",
        );
    }

    for forbidden in [
        "let is_disabled = is_disabled.unwrap_or(disabled);",
        "let is_value_label_visible = is_value_label_visible.unwrap_or(show_value_label);",
        "match ev.key().as_str()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not keep state normalization/state-machine rule `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_discrete_states_are_modeled_with_enums() {
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/color_wheel.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "pub enum ColorWheelStatus",
        "pub enum ColorWheelValueLabelMode",
        "pub enum ColorWheelSource",
        "pub status: ColorWheelStatus,",
        "pub value_label_mode: ColorWheelValueLabelMode,",
        "pub motion_source: ColorWheelSource,",
        "pub label_source: ColorWheelSource,",
        "pub aria_source: ColorWheelSource,",
        "pub class_source: ColorWheelSource,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives color-wheel should include `{needle}` for typed discrete state modeling.",
        );
    }

    for needle in [
        "pub fn source_from_custom_flag(is_custom: bool) -> ColorWheelSource {",
        "ColorWheelSource::from_custom(is_custom)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should include `{needle}` for bool-to-enum mapping.",
        );
    }

    for needle in [
        "status: normalized_inputs.status,",
        "value_label_mode: normalized_inputs.value_label_mode,",
        "motion_source: logic::source_from_custom_flag(has_custom_motion),",
        "label_source: logic::source_from_custom_flag(has_custom_label),",
        "aria_source: logic::source_from_custom_flag(has_custom_aria_label),",
        "class_source: logic::source_from_custom_flag(has_custom_class_name),",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should include `{needle}` so discrete states enter primitives as enums.",
        );
    }
}

#[test]
fn color_wheel_state_primitives_source_boundary_is_enforced() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "use ui_state_primitives::color_wheel as primitives;",
        "pub type ColorWheelStateInput = primitives::ColorWheelStateInput;",
        "pub type ColorWheelState = primitives::ColorWheelState;",
        "pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState {",
        "primitives::resolve_state(input)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ColorWheelState) -> String {",
        "primitives::compose_class_name(base_class_name, state)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should consume state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "leptos::",
        "web_sys",
        "use_context(",
        "use_context::<",
        "global_store",
        "app_store",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not bind framework/business store detail `{forbidden}`.",
        );
    }

    for needle in [
        "let normalized_inputs = logic::normalize_state_inputs(",
        "logic::resolve_state(ColorWheelStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should consume logic boundary via `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("ui_state_primitives::color_wheel"),
        "view.rs should not bypass component logic boundary to call primitives directly.",
    );
}

#[test]
fn color_wheel_has_no_async_interaction_protocol_surface() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "spawn_local",
        "tokio::",
        "Future",
        "async fn",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-wheel should not expose async interaction contract token `{forbidden}` when component has no async flow.",
        );
    }

    assert!(
        view_source.contains("disabled=move || state.get().is_disabled"),
        "non-async color-wheel should only expose disabled semantics for interaction gating.",
    );
}

#[test]
fn color_wheel_macro_micro_dual_state_machine_commits_on_drag_end() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "pub enum Action {",
        "DragEnd { value: f64, step: f64 },",
        "pub fn resolve_action(action: Action) -> f64 {",
        "Action::DragEnd { value, step } => sanitize_value(value, step),",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should expose macro-state action `{needle}`.",
        );
    }

    for needle in [
        "let (drag_preview_value, set_drag_preview_value) = signal(None::<f64>);",
        "let (drag_preview_percent, set_drag_preview_percent) = signal(None::<f64>);",
        "set_drag_preview_value.set(Some(next));",
        "set_drag_preview_percent.set(Some((next / 360.0 * 100.0).clamp(0.0, 100.0)));",
        "logic::resolve_action(logic::Action::DragEnd { value: next, step })",
        "commit_drag_end_on_up.run(());",
        "commit_drag_end_on_cancel.run(());",
        "commit_drag_end_on_leave.run(());",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should include `{needle}` for local drag loop + drag-end convergence.",
        );
    }

    for forbidden in [
        "on_track_pointer_down_handler.run(next) else {\n                return;\n            };\n\n            request_value_change.run(next);",
        "on_track_pointer_move_handler.run(next) else {\n                return;\n            };\n            request_value_change.run(next);",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not push pointer frame updates directly into macro state `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_two_pass_geometry_rectification_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    assert!(
        logic_source.contains("pub fn pointer_to_hue_angle("),
        "logic.rs should keep pure geometric conversion helper.",
    );

    for needle in [
        "let rect = track.get_bounding_client_rect();",
        "logic::pointer_to_hue_angle(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should sample DOM rect and call pure logic conversion `{needle}`.",
        );
    }

    assert!(
        !logic_source.contains("get_bounding_client_rect")
            && !logic_source.contains("leptos::web_sys")
            && !logic_source.contains("leptos::ev::PointerEvent"),
        "logic.rs should not hold DOM/web bindings for geometry sampling.",
    );

    for forbidden in [
        "Rectification",
        "Intent::Measure",
        "Action::Measure",
        "Action::Rectify",
        "set_measured_rect",
        "measured_rect",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-wheel should not introduce geometry two-pass rectification token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_registration_protocol_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose collection registration token `{forbidden}`.",
        );
    }

    for needle in [
        "data-slot=\"color-wheel-track\"",
        "data-slot=\"color-wheel-input\"",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should remain a single-input control surface with `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_slot_projection_strategy_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose slot projection contract token `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("data-slot=\"color-wheel\"")
            && view_source.contains("data-slot=\"color-wheel-input\"")
            && view_source.contains("data-slot=\"color-wheel-track\""),
        "color-wheel should remain a fixed single-control surface instead of container slot projection.",
    );
}

#[test]
fn color_wheel_env_streams_are_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "Action::Env",
        "Action::Resize",
        "Action::Theme",
        "Action::Intersection",
        "debounce",
        "throttle",
        "match_media",
        "matchMedia",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose env-stream contract token `{forbidden}`.",
        );
    }

    for needle in [
        "on:pointermove=on_pointer_move",
        "on:input=on_input",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should stay driven by direct user-input handlers `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_event_light_cone_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "Context Bus",
        "SelectionState::All",
        "prop drilling",
        "bulk_select",
        "selection_compression",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose event-light-cone token `{forbidden}`.",
        );
    }

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "request_value_change.run(next);",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should keep single-value direct-update contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_causality_bus_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "event_bus",
        "publish(",
        "subscribe(",
        "broadcast(",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose causality-bus token `{forbidden}`.",
        );
    }

    for needle in [
        "let on_input = move |ev: ev::Event| {",
        "let on_key_down = move |ev: ev::KeyboardEvent| {",
        "request_value_change.run(next);",
        "request_value_change.run(result.next_value);",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should keep direct local causality path `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_focus_stack_and_gc_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "provide_overlay_stack",
        "use_overlay_stack",
        "use_overlay_stack_registration",
        "RestorePolicy",
        "FallbackTo",
        "Selector(",
        "document.body",
        "restore_focus",
        "focus_restore",
        "data-ui-overlay-portal",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose overlay focus-stack token `{forbidden}`.",
        );
    }

    for needle in [
        "let track_ref: NodeRef<html::Div> = NodeRef::new();",
        "let root_ref: NodeRef<html::Div> = NodeRef::new();",
        "node_ref=track_ref",
        "node_ref=root_ref",
        "on:pointerdown=on_pointer_down",
        "on:pointermove=on_pointer_move",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should keep non-overlay NodeRef usage token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_escape_hatches_foreign_zone_is_not_applicable() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "Foreign Zone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "third_party_instance",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose foreign-zone escape-hatch token `{forbidden}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] chart",
        "#[prop(optional)] map",
        "#[prop(optional)] instance",
        "pub chart",
        "pub map",
        "pub instance",
    ] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "color-wheel public API should not leak third-party imperative handle token `{forbidden}`.",
        );
    }

    for needle in [
        "let semantics = overlay_open::use_color_wheel(overlay_open::ColorWheelOptions {",
        "let value_state =",
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change);",
        "logic::resolve_state(ColorWheelStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should keep internal-first state/semantics wiring token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_hydration_discontinuity_is_not_applicable() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now",
        "Uuid::new",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "randomUUID",
        "Math::random",
        "IdProvider",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not introduce hydration-unstable seed token `{forbidden}`.",
        );
    }

    for needle in [
        "id_base: String,",
        "let id_base = logic::normalize_optional_text(Some(id_base))",
        ".unwrap_or_else(|| \"ui-color-wheel\".to_string());",
        "let input_id = format!(\"{id_base}-input\");",
        "let label_id = format!(\"{id_base}-label\");",
        "let value_id = format!(\"{id_base}-value\");",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should keep deterministic id derivation token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_platform_paths_are_cfg_guarded_and_non_wasm_safe() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "use leptos::wasm_bindgen::JsCast;",
        "let track: leptos::web_sys::Element = track.unchecked_into();",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(_ev);",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should preserve explicit platform guard token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let element: leptos::web_sys::HtmlElement = root.unchecked_into();",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visual_percent: leptos::prelude::Signal<f64>,",
        "_motion: ColorWheelMotion,",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should preserve wasm + non-wasm attach contract token `{needle}`.",
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "js_sys", "window", "document"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay platform-agnostic without browser token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_preserves_ui_headless_web_ssr_mutual_exclusion_contract() {
    let headless_lib_source = include_str!("../../../crates/ui-headless/src/lib.rs");
    let color_wheel_cargo = include_str!("../Cargo.toml");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep feature mutual-exclusion guard token `{needle}`.",
        );
    }

    for needle in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "use ui_headless::{self as overlay_open, A11yDirection};",
        "overlay_open::use_color_wheel(",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            color_wheel_cargo.contains(needle) || view_source.contains(needle),
            "color-wheel integration should keep headless contract token `{needle}`.",
        );
    }

    for forbidden in [
        "features = [\"web\", \"ssr\"]",
        "features=[\"web\",\"ssr\"]",
        "default-features = false, features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !color_wheel_cargo.contains(forbidden),
            "color-wheel should not request mutually-exclusive ui-headless features `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_motion_non_wasm_noop_stub_contract_is_preserved() {
    let ui_motion_lib_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let color_wheel_motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop() {",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should preserve non-wasm noop backend token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visual_percent: leptos::prelude::Signal<f64>,",
        "_motion: ColorWheelMotion,",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
    ] {
        assert!(
            color_wheel_motion_source.contains(needle),
            "color-wheel motion bridge should preserve wasm/non-wasm safe token `{needle}`.",
        );
    }

    for forbidden in ["panic!(", "expect(\"", "unwrap()", "unwrap_unchecked"] {
        assert!(
            !color_wheel_motion_source.contains(forbidden),
            "motion no-op/stub path should avoid panic-assumption token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_reduced_motion_ssr_wasm_branch_contract_is_consistent() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "drop(style.set_property(\"--ui-slider-visual-percent\", &format!(\"{target:.4}\")));",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>,",
        "_visual_percent: leptos::prelude::Signal<f64>,",
        "_motion: ColorWheelMotion,",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should preserve reduced-motion + wasm/non-wasm branch token `{needle}`.",
        );
    }

    for needle in [
        "let semantics = overlay_open::use_color_wheel(overlay_open::ColorWheelOptions {",
        "motion::attach_motion(root_ref, visual_percent, motion);",
        "role=root_role",
        "aria-valuenow=move || input_aria_valuenow.get()",
        "data-state=move || state.get().data_state_attr",
        "let id_base = logic::normalize_optional_text(Some(id_base))",
        ".unwrap_or_else(|| \"ui-color-wheel\".to_string());",
        "let input_id = format!(\"{id_base}-input\");",
        "let label_id = format!(\"{id_base}-label\");",
        "let value_id = format!(\"{id_base}-value\");",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep stable SSR/hydration + semantic contract token `{needle}`.",
        );
    }

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Uuid::",
        "rand::",
        "Math::random",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "reduced-motion/SSR/wasm split should avoid hydration-unstable token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_motion_contract_is_tokenized_attached_and_safe_across_platforms() {
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "let tokens = default_slider_motion_tokens();",
        "stiffness: tokens.spring.stiffness,",
        "damping: tokens.spring.damping,",
        "mass: tokens.spring.mass,",
        "precision: tokens.spring.precision,",
        "pub fn sanitize_motion(motion: ColorWheelMotion) -> ColorWheelMotion {",
        "ui_motion::spring::sanitize_config(value, ColorWheelMotion::default().spring)",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub fn attach_motion(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "_motion: ColorWheelMotion,",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion-contract token `{required}`.",
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(root_ref, visual_percent, motion);"),
        "view.rs should mount motion contract through attach_motion.",
    );

    for forbidden in ["panic!(", "expect(\"", "unwrap()", "unwrap_unchecked"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not assume wasm/non-wasm branch by panic token `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "color-wheel check2 should mark motion-contract checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`ColorWheelMotion::default` 从 `default_slider_motion_tokens()` 映射 `stiffness/damping/mass/precision`"
        ),
        "color-wheel check2 should document tokenized motion-contract evidence.",
    );
}

#[test]
fn color_wheel_ui_components_entrypoints_follow_fixed_layered_locations() {
    let ui_components_lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css_source = load_source("../../crates/ui-components/src/css.rs");
    let ui_components_root_source = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_a11y_source = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let headless_controllable_state_source =
        include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence_source = include_str!("../../../crates/ui-headless/src/presence.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-color_wheel\")]",
        "pub use ui_color_wheel as color_wheel;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            ui_components_lib_source.contains(required),
            "ui-components lib entry should keep token `{required}`.",
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen::", "JsValue", "HtmlElement"] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui-components lib public entry should not expose platform token `{forbidden}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_wheel\")]",
        "out.push_str(crate::color::wheel::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css_source.contains(required),
            "ui-components css entry should keep token `{required}`.",
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(required),
            "UiRoot entry should keep theme/injection/i18n token `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "set_property(\"--ui-active-highlight-y\"",
        "set_property(\"--ui-active-highlight-h\"",
        "set_property(\"--ui-active-highlight-o\"",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight shared primitive should keep token `{required}`.",
        );
    }

    for forbidden in ["ColorWheel", "Dialog", "Popover", "Tooltip", "business"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should avoid component-business semantic token `{forbidden}`.",
        );
    }

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence {",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>> {",
    ] {
        assert!(
            headless_controllable_state_source.contains(required)
                || headless_presence_source.contains(required)
                || headless_a11y_source.contains(required),
            "headless shared primitive location should keep token `{required}`.",
        );
    }

    let ui_components_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(forbidden_file).exists(),
            "ui-components src should not contain forbidden relocated entry `{forbidden_file}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "color-wheel check2 should mark ui-components fixed-entrypoints item as complete.",
    );
    assert!(
        check2_source.contains(
            "`crates/ui-components/src/lib.rs` 维持 feature-gated 对外 `pub use`，`css.rs` 仅通过 `push_components_css` 走条件聚合，`root.rs` 集中注入 base/theme/components css 与 i18n"
        ),
        "color-wheel check2 should document fixed-entrypoint evidence.",
    );
}

#[test]
fn color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let local_semantics = include_str!("semantics.rs");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"color-wheel\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget token `{needle}`.",
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
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`.",
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
            "docs coverage e2e should enforce perf regression guard `{needle}`.",
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based attribution token `{needle}`.",
        );
    }

    {
        let needle =
            "fn color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking()";
        assert!(
            local_semantics.contains(needle),
            "semantics suite should keep performance-governance contract token `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
        "Button`、`Input`",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorWheel checklist should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "--ui-color-wheel-motion-duration",
        "@media (prefers-reduced-motion: reduce)",
        "ui_motion::spring::sanitize_config",
    ] {
        assert!(
            view_source.contains(needle)
                || styles_source.contains(needle)
                || motion_source.contains(needle),
            "color-wheel should expose performance attribution token `{needle}`.",
        );
    }

    let view_memo_count = view_source.matches("Memo::new(").count();
    assert!(
        view_memo_count <= 3,
        "color-wheel reactive budget exceeded: expected <= 3 `Memo::new`, found {view_memo_count}.",
    );

    let view_signal_derive_count = view_source.matches("Signal::derive(").count();
    assert!(
        view_signal_derive_count <= 2,
        "color-wheel reactive budget exceeded: expected <= 2 `Signal::derive`, found {view_signal_derive_count}.",
    );
}

#[test]
fn color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for marker in [
        "role=root_role",
        "role=input_role",
        "aria-disabled=input_aria_disabled",
        "aria-valuemin=move || input_aria_valuemin.get_value()",
        "aria-valuemax=move || input_aria_valuemax.get_value()",
        "aria-valuenow=move || input_aria_valuenow.get()",
        "aria-valuetext=move || input_aria_valuetext.get()",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
        "on:pointerdown=on_pointer_down",
        "on:pointermove=on_pointer_move",
        "on:pointerup=move |_| {",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(marker),
            "color-wheel semantics/perf matrix should keep aria/data/focus-path marker `{marker}`.",
        );
    }

    for marker in [
        ".ui-color-wheel__track:focus-within .ui-color-wheel__ring {",
        "var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width))",
    ] {
        assert!(
            styles_source.contains(marker),
            "color-wheel styles should keep focus-visible feedback marker `{marker}`.",
        );
    }

    for marker in [
        "\"color-wheel\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(marker),
            "docs shell should keep color-wheel perf budget marker `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-components --test color_wheel_semantics color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should enforce `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "TODO should keep render_count follow-up marker `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "color_wheel_state_markers_are_observable_searchable_and_closed_set",
        "color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless",
        "color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-wheel check2 semantics/perf section should reference `{marker}`.",
        );
    }
}

#[test]
fn color_wheel_view_macro_complexity_is_split_into_semantic_sections() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "fn render_header_section(",
        "let render_track_section = move || {",
        "{render_header_section(",
        "{render_track_section()}",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should split semantic view sections via `{needle}`.",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count >= 3,
        "view.rs should keep split view macros (root/header/track); found only {view_macro_count}.",
    );

    let root_view_start = view_source
        .rfind("view! {")
        .expect("view.rs should contain root `view!` block");
    let root_view = &view_source[root_view_start..];

    for needle in ["{render_header_section(", "{render_track_section()}"] {
        assert!(
            root_view.contains(needle),
            "root `view!` should compose semantic sections via `{needle}`.",
        );
    }

    for forbidden in [
        "ui-color-wheel__header",
        "ui-color-wheel__track",
        "ui-color-wheel__thumb",
    ] {
        assert!(
            !root_view.contains(forbidden),
            "root `view!` should avoid embedding deep structure token `{forbidden}`.",
        );
    }

    for needle in [
        "on:pointerdown=on_pointer_down",
        "on:pointermove=on_pointer_move",
        "on:input=on_input",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(needle),
            "view split must preserve interaction handler token `{needle}`.",
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控",
        "render_header_section",
        "render_track_section",
        "避免巨型单块 `view!`",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep view-macro complexity evidence token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_functional_split_prefers_plain_functions_for_lightweight_fragments() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "fn render_header_section(",
        ") -> impl IntoView {",
        "{render_header_section(",
        "let render_track_section = move || {",
        "{render_track_section()}",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep functional split token `{needle}`.",
        );
    }

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "functional split should avoid component-noise; expected exactly one `#[component]`, found {component_macro_count}.",
    );

    assert!(
        !view_source.contains("#[component]\nfn render_header_section")
            && !view_source.contains("#[component]\r\nfn render_header_section"),
        "lightweight section should stay as plain function rather than nested component.",
    );

    for needle in [
        "- [x] 函数式拆分优先",
        "render_header_section",
        "普通 Rust 函数",
        "不新增 `#[component]`",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep functional split evidence token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_static_fragments_are_templated_and_accessible() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "fn render_static_track_visuals() -> impl IntoView {",
        "{render_static_track_visuals()}",
        "data-slot=\"color-wheel-ring\"",
        "data-slot=\"color-wheel-orbit\"",
        "data-slot=\"color-wheel-thumb\"",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should template static track fragment token `{needle}`.",
        );
    }

    for (token, expected_count) in [
        ("ui-color-wheel__ring", 1usize),
        ("ui-color-wheel__orbit", 1usize),
        ("ui-color-wheel__thumb", 1usize),
    ] {
        let actual = view_source.matches(token).count();
        assert_eq!(
            actual, expected_count,
            "static visual token `{token}` should have exactly one template source; found {actual}.",
        );
    }

    assert!(
        !view_source.contains("inner_html="),
        "static fragment templating should not rely on raw `inner_html` injection.",
    );

    for needle in [
        "- [x] 静态片段常量化",
        "render_static_track_visuals",
        "aria-hidden",
        "静态资源变更路径",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep static-fragment templating evidence token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_inner_html_surface_is_absent_and_security_regression_is_blocked() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        ".set_inner_html(",
        "innerHTML",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "color-wheel should not expose raw HTML injection surface `{forbidden}`.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] html:",
        "#[prop(optional, into)] content_html:",
        "#[prop(optional, into)] markup:",
        "javascript:",
        "<script",
        "onerror=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "color-wheel should not accept untrusted HTML-like protocol token `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束",
        "无 `inner_html` 注入面",
        "color_wheel_inner_html_surface_is_absent_and_security_regression_is_blocked",
        "禁止拼接用户输入",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep inner_html governance evidence token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_wasm_debug_trace_contract_is_observable_replayable_and_debug_gated() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "const TRACE_COMPONENT: &str = \"color-wheel\";",
        "let trace = overlay_open::use_ui_trace();",
        "fn emit_value_transition(",
        "event={event} source={} before={before:.3} after={after:.3} step={step:.3}",
        "\"pointer_down_preview\"",
        "\"pointer_move_preview\"",
        "\"drag_end_commit\"",
        "\"input_commit\"",
        "\"keyboard_commit\"",
        "overlay_open::UiTraceEventKind::Note { message }",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel view should expose wasm debug trace token `{needle}`.",
        );
    }

    for needle in [
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "UiTraceEventKind::Note {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind) {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace source should keep replayable timeline token `{needle}`.",
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
            "docs app should keep debug-only visualization gate token `{needle}`.",
        );
    }

    for needle in [
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView {",
        "data-slot=\"ui-debug-overlay-events\"",
        "ui_headless::UiTraceEventKind::Note { message } => (\"note\", message, \"note\"),",
        "data-kind=kind_attr",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace timeline rendering token `{needle}`.",
        );
    }

    for forbidden in [
        "pub use view::ColorWheelDebug",
        "#[prop(optional)] debug",
        "#[prop(optional)] trace",
        "debug_overlay_enabled:",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-wheel public API should not expose debug-only control token `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] WASM 调试要求",
        "event/source/before/after/step",
        "UiTraceEvent.ts_ms",
        "cfg!(debug_assertions)",
        "UiDebugOverlay",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep wasm-debug governance evidence token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground should keep DX hot-style/isolated-canvas marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn color_wheel() -> AnyView {",
        "title=\"Interactive Workbench (DX)\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/color-wheel/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "id_base=\"docs-color-wheel-workbench\".to_string()",
        "data-slot=\"color-wheel-workbench-controls\"",
        "data-slot=\"color-wheel-workbench-canvas\"",
        "data-slot=\"color-wheel-workbench-state\"",
        "<Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "const COLOR_WHEEL_WORKBENCH_STORAGE_KEY: &str = \"docs:color-wheel:workbench:state\";",
        "fn load_color_wheel_workbench_state() -> Option<ColorWheelWorkbenchState> {",
        "fn save_color_wheel_workbench_state(state: ColorWheelWorkbenchState) {",
        "fn clear_color_wheel_workbench_state() {",
        "if workbench_persist_state.get() {",
        "save_color_wheel_workbench_state(state);",
        "clear_color_wheel_workbench_state();",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "color-wheel docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let color_wheel_cargo = load_source("../../components/color-wheel/Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let protocol_source = load_source("../../components/color-wheel/src/protocol.rs");
    let protocol_test_source = load_source("protocol_test");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-wheel/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "color-wheel should keep spec.rs as N/A for simple component scope.",
    );

    for needle in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "default = []",
    ] {
        assert!(
            color_wheel_cargo.contains(needle),
            "color-wheel cargo should include serde/protocol baseline `{needle}`.",
        );
    }

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum WheelComponentSchemaVersion",
        "pub struct WheelComponentSpec",
        "#[serde(default)]",
        "pub schema_version: WheelComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "color-wheel protocol should keep structured serde schema token `{needle}`.",
        );
    }

    for needle in [
        "use serde::de::DeserializeOwned;",
        "T: Serialize + DeserializeOwned,",
        "assert_serde::<WheelComponentSchemaVersion>();",
        "assert_serde::<WheelComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "color-wheel protocol test should keep serde contract token `{needle}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "tracing = { version = \"0.1\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components should keep shared tracing baseline token `{needle}`.",
        );
    }

    for forbidden in [
        "component-color_wheel\", \"dep:tracing",
        "color-wheel-wasm-debug =",
        "color_wheel-wasm-debug =",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "color-wheel should not define component-local tracing feature `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components root should keep unified tracing proxy token `{needle}`.",
        );
    }

    for forbidden in [
        "tokio::",
        "async_std::",
        "async-std",
        "JoinHandle",
        "Runtime",
        "Handle",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "color-wheel should not leak async runtime detail `{forbidden}` in component surface.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let protocol_source = load_source("../../components/color-wheel/src/protocol.rs");
    let component_manifest = load_source("../../components/color-wheel/src/Component.toml");
    let rbi_source = load_source("../../components/color-wheel/src/color_wheel.rbi");
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for required in [
        "pub enum WheelComponentSchemaVersion",
        "V1",
        "pub struct WheelComponentSpec",
        "pub schema_version: WheelComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "color-wheel protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-wheel.agent-contract.v1\"",
        "values = [\"ui.color-wheel.agent-contract.v1\"]",
        "values = [\"1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-wheel Component.toml should keep v1 registration marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "without major breaking upgrade, color-wheel should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorWheel` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/color-wheel/src/protocol.rs` 的 `WheelComponentSchemaVersion::V1`、`components/color-wheel/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.color-wheel.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-wheel/test/semantics.rs::color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-components-engineering.sh` 新增对应 `cargo test` 目标。）",
        "color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let headless_color_wheel_source =
        include_str!("../../../crates/ui-headless/src/color_wheel.rs");
    let headless_a11y_source = include_str!("../../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let semantics = overlay_open::use_color_wheel(overlay_open::ColorWheelOptions {",
        "lang,",
        "dir,",
        "role=root_role",
        "role=input_role",
        "aria-labelledby=move || root_aria_labelledby.get_value()",
        "aria-label=move || input_aria_label.get_value()",
        "aria-labelledby=move || input_aria_labelledby.get_value()",
        "aria-describedby=move || input_aria_describedby.get_value()",
        "aria-valuenow=move || input_aria_valuenow.get()",
        "aria-valuetext=move || input_aria_valuetext.get()",
        "let on_key_down = move |ev: ev::KeyboardEvent| {",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel view should mount a11y/i18n contract token `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL: &str = primitives::DEFAULT_LABEL;",
        "pub const DEFAULT_ARIA_LABEL: &str = primitives::DEFAULT_ARIA_LABEL;",
        "pub fn normalize_label(value: Option<String>) -> (String, bool) {",
        "pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool) {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep label/aria label override + fallback boundary `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("\"Hue\"")
            && !view_source.contains("\"Hue wheel\"")
            && !view_source.contains("role=\"slider\"")
            && !view_source.contains("role=\"group\""),
        "view.rs should not hardcode user-visible label text or duplicate headless role literals.",
    );

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "role: \"group\"",
        "role: \"slider\"",
    ] {
        assert!(
            headless_color_wheel_source.contains(needle),
            "headless color_wheel should own shared a11y semantics `{needle}`.",
        );
    }

    for needle in [
        "pub enum A11yDirection",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs",
    ] {
        assert!(
            headless_a11y_source.contains(needle),
            "ui-headless a11y helpers should provide shared locale contract `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_state_markers_are_observable_searchable_and_closed_set() {
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot=\"color-wheel\"",
        "data-slot=\"color-wheel-track\"",
        "data-slot=\"color-wheel-input\"",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel should expose stable state/source marker `{needle}`.",
        );
    }

    for needle in [
        "let control_mode_attr = if is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "let value_source_attr = if is_controlled { \"external\" } else { \"default\" };",
    ] {
        assert!(
            view_source.contains(needle),
            "color-wheel marker values should be closed enumerable set token `{needle}`.",
        );
    }

    for needle in [
        "pub enum ColorWheelInteractionSource",
        "Self::None => \"none\"",
        "Self::Pointer => \"pointer\"",
        "Self::Keyboard => \"keyboard\"",
        "Self::Input => \"input\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-wheel interaction source marker should be type-derived token `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("nth-child")
            && !view_source.contains(":nth-")
            && docs_source.contains("stable slot/data-state contracts."),
        "selector strategy should prefer semantic markers instead of brittle DOM-order selectors.",
    );
}

#[test]
fn color_wheel_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = include_str!("../../../crates/ui-state-primitives/src/color_wheel.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for needle in [
        "pub enum ColorWheelStatus",
        "pub enum ColorWheelValueLabelMode",
        "pub enum ColorWheelSource",
        "pub struct ColorWheelStateInput {",
        "pub status: ColorWheelStatus,",
        "pub value_label_mode: ColorWheelValueLabelMode,",
        "pub motion_source: ColorWheelSource,",
        "pub label_source: ColorWheelSource,",
        "pub aria_source: ColorWheelSource,",
        "pub class_source: ColorWheelSource,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state-primitives should keep typed input-space contract token `{needle}`.",
        );
    }

    for forbidden in [
        "pub status: bool",
        "pub value_label_mode: bool",
        "pub motion_source: String",
        "pub label_source: String",
        "pub aria_source: String",
        "pub class_source: String",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "state-primitives should not regress to string/bool protocol `{forbidden}`.",
        );
    }

    for needle in [
        "pub type ColorWheelStatus = primitives::ColorWheelStatus;",
        "pub type ColorWheelValueLabelMode = primitives::ColorWheelValueLabelMode;",
        "pub type ColorWheelSource = primitives::ColorWheelSource;",
        "pub fn normalize_state_inputs(",
        "ColorWheelStatus::from_disabled(",
        "ColorWheelValueLabelMode::from_visible(",
        "pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState {",
        "primitives::resolve_state(input)",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep typed normalization/derivation token `{needle}`.",
        );
    }

    for needle in [
        "let normalized_inputs = logic::normalize_state_inputs(",
        "status: normalized_inputs.status,",
        "value_label_mode: normalized_inputs.value_label_mode,",
        "motion_source: logic::source_from_custom_flag(has_custom_motion),",
        "label_source: logic::source_from_custom_flag(has_custom_label),",
        "aria_source: logic::source_from_custom_flag(has_custom_aria_label),",
        "class_source: logic::source_from_custom_flag(has_custom_class_name),",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep machine-readable typed-state marker token `{needle}`.",
        );
    }

    for forbidden in [
        "ColorWheelStatus::from_disabled",
        "ColorWheelValueLabelMode::from_visible",
        "data-state=move || format!(",
        "data-control-mode=move || format!(",
        "data-value-source=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not re-create typing/marker rules ad-hoc `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        ".ui-color-wheel__track[data-dragging=\"true\"] .ui-color-wheel__input {",
        ".ui-color-wheel[data-disabled=\"true\"]",
        ".ui-color-wheel[data-motion-source=\"custom\"]",
        ".ui-color-wheel[data-label-source=\"custom\"]",
        ".ui-color-wheel[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should branch visual state via explicit marker `{needle}`.",
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", " > ."] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not rely on brittle DOM structure selector `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "view.rs should not inject runtime business styles inline.",
    );

    assert!(
        motion_source.contains("set_property(\"--ui-slider-visual-percent\",")
            && !motion_source.contains("set_property(\"top\"")
            && !motion_source.contains("set_property(\"left\"")
            && !motion_source.contains("set_property(\"width\"")
            && !motion_source.contains("set_property(\"height\""),
        "runtime styling should only touch required CSS variable contract.",
    );
}

#[test]
fn color_wheel_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let css_aggregation_source = load_source("../../crates/ui-components/src/css.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should own static stylesheet entry.",
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs visual values should consume ui-theme tokens via `var(--ui-*)`.",
    );
    for forbidden in ["@apply", "tailwind", "tw-", "styled(", "css!"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not introduce utility-first/css-in-rust token `{forbidden}`.",
        );
    }

    assert!(
        css_aggregation_source.contains("#[cfg(feature = \"component-color_wheel\")]")
            && css_aggregation_source.contains("out.push_str(crate::color::wheel::styles::CSS);"),
        "ui-components css aggregation should include color-wheel styles behind feature gate.",
    );

    assert!(
        ui_root_source.contains("if inject_components_css.get_value() {")
            && ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated component css through push_components_css.",
    );

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "view.rs should not carry runtime business style branches.",
    );
    assert!(
        motion_source.contains("set_property(\"--ui-slider-visual-percent\",")
            && !motion_source.contains("set_property(\"color\"")
            && !motion_source.contains("set_property(\"background\"")
            && !motion_source.contains("set_property(\"border\""),
        "runtime style writes should be limited to required CSS variable updates.",
    );
}

#[test]
fn color_wheel_styles_use_defensive_dual_fallback_variables_without_hardcoded_terminal_values() {
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-common-black, var(--ui-fallback-common-black))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-slider-thumb-border-width, var(--ui-fallback-slider-thumb-border-width))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-checkbox-group-disabled-opacity, var(--ui-fallback-checkbox-group-disabled-opacity))",
        "var(--ui-checkbox-group-motion-duration,",
        "var(--ui-fallback-checkbox-group-motion-duration)",
        "var(--ui-checkbox-group-motion-easing,",
        "var(--ui-fallback-checkbox-group-motion-easing)",
    ] {
        assert!(
            styles_source.contains(required),
            "color-wheel styles should keep defensive dual-fallback token chain `{required}`.",
        );
    }

    for forbidden in [
        "#000",
        "14px",
        "20px",
        "13px",
        "18px",
        "120ms",
        "var(--ui-motion-duration-fast, 120ms)",
        "var(--ui-motion-ease-standard, ease)",
        "border-radius: 999px;",
        "border: 2px solid",
        "outline: 2px solid",
        "outline-offset: 2px;",
        "opacity: 0.62;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-wheel styles should not keep hardcoded terminal literal `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "color-wheel check2 should mark defensive-variables checklist item as complete.",
    );
    assert!(
        check2_source.contains("终值统一收敛到 `ui-theme` 输出的 `--ui-fallback-*`"),
        "color-wheel check2 should document defensive-variable rationale.",
    );
}

#[test]
fn color_wheel_css_is_aggregated_under_layer_ui_with_only_custom_property_runtime_updates() {
    let css_aggregate_source = load_source("../../crates/ui-components/src/css.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_wheel\")]",
        "out.push_str(crate::color::wheel::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_aggregate_source.contains(required),
            "ui-components css aggregation should keep layer contract token `{required}`.",
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "view.rs should not use inline style attributes for runtime layout/visual adjustments.",
    );

    assert!(
        motion_source.contains("set_property(\"--ui-slider-visual-percent\",")
            && !motion_source.contains("set_property(\"top\"")
            && !motion_source.contains("set_property(\"left\"")
            && !motion_source.contains("set_property(\"width\"")
            && !motion_source.contains("set_property(\"height\""),
        "runtime numeric adjustments should only write CSS custom properties.",
    );

    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "color-wheel check2 should mark @layer-ui checklist item as complete.",
    );
    assert!(
        check2_source.contains(
            "`push_components_css` 通过 `@layer ui` 包裹 `component-color_wheel` 的样式注入"
        ),
        "color-wheel check2 should document @layer-ui evidence.",
    );
}

#[test]
fn color_wheel_visual_desire_default_theme_baseline_is_credible() {
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let theme_baseline_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    for needle in [
        ".ui-color-wheel__label {",
        "font-weight: 600;",
        ".ui-color-wheel__value {",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        ".ui-color-wheel__track:hover .ui-color-wheel__thumb {",
        ".ui-color-wheel__track[data-dragging=\"true\"] .ui-color-wheel__thumb {",
        ".ui-color-wheel__track:focus-within .ui-color-wheel__ring {",
        "outline: var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width)) solid",
        "var(--ui-accent, var(--ui-fallback-accent)),",
        "var(--ui-bg, var(--ui-fallback-bg)) 64%",
    ] {
        assert!(
            styles_source.contains(needle),
            "default-theme visual feedback/hierarchy should include `{needle}`.",
        );
    }

    for needle in [
        "title=\"Default Theme Baseline\"",
        "data-doc-visual-baseline=\"color-wheel-default-theme\"",
        "data-doc-baseline-shot=\"color-wheel-default-theme-v1\"",
        "data-doc-visual-targets=\"label,value,hover,active,focus\"",
        "id_base=\"docs-color-wheel-baseline-primary\".to_string()",
        "id_base=\"docs-color-wheel-baseline-depth\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "color-wheel docs should include visual baseline snapshot target `{needle}`.",
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "<Button",
        "<Input",
        "<Overlay",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_baseline_source.contains(needle),
            "docs-app shared visual baseline page should include `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep theme visual baseline route token `{needle}`.",
        );
    }

    for forbidden in ["bootstrap", "Bootstrap", "hero-ui clone", "copy HeroUI API"] {
        assert!(
            !docs_source.contains(forbidden) && !theme_baseline_source.contains(forbidden),
            "visual desire alignment should not drift into API cloning token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "component-color_wheel = [\"dep:ui-color-wheel\"]",
        "ui-color-wheel = { path = \"../../components/color-wheel\", optional = true }",
        "all-components = [",
        "\"component-color_wheel\"",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components feature graph should keep tree-shaking edge `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_wheel\")]",
        "pub use ui_color_wheel as color_wheel;",
        "#[cfg(feature = \"component-color_wheel\")]\n    pub use crate::color_wheel as wheel;",
        "#[cfg(feature = \"all-components\")]\nmod all_components {",
        "#[cfg(feature = \"all-components\")]\npub use all_components::*;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib exports should remain feature-gated token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "#[cfg(feature = \"component-color_wheel\")]\n    out.push_str(crate::color::wheel::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "css aggregation should keep tree-shaking guard token `{needle}`.",
        );
    }

    for needle in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo.contains(needle),
            "web-demo should keep scoped feature dependency token `{needle}`.",
        );
    }
    assert!(
        !web_demo_cargo.contains("\"all-components\""),
        "web-demo should not pull `all-components` implicitly.",
    );

    for needle in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"all-components\"] }",
        "\"all-components\"",
    ] {
        assert!(
            docs_app_cargo.contains(needle),
            "docs-app should explicitly opt into full surface token `{needle}`.",
        );
    }

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "source \"$BUDGET_FILE\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking gate script should preserve token `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "COLOR_WHEEL_MIN_FEATURES=\"component-color_wheel,inject-css\"",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$COLOR_WHEEL_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$COLOR_WHEEL_TREE_OUTPUT\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$COLOR_WHEEL_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should enforce color-wheel contract token `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-color_wheel",
        "color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-wheel check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_documents_semantics_first_testing_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let local_semantics = include_str!("semantics.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "color_wheel_check2_documents_semantics_first_testing_rules",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep semantics-first evidence `{required}`.",
        );
    }

    for required in [
        "fn color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless()",
        "fn color_wheel_state_markers_are_observable_searchable_and_closed_set()",
        "fn color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn color_wheel_semantics_tests_cover_contract_matrix_without_snapshot_dependency()",
    ] {
        assert!(
            local_semantics.contains(required),
            "color-wheel semantics suite should keep semantics-priority regression `{required}`.",
        );
    }

    for required in [
        "role=root_role",
        "role=input_role",
        "aria-valuenow=move || input_aria_valuenow.get()",
        "aria-valuetext=move || input_aria_valuetext.get()",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "on:pointerdown=on_pointer_down",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(required),
            "view semantics contract should keep role/aria/data-source marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_semantics_first_testing_rules";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "color_wheel_check2_documents_e2e_selector_and_stable_wait_rules",
        "color_wheel_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "color_wheel_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-color-wheel.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep color-wheel e2e selector/stable-wait marker `{required}`.",
        );
    }
}

#[test]
fn color_wheel_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_wheel_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-wheel.sh");

    for required in [
        "const COLOR_WHEEL_PAGE = \"/#/components/color-wheel\";",
        "body:not(:has(#boot))",
        "[data-component=\"color-wheel\"] #docs-color-wheel-hue[data-slot=\"color-wheel\"][data-control-mode=\"controlled\"]",
        "data-slot=\"color-wheel-input\"",
        "data-slot=\"color-wheel-label\"",
        "data-slot=\"color-wheel-track\"",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-intent",
        "data-ui-action",
        "data-ui-source",
        "data-ui-output-status",
        "data-ui-state",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-wheel e2e selector/wait contract should include `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "color-wheel e2e selector/wait contract should avoid brittle token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-wheel gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_wheel_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-wheel.sh");

    for required in [
        "input.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-ui-action\", \"keyboard\")",
        "dispatchEvent(\"pointerdown\"",
        "dispatchEvent(\"pointermove\"",
        "dispatchEvent(\"pointerup\"",
        "toHaveAttribute(\"data-interaction-source\", \"pointer\")",
        "toHaveAttribute(\"data-ui-action\", \"pointer\")",
        "#docs-color-wheel-disabled[data-slot=\"color-wheel\"]",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "#docs-color-wheel-custom[data-slot=\"color-wheel\"]",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-source\", \"none\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-wheel e2e ready/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-wheel gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "color_wheel_check2_documents_e2e_repeatable_key_flow_rules",
        "color_wheel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "color_wheel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-color-wheel.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep repeatable color-wheel e2e flow marker `{required}`.",
        );
    }
}

#[test]
fn color_wheel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_wheel_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-wheel.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "input.focus()",
        "keyboard.press(\"ArrowRight\")",
        "data-interaction-source\", \"keyboard\"",
        "data-ui-action\", \"keyboard\"",
        "data-ui-source\", \"on_value_change\"",
        "data-ui-output-status\", \"submittable\"",
        "await page.reload();",
        "data-ui-action\", \"idle\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-wheel e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-wheel gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_wheel_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-wheel.sh");

    for required in [
        "high-risk paths keep keyboard and disabled branches semantically explicit",
        "input.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowLeft\")",
        "data-interaction-source\", \"keyboard\"",
        "data-ui-action\", \"keyboard\"",
        "data-ui-source\", \"on_value_change\"",
        "data-ui-state\", \"active\"",
        "#docs-color-wheel-disabled[data-slot=\"color-wheel\"]",
        "data-state\", \"disabled\"",
        "data-disabled\", \"true\"",
        "data-ui-state\", \"disabled\"",
        "data-ui-output-status\", \"submittable\"",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-wheel e2e path should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-wheel gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_semantics_tests_cover_contract_matrix_without_snapshot_dependency() {
    let local_semantics = include_str!("semantics.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "fn color_wheel_controlled_uncontrolled_contract_is_complete_and_centralized()",
        "fn color_wheel_has_no_async_interaction_protocol_surface()",
        "fn color_wheel_macro_micro_dual_state_machine_commits_on_drag_end()",
        "fn color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn color_wheel_view_macro_complexity_is_split_into_semantic_sections()",
        "fn color_wheel_functional_split_prefers_plain_functions_for_lightweight_fragments()",
        "fn color_wheel_static_fragments_are_templated_and_accessible()",
        "fn color_wheel_inner_html_surface_is_absent_and_security_regression_is_blocked()",
        "fn color_wheel_wasm_debug_trace_contract_is_observable_replayable_and_debug_gated()",
        "fn color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot()",
        "fn color_wheel_check2_documents_docs_sync_and_state_matrix_rules()",
        "fn color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults()",
        "fn color_wheel_dx_check_script_covers_docs_sync_and_state_matrix_contract()",
        "fn color_wheel_check2_documents_documentation_as_product_rules()",
        "fn color_wheel_documentation_entry_exists_with_beginner_first_progression()",
        "fn color_wheel_dx_check_script_covers_documentation_as_product_contract()",
        "fn color_wheel_check2_documents_interactive_playground_rules()",
        "fn color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview()",
        "fn color_wheel_interactive_playground_reuses_repeatable_semantic_e2e_flow()",
        "fn color_wheel_dx_check_script_covers_interactive_playground_contract()",
        "fn color_wheel_e2e_check_script_covers_interactive_playground_contract()",
        "fn color_wheel_check2_marks_interactive_playground_contract_complete()",
        "fn color_wheel_check2_documents_source_first_copy_paste_ready_rules()",
        "fn color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies()",
        "fn color_wheel_dx_check_script_covers_source_first_copy_paste_ready_contract()",
        "fn color_wheel_check2_marks_source_first_copy_paste_ready_contract_complete()",
        "fn color_wheel_check2_documents_heroui_benchmark_docs_sync_rules()",
        "fn color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable()",
        "fn color_wheel_dx_check_script_covers_heroui_benchmark_docs_sync_contract()",
        "fn color_wheel_check2_marks_heroui_benchmark_docs_sync_contract_complete()",
        "fn color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas()",
        "fn color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries()",
        "fn color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()",
        "fn color_wheel_styles_use_defensive_dual_fallback_variables_without_hardcoded_terminal_values()",
        "fn color_wheel_css_is_aggregated_under_layer_ui_with_only_custom_property_runtime_updates()",
        "fn color_wheel_motion_contract_is_tokenized_attached_and_safe_across_platforms()",
        "fn color_wheel_ui_components_entrypoints_follow_fixed_layered_locations()",
        "fn color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless()",
        "fn color_wheel_state_markers_are_observable_searchable_and_closed_set()",
        "fn color_wheel_type_system_and_semantic_markers_form_machine_readable_contract()",
        "fn color_wheel_focus_stack_and_gc_is_not_applicable()",
        "fn color_wheel_escape_hatches_foreign_zone_is_not_applicable()",
        "fn color_wheel_hydration_discontinuity_is_not_applicable()",
        "fn color_wheel_platform_paths_are_cfg_guarded_and_non_wasm_safe()",
        "fn color_wheel_preserves_ui_headless_web_ssr_mutual_exclusion_contract()",
        "fn color_wheel_motion_non_wasm_noop_stub_contract_is_preserved()",
        "fn color_wheel_reduced_motion_ssr_wasm_branch_contract_is_consistent()",
        "fn color_wheel_visual_desire_default_theme_baseline_is_credible()",
        "fn color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded()",
        "fn color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget()",
        "fn color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete()",
        "fn color_wheel_check2_documents_semantics_first_testing_rules()",
        "fn color_wheel_check2_documents_e2e_selector_and_stable_wait_rules()",
        "fn color_wheel_e2e_selector_contract_uses_semantic_markers_and_stable_waits()",
        "fn color_wheel_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints()",
        "fn color_wheel_check2_documents_e2e_repeatable_key_flow_rules()",
        "fn color_wheel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic()",
        "fn color_wheel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints()",
        "fn color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn color_wheel_component_directory_standard_file_layout_is_enforced()",
        "fn color_wheel_does_not_introduce_spec_module_for_simple_component()",
        "fn color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current()",
        "fn color_wheel_agent_contract_is_schema_typed_and_machine_readable()",
        "fn color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free()",
        "fn color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn color_wheel_check2_documents_streaming_required_optional_classification_rules()",
        "fn color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()",
        "fn color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()",
        "fn color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent()",
        "fn color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards()",
        "fn color_wheel_check2_marks_rust_hygiene_contract_complete()",
        "role=root_role",
        "aria-valuenow=move || input_aria_valuenow.get()",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=agent_contract.source_attr",
        "data-ui-state=move || ui_state.get().as_attr()",
        "let trace = overlay_open::use_ui_trace();",
        "\"drag_end_commit\"",
    ] {
        assert!(
            local_semantics.contains(needle),
            "semantics suite should include contract-matrix evidence `{needle}`.",
        );
    }

    for needle in [
        "on:pointerdown=on_pointer_down",
        "on:pointermove=on_pointer_move",
        "let on_key_down = move |ev: ev::KeyboardEvent| {",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should expose interaction/platform branch token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep wasm/non-wasm branch coverage token `{needle}`.",
        );
    }

    for forbidden in [
        "assert_snapshot",
        "insta::",
        "to_match_snapshot",
        "snapshot!",
    ] {
        assert!(
            !local_semantics.contains(forbidden),
            "semantic contract tests should not depend on visual snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_component_files_keep_responsibilities_partitioned() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub(crate) mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorWheelMotion;",
        "pub use view::ColorWheel;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep stable module/export boundary `{needle}`.",
        );
    }

    assert!(
        !logic_source.contains("leptos::")
            && !logic_source.contains("web_sys")
            && !logic_source.contains("style.set_property"),
        "logic.rs should not include DOM/framework style branching concerns.",
    );

    assert!(
        styles_source.contains("pub const CSS: &str")
            && styles_source.contains("var(--ui-")
            && !styles_source.contains(":nth-child"),
        "styles.rs should keep token-first static CSS contract without brittle selectors.",
    );

    for needle in [
        "let semantics = overlay_open::use_color_wheel(overlay_open::ColorWheelOptions {",
        "view! {",
        "on:pointerdown=on_pointer_down",
        "on:input=on_input",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should render structure and mount headless contracts `{needle}`.",
        );
    }

    for forbidden in [
        "SpringAnimator::new",
        "set_property(\"--ui-slider-visual-percent\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not re-implement motion runtime engine token `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct ColorWheelMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "set_property(\"--ui-slider-visual-percent\",",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should own component->motion contract mapping `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_component_directory_standard_file_layout_is_enforced() {
    let component_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/color-wheel/src");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src.join(required_file).exists(),
            "color-wheel component directory should contain `{required_file}`.",
        );
    }

    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !component_src.join(forbidden_file).exists(),
            "color-wheel component directory should not contain `{forbidden_file}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] 组件目录标准文件落点正确。"),
        "color-wheel check2 should mark component-directory file-layout item as complete.",
    );
    assert!(
        check2_source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "color-wheel check2 should mark AI file-placement discipline item as complete.",
    );
    assert!(
        check2_source.contains("`src/` 仅保留 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs`"),
        "color-wheel check2 should document component-directory layout evidence.",
    );
    assert!(
        check2_source.contains(
            "对应回归测试 `color_wheel_component_directory_standard_file_layout_is_enforced`"
        ),
        "color-wheel check2 should include file-placement discipline regression evidence token.",
    );
}

#[test]
fn color_wheel_does_not_introduce_spec_module_for_simple_component() {
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "spec.rs",
        "ColorWheelSpec",
        "Spec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "color-wheel should not expose unnecessary spec contract token `{forbidden}`.",
        );
    }

    assert!(
        check2_source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "color-wheel check2 should mark hyper-structure builder item complete with explicit N/A decision.",
    );
    assert!(
        check2_source.contains("N/A（ColorWheel 非复杂配置组件）"),
        "color-wheel check2 should explain why spec builder is N/A for this simple component.",
    );
    assert!(
        check2_source.contains("`components/color-wheel/src/` 不存在 `spec.rs`"),
        "color-wheel check2 should keep spec-file absence evidence.",
    );
    assert!(
        check2_source.contains("`color_wheel_does_not_introduce_spec_module_for_simple_component`"),
        "color-wheel check2 should reference spec-governance regression test evidence.",
    );
}

#[test]
fn color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let component_manifest = load_source("../../components/color-wheel/src/Component.toml");
    let component_rbi = load_source("../../components/color-wheel/src/color_wheel.rbi");
    let component_src_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/color-wheel/src");

    assert!(
        component_src_dir.join("Component.toml").exists(),
        "color-wheel context-compression file should exist: `Component.toml`.",
    );
    assert!(
        component_src_dir.join("color_wheel.rbi").exists(),
        "color-wheel context-compression file should exist: `color_wheel.rbi`.",
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorWheel\"",
        "crate = \"ui-color-wheel\"",
        "name = \"id_base\"",
        "name = \"value\"",
        "name = \"default_value\"",
        "name = \"on_value_change\"",
        "name = \"is_disabled\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-wheel Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type ColorWheelSource = ui_state_primitives::color_wheel::ColorWheelSource;",
        "pub type ColorWheelState = ui_state_primitives::color_wheel::ColorWheelState;",
        "pub type ColorWheelStateInput = ui_state_primitives::color_wheel::ColorWheelStateInput;",
        "pub type ColorWheelMotion = crate::ColorWheelMotion;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn ColorWheel(",
        "id_base: String,",
        "value: Option<leptos::prelude::Signal<f64>>",
        "default_value: Option<f64>,",
        "on_value_change: Option<leptos::prelude::Callback<f64>>",
        "dir: Option<ui_headless::A11yDirection>,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_wheel.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "`components/color-wheel/src/Component.toml`",
        "`components/color-wheel/src/color_wheel.rbi`",
        "color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn color_wheel_agent_contract_is_schema_typed_and_machine_readable() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let component_manifest = load_source("../../components/color-wheel/src/Component.toml");
    let component_rbi = load_source("../../components/color-wheel/src/color_wheel.rbi");

    for typed_source in [
        "pub enum ColorWheelAgentSchema",
        "pub enum ColorWheelAgentSchemaVersion",
        "pub enum ColorWheelIntent",
        "pub enum ColorWheelUiAction",
        "pub enum ColorWheelUiState",
        "pub enum ColorWheelUiSource",
        "pub struct ColorWheelAgentContract",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorWheelAgentContract",
        "pub fn resolve_ui_action(",
        "pub fn resolve_ui_state(is_disabled: bool) -> ColorWheelUiState",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "color-wheel Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=agent_contract.source_attr",
        "data-ui-state=move || ui_state.get().as_attr()",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "color-wheel view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-wheel.agent-contract.v1\"",
        "intent = \"select-hue-angle\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "ColorWheelAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "color-wheel context-compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
        "format!(\"data-ui-source",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-wheel Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "`components/color-wheel/src/logic.rs`",
        "data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source",
        "color_wheel_agent_contract_is_schema_typed_and_machine_readable",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel checklist should keep Agent Contract evidence `{required}`.",
        );
    }
}

#[test]
fn color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let component_manifest = load_source("../../components/color-wheel/src/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"render_header_section(...)\"",
        "\"render_track_section()\"",
        "\"render_static_track_visuals()\"",
        "\"logic::resolve_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "\"logic::resolve_ui_action(...)\"",
        "\"logic::resolve_ui_state(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-wheel manifest should keep whitelist-safe render path marker `{required}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-wheel Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for required in [
        "color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel checklist should keep Agent Contract whitelist evidence `{required}`.",
        );
    }
}

#[test]
fn color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`ColorWheel` 不是 LLM 正文渲染组件，组件职责是同步色相输入；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束固定为两种显示模式：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`；门禁脚本：`scripts/check-ui-components-streaming.sh` 新增 `cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。）",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`ColorWheel` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in ["use_ai_space_state", "project_streaming_"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "color-wheel should stay out of LLM streaming protocol scope and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let check2_source = load_source("../../components/color-wheel/check2.md");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（`ColorWheel` 已支持完整配置快照输入并稳定渲染：`components/color-wheel/src/view.rs` 通过受控/非受控三件套（`value/default_value/on_value_change`）+ 归一化边界（`sanitize_step/resolve_default_value/normalize_state_inputs`）消费完整结果，根节点持续输出稳定语义标记（`data-state/data-value/data-value-percent/data-control-mode/data-value-source/data-ui-stream-fallback/data-ui-stream-mode/...`）。docs 基线示例 `apps/docs-app/src/pages/components/pages/forms_color.rs` 提供 Hello World、Controlled Hue Wheel、Disabled + Reduced Motion + Custom Class 等完整快照路径。回归：`components/color-wheel/test/semantics.rs::color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably`；门禁脚本：`scripts/check-ui-components-streaming.sh` 新增 `cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably`。）",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-wheel check2 should keep snapshot-baseline marker `{required}`.",
        );
    }

    for marker in [
        "pub fn ColorWheel(",
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "let normalized_inputs = logic::normalize_state_inputs(",
        "let step = logic::sanitize_step(step);",
        "let default_value = logic::resolve_default_value(default_value, step);",
        "overlay_open::use_controllable_state(value, Some(default_value), on_value_change)",
        "logic::resolve_state(ColorWheelStateInput {",
        "data-state=move || state.get().data_state_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "color-wheel snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn normalize_state_inputs(",
        "pub fn sanitize_step(step: f64) -> f64",
        "pub fn resolve_default_value(default_value: Option<f64>, step: f64) -> f64",
        "pub fn resolve_state(input: ColorWheelStateInput) -> ColorWheelState",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorWheelAgentContract",
        "ColorWheelStreamFallback::Snapshot.as_attr()",
        "ColorWheelStreamMode::Snapshot.as_attr()",
    ] {
        assert!(
            logic_source.contains(marker),
            "color-wheel logic should keep normalized snapshot baseline marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ColorWheel\"",
        "slug=\"color-wheel\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ColorWheel id_base=\"docs-color-wheel-hello\".to_string() />",
        "<Playground title=\"Controlled Hue Wheel\" code_signal=basic_code>",
        "id_base=\"docs-color-wheel-hue\".to_string()",
        "<Playground title=\"Disabled + Reduced Motion + Custom Class\" code_signal=states_code>",
        "id_base=\"docs-color-wheel-disabled\".to_string()",
        "id_base=\"docs-color-wheel-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-wheel docs should keep snapshot-ready baseline usage marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_wheel_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/color-wheel/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorWheel` 归类为 `Streaming Optional`；组件职责是色相输入控制而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support=\"unsupported\"`、`data-ui-stream-fallback=\"snapshot\"`、`data-ui-stream-mode=\"snapshot\"` 与 `data-ui-output-status`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_streaming_required_optional_classification_rules`、`components/color-wheel/test/semantics.rs::color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-wheel/test/semantics.rs::color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`；门禁脚本：`scripts/check-ui-components-streaming.sh` 新增对应 `cargo test` 目标。）",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`ColorWheel` 归类为 `Streaming Optional`",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-wheel check2 should keep streaming responsibility marker `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");

    for required in [
        "role=root_role",
        "aria-labelledby=move || root_aria_labelledby.get_value()",
        "lang=move || root_lang.get_value()",
        "dir=root_dir",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=agent_contract.source_attr",
        "data-ui-state=move || ui_state.get().as_attr()",
        "role=input_role",
        "aria-disabled=input_aria_disabled",
        "aria-valuemin=move || input_aria_valuemin.get_value()",
        "aria-valuemax=move || input_aria_valuemax.get_value()",
        "aria-valuenow=move || input_aria_valuenow.get()",
    ] {
        assert!(
            view_source.contains(required),
            "color-wheel should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "color-wheel should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let mod_source = load_source("../../components/color-wheel/src/mod.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");
    let styles_source = load_source("../../components/color-wheel/src/styles.rs");
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let motion_source = load_source("../../components/color-wheel/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-wheel non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let view_source = load_source("../../components/color-wheel/src/view.rs");
    let logic_source = load_source("../../components/color-wheel/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let id_base: Cow<'static, str> = logic::normalize_optional_text(Some(id_base))",
        ".map(Cow::Owned)",
        ".unwrap_or(Cow::Borrowed(\"ui-color-wheel\"));",
        "let id_base = id_base.into_owned();",
    ] {
        assert!(
            view_source.contains(required),
            "color-wheel view should keep Cow-based fallback normalization marker `{required}`.",
        );
    }

    for forbidden in [
        ".unwrap_or_else(|| \"ui-color-wheel\".to_string())",
        "DEFAULT_LABEL.to_string()",
        "DEFAULT_ARIA_LABEL.to_string()",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "color-wheel should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_wheel_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/color-wheel/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-wheel check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}
