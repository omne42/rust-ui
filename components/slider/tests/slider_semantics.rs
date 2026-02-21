use std::fs;
use std::path::{Path, PathBuf};

fn resolve_workspace_dir(manifest_dir: &Path) -> PathBuf {
    if let Ok(path) = std::env::var("OMNE_WORKSPACE_DIR") {
        return PathBuf::from(path);
    }

    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
}

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = resolve_workspace_dir(manifest_dir);

    let path = if let Some(suffix) = rel_path.strip_prefix("src/slider/") {
        manifest_dir.join("src").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("src/") {
        workspace_dir.join("crates/ui-components/src").join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui-components/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui-components/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui-components/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-state-primitives/") {
        workspace_dir
            .join("crates/ui-state-primitives")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-headless/") {
        workspace_dir.join("crates/ui-headless").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-motion/") {
        workspace_dir.join("crates/ui-motion").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-visual-primitive/") {
        workspace_dir
            .join("crates/ui-visual-primitive")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = resolve_workspace_dir(manifest_dir);

    if let Some(suffix) = rel_path.strip_prefix("src/slider/") {
        manifest_dir.join("src").join(suffix).exists()
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix).exists()
    } else {
        manifest_dir.join(rel_path).exists()
    }
}

#[test]
fn slider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/slider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Slider internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn slider_module_reexports_state_primitive_contracts() {
    let source = load_source("src/slider/mod.rs");

    assert!(
        source.contains(
            "pub use ui_state_primitives::slider::{SliderPhase, SliderState, SliderStateInput};"
        ),
        "Slider module should expose state contracts from ui-state-primitives."
    );
}

#[test]
fn slider_logic_only_normalizes_inputs_and_source_markers() {
    let source = load_source("src/slider/logic.rs");

    for needle in [
        "use ui_state_primitives::slider as slider_state;",
        "pub enum SliderControlMode",
        "pub enum SliderValueSource",
        "pub enum SliderValueChangeSource",
        "pub enum SliderDisabledSource",
        "pub struct ValueAxisInput",
        "pub struct ValueAxisState",
        "pub fn normalize_default_value(default_value: Option<f64>) -> f64",
        "pub fn normalize_on_value_change_handler(",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub struct AccessibilityStateInput",
        "pub fn normalize_accessibility_state(",
        "pub struct IdState",
        "pub fn normalize_id(id: String) -> IdState",
        "pub enum SliderAgentSchema",
        "pub enum SliderStreamSupport",
        "pub enum SliderStreamFallback",
        "pub enum SliderStreamMode",
        "pub enum SliderOutputStatus",
        "pub enum SliderIntent",
        "pub enum SliderUiAction",
        "pub struct SliderAgentContract",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> SliderAgentContract",
        "pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> SliderUiAction",
    ] {
        assert!(
            source.contains(needle),
            "Slider logic should include `{needle}` for centralized normalization contracts."
        );
    }

    for forbidden in ["web_sys", "event_target_value", "on:input", "NodeRef<"] {
        assert!(
            !source.contains(forbidden),
            "Slider logic should avoid view/DOM concerns; found `{forbidden}`."
        );
    }
}

#[test]
fn slider_view_mounts_headless_contract_without_state_machine_reimplementation() {
    let source = load_source("src/slider/view.rs");

    for needle in [
        "use ui_headless::{",
        "use_slider",
        "A11yDirection",
        "SliderOptions",
        "logic::normalize_id(id.unwrap_or_default())",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "use_slider(SliderOptions {",
        "logic::resolve_state(SliderStateInput {",
        "motion::attach_motion(root_ref, visual_percent, motion)",
        "slider_aria.handlers.on_input.run(event_target_value(&ev));",
        "logic::resolve_ui_action(",
        "data-ui-action=move || ui_action.get().as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "Slider view should compose logic+headless contracts via `{needle}`."
        );
    }

    for forbidden in [
        "logic::parse_value(&event_target_value(&ev))",
        "logic::sanitize_value(parsed, min, max, step)",
        "if id.is_empty()",
        "if disabled {",
        "data-ui-action=move || ui_action.get(),",
    ] {
        assert!(
            !source.contains(forbidden),
            "Slider view should not reimplement normalization/state-machine rule `{forbidden}`."
        );
    }
}

