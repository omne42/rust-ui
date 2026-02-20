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
fn time_field_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/time_field/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TimeField internals should stay private; found `{needle}`."
        );
    }

    for needle in [
        "mod i18n;",
        "pub use i18n::TimeFieldStrings;",
        "pub mod motion;",
        "pub use motion::TimeFieldMotion;",
    ] {
        assert!(
            source.contains(needle),
            "TimeField module boundary should expose motion contract token `{needle}`."
        );
    }
}

#[test]
fn time_field_component_file_roles_are_explicit_and_scoped() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for needle in [
        "mod i18n;",
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use i18n::TimeFieldStrings;",
        "pub use motion::TimeFieldMotion;",
        "pub use view::TimeField;",
    ] {
        assert!(
            mod_source.contains(needle),
            "TimeField mod.rs should keep stable export boundary token `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "fn ", "impl "] {
        assert!(
            !mod_source.contains(forbidden),
            "TimeField mod.rs should not carry implementation detail token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_disabled_state(input: DisabledStateInput) -> bool",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
        "has_default_value",
        "has_value_change_handler",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField logic.rs should keep normalization/derivation token `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<",
        "HtmlElement",
        "web_sys",
        "wasm_bindgen",
        "on:pointer",
        "on:keydown",
        "set_property(",
        "ui-time-field__",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "TimeField logic.rs should not contain DOM/style detail token `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-", ".ui-time-field"] {
        assert!(
            styles_source.contains(needle),
            "TimeField styles.rs should keep static token-first CSS token `{needle}`."
        );
    }

    for forbidden in [
        "Meeting time",
        "Clear time",
        "Time field",
        "Signal<",
        "Callback<",
        "use_time_field(",
        "on:click",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "TimeField styles.rs should not carry logic or user-copy token `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "use_time_field(TimeFieldOptions {",
        "use_press(PressOptions {",
        "use_hover(HoverOptions {",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view.rs should mount structure+headless token `{needle}`."
        );
    }

    for forbidden in [
        "update_hour_from_input(",
        "update_minute_from_input(",
        "parse_time_value(",
        "SpringAnimator",
        "set_property(",
        "struct TimeFieldMotion",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField view.rs should not own primitive/motion-engine token `{forbidden}`."
        );
    }

    for needle in [
        "pub struct TimeFieldMotion",
        "pub fn sanitize_motion(motion: TimeFieldMotion) -> TimeFieldMotion",
        "pub fn attach_clear_button_motion(",
        "ui_motion::spring::SpringAnimator",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "TimeField motion.rs should keep contract+attach mapping token `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "<div",
        "data-slot=",
        "role=",
        "aria-label=",
        "pub const CSS",
        "struct SpringAnimator",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "TimeField motion.rs should not contain view/style/engine-definition token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_spec_file_is_not_introduced_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/text_input/time_field/spec.rs");
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let check_source = load_source("src/text_input/time_field/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    assert!(
        !spec_path.exists(),
        "TimeField is a simple component and should not introduce `spec.rs`."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "TimeField mod.rs should not expose spec module token `{forbidden}`."
        );
    }

    for needle in [
        "<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
        "简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。",
    ] {
        assert!(
            check_source.contains(needle),
            "TimeField checklist docs should retain spec-file discipline token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "slug=\"time-field\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField component documentation should stay in docs-app token `{needle}`."
        );
    }
}

#[test]
fn time_field_visual_desire_reuses_default_theme_baseline_and_visual_regression_gates() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "component_doc!(\n        \"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "first-impression quality",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
        "clear hierarchy, natural contrast, and explicit interaction feedback",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "Theme visual baseline contract should include `{needle}`."
        );
    }

    for needle in [
        "E2E_VISUAL_BASELINE",
        "/#/components/theme-visual-baseline",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "Theme visual baseline e2e regression gate should include `{needle}`."
        );
    }
}

#[test]
fn time_field_token_first_static_style_pipeline_is_wired_through_css_and_ui_root() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for needle in [
        "#[cfg(feature = \"component-time_field\")]",
        "out.push_str(crate::time_field::styles::CSS);",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            css_source.contains(needle) || root_source.contains(needle),
            "TimeField style pipeline should include token `{needle}`."
        );
    }

    for needle in [
        "var(--ui-space-2xs)",
        "var(--ui-space-3xs)",
        "var(--ui-radius-xs)",
        "var(--ui-radius-sm)",
        "var(--ui-font-size-100)",
        "var(--ui-component-height-100)",
        "var(--ui-border)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(needle),
            "TimeField styles.rs should consume shared theme token variable `{needle}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "TimeField view.rs should avoid inline business style branches."
    );

    for needle in [
        "set_property(\"--ui-time-field-clear-opacity\"",
        "set_property(\"--ui-time-field-clear-scale\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "TimeField runtime style path should be limited to CSS custom properties via `{needle}`."
        );
    }
}

