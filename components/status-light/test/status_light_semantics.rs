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

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/status_light/") {
        let path = workspace_dir()
            .join("components/status-light/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_status_light_docs_section() -> String {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    docs_tail[..status_light_end].to_string()
}

#[test]
fn status_light_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/status_light/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "StatusLight internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn status_light_avoids_component_motion_contract_when_no_transition_semantics() {
    let mod_source = load_source("src/status_light/mod.rs");
    let view_source = load_source("src/status_light/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_path = manifest_dir.join("src/status_light/motion.rs");

    assert!(
        !motion_path.exists(),
        "StatusLight should not define component motion module when no reusable transition semantics exist."
    );

    for forbidden in ["mod motion", "pub mod motion"] {
        assert!(
            !mod_source.contains(forbidden),
            "StatusLight module should not expose motion wiring; found `{forbidden}`."
        );
    }

    for forbidden in ["use ui_motion", "ui_motion::", "attach_motion(", "motion::"] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view should not bind motion engine directly; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_component_layer_keeps_assembly_boundaries_and_public_api() {
    let mod_source = load_source("src/status_light/mod.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let view_source = load_source("src/status_light/view.rs");
    let styles_source = load_source("src/status_light/styles.rs");
    let lib_source = load_source("src/lib.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{StatusLightRole, StatusLightVariant};",
        "pub use view::StatusLight;",
    ] {
        assert!(
            mod_source.contains(required),
            "StatusLight module boundary should include `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::status_light::{",
        "pub struct StatusLightRootInput {",
        "pub struct StatusLightRootState {",
        "StatusLightStateInput",
        "StatusLightState",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
        "pub fn normalize_root_state(input: StatusLightRootInput) -> StatusLightRootState {",
    ] {
        assert!(
            logic_source.contains(required),
            "StatusLight logic should remain assembly over state primitives; missing `{required}`."
        );
    }

    for forbidden in ["use ui_headless", "ui_headless::", "web_sys", "web-sys"] {
        assert!(
            !logic_source.contains(forbidden),
            "StatusLight logic should not depend on headless/web internals; found `{forbidden}`."
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, StatusLightOptions, use_status_light};",
        "logic::{self, StatusLightRootInput},",
        "let root = logic::normalize_root_state(StatusLightRootInput {",
        "let semantics = use_status_light(StatusLightOptions {",
        "state: root.state,",
        "class=root.class_name",
        "role=semantics.attrs.role",
    ] {
        assert!(
            view_source.contains(required),
            "StatusLight view should mount headless semantics from normalized logic output; missing `{required}`."
        );
    }

    for forbidden in ["web_sys", "web-sys"] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view should not expose DOM/web-sys detail types; found `{forbidden}`."
        );
    }

    for forbidden in ["ui_headless", "ui_state_primitives", "web_sys", "web-sys"] {
        assert!(
            !styles_source.contains(forbidden),
            "StatusLight styles should remain static token-first CSS only; found `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-status_light\")]",
        "pub use ui_status_light as status_light;",
        "pub use status_light::{StatusLight, StatusLightRole, StatusLightVariant};",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components public API should gate and export StatusLight stably; missing `{required}`."
        );
    }
}

#[test]
fn status_light_public_api_naming_contract_is_consistent_and_has_no_alias_drift() {
    let view_source = load_source("src/status_light/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "#[prop(optional)] variant: Option<StatusLightVariant>,",
        "#[prop(optional)] role: Option<StatusLightRole>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            view_source.contains(required),
            "StatusLight public prop naming should stay stable; missing `{required}`."
        );
    }

    for forbidden in [
        "className",
        "on_role_change",
        "on_variant_change",
        "default_role",
        "default_variant",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight should not introduce alias drift in prop/callback naming; found `{forbidden}`."
        );
    }

    // This component currently has no bool/callback/default public state axis,
    // so is_ / on_ / default_ naming is N/A by design at prop declaration level.
    for forbidden in [
        "#[prop(optional)] is_",
        "#[prop(optional, into)] is_",
        "#[prop(optional)] on_",
        "#[prop(optional, into)] on_",
        "#[prop(optional)] default_",
        "#[prop(optional, into)] default_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight should not expose unmatched prop naming axes; found `{forbidden}`."
        );
    }

    for required in [
        "<StatusLight variant=StatusLightVariant::Default>\"Idle\"</StatusLight>",
        "<StatusLight role=StatusLightRole::Status>\"Background sync complete\"</StatusLight>",
        "class_name=\"docs-status-light-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "StatusLight docs/examples should use canonical API names; missing `{required}`."
        );
    }
}

