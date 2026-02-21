use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn link_public_api_stays_minimal() {
    let source = load_source("src/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Link;",
    ] {
        assert!(
            source.contains(needle),
            "link module should include `{needle}` in its public assembly boundary.",
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "pub use logic::"] {
        assert!(
            !source.contains(forbidden),
            "link module should keep internals private; found `{forbidden}`.",
        );
    }
}

#[test]
fn link_view_mounts_headless_semantics_contract() {
    let source = load_source("src/view.rs");

    for needle in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};",
        "let locale = locale_attrs(lang, dir);",
        "let focus_ring = use_focus_ring(FocusRingOptions {",
        "let hover = use_hover(HoverOptions {",
        "data-state=state.state.as_attr()",
        "data-hovered=move || hover.is_hovered.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "link view should mount headless + a11y semantics via `{needle}`.",
        );
    }
}

#[test]
fn link_component_does_not_leak_dom_platform_types() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in ["web_sys::", "wasm_bindgen::", "Element", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "link component public assembly should not expose DOM platform detail `{forbidden}`.",
        );
    }
}

#[test]
fn link_api_naming_uses_contract_prefixes() {
    let source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "#[prop(optional)] target: Option<&'static str>",
    ] {
        assert!(
            source.contains(needle),
            "link public props should include `{needle}` as naming baseline.",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] disabled: Option<bool>",
        "on_disabled_change",
        "default_disabled",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !source.contains(forbidden),
            "link API should not expose non-contract naming `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_controlled_or_uncontrolled_state_axis() {
    let source = load_source("src/view.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] is_open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional, into)] on_value_change:",
        "#[prop(optional, into)] on_open_change:",
        "on_value_change",
        "on_open_change",
        "default_value",
        "default_open",
    ] {
        assert!(
            !source.contains(forbidden),
            "link should not expose controlled/uncontrolled state axis token `{forbidden}`.",
        );
    }
}

#[test]
fn link_defaults_are_normalized_in_logic_only() {
    let source = load_source("src/view.rs");

    for needle in [
        "let href = logic::normalize_href(href);",
        "let (is_disabled, disabled_source) = logic::normalize_is_disabled(is_disabled);",
        "let rel = logic::normalize_optional_text(rel);",
        "let aria_label = logic::normalize_optional_text(aria_label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let target_kind = logic::resolve_target_kind(target);",
        "let state = logic::resolve_state(LinkStateInput {",
        "let rel = logic::resolve_rel(target_kind, rel);",
    ] {
        assert!(
            source.contains(needle),
            "link view should consume logic-normalized defaults via `{needle}`.",
        );
    }

    for forbidden in [
        "unwrap_or(",
        "unwrap_or_else(",
        ".or_else(",
        ".or(",
        "default_open",
        "default_value",
    ] {
        assert!(
            !source.contains(forbidden),
            "link view should not apply local default fallback via `{forbidden}`.",
        );
    }
}

#[test]
fn link_state_normalization_stays_in_logic_layer() {
    let source = load_source("src/view.rs");

    for needle in [
        "let state = logic::resolve_state(LinkStateInput {",
        "is_disabled,",
        "has_href: href.is_some(),",
        "target_kind,",
        "has_explicit_rel: rel.is_some(),",
        "has_aria_label: aria_label.is_some(),",
        "has_custom_class_name: class_name.is_some(),",
    ] {
        assert!(
            source.contains(needle),
            "link view should forward typed input to logic state resolver via `{needle}`.",
        );
    }

    for forbidden in [
        "if is_disabled {",
        "if href.is_some() {",
        "if rel.is_some() {",
        "if aria_label.is_some() {",
        "if class_name.is_some() {",
    ] {
        assert!(
            !source.contains(forbidden),
            "link view should not rebuild state-machine branches via `{forbidden}`.",
        );
    }
}

#[test]
fn link_discrete_states_use_typed_enums() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/link.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub enum LinkTargetKind",
        "pub enum LinkVisualState",
        "pub enum LinkRelSource",
        "pub target_kind: LinkTargetKind,",
        "pub state: LinkVisualState,",
        "pub rel_source: LinkRelSource,",
        "pub fn resolve_target_kind(target: Option<&str>) -> LinkTargetKind",
    ] {
        assert!(
            primitive_source.contains(needle),
            "link primitive should define typed discrete state via `{needle}`.",
        );
    }

    for needle in [
        "let target_kind = logic::resolve_target_kind(target);",
        "target_kind,",
        "data-state=state.state.as_attr()",
        "data-target=state.target_kind.as_attr()",
        "data-rel=state.rel_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "link view should consume enum-driven state mapping via `{needle}`.",
        );
    }
}

