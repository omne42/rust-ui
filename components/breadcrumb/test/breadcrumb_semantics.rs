use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_breadcrumb_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/breadcrumb").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_breadcrumb_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-breadcrumb\")]")
            && lib_source.contains("pub use ui_breadcrumb as breadcrumb;"),
        "ui should re-export the external ui-breadcrumb crate as `breadcrumb`.",
    );
    assert!(
        cargo_source.contains("component-breadcrumb = [\"dep:ui-breadcrumb\"]"),
        "component-breadcrumb feature should depend on dep:ui-breadcrumb after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-breadcrumb = { path = \"../../components/breadcrumb\", optional = true }"
        ),
        "ui Cargo.toml should include the optional ui-breadcrumb dependency.",
    );
    assert!(
        !cargo_source.contains("component-breadcrumbs ="),
        "component-breadcrumbs should be removed after merge.",
    );
}

#[test]
fn breadcrumb_component_module_exposes_unified_api() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");

    for needle in [
        "pub use logic::BreadcrumbItem;",
        "pub use view::Breadcrumb;",
        "pub mod styles;",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module_source.contains(needle),
            "breadcrumb component module should export `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_view_accepts_items_and_optional_root_props() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "pub fn Breadcrumb(",
        "items: Vec<BreadcrumbItem>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] separator: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose `{needle}` in public props."
        );
    }
}

