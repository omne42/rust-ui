use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn radio_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/radio/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Radio internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn radio_group_uses_headless_roving_and_interaction_hooks() {
    let source = load_source("src/radio/view.rs");
    let headless_radio_source = load_source("../ui-headless/src/radio.rs");

    for needle in ["use_radio", "use_focus_ring", "use_hover", "use_press"] {
        assert!(
            source.contains(needle),
            "RadioGroup should use headless `{needle}` hooks."
        );
    }

    for needle in [
        "pub struct RadioAttrs",
        "pub struct RadioState",
        "pub struct RadioContract",
        "pub fn use_radio(options: RadioOptions) -> RadioContract",
        "locale_attrs(lang, dir)",
    ] {
        assert!(
            headless_radio_source.contains(needle),
            "Headless radio contract should provide typed attrs/handlers/state with locale support; missing `{needle}`.",
        );
    }
}

#[test]
fn radio_group_consumes_state_primitives_model() {
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/radio.rs");

    for needle in [
        "pub use ui_state_primitives::radio::{",
        "normalize_optional_text",
        "resolve_accessible_name",
        "resolve_state",
        "pub fn roving_orientation(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Radio logic should stay as a thin bridge to primitives; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct RadioGroupState",
        "pub fn resolve_state(",
        "pub item_count: usize",
        "pub has_disabled_options: bool",
        "pub selected_index: Option<usize>",
        "pub has_selection: bool",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Radio state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "logic::resolve_state(",
        "logic::normalize_optional_text(",
        "logic::resolve_accessible_name(",
        "logic::roving_orientation(orientation)",
        "aria.state.selected_index.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "RadioGroup view should consume bridge outputs from logic/primitives; missing `{needle}`."
        );
    }
}

#[test]
fn radio_group_supports_accessible_name_resolution() {
    let view_source = load_source("src/radio/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/radio.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_labelledby: Option<String>",
        "resolve_accessible_name",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "RadioGroup should wire `{needle}` for baseline-style accessible naming."
        );
    }

    assert!(
        primitive_source.contains("pub const DEFAULT_ARIA_LABEL: &str = \"Radio group\";"),
        "Radio primitive should provide a fallback accessible label when no labels are supplied."
    );
}

#[test]
fn radio_check2_marks_status_primitives_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "ui-state-primitives/src/radio.rs",
        "radio/logic.rs` 仅通过 `pub use ui_state_primitives::radio::{...}`",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record status-primitives completion evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_ui_headless_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] `ui-headless` 定义",
        "crates/ui-headless/src/radio.rs",
        "use_radio(RadioOptions {",
        "lang / dir",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record ui-headless completion evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_ui_motion_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] `ui-motion` 定义",
        "radio/motion.rs",
        "ui_motion::spring::SpringAnimator",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record ui-motion completion evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_group_exposes_state_and_orientation_data_attributes() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "data-slot=\"radio-group\"",
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
        "data-slot=\"radio\"",
        "data-index=index",
        "data-active=move || (aria.state.active_index.get() == index).then_some(\"true\")",
        "data-checked",
        "data-checked-control-mode=checked_control_mode_attr",
        "data-default-checked-source=default_checked_source_attr",
        "data-focus-visible",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-stream-mode=\"snapshot\"",
        "data-stream-fallback=\"snapshot\"",
        "data-output-status=\"verified\"",
        "data-ui-schema=\"ui.radio-group\"",
        "data-ui-schema-version=\"1\"",
        "data-ui-intent=\"single-selection\"",
        "data-ui-action=\"select-option\"",
        "data-ui-state=move || if state.get().has_selection { \"has-selection\" } else { \"empty-selection\" }",
        "data-ui-schema=\"ui.radio\"",
        "data-ui-intent=\"single-option\"",
        "data-ui-action=\"toggle-request\"",
        "data-ui-state=move || if checked.get() { \"checked\" } else { \"unchecked\" }",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should expose `{needle}` for baseline-style state styling and inspection."
        );
    }
}

