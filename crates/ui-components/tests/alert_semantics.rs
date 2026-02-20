use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_alert_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/alert").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_alert_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/alert/test").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_alert_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-alert\")]")
            && lib_source.contains("pub use ui_alert as alert;"),
        "ui-components should re-export the external ui-alert crate as `alert`.",
    );
    assert!(
        cargo_source.contains("component-alert = [\"dep:ui-alert\"]"),
        "component-alert feature should depend on dep:ui-alert after extraction.",
    );
    assert!(
        cargo_source.contains("ui-alert = { path = \"../../components/alert\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-alert dependency.",
    );
}

#[test]
fn alert_component_module_exposes_unified_notification_contract() {
    let module_source = load_alert_component_source("src/mod.rs");

    for needle in [
        "pub use logic::{AlertFill, AlertLayout, AlertTone, AlertVariant};",
        "pub use motion::AlertMotion;",
        "pub use view::Alert;",
    ] {
        assert!(
            module_source.contains(needle),
            "alert component module should export `{needle}`."
        );
    }
}

#[test]
fn alert_component_manifest_defines_machine_readable_contract() {
    let manifest_source = load_alert_component_source("src/Component.toml");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Alert\"",
        "crate = \"ui-alert\"",
        "name = \"tone\"",
        "name = \"variant\"",
        "name = \"layout\"",
        "name = \"fill\"",
        "name = \"motion\"",
        "name = \"a11y_status_region\"",
        "name = \"ui-state-primitives\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Alert component manifest should include `{needle}`."
        );
    }
}

#[test]
fn alert_component_rbi_projects_public_interface_signatures() {
    let rbi_source = load_alert_component_source("src/alert.rbi");

    for needle in [
        "pub enum AlertLayout",
        "pub enum AlertVariant",
        "pub type AlertTone = ui_state_primitives::alert_banner::AlertBannerTone;",
        "pub type AlertFill = ui_state_primitives::alert_banner::AlertBannerFill;",
        "pub struct AlertMotion",
        "pub fn Alert(",
        "-> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "Alert RBI projection should include `{needle}`."
        );
    }
}

#[test]
fn alert_view_accepts_tone_fill_layout_and_slot_props() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "#[prop(optional)] tone: Option<AlertTone>",
        "#[prop(optional)] variant: Option<AlertVariant>",
        "#[prop(optional)] layout: Option<AlertLayout>",
        "#[prop(optional)] fill: Option<AlertFill>",
        "#[prop(optional)] is_hide_icon: Option<bool>",
        "#[prop(optional)] hide_icon: Option<bool>",
        "#[prop(optional, into)] start_content: Option<ViewFn>",
        "#[prop(optional, into)] end_content: Option<ViewFn>",
        "#[prop(optional)] motion: AlertMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should expose `{needle}` in public props."
        );
    }
}

#[test]
fn alert_api_naming_contract_keeps_is_prefix_with_legacy_alias_bridge() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        logic_source.contains("pub fn resolve_hide_icon("),
        "Alert logic should provide a centralized hide_icon compatibility resolver."
    );
    for needle in [
        "#[prop(optional)] is_hide_icon: Option<bool>",
        "#[prop(optional)] hide_icon: Option<bool>",
        "let (hide_icon, hide_icon_source) = logic::resolve_hide_icon(is_hide_icon, hide_icon);",
        "data-hide-icon-source=hide_icon_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert API naming compatibility contract should include `{needle}`."
        );
    }
}

#[test]
fn alert_defaults_and_state_normalization_stay_in_logic_layer() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "pub fn normalize_fill(fill: Option<AlertFill>) -> AlertFill",
        "pub fn normalize_layout(layout: Option<AlertLayout>) -> AlertLayout",
        "pub fn resolve_hide_icon(",
        "pub fn resolve_state(input: AlertStateInput) -> AlertState",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should own normalization/default contract `{needle}`."
        );
    }

    for needle in [
        "let title = logic::normalize_optional_text(title);",
        "let description = logic::normalize_optional_text(description);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let (hide_icon, hide_icon_source) = logic::resolve_hide_icon(is_hide_icon, hide_icon);",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should consume logic-normalized values via `{needle}`."
        );
    }
    assert!(
        !view_source.contains("match (is_hide_icon, hide_icon)"),
        "Alert view should not duplicate hide_icon default logic outside logic.rs."
    );
}

#[test]
fn alert_logic_uses_notification_primitives_and_variant_mapping() {
    let logic_source = load_alert_component_source("src/logic.rs");

    for needle in [
        "AlertBannerFill as AlertFill",
        "AlertBannerTone as AlertTone",
        "pub enum AlertLayout",
        "pub enum AlertVariant",
        "pub fn as_tone(self) -> AlertTone",
        "pub fn resolve_state(input: AlertStateInput) -> AlertState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: AlertState) -> String",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should include `{needle}`."
        );
    }
}

#[test]
fn alert_view_derives_state_from_logic_layer() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "let state = logic::resolve_state(AlertStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
        "let motion = crate::motion::sanitize_motion(motion);",
        "alert_motion::attach_motion(node_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should derive behavior via `{needle}`."
        );
    }
}

#[test]
fn alert_emits_unified_state_markers() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "data-slot=\"alert\"",
        "data-layout=state.layout_attr",
        "data-tone=state.tone_attr",
        "data-fill=state.fill_attr",
        "data-icon=state.icon_attr",
        "data-variant-source=state.variant_source_attr",
        "data-hide-icon-source=hide_icon_source",
        "data-icon-label-source=icon_label_source",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-motion-source=if motion == AlertMotion::default()",
        "data-custom-motion=(motion != AlertMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should expose `{needle}` for semantic/state inspection."
        );
    }
}

