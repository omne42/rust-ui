fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "protocol" => include_str!("../src/protocol.rs"),
        "readme" => include_str!("../src/README.md"),
        "component_manifest" => include_str!("../src/Component.toml"),
        "component_rbi" => include_str!("../src/list.rbi"),
        "check2" => include_str!("../check2.md"),
        "ui_components_cargo" => include_str!("../../../crates/ui/Cargo.toml"),
        "ui_components_lib" => include_str!("../../../crates/ui/src/lib.rs"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui/src/root.rs"),
        "ui_headless_id_provider" => include_str!("../../../crates/ui-headless/src/id_provider.rs"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_headless_controllable_state" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "ui_headless_presence" => include_str!("../../../crates/ui-headless/src/presence.rs"),
        "ui_headless_a11y" => include_str!("../../../crates/ui-headless/src/a11y.rs"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_motion_spring" => include_str!("../../../crates/ui-motion/src/spring.rs"),
        "active_highlight_motion" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "illustrated_message_motion" => include_str!("../../illustrated-message/src/motion.rs"),
        "perf_script" => include_str!("../../../scripts/check-ui-performance.sh"),
        "dx_script" => include_str!("../../../scripts/check-ui-dx.sh"),
        "engineering_script" => include_str!("../../../scripts/check-ui-engineering.sh"),
        "contract_hygiene_script" => {
            include_str!("../../../scripts/check-ui-contract-hygiene.sh")
        }
        "component_files_script" => {
            include_str!("../../../scripts/check-ui-component-files.sh")
        }
        "entrypoints_script" => include_str!("../../../scripts/check-ui-entrypoints.sh"),
        "tree_shaking_script" => {
            include_str!("../../../scripts/check-ui-tree-shaking.sh")
        }
        "tree_shaking_budget" => include_str!("../../../scripts/tree_shaking_budget.env"),
        "platforms_script" => include_str!("../../../scripts/check-ui-platforms.sh"),
        "view_macro_script" => include_str!("../../../scripts/check-ui-view-macro.sh"),
        "inner_html_script" => include_str!("../../../scripts/check-ui-inner-html.sh"),
        "wasm_debug_script" => include_str!("../../../scripts/check-ui-wasm-debug.sh"),
        "streaming_script" => include_str!("../../../scripts/check-ui-streaming.sh"),
        "list_cargo" => include_str!("../Cargo.toml"),
        "docs_collections" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/collections.rs")
        }
        "docs_app_lib" => include_str!("../../../apps/docs-app/src/lib.rs"),
        "debug_overlay" => include_str!("../../../apps/docs-app/src/debug_overlay.rs"),
        "ui_headless_trace" => include_str!("../../../crates/ui-headless/src/trace.rs"),
        "docs_shell" => include_str!("../../../apps/docs-app/src/pages/components/shell.rs"),
        "todo_plan" => include_str!("../../../docs/plan/TODO.md"),
        "list_e2e_contract" => include_str!("../../../e2e/tests/docs_app_list_contract.spec.mjs"),
        "list_e2e_script" => {
            include_str!("../../../components/list/scripts/check-ui-e2e-list.sh")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

fn fn_signature_block<'a>(source: &'a str, fn_name: &str) -> &'a str {
    let marker = format!("pub fn {fn_name}(");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing `{marker}` in source"));
    let tail = &source[start..];
    let end = tail
        .find(") -> impl IntoView")
        .unwrap_or_else(|| panic!("missing return signature for `{fn_name}`"));
    &tail[..end]
}

fn markdown_section<'a>(source: &'a str, title: &str) -> &'a str {
    let marker = format!("## {title}");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing markdown section `{marker}`"));
    let tail = &source[start..];
    let next = tail.find("\n## ").unwrap_or(tail.len());
    &tail[..next]
}

fn list_docs_section(source: &str) -> &str {
    let start = source
        .find("pub(super) fn list() -> AnyView")
        .expect("list docs function should exist");
    let tail = &source[start..];
    let end = tail
        .find("pub(super) fn menu() -> AnyView")
        .expect("list docs section should end before menu docs function");
    &tail[..end]
}

#[test]
fn list_module_boundary_is_minimal_and_wires_local_semantics_tests() {
    let module = load_source("mod");

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ListItemSelectionIndicator, ListSectionHeadingTone, ListState};",
        "pub use motion::ListMotion;",
        "pub use motion::ListSectionMotion;",
        "pub use view::{List, ListItem, ListSection};",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module.contains(required),
            "list module boundary should include `{required}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "pub mod protocol"] {
        assert!(
            !module.contains(forbidden),
            "list internals should stay private: `{forbidden}`."
        );
    }
}

#[test]
fn list_layered_files_keep_ui_components_assembly_split() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for required in [
        "use ui_state_primitives::list as primitives;",
        "pub fn resolve_accessible_name(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep props normalization/state derivation via `{required}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "use_focus_ring("] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not carry rendering/headless mounting `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_listbox(ListBoxOptions {",
        "use_focus_ring(FocusRingOptions {",
        "crate::motion::attach_motion(",
        "logic::resolve_state(",
        "data-slot=\"listbox\"",
    ] {
        assert!(
            view.contains(required),
            "view.rs should render structure + headless mounting via `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::list::resolve_view_state(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not bypass logic/motion boundaries with `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub const ITEM_CSS: &str = r#\"",
        "var(--ui-",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "attach_active_highlight_motion("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry logic/view/runtime integration `{forbidden}`."
        );
    }

    for required in [
        "pub type ListMotion = ActiveHighlightMotion;",
        "pub fn sanitize_motion(",
        "pub fn resolve_motion(",
        "pub fn attach_motion(",
        "pub fn attach_section_motion(",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should keep semantic-to-motion mapping via `{required}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "resolve_view_state("] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not carry rendering/headless/state-machine logic `{forbidden}`."
        );
    }
}

#[test]
fn list_component_directory_has_standard_file_layout() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");
    let component_files_script = load_source("component_files_script");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "list component directory should contain required file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "list simple component scope should not introduce `{forbidden}`."
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ListItemSelectionIndicator, ListSectionHeadingTone, ListState};",
        "pub use motion::ListMotion;",
        "pub use motion::ListSectionMotion;",
        "pub use view::{List, ListItem, ListSection};",
    ] {
        assert!(
            module.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "mod render;", "mod spec;"] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not over-export or drift to forbidden module `{forbidden}`."
        );
    }

    for required in [
        "use ui_state_primitives::list as primitives;",
        "pub fn resolve_accessible_name(",
        "pub fn resolve_state(",
        "pub fn resolve_selection_source_state(",
        "pub fn normalize_options_axis(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep normalization/derivation marker `{required}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "NodeRef<", "web_sys::"] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain view/headless/dom binding marker `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub const ITEM_CSS: &str = r#\"",
        "pub const SECTION_CSS: &str = r#\"",
        "var(--ui-",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep static token-first css marker `{required}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "attach_active_highlight_motion("] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not include runtime/interaction marker `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_listbox(ListBoxOptions {",
        "use_focus_ring(FocusRingOptions {",
        "listbox_option_a11y_attrs(ListBoxOptionA11yInput {",
        "locale_attrs(lang, dir)",
        "crate::motion::attach_motion(",
        "data-selection-mode=selection_sources.selection_mode_attr",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-selection-change-source=selection_sources.selection_change_source_attr",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep structure + headless mounting marker `{required}`."
        );
    }

    for forbidden in ["resolve_view_state(", "mod render;", "render::"] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not drift to forbidden structure marker `{forbidden}`."
        );
    }

    for required in [
        "pub type ListMotion = ActiveHighlightMotion;",
        "pub fn sanitize_motion(",
        "pub fn resolve_motion(",
        "pub fn attach_motion(",
        "pub type ListSectionMotion = ui_illustrated_message::IllustratedMessageMotion;",
        "pub fn attach_section_motion(",
    ] {
        assert!(
            motion.contains(required),
            "motion.rs should keep motion-contract mapping marker `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "use_listbox(",
        "use_focus_ring(",
        "resolve_view_state(",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should stay scoped away from view/headless/state-machine marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_component_directory_has_standard_file_layout";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include list file-layout command."
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "components/list/test/semantics.rs::list_component_directory_has_standard_file_layout",
        "components/list/test/list_module_semantics.rs::list_component_directory_has_standard_file_layout",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include component-directory evidence marker `{required}`."
        );
    }
}

#[test]
fn list_public_component_api_does_not_expose_web_sys_types() {
    let module = load_source("mod");
    let view = load_source("view");

    for forbidden in ["web_sys::", "web-sys", "HtmlElement"] {
        assert!(
            !module.contains(forbidden) && !view.contains(forbidden),
            "list component public surface should not expose DOM/web-sys detail `{forbidden}`."
        );
    }

    for fn_name in ["List", "ListItem", "ListSection"] {
        let signature = fn_signature_block(view, fn_name);
        for forbidden in ["NodeRef", "web_sys::", "HtmlElement", "Element"] {
            assert!(
                !signature.contains(forbidden),
                "{fn_name} public signature should not expose DOM detail `{forbidden}`."
            );
        }
    }
}

