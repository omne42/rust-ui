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
#[test]
fn toaster_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/toaster/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Toaster internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toaster_is_publicly_exported_from_module_and_crate_root() {
    let toaster_mod = load_source("src/toaster/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toaster_mod.contains("pub use view::Toaster;"),
        "toaster::mod should re-export Toaster."
    );
    assert!(
        toaster_mod.contains("DEFAULT_PORTAL") && toaster_mod.contains("DEFAULT_MAX_TOASTS"),
        "toaster::mod should expose default portal/max-toasts contracts."
    );
    assert!(
        crate_root.contains("pub use ui_toast::toaster;"),
        "crate root should expose Toaster contracts through ui_toast re-export."
    );
}

#[test]
fn toaster_api_naming_contract_matches_overlay_family_without_alias_drift() {
    let toaster_view = load_source("src/toaster/view.rs");
    let toast_view = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] motion: ToastMotion",
    ] {
        assert!(
            toaster_view.contains(needle),
            "Toaster API should keep stable naming contract via `{needle}`."
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
            !toaster_view.contains(forbidden),
            "Toaster should avoid naming alias drift in public API: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "#[prop(optional)] position: ToasterPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "if let Some(provided_store) = store",
        "ToasterStoreSource::Provided",
        "ToasterStoreSource::Context",
        "ToasterStoreSource::Local",
    ] {
        assert!(
            source.contains(needle),
            "Toaster should keep explicit non-controllable host contract via `{needle}`."
        );
    }

    for forbidden in [
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
            "Toaster should not expose half-controlled API surface: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_module_re_exports_slot_and_part_state_contracts_from_primitives() {
    let source = load_source("src/toaster/mod.rs");

    for needle in [
        "pub use ui_state_primitives::toaster::{",
        "ToasterPartState",
        "ToasterPartStateInput",
        "ToasterPosition",
        "ToasterSlot",
        "ToasterStoreSource",
    ] {
        assert!(
            source.contains(needle),
            "Toaster module should re-export `{needle}` from ui-state-primitives for stable slot/part-state contracts."
        );
    }
}

#[test]
fn toaster_view_uses_logic_state_contracts() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, region_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "logic::resolve_state(ToasterPartStateInput {",
        "slot: ToasterSlot::Root",
        "slot: ToasterSlot::Sonner",
        "logic::compose_class_name(class_name.get_value(), root_state)",
        "logic::compose_class_name(None, sonner_state)",
        "logic::map_to_sonner_position(root_state.position)",
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
        "data-sonner-slot=sonner_state.slot_attr",
        "data-sonner-state=sonner_state.state_attr",
        "data-sonner-position=sonner_state.position_attr",
        "data-sonner-portal=sonner_state.portal_attr",
        "data-sonner-queue=sonner_state.queue_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "lang=region_a11y.lang",
        "dir=region_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Toaster view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toaster_mounts_headless_region_a11y_contract_in_view() {
    let source = load_source("src/toaster/view.rs");

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
            "Toaster should mount headless region A11y contract via `{needle}`."
        );
    }

    for forbidden in ["role=\"region\"", "aria-label=aria_label.get_value()"] {
        assert!(
            !source.contains(forbidden),
            "Toaster should not inline A11y mapping when headless contract exists: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_a11y_i18n_contract_uses_headless_and_no_view_text_hardcode() {
    let view_source = load_source("src/toaster/view.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let logic_source = load_source("src/toaster/logic.rs");
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
        "pub use ui_state_primitives::toaster::{",
        "DEFAULT_ARIA_LABEL",
        "aria_label=\"Alert stream\".to_string()",
    ] {
        assert!(
            view_source.contains(needle)
                || headless_a11y.contains(needle)
                || logic_source.contains(needle)
                || docs_source.contains(needle),
            "Toaster a11y/i18n-l10n contract should include `{needle}`."
        );
    }

    for forbidden in [
        "role=\"region\"",
        "aria-label=\"Toaster notifications\"",
        "let aria_label = \"Toaster notifications\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should not hardcode a11y text or mapping when headless contract exists: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_view_tracks_store_source_resolution() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "if let Some(provided_store) = store",
        "ToasterStoreSource::Provided",
        "crate::toast::use_toast_store()",
        "ToasterStoreSource::Context",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "ToasterStoreSource::Local",
        "max_toasts: normalized.max_toasts",
    ] {
        assert!(
            source.contains(needle),
            "Toaster view should include `{needle}` for stable store-source derivation."
        );
    }
}

#[test]
fn toaster_default_values_have_single_logic_source() {
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");

    for needle in [
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "max_toasts: normalize_max_toasts(input.max_toasts)",
        "has_custom_portal: input.portal != DEFAULT_PORTAL",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS",
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Toaster default normalization should include `{needle}`."
        );
    }

    for forbidden in [
        "portal != logic::DEFAULT_PORTAL",
        "max_toasts != logic::DEFAULT_MAX_TOASTS",
        "logic::normalize_max_toasts(max_toasts)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should not contain secondary default fallback `{forbidden}`."
        );
    }
}

#[test]
fn toaster_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");
    let styles_source = load_source("src/toaster/styles.rs");

    for needle in [
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "let class_name = normalize_optional_text(input.class_name);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "has_custom_position: input.position != ToasterPosition::default()",
        "has_custom_portal: input.portal != DEFAULT_PORTAL",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS",
    ] {
        assert!(
            logic_source.contains(needle),
            "Toaster logic should centralize typed normalization via `{needle}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
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
            "Toaster view should consume normalized state contracts via `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(",
        "logic::normalize_aria_label(",
        "logic::normalize_max_toasts(",
        "portal != logic::DEFAULT_PORTAL",
        "max_toasts != logic::DEFAULT_MAX_TOASTS",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should not rebuild normalization rules outside logic: `{forbidden}`."
        );
    }

    for needle in [
        ".ui-toaster[data-position-source=\"custom\"]",
        ".ui-toaster[data-portal-source=\"custom\"]",
        ".ui-toaster[data-max-toasts-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Toaster styles should consume state markers instead of deriving state via `{needle}`."
        );
    }
}

#[test]
fn toaster_discrete_state_axes_are_enum_typed() {
    let primitives_source = load_source("../../crates/ui-state-primitives/src/toaster.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");

    for needle in [
        "pub enum ToasterPosition",
        "pub enum ToasterSlot",
        "pub enum ToasterStoreSource",
        "#[prop(optional)] position: ToasterPosition",
        "pub fn map_to_sonner_position(position: ToasterPosition) -> crate::sonner::SonnerPosition",
        "match position {",
        "ToasterPosition::TopLeft =>",
        "ToasterPosition::TopCenter =>",
        "ToasterPosition::TopRight =>",
        "ToasterPosition::BottomLeft =>",
        "ToasterPosition::BottomCenter =>",
        "ToasterPosition::BottomRight =>",
    ] {
        assert!(
            primitives_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Toaster discrete state axis contract should include `{needle}`."
        );
    }

    for forbidden in [
        "position: Option<String>",
        "position: String",
        "position.as_str()",
        "if position == \"top-left\"",
        "if position == \"top-right\"",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Toaster should avoid free-form string modeling for discrete position axis: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");

    for needle in [
        "use ui_state_primitives::toaster as toaster_state;",
        "toaster_state::normalize_optional_text(value)",
        "toaster_state::normalize_aria_label(value)",
        "toaster_state::normalize_max_toasts(max_toasts)",
        "toaster_state::resolve_state(input)",
        "let (store, store_source) = if let Some(provided_store) = store {",
        "ToasterStoreSource::Provided",
        "crate::toast::use_toast_store()",
        "ToasterStoreSource::Context",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
        "ToasterStoreSource::Local",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Toaster should keep primitive-source and store-adapter boundary via `{needle}`."
        );
    }

    for forbidden in [
        "use ui_state_primitives::toaster as",
        "toaster_state::resolve_state(",
        "toaster_state::normalize_max_toasts(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should not directly bind state primitives; found `{forbidden}`."
        );
    }
}

