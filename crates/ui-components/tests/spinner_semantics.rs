use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn spinner_component_src_dir() -> std::path::PathBuf {
    workspace_dir().join("components/spinner/src")
}

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(rest) = rel_path.strip_prefix("src/spinner/") {
        spinner_component_src_dir().join(rest)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn spinner_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/spinner/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Spinner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn spinner_component_assembly_keeps_layered_responsibilities() {
    let mod_source = load_source("src/spinner/mod.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let view_source = load_source("src/spinner/view.rs");
    let motion_source = load_source("src/spinner/motion.rs");
    let styles_source = load_source("src/spinner/styles.rs");

    for needle in [
        "pub use logic::SpinnerSize;",
        "pub use motion::SpinnerMotion;",
        "pub use view::Spinner;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Spinner module should expose stable assembly API via `{needle}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::spinner::{"),
        "Spinner logic should consume primitives from ui-state-primitives."
    );

    for needle in [
        "i18n::use_ui_i18n()",
        "logic::resolve_render_state(SpinnerRenderInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner view should keep render + semantic mounting assembly via `{needle}`."
        );
    }

    for needle in ["pub fn sanitize_motion(", "pub fn attach_motion("] {
        assert!(
            motion_source.contains(needle),
            "Spinner motion contract should provide `{needle}`."
        );
    }

    for needle in ["--ui-button-spinner-size", "--ui-button-spinner-border"] {
        assert!(
            styles_source.contains(needle),
            "Spinner styles should stay token-first and include `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen::", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Spinner public/component surface must not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn spinner_public_api_naming_stays_consistent() {
    let view_source = load_source("src/spinner/view.rs");
    let mod_source = load_source("src/spinner/mod.rs");

    for needle in [
        "#[prop(optional)] size: SpinnerSize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] motion: SpinnerMotion",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "pub use logic::SpinnerSize;",
        "pub use motion::SpinnerMotion;",
    ] {
        assert!(
            view_source.contains(needle) || mod_source.contains(needle),
            "Spinner public API naming should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] open: bool",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "Spinner should not drift into unrelated alias naming; found `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains(": bool"),
        "Spinner currently exposes no boolean state axis props, so `is_*` naming rule is N/A here."
    );
}

#[test]
fn spinner_controlled_uncontrolled_contract_is_not_applicable() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "on_value_change",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "on_open_change",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Spinner has no controllable state axis and must not expose half-controlled API `{forbidden}`."
        );
    }
}

#[test]
fn spinner_discrete_state_inputs_are_type_constrained() {
    let view_source = load_source("src/spinner/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spinner.rs");

    for needle in [
        "pub enum SpinnerSize",
        "#[default]",
        "Sm,",
        "Md,",
        "Lg,",
        "#[prop(optional)] size: SpinnerSize",
    ] {
        assert!(
            primitive_source.contains(needle) || view_source.contains(needle),
            "Spinner discrete input contract should include `{needle}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "size: String",
        "#[prop(optional)] is_sm: bool",
        "#[prop(optional)] is_md: bool",
        "#[prop(optional)] is_lg: bool",
    ] {
        assert!(
            !view_source.contains(forbidden) && !primitive_source.contains(forbidden),
            "Spinner should avoid string/bool explosion discrete-state APIs; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_state_primitives_source_boundary_is_enforced() {
    let logic_source = load_source("src/spinner/logic.rs");
    let view_source = load_source("src/spinner/view.rs");

    for needle in [
        "pub use ui_state_primitives::spinner::{",
        "resolve_state(SpinnerStateInput {",
        "compose_class_name(class_name, state)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner logic should consume state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct SpinnerStateInput",
        "pub struct SpinnerState",
        "create_rw_signal",
        "RwSignal",
        "Signal<",
        "Store",
        "AppStore",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Spinner should not reimplement primitives or bind business store types; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_async_contract_is_not_applicable() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let styles_source = load_source("src/spinner/styles.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "error",
        "use_async_action",
        "data-loading",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Spinner has no async interaction contract and should not drift to `{forbidden}`."
        );
    }
}

#[test]
fn spinner_a11y_i18n_locale_contract_uses_headless() {
    let view_source = load_source("src/spinner/view.rs");

    for needle in [
        "use ui_headless::i18n::CommonStrings;",
        "use ui_headless::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "default_aria_label: common.loading_aria_label.as_ref()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "<CircularProgress aria_label=render.aria_label class_name=\"ui-spinner__progress\" />",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner should keep A11y+i18n contract via `{needle}`."
        );
    }

    for forbidden in [
        "default_aria_label: \"Loading\"",
        "aria_label=\"Loading\"",
        "lang=\"en\"",
        "dir=\"ltr\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner should not hardcode locale-visible defaults; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_dx_paradox_stays_simple_by_default() {
    let view_source = load_source("src/spinner/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "#[prop(optional)] size: SpinnerSize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] motion: SpinnerMotion",
        "Playground title=\"Hello World\"",
        "r#\"<Spinner />\"#",
        "<Spinner />",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "Spinner DX baseline should keep `{needle}` available.",
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] spinner_state:",
        "state_machine=",
        "SpinnerStateInput",
        "SpinnerState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner default usage must not require primitive wiring via `{forbidden}`."
        );
    }
}

#[test]
fn spinner_composite_api_rule_is_not_applicable() {
    let mod_source = load_source("src/spinner/mod.rs");
    let view_source = load_source("src/spinner/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop(optional)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] children:",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner is a leaf component and must not expose composite API shape `{forbidden}`."
        );
    }

    assert!(
        mod_source.contains("pub use view::Spinner;"),
        "Spinner should keep direct leaf export without Parent/Item API wrappers."
    );

    let spinner_start = docs_source
        .find("pub(super) fn spinner() -> AnyView")
        .expect("spinner docs section should exist");
    let spinner_tail = &docs_source[spinner_start..];
    let progress_offset = spinner_tail
        .find("pub(super) fn progress() -> AnyView")
        .expect("spinner section should be delimited by progress section");
    let spinner_section = &spinner_tail[..progress_offset];

    for needle in ["<Spinner />", "<Spinner size=SpinnerSize::Sm />"] {
        assert!(
            spinner_section.contains(needle),
            "Spinner docs should keep explicit leaf usage `{needle}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "ItemSpec"] {
        assert!(
            !spinner_section.contains(forbidden),
            "Spinner docs should not introduce implicit paired-array composition `{forbidden}`."
        );
    }
}

#[test]
fn spinner_defaults_are_normalized_in_logic_only() {
    let logic_source = load_source("src/spinner/logic.rs");
    let view_source = load_source("src/spinner/view.rs");

    for needle in [
        "pub fn resolve_render_state(",
        "let class_name = normalize_optional_text(input.class_name);",
        "resolve_aria_label(input.aria_label, input.default_aria_label)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner defaults and priority should be centralized in logic via `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label, common.loading_aria_label.as_ref())",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner view must not keep default-value fallback branches; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_consumes_state_primitives_state_model() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spinner.rs");

    for needle in [
        "pub use ui_state_primitives::spinner::{",
        "SpinnerStateInput",
        "SpinnerState",
        "pub struct SpinnerRenderInput",
        "pub struct SpinnerRenderState",
        "pub fn resolve_render_state(",
        "normalize_optional_text",
        "resolve_aria_label",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner logic should consume primitives via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct SpinnerStateInput",
        "pub struct SpinnerState",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Spinner logic must not re-implement primitive state model; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SpinnerStateInput",
        "pub struct SpinnerState",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spinner primitive should include `{needle}`."
        );
    }

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<CommonStrings>()",
        "logic::resolve_render_state(SpinnerRenderInput {",
        "default_aria_label: common.loading_aria_label.as_ref()",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_aria_label(aria_label, common.loading_aria_label.as_ref())",
        "logic::resolve_state(SpinnerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner view should not keep normalization branches; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/spinner/view.rs");

    for attr in [
        "data-slot=\"spinner\"",
        "data-size=state.state.size_attr",
        "data-state=\"indeterminate\"",
        "data-indeterminate=\"true\"",
        "data-label-source=state.state.label_source_attr",
        "data-custom-aria-label=state.state.has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=state.state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.state.class_source_attr",
        "style=state.style_vars",
        "data-motion-source=state.motion_source",
        "data-custom-motion=(state.motion_source == \"custom\").then_some(\"true\")",
        "class_name=\"ui-spinner__progress\"",
    ] {
        assert!(
            source.contains(attr),
            "Spinner should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn spinner_styles_include_size_and_source_markers() {
    let source = load_source("src/spinner/styles.rs");

    for selector in [
        ".ui-spinner__progress",
        ".ui-spinner--size-sm",
        ".ui-spinner[data-size=\"md\"]",
        ".ui-spinner--size-lg",
        ".ui-spinner--label-custom .ui-spinner__progress",
        ".ui-spinner[data-label-source=\"custom\"] .ui-spinner__progress",
        ".ui-spinner--custom-class",
        ".ui-spinner[data-custom-class=\"true\"]",
        ".ui-spinner[data-class-source=\"custom\"] .ui-spinner__progress",
        ".ui-spinner[data-motion-source=\"custom\"]",
        ".ui-spinner[data-custom-motion=\"true\"]",
        "--ui-spinner-rotation-duration",
        "--ui-button-spinner-duration",
        "--ui-button-spinner-size",
        "--ui-button-spinner-border",
        "--ui-space-2xs",
        "--ui-space-sm",
        "--ui-space-3xs",
        ".ui-spinner[data-state=\"indeterminate\"] .ui-spinner__progress",
        "@media (prefers-reduced-motion: reduce)",
        "animation-iteration-count: 1;",
    ] {
        assert!(
            source.contains(selector),
            "Spinner styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn spinner_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("src/spinner/mod.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let view_source = load_source("src/spinner/view.rs");
    let styles_source = load_source("src/spinner/styles.rs");
    let motion_source = load_source("src/spinner/motion.rs");

    for needle in [
        "pub use logic::SpinnerSize;",
        "pub use motion::SpinnerMotion;",
        "pub use view::Spinner;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Spinner mod boundary should keep minimal exports via `{needle}`."
        );
    }

    for forbidden in ["view!", "NodeRef", "data-slot"] {
        assert!(
            !logic_source.contains(forbidden),
            "Spinner logic must stay render-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_render_state(",
        "resolve_state(SpinnerStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner logic should centralize normalization via `{needle}`."
        );
    }

    for forbidden in ["resolve_state(SpinnerStateInput {", "compose_class_name("] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner view must not hide primitive state decisions via `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "logic::resolve_render_state(SpinnerRenderInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner view should only assemble rendering via `{needle}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "Spinner styles should stay token-first static css via `{needle}`."
        );
    }

    for forbidden in ["view! {", "fn sanitize_motion(", "fn resolve_render_state("] {
        assert!(
            !styles_source.contains(forbidden),
            "Spinner styles must not contain logic/motion assembly via `{forbidden}`."
        );
    }

    for needle in ["pub fn sanitize_motion(", "pub fn attach_motion("] {
        assert!(
            motion_source.contains(needle),
            "Spinner motion should keep contract mapping entry `{needle}`."
        );
    }

    for forbidden in ["view! {", "ui_motion::spring::", "resolve_state("] {
        assert!(
            !motion_source.contains(forbidden),
            "Spinner motion must not own view/state-engine concerns via `{forbidden}`."
        );
    }
}

#[test]
fn spinner_does_not_introduce_spec_rs_for_simple_contract() {
    let spec_path = spinner_component_src_dir().join("spec.rs");
    assert!(
        !spec_path.exists(),
        "Spinner is a simple component and should not introduce `spec.rs`."
    );
}

#[test]
fn spinner_motion_contract_stays_mapping_only() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let motion_source = load_source("src/spinner/motion.rs");

    for needle in [
        "#[prop(optional)] motion: SpinnerMotion",
        "data-motion-source=render.motion_source",
        "data-custom-motion=(render.motion_source == \"custom\").then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner view should map motion contract via `{needle}`."
        );
    }

    for needle in [
        "motion::sanitize_motion(input.motion)",
        "motion::source_attr(motion)",
        "motion::attach_motion(None, motion)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spinner logic should centralize motion normalization via `{needle}`."
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "ui_motion::web::animate",
        "request_animation_frame",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Spinner motion layer must not reimplement a runtime driver; found `{forbidden}`."
        );
    }

    for needle in [
        "use ui_theme::default_button_layout_tokens;",
        "spinner_duration_ms",
    ] {
        assert!(
            motion_source.contains(needle),
            "Spinner motion default should read tokenized duration via `{needle}`."
        );
    }
}