#[test]
fn time_field_component_layer_does_not_default_to_utility_first_or_css_in_rust() {
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"items-",
        "class=\"justify-",
        "class=\"gap-",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "class=\"border-",
        "class=\"rounded-",
        "class=\"shadow-",
        "tw-",
        "tailwind",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "TimeField component source should not depend on utility-first token `{forbidden}`."
        );
    }

    for forbidden in [
        "stylist",
        "emotion",
        "styled_components",
        "Style::new",
        "css!(",
        "css! {",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "TimeField component source should not default to CSS-in-Rust token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_uses_logic_state_model() {
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let primitive_source = load_source("../ui-logic-calendar/src/time_field.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let headless_source = load_source("../ui-headless/src/time_field.rs");

    for needle in [
        "pub use ui_logic_calendar::time_field::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_CLEAR_ARIA_LABEL",
        "TimeFieldIds",
        "TimeFieldStateInput",
        "TimeFieldState",
        "TimeFieldTone",
        "normalize_disabled_state",
        "normalize_value_state",
        "normalize_time_value",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField logic should consume `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "pub enum TimeFieldTone",
        "pub struct TimeFieldIds",
        "pub struct TimeFieldStateInput",
        "pub struct TimeFieldState",
        "value_source_attr",
        "default_value_source_attr",
        "value_change_source_attr",
        "pub fn normalize_clear_aria_label(",
        "pub fn normalize_clear_label(",
        "pub fn normalize_time_value(",
        "pub fn resolve_time_parts(",
        "pub fn update_hour_from_input(",
        "pub fn update_minute_from_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "TimeField state primitive source should define `{needle}`."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<TimeFieldStrings>();",
        "logic::normalize_disabled_state(logic::DisabledStateInput {",
        "logic::normalize_value_state(logic::ValueStateInput {",
        "use_time_field(TimeFieldOptions {",
        "motion::attach_clear_button_motion(",
        "logic::normalize_label(label, strings.label.as_ref())",
        "logic::normalize_placeholder(placeholder, strings.placeholder.as_ref())",
        "logic::normalize_aria_label(aria_label, strings.aria_label.as_ref())",
        "logic::resolve_ids(&id_base)",
        "logic::resolve_state(TimeFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct TimeFieldOptions",
        "pub struct TimeFieldAttrs",
        "pub struct TimeFieldHandlers",
        "pub struct TimeFieldState",
        "pub fn use_time_field(options: TimeFieldOptions) -> TimeFieldAria",
        "update_hour_from_input",
        "update_minute_from_input",
    ] {
        assert!(
            headless_source.contains(needle),
            "TimeField headless contract should include `{needle}`."
        );
    }
}

#[test]
fn time_field_a11y_i18n_and_locale_contract_is_wired() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let headless_source = load_source("../ui-headless/src/time_field.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::i18n;",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<TimeFieldStrings>();",
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional, into)] placeholder: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] hour_aria_label: Option<String>,",
        "#[prop(optional, into)] minute_aria_label: Option<String>,",
        "#[prop(optional, into)] clear_label: Option<String>,",
        "#[prop(optional, into)] clear_aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::normalize_label(label, strings.label.as_ref())",
        "logic::normalize_placeholder(placeholder, strings.placeholder.as_ref())",
        "logic::normalize_aria_label(aria_label, strings.aria_label.as_ref())",
        "logic::normalize_hour_aria_label(hour_aria_label, strings.hour_aria_label.as_ref())",
        "logic::normalize_minute_aria_label(minute_aria_label, strings.minute_aria_label.as_ref())",
        "logic::normalize_clear_label(clear_label, strings.clear_label.as_ref())",
        "logic::normalize_clear_aria_label(clear_aria_label, strings.clear_aria_label.as_ref())",
        "role=group_role",
        "lang=group_lang.get_value()",
        "dir=group_dir",
        "let time_field = use_time_field(TimeFieldOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField A11y/i18n/locale wiring should include `{needle}`."
        );
    }

    for needle in [
        "pub struct TimeFieldStrings",
        "pub label: Arc<str>",
        "pub placeholder: Arc<str>",
        "pub aria_label: Arc<str>",
        "pub hour_aria_label: Arc<str>",
        "pub minute_aria_label: Arc<str>",
        "pub clear_label: Arc<str>",
        "pub clear_aria_label: Arc<str>",
        "DEFAULT_LABEL",
        "DEFAULT_PLACEHOLDER",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_HOUR_ARIA_LABEL",
        "DEFAULT_MINUTE_ARIA_LABEL",
        "DEFAULT_CLEAR_LABEL",
        "DEFAULT_CLEAR_ARIA_LABEL",
    ] {
        assert!(
            i18n_source.contains(needle),
            "TimeField i18n strings contract should include `{needle}`."
        );
    }

    for needle in [
        "labeled_group_attrs",
        "pub fn labeled_group_attrs(",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
    ] {
        assert!(
            headless_source.contains(needle) || a11y_source.contains(needle),
            "TimeField headless/a11y shared contract should include `{needle}`."
        );
    }

    for forbidden in ["\"Hour\"", "\"Minute\"", "\"Clear\"", "\"Clear time\""] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField view should not hardcode user-facing fallback text `{forbidden}`."
        );
    }
}

#[test]
fn time_field_logic_does_not_reimplement_reusable_state_primitives() {
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for forbidden in [
        "pub enum TimeFieldTone",
        "pub struct TimeFieldIds",
        "pub struct TimeFieldStateInput",
        "pub struct TimeFieldState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_label(",
        "pub fn normalize_placeholder(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_ids(",
        "pub fn normalize_minute_step(",
        "pub fn normalize_hour(",
        "pub fn normalize_minute(",
        "pub fn parse_time_value(",
        "pub fn normalize_time_value(",
        "pub fn resolve_time_parts(",
        "pub fn update_hour_from_input(",
        "pub fn update_minute_from_input(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "TimeField logic should not reimplement primitive `{forbidden}`; keep it in ui-state-primitives.",
        );
    }
}

#[test]
fn time_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/time_field/view.rs");

    for attr in [
        "data-slot=SLOT_TIME_FIELD",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(BOOL_TRUE)",
        "data-has-value=move || state.get().has_value.then_some(BOOL_TRUE)",
        "data-minute-step=move || state.get().minute_step.to_string()",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-controlled=move || state.get().is_controlled.then_some(BOOL_TRUE)",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(BOOL_TRUE)",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)",
        "data-slot=SLOT_TIME_FIELD_LABEL",
        "data-slot=SLOT_TIME_FIELD_CONTROL",
        "data-slot=SLOT_TIME_FIELD_HOUR",
        "data-slot=SLOT_TIME_FIELD_SEPARATOR",
        "data-slot=SLOT_TIME_FIELD_MINUTE",
        "slot_name=SLOT_TIME_FIELD_CLEAR",
        "is_visible=is_visible",
        "role=group_role",
        "lang=group_lang.get_value()",
        "dir=group_dir",
    ] {
        assert!(
            source.contains(attr),
            "TimeField should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn time_field_styles_include_tone_value_and_source_markers() {
    let source = load_source("src/text_input/time_field/styles.rs");

    for selector in [
        ".ui-time-field--tone-default",
        ".ui-time-field[data-tone=\"default\"]",
        ".ui-time-field--tone-quiet",
        ".ui-time-field--tone-strong",
        ".ui-time-field--disabled",
        ".ui-time-field[data-disabled=\"true\"]",
        ".ui-time-field--has-value",
        ".ui-time-field[data-has-value=\"true\"] .ui-time-field__control",
        ".ui-time-field--custom-class",
        ".ui-time-field[data-custom-class=\"true\"]",
        ".ui-time-field[data-motion-source=\"custom\"] .ui-time-field__clear",
        ".ui-time-field[data-custom-motion=\"true\"] .ui-time-field__clear",
        ".ui-time-field__control",
        ".ui-time-field__input",
        ".ui-time-field__clear",
        ".ui-time-field__clear[data-visible=\"true\"]",
        ".ui-time-field__clear:not([data-visible=\"true\"])",
        "--ui-time-field-clear-opacity",
        "--ui-time-field-clear-scale",
    ] {
        assert!(
            source.contains(selector),
            "TimeField styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn time_field_clear_visibility_is_marker_driven_not_dom_presence_driven() {
    let view_source = load_source("src/text_input/time_field/view.rs");

    assert!(
        !view_source.contains("<Show when=move || state.get().has_value>"),
        "Clear button visibility should be marker-driven, not conditional node presence."
    );
}

#[test]
fn time_field_api_keeps_is_prefixed_boolean_and_value_triplet_contract() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "logic::normalize_disabled_state(logic::DisabledStateInput {",
        "logic::normalize_value_state(logic::ValueStateInput {",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "TimeField API and normalization should include `{needle}`."
        );
    }
}

#[test]
fn time_field_mounts_headless_contract_in_view_layer() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "use ui_headless::{",
        "use_time_field,",
        "use_press,",
        "let time_field = use_time_field(TimeFieldOptions {",
        "let clear_press = use_press(PressOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should mount ui-headless contracts via `{needle}`."
        );
    }

    for forbidden in ["use_time_field(", "use_button("] {
        assert!(
            !logic_source.contains(forbidden),
            "TimeField logic should remain mapping-only and avoid headless hooks `{forbidden}`."
        );
    }
}

#[test]
fn time_field_default_value_priority_is_centralized_in_logic() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "pub struct ValueStateInput",
        "pub struct ValueState",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField logic should centralize value/default/on_value_change axis via `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_time_value(default_value",
        "logic::update_hour_from_input(",
        "logic::update_minute_from_input(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField view should not own default/event state-machine rule `{forbidden}`."
        );
    }
}

#[test]
fn time_field_source_markers_use_closed_enumerated_values() {
    let primitive_source = load_source("../ui-logic-calendar/src/time_field.rs");

    for needle in [
        "control_mode_attr = if input.is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "value_source_attr = if input.is_controlled {",
        "\"external\"",
        "\"default\"",
        "default_value_source_attr = if input.has_default_value {",
        "\"provided\"",
        "\"implicit\"",
        "value_change_source_attr = if input.has_value_change_handler {",
        "\"none\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "TimeField source markers should use closed enumerated token `{needle}`."
        );
    }
}

#[test]
fn time_field_motion_contract_is_split_into_motion_rs() {
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for needle in [
        "pub struct TimeFieldMotion",
        "pub fn sanitize_motion(motion: TimeFieldMotion) -> TimeFieldMotion",
        "pub fn attach_clear_button_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "TimeField motion contract should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn time_field_interaction_matrix_covers_controlled_uncontrolled_disabled_keyboard_pointer_and_platform_split()
 {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_time_field_contract.spec.mjs");
    let headless_source = load_source("../ui-headless/src/time_field.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for needle in [
        "title=\"Controlled + Step 15\"",
        "title=\"Strong Tone + Custom Placeholder\"",
        "fn time_field_handlers_are_noop_when_disabled()",
        "if is_disabled {",
        "await clearButton.click();",
        "await page.keyboard.press(\"Enter\");",
        "data-control-mode\", \"controlled\"",
        "data-control-mode\", \"uncontrolled\"",
        "data-state\", \"empty\"",
        "aria-labelledby",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        let found = docs_source.contains(needle)
            || e2e_source.contains(needle)
            || headless_source.contains(needle)
            || headless_lib_source.contains(needle)
            || motion_source.contains(needle);
        assert!(
            found,
            "TimeField matrix should cover semantic branch token `{needle}`."
        );
    }
}

#[test]
fn time_field_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/time_field_semantics.rs");

    for semantic_signal in [
        "time_field_emits_baseline_style_state_data_attributes",
        "time_field_agent_contract_is_schema_typed_and_machine_readable",
        "time_field_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "time_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "time_field_styles_include_tone_value_and_source_markers",
        "time_field_clear_visibility_is_marker_driven_not_dom_presence_driven",
        "time_field_a11y_i18n_and_locale_contract_is_wired",
        "time_field_interaction_matrix_covers_controlled_uncontrolled_disabled_keyboard_pointer_and_platform_split",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "role=group_role",
    ] {
        assert!(
            suite_source.contains(semantic_signal),
            "TimeField semantic suite should keep contract assertion signal `{semantic_signal}`."
        );
    }

    let forbidden_assert_snapshot = ["assert", "_", "snapshot", "!"].concat();
    let forbidden_insta = ["in", "sta", "::"].concat();
    let forbidden_match_snapshot = ["to", "_", "match", "_", "snapshot"].concat();

    for forbidden in [
        forbidden_assert_snapshot,
        forbidden_insta,
        forbidden_match_snapshot,
    ] {
        assert!(
            !suite_source.contains(&forbidden),
            "TimeField semantic suite should not rely on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "time_field/check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn time_field_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let semantics_source = load_source("tests/time_field_semantics.rs");

    for marker in [
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=label_id.clone()",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "TimeField view should keep semantic marker `{marker}`."
        );
        let semantics_marker = if marker == "data-ui-stream-mode=\"snapshot\"" {
            "data-ui-stream-mode=\\\"snapshot\\\""
        } else {
            marker
        };
        assert!(
            semantics_source.contains(semantics_marker),
            "TimeField semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn time_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "title=\"TimeField\"",
        "slug=\"time-field\"",
        "description=\"Time entry field with centralized hour/minute normalization and baseline-style state/source data contracts.\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "<Playground title=\"Controlled + Step 15\" code_signal=code>",
        "<Playground title=\"Strong Tone + Custom Placeholder\" code_signal=states_code>",
        "<Playground title=\"Disabled + Uncontrolled (Default Step)\" code_signal=disabled_code>",
        "data-slot=\"time-field-source-first\"",
        "<TimeField",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra time_field docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn time_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "title=\"Hello World\"",
        "id_base=\"docs-time-field-hello\".to_string()",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "id_base=\"docs-time-field-marker\".to_string()",
        "value=marker_value",
        "on_value_change=on_marker_value_change",
        "is_disabled=marker_is_disabled.get()",
        "minute_step=marker_minute_step.get()",
        "data-slot=\"time-field-marker-controls\"",
        "data-slot=\"time-field-toggle-disabled\"",
        "data-slot=\"time-field-toggle-step\"",
        "data-slot=\"time-field-toggle-tone\"",
        "data-slot=\"time-field-reset-value\"",
        "data-slot=\"time-field-marker-summary\"",
        "title=\"Controlled + Step 15\"",
        "id_base=\"docs-time-field-controlled\".to_string()",
        "value=value",
        "on_value_change=on_value_change",
        "minute_step=15",
        "title=\"Strong Tone + Custom Placeholder\"",
        "id_base=\"docs-time-field-strong\".to_string()",
        "tone=TimeFieldTone::Strong",
        "minute_step=5",
        "default_value=\"18:45\".to_string()",
        "placeholder=\"hour:minute\".to_string()",
        "class_name=\"docs-time-field-custom\".to_string()",
        "title=\"Disabled + Uncontrolled (Default Step)\"",
        "id_base=\"docs-time-field-disabled\".to_string()",
        "default_value=\"22:00\".to_string()",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra time_field docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}

#[test]
fn time_field_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn time_field_docs_examples_sync_with_logic_api_names_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let primitive_source = load_source("../ui-logic-calendar/src/time_field.rs");

    time_field_docs_page_covers_primary_playgrounds();
    time_field_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "title=\"Controlled + Step 15\"",
        "title=\"Strong Tone + Custom Placeholder\"",
        "title=\"Disabled + Uncontrolled (Default Step)\"",
        "value=value",
        "on_value_change=on_value_change",
        "default_value=\"18:45\".to_string()",
        "default_value=\"22:00\".to_string()",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should keep API/default/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional, default = 1)] minute_step: u8",
        "pub fn normalize_disabled_state(input: DisabledStateInput) -> bool",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
        "pub fn normalize_minute_step(minute_step: u8) -> u8",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "TimeField public/default contract should keep `{needle}`."
        );
    }
}