#[test]
fn list_public_props_follow_is_on_default_naming_contract() {
    let view = load_source("view");

    let list_signature = fn_signature_block(view, "List");
    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional, default = 0)] default_active_index: usize",
        "#[prop(optional, default = true)] is_active_index_synced_to_selected: bool",
        "#[prop(optional)] on_action: Option<Callback<usize>>",
    ] {
        assert!(
            list_signature.contains(required),
            "List public props should include naming-contract marker `{required}`."
        );
    }

    for forbidden in [
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = 0)] default_index: usize",
        "#[prop(optional, default = true)] sync_active_index_to_selected: bool",
    ] {
        assert!(
            !list_signature.contains(forbidden),
            "List public props should not keep legacy naming alias `{forbidden}`."
        );
    }

    let list_item_signature = fn_signature_block(view, "ListItem");
    for required in [
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_selection_indicator_visible: bool",
        "#[prop(optional)] is_divider_visible: bool",
        "#[prop(optional)] on_press: Option<Callback<()>>",
        "#[prop(optional)] on_pointer_move: Option<Callback<()>>",
    ] {
        assert!(
            list_item_signature.contains(required),
            "ListItem public props should include naming-contract marker `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] selected: bool",
        "#[prop(optional)] focused: bool",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] show_selection_indicator: bool",
        "#[prop(optional)] has_divider: bool",
    ] {
        assert!(
            !list_item_signature.contains(forbidden),
            "ListItem public props should not keep legacy naming alias `{forbidden}`."
        );
    }

    let list_section_signature = fn_signature_block(view, "ListSection");
    for required in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_sticky_heading: bool",
        "#[prop(optional)] is_divider_visible: bool",
    ] {
        assert!(
            list_section_signature.contains(required),
            "ListSection public props should include naming-contract marker `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] sticky_heading: bool",
        "#[prop(optional)] show_divider: bool",
    ] {
        assert!(
            !list_section_signature.contains(forbidden),
            "ListSection public props should not keep legacy naming alias `{forbidden}`."
        );
    }
}

#[test]
fn list_defaults_are_normalized_in_logic_layer() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub fn normalize_list_class_name(",
        "pub fn normalize_callbacks(",
        "pub fn normalize_item_count(",
        "pub fn resolve_title_text(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should keep default normalization contract `{required}`."
        );
    }

    for required in [
        "let class = logic::normalize_list_class_name(class_name);",
        "logic::item::normalize_callbacks(on_press, on_pointer_move)",
        "let resolved_item_count = logic::section::normalize_item_count(item_count);",
        "let title_text = logic::section::resolve_title_text(title);",
    ] {
        assert!(
            view.contains(required),
            "view.rs should consume logic default normalization `{required}`."
        );
    }

    for forbidden in [
        ".unwrap_or(base_class)",
        "on_press.unwrap_or_else(",
        "on_pointer_move.unwrap_or_else(",
        "item_count.unwrap_or(1)",
        "title.get_value().unwrap_or_default()",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not keep local default fallback `{forbidden}`."
        );
    }
}

#[test]
fn list_state_normalization_is_concentrated_in_logic_layer() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub struct ListOptionsAxisInput",
        "pub struct ListOptionStateInput",
        "pub fn normalize_options_axis(",
        "pub fn resolve_option_state(",
        "pub fn is_disabled_index(",
        "pub fn is_interaction_blocked(",
    ] {
        assert!(
            logic.contains(required),
            "logic.rs should own typed state-normalization contract `{required}`."
        );
    }

    for required in [
        "let options_axis = logic::normalize_options_axis(logic::ListOptionsAxisInput {",
        "let option_state = logic::resolve_option_state(logic::ListOptionStateInput {",
        "logic::is_disabled_index(&disabled_indices, index)",
        "let is_interaction_blocked = logic::item::is_interaction_blocked(is_disabled);",
    ] {
        assert!(
            view.contains(required),
            "view.rs should consume normalized state contract `{required}`."
        );
    }

    for forbidden in [
        "let is_disabled = is_disabled || disabled_indices.contains(&index);",
        "if is_disabled {\n                    return;",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not rebuild state-machine rule `{forbidden}`."
        );
    }
}

#[test]
fn list_discrete_state_axes_are_type_constrained() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "pub use primitives::{ListItemSelectionIndicator, ListSectionHeadingTone};",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ListMotion",
        "#[prop(optional)] heading_tone: logic::ListSectionHeadingTone",
        "#[prop(optional)] motion: ListSectionMotion",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "list discrete axes should stay type-constrained by `{required}`."
        );
    }

    for forbidden in [
        "Option<bool>",
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "list should not model mutually-exclusive state with free-form/optional bool axis `{forbidden}`."
        );
    }
}

#[test]
fn list_state_primitives_are_consumed_via_logic_without_business_store_binding() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "use ui_state_primitives::list as primitives;",
        "pub type ListState = primitives::ListViewState;",
        "primitives::resolve_view_state(",
        "primitives::resolve_item_state(",
        "primitives::resolve_section_state(",
    ] {
        assert!(
            logic.contains(required),
            "list logic should consume ui-state-primitives contract via `{required}`."
        );
    }

    for required in [
        "let selection_axis = logic::normalize_selection_axis(logic::ListSelectionAxisInput {",
        "let selected_state = use_controllable_state(",
        "logic::resolve_state(",
        "logic::item::resolve_state(logic::item::ListItemStateInput {",
        "logic::section::resolve_state(logic::section::ListSectionStateInput {",
    ] {
        assert!(
            view.contains(required),
            "list view should consume logic-mapped state primitives via `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::list::resolve_view_state(",
        "ui_state_primitives::list::resolve_item_state(",
        "ui_state_primitives::list::resolve_section_state(",
        "GlobalStore",
        "AppStore",
        "BusinessStore",
        "use_app_state(",
        "use_global_store(",
    ] {
        assert!(
            !view.contains(forbidden),
            "list view should not bypass logic boundary or bind business store `{forbidden}`."
        );
    }
}

#[test]
fn list_dx_hello_world_is_minimal_and_does_not_require_state_wiring() {
    let view = load_source("view");
    let readme = load_source("readme");
    let hello = markdown_section(readme, "Hello World");

    for required in [
        "#[prop(into)] items: Arc<[String]>",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            view.contains(required),
            "List API should keep optional advanced control axis `{required}`."
        );
    }

    for required in [
        "let items: Arc<[String]> = vec![\"Overview\".to_string(), \"Billing\".to_string()].into();",
        "view! { <List id_base=\"list-hello\".to_string() items=items aria_label=\"Settings navigation\".to_string() /> }",
    ] {
        assert!(
            hello.contains(required),
            "Hello World should keep minimal default call path `{required}`."
        );
    }

    for forbidden in ["selected_index=", "on_selected_index_change=", "state="] {
        assert!(
            !hello.contains(forbidden),
            "Hello World should not force advanced state wiring `{forbidden}`."
        );
    }
}

