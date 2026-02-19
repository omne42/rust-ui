use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn alert_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/alert/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Alert internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn alert_module_exposes_stable_component_contract() {
    let module_source = load_source("src/alert/mod.rs");

    for needle in [
        "pub use logic::AlertVariant;",
        "pub use motion::AlertMotion;",
        "pub use view::Alert;",
    ] {
        assert!(
            module_source.contains(needle),
            "alert module should export `{needle}` as the stable public contract."
        );
    }

    for forbidden in [
        "pub use logic::AlertState",
        "pub use logic::AlertStateInput",
        "pub use logic::resolve_state",
        "pub use motion::attach_motion",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "alert module should keep internals private; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_public_api_names_follow_contract() {
    let view_source = load_source("src/alert/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "#[prop(optional)] variant: Option<AlertVariant>",
        "#[prop(optional, into)] title: Option<String>",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: AlertMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert public props should keep stable naming; missing `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] is_",
        "#[prop(optional)] on_",
        "#[prop(optional)] default_",
        "#[prop(optional, into)] class: Option<String>",
        "on_open_change",
        "open=",
        "default_open",
        "title_text",
        "description_text",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should avoid naming drift/alias props; found `{forbidden}`."
        );
    }

    for needle in [
        "variant=AlertVariant::Default",
        "title=\"Notice\".to_string()",
        "description=\"Something happened.\".to_string()",
        "class_name=\"docs-alert-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "Alert docs should use canonical prop names; missing `{needle}`."
        );
    }
}

#[test]
fn alert_has_no_controllable_uncontrollable_state_axis() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "default_open",
        "on_open_change",
        "open: Option<bool>",
        "value: Option<",
        "default_value",
        "on_value_change",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should not expose partial controlled/uncontrolled contracts; found `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Alert docs should not teach uncontrolled/controlled aliases for a stateless component; found `{forbidden}`."
        );
    }

    for forbidden in [
        "signal(",
        "create_signal(",
        "ReadSignal<",
        "WriteSignal<",
        "RwSignal<",
        "use_controllable_state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should not hide local state for a stateless component; found `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Alert logic should not hide local state for a stateless component; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_uses_logic_state_model() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");

    for needle in [
        "use ui_headless::{LiveRegionPriority, live_region_attrs};",
        "use ui_state_primitives::alert::{AlertStateCoreInput, resolve_state_core};",
        "pub use ui_state_primitives::alert::normalize_optional_text;",
        "pub fn normalize_variant(variant: Option<AlertVariant>) -> AlertVariant",
        "pub struct AlertStateInput",
        "pub struct AlertState",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should include `{needle}` for primitive-backed state derivation."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let motion = crate::alert::motion::sanitize_motion(motion);",
        "let variant = logic::normalize_variant(variant);",
        "logic::normalize_optional_text(title)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "let locale = locale_attrs(lang, dir);",
        "logic::resolve_state(AlertStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn alert_defaults_are_normalized_only_in_logic() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");

    assert!(
        logic_source
            .contains("pub fn normalize_variant(variant: Option<AlertVariant>) -> AlertVariant"),
        "Alert logic should own variant default normalization."
    );
    assert!(
        logic_source.contains("variant.unwrap_or_default()"),
        "Alert logic should use a single explicit default rule."
    );
    assert!(
        view_source.contains("let variant = logic::normalize_variant(variant);"),
        "Alert view should consume normalized variant from logic."
    );

    for forbidden in [
        "unwrap_or(",
        "unwrap_or_else(",
        "unwrap_or_default(",
        "AlertVariant::Default",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should not own default fallback branches; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");
    let styles_source = load_source("src/alert/styles.rs");

    assert!(
        logic_source.contains("pub struct AlertStateInput"),
        "Alert logic should define the typed input boundary for normalization."
    );
    assert!(
        logic_source.contains("pub fn resolve_state(input: AlertStateInput) -> AlertState"),
        "Alert logic should own state derivation."
    );
    assert!(
        view_source.contains("logic::resolve_state(AlertStateInput {"),
        "Alert view should consume normalized state from logic."
    );

    for forbidden in [
        "resolve_state_core(",
        "live_region_attrs(",
        "AlertStateCoreInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should not reimplement state-machine rules; found `{forbidden}`."
        );
    }

    for forbidden in ["state_attr == ", "variant_attr == ", "title_attr == "] {
        assert!(
            !styles_source.contains(forbidden),
            "Alert styles should consume state markers, not rebuild state logic; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_discrete_state_axes_are_type_constrained() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");

    assert!(
        logic_source.contains("pub enum AlertVariant"),
        "Alert should model discrete variant state using a typed enum."
    );
    assert!(
        view_source.contains("#[prop(optional)] variant: Option<AlertVariant>"),
        "Alert public API should keep variant typed as AlertVariant."
    );

    for forbidden in [
        "variant: Option<String>",
        "variant: String",
        "variant: Option<bool>",
        "is_default:",
        "is_danger:",
        "is_accent:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should avoid bool/string state explosions for variant; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_uses_state_primitives_as_the_state_source() {
    let logic_source = load_source("src/alert/logic.rs");
    let view_source = load_source("src/alert/view.rs");

    assert!(
        logic_source
            .contains("use ui_state_primitives::alert::{AlertStateCoreInput, resolve_state_core};"),
        "Alert logic should consume ui-state-primitives as the state source."
    );
    assert!(
        logic_source.contains("resolve_state_core(AlertStateCoreInput {"),
        "Alert logic should delegate core state derivation to ui-state-primitives."
    );

    for forbidden in [
        "redux",
        "zustand",
        "pinia",
        "use_store(",
        "GlobalStore",
        "app_state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should not bind directly to app-level stores; found `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Alert logic should not bind directly to app-level stores; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_has_no_async_interaction_protocol() {
    let view_source = load_source("src/alert/view.rs");
    let logic_source = load_source("src/alert/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "spawn_local",
        "async ",
        ".await",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should stay synchronous and stateless; found `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Alert logic should stay synchronous and stateless; found `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("pub(super) fn alert() -> AnyView"),
        "Alert docs entry should exist for synchronous/snapshot rendering usage."
    );
}

#[test]
fn alert_file_responsibilities_stay_layered() {
    let logic_source = load_source("src/alert/logic.rs");
    let view_source = load_source("src/alert/view.rs");
    let motion_source = load_source("src/alert/motion.rs");
    let styles_source = load_source("src/alert/styles.rs");

    for forbidden in ["view!", "NodeRef<", "children()"] {
        assert!(
            !logic_source.contains(forbidden),
            "alert logic should remain pure normalization/derivation; found `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_state(AlertStateInput {",
        "attach_motion",
        "locale_attrs(lang, dir)",
    ] {
        assert!(
            view_source.contains(needle),
            "alert view should mount composed semantics/motion via `{needle}`."
        );
    }

    for forbidden in ["resolve_state_core(", "live_region_attrs("] {
        assert!(
            !view_source.contains(forbidden),
            "alert view should not bypass logic/state contracts; found `{forbidden}`."
        );
    }

    for forbidden in ["role=", "aria-live", "data-slot"] {
        assert!(
            !motion_source.contains(forbidden),
            "alert motion should not own component semantics/markup; found `{forbidden}`."
        );
    }

    for forbidden in ["view!", "#[component]", "use leptos"] {
        assert!(
            !styles_source.contains(forbidden),
            "alert styles should remain static token-first CSS; found `{forbidden}`."
        );
    }
}

#[test]
fn alert_attaches_motion_driver() {
    let source = load_source("src/alert/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Alert should attach its motion driver to deliver spring-based reveal motion."
    );
}

#[test]
fn alert_exposes_motion_source_markers() {
    let source = load_source("src/alert/view.rs");

    for needle in [
        "data-slot=\"alert\"",
        "data-motion-source=if motion == AlertMotion::default()",
        "data-custom-motion=(motion != AlertMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Alert should expose `{needle}` for baseline motion inspection."
        );
    }
}

#[test]
fn alert_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/alert/view.rs");

    for attr in [
        "data-slot=\"alert\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-title=state.title_attr",
        "data-description=state.description_attr",
        "data-actions=state.actions_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "lang=locale.lang",
        "dir=locale.dir",
        "data-motion-source=if motion == AlertMotion::default()",
        "data-custom-motion=(motion != AlertMotion::default()).then_some(\"true\")",
        "data-slot=\"alert-title\"",
        "data-slot=\"alert-description\"",
        "data-slot=\"alert-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "Alert should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn alert_motion_uses_spring_animator() {
    let source = load_source("src/alert/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Alert motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn alert_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/alert/motion.rs");

    for needle in [
        "pub struct AlertMotion",
        "fn default_motion_matches_alert_spring_contract()",
        "fn supports_custom_spring_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Alert motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn alert_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/alert/motion.rs");
    let view_source = load_source("src/alert/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AlertMotion) -> AlertMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Alert motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::alert::motion::sanitize_motion(motion);"),
        "Alert view should sanitize motion before attaching spring driver.",
    );
}

#[test]
fn alert_styles_include_state_marker_contracts() {
    let source = load_source("src/alert/styles.rs");

    for selector in [
        "--ui-alert-opacity",
        "--ui-alert-translate-y",
        "--ui-alert-scale",
        "gap: var(--ui-space-2xs);",
        ".ui-alert[data-motion-source=\"custom\"]",
        ".ui-alert[data-custom-motion=\"true\"]",
        ".ui-alert--detailed",
        ".ui-alert[data-state=\"compact\"]",
        ".ui-alert--with-actions",
        ".ui-alert[data-actions=\"absent\"] .ui-alert__actions",
        ".ui-alert--variant-accent",
        ".ui-alert[data-variant=\"danger\"]",
        ".ui-alert--custom-class",
        ".ui-alert[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Alert styles should include `{selector}` as stable state-marker contracts."
        );
    }

    assert!(
        !source.contains("var(--ui-space-2xs, 4px)"),
        "Alert styles should consume theme spacing tokens directly without hardcoded fallback literals.",
    );
}

#[test]
fn alert_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn alert() -> AnyView",
        "title=\"Alert\"",
        "slug=\"alert\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Variants + Live Region\"",
        "Playground title=\"Custom Class + Compact\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Alert.",
        );
    }
}

#[test]
fn alert_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Variants + Live Region\"",
        "variant=AlertVariant::Default",
        "variant=AlertVariant::Accent",
        "variant=AlertVariant::Danger",
        "title=\"Custom Class + Compact\"",
        "description=\"Custom class without title\".to_string()",
        "class_name=\"docs-alert-custom\".to_string()",
        "<Alert variant=AlertVariant::Default title=\"Heads up\".to_string()>",
    ] {
        assert!(
            source.contains(needle),
            "alert docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn alert_docs_keep_dx_first_and_hide_internal_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "r#\"<Alert title=\"Notice\".to_string()>",
        "Playground title=\"Hello World\" code_signal=hello_world_code",
        "<Alert title=\"Notice\".to_string()>",
    ] {
        assert!(
            source.contains(needle),
            "alert docs should provide a minimal hello-world path; missing `{needle}`."
        );
    }

    for forbidden in [
        "state=",
        "ui_state_primitives",
        "ui_headless",
        "use_controllable_state",
        "labels=",
        "titles=",
    ] {
        assert!(
            !source.contains(forbidden),
            "alert docs should not force internal wiring or implicit array pairing; found `{forbidden}`."
        );
    }
}