#[test]
fn breadcrumb_component_files_follow_responsibility_boundaries() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::BreadcrumbItem;",
        "pub use view::Breadcrumb;",
    ] {
        assert!(
            module_source.contains(needle),
            "mod.rs should keep stable export boundary marker `{needle}`."
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod motion;",
        "mod motion;",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not expose implementation-heavy module token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_root_state(",
        "pub fn resolve_separator(",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "pub fn resolve_item_href(",
        "aria_source_attr",
        "class_source_attr",
        "separator_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "data-slot=",
        ".ui-breadcrumb__",
        "navigation_attrs(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include view/styles/headless mount token `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-breadcrumb__",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep static token-first css marker `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "navigation_attrs(",
        "resolve_state(",
        "resolve_root_state(",
        "CommonStrings",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include runtime logic token `{forbidden}`."
        );
    }

    for needle in [
        "let a11y = navigation_attrs(aria_label, lang, dir);",
        "let state = logic::resolve_state(&items);",
        "let is_current_page = logic::is_current_page(index, item_count);",
        "let href = logic::resolve_item_href(&item, index, item_count);",
        "data-slot=\"breadcrumb\"",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep structure + headless mount marker `{needle}`."
        );
    }
    for forbidden in [
        "use ui_state_primitives",
        "breadcrumbs_primitives::",
        "breadcrumb_primitives::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not directly consume primitives token `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    assert!(
        !workspace_dir
            .join("components/breadcrumb/src/motion.rs")
            .exists(),
        "breadcrumb has no semantic motion axis; motion.rs should stay N/A and absent."
    );
}

#[test]
fn breadcrumb_spec_rs_is_not_introduced_for_simple_component() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    assert!(
        !workspace_dir
            .join("components/breadcrumb/src/spec.rs")
            .exists(),
        "breadcrumb is a simple navigation component; `src/spec.rs` should not exist."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "Spec::new()",
        "BreadcrumbSpec::new(",
    ] {
        assert!(
            !module_source.contains(forbidden) && !protocol_source.contains(forbidden),
            "breadcrumb should not expose builder-spec token `{forbidden}`."
        );
    }

    for needle in [
        "pub enum BreadcrumbComponentSchemaVersion {",
        "pub struct BreadcrumbComponentSpec {",
    ] {
        assert!(
            protocol_source.contains(needle),
            "breadcrumb schema contract should stay in protocol.rs via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_api_naming_contract_has_no_state_alias_drift() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for forbidden in [
        "#[prop(optional)] open: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] default_open:",
        "#[prop(optional)] default_value:",
        "#[prop(optional, into)] on_open_change:",
        "#[prop(optional, into)] on_value_change:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb should not expose stateful alias drift token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_controlled_or_uncontrolled_state_axes() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional, into)] on_value_change:",
        "#[prop(optional, into)] on_open_change:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb should keep stateless API surface and avoid controlled axis token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_default_values_have_single_source_in_logic() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "pub fn resolve_root_state(",
        "let merged_aria_label =",
        "breadcrumb_primitives::normalize_aria_label(merged_aria_label);",
        "breadcrumb_primitives::normalize_optional_text(class_name);",
        "let class_name = if let Some(class_name) = normalized_class_name {",
        "Cow::Borrowed(\"ui-breadcrumb\")",
        "pub fn resolve_separator(",
        "separator_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb defaults should be normalized in logic.rs via `{needle}`."
        );
    }

    for needle in [
        "let logic::BreadcrumbRootState {",
        "} = logic::resolve_root_state(aria_label, Some(aria_label_fallback), class_name);",
        "} = logic::resolve_separator(separator, separator_fallback);",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should only consume normalized state via `{needle}`."
        );
    }

    for forbidden in [
        "aria_label.unwrap_or",
        "class_name.unwrap_or",
        "unwrap_or(",
        "unwrap_or_else(",
        "DEFAULT_ARIA_LABEL",
        "normalize_aria_label(",
        "normalize_optional_text(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view must not add fallback logic token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_state_normalization_is_centralized_in_logic() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");

    for needle in [
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "pub fn is_current_page(item_index: usize, item_count: usize) -> bool",
        "pub fn resolve_item_href(",
        "breadcrumbs_primitives::resolve_state(BreadcrumbsStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb state normalization should stay in logic.rs via `{needle}`."
        );
    }

    for needle in [
        "let state = logic::resolve_state(&items);",
        "let is_current_page = logic::is_current_page(index, item_count);",
        "let href = logic::resolve_item_href(&item, index, item_count);",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-has-current-page=state.has_current_page.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should only consume normalized state via `{needle}`."
        );
    }

    for forbidden in [
        "use ui_state_primitives",
        "BreadcrumbsStateInput",
        "is_last_item(",
        "item.href.as_deref()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view must not rebuild state derivation details `{forbidden}`."
        );
    }

    for forbidden in [
        "resolve_state(",
        "is_current_page(",
        "resolve_item_href(",
        "BreadcrumbsStateInput",
        "ui_state_primitives",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles must not perform state derivation via `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_discrete_state_axes_or_boolean_explosion() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "#[prop(optional, into)] variant: Option<String>",
        "#[prop(optional, into)] size: Option<String>",
        "#[prop(optional, into)] mode: Option<String>",
        "#[prop(optional, into)] status: Option<String>",
        "#[prop(optional)] is_compact: Option<bool>",
        "#[prop(optional)] is_dense: Option<bool>",
        "#[prop(optional)] is_inline: Option<bool>",
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb should not expose untyped discrete axis token `{forbidden}`."
        );
    }

    for forbidden in [
        "match variant",
        "match size",
        "match mode",
        "match status",
        "if is_compact",
        "if is_dense",
        "if is_inline",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not derive state from ad-hoc discrete/boolean token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_consumes_state_primitives_without_business_store_binding() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "use ui_state_primitives::breadcrumb as breadcrumb_primitives;",
        "use ui_state_primitives::breadcrumbs as breadcrumbs_primitives;",
        "breadcrumb_primitives::resolve_root_state(",
        "breadcrumbs_primitives::resolve_state(BreadcrumbsStateInput {",
        "breadcrumbs_primitives::resolve_item_href(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb logic should consume state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "use ui_state_primitives",
        "breadcrumbs_primitives::",
        "breadcrumb_primitives::",
        "RwSignal",
        "Signal<",
        "store",
        "global_state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not own primitive derivation or store binding `{forbidden}`."
        );
    }

    for forbidden in [
        "redux",
        "zustand",
        "mobx",
        "pinia",
        "global_store",
        "app_state",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic must not directly bind app business store token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_async_interaction_contract() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "#[prop(optional)] is_loading:",
        "#[prop(optional)] is_disabled:",
        "#[prop(optional, into)] on_retry:",
        "#[prop(optional, into)] on_error:",
        "aria-busy",
        "data-loading",
        "data-error",
        "disabled=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not expose async interaction token `{forbidden}`."
        );
    }

    for forbidden in [
        "use_async_action",
        "async fn",
        ".await",
        "Future",
        "retry",
        "is_loading",
        "aria_busy",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not implement async protocol token `{forbidden}`."
        );
    }
}

#[test]
fn docs_page_exposes_minimal_breadcrumb_hello_world_path() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for needle in [
        "<Playground\n                title=\"Hello World\"",
        "code_signal=hello_world_code",
        "let hello_world_items = vec![",
        "<Breadcrumb items=hello_world_items_for_hello />",
        "let items = items.get();",
        "<Breadcrumb items=items />",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumb docs should expose minimal hello-world path `{needle}`."
        );
    }

    for forbidden in ["ui_state_primitives::", "ui_headless::", "state="] {
        assert!(
            !source.contains(forbidden),
            "breadcrumb docs hello-world path must not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_composite_api_uses_typed_item_spec_without_parallel_arrays() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let docs = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for needle in [
        "items: Vec<BreadcrumbItem>",
        "pub struct BreadcrumbItem {",
        "pub label: String,",
        "pub href: Option<String>,",
        "<Breadcrumb items=items />",
        "BreadcrumbItem { label:",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle) || docs.contains(needle),
            "Breadcrumb should keep typed item-spec composite API via `{needle}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "children: Vec<",
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional, into)] panels:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb must not expose parallel-array composite API token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_macro_micro_drag_state_machine() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let module_source = load_breadcrumb_component_source("src/mod.rs");

    for forbidden in ["mod motion;", "pub mod motion;"] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb should not expose drag-motion module token `{forbidden}`."
        );
    }

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "on:mousemove",
        "pointermove",
        "request_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not implement drag micro-loop token `{forbidden}`."
        );
    }

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "drag_delta",
        "pointer_delta",
        "drag_offset",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not host drag macro-state token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_two_pass_geometry_rendering_pipeline() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "get_bounding_client_rect",
        "bounding_client_rect",
        "offset_width",
        "offset_height",
        "client_width",
        "client_height",
        "ResizeObserver",
        "IntersectionObserver",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not implement geometry measure stage token `{forbidden}`."
        );
    }

    for forbidden in [
        "Rectification",
        "geometry",
        "placement",
        "anchor_rect",
        "measure_result",
        "popover",
        "tooltip",
        "menu",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not host geometry rectification stage token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_registration_protocol_path() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "IndexSet",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not contain registration protocol token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not contain registration protocol token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "let item_inputs: Vec<_> = items",
        ".iter()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb order should come from typed Vec input via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_slot_projection_policy_path() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "pause_polling",
        "resume_polling",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include slot projection lifecycle token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include slot projection lifecycle token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_env_stream_subscription_path() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "on:resize",
        "match_media",
        "media_query",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include env-stream sampling token `{forbidden}`."
        );
    }

    for forbidden in [
        "BreakpointChanged",
        "EnvAction",
        "env_event",
        "resize_event",
        "intersection_event",
        "theme_changed",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include env-stream projection token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_event_light_cone_batch_selection_path() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Context Bus",
        "context_bus",
        "Selector",
        "SelectionState",
        "SelectionState::All",
        "select_all",
        "batch_select",
        "batch",
        "prop drilling",
        "prop_drilling",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not implement event-light-cone batch selection token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not implement event-light-cone batch selection token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "let item_inputs: Vec<_> = items",
        ".iter()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb should remain linear item mapping without context bus path via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_unified_causality_bus_path() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Causality Bus",
        "CausalityBus",
        "TraceId",
        "trace_id",
        "causality",
        "context_bus",
        "bus_publish",
        "bus_subscribe",
        "broadcast",
        "subscriber",
        "dispatch_with_trace",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include unified causality-bus token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include unified causality-bus token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "let item_inputs: Vec<_> = items",
        ".iter()",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb should remain direct linear mapping without causality bus path via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_focus_stack_or_overlay_restore_path() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Overlay",
        "overlay",
        "NodeRef",
        "node_ref",
        "FocusManager",
        "FallbackTo",
        "Selector",
        "document.body",
        "restore_focus",
        "focus_stack",
        "focus_manager",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include overlay focus-stack token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include overlay focus-stack token `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "data-slot=\"breadcrumb\"",
        "aria-current=\"page\"",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Breadcrumb should remain linear nav semantic path via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_foreign_zone_escape_hatch_path() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "Foreign Zone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "mapbox",
        "Leaflet",
        "leaflet",
        "google.maps",
        "GoogleMap",
        "Amap",
        "AMap",
        "OpenLayers",
        "openlayers",
        "imperative_instance",
        "external_instance",
        "wasm_bindgen",
        "js_sys",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include foreign-zone escape-hatch token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include foreign-zone escape-hatch token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include foreign-zone escape-hatch token `{forbidden}`."
        );
    }

    for needle in [
        "pub struct BreadcrumbItem",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "data-slot=\"breadcrumb\"",
    ] {
        assert!(
            module_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Breadcrumb should remain pure semantic assembly without foreign-zone path via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_has_no_hydration_discontinuity_time_or_random_id_path() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for forbidden in [
        "now()",
        "Instant::now",
        "SystemTime::now",
        "Date::now",
        "Uuid",
        "uuid",
        "new_v4",
        "rand::",
        "thread_rng",
        "random::<",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include hydration-unstable token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include hydration-unstable token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include hydration-unstable token `{forbidden}`."
        );
    }

    for forbidden in [
        "UiIdProvider",
        "provide_ui_id_provider",
        "use_ui_id_provider",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not wire id-provider token `{forbidden}` when breadcrumb has no id axis."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not wire id-provider token `{forbidden}` when breadcrumb has no id axis."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not wire id-provider token `{forbidden}` when breadcrumb has no id axis."
        );
    }

    for needle in [
        "pub struct BreadcrumbItem",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "data-slot=\"breadcrumb\"",
    ] {
        assert!(
            module_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "Breadcrumb should remain deterministic semantic assembly without hydration id bootstrap via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_logic_uses_state_primitives_and_item_mapping() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::breadcrumbs as breadcrumbs_primitives;",
        "pub struct BreadcrumbItem",
        "pub fn resolve_root_state(",
        "pub fn resolve_separator(",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "pub fn is_current_page(item_index: usize, item_count: usize) -> bool",
        "pub fn resolve_item_href(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb logic should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_view_emits_unified_state_markers() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "data-slot=\"breadcrumb\"",
        "data-slot=\"breadcrumb-list\"",
        "data-slot=\"breadcrumb-item\"",
        "data-slot=\"breadcrumb-link\"",
        "data-slot=\"breadcrumb-page\"",
        "data-slot=\"breadcrumb-label\"",
        "data-slot=\"breadcrumb-separator\"",
        "data-aria-source=aria_source_attr",
        "data-separator-source=separator_source_attr",
        "data-class-source=class_source_attr",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-count=state.item_count",
        "aria-current=\"page\"",
        "lang=a11y.lang",
        "dir=a11y.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose `{needle}` for semantic/state inspection."
        );
    }
}

#[test]
fn breadcrumb_view_mounts_headless_navigation_a11y_contract() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let cargo_source = load_breadcrumb_component_source("Cargo.toml");

    for needle in [
        "use ui_headless::{A11yDirection, navigation_attrs};",
        "use ui_headless::{CommonStrings, use_ui_i18n};",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "let aria_label_fallback = common_strings.breadcrumb_aria_label.as_ref();",
        "let separator_fallback = common_strings.breadcrumb_separator.as_ref();",
        "let a11y = navigation_attrs(aria_label, lang, dir);",
        "aria-label=a11y.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should mount headless a11y contract `{needle}`."
        );
    }

    assert!(
        cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "breadcrumb component should depend on ui-headless for a11y contract assembly.",
    );
}

#[test]
fn breadcrumb_a11y_and_i18n_contract_is_headless_backed() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let a11y_source = load_ui_components_source("../ui-headless/src/a11y.rs");
    let i18n_source = load_ui_components_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "aria-label=a11y.aria_label",
        "lang=a11y.lang",
        "dir=a11y.dir",
        "aria-current=\"page\"",
        "data-separator-source=separator_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose a11y/i18n semantic marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_root_state(",
        "aria_label_fallback: Option<&str>",
        "aria_source_attr: if has_custom_aria_label {",
        "\"i18n\"",
        "pub fn resolve_separator(",
        "separator_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb logic should keep i18n fallback/source marker `{needle}`."
        );
    }

    for needle in [
        "pub fn navigation_attrs(",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs",
    ] {
        assert!(
            a11y_source.contains(needle),
            "ui-headless a11y shared tool should include `{needle}`."
        );
    }

    for needle in [
        "pub breadcrumb_aria_label: Arc<str>",
        "pub breadcrumb_separator: Arc<str>",
    ] {
        assert!(
            i18n_source.contains(needle),
            "ui-headless i18n common bundle should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("\"/\""),
        "Breadcrumb view should not hardcode separator text in view.rs.",
    );
}

#[test]
fn breadcrumb_state_markers_are_observable_queryable_and_stable() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    for needle in [
        "data-slot=\"breadcrumb\"",
        "data-slot=\"breadcrumb-list\"",
        "data-slot=\"breadcrumb-item\"",
        "data-slot=\"breadcrumb-link\"",
        "data-slot=\"breadcrumb-page\"",
        "data-slot=\"breadcrumb-label\"",
        "data-slot=\"breadcrumb-separator\"",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-has-current-page=state.has_current_page.then_some(\"true\")",
        "data-count=state.item_count",
        "data-last=is_current_page.then_some(\"true\")",
        "data-href=href_for_attr",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-separator-source=separator_source_attr",
        "aria-current=\"page\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should expose stable observability marker `{needle}`."
        );
    }

    for needle in [
        "aria_source_attr: if has_custom_aria_label {",
        "\"custom\"",
        "\"i18n\"",
        "\"default\"",
        "class_source_attr: primitive_state.class_source_attr",
        "separator_source_attr: \"custom\"",
        "separator_source_attr: \"i18n\"",
        "separator_source_attr: \"default\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "Breadcrumb logic should keep source markers as closed enumerable set token `{needle}`."
        );
    }

    for forbidden in ["open", "expanded", "focus-visible", "loading"] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb has no `{forbidden}` axis; observability should stay scoped to actual state axes.",
        );
    }
}

#[test]
fn breadcrumb_styles_depend_on_explicit_semantic_selectors() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        ".ui-breadcrumb__link:hover",
        ".ui-breadcrumb__link:focus-visible",
        ".ui-breadcrumb__page",
        ".ui-breadcrumb__label",
        ".ui-breadcrumb__separator",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should keep explicit stable selector `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "> .", " + .", " ~ ."] {
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles must not rely on fragile structural selector token `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "set_property(", "cssText", "css_text"] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view must not inject runtime business style token `{forbidden}`."
        );
    }

    for needle in [
        "data-slot=\"breadcrumb-item\"",
        "data-last=is_current_page.then_some(\"true\")",
        "aria-current=\"page\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb visual-state explanation should stay on semantic marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_token_first_static_styles_contract_is_aggregated_and_injected_via_ui_root() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should keep token-first static contract via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-breadcrumb\")]",
        "out.push_str(crate::breadcrumb::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css.rs should aggregate breadcrumb styles via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized component-css injection path `{needle}`."
        );
    }

    assert!(
        root_source.contains("#[prop(optional)] inject_components_css: bool"),
        "UiRoot should expose optional style injection switch for component CSS aggregation."
    );

    for forbidden in [
        "--ui-breadcrumb-",
        "@apply",
        "tailwind",
        "css!",
        "styled!",
        "styled(",
        "emotion",
        "stylex",
        "linaria",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "Breadcrumb should not adopt utility-first/CSS-in-Rust default marker `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let baseline_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs",
    );
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        ".ui-breadcrumb__link:hover {",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        ".ui-breadcrumb__link:focus-visible {",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));",
        ".ui-breadcrumb__page {",
        "font-weight: 500;",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should keep visual-quality marker `{needle}`."
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
            "Theme visual baseline page should keep visual-desire marker `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_source.contains(needle),
            "Docs catalog should expose theme visual baseline route token `{needle}`."
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
fn breadcrumb_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_ui_components_source("Cargo.toml");
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let web_demo_cargo = load_ui_components_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_ui_components_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-breadcrumb = [\"dep:ui-breadcrumb\"]",
        "\"component-breadcrumb\"",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui tree-shaking feature map should include `{needle}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-breadcrumb\")]")
            && lib_source.contains("pub use ui_breadcrumb as breadcrumb;"),
        "lib.rs should feature-gate breadcrumb export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-breadcrumb\")]")
            && css_source.contains("out.push_str(crate::breadcrumb::styles::CSS);"),
        "css.rs should gate breadcrumb CSS aggregation behind component-breadcrumb feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep inject-css top-level gate for component CSS injection."
    );

    for forbidden in ["component_registry", "ALL_COMPONENTS_MAP", "lazy_static!"] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "tree-shaking contract should avoid global keep-alive registries `{forbidden}`."
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components for full docs coverage."
    );
}

#[test]
fn breadcrumb_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_ui_components_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking gate script should include `{needle}`."
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
fn breadcrumb_platform_checks_cover_native_ssr_wasm_and_non_wasm_source_guard() {
    let script_source = load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let cargo_source = load_breadcrumb_component_source("Cargo.toml");
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "cargo check -p ui --no-default-features --features component-breadcrumb,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-breadcrumb,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "components/breadcrumb/src/mod.rs",
        "components/breadcrumb/src/logic.rs",
        "components/breadcrumb/src/styles.rs",
        "components/breadcrumb/src/view.rs",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include breadcrumb platform guard token `{needle}`."
        );
    }

    assert!(
        cargo_source.contains("default = []"),
        "breadcrumb crate should keep empty default features to avoid accidental platform pull-in."
    );

    for forbidden in [
        "web_sys",
        "js_sys",
        "wasm_bindgen",
        "window()",
        "document()",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include non-wasm forbidden browser token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include non-wasm forbidden browser token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles should not include non-wasm forbidden browser token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include non-wasm forbidden browser token `{forbidden}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"web\")]",
        "#[cfg(feature = \"ssr\")]",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include platform split token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include platform split token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles should not include platform split token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include platform split token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_ui_headless_web_ssr_mutex_compile_guard_is_enforced() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let cargo_source = load_breadcrumb_component_source("Cargo.toml");
    let headless_lib_source = load_ui_components_source("../ui-headless/src/lib.rs");
    let script_source = load_ui_components_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "use ui_headless::{A11yDirection, navigation_attrs};",
        "use ui_headless::{CommonStrings, use_ui_i18n};",
        "let a11y = navigation_attrs(aria_label, lang, dir);",
    ] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb view should keep ui-headless integration marker `{needle}`."
        );
    }

    assert!(
        cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "breadcrumb crate should depend on ui-headless via workspace path."
    );

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless mutex guard in lib.rs should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should enforce ui-headless web/ssr mutex contract via `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_ui_motion_non_wasm_stub_contract_is_preserved() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let motion_lib_source = load_ui_components_source("../ui-motion/src/lib.rs");
    let platform_script_source =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should include ui-motion compile/stub guard `{needle}`."
        );
    }

    assert!(
        !workspace_dir
            .join("components/breadcrumb/src/motion.rs")
            .exists(),
        "breadcrumb has no motion axis; `src/motion.rs` should remain absent."
    );

    for forbidden in ["use ui_motion", "ui_motion::", "attach_motion(", "panic!("] {
        assert!(
            !module_source.contains(forbidden),
            "breadcrumb module should not include motion runtime token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not include motion runtime token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles should not include motion runtime token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not include motion runtime token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let platform_script_source =
        load_ui_components_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-breadcrumb__link {",
        "transition: none;",
    ] {
        assert!(
            styles_source.contains(needle),
            "breadcrumb reduced-motion branch should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui --no-default-features --features component-breadcrumb,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-breadcrumb,inject-css",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should include breadcrumb compile-only branch `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "#[cfg(feature = \"web\")]",
        "#[cfg(feature = \"ssr\")]",
        "web_sys",
        "js_sys",
        "wasm_bindgen",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Breadcrumb view should not split semantic contract by platform token `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Breadcrumb logic should not split semantic contract by platform token `{forbidden}`."
        );
    }

    for needle in ["data-slot=\"breadcrumb\"", "aria-current=\"page\""] {
        assert!(
            view_source.contains(needle),
            "Breadcrumb semantic marker `{needle}` should stay stable across reduced-motion/SSR/wasm branches."
        );
    }
}

#[test]
fn breadcrumb_type_system_and_semantic_markers_keep_machine_readable_contract() {
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let primitive_source = load_ui_components_source("../ui-state-primitives/src/breadcrumbs.rs");
    let logic_test_source = load_breadcrumb_component_source("test/logic.rs");

    for needle in [
        "pub struct BreadcrumbItem {",
        "pub label: String,",
        "pub href: Option<String>,",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "BreadcrumbsItemInput {",
        "BreadcrumbsStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "typed input/state contract should include `{needle}`."
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "#[prop(optional)] is_compact:",
        "#[prop(optional)] is_dense:",
        "#[prop(optional)] is_inline:",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Breadcrumb should avoid untyped discrete axis token `{forbidden}`."
        );
    }

    for needle in [
        "data-slot=\"breadcrumb\"",
        "data-slot=\"breadcrumb-item\"",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-links=state.has_links.then_some(\"true\")",
        "data-has-current-page=state.has_current_page.then_some(\"true\")",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-separator-source=separator_source_attr",
        "data-last=is_current_page.then_some(\"true\")",
        "aria-current=\"page\"",
    ] {
        assert!(
            view_source.contains(needle),
            "machine-readable semantic marker contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct BreadcrumbsItemInput<'a> {",
        "pub struct BreadcrumbsStateInput<'a> {",
        "pub fn resolve_item_href(",
        "if item_index >= item_count || is_last_item(item_index, item_count) {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should keep normalization boundary token `{needle}`."
        );
    }

    for needle in [
        "fn resolve_state_ignores_blank_and_last_item_links()",
        "fn state_source_attrs_are_closed_enumerations()",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "logic tests should keep contract regression case `{needle}`."
        );
    }

    for needle in [
        "pub enum BreadcrumbComponentSchemaVersion {",
        "pub struct BreadcrumbComponentSpec {",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol should keep typed machine-readable schema token `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_semantics_contract_tests_prioritize_contracts_over_snapshots() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let local_semantics_source = load_breadcrumb_component_source("test/semantics.rs");
    let workspace_semantics_source =
        load_ui_components_source("../../components/breadcrumb/test/semantics.rs");

    for needle in [
        "aria-current=\"page\"",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-separator-source=separator_source_attr",
        "data-last=is_current_page.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "semantic contract marker `{needle}` should be asserted by tests."
        );
    }

    for needle in [
        "fn breadcrumb_has_no_controlled_or_uncontrolled_state_axes()",
        "fn breadcrumb_has_no_async_interaction_contract()",
        "fn breadcrumb_state_markers_are_observable_queryable_and_stable()",
        "fn breadcrumb_a11y_and_i18n_contract_is_headless_backed()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "component semantic test matrix should include `{needle}`."
        );
        assert!(
            workspace_semantics_source.contains(needle),
            "workspace semantic test matrix should include `{needle}`."
        );
    }

    let forbidden_snapshot_calls = [
        ["assert", "_snapshot", "("].concat(),
        ["to_match", "_snapshot", "("].concat(),
        ["insta::assert", "_snapshot", "("].concat(),
        ["snapshot", "!("].concat(),
    ];
    for forbidden in forbidden_snapshot_calls {
        assert!(
            !local_semantics_source.contains(forbidden.as_str()),
            "component semantic tests must not depend on snapshot assertion token `{forbidden}`."
        );
        assert!(
            !workspace_semantics_source.contains(forbidden.as_str()),
            "workspace semantic tests must not depend on snapshot assertion token `{forbidden}`."
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:pointerdown",
        "on:pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "breadcrumb has no component-level keyboard/pointer handler token `{forbidden}`; this axis is N/A."
        );
    }
}