#[test]
fn link_logic_only_assembles_state_primitives_without_store_binding() {
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "pub use ui_state_primitives::link::{",
        "LinkStateInput",
        "normalize_is_disabled",
        "resolve_state",
        "resolve_rel",
    ] {
        assert!(
            logic_source.contains(needle),
            "link logic should consume state primitives via `{needle}`.",
        );
    }

    for forbidden in [
        "pub struct LinkStateInput",
        "pub struct LinkState",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "store",
        "global_state",
        "app_state",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "link logic should stay as assembly-only boundary; found `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_async_interaction_protocol() {
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "data-loading",
        "data-error",
        "data-retry",
        "on_retry",
        "use_async_action",
        "retry",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link should not introduce async interaction protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn link_dx_paradox_keeps_default_api_and_hello_world_docs() {
    let view_source = load_source("src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for forbidden in [
        "#[prop()] state:",
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "state=state",
        "state=link_state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link should not require internal state wiring via `{forbidden}`.",
        );
    }

    for needle in [
        "title=\"Hello World (Default API)\"",
        "<Link href=\"#/docs/welcome\".to_string()>\"Read docs\"</Link>",
    ] {
        assert!(
            docs_source.contains(needle),
            "link docs should keep default API onboarding path via `{needle}`.",
        );
    }
}

#[test]
fn link_is_leaf_api_without_composite_parent_item_contract() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "pub use view::LinkItem;",
        "pub use view::Item;",
        "pub use view::Parent;",
        "pub use view::LinkGroup;",
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "items: Vec",
        "ItemSpec",
        "item_specs",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "link should stay leaf-level API and must not expose composite convention `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_macro_micro_drag_state_machine() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "DragEnd",
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "on:pointermove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce drag macro/micro state-machine token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_two_pass_geometry_rendering_path() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "NodeRef",
        "getBoundingClientRect",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "Intent",
        "Measure",
        "Rectification",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce two-pass geometry token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_registration_protocol_for_dynamic_items() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "link should not introduce collection registration protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_slot_projection_lifecycle_contract() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "projection_mode",
        "on_hidden",
        "on_shown",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce slot projection lifecycle token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_env_stream_subscription_pipeline() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "Action::",
        "on_resize",
        "on_theme",
        "on_intersection",
        "debounce",
        "throttle",
        "subscribe",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce env stream subscription token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_event_light_cone_bulk_collection_contract() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "selection_state",
        "bulk_select",
        "prop_drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce event-light-cone token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_causality_bus_trace_chain_contract() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast",
        "subscriber",
        "publish",
        "dispatch_command",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link should not introduce causality-bus token `{forbidden}`.",
        );
    }
}

#[test]
fn link_exposes_a11y_and_i18n_contract_without_hardcoded_copy() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let locale = locale_attrs(lang, dir);",
        "aria-label=aria_label",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "lang=locale.lang",
        "dir=locale.dir",
        "tabindex=state.is_disabled.then_some(-1)",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
        "{children()}",
    ] {
        assert!(
            view_source.contains(needle),
            "link should keep a11y/i18n contract via `{needle}`.",
        );
    }

    for forbidden in [
        "fn locale_attrs(",
        "enum A11yDirection",
        "\"Internal docs link\"",
        "\"External link\"",
        "\"Disabled\"",
        "\"Missing href\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link view should not define local a11y helpers or hardcode visible copy `{forbidden}`.",
        );
    }
}