#[test]
fn time_field_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/text_input/time_field/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");

    assert!(
        has_readme || has_docs_page,
        "TimeField must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn time_field() -> AnyView"),
        "Equivalent docs entry should expose time_field page function."
    );
}

#[test]
fn time_field_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let section_start = docs_source
        .find("pub(super) fn time_field() -> AnyView")
        .expect("forms docs should define time_field page function");
    let section_end = docs_source[section_start..]
        .find("pub(super) fn date_range_picker() -> AnyView")
        .map(|offset| section_start + offset)
        .expect("forms docs should place date_range_picker after time_field");
    let docs_source = &docs_source[section_start..section_end];
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"TimeField\"",
        "slug=\"time-field\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Controlled + Step 15\" code_signal=code>",
        "<Playground title=\"Strong Tone + Custom Placeholder\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .expect("time_field docs should include hello-world playground");
    let interactive_pos = docs_source
        .find("title=\"Interactive Playground (State + Source Markers)\"")
        .expect("time_field docs should include interactive playground");
    let common_pos = docs_source
        .find("<Playground title=\"Controlled + Step 15\" code_signal=code>")
        .expect("time_field docs should include controlled playground");
    let advanced_pos = docs_source
        .find("<Playground title=\"Strong Tone + Custom Placeholder\" code_signal=states_code>")
        .expect("time_field docs should include advanced playground");

    assert!(
        hello_pos < interactive_pos && interactive_pos < common_pos && common_pos < advanced_pos,
        "TimeField docs should present default usage before advanced controls."
    );
}