#[test]
fn slider_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "Slider should keep explicit render blocks in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        5,
        "Slider should keep one main render block and four semantic subrender blocks."
    );
    assert!(
        view_source.lines().count() <= 320,
        "Slider view.rs should stay bounded; split further if this grows significantly."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        ".fold(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Slider view should avoid expansion-heavy rendering token `{forbidden}`."
        );
    }

    for needle in [
        "fn render_label(",
        "struct SliderInputRenderInput {",
        "fn render_input(input: SliderInputRenderInput) -> impl IntoView",
        "fn render_track() -> impl IntoView",
        "fn render_control(input: SliderInputRenderInput) -> impl IntoView",
        "let label_view = render_label(",
        "let control_view = render_control(",
        "{label_view}",
        "{control_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider view should keep semantic subblock marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn slider_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Slider should keep a single public component boundary for current layout."
    );

    for needle in [
        "fn render_label(",
        "fn render_input(input: SliderInputRenderInput) -> impl IntoView",
        "fn render_track() -> impl IntoView",
        "fn render_control(input: SliderInputRenderInput) -> impl IntoView",
        ") -> impl IntoView {",
        "pub fn Slider(",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider view should prefer plain function split marker `{needle}`."
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\nfn slider_"] {
        assert!(
            !view_source.contains(forbidden),
            "Slider should not introduce local component abstraction noise `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn slider_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/slider/check2.md");

    for needle in [
        "const SLOT_SLIDER: &str = \"slider\";",
        "const SLOT_SLIDER_LABEL: &str = \"slider-label\";",
        "const SLOT_SLIDER_CONTROL: &str = \"slider-control\";",
        "const SLOT_SLIDER_INPUT: &str = \"slider-input\";",
        "const SLOT_SLIDER_TRACK: &str = \"slider-track\";",
        "const SLOT_SLIDER_FILL: &str = \"slider-fill\";",
        "const SLOT_SLIDER_THUMB: &str = \"slider-thumb\";",
        "const CLASS_SLIDER_LABEL: &str = \"ui-slider__label\";",
        "const CLASS_SLIDER_CONTROL: &str = \"ui-slider__control\";",
        "const CLASS_SLIDER_INPUT: &str = \"ui-slider__input\";",
        "const CLASS_SLIDER_TRACK: &str = \"ui-slider__track\";",
        "const CLASS_SLIDER_FILL: &str = \"ui-slider__fill\";",
        "const CLASS_SLIDER_THUMB: &str = \"ui-slider__thumb\";",
        "const BOOL_TRUE: &str = \"true\";",
        "const INPUT_TYPE_RANGE: &str = \"range\";",
        "fn render_track() -> impl IntoView",
        "aria-hidden=BOOL_TRUE",
        "type=INPUT_TYPE_RANGE",
        "role=slider_aria.input.role",
        "aria-label=move || label.with_value(|label| label.clone())",
        "data-slot=SLOT_SLIDER_TRACK",
        "data-slot=SLOT_SLIDER_FILL",
        "data-slot=SLOT_SLIDER_THUMB",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider static-fragment contract should include `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("ui-slider__track").count(),
        1,
        "Static track class token should be centralized once via constant definition."
    );
    assert_eq!(
        view_source.matches("slider-track").count(),
        1,
        "Static track slot token should be centralized once via constant definition."
    );

    let script_needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    assert!(
        check2_source.contains("静态片段常量化"),
        "Slider checklist should keep static fragment constantization gate item."
    );
}

#[test]
fn slider_public_api_keeps_prefixed_naming_and_legacy_compatibility() {
    let source = load_source("src/slider/view.rs");

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>,",
        "#[prop(optional)] default_value: Option<f64>,",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] set_value: Option<WriteSignal<f64>>,",
        "#[prop(optional)] on_change: Option<Callback<f64>>,",
    ] {
        assert!(
            source.contains(needle),
            "Slider API should include `{needle}` for naming consistency and migration compatibility."
        );
    }
}

#[test]
fn slider_exposes_machine_readable_state_and_source_markers() {
    let source = load_source("src/slider/view.rs");

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-pressed=move || slider_aria.state.is_pressed.get().then_some(\"true\")",
        "data-hovered=move || slider_aria.state.is_hovered.get().then_some(\"true\")",
        "data-focused=move || slider_aria.state.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some(\"true\")",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-id-source=id_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get()",
        "data-ui-source=value_change_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Slider view should expose semantic marker `{needle}`."
        );
    }
}

#[test]
fn slider_styles_are_token_first_and_marker_driven() {
    let source = load_source("src/slider/styles.rs");

    for needle in [
        "var(--ui-space-2xs, 4px)",
        "var(--ui-icon-size-100, 20px)",
        "var(--ui-space-xs, 6px)",
        "var(--ui-slider-max-width, 352px)",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-slider-thumb-border-width, 2px)",
        "var(--ui-slider-focus-ring-width, 2px)",
        ".ui-slider[data-state=\"disabled\"]",
        ".ui-slider[data-motion-source=\"custom\"]",
        ".ui-slider[data-label-source=\"custom\"]",
        ".ui-slider[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "Slider styles should include token-first marker contract `{needle}`."
        );
    }

    assert!(
        !source.contains(":nth-child"),
        "Slider styles should not depend on brittle DOM index selectors."
    );
}

#[test]
fn slider_motion_consumes_theme_tokens_and_shared_spring_sanitizer() {
    let source = load_source("src/slider/motion.rs");

    for needle in [
        "use ui_theme::default_slider_motion_tokens;",
        "let tokens = default_slider_motion_tokens();",
        "ui_motion::spring::sanitize_config(value, SliderMotion::default().spring)",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            source.contains(needle),
            "Slider motion should include `{needle}` for shared motion contract compliance."
        );
    }
}

#[test]
fn ui_motion_non_wasm_stub_contract_is_present_and_predictable() {
    let source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }
}

#[test]
fn slider_headless_contract_is_typed_and_lang_dir_aware() {
    let source = load_source("../ui-headless/src/slider.rs");

    for needle in [
        "pub struct SliderOptions",
        "pub struct SliderInputAttrs",
        "pub struct SliderHandlers",
        "pub struct SliderState",
        "pub struct SliderAria",
        "pub fn use_slider(options: SliderOptions) -> SliderAria",
        "locale_attrs(lang, dir)",
        "crate::use_controllable_state(value, default_value, on_value_change)",
        "role: \"slider\"",
        "aria_valuemin",
        "aria_valuemax",
        "aria_valuenow",
        "aria_valuetext",
    ] {
        assert!(
            source.contains(needle),
            "Slider headless contract should include `{needle}`."
        );
    }
}

#[test]
fn slider_docs_include_hello_world_and_controlled_matrix() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn slider() -> AnyView",
        "title=\"Slider\"",
        "slug=\"slider\"",
        "Hello World (Uncontrolled)",
        "Controlled + Source Markers",
        "default_value=36.0",
        "on_value_change=on_value_change",
        "use leptos::prelude::*;",
        "use ui_components::Slider;",
    ] {
        assert!(
            source.contains(needle),
            "slider docs should include `{needle}` for DX and source-marker coverage."
        );
    }
}

#[test]
fn slider_check2_documents_documentation_as_product_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep documentation-as-product rule `{required}`."
        );
    }
}

#[test]
fn slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/slider/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    assert!(
        has_readme || has_docs_page,
        "Slider must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn slider() -> AnyView"),
        "Equivalent docs entry should expose slider page function."
    );
}

#[test]
fn slider_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = load_source("src/slider/README.md");

    for needle in [
        "title=\"Slider\"",
        "slug=\"slider\"",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "<Playground title=\"Controlled + Source Markers\" code_signal=code>",
        "<Playground title=\"Disabled + Fine Step\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>")
        .expect("slider docs should include hello-world playground");
    let controlled_pos = docs_source
        .find("<Playground title=\"Controlled + Source Markers\" code_signal=code>")
        .expect("slider docs should include controlled playground");
    let advanced_pos = docs_source
        .find("<Playground title=\"Disabled + Fine Step\" code_signal=states_code>")
        .expect("slider docs should include advanced playground");
    assert!(
        hello_pos < controlled_pos && controlled_pos < advanced_pos,
        "Slider docs should present default usage before advanced controls."
    );

    let readme_hello_pos = readme_source
        .find("## Hello World")
        .expect("slider README should include hello-world section");
    let readme_controlled_pos = readme_source
        .find("## 受控用法")
        .expect("slider README should include controlled usage section");
    assert!(
        readme_hello_pos < readme_controlled_pos,
        "Slider README should present default path before controlled/advanced path."
    );
}

#[test]
fn slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = load_source("src/slider/README.md");

    for needle in [
        "use leptos::prelude::*;",
        "use ui_components::Slider;",
        "title=\"Hello World (Uncontrolled)\"",
        "default_value=36.0",
        "<Slider",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs hello-world should keep zero-threshold marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "use_slider(",
        "state=...",
        "logic::",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "Slider README hello-world path should avoid architecture-wiring token `{forbidden}`."
        );
    }
}

#[test]
fn slider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "label=\"Volume\".to_string()",
        "default_value=36.0",
        "title=\"Controlled + Source Markers\"",
        "id=\"docs-slider-volume\".to_string()",
        "value=controlled_value",
        "default_value=20.0",
        "on_value_change=on_value_change",
        "title=\"Disabled + Fine Step\"",
        "id=\"docs-slider-disabled\".to_string()",
        "is_disabled=true",
        "id=\"docs-slider-fine\".to_string()",
        "value=fine_value",
        "on_value_change=on_fine_value_change",
        "step=0.05",
        "motion=SliderMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra slider docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}

#[test]
fn slider_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn slider_docs_examples_sync_with_logic_api_names_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");

    slider_docs_include_hello_world_and_controlled_matrix();
    slider_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn slider() -> AnyView",
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"Controlled + Source Markers\"",
        "title=\"Disabled + Fine Step\"",
        "value=controlled_value",
        "default_value=20.0",
        "on_value_change=on_value_change",
        "is_disabled=true",
        "value=fine_value",
        "on_value_change=on_fine_value_change",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs should keep API/default/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = logic::DEFAULT_MIN)] min: f64",
        "#[prop(optional, default = logic::DEFAULT_MAX)] max: f64",
        "#[prop(optional, default = logic::DEFAULT_STEP)] step: f64",
        "pub fn normalize_default_value(default_value: Option<f64>) -> f64",
        "default_value.unwrap_or(DEFAULT_MIN)",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Slider public/default contract should keep `{needle}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_examples_sync_with_logic_api_names_and_state_matrix",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code/src/view.rs");
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");

    for needle in [
        "pub(super) fn slider() -> AnyView",
        "data-slot=\"slider-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<Snippet",
        "copyable=true",
        "class_name=\"docs-slider-source-copy\".to_string()",
        "data-slot=\"slider-source-paths\"",
        "\"components/slider/src/mod.rs\"",
        "\"components/slider/src/logic.rs\"",
        "\"components/slider/src/view.rs\"",
        "\"components/slider/src/styles.rs\"",
        "\"components/slider/src/motion.rs\"",
        "data-slot=\"slider-source-prerequisites\"",
        "\"component-slider\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs should keep copy-ready marker `{needle}`."
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
        "#[component]",
        "pub fn Code(",
        "data-slot=\"code\"",
        "state.has_custom_class_name",
    ] {
        assert!(
            code_block_source.contains(needle),
            "Code view should keep stable render marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = logic::DEFAULT_MIN)] min: f64",
        "#[prop(optional, default = logic::DEFAULT_MAX)] max: f64",
        "#[prop(optional, default = logic::DEFAULT_STEP)] step: f64",
        "pub fn normalize_default_value(default_value: Option<f64>) -> f64",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Slider docs copy-ready snippets should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");

    for needle in [
        "### Slider 同步记录（2026-02-18）",
        "value + on_value_change + default_value",
        "is_disabled",
        "min/max/step",
        "component_doc!(\"Slider\", \"slider\", \"Forms\", forms_extra::slider)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::slider()",
        "`Hello World (Uncontrolled)`、`Controlled + Source Markers`、`Disabled + Fine Step`",
        "Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
        "参数语义变更必须先同步本策略文档与 docs 页面",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_index_source.contains(needle),
            "Slider HeroUI/doc sync record should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn slider() -> AnyView",
        "slug=\"slider\"",
        "title=\"Slider\"",
        "data-slot=\"slider-source-first\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "Slider docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = logic::DEFAULT_MIN)] min: f64",
        "#[prop(optional, default = logic::DEFAULT_MAX)] max: f64",
        "#[prop(optional, default = logic::DEFAULT_STEP)] step: f64",
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Slider parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn slider_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/slider/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "Slider checklist should keep HeroUI/doc sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_explicit_forbidden_antipattern_rules() {
    let check2_source = load_source("src/slider/check2.md");

    for needle in [
        "### 8. 明确禁止的反模式",
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
            "Slider checklist should keep explicit forbidden-antipattern rule `{needle}`."
        );
    }
}

