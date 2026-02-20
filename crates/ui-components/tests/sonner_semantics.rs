use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sonner_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/toast/src/sonner/mod.rs");

    for needle in ["pub mod logic", "pub mod view", "pub mod motion"] {
        assert!(
            !source.contains(needle),
            "Sonner internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sonner_is_publicly_exported_from_module_and_crate_root() {
    let sonner_mod = load_source("../../components/toast/src/sonner/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        sonner_mod.contains("pub use view::Sonner;"),
        "sonner::mod should re-export Sonner."
    );
    assert!(
        sonner_mod.contains("DEFAULT_PORTAL") && sonner_mod.contains("DEFAULT_MAX_TOASTS"),
        "sonner::mod should expose default portal/max-toasts contracts."
    );
    assert!(
        crate_root.contains("pub use sonner::{Sonner, SonnerPosition};"),
        "crate root should expose Sonner and SonnerPosition."
    );
}

#[test]
fn sonner_api_naming_contract_matches_overlay_family_without_alias_drift() {
    let sonner_view = load_source("../../components/toast/src/sonner/view.rs");
    let toast_view = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] motion: ToastMotion",
    ] {
        assert!(
            sonner_view.contains(needle),
            "Sonner API should keep stable naming contract via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, default = logic::DEFAULT_VIEWPORT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_VIEWPORT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            toast_view.contains(needle),
            "ToastViewport should keep same overlay naming contract via `{needle}`."
        );
    }

    for forbidden in [
        "is_portal",
        "default_portal",
        "on_portal_change",
        "is_max_toasts",
        "default_max_toasts",
        "on_max_toasts_change",
    ] {
        assert!(
            !sonner_view.contains(forbidden),
            "Sonner should avoid naming alias drift in public API: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "#[prop(optional)] position: SonnerPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "if let Some(provided_store) = store",
        "SonnerStoreSource::Provided",
        "SonnerStoreSource::Context",
        "SonnerStoreSource::Local",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should keep explicit non-controllable host contract via `{needle}`."
        );
    }

    for forbidden in [
        "is_open",
        "open:",
        "default_open",
        "on_open_change",
        "value: ",
        "on_value_change",
        "default_value",
        "default_portal",
        "default_max_toasts",
        "on_portal_change",
        "on_max_toasts_change",
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Sonner should not expose half-controlled API surface: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_module_exposes_slot_and_part_state_contracts() {
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let primitive_source = load_source("../ui-state-primitives/src/sonner.rs");

    for needle in [
        "pub use ui_state_primitives::sonner::{",
        "SonnerSlot",
        "SonnerStoreSource",
        "SonnerPartStateInput",
        "SonnerPartState",
    ] {
        assert!(
            mod_source.contains(needle),
            "Sonner module should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub enum SonnerSlot",
        "pub enum SonnerStoreSource",
        "pub struct SonnerPartStateInput",
        "pub struct SonnerPartState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn base_class(self) -> &'static str",
        "pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives sonner module should define `{needle}`."
        );
    }
}

#[test]
fn sonner_view_uses_logic_state_contracts() {
    let source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, region_attrs};",
        "let motion = crate::sonner::motion::sanitize_motion(motion);",
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "logic::resolve_state(SonnerPartStateInput {",
        "slot: SonnerSlot::Root",
        "slot: SonnerSlot::Viewport",
        "logic::compose_class_name(class_name.get_value(), root_state)",
        "logic::compose_class_name(None, viewport_state)",
        "let agent_contract = logic::agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-portal=root_state.portal_attr",
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-max-toasts-source=root_state.max_toasts_source_attr",
        "data-aria-source=root_state.aria_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
        "data-custom-position=root_state.has_custom_position.then_some(\"true\")",
        "data-custom-portal=root_state.has_custom_portal.then_some(\"true\")",
        "data-custom-max-toasts=root_state.has_custom_max_toasts.then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-class=root_state.has_custom_class_name.then_some(\"true\")",
        "data-custom-aria=root_state.has_custom_aria_label.then_some(\"true\")",
        "data-viewport-slot=viewport_state.slot_attr",
        "data-viewport-state=viewport_state.state_attr",
        "data-viewport-position=viewport_state.position_attr",
        "data-viewport-portal=viewport_state.portal_attr",
        "data-viewport-queue=viewport_state.queue_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "lang=region_a11y.lang",
        "dir=region_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Sonner view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn sonner_state_markers_are_closed_sets_and_selector_friendly() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/sonner.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");

    for needle in [
        "fn source_attr(is_custom: bool) -> &'static str",
        "if is_custom { \"custom\" } else { \"default\" }",
        "pub fn state_attr(portal: bool) -> &'static str",
        "if portal { \"portal\" } else { \"inline\" }",
        "pub fn queue_attr(max_toasts: usize) -> &'static str",
        "\"single\"",
        "\"bounded\"",
        "\"extended\"",
        "portal_attr: if input.portal { \"true\" } else { \"false\" },",
        "SonnerStoreSource::Provided => \"provided\"",
        "SonnerStoreSource::Context => \"context\"",
        "SonnerStoreSource::Local => \"local\"",
        "SonnerPosition::TopLeft => \"top-left\"",
        "SonnerPosition::TopCenter => \"top-center\"",
        "SonnerPosition::TopRight => \"top-right\"",
        "SonnerPosition::BottomLeft => \"bottom-left\"",
        "SonnerPosition::BottomCenter => \"bottom-center\"",
        "SonnerPosition::BottomRight => \"bottom-right\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Sonner primitive markers should remain closed/enumerable via `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-portal=root_state.portal_attr",
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-max-toasts-source=root_state.max_toasts_source_attr",
        "data-aria-source=root_state.aria_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner view should expose semantic markers from typed state via `{needle}`."
        );
    }

    for forbidden in [
        "data-state=\"",
        "data-queue=\"",
        "data-position=\"",
        "data-store-source=\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should avoid hardcoded free-text marker values: `{forbidden}`."
        );
    }

    for needle in [
        "[data-slot=\"sonner\"][data-state=\"inline\"][data-position=\"top-left\"][data-store-source=\"provided\"][data-motion-source=\"custom\"]",
        "[data-slot=\"toast-viewport\"][data-state=\"portal\"][data-store-source=\"provided\"]",
        "[data-slot=\"sonner-source-controls\"]",
        "[data-slot=\"sonner-source-push\"] [data-slot=\"button\"]",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner e2e should prioritize semantic selectors via `{needle}`."
        );
    }

    for forbidden in ["nth-child", "text=", "getByText(", ".ui-sonner >"] {
        assert!(
            !e2e_source.contains(forbidden),
            "Sonner e2e should avoid brittle DOM/text-based selectors: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_mounts_headless_region_a11y_contract_in_view() {
    let source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, region_attrs};",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "lang=region_a11y.lang",
        "dir=region_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should mount headless region A11y contract via `{needle}`."
        );
    }

    for forbidden in ["role=\"region\"", "aria-label=aria_label.get_value()"] {
        assert!(
            !source.contains(forbidden),
            "Sonner should not inline A11y mapping when headless contract exists: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_a11y_i18n_contract_uses_headless_and_no_view_text_hardcode() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "use ui_headless::{A11yDirection, region_attrs};",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "lang=region_a11y.lang",
        "dir=region_a11y.dir",
        "pub struct RegionA11yAttrs",
        "pub fn region_attrs(",
        "DEFAULT_ARIA_LABEL",
        "aria_label=\"Status updates\".to_string()",
    ] {
        assert!(
            view_source.contains(needle)
                || headless_a11y.contains(needle)
                || logic_source.contains(needle)
                || docs_source.contains(needle),
            "Sonner a11y/i18n-l10n contract should include `{needle}`."
        );
    }

    for forbidden in [
        "role=\"region\"",
        "aria-label=\"Sonner notifications\"",
        "let aria_label = \"Sonner notifications\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should not hardcode a11y text or mapping when headless contract exists: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_a11y_label_source_priority_and_locale_passthrough_are_stable() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/sonner.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "lang=region_a11y.lang",
        "dir=region_a11y.dir",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {",
        "sonner_state::normalize_aria_label(value)",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Notifications\";",
        "pub fn region_attrs(",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle)
                || headless_a11y.contains(needle),
            "Sonner A11y/i18n-l10n source chain should contain `{needle}`."
        );
    }

    for forbidden in [
        "aria-label=\"Notifications\"",
        "aria-label=\"Sonner notifications\"",
        "let aria_label = \"Notifications\"",
        "dir=Some(\"ltr\")",
        "fn region_attrs(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Sonner component layer should avoid hardcoded a11y locale/text mapping: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_view_tracks_store_source_resolution() {
    let source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "if let Some(provided_store) = store",
        "SonnerStoreSource::Provided",
        "crate::toast::use_toast_store()",
        "SonnerStoreSource::Context",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "SonnerStoreSource::Local",
        "max_toasts: normalized.max_toasts",
    ] {
        assert!(
            source.contains(needle),
            "Sonner view should include `{needle}` for stable store-source derivation."
        );
    }
}