#[test]
fn toaster_has_no_async_interaction_protocol_surface() {
    let view_source = load_source("src/toaster/view.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let motion_source = load_source("src/toaster/motion.rs");

    for forbidden in [
        "is_loading",
        "on_retry",
        "retry:",
        "error:",
        "aria-busy",
        "data-loading",
        "data-error",
        "data-retry",
        "use_async_action(",
        "disabled=is_loading",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Toaster should not define a local async interaction protocol in component layer: `{forbidden}`."
        );
    }

    for needle in [
        "let (store, store_source) = if let Some(provided_store) = store {",
        "crate::toast::use_toast_store()",
        "crate::toast::provide_toast_store(ToastStoreOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster should remain a synchronous host adapter contract via `{needle}`."
        );
    }
}

#[test]
fn toaster_api_dx_exposes_hello_world_without_manual_state_wiring() {
    let view_source = load_source("src/toaster/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "#[prop(optional)] position: ToasterPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional)] motion: ToastMotion",
        "#[prop(optional)] store: Option<ToastStore>",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster public API should keep easy optional surface via `{needle}`."
        );
    }

    for needle in [
        "let hello_world_code = Signal::derive(move || r#\"<Toaster />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Toaster />",
    ] {
        assert!(
            docs_source.contains(needle),
            "Toaster docs should expose minimal hello-world path via `{needle}`."
        );
    }

    for forbidden in [
        "<Toaster state=",
        "<Toaster headless_state=",
        "<Toaster primitive=",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Toaster docs hello-world path should not require internal state wiring: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_non_composite_api_avoids_parallel_array_conventions() {
    let view_source = load_source("src/toaster/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "#[component]",
        "pub fn Toaster(",
        "<Toaster />",
        "<Toaster store=portal_store.get_value() />",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "Toaster should keep explicit host API usage via `{needle}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children:",
        "ItemSpec",
        "<ToasterItem",
        "items:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "Toaster should not expose parallel-array or item-spec composition API: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_composes_sonner_as_host_layer() {
    let source = load_source("src/toaster/view.rs");

    for needle in [
        "<Sonner",
        "store=store",
        "position=sonner_position",
        "class_name=sonner_class_name",
        "max_toasts=sonner_state.max_toasts",
        "portal=sonner_state.portal",
        "motion=motion",
    ] {
        assert!(
            source.contains(needle),
            "Toaster should compose Sonner via `{needle}`."
        );
    }
}

#[test]
fn toaster_logic_models_positions_queue_and_part_state() {
    let source = load_source("src/toaster/logic.rs");

    for needle in [
        "use ui_state_primitives::toaster as toaster_state;",
        "pub use ui_state_primitives::toaster::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_MAX_TOASTS",
        "DEFAULT_PORTAL",
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "has_custom_portal: input.portal != DEFAULT_PORTAL",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS",
        "toaster_state::normalize_optional_text(value)",
        "toaster_state::normalize_aria_label(value)",
        "toaster_state::normalize_max_toasts(max_toasts)",
        "toaster_state::state_attr(portal)",
        "toaster_state::queue_attr(max_toasts)",
        "toaster_state::resolve_state(input)",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn state_attr(portal: bool) -> &'static str",
        "pub fn queue_attr(max_toasts: usize) -> &'static str",
        "pub fn resolve_state(input: ToasterPartStateInput) -> ToasterPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ToasterPartState)",
        "pub fn map_to_sonner_position(position: ToasterPosition)",
    ] {
        assert!(
            source.contains(needle),
            "Toaster logic should include `{needle}` for centralized state/source contracts."
        );
    }

    for forbidden in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Toaster notifications\";",
        "pub const DEFAULT_PORTAL: bool = true;",
        "pub const DEFAULT_MAX_TOASTS: usize = 3;",
        "fn source_attr(is_custom: bool) -> &'static str",
        "ToasterPartState {",
        "slot_attr: input.slot.as_attr()",
        "portal_attr: if input.portal { \"true\" } else { \"false\" }",
    ] {
        assert!(
            !source.contains(forbidden),
            "Toaster logic should not reimplement state primitives in ui: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_state_primitives_live_in_ui_state_primitives() {
    let source = load_source("../../crates/ui-state-primitives/src/toaster.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Toaster notifications\";",
        "pub const DEFAULT_PORTAL: bool = true;",
        "pub const DEFAULT_MAX_TOASTS: usize = 3;",
        "pub enum ToasterPosition",
        "pub enum ToasterSlot",
        "pub enum ToasterStoreSource",
        "pub struct ToasterPartStateInput",
        "pub struct ToasterPartState",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "pub fn state_attr(portal: bool) -> &'static str",
        "pub fn queue_attr(max_toasts: usize) -> &'static str",
        "pub fn resolve_state(input: ToasterPartStateInput) -> ToasterPartState",
    ] {
        assert!(
            source.contains(needle),
            "Toaster state primitive module should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toaster_motion_contract_is_delegated_without_local_driver_reimplementation() {
    let motion = load_source("src/toaster/motion.rs");
    let ui_motion = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion",
        "crate::toast::motion::sanitize_motion(motion)",
        "fn sanitize_motion_delegates_to_toast_contract()",
    ] {
        assert!(
            motion.contains(needle),
            "Toaster motion module should keep delegated motion contract via `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "fn attach_motion(",
        "ui_motion::web::animate(",
    ] {
        assert!(
            !motion.contains(forbidden),
            "Toaster motion module should not reimplement motion driver logic: `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            ui_motion.contains(needle),
            "ui-motion should keep non-wasm predictable no-op path via `{needle}`."
        );
    }
}

#[test]
fn toaster_non_wasm_motion_fallback_is_safe_predictable_and_tooling_friendly() {
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");
    let ui_motion = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
        "Effect::new(move |_| {",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            toast_motion.contains(needle) || ui_motion.contains(needle),
            "non-wasm motion fallback contract should include `{needle}`."
        );
    }

    for forbidden in ["panic!(", "unreachable!(", "todo!(", "unimplemented!("] {
        assert!(
            !toast_motion.contains(forbidden) && !ui_motion.contains(forbidden),
            "motion fallback should stay predictable and avoid panic stubs: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_ui_components_layer_assembles_four_layers_without_public_dom_leakage() {
    let module = load_source("src/toaster/mod.rs");
    let logic = load_source("src/toaster/logic.rs");
    let view = load_source("src/toaster/view.rs");
    let styles = load_source("src/toaster/styles.rs");
    let motion = load_source("src/toaster/motion.rs");
    let crate_root = load_source("src/lib.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Toaster;",
        "pub use ui_state_primitives::toaster::{",
        "ToasterPartState",
        "ToasterPartStateInput",
        "ToasterPosition",
        "ToasterSlot",
        "ToasterStoreSource",
    ] {
        assert!(
            module.contains(needle),
            "Toaster module should keep layered assembly boundary via `{needle}`."
        );
    }

    for needle in [
        "use ui_state_primitives::toaster as toaster_state;",
        "toaster_state::normalize_optional_text(value)",
        "toaster_state::resolve_state(input)",
    ] {
        assert!(
            logic.contains(needle),
            "Toaster logic should delegate state primitives via `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, region_attrs};",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "<Sonner",
        "motion=motion",
    ] {
        assert!(
            view.contains(needle),
            "Toaster view should assemble headless + motion contracts via `{needle}`."
        );
    }

    for needle in [
        "--ui-toaster-single-max-width: var(--ui-overlay-panel-min-width);",
        "--ui-toaster-max-inline-width: calc(var(--ui-overlay-panel-min-width) + var(--ui-space-lg) * 9);",
    ] {
        assert!(
            styles.contains(needle),
            "Toaster styles should consume theme tokens via `{needle}`."
        );
    }

    assert!(
        motion.contains("crate::toast::motion::sanitize_motion(motion)"),
        "Toaster motion should keep delegated motion contract."
    );

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !module.contains(forbidden) && !crate_root.contains(forbidden),
            "Public toaster API should not leak DOM/web-sys details: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_styles_include_state_and_source_marker_contracts() {
    let source = load_source("src/toaster/styles.rs");

    for selector in [
        ".ui-toaster[data-motion-source=\"custom\"]",
        ".ui-toaster[data-custom-motion=\"true\"]",
        ".ui-toaster--custom-motion",
        ".ui-toaster[data-position-source=\"custom\"]",
        ".ui-toaster[data-custom-position=\"true\"]",
        ".ui-toaster--custom-position",
        ".ui-toaster[data-portal-source=\"custom\"]",
        ".ui-toaster[data-custom-portal=\"true\"]",
        ".ui-toaster--custom-portal",
        ".ui-toaster[data-max-toasts-source=\"custom\"]",
        ".ui-toaster[data-custom-max-toasts=\"true\"]",
        ".ui-toaster--custom-max-toasts",
        ".ui-toaster[data-aria-source=\"custom\"]",
        ".ui-toaster[data-custom-aria=\"true\"]",
        ".ui-toaster--custom-aria",
        ".ui-toaster[data-class-source=\"custom\"]",
        ".ui-toaster[data-custom-class=\"true\"]",
        ".ui-toaster--custom-class",
        ".ui-toaster[data-store-source=\"provided\"]",
        ".ui-toaster[data-store-source=\"context\"]",
        ".ui-toaster[data-store-source=\"local\"]",
        ".ui-toaster[data-state=\"inline\"]",
        ".ui-toaster[data-queue=\"single\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-queue=\"bounded\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster__sonner[data-slot=\"toaster-sonner\"].ui-sonner",
    ] {
        assert!(
            source.contains(selector),
            "Toaster styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn toaster_state_markers_are_observable_queryable_and_closed_set() {
    let view_source = load_source("src/toaster/view.rs");
    let styles_source = load_source("src/toaster/styles.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/toaster.rs");

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
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster view should expose observable/queryable markers via `{needle}`."
        );
    }

    for needle in [
        ".ui-toaster[data-store-source=\"provided\"]",
        ".ui-toaster[data-store-source=\"context\"]",
        ".ui-toaster[data-store-source=\"local\"]",
        ".ui-toaster[data-state=\"inline\"]",
        ".ui-toaster[data-queue=\"single\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-queue=\"bounded\"] .ui-toaster__sonner.ui-sonner",
    ] {
        assert!(
            styles_source.contains(needle),
            "Toaster styles should consume semantic markers for stable selectors via `{needle}`."
        );
    }

    for needle in [
        "ToasterStoreSource::Provided => \"provided\"",
        "ToasterStoreSource::Context => \"context\"",
        "ToasterStoreSource::Local => \"local\"",
        "if portal { \"portal\" } else { \"inline\" }",
        "if max_toasts <= 1 {",
        "\"single\"",
        "\"bounded\"",
        "\"extended\"",
        "if is_custom { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            primitives_source.contains(needle),
            "Toaster marker values should come from closed-set primitive contracts via `{needle}`."
        );
    }
}

#[test]
fn toaster_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/toaster/styles.rs");
    let view_source = load_source("src/toaster/view.rs");

    for needle in [
        ".ui-toaster[data-state=\"inline\"]",
        ".ui-toaster[data-portal=\"false\"]",
        ".ui-toaster[data-queue=\"single\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-queue=\"bounded\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-queue=\"extended\"] .ui-toaster__sonner.ui-sonner",
        ".ui-toaster[data-store-source=\"provided\"]",
        ".ui-toaster[data-store-source=\"context\"]",
        ".ui-toaster[data-store-source=\"local\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Toaster styles should express visual switching via explicit semantic markers `{needle}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        "style=",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "Toaster should avoid fragile DOM-guessing selectors and inline runtime style logic: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_semantics_contract_checks_prioritize_semantics_over_snapshots() {
    let source = load_source("tests/semantics.rs");
    let self_test_start = source
        .find("fn toaster_semantics_contract_checks_prioritize_semantics_over_snapshots()")
        .expect("self-check test should exist in toaster semantics suite");
    let self_test_rest = &source[self_test_start..];
    let self_test_end_rel = self_test_rest
        .find("\n}\n\n#[test]")
        .expect("self-check test should be followed by another #[test] block");
    let self_test_end = self_test_start + self_test_end_rel + 3;
    let outside_self_test = format!("{}{}", &source[..self_test_start], &source[self_test_end..]);

    for needle in [
        "fn toaster_view_uses_logic_state_contracts()",
        "fn toaster_mounts_headless_region_a11y_contract_in_view()",
        "fn toaster_has_no_controllable_state_axis_and_no_half_controlled_api()",
        "fn toaster_view_tracks_store_source_resolution()",
        "fn toaster_styles_include_state_and_source_marker_contracts()",
        "fn toaster_state_markers_are_observable_queryable_and_closed_set()",
        "fn toaster_agent_contract_schema_is_typed_traceable_and_whitelisted()",
        "fn toaster_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status()",
        "fn toaster_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable()",
        "fn toaster_e2e_repeatable_key_flow_covers_overlay_focus_keyboard_and_async_paths()",
        "fn toaster_docs_examples_and_matrices_stay_synced_with_logic_defaults()",
        "fn toaster_documentation_is_beginner_friendly_with_readme_or_equivalent_entry()",
        "fn toaster_source_first_docs_are_copy_paste_ready_and_traceable()",
        "fn toaster_heroui_strategy_and_component_docs_stay_synced()",
        "fn toaster_anti_patterns_are_blocked_by_contracts()",
        "fn toaster_check2_has_no_remaining_unchecked_items()",
    ] {
        assert!(
            source.contains(needle),
            "Toaster semantics test matrix should include `{needle}`."
        );
    }

    for forbidden in [
        "insta::assert_snapshot!",
        "assert_snapshot!",
        "assert_debug_snapshot!",
        "assert_yaml_snapshot!",
    ] {
        assert!(
            !outside_self_test.contains(forbidden),
            "Toaster semantics contract should not rely on snapshot-only assertions outside forbid-list literals: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_anti_patterns_are_blocked_by_contracts() {
    let suite_source = load_source("tests/semantics.rs");
    let check2_source = load_source("src/toaster/check2.md");

    for needle in [
        "fn toaster_state_primitives_live_in_ui_state_primitives()",
        "fn toaster_state_primitive_source_boundary_is_enforced()",
        "fn toaster_a11y_i18n_contract_uses_headless_and_no_view_text_hardcode()",
        "fn toaster_state_normalization_is_centralized_in_logic()",
        "fn toaster_api_naming_contract_matches_overlay_family_without_alias_drift()",
        "fn toaster_non_composite_api_avoids_parallel_array_conventions()",
        "fn toaster_ui_components_layer_assembles_four_layers_without_public_dom_leakage()",
        "fn toaster_logic_models_positions_queue_and_part_state()",
    ] {
        assert!(
            suite_source.contains(needle),
            "Toaster anti-pattern gate should keep backing contract test `{needle}`."
        );
    }

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
            "Toaster checklist anti-pattern section should keep checked governance marker `{needle}`."
        );
    }
}

#[test]
fn toaster_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/toaster/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "Toaster check2.md should not keep unchecked checklist items once governance is marked complete."
    );
}

#[test]
fn toaster_component_files_respect_layered_responsibilities() {
    let module = load_source("src/toaster/mod.rs");
    let logic = load_source("src/toaster/logic.rs");
    let styles = load_source("src/toaster/styles.rs");
    let view = load_source("src/toaster/view.rs");
    let motion = load_source("src/toaster/motion.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Toaster;",
    ] {
        assert!(
            module.contains(needle),
            "toaster::mod boundary should include `{needle}`."
        );
    }

    for forbidden in ["#[component]", "view!", "pub mod logic", "pub mod view"] {
        assert!(
            !module.contains(forbidden),
            "toaster::mod should stay export-only and avoid impl details: `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "pub fn resolve_state(input: ToasterPartStateInput) -> ToasterPartState",
    ] {
        assert!(
            logic.contains(needle),
            "toaster::logic should keep normalization/derivation contract via `{needle}`."
        );
    }

    for forbidden in [
        "view!",
        "#[component]",
        "pub const CSS",
        "<Sonner",
        "region_attrs(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "toaster::logic should not mix view/style concerns: `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "--ui-toaster-single-max-width: var(--ui-overlay-panel-min-width);",
    ] {
        assert!(
            styles.contains(needle),
            "toaster::styles should keep static token-first css via `{needle}`."
        );
    }

    for forbidden in [
        "#[component]",
        "view!",
        "fn normalize_props(",
        "region_attrs(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "toaster::styles should not carry logic/view behavior: `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn Toaster(",
        "use ui_headless::{A11yDirection, region_attrs};",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "<Sonner",
    ] {
        assert!(
            view.contains(needle),
            "toaster::view should keep structure + headless mount via `{needle}`."
        );
    }

    for forbidden in ["pub const CSS", "pub fn sanitize_motion("] {
        assert!(
            !view.contains(forbidden),
            "toaster::view should not own styles/motion engines: `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion",
        "crate::toast::motion::sanitize_motion(motion)",
    ] {
        assert!(
            motion.contains(needle),
            "toaster::motion should keep delegated motion contract via `{needle}`."
        );
    }

    for forbidden in ["#[component]", "view!", "pub const CSS"] {
        assert!(
            !motion.contains(forbidden),
            "toaster::motion should not mix view/style responsibilities: `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toaster_directory_standard_files_and_boundaries_follow_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let toaster_dir = manifest_dir.join("src/toaster");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = toaster_dir.join(required);
        assert!(
            path.exists(),
            "toaster directory should include standard file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = toaster_dir.join(forbidden);
        assert!(
            !path.exists(),
            "toaster directory should not introduce `{forbidden}` for current host scope."
        );
    }

    let mod_source = load_source("src/toaster/mod.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let styles_source = load_source("src/toaster/styles.rs");
    let view_source = load_source("src/toaster/view.rs");
    let motion_source = load_source("src/toaster/motion.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Toaster;",
    ] {
        assert!(
            mod_source.contains(needle),
            "toaster::mod should keep minimal stable export boundary marker `{needle}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "toaster::mod should avoid implementation/spec over-export marker `{forbidden}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "NodeRef<",
        "#[component]",
        "view! {",
        "pub const CSS",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "toaster::logic should stay normalize/derive/source-marker only; forbid `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-overlay-panel-min-width)",
        "var(--ui-space-lg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "toaster::styles should keep token-first static css marker `{needle}`."
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "on:click=",
        "on:keydown=",
        "normalize_props(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "toaster::styles should not own render/event/logic behavior `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn Toaster(",
        "use ui_headless::{A11yDirection, region_attrs};",
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
        "<Sonner",
    ] {
        assert!(
            view_source.contains(needle),
            "toaster::view should keep render + headless mount contract marker `{needle}`."
        );
    }
    for forbidden in ["pub const CSS", "pub fn sanitize_motion("] {
        assert!(
            !view_source.contains(forbidden),
            "toaster::view should not absorb styles/motion engine responsibilities `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: crate::toast::ToastMotion) -> crate::toast::ToastMotion",
        "crate::toast::motion::sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(needle),
            "toaster::motion should keep semantic-to-motion contract delegation marker `{needle}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "pub const CSS"] {
        assert!(
            !motion_source.contains(forbidden),
            "toaster::motion should not mix view/style responsibilities `{forbidden}`."
        );
    }

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
            "Toaster checklist should keep directory-standard governance rule `{required}`."
        );
    }
}

#[test]
fn toaster_does_not_define_spec_module_for_simple_host_component() {
    let source = load_source("src/toaster/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let toaster_spec = manifest_dir.join("src/toaster/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "use crate::toaster::spec"] {
        assert!(
            !source.contains(forbidden),
            "Toaster simple host component should not introduce spec module wiring: `{forbidden}`."
        );
    }

    assert!(
        !toaster_spec.exists(),
        "Toaster should not define `src/toaster/spec.rs` without explicit complex spec-contract need."
    );
}

#[test]
fn toaster_engineering_contract_is_spec_free_tracing_aligned_and_runtime_agnostic() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let toaster_mod_source = load_source("src/toaster/mod.rs");
    let toaster_logic_source = load_source("src/toaster/logic.rs");
    let toaster_motion_source = load_source("src/toaster/motion.rs");
    let toaster_view_source = load_source("src/toaster/view.rs");
    let toast_view_source = load_source("../../components/toast/src/toast/view.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    assert!(
        cargo_source.contains("component-toaster = [\"dep:ui-toast\"]"),
        "Toaster feature should stay lightweight and avoid implicit engineering dependency fan-out."
    );
    for forbidden in [
        "component-toaster = [\"dep:serde\"",
        "component-toaster = [\"dep:serde_json\"",
        "component-toaster = [\"dep:tracing\"",
        "component-toaster = [\"dep:tokio\"",
        "component-toaster = [\"dep:async-std\"",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "Toaster feature should not pin serde/tracing/runtime deps directly: `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir.join("src/toaster/spec.rs").exists(),
        "Toaster host scope should keep spec/config serde migration path as N/A without local spec.rs."
    );
    for forbidden in ["mod spec;", "pub mod spec;", "use crate::toaster::spec"] {
        assert!(
            !toaster_mod_source.contains(forbidden),
            "Toaster module boundary should stay spec-free for current host scope: `{forbidden}`."
        );
    }

    let toaster_combined = format!(
        "{toaster_mod_source}\n{toaster_logic_source}\n{toaster_motion_source}\n{toaster_view_source}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "tokio::",
        "async_std::",
        "async-std::",
        "Runtime",
        "JoinHandle",
        "#[tokio::main]",
        "async fn ",
    ] {
        assert!(
            !toaster_combined.contains(forbidden),
            "Toaster implementation should not leak spec serialization/runtime details into component API: `{forbidden}`."
        );
    }

    for forbidden in [
        "pub use toaster::{Toaster, ToasterPosition,",
        "tokio",
        "async_std",
        "serde",
    ] {
        assert!(
            !crate_root_source.contains(forbidden),
            "ui crate root should not leak runtime/spec details through toaster public exports: `{forbidden}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(\"toast\", controlled_open, default_open, on_open_change);",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            toast_view_source.contains(needle) || trace_source.contains(needle),
            "Toaster interaction tracing should stay aligned with shared ui-headless trace semantics via `{needle}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "info_span!(",
        "debug_span!(",
    ] {
        assert!(
            !toaster_combined.contains(forbidden),
            "Toaster host should not introduce ad-hoc tracing vocabulary outside shared contracts: `{forbidden}`."
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
            "Toaster checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn toaster_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    for needle in [
        "#[cfg(feature = \"component-toaster\")]\npub use ui_toast::toaster;",
        "pub use root::UiRoot;",
        "pub use toaster::{Toaster, ToasterPosition};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep stable export/gate marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-toaster\")]\n    out.push_str(crate::toaster::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry should keep feature-gated component aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight entry should keep shared style/motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn Toaster(", "ui-toaster"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain generic shared utility, not component-business implementation: `{forbidden}`."
        );
    }

    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "ui should keep shared `../ui-visual-primitive/src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
    );

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless layer should keep canonical primitive entry marker `{needle}`."
        );
    }

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toaster checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn toaster_token_first_static_styles_are_injected_via_uiroot() {
    let styles = load_source("src/toaster/styles.rs");
    let view = load_source("src/toaster/view.rs");
    let css = load_source("src/css.rs");
    let root = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-overlay-panel-min-width)",
        "var(--ui-space-lg)",
        "max-width: min(100%, var(--ui-toaster-single-max-width));",
    ] {
        assert!(
            styles.contains(needle),
            "Toaster styles should keep token-first static CSS via `{needle}`."
        );
    }

    for needle in [
        "out.push_str(crate::toaster::styles::CSS);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            css.contains(needle) || root.contains(needle),
            "Toaster CSS should be aggregated and injected via UiRoot pipeline: `{needle}`."
        );
    }

    assert!(
        !view.contains("style="),
        "Toaster view should avoid inline business styling and rely on styles.rs contract."
    );
}

#[test]
fn toaster_tree_shaking_feature_gates_are_component_scoped() {
    let cargo_toml = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-toaster = [\"dep:ui-toast\"]",
        "all-components = [",
        "\"component-toaster\"",
        "#[cfg(feature = \"component-toaster\")]\npub use ui_toast::toaster;",
        "#[cfg(feature = \"component-toaster\")]\n    out.push_str(crate::toaster::styles::CSS);",
    ] {
        assert!(
            cargo_toml.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Toaster tree-shaking contract should include `{needle}`."
        );
    }

    assert_eq!(
        lib_source.matches("pub use ui_toast::toaster;").count(),
        1,
        "toaster module should have a single, feature-gated export path."
    );
    assert_eq!(
        css_source
            .matches("out.push_str(crate::toaster::styles::CSS);")
            .count(),
        1,
        "toaster css should have a single, feature-gated aggregation path."
    );
}

#[test]
fn toaster_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitives = load_source("../../crates/ui-state-primitives/src/toaster.rs");
    let logic = load_source("src/toaster/logic.rs");
    let view = load_source("src/toaster/view.rs");

    for needle in [
        "pub enum ToasterPosition",
        "pub enum ToasterSlot",
        "pub enum ToasterStoreSource",
        "pub struct ToasterPartStateInput",
        "pub struct ToasterPartState",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
        "max_toasts.max(1)",
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "has_custom_position: input.position != ToasterPosition::default()",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-position-source=root_state.position_source_attr",
        "data-store-source=root_state.store_source_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            primitives.contains(needle) || logic.contains(needle) || view.contains(needle),
            "Toaster machine-readable contract should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] position: Option<String>",
        "data-state=\"custom\"",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "Toaster should avoid untyped/disjoint machine-readable state contracts: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    for needle in [
        "pub enum ToasterAgentIntent",
        "pub enum ToasterAgentActionModel",
        "pub enum ToasterAgentStreamSupport",
        "pub enum ToasterAgentStreamFallback",
        "pub enum ToasterAgentOutputStatus",
        "pub struct ToasterAgentContract",
        "pub fn agent_contract() -> ToasterAgentContract",
        "schema_attr: \"ui.toaster.v1\"",
        "ToasterAgentIntent::NotificationHost.as_attr()",
        "ToasterAgentActionModel::PushClearDismiss.as_attr()",
        "ToasterAgentStreamSupport::Optional.as_attr()",
        "ToasterAgentStreamFallback::Snapshot.as_attr()",
        "ToasterAgentOutputStatus::Verified.as_attr()",
        "state_axis_attr: \"state|queue|position|portal|max-toasts\"",
        "source_axis_attr: \"position|portal|max-toasts|aria|class|motion|store\"",
        "let agent_contract = logic::agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-state=root_state.state_attr",
        "data-position-source=root_state.position_source_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Toaster agent contract should include typed schema marker `{needle}`."
        );
    }

    for forbidden in [
        "format!(\"ui.toaster",
        "String::from(\"ui.toaster",
        "let ui_schema =",
        "data-ui-schema=aria_label",
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "<script",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "Toaster agent contract render path should stay whitelisted and reject script injection token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toaster checklist should keep agent-contract governance rule `{required}`."
        );
    }
}

#[test]
fn toaster_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/toaster/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "toaster/check2.md should pin streaming-definition marker `{needle}`."
        );
    }
}