#[test]
fn alert_a11y_and_i18n_contract_is_mounted_from_headless_locale_bridge() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] icon_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-icon-label-source=icon_label_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert A11y/i18n contract should include `{needle}`."
        );
    }
}

#[test]
fn alert_styles_cover_tone_fill_layout_and_motion_contracts() {
    let styles_source = load_alert_component_source("src/styles.rs");

    for needle in [
        "--ui-alert-opacity",
        "--ui-alert-translate-y",
        "--ui-alert-scale",
        ".ui-alert[data-motion-source=\"custom\"]",
        ".ui-alert--layout-inline",
        ".ui-alert[data-layout=\"inline\"]",
        ".ui-alert--tone-info",
        ".ui-alert[data-tone=\"negative\"]",
        ".ui-alert--fill-border",
        ".ui-alert[data-fill=\"bold\"]",
        ".ui-alert__icon",
        ".ui-alert__content",
    ] {
        assert!(
            styles_source.contains(needle),
            "Alert styles should include `{needle}`."
        );
    }
}

#[test]
fn alert_styles_depend_on_explicit_state_selectors() {
    let styles_source = load_alert_component_source("src/styles.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for selector in [
        ".ui-alert[data-layout=\"inline\"]",
        ".ui-alert[data-tone=\"negative\"]",
        ".ui-alert[data-fill=\"bold\"]",
        ".ui-alert[data-icon=\"hidden\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Alert styles should include explicit state selector `{selector}`."
        );
    }

    for bad_selector in [":nth-child", ":nth-of-type"] {
        assert!(
            !styles_source.contains(bad_selector),
            "Alert styles should avoid fragile structural selector `{bad_selector}`."
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "Alert view should not inject inline style properties."
    );
}

#[test]
fn alert_motion_contract_remains_spring_based() {
    let motion_source = load_alert_component_source("src/motion.rs");
    let motion_checks_source = load_alert_test_source("motion.rs");
    let combined = format!("{motion_source}\n{motion_checks_source}");

    for needle in [
        "pub struct AlertMotion",
        "SpringAnimator",
        "pub fn sanitize_motion(motion: AlertMotion) -> AlertMotion",
        "fn default_motion_matches_alert_spring_contract()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            combined.contains(needle),
            "Alert motion should include `{needle}`."
        );
    }
}

#[test]
fn alert_component_file_responsibilities_match_standard_layout() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let alert_src = workspace_dir.join("components/alert/src");

    for required in [
        "mod.rs",
        "logic.rs",
        "view.rs",
        "styles.rs",
        "motion.rs",
        "protocol.rs",
    ] {
        assert!(
            alert_src.join(required).exists(),
            "Alert component should include `{required}`."
        );
    }

    for forbidden in ["spec.rs", "render.rs"] {
        assert!(
            !alert_src.join(forbidden).exists(),
            "Alert component should not include `{forbidden}` for current complexity tier."
        );
    }
}

#[test]
fn alert_tree_shaking_contract_is_feature_gated_for_module_and_css() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-alert\")]")
            && lib_source.contains("pub use ui_alert as alert;"),
        "Alert module export should stay under component-alert feature gate."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-alert\")]")
            && css_source.contains("out.push_str(crate::alert::styles::CSS);"),
        "Alert CSS aggregation should stay under component-alert feature gate."
    );
    assert!(
        cargo_source.contains("component-alert = [\"dep:ui-alert\"]"),
        "component-alert feature should map to dep:ui-alert in Cargo.toml."
    );
}

#[test]
fn alert_dom_environment_contracts_cover_cfg_and_hydration_boundaries() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let motion_source = load_alert_component_source("src/motion.rs");
    let headless_lib_source = load_ui_components_source("../../crates/ui-headless/src/lib.rs");

    assert!(
        headless_lib_source.contains("features `web` and `ssr` are mutually exclusive"),
        "ui-headless should keep compile_error guard for web/ssr mutual exclusion."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion()",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Alert motion should include platform/reduced-motion guard `{needle}`."
        );
    }

    for forbidden in [
        "now()",
        "Uuid::new_v4",
        "rand::",
        "inner_html",
        "set_inner_html",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Alert should not depend on forbidden hydration/unsafe HTML token `{forbidden}`."
        );
    }
}

#[test]
fn alert_view_structure_avoids_macro_bloat_and_prefers_function_split() {
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        view_source
            .contains("fn render_alert_icon(layout: AlertLayout, tone: AlertTone) -> AnyView"),
        "Alert view should split icon rendering into plain function helper."
    );
    assert!(
        view_source.contains("{render_alert_icon(state.layout, state.tone)}"),
        "Alert main view! block should consume icon helper instead of inlining giant match."
    );
}

#[test]
fn docs_page_uses_unified_alert_api() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn alert() -> AnyView",
        "title=\"Alert\"",
        "slug=\"alert\"",
        "tone=AlertTone::Info",
        "fill=AlertFill::Border",
        "layout=AlertLayout::Inline",
        "is_hide_icon=true",
        "variant=AlertVariant::Danger",
        "motion=AlertMotion {",
    ] {
        assert!(
            display_source.contains(needle),
            "Alert docs should include `{needle}`."
        );
    }
}

#[test]
fn docs_navigation_no_longer_lists_alert_banner_or_inline_alert() {
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        !pages_source.contains("\"alert-banner\""),
        "components pages should no longer expose a separate alert-banner route."
    );
    assert!(
        !pages_source.contains("\"inline-alert\""),
        "components pages should no longer expose a separate inline-alert route."
    );
}