#[test]
fn slider_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/slider.rs");
    let headless_source = load_source("../../crates/ui-headless/src/slider.rs");

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "leptos::",
        "view! {",
        "NodeRef<",
        "on:click",
        "on:input",
        "class=",
        "style=",
        "document.",
        "window.",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives slider should stay pure POJO/state math and avoid `{forbidden}`."
        );
    }

    for required in [
        "pub struct SliderStateInput",
        "pub struct SliderState",
        "pub fn resolve_state(",
        "pub fn sanitize_value(",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives slider should keep stable state-primitive marker `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-",
        "class=",
        ".ui-",
        "style=",
        "spring",
        "keyframe",
        "animation",
        "timeline",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless slider should avoid visual/animation orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct SliderInputAttrs",
        "pub struct SliderHandlers",
        "pub struct SliderState",
        "pub struct SliderAria",
        "pub fn use_slider(options: SliderOptions) -> SliderAria",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless slider should keep typed attrs/handlers/state contract marker `{required}`."
        );
    }
}

#[test]
fn slider_forbidden_antipatterns_keep_key_state_decisions_out_of_view() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");

    for required in [
        "logic::normalize_id(id.unwrap_or_default())",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "logic::resolve_agent_contract(has_value_change_handler)",
        "logic::resolve_state(SliderStateInput {",
        "logic::resolve_ui_action(",
    ] {
        assert!(
            view_source.contains(required),
            "Slider view should consume normalized logic output via `{required}`."
        );
    }

    for forbidden in [
        "if id.is_empty()",
        "if disabled {",
        "logic::parse_value(&event_target_value(&ev))",
        "logic::sanitize_value(parsed, min, max, step)",
        "sanitize_bounds(",
        "sanitize_step(",
        "match value {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Slider view should not hide key state decisions via `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_value_axis(input: ValueAxisInput) -> ValueAxisState",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
        "pub fn normalize_id(id: String) -> IdState",
    ] {
        assert!(
            logic_source.contains(required),
            "Slider logic should keep centralized normalization marker `{required}`."
        );
    }
}