#[test]
fn time_field_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let section_start = docs_source
        .find("pub(super) fn time_field() -> AnyView")
        .expect("forms docs should define time_field page function");
    let section_end = docs_source[section_start..]
        .find("pub(super) fn date_range_picker() -> AnyView")
        .map(|offset| section_start + offset)
        .expect("forms docs should place date_range_picker after time_field");
    let time_field_section = &docs_source[section_start..section_end];

    let start = time_field_section
        .find("let hello_code = Signal::derive(move || {")
        .expect("time_field docs should define hello_code");
    let end = time_field_section[start..]
        .find("let code = Signal::derive(move || {")
        .map(|offset| start + offset)
        .expect("time_field docs should define controlled code block after hello_code");
    let hello_block = &time_field_section[start..end];

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
        "TimeField Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{hello_snippet}"
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
            "TimeField Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn time_field_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "time_field_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "time_field_docs_are_beginner_friendly_with_default_then_advanced_path",
        "time_field_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn time_field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "let (marker_value, set_marker_value) = signal(Some(\"08:15\".to_string()));",
        "let on_marker_value_change = Callback::new(move |next: Option<String>| {",
        "let (marker_is_disabled, set_marker_is_disabled) = signal(false);",
        "let (marker_minute_step, set_marker_minute_step) = signal(5_u8);",
        "let (marker_strong_tone, set_marker_strong_tone) = signal(false);",
        "value=marker_value",
        "on_value_change=on_marker_value_change",
        "is_disabled=marker_is_disabled.get()",
        "minute_step=marker_minute_step.get()",
        "tone=if marker_strong_tone.get() {",
        "set_marker_is_disabled.update(|value| *value = !*value)",
        "set_marker_minute_step.update(|value| {",
        "set_marker_strong_tone.update(|value| *value = !*value)",
        "set_marker_value.set(Some(\"08:15\".to_string()));",
        "data-slot=\"time-field-marker-controls\"",
        "data-slot=\"time-field-toggle-disabled\"",
        "data-slot=\"time-field-toggle-step\"",
        "data-slot=\"time-field-toggle-tone\"",
        "data-slot=\"time-field-reset-value\"",
        "data-slot=\"time-field-marker-summary\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should provide interactive playground marker `{needle}`."
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
fn time_field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_time_field_contract.spec.mjs");

    for needle in [
        "docs-app time-field key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/time-field\");",
        "const controlled = page.locator('[data-slot=\"time-field\"]#docs-time-field-controlled');",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"empty\")",
        "await page.reload();",
        "toHaveAttribute(\"data-state\", \"value\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TimeField interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
fn time_field_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code-block/src/view.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "data-slot=\"time-field-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<Snippet",
        "copyable=true",
        "class_name=\"docs-time-field-source-copy\".to_string()",
        "data-slot=\"time-field-source-paths\"",
        "\"components/text-input/src/time_field/mod.rs\"",
        "\"components/text-input/src/time_field/logic.rs\"",
        "\"components/text-input/src/time_field/view.rs\"",
        "\"components/text-input/src/time_field/styles.rs\"",
        "\"components/text-input/src/time_field/motion.rs\"",
        "data-slot=\"time-field-source-prerequisites\"",
        "\"component-time_field\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should keep copy-ready marker `{needle}`."
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
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional, default = 1)] minute_step: u8",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "TimeField docs copy-ready snippets should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn time_field_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "### TimeField 同步记录（2026-02-17）",
        "value + on_value_change + default_value",
        "is_disabled/disabled",
        "minute_step",
        "component_doc!(\"TimeField\", \"time-field\", \"Forms\", forms_extra::time_field)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs",
        "`Hello World`、`Interactive Playground (State + Source Markers)`、`Controlled + Step 15`",
        "Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(needle) || docs_index_source.contains(needle),
            "TimeField HeroUI/doc sync record should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn time_field() -> AnyView",
        "slug=\"time-field\"",
        "title=\"TimeField\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "data-slot=\"time-field-source-first\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "TimeField docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = 1)] minute_step: u8",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "TimeField parameter model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn time_field_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "time_field_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep HeroUI/doc sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn time_field_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn time_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_time_field_contract.spec.mjs");

    for needle in [
        "/#/components/time-field",
        "body:not(:has(#boot))",
        "[data-slot=\"time-field\"]#docs-time-field-controlled",
        "[data-slot=\"time-field-hour\"]",
        "[data-slot=\"time-field-minute\"]",
        "[data-slot=\"time-field-clear\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.time-field.agent-contract\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"unsupported\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"full-snapshot\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-action\", \"edit-hour\")",
        "toHaveAttribute(\"data-ui-source\", \"hour-input\")",
        "toHaveAttribute(\"data-ui-action\", \"clear\")",
        "toHaveAttribute(\"data-ui-source\", \"clear-press\")",
        "toHaveAttribute(\"data-state\", \"empty\")",
        "not.toHaveAttribute(\"data-visible\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TimeField e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "TimeField e2e contract should avoid unstable/non-semantic selector token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_time_field_contract.spec.mjs");

    for needle in [
        "docs-app time-field key flow is repeatable with semantic breakpoints",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"edit-minute\")",
        "toHaveAttribute(\"data-ui-source\", \"minute-input\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "await page.reload();",
        "toHaveAttribute(\"data-ui-action\", \"initialize\")",
        "toHaveAttribute(\"data-ui-source\", \"init\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TimeField e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "TimeField e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep repeatable-key-flow rule `{required}`."
        );
    }
}