#[test]
fn sonner_composes_toast_viewport_and_forwards_stateful_props() {
    let source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "<ToastViewport",
        "store=store",
        "class_name=viewport_class_name",
        "max_toasts=viewport_state.max_toasts",
        "portal=viewport_state.portal",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "Sonner should compose ToastViewport using `{needle}`."
        );
    }
}

#[test]
fn sonner_delegates_interaction_semantics_to_toast_headless_layer() {
    let sonner_view = load_source("../../components/toast/src/sonner/view.rs");
    let toast_view = load_source("../../components/toast/src/toast/view.rs");

    for needle in ["<ToastViewport", "store=store", "motion=motion"] {
        assert!(
            sonner_view.contains(needle),
            "Sonner should delegate interactive host behavior via `{needle}`.",
        );
    }

    for forbidden in [
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
        "on:click=",
    ] {
        assert!(
            !sonner_view.contains(forbidden),
            "Sonner root should avoid inlining reusable interaction contract `{forbidden}`.",
        );
    }

    for needle in [
        "use ui_headless::{",
        "live_region_attrs",
        "on:keydown=on_key_down",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
    ] {
        assert!(
            toast_view.contains(needle),
            "Toast viewport should own headless/a11y contract token `{needle}`.",
        );
    }
}

#[test]
fn sonner_logic_models_positions_queue_and_part_state() {
    let source = load_source("../../components/toast/src/sonner/logic.rs");

    for needle in [
        "pub use ui_state_primitives::sonner::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_PORTAL",
        "DEFAULT_MAX_TOASTS",
        "pub struct SonnerNormalizeInput",
        "pub struct SonnerNormalizedProps",
        "pub fn normalize_props(input: SonnerNormalizeInput) -> SonnerNormalizedProps",
        "pub enum SonnerAgentIntent",
        "pub enum SonnerAgentActionModel",
        "pub struct SonnerAgentContract",
        "pub fn agent_contract() -> SonnerAgentContract",
        "pub fn compose_class_name(base_class_name: Option<String>, state: SonnerPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Sonner logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn sonner_default_values_have_single_logic_source() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");

    assert!(
        view_source.contains("logic::normalize_props(logic::SonnerNormalizeInput {")
            && view_source.contains("let normalized = logic::normalize_props("),
        "Sonner view should consume one normalized props source from logic.rs."
    );

    for needle in [
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner view prop defaults should reference logic defaults via `{needle}`."
        );
    }

    for forbidden in [
        "normalize_aria_label(aria_label)",
        "normalize_optional_text(class_name)",
        "normalize_max_toasts(max_toasts)",
        "portal != logic::DEFAULT_PORTAL",
        "max_toasts != logic::DEFAULT_MAX_TOASTS",
        "unwrap_or(",
        "unwrap_or_else(",
        "if let None",
        "match None",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should not keep duplicated default/source normalization: `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_props(input: SonnerNormalizeInput) -> SonnerNormalizedProps",
        "has_custom_position",
        "has_custom_portal",
        "has_custom_max_toasts",
        "has_custom_aria_label",
        "has_custom_class_name",
        "has_custom_motion",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sonner logic should centralize default/source decision via `{needle}`."
        );
    }
}

#[test]
fn sonner_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");

    for needle in [
        "pub struct SonnerNormalizeInput",
        "pub struct SonnerNormalizedProps",
        "pub fn normalize_props(input: SonnerNormalizeInput) -> SonnerNormalizedProps",
        "pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sonner normalization should live in logic via `{needle}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "position: normalized.position",
        "portal: normalized.portal",
        "max_toasts: normalized.max_toasts",
        "has_custom_position: normalized.has_custom_position",
        "has_custom_portal: normalized.has_custom_portal",
        "has_custom_max_toasts: normalized.has_custom_max_toasts",
        "has_custom_aria_label: normalized.has_custom_aria_label",
        "has_custom_motion: normalized.has_custom_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner view should only consume logic-normalized state inputs via `{needle}`."
        );
    }

    for forbidden in [
        "if portal { \"portal\" } else { \"inline\" }",
        "if max_toasts <= 1",
        "if max_toasts <= 3",
        "state_attr(",
        "queue_attr(",
        "normalize_max_toasts(",
        "normalize_aria_label(",
        "normalize_optional_text(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should avoid rebuilding state machine fragments: `{forbidden}`."
        );
    }

    for selector in [
        ".ui-sonner[data-state=\"inline\"]",
        ".ui-sonner[data-portal=\"false\"]",
        ".ui-sonner[data-queue=\"single\"]",
        ".ui-sonner[data-queue=\"bounded\"]",
        ".ui-sonner[data-queue=\"extended\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Sonner styles should only consume semantic state markers via `{selector}`."
        );
    }
}

#[test]
fn sonner_discrete_state_axes_are_enum_typed() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/sonner.rs");

    for needle in [
        "#[prop(optional)] position: SonnerPosition",
        "pub enum SonnerPosition",
        "pub enum SonnerSlot",
        "pub enum SonnerStoreSource",
    ] {
        assert!(
            view_source.contains(needle) || primitive_source.contains(needle),
            "Sonner discrete state axes should be enum-typed via `{needle}`."
        );
    }

    for forbidden in [
        "position: Option<String>",
        "position: String",
        "variant: String",
        "size: String",
        "mode: String",
        "status: String",
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "position: Option<bool>",
        "variant: Option<bool>",
        "size: Option<bool>",
        "mode: Option<bool>",
        "status: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Sonner should avoid string/Option<bool> discrete-state drift: `{forbidden}`."
        );
    }

    for needle in [
        "has_custom_position: bool",
        "has_custom_portal: bool",
        "has_custom_max_toasts: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Source-marker bool fields are allowed only as metadata, not as discrete state axes: `{needle}`."
        );
    }
}

#[test]
fn sonner_state_primitives_live_in_ui_state_primitives() {
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");

    for needle in [
        "use ui_state_primitives::sonner as sonner_state;",
        "sonner_state::normalize_optional_text(value)",
        "sonner_state::normalize_aria_label(value)",
        "sonner_state::normalize_max_toasts(max_toasts)",
        "sonner_state::resolve_state(input)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sonner should consume state primitives from ui-state-primitives via `{needle}`."
        );
    }
}