#[test]
fn link_state_markers_are_observable_queryable_and_enumerable() {
    let view_source = load_source("src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/link.rs");

    for needle in [
        "data-state=state.state.as_attr()",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-disabled-source=disabled_source.as_attr()",
        "data-missing-href=(!state.has_href).then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "data-target=state.target_kind.as_attr()",
        "data-rel=state.rel_source.as_attr()",
        "data-ui-state=state.state.as_attr()",
        "data-ui-source=disabled_source.as_attr()",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "link should expose stable state/source marker `{needle}` for automated querying.",
        );
    }

    for forbidden in [
        "data-state=format!(",
        "data-target=format!(",
        "data-rel=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link state/source markers should stay enumerable; found free-text marker `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum LinkVisualState",
        "Self::Enabled => \"enabled\"",
        "Self::Disabled => \"disabled\"",
        "Self::MissingHref => \"missing-href\"",
        "pub enum LinkTargetKind",
        "Self::SelfContext => \"self\"",
        "Self::Blank => \"blank\"",
        "Self::Custom => \"custom\"",
        "pub enum LinkRelSource",
        "Self::Provided => \"provided\"",
        "Self::Auto => \"auto\"",
        "pub enum LinkDisabledSource",
        "Self::IsProp => \"is-prop\"",
        "Self::Default => \"default\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "link primitive should keep closed marker set via `{needle}`.",
        );
    }
}

#[test]
fn link_styles_depend_on_explicit_state_markers_not_dom_shape() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        ".ui-link[data-state=\"enabled\"]",
        ".ui-link[data-state=\"disabled\"]",
        ".ui-link[data-state=\"missing-href\"]",
        ".ui-link[data-hovered=\"true\"]",
        ".ui-link[data-focus-visible=\"true\"]",
        ".ui-link[data-target=\"blank\"]",
        ".ui-link[data-rel=\"provided\"]",
        ".ui-link[data-aria-label=\"custom\"]",
        ".ui-link[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "link styles should branch by explicit semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        ".ui-link .",
        ".ui-link >",
        ".ui-link +",
        ".ui-link ~",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "link styles should not infer state from DOM structure token `{forbidden}`.",
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "link view should not embed inline style logic token `{forbidden}`.",
        );
    }
}

#[test]
fn link_semantics_tests_cover_contract_matrix_not_visual_snapshot_only() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let local_semantics = load_source("test/semantics.rs");
    let integration_semantics = load_source("../../crates/ui-components/tests/link_semantics.rs");

    for needle in [
        "data-state=state.state.as_attr()",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-disabled-source=disabled_source.as_attr()",
        "aria-disabled=state.is_disabled.then_some(\"true\")",
        "on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
        "href=if state.is_enabled { href } else { None }",
        "tabindex=state.is_disabled.then_some(-1)",
    ] {
        assert!(
            view_source.contains(needle),
            "link semantic contract matrix should include interaction/state branch `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional, into)] on_value_change:",
        "on_value_change",
        "default_value",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link has no controlled/uncontrolled axis; found unexpected token `{forbidden}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "link semantic contract should not fork by platform for this leaf component; found `{forbidden}`.",
        );
    }

    for needle in [
        "data-state",
        "aria-disabled",
        "data-disabled-source",
        "on:pointerenter",
        "on:focus",
    ] {
        assert!(
            local_semantics.contains(needle) && integration_semantics.contains(needle),
            "semantic tests should assert contract marker/path `{needle}` in both local and integration suites.",
        );
    }

    let snapshot_assert_macro = ["assert_", "snapshot!"].concat();
    let insta_snapshot_macro = ["insta::", "assert_"].concat();
    assert!(
        !local_semantics.contains(&snapshot_assert_macro)
            && !integration_semantics.contains(&snapshot_assert_macro)
            && !local_semantics.contains(&insta_snapshot_macro)
            && !integration_semantics.contains(&insta_snapshot_macro),
        "link contract tests should not rely on snapshot macros as primary oracle.",
    );
}

#[test]
fn link_component_files_keep_layered_responsibilities_with_motion_na() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let cargo_source = load_source("Cargo.toml");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Link;",
    ] {
        assert!(
            mod_source.contains(needle),
            "link module boundary should keep assembly export contract `{needle}`.",
        );
    }

    for forbidden in [
        "pub use logic::",
        "pub use styles::",
        "pub use view::LinkState",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "link module boundary should avoid leaking implementation detail `{forbidden}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::link::{",
        "normalize_href",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "link logic should stay in normalize/derive/source-marking responsibility via `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "<a",
        "on:pointerenter",
        "data-state=",
        "var(--ui-",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "link logic should not include view/dom/style branch token `{forbidden}`.",
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "link styles should keep token-first static css contract via `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "logic::",
        "resolve_state(",
        "use_focus_ring",
        "on:pointerenter",
        "children()",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "link styles should not include logic/view behavior token `{forbidden}`.",
        );
    }

    for needle in [
        "view! {",
        "use_focus_ring",
        "use_hover",
        "let state = logic::resolve_state(LinkStateInput {",
        "data-state=state.state.as_attr()",
        "on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "link view should keep render + headless mount responsibility via `{needle}`.",
        );
    }

    for forbidden in [
        "pub enum LinkVisualState",
        "pub struct LinkStateInput",
        "pub fn compose_class_name(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link view should not redefine primitive/logic implementation token `{forbidden}`.",
        );
    }

    assert!(
        !cargo_source.contains("ui-motion"),
        "link is a leaf component without motion contract and should not depend on ui-motion.",
    );
    assert!(
        !manifest_dir.join("src/motion.rs").exists() && !mod_source.contains("mod motion;"),
        "link keeps motion.rs as N/A for this leaf component (no component motion contract).",
    );
}