#[test]
fn list_composite_api_avoids_parallel_array_conventions() {
    let view = load_source("view");

    for required in [
        "pub fn ListItem(",
        "pub fn ListSection(",
        "children: Children,",
        "#[prop(into)] items: Arc<[String]>",
    ] {
        assert!(
            view.contains(required),
            "list composite-facing API should include explicit structure marker `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<String>",
        "titles: Vec<String>",
        "panels: Vec<",
        "item_specs: Vec<",
    ] {
        assert!(
            !view.contains(forbidden),
            "list API should not expose parallel-array/config sugar contract `{forbidden}`."
        );
    }
}

#[test]
fn list_macro_micro_dragging_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "list interaction/motion path should stay focused on pointer/listbox highlight contract `{required}`."
        );
    }

    for forbidden in [
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "Dragging",
        "Action::DragEnd",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not expose dragging macro/micro state-machine contract `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_two_pass_geometry_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "use_listbox(ListBoxOptions {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "list should keep listbox interaction + highlight motion path `{required}`."
        );
    }

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "Rectification",
        "Action::Measure",
        "Action::Rectification",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement two-pass geometry rendering contract `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_registration_protocol_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "pub fn is_disabled_index(disabled_indices: &HashSet<usize>, index: usize) -> bool",
        "disabled_indices.contains(&index)",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "list should keep deterministic item order and membership-only disabled lookup marker `{required}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "registration_context",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement dynamic child registration protocol marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_slot_projection_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "data-slot=\"listbox-options\"",
        "items.iter().cloned().enumerate()",
        ".collect_view()",
        "attach_motion(",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "list should keep eager listbox rendering contract marker `{required}`."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "slot_projection",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement slot projection lifecycle contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_env_streams_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "attach_motion(",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "list should keep direct listbox interaction path marker `{required}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "ThemeChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "window.matchMedia",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement environment subscription stream contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_event_light_cone_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "use_listbox(ListBoxOptions {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "list should keep single-listbox interaction contract marker `{required}`."
        );
    }

    for forbidden in [
        "SelectionState::All",
        "ContextBus",
        "use_context_bus",
        "selector_subscribe",
        "on_select_all",
        "prop_drilling",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement event light-cone bulk contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_causality_bus_contract_is_not_applicable_in_current_scope() {
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    for required in [
        "#[prop(optional)] on_action: Option<Callback<usize>>",
        "use_listbox(ListBoxOptions {",
        "on_action,",
        "on:click=move |_| {",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "list should keep direct user-intent callback path marker `{required}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "publish(",
        "subscribe(",
        "broadcast(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list should not implement unified causality bus marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "use ui_headless::a11y::locale_attrs;",
        "listbox_option_a11y_attrs(ListBoxOptionA11yInput {",
        "role=move || option_a11y().role",
        "aria-selected=move || option_a11y().aria_selected",
        "aria-disabled=move || option_a11y().aria_disabled",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "#[prop(optional, into)] selected_text: Option<String>",
        "#[prop(optional, into)] unselected_text: Option<String>",
        "normalize_selection_status_text(selected_text, unselected_text)",
        "selection_selected_text.get_value()",
        "selection_unselected_text.get_value()",
    ] {
        assert!(
            view.contains(required),
            "list view should keep a11y+i18n+l10n contract marker `{required}`."
        );
    }

    for required in [
        "pub fn normalize_selection_status_text(",
        "DEFAULT_SELECTED_TEXT",
        "DEFAULT_UNSELECTED_TEXT",
    ] {
        assert!(
            logic.contains(required),
            "list logic should keep overridable fallback-text contract marker `{required}`."
        );
    }

    assert!(
        !view.contains("\"selected\" } else { \"not selected\"")
            && !view.contains("{ \"selected\" } else { \"not selected\" }"),
        "list view should not hardcode selection copy directly in render path."
    );
}

#[test]
fn list_state_observability_contract_uses_stable_data_and_aria_markers() {
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "data-state=move || option_a11y().data_state",
        "data-selected=move || option_a11y().data_selected",
        "data-focused=move || option_a11y().data_focused",
        "data-disabled=move || option_a11y().data_disabled",
        "data-selection-mode=selection_sources.selection_mode_attr",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-default-selection-source=selection_sources.default_selection_source_attr",
        "data-selection-change-source=selection_sources.selection_change_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-title-source=move || state.get().title_source_attr",
    ] {
        assert!(
            view.contains(required),
            "list view should expose stable observability marker `{required}`."
        );
    }

    for required in [
        "pub struct ListSelectionSourceStateInput",
        "pub struct ListSelectionSourceState",
        "pub enum ListInteractionSource",
        "selection_mode_attr: if input.is_controlled {",
        "selection_value_source_attr: if input.is_controlled {",
        "default_selection_source_attr: if input.has_default_selected_index {",
        "selection_change_source_attr: if input.has_on_selected_index_change {",
        "ListInteractionSource::None => \"none\"",
        "ListInteractionSource::Keyboard => \"keyboard\"",
        "ListInteractionSource::Pointer => \"pointer\"",
    ] {
        assert!(
            logic.contains(required),
            "list logic should keep closed-set source marker mapping `{required}`."
        );
    }
}

#[test]
fn list_styles_depend_on_explicit_state_markers_not_fragile_dom_guessing() {
    let styles = load_source("styles");
    let view = load_source("view");

    for required in [
        ".ui-listbox__option[data-selected=\"true\"]",
        ".ui-listbox__option[data-disabled=\"true\"]",
        ".ui-listbox-item[data-selected=\"true\"]",
        ".ui-listbox-item[data-focused=\"true\"]",
        ".ui-listbox-item[data-disabled=\"true\"]",
        ".ui-listbox-item[data-show-selection-indicator=\"true\"]",
        ".ui-listbox-item[data-has-divider=\"true\"]",
        ".ui-listbox-section[data-tone=\"default\"] .ui-listbox-section__header",
        ".ui-listbox-section[data-tone=\"quiet\"] .ui-listbox-section__header",
        ".ui-listbox-section[data-empty=\"true\"]",
        ".ui-listbox-section[data-disabled=\"true\"]",
        ".ui-listbox-section[data-sticky-heading=\"true\"]",
        ".ui-listbox-section[data-divided=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "list styles should express visual state by explicit marker selector `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles.contains(forbidden),
            "list styles should not use fragile structural selector `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "top: 10px", "left: 10px"] {
        assert!(
            !view.contains(forbidden),
            "list view should not inject business styling via inline style marker `{forbidden}`."
        );
    }
}

#[test]
fn list_styles_remain_token_first_and_avoid_utility_or_css_in_rust_contracts() {
    let styles = load_source("styles");
    let view = load_source("view");
    let logic = load_source("logic");

    for required in [
        "pub const CSS: &str = r#\"",
        "pub const ITEM_CSS: &str = r#\"",
        "pub const SECTION_CSS: &str = r#\"",
        "var(--ui-",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep token-first static CSS contract marker `{required}`."
        );
    }

    assert!(
        !styles.contains("--ui-listbox-"),
        "styles.rs should not define component-private token namespace `--ui-listbox-*`."
    );

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "class=\"rounded-",
        "class=\"shadow-",
        "@apply ",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "list component contract should avoid utility-first marker `{forbidden}`."
        );
    }

    for forbidden in ["css!(", "style!(", "styled::", "StyleSheet::", "emotion::"] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !styles.contains(forbidden),
            "list component contract should avoid css-in-rust default marker `{forbidden}`."
        );
    }
}

#[test]
fn list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions() {
    let semantics_source = include_str!("semantics.rs");
    let logic_tests_source = include_str!("logic.rs");
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "check2.md should mark the semantic-contract test item as completed."
    );

    for required in [
        "list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
        "list_styles_depend_on_explicit_state_markers_not_fragile_dom_guessing",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantics.rs should keep semantic-contract regression `{required}`."
        );
    }

    for required in [
        "resolve_selection_source_state_covers_controlled_and_uncontrolled_matrix",
        "resolve_option_state_derives_selected_focused_and_disabled_bits",
        "list_interaction_source_attr_is_closed_set_for_none_keyboard_and_pointer",
    ] {
        assert!(
            logic_tests_source.contains(required),
            "logic.rs tests should keep matrix axis regression `{required}`."
        );
    }

    for required in [
        "on:keydown=on_key_down",
        "aria.handlers.on_key_down.run(ev.key())",
        "on:pointerdown=move |_| {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
    ] {
        assert!(
            view.contains(required),
            "view.rs should keep keyboard/pointer semantic path marker `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list semantic contract should not fork by platform in component layer `{forbidden}`."
        );
    }

    let assert_snapshot = ["assert_", "snapshot!"].concat();
    let assert_debug_snapshot = ["assert_", "debug_snapshot!"].concat();
    let to_match_snapshot = [".to_match_", "snapshot("].concat();
    let to_match_snapshot_js = ["toMatch", "Snapshot("].concat();
    for forbidden in [
        assert_snapshot.as_str(),
        assert_debug_snapshot.as_str(),
        to_match_snapshot.as_str(),
        to_match_snapshot_js.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden) && !logic_tests_source.contains(forbidden),
            "semantic regressions should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn list_checklist_marks_ui_components_boundary_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。"),
        "check2.md should mark ui definition as completed."
    );
    assert!(
        check2.contains("components/list/test/semantics.rs"),
        "check2.md should include component-local semantics.rs evidence."
    );
}

#[test]
fn list_check2_documents_semantics_first_testing_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn list_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = include_str!("semantics.rs");
    let logic_tests_source = include_str!("logic.rs");
    let module = load_source("mod");

    for required in [
        "list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
        "list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions",
        "list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            semantics_source.contains(required),
            "list semantics suite should keep contract-first assertion `{required}`."
        );
    }

    for required in [
        "resolve_selection_source_state_covers_controlled_and_uncontrolled_matrix",
        "resolve_option_state_derives_selected_focused_and_disabled_bits",
        "list_interaction_source_attr_is_closed_set_for_none_keyboard_and_pointer",
    ] {
        assert!(
            logic_tests_source.contains(required),
            "list logic regression should keep semantic matrix axis `{required}`."
        );
    }

    assert!(
        module.contains("#[path = \"../test/semantics.rs\"]"),
        "list module should keep `*_semantics.rs` test entry point."
    );

    let assert_snapshot = ["assert_", "snapshot!"].concat();
    let assert_debug_snapshot = ["assert_", "debug_snapshot!"].concat();
    let to_match_snapshot = [".to_match_", "snapshot("].concat();
    for forbidden in [
        assert_snapshot.as_str(),
        assert_debug_snapshot.as_str(),
        to_match_snapshot.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden) && !logic_tests_source.contains(forbidden),
            "list semantic suite should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view = load_source("view");
    let local_semantics = include_str!("semantics.rs");
    let aggregated_semantics =
        include_str!("../../../components/list/test/list_module_semantics.rs");

    for marker in [
        "role=aria.attrs.role",
        "aria-label=aria_label.get_value()",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-state=move || option_a11y().data_state",
        "data-selected=move || option_a11y().data_selected",
        "data-focused=move || option_a11y().data_focused",
        "data-disabled=move || option_a11y().data_disabled",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(marker),
            "list view should keep semantic marker `{marker}`."
        );
        assert!(
            local_semantics.contains(marker),
            "list local semantics tests should cover semantic marker `{marker}` changes."
        );
        assert!(
            aggregated_semantics.contains(marker),
            "list aggregated semantics tests should cover semantic marker `{marker}` changes."
        );
    }
}

#[test]
fn list_semantics_first_testing_script_covers_contract() {
    let contract_hygiene_script = load_source("contract_hygiene_script");

    for marker in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            contract_hygiene_script.contains(marker),
            "contract-hygiene script should include `{marker}`."
        );
    }
}

#[test]
fn list_check2_marks_semantics_first_testing_contract_complete() {
    let check2 = load_source("check2");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/list/test/semantics.rs::list_check2_documents_semantics_first_testing_rules",
        "components/list/test/semantics.rs::list_semantics_suite_is_contract_first_not_snapshot_only",
        "components/list/test/semantics.rs::list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "components/list/test/semantics.rs::list_semantics_first_testing_script_covers_contract",
        "components/list/test/list_module_semantics.rs::list_check2_documents_semantics_first_testing_rules",
        "components/list/test/list_module_semantics.rs::list_semantics_suite_is_contract_first_not_snapshot_only",
        "components/list/test/list_module_semantics.rs::list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "components/list/test/list_module_semantics.rs::list_semantics_first_testing_script_covers_contract",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "check2.md should include semantics-first testing evidence `{marker}`."
        );
    }
}

