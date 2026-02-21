use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn date_input_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "DateInputGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn date_input_group_uses_state_primitives_and_keeps_logic_as_assembly() {
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub use ui_state_primitives::date_input_group::{",
        "DateInputGroupStateInput",
        "DateInputGroupState",
        "DateInputGroupVariant",
        "DateInputGroupWidth",
        "DateInputGroupStatus",
        "normalize_aria_label",
        "normalize_optional_text",
        "resolve_state",
        "resolve_width",
        "resolve_status",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should include `{needle}` for primitives delegation and assembly."
        );
    }

    for forbidden in [
        "pub struct DateInputGroupStateInput {",
        "pub struct DateInputGroupState {",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "DateInputGroup module should not implement local state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn date_input_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"date-input-group\"",
        "data-slot=\"date-input-group-prefix\"",
        "data-slot=\"date-input-group-input\"",
        "data-slot=\"date-input-group-segment\"",
        "data-slot=\"date-input-group-suffix\"",
        "data-variant=move || state.get().variant_attr",
        "data-width=move || state.get().width_attr",
        "data-state=move || state.get().data_state_attr",
        "data-full-width=move || state.get().is_full_width.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-segmented=move || state.get().is_segmented.then_some(\"true\")",
        "data-has-prefix=move || state.get().has_prefix.then_some(\"true\")",
        "data-has-suffix=move || state.get().has_suffix.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=custom_motion_attr",
    ] {
        assert!(
            source.contains(attr),
            "DateInputGroup should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn date_input_group_styles_include_variant_and_state_markers() {
    let source = load_source("src/styles.rs");

    for selector in [
        ".ui-date-input-group--variant-primary",
        ".ui-date-input-group[data-variant=\"secondary\"]",
        ".ui-date-input-group--full-width",
        ".ui-date-input-group[data-width=\"full\"]",
        ".ui-date-input-group--disabled",
        ".ui-date-input-group[data-disabled=\"true\"]",
        ".ui-date-input-group--invalid",
        ".ui-date-input-group[data-invalid=\"true\"]",
        ".ui-date-input-group--segmented .ui-date-input-group__segment",
        ".ui-date-input-group[data-segmented=\"true\"] .ui-date-input-group__segment",
        ".ui-date-input-group--custom-class",
        ".ui-date-input-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "DateInputGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn date_input_group_styles_consume_ui_theme_tokens_without_theme_reconstruction() {
    let styles = load_source("src/styles.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_map = load_source("../../crates/ui-theme/src/theme.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
    ] {
        assert!(
            styles.contains(needle),
            "DateInputGroup styles should consume ui-theme variables via `{needle}`."
        );
    }

    for needle in [
        "--ui-border-width:",
        "--ui-fallback-border-width:",
        "--ui-component-height-100:",
        "--ui-fallback-component-height-100:",
        "--ui-font-size-150:",
        "--ui-fallback-font-size-150:",
        "--ui-line-height-150:",
        "--ui-fallback-line-height-150:",
        "pub struct ComponentLayoutTokens",
        "component_height_100_px",
        "pub struct TypographyTokens",
        "font_size_150_px",
        "line_height_150_px",
        "pub struct ThemeContext",
        "pub scale: ThemeScale",
    ] {
        assert!(
            theme_css.contains(needle)
                || theme_tokens.contains(needle)
                || theme_map.contains(needle)
                || styling_spec.contains(needle),
            "ui-theme contract should expose `{needle}` for date_input_group token-first styling."
        );
    }

    for forbidden in [
        "min-height: 2.25rem;",
        "min-width: 2.25rem;",
        "font-size: var(--ui-font-size-150, 14px);",
        "line-height: var(--ui-line-height-150, 20px);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "DateInputGroup styles should not keep local visual constants once ui-theme tokens exist: `{forbidden}`."
        );
    }

    for forbidden in [":root {", "--ui-system:", "color-scheme:"] {
        assert!(
            !styles.contains(forbidden),
            "DateInputGroup component styles should not rebuild theme CSS root contracts: `{forbidden}`."
        );
    }

    for forbidden in [
        "ThemeContext",
        "Theme::new(",
        "Theme::light(",
        "to_css_variables(",
        "theme_to_css_variables(",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "DateInputGroup logic/view should consume theme context from app root, not reconstruct it via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles = load_source("src/styles.rs");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-date-input-group-scale,",
        "var(--ui-alert-scale, var(--ui-fallback-alert-scale))",
    ] {
        assert!(
            styles.contains(required),
            "date-input-group styles should keep defensive fallback chain marker `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-component-height-100, 2.25rem)",
        "var(--ui-border-width, 1px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "var(--ui-date-input-group-scale, 1)",
    ] {
        assert!(
            !styles.contains(forbidden),
            "date-input-group styles should not keep local hardcoded fallback terminal `{forbidden}`."
        );
    }

    assert!(
        !styles.contains('#'),
        "date-input-group styles should not include hardcoded hex color literals."
    );

    for required in [
        "--ui-fallback-component-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-danger:",
        "--ui-fallback-accent:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-alert-scale:",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme SSOT fallback output should provide `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/date-input-group/test/semantics.rs::date_input_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should pin defensive-variable contract marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should pin defensive-variable contract marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let components_css_source = load_source("../../crates/ui-components/src/css.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-date_input_group\")]",
        "out.push_str(crate::text_input::date_input_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            components_css_source.contains(marker),
            "ui-components css aggregation should keep @layer ui marker `{marker}`."
        );
    }

    for marker in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(marker),
            "UiRoot should inject aggregated component styles through css.rs via `{marker}`."
        );
    }

    for forbidden in [
        "style=\"top: 10px\"",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=move || format!(\"top:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !motion_source.contains(forbidden),
            "date-input-group should not use ordinary inline style mutation `{forbidden}`."
        );
    }

    for marker in [
        "style.set_property(\"--ui-date-input-group-scale\", \"1\")",
        "style.set_property(\"--ui-date-input-group-scale\", &format!(\"{}\", motion.enter_scale))",
        "style.set_property(\"--ui-date-input-group-scale\", &format!(\"{scale}\"))",
    ] {
        assert!(
            motion_source.contains(marker),
            "date-input-group runtime style updates should stay css-variable-only via `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "components/date-input-group/test/semantics.rs::date_input_group_cascade_layer_contract_uses_ui_layer_and_css_variable_only_runtime_updates",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should pin cascade-layer contract marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should pin cascade-layer contract marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_supports_group_accessibility_and_children_layout() {
    let source = load_source("src/view.rs");

    for needle in [
        "DateInputGroupMotion",
        "motion as date_input_group_motion",
        "#[prop(optional)] motion: DateInputGroupMotion,",
        "let motion_source_attr = if motion == DateInputGroupMotion::default() {",
        "let custom_motion_attr = (motion_source_attr == \"custom\").then_some(\"true\");",
        "let node_ref: NodeRef<html::Div> = NodeRef::new();",
        "date_input_group_motion::attach_motion(node_ref, motion);",
        "node_ref=node_ref",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));",
        "<div",
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "{children()}",
    ] {
        assert!(
            source.contains(needle),
            "DateInputGroup should include `{needle}` for accessibility and composition."
        );
    }

    assert!(
        !source.contains("role=\"group\""),
        "DateInputGroup should not hardcode group role in view; use headless a11y contract."
    );
}

#[test]
fn date_input_group_public_api_bool_props_follow_is_prefix_contract() {
    let view_source = load_source("src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "#[prop(optional)] is_full_width: bool,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_segmented: bool,",
        "width: logic::resolve_width(is_full_width),",
        "status: logic::resolve_status(is_disabled, is_invalid),",
        "is_segmented,",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup public bool prop naming should use is_* contract via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] full_width: bool,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] invalid: bool,",
        "#[prop(optional)] segmented: bool,",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup public bool prop naming should not keep old aliases: `{forbidden}`."
        );
    }

    for needle in ["is_full_width=true", "is_invalid=true", "is_segmented=true"] {
        assert!(
            docs_source.contains(needle),
            "docs DateInputGroup usage should align with is_* API naming via `{needle}`."
        );
    }

    for forbidden in [
        "<DateInputGroup\n  full_width=true",
        "<DateInputGroup\n  aria_label=\"Invoice date controls\".to_string()\n  segmented=true",
        "variant=DateInputGroupVariant::Secondary\n  invalid=true\n  segmented=true",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "docs DateInputGroup usage should not keep pre-contract naming pattern `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_uses_enum_constraints_for_discrete_state_axes() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/date_input_group.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub enum DateInputGroupVariant {",
        "pub enum DateInputGroupWidth {",
        "pub enum DateInputGroupStatus {",
        "pub width: DateInputGroupWidth,",
        "pub status: DateInputGroupStatus,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "DateInputGroup primitives should type discrete axes with enums via `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::date_input_group::{",
        "resolve_width",
        "resolve_status",
        "width: input.width,",
        "status: input.status,",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should map boolean API input into enum-based discrete state via `{needle}`."
        );
    }

    for needle in [
        "width: logic::resolve_width(is_full_width),",
        "status: logic::resolve_status(is_disabled, is_invalid),",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should forward discrete axis mapping through logic helpers via `{needle}`."
        );
    }

    for forbidden in [
        "width: if is_full_width",
        "status: if is_disabled",
        "status: if is_invalid",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup view should not handcraft discrete-state branching via `{forbidden}`."
        );
    }

    for forbidden in [
        "pub fn resolve_width(is_full_width: bool) -> DateInputGroupWidth {",
        "pub fn resolve_status(is_disabled: bool, is_invalid: bool) -> DateInputGroupStatus {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateInputGroup logic should not reimplement primitive mapping helpers via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_consumes_state_primitives_without_business_store_binding() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let module_source = load_source("src/mod.rs");

    for needle in [
        "pub use ui_state_primitives::date_input_group::{",
        "DateInputGroupStateInput",
        "DateInputGroupState",
        "DateInputGroupWidth",
        "DateInputGroupStatus",
        "resolve_state",
        "resolve_width",
        "resolve_status",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should consume state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct DateInputGroupStateInput {",
        "pub struct DateInputGroupState {",
        "pub enum DateInputGroupWidth {",
        "pub enum DateInputGroupStatus {",
        "pub fn resolve_width(is_full_width: bool) -> DateInputGroupWidth {",
        "pub fn resolve_status(is_disabled: bool, is_invalid: bool) -> DateInputGroupStatus {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateInputGroup logic should not reimplement primitive contracts via `{forbidden}`."
        );
    }

    for forbidden in [
        "AppStore",
        "GlobalState",
        "GlobalStore",
        "use_app_store",
        "use_global_store",
        "crate::app::state",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !module_source.contains(forbidden),
            "DateInputGroup should not bind business store types directly via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_has_no_async_interaction_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "#[prop(optional)] is_loading: bool,",
        "#[prop(optional)] on_retry:",
        "#[prop(optional)] on_error:",
        "aria-busy",
        "data-loading",
        "data-error",
        "data-retry",
        "use_async_action",
        "spawn_local(",
        "tokio::spawn(",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "DateInputGroup should not expose async interaction protocol via `{forbidden}`."
        );
    }

    for needle in [
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep synchronous disabled semantics via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_local_controlled_uncontrolled_value_axis() {
    let view_source = load_source("src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "#[prop(optional, into)] value:",
        "#[prop(optional, into)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup should not fake a local controllable value axis via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "children: Children,",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as local-state-free assembly surface via `{needle}`."
        );
    }

    for needle in [
        "<DateField",
        "value=invoice_date",
        "on_value_change=on_invoice_date_change",
        "<TimeField",
        "value=ship_window",
        "on_value_change=on_ship_window_change",
    ] {
        assert!(
            docs_source.contains(needle),
            "Controlled/uncontrolled value contract should stay on child field components via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_normalizes_default_sources_in_logic_only() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub fn resolve_motion_source_attrs(",
        "DateInputGroupMotion::default()",
        "\"default\"",
        "\"custom\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should own default-source normalization via `{needle}`."
        );
    }

    assert!(
        view_source.contains("logic::resolve_motion_source_attrs(motion)"),
        "DateInputGroup view should consume motion default normalization from logic."
    );

    for forbidden in [
        "if motion == DateInputGroupMotion::default()",
        "unwrap_or(",
        "unwrap_or_else(",
        ".or_else(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup view should not perform fallback/default branching via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_centralizes_state_normalization_in_logic() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");

    for needle in [
        "pub struct DateInputGroupStateDeriveInput {",
        "pub fn derive_state(input: DateInputGroupStateDeriveInput) -> DateInputGroupState {",
        "resolve_state(DateInputGroupStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should centralize typed state derivation via `{needle}`."
        );
    }

    for needle in [
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should consume logic state output via `{needle}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(DateInputGroupStateInput {",
        "DateInputGroupStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup view should not rebuild state machine input via `{forbidden}`."
        );
    }

    for forbidden in [
        ".ui-date-input-group[data-state=\"disabled-invalid\"]",
        "data-state=\"disabled-invalid\"",
        "data-state=\"invalid\"",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "DateInputGroup styles should consume state markers only, not judge state logic via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_motion_contract_delegates_to_ui_motion_and_has_non_wasm_stub() {
    let source = load_source("src/motion.rs");

    for needle in [
        "ui_motion::presets::spring_soft()",
        "ui_motion::spring::sanitize_config(value, default)",
        "ui_motion::spring::SpringAnimator::new(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "DateInputGroup motion contract should include `{needle}`."
        );
    }
}

#[test]
fn date_input_group_stays_as_ui_components_assembly_layer() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{DEFAULT_ARIA_LABEL, DateInputGroupVariant};",
        "pub use motion::DateInputGroupMotion;",
        "pub use ui_state_primitives::date_input_group::{DateInputGroupState, DateInputGroupStateInput};",
        "pub use view::DateInputGroup;",
        "#[cfg(test)]",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module_source.contains(needle),
            "DateInputGroup module boundary should include `{needle}`."
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "HtmlElement"] {
        assert!(
            !module_source.contains(forbidden),
            "DateInputGroup public module boundary should not expose platform DOM details: `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::date_input_group::{",
        "resolve_state",
        "resolve_width",
        "resolve_status",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic should stay as primitive assembly layer via `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));",
        "role=move || group_a11y.get_value().role",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should mount headless contracts via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-date-input-group[data-variant=\"secondary\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateInputGroup styles should remain token-first static css via `{needle}`."
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(needle),
            "DateInputGroup motion contract should stay in motion.rs via `{needle}`."
        );
    }

    for forbidden in ["view!", "data-slot=", "role=move ||"] {
        assert!(
            !motion_source.contains(forbidden),
            "DateInputGroup motion.rs should not absorb view-level semantics: `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_ui_theme_boundary_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `ui-theme` 定义",
        "components/date-input-group/src/styles.rs",
        "date_input_group_styles_consume_ui_theme_tokens_without_theme_reconstruction",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin ui-theme completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin ui-theme completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_ui_components_boundary_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `ui-components` 定义",
        "components/date-input-group/src/mod.rs",
        "components/date-input-group/test/semantics.rs",
        "date_input_group_stays_as_ui_components_assembly_layer",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin ui-components completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin ui-components completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_api_naming_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] API 命名契约统一",
        "`DateInputGroup` 公共布尔 props 已统一为 `is_full_width/is_disabled/is_invalid/is_segmented`",
        "date_input_group_public_api_bool_props_follow_is_prefix_contract",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin api naming completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin api naming completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_controlled_uncontrolled_pairing_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 受控/非受控必须成对",
        "N/A：`DateInputGroup` 自身不持有可变业务值状态轴",
        "date_input_group_has_no_local_controlled_uncontrolled_value_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin controlled-uncontrolled completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin controlled-uncontrolled completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_default_source_singleton_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 默认值单一来源",
        "components/date-input-group/src/logic.rs::resolve_motion_source_attrs",
        "date_input_group_normalizes_default_sources_in_logic_only",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin default-source completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin default-source completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_state_normalization_centralized_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 状态归一化集中",
        "components/date-input-group/src/logic.rs::derive_state",
        "date_input_group_centralizes_state_normalization_in_logic",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin state normalization completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin state normalization completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_discrete_state_type_constraints_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 离散状态必须类型约束",
        "DateInputGroupWidth` / `DateInputGroupStatus",
        "date_input_group_uses_enum_constraints_for_discrete_state_axes",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin discrete-state enum completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin discrete-state enum completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_state_primitive_source_boundary_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 状态原语来源正确",
        "components/date-input-group/src/logic.rs` 仅通过 re-export 消费",
        "date_input_group_consumes_state_primitives_without_business_store_binding",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin state-primitive-source completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin state-primitive-source completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_async_semantics_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 如果无异步相关，直接打勾。",
        "N/A：`DateInputGroup` 仅提供同步容器装配",
        "date_input_group_has_no_async_interaction_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin async-semantics completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin async-semantics completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_dragging_high_frequency_interaction_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "Dragging",
        "dragging",
        "DragEnd",
        "Action::DragEnd",
        "#[prop(optional)] on_drag:",
        "#[prop(optional)] on_drag_end:",
        "draggable=",
        "aria-grabbed",
        "data-dragging",
        "pointermove",
        "mousemove",
        "touchmove",
        "set_pointer_capture",
        "release_pointer_capture",
        "requestAnimationFrame",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose drag macro/micro state machine contracts via `{forbidden}`."
        );
    }

    for needle in [
        "date_input_group_motion::attach_motion(node_ref, motion);",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "DateInputGroup motion should stay as one-shot attach contract via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_macro_micro_duality_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 宏观/微观双状态机（Macro/Micro Duality）",
        "N/A：`DateInputGroup` 不包含拖拽/高频指针跟随交互",
        "date_input_group_has_no_dragging_high_frequency_interaction_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin macro-micro completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin macro-micro completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_dom_measurement_two_pass_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "Intent",
        "Measure(",
        "Rectification",
        "measure_phase",
        "rectify_phase",
        "layout_effect",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose two-pass geometry measurement contracts via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "date_input_group_motion::attach_motion(node_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as direct assembly surface without geometry-measure loop via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_two_pass_rendering_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）",
        "N/A：`DateInputGroup` 无 overlay 定位与 DOM 几何测量流程",
        "date_input_group_has_no_dom_measurement_two_pass_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin two-pass-rendering completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin two-pass-rendering completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_registration_protocol_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "active_index",
        "set_active_index",
        "roving",
        "tablist",
        "menuitem",
        "aria-activedescendant",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose dynamic-item registration protocol contracts via `{forbidden}`."
        );
    }

    for needle in ["children: Children,", "{children()}"] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep plain slot composition without registration protocol via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_registration_protocol_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 集合注册协议（Registration Protocol）",
        "N/A：`DateInputGroup` 不管理动态子项集合导航",
        "date_input_group_has_no_registration_protocol_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin registration-protocol completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin registration-protocol completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_slot_projection_lifecycle_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "on_shown",
        "pause_polling",
        "resume_polling",
        "visibilitychange",
        "suspend_animation",
        "slot_projection",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose slot-projection lifecycle contracts via `{forbidden}`."
        );
    }

    for needle in ["children: Children,", "{children()}"] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep eager direct slot rendering via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_slot_projection_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 插槽投影策略（Slot Projection）",
        "N/A：`DateInputGroup` 不提供 `Lazy/KeepAlive/Eager` 投影模式",
        "date_input_group_has_no_slot_projection_lifecycle_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin slot-projection completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin slot-projection completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_env_subscription_stream_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "matchMedia",
        "BreakpointChanged",
        "Action::",
        "subscribe",
        "stream",
        "debounce",
        "throttle",
        "on_resize",
        "on_theme_change",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose env-subscription stream contracts via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as direct props->state assembly without env stream actions via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_env_streams_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 环境订阅流（Env Streams）",
        "N/A：`DateInputGroup` 不订阅 `Resize/Theme/Intersection` 环境流",
        "date_input_group_has_no_env_subscription_stream_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin env-streams completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin env-streams completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_event_light_cone_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "bulk_select",
        "batch_select",
        "prop_drilling",
        "row_selection",
        "grid_selection",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose event-light-cone bulk-collection contracts via `{forbidden}`."
        );
    }

    for needle in [
        "children: Children,",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as local container assembly without collection bus/selectors via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_event_light_cone_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 事件光锥（Event Light Cone）",
        "N/A：`DateInputGroup` 不涉及大型集合批量操作",
        "date_input_group_has_no_event_light_cone_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin event-light-cone completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin event-light-cone completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_causality_bus_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "publish(",
        "broadcast(",
        "subscriber",
        "subscribe(",
        "dispatch(",
        "event_bus",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose causality-bus tracing contracts via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as local assembly path without bus trace propagation via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_causality_bus_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 统一因果总线（Causality Bus）",
        "N/A：`DateInputGroup` 不涉及复杂派生总线广播链路",
        "date_input_group_has_no_causality_bus_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin causality-bus completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin causality-bus completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_a11y_i18n_contract_uses_headless_and_lang_dir() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));",
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "{children()}",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should expose a11y+i18n contract via `{needle}`."
        );
    }

    for needle in [
        "pub enum A11yDirection {",
        "pub struct LabeledGroupA11yAttrs {",
        "pub fn labeled_group_attrs(",
    ] {
        assert!(
            headless_a11y_source.contains(needle),
            "ui-headless a11y shared contract should provide `{needle}`."
        );
    }

    for forbidden in [
        "fn labeled_group_attrs(",
        "fn locale_attrs(",
        "Invoice date",
        "Ship window",
        "Group related fields",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup view should not redefine a11y helpers or hardcode business-visible copy via `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("normalize_aria_label"),
        "DateInputGroup should keep aria-label fallback normalization in logic."
    );
}

#[test]
fn date_input_group_check2_marks_a11y_i18n_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 存在 A11y 实现、国际化与本地化实现",
        "`DateInputGroup` 通过 `ui_headless::labeled_group_attrs`",
        "date_input_group_a11y_i18n_contract_uses_headless_and_lang_dir",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin a11y+i18n completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin a11y+i18n completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_exposes_stable_observable_state_markers() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/date_input_group.rs");

    for needle in [
        "data-slot=\"date-input-group\"",
        "data-variant=move || state.get().variant_attr",
        "data-width=move || state.get().width_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-segmented=move || state.get().is_segmented.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup observability contract should expose stable marker `{needle}`."
        );
    }

    for needle in [
        "DateInputGroupVariant::Primary => \"primary\"",
        "DateInputGroupVariant::Secondary => \"secondary\"",
        "DateInputGroupWidth::Fit => \"fit\"",
        "DateInputGroupWidth::Full => \"full\"",
        "DateInputGroupStatus::Default => \"default\"",
        "DateInputGroupStatus::Invalid => \"invalid\"",
        "DateInputGroupStatus::Disabled => \"disabled\"",
        "DateInputGroupStatus::DisabledInvalid => \"disabled-invalid\"",
        "if input.segmented && input.status == DateInputGroupStatus::Default {",
        "\"segmented\"",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "DateInputGroup marker values should stay as enumerable closed-set contracts via `{needle}`."
        );
    }

    for needle in [
        "let motion_source_attr = if motion == DateInputGroupMotion::default() {",
        "\"default\"",
        "\"custom\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup motion-source marker should stay enumerable via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_observable_marker_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 状态可观测、可检索、可验证",
        "`DateInputGroup` 已暴露稳定 `data-*` / `aria-*` 标记",
        "date_input_group_exposes_stable_observable_state_markers",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin observable-marker completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin observable-marker completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_styles_depend_on_explicit_semantic_state_markers() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        ".ui-date-input-group--variant-primary",
        ".ui-date-input-group[data-variant=\"primary\"]",
        ".ui-date-input-group--variant-secondary",
        ".ui-date-input-group[data-variant=\"secondary\"]",
        ".ui-date-input-group--full-width",
        ".ui-date-input-group[data-width=\"full\"]",
        ".ui-date-input-group--disabled",
        ".ui-date-input-group[data-disabled=\"true\"]",
        ".ui-date-input-group--invalid",
        ".ui-date-input-group[data-invalid=\"true\"]",
        ".ui-date-input-group--segmented .ui-date-input-group__segment",
        ".ui-date-input-group[data-segmented=\"true\"] .ui-date-input-group__segment",
        ".ui-date-input-group--custom-class",
        ".ui-date-input-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateInputGroup styles should branch from explicit semantic markers via `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "DateInputGroup should not depend on fragile DOM guessing or inline business style logic via `{forbidden}`."
        );
    }

    for needle in [
        "data-variant=move || state.get().variant_attr",
        "data-width=move || state.get().width_attr",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should expose semantic markers used by styles via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_explicit_state_style_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 样式依赖显式状态（`data-*`/class）",
        "`styles.rs` 状态分支已基于稳定 `data-*` / class",
        "date_input_group_styles_depend_on_explicit_semantic_state_markers",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin explicit-state-style completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin explicit-state-style completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_follows_token_first_static_style_contract() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let css_aggregator_source = load_source("../../crates/ui-components/src/css.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateInputGroup styles.rs should remain token-first static css via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "@layer ui",
        "#[cfg(feature = \"component-date_input_group\")]",
        "out.push_str(crate::text_input::date_input_group::styles::CSS);",
    ] {
        assert!(
            css_aggregator_source.contains(needle),
            "ui-components css aggregator should collect date_input_group styles via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated component styles through css.rs via `{needle}`."
        );
    }

    for forbidden in [
        "style=",
        "tailwind",
        "utility-first",
        "styled_components",
        "stylex",
        "emotion::",
        "stylist::",
        "cva(",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "DateInputGroup component layer should not depend on utility-first/css-in-rust defaults via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_token_first_static_style_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "`crates/ui-components/src/css.rs` 通过 `component-date_input_group` feature 聚合 `styles::CSS`",
        "date_input_group_follows_token_first_static_style_contract",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin token-first-style completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin token-first-style completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_visual_desire_baseline_is_documented_and_tokenized() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let styles_source = load_source("src/styles.rs");

    for needle in [
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "description=\"baseline-style date-input grouping primitive",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "DateFieldTone::Quiet",
        "TimeFieldTone::Strong",
        "class_name=\"docs-date-input-group-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "DateInputGroup docs baseline should provide visual hierarchy and state contrast evidence via `{needle}`."
        );
    }

    for needle in [
        "border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color-mix(",
        ".ui-date-input-group--invalid,",
        ".ui-date-input-group[data-invalid=\"true\"] {",
        "opacity: 0.62;",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateInputGroup styles should keep modern default-theme contrast/feedback tokens via `{needle}`."
        );
    }

    for forbidden in [".form-control", ".input-group", ".btn", "bootstrap", "#"] {
        assert!(
            !styles_source.contains(forbidden),
            "DateInputGroup visual contract should avoid bootstrap-like degradation or hardcoded palette via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_visual_desire_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "N/A：Button/Input/Overlay 跨组件截图基线属于仓库级视觉回归任务",
        "N/A：hover/active/focus 细粒度反馈由子组件 `DateField`/`TimeField` 负责",
        "date_input_group_visual_desire_baseline_is_documented_and_tokenized",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin visual-desire completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin visual-desire completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_tree_shaking_is_feature_gated_in_ui_components() {
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");

    for needle in [
        "[features]",
        "default = [\"inject-css\", \"all-components\"]",
        "component-date_input_group = [\"dep:ui-date-input-group\"]",
        "all-components = [",
        "\"component-date_input_group\",",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components feature table should expose tree-shaking contract for date_input_group via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(any(",
        "feature = \"component-date_input_group\",",
        "pub mod text_input;",
        "#[cfg(feature = \"all-components\")]",
        "mod all_components {",
        "pub use text_input::date_input_group::{DateInputGroup, DateInputGroupVariant};",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib.rs should keep date_input_group behind explicit feature-gated exports via `{needle}`."
        );
    }

    let date_input_group_export_count = ui_components_lib
        .matches("pub use text_input::date_input_group::{DateInputGroup, DateInputGroupVariant};")
        .count();
    assert_eq!(
        date_input_group_export_count, 1,
        "date_input_group re-export should stay scoped to gated aggregate surfaces (all-components), found {date_input_group_export_count}."
    );

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-date_input_group\")]",
        "out.push_str(crate::text_input::date_input_group::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui-components css aggregation should tree-shake date_input_group css via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_script_source = include_str!("../../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "DATE_INPUT_GROUP_MIN_FEATURES=\"component-date_input_group,inject-css\"",
        "cargo test -p ui-date-input-group date_input_group_tree_shaking_is_feature_gated_in_ui_components",
        "cargo test -p ui-date-input-group date_input_group_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-date-input-group date_input_group_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$DATE_INPUT_GROUP_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$DATE_INPUT_GROUP_TREE_OUTPUT\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$DATE_INPUT_GROUP_MIN_FEATURES\"",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking script should enforce date-input-group contract marker `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_check2_marks_tree_shaking_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "`cargo tree -e features -p ui-components --no-default-features --features component-date_input_group,inject-css -f '{p} {f}'`",
        "`ui-components v0.0.0 ... component-date_input_group,inject-css`",
        "反向依赖检查：`cargo tree -e features -i ui-components -p web-demo -f '{p} {f}'`",
        "`web-demo` 路径观测到 `web-demo-components + inject-css`，未出现 `all-components` 拉起",
        "`scripts/check-ui-components-tree-shaking.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_tree_shaking_is_feature_gated_in_ui_components`、`cargo test -p ui-date-input-group date_input_group_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、`cargo test -p ui-date-input-group date_input_group_check2_marks_tree_shaking_feature_pruning_contract_complete`。",
        "N/A：体积预算阈值（如 `< 50KB`）属于仓库级 CI 策略",
        "date_input_group_tree_shaking_is_feature_gated_in_ui_components",
        "date_input_group_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "date_input_group_check2_marks_tree_shaking_feature_pruning_contract_complete",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin tree-shaking completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin tree-shaking completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    date_input_group_check2_marks_tree_shaking_complete();
}

#[test]
fn date_input_group_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/date_input_group.rs");
    let primitive_tests =
        load_source("../../crates/ui-state-primitives/src/test/date_input_group.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub enum DateInputGroupVariant {",
        "pub enum DateInputGroupWidth {",
        "pub enum DateInputGroupStatus {",
        "pub struct DateInputGroupStateInput {",
        "pub struct DateInputGroupState {",
        "pub variant: DateInputGroupVariant,",
        "pub width: DateInputGroupWidth,",
        "pub status: DateInputGroupStatus,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "DateInputGroup primitive typing contract should keep `{needle}`."
        );
    }

    for needle in [
        "pub struct DateInputGroupStateDeriveInput {",
        "pub variant: DateInputGroupVariant,",
        "pub width: DateInputGroupWidth,",
        "pub status: DateInputGroupStatus,",
        "status: logic::resolve_status(is_disabled, is_invalid),",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "DateInputGroup assembly path should keep typed normalization edge `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] variant: DateInputGroupVariant,",
        "data-variant=move || state.get().variant_attr",
        "data-width=move || state.get().width_attr",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup machine-readable semantic marker contract should keep `{needle}`."
        );
    }

    for needle in [
        "if is_disabled && is_invalid {",
        "DateInputGroupStatus::DisabledInvalid",
        "if input.segmented && input.status == DateInputGroupStatus::Default {",
        "DateInputGroupStatus::DisabledInvalid => \"disabled-invalid\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "DateInputGroup invalid-state normalization contract should keep `{needle}`."
        );
    }

    for needle in [
        "fn width_and_status_contract_are_stable()",
        "fn resolve_state_tracks_markers()",
        "fn resolve_state_prefers_disabled_invalid_state()",
    ] {
        assert!(
            primitive_tests.contains(needle),
            "DateInputGroup primitive regression suite should keep `{needle}` for direct contract breakage localization."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] variant: Option<String>,",
        "#[prop(optional, into)] status: Option<String>,",
        "data-state=move || format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup should not regress to string/format-based state protocol via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_type_system_semantic_marker_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "`crates/ui-state-primitives/src/date_input_group.rs` 使用 `DateInputGroupVariant` / `DateInputGroupWidth` / `DateInputGroupStatus`",
        "`resolve_status` + `resolve_state` 在原语层统一归一化无效组合",
        "`components/date-input-group/src/view.rs` 暴露 `data-variant` / `data-width` / `data-state` / `data-aria-source` / `data-class-source` / `data-motion-source`",
        "width_and_status_contract_are_stable",
        "resolve_state_tracks_markers",
        "date_input_group_type_system_and_semantic_markers_form_machine_readable_contract",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin type-system+semantic-marker completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin type-system+semantic-marker completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_overlay_focus_stack_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "FocusManager",
        "focus manager",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "focus_stack",
        "overlay_stack",
        "document.body",
        "focus_trap",
        "last_focused",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not implement overlay focus-stack recovery contracts via `{forbidden}`."
        );
    }

    for needle in [
        "let node_ref: NodeRef<html::Div> = NodeRef::new();",
        "date_input_group_motion::attach_motion(node_ref, motion);",
        "node_ref=node_ref",
        "children: Children,",
        "{children()}",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep NodeRef usage limited to motion attach/container assembly via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct Overlay",
        "pub enum Overlay",
        "popover",
        "dialog",
        "menu",
        "tooltip",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "DateInputGroup should not own overlay layering semantics via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_focus_stack_gc_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A：`DateInputGroup` 非层叠 `Overlay` 组件，无焦点恢复栈职责；组件 `NodeRef` 仅用于 `date_input_group_motion::attach_motion` 节点挂载。",
        "date_input_group_has_no_overlay_focus_stack_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin focus-stack completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin focus-stack completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_escape_hatch_foreign_zone_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "Mapbox",
        "Leaflet",
        "google.maps",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "third_party_instance",
        "thirdPartyInstance",
        "JsValue",
        "wasm_bindgen::closure::Closure",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not expose Foreign-Zone third-party imperative integration contracts via `{forbidden}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] chart_instance:",
        "#[prop(optional)] map_instance:",
        "#[prop(optional)] foreign_handle:",
        "pub struct DateInputGroupForeign",
        "pub enum DateInputGroupForeign",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "DateInputGroup public API should not leak third-party imperative instances via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "children: Children,",
        "{children()}",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as pure state+slot assembly without foreign imperative zone via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_escape_hatches_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A：`DateInputGroup` 不集成 ECharts/Map 等命令式第三方实例，不存在 `Foreign Zone`（`YieldControl/CleanupForeign`）边界；组件公共 API 未暴露第三方实例句柄。",
        "date_input_group_has_no_escape_hatch_foreign_zone_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin escape-hatches completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin escape-hatches completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_hydration_discontinuity_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now(",
        "Math::random",
        "js_sys::Math::random",
        "rand::",
        "Uuid::new_v4",
        "uuid::Uuid::new_v4",
        "random_uuid",
        "use_ui_id_provider",
        "provide_ui_id_provider",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should not depend on time/random/id-provider runtime ID generation via `{forbidden}`."
        );
    }

    for needle in [
        "pub fn DateInputGroup(",
        "children: Children,",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
        "let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as deterministic props->state assembly without hydration-time ID seeds via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_hydration_discontinuity_na_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A：`DateInputGroup` 不生成运行时随机/时间相关 ID（无 `now()` / 随机 UUID），也不消费 `IdProvider`；当前仅承载容器装配与子节点透传，不存在 SSR/Hydration ID 漂移面。",
        "date_input_group_has_no_hydration_discontinuity_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin hydration-discontinuity completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin hydration-discontinuity completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_platform_paths_are_cfg_guarded_and_non_wasm_safe() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    let wasm_cfg = "#[cfg(target_arch = \"wasm32\")]";
    let non_wasm_cfg = "#[cfg(not(target_arch = \"wasm32\"))]";

    let wasm_start = motion_source.find(wasm_cfg).unwrap_or_else(|| {
        panic!("DateInputGroup motion should declare wasm cfg branch `{wasm_cfg}`.")
    });
    let non_wasm_start = motion_source.find(non_wasm_cfg).unwrap_or_else(|| {
        panic!("DateInputGroup motion should declare non-wasm cfg branch `{non_wasm_cfg}`.")
    });

    assert!(
        wasm_start < non_wasm_start,
        "DateInputGroup motion cfg branches should keep wasm branch before non-wasm branch."
    );

    let wasm_branch = &motion_source[wasm_start..non_wasm_start];
    let non_wasm_branch = &motion_source[non_wasm_start..];

    for needle in [
        "use leptos::wasm_bindgen::JsCast;",
        "let element: leptos::web_sys::HtmlElement = node.unchecked_into();",
        "let style = element.style();",
    ] {
        assert!(
            wasm_branch.contains(needle),
            "DateInputGroup wasm branch should contain browser-only code `{needle}`."
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "HtmlElement"] {
        assert!(
            !non_wasm_branch.contains(forbidden),
            "DateInputGroup non-wasm branch should not depend on browser APIs via `{forbidden}`."
        );
    }

    assert!(
        non_wasm_branch.contains("std::hint::black_box(sanitize_motion(motion));"),
        "DateInputGroup non-wasm branch should keep deterministic no-op/stub downgrade."
    );

    for forbidden in ["web_sys", "wasm_bindgen", "js_sys::", "window()"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "DateInputGroup logic/view should stay platform-agnostic and avoid browser APIs via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_ssr_cross_platform_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "`cargo check -p ui-date-input-group --target wasm32-unknown-unknown`",
        "`cargo check -p ui-date-input-group --target x86_64-unknown-linux-gnu`",
        "`cargo check -p ui-date-input-group`",
        "命令在当前环境均受 `Invalid cross-device link (os error 18)` 阻塞",
        "`components/date-input-group/src/motion.rs` 通过 `#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]` 显式分支平台行为",
        "date_input_group_platform_paths_are_cfg_guarded_and_non_wasm_safe",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin SSR cross-platform completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin SSR cross-platform completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_respects_ui_headless_web_ssr_mutex_contract() {
    let component_cargo = load_source("Cargo.toml");
    let view_source = load_source("src/view.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep consuming ui-headless a11y contracts via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should keep web/ssr mutual-exclusion guard via `{needle}`."
        );
    }

    for needle in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = []",
    ] {
        assert!(
            component_cargo.contains(needle),
            "DateInputGroup dependency contract should keep `{needle}`."
        );
    }

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"]",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !component_cargo.contains(forbidden),
            "DateInputGroup should not force conflicting ui-headless features via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_ui_headless_web_ssr_mutex_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "`crates/ui-headless/src/lib.rs` 存在 `#[cfg(all(feature = \"web\", feature = \"ssr\"))]` + `compile_error!` 互斥保护",
        "`cargo check -p ui-headless --no-default-features --features web`",
        "`cargo check -p ui-headless --no-default-features --features ssr`",
        "`cargo check -p ui-headless --no-default-features --features web,ssr`",
        "命令在当前环境均受 `Invalid cross-device link (os error 18)` 阻塞",
        "date_input_group_respects_ui_headless_web_ssr_mutex_contract",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin ui-headless web/ssr mutex completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin ui-headless web/ssr mutex completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_relies_on_ui_motion_non_wasm_stub_contract() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let component_motion = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");

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
            ui_motion_lib.contains(needle),
            "ui-motion non-wasm no-op/stub contract should keep `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            component_motion.contains(needle),
            "DateInputGroup motion should keep non-wasm safe downgrade via `{needle}`."
        );
    }

    assert!(
        view_source.contains("date_input_group_motion::attach_motion(node_ref, motion);"),
        "DateInputGroup view should always attach motion through component motion contract."
    );
}