#[test]
fn link_avoids_spec_rs_for_simple_component_surface() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "link is a simple component and should not introduce `src/spec.rs`.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "use crate::spec::",
        "LinkSpec::",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "link simple component surface should not depend on spec module token `{forbidden}`.",
        );
    }
}

#[test]
fn link_token_first_static_styles_are_aggregated_via_uiroot_contract() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
    ] {
        assert!(
            styles_source.contains(needle),
            "link styles should follow token-first contract via `{needle}`.",
        );
    }

    for forbidden in ["rgb(", "rgba(", "hsl(", "@apply"] {
        assert!(
            !styles_source.contains(forbidden),
            "link styles should avoid private hardcoded color/utility token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-link\")]",
        "out.push_str(crate::link::styles::CSS);",
        "out.push_str(\"\\n@layer ui {\\n\");",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "link component css should be aggregated through ui-components css pipeline `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated component css via `{needle}`.",
        );
    }

    for forbidden in [
        "style=",
        "style:",
        "stylist::",
        "css!(",
        "tailwind",
        "tw_merge",
        "class=\\\"flex",
        "class=\\\"grid",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link view should not adopt utility-first or css-in-rust default pattern `{forbidden}`.",
        );
    }
}

#[test]
fn link_tree_shaking_contract_is_feature_gated_in_ui_components() {
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");

    for needle in [
        "component-link = [\"dep:ui-link\"]",
        "#[cfg(feature = \"component-link\")]\npub use ui_link as link;",
        "#[cfg(feature = \"component-link\")]\n    out.push_str(crate::link::styles::CSS);",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
    ] {
        assert!(
            ui_components_cargo.contains(needle)
                || ui_components_lib.contains(needle)
                || ui_components_css.contains(needle),
            "tree-shaking contract should be feature-gated via `{needle}`.",
        );
    }

    assert!(
        !ui_components_cargo.contains("default = [\"all-components\"]"),
        "ui-components default feature should not force full registry without inject-css baseline.",
    );
}