#[test]
fn breadcrumb_semantics_suite_is_contract_first_not_snapshot_only() {
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let local_semantics_source = load_breadcrumb_component_source("test/semantics.rs");
    let workspace_semantics_source = load_ui_components_source("tests/breadcrumb_semantics.rs");

    for marker in [
        "<nav",
        "aria-label=a11y.aria_label",
        "aria-current=\"page\"",
        "data-slot=\"breadcrumb\"",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-separator-source=separator_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "breadcrumb semantic contract should expose `{marker}` for contract-first assertions."
        );
    }

    for marker in [
        "fn breadcrumb_semantics_contract_tests_prioritize_contracts_over_snapshots()",
        "fn breadcrumb_state_markers_are_observable_queryable_and_stable()",
        "fn breadcrumb_a11y_and_i18n_contract_is_headless_backed()",
        "fn breadcrumb_semantics_suite_is_contract_first_not_snapshot_only()",
    ] {
        assert!(
            local_semantics_source.contains(marker) && workspace_semantics_source.contains(marker),
            "local/workspace semantics suites should both contain marker `{marker}`."
        );
    }

    let forbidden_snapshot_calls = [
        ["assert", "_snapshot", "("].concat(),
        ["to_match", "_snapshot", "("].concat(),
        ["insta::assert", "_snapshot", "("].concat(),
        ["snapshot", "!("].concat(),
    ];
    for forbidden in forbidden_snapshot_calls {
        assert!(
            !local_semantics_source.contains(forbidden.as_str())
                && !workspace_semantics_source.contains(forbidden.as_str()),
            "semantic suites must avoid snapshot-only token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_semantics_first_testing_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "语义优先断言：本地与聚合测试均显式禁止快照断言依赖（`assert_snapshot/to_match_snapshot/insta::assert_snapshot/snapshot!`），并要求语义字段变化时回归测试同步更新。",
        "回归锁定：`components/breadcrumb/test/semantics.rs::breadcrumb_check2_documents_semantics_first_testing_rules`",
        "components/breadcrumb/test/semantics.rs::breadcrumb_semantics_contract_tests_prioritize_contracts_over_snapshots",
        "components/breadcrumb/test/semantics.rs::breadcrumb_semantics_suite_is_contract_first_not_snapshot_only",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 semantics-first section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_contract_hygiene_script_covers_semantics_first_testing_rules() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "echo \"[contract-hygiene] contract: breadcrumb semantics priority asserts role/aria/data-source before snapshots\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_semantics_suite_is_contract_first_not_snapshot_only",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_styles_cover_core_accessibility_and_structure_contracts() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");

    for needle in [
        ".ui-breadcrumb {",
        ".ui-breadcrumb__list",
        ".ui-breadcrumb__item",
        ".ui-breadcrumb__link",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "--ui-button-focus-outline-offset,",
        ".ui-breadcrumb__label",
        ".ui-breadcrumb__page",
        ".ui-breadcrumb__separator",
        "@media (forced-colors: active)",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should include `{needle}`."
        );
    }

    assert!(
        !styles_source.contains("transition: color 180ms ease;"),
        "Breadcrumb styles should not hardcode motion constants in transition rules.",
    );
    for forbidden in [
        "currentColor",
        "var(--ui-fg-subtle",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "var(--ui-text-field-motion-duration, 180ms)",
        "var(--ui-text-field-motion-easing, ease)",
        "outline: 3px solid",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles should not include hardcoded terminal token `{forbidden}`.",
        );
    }
    assert!(
        !styles_source.contains("--ui-breadcrumb-link-motion-duration"),
        "Breadcrumb should consume theme motion tokens directly instead of defining parallel private motion token names.",
    );
    assert!(
        !styles_source.contains("--ui-breadcrumb-link-motion-easing"),
        "Breadcrumb should consume theme motion tokens directly instead of defining parallel private motion token names.",
    );
    assert!(
        !styles_source.contains("--ui-breadcrumb-"),
        "Breadcrumb should not introduce a private component token namespace in styles.rs; consume shared ui-theme variables instead.",
    );
}