#[test]
fn slider_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks() {
    let view_source = load_source("src/slider/view.rs");
    let mod_source = load_source("src/slider/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "labels + children",
        "titles + panels",
        "web_sys::",
        "js_sys::",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "Slider should avoid parallel-array API or platform leak token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Slider(",
        "pub use view::Slider;",
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
    ] {
        assert!(
            view_source.contains(required) || mod_source.contains(required),
            "Slider public API should remain typed and stable via `{required}`."
        );
    }
}

#[test]
fn slider_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk() {
    let mod_source = load_source("src/slider/mod.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let view_source = load_source("src/slider/view.rs");
    let combined = format!("{mod_source}\n{logic_source}\n{view_source}");

    for forbidden in [
        "temporary patch",
        "TEMP PATCH",
        "TODO(temp)",
        "FIXME(temp)",
        "HACK:",
        "quick fix",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Slider should not carry temporary patch drift marker `{forbidden}`."
        );
    }

    for required in [
        "use ui_state_primitives::slider as slider_state;",
        "pub use slider_state::{",
        "use ui_headless::{",
        "A11yDirection",
        "SliderAria",
        "SliderOptions",
        "use_slider",
        "use_slider(SliderOptions {",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "Slider should consume sunk primitives/headless contracts via `{required}`."
        );
    }

    for forbidden in ["pub struct SliderState {", "pub enum SliderPhase {"] {
        assert!(
            !logic_source.contains(forbidden),
            "Slider component logic should not re-declare reusable primitive `{forbidden}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_forbidden_antipattern_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_explicit_forbidden_antipattern_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_keep_key_state_decisions_out_of_view",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_final_merge_gate_rules() {
    let check2_source = load_source("src/slider/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
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
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Slider checklist should keep final merge-gate rule `{needle}`."
        );
    }
}

#[test]
fn slider_final_merge_gate_capabilities_are_backed_by_contract_checks() {
    slider_component_file_responsibilities_remain_scoped();
    slider_view_mounts_headless_contract_without_state_machine_reimplementation();
    slider_public_api_keeps_prefixed_naming_and_legacy_compatibility();
    slider_type_system_and_machine_readable_markers_form_a_closed_contract();
    slider_exposes_machine_readable_state_and_source_markers();
    slider_reduced_motion_ssr_wasm_paths_keep_semantics_stable();
    slider_docs_examples_sync_with_logic_api_names_and_state_matrix();
    slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes();
    slider_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free();
}

#[test]
fn slider_final_merge_gate_marks_full_repo_gate_as_component_scoped_na() {
    let check2_source = load_source("src/slider/check2.md");

    assert!(
        check2_source.contains(
            "说明：本项按 slider 负责范围执行（`fmt/clippy/test/check/e2e/tree-shaking`）；仓库级 smoke 属于整仓门禁，在并行开发环境下标记为 `N/A`，不作为 slider 单组件阻断。"
        ),
        "Slider final merge-gate should explicitly mark full repo smoke gate as scoped N/A for component-only responsibility."
    );
}

#[test]
fn slider_contract_hygiene_script_covers_final_merge_gate_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_final_merge_gate_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_capabilities_are_backed_by_contract_checks",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_final_merge_gate_marks_full_repo_gate_as_component_scoped_na",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_component_directory_has_standard_file_layout_and_no_spec_file() {
    for rel in [
        "src/slider/mod.rs",
        "src/slider/logic.rs",
        "src/slider/styles.rs",
        "src/slider/view.rs",
        "src/slider/motion.rs",
        "src/slider/check2.md",
        "src/slider/README.md",
    ] {
        assert!(
            path_exists(rel),
            "Slider should keep required file `{rel}`."
        );
    }

    assert!(
        !path_exists("src/slider/spec.rs"),
        "Slider is a simple component and should not introduce `src/slider/spec.rs`."
    );
}

#[test]
fn slider_check2_documents_component_directory_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn slider_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/slider/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use ui_state_primitives::slider::{SliderPhase, SliderState, SliderStateInput};",
        "pub use motion::SliderMotion;",
        "pub use view::Slider;",
    ] {
        assert!(
            mod_source.contains(needle),
            "slider/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use logic::*;",
        "pub use view::*;",
        "mod render;",
        "pub mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "slider/mod.rs should not over-export/introduce drift token `{forbidden}`."
        );
    }
}