#[test]
fn sonner_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "use ui_state_primitives::sonner as sonner_state;",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String> {",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize {",
        "pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState {",
        "sonner_state::normalize_optional_text(value)",
        "sonner_state::normalize_aria_label(value)",
        "sonner_state::normalize_max_toasts(max_toasts)",
        "sonner_state::resolve_state(input)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sonner logic should delegate stable state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "ToastStore",
        "provide_toast_store(",
        "use_toast_store(",
        "Signal<",
        "RwSignal",
        "WriteSignal",
        "ReadSignal",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Sonner logic should stay primitive-only and avoid framework/store coupling: `{forbidden}`."
        );
    }

    for needle in [
        "if let Some(provided_store) = store",
        "crate::toast::use_toast_store()",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "SonnerStoreSource::Provided",
        "SonnerStoreSource::Context",
        "SonnerStoreSource::Local",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner view should keep store adapter boundary via `{needle}`."
        );
    }

    for forbidden in ["ui_state_primitives::sonner::", "sonner_state::"] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should not bypass logic and call primitives directly: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_has_no_async_interaction_protocol_surface() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "error",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Sonner should remain async-protocol free in host layer: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_async_na_reason_is_explicit_in_checklist() {
    let checklist_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "`Sonner` 无远程请求与异步状态，按 N/A 通过",
        "未引入 `is_loading/retry/aria-busy`",
        "回归：`sonner_has_no_async_interaction_protocol_surface`",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Sonner async checklist should keep explicit N/A reason and regression evidence via `{needle}`."
        );
    }
}

#[test]
fn sonner_engineering_capability_contract_keeps_serde_tracing_and_async_runtime_boundaries() {
    let checklist_source = load_source("../../components/toast/src/sonner/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let sonner_mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let sonner_logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let sonner_motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let sonner_view_source = load_source("../../components/toast/src/sonner/view.rs");
    let toast_view_source = load_source("../../components/toast/src/toast/view.rs");
    let headless_controllable_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "`Sonner` 无 spec/async runtime API 面，按 N/A 收口；trace 语义复用 toast/headless 统一链路。",
        "回归：`crates/ui-components/tests/sonner_semantics.rs::sonner_engineering_capability_contract_keeps_serde_tracing_and_async_runtime_boundaries`",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Sonner engineering checklist should keep explicit evidence marker `{needle}`."
        );
    }

    assert!(
        cargo_source.contains("component-sonner = [\"dep:ui-toast\"]"),
        "component-sonner feature should stay dependency-light for engineering boundary."
    );

    for forbidden in [
        "component-sonner = [\"dep:serde\"",
        "component-sonner = [\"dep:serde_json\"",
        "component-sonner = [\"dep:tracing\"",
        "component-sonner = [\"dep:tokio\"",
        "component-sonner = [\"dep:async-std\"",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "component-sonner feature should not pull spec/runtime/debug deps directly: `{forbidden}`."
        );
    }

    let sonner_combined = format!(
        "{sonner_mod_source}\n{sonner_logic_source}\n{sonner_motion_source}\n{sonner_view_source}"
    );

    for forbidden in [
        "serde::",
        "#[serde(",
        "Serialize",
        "Deserialize",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "JoinHandle",
        "async fn",
    ] {
        assert!(
            !sonner_combined.contains(forbidden),
            "Sonner component boundary should not leak spec/runtime implementation details: `{forbidden}`."
        );
    }

    for needle in [
        "<ToastViewport",
        "use_controllable_open_state_traced(\"toast\"",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            sonner_view_source.contains(needle)
                || toast_view_source.contains(needle)
                || headless_controllable_source.contains(needle)
                || headless_trace_source.contains(needle),
            "Sonner tracing path should stay aligned with shared headless trace contract via `{needle}`."
        );
    }
}

#[test]
fn sonner_api_dx_exposes_hello_world_without_manual_state_wiring() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");

    for needle in [
        "let hello_world_code = Signal::derive(move || r#\"<Sonner />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Sonner />",
        "Default path mounts a notification host with sensible defaults",
        "### Hello World（最小可用）",
    ] {
        assert!(
            docs_source.contains(needle) || readme_source.contains(needle),
            "Sonner DX baseline should include `{needle}`."
        );
    }
}

#[test]
fn sonner_dx_hello_world_is_short_and_requires_no_state_wiring() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    let marker = "let hello_world_code = Signal::derive(move || r#\"";
    let start = docs_source
        .find(marker)
        .unwrap_or_else(|| panic!("Sonner docs should define hello_world_code marker."));
    let snippet_start = start + marker.len();
    let snippet_tail = &docs_source[snippet_start..];
    let end = snippet_tail.find("\"#.to_string());").unwrap_or_else(|| {
        panic!("Sonner hello_world_code snippet should terminate as raw string.")
    });
    let hello_world_snippet = &snippet_tail[..end];

    let non_empty_lines = hello_world_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        non_empty_lines <= 5,
        "Sonner Hello World snippet should stay <= 5 lines, got {non_empty_lines}."
    );
    assert!(
        hello_world_snippet.contains("<Sonner />"),
        "Sonner Hello World snippet should keep default render path."
    );

    for forbidden in ["state=", "store=", "use_headless", "use_state_primitives"] {
        assert!(
            !hello_world_snippet.contains(forbidden),
            "Sonner Hello World should not require manual state-machine wiring: `{forbidden}`."
        );
    }

    for needle in [
        "### Hello World（最小可用）",
        "<Sonner />",
        "默认路径不需要用户手动接线 `ui-state-primitives` / `ui-headless`。",
    ] {
        assert!(
            readme_source.contains(needle),
            "Sonner README should keep beginner-first DX entry via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] store: Option<ToastStore>",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "Sonner surface should keep optional advanced wiring and explicit hello world entry via `{needle}`."
        );
    }

    let hello_index = docs_source.find("title=\"Hello World\"");
    let portal_index = docs_source.find("title=\"Portal Queue + Variants\"");
    let inline_index = docs_source.find("title=\"Inline Top-Center + Max Queue\"");
    assert!(
        hello_index.is_some()
            && portal_index.is_some()
            && inline_index.is_some()
            && hello_index < portal_index
            && portal_index < inline_index,
        "Sonner docs should keep default DX path first, then advanced examples."
    );
}