#[test]
fn radio_group_sets_aria_orientation_and_option_label_fallback() {
    let source = load_source("src/radio/view.rs");

    for needle in [
        "aria-orientation=orientation.aria_orientation()",
        "format!(\"Option {}\", index + 1)",
    ] {
        assert!(
            source.contains(needle),
            "RadioGroup should keep `{needle}` for robust ARIA semantics and predictable labels."
        );
    }
}

#[test]
fn radio_attaches_motion_driver() {
    let source = load_source("src/radio/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Radio should attach motion via `radio::motion::attach_motion`."
    );
}

#[test]
fn radio_styles_include_motion_marker_contracts() {
    let source = load_source("src/radio/styles.rs");

    for selector in [
        ".ui-radio[data-motion-source=\"custom\"]",
        ".ui-radio[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Radio styles should include `{selector}` as stable custom-motion selectors."
        );
    }

    for token_var in [
        "var(--ui-font-size-100)",
        "var(--ui-button-focus-outline-width)",
        "var(--ui-button-focus-outline-offset)",
        "var(--ui-icon-size-100)",
        "var(--ui-button-radius-full)",
        "var(--ui-space-2xs)",
    ] {
        assert!(
            source.contains(token_var),
            "Radio styles should consume theme tokens via `{token_var}`."
        );
    }
}

#[test]
fn radio_motion_uses_spring_animator() {
    let source = load_source("src/radio/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Radio motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn radio_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/radio/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: RadioMotion) -> RadioMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "Radio motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn radio_component_files_follow_expected_layout_and_no_spec_file() {
    let mod_source = load_source("src/radio/mod.rs");

    assert!(
        mod_source.contains("mod logic;"),
        "radio module should include private logic.rs."
    );
    assert!(
        mod_source.contains("pub mod styles;"),
        "radio module should expose styles.rs for css aggregation."
    );
    assert!(
        mod_source.contains("pub mod motion;"),
        "radio module should expose motion.rs contract."
    );
    assert!(
        mod_source.contains("mod view;"),
        "radio module should keep view.rs private implementation detail."
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/radio/spec.rs");
    assert!(
        !spec_path.exists(),
        "radio should not introduce spec.rs without schema-level need."
    );
}

#[test]
fn radio_view_does_not_use_inner_html() {
    let source = load_source("src/radio/view.rs");
    assert!(
        !source.contains("inner_html"),
        "radio view must not inject inner_html for untrusted content safety."
    );
}

#[test]
fn radio_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn radio_group() -> AnyView",
        "title=\"RadioGroup\"",
        "slug=\"radio-group\"",
        "description=\"Roving tabindex radiogroup with baseline-level spring motion and baseline-style root state attrs.\"",
        "<Playground title=\"Hello World（默认路径）\" code_signal=code>",
        "<Playground title=\"Interactive Matrix（方向/禁用/状态）\" code_signal=matrix_code>",
        "pub(super) fn radio() -> AnyView",
        "title=\"Radio\"",
        "slug=\"radio\"",
        "<Playground title=\"Hello World（默认路径）\" code_signal=code>",
        "<Playground title=\"状态矩阵（受控 + disabled）\" code_signal=matrix_code>",
        "<RadioGroup",
        "<Radio",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for radio primary playground coverage.",
        );
    }
}