#[test]
fn spinner_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn spinner() -> AnyView",
        "title=\"Spinner\"",
        "slug=\"spinner\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Size Matrix\"",
        "Playground title=\"Custom Label + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Spinner.",
        );
    }
}

#[test]
fn spinner_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "r#\"<Spinner />\"#",
        "<Spinner />",
        "title=\"Size Matrix\"",
        "<Spinner size=SpinnerSize::Sm />",
        "<Spinner size=SpinnerSize::Md />",
        "<Spinner size=SpinnerSize::Lg />",
        "title=\"Custom Label + Class\"",
        "<Spinner aria_label=\"Fetching notifications\".to_string() />",
        "aria_label=\"   \".to_string()",
        "aria_label=\"Syncing inbox\".to_string()",
        "class_name=\"docs-spinner-custom\".to_string()",
        "size=SpinnerSize::Lg",
    ] {
        assert!(
            source.contains(needle),
            "spinner docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn spinner_heroui_strategy_doc_and_docs_entry_are_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "### Spinner 同步记录（2026-02-17）",
        "参数模型同步：`Spinner` 维持 display primitive 定位",
        "`size/aria_label/class_name/motion/lang/dir`",
        "component_doc!(\"Spinner\", \"spinner\", \"Display\", display::spinner)",
        "pub(super) fn spinner() -> AnyView",
        "slug=\"spinner\"",
    ] {
        assert!(
            strategy_source.contains(needle)
                || docs_index_source.contains(needle)
                || docs_page_source.contains(needle),
            "Spinner HeroUI/doc sync contract should include `{needle}`."
        );
    }
}