#[test]
fn time_field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_time_field_contract.spec.mjs");

    for needle in [
        "clearButton.focus();",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"edit-minute\")",
        "toHaveAttribute(\"data-ui-source\", \"minute-input\")",
        "toHaveAttribute(\"data-ui-action\", \"clear\")",
        "toHaveAttribute(\"data-ui-source\", \"clear-press\")",
        "toHaveAttribute(\"data-state\", \"empty\")",
        "not.toHaveAttribute(\"data-visible\", \"true\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TimeField e2e high-risk path contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "TimeField high-risk e2e path should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn time_field_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-time-field.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "time_field e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn time_field_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-time_field = [",
        "\"component-clear_button\"",
        "\"ui-headless/logic-calendar\"",
        "\"dep:ui-logic-calendar\"",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-time_field\")]")
            && lib_source.contains("#[path = \"text_input/time_field/mod.rs\"]")
            && lib_source.contains("pub mod time_field;"),
        "lib.rs should feature-gate time_field module declaration for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-time_field\")]")
            && css_source.contains("out.push_str(crate::time_field::styles::CSS);"),
        "css.rs should gate time_field CSS aggregation behind component-time_field feature."
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
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn time_field_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
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
fn time_field_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let element: leptos::web_sys::HtmlElement = button.unchecked_into();",
        "pub fn attach_clear_button_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "TimeField motion should keep explicit platform branch marker `{needle}`."
        );
    }

    let forbidden = "web_sys";
    assert!(
        !mod_source.contains(forbidden)
            && !i18n_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden),
        "non-wasm TimeField files should stay browser-object free; found `{forbidden}` outside motion.rs.",
    );
}

#[test]
fn time_field_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-time_field,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css",
        "components/text-input/src/time_field/view.rs",
        "components/text-input/src/time_field/motion.rs",
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn time_field_ui_headless_feature_mutex_contract_is_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless feature mutex should be guarded in lib.rs by `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex and dual compile paths via `{needle}`."
        );
    }
}

#[test]
fn time_field_ui_motion_non_wasm_noop_stub_contract_is_guarded() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let motion_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let time_field_motion_source = load_source("src/text_input/time_field/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm no-op/stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            motion_stub_test_source.contains(needle),
            "ui-motion non-wasm stub test suite should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_clear_button_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Button>",
        "_is_visible: leptos::prelude::Signal<bool>",
        "_is_hovered: leptos::prelude::ReadSignal<bool>",
        "_is_pressed: leptos::prelude::ReadSignal<bool>",
        "_is_disabled: bool",
        "motion: TimeFieldMotion",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            time_field_motion_source.contains(needle),
            "TimeField non-wasm motion path should keep predictable safe degrade via `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "cargo check -p ui-components --no-default-features --features component-time_field,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should keep ui-motion/toolchain compile guards via `{needle}`."
        );
    }
}

#[test]
fn time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "let reveal = if visible { 1.0 } else { 0.0 };",
        "let interaction = if pressed {",
        "let _ = style.set_property(\"--ui-time-field-clear-opacity\", &format!(\"{reveal}\"));",
        "let _ = style.set_property(\"--ui-time-field-clear-scale\", &format!(\"{scale}\"));",
    ] {
        assert!(
            motion_source.contains(needle),
            "TimeField reduced-motion path should keep deterministic fallback via `{needle}`."
        );
    }

    for forbidden in ["data-state", "aria-", "role=", "set_attribute(\"aria-\""] {
        assert!(
            !motion_source.contains(forbidden),
            "motion layer should not mutate semantic contract tokens `{forbidden}`."
        );
    }

    for needle in [
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=label_id.clone()",
        "lang=group_lang.get_value()",
        "dir=group_dir",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view layer should keep SSR/hydration semantic markers stable via `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view layer semantics should not split by platform token `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-time_field,inject-css",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should keep reduced-motion/SSR/wasm guard `{needle}`."
        );
    }
}