#[test]
fn slider_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/slider/logic.rs");
    let styles_source = load_source("src/slider/styles.rs");
    let view_source = load_source("src/slider/view.rs");
    let motion_source = load_source("src/slider/motion.rs");

    assert!(
        !path_exists("src/slider/render.rs"),
        "Slider should keep render implementation in `view.rs` without `render.rs` drift."
    );
    assert!(
        !path_exists("src/slider/spec.rs"),
        "Slider should keep simple-component scope and avoid `spec.rs`."
    );

    for forbidden in [
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
        "role=",
        "aria-",
        "var(--ui-",
        ".ui-slider",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "slider/logic.rs should stay normalize/derive-only and avoid `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "slider/styles.rs should keep static token-first css marker `{needle}`."
        );
    }
    for forbidden in [
        "Signal::derive(",
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "slider/styles.rs should avoid runtime/view logic token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Slider(",
        "use_slider(SliderOptions {",
        "render_label(",
        "render_control(",
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
    ] {
        assert!(
            view_source.contains(required),
            "slider/view.rs should keep render + headless semantic mount marker `{required}`."
        );
    }
    for forbidden in [
        "ui_state_primitives::slider::resolve_state(",
        "pub const CSS",
        "mod logic;",
        "mod styles;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "slider/view.rs should not own style/module/primitive bypass token `{forbidden}`."
        );
    }

    for required in [
        "pub struct SliderMotion",
        "pub fn attach_motion(",
        "sanitize_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "slider/motion.rs should keep motion-contract marker `{required}`."
        );
    }
    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "slider/motion.rs should not carry view semantics token `{forbidden}`."
        );
    }
}

#[test]
fn slider_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_directory_has_standard_file_layout_and_no_spec_file",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_tree_shaking_feature_wiring_is_component_scoped() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-slider = [\"dep:ui-slider\"]",
        "#[cfg(feature = \"component-slider\")]\npub use ui_slider as slider;",
        "#[cfg(feature = \"component-slider\")]\n    out.push_str(crate::slider::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Slider tree-shaking contract should include `{needle}`."
        );
    }
}

#[test]
fn slider_platform_guards_cover_wasm_and_non_wasm_motion_paths() {
    let source = load_source("src/slider/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            source.contains(needle),
            "Slider motion should include platform/reduced-motion guard `{needle}`."
        );
    }
}

#[test]
fn slider_reduced_motion_ssr_wasm_paths_keep_semantics_stable() {
    let motion_source = load_source("src/slider/motion.rs");
    let view_source = load_source("src/slider/view.rs");

    for needle in [
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_visual_percent: leptos::prelude::Signal<f64>",
        "_motion: SliderMotion,",
    ] {
        assert!(
            motion_source.contains(needle),
            "slider motion should keep reduced-motion/non-wasm predictable branch `{needle}`."
        );
    }

    for needle in [
        "motion::attach_motion(root_ref, visual_percent, motion)",
        "data-state=move || state.get().phase_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "slider view should keep stable semantic contract marker `{needle}` across platforms."
        );
    }

    for forbidden in ["target_arch", "#[cfg(", "web_sys", "wasm_bindgen"] {
        assert!(
            !view_source.contains(forbidden),
            "slider view semantics should not split by platform via `{forbidden}`."
        );
    }
}

#[test]
fn ui_headless_web_ssr_mutex_guard_is_present() {
    let source = load_source("../ui-headless/src/lib.rs");
    assert!(
        source.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]"),
        "ui-headless should keep explicit web+ssr mutex cfg guard."
    );
    assert!(
        source
            .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should keep compile_error mutex contract for web+ssr features."
    );
}

#[test]
fn slider_non_wasm_component_files_stay_browser_object_free() {
    for rel in [
        "src/slider/mod.rs",
        "src/slider/logic.rs",
        "src/slider/styles.rs",
        "src/slider/view.rs",
    ] {
        let source = load_source(rel);
        for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
            assert!(
                !source.contains(forbidden),
                "non-wasm slider component file `{rel}` should avoid `{forbidden}`."
            );
        }
    }
}

#[test]
fn slider_inner_html_usage_is_forbidden_in_component_and_docs() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    for rel in [
        "src/slider/view.rs",
        "../../apps/docs-app/src/pages/components/pages/forms_extra.rs",
        "src/slider/README.md",
    ] {
        let source = load_source(rel);
        assert!(
            !source.contains("inner_html"),
            "Slider path `{rel}` should not use `inner_html`."
        );
    }

    let script_needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_inner_html_usage_is_forbidden_in_component_and_docs";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`."
    );
}