#[test]
fn toaster_stays_snapshot_host_and_does_not_mount_stream_protocol_fields() {
    let view_source = load_source("src/toaster/view.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let toaster_docs_start = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist");
    let toaster_docs_end = docs_source
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after toaster");
    let toaster_docs = &docs_source[toaster_docs_start..toaster_docs_end];

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "AiRenderMode",
        "AiOutputStatus",
        "data-draft",
        "token_delta",
        "partial_chunk",
        "incremental_render",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !toaster_docs.contains(forbidden),
            "Toaster is a snapshot-host overlay component and should not expose streaming protocol token `{forbidden}`."
        );
    }

    for marker in [
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Toaster should expose declarative streaming-policy/output-status marker `{marker}`."
        );
    }
}

#[test]
fn toaster_snapshot_baseline_consumes_complete_configuration_and_renders_stably() {
    let view_source = load_source("src/toaster/view.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    for needle in [
        "#[prop(optional)] position: ToasterPosition",
        "#[prop(optional, default = logic::DEFAULT_PORTAL)] portal: bool",
        "#[prop(optional, default = logic::DEFAULT_MAX_TOASTS)] max_toasts: usize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ToastMotion",
        "#[prop(optional)] store: Option<ToastStore>",
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "let root_state = logic::resolve_state(ToasterPartStateInput {",
        "let sonner_state = logic::resolve_state(ToasterPartStateInput {",
        "let root_class_name = logic::compose_class_name(class_name.get_value(), root_state);",
        "let sonner_class_name = logic::compose_class_name(None, sonner_state);",
        "let sonner_position = logic::map_to_sonner_position(root_state.position);",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-store-source=root_state.store_source_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "<Sonner",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster snapshot baseline should consume complete configuration and render stable host contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct ToasterNormalizeInput",
        "pub struct ToasterNormalizedProps",
        "pub fn normalize_props(input: ToasterNormalizeInput) -> ToasterNormalizedProps",
        "max_toasts: normalize_max_toasts(input.max_toasts)",
        "has_custom_position: input.position != ToasterPosition::default()",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS",
    ] {
        assert!(
            logic_source.contains(needle),
            "Toaster logic should provide deterministic complete-input normalization marker `{needle}`."
        );
    }

    let toaster_docs_start = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist");
    let toaster_docs_end = docs_source
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after toaster");
    let toaster_docs = &docs_source[toaster_docs_start..toaster_docs_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue Host\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center Host\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
        "<Toaster />",
        "store=portal_store.get_value()",
        "store=inline_store.get_value()",
        "store=source_store.get_value()",
    ] {
        assert!(
            toaster_docs.contains(needle),
            "Toaster docs should demonstrate complete snapshot configuration markers via `{needle}`."
        );
    }

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toaster checklist should keep snapshot-baseline governance rule `{required}`."
        );
    }
}