#[test]
fn docs_page_uses_unified_breadcrumb_api() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for needle in [
        "pub(super) fn breadcrumb() -> AnyView",
        "title=\"Breadcrumb\"",
        "slug=\"breadcrumb\"",
        "<Breadcrumb",
        "items=items",
    ] {
        assert!(
            source.contains(needle),
            "breadcrumb docs page should include `{needle}`."
        );
    }
}

#[test]
fn docs_navigation_no_longer_lists_breadcrumbs_or_primitives() {
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        !pages_source.contains("\"breadcrumbs\""),
        "components pages should no longer expose a separate breadcrumbs route."
    );
    assert!(
        !pages_source.contains("\"breadcrumb-list\""),
        "components pages should no longer expose breadcrumb primitive routes."
    );
}

#[test]
fn breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let shell_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_ui_components_source("../../apps/docs-app/src/perf_probe.rs");
    let perf_script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");
    let accordion_semantics_source = load_ui_components_source("tests/accordion_semantics.rs");
    let component_semantics_source = load_breadcrumb_component_source("test/semantics.rs");

    for needle in [
        "<ComponentPage",
        "slug=\"breadcrumb\"",
        "title=\"Breadcrumb\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs page should stay on shared component-page perf path via `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
        "let perf_budget = component_page_perf_budget(slug);",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep shared perf budget/probe wiring token `{needle}`."
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
            "UiPerfProbe should expose observability marker `{needle}` for triage."
        );
    }

    for needle in [
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
        "breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance script should include blocking contract token `{needle}`."
        );
    }

    for needle in [
        "fn perf_render_count_follow_up_is_tracked_in_plan()",
        "render_count",
    ] {
        assert!(
            accordion_semantics_source.contains(needle),
            "shared performance governance should keep render_count follow-up token `{needle}`."
        );
        assert!(
            component_semantics_source.contains(needle),
            "breadcrumb component semantics should mirror render_count follow-up governance token `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_view_macro_complexity_is_split_into_local_render_helpers() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "fn render_breadcrumb_separator(",
        "fn render_breadcrumb_item_content(",
        "fn render_breadcrumb_item(",
        "fn render_breadcrumb_list(",
        "render_breadcrumb_list(items, item_count, separator_text.into_owned())",
        "render_breadcrumb_item(index, item, item_count, separator_text.clone())",
    ] {
        assert!(
            view_source.contains(needle),
            "breadcrumb view should keep macro complexity split marker `{needle}`."
        );
    }

    for forbidden in [
        "let separator = (!is_current_page).then_some(view! {",
        "let content: AnyView = if is_current_page {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "breadcrumb view should not regress to giant nested macro token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_prefers_functional_splitting_without_local_component_noise() {
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "fn render_breadcrumb_separator(",
        "fn render_breadcrumb_item_content(",
        "fn render_breadcrumb_item(",
        "fn render_breadcrumb_list(",
    ] {
        assert!(
            view_source.contains(needle),
            "breadcrumb should keep lightweight view fragments as plain functions via `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "breadcrumb view should keep only one public component entry point."
    );
    assert!(
        view_source.contains("#[component]\npub fn Breadcrumb("),
        "breadcrumb should expose only the root component while helpers stay plain functions."
    );

    for forbidden in [
        "#[component]\nfn render_breadcrumb_separator(",
        "#[component]\nfn render_breadcrumb_item_content(",
        "#[component]\nfn render_breadcrumb_item(",
        "#[component]\nfn render_breadcrumb_list(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "breadcrumb helper should not be promoted to local component token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_static_fragments_are_constantized_or_marked_not_applicable() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "breadcrumb static visual fragment should stay centralized in styles.rs CSS constant."
    );

    for needle in [
        ".ui-breadcrumb {",
        ".ui-breadcrumb__list",
        ".ui-breadcrumb__item",
        ".ui-breadcrumb__link",
        ".ui-breadcrumb__separator",
    ] {
        assert!(
            styles_source.contains(needle),
            "static style fragment `{needle}` should live in styles.rs CSS constant."
        );
    }

    for forbidden in ["<svg", "inner_html=", "include_str!("] {
        assert!(
            !view_source.contains(forbidden),
            "breadcrumb view should not construct heavy static fragment token `{forbidden}` at runtime."
        );
    }
}

#[test]
fn breadcrumb_inner_html_contract_is_n_a_and_docs_shell_is_static_whitelist_only() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let docs_shell_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/shell.rs");

    let forbidden = "inner_html=";
    assert!(
        !module_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden)
            && !docs_page_source.contains(forbidden),
        "breadcrumb component surface should not use `{forbidden}`."
    );

    for needle in [
        "fn component_readme_markdown(slug: &str) -> Option<&'static str>",
        "include_str!(\"../../../../../components/accordion/src/README.md\")",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "inner_html=html",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell inner_html path should stay in static-whitelist contract via `{needle}`."
        );
    }

    for forbidden in ["reqwest", "fetch(", "http://", "https://"] {
        assert!(
            !docs_shell_source.contains(forbidden),
            "docs shell should not source inner_html payload from remote/user token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_wasm_debug_contract_is_n_a_and_playground_surface_is_stable() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let cargo_source = load_breadcrumb_component_source("Cargo.toml");
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for forbidden in [
        "trace_id",
        "Replay",
        "replay",
        "debug_trace",
        "debug_panel",
        "record_event",
        "event_log",
        "tracing::",
        "web_sys::console",
        "console_log",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "breadcrumb component should not leak wasm debug runtime token `{forbidden}` into public surface."
        );
    }

    for needle in ["[features]", "default = []"] {
        assert!(
            cargo_source.contains(needle),
            "breadcrumb crate should keep feature-isolated baseline token `{needle}`."
        );
    }
    for forbidden in ["debug", "devtools", "trace-replay", "wasm-debug"] {
        assert!(
            !cargo_source.contains(forbidden),
            "breadcrumb crate should not publish debug feature token `{forbidden}` by default."
        );
    }

    for needle in [
        "<Playground\n                title=\"Trail\"",
        "let actual_config = Signal::derive(",
        "test_config_signal=actual_config",
        "selected_index=scenario_index",
        "set_selected_index=set_scenario_index",
        "checked=custom_aria_label",
        "set_checked=set_custom_aria_label",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs playground should keep observable/reproducible debug surface token `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_dx_contract_uses_scoped_playground_for_hot_style_and_context_retention() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "<Playground\n                title=\"Trail\"",
        "test_css_source=test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/breadcrumb/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs should keep DX playground contract token `{needle}`."
        );
    }

    for needle in [
        "let (scenario_index, set_scenario_index) = signal(Some(0_usize));",
        "let (custom_aria_label, set_custom_aria_label) = signal(false);",
        "selected_index=scenario_index",
        "set_selected_index=set_scenario_index",
        "checked=custom_aria_label",
        "set_checked=set_custom_aria_label",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs should preserve interaction context via `{needle}`."
        );
    }

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground should provide scoped hot-style editing token `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_engineering_contract_keeps_serde_protocol_and_runtime_agnostic_api() {
    let module_source = load_breadcrumb_component_source("src/mod.rs");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let cargo_source = load_breadcrumb_component_source("Cargo.toml");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum BreadcrumbComponentSchemaVersion {",
        "V1,",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "pub struct BreadcrumbComponentSpec {",
        "#[serde(default)]",
        "pub schema_version: BreadcrumbComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "breadcrumb protocol should keep structured serde/version contract token `{needle}`."
        );
    }

    for needle in [
        "pub mod protocol;",
        "pub use logic::BreadcrumbItem;",
        "pub use view::Breadcrumb;",
    ] {
        assert!(
            module_source.contains(needle),
            "breadcrumb public API boundary should keep token `{needle}`."
        );
    }

    for forbidden in [
        "tracing::",
        "span!(",
        "event!(",
        "tokio::",
        "async_std::",
        "tokio-",
        "async-std",
        "JoinHandle",
        "Runtime",
        "runtime::",
        "spawn(",
        "async fn",
        ".await",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !cargo_source.contains(forbidden),
            "breadcrumb component should not leak runtime/tracing token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_defensive_variables_use_two_level_theme_fallback_chain() {
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let theme_css_source = load_ui_components_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "--ui-button-focus-outline-offset,",
        "var(--ui-fallback-button-focus-outline-offset)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Breadcrumb styles should keep defensive double-fallback token `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-button-focus-outline-offset:",
        "--ui-fallback-text-field-motion-duration:",
        "--ui-fallback-text-field-motion-easing:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme should stay SSOT for fallback terminal token `{needle}`."
        );
    }

    for forbidden in [
        "currentColor",
        "var(--ui-fg-subtle",
        "var(--ui-space-xs)",
        "var(--ui-radius-sm)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "var(--ui-text-field-motion-duration, 180ms)",
        "var(--ui-text-field-motion-easing, ease)",
        "outline: 3px solid",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Breadcrumb styles should not keep hardcoded fallback terminal token `{forbidden}`.",
        );
    }
}