#[test]
fn date_input_group_check2_marks_ui_motion_non_wasm_stub_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::animate` no-op 与 `prefers_reduced_motion` 可预测返回",
        "`components/date-input-group/src/motion.rs` 的 non-wasm `attach_motion` 仅执行 `sanitize_motion` + `black_box` 安全降级",
        "`cargo check -p ui-motion`",
        "命令结果包含 `Finished` 与 `dev` profile",
        "date_input_group_relies_on_ui_motion_non_wasm_stub_contract",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin ui-motion non-wasm stub completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin ui-motion non-wasm stub completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_motion_covers_reduced_motion_ssr_and_wasm_paths() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");

    let wasm_cfg = "#[cfg(target_arch = \"wasm32\")]";
    let non_wasm_cfg = "#[cfg(not(target_arch = \"wasm32\"))]";
    let wasm_start = motion_source.find(wasm_cfg).unwrap_or_else(|| {
        panic!("DateInputGroup motion should declare wasm cfg branch `{wasm_cfg}`.")
    });
    let non_wasm_start = motion_source.find(non_wasm_cfg).unwrap_or_else(|| {
        panic!("DateInputGroup motion should declare non-wasm cfg branch `{non_wasm_cfg}`.")
    });
    let wasm_branch = &motion_source[wasm_start..non_wasm_start];
    let non_wasm_branch = &motion_source[non_wasm_start..];

    for needle in [
        "if ui_motion::web::prefers_reduced_motion() {",
        "drop(style.set_property(\"--ui-date-input-group-scale\", \"1\"));",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            wasm_branch.contains(needle),
            "DateInputGroup wasm motion branch should keep `{needle}`."
        );
    }

    let reduced_motion_start = wasm_branch
        .find("if ui_motion::web::prefers_reduced_motion() {")
        .expect("DateInputGroup wasm motion should have reduced-motion gate.");
    let animator_start = wasm_branch
        .find("ui_motion::spring::SpringAnimator::new(")
        .expect("DateInputGroup wasm motion should create SpringAnimator in enhanced path.");
    let reduced_block = &wasm_branch[reduced_motion_start..animator_start];
    assert!(
        reduced_block.contains("return;"),
        "DateInputGroup reduced-motion branch should early-return before spring animator setup."
    );

    assert!(
        non_wasm_branch.contains("std::hint::black_box(sanitize_motion(motion));"),
        "DateInputGroup non-wasm branch should keep deterministic no-op/stub downgrade."
    );

    for needle in [
        "data-motion-source=motion_source_attr",
        "data-custom-motion=custom_motion_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should expose stable motion semantics via `{needle}` across SSR/wasm."
        );
    }
}

#[test]
fn date_input_group_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "pub struct DateInputGroupMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "ui_motion::presets::spring_soft()",
        "ui_motion::spring::sanitize_config(value, default)",
        "pub fn sanitize_motion(motion: DateInputGroupMotion) -> DateInputGroupMotion {",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "motion.spring,",
        "if ui_motion::web::prefers_reduced_motion() {",
        "drop(style.set_property(\"--ui-date-input-group-scale\", \"1\"));",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "DateInputGroup motion contractualization should keep `{needle}`."
        );
    }

    let reduced_start = motion_source
        .find("if ui_motion::web::prefers_reduced_motion() {")
        .expect("DateInputGroup motion contract should contain reduced-motion branch.");
    let animator_start = motion_source
        .find("ui_motion::spring::SpringAnimator::new(")
        .expect("DateInputGroup motion contract should contain spring animator setup.");
    let reduced_block = &motion_source[reduced_start..animator_start];
    assert!(
        reduced_block.contains("return;"),
        "DateInputGroup reduced-motion branch should early-return before spring animator setup."
    );

    assert!(
        view_source.contains("date_input_group_motion::attach_motion(node_ref, motion);"),
        "DateInputGroup view should mount motion contract via attach_motion."
    );

    let platform_script_needle = "cargo test -p ui-date-input-group date_input_group_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platforms_script_source.contains(platform_script_needle),
        "platform gate script should include `{platform_script_needle}`."
    );

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "components/date-input-group/test/semantics.rs::date_input_group_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should pin motion-contract completion marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should pin motion-contract completion marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");
    let ui_components_root = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight = load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let entrypoints_script = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let ui_components_src_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    let ui_headless_src_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-headless/src");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(any(",
        "feature = \"component-date_input_group\",",
        "pub mod text_input;",
        "pub use text_input::date_input_group::{DateInputGroup, DateInputGroupVariant};",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components public API should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-date_input_group\")]",
        "out.push_str(crate::text_input::date_input_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep marker `{required}`."
        );
    }

    for forbidden in ["DateInputGroup", "date-input-group", "date_input_group"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight shared primitive should avoid date-input-group business token `{forbidden}`."
        );
    }

    for forbidden_path in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_root.join(forbidden_path).exists(),
            "ui-components src should not host duplicated headless primitive `{forbidden_path}`."
        );
    }

    for required_path in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        assert!(
            ui_headless_src_root.join(required_path).exists(),
            "ui-headless should host shared primitive `{required_path}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints script should include `{script_needle}`."
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "scripts/check-ui-components-entrypoints.sh",
        "components/date-input-group/test/semantics.rs::date_input_group_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep fixed-entrypoint evidence `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep fixed-entrypoint evidence `{required}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_reduced_motion_ssr_wasm_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "`components/date-input-group/src/motion.rs` 在 wasm 分支通过 `ui_motion::web::prefers_reduced_motion()` 命中时将 `--ui-date-input-group-scale` 设为 `1` 并提前返回，跳过弹簧动画创建",
        "`components/date-input-group/src/motion.rs` 继续通过 `#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]` 维持 wasm 增强与 SSR/non-wasm 安全降级",
        "`components/date-input-group/src/view.rs` 统一暴露 `data-motion-source` / `data-custom-motion`，确保 SSR 与 wasm 语义标记契约一致",
        "date_input_group_motion_covers_reduced_motion_ssr_and_wasm_paths",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin reduced-motion/SSR/wasm completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin reduced-motion/SSR/wasm completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_performance_governance_is_mount_only_traceable_and_blocking_via_shared_gates() {
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let docs_perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let docs_forms_groups_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let accordion_semantics_source =
        load_source("../../crates/ui-components/tests/accordion_semantics.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_forms_groups_source.contains(needle),
            "DateInputGroup docs page should stay wired to shared perf governance shell via `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell should keep repeatable performance budget baseline plumbing via `{needle}`."
        );
    }

    for needle in [
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "\"mount-plus-budget\"",
        "\"mount-only\"",
    ] {
        assert!(
            docs_perf_probe_source.contains(needle),
            "UiPerfProbe should expose traceable budget/violation markers via `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"ui-perf-probe\"",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs e2e coverage should keep blocking performance regression assertion `{needle}`."
        );
    }

    for needle in [
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "shared perf gate script should keep blocking check `{needle}`."
        );
    }

    for needle in [
        "fn perf_render_count_follow_up_is_tracked_in_plan()",
        "render_count",
    ] {
        assert!(
            accordion_semantics_source.contains(needle),
            "shared perf follow-up regression test should keep `{needle}`."
        );
    }

    assert!(
        todo_source.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据"
        ),
        "performance governance should keep explicit render_count follow-up item in docs plan."
    );
}