#[test]
fn radio_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Hello World（默认路径）\"",
        "id_base=\"docs-radio-group\".to_string()",
        "label=\"Size\".to_string()",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "title=\"Interactive Matrix（方向/禁用/状态）\"",
        "set_billing_is_horizontal.update(|value| *value = !*value)",
        "set_billing_group_disabled.update(|value| *value = !*value)",
        "set_billing_disable_middle.update(|value| *value = !*value)",
        "class=\"ui-button\"",
        "id_base=\"docs-radio-group-billing\".to_string()",
        "orientation=orientation",
        "is_disabled=is_disabled",
        "disabled_indices=disabled_indices",
        "aria_labelledby=external_label_id.clone()",
        "selected_index=billing_selected",
        "set_selected_index=set_billing_selected",
        "id_base=\"docs-radio-group-empty\".to_string()",
        "options=empty_options",
        "is_disabled=true",
        "aria_label=\"No options available\".to_string()",
        "selected_index=empty_selected",
        "set_selected_index=set_empty_selected",
        "title=\"状态矩阵（受控 + disabled）\"",
        "id=\"docs-radio\".to_string()",
        "id=\"docs-radio-controlled\".to_string()",
        "id=\"docs-radio-disabled-on\".to_string()",
        "id=\"docs-radio-disabled-off\".to_string()",
        "id=\"docs-radio-uncontrolled-default\".to_string()",
        "label=\"Controlled\".to_string()",
        "is_checked=Signal::derive(move || checked.get())",
        "default_checked=true",
        "on_checked_change=on_checked_change",
    ] {
        assert!(
            source.contains(needle),
            "forms docs playgrounds should contain `{needle}` for radio contracts.",
        );
    }
}

#[test]
fn radio_checked_axis_is_normalized_in_logic_and_consumed_by_view() {
    let logic_source = load_source("src/radio/logic.rs");
    let view_source = load_source("src/radio/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/radio.rs");

    for needle in [
        "pub struct CheckedAxisInput",
        "pub struct CheckedAxisState",
        "pub fn normalize_checked_axis(",
        "default_checked: input.default_checked.unwrap_or(DEFAULT_CHECKED)",
        "resolve_checked_axis(PrimitiveRadioCheckedAxisInput {",
        "control_mode_attr",
        "default_checked_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Radio logic should normalize checked axis centrally; missing `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_CHECKED: bool = false;",
        "pub struct RadioCheckedAxisInput",
        "pub struct RadioCheckedAxisState",
        "pub fn resolve_checked_axis(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Radio checked-axis primitive should live in ui-state-primitives; missing `{needle}`."
        );
    }

    for needle in [
        "let checked_axis = logic::normalize_checked_axis(",
        "let controllable_checked = use_controllable_state(",
        "Some(checked_axis.default_checked)",
        "checked_axis.control_mode_attr",
        "request_checked_change.run(!checked.get_untracked());",
    ] {
        assert!(
            view_source.contains(needle),
            "Radio view should consume normalized checked axis without rebuilding state rules; missing `{needle}`."
        );
    }
}

#[test]
fn radio_docs_include_beginner_friendly_readme() {
    let source = load_source("src/radio/README.md");

    for needle in [
        "# Radio / RadioGroup",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "## docs-app 入口",
        "## Source-first Copy-Paste Ready",
        "apps/docs-app/src/pages/components/pages/forms.rs",
        "crates/ui-components/src/radio/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "Radio README should stay beginner-friendly and source-first; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_docs_playground_and_copy_ready_contract_complete() {
    let source = load_source("src/radio/check2.md");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 组件文档必须对新手友好（Documentation as Product）",
        "- [x] `apps/docs-app` 必须提供 Interactive Playground",
        "- [x] Source-first 文档必须 Copy-Paste Ready",
        "- [x] HeroUI 对标文档与组件文档同步",
        "apps/docs-app/src/pages/components/pages/forms.rs",
        "crates/ui-components/src/radio/README.md",
        "apps/docs-app/src/playground.rs",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record docs/playground/copy-ready evidence; missing `{needle}`."
        );
    }

    for needle in [
        "compose_copy_ready_code(",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_signal",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should keep `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_architecture_anti_pattern_guards_complete() {
    let check2 = load_source("src/radio/check2.md");
    let primitive = load_source("../ui-state-primitives/src/radio.rs");
    let headless = load_source("../ui-headless/src/radio.rs");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 公共 API 泄露底层实现细节类型。",
    ] {
        assert!(
            check2.contains(needle),
            "Radio check2 should record anti-pattern guard evidence; missing `{needle}`."
        );
    }

    for forbidden in ["web_sys", "leptos", "class=", ".ui-", ":hover"] {
        assert!(
            !primitive.contains(forbidden),
            "radio state primitive must stay platform/style agnostic; found forbidden token `{forbidden}`."
        );
    }

    for forbidden in ["class=", ".ui-", "SpringAnimator", "attach_motion"] {
        assert!(
            !headless.contains(forbidden),
            "radio headless contract must not include visual/motion orchestration; found `{forbidden}`."
        );
    }
}