#[test]
fn breadcrumb_css_is_aggregated_into_ui_layer_without_inline_style_overrides() {
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-breadcrumb\")]",
        "out.push_str(crate::breadcrumb::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(needle),
            "components CSS aggregation should keep `@layer ui` contract token `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized CSS-layer injection token `{needle}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style=",
        "style:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "breadcrumb view should not inject plain inline style token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_ui_components_fixed_entry_files_contract_is_preserved() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");
    let active_highlight_source =
        load_ui_components_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "#[cfg(feature = \"component-breadcrumb\")]",
        "pub use ui_breadcrumb as breadcrumb;",
        "pub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib.rs fixed entry contract should include `{needle}`."
        );
    }
    for forbidden in ["pub use web_sys", "pub use wasm_bindgen", "pub use js_sys"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui public API should not leak platform detail token `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-breadcrumb\")]",
        "out.push_str(crate::breadcrumb::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css.rs fixed entry contract should include `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "ui root.rs fixed entry contract should include `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "ui_motion::spring::SpringConfig",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "ui-visual-primitive active_highlight entry should include `{needle}`."
        );
    }
    for forbidden in ["breadcrumb", "accordion", "tabs", "menu"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component-specific token `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    assert!(
        !workspace_dir
            .join("crates/ui/src/overlay_open.rs")
            .exists(),
        "ui should not define overlay_open.rs; open-state primitive belongs to ui-headless controllable_state."
    );
    assert!(
        !workspace_dir
            .join("crates/ui/src/presence.rs")
            .exists(),
        "ui should not define presence.rs; presence primitive belongs to ui-headless."
    );
    assert!(
        !workspace_dir
            .join("crates/ui/src/a11y.rs")
            .exists(),
        "ui should not define a11y.rs; shared a11y tools belong to ui-headless."
    );

    assert!(
        workspace_dir
            .join("crates/ui-headless/src/controllable_state.rs")
            .exists(),
        "ui-headless controllable_state primitive should exist as open-state source of truth."
    );
    assert!(
        workspace_dir
            .join("crates/ui-headless/src/presence.rs")
            .exists(),
        "ui-headless presence primitive should exist as source of truth."
    );
    assert!(
        workspace_dir
            .join("crates/ui-headless/src/a11y.rs")
            .exists(),
        "ui-headless a11y shared tools should exist as source of truth."
    );
}

#[test]
fn breadcrumb_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_source = load_breadcrumb_component_source("src/Component.toml");
    let rbi_source = load_breadcrumb_component_source("src/breadcrumb.rbi");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Breadcrumb\"",
        "crate = \"ui-breadcrumb\"",
        "name = \"items[].label\"",
        "name = \"items[].href\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"separator\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"snapshot_rendering\"",
        "name = \"streaming_optional_snapshot_fallback\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "breadcrumb Component.toml should contain manifest marker `{needle}`."
        );
    }

    for needle in [
        "pub struct BreadcrumbItem {",
        "pub label: String,",
        "pub href: Option<String>,",
        "pub enum BreadcrumbComponentSchemaVersion {",
        "pub struct BreadcrumbComponentSpec {",
        "pub fn Breadcrumb(",
        "items: Vec<BreadcrumbItem>",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "separator: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "breadcrumb.rbi should contain signature projection marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] separator: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should include projected public input marker `{needle}`."
        );
    }

    for needle in [
        "pub enum BreadcrumbComponentSchemaVersion {",
        "pub struct BreadcrumbComponentSpec {",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep schema/version contract marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let manifest_source = load_breadcrumb_component_source("src/Component.toml");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");

    for needle in [
        "name = \"agent_contract_schema\"",
        "data-ui-schema",
        "data-ui-schema-version",
        "data-ui-intent",
        "data-ui-action",
        "data-ui-state",
        "data-ui-source",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should include Agent Contract marker `{needle}`."
        );
    }

    for needle in [
        "pub const BREADCRUMB_AGENT_SCHEMA_NAME: &str = \"ui.breadcrumb.agent-contract\";",
        "pub enum BreadcrumbAgentSchemaVersion {",
        "pub enum BreadcrumbAgentIntent {",
        "pub enum BreadcrumbAgentAction {",
        "pub enum BreadcrumbAgentState {",
        "pub enum BreadcrumbAgentSource {",
        "pub struct BreadcrumbAgentContract {",
        "pub const fn as_str(self) -> &'static str {",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should expose typed Agent Contract schema marker `{needle}`."
        );
    }

    for needle in [
        "fn parse_source_attr(value: &str) -> BreadcrumbSourceAttr {",
        "\"custom\" => BreadcrumbSourceAttr::Custom,",
        "\"i18n\" => BreadcrumbSourceAttr::I18n,",
        "\"default\" => BreadcrumbSourceAttr::Default,",
        "_ => BreadcrumbSourceAttr::Unknown,",
        "pub fn resolve_agent_state(state: &BreadcrumbsState) -> BreadcrumbAgentState {",
        "pub fn resolve_agent_source(",
        "pub fn resolve_agent_contract(",
        "schema_name: BREADCRUMB_AGENT_SCHEMA_NAME,",
        "state: resolve_agent_state(state),",
        "source: resolve_agent_source(aria_source_attr, class_source_attr, separator_source_attr),",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep typed/whitelisted agent contract mapping marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = logic::resolve_agent_contract(",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount Agent Contract marker `{needle}`."
        );
    }

    for forbidden in ["inner_html=", "javascript:", "<script", "eval("] {
        assert!(
            !view_source.contains(forbidden),
            "agent contract rendering path must stay in whitelist-only boundary without `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_streaming_definition_is_llm_output_only_with_two_modes() {
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in [
        "pub enum BreadcrumbAgentRenderMode {",
        "Streaming,",
        "Snapshot,",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "pub struct BreadcrumbAgentContract {",
        "pub render_mode: BreadcrumbAgentRenderMode,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should type render mode contract marker `{needle}`."
        );
    }

    for needle in [
        "render_mode: BreadcrumbAgentRenderMode::Snapshot,",
        "data-ui-render-mode=agent_contract.render_mode.as_str()",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "logic/view should expose render mode marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2.md should explicitly lock LLM streaming two-mode definition `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_snapshot_is_foundational_and_complete_config_path_is_stable() {
    let manifest_source = load_breadcrumb_component_source("src/Component.toml");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in ["name = \"snapshot_rendering\"", "data-ui-render-mode"] {
        assert!(
            manifest_source.contains(needle),
            "Component.toml should lock snapshot baseline marker `{needle}`."
        );
    }

    for needle in [
        "pub enum BreadcrumbAgentRenderMode {",
        "#[default]",
        "Snapshot,",
        "Self::Snapshot => \"snapshot\"",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep snapshot baseline type marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_root_state(",
        "pub fn resolve_separator(",
        "pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState",
        "pub fn resolve_agent_contract(",
        "render_mode: BreadcrumbAgentRenderMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep snapshot-capable full-config assembly marker `{needle}`."
        );
    }

    for needle in [
        "items: Vec<BreadcrumbItem>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] separator: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "data-ui-render-mode=agent_contract.render_mode.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should consume complete config and expose snapshot render marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "check2.md should mark snapshot foundational capability as checked."
    );
}

#[test]
fn breadcrumb_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status() {
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in [
        "pub enum BreadcrumbAgentStreamSupport {",
        "Required,",
        "Optional,",
        "Self::Required => \"required\"",
        "Self::Optional => \"optional\"",
        "pub enum BreadcrumbAgentStreamFallback {",
        "Self::Snapshot => \"snapshot\"",
        "pub enum BreadcrumbAgentOutputStatus {",
        "Draft,",
        "Verified,",
        "Submittable,",
        "Self::Draft => \"draft\"",
        "Self::Verified => \"verified\"",
        "Self::Submittable => \"submittable\"",
        "pub stream_support: BreadcrumbAgentStreamSupport,",
        "pub stream_fallback: BreadcrumbAgentStreamFallback,",
        "pub output_status: BreadcrumbAgentOutputStatus,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep streaming policy type marker `{needle}`."
        );
    }

    for needle in [
        "stream_support: BreadcrumbAgentStreamSupport::Optional,",
        "stream_fallback: BreadcrumbAgentStreamFallback::Snapshot,",
        "output_status: BreadcrumbAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep optional streaming snapshot fallback/output marker `{needle}`."
        );
    }

    for needle in [
        "aria-label=a11y.aria_label",
        "data-slot=\"breadcrumb\"",
        "data-ui-stream-support=agent_contract.stream_support.as_str()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_str()",
        "data-ui-output-status=agent_contract.output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should expose continuous aria/data markers for streaming policy via `{needle}`."
        );
    }

    for forbidden in ["retry", "reconnect", "validation_error", "transport_error"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "component layer should not absorb upper-layer resilience token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2.md should document streaming optional policy boundary marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in [
        "const BREADCRUMB_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui::{Breadcrumb, BreadcrumbItem};",
        "<Playground\n                title=\"Hello World\"",
        "title=\"State Matrix (Linked / Label-only / Empty)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "Breadcrumb has no internal controlled/uncontrolled runtime axis.",
        "requested mode:",
        "effective markers: data-ui-render-mode=snapshot data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified",
        "data-slot=\"breadcrumb-state-matrix\"",
        "data-slot=\"breadcrumb-source-first\"",
        "component-breadcrumb",
        "inject-css",
        "code_imports=BREADCRUMB_DOC_IMPORTS.to_string()",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs product surface should include `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "class_name=\"ui-code-block__copy-button\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy-ready pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb check2 docs-product section should reference `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_docs_product_copy_paste_ready_item_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "breadcrumb check2 should mark docs-product copy-paste-ready item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs::breadcrumb",
        "BREADCRUMB_DOC_IMPORTS",
        "breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "breadcrumb_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb check2 docs-product section should reference `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: breadcrumb docs product copy-paste-ready + streaming/snapshot + source-first imports\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-product contract marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 docs-sync/state-matrix section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");

    breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot();

    for needle in [
        "pub(super) fn breadcrumb() -> AnyView",
        "title=\"Trail\"",
        "title=\"State Matrix (Linked / Label-only / Empty)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "let scenario_options = vec![",
        "\"trail\".to_string()",
        "\"label_only\".to_string()",
        "\"empty\".to_string()",
        "let (custom_aria_label, set_custom_aria_label) = signal(false);",
        "let (scenario_index, set_scenario_index) = signal(Some(0_usize));",
        "aria_label=\"Documentation navigation\".to_string()",
        "data-slot=\"breadcrumb-state-matrix\"",
        "data-slot=\"breadcrumb-state-linked\"",
        "data-slot=\"breadcrumb-state-label-only\"",
        "data-slot=\"breadcrumb-state-empty\"",
        "<Breadcrumb items=items />",
        "class: \\\"ui-breadcrumb\\\",",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "breadcrumb docs should keep docs-sync/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] separator: Option<String>",
        "pub const DEFAULT_SEPARATOR: &str = \"/\";",
        "Cow::Borrowed(\"ui-breadcrumb\")",
        "separator_source_attr: \"default\"",
        "resolve_root_state(aria_label, Some(aria_label_fallback), class_name);",
        "resolve_separator(separator, separator_fallback);",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "breadcrumb API/default contract should keep `{needle}`."
        );
    }

    for forbidden in [
        "ariaLabel=",
        "className=",
        "separatorText=",
        "default_separator=",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !docs_page_source.contains(forbidden),
            "breadcrumb docs should avoid API alias-drift token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_documentation_as_product_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb check2 documentation-as-product section should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_documentation_entry_exists_with_beginner_first_progression() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let has_readme = workspace_dir
        .join("components/breadcrumb/src/README.md")
        .exists();
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let catalog_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs",
    );
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        has_readme || docs_page_source.contains("pub(super) fn breadcrumb() -> AnyView"),
        "breadcrumb should provide README or equivalent docs-app entry."
    );

    for needle in [
        "pub(super) const BREADCRUMB_DOC: ComponentDoc = ComponentDoc {",
        "name: \"Breadcrumb\"",
        "slug: \"breadcrumb\"",
        "group: \"Collections\"",
        "page: super::collections_breadcrumb::breadcrumb,",
        "collections_breadcrumb_catalog::BREADCRUMB_DOC,",
        "title=\"Hello World\"",
        "title=\"Trail\"",
        "title=\"State Matrix (Linked / Label-only / Empty)\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
    ] {
        assert!(
            catalog_source.contains(needle)
                || pages_source.contains(needle)
                || docs_page_source.contains(needle),
            "breadcrumb docs entry should include `{needle}`."
        );
    }

    let hello_world_pos = docs_page_source
        .find("title=\"Hello World\"")
        .expect("breadcrumb docs should include hello-world section");
    let trail_pos = docs_page_source
        .find("title=\"Trail\"")
        .expect("breadcrumb docs should include common-usage trail section");
    let state_matrix_pos = docs_page_source
        .find("title=\"State Matrix (Linked / Label-only / Empty)\"")
        .expect("breadcrumb docs should include state matrix section");
    let source_first_pos = docs_page_source
        .find("title=\"Source-first Starter (Copy-Paste Ready)\"")
        .expect("breadcrumb docs should include source-first advanced section");
    assert!(
        hello_world_pos < trail_pos
            && trail_pos < state_matrix_pos
            && state_matrix_pos < source_first_pos,
        "breadcrumb docs should keep beginner-first progression order (hello -> common -> state matrix -> advanced)."
    );

    docs_page_exposes_minimal_breadcrumb_hello_world_path();
}

#[test]
fn breadcrumb_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: breadcrumb documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "breadcrumb check2 should mark documentation-as-product item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
        "apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs",
        "apps/docs-app/src/pages/components/pages.rs",
        "title=\"Hello World\"",
        "title=\"Trail\"",
        "breadcrumb_check2_documents_documentation_as_product_rules",
        "breadcrumb_documentation_entry_exists_with_beginner_first_progression",
        "breadcrumb_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb check2 documentation-as-product section should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_interactive_playground_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 联动示例 N/A（`Breadcrumb` 非 AI Spec 输入组件）",
        "breadcrumb_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "breadcrumb_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 interactive-playground section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "title=\"Trail\"",
        "test_config_signal=actual_config",
        "test_css_source=test_css_source",
        "controls=move || view!",
        "SegmentedControl",
        "selected_index=scenario_index",
        "set_selected_index=set_scenario_index",
        "Switch checked=custom_aria_label set_checked=set_custom_aria_label",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"breadcrumb-streaming-contract\"",
        "data-requested-stream-mode=move || requested_stream_mode.get()",
        "selected_index=stream_mode_index",
        "set_selected_index=set_stream_mode_index",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "breadcrumb docs interactive playground should include `{marker}`."
        );
    }

    for marker in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "Card class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview marker `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_breadcrumb_contract.spec.mjs");
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for marker in [
        "docs-app breadcrumb key flow is repeatable with semantic breakpoints",
        "[data-slot=\"breadcrumb-streaming-contract\"]",
        "[data-slot=\"segmented-control-option\"][data-index=\"0\"]",
        "[data-slot=\"segmented-control-option\"][data-index=\"1\"]",
        "for (const cycle of [1, 2]) {",
        "breadcrumb repeatable key flow cycle ${cycle}",
        "toHaveAttribute(\"data-requested-stream-mode\", \"streaming\")",
        "toHaveAttribute(\"data-requested-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-render-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "breadcrumb interactive-playground e2e flow should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"breadcrumb-streaming-contract\"",
        "id_base=\"docs-breadcrumb-stream-mode\".to_string()",
        "data-requested-stream-mode=move || requested_stream_mode.get()",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "breadcrumb docs should expose stable interactive anchor `{marker}` for repeatable e2e replay."
        );
    }
}