#[test]
fn date_input_group_check2_marks_performance_governance_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "`apps/docs-app/src/pages/components/shell.rs` 的 `component_page_perf_budget` 为未显式登记组件提供 `_ => UiPerfBudget::mount_only(120.0)` 基线",
        "`apps/docs-app/src/perf_probe.rs` 输出 `data-perf-budget-ms` / `data-perf-budget-update-ms` / `data-perf-budget-heap-kb` / `data-perf-violation` 机器可读标记",
        "`e2e/tests/docs_app_components_coverage.spec.mjs` 断言 perf probe 存在预算且 `data-perf-violation != true`",
        "`scripts/check-ui-components-performance.sh` 已纳入 `docs_perf_probe_budgets_are_wired_for_component_pages` 与 `perf_render_count_follow_up_is_tracked_in_plan`",
        "N/A（精确 `render_count` 自动计数）：当前仓库仍在 `docs/plan/TODO.md` 跟踪“建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据”",
        "date_input_group_performance_governance_is_mount_only_traceable_and_blocking_via_shared_gates",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin performance-governance completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin performance-governance completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for marker in [
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group semantics/perf matrix should keep aria/data marker `{marker}`.",
        );
    }

    for marker in [
        "fn date_input_group_supports_group_accessibility_and_children_layout()",
        "fn date_input_group_exposes_stable_observable_state_markers()",
        "fn date_input_group_semantic_contract_tests_cover_state_matrix_without_snapshot_dependency()",
        "fn date_input_group_has_no_keyboard_pointer_interaction_axis()",
        "fn date_input_group_performance_governance_is_mount_only_traceable_and_blocking_via_shared_gates()",
        "render_count",
    ] {
        assert!(
            semantics_source.contains(marker),
            "date-input-group semantics/perf matrix should keep coverage marker `{marker}`.",
        );
    }

    let perf_script_needle = "cargo test -p ui-date-input-group date_input_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement";
    assert!(
        perf_script_source.contains(perf_script_needle),
        "performance gate script should include `{perf_script_needle}`.",
    );

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "N/A：`DateInputGroup` 焦点流转由子组件 `DateField`/`TimeField` 承载",
        "N/A（精确 `render_count` 自动计数）：当前仓库仍在 `docs/plan/TODO.md` 跟踪“建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据”",
        "date_input_group_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin semantics+performance completion evidence `{needle}`.",
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin semantics+performance completion evidence `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_view_macro_complexity_is_structured_and_shallow() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "fn render_prefix_slot(prefix: StoredValue<ViewFn>) -> impl IntoView {",
        "fn render_suffix_slot(suffix: StoredValue<ViewFn>) -> impl IntoView {",
        "{prefix.map(render_prefix_slot)}",
        "{suffix.map(render_suffix_slot)}",
        "data-slot=\"date-input-group-input\"",
        "data-slot=\"date-input-group-segment\"",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view complexity contract should keep structured slot rendering via `{needle}`."
        );
    }

    for forbidden in [
        "{prefix.map(|prefix| {",
        "{suffix.map(|suffix| {",
        "view! {\n                    <div class=\"ui-date-input-group__prefix\"",
        "view! {\n                    <div class=\"ui-date-input-group__suffix\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup main view should avoid inline nested macro duplication via `{forbidden}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 3,
        "DateInputGroup view macro count should stay controlled (<=3); found {view_macro_count}."
    );
}