#[test]
fn radio_check2_marks_state_management_contract_complete() {
    let check2 = load_source("src/radio/check2.md");
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");

    for needle in [
        "- [x] 受控/非受控必须成对",
        "- [x] 默认值单一来源",
        "- [x] 状态归一化集中",
        "- [x] 离散状态必须类型约束",
        "- [x] 状态原语来源正确",
        "- [x] 如果无异步相关，直接打勾。",
        "- [x] API 易用性验收标准（DX Paradox）",
        "- [x] 组合型组件主 API 必须“显示优于约定”",
        "N/A：`radio` 无远程请求与异步状态轴",
    ] {
        assert!(
            check2.contains(needle),
            "Radio check2 should record state-management completion evidence; missing `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] default_checked: Option<bool>",
        "data-checked-control-mode=checked_control_mode_attr",
        "data-default-checked-source=default_checked_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Radio view should expose controlled/uncontrolled semantic markers; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct CheckedAxisInput",
        "pub fn normalize_checked_axis(",
        "control_mode_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Radio logic should centralize checked-axis normalization; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_a11y_observability_and_file_layout_complete() {
    let check2 = load_source("src/radio/check2.md");
    let view_source = load_source("src/radio/view.rs");
    let styles_source = load_source("src/radio/styles.rs");
    let motion_source = load_source("src/radio/motion.rs");
    let mod_source = load_source("src/radio/mod.rs");

    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "- [x] 状态可观测、可检索、可验证",
        "- [x] 样式依赖显式状态（`data-*`/class）",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] 组件文件职责正确",
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "- [x] 组件层遵循 token-first 静态样式契约",
        "- [x] 类型系统 + 语义标记共同提供机器可读状态",
    ] {
        assert!(
            check2.contains(needle),
            "Radio check2 should record implementation-layer evidence; missing `{needle}`."
        );
    }

    for needle in [
        "role=aria.attrs.role",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
        "data-ui-schema=\"ui.radio-group\"",
        "data-ui-schema=\"ui.radio\"",
        "data-ui-source=checked_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Radio view should expose A11y + machine-readable markers; missing `{needle}`."
        );
    }

    for needle in [
        ".ui-radio[data-checked=\\\"true\\\"] .ui-radio__indicator",
        ".ui-radio[data-motion-source=\"custom\"]",
        ".ui-radio[data-custom-motion=\"true\"]",
        "var(--ui-icon-size-100)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Radio styles should stay token-first and state-explicit; missing `{needle}`."
        );
    }

    assert!(
        motion_source.contains("ui_motion::web::prefers_reduced_motion()"),
        "Radio motion should respect reduced-motion contract."
    );

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Radio module layout should keep expected file responsibility boundary; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_platform_and_tree_shaking_contract_complete() {
    let check2 = load_source("src/radio/check2.md");
    let motion_source = load_source("src/radio/motion.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let motion_lib = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "- [x] Tree Shaking 是一等能力",
        "--features component-radio,inject-css --depth 2",
        "- [x] SSR 与跨平台检查",
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护",
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub",
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
    ] {
        assert!(
            check2.contains(needle),
            "Radio check2 should record platform/tree-shaking evidence; missing `{needle}`."
        );
    }

    assert!(
        headless_lib.contains("compile_error!(\"features `web` and `ssr` are mutually exclusive"),
        "ui-headless must keep compile_error guard for web/ssr mutual exclusion."
    );
    assert!(
        motion_lib.contains("pub mod web {"),
        "ui-motion should expose platform-safe web module stubs."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "radio motion should cover wasm/non-wasm/reduced-motion branches; missing `{needle}`."
        );
    }
}