#[test]
fn toaster_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status() {
    let logic_source = load_source("src/toaster/logic.rs");
    let view_source = load_source("src/toaster/view.rs");
    let checklist_source = load_source("src/toaster/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "toaster/check2.md should keep streaming-duty governance marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ToasterAgentStreamSupport",
        "Self::Optional => \"optional\"",
        "pub enum ToasterAgentStreamFallback",
        "Self::Snapshot => \"snapshot\"",
        "pub enum ToasterAgentOutputStatus",
        "Self::Draft => \"draft\"",
        "Self::Verified => \"verified\"",
        "Self::Submittable => \"submittable\"",
        "stream_support_attr: ToasterAgentStreamSupport::Optional.as_attr()",
        "stream_fallback_attr: ToasterAgentStreamFallback::Snapshot.as_attr()",
        "output_status_attr: ToasterAgentOutputStatus::Verified.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
        "data-state=root_state.state_attr",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Toaster streaming-policy contract should include `{needle}`."
        );
    }

    for forbidden in [
        "use_ai_space_state(",
        "AiRenderMode::Streaming",
        "AiOutputStatus::",
        "token_delta",
        "partial_chunk",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Toaster should keep streaming transport/validation concerns out of component layer `{forbidden}`."
        );
    }
}

#[test]
fn toaster_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_toaster_contract.spec.mjs");

    for needle in [
        "await page.goto(\"/#/components/toaster\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "const docsRoot = page.locator('[data-component=\"toaster\"][data-slot=\"toaster\"]');",
        "[data-slot=\"toaster-source-controls\"]",
        "[data-slot=\"toaster-source-push\"] [data-slot=\"button\"]",
        "[data-slot=\"toaster-source-clear\"] [data-slot=\"button\"]",
        "[data-slot=\"toaster\"][data-state=\"inline\"][data-position=\"top-left\"][data-store-source=\"provided\"][data-motion-source=\"custom\"]",
        "const sourceViewport = sourceHost.locator('[data-slot=\"toast-viewport\"]').first();",
        "toHaveAttribute(\"data-ui-stream-support\", \"optional\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveCount(0);",
        "toHaveCount(0, {\n    timeout: 6000,",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Toaster e2e selector/wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "hasText:",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Toaster e2e should use semantic-ready waits and avoid brittle selector/wait API `{forbidden}`."
        );
    }
}