#[test]
fn date_input_group_check2_marks_view_macro_complexity_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "`components/date-input-group/src/view.rs` 将 prefix/suffix 子片段拆分为 `render_prefix_slot` / `render_suffix_slot` 局部渲染函数，避免主 `view!` 内联重复嵌套",
        "`DateInputGroup` 主体 `view!` 仅保留容器骨架与 slot 装配（`prefix.map(render_prefix_slot)` / `suffix.map(render_suffix_slot)`）",
        "`view.rs` 中 `view! {` 总数受控（当前 3 处：主视图 + 两个局部渲染函数）",
        "date_input_group_view_macro_complexity_is_structured_and_shallow",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin view-macro-complexity completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin view-macro-complexity completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_prefers_functional_fragment_extraction_over_extra_components() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "fn render_prefix_slot(prefix: StoredValue<ViewFn>) -> impl IntoView {",
        "fn render_suffix_slot(suffix: StoredValue<ViewFn>) -> impl IntoView {",
        "{prefix.map(render_prefix_slot)}",
        "{suffix.map(render_suffix_slot)}",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep functional fragment extraction via `{needle}`."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert!(
        component_count == 1,
        "DateInputGroup view should keep exactly one component boundary; found {component_count}."
    );

    for forbidden in [
        "#[component]\nfn render_prefix_slot(",
        "#[component]\nfn render_suffix_slot(",
        "#[component]\r\nfn render_prefix_slot(",
        "#[component]\r\nfn render_suffix_slot(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup should not promote lightweight fragments to extra components via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_functional_split_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "`components/date-input-group/src/view.rs` 将轻量 prefix/suffix 片段下沉为普通函数 `render_prefix_slot` / `render_suffix_slot`（返回 `impl IntoView`）",
        "`DateInputGroup` 保持单一 `#[component]` 边界，未把局部片段升级为额外组件",
        "date_input_group_prefers_functional_fragment_extraction_over_extra_components",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin functional-split completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin functional-split completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_heavy_static_fragment_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");

    for forbidden in [
        "<svg",
        "</svg>",
        "inner_html=",
        "dangerously_set_inner_html",
        "<footer",
        "long-description",
        "lorem ipsum",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "DateInputGroup should not own heavyweight static fragment payload via `{forbidden}`."
        );
    }

    for needle in [
        "children: Children,",
        "{children()}",
        "#[prop(optional, into)] prefix: Option<ViewFn>,",
        "#[prop(optional, into)] suffix: Option<ViewFn>,",
        "fn render_prefix_slot(prefix: StoredValue<ViewFn>) -> impl IntoView {",
        "fn render_suffix_slot(suffix: StoredValue<ViewFn>) -> impl IntoView {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep static payload ownership at caller-side slots via `{needle}`."
        );
    }

    for needle in [
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "lang=move || group_a11y.get_value().lang",
        "dir=move || group_a11y.get_value().dir",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should preserve accessibility semantics while keeping static payload external via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_static_fragment_constantization_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A：`DateInputGroup` 组件自身不内置复杂 SVG/页脚/长说明文本等重型静态片段",
        "`components/date-input-group/src/view.rs` 仅保留容器骨架与 `children/prefix/suffix` 插槽装配",
        "date_input_group_has_no_heavy_static_fragment_axis",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin static-fragment-constantization completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin static-fragment-constantization completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_forbids_inner_html_in_component_and_docs() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        ".set_inner_html(",
        "insert_adjacent_html(",
        "Html::from_html_unchecked",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "DateInputGroup should forbid raw html injection surface `{forbidden}` in component and docs paths."
        );
    }

    for needle in [
        "children: Children,",
        "{children()}",
        "#[prop(optional, into)] prefix: Option<ViewFn>,",
        "#[prop(optional, into)] suffix: Option<ViewFn>,",
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should keep safe slot + semantics mounting path via `{needle}` instead of raw html injection."
        );
    }
}