#[test]
fn breadcrumb_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "echo \"[dx] contract: breadcrumb interactive playground docs acceptance surface\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include interactive-playground marker `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "breadcrumb check2 should mark interactive-playground item complete."
    );

    for marker in [
        "title=\"Trail\"",
        "test_config_signal=actual_config",
        "data-slot=\"breadcrumb-streaming-contract\"",
        "AI Spec 联动示例 N/A（`Breadcrumb` 非 AI Spec 输入组件）",
        "breadcrumb_check2_documents_interactive_playground_rules",
        "breadcrumb_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "breadcrumb_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "breadcrumb_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "e2e/tests/docs_app_breadcrumb_contract.spec.mjs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "breadcrumb check2 interactive-playground section should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 source-first copy-paste-ready section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );
    let playground_source = load_ui_components_source("../../apps/docs-app/src/playground.rs");

    breadcrumb_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot();

    for marker in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=BREADCRUMB_DOC_IMPORTS.to_string()",
        "data-slot=\"breadcrumb-source-first\"",
        "data-slot=\"breadcrumb-source-first-contract\"",
        "data-slot=\"breadcrumb-source-first-dependency-baseline\"",
        "data-slot=\"breadcrumb-source-paths\"",
        "data-slot=\"breadcrumb-source-prerequisites\"",
        "docs entry: apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs::breadcrumb",
        "ui = { default-features = false, features = [\\\"component-breadcrumb\\\", \\\"inject-css\\\"] }",
        "components/breadcrumb/src/mod.rs",
        "components/breadcrumb/src/logic.rs",
        "components/breadcrumb/src/view.rs",
        "components/breadcrumb/src/styles.rs",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "breadcrumb source-first docs should include `{marker}`."
        );
    }

    for marker in [
        "code_imports: Option<String>",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "<CodeBlock code=resolved_code.get() />",
        "class_name=\"ui-code-block__copy-button\"",
    ] {
        assert!(
            playground_source.contains(marker),
            "playground source-first copy path should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "echo \"[dx] contract: breadcrumb source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include source-first copy-paste-ready marker `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "breadcrumb check2 should mark source-first copy-paste-ready item complete."
    );

    for marker in [
        "`Source-first Starter (Copy-Paste Ready)`",
        "code_imports=BREADCRUMB_DOC_IMPORTS.to_string()",
        "data-slot=\"breadcrumb-source-first\"",
        "components/breadcrumb/src/mod.rs",
        "components/breadcrumb/src/logic.rs",
        "components/breadcrumb/src/view.rs",
        "components/breadcrumb/src/styles.rs",
        "breadcrumb_check2_documents_source_first_copy_paste_ready_rules",
        "breadcrumb_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "breadcrumb_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "breadcrumb check2 source-first copy-paste-ready section should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 heroui-benchmark docs-sync section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let catalog_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs",
    );
    let docs_page_source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
    );

    for marker in [
        "### Breadcrumb 同步记录（2026-02-20）",
        "参数模型同步：`Breadcrumb` 参数主轴保持 `items/aria_label/class_name/separator/lang/dir`",
        "`components/breadcrumb/src/logic.rs::resolve_root_state + resolve_separator`",
        "`apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs::BREADCRUMB_DOC`",
        "`apps/docs-app/src/pages/components/pages.rs` 收录 `collections_breadcrumb_catalog::BREADCRUMB_DOC`",
        "`apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs::breadcrumb()`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include breadcrumb synchronization marker `{marker}`."
        );
    }

    let marker = "collections_breadcrumb_catalog::BREADCRUMB_DOC,";
    assert!(
        pages_source.contains(marker),
        "component docs index should expose breadcrumb entry marker `{marker}`."
    );

    for marker in [
        "pub(super) const BREADCRUMB_DOC: ComponentDoc = ComponentDoc {",
        "name: \"Breadcrumb\"",
        "slug: \"breadcrumb\"",
        "group: \"Collections\"",
        "page: super::collections_breadcrumb::breadcrumb,",
    ] {
        assert!(
            catalog_source.contains(marker),
            "breadcrumb catalog should stay indexable via marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn breadcrumb() -> AnyView {",
        "title=\"Breadcrumb\"",
        "slug=\"breadcrumb\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "breadcrumb docs-app page should stay indexable via marker `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_ui_components_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "echo \"[dx] contract: breadcrumb heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include heroui-benchmark docs-sync marker `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "breadcrumb check2 should mark heroui-benchmark docs-sync item complete."
    );

    for marker in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs",
        "apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs",
        "collections_breadcrumb_catalog::BREADCRUMB_DOC",
        "breadcrumb_check2_documents_heroui_benchmark_docs_sync_rules",
        "breadcrumb_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "breadcrumb_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "breadcrumb check2 heroui-benchmark docs-sync section should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "异步/动画 ready-settled 适用性：`breadcrumb` 无异步请求与组件级动效状态轴，本项按 `N/A` 处理；E2E 显式锁定",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 e2e-selector/stable-wait section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_breadcrumb_contract.spec.mjs");

    for needle in [
        "/#/components/breadcrumb",
        "body:not(:has(#boot))",
        "[data-component=\"breadcrumb\"]",
        "[data-slot=\"breadcrumb-state-linked\"] [data-slot=\"breadcrumb\"]",
        "[data-slot=\"breadcrumb-page\"][aria-current=\"page\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.breadcrumb.agent-contract\")",
        "toHaveAttribute(\"data-ui-schema-version\", \"v1\")",
        "toHaveAttribute(\"data-ui-render-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"optional\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "breadcrumb e2e selector/stable-wait contract should include `{needle}`."
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
            "breadcrumb e2e should avoid unstable/non-semantic selector token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "breadcrumb check2 repeatable-key-flow section should include `{required}`."
        );
    }
}