#[test]
fn toaster_e2e_repeatable_key_flow_covers_overlay_focus_keyboard_and_async_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_toaster_contract.spec.mjs");

    for needle in [
        "docs-app toaster key flow is repeatable with semantic breakpoints",
        "const portalControls = docsRoot.locator('[data-slot=\"toaster-portal-controls\"]');",
        "[data-slot=\"toaster-portal-push-success\"] [data-slot=\"button\"]",
        "const portalViewport = page",
        "[data-slot=\"toast-viewport\"][data-state=\"portal\"][data-store-source=\"provided\"]",
        "const closeButton = toast.locator('[data-slot=\"toast-close\"]').first();",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "await closeButton.focus();",
        "await expect(closeButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "toHaveCount(0, {\n    timeout: 6000,",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Toaster key-flow e2e contract should include high-risk semantic breakpoint `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Toaster key-flow e2e should avoid unstable fixed-delay waits `{forbidden}`."
        );
    }
}

#[test]
fn toaster_cross_platform_compile_contract_has_explicit_cfg_and_no_non_wasm_web_sys_usage() {
    let toaster_view = load_source("src/toaster/view.rs");
    let toaster_motion = load_source("src/toaster/motion.rs");
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");
    let ui_motion = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if ui_motion::web::prefers_reduced_motion()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            toast_motion.contains(needle) || ui_motion.contains(needle),
            "Cross-platform contract should keep explicit cfg/no-op motion paths via `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "window().",
        "document().",
        "leptos::web_sys",
        "wasm_bindgen::",
    ] {
        assert!(
            !toaster_view.contains(forbidden) && !toaster_motion.contains(forbidden),
            "Toaster non-wasm-safe paths should not depend on browser-only APIs: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let toaster_view = load_source("src/toaster/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "use ui_headless::{A11yDirection, region_attrs};",
    ] {
        assert!(
            headless_lib.contains(needle) || toaster_view.contains(needle),
            "ui-headless web/ssr mutex contract should include `{needle}`."
        );
    }
}