#[test]
fn spinner_performance_budget_has_static_equivalent_evidence() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let motion_source = load_source("src/spinner/motion.rs");

    for forbidden in [
        "signal(",
        "RwSignal",
        "Memo::new",
        "Effect::new",
        "set_interval",
        "request_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Spinner should keep a static render path after init; found `{forbidden}`."
        );
    }

    for needle in [
        "let render = logic::resolve_render_state(SpinnerRenderInput {",
        "let style = motion::attach_motion(None, motion);",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Spinner render pipeline should stay deterministic via `{needle}`."
        );
    }
}

#[test]
fn spinner_view_macro_complexity_is_controlled() {
    let view_source = load_source("src/spinner/view.rs");
    let view_macro_count = view_source.matches("view! {").count();
    assert_eq!(
        view_macro_count, 1,
        "Spinner should keep a single small `view!` block."
    );

    let line_count = view_source.lines().count();
    assert!(
        line_count < 90,
        "Spinner view should stay compact; found {line_count} lines."
    );
}

#[test]
fn spinner_function_split_and_static_fragment_rules_are_not_abused() {
    let view_source = load_source("src/spinner/view.rs");

    let component_macro_count = view_source.matches("#[component]").count();
    assert!(
        component_macro_count == 1,
        "Spinner should not split tiny fragments into noisy extra components."
    );

    for forbidden in ["<svg", "inner_html"] {
        assert!(
            !view_source.contains(forbidden),
            "Spinner has no heavy static fragment/inner_html need; found `{forbidden}`."
        );
    }
}