#[test]
fn date_input_group_check2_marks_inner_html_constraint_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`DateInputGroup` 组件与 docs 示例均未使用 `inner_html`/`dangerously_set_inner_html`，不存在 HTML 注入入口",
        "`components/date-input-group/src/view.rs` 仅通过 `children/prefix/suffix` 插槽与 headless 语义挂载渲染内容，不接收原始 HTML 字符串",
        "date_input_group_forbids_inner_html_in_component_and_docs",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin inner_html-constraint completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin inner_html-constraint completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_has_no_keyboard_pointer_interaction_axis() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:pointerdown",
        "on:pointermove",
        "on:pointerup",
        "on:mousedown",
        "on:mouseup",
        "on:touchstart",
        "on:touchmove",
        "on:touchend",
        "on_key_down",
        "on_key_up",
        "on_pointer_down",
        "on_pointer_move",
        "on_pointer_up",
        "aria-keyshortcuts",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "DateInputGroup should not own keyboard/pointer interaction protocol via `{forbidden}`."
        );
    }

    for needle in [
        "children: Children,",
        "{children()}",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup should stay as semantic container assembly via `{needle}`."
        );
    }
}

#[test]
fn date_input_group_semantic_contract_tests_cover_state_matrix_without_snapshot_dependency() {
    let semantics_source = load_source("test/semantics.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "fn date_input_group_supports_group_accessibility_and_children_layout()",
        "fn date_input_group_exposes_stable_observable_state_markers()",
        "fn date_input_group_has_no_local_controlled_uncontrolled_value_axis()",
        "fn date_input_group_has_no_keyboard_pointer_interaction_axis()",
        "fn date_input_group_motion_contract_delegates_to_ui_motion_and_has_non_wasm_stub()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "DateInputGroup semantics suite should cover semantic contract matrix via `{needle}`."
        );
    }

    for needle in [
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view should expose semantic-contract markers via `{needle}`."
        );
    }

    for forbidden in [
        "assert_snapshot!",
        "insta::assert_snapshot",
        "insta::assert_debug_snapshot",
        "to_match_snapshot",
        ".matches_snapshot(",
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "DateInputGroup semantics coverage should not depend on snapshot-only assertions via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_semantic_contract_testing_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "N/A：键盘/指针路径由子组件（如 `DateField`/`TimeField`）承载",
        "date_input_group_supports_group_accessibility_and_children_layout",
        "date_input_group_exposes_stable_observable_state_markers",
        "date_input_group_has_no_local_controlled_uncontrolled_value_axis",
        "date_input_group_has_no_keyboard_pointer_interaction_axis",
        "date_input_group_motion_contract_delegates_to_ui_motion_and_has_non_wasm_stub",
        "date_input_group_semantic_contract_tests_cover_state_matrix_without_snapshot_dependency",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin semantic-contract-testing completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin semantic-contract-testing completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for required in [
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "date-input-group semantic-priority contract should keep aria/data/source marker `{required}`."
        );
    }

    for required in [
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
        "fn date_input_group_supports_group_accessibility_and_children_layout()",
        "fn date_input_group_exposes_stable_observable_state_markers()",
        "fn date_input_group_has_no_keyboard_pointer_interaction_axis()",
        "fn date_input_group_semantic_contract_tests_cover_state_matrix_without_snapshot_dependency()",
    ] {
        assert!(
            mod_source.contains(required) || semantics_source.contains(required),
            "date-input-group should keep local semantics contract coverage marker `{required}`."
        );
    }

    for forbidden in [
        "assert_snapshot!",
        "assert_debug_snapshot!",
        "insta::assert",
        "pixelmatch",
        "to_match_snapshot",
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "date-input-group semantic-priority contract should avoid snapshot-only assertion marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/date-input-group/test/semantics.rs",
        "date_input_group_supports_group_accessibility_and_children_layout",
        "date_input_group_exposes_stable_observable_state_markers",
        "date_input_group_has_no_keyboard_pointer_interaction_axis",
        "date_input_group_semantic_contract_tests_cover_state_matrix_without_snapshot_dependency",
        "date_input_group_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-components-performance.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep semantic-test-priority evidence marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep semantic-test-priority evidence marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_check2_documents_e2e_selector_and_stable_wait_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "e2e/tests/docs_app_date_input_group_contract.spec.mjs",
        "scripts/check-ui-components-e2e-date-input-group.sh",
        "date_input_group_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "date_input_group_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep e2e selector/stable-wait evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep e2e selector/stable-wait evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_date_input_group_contract.spec.mjs");

    for needle in [
        "/#/components/date-input-group",
        "body:not(:has(#boot))",
        "[data-component=\"date-input-group\"]",
        "[data-slot=\"date-input-group-state-matrix\"]",
        "[data-slot=\"date-input-group\"][data-state=\"default\"][data-variant=\"primary\"][data-width=\"fit\"][data-ui-source=\"state-primitives\"]",
        "[data-slot=\"date-input-group\"][data-state=\"segmented\"][data-segmented=\"true\"][data-has-prefix=\"true\"][data-has-suffix=\"true\"][data-aria-source=\"custom\"]",
        "[data-slot=\"date-input-group\"][data-state=\"disabled-invalid\"][data-variant=\"secondary\"][data-width=\"full\"][data-disabled=\"true\"][data-invalid=\"true\"][data-custom-class=\"true\"][data-class-source=\"custom\"]",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-state\", \"segmented\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "date-input-group e2e selector/stable-wait contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        ":nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "date-input-group e2e contract should avoid flaky/non-semantic selector or wait token `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_e2e_flow_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_date_input_group_contract.spec.mjs");

    for needle in [
        "docs-app date-input-group motion/stream path uses semantic ready/settled breakpoints",
        "[data-slot=\"date-input-group-streaming-contract\"]",
        "[data-slot=\"date-input-group\"][data-ui-stream-support=\"unsupported\"][data-ui-stream-fallback=\"snapshot\"][data-ui-stream-mode=\"snapshot\"][data-ui-output-status=\"verified\"]",
        "toHaveAttribute(\"data-ui-action\", \"snapshot-render\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
        "await page.reload();",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "date-input-group e2e ready/settled contract should include `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "date-input-group ready/settled e2e contract should avoid unstable fixed-delay wait `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_e2e_repeatable_key_flow_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "date_input_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep repeatable-key-flow evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep repeatable-key-flow evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_date_input_group_contract.spec.mjs");

    for needle in [
        "docs-app date-input-group key flow is repeatable with semantic failure breakpoints",
        "[data-slot=\"date-input-group-streaming-contract\"]",
        "[data-slot=\"date-input-group-requested-stream-mode\"]",
        "[data-slot=\"date-input-group-requested-output-status\"]",
        "[data-slot=\"date-input-group-streaming-requested-state\"]",
        "modeSelect.selectOption(\"snapshot\")",
        "outputSelect.selectOption(\"verified\")",
        "modeSelect.focus();",
        "page.keyboard.press(\"ArrowUp\")",
        "toContainText(\"requested mode: streaming\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-action\", \"snapshot-render\")",
        "await page.reload();",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "date-input-group repeatable key-flow e2e contract should include `{needle}`.",
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
            "date-input-group repeatable key-flow contract should avoid flaky/non-semantic token `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_e2e_check_script_covers_selector_and_settled_wait_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-date-input-group.sh");

    for needle in [
        "cargo test -p ui-date-input-group date_input_group_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-date-input-group date_input_group_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-date-input-group date_input_group_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "cargo test -p ui-date-input-group date_input_group_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-date-input-group date_input_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-date-input-group date_input_group_e2e_check_script_covers_selector_and_settled_wait_contracts",
    ] {
        assert!(
            script_source.contains(needle),
            "date-input-group e2e check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_component_files_follow_role_boundaries() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::DateInputGroup;",
        "#[cfg(test)]",
        "mod semantics_tests;",
    ] {
        assert!(
            mod_source.contains(needle),
            "DateInputGroup mod.rs should keep export-boundary responsibility via `{needle}`."
        );
    }

    for forbidden in [
        "view!",
        "pub fn derive_state(",
        "pub const CSS: &str = r#\"",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "DateInputGroup mod.rs should not carry implementation details via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct DateInputGroupStateDeriveInput {",
        "pub fn derive_state(input: DateInputGroupStateDeriveInput) -> DateInputGroupState {",
        "pub fn resolve_motion_source_attrs(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateInputGroup logic.rs should own normalization/derivation/source markers via `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "<div",
        "NodeRef<html::Div>",
        "ui_motion::spring::SpringAnimator::new(",
        "style.set_property(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateInputGroup logic.rs should not leak DOM/view/motion runtime details via `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "var(--ui-fallback-",
        ".ui-date-input-group[data-variant=\"secondary\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateInputGroup styles.rs should stay token-first static css via `{needle}`."
        );
    }

    for forbidden in [
        "fn ",
        "#[component]",
        "view! {",
        "NodeRef<html::Div>",
        "DateInputGroupStateDeriveInput",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "DateInputGroup styles.rs should not absorb logic/view details via `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn DateInputGroup(",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateInputGroup view.rs should focus on structure rendering + headless mounting via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct DateInputGroupStateDeriveInput {",
        "pub fn derive_state(input: DateInputGroupStateDeriveInput)",
        "pub const CSS: &str = r#\"",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup view.rs should not hide core state or motion-engine decisions via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct DateInputGroupMotion {",
        "pub fn sanitize_motion(motion: DateInputGroupMotion) -> DateInputGroupMotion {",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(needle),
            "DateInputGroup motion.rs should keep motion contract + attach responsibility via `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=\"date-input-group\"",
        "role=move || group_a11y.get_value().role",
        "pub struct DateInputGroupStateDeriveInput {",
        "pub const CSS: &str = r#\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "DateInputGroup motion.rs should not absorb view/logic/styles responsibilities via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_component_file_responsibilities_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "`components/date-input-group/src/mod.rs` 仅保留最小导出边界",
        "date_input_group_component_files_follow_role_boundaries",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin component-file-responsibility completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin component-file-responsibility completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_component_directory_standard_files_follow_contract_and_na_paths() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let component_files_script =
        load_source("../../scripts/check-ui-components-component-files.sh");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        let path = manifest_dir.join(required);
        assert!(
            path.exists(),
            "date-input-group should keep required standard component file `{path:?}`."
        );
    }

    for forbidden in ["src/render.rs", "src/protocol.rs"] {
        let path = manifest_dir.join(forbidden);
        assert!(
            !path.exists(),
            "date-input-group should not introduce forbidden drift file `{path:?}` for this simple component scope."
        );
    }

    let spec_path = manifest_dir.join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "date-input-group should keep `spec.rs` optional and absent for simple component scope: `{spec_path:?}`."
    );

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::DateInputGroup;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep minimal stable export boundary marker `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "pub fn derive_state(",
        "pub const CSS: &str = r#\"",
        "pub fn attach_motion(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not absorb logic/view/styles/motion implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub struct DateInputGroupStateDeriveInput {",
        "pub fn derive_state(input: DateInputGroupStateDeriveInput) -> DateInputGroupState {",
        "pub fn resolve_motion_source_attrs(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation/source-marker responsibility via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<html::Div>",
        "ui_motion::spring::SpringAnimator::new(",
        "style.set_property(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not absorb view/motion runtime details via `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        "var(--ui-fallback-",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS contract via `{required}`."
        );
    }

    for forbidden in ["#[component]", "view! {", "NodeRef<html::Div>"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not absorb component/render runtime details via `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn DateInputGroup(",
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "logic::derive_state(logic::DateInputGroupStateDeriveInput {",
        "view! {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep Leptos structure + headless mounting responsibility via `{required}`."
        );
    }

    for forbidden in [
        "pub const CSS: &str = r#\"",
        "pub struct DateInputGroupStateDeriveInput {",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not absorb styles/logic/motion-engine implementation via `{forbidden}`."
        );
    }

    for required in [
        "pub struct DateInputGroupMotion {",
        "pub fn sanitize_motion(motion: DateInputGroupMotion) -> DateInputGroupMotion {",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract + attach responsibility via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=\"date-input-group\"",
        "pub const CSS: &str = r#\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not absorb view/styles responsibilities via `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "scripts/check-ui-components-component-files.sh",
        "components/date-input-group/test/semantics.rs::date_input_group_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should pin standard-file-layout completion evidence `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should pin standard-file-layout completion evidence `{required}`."
        );
    }
}