#[test]
fn slider_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let slider_mod_source = load_source("src/slider/mod.rs");
    let slider_logic_source = load_source("src/slider/logic.rs");
    let slider_motion_source = load_source("src/slider/motion.rs");
    let slider_view_source = load_source("src/slider/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_slider_contract.spec.mjs");
    let slider_check2_source = load_source("src/slider/check2.md");

    let needle = "macro_rules! wasm_debug_proxy";
    assert!(
        crate_root_source.contains(needle),
        "ui-components should keep wasm debug capability isolated via `{needle}`."
    );

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("slider-wasm-debug")
            && !cargo_source.contains("component-slider-wasm-debug"),
        "Slider should not expose a dedicated wasm-debug feature and should reuse global trace/debug overlay."
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
        !all_components_block.contains("button-wasm-debug")
            && !all_components_block.contains("accordion-wasm-debug"),
        "wasm debug features must not leak into all-components production path."
    );

    let slider_combined = format!(
        "{slider_mod_source}\n{slider_logic_source}\n{slider_motion_source}\n{slider_view_source}"
    );
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
            !slider_combined.contains(forbidden),
            "Slider production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for marker in [
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
    ] {
        assert!(
            slider_view_source.contains(marker),
            "Slider should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
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
            "docs-app should gate debug visualization by debug_assertions via `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
        "UiTraceEventKind::OpenChange { open }",
        "UiTraceEventKind::Inspect { tag, data_slot }",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace event visualization marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
    ] {
        assert!(
            trace_source.contains(needle),
            "headless trace contract should keep timestamped event model via `{needle}`."
        );
    }

    for needle in [
        "docs-app slider key flow uses semantic breakpoints with explicit settled conditions",
        "await input.focus();",
        "await page.keyboard.press(\"ArrowRight\");",
        "toHaveAttribute(\"data-value\", \"37\")",
        "toHaveAttribute(\"data-value-percent\", \"37\")",
        "toHaveAttribute(\"data-ui-source\", \"on_value_change\")",
        "not.toHaveAttribute(\"data-pressed\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Slider interaction path should remain replayable through deterministic e2e chain `{needle}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            slider_check2_source.contains(needle),
            "Slider checklist should keep wasm-debug governance marker `{needle}`."
        );
    }
}

#[test]
fn slider_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn slider_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

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
        "pub(super) fn slider() -> AnyView",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "title=\"Controlled + Source Markers\"",
        "title=\"Disabled + Fine Step\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn slider_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("src/slider/check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<Card class_name=\"playground__panel playground__controls\".to_string()>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (controlled_value_raw, set_controlled_value_raw) = signal(36.0_f64);",
        "let (last_change, set_last_change) = signal(\"none\".to_string());",
        "set_last_change.set(format!(\"{next:.1}\"));",
        "\" · last on_value_change: \" {move || last_change.get()}",
        "let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);",
        "let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));",
    ] {
        assert!(
            docs_source.contains(needle),
            "Slider docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "SLIDER_WORKBENCH_STORAGE_KEY",
        "load_slider_workbench_state(",
        "save_slider_workbench_state(",
        "clear_slider_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Slider keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
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
            "Slider checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn slider_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/slider/mod.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let view_source = load_source("src/slider/view.rs");
    let styles_source = load_source("src/slider/styles.rs");
    let motion_source = load_source("src/slider/motion.rs");
    let checklist_source = load_source("src/slider/check2.md");

    assert!(
        !path_exists("src/slider/spec.rs"),
        "Slider should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-slider = [\"dep:ui-slider\"]"),
        "Slider feature should stay lightweight and only depend on the extracted ui-slider crate."
    );
    assert!(
        !cargo_source.contains("component-slider = [\"dep:serde\"")
            && !cargo_source.contains("component-slider = [\"dep:serde_json\""),
        "Slider should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
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
            "Slider engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn slider_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/slider/mod.rs"),
        load_source("src/slider/logic.rs"),
        load_source("src/slider/view.rs"),
        load_source("src/slider/styles.rs"),
        load_source("src/slider/motion.rs"),
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
        !cargo_source.contains("slider-wasm-debug"),
        "Slider should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::slider::",
        "const SLIDER_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Slider should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn slider_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/slider/mod.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let view_source = load_source("src/slider/view.rs");
    let styles_source = load_source("src/slider/styles.rs");
    let motion_source = load_source("src/slider/motion.rs");

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
                "Slider engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Slider public module boundary should not leak web_sys types."
    );
}

#[test]
fn slider_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep ui-components entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn slider_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-slider\")]",
        "pub use ui_slider as slider;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
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
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn slider_ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-slider\")]",
        "out.push_str(crate::slider::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn slider_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn slider_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn slider_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
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
fn slider_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_css_registry_remains_feature_gated_and_non_global",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_root_centralizes_theme_injection_and_i18n_context",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            script_source.contains(needle),
            "entrypoints check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn slider_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");

    for needle in [
        "pub enum SliderAgentSchema",
        "pub enum SliderStreamSupport",
        "pub enum SliderStreamFallback",
        "pub enum SliderStreamMode",
        "pub enum SliderOutputStatus",
        "pub enum SliderIntent",
        "pub enum SliderUiAction",
        "pub struct SliderAgentContract",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> SliderAgentContract",
        "pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> SliderUiAction",
    ] {
        assert!(
            logic_source.contains(needle),
            "Slider agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-value-source=value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn slider_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "data-ui-schema=move || format!(",
        "data-ui-intent=move || format!(",
        "data-ui-action=move || format!(",
        "data-ui-source=move || format!(",
        "format!(\"data-ui-",
        "intent=\"",
        "action=\"",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Slider should avoid free-form schema string splicing token `{forbidden}`."
        );
    }

    for required in [
        "SliderAgentSchema::V1.as_attr()",
        "SliderStreamSupport::Unsupported.as_attr()",
        "SliderStreamFallback::Snapshot.as_attr()",
        "SliderStreamMode::Snapshot.as_attr()",
        "SliderOutputStatus::Verified",
        "SliderOutputStatus::Submittable",
        "SliderIntent::AdjustValue.as_attr()",
        "ui_action.get().as_attr()",
    ] {
        assert!(
            combined.contains(required),
            "Slider agent contract fields should stay type-derived via `{required}`."
        );
    }
}