#[test]
fn list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2 = load_source("check2");
    let streaming_script = load_source("streaming_script");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "components/list/test/semantics.rs::list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "components/list/test/list_module_semantics.rs::list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include streaming definition marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list two-mode definition command."
    );
}

#[test]
fn list_check2_documents_snapshot_as_default_baseline_capability() {
    let check2 = load_source("check2");
    let streaming_script = load_source("streaming_script");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/list/test/semantics.rs::list_check2_documents_snapshot_as_default_baseline_capability",
        "components/list/test/list_module_semantics.rs::list_check2_documents_snapshot_as_default_baseline_capability",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include snapshot-baseline marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_snapshot_as_default_baseline_capability";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list snapshot-baseline checklist command."
    );
}

#[test]
fn list_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let streaming_script = load_source("streaming_script");
    let list_signature = fn_signature_block(view, "List");

    for required in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "logic::resolve_state(",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required) || list_signature.contains(required),
            "list snapshot baseline should keep complete-result render marker `{required}`."
        );
    }

    for required in [
        "[[capabilities]]\nname = \"snapshot_rendering\"\nenabled = true",
        "name = \"items\"",
        "ty = \"Arc<[String]>\"",
        "default = \"required\"",
    ] {
        assert!(
            manifest.contains(required),
            "list manifest should keep snapshot baseline capability marker `{required}`."
        );
    }

    assert!(
        rbi.contains("items: std::sync::Arc<[String]>,"),
        "list RBI should keep complete snapshot input projection for `items`."
    );

    for forbidden in [
        "stream_chunk",
        "token_delta",
        "partial_payload",
        "incremental_patch",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "list baseline snapshot render path should not depend on streaming-only token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list snapshot-stability command."
    );

    for required in [
        "components/list/test/semantics.rs::list_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "components/list/test/list_module_semantics.rs::list_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include snapshot-baseline evidence marker `{required}`."
        );
    }
}

#[test]
fn list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let manifest = load_source("component_manifest");
    let streaming_script = load_source("streaming_script");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "归类为 `Streaming Optional`",
        "`Snapshot` 渲染为基线",
        "`fallback=snapshot`",
        "`data-ui-output-status`",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "components/list/test/semantics.rs::list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback",
        "components/list/test/list_module_semantics.rs::list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include streaming required/optional governance marker `{required}`."
        );
    }

    for required in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "view should keep streaming-governance semantic marker `{required}`."
        );
    }

    for required in [
        "ListAgentStreamSupport::Optional",
        "ListAgentStreamFallback::Snapshot",
        "ListAgentOutputStatus::Verified",
    ] {
        assert!(
            logic.contains(required),
            "logic should keep streaming-governance typed contract `{required}`."
        );
    }

    for required in [
        "name = \"stream_support\"",
        "values = [\"optional\"]",
        "name = \"stream_fallback\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            manifest.contains(required),
            "component manifest should keep streaming-governance marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list streaming required/optional governance command."
    );
}

#[test]
fn list_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2 = load_source("check2");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let active_highlight = load_source("active_highlight_motion");
    let headless_controllable_state = load_source("ui_headless_controllable_state");
    let headless_presence = load_source("ui_headless_presence");
    let headless_a11y = load_source("ui_headless_a11y");
    let entrypoints_script = load_source("entrypoints_script");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_components_src = manifest_dir.join("../../crates/ui/src");

    for required in [
        "#[cfg(feature = \"component-list\")]",
        "pub use ui_list as list;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib entry should keep marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css entry should keep marker `{required}`."
        );
    }

    for required in [
        "#[component]",
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
            "UiRoot entry should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "pub fn List(", "ui-listbox"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic shared primitive, found `{forbidden}`."
        );
    }

    assert!(
        !ui_components_src.join("overlay_open.rs").exists(),
        "ui should not define `src/overlay_open.rs`."
    );
    assert!(
        !ui_components_src.join("presence.rs").exists(),
        "ui should not define `src/presence.rs`."
    );
    assert!(
        !ui_components_src.join("a11y.rs").exists(),
        "ui should not define `src/a11y.rs`."
    );

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable_state.contains(required)
                || headless_presence.contains(required)
                || headless_a11y.contains(required),
            "headless canonical primitive should keep marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints gate script should include list fixed-entry command."
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "components/list/test/semantics.rs::list_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/list/test/list_module_semantics.rs::list_ui_components_fixed_entry_files_follow_layered_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include fixed-entry evidence marker `{required}`."
        );
    }
}

#[test]
fn list_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2 = load_source("check2");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2.contains(marker),
            "check2.md should keep list e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn list_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e = load_source("list_e2e_contract");
    let docs = load_source("docs_collections");
    let docs_section = list_docs_section(docs);

    for marker in [
        "page.goto(\"/#/components/list\")",
        "body:not(:has(#boot))",
        "[data-component=\"list\"]",
        "[data-slot=\"list-showcase\"]",
        "[data-slot=\"listbox\"][aria-label=\"Default list\"]",
        "toHaveAttribute(\"data-selection-mode\", \"controlled\")",
        "toHaveAttribute(\"data-selection-value-source\", \"external\")",
        "toHaveAttribute(\"data-default-selection-source\", \"none\")",
        "toHaveAttribute(\"data-selection-change-source\", \"provided\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-interaction-source\", \"none\")",
        "[data-slot=\"listbox-option\"][data-index=\"2\"][data-disabled=\"true\"]",
        "[data-slot=\"list-streaming-snapshot\"] [data-ui-output-state=\"snapshot\"]",
        "[data-slot=\"list-streaming-snapshot\"] [data-ui-output-state=\"streaming\"]",
    ] {
        assert!(
            e2e.contains(marker),
            "list e2e selector/stable-wait contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"list-showcase\"",
        "data-slot=\"list-state-matrix\"",
        "data-slot=\"list-workbench\"",
        "data-slot=\"list-workbench-controls\"",
        "data-slot=\"list-workbench-canvas\"",
        "data-slot=\"list-streaming-snapshot\"",
    ] {
        assert!(
            docs_section.contains(marker),
            "list docs should keep e2e semantic anchor `{marker}`."
        );
    }

    let to_match_snapshot_js = ["toMatch", "Snapshot("].concat();
    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "toHaveScreenshot(",
        to_match_snapshot_js.as_str(),
    ] {
        assert!(
            !e2e.contains(forbidden),
            "list e2e contract should avoid flaky/text/snapshot selector token `{forbidden}`."
        );
    }
}

#[test]
fn list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths() {
    let e2e = load_source("list_e2e_contract");

    for marker in [
        "async function expectListReady(root)",
        "async function expectListPointerSettled(root)",
        "async function expectListKeyboardSettled(root, previousActiveDescendant)",
        "async function runListReadySettledFlow(page, root)",
        "await pointerTarget.click();",
        "const activeAfterPointer = await root.getAttribute(\"aria-activedescendant\");",
        "await page.keyboard.press(\"ArrowDown\");",
        "await page.keyboard.press(\"Enter\");",
        "toHaveAttribute(\"data-interaction-source\", \"pointer\")",
        "toHaveAttribute(\"data-interaction-source\", \"keyboard\")",
        "toHaveAttribute(\"data-ui-state\", \"has-selection\")",
        "toHaveCount(1);",
    ] {
        assert!(
            e2e.contains(marker),
            "list e2e ready/settled contract should include `{marker}`."
        );
    }
}

#[test]
fn list_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script = load_source("list_e2e_script");

    for marker in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths",
    ] {
        assert!(
            script.contains(marker),
            "list e2e check script should include `{marker}`."
        );
    }
}

#[test]
fn list_check2_marks_e2e_selector_stability_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "check2.md should mark list e2e selector stability item complete."
    );

    for marker in [
        "e2e/tests/docs_app_list_contract.spec.mjs",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "components/list/scripts/check-ui-e2e-list.sh",
        "components/list/test/semantics.rs::list_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/list/test/semantics.rs::list_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/list/test/semantics.rs::list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths",
        "components/list/test/semantics.rs::list_e2e_check_script_covers_selector_and_settled_wait_contract",
        "components/list/test/list_module_semantics.rs::list_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/list/test/list_module_semantics.rs::list_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/list/test/list_module_semantics.rs::list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths",
        "components/list/test/list_module_semantics.rs::list_e2e_check_script_covers_selector_and_settled_wait_contract",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "check2.md should include list e2e selector stability evidence marker `{marker}`."
        );
    }
}

#[test]
fn list_component_does_not_introduce_spec_rs_for_simple_scope() {
    let module = load_source("mod");
    let readme = load_source("readme");
    let check2 = load_source("check2");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "list module should not expose `spec.rs` surface `{forbidden}` in current scope."
        );
    }

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/spec.rs")
            .exists(),
        "list simple component scope should not ship a `src/spec.rs` file."
    );

    assert!(
        readme.contains("## 组件结构"),
        "list README should keep component-level documentation instead of moving simple scope to `spec.rs`."
    );

    assert!(
        check2.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "check2.md should mark `spec.rs` governance item as completed."
    );
}