#[test]
fn status_light_has_no_controllable_state_axis_and_avoids_half_controlled_contracts() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    let status_light_docs = &docs_tail[..status_light_end];

    // StatusLight is a pure presentational/status primitive consumer with no
    // internal mutable state axis, so controlled/uncontrolled triple is N/A.
    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_",
        "#[prop(optional)] on_",
        "#[prop(optional, into)] default_",
        "#[prop(optional, into)] on_",
        "on_value_change",
        "default_value",
        "use_controllable_state",
        "use_controllable_open_state_traced",
        "RwSignal",
        "WriteSignal",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight should not expose half-controlled public API or local control state; found `{forbidden}` in view."
        );
    }

    for forbidden in [
        "on_value_change",
        "default_value",
        "RwSignal",
        "WriteSignal",
        "signal(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "StatusLight logic should not own mutable control state; found `{forbidden}`."
        );
    }

    for forbidden in [
        "default_",
        "on_",
        "on_value_change",
        "default_value",
        "value=",
    ] {
        assert!(
            !status_light_docs.contains(forbidden),
            "StatusLight docs should not document unsupported controlled/uncontrolled props; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_uses_logic_state_model() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    for needle in [
        "pub use ui_state_primitives::status_light::{",
        "StatusLightRootInput",
        "StatusLightRootState",
        "StatusLightStateInput",
        "StatusLightState",
        "normalize_optional_text",
        "resolve_state",
        "pub fn compose_class_name(",
        "pub fn normalize_root_state(",
        "variant.unwrap_or_default()",
        "ui-status-light--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "StatusLight logic should include `{needle}` for primitive consumption + assembly."
        );
    }

    for forbidden in [
        "pub struct StatusLightStateInput {",
        "pub struct StatusLightState {",
        "pub fn resolve_state(input: StatusLightStateInput)",
        "pub fn normalize_optional_text(value: Option<String>)",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "StatusLight component logic should not re-implement primitive `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_root_state(StatusLightRootInput {",
        "state: root.state,",
        "class=root.class_name",
        "use ui_headless::{A11yDirection, StatusLightOptions, use_status_light};",
        "let semantics = use_status_light(StatusLightOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "StatusLight view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "role=state.role_attr",
        "data-live=state.is_live.then_some(\"true\")",
        "data-role-source=state.role_source_attr",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(StatusLightStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view should mount headless semantics instead of in-view derivation; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_defaults_are_single_sourced_in_logic() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    assert!(
        logic_source.contains("variant.unwrap_or_default()"),
        "StatusLight default variant should be resolved only in logic via unwrap_or_default()."
    );
    assert!(
        !view_source.contains("unwrap_or"),
        "StatusLight view must not apply default fallback branches."
    );
}

#[test]
fn status_light_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    assert!(
        logic_source.contains("pub fn normalize_root_state("),
        "StatusLight logic should provide a centralized normalization entry."
    );

    for forbidden in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(StatusLightStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view should not reconstruct state machine/normalization; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_discrete_state_axes_are_enum_typed() {
    let primitive_source = load_source("../ui-state-primitives/src/status_light.rs");
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    for required in [
        "pub enum StatusLightVariant {",
        "pub enum StatusLightRole {",
        "variant: Option<StatusLightVariant>",
        "role: Option<StatusLightRole>",
    ] {
        let found = primitive_source.contains(required)
            || view_source.contains(required)
            || logic_source.contains(required);
        assert!(
            found,
            "StatusLight discrete states should remain enum-typed; missing `{required}`."
        );
    }

    for forbidden in ["Option<bool>", "variant: String", "role: String"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "StatusLight should avoid bool explosion/stringly typed state axes; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_consumes_state_primitives_without_direct_store_binding() {
    let logic_source = load_source("src/status_light/logic.rs");
    let view_source = load_source("src/status_light/view.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::status_light::{"),
        "StatusLight should source its reusable state primitives from ui-state-primitives."
    );
    assert!(
        logic_source.contains("resolve_state"),
        "StatusLight logic should assemble from primitive output instead of reimplementing a local state machine."
    );

    for forbidden in [
        "RwSignal",
        "WriteSignal",
        "ReadSignal",
        "store",
        "app_store",
        "global_store",
        "signal_store",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "StatusLight should stay store-agnostic in component layer; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_marks_async_interaction_contract_as_not_applicable() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    let status_light_docs = &docs_tail[..status_light_end];

    // StatusLight is synchronous status rendering only; no async workflow in component contract.
    for forbidden in [
        "async fn",
        ".await",
        "is_loading",
        "aria-busy",
        "retry",
        "error",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !status_light_docs.contains(forbidden),
            "StatusLight async contract should be N/A; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_api_is_dx_friendly_without_exposing_internal_state_wiring() {
    let view_source = load_source("src/status_light/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    let status_light_docs = &docs_tail[..status_light_end];

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] machine:",
        "#[prop(optional)] semantics:",
        "StatusLightState",
        "StatusLightOptions",
        "use_status_light(",
    ] {
        assert!(
            !status_light_docs.contains(forbidden),
            "StatusLight docs should keep internal state/headless wiring hidden from basic users; found `{forbidden}`."
        );
    }

    assert!(
        status_light_docs
            .contains("<StatusLight variant=StatusLightVariant::Default>\"Idle\"</StatusLight>"),
        "StatusLight docs should keep a copy-paste-ready one-line baseline usage."
    );

    for forbidden in ["#[prop(optional)] state:", "#[prop(optional)] machine:"] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight public API should not require internal state objects; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_avoids_composite_parallel_array_api_patterns() {
    let view_source = load_source("src/status_light/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    let status_light_docs = &docs_tail[..status_light_end];

    for forbidden in [
        "labels",
        "titles",
        "panels",
        "item_specs",
        "ItemSpec",
        "Vec<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !status_light_docs.contains(forbidden),
            "StatusLight should not introduce composite parallel-array conventions; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_a11y_and_i18n_contracts_are_headless_backed() {
    let view_source = load_source("src/status_light/view.rs");
    let headless_source = load_source("../ui-headless/src/status_light.rs");

    for required in [
        "use ui_headless::{A11yDirection, StatusLightOptions, use_status_light};",
        "role=semantics.attrs.role",
        "aria-live=semantics.attrs.aria_live",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
        "{children()}",
    ] {
        assert!(
            view_source.contains(required),
            "StatusLight view should mount a11y/i18n contract output from headless; missing `{required}`."
        );
    }

    for required in [
        "use crate::a11y::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};",
        "pub struct StatusLightOptions {",
        "pub lang: Option<String>,",
        "pub dir: Option<A11yDirection>,",
        "let locale = locale_attrs(options.lang, options.dir);",
        "live_region_attrs(LiveRegionPriority::Polite)",
    ] {
        assert!(
            headless_source.contains(required),
            "StatusLight a11y/i18n contract should come from ui-headless shared tools; missing `{required}`."
        );
    }
}

#[test]
fn status_light_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/status_light/view.rs");

    for attr in [
        "data-slot=\"status-light\"",
        "role=semantics.attrs.role",
        "aria-live=semantics.attrs.aria_live",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
        "data-variant=semantics.attrs.data_variant",
        "data-state=semantics.attrs.data_state",
        "data-live=semantics.attrs.data_live",
        "data-static=semantics.attrs.data_static",
        "data-role=semantics.attrs.data_role",
        "data-role-source=semantics.attrs.data_role_source",
        "data-custom-class=semantics.attrs.data_custom_class",
        "data-class-source=semantics.attrs.data_class_source",
        "data-slot=\"status-light-indicator\"",
        "data-variant=semantics.attrs.data_variant",
        "data-slot=\"status-light-label\"",
    ] {
        assert!(
            source.contains(attr),
            "StatusLight should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn status_light_styles_include_variant_state_and_source_markers() {
    let source = load_source("src/status_light/styles.rs");

    for selector in [
        ".ui-status-light--variant-default",
        ".ui-status-light[data-variant=\"accent\"]",
        ".ui-status-light--variant-danger",
        ".ui-status-light--live",
        ".ui-status-light[data-state=\"live\"]",
        ".ui-status-light--static",
        ".ui-status-light[data-static=\"true\"]",
        ".ui-status-light[data-state=\"static\"] .ui-status-light__dot",
        ".ui-status-light--role-custom",
        ".ui-status-light[data-role-source=\"custom\"]",
        ".ui-status-light--custom-class",
        ".ui-status-light[data-custom-class=\"true\"]",
        ".ui-status-light[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "StatusLight styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn status_light_styles_consume_ui_theme_tokens_for_core_visual_values() {
    let source = load_source("src/status_light/styles.rs");

    for expected in [
        "gap: var(--ui-space-sm);",
        "font-size: var(--ui-font-size-150);",
        "width: var(--ui-space-sm);",
        "height: var(--ui-space-sm);",
        "border-radius: var(--ui-radius-lg);",
        "--ui-status-light-dot: var(--ui-fg-muted);",
        "--ui-status-light-label: var(--ui-fg-muted);",
        "--ui-status-light-dot: var(--ui-accent);",
        "--ui-status-light-dot: var(--ui-danger);",
    ] {
        assert!(
            source.contains(expected),
            "StatusLight styles should consume ui-theme CSS variables; missing `{expected}`."
        );
    }

    for forbidden in [
        "font-size: 14px;",
        "width: 10px;",
        "height: 10px;",
        "border-radius: 9999px;",
    ] {
        assert!(
            !source.contains(forbidden),
            "StatusLight styles should avoid hardcoded visual constants; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_component_file_responsibilities_stay_within_layer_boundaries() {
    let mod_source = load_source("src/status_light/mod.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let styles_source = load_source("src/status_light/styles.rs");
    let view_source = load_source("src/status_light/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_path = manifest_dir.join("src/status_light/motion.rs");

    for required in ["mod logic;", "pub mod styles;", "mod view;"] {
        assert!(
            mod_source.contains(required),
            "StatusLight module should keep standard file layout; missing `{required}`."
        );
    }

    for forbidden in ["web_sys", "ui_headless::", "use ui_headless"] {
        assert!(
            !logic_source.contains(forbidden),
            "StatusLight logic should not carry DOM/headless mounting details; found `{forbidden}`."
        );
    }

    for forbidden in [
        "resolve_state(StatusLightStateInput {",
        "compose_class_name(class_name, state)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view should not hide state-machine decisions; found `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "ui_headless", "StatusLightStateInput"] {
        assert!(
            !styles_source.contains(forbidden),
            "StatusLight styles should remain static style contract only; found `{forbidden}`."
        );
    }

    assert!(
        !motion_path.exists(),
        "StatusLight should not define component motion.rs when no reusable transition semantic axis exists."
    );
}

#[test]
fn status_light_does_not_introduce_unnecessary_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/status_light/spec.rs");
    assert!(
        !spec_path.exists(),
        "StatusLight should not add spec.rs without stable external schema/config pressure."
    );
}

#[test]
fn status_light_token_first_styles_are_feature_gated_in_css_aggregation() {
    let css_source = load_source("src/css.rs");
    let styles_source = load_source("src/status_light/styles.rs");

    assert!(
        css_source.contains("#[cfg(feature = \"component-status_light\")]")
            && css_source.contains("out.push_str(crate::status_light::styles::CSS);"),
        "StatusLight CSS should be aggregated through feature-gated push_components_css entry."
    );

    for forbidden in ["style=", ":nth-child", ".docs-"] {
        assert!(
            !styles_source.contains(forbidden),
            "StatusLight component styles should stay token-first/static and avoid fragile/runtime styling; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_respects_ui_components_entrypoint_contracts() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for required in [
        "#[cfg(feature = \"component-status_light\")]",
        "pub use ui_status_light as status_light;",
        "pub use status_light::{StatusLight, StatusLightRole, StatusLightVariant};",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components lib.rs should expose StatusLight behind feature gate; missing `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-status_light\")]",
        "out.push_str(crate::status_light::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css.rs should aggregate StatusLight CSS via feature-gated push; missing `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should centralize css/theme/i18n injection; missing `{required}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        let path = manifest_dir.join(forbidden);
        assert!(
            !path.exists(),
            "ui-components should not host `{forbidden}` root helper; contract belongs to ui-headless."
        );
    }
}

#[test]
fn status_light_avoids_forbidden_architecture_antipatterns() {
    let primitive_source = load_source("../ui-state-primitives/src/status_light.rs");
    let headless_source = load_source("../ui-headless/src/status_light.rs");
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");

    for forbidden in [
        "web_sys",
        "web-sys",
        "class=",
        "style=",
        "ui_motion::",
        "attach_motion(",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "status-primitives layer must stay DOM/style free; found `{forbidden}`."
        );
    }

    for forbidden in ["ui-motion", "attach_motion(", "class=", "style="] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless layer must not carry visual/motion orchestration; found `{forbidden}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(StatusLightStateInput {",
        "logic::compose_class_name(class_name, state)",
        "labels",
        "titles",
        "panels",
        "className",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight view/API should avoid hidden state decisions and implicit array conventions; found `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::status_light::{"),
        "Reusable state primitive should be sourced from ui-state-primitives instead of remaining in component logic."
    );
}

#[test]
fn status_light_view_macro_complexity_is_controlled() {
    let view_source = load_source("src/status_light/view.rs");
    let view_macro_count = view_source.matches("view!").count();

    assert_eq!(
        view_macro_count, 1,
        "StatusLight view should keep a single compact view! block."
    );
    assert!(
        view_source.lines().count() < 120,
        "StatusLight view should remain small and semantically split; oversized macro block detected."
    );
}

#[test]
fn status_light_uses_minimal_component_surface_and_no_inner_html() {
    let view_source = load_source("src/status_light/view.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let styles_source = load_source("src/status_light/styles.rs");
    let mod_source = load_source("src/status_light/mod.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "StatusLight should expose a minimal single component surface."
    );

    for forbidden in ["inner_html", "<svg", "dangerously_set_inner_html"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "StatusLight should avoid unsafe/static-fragment anti-patterns in runtime rendering; found `{forbidden}`."
        );
    }
}

#[test]
fn status_light_streaming_mode_is_not_applicable_and_snapshot_render_is_default() {
    let view_source = load_source("src/status_light/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let status_light_start = docs_source
        .find("pub(super) fn status_light() -> AnyView")
        .expect("status_light docs section should exist");
    let docs_tail = &docs_source[status_light_start..];
    let status_light_end = docs_tail
        .find("\npub(super) fn ")
        .unwrap_or(docs_tail.len());
    let status_light_docs = &docs_tail[..status_light_end];

    for forbidden in [
        "is_streaming",
        "stream",
        "snapshot",
        "fallback=",
        "draft",
        "verified",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight component contract should not introduce LLM-streaming-specific props; found `{forbidden}`."
        );
    }

    for required in [
        "<StatusLight variant=StatusLightVariant::Default>\"Idle\"</StatusLight>",
        "<StatusLight role=StatusLightRole::Status>\"Background sync complete\"</StatusLight>",
    ] {
        assert!(
            status_light_docs.contains(required),
            "StatusLight docs should demonstrate stable full-result snapshot rendering; missing `{required}`."
        );
    }
}

#[test]
fn status_light_docs_page_covers_primary_playgrounds() {
    let source = load_status_light_docs_section();

    for needle in [
        "pub(super) fn status_light() -> AnyView",
        "title=\"StatusLight\"",
        "slug=\"status-light\"",
        "title=\"Hello World\"",
        "title=\"Variants\"",
        "title=\"Live Region Role\"",
        "title=\"Custom Class + Static\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for StatusLight.",
        );
    }
}

#[test]
fn status_light_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_status_light_docs_section();

    for needle in [
        "let hello_world_code =",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<StatusLight>\"Idle\"</StatusLight>",
        "title=\"Variants\"",
        "code_signal=variants_code",
        "<StatusLight variant=StatusLightVariant::Default>\"Idle\"</StatusLight>",
        "<StatusLight variant=StatusLightVariant::Accent>\"Deploying\"</StatusLight>",
        "<StatusLight variant=StatusLightVariant::Danger>\"Failed\"</StatusLight>",
        "title=\"Live Region Role\"",
        "code_signal=role_code",
        "<StatusLight role=StatusLightRole::Status>\"Background sync complete\"</StatusLight>",
        "title=\"Custom Class + Static\"",
        "code_signal=custom_code",
        "class_name=\"docs-status-light-custom\".to_string()",
        "role=StatusLightRole::Status",
    ] {
        assert!(
            source.contains(needle),
            "status-light docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn status_light_docs_are_newcomer_friendly_with_default_path_first() {
    let status_light_docs = load_status_light_docs_section();

    for needle in [
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "<StatusLight>\"Idle\"</StatusLight>",
        "title=\"Variants\"",
        "code_signal=variants_code",
        "title=\"Live Region Role\"",
        "code_signal=role_code",
        "title=\"Custom Class + Static\"",
        "code_signal=custom_code",
    ] {
        assert!(
            status_light_docs.contains(needle),
            "StatusLight docs should keep newcomer-friendly marker `{needle}`."
        );
    }

    let hello_world_idx = status_light_docs
        .find("title=\"Hello World\"")
        .expect("StatusLight docs should provide a Hello World entry point.");
    let variants_idx = status_light_docs
        .find("title=\"Variants\"")
        .expect("StatusLight docs should provide common-usage examples.");
    let role_idx = status_light_docs
        .find("title=\"Live Region Role\"")
        .expect("StatusLight docs should provide advanced role controls.");
    let custom_idx = status_light_docs
        .find("title=\"Custom Class + Static\"")
        .expect("StatusLight docs should provide advanced custom styling controls.");

    assert!(
        hello_world_idx < variants_idx && variants_idx < role_idx && role_idx < custom_idx,
        "StatusLight docs should be progressive: Hello World -> common usage -> advanced controls."
    );

    for forbidden in [
        "use_status_light(",
        "StatusLightRootInput",
        "StatusLightStateInput",
    ] {
        assert!(
            !status_light_docs.contains(forbidden),
            "StatusLight docs should stay newcomer-oriented and avoid internal architecture token `{forbidden}`."
        );
    }
}

#[test]
fn status_light_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let e2e_coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/status_light/view.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep performance budget wiring token `{needle}`."
        );
    }

    for needle in [
        "const coverageMode = process.env.E2E_COVERAGE ?? \"sample\";",
        "body:not(:has(#boot))",
        "[data-slot=\"ui-perf-probe\"]",
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
    ] {
        assert!(
            e2e_coverage_source.contains(needle),
            "docs-app coverage e2e should keep repeatable perf baseline marker `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance check script should keep blocking governance token `{needle}`."
        );
    }

    for needle in [
        "data-variant=semantics.attrs.data_variant",
        "data-state=semantics.attrs.data_state",
        "data-role-source=semantics.attrs.data_role_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "StatusLight should expose semantic attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn status_light_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let styles_source = load_source("src/status_light/styles.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "font-size: var(--ui-font-size-150);",
        "font-weight: 500;",
        "font-weight: 600;",
        "box-shadow: 0 0 0 1px color-mix(in oklch, var(--ui-fg) 12%, transparent);",
        "--ui-status-light-dot: var(--ui-accent);",
        "--ui-status-light-dot: var(--ui-danger);",
    ] {
        assert!(
            styles_source.contains(needle),
            "StatusLight styles should keep visual-quality token `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "Theme visual baseline page should keep visual desire marker `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_source.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc_source.contains(needle),
            "HeroUI strategy doc should keep alignment marker `{needle}`."
        );
    }
}

#[test]
fn status_light_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let ui_components_lib_source = load_source("src/lib.rs");
    let docs_app_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let view_source = load_source("src/status_light/view.rs");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("status-light-wasm-debug"),
        "StatusLight should not introduce a component-local wasm-debug feature."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_lib_source.contains(needle),
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "let ts_ms = event.ts_ms;",
        "data-component=component",
        "data-kind=kind_attr",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep temporal trace marker `{needle}`."
        );
    }

    for needle in [
        "data-state=semantics.attrs.data_state",
        "data-role-source=semantics.attrs.data_role_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "StatusLight should expose reproducible semantic markers for debug tracing via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] debug",
        "data-debug-source=",
        "data-debug-before=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight public API should not leak wasm-debug internals `{forbidden}`."
        );
    }
}

#[test]
fn status_light_dx_playground_supports_css_hot_reload_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let status_light_docs = load_status_light_docs_section();

    for needle in [
        "fn compose_scoped_css(scope_selector: &str, raw: &str) -> String {",
        "compose_original_css_source()",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "<UiPerfProbe name=format!(\"Playground::{title}\")>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-reload/canvas marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Variants\"",
        "title=\"Live Region Role\"",
        "title=\"Custom Class + Static\"",
        "code_signal=variants_code",
        "code_signal=role_code",
        "code_signal=custom_code",
    ] {
        assert!(
            status_light_docs.contains(needle),
            "StatusLight docs should keep interactive playground marker `{needle}`."
        );
    }

    for forbidden in [
        "workbench",
        "persisted_workbench_state",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !status_light_docs.contains(forbidden),
            "StatusLight is non-workbench scope; optional state persistence should stay N/A `{forbidden}`."
        );
    }
}

#[test]
fn status_light_engineering_contract_is_runtime_agnostic_and_narrow() {
    let mod_source = load_source("src/status_light/mod.rs");
    let logic_source = load_source("src/status_light/logic.rs");
    let view_source = load_source("src/status_light/view.rs");
    let styles_source = load_source("src/status_light/styles.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/status_light/spec.rs");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "serde::",
            "Serialize",
            "Deserialize",
            "tracing::",
            "tokio::",
            "async_std::",
            "async-std",
            "Runtime",
            "async fn",
            ".await",
        ] {
            assert!(
                !source.contains(forbidden),
                "StatusLight should remain runtime-agnostic without serde/tracing/runtime leaks; found `{forbidden}`."
            );
        }
    }

    assert!(
        !spec_path.exists(),
        "StatusLight should not add spec serialization boundary when no spec/config axis exists."
    );
}

#[test]
fn status_light_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_status_light_contract.spec.mjs");

    for needle in [
        "/#/components/status-light",
        "body:not(:has(#boot))",
        "[data-component=\"status-light\"] [data-slot=\"status-light\"]",
        "toHaveAttribute(\"data-variant\", \"default\")",
        "toHaveAttribute(\"data-state\", \"static\")",
        "toHaveAttribute(\"data-role-source\", \"none\")",
        "toHaveAttribute(\"data-class-source\", \"default\")",
        "toHaveAttribute(\"role\", \"status\")",
        "toHaveAttribute(\"aria-live\", \"polite\")",
        "toHaveAttribute(\"data-state\", \"live\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "StatusLight e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "StatusLight e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn status_light_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_status_light_contract.spec.mjs");

    for needle in [
        "status-light key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/badge\");",
        "await expect(page.locator(\".docs-page-title\")).toHaveText(\"Badge\");",
        "await page.goto(\"/#/components/status-light\");",
        "toHaveAttribute(\"data-state\", \"static\")",
        "toHaveAttribute(\"data-role-source\", \"none\")",
        "toHaveAttribute(\"data-state\", \"live\")",
        "toHaveAttribute(\"data-role-source\", \"custom\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "StatusLight e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "StatusLight e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn status_light_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let component_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let status_light_docs = load_status_light_docs_section();
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_status_light_contract.spec.mjs");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "Source: {path}",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy pipeline should keep import-ready marker `{needle}`."
        );
    }

    for needle in [
        "let import_text = format!(\"use ui_components::{title};\");",
        "label=\"Import\".to_string()",
    ] {
        assert!(
            component_shell_source.contains(needle),
            "ComponentPage should keep dependency/import prerequisite marker `{needle}`."
        );
    }

    for needle in [
        "let hello_world_code =",
        "let variants_code = Signal::derive(move || {",
        "let role_code = Signal::derive(move || {",
        "let custom_code = Signal::derive(move || {",
        "title=\"Hello World\"",
        "code_signal=hello_world_code",
        "title=\"Variants\"",
        "code_signal=variants_code",
        "title=\"Live Region Role\"",
        "code_signal=role_code",
        "title=\"Custom Class + Static\"",
        "code_signal=custom_code",
    ] {
        assert!(
            status_light_docs.contains(needle),
            "StatusLight docs should keep source-first code_signal marker `{needle}`."
        );
    }

    let source_path_marker = "test_source_path=\"components/status-light/src/view.rs\".to_string()";
    let source_path_count = status_light_docs.matches(source_path_marker).count();
    assert!(
        source_path_count >= 4,
        "StatusLight docs should point each playground to real source path marker `{source_path_marker}`."
    );

    for needle in [
        "#[prop(optional, default = true)] copyable: bool,",
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "<Show when=move || state.copyable>",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep copy action contract marker `{needle}`."
        );
    }

    for needle in [
        "status-light playground code path remains copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "<StatusLight variant=StatusLightVariant::Default>",
    ] {
        assert!(
            e2e_source.contains(needle),
            "StatusLight e2e should keep copy-paste-ready marker `{needle}`."
        );
    }
}

#[test]
fn status_light_heroui_strategy_and_component_docs_are_synced_for_parameter_model_or_marked_na() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let research_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let status_light_docs = load_status_light_docs_section();
    let view_source = load_source("src/status_light/view.rs");

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "### Recommendation",
        "HeroUI 对齐结论",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy doc should keep baseline marker `{needle}`."
        );
    }

    for needle in [
        "# Spectrum × HeroUI 样式与接口综合学习（v0）",
        "Option C: Hybrid（推荐）",
    ] {
        assert!(
            research_source.contains(needle),
            "HeroUI/Spectrum research doc should remain accessible marker `{needle}`."
        );
    }

    for needle in ["\"StatusLight\"", "\"status-light\""] {
        assert!(
            pages_source.contains(needle),
            "StatusLight docs route should stay indexable token `{needle}`."
        );
    }

    for needle in [
        "title=\"StatusLight\"",
        "slug=\"status-light\"",
        "description=\"Status indicator + label with centralized variant/live/role-source state attrs and optional custom-class contract.\"",
    ] {
        assert!(
            status_light_docs.contains(needle),
            "StatusLight docs should keep discoverable entry marker `{needle}`."
        );
    }

    for forbidden in [
        "default_variant",
        "on_variant_change",
        "default_role",
        "on_role_change",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "StatusLight has no recent breaking parameter rename; found drift marker `{forbidden}`."
        );
    }
}

#[test]
fn status_light_contract_consistency_has_no_temporary_patch_markers() {
    for rel_path in [
        "src/status_light/mod.rs",
        "src/status_light/logic.rs",
        "src/status_light/view.rs",
        "src/status_light/styles.rs",
        "src/status_light/check2.md",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "TODO(temp)",
            "TEMP PATCH",
            "temporary patch",
            "HACK(status-light)",
            "FIXME(status-light)",
            "compat alias",
            "legacy alias",
        ] {
            assert!(
                !source.contains(forbidden),
                "StatusLight contract path `{rel_path}` should not keep temporary consistency marker `{forbidden}`."
            );
        }
    }
}

#[test]
fn status_light_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/status_light_semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_status_light_contract.spec.mjs");

    for needle in [
        "fn status_light_emits_baseline_style_state_data_attributes()",
        "fn status_light_a11y_and_i18n_contracts_are_headless_backed()",
        "fn status_light_e2e_selector_contract_uses_semantic_markers_and_settled_waits()",
        "fn status_light_e2e_key_flow_is_repeatable_and_failure_points_are_semantic()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "StatusLight semantics suite should include contract-first test `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-state\", \"static\")",
        "toHaveAttribute(\"data-role-source\", \"none\")",
        "toHaveAttribute(\"role\", \"status\")",
        "toHaveAttribute(\"aria-live\", \"polite\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "StatusLight e2e contract should assert semantic marker `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot("] {
        assert!(
            !e2e_source.contains(forbidden),
            "StatusLight e2e contract should not be snapshot-first `{forbidden}`."
        );
    }
}