#[test]
fn date_input_group_has_no_spec_rs_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        !spec_path.exists(),
        "DateInputGroup is a simple assembly component; `spec.rs` should not exist at `{spec_path:?}`."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "::spec::",
        "Spec::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "DateInputGroup should not wire spec-schema layer for this simple component via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_spec_rs_scope_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "N/A：`DateInputGroup` 当前不承载独立外部 Schema 契约",
        "date_input_group_has_no_spec_rs_for_simple_component",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin spec.rs-scope completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin spec.rs-scope completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_files_script =
        load_source("../../scripts/check-ui-components-component-files.sh");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    assert!(
        !spec_path.exists(),
        "DateInputGroup keeps Hyper-Structure spec builder as N/A; `spec.rs` should remain absent at `{spec_path:?}`."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "DateInputGroupSpec",
        "impl DateInputGroupSpec",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "DateInputGroup should not expose hyper-structure builder surface via `{forbidden}` for simple component scope."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A：`DateInputGroup` 为简单装配组件，当前不承载复杂外部 Schema/Builder 规范；保持 `spec.rs` 缺席并避免暴露 `*Spec::new()...render()` API。",
        "scripts/check-ui-components-component-files.sh",
        "date_input_group_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin hyper-structure-builder completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin hyper-structure-builder completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let component_files_script =
        load_source("../../scripts/check-ui-components-component-files.sh");
    let component_manifest = load_source("src/Component.toml");
    let component_rbi = load_source("src/date_input_group.rbi");
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required_file in ["Component.toml", "date_input_group.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "date-input-group context-compression file should exist: `{required_file}`."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"DateInputGroup\"",
        "crate = \"ui-date-input-group\"",
        "name = \"is_full_width\"",
        "name = \"variant\"",
        "name = \"is_disabled\"",
        "name = \"is_invalid\"",
        "name = \"is_segmented\"",
        "name = \"motion\"",
        "name = \"aria_label\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"prefix\"",
        "name = \"suffix\"",
        "name = \"class_name\"",
        "name = \"children\"",
        "name = \"semantic-markers\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "date-input-group Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub type DateInputGroupVariant = crate::DateInputGroupVariant;",
        "pub type DateInputGroupMotion = crate::DateInputGroupMotion;",
        "pub type DateInputGroupState = ui_state_primitives::date_input_group::DateInputGroupState;",
        "pub type DateInputGroupStateInput = ui_state_primitives::date_input_group::DateInputGroupStateInput;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn sanitize_motion(motion: crate::DateInputGroupMotion) -> crate::DateInputGroupMotion;",
        "pub fn DateInputGroup(",
        "is_full_width: bool,",
        "variant: crate::DateInputGroupVariant,",
        "motion: crate::DateInputGroupMotion,",
        "dir: Option<ui_headless::A11yDirection>,",
        "children: leptos::children::Children,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "date_input_group.rbi should keep signature-projection marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（`components/date-input-group/src/Component.toml` 与 `components/date-input-group/src/date_input_group.rbi` 已同步维护；`Component.toml` 覆盖输入输出轴与能力清单，`.rbi` 提供 `DateInputGroup` 接口签名投影，避免 AI 检索漂移。门禁脚本：`scripts/check-ui-components-component-files.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current`。回归：`components/date-input-group/test/semantics.rs::date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current`。）",
        "date_input_group_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep context-compression marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep context-compression marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_manifest = load_source("src/Component.toml");
    let component_rbi = load_source("src/date_input_group.rbi");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let hygiene_script = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for typed_source in [
        "pub const DATE_INPUT_GROUP_AGENT_SCHEMA: &str = \"ui.date-input-group.agent-contract\";",
        "pub enum DateInputGroupAgentSchemaVersion",
        "pub enum DateInputGroupAgentIntent",
        "pub enum DateInputGroupAgentAction",
        "pub enum DateInputGroupAgentState",
        "pub enum DateInputGroupAgentSource",
        "pub struct DateInputGroupAgentContract",
        "pub struct DateInputGroupAgentContractInput",
        "fn resolve_agent_state(render_state: DateInputGroupState) -> DateInputGroupAgentState",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "date-input-group Agent Contract should remain type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-aria-source=move || agent_contract.get().aria_source",
        "data-ui-class-source=move || agent_contract.get().class_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group view should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.date-input-group.agent-contract.v1\"",
        "intent = \"date.input-group\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "DATE_INPUT_GROUP_AGENT_SCHEMA",
        "DateInputGroupAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "date-input-group context-compression assets should keep Agent Contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-action",
        "format!(\"data-ui-state",
        "format!(\"data-ui-source",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "date-input-group Agent Contract markers must be type-derived, not free-form spliced via `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_agent_contract_is_schema_typed_and_machine_readable";
    assert!(
        hygiene_script.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "date_input_group_agent_contract_is_schema_typed_and_machine_readable",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep Agent Contract governance marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep Agent Contract governance marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/view.rs");
    let component_manifest = load_source("src/Component.toml");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let hygiene_script = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"render_prefix_slot(...)\"",
        "\"render_suffix_slot(...)\"",
        "\"logic::derive_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "\"date_input_group_motion::attach_motion(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
        "name = \"agent_contract_whitelist_boundary\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "date-input-group Component.toml should keep whitelist boundary marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "date-input-group render path should remain whitelist-safe and injection-free; forbidden marker `{forbidden}` was found."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free";
    assert!(
        hygiene_script.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for required in [
        "date_input_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep Agent Contract whitelist marker `{required}`."
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep Agent Contract whitelist marker `{required}`."
        );
    }
}

#[test]
fn date_input_group_streaming_term_is_limited_to_llm_output_render_modes() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_manifest = load_source("src/Component.toml");
    let hygiene_script = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "date_input_group_streaming_term_is_limited_to_llm_output_render_modes",
    ] {
        assert!(
            root_check2.contains(marker),
            "date-input-group/check2.md should keep streaming-definition marker `{marker}`."
        );
        assert!(
            src_check2.contains(marker),
            "date-input-group/src/check2.md should keep streaming-definition marker `{marker}`."
        );
    }

    for marker in [
        "pub enum DateInputGroupAgentStreamMode",
        "DateInputGroupAgentStreamMode::Streaming => \"streaming\"",
        "DateInputGroupAgentStreamMode::Snapshot => \"snapshot\"",
        "stream_support: DateInputGroupAgentStreamSupport::Unsupported,",
        "stream_fallback: DateInputGroupAgentStreamFallback::Snapshot,",
        "stream_mode: DateInputGroupAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "date-input-group logic should keep typed stream/snapshot marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group view should expose stream/snapshot contract marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "term_scope = \"llm-output-rendering\"",
        "defined_modes = [\"streaming\", \"snapshot\"]",
        "required = false",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "date-input-group manifest should keep stream/snapshot boundary marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_streaming_term_is_limited_to_llm_output_render_modes";
    assert!(
        hygiene_script.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn date_input_group_snapshot_is_foundational_and_complete_config_renders_stably() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_manifest = load_source("src/Component.toml");
    let hygiene_script = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "date_input_group_snapshot_is_foundational_and_complete_config_renders_stably",
    ] {
        assert!(
            root_check2.contains(marker),
            "date-input-group/check2.md should keep snapshot-foundation marker `{marker}`."
        );
        assert!(
            src_check2.contains(marker),
            "date-input-group/src/check2.md should keep snapshot-foundation marker `{marker}`."
        );
    }

    // Full-config render path: DateInputGroup accepts complete snapshot config surface.
    for marker in [
        "pub fn DateInputGroup(",
        "#[prop(optional)] is_full_width: bool,",
        "#[prop(optional)] variant: DateInputGroupVariant,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_invalid: bool,",
        "#[prop(optional)] is_segmented: bool,",
        "#[prop(optional)] motion: DateInputGroupMotion,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "#[prop(optional, into)] prefix: Option<ViewFn>,",
        "#[prop(optional, into)] suffix: Option<ViewFn>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "children: Children,",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group snapshot render should keep complete-config marker `{marker}`."
        );
    }

    for marker in [
        "DateInputGroupAgentOutputStatus::Verified => \"verified\"",
        "output_status: DateInputGroupAgentOutputStatus::Verified,",
        "DateInputGroupAgentStreamMode::Snapshot => \"snapshot\"",
        "stream_mode: DateInputGroupAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "date-input-group logic should keep snapshot/output-status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group view should expose snapshot output marker `{marker}`."
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "date-input-group manifest should keep snapshot-foundation marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_snapshot_is_foundational_and_complete_config_renders_stably";
    assert!(
        hygiene_script.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn date_input_group_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let component_manifest = load_source("src/Component.toml");
    let hygiene_script = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "date_input_group_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
    ] {
        assert!(
            root_check2.contains(marker),
            "date-input-group/check2.md should keep streaming-requirement marker `{marker}`."
        );
        assert!(
            src_check2.contains(marker),
            "date-input-group/src/check2.md should keep streaming-requirement marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "required = false",
        "owner = \"upstream\"",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "date-input-group manifest should keep streaming-optional marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "role=move || group_a11y.get_value().role",
        "aria-label=move || group_a11y.get_value().aria_label",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "date-input-group view should keep continuous stream/output/a11y marker `{marker}`."
        );
    }

    for marker in [
        "stream_mode: DateInputGroupAgentStreamMode::Snapshot,",
        "output_status: DateInputGroupAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(marker),
            "date-input-group logic should keep stream/output decision marker `{marker}`."
        );
    }

    let combined = format!("{logic_source}\n{view_source}");
    for forbidden in [
        "on_retry",
        "retry_count",
        "is_loading",
        "set_loading",
        "fetch(",
        "reqwest::",
        "tokio::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "date-input-group component layer should not absorb upstream retry/validation concerns via `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status";
    assert!(
        hygiene_script.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn date_input_group_docs_expose_hello_world_dx_path() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "<Playground",
        "title=\"Hello World (Default API)\"",
        "description=\"Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines.\"",
        "code_signal=hello_code",
        "let hello_code = Signal::derive(move || {",
        "<DateInputGroup>",
        "<DateField id_base=\"hello-date\".to_string() />",
        "<DateField id_base=\"docs-date-input-group-hello\".to_string() />",
    ] {
        assert!(
            source.contains(needle),
            "DateInputGroup docs should expose DX hello-world path via `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::",
        "DateInputGroupStateInput",
        "state=",
        "primitive_state=",
    ] {
        assert!(
            !source.contains(forbidden),
            "DateInputGroup docs hello-world path should not require internal wiring via `{forbidden}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] primitive_state:",
        "#[prop(optional)] headless_state:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup public API should not require internal state objects via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_dx_paradox_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] API 易用性验收标准（DX Paradox）",
        "`Hello World` 最小调用路径",
        "date_input_group_docs_expose_hello_world_dx_path",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin DX paradox completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin DX paradox completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_prefers_explicit_composition_over_parallel_slots() {
    let view_source = load_source("src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub fn DateInputGroup(",
        "children: Children,",
        "<DateInputGroup>",
        "<DateField id_base=\"hello-date\".to_string() />",
        "<DateField id_base=\"docs-date-input-group-hello\".to_string() />",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "DateInputGroup should keep explicit composition API evidence via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional, into)] panels:",
        "#[prop(optional, into)] item_specs:",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateInputGroup should not expose parallel-array/parallel-slot API via `{forbidden}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_explicit_composition_api_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 组合型组件主 API 必须“显示优于约定”",
        "`DateInputGroup` 仅暴露 `children: Children` 显式组合入口",
        "date_input_group_prefers_explicit_composition_over_parallel_slots",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin explicit-composition completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin explicit-composition completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "pub(super) fn date_input_group() -> AnyView",
        "let date_input_group_imports =",
        "code_imports=date_input_group_imports.clone()",
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "description=\"baseline-style date-input grouping primitive with centralized variant/width/prefix-suffix state contracts and segmented slot markers.\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "<DateInputGroup",
        "variant=DateInputGroupVariant::Secondary",
        "is_full_width=true",
        "is_invalid=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_groups docs page should include `{needle}` for date_input_group primary coverage.",
        );
    }
}

#[test]
fn date_input_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/date_input_group.rs");

    let date_input_group_start = docs_source
        .find("pub(super) fn date_input_group() -> AnyView")
        .unwrap_or_else(|| panic!("forms_groups should contain date_input_group docs entry."));
    let date_input_group_tail = &docs_source[date_input_group_start..];
    let date_input_group_end = date_input_group_tail
        .find("\npub(super) fn ")
        .unwrap_or(date_input_group_tail.len());
    let date_input_group_docs = &date_input_group_tail[..date_input_group_end];

    for required in [
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "data-slot=\"date-input-group-state-matrix\"",
        "variant=DateInputGroupVariant::Secondary",
        "is_full_width=true",
        "is_invalid=true",
        "is_segmented=true",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "data-slot=\"date-input-group-controlled-matrix\"",
        "value=controlled_date",
        "on_value_change=on_controlled_date_change",
        "default_value=Some(\"2026-04-09\".to_string())",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"date-input-group-streaming-contract\"",
    ] {
        assert!(
            date_input_group_docs.contains(required),
            "date-input-group docs section should keep docs/example/matrix sync marker `{required}`.",
        );
    }

    for forbidden in [
        "\n                        full_width=",
        "\n                        invalid=",
        "\n                        segmented=",
        "\n                        disabled=",
    ] {
        assert!(
            !date_input_group_docs.contains(forbidden),
            "date-input-group docs should use current `is_*` API names; found legacy alias `{forbidden}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_full_width: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] is_segmented: bool",
        "#[prop(optional)] variant: DateInputGroupVariant,",
        "logic::resolve_width(is_full_width)",
        "logic::resolve_status(is_disabled, is_invalid)",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "date-input-group source should keep API/default mapping marker `{required}`.",
        );
    }

    for required in [
        "pub enum DateInputGroupVariant {",
        "#[default]",
        "Primary,",
        "pub enum DateInputGroupWidth {",
        "Fit,",
        "pub enum DateInputGroupStatus {",
        "Default,",
    ] {
        assert!(
            primitive_source.contains(required),
            "state primitive defaults should remain stable for docs/default-value sync marker `{required}`.",
        );
    }
}

#[test]
fn date_input_group_documentation_entry_exists_with_beginner_first_progression() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let readme_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");

    let section_start = docs_source
        .find("pub(super) fn date_input_group() -> AnyView")
        .unwrap_or_else(|| panic!("forms_groups should contain date_input_group docs entry."));
    let section_tail = &docs_source[section_start..];
    let section_end = section_tail
        .find("\npub(super) fn ")
        .unwrap_or(section_tail.len());
    let date_input_group_docs = &section_tail[..section_end];

    assert!(
        readme_path.exists() || docs_source.contains("pub(super) fn date_input_group() -> AnyView"),
        "date-input-group should provide README or equivalent docs entry (docs-app route).",
    );

    for required in [
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "title=\"Hello World (Default API)\"",
        "description=\"Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines.\"",
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            date_input_group_docs.contains(required),
            "date-input-group docs entry should keep beginner-first marker `{required}`.",
        );
    }

    let hello_index = date_input_group_docs
        .find("title=\"Hello World (Default API)\"")
        .unwrap_or_else(|| panic!("docs should contain hello-world section."));
    let state_matrix_index = date_input_group_docs
        .find("title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"")
        .unwrap_or_else(|| panic!("docs should contain state-matrix section."));
    let controlled_index = date_input_group_docs
        .find("title=\"Controlled vs Uncontrolled (Child Field Axis)\"")
        .unwrap_or_else(|| panic!("docs should contain controlled/uncontrolled section."));
    let streaming_index = date_input_group_docs
        .find("title=\"Streaming / Snapshot Contract\"")
        .unwrap_or_else(|| panic!("docs should contain streaming/snapshot section."));

    assert!(
        hello_index < state_matrix_index
            && state_matrix_index < controlled_index
            && controlled_index < streaming_index,
        "date-input-group docs should keep progression: Hello World -> State Matrix -> Controlled/Uncontrolled -> Streaming/Snapshot.",
    );
}

#[test]
fn date_input_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for needle in [
        "let (invoice_date, set_invoice_date) = signal(Some(\"2026-03-14\".to_string()));",
        "let state_matrix_code = Signal::derive(move || {",
        "data-slot=\"date-input-group-state-matrix\"",
        "id_base=\"docs-date-input-group-matrix-default\".to_string()",
        "id_base=\"docs-date-input-group-invoice\".to_string()",
        "aria_label=\"Invoice date controls\".to_string()",
        "is_segmented=true",
        "prefix=move || view! { <span>\"📅\"</span> }",
        "suffix=move || view! { <span>\"UTC+0\"</span> }",
        "\"invoice date: \"",
        "let (ship_window, set_ship_window) = signal(Some(\"18:30\".to_string()));",
        "id_base=\"docs-date-input-group-time\".to_string()",
        "variant=DateInputGroupVariant::Secondary",
        "is_full_width=true",
        "is_invalid=true",
        "aria_label=\"Ship window controls\".to_string()",
        "class_name=\"docs-date-input-group-custom\".to_string()",
        "prefix=move || view! { <span>\"🕒\"</span> }",
        "suffix=move || view! { <span>\"5m\"</span> }",
        "minute_step=5",
        "\"ship window: \"",
        "let controlled_vs_uncontrolled_code = Signal::derive(move || {",
        "data-slot=\"date-input-group-controlled-matrix\"",
        "id_base=\"docs-date-input-group-controlled\".to_string()",
        "id_base=\"docs-date-input-group-uncontrolled\".to_string()",
        "default_value=Some(\"2026-04-09\".to_string())",
        "\"controlled date: \"",
        "\"uncontrolled DateField uses default_value and internal state after mount.\"",
        "let stream_snapshot_code = Signal::derive(move || {",
        "data-slot=\"date-input-group-streaming-contract\"",
        "description=\"Streaming is optional; fallback stays snapshot.\"",
        "\"requested mode: \"",
        "\"requested output status: \"",
        "data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified",
        "let source_first_code = Signal::derive(move || {",
        "data-slot=\"date-input-group-source-first\"",
        "Copy action auto-injects missing imports for direct run.",
    ] {
        assert!(
            source.contains(needle),
            "date_input_group docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_docs_sync_and_state_matrix_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        "apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group",
        "date_input_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "date_input_group_check2_documents_docs_sync_and_state_matrix_rules",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep docs-sync/state-matrix evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep docs-sync/state-matrix evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_documentation_as_product_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "date_input_group_documentation_entry_exists_with_beginner_first_progression",
        "date_input_group_check2_documents_documentation_as_product_rules",
        "date_input_group_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep documentation-as-product evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep documentation-as-product evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_interactive_playground_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "date_input_group_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "date_input_group_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "date_input_group_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep interactive-playground evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep interactive-playground evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_date_input_group_contract.spec.mjs");

    for required in [
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"date-input-group-streaming-contract\"",
        "let (requested_stream_mode, set_requested_stream_mode) = signal(\"streaming\".to_string());",
        "let (requested_output_status, set_requested_output_status) = signal(\"draft\".to_string());",
        "data-slot=\"date-input-group-requested-stream-mode\"",
        "data-slot=\"date-input-group-requested-output-status\"",
        "data-slot=\"date-input-group-streaming-requested-state\"",
        "data-slot=\"date-input-group-streaming-effective-state\"",
        "set_requested_stream_mode.set(event_target_value(&ev))",
        "set_requested_output_status.set(event_target_value(&ev))",
        "requested mode:",
        "requested output status:",
        "effective component markers: data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified",
    ] {
        assert!(
            docs_source.contains(required),
            "date-input-group docs should keep interactive-playground marker `{required}`.",
        );
    }

    for required in [
        "docs-app date-input-group key flow is repeatable with semantic failure breakpoints",
        "modeSelect.selectOption(\"snapshot\")",
        "outputSelect.selectOption(\"verified\")",
        "toContainText(\"requested mode: snapshot\")",
        "toContainText(\"requested output status: verified\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "date-input-group interactive-playground e2e should include `{required}`.",
        );
    }
}

#[test]
fn date_input_group_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_date_input_group_contract.spec.mjs");

    for required in [
        "async function gotoDateInputGroupDocsAndWaitSettled(page)",
        "docs-app date-input-group key flow is repeatable with semantic failure breakpoints",
        "await modeSelect.selectOption(\"snapshot\");",
        "await outputSelect.selectOption(\"verified\");",
        "await modeSelect.focus();",
        "await page.keyboard.press(\"ArrowUp\");",
        "await page.reload();",
        "body:not(:has(#boot))",
        "requested mode: streaming",
        "requested output status: draft",
    ] {
        assert!(
            e2e_source.contains(required),
            "date-input-group interactive-playground should reuse repeatable semantic e2e marker `{required}`.",
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
            "date-input-group interactive-playground should avoid flaky/non-semantic token `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "cargo test -p ui-date-input-group date_input_group_check2_documents_interactive_playground_rules",
        "cargo test -p ui-date-input-group date_input_group_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-date-input-group date_input_group_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(required),
            "date-input-group dx script should cover interactive-playground command `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_source_first_copy_paste_ready_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "date_input_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "date_input_group_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep source-first copy-paste-ready rule `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn date_input_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "let source_first_code = Signal::derive(move || {",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"date-input-group-source-first\"",
        "code_imports=date_input_group_imports",
        "Copy action auto-injects missing imports for direct run.",
        "component-date_input_group",
        "inject-css",
        "components/date-input-group/src/{mod,logic,view,styles,motion}.rs",
        "apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group",
        "data-slot=\"date-input-group-source-first-contract\"",
        "data-slot=\"date-input-group-source-first-dependency-baseline\"",
        "data-slot=\"date-input-group-source-prerequisites\"",
        "data-slot=\"date-input-group-source-paths\"",
        "components/date-input-group/src/mod.rs",
        "components/date-input-group/src/logic.rs",
        "components/date-input-group/src/view.rs",
        "components/date-input-group/src/styles.rs",
        "components/date-input-group/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(required),
            "date-input-group source-first docs should contain `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "#[prop(optional, into)] code_imports: Option<String>",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "missing_import_lines",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should contain `{required}`.",
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "cargo test -p ui-date-input-group date_input_group_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-date-input-group date_input_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "cargo test -p ui-date-input-group date_input_group_check2_marks_source_first_copy_paste_ready_contract_complete",
    ] {
        assert!(
            script_source.contains(required),
            "date-input-group dx script should cover source-first copy-paste-ready command `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group",
        "date_input_group_check2_documents_source_first_copy_paste_ready_rules",
        "date_input_group_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "date_input_group_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep source-first completion evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep source-first completion evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_documents_heroui_benchmark_docs_sync_rules() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep heroui-benchmark docs-sync rule `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn date_input_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");

    for required in [
        "### DateInputGroup 同步记录（2026-02-20）",
        "参数模型同步：`DateInputGroup` 参数主轴保持",
        "component_doc!(\"DateInputGroup\", \"date-input-group\", \"Forms\", forms_groups::date_input_group)",
        "forms_groups.rs::date_input_group()",
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include date-input-group synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"DateInputGroup\"",
        "\"date-input-group\"",
        "forms_groups::date_input_group",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose date-input-group entry marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn date_input_group() -> AnyView",
        "title=\"DateInputGroup\"",
        "slug=\"date-input-group\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app date-input-group page should stay indexable via marker `{required}`.",
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "cargo test -p ui-date-input-group date_input_group_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-date-input-group date_input_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "cargo test -p ui-date-input-group date_input_group_check2_marks_heroui_benchmark_docs_sync_contract_complete",
    ] {
        assert!(
            script_source.contains(required),
            "date-input-group dx script should cover heroui-benchmark docs-sync command `{required}`.",
        );
    }
}

#[test]
fn date_input_group_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "date_input_group_check2_documents_heroui_benchmark_docs_sync_rules",
        "date_input_group_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "date_input_group_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(required),
            "date-input-group/check2.md should keep heroui-benchmark docs-sync evidence `{required}`.",
        );
        assert!(
            src_check2.contains(required),
            "date-input-group/src/check2.md should keep heroui-benchmark docs-sync evidence `{required}`.",
        );
    }
}

#[test]
fn date_input_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot()
{
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "let date_input_group_imports =",
        "use leptos::prelude::*;\\nuse ui_components::{DateField, DateFieldTone, DateInputGroup, DateInputGroupVariant, TimeField, TimeFieldTone};",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "Streaming is optional; fallback stays snapshot.",
        "data-slot=\"date-input-group-state-matrix\"",
        "data-slot=\"date-input-group-controlled-matrix\"",
        "data-slot=\"date-input-group-streaming-contract\"",
        "data-slot=\"date-input-group-source-first\"",
        "code_imports=date_input_group_imports.clone()",
        "code_imports=date_input_group_imports",
        "requested mode:",
        "requested output status:",
        "effective component markers: data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified",
    ] {
        assert!(
            docs_source.contains(required),
            "date-input-group docs should keep copy-paste-ready docs-product marker `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "#[prop(optional, into)] code_imports: Option<String>",
        "compose_copy_ready_code",
        "missing_import_lines",
    ] {
        assert!(
            playground_source.contains(required),
            "shared Playground copy pipeline should keep `{required}`."
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-date-input-group date_input_group_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-date-input-group date_input_group_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_docs_sync_and_state_matrix_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix contract `{needle}`."
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-date-input-group date_input_group_documentation_entry_exists_with_beginner_first_progression",
        "cargo test -p ui-date-input-group date_input_group_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-date-input-group date_input_group_dx_check_script_covers_documentation_as_product_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`."
        );
    }
}

#[test]
fn date_input_group_dx_check_script_covers_docs_copy_paste_ready_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-date-input-group date_input_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-date-input-group date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn date_input_group_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "`Hello World (Default API)`",
        "`State Matrix (Default / Prefix-Suffix / Secondary+Invalid)`",
        "`Controlled vs Uncontrolled (Child Field Axis)`",
        "`Streaming / Snapshot Contract`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`apps/docs-app/src/playground.rs::compose_copy_ready_code`",
        "date_input_group_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "date_input_group_dx_check_script_covers_docs_copy_paste_ready_and_workbench_contract",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should keep docs-product evidence marker `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should keep docs-product evidence marker `{needle}`."
        );
    }
}

#[test]
fn date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
 {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dx_script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground should keep DX hot-reload/isolated-canvas marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn date_input_group() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix (Default / Prefix-Suffix / Secondary+Invalid)\"",
        "title=\"Controlled vs Uncontrolled (Child Field Axis)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let (invoice_date, set_invoice_date) = signal(Some(\"2026-03-14\".to_string()));",
        "let (ship_window, set_ship_window) = signal(Some(\"18:30\".to_string()));",
        "let (controlled_date, set_controlled_date) = signal(Some(\"2026-04-01\".to_string()));",
        "let (requested_stream_mode, set_requested_stream_mode) = signal(\"streaming\".to_string());",
        "let (requested_output_status, set_requested_output_status) = signal(\"draft\".to_string());",
        "let on_invoice_date_change = Callback::new(move |next: Option<String>| {",
        "let on_ship_window_change = Callback::new(move |next: Option<String>| {",
        "let on_controlled_date_change = Callback::new(move |next: Option<String>| {",
        "\"invoice date: \"",
        "\"ship window: \"",
        "Copy action auto-injects missing imports for direct run.",
    ] {
        assert!(
            docs_source.contains(required),
            "date-input-group docs should keep DX workbench/context marker `{required}`."
        );
    }

    for forbidden in [
        "DATE_INPUT_GROUP_WORKBENCH_STORAGE_KEY",
        "load_date_input_group_workbench_state(",
        "save_date_input_group_workbench_state(",
        "clear_date_input_group_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "date-input-group keeps optional persisted state as N/A in current scope; `{forbidden}` should remain absent."
        );
    }

    let dx_script_needle = "cargo test -p ui-date-input-group date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        dx_script_source.contains(dx_script_needle),
        "dx gate script should include `{dx_script_needle}`."
    );
}