#[test]
fn link_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/link.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub enum LinkDisabledSource",
        "pub enum LinkTargetKind",
        "pub enum LinkVisualState",
        "pub enum LinkRelSource",
        "pub struct LinkStateInput",
        "pub struct LinkState",
        "pub fn resolve_target_kind(target: Option<&str>) -> LinkTargetKind",
        "Some(\"_blank\") => LinkTargetKind::Blank",
        "Some(_) => LinkTargetKind::Custom",
        "None => LinkTargetKind::SelfContext",
        "pub fn resolve_state(input: LinkStateInput) -> LinkState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "link primitive should keep typed state contract via `{needle}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::link::{",
        "LinkStateInput",
        "resolve_target_kind",
        "resolve_state",
        "resolve_rel",
    ] {
        assert!(
            logic_source.contains(needle),
            "link logic should normalize/derive through primitives via `{needle}`.",
        );
    }

    for needle in [
        "let target_kind = logic::resolve_target_kind(target);",
        "let state = logic::resolve_state(LinkStateInput {",
        "data-state=state.state.as_attr()",
        "data-target=state.target_kind.as_attr()",
        "data-rel=state.rel_source.as_attr()",
        "data-ui-state=state.state.as_attr()",
        "data-ui-source=disabled_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "link view should expose machine-readable state marker `{needle}`.",
        );
    }

    for forbidden in [
        "if target == \"_blank\"",
        "data-state=format!(",
        "data-target=format!(",
        "data-rel=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "link should avoid stringly state contract token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_overlay_focus_stack_gc_contract() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "NodeRef",
        "FocusTrap",
        "RestorePolicy",
        "FallbackTo",
        "Selector(",
        "focus_stack",
        "overlay_stack",
        "provide_overlay_stack",
        "use_overlay_stack",
        "restore_focus",
        "document.body",
        "data-ui-overlay-portal",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "link should remain overlay-free and must not embed focus-stack token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_escape_hatch_foreign_zone_contract() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "third_party_instance",
        "imperative_instance",
        "js_sys::",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "link should not integrate imperative third-party escape-hatch token `{forbidden}`.",
        );
    }
}

#[test]
fn link_has_no_hydration_discontinuity_and_relies_on_deterministic_id_provider_path() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "Uuid::",
        "new_v4",
        "rand::",
        "thread_rng",
        "random(",
        "use_id(",
        "id_seed",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "link should not introduce hydration-unstable init token `{forbidden}`.",
        );
    }

    for needle in [
        "provide_ui_id_provider(id_seed);",
        "#[prop(optional, default = 1)] id_seed: u64,",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "deterministic id-provider injection path should remain available via `{needle}`.",
        );
    }
}

#[test]
fn link_ssr_cross_platform_contract_avoids_browser_only_bindings() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "window()",
        "document()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "link cross-platform contract should avoid browser-only/platform-fork token `{forbidden}`.",
        );
    }
}

#[test]
fn link_headless_feature_mutex_contract_is_guarded() {
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let link_cargo = load_source("Cargo.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should keep web/ssr mutex guard via `{needle}`.",
        );
    }

    assert!(
        link_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "link should consume ui-headless through workspace dependency path.",
    );
}

#[test]
fn link_motion_non_wasm_noop_contract_is_upstream_and_link_stays_motionless() {
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let mod_source = load_source("src/mod.rs");
    let link_cargo = load_source("Cargo.toml");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion should keep predictable non-wasm no-op contract via `{needle}`.",
        );
    }

    assert!(
        !link_cargo.contains("ui-motion") && !mod_source.contains("mod motion;"),
        "link should remain motion-free and avoid direct ui-motion coupling.",
    );
}

#[test]
fn link_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/styles.rs");

    for needle in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
    ] {
        assert!(
            styles_source.contains(needle),
            "link styles should use defensive variable fallback chain `{needle}`.",
        );
    }

    for forbidden in [
        "var(--ui-space-xs);",
        "var(--ui-accent);",
        "var(--ui-focus-ring);",
        "var(--ui-radius-sm);",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "link styles should avoid single-layer token usage `{forbidden}`.",
        );
    }
}

#[test]
fn link_view_does_not_use_inner_html_injection() {
    let view_source = load_source("src/view.rs");

    for forbidden in ["inner_html=", "inner_html:"] {
        assert!(
            !view_source.contains(forbidden),
            "link view should not use raw html injection token `{forbidden}`.",
        );
    }
}

#[test]
fn link_manifest_and_rbi_projection_contract_are_present() {
    let manifest = load_source("Component.toml");
    let rbi = load_source("Component.rbi");

    for needle in [
        "id = \"ui-link\"",
        "name = \"Link\"",
        "rbi = \"Component.rbi\"",
        "mod_rs = \"src/mod.rs\"",
        "logic_rs = \"src/logic.rs\"",
        "styles_rs = \"src/styles.rs\"",
        "view_rs = \"src/view.rs\"",
        "schema = \"ui.link.agent-contract/v1\"",
    ] {
        assert!(
            manifest.contains(needle),
            "link component manifest should include `{needle}`.",
        );
    }

    for needle in [
        "component \"ui-link\"",
        "signature Link(",
        "mode: \"snapshot\"",
        "fallback: \"snapshot\"",
        "agent_contract_schema \"ui.link.agent-contract/v1\"",
    ] {
        assert!(
            rbi.contains(needle),
            "link RBI projection should include `{needle}`.",
        );
    }
}

#[test]
fn link_agent_streaming_snapshot_markers_are_explicit() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "data-ui-schema=\"ui.link.agent-contract\"",
        "data-ui-schema-version=\"1\"",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
    ] {
        assert!(
            view_source.contains(needle),
            "link view should expose explicit agent marker `{needle}`.",
        );
    }
}

#[test]
fn link_non_test_sources_meet_rust_hygiene_baseline() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/protocol.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["unwrap(", "expect(", "let _ = "] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not contain rust-hygiene violation token `{forbidden}`.",
            );
        }
    }
}