#[test]
fn toaster_component_paths_cover_reduced_motion_ssr_and_wasm_without_semantic_split() {
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");
    let toaster_view = load_source("src/toaster/view.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "drop(style.set_property(\"--ui-toast-opacity\", \"1\"));",
        "drop(style.set_property(\"--ui-toast-y\", \"0px\"));",
        "drop(style.set_property(\"--ui-toast-scale\", \"1\"));",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            toast_motion.contains(needle),
            "Toast motion should cover reduced-motion/wasm/non-wasm branch contract via `{needle}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
        "data-state=root_state.state_attr",
        "data-queue=root_state.queue_attr",
        "data-position=root_state.position_attr",
        "data-portal=root_state.portal_attr",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            toaster_view.contains(needle),
            "Toaster view should keep SSR/wasm semantic contract stable via `{needle}`."
        );
    }

    for forbidden in ["#[cfg(", "cfg!(", "if cfg!(", "if cfg("] {
        assert!(
            !toaster_view.contains(forbidden),
            "Toaster semantic markup should not split by platform in view layer: `{forbidden}`."
        );
    }
}

#[test]
fn toaster_performance_governance_contract_is_budgeted_repeatable_attributable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/toaster/check2.md");
    let toaster_view = load_source("src/toaster/view.rs");
    let toast_motion = load_source("../../components/toast/src/toast/motion.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "UiPerfBudget::mount_only(120.0)",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep repeatable perf budget/probe contract via `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Toaster\", \"toaster\", \"Overlays\", overlays_extra::toaster)",
        "\"toaster\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "Toaster docs should stay in component coverage traversal via `{needle}`.",
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
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance script should keep blocking gate `{needle}`."
        );
    }

    assert!(
        todo_source.contains("render_count"),
        "performance governance should keep explicit render_count follow-up tracking in plan."
    );

    for needle in [
        "render_count",
        "Button`、`Input`",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Toaster checklist should keep perf baseline and follow-up contract marker `{needle}`."
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
            toaster_view.contains(needle),
            "Toaster view should expose attribution markers for perf triage via `{needle}`."
        );
    }

    let view_effect_count = toaster_view.matches("Effect::new(").count();
    assert_eq!(
        view_effect_count, 0,
        "Toaster host view should avoid direct effect loops; found {view_effect_count}.",
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
fn toaster_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("src/toaster/view.rs");

    assert!(
        view_source.contains("view! {"),
        "Toaster should keep explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Toaster should keep one bounded `view!` block for current host scope."
    );
    assert!(
        view_source.lines().count() <= 160,
        "Toaster view.rs should stay compact; split semantic subrenders if this grows significantly."
    );

    for forbidden in ["for item in", ".map(|", "collect::<Vec<_>>()", "match ("] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should avoid loop-heavy or branch-heavy macro patterns `{forbidden}`."
        );
    }
}