#[test]
fn list_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2 = load_source("check2");
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let component_files_script = load_source("component_files_script");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    assert!(
        src_dir.join("Component.toml").exists(),
        "list component should provide `src/Component.toml` manifest for context compression."
    );
    assert!(
        src_dir.join("list.rbi").exists(),
        "list component should provide `src/list.rbi` interface projection."
    );

    for required in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"List\"",
        "crate = \"ui-list\"",
        "[[capabilities]]\nname = \"context_compression_manifest\"\nenabled = true",
        "[[capabilities]]\nname = \"rbi_signature_projection\"\nenabled = true",
    ] {
        assert!(
            manifest.contains(required),
            "list manifest should include `{required}`."
        );
    }

    for required in [
        "pub type ListState = ui_state_primitives::list::ListViewState;",
        "pub type ListMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn List(",
        "pub fn ListItem(",
        "pub fn ListSection(",
    ] {
        assert!(
            rbi.contains(required),
            "list RBI projection should include `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include list manifest/rbi command."
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/list/src/Component.toml",
        "components/list/src/list.rbi",
        "components/list/test/semantics.rs::list_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "components/list/test/list_module_semantics.rs::list_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include manifest/rbi evidence marker `{required}`."
        );
    }
}

#[test]
fn list_check2_documents_agent_contract_schema_governance_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include agent-contract governance marker `{required}`."
        );
    }
}

#[test]
fn list_agent_contract_is_schema_typed_and_machine_readable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let contract_hygiene_script = load_source("contract_hygiene_script");

    for required in [
        "pub const LIST_AGENT_SCHEMA: &str = \"ui.list.agent-contract\";",
        "pub enum ListAgentSchemaVersion",
        "pub enum ListAgentIntent",
        "pub enum ListAgentAction",
        "pub enum ListAgentState",
        "pub enum ListAgentSource",
        "pub enum ListAgentConfigPolicy",
        "pub struct ListAgentContractInput",
        "pub struct ListAgentContract",
        "pub fn resolve_agent_contract(input: ListAgentContractInput) -> ListAgentContract",
    ] {
        assert!(
            logic.contains(required),
            "list logic should keep typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_attr()",
        "data-ui-intent=move || agent_contract.get().intent.as_attr()",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()",
    ] {
        assert!(
            view.contains(required),
            "list view should mount schemaized agent-contract field `{required}`."
        );
    }

    for required in [
        "[[capabilities]]\nname = \"agent_contract_schema_markers\"\nenabled = true",
        "[agent_contract]",
        "schema = \"ui.list.agent-contract\"",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
    ] {
        assert!(
            manifest.contains(required),
            "list Component.toml should include schemaized agent-contract marker `{required}`."
        );
    }

    for required in [
        "pub const LIST_AGENT_SCHEMA: &str;",
        "pub enum ListAgentSchemaVersion",
        "pub struct ListAgentContract",
        "pub fn resolve_agent_contract(input: ListAgentContractInput) -> ListAgentContract;",
    ] {
        assert!(
            rbi.contains(required),
            "list RBI projection should include typed agent-contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-schema-version=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
    ] {
        assert!(
            !view.contains(forbidden),
            "list view should not build agent-contract fields via free-form splice `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_agent_contract_is_schema_typed_and_machine_readable";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene script should include list typed agent-contract command."
    );
}

#[test]
fn list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let protocol = load_source("protocol");
    let manifest = load_source("component_manifest");
    let contract_hygiene_script = load_source("contract_hygiene_script");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "typed_agent_contract_from_logic::resolve_agent_contract",
        "typed_render_mount_from_view",
        "blocked = [",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest.contains(required),
            "list Component.toml should include whitelist guard marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !protocol.contains(forbidden),
            "list render path should stay script-injection free and reject `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene script should include list whitelist-safe agent-contract command."
    );

    for required in [
        "components/list/test/semantics.rs::list_check2_documents_agent_contract_schema_governance_rules",
        "components/list/test/semantics.rs::list_agent_contract_is_schema_typed_and_machine_readable",
        "components/list/test/semantics.rs::list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "components/list/test/list_module_semantics.rs::list_check2_documents_agent_contract_schema_governance_rules",
        "components/list/test/list_module_semantics.rs::list_agent_contract_is_schema_typed_and_machine_readable",
        "components/list/test/list_module_semantics.rs::list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include list agent-contract evidence marker `{required}`."
        );
    }
}

#[test]
fn list_visual_desire_baseline_is_documented_for_component_scope() {
    let check2 = load_source("check2");
    let readme = load_source("readme");
    let styles = load_source("styles");

    assert!(
        check2.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "check2.md should mark visual-desire governance item as completed."
    );

    for required in [
        "## 展示 (Display)",
        "## Config (Workbench Settings)",
        "## CSS Test (Scoped CSS)",
        "展示区包含多场景对比",
        "CSS Test 面板用于局部覆盖与回放",
    ] {
        assert!(
            readme.contains(required),
            "README should keep visual baseline documentation marker `{required}`."
        );
    }

    for required in [
        ".ui-listbox--focus-visible",
        ".ui-listbox__option[data-selected=\"true\"]",
        ".ui-listbox__option[data-disabled=\"true\"]",
        ".ui-listbox-item[data-focused=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "styles.rs should keep visual hierarchy/feedback marker `{required}`."
        );
    }

    for forbidden in ["bootstrap", "Bootstrap"] {
        assert!(
            !readme.contains(forbidden) && !styles.contains(forbidden),
            "list visual baseline should not regress to coarse legacy style marker `{forbidden}`."
        );
    }
}

#[test]
fn list_tree_shaking_contract_uses_feature_gates_and_no_unconditional_registry_path() {
    let check2 = load_source("check2");
    let cargo = load_source("ui_components_cargo");
    let lib = load_source("ui_components_lib");
    let css = load_source("ui_components_css");

    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "check2.md should mark tree-shaking governance item as completed."
    );

    for required in [
        "component-list = [",
        "\"component-active_highlight\"",
        "\"component-illustrated_message\"",
        "\"dep:ui-list\"",
    ] {
        assert!(
            cargo.contains(required),
            "ui feature graph should include list feature dependency marker `{required}`."
        );
    }

    assert!(
        lib.contains("#[cfg(feature = \"component-list\")]\npub use ui_list as list;"),
        "ui lib should gate list export behind `component-list` feature."
    );

    for required in [
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
    ] {
        assert!(
            css.contains(required),
            "ui css aggregation should keep feature-gated list marker `{required}`."
        );
    }

    assert!(
        lib.contains("#[cfg(feature = \"all-components\")]\npub use all_components::*;"),
        "all-components export path should remain feature-gated."
    );
}

#[test]
fn list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script = load_source("tree_shaking_script");
    let budget = load_source("tree_shaking_budget");

    for required in [
        "LIST_MIN_FEATURES=\"component-list,inject-css\"",
        "cargo test -p ui-list list_tree_shaking_contract_uses_feature_gates_and_no_unconditional_registry_path",
        "cargo test -p ui-list list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-list list_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_tree_shaking_contract_uses_component_feature_gates",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "LIST_TREE_OUTPUT",
        "if grep -q 'all-components' <<<\"$LIST_TREE_OUTPUT\";",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$LIST_MIN_FEATURES\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$LIST_MIN_FEATURES\"",
    ] {
        assert!(
            script.contains(required),
            "tree-shaking script should include list marker `{required}`."
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget.contains(required),
            "tree-shaking budget file should define `{required}`."
        );
    }
}

#[test]
fn list_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "check2.md should keep first-class tree-shaking governance item checked."
    );
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "check2.md should mark tree-shaking feature-pruning checklist item as completed."
    );

    for required in [
        "list_tree_shaking_contract_uses_feature_gates_and_no_unconditional_registry_path",
        "list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "list_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-list,inject-css",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2.contains(required),
            "check2 tree-shaking section should reference `{required}`."
        );
    }
}

#[test]
fn list_type_system_and_semantic_markers_form_machine_readable_contract() {
    let check2 = load_source("check2");
    let semantics_source = include_str!("semantics.rs");
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        check2.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "check2.md should mark typed-state + semantic-marker governance item as completed."
    );

    for required in [
        "list_discrete_state_axes_are_type_constrained",
        "list_state_normalization_is_concentrated_in_logic_layer",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantics regression should include `{required}`."
        );
    }

    for required in [
        "pub struct ListOptionStateInput",
        "pub struct ListSelectionSourceStateInput",
        "pub struct ListSelectionSourceState",
        "pub enum ListInteractionSource",
        "pub fn normalize_options_axis(",
        "pub fn resolve_option_state(",
        "pub fn resolve_selection_source_state(",
    ] {
        assert!(
            logic.contains(required),
            "logic typed-state contract should include `{required}`."
        );
    }

    for required in [
        "data-state=move || option_a11y().data_state",
        "data-selection-mode=selection_sources.selection_mode_attr",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-selection-change-source=selection_sources.selection_change_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(required),
            "view machine-readable semantic marker contract should include `{required}`."
        );
    }
}

#[test]
fn list_focus_stack_and_gc_contract_is_not_applicable_in_current_scope() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "check2.md should mark focus-stack governance item as completed."
    );

    for required in [
        "use_focus_ring(FocusRingOptions { is_disabled });",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
    ] {
        assert!(
            view.contains(required),
            "list should keep local focus-ring hook marker `{required}`."
        );
    }

    for forbidden in [
        "overlay_stack",
        "focus_stack",
        "FocusManager",
        "FallbackTo",
        "Selector",
        "document.body",
        "active_element",
        "restore_focus",
        "on_close_restore_focus",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list current scope should not implement overlay focus-stack contract marker `{forbidden}`."
        );
    }
}