#[test]
fn date_input_group_check2_marks_dx_requirements_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "N/A：`DateInputGroup` 当前不提供 workbench 持久化存储",
        "`apps/docs-app/src/playground.rs` 复用 `compose_scoped_css` + `data-playground-scope` + `playground__preview-stage` + `Restore original CSS` 形成无需重编 wasm 的样式热重载路径",
        "`apps/docs-app/src/pages/components/pages/forms_groups.rs` 的 `DateInputGroup` 页面提供 5 组 `Playground`（Hello World / State Matrix / Controlled vs Uncontrolled / Streaming-Snapshot / Source-first），并通过 `invoice_date/ship_window/controlled_date` 信号保持交互上下文可见",
        "`scripts/check-ui-components-dx.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na`",
        "date_input_group_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin dx-requirements completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin dx-requirements completion evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_engineering_capability_contract_is_na_and_runtime_agnostic() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let component_cargo = load_source("Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let engineering_script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    let protocol_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/protocol.rs");

    assert!(
        !spec_path.exists() && !protocol_path.exists(),
        "DateInputGroup should keep spec/config serde path as N/A for simple assembly scope."
    );

    for forbidden in [
        "serde",
        "tracing",
        "tokio",
        "async-std",
        "async_std",
        "smol",
    ] {
        assert!(
            !component_cargo.contains(forbidden),
            "date-input-group Cargo.toml should not leak engineering dependency `{forbidden}`."
        );
    }

    for required in [
        "[engineering] contract: serde schema + structured migration errors",
        "[engineering] contract: tracing target semantics",
        "[engineering] contract: runtime boundary leakage",
    ] {
        assert!(
            engineering_script_source.contains(required),
            "engineering gate script should keep shared baseline marker `{required}`."
        );
    }

    let engineering_script_needle = "cargo test -p ui-date-input-group date_input_group_engineering_capability_contract_is_na_and_runtime_agnostic";
    assert!(
        engineering_script_source.contains(engineering_script_needle),
        "engineering gate script should include `{engineering_script_needle}`."
    );

    assert!(
        ui_components_cargo.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "ui-components feature surface should keep shared tracing/debug baseline marker."
    );

    let combined =
        format!("{module_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for forbidden in [
        "use serde::",
        "Serialize",
        "Deserialize",
        "serde_json",
        "tracing::",
        "span!(",
        "event!(",
        "#[tracing::instrument]",
        "tokio::",
        "tokio_",
        "async_std::",
        "async-std",
        "smol::",
        "runtime::Handle",
        "spawn_blocking(",
        "JoinHandle",
    ] {
        assert!(
            !combined.contains(forbidden),
            "date-input-group source should avoid serde/tracing/runtime leakage marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`DateInputGroup` 无 spec/config 序列化输入与异步边界",
        "`scripts/check-ui-components-engineering.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_engineering_capability_contract_is_na_and_runtime_agnostic`",
        "date_input_group_engineering_capability_contract_is_na_and_runtime_agnostic",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin engineering-governance evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin engineering-governance evidence `{needle}`."
        );
    }
}