#[test]
fn toaster_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("src/toaster/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Toaster should keep a single public component boundary for current host layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn toaster_",
        "#[component]\nfn sonner_",
        "pub fn render_",
        "pub fn section_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster should avoid extra local component abstraction noise `{forbidden}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_props(logic::ToasterNormalizeInput {",
        "let root_state = logic::resolve_state(ToasterPartStateInput {",
        "let sonner_state = logic::resolve_state(ToasterPartStateInput {",
        "data-state=root_state.state_attr",
        "data-store-source=root_state.store_source_attr",
        "<Sonner",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster functional assembly should keep stable semantic markers after split choices via `{needle}`."
        );
    }
}

#[test]
fn toaster_static_fragments_are_constantized_or_absent_for_simple_host_layout() {
    let view_source = load_source("src/toaster/view.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/toaster.rs");

    for forbidden in [
        "inner_html=",
        "<svg",
        "<path",
        "<footer",
        "<article",
        "let markdown",
        "let long_text",
        "Toaster notifications",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toaster view should avoid inlined heavy static fragments and keep host layout lean: `{forbidden}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Toaster notifications\";",
        "pub use ui_state_primitives::toaster::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "aria-label=region_a11y.aria_label",
        "role=region_a11y.role",
    ] {
        assert!(
            primitives_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Toaster static/a11y fragment path should stay centralized and traceable via `{needle}`."
        );
    }
}

#[test]
fn toaster_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked() {
    let view_source = load_source("src/toaster/view.rs");
    let logic_source = load_source("src/toaster/logic.rs");
    let motion_source = load_source("src/toaster/motion.rs");
    let styles_source = load_source("src/toaster/styles.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "inner_html=",
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
            "Toaster component implementation should forbid HTML injection path `{forbidden}`."
        );
    }

    let toaster_docs_start = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist");
    let toaster_docs_end = docs_source
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after toaster");
    let toaster_docs = &docs_source[toaster_docs_start..toaster_docs_end];

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !toaster_docs.contains(forbidden),
            "Toaster docs section should not introduce untrusted HTML injection path `{forbidden}`."
        );
    }

    for needle in [
        "let region_a11y = region_attrs(normalized.aria_label, lang, dir);",
        "role=region_a11y.role",
        "aria-label=region_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Toaster should keep semantic/a11y mounting without inner_html fallback via `{needle}`."
        );
    }
}

#[test]
fn toaster_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let toaster_mod_source = load_source("src/toaster/mod.rs");
    let toaster_logic_source = load_source("src/toaster/logic.rs");
    let toaster_motion_source = load_source("src/toaster/motion.rs");
    let toaster_view_source = load_source("src/toaster/view.rs");
    let toast_view_source = load_source("../../components/toast/src/toast/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let wasm_debug_script = load_source("../../scripts/check-ui-wasm-debug.sh");

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root_source.contains(needle),
            "ui should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("toaster-wasm-debug"),
        "Toaster should not expose a dedicated wasm-debug feature because it is a host assembler without private debug runtime."
    );

    let toaster_combined = format!(
        "{toaster_mod_source}\n{toaster_logic_source}\n{toaster_motion_source}\n{toaster_view_source}"
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
            !toaster_combined.contains(forbidden),
            "Toaster production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for marker in [
        "data-position-source=root_state.position_source_attr",
        "data-portal-source=root_state.portal_source_attr",
        "data-max-toasts-source=root_state.max_toasts_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-store-source=root_state.store_source_attr",
    ] {
        assert!(
            toaster_view_source.contains(marker),
            "Toaster should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(\"toast\",",
        "request_open_change.run(",
    ] {
        assert!(
            toast_view_source.contains(needle),
            "Toast interaction path should remain traceable/replayable through headless trace contract `{needle}`."
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
            "docs-app should keep wasm dev visual-entry gate `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "ui_headless::UiTraceEventKind::Inspect { tag, data_slot }",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "OpenChange {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamp/source event markers `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting",
    ] {
        assert!(
            wasm_debug_script.contains(needle),
            "wasm-debug check script should keep feature-isolated verification marker `{needle}`."
        );
    }
}

#[test]
fn toaster_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
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

    for needle in [
        "pub(super) fn toaster() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue Host\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center Host\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Toaster docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toaster_dx_workbench_uses_interactive_playground_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

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

    let toaster_docs_start = docs_source
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist");
    let toaster_docs_end = docs_source
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after toaster");
    let toaster_docs = &docs_source[toaster_docs_start..toaster_docs_end];

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue Host\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center Host\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
        "<Button on_press=push_inline>\"Push inline toast\"</Button>",
        "<Button on_press=push_source>\"Push source toast\"</Button>",
    ] {
        assert!(
            toaster_docs.contains(needle),
            "Toaster docs should provide isolated interactive playground entry `{needle}`."
        );
    }

    for forbidden in [
        "TOASTER_WORKBENCH_STORAGE_KEY",
        "load_toaster_workbench_state(",
        "save_toaster_workbench_state(",
        "clear_toaster_workbench_state(",
        "Persist workbench state",
        "test_config_signal=",
    ] {
        assert!(
            !toaster_docs.contains(forbidden),
            "Toaster host docs should keep optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn toaster_dx_check_script_keeps_shared_playground_contract_gate() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "[dx] contract: playground css hot-reload path",
        "cargo test -p ui --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should keep shared playground contract gate `{needle}`."
        );
    }
}

#[test]
fn toaster_styles_consume_ui_theme_tokens_for_layout_bounds() {
    let source = load_source("src/toaster/styles.rs");

    for needle in [
        "--ui-toaster-single-max-width: var(--ui-overlay-panel-min-width);",
        "--ui-toaster-max-inline-width: calc(var(--ui-overlay-panel-min-width) + var(--ui-space-lg) * 9);",
        "max-width: min(100%, var(--ui-toaster-single-max-width));",
        "max-width: min(100%, var(--ui-toaster-max-inline-width));",
    ] {
        assert!(
            source.contains(needle),
            "Toaster styles should consume ui-theme variables via `{needle}`."
        );
    }

    for forbidden in ["420px", "360px"] {
        assert!(
            !source.contains(forbidden),
            "Toaster styles should not hardcode pixel bounds; found `{forbidden}`."
        );
    }
}

#[test]
fn toaster_docs_page_contains_state_source_playground() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn toaster() -> AnyView",
        "title=\"Toaster\"",
        "slug=\"toaster\"",
        "State + Source Markers",
        "data-position-source",
        "data-store-source",
        "<Toaster",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs page should contain `{needle}`."
        );
    }
}

#[test]
fn toaster_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::toaster::styles::CSS);"),
        "ui css aggregator should include toaster styles."
    );
}