#[test]
fn sonner_non_composite_api_avoids_parallel_array_conventions() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in ["labels", "titles", "panels", "children_by_index"] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner should not expose parallel-array composition conventions: `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("<Sonner") && !docs_source.contains("labels + children"),
        "Sonner docs should keep explicit component API usage."
    );
}

#[test]
fn sonner_non_composite_api_stays_explicit_without_itemspec_sugar() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");
    let checklist_source = load_source("../../components/toast/src/sonner/check2.md");

    for forbidden in [
        "items:",
        "labels:",
        "titles:",
        "panels:",
        "children_by_index",
        "ItemSpec",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !docs_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "Sonner should not drift into parallel-array/composite sugar contracts: `{forbidden}`."
        );
    }

    let sonner_docs_usage_count = docs_source.matches("<Sonner").count();
    assert!(
        sonner_docs_usage_count >= 4,
        "Sonner docs should keep explicit host composition examples, found {sonner_docs_usage_count} `<Sonner` usages."
    );
    assert!(
        readme_source.contains("<Sonner />") && readme_source.contains("<Sonner\n"),
        "Sonner README should keep explicit default + advanced `<Sonner .../>` host API."
    );

    for needle in [
        "组合型组件主 API 必须“显示优于约定”",
        "`Sonner` 非并行数组型组合器，公开 API 为显式宿主装配 `<Sonner .../>`，按 N/A 通过。",
        "回归：`sonner_non_composite_api_avoids_parallel_array_conventions`",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Sonner checklist should keep explicit non-composite N/A rationale via `{needle}`."
        );
    }
}

#[test]
fn sonner_styles_include_state_and_source_marker_contracts() {
    let source = load_source("../../components/toast/src/sonner/styles.rs");

    for needle in [
        "var(--ui-overlay-viewport-inset",
        "var(--ui-space-md)",
        "var(--ui-overlay-panel-min-width",
    ] {
        assert!(
            source.contains(needle),
            "Sonner styles should consume theme tokens via `{needle}`.",
        );
    }

    assert!(
        !source.contains("420px"),
        "Sonner styles should not keep component-local pixel fallbacks once ui-theme token exists."
    );

    for selector in [
        ".ui-sonner[data-motion-source=\"custom\"]",
        ".ui-sonner[data-custom-motion=\"true\"]",
        ".ui-sonner--custom-motion",
        ".ui-sonner[data-position-source=\"custom\"]",
        ".ui-sonner[data-custom-position=\"true\"]",
        ".ui-sonner--custom-position",
        ".ui-sonner[data-portal-source=\"custom\"]",
        ".ui-sonner[data-custom-portal=\"true\"]",
        ".ui-sonner--custom-portal",
        ".ui-sonner[data-max-toasts-source=\"custom\"]",
        ".ui-sonner[data-custom-max-toasts=\"true\"]",
        ".ui-sonner--custom-max-toasts",
        ".ui-sonner[data-aria-source=\"custom\"]",
        ".ui-sonner[data-custom-aria=\"true\"]",
        ".ui-sonner--custom-aria",
        ".ui-sonner[data-class-source=\"custom\"]",
        ".ui-sonner[data-custom-class=\"true\"]",
        ".ui-sonner--custom-class",
        ".ui-sonner[data-store-source=\"provided\"]",
        ".ui-sonner[data-store-source=\"context\"]",
        ".ui-sonner[data-store-source=\"local\"]",
        ".ui-sonner[data-state=\"inline\"]",
        ".ui-sonner[data-queue=\"single\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner[data-queue=\"bounded\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner__viewport--inline.ui-toast-viewport",
        ".ui-sonner__viewport--top-center.ui-toast-viewport",
        ".ui-sonner__viewport--bottom-right.ui-toast-viewport",
    ] {
        assert!(
            source.contains(selector),
            "Sonner styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn sonner_styles_depend_on_semantic_state_not_dom_structure_guessing() {
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        ".ui-sonner[data-state=\"inline\"]",
        ".ui-sonner[data-portal=\"false\"]",
        ".ui-sonner[data-queue=\"single\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner[data-queue=\"bounded\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner[data-queue=\"extended\"] .ui-sonner__viewport.ui-toast-viewport",
        ".ui-sonner[data-position-source=\"custom\"]",
        ".ui-sonner[data-store-source=\"provided\"]",
        ".ui-sonner__viewport--inline.ui-toast-viewport",
    ] {
        assert!(
            styles_source.contains(needle),
            "Sonner styles should derive visual state from semantic markers/stable classes via `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":first-child",
        ":last-child",
        "ul li li",
        ".ui-sonner > :not(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Sonner styles should avoid brittle DOM-structure guessing selectors: `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "set_property("] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should avoid inline business-style logic and only mount semantic attrs: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_semantic_test_matrix_covers_contract_paths_not_snapshot_only() {
    let semantics_source = load_source("tests/sonner_semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "fn sonner_view_uses_logic_state_contracts()",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-store-source=root_state.store_source_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            semantics_source.contains(needle),
            "Sonner semantics suite should cover core role/aria/data-state/source contracts via `{needle}`."
        );
    }

    for needle in [
        "fn sonner_has_no_controllable_state_axis_and_no_half_controlled_api()",
        "fn sonner_state_markers_are_closed_sets_and_selector_friendly()",
        "fn sonner_headless_web_ssr_mutex_compile_error_is_present()",
        "fn sonner_motion_non_wasm_stub_exists()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "Sonner semantics matrix should cover key branch classes via `{needle}`."
        );
    }

    for needle in [
        "await pushSource.click();",
        "await pushSuccess.click();",
        "await page.keyboard.press(\"Enter\");",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner E2E matrix should include pointer/keyboard/wasm-stable paths via `{needle}`."
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(marker),
            "Sonner view should expose semantic contract marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Semantic-field change must be traceable in semantics tests; missing marker assertion `{marker}`."
        );
    }

    for forbidden in ["toMatchSnapshot(", "imageSnapshot", "screenshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Sonner contract tests should not rely on snapshot-only assertions: `{forbidden}`."
        );
    }

    for forbidden in [
        "sonner_visual_snapshot",
        "tests/sonner_snapshot",
        "toMatchSnapshot",
    ] {
        assert!(
            !check2_source.contains(forbidden),
            "Sonner checklist evidence should remain semantic-first, not snapshot-first: `{forbidden}`."
        );
    }

    for forbidden in ["is_disabled", "disabled=", "aria-disabled"] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner has no disabled axis and should not introduce hidden disabled-state surface: `{forbidden}`."
        );
    }

    for needle in [
        "测试验证“语义契约”而不只验证视觉快照",
        "sonner_view_uses_logic_state_contracts",
        "sonner_state_markers_are_closed_sets_and_selector_friendly",
        "sonner_semantic_test_matrix_covers_contract_paths_not_snapshot_only",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "回归：`sonner_view_uses_logic_state_contracts`、`sonner_state_markers_are_closed_sets_and_selector_friendly`、`sonner_semantic_test_matrix_covers_contract_paths_not_snapshot_only`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_semantic_test_matrix_covers_contract_paths_not_snapshot_only",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should document semantic-first verification evidence via `{needle}`."
        );
    }
}