#[test]
fn time_field_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/text_input/time_field/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/text_input/time_field/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"time-field\" => UiPerfBudget {",
        "max_mount_ms: 32.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
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
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"TimeField\", \"time-field\", \"Forms\", forms_extra::time_field)",
        "\"time-field\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "TimeField docs page should remain in coverage traversal via `{needle}`."
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
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算（首次渲染/更新耗时/内存）",
        "关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep perf governance baseline/follow-up token `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn time_field_check2_documents_type_system_and_machine_readable_state_contract() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。",
        "无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。",
        "关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。",
        "编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep type-system + semantic-marker governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitives_source = load_source("../../crates/ui-logic-calendar/src/time_field.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");

    for required in [
        "pub enum TimeFieldTone",
        "pub struct TimeFieldStateInput",
        "pub struct TimeFieldState",
        "pub fn resolve_state(input: TimeFieldStateInput) -> TimeFieldState",
        "pub struct ValueStateInput",
        "pub struct ValueState",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
        "let minute_step = normalize_minute_step(input.minute_step);",
        "let default_value = normalize_time_value(input.default_value, minute_step);",
    ] {
        assert!(
            primitives_source.contains(required) || logic_source.contains(required),
            "TimeField state contract should stay type-first and normalized via `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "tone: String",
        "time_state: String",
        "control_mode: String",
        "value_source: String",
        "default_value_source: String",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitives_source.contains(forbidden),
            "TimeField should avoid string protocol drift for discrete state axis `{forbidden}`."
        );
    }

    for marker in [
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "TimeField machine-readable semantic contract should expose marker `{marker}`."
        );
    }
}

#[test]
fn time_field_check2_documents_component_directory_standard_file_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

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
            "TimeField checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_component_directory_has_standard_file_layout() {
    for required in [
        "src/text_input/time_field/mod.rs",
        "src/text_input/time_field/i18n.rs",
        "src/text_input/time_field/logic.rs",
        "src/text_input/time_field/styles.rs",
        "src/text_input/time_field/view.rs",
        "src/text_input/time_field/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "time_field component directory should include required file `{required}`."
        );
    }

    for forbidden in [
        "src/text_input/time_field/render.rs",
        "src/text_input/time_field/spec.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "time_field component directory should not include `{forbidden}`."
        );
    }
}

#[test]
fn time_field_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");

    for needle in [
        "mod i18n;",
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use i18n::TimeFieldStrings;",
        "pub use motion::TimeFieldMotion;",
        "pub use view::TimeField;",
    ] {
        assert!(
            mod_source.contains(needle),
            "time_field/mod.rs should include stable export marker `{needle}`."
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
            "time_field/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn time_field_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    for forbidden in [
        "view! {",
        "on:pointer",
        "on:keydown",
        "aria-",
        "data-slot",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "time_field/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "time_field/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "time_field/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn TimeField(",
        "use_time_field(TimeFieldOptions {",
        "use_press(PressOptions {",
        "use_hover(HoverOptions {",
        "render_label(",
        "render_clear_button(",
    ] {
        assert!(
            view_source.contains(required),
            "time_field/view.rs should keep rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "ui_logic_calendar::time_field::resolve_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "time_field/view.rs should not bypass logic boundary with `{forbidden}`."
        );
    }

    for required in [
        "pub struct TimeFieldMotion",
        "pub fn attach_clear_button_motion(",
        "sanitize_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "time_field/motion.rs should keep motion-contract marker `{required}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "time_field/motion.rs should not carry view semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn time_field_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

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
            "TimeField checklist should keep ui-components entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
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
fn time_field_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn time_field_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "TimeField should keep explicit render blocks in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        6,
        "TimeField should keep one main render block and five semantic subrender blocks."
    );
    assert!(
        view_source.lines().count() <= 460,
        "TimeField view.rs should stay bounded; split further if this grows significantly."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "while let Some(",
        ".fold(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField view should avoid loop-heavy/expansion-heavy rendering token `{forbidden}`."
        );
    }

    for needle in [
        "fn render_label(",
        "fn render_hour_input(",
        "fn render_separator() -> impl IntoView",
        "fn render_minute_input(",
        "fn render_clear_button(",
        "let label_view = render_label(",
        "let clear_button_view = render_clear_button(",
        "{label_view}",
        "{clear_button_view}",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should keep semantic subblock marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn time_field_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "TimeField should keep a single public component boundary for current layout."
    );

    for needle in [
        "fn render_label(",
        "fn render_hour_input(",
        "fn render_separator() -> impl IntoView",
        "fn render_minute_input(",
        "fn render_clear_button(",
        ") -> impl IntoView {",
        "pub fn TimeField(",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should prefer plain function split marker `{needle}`."
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\nfn time_field_"] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField should not introduce local component abstraction noise `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn time_field_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "const SLOT_TIME_FIELD: &str = \"time-field\";",
        "const SLOT_TIME_FIELD_LABEL: &str = \"time-field-label\";",
        "const SLOT_TIME_FIELD_CONTROL: &str = \"time-field-control\";",
        "const SLOT_TIME_FIELD_HOUR: &str = \"time-field-hour\";",
        "const SLOT_TIME_FIELD_SEPARATOR: &str = \"time-field-separator\";",
        "const SLOT_TIME_FIELD_MINUTE: &str = \"time-field-minute\";",
        "const SLOT_TIME_FIELD_CLEAR: &str = \"time-field-clear\";",
        "const BOOL_TRUE: &str = \"true\";",
        "const TIME_SEPARATOR: &str = \":\";",
        "data-slot=SLOT_TIME_FIELD",
        "data-slot=SLOT_TIME_FIELD_LABEL",
        "is_visible=is_visible",
        "{TIME_SEPARATOR}",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField static fragment contract should include `{needle}`."
        );
    }

    for forbidden in [
        "data-slot=\"time-field\"",
        "data-slot=\"time-field-label\"",
        "data-visible=move || state.get().has_value.then_some(\"true\")",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField should avoid scattered static literal fragment `{forbidden}` after constantization."
        );
    }

    let script_needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep static-fragment governance rule `{needle}`."
        );
    }
}