#[test]
fn list_escape_hatches_foreign_zone_contract_is_not_applicable_in_current_scope() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let logic = load_source("logic");
    let motion = load_source("motion");

    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "check2.md should mark escape-hatches governance item as completed."
    );

    for required in [
        "use_listbox(ListBoxOptions {",
        "logic::resolve_state(",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            view.contains(required) || logic.contains(required) || motion.contains(required),
            "list should keep native headless/logic/motion integration marker `{required}`."
        );
    }

    for forbidden in [
        "echarts",
        "ECharts",
        "mapbox",
        "leaflet",
        "GoogleMap",
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "js_sys::",
        "wasm_bindgen::JsValue",
        "third_party_instance",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "list current scope should not include foreign-zone third-party imperative marker `{forbidden}`."
        );
    }
}

#[test]
fn list_hydration_discontinuity_contract_uses_deterministic_id_provider_path() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let root = load_source("ui_components_root");
    let id_provider = load_source("ui_headless_id_provider");

    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "check2.md should mark hydration-discontinuity governance item as completed."
    );

    for required in [
        "#[prop(optional, into)] id_base: Option<String>,",
        "use_ui_id_provider().map(|provider| provider.next_prefixed_id(logic::DEFAULT_ID_BASE))",
        "let id_base = logic::normalize_id_base(id_base);",
    ] {
        assert!(
            view.contains(required),
            "list view should consume deterministic IdProvider path `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-list\";",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "primitives::normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())",
    ] {
        assert!(
            logic.contains(required),
            "list logic should normalize deterministic id-base contract `{required}`."
        );
    }

    assert!(
        root.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should keep deterministic id-seed injection path."
    );

    for required in [
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
    ] {
        assert!(
            id_provider.contains(required),
            "headless id-provider contract should expose `{required}`."
        );
    }

    for forbidden in [
        "now()",
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "js_sys::Date::now",
        "uuid::",
        "Uuid::new_v4",
        "rand::",
        "thread_rng",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "list should not introduce non-deterministic hydration id source `{forbidden}`."
        );
    }
}

#[test]
fn list_ssr_cross_platform_contract_keeps_non_wasm_safe_and_cfg_explicit() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");
    let active_highlight_motion = load_source("active_highlight_motion");
    let illustrated_message_motion = load_source("illustrated_message_motion");

    assert!(
        check2
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "check2.md should mark SSR/cross-platform governance item as completed."
    );

    for required in [
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include compile-only evidence marker `{required}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "web-sys",
        "js_sys::",
        "window.",
        "document.",
        "wasm_bindgen::",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !motion.contains(forbidden),
            "list component layer should not reference browser-only API `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_motion.contains(required),
            "active-highlight motion primitive should keep explicit platform cfg marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            illustrated_message_motion.contains(required),
            "illustrated-message motion primitive should keep explicit platform cfg marker `{required}`."
        );
    }
}

#[test]
fn list_ui_headless_web_ssr_mutual_exclusion_contract_is_preserved() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let ui_headless_lib = load_source("ui_headless_lib");

    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "check2.md should mark ui-headless web/ssr mutual-exclusion governance item as completed."
    );

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should keep mutual-exclusion guard marker `{required}`."
        );
    }

    for required in [
        "use_listbox(ListBoxOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_ui_id_provider().map(|provider| provider.next_prefixed_id(logic::DEFAULT_ID_BASE))",
    ] {
        assert!(
            view.contains(required),
            "list should keep ui-headless integration marker `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(feature = \"web\")]",
        "#[cfg(feature = \"ssr\")]",
        "cfg!(feature = \"web\")",
        "cfg!(feature = \"ssr\")",
        "feature = \"web\", feature = \"ssr\"",
        "compile_error!(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden),
            "list component layer should not redefine headless feature-mutex contract `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include ui-headless feature-path evidence marker `{required}`."
        );
    }
}

#[test]
fn list_ui_motion_non_wasm_noop_contract_is_preserved() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");

    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "check2.md should mark ui-motion non-wasm noop governance item as completed."
    );

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion non-wasm backend should keep noop/stub marker `{required}`."
        );
    }

    for required in [
        "pub fn attach_motion(",
        "pub fn attach_section_motion(",
        "sanitize_section_motion(motion)",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            motion.contains(required) || view.contains(required),
            "list motion mapping should keep safe attach contract `{required}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen::",
        "request_animation_frame",
        "raf_handle",
        "Animation",
        "panic!(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden),
            "list component layer should not assume browser animation handles or panic path `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include ui-motion compile evidence marker `{required}`."
        );
    }
}

#[test]
fn list_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");
    let ui_motion_spring = load_source("ui_motion_spring");
    let active_highlight_motion = load_source("active_highlight_motion");
    let illustrated_message_motion = load_source("illustrated_message_motion");

    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "check2.md should mark reduced-motion/SSR/wasm governance item as completed."
    );

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "ui-motion spring should keep reduced-motion short-circuit marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
        "height: var(--ui-active-highlight-h, 0px);",
        "transform: translateY(var(--ui-active-highlight-y, 0px));",
        "opacity: var(--ui-active-highlight-o, 0);",
    ] {
        assert!(
            active_highlight_motion.contains(required),
            "active-highlight primitive should keep SSR/wasm parity marker `{required}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            illustrated_message_motion.contains(required),
            "illustrated-message primitive should keep explicit platform branch marker `{required}`."
        );
    }

    for required in [
        "role=aria.attrs.role",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-slot=\"listbox\"",
        "data-state=move || option_a11y().data_state",
        "data-selected=move || option_a11y().data_selected",
        "data-focused=move || option_a11y().data_focused",
    ] {
        assert!(
            view.contains(required),
            "list view should keep cross-platform semantic marker `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !motion.contains(forbidden),
            "list component layer should not split semantics by platform marker `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "cargo test -p ui-list list_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable -- --nocapture",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include reduced-motion/SSR/wasm evidence marker `{required}`."
        );
    }
}

#[test]
fn list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let motion = load_source("motion");
    let styles = load_source("styles");
    let ui_motion_spring = load_source("ui_motion_spring");
    let ui_motion_lib = load_source("ui_motion_lib");
    let platforms_script = load_source("platforms_script");

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include list motion-contract evidence marker `{required}`."
        );
    }

    for required in [
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {",
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {",
        "damping: if value.damping.is_finite() && value.damping > 0.0 {",
        "mass: if value.mass.is_finite() && value.mass > 0.0 {",
        "precision: if value.precision.is_finite() && value.precision > 0.0 {",
        "pub fn sanitize_motion(motion: ListMotion) -> ListMotion {",
        "pub fn resolve_motion(motion: ListMotion) -> (ListMotion, bool) {",
        "pub fn attach_motion(",
        "ui_visual_primitive::active_highlight::attach_active_highlight_motion(",
        "pub fn attach_section_motion(",
        "ui_illustrated_message::motion::attach_motion(node_ref, sanitize_section_motion(motion));",
    ] {
        assert!(
            motion.contains(required),
            "list motion source should keep component contract marker `{required}`."
        );
    }

    for required in [
        "crate::motion::attach_motion(",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "list view should mount motion contract via `{required}`."
        );
    }

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring.contains(required),
            "ui-motion spring should keep reduced-motion short-circuit marker `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion non-wasm backend should keep safe no-op marker `{required}`."
        );
    }

    assert!(
        styles.contains(
            "var(--ui-motion-duration-medium, var(--ui-fallback-text-field-motion-duration))"
        ),
        "list styles should keep ui-motion tokenized transition contract in component scope."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platforms_script.contains(script_needle),
        "platform gate should include list motion contractualization command."
    );
}

#[test]
fn list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates() {
    let check2 = load_source("check2");
    let perf_script = load_source("perf_script");
    let docs_shell = load_source("docs_shell");
    let todo_plan = load_source("todo_plan");
    let view = load_source("view");
    let motion = load_source("motion");

    for required in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates",
    ] {
        assert!(
            perf_script.contains(required),
            "performance gate script should keep blocking governance command `{required}`."
        );
    }

    for required in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "\"list\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell.contains(required),
            "docs shell should keep list perf budget/probe marker `{required}`."
        );
    }

    assert!(
        todo_plan.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "performance governance should keep explicit render_count automation follow-up in TODO plan."
    );

    for required in [
        "on:keydown=on_key_down",
        "on:pointerdown=move |_| {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "crate::motion::attach_motion(",
        "Effect::new(move |_| {",
    ] {
        assert!(
            view.contains(required) || motion.contains(required),
            "list perf attribution should keep interaction/render/motion path marker `{required}`."
        );
    }

    for required in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "`scripts/check-ui-performance.sh` 已纳入 `list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates` 阻断命令",
        "`apps/docs-app/src/pages/components/shell.rs` 在 `component_page_perf_budget` 为 `\"list\"` 提供 `UiPerfBudget`（mount/update/heap）并由 `UiPerfProbe` 输出 `data-perf-*`",
        "`docs/plan/TODO.md` 保留 `render_count` 自动化补齐项（当前以可重复 perf probe 基线替代精确计数）",
        "回归锁定：`components/list/test/semantics.rs::list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates` 与 `components/list/test/list_module_semantics.rs::list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates`。",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2 should record performance governance evidence `{required}`."
        );
    }
}