#[test]
fn sonner_motion_contract_is_delegated_without_local_driver_reimplementation() {
    let motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion",
        "crate::toast::motion::sanitize_motion(motion)",
        "let motion = crate::sonner::motion::sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle) || view_source.contains(needle),
            "Sonner motion contract should delegate via `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new",
        "ui_motion::keyframes",
        "requestAnimationFrame",
    ] {
        assert!(
            !motion_source.contains(forbidden) && !view_source.contains(forbidden),
            "Sonner should not reimplement motion engine internals: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_component_files_respect_layered_responsibilities() {
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in ["mod logic;", "mod motion;", "pub mod styles;", "mod view;"] {
        assert!(
            mod_source.contains(needle),
            "Sonner module boundary should include `{needle}`."
        );
    }

    for required in [
        "../../components/toast/src/sonner/mod.rs",
        "../../components/toast/src/sonner/logic.rs",
        "../../components/toast/src/sonner/styles.rs",
        "../../components/toast/src/sonner/view.rs",
        "../../components/toast/src/sonner/motion.rs",
    ] {
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(required)
                .exists(),
            "Sonner component standard file should exist: `{required}`."
        );
    }

    for absent in [
        "../../components/toast/src/sonner/render.rs",
        "../../components/toast/src/sonner/spec.rs",
    ] {
        assert!(
            !Path::new(env!("CARGO_MANIFEST_DIR")).join(absent).exists(),
            "Sonner should avoid non-standard file drift: `{absent}`."
        );
    }

    assert!(
        logic_source.contains("pub fn normalize_props")
            && logic_source.contains("pub fn resolve_state")
            && !logic_source.contains("NodeRef")
            && !logic_source.contains("on:click"),
        "sonner/logic.rs should focus on normalization/derivation only."
    );

    assert!(
        view_source.contains("view! {")
            && view_source.contains("<ToastViewport")
            && !view_source.contains("SpringAnimator::new"),
        "sonner/view.rs should focus on render + contract mounting only."
    );

    assert!(
        styles_source.contains("pub const CSS: &str")
            && styles_source.contains("var(--ui-")
            && !styles_source.contains("ToastMotion")
            && !styles_source.contains("on:click"),
        "sonner/styles.rs should remain token-first static css only."
    );

    assert!(
        motion_source.contains("crate::toast::motion::sanitize_motion")
            && !motion_source.contains("pub fn attach_motion"),
        "sonner/motion.rs should only map semantic motion contract to shared driver sanitization."
    );

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "回归：`sonner_component_files_respect_layered_responsibilities`、`sonner_component_file_responsibilities_are_strictly_scoped`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_component_files_respect_layered_responsibilities",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_component_file_responsibilities_are_strictly_scoped",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit component-file evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_component_file_responsibilities_are_strictly_scoped() {
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let motion_source = load_source("../../components/toast/src/sonner/motion.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Sonner;",
    ] {
        assert!(
            mod_source.contains(needle),
            "sonner/mod.rs should keep minimal stable export boundary via `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "#[component]",
        "pub mod logic",
        "pub mod view",
        "impl ",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "sonner/mod.rs should not hold implementation details: `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SonnerNormalizeInput",
        "pub struct SonnerNormalizedProps",
        "pub fn normalize_props(input: SonnerNormalizeInput) -> SonnerNormalizedProps",
        "pub fn resolve_state(input: SonnerPartStateInput) -> SonnerPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: SonnerPartState)",
    ] {
        assert!(
            logic_source.contains(needle),
            "sonner/logic.rs should focus on normalize/derive/source markers via `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
        "NodeRef",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "sonner/logic.rs should avoid render/dom/event concerns: `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "var(--ui-",
        "[data-state=",
        "[data-queue=",
    ] {
        assert!(
            styles_source.contains(needle),
            "sonner/styles.rs should remain token-first static css using semantic selectors via `{needle}`."
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "on:click",
        "ToastStore",
        "provide_toast_store",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "sonner/styles.rs should not mix logic/render/store concerns: `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "<ToastViewport",
        "logic::resolve_state(SonnerPartStateInput {",
        "data-state=root_state.state_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "sonner/view.rs should do structure + headless contract mount via `{needle}`."
        );
    }
    for forbidden in [
        "normalize_max_toasts(",
        "normalize_aria_label(",
        "sonner_state::",
        "SpringAnimator::new",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "sonner/view.rs should not hide primitive/motion-engine decisions: `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion",
        "crate::toast::motion::sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(needle),
            "sonner/motion.rs should only map semantic motion contract via `{needle}`."
        );
    }
    for forbidden in [
        "pub fn attach_motion",
        "SpringAnimator::new",
        "requestAnimationFrame",
        "ui_headless",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "sonner/motion.rs should not reimplement engine/headless concerns: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_ui_components_layer_assembles_four_layers_without_public_dom_leakage() {
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "pub use view::Sonner;",
        "pub use ui_state_primitives::sonner::{",
        "use ui_state_primitives::sonner as sonner_state;",
        "use ui_headless::{A11yDirection, region_attrs};",
        "<ToastViewport",
        "crate::toast::motion::sanitize_motion(motion)",
        "pub const CSS: &str",
        "var(--ui-",
    ] {
        assert!(
            mod_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle)
                || styles_source.contains(needle)
                || motion_source.contains(needle),
            "Sonner should compose layered contracts via `{needle}`."
        );
    }

    assert!(
        lib_source.contains("pub use sonner::{Sonner, SonnerPosition};"),
        "crate root should expose stable Sonner API surface."
    );

    for forbidden in ["pub use web_sys", "web_sys::", "web-sys", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Sonner public API should not leak DOM/web-sys details: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_view_macro_complexity_stays_bounded_and_semantically_flat() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Sonner should keep one bounded `view!` block."
    );
    assert_eq!(
        view_source.matches("<section").count(),
        1,
        "Sonner should keep a single host section in view macro."
    );
    assert_eq!(
        view_source.matches("<ToastViewport").count(),
        1,
        "Sonner should keep one semantic child viewport mount."
    );

    let line_count = view_source.lines().count();
    assert!(
        line_count <= 140,
        "sonner/view.rs should stay compact; split semantic subrenders if this grows, got {line_count} lines."
    );

    let max_indent = view_source
        .lines()
        .map(|line| line.chars().take_while(|c| *c == ' ').count())
        .max()
        .unwrap_or(0);
    assert!(
        max_indent <= 20,
        "Sonner view macro nesting/indent should stay bounded (<=20 spaces), got {max_indent}."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "view! { view! {",
        "view! {\n        view! {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view macro should avoid loop-heavy or nested macro bloat pattern `{forbidden}`."
        );
    }
}

#[test]
fn sonner_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Sonner should keep a single public component boundary for current host layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn sonner_",
        "pub fn render_",
        "pub fn section_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner should avoid extra local component abstraction noise `{forbidden}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "let root_state = logic::resolve_state(SonnerPartStateInput {",
        "let viewport_state = logic::resolve_state(SonnerPartStateInput {",
        "data-state=root_state.state_attr",
        "data-store-source=root_state.store_source_attr",
        "<ToastViewport",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner functional assembly should keep stable semantic markers after split choices via `{needle}`."
        );
    }
}

#[test]
fn sonner_does_not_define_spec_module_for_simple_host_component() {
    let mod_source = load_source("../../components/toast/src/sonner/mod.rs");

    assert!(
        !mod_source.contains("mod spec;")
            && !Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../components/toast/src/sonner/spec.rs")
                .exists(),
        "Sonner should not define spec.rs for simple host contract."
    );
}

#[test]
fn sonner_spec_module_policy_stays_na_with_docs_in_check2_and_readme() {
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "`sonner` 未新增 `spec.rs`，按简单宿主组件策略保持 N/A。",
        "说明文档应留在 `check2.md`/组件文档。",
        "回归：`sonner_does_not_define_spec_module_for_simple_host_component`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit spec.rs N/A policy via `{needle}`."
        );
    }

    assert!(
        readme_source.contains("### Hello World（最小可用）")
            && docs_source.contains("pub(super) fn sonner() -> AnyView"),
        "Sonner documentation should remain in README/docs page instead of introducing spec.rs for a simple host component."
    );
}