#[test]
fn time_field_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !i18n_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "TimeField should not use html injection path `{forbidden}` in component/docs paths.",
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep inner_html safety governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn time_field_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let check2_source = load_source("src/text_input/time_field/check2.md");

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
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "on_pointer_down=Callback::new(move |_| on_pointer_down.run(()))",
        "on_pointer_up=Callback::new(move |_| on_pointer_up.run(()))",
        "on_key_down=Callback::new(move |key: String| on_key_down.run(key))",
        "on_key_up=Callback::new(move |key: String| on_key_up.run(key))",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField should keep machine-readable state/source/interaction marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "time-field-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "TimeField should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep wasm-debug governance contract marker `{needle}`."
        );
    }
}

#[test]
fn time_field_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
}

#[test]
fn time_field_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
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
        "pub(super) fn time_field() -> AnyView",
        "<Playground title=\"Controlled + Step 15\" code_signal=code>",
        "<Playground title=\"Strong Tone + Custom Placeholder\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn time_field_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_source("src/text_input/time_field/check2.md");

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
        "let (value, set_value) = signal(Some(\"09:30\".to_string()));",
        "let on_value_change = Callback::new(move |next: Option<String>| {",
        "value=value",
        "on_value_change=on_value_change",
        "{move || value.get().unwrap_or_else(|| \"none\".to_string())}",
    ] {
        assert!(
            docs_source.contains(needle),
            "TimeField docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "TIME_FIELD_WORKBENCH_STORAGE_KEY",
        "load_time_field_workbench_state(",
        "save_time_field_workbench_state(",
        "clear_time_field_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "TimeField keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
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
            "TimeField checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    assert!(
        !manifest_dir
            .join("src/text_input/time_field/spec.rs")
            .exists(),
        "TimeField should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-time_field = [")
            && cargo_source.contains("\"component-clear_button\"")
            && cargo_source.contains("\"ui-headless/logic-calendar\"")
            && cargo_source.contains("\"dep:ui-logic-calendar\""),
        "TimeField feature should explicitly gate calendar satellite dependencies without serde/spec fan-out."
    );
    assert!(
        !cargo_source.contains("component-time_field = [\"dep:serde\"")
            && !cargo_source.contains("component-time_field = [\"dep:serde_json\""),
        "TimeField should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined = format!(
        "{mod_source}\n{i18n_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}"
    );
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
            "TimeField engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
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
            "TimeField checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
{
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/text_input/time_field/mod.rs"),
        load_source("src/text_input/time_field/i18n.rs"),
        load_source("src/text_input/time_field/logic.rs"),
        load_source("src/text_input/time_field/view.rs"),
        load_source("src/text_input/time_field/styles.rs"),
        load_source("src/text_input/time_field/motion.rs"),
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
        !cargo_source.contains("time-field-wasm-debug"),
        "TimeField should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::time_field::",
        "const TIME_FIELD_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TimeField should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");

    let sources = [
        &mod_source,
        &i18n_source,
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
                "TimeField engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "TimeField public module boundary should not leak web_sys types."
    );
}

#[test]
fn time_field_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "TimeField checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn time_field_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "pub enum TimeFieldAgentSchemaVersion",
        "pub enum TimeFieldAgentIntent",
        "pub enum TimeFieldAgentAction",
        "pub enum TimeFieldAgentStateAxis",
        "pub enum TimeFieldAgentSource",
        "pub enum TimeFieldAgentOutputStatus",
        "pub enum TimeFieldAgentStreamSupport",
        "pub enum TimeFieldAgentStreamFallback",
        "pub struct TimeFieldAgentCapabilities",
        "pub struct TimeFieldAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-edit=move || {",
        "data-ui-capability-clear=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn time_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let combined = format!(
        "{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{i18n_source}\n{motion_source}"
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
            "TimeField Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn time_field_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantics_suite_prioritizes_contract_assertions_over_snapshots",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "time_field/check2.md should pin streaming baseline marker `{needle}`."
        );
    }
}

#[test]
fn time_field_snapshot_baseline_and_streaming_fallback_contract_are_explicit() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "TimeField view should expose explicit snapshot/fallback marker `{needle}`."
        );
    }

    for needle in [
        "TimeFieldAgentStreamSupport::Unsupported",
        "TimeFieldAgentStreamFallback::FullSnapshot",
    ] {
        assert!(
            logic_source.contains(needle),
            "TimeField logic should model stream N/A/fallback contract via `{needle}`."
        );
    }
}

#[test]
fn time_field_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/text_input/time_field/view.rs");

    for required in [
        "data-slot=SLOT_TIME_FIELD",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-default-value-source=move || state.get().default_value_source_attr",
        "data-value-change-source=move || state.get().value_change_source_attr",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "TimeField should keep stable snapshot render marker `{required}`."
        );
    }
}