#[test]
fn radio_public_api_does_not_expose_dom_node_ref_prop() {
    let source = load_source("src/radio/view.rs");
    assert!(
        !source.contains("#[prop(optional)] node_ref: NodeRef<html::Button>"),
        "Radio public API should not expose DOM-specific `NodeRef<html::Button>` props."
    );
}

#[test]
fn radio_public_api_uses_prefixed_names_with_legacy_alias_compatibility() {
    let view_source = load_source("src/radio/view.rs");
    let logic_source = load_source("src/radio/logic.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, into)] is_checked: Option<Signal<bool>>",
        "#[prop(optional, into)] checked: Option<Signal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] on_checked_change: Option<Callback<bool>>",
        "#[prop(optional)] on_change: Option<Callback<bool>>",
        "data-checked-control-mode=checked_control_mode_attr",
        "data-checked-controlled=is_checked_controlled.then_some(\"true\")",
        "data-checked-uncontrolled=(!is_checked_controlled).then_some(\"true\")",
        "data-disabled-source=disabled_source_attr",
        "data-checked-source=checked_source_attr",
        "data-default-checked-source=default_checked_source_attr",
        "data-checked-change-source=checked_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Radio view API should expose `{needle}` for naming-contract compliance."
        );
    }

    for needle in [
        "normalize_disabled_prop",
        "normalize_checked_axis",
        "resolve_checked_axis",
        "default_checked",
        "control_mode_attr",
        "checked_change_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Radio logic should centralize naming compatibility via `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_streaming_and_snapshot_contract_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染",
        "`Streaming Optional`",
        "fallback=snapshot",
        "- [x] `Snapshot` 是所有组件的基础能力",
        "data-output-status=\"verified\"",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record streaming/snapshot contract evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_semantics_and_e2e_regression_contract_complete() {
    let source = load_source("src/radio/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_radio.spec.mjs");

    for needle in [
        "- [x] 语义测试优先",
        "radio_semantics.rs",
        "- [x] E2E 选择器稳定",
        "- [x] 关键流程纳入可重复回归集合",
        "docs_app_radio.spec.mjs",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record semantics/e2e evidence; missing `{needle}`."
        );
    }

    for needle in [
        "[data-slot=\"radio-group\"][role=\"radiogroup\"]",
        "toHaveAttribute(\"data-selected-index\", \"2\")",
        "await page.keyboard.press(\"ArrowDown\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Radio e2e should keep stable semantic selector and repeatable flow marker `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_ui_theme_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] `ui-theme` 定义",
        "radio/styles.rs",
        "var(--ui-font-size-100)",
        "var(--ui-icon-size-100)",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record ui-theme completion evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_api_naming_contract_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] API 命名契约统一",
        "is_disabled",
        "is_checked",
        "on_checked_change",
        "docs 示例默认使用新名",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record API naming completion evidence; missing `{needle}`."
        );
    }
}

#[test]
fn radio_check2_marks_ui_components_complete() {
    let source = load_source("src/radio/check2.md");

    for needle in [
        "- [x] `ui-components` 定义",
        "mod.rs / logic.rs / styles.rs / view.rs / motion.rs",
        "use ui_state_primitives::radio",
        "use_radio(RadioOptions {",
        "Radio` 公共 API 移除 `node_ref",
    ] {
        assert!(
            source.contains(needle),
            "Radio check2 should record ui-components completion evidence; missing `{needle}`."
        );
    }
}