#[test]
fn sonner_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn sonner() -> AnyView",
        "title=\"Sonner\"",
        "slug=\"sonner\"",
        "description=\"baseline-style toast host that composes ToastViewport with position presets, queue limits, and stable Sonner slot/source-state data contracts.\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue + Variants\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
        "<Sonner",
    ] {
        assert!(
            source.contains(needle),
            "overlays_extra sonner docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn sonner_docs_app_interactive_playground_supports_props_state_preview_and_repeatable_flow() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue + Variants\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
        "data-slot=\"sonner-portal-controls\"",
        "data-slot=\"sonner-inline-controls\"",
        "data-slot=\"sonner-source-controls\"",
        "on_press=push_saved",
        "on_press=push_inline",
        "on_press=push_source",
        "on_press=clear_inline",
        "on_press=clear_source",
    ] {
        assert!(
            docs_source.contains(needle),
            "Sonner docs playground should expose interactive props/state controls via `{needle}`."
        );
    }

    for needle in [
        "docs-app sonner uses semantic selectors with wasm-stable ready waits",
        "docs-app sonner covers async/motion ready-settled path with semantic markers",
        "docs-app sonner key flow is repeatable with semantic breakpoints",
        "await pushSource.click();",
        "await clearSource.click();",
        "await page.keyboard.press(\"Enter\");",
        "toHaveCount(0, {",
        "timeout: 6000",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner playground acceptance flow should be repeatable via `{needle}`."
        );
    }

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Sonner 非 AI Spec 组件，本项按 N/A 收口。",
        "回归：`sonner_docs_app_interactive_playground_supports_props_state_preview_and_repeatable_flow`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_docs_app_interactive_playground_supports_props_state_preview_and_repeatable_flow`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit interactive-playground evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_source_first_docs_are_copy_paste_ready_and_synced() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "None => DEFAULT_PLAYGROUND_IMPORTS.to_string(),",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should include `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn sonner() -> AnyView",
        "let hello_world_code = Signal::derive(move || r#\"<Sonner />\"#.to_string());",
        "let basic_code = Signal::derive(move || {",
        "let state_code = Signal::derive(move || {",
        "let source_code = Signal::derive(move || {",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue + Variants\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Sonner docs page should keep copy-paste-ready snippet source via `{needle}`."
        );
    }

    for needle in [
        "## Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "components/toast/src/sonner/mod.rs",
        "components/toast/src/sonner/logic.rs",
        "components/toast/src/sonner/view.rs",
        "components/toast/src/sonner/styles.rs",
        "components/toast/src/sonner/motion.rs",
        "`component-sonner` 与 `component-toast`",
    ] {
        assert!(
            readme_source.contains(needle),
            "Sonner README should keep source-first copy/paste prerequisites via `{needle}`."
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "回归：`sonner_source_first_docs_are_copy_paste_ready_and_synced`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_source_first_docs_are_copy_paste_ready_and_synced`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit source-first evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_heroui_benchmark_docs_and_component_docs_stay_synced() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let readme_source = load_source("../../components/toast/src/sonner/README.md");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "### Sonner 同步记录（2026-02-18）",
        "`Sonner` 继续保持宿主定位",
        "component_doc!(\"Sonner\", \"sonner\", \"Overlays\", overlays_extra::sonner)",
        "`#/components/sonner`",
        "`Hello World / Portal Queue + Variants / Inline Top-Center + Max Queue / State + Source Markers`",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "参数语义若变更，必须先同步本策略文档与组件文档入口",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI benchmark doc should keep Sonner sync record marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Sonner\", \"sonner\", \"Overlays\", overlays_extra::sonner)",
        "pub(super) fn sonner() -> AnyView",
        "slug=\"sonner\"",
        "<section class=\"docs-card docs-prose\" data-slot=\"sonner-api-matrix\">",
        "<section class=\"docs-card docs-prose\" data-slot=\"sonner-state-matrix\">",
    ] {
        assert!(
            docs_registry_source.contains(needle) || docs_page_source.contains(needle),
            "Sonner docs entry should remain indexable and synced via `{needle}`."
        );
    }

    for needle in [
        "## docs-app 等价入口",
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs",
        "## Source-first / Copy-Paste Ready",
    ] {
        assert!(
            readme_source.contains(needle),
            "Sonner README should stay accessible and aligned with docs entry via `{needle}`."
        );
    }

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "回归：`sonner_heroui_benchmark_docs_and_component_docs_stay_synced`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_heroui_benchmark_docs_and_component_docs_stay_synced`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit HeroUI-doc-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_forbidden_antipatterns_are_blocked_by_layer_and_contract_guards() {
    let primitive_source = load_source("../ui-state-primitives/src/sonner.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let headless_controllable_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let sonner_logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let sonner_view_source = load_source("../../components/toast/src/sonner/view.rs");
    let sonner_mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let toaster_view_source = load_source("../../components/toaster/src/view.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for forbidden in [
        "web_sys::",
        "leptos::",
        "view! {",
        "class=",
        "style=",
        ".ui-",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives/sonner should stay pure POJO without DOM/style token `{forbidden}`."
        );
    }

    for forbidden in [
        "class=",
        ".ui-",
        "@keyframes",
        "SpringAnimator::new",
        "transition:",
    ] {
        assert!(
            !headless_a11y_source.contains(forbidden)
                && !headless_controllable_source.contains(forbidden),
            "ui-headless contracts should not contain visual/animation authoring token `{forbidden}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "let root_state = logic::resolve_state(SonnerPartStateInput {",
        "let viewport_state = logic::resolve_state(SonnerPartStateInput {",
    ] {
        assert!(
            sonner_view_source.contains(needle),
            "sonner/view.rs should consume normalized + derived state from logic via `{needle}`."
        );
    }

    for forbidden in [
        "labels + children",
        "titles + panels",
        "Vec<Option<bool>>",
        "on_open_change",
    ] {
        assert!(
            !sonner_view_source.contains(forbidden),
            "sonner/view.rs should avoid anti-pattern token `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] position: SonnerPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] motion: ToastMotion",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
    ] {
        assert!(
            sonner_view_source.contains(needle),
            "new Sonner params/markers should be typed and contractized via `{needle}`."
        );
    }

    assert!(
        sonner_logic_source.contains("use ui_state_primitives::sonner as sonner_state;")
            && sonner_logic_source.contains("pub fn normalize_props(input: SonnerNormalizeInput)")
            && sonner_logic_source.contains("pub fn resolve_state(input: SonnerPartStateInput)"),
        "reusable state invariants must stay in ui-state-primitives; logic only maps/consumes."
    );

    assert!(
        !sonner_mod_source.contains("web_sys::")
            && !sonner_mod_source.contains("HtmlElement")
            && !sonner_mod_source.contains("pub mod logic")
            && !sonner_mod_source.contains("pub mod view"),
        "public API should not leak platform internals or private implementation modules."
    );

    for needle in [
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            sonner_view_source.contains(needle) && toaster_view_source.contains(needle),
            "overlay family naming consistency should block temporary divergence via `{needle}`."
        );
    }

    for needle in [
        "### 8. 明确禁止的反模式",
        "统一回归：`sonner_forbidden_antipatterns_are_blocked_by_layer_and_contract_guards`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_forbidden_antipatterns_are_blocked_by_layer_and_contract_guards`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit anti-pattern guard evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_visual_desire_default_theme_baseline_is_wired_for_docs_and_e2e_regression() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "theme_visual_baseline::theme_visual_baseline",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Overlay",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "Theme visual baseline docs gate should include `{needle}`."
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
            "Theme visual baseline E2E regression gate should include `{needle}`."
        );
    }

    for needle in [
        "默认主题美学质量达标（Visual Desire）",
        "HeroUI",
        "docs-app 必须提供默认主题基线页面与截图基线",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit visual-desire gate wording via `{needle}`."
        );
    }
}

#[test]
fn sonner_docs_examples_and_matrices_stay_synced_with_logic_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "pub(super) fn sonner() -> AnyView",
        "data-slot=\"sonner-api-matrix\"",
        "data-slot=\"sonner-state-matrix\"",
        "<h3>\"API Matrix\"</h3>",
        "<h3>\"State Matrix\"</h3>",
        "ui_components::sonner::DEFAULT_PORTAL",
        "ui_components::sonner::DEFAULT_MAX_TOASTS",
        "ui_components::sonner::DEFAULT_ARIA_LABEL",
        "control mode",
        "N/A (Sonner is host config, no controlled/uncontrolled runtime axis)",
    ] {
        assert!(
            docs_source.contains(needle),
            "Sonner docs should include synced matrix contract `{needle}`."
        );
    }

    assert!(
        logic_source.contains("DEFAULT_PORTAL")
            && logic_source.contains("DEFAULT_MAX_TOASTS")
            && logic_source.contains("DEFAULT_ARIA_LABEL"),
        "Sonner logic should own defaults consumed by docs matrix."
    );

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "回归：`sonner_docs_examples_and_matrices_stay_synced_with_logic_defaults`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_docs_examples_and_matrices_stay_synced_with_logic_defaults`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit docs/matrix sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_documentation_is_beginner_friendly_with_readme_or_equivalent_entry() {
    let readme = load_source("../../components/toast/src/sonner/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "<Sonner />",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "Playground 顺序保持",
    ] {
        assert!(
            readme.contains(needle),
            "Sonner README should remain beginner-friendly and staged via `{needle}`."
        );
    }

    assert!(
        docs_source.contains("pub(super) fn sonner() -> AnyView")
            && docs_source
                .contains("<Playground title=\"Hello World\" code_signal=hello_world_code>"),
        "Sonner docs-app entry should remain indexable and beginner-facing."
    );

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "回归：`sonner_documentation_is_beginner_friendly_with_readme_or_equivalent_entry`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_documentation_is_beginner_friendly_with_readme_or_equivalent_entry`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit beginner-doc evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "portal=false",
        "position=SonnerPosition::TopLeft",
        "max_toasts=4",
        "aria_label=\"Status updates\".to_string()",
        "class_name=\"docs-sonner-source\".to_string()",
        "let custom_motion = ToastMotion {",
        "initial_y_px: 22.0",
        "initial_scale: 0.94",
        "..ToastMotion::default()",
        "motion=custom_motion",
        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "sonner docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn sonner_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::sonner::styles::CSS);"),
        "ui-components css aggregator should include sonner styles."
    );
}

#[test]
fn sonner_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "pub const CSS: &str",
        ".ui-sonner",
        "var(--ui-",
        "--ui-sonner-offset",
        "--ui-sonner-max-inline-width",
    ] {
        assert!(
            styles_source.contains(needle),
            "Sonner styles should remain token-first static css via `{needle}`."
        );
    }

    for forbidden in [
        "@apply",
        "tailwind",
        "tw-",
        "css!(",
        "style!(",
        "StyleSheet",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Sonner styles contract should avoid utility/CSS-in-Rust default drift: `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-sonner\")]",
        "out.push_str(crate::sonner::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css aggregation should include sonner via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized component-css injection entry via `{needle}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should avoid runtime inline business styling and consume static css contract: `{forbidden}`."
        );
    }
}