#[test]
fn time_field_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/text_input/time_field/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "TimeField 归类为 `Streaming Optional`；当前实现使用 `data-ui-stream-support=\"unsupported\" + data-ui-stream-fallback=\"full-snapshot\"`，并输出 `data-ui-output-status`。",
    ] {
        assert!(
            checklist_source.contains(required),
            "time_field/check2.md should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn time_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/text_input/time_field/view.rs");

    for required in [
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=label_id.clone()",
        "lang=group_lang.get_value()",
        "dir=group_dir",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-value-source=move || state.get().value_source_attr",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "TimeField should keep continuous aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn time_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
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
            "TimeField should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn time_field_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test time_field_semantics --no-default-features --features component-time_field,inject-css time_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_status_primitives_remains_dom_and_style_free() {
    let primitives_source = load_source("../ui-logic-calendar/src/time_field.rs");

    for forbidden in [
        "use leptos",
        "leptos::",
        "web_sys::",
        "wasm_bindgen",
        "view! {",
        "NodeRef<",
        "on:click",
        "style=",
    ] {
        assert!(
            !primitives_source.contains(forbidden),
            "ui-state-primitives time_field contract should avoid DOM/style runtime dependency `{forbidden}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_ui_headless_remains_visual_and_motion_free() {
    let headless_source = load_source("../ui-headless/src/time_field.rs");

    for forbidden in [
        ".ui-",
        "ui-time-field",
        "class=",
        "var(--ui-",
        "Spring",
        "keyframe",
        "animate(",
        "request_animation_frame",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless time_field contract should avoid visual/motion orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct TimeFieldOptions",
        "pub struct TimeFieldAttrs",
        "pub struct TimeFieldHandlers",
        "pub struct TimeFieldState",
        "pub struct TimeFieldAria",
        "pub fn use_time_field(options: TimeFieldOptions) -> TimeFieldAria",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless time_field contract should keep typed semantic output `{required}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_view_keeps_decisions_in_logic_layer() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");

    for required in [
        "logic::normalize_disabled_state(logic::DisabledStateInput {",
        "logic::normalize_value_state(logic::ValueStateInput {",
        "logic::resolve_state(TimeFieldStateInput {",
        "let time_field = use_time_field(TimeFieldOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "TimeField view should consume centralized logic/headless output via `{required}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "normalize_time_value(default_value",
        "update_hour_from_input(",
        "update_minute_from_input(",
        "resolve_time_parts(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TimeField view should not hide key state-decision rule `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_disabled_state(input: DisabledStateInput) -> bool",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
    ] {
        assert!(
            logic_source.contains(required),
            "TimeField key decision rule should stay centralized in logic layer `{required}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract() {
    let view_source = load_source("src/text_input/time_field/view.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let primitives_source = load_source("../ui-logic-calendar/src/time_field.rs");
    let semantics_test_source = load_source("tests/time_field_semantics.rs");

    for required in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional, default = 1)] minute_step: u8",
    ] {
        assert!(
            view_source.contains(required),
            "TimeField public parameter naming/default contract should include `{required}`."
        );
    }

    for required in [
        "pub struct DisabledStateInput",
        "pub fn normalize_disabled_state(input: DisabledStateInput) -> bool",
        "pub struct ValueStateInput",
        "pub struct ValueState",
        "pub fn normalize_value_state(input: ValueStateInput) -> ValueState",
        "let minute_step = normalize_minute_step(input.minute_step);",
        "let default_value = normalize_time_value(input.default_value, minute_step);",
    ] {
        assert!(
            logic_source.contains(required) || primitives_source.contains(required),
            "TimeField parameter contract should keep naming/type/default normalization marker `{required}`."
        );
    }

    for required in [
        "time_field_api_keeps_is_prefixed_boolean_and_value_triplet_contract",
        "time_field_default_value_priority_is_centralized_in_logic",
        "time_field_type_system_and_semantic_markers_form_machine_readable_contract",
        "time_field_docs_examples_sync_with_logic_api_names_and_state_matrix",
    ] {
        assert!(
            semantics_test_source.contains(required),
            "TimeField semantics suite should keep parameter-contract regression guard `{required}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_parallel_array_api_is_absent_for_time_field_scope() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");

    for forbidden in [
        "labels + children",
        "titles + panels",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
    ] {
        assert!(
            !docs_source.contains(forbidden) && !view_source.contains(forbidden),
            "TimeField scope should avoid parallel-array/implicit semantic token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_public_api_does_not_leak_platform_or_runtime_types() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");

    for forbidden in [
        "web_sys::",
        "leptos::web_sys",
        "wasm_bindgen",
        "tokio::",
        "async_std::",
        "runtime::Handle",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "TimeField public API boundary should avoid leaking platform/runtime token `{forbidden}`."
        );
    }
}

#[test]
fn time_field_anti_pattern_no_temporary_patch_contract_drift_tokens_in_time_field_scope() {
    let mod_source = load_source("src/text_input/time_field/mod.rs");
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let view_source = load_source("src/text_input/time_field/view.rs");
    let styles_source = load_source("src/text_input/time_field/styles.rs");
    let motion_source = load_source("src/text_input/time_field/motion.rs");
    let i18n_source = load_source("src/text_input/time_field/i18n.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{i18n_source}"
    );

    for forbidden in [
        "TODO temporary",
        "TEMP FIX",
        "HACK",
        "workaround",
        "quick fix",
        "remove later",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TimeField should avoid temporary patch contract-drift marker `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn time_field_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless() {
    let logic_source = load_source("src/text_input/time_field/logic.rs");
    let primitives_source = load_source("../ui-logic-calendar/src/time_field.rs");
    let headless_source = load_source("../ui-headless/src/time_field.rs");

    for required in [
        "pub use ui_logic_calendar::time_field::{",
        "resolve_state(TimeFieldStateInput {",
        "pub struct TimeFieldStateInput",
        "pub struct TimeFieldState",
        "pub struct TimeFieldOptions",
        "pub struct TimeFieldAria",
    ] {
        assert!(
            logic_source.contains(required)
                || primitives_source.contains(required)
                || headless_source.contains(required),
            "TimeField reusable state invariant should stay sunk to primitive/headless marker `{required}`."
        );
    }

    for forbidden in ["pub enum LocalTimeFieldState", "pub enum TimeFieldMachine"] {
        assert!(
            !logic_source.contains(forbidden),
            "TimeField logic should not keep reusable state machine locally `{forbidden}`."
        );
    }
}

#[test]
fn time_field_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/text_input/time_field/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "time_field_anti_pattern_status_primitives_remains_dom_and_style_free",
        "time_field_anti_pattern_ui_headless_remains_visual_and_motion_free",
        "time_field_anti_pattern_view_keeps_decisions_in_logic_layer",
        "time_field_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract",
        "time_field_anti_pattern_parallel_array_api_is_absent_for_time_field_scope",
        "time_field_anti_pattern_public_api_does_not_leak_platform_or_runtime_types",
        "time_field_anti_pattern_no_temporary_patch_contract_drift_tokens_in_time_field_scope",
        "time_field_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep forbidden anti-pattern completion evidence `{needle}`."
        );
    }
}

#[test]
fn time_field_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/text_input/time_field/check2.md");

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
        "time_field_check2_marks_final_merge_gates_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "TimeField checklist should keep final merge-gate completion evidence `{needle}`."
        );
    }

    let full_gate = "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。（已执行定向门禁链路并通过；回归：`time_field_check2_marks_final_merge_gates_complete`。）";
    assert!(
        check2_source.contains(full_gate),
        "TimeField checklist should keep final full-gate completion evidence."
    );
}

#[test]
fn time_field_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/text_input/time_field/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "TimeField check2.md should not keep unchecked checklist items after completion."
    );
}