#[test]
fn list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let local_semantics = include_str!("semantics.rs");
    let aggregated_semantics =
        include_str!("../../../components/list/test/list_module_semantics.rs");
    let view = load_source("view");
    let todo_plan = load_source("todo_plan");

    for required_test in [
        "fn list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions()",
        "fn list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates()",
        "fn list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "role=aria.attrs.role",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-state=move || option_a11y().data_state",
        "data-selected=move || option_a11y().data_selected",
        "data-focused=move || option_a11y().data_focused",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
    ] {
        assert!(
            view.contains(marker),
            "list view should keep semantic/focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_plan.contains(marker),
            "render_count follow-up governance should include `{marker}`."
        );
    }
}

#[test]
fn list_semantics_and_performance_script_covers_contract() {
    let perf_script = load_source("perf_script");

    for marker in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn list_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2 = load_source("check2");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions",
        "list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates",
        "list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(marker),
            "list check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn list_view_macro_complexity_is_split_into_semantic_subrenders() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = load_source("view_macro_script");

    assert!(
        check2.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "check2.md should mark view-macro complexity governance item as completed."
    );

    for required in [
        "fn render_list_option(",
        "render_list_option(",
        ".map(|(index, label)| {",
        "collect_view()",
    ] {
        assert!(
            view.contains(required),
            "list view should keep semantic subrender split marker `{required}`."
        );
    }

    let view_macro_count = view.matches("view! {").count();
    assert!(
        view_macro_count <= 4,
        "list view macro complexity should stay bounded after semantic split; expected <= 4, found {view_macro_count}."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include list complexity check command."
    );

    for required in [
        "render_list_option",
        "`view.rs` 已将 option 行渲染从主 `List` `view!` 中下沉到局部函数 `render_list_option(...)`",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_macro_complexity_is_split_into_semantic_subrenders",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include view-macro complexity evidence marker `{required}`."
        );
    }
}

#[test]
fn list_view_functional_split_prefers_plain_functions_over_local_components() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = load_source("view_macro_script");

    assert!(
        check2.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "check2.md should mark function-first split governance item as completed."
    );

    for required in [
        "fn render_list_option(",
        ") -> impl IntoView {",
        "render_list_option(",
        "#[component]\npub fn List(",
        "#[component]\npub fn ListItem(",
        "#[component]\npub fn ListSection(",
    ] {
        assert!(
            view.contains(required),
            "list view should keep function-first split marker `{required}`."
        );
    }

    let component_count = view.matches("#[component]").count();
    assert_eq!(
        component_count, 3,
        "list view should keep exactly three public component boundaries; found {component_count}."
    );

    for forbidden in [
        "#[component]\nfn render_list_option(",
        "#[component]\nfn render_option",
    ] {
        assert!(
            !view.contains(forbidden),
            "list should not escalate lightweight local fragments into component `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include list function-first command."
    );

    for required in [
        "render_list_option",
        "仅保留 3 个公共 `#[component]` 边界（`List`/`ListItem`/`ListSection`）",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_functional_split_prefers_plain_functions_over_local_components",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include function-first split evidence marker `{required}`."
        );
    }
}

#[test]
fn list_static_fragments_are_constantized_with_stable_a11y_markers() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let script = load_source("view_macro_script");

    assert!(
        check2.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "check2.md should mark static fragment constantization governance item as completed."
    );

    for required in [
        "const LISTBOX_HIGHLIGHT_CLASS: &str = \"ui-active-highlight\";",
        "const LISTBOX_HIGHLIGHT_SLOT: &str = \"listbox-highlight\";",
        "const LIST_ITEM_DIVIDER_CLASS: &str = \"ui-listbox-item__divider\";",
        "const LIST_ITEM_DIVIDER_SLOT: &str = \"listbox-item-divider\";",
        "const LIST_SECTION_DIVIDER_CLASS: &str = \"ui-listbox-section__divider\";",
        "const LIST_SECTION_DIVIDER_SLOT: &str = \"listbox-section-divider\";",
        "class=LISTBOX_HIGHLIGHT_CLASS",
        "data-slot=LISTBOX_HIGHLIGHT_SLOT",
        "class=LIST_ITEM_DIVIDER_CLASS",
        "data-slot=LIST_ITEM_DIVIDER_SLOT",
        "class=LIST_SECTION_DIVIDER_CLASS",
        "data-slot=LIST_SECTION_DIVIDER_SLOT",
    ] {
        assert!(
            view.contains(required),
            "list view should keep static fragment constantization marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_static_fragments_are_constantized_with_stable_a11y_markers";
    assert!(
        script.contains(script_needle),
        "view-macro gate script should include list static fragment constantization command."
    );

    for required in [
        "LISTBOX_HIGHLIGHT_CLASS",
        "LIST_ITEM_DIVIDER_CLASS",
        "LIST_SECTION_DIVIDER_CLASS",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_static_fragments_are_constantized_with_stable_a11y_markers",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include static fragment constantization evidence marker `{required}`."
        );
    }
}

#[test]
fn list_inner_html_usage_is_explicitly_na_and_guarded_locally() {
    for path in ["mod", "logic", "styles", "view", "motion"] {
        let source = load_source(path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "list source `{path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("docs_collections");
    let list_section = list_docs_section(docs_source);
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !list_section.contains(forbidden),
            "list docs section must not contain raw-html injection token `{forbidden}`."
        );
    }

    let check2 = load_source("check2");
    for required in [
        "- [x] `inner_html` 使用约束：",
        "零注入面",
        "components/list/test/semantics.rs::list_inner_html_usage_is_explicitly_na_and_guarded_locally",
        "components/list/test/list_module_semantics.rs::list_inner_html_usage_is_explicitly_na_and_guarded",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include inner_html governance evidence marker `{required}`."
        );
    }

    let script = load_source("inner_html_script");
    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script.contains(script_needle),
        "inner-html gate script should include list contract command."
    );
}

#[test]
fn list_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");
    let check2 = load_source("check2");
    let list_cargo = load_source("list_cargo");
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let docs_app_lib = load_source("docs_app_lib");
    let debug_overlay = load_source("debug_overlay");
    let trace = load_source("ui_headless_trace");
    let wasm_debug_script = load_source("wasm_debug_script");
    let docs_source = load_source("docs_collections");
    let list_section = list_docs_section(docs_source);

    for required in ["[features]", "default = []"] {
        assert!(
            list_cargo.contains(required),
            "list crate feature boundary should include `{required}`."
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing"] {
        assert!(
            !list_cargo.contains(forbidden),
            "list crate should not leak wasm-debug feature `{forbidden}`."
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui shared wasm-debug feature graph should include `{required}`."
        );
    }

    for forbidden in [
        "list-wasm-debug =",
        "list_wasm_debug =",
        "component-list\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui should not expose list-local wasm debug toggle `{forbidden}`."
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui root should keep global wasm-debug isolation marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_lib.contains(required),
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
            trace.contains(required) || debug_overlay.contains(required),
            "global trace/debug-overlay contract should keep marker `{required}`."
        );
    }

    for required in [
        "on:keydown=on_key_down",
        "on:pointerdown=move |_| {",
        "on:pointermove=move |_| on_option_pointer_move_for_move.run(index)",
        "data-selection-mode=selection_sources.selection_mode_attr",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-default-selection-source=selection_sources.default_selection_source_attr",
        "data-selection-change-source=selection_sources.selection_change_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
    ] {
        assert!(
            view.contains(required),
            "list should keep state/source and interaction markers for replayability via `{required}`."
        );
    }

    for required in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "data-slot=\"list-workbench\"",
        "\"selected: \"",
        "set_workbench_selected.set(next)",
    ] {
        assert!(
            list_section.contains(required),
            "list docs section should keep minimal replay path marker `{required}`."
        );
    }

    for source in [module, logic, styles, view, motion] {
        for forbidden in [
            "use_ui_trace(",
            "provide_ui_trace(",
            "trace.emit(",
            "debug_overlay",
            "request_replay",
            "replay",
            "trace_id",
            "wasm_debug_proxy!",
            "observability::",
            "#[prop(optional)] debug",
        ] {
            assert!(
                !source.contains(forbidden),
                "list runtime/public contract should not leak wasm-debug internals `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        wasm_debug_script.contains(script_needle),
        "wasm-debug gate script should include list contract command."
    );

    for required in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "本组件判定：N/A（组件级不自建 wasm 调试/回放管线）",
        "components/list/test/semantics.rs::list_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "components/list/test/list_module_semantics.rs::list_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include wasm-debug governance evidence marker `{required}`."
        );
    }
}