#[test]
fn sonner_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "#[cfg(feature = \"component-sonner\")]",
        "pub use ui_toast::sonner;",
        "pub use sonner::{Sonner, SonnerPosition};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep sonner feature-gated export `{needle}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-sonner\")]")
            && css_source.contains("out.push_str(crate::sonner::styles::CSS);"),
        "ui-components css entry should keep sonner feature-gated aggregation."
    );

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry contract should remain centralized via `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared capability should keep stable generic contract `{needle}`."
        );
    }

    for forbidden in ["Sonner", "Toast", "Button", "role=", "aria-", "data-slot="] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should avoid component business/a11y contract pollution: `{forbidden}`."
        );
    }

    for absent in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !Path::new(env!("CARGO_MANIFEST_DIR")).join(absent).exists(),
            "forbidden ui-components entry file should stay absent: `{absent}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(required)
                .exists(),
            "ui-headless canonical primitive file should exist for entry boundary mapping: `{required}`."
        );
    }

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "回归：`sonner_ui_components_fixed_entry_files_follow_layered_boundaries`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit entry-file evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "pub enum SonnerAgentIntent",
        "pub enum SonnerAgentActionModel",
        "pub enum SonnerAgentStreamSupport",
        "pub enum SonnerAgentStreamFallback",
        "pub enum SonnerAgentOutputStatus",
        "pub struct SonnerAgentContract",
        "pub fn agent_contract() -> SonnerAgentContract",
        "schema_attr: \"ui.sonner.v1\"",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "let agent_contract = logic::agent_contract();",
        "stream_support_attr: SonnerAgentStreamSupport::Optional.as_attr()",
        "stream_fallback_attr: SonnerAgentStreamFallback::Snapshot.as_attr()",
        "output_status_attr: SonnerAgentOutputStatus::Verified.as_attr()",
        "state_axis_attr: \"state|queue|position|portal|max-toasts\"",
        "source_axis_attr: \"position|portal|max-toasts|aria|class|motion|store\"",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Sonner agent contract should be typed and traceable via `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action-model=format!(",
        "data-ui-state-axis=format!(",
        "data-ui-source-axis=format!(",
        "inner_html=",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Sonner render chain should remain in whitelist-safe path: `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "回归：`sonner_agent_contract_schema_is_typed_traceable_and_whitelisted`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_agent_contract_schema_is_typed_traceable_and_whitelisted",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit agent-contract evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "pub enum SonnerAgentStreamSupport {",
        "Optional,",
        "pub enum SonnerAgentStreamFallback {",
        "Snapshot,",
        "stream_support_attr: SonnerAgentStreamSupport::Optional.as_attr()",
        "stream_fallback_attr: SonnerAgentStreamFallback::Snapshot.as_attr()",
        "output_status_attr: SonnerAgentOutputStatus::Verified.as_attr()",
        "Self::Draft => \"draft\"",
        "Self::Verified => \"verified\"",
        "Self::Submittable => \"submittable\"",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "data-state=root_state.state_attr",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Sonner streaming policy should remain explicit via `{needle}`."
        );
    }

    for forbidden in [
        "Required,",
        "SonnerAgentStreamSupport::Required",
        "SonnerAgentStreamFallback::Streaming",
        "AiRenderMode::Streaming",
        "token_delta",
        "partial_chunk",
        "on_retry",
        "retry",
        "reconnect",
        "backoff",
        "validate_chunk",
        "stream_validator",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Sonner should keep streaming model constrained to optional/snapshot-only host contract: `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "回归：`sonner_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit streaming-mode evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_snapshot_baseline_consumes_complete_configuration_and_renders_stably() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "#[prop(optional)] position: SonnerPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ToastMotion",
        "#[prop(optional)] store: Option<ToastStore>",
        "let normalized = logic::normalize_props(logic::SonnerNormalizeInput {",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner snapshot baseline should include `{needle}`."
        );
    }

    let sonner_docs_start = docs_source
        .find("pub(super) fn sonner() -> AnyView")
        .expect("sonner docs section should exist");
    let sonner_docs_end = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist after sonner");
    let sonner_docs = &docs_source[sonner_docs_start..sonner_docs_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Sonner />",
        "<Sonner store=portal_store.get_value() />",
        "store=inline_store.get_value()",
        "portal=false",
        "position=SonnerPosition::TopCenter",
        "max_toasts=2",
        "aria_label=\"Status updates\".to_string()",
        "class_name=\"docs-sonner-source\".to_string()",
        "motion=custom_motion",
    ] {
        assert!(
            sonner_docs.contains(needle),
            "Sonner docs should keep full snapshot configuration path marker `{needle}`."
        );
    }

    for forbidden in ["token_delta", "partial_chunk", "stream_chunk"] {
        assert!(
            !view_source.contains(forbidden) && !sonner_docs.contains(forbidden),
            "Sonner snapshot baseline should avoid partial-output rendering protocol marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "回归：`sonner_snapshot_baseline_consumes_complete_configuration_and_renders_stably`",
        "cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_snapshot_baseline_consumes_complete_configuration_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit snapshot-baseline evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"sonner\"][data-slot=\"sonner\"]",
        "[data-slot=\"sonner-source-controls\"]",
        "[data-slot=\"sonner-source-push\"] [data-slot=\"button\"]",
        "[data-slot=\"toast-viewport\"]",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveCount(0, {",
        "timeout: 6000",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner e2e should keep semantic selectors and wasm-ready waits via `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout",
        "setTimeout",
        "sleep(",
        "getByText(",
        "text=",
        "nth-child(",
        "xpath=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Sonner e2e should avoid brittle selector/wait patterns: `{forbidden}`."
        );
    }

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "回归：`sonner_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit E2E stability evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_e2e_repeatable_key_flow_covers_overlay_focus_keyboard_and_async_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "docs-app sonner key flow is repeatable with semantic breakpoints",
        "[data-slot=\"sonner-portal-controls\"]",
        "[data-slot=\"sonner-portal-push-success\"] [data-slot=\"button\"]",
        "[data-slot=\"toast-viewport\"][data-state=\"portal\"][data-store-source=\"provided\"]",
        "await closeButton.focus();",
        "await expect(closeButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "[data-slot=\"toast\"][data-open=\"true\"]",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveCount(0, {",
        "timeout: 6000",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner e2e repeatable key-flow coverage should include `{needle}`."
        );
    }

    for forbidden in [
        "toMatchSnapshot(",
        "screenshot(",
        "waitForTimeout(",
        "sleep(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Repeatable key-flow regression should stay semantic-breakpoint based, not visual/timer based: `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "回归：`sonner_e2e_repeatable_key_flow_covers_overlay_focus_keyboard_and_async_paths`",
        "实测命令：`cargo test -p ui-components --no-default-features --features component-sonner,component-toast --test sonner_semantics sonner_e2e_repeatable_key_flow_covers_overlay_focus_keyboard_and_async_paths`",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit repeatable-key-flow evidence marker `{needle}`."
        );
    }
}

#[test]
fn sonner_static_fragments_are_constantized_or_absent_for_simple_host_layout() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/sonner.rs");

    for forbidden in [
        "inner_html=",
        "<svg",
        "<path",
        "<footer",
        "<article",
        "let markdown",
        "let long_text",
        "Notifications",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sonner view should avoid inlined heavy static fragments and keep host layout lean: `{forbidden}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Notifications\";",
        "pub use ui_state_primitives::sonner::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "aria-label=region_a11y.aria_label",
        "role=region_a11y.role",
    ] {
        assert!(
            primitives_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Sonner static/a11y fragment path should stay centralized and traceable via `{needle}`."
        );
    }
}

#[test]
fn sonner_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked() {
    let view_source = load_source("../../components/toast/src/sonner/view.rs");
    let logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let styles_source = load_source("../../components/toast/src/sonner/styles.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    let docs_start = docs_source
        .find("pub(super) fn sonner() -> AnyView")
        .expect("sonner docs section should exist");
    let docs_end = docs_source
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after sonner");
    let sonner_docs = &docs_source[docs_start..docs_end];

    for forbidden in [
        "inner_html=",
        "set_inner_html",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Sonner component implementation should forbid HTML injection path `{forbidden}`."
        );
        assert!(
            !sonner_docs.contains(forbidden),
            "Sonner contract should keep zero inner_html injection surface: `{forbidden}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Sonner should keep semantic/a11y mounting without inner_html fallback via `{needle}`."
        );
    }
}

#[test]
fn sonner_wasm_debug_capability_reuses_global_trace_overlay_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let sonner_mod_source = load_source("../../components/toast/src/sonner/mod.rs");
    let sonner_logic_source = load_source("../../components/toast/src/sonner/logic.rs");
    let sonner_motion_source = load_source("../../components/toast/src/sonner/motion.rs");
    let sonner_view_source = load_source("../../components/toast/src/sonner/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_sonner_contract.spec.mjs");
    let wasm_debug_script = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components should keep wasm debug capability isolated via `{needle}`."
        );
    }

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
        !cargo_source.contains("sonner-wasm-debug"),
        "Sonner should not expose a dedicated wasm-debug feature and should reuse global trace/debug overlay."
    );

    let sonner_combined = format!(
        "{sonner_mod_source}\n{sonner_logic_source}\n{sonner_motion_source}\n{sonner_view_source}"
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
            !sonner_combined.contains(forbidden),
            "Sonner production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for marker in [
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-max-toasts-source=root_state.max_toasts_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            sonner_view_source.contains(marker),
            "Sonner should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
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
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "ui_headless::UiTraceEventKind::Inspect { tag, data_slot }",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
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
            "headless trace contract should keep timestamped, replayable event model via `{needle}`."
        );
    }

    for needle in [
        "docs-app sonner key flow is repeatable with semantic breakpoints",
        "await closeButton.focus();",
        "await page.keyboard.press(\"Enter\");",
        "toHaveCount(0, {",
        "timeout: 6000",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Sonner interaction path should remain replayable through deterministic e2e chain `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting",
    ] {
        assert!(
            wasm_debug_script.contains(needle),
            "wasm-debug check script should keep feature-isolated verification marker `{needle}`."
        );
    }

    assert!(
        !wasm_debug_script.contains("sonner-wasm-debug"),
        "wasm-debug gate should not add Sonner-specific debug feature path."
    );
}

#[test]
fn sonner_tree_shaking_feature_gates_exist() {
    let cargo_toml = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-sonner = [\"dep:ui-toast\"]",
        "component-toast = [\"component-close_button\", \"dep:ui-toast\"]",
        "component-toaster = [\"dep:ui-toast\"]",
        "#[cfg(feature = \"component-sonner\")]",
        "pub use ui_toast::sonner;",
        "out.push_str(crate::sonner::styles::CSS);",
    ] {
        assert!(
            cargo_toml.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Sonner tree-shaking contract should contain `{needle}`."
        );
    }
}

#[test]
fn sonner_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-sonner = [\"dep:ui-toast\"]",
        "\"component-sonner\"",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-sonner\")]\npub use ui_toast::sonner;"),
        "lib.rs should feature-gate sonner module export for tree-shaking.",
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
        css_source.contains("#[cfg(feature = \"component-sonner\")]")
            && css_source.contains("out.push_str(crate::sonner::styles::CSS);"),
        "css.rs should gate sonner CSS aggregation behind component-sonner feature."
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
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn sonner_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
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
fn sonner_headless_web_ssr_mutex_compile_error_is_present() {
    let source = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            source.contains(needle),
            "ui-headless web/ssr mutex guard should remain intact via `{needle}`."
        );
    }
}

#[test]
fn sonner_motion_non_wasm_stub_exists() {
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            toast_motion.contains(needle),
            "toast motion non-wasm path should keep predictable stub contract via `{needle}`."
        );
    }
}

#[test]
fn sonner_component_paths_cover_reduced_motion_ssr_and_wasm_without_semantic_split() {
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");
    let sonner_view = load_source("../../components/toast/src/sonner/view.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            toast_motion.contains(needle),
            "toast motion should keep reduced-motion + wasm/non-wasm branch contracts via `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            sonner_view.contains(needle),
            "sonner view should keep stable hydration-facing semantic markers via `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !sonner_view.contains(forbidden),
            "sonner view should avoid platform-split semantic rendering via `{forbidden}`."
        );
    }
}