#[test]
fn spinner_engineering_unified_contract_is_not_applicable_for_simple_component() {
    let logic_source = load_source("src/spinner/logic.rs");
    let view_source = load_source("src/spinner/view.rs");
    let motion_source = load_source("src/spinner/motion.rs");

    for forbidden in [
        "serde",
        "Serialize",
        "Deserialize",
        "tracing::",
        "tokio::",
        "async_std::",
        "async fn",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Spinner should not leak non-applicable engineering runtime/spec concerns `{forbidden}`."
        );
    }
}

#[test]
fn spinner_ui_components_entry_points_stay_correct() {
    let lib_source = load_source("src/lib.rs");
    let cargo_source = load_source("Cargo.toml");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "#[cfg(feature = \"component-spinner\")]",
        "pub use ui_spinner as spinner;",
        "pub use spinner::{Spinner, SpinnerSize};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should include `{needle}`."
        );
    }

    for needle in [
        "component-spinner = [\"dep:ui-spinner\"]",
        "ui-spinner = { path = \"../../components/spinner\", optional = true }",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo entry should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-spinner\")]",
        "out.push_str(crate::spinner::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregator should gate spinner css via `{needle}`."
        );
    }

    for needle in [
        "provide_ui_i18n(i18n);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized inject/i18n path `{needle}`."
        );
    }

    assert!(
        active_highlight_source.contains("pub const CSS"),
        "active_highlight entry should remain dedicated shared style capability."
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui-components root should not host `{forbidden}`."
        );
    }
}