#[test]
fn breadcrumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_breadcrumb_contract.spec.mjs");

    for needle in [
        "docs-app breadcrumb key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "breadcrumb repeatable key flow cycle ${cycle}",
        "streamingOption.focus();",
        "snapshotOption.focus();",
        "await expect(streamingOption).toBeFocused();",
        "await expect(snapshotOption).toBeFocused();",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-requested-stream-mode\", \"streaming\")",
        "toHaveAttribute(\"data-requested-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-render-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-state\", \"linked-trail\")",
        "await page.reload();",
        "toHaveAttribute(\"data-requested-stream-mode\", \"snapshot\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "breadcrumb e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "breadcrumb e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_breadcrumb_contract.spec.mjs");

    for needle in [
        "docs-app breadcrumb high-risk path covers focus keyboard and settled semantic breakpoints",
        "streamingOption.focus();",
        "await expect(streamingOption).toBeFocused();",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-state\", \"linked-trail\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "[data-slot=\"breadcrumb-page\"][aria-current=\"page\"]",
        "not.toHaveAttribute(\"aria-busy\", /.+/)",
        "not.toHaveAttribute(\"data-loading\", /.+/)",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "breadcrumb e2e high-risk path contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "breadcrumb high-risk e2e path should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn breadcrumb_e2e_async_and_animation_axes_are_explicitly_not_applicable_and_semantically_settled()
{
    let e2e_source =
        load_ui_components_source("../../e2e/tests/docs_app_breadcrumb_contract.spec.mjs");

    for needle in [
        "docs-app breadcrumb streaming fallback stays semantically settled without async-ready loops",
        "data-requested-stream-mode",
        "toHaveAttribute(\"data-requested-stream-mode\", \"streaming\")",
        "toHaveAttribute(\"data-ui-render-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "not.toHaveAttribute(\"aria-busy\", /.+/)",
        "not.toHaveAttribute(\"data-loading\", /.+/)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "breadcrumb e2e ready/settled N/A contract should include `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_e2e_check_script_covers_selector_and_stable_wait_contracts() {
    let script_source =
        load_ui_components_source("../../components/breadcrumb/scripts/check-ui-e2e-breadcrumb.sh");

    for needle in [
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_async_and_animation_axes_are_explicitly_not_applicable_and_semantically_settled",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "breadcrumb e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = load_breadcrumb_component_source("test/semantics.rs");
    let aggregated_semantics = load_ui_components_source("tests/breadcrumb_semantics.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let perf_probe_source = load_ui_components_source("../../apps/docs-app/src/perf_probe.rs");
    let todo_source = load_ui_components_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn breadcrumb_semantics_contract_tests_prioritize_contracts_over_snapshots()",
        "fn breadcrumb_has_no_focus_stack_or_overlay_restore_path()",
        "fn breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn breadcrumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "aria-label=a11y.aria_label",
        "aria-current=\"page\"",
        "data-slot=\"breadcrumb\"",
        "data-aria-source=aria_source_attr",
        "data-class-source=class_source_attr",
        "data-separator-source=separator_source_attr",
        "data-last=is_current_page.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose semantic aria/data marker `{marker}`."
        );
    }

    for marker in [
        ".ui-breadcrumb__link:focus-visible",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))",
        "--ui-button-focus-outline-offset,",
    ] {
        assert!(
            styles_source.contains(marker),
            "styles.rs should keep focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
    ] {
        assert!(
            perf_probe_source.contains(marker),
            "UiPerfProbe should expose stable perf observability marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_semantics_and_performance_script_covers_contract() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test breadcrumb_semantics breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test breadcrumb_semantics breadcrumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_breadcrumb_component_source("check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "breadcrumb_semantics_contract_tests_prioritize_contracts_over_snapshots",
        "breadcrumb_has_no_focus_stack_or_overlay_restore_path",
        "breadcrumb_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "breadcrumb_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归当前由仓库统一 follow-up 路线跟踪",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn breadcrumb_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_breadcrumb_component_source("src/Component.toml");
    let rbi_source = load_breadcrumb_component_source("src/breadcrumb.rbi");
    let mod_source = load_breadcrumb_component_source("src/mod.rs");
    let logic_source = load_breadcrumb_component_source("src/logic.rs");
    let view_source = load_breadcrumb_component_source("src/view.rs");
    let styles_source = load_breadcrumb_component_source("src/styles.rs");
    let protocol_source = load_breadcrumb_component_source("src/protocol.rs");
    let check2_source = load_breadcrumb_component_source("check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Breadcrumb\"",
        "crate = \"ui-breadcrumb\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "breadcrumb manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Breadcrumb(",
        "items: Vec<BreadcrumbItem>",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "separator: Option<String>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "breadcrumb RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{protocol_source}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "breadcrumb should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Breadcrumb` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "breadcrumb_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "breadcrumb/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn breadcrumb_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source =
        load_ui_components_source("../../scripts/check-ui-engineering.sh");

    let marker = "cargo test -p ui --test breadcrumb_semantics --no-default-features --features component-breadcrumb,inject-css breadcrumb_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}