#[test]
fn sonner_performance_governance_contract_is_budgeted_repeatable_attributable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");
    let sonner_view = load_source("../../components/toast/src/sonner/view.rs");
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "UiPerfBudget::mount_only(120.0)",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep repeatable perf budget/probe contract via `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Sonner\", \"sonner\", \"Overlays\", overlays_extra::sonner)",
        "\"sonner\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "Sonner docs should stay in component coverage traversal via `{needle}`.",
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose machine-readable perf regression markers via `{needle}`."
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
            "e2e coverage should keep repeatable perf threshold assertions via `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should keep blocking gate `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep explicit render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "render_count",
        "等价证据",
        "sonner_performance_governance_contract_is_budgeted_repeatable_attributable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep perf baseline and follow-up contract marker `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            sonner_view.contains(needle),
            "Sonner view should expose attribution markers for perf triage via `{needle}`."
        );
    }

    let view_effect_count = sonner_view.matches("Effect::new(").count();
    assert_eq!(
        view_effect_count, 0,
        "Sonner host view should avoid direct effect loops; found {view_effect_count}.",
    );

    let toast_motion_effect_count = toast_motion.matches("Effect::new(").count();
    assert!(
        toast_motion_effect_count <= 3,
        "Toast motion should keep bounded effect loops (<=3), found {toast_motion_effect_count}.",
    );

    let toast_motion_spring_count = toast_motion.matches("SpringAnimator::new").count();
    assert!(
        toast_motion_spring_count <= 3,
        "Toast motion should keep bounded spring count (<=3), found {toast_motion_spring_count}.",
    );
}

#[test]
fn sonner_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    let sonner_docs_start = docs_source
        .find("pub(super) fn sonner() -> AnyView")
        .expect("sonner docs section should exist");
    let sonner_docs_end = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist after sonner");
    let sonner_docs = &docs_source[sonner_docs_start..sonner_docs_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue + Variants\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            sonner_docs.contains(needle),
            "Sonner docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn sonner_dx_workbench_uses_interactive_playground_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2_source = load_source("../../components/toast/src/sonner/check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "<Card class_name=\"playground__panel playground__controls\".to_string()>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    let sonner_docs_start = docs_source
        .find("pub(super) fn sonner() -> AnyView")
        .expect("sonner docs section should exist");
    let sonner_docs_end = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist after sonner");
    let sonner_docs = &docs_source[sonner_docs_start..sonner_docs_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue + Variants\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center + Max Queue\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
        "data-slot=\"sonner-inline-controls\"",
        "data-slot=\"sonner-source-controls\"",
        "on_press=push_inline",
        "on_press=push_source",
    ] {
        assert!(
            sonner_docs.contains(needle),
            "Sonner docs should provide isolated interactive playground entry `{needle}`."
        );
    }

    for forbidden in [
        "SONNER_WORKBENCH_STORAGE_KEY",
        "load_sonner_workbench_state(",
        "save_sonner_workbench_state(",
        "clear_sonner_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !sonner_docs.contains(forbidden),
            "Sonner host docs should keep optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "sonner_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "sonner_dx_workbench_uses_interactive_playground_and_marks_persist_state_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sonner checklist should keep explicit DX evidence marker `{needle}`."
        );
    }
}