#[test]
fn spinner_directory_file_layout_is_standard() {
    let component_src = spinner_component_src_dir();
    for expected in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src.join(expected).exists(),
            "spinner component directory should include `{expected}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_src.join(forbidden).exists(),
            "spinner component directory should not include `{forbidden}`."
        );
    }
}

#[test]
fn spinner_agent_schema_and_streaming_rules_are_snapshot_only() {
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");

    for needle in [
        "data-slot=\"spinner\"",
        "data-state=\"indeterminate\"",
        "data-label-source=render.state.label_source_attr",
        "data-class-source=render.state.class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Spinner agent-readable semantic markers should include `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema",
        "streaming",
        "is_streaming",
        "fallback=snapshot",
        "draft",
        "verified",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Spinner is snapshot-only and should not expose unrelated streaming protocol `{forbidden}`."
        );
    }
}

#[test]
fn spinner_docs_are_playground_and_copy_paste_ready() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn spinner() -> AnyView",
        "Playground title=\"Hello World\"",
        "Playground title=\"Size Matrix\"",
        "Playground title=\"Custom Label + Class\"",
        "let hello_code = Signal::derive(move || r#\"<Spinner />\"#.to_string());",
    ] {
        assert!(
            docs_source.contains(needle),
            "Spinner docs should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "#[component]\npub fn Playground(",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should include `{needle}`."
        );
    }
}

#[test]
fn spinner_anti_patterns_are_blocked_by_source_layout() {
    let primitive_source = load_source("../ui-state-primitives/src/spinner.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let view_source = load_source("src/spinner/view.rs");
    let logic_source = load_source("src/spinner/logic.rs");
    let mod_source = load_source("src/spinner/mod.rs");

    for forbidden in ["leptos::", "view! {", "web_sys::", "class="] {
        assert!(
            !primitive_source.contains(forbidden),
            "state primitives must remain pure and avoid `{forbidden}`."
        );
    }

    for forbidden in ["animation", "box-shadow", "class="] {
        assert!(
            !headless_a11y_source.contains(forbidden),
            "ui-headless a11y helpers should not embed visual orchestration `{forbidden}`."
        );
    }

    for forbidden in ["resolve_state(SpinnerStateInput {", "compose_class_name("] {
        assert!(
            !view_source.contains(forbidden),
            "spinner view should not hide state-machine decisions via `{forbidden}`."
        );
    }

    assert!(
        !logic_source.contains("pub struct SpinnerStateInput"),
        "reusable state primitive should not stay reimplemented in spinner logic."
    );

    for forbidden in ["web_sys::", "HtmlElement", "Element"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "spinner public API surface must not leak platform detail `{forbidden}`."
        );
    }
}