#[test]
fn slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let styles_source = load_source("src/slider/styles.rs");
    let mod_source = load_source("src/slider/mod.rs");
    let motion_source = load_source("src/slider/motion.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

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
            "Slider Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "slider/check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn slider_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/slider_semantics.rs");

    for required in [
        "slider_exposes_machine_readable_state_and_source_markers",
        "slider_headless_contract_is_typed_and_lang_dir_aware",
        "slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "slider_agent_contract_is_schema_typed_and_machine_readable",
        "slider_check2_documents_semantics_first_testing_rules",
        "slider_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
        "role=slider_aria.input.role",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Slider semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["to", "_match", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Slider semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn slider_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/slider/view.rs");
    let semantics_source = load_source("tests/slider_semantics.rs");

    for marker in [
        "role=slider_aria.input.role",
        "aria-label=move || label.with_value(|label| label.clone())",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
        "aria-valuetext=move || slider_aria.input.aria_valuetext.get()",
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-pressed=move || slider_aria.state.is_pressed.get().then_some(\"true\")",
        "data-hovered=move || slider_aria.state.is_hovered.get().then_some(\"true\")",
        "data-focused=move || slider_aria.state.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some(\"true\")",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Slider view should keep semantic marker `{marker}`."
        );
        let escaped_marker = marker.replace('"', "\\\"");
        assert!(
            semantics_source.contains(marker) || semantics_source.contains(&escaped_marker),
            "Slider semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn slider_contract_hygiene_script_covers_semantics_first_testing_rules() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_readme_keeps_beginner_first_documentation_path() {
    let source = load_source("src/slider/README.md");
    for needle in [
        "# Slider",
        "## Hello World",
        "## 受控用法",
        "受控/非受控轴",
        "is_disabled",
        "lang` / `dir",
    ] {
        assert!(
            source.contains(needle),
            "Slider README should include `{needle}` for beginner-first documentation."
        );
    }
}

#[test]
fn slider_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_slider_contract.spec.mjs");

    for needle in [
        "const CONTROLLED_SLIDER_ROOT =",
        "[data-component=\"slider\"] [data-slot=\"slider\"][data-control-mode=\"controlled\"][data-max=\"100\"]",
        "body:not(:has(#boot))",
        "resolveControlledSliderRoot",
        "toHaveAttribute(\"data-ui-schema\", \"ui.slider.agent-contract.v1\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"unsupported\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "locator('[data-slot=\"slider-input\"]')",
        "toHaveAttribute(\"data-control-mode\", \"controlled\")",
        "toHaveAttribute(\"data-value-source\", \"external\")",
        "toHaveAttribute(\"data-ui-action\", \"idle\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Slider e2e selector/stable-wait contract should include `{needle}`."
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
            "Slider e2e selector contract should avoid unstable/non-semantic token `{forbidden}`."
        );
    }
}

#[test]
fn slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_slider_contract.spec.mjs");

    for needle in [
        "docs-app slider key flow uses semantic breakpoints with explicit settled conditions",
        "await input.focus();",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "page.keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-value\", \"37\")",
        "toHaveAttribute(\"data-value-percent\", \"37\")",
        "toHaveAttribute(\"data-ui-source\", \"on_value_change\")",
        "not.toHaveAttribute(\"data-pressed\", \"true\")",
        "[data-slot=\"slider\"][data-disabled=\"true\"]",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Slider e2e ready/settled contract should include `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/slider/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Slider checklist should keep repeatable-flow rule `{required}`."
        );
    }
}

#[test]
fn slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_slider_contract.spec.mjs");

    for needle in [
        "docs-app slider key flow is repeatable and failures map to semantic breakpoints",
        "await page.reload();",
        "toHaveAttribute(\"data-value\", \"37\")",
        "toHaveAttribute(\"data-value\", \"36\")",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "toHaveAttribute(\"data-ui-action\", \"idle\")",
        "toHaveAttribute(\"data-ui-source\", \"on_value_change\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Slider e2e repeatable-flow contract should include `{needle}`."
        );
    }
}

#[test]
fn slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_slider_contract.spec.mjs");

    for needle in [
        "await input.focus();",
        "page.keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-focused\", \"true\")",
        "toHaveAttribute(\"data-focus-visible\", \"true\")",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "toHaveAttribute(\"data-value-percent\", \"37\")",
        "not.toHaveAttribute(\"data-pressed\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Slider e2e high-risk path contract should include `{needle}`."
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
            "Slider high-risk e2e path should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn slider_e2e_check_script_covers_selector_and_settled_wait_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-slider.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "slider e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_agent_contract_markers_are_schema_like_and_closed_set() {
    let logic_source = load_source("src/slider/logic.rs");
    let view_source = load_source("src/slider/view.rs");

    for needle in [
        "ui.slider.agent-contract.v1",
        "\"unsupported\"",
        "\"snapshot\"",
        "\"verified\"",
        "\"submittable\"",
        "\"adjust-value\"",
        "\"idle\"",
        "\"focus\"",
        "\"press\"",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Slider agent contract should keep closed-set marker `{needle}`."
        );
    }
}

#[test]
fn slider_type_system_and_machine_readable_markers_form_a_closed_contract() {
    let logic_source = load_source("src/slider/logic.rs");
    let state_source = load_source("../ui-state-primitives/src/slider.rs");
    let view_source = load_source("src/slider/view.rs");

    for needle in [
        "pub enum SliderControlMode",
        "pub enum SliderValueSource",
        "pub enum SliderValueChangeSource",
        "pub enum SliderDisabledSource",
        "pub enum SliderUiAction",
        "pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> SliderUiAction",
    ] {
        assert!(
            logic_source.contains(needle),
            "slider logic should keep typed discrete axis `{needle}`."
        );
    }

    for needle in [
        "pub fn sanitize_bounds(min: f64, max: f64) -> (f64, f64)",
        "pub fn sanitize_step(step: f64, min: f64, max: f64) -> f64",
        "pub fn sanitize_value(value: f64, min: f64, max: f64, step: f64) -> f64",
        "pub fn resolve_state(input: SliderStateInput) -> SliderState",
    ] {
        assert!(
            state_source.contains(needle),
            "slider state primitive should normalize invalid states through `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "slider view should expose machine-readable marker `{needle}`."
        );
    }
}