#[test]
fn date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");
    let engineering_script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let component_manifest = load_source("src/Component.toml");
    let rbi_source = load_source("src/date_input_group.rbi");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.date-input-group.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "date-input-group Component.toml should keep stable v1 registration marker `{required}`.",
        );
    }

    for required in [
        "pub enum DateInputGroupAgentSchemaVersion {",
        "DateInputGroupAgentSchemaVersion::V1 => \"v1\"",
        "pub struct DateInputGroupAgentContract {",
    ] {
        assert!(
            logic_source.contains(required),
            "date-input-group logic should keep stable v1 contract marker `{required}`.",
        );
    }

    for required in [
        "pub enum DateInputGroupAgentSchemaVersion {",
        "V1,",
        "pub fn DateInputGroup(",
    ] {
        assert!(
            rbi_source.contains(required),
            "date-input-group RBI should keep stable public API/schema marker `{required}`.",
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "SchemaRegistry",
        "schema_version = \"2\"",
        "agent-contract.v2",
        "V2",
    ] {
        assert!(
            !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "date-input-group should not introduce major-version migration token `{forbidden}` in current scope.",
        );
    }

    let script_needle = "cargo test -p ui-date-input-group date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        engineering_script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `DateInputGroup` 改动未引入跨大版本 API 破坏升级，组件 Agent Contract 仍保持 `v1`（`components/date-input-group/src/logic.rs` 的 `DateInputGroupAgentSchemaVersion::V1`，以及 `components/date-input-group/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.date-input-group.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/date-input-group/test/semantics.rs::date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-components-engineering.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`。）",
        "date_input_group_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin version-migration marker `{needle}`.",
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin version-migration marker `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
 {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "date-input-group non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-date-input-group\")",
        "Cow::Borrowed(state.variant_class)",
        "Cow::Borrowed(state.width_class)",
        "classes.push(Cow::Owned(base_class_name));",
        "composed.push_str(class_name.as_ref());",
    ] {
        assert!(
            logic_source.contains(required),
            "date-input-group logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-date-input-group\".to_string()",
        "state.variant_class.to_string()",
        "state.width_class.to_string()",
        "String::from(\"ui-date-input-group\")",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "date-input-group class-name composition should avoid clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn date_input_group_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui-date-input-group date_input_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-date-input-group date_input_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-date-input-group date_input_group_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_check2_marks_rust_hygiene_contract_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/date-input-group/src/mod.rs`、`components/date-input-group/src/logic.rs`、`components/date-input-group/src/styles.rs`、`components/date-input-group/src/view.rs`、`components/date-input-group/src/motion.rs` 非测试源码已保持无 `unwrap/expect` 且无 `let _ = ...`；`components/date-input-group/src/logic.rs` 的 `compose_class_name` 采用 `Vec<Cow<'static, str>>` 收敛 class 字符串复制热点。回归：`components/date-input-group/test/semantics.rs::date_input_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/date-input-group/test/semantics.rs::date_input_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/date-input-group/test/semantics.rs::date_input_group_rust_hygiene_script_enforces_repo_level_hygiene_guards`；门禁脚本：`scripts/check-ui-components-engineering.sh` 已接入对应 `cargo test` 目标；另执行：`./scripts/check-rust-hygiene.sh`（若失败以脚本输出为准）。）",
        "date_input_group_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "date_input_group_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "date_input_group_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should keep rust-hygiene evidence marker `{needle}`.",
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should keep rust-hygiene evidence marker `{needle}`.",
        );
    }
}

#[test]
fn date_input_group_wasm_debug_contract_is_na_and_feature_isolated() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let component_cargo = load_source("Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let wasm_debug_script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for required in ["[features]", "default = []"] {
        assert!(
            component_cargo.contains(required),
            "date-input-group crate should keep minimal feature boundary marker `{required}`."
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing"] {
        assert!(
            !component_cargo.contains(forbidden),
            "date-input-group crate should not leak wasm-debug feature surface `{forbidden}`."
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components feature graph should keep shared wasm-debug marker `{required}`."
        );
    }

    for forbidden in [
        "date-input-group-wasm-debug =",
        "date_input_group-wasm-debug =",
        "date_input_group_wasm_debug =",
        "component-date_input_group\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components feature graph should not leak date-input-group debug toggle `{forbidden}`."
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components root should keep global wasm-debug isolation marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs-app should keep dev-only debug overlay entry `{required}`."
        );
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "events.into_iter().rev().take(40)",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(required) || debug_overlay_source.contains(required),
            "global trace/debug-overlay contract should keep marker `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "DateInputGroup should keep stable state/source marker `{required}` for debug traceability."
        );
    }

    let combined =
        format!("{module_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for forbidden in [
        "use_ui_trace(",
        "provide_ui_trace(",
        "trace.emit(",
        "debug_overlay",
        "request_replay",
        "trace_id",
        "wasm_debug_proxy!",
        "observability::",
        "#[prop(optional)] debug",
        "data-debug-",
    ] {
        assert!(
            !combined.contains(forbidden),
            "date-input-group runtime/public contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    let wasm_debug_script_needle = "cargo test -p ui-date-input-group date_input_group_wasm_debug_contract_is_na_and_feature_isolated";
    assert!(
        wasm_debug_script_source.contains(wasm_debug_script_needle),
        "wasm-debug gate script should include `{wasm_debug_script_needle}`."
    );
}

#[test]
fn date_input_group_check2_marks_wasm_debug_complete() {
    let root_check2 = load_source("check2.md");
    let src_check2 = load_source("src/check2.md");

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "本组件判定：N/A（组件级不自建 wasm 调试/回放管线）",
        "`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` 并挂载 `<debug_overlay::UiDebugOverlay enabled=true />`",
        "`scripts/check-ui-components-wasm-debug.sh` 已接入 `cargo test -p ui-date-input-group date_input_group_wasm_debug_contract_is_na_and_feature_isolated`",
        "date_input_group_wasm_debug_contract_is_na_and_feature_isolated",
    ] {
        assert!(
            root_check2.contains(needle),
            "date-input-group/check2.md should pin wasm-debug completion evidence `{needle}`."
        );
        assert!(
            src_check2.contains(needle),
            "date-input-group/src/check2.md should pin wasm-debug completion evidence `{needle}`."
        );
    }
}