#[test]
fn toaster_docs_custom_motion_playground_locks_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "portal=false",
        "position=ToasterPosition::TopLeft",
        "max_toasts=4",
        "aria_label=\"Alert stream\".to_string()",
        "class_name=\"docs-toaster-source\".to_string()",
        "motion=ToastMotion {",
        "let custom_motion = ToastMotion {",
        "initial_y_px: 20.0",
        "initial_scale: 0.95",
        "..ToastMotion::default()",
        "motion=custom_motion",
        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools.",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn toaster_docs_examples_and_matrices_stay_synced_with_logic_defaults() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let logic = load_source("src/toaster/logic.rs");

    for needle in [
        "data-slot=\"toaster-api-matrix\"",
        "data-slot=\"toaster-api-rows\"",
        "data-slot=\"toaster-state-matrix\"",
        "data-slot=\"toaster-state-rows\"",
        "\"API Matrix\"",
        "\"State Matrix\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue Host\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center Host\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs should keep synced examples/matrix marker `{needle}`."
        );
    }

    for needle in [
        "\"position: ToasterPosition\"",
        "\"portal: bool\"",
        "\"max_toasts: usize\"",
        "\"aria_label: Option<String>\"",
        "\"class_name: Option<String>\"",
        "\"lang: Option<String>, dir: Option<A11yDirection>\"",
        "\"motion: ToastMotion\"",
        "\"store: Option<ToastStore>\"",
        "ToasterPosition::default()",
        "ui::toaster::DEFAULT_PORTAL",
        "ui::toaster::DEFAULT_MAX_TOASTS",
        "ui::toaster::DEFAULT_ARIA_LABEL",
        "default path = provided -> context -> local",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs API matrix should stay aligned with logic defaults via `{needle}`."
        );
    }

    for needle in [
        "\"data-state\"",
        "\"data-queue\"",
        "\"data-position\"",
        "\"data-store-source\"",
        "\"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source\"",
        "\"control mode\"",
        "no controlled/uncontrolled runtime axis",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster docs state matrix should keep semantic/state-axis marker `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::toaster::{DEFAULT_ARIA_LABEL, DEFAULT_MAX_TOASTS, DEFAULT_PORTAL};",
        "has_custom_portal: input.portal != DEFAULT_PORTAL,",
        "has_custom_max_toasts: input.max_toasts != DEFAULT_MAX_TOASTS,",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
    ] {
        assert!(
            logic.contains(needle),
            "Toaster logic default/source contract should include `{needle}`."
        );
    }
}

#[test]
fn toaster_documentation_is_beginner_friendly_with_readme_or_equivalent_entry() {
    let readme = load_source("src/toaster/README.md");
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2 = load_source("src/toaster/check2.md");

    for needle in [
        "# Toaster",
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "<Toaster />",
        "## 常见用法",
        "## 再进阶（高级控制）",
        "默认 API 路径优先",
        "不需要用户手动接线 `ui-state-primitives` / `ui-headless`",
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs",
    ] {
        assert!(
            readme.contains(needle),
            "Toaster README should include beginner-friendly marker `{needle}`."
        );
    }

    let hello_idx = readme
        .find("### Hello World（最小可用）")
        .expect("Toaster README should contain Hello World section");
    let advanced_idx = readme
        .find("## 再进阶（高级控制）")
        .expect("Toaster README should contain advanced section");
    assert!(
        hello_idx < advanced_idx,
        "Toaster README should keep default path before advanced path."
    );

    for needle in [
        "pub(super) fn toaster() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "<Playground title=\"Portal Queue Host\" code_signal=basic_code>",
        "<Playground title=\"Inline Top-Center Host\" code_signal=state_code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            docs.contains(needle),
            "docs-app equivalent entry should include `{needle}`."
        );
    }

    assert!(
        check2.contains("组件文档必须对新手友好（Documentation as Product）"),
        "Toaster checklist should keep documentation-as-product governance item."
    );
}

#[test]
fn toaster_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let playground = load_source("../../apps/docs-app/src/playground.rs");
    let check2 = load_source("src/toaster/check2.md");

    for needle in [
        "data-slot=\"toaster-source-first\"",
        "\"Source-first / Copy-Paste Ready\"",
        "Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "use leptos::prelude::*;\\nuse ui::*;\\n\\n<Toaster />",
        "data-slot=\"toaster-source-paths\"",
        "components/toaster/src/mod.rs",
        "components/toaster/src/logic.rs",
        "components/toaster/src/view.rs",
        "components/toaster/src/styles.rs",
        "components/toaster/src/motion.rs",
        "data-slot=\"toaster-source-prerequisites\"",
        "\"component-toaster\"",
        "\"component-toast\"",
        "\"component-sonner\"",
    ] {
        assert!(
            docs.contains(needle),
            "Toaster source-first docs should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "Show code",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground.contains(needle),
            "Playground copy-ready path should include `{needle}`."
        );
    }

    assert!(
        check2.contains("Source-first 文档必须 Copy-Paste Ready"),
        "Toaster checklist should keep source-first governance item."
    );
}

#[test]
fn toaster_heroui_strategy_and_component_docs_stay_synced() {
    let strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2 = load_source("src/toaster/check2.md");

    for needle in [
        "### Toaster 同步记录（2026-02-18）",
        "`Toaster` 继续保持宿主定位，公开参数为 `position/portal/max_toasts/aria_label/class_name/lang/dir/motion/store`",
        "component_doc!(\"Toaster\", \"toaster\", \"Overlays\", overlays_extra::toaster)",
        "Source-first / Copy-Paste Ready",
        "HeroUI 对齐结论：保持“默认路径简洁、进阶参数按需开启”的体验目标",
    ] {
        assert!(
            strategy.contains(needle),
            "HeroUI strategy sync should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Toaster\", \"toaster\", \"Overlays\", overlays_extra::toaster)",
        "pub(super) fn toaster() -> AnyView",
        "slug=\"toaster\"",
        "data-slot=\"toaster-source-first\"",
    ] {
        assert!(
            docs_index.contains(needle) || docs_page.contains(needle),
            "Toaster docs entry/index should include `{needle}`."
        );
    }

    assert!(
        check2.contains("HeroUI 对标文档与组件文档同步"),
        "Toaster checklist should keep HeroUI sync governance item."
    );
}

#[test]
fn toaster_docs_page_covers_primary_playgrounds() {
    toaster_docs_page_contains_state_source_playground();
}

#[test]
fn toaster_docs_playgrounds_lock_state_matrix_contract_values() {
    toaster_docs_custom_motion_playground_locks_contract_values();
}

#[test]
fn toaster_docs_visual_baseline_uses_design_system_primitives() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let start = docs
        .find("pub(super) fn toaster() -> AnyView")
        .expect("toaster docs section should exist");
    let end = docs
        .find("pub(super) fn underlay() -> AnyView")
        .expect("underlay docs section should exist after toaster");
    let toaster_docs = &docs[start..end];

    for needle in [
        "title=\"Toaster\"",
        "<Playground title=\"Hello World\"",
        "<Playground title=\"Portal Queue Host\"",
        "<Playground title=\"Inline Top-Center Host\"",
        "title=\"State + Source Markers\"",
        "<Button",
    ] {
        assert!(
            toaster_docs.contains(needle),
            "Toaster docs visual baseline should include `{needle}`."
        );
    }

    for forbidden in ["<button", "btn btn-", "Bootstrap"] {
        assert!(
            !toaster_docs.contains(forbidden),
            "Toaster docs should avoid fallback legacy styling patterns: `{forbidden}`."
        );
    }
}