#[test]
fn slider_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/slider/view.rs");

    for needle in [
        "\"slider\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep slider performance budget contract token `{needle}`."
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Slider\", \"slider\", \"Forms\", forms_extra::slider)"),
        "Slider docs entry should remain in docs perf probe traversal."
    );

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
            "UiPerfProbe should expose repeatable performance regression marker `{needle}`."
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
            "docs coverage e2e should enforce slider perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
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
            "Slider checklist should keep performance governance token `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Slider view should expose attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn slider_check2_documents_streaming_optional_snapshot_fallback_for_non_llm_component() {
    let source = load_source("src/slider/check2.md");
    for needle in [
        "`Streaming`：LLM 还在生成",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "Slider 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
    ] {
        assert!(
            source.contains(needle),
            "Slider check2 should explicitly document streaming contract `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let source = load_source("src/slider/check2.md");
    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            source.contains(needle),
            "Slider check2 should keep streaming two-mode definition marker `{needle}`."
        );
    }
}

#[test]
fn slider_streaming_check_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let needle = "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn slider_check2_documents_snapshot_as_default_baseline_capability() {
    let source = load_source("src/slider/check2.md");
    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            source.contains(needle),
            "Slider check2 should keep snapshot-baseline marker `{needle}`."
        );
    }
}

#[test]
fn slider_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/slider.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for marker in [
        "#[component]",
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "logic::normalize_value_axis(logic::ValueAxisInput {",
        "use_slider(SliderOptions {",
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Slider snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> SliderAgentContract",
        "SliderStreamSupport::Unsupported",
        "SliderStreamFallback::Snapshot",
        "SliderStreamMode::Snapshot",
        "SliderOutputStatus::Verified",
        "SliderOutputStatus::Submittable",
        "pub fn resolve_state(input: SliderStateInput) -> SliderState",
        "pub fn sanitize_value(value: f64, min: f64, max: f64, step: f64) -> f64",
    ] {
        assert!(
            logic_source.contains(marker) || primitives_source.contains(marker),
            "Slider snapshot baseline should keep stable normalization/state marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn slider() -> AnyView",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "default_value=36.0",
        "<Playground title=\"Controlled + Source Markers\" code_signal=code>",
        "id=\"docs-slider-volume\".to_string()",
        "id=\"docs-slider-disabled\".to_string()",
        "id=\"docs-slider-fine\".to_string()",
        "on_value_change=on_value_change",
    ] {
        assert!(
            docs_source.contains(marker),
            "Slider docs should include complete snapshot result marker `{marker}`."
        );
    }
}

#[test]
fn slider_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_streaming_required_optional_classification_rules() {
    let source = load_source("src/slider/check2.md");
    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Slider 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
    ] {
        assert!(
            source.contains(needle),
            "Slider check2 should keep streaming responsibility marker `{needle}`."
        );
    }
}

#[test]
fn slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/slider/view.rs");

    for required in [
        "role=slider_aria.input.role",
        "aria-label=move || label.with_value(|label| label.clone())",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
        "aria-valuetext=move || slider_aria.input.aria_valuetext.get()",
        "lang=move || slider_aria.input.lang.clone()",
        "dir=move || slider_aria.input.dir",
        "data-state=move || state.get().phase_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Slider should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/slider/view.rs");
    let logic_source = load_source("src/slider/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "revalidate",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Slider should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn slider_streaming_check_script_covers_streaming_responsibility_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test slider_semantics --no-default-features --features component-slider,inject-css slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn slider_check2_documents_async_semantics_as_na_for_sync_only_component_scope() {
    let source = load_source("src/slider/check2.md");
    assert!(
        source.contains(
            "Slider 无远程请求与异步状态，异步交互语义项为 `N/A`（仅同步输入与状态派生）。"
        ),
        "Slider check2 should explicitly mark async semantics as N/A for sync-only scope."
    );
}

#[test]
fn slider_check2_records_tree_shaking_gate_results() {
    let source = load_source("src/slider/check2.md");
    for needle in [
        "cargo tree -e features -p ui-components --no-default-features --features component-accordion,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features component-accordion,inject-css",
        "BUDGET_OK",
    ] {
        assert!(
            source.contains(needle),
            "Slider check2 should retain tree-shaking evidence `{needle}`."
        );
    }
}

#[test]
fn slider_check2_has_no_unchecked_items_after_stepwise_verification() {
    let source = load_source("src/slider/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "slider check2 should not keep unchecked checklist items after sequential real verification."
    );
}

#[test]
fn slider_heroui_strategy_doc_is_synced_with_slider_parameter_changes() {
    let source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    for needle in [
        "### Slider 同步记录（2026-02-18）",
        "component_doc!(\"Slider\", \"slider\", \"Forms\", forms_extra::slider)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::slider()",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "e2e/tests/docs_app_slider_contract.spec.mjs",
    ] {
        assert!(
            source.contains(needle),
            "HeroUI strategy doc should keep slider sync evidence `{needle}`."
        );
    }
}

#[test]
fn slider_docs_registry_entry_is_indexable() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    assert!(
        source.contains("component_doc!(\"Slider\", \"slider\", \"Forms\", forms_extra::slider)"),
        "docs registry should keep the slider component entry indexable."
    );
}