#[test]
fn list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let check2 = load_source("check2");
    let docs_source = load_source("docs_collections");
    let list_section = list_docs_section(docs_source);
    let readme = load_source("readme");
    let dx_script = load_source("dx_script");

    for required in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"list-workbench-controls\"",
        "data-slot=\"list-workbench-canvas\"",
        "data-slot=\"list-workbench\"",
    ] {
        assert!(
            list_section.contains(required),
            "list docs section should keep DX workbench marker `{required}`."
        );
    }

    for required in [
        "set_workbench_sync_active.set(event_target_checked(&ev))",
        "set_workbench_disable_last.set(event_target_checked(&ev))",
        "set_workbench_root_disabled.set(event_target_checked(&ev))",
        "set_workbench_custom_class.set(event_target_checked(&ev))",
        "set_workbench_selected.set(next)",
        "\"selected: \"",
    ] {
        assert!(
            list_section.contains(required),
            "list workbench should keep context-preserving interaction marker `{required}`."
        );
    }

    for required in [
        "## Config (Workbench Settings)",
        "## CSS Test (Scoped CSS)",
        "Workbench 使用统一 `Playground controls` 面板调节：",
        "重点是让语义状态（selection/disabled/data-*）在同一画布连续观察，降低回归定位成本。",
        "CSS Test 面板用于局部覆盖与回放",
    ] {
        assert!(
            readme.contains(required),
            "list README should keep DX contract marker `{required}`."
        );
    }

    for forbidden in [
        "localStorage",
        "sessionStorage",
        "save_list_workbench",
        "load_list_workbench",
        "persist_list",
    ] {
        assert!(
            !list_section.contains(forbidden),
            "list DX scope should keep optional persist-state as N/A; found `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        dx_script.contains(script_needle),
        "dx gate script should include list DX contract command."
    );

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "本组件已提供 Workbench 隔离画布（含 settings/code/css-test）",
        "可选状态保留在 List scope 判定为 N/A（不引入本地持久化存储）",
        "components/list/test/semantics.rs::list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "components/list/test/list_module_semantics.rs::list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include DX governance evidence marker `{required}`."
        );
    }
}

#[test]
fn list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let check2 = load_source("check2");
    let docs_source = load_source("docs_collections");
    let list_section = list_docs_section(docs_source);
    let readme = load_source("readme");
    let dx_script = load_source("dx_script");

    for required in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"状态矩阵 State Matrix（受控 / 非受控）\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-slot=\"list-state-matrix\"",
        "data-slot=\"list-streaming-snapshot\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "data-ui-output-state=\"streaming\"",
        "aria_label=\"Matrix uncontrolled list\".to_string()",
        "aria_label=\"Matrix controlled list\".to_string()",
        "code_imports=list_code_imports.clone()",
        "code_imports=list_code_imports",
    ] {
        assert!(
            list_section.contains(required),
            "list docs section should keep copy-paste-ready marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"list-source-first\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy starter\"",
        "data-slot=\"list-source-paths\"",
        "data-slot=\"list-source-prerequisites\"",
    ] {
        assert!(
            list_section.contains(required),
            "list docs section should keep source-first marker `{required}`."
        );
    }

    for required in [
        "## 状态矩阵（受控 / 非受控）",
        "## Streaming/Snapshot Display",
        "## Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
    ] {
        assert!(
            readme.contains(required),
            "list README should keep docs-as-product marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script.contains(script_needle),
        "dx gate script should include list docs-as-product contract command."
    );

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/list/test/semantics.rs::list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "components/list/test/list_module_semantics.rs::list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include docs-as-product governance evidence marker `{required}`."
        );
    }
}

#[test]
fn list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let contract_hygiene_script = load_source("contract_hygiene_script");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-motion-duration-medium, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-motion-ease-emphasized, var(--ui-fallback-text-field-motion-easing))",
    ] {
        assert!(
            styles.contains(required),
            "list styles should keep defensive fallback-chain marker `{required}`."
        );
    }

    for line in styles.lines().filter(|line| line.contains("var(--ui-")) {
        assert!(
            line.contains("var(--ui-fallback-"),
            "list styles should use two-level ui-theme fallback chain; offending line: `{line}`."
        );
    }

    for forbidden in ["#000", "#fff", " 8px", " 12px"] {
        assert!(
            !styles.contains(forbidden),
            "list styles should not include hardcoded `{forbidden}` terminals in component scope."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene script should include list defensive-variables command."
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include defensive-variables evidence marker `{required}`."
        );
    }
}

#[test]
fn list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules() {
    let ui_components_css = load_source("ui_components_css");
    let view = load_source("view");
    let check2 = load_source("check2");
    let contract_hygiene_script = load_source("contract_hygiene_script");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should keep list cascade-layer marker `{required}`."
        );
    }

    for forbidden in [
        "style=",
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=\"transform:",
        "style=\"display:",
        "style=move || format!(\"top:",
        "style=move || format!(\"left:",
        "style=move || format!(\"transform:",
    ] {
        assert!(
            !view.contains(forbidden),
            "list view should reject plain inline style injection marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene script should include list cascade-layer command."
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "已核验（聚合层）：`crates/ui/src/css.rs::push_components_css` 使用 `out.push_str(\"\\n@layer ui {\\n\")` 包裹组件样式并在末尾闭合",
        "已核验（运行时样式边界）：`components/list/src/view.rs` 不含 `style=`/`style:\\\"top`/`style:\\\"left` 等普通内联样式写法",
        "N/A（list，运行时数值注入）：当前 `List/ListItem/ListSection` 无运行时动态样式写入路径，后续若引入仅允许 CSS 自定义变量注入（`style:--ui-*`）",
        "list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include cascade-layer evidence marker `{required}`."
        );
    }
}

#[test]
fn list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    let check2 = load_source("check2");
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let protocol = load_source("protocol");
    let mod_source = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");

    for required in [
        "pub enum ListComponentSchemaVersion {",
        "V1,",
        "pub struct ListComponentSpec {",
        "pub schema_version: ListComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "list protocol should keep v1 schema marker `{required}`."
        );
    }

    for required in ["schema_version = \"1\"", "values = [\"v1\"]"] {
        assert!(
            manifest.contains(required),
            "list Component.toml should keep v1 contract marker `{required}`."
        );
    }

    for required in ["pub enum ListAgentSchemaVersion {", "V1,"] {
        assert!(
            rbi.contains(required),
            "list RBI should keep v1 contract marker `{required}`."
        );
    }

    let combined = [
        mod_source, logic, view, styles, motion, protocol, manifest, rbi,
    ]
    .join("\n");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, list should not introduce migration marker `{forbidden}`."
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `List` 变更未引入跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include version-deprecation migration marker `{required}`."
        );
    }
}

#[test]
fn list_version_deprecation_migration_script_covers_engineering_gate() {
    let engineering_script = load_source("engineering_script");

    let marker = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        engineering_script.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let motion = load_source("motion");
    let protocol = load_source("protocol");
    let check2 = load_source("check2");
    let ui_components_cargo = load_source("ui_components_cargo");
    let engineering_script = load_source("engineering_script");
    let button_view = include_str!("../../button/src/view.rs");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum ListComponentSchemaVersion {",
        "pub struct ListComponentSpec {",
        "#[serde(default)]",
        "pub schema_version: ListComponentSchemaVersion,",
        "#[cfg(test)]",
        "#[path = \"../test/protocol.rs\"]",
    ] {
        assert!(
            protocol.contains(required),
            "list protocol should keep structured serde schema contract marker `{required}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "migrate_v1_to_v2(",
    ] {
        assert!(
            !protocol.contains(forbidden),
            "list protocol should avoid ad-hoc serde/migration helper token `{forbidden}`."
        );
    }

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view.contains(required),
            "engineering baseline should keep canonical tracing semantics marker `{required}`."
        );
    }

    for forbidden_feature in [
        "list-wasm-debug =",
        "list_wasm_debug =",
        "component-list\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden_feature),
            "list should not define component-local tracing feature `{forbidden_feature}`."
        );
    }

    let combined = [module, logic, view, styles, motion, protocol].join("\n");
    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::list::",
        "const LIST_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "list should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }

    for source in [module, logic, view, styles, motion] {
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
                "list engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !load_source("mod").contains("web_sys"),
        "list public module boundary should not leak web_sys types."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        engineering_script.contains(script_needle),
        "engineering gate script should include list contract command."
    );

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "components/list/src/protocol.rs",
        "list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include engineering governance evidence marker `{required}`."
        );
    }
}

#[test]
fn list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");
    let protocol = load_source("protocol");
    let combined = format!("{module}\n{logic}\n{styles}\n{view}\n{motion}\n{protocol}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "list non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(\"ui-listbox-item\")];",
        "Cow::Borrowed(\"ui-listbox-item--custom-class\")",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-listbox-section\")",
        "Cow::Borrowed(state.heading_tone_class)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic.contains(required),
            "list logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in [
        "DEFAULT_LIST_CLASS_NAME.to_string()",
        "\"ui-listbox-item\".to_string()",
        "\"ui-listbox-item--selected\".to_string()",
        "\"ui-listbox-item--focused\".to_string()",
        "\"ui-listbox-item--disabled\".to_string()",
        "\"ui-listbox-item--selection-indicator\".to_string()",
        "\"ui-listbox-item--divider\".to_string()",
        "\"ui-listbox-item--custom-class\".to_string()",
        "\"ui-listbox-section\".to_string()",
        "\"ui-listbox-section--has-title\".to_string()",
        "\"ui-listbox-section--empty\".to_string()",
        "\"ui-listbox-section--disabled\".to_string()",
        "\"ui-listbox-section--sticky-heading\".to_string()",
        "\"ui-listbox-section--divided\".to_string()",
        "\"ui-listbox-section--custom-class\".to_string()",
        "String::from(\"ui-listbox-item\")",
        "String::from(\"ui-listbox-section\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "list string hotspot contract should avoid `{forbidden}`."
        );
    }
}

#[test]
fn list_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("engineering_script");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering gate script should include list rust-hygiene command `{needle}`."
        );
    }
}

#[test]
fn list_check2_marks_rust_hygiene_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/list/test/semantics.rs::list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/list/test/semantics.rs::list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/list/test/semantics.rs::list_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "components/list/test/list_module_semantics.rs::list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/list/test/list_module_semantics.rs::list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/list/test/list_module_semantics.rs::list_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-engineering.sh",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include rust-hygiene evidence marker `{required}`."
        );
    }
}
