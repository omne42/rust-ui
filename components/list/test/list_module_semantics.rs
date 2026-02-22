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

fn playground_block<'a>(source: &'a str, title: &str) -> &'a str {
    let marker = format!("title=\"{title}\"");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing playground title `{marker}`"));
    let tail = &source[start..];
    let next = tail.find("<Playground").unwrap_or(tail.len());
    &tail[..next]
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

#[test]
fn list_module_reexports_canonical_list_contracts() {
    let source = load_source("src/list/mod.rs");

    for needle in [
        "pub use logic::{ListItemSelectionIndicator, ListSectionHeadingTone, ListState};",
        "pub use motion::ListMotion;",
        "pub use motion::ListSectionMotion;",
        "pub use view::{List, ListItem, ListSection};",
    ] {
        assert!(
            source.contains(needle),
            "list module should expose canonical `{needle}`."
        );
    }

    for removed in [
        "pub use crate::listbox::ListBox as ListView;",
        "pub use crate::item::Item;",
    ] {
        assert!(
            !source.contains(removed),
            "list module should not keep removed alias `{removed}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn crate_root_registers_list_and_hides_listbox_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod list;"),
        "crate root should include `pub mod list;`."
    );
    assert!(
        !source.contains("mod listbox;"),
        "crate root should not keep legacy listbox module."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn list_docs_use_list_family_slugs_and_components() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let collections_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "component_doc!(\"List\", \"list\", \"Collections\", collections::list)",
        "\"ListItem\"",
        "\"list-item\"",
        "\"ListSection\"",
        "\"list-section\"",
        "collections_extra::list_item",
        "collections_extra::list_section",
    ] {
        assert!(
            pages_source.contains(needle),
            "components catalog should include `{needle}` for list family docs."
        );
    }

    for needle in ["title=\"List\"", "slug=\"list\"", "<List"] {
        assert!(
            list_section.contains(needle),
            "collections docs should include `{needle}` for the canonical List page."
        );
    }

    for needle in [
        "pub(super) fn list_item() -> AnyView",
        "title=\"ListItem\"",
        "slug=\"list-item\"",
        "<ListItem",
        "pub(super) fn list_section() -> AnyView",
        "title=\"ListSection\"",
        "slug=\"list-section\"",
        "<ListSection",
    ] {
        assert!(
            collections_extra_source.contains(needle),
            "collections-extra docs should include `{needle}` for list item/section pages."
        );
    }

    assert!(
        mod_source.contains("\"list\" => &[\"list\", \"list-item\", \"list-section\"]"),
        "components mapping should point `list` to list/list-item/list-section."
    );
    assert!(
        !mod_source.contains("\"list-box\" =>"),
        "components mapping should not contain the removed `list-box` alias."
    );
}

#[test]
fn list_docs_page_exposes_showcase_and_workbench_contracts() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = list_docs_section(&collections_source);

    for needle in [
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"list-showcase\"",
        "data-slot=\"list-workbench-controls\"",
        "data-slot=\"list-workbench-canvas\"",
        "is_active_index_synced_to_selected=false",
        "disabled_indices=vec![2]",
        "is_disabled=true",
        "items=empty_items",
    ] {
        assert!(
            section.contains(needle),
            "list docs section should contain `{needle}` for showcase/workbench coverage."
        );
    }
}

#[test]
fn list_readme_documents_display_config_code_css_test_sections() {
    let source = load_source("src/list/README.md");

    for needle in [
        "## 展示 (Display)",
        "## Config (Workbench Settings)",
        "## Code (Workbench Snippet)",
        "## CSS Test (Scoped CSS)",
        "collections.rs` 的 `list()`",
        "is_active_index_synced_to_selected=false",
        "test_css_source",
        "test_config_signal",
    ] {
        assert!(
            source.contains(needle),
            "list README should contain `{needle}` to lock workbench docs contract."
        );
    }
}

#[test]
fn list_dx_hello_world_is_minimal_and_state_machine_free() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let hello_playground = playground_block(list_section, "Hello World (Uncontrolled)");
    let readme_source = load_source("src/list/README.md");
    let readme_hello = markdown_section(&readme_source, "Hello World");

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "id_base=\"docs-list-hello\".to_string()",
        "aria_label=\"Settings navigation\".to_string()",
    ] {
        assert!(
            hello_playground.contains(needle),
            "list docs hello path should include `{needle}`."
        );
    }

    for forbidden in ["selected_index=", "on_selected_index_change=", "state="] {
        assert!(
            !hello_playground.contains(forbidden),
            "list docs hello path should not require advanced state wiring `{forbidden}`."
        );
    }

    for needle in [
        "let items: Arc<[String]> = vec![\"Overview\".to_string(), \"Billing\".to_string()].into();",
        "view! { <List id_base=\"list-hello\".to_string() items=items aria_label=\"Settings navigation\".to_string() /> }",
    ] {
        assert!(
            readme_hello.contains(needle),
            "list README hello path should include `{needle}`."
        );
    }

    for forbidden in ["selected_index=", "on_selected_index_change=", "state="] {
        assert!(
            !readme_hello.contains(forbidden),
            "list README hello path should not require advanced state wiring `{forbidden}`."
        );
    }
}

#[test]
fn list_composite_api_prefers_explicit_parent_item_structure() {
    let view_source = load_source("src/list/view.rs");
    let collections_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub fn ListItem(",
        "pub fn ListSection(",
        "children: Children,",
    ] {
        assert!(
            view_source.contains(needle),
            "list API should keep explicit composition marker `{needle}`."
        );
    }

    for needle in ["<ListSection", "<ListItem"] {
        assert!(
            collections_extra_source.contains(needle),
            "docs should demonstrate explicit parent-item composition via `{needle}`."
        );
    }

    for forbidden in [
        "labels: Vec<String>",
        "titles: Vec<String>",
        "panels: Vec<",
        "item_specs: Vec<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "list API should not expose parallel-array/config sugar `{forbidden}`."
        );
    }
}

#[test]
fn list_macro_micro_dragging_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "list interaction/motion path should stay focused on pointer/listbox highlight contract `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not expose dragging macro/micro state-machine contract `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_two_pass_geometry_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "use_listbox(ListBoxOptions {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "list should keep listbox interaction + highlight motion path `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement two-pass geometry rendering contract `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_registration_protocol_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "pub fn is_disabled_index(disabled_indices: &HashSet<usize>, index: usize) -> bool",
        "disabled_indices.contains(&index)",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "list should keep deterministic item order and membership-only disabled lookup marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement dynamic child registration protocol marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_slot_projection_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "data-slot=\"listbox-options\"",
        "items.iter().cloned().enumerate()",
        ".collect_view()",
        "attach_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "list should keep eager listbox rendering contract marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement slot projection lifecycle contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_env_streams_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "attach_motion(",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "list should keep direct listbox interaction path marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement environment subscription stream contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_event_light_cone_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "use_listbox(ListBoxOptions {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "list should keep single-listbox interaction contract marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement event light-cone bulk contract marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_causality_bus_contract_is_not_applicable_in_current_scope() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "#[prop(optional)] on_action: Option<Callback<usize>>",
        "use_listbox(ListBoxOptions {",
        "on_action,",
        "on:click=move |_| {",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "list should keep direct user-intent callback path marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not implement unified causality bus marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");

    for needle in [
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
            view_source.contains(needle),
            "list view should keep a11y+i18n+l10n contract marker `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_selection_status_text(",
        "DEFAULT_SELECTED_TEXT",
        "DEFAULT_UNSELECTED_TEXT",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should keep overridable fallback-text contract marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains("\"selected\" } else { \"not selected\"")
            && !view_source.contains("{ \"selected\" } else { \"not selected\" }"),
        "list view should not hardcode selection copy directly in render path."
    );
}

#[test]
fn list_state_observability_contract_uses_stable_data_and_aria_markers() {
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");

    for needle in [
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
            view_source.contains(needle),
            "list view should expose stable observability marker `{needle}`."
        );
    }

    for needle in [
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
            logic_source.contains(needle),
            "list logic should keep closed-set source marker mapping `{needle}`."
        );
    }
}

#[test]
fn list_styles_depend_on_explicit_state_markers_not_fragile_dom_guessing() {
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
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
            styles_source.contains(needle),
            "list styles should express visual state by explicit marker selector `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "list styles should not use fragile structural selector `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:", "top: 10px", "left: 10px"] {
        assert!(
            !view_source.contains(forbidden),
            "list view should not inject business styling via inline style marker `{forbidden}`."
        );
    }
}

#[test]
fn list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions() {
    let checklist_source = load_source("../../components/list/check2.md");
    let component_semantics_source = load_source("../../components/list/test/semantics.rs");
    let component_logic_tests_source = load_source("../../components/list/test/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    assert!(
        checklist_source.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "components/list/check2.md should mark semantic-contract regression as completed."
    );

    for needle in [
        "list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
        "list_styles_depend_on_explicit_state_markers_not_fragile_dom_guessing",
    ] {
        assert!(
            component_semantics_source.contains(needle),
            "component semantics regression should keep `{needle}`."
        );
    }

    for needle in [
        "resolve_selection_source_state_covers_controlled_and_uncontrolled_matrix",
        "resolve_option_state_derives_selected_focused_and_disabled_bits",
        "list_interaction_source_attr_is_closed_set_for_none_keyboard_and_pointer",
    ] {
        assert!(
            component_logic_tests_source.contains(needle),
            "component logic regression should keep `{needle}`."
        );
    }

    for needle in [
        "on:keydown=on_key_down",
        "aria.handlers.on_key_down.run(ev.key())",
        "on:pointerdown=move |_| {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should keep keyboard/pointer semantic path marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "component layer should not fork list semantics by platform marker `{forbidden}`."
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
            !component_semantics_source.contains(forbidden)
                && !component_logic_tests_source.contains(forbidden),
            "semantic matrix should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn list_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("../../components/list/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep semantics-first testing rule `{needle}`."
        );
    }
}

#[test]
fn list_semantics_suite_is_contract_first_not_snapshot_only() {
    let local_semantics = load_source("../../components/list/test/semantics.rs");
    let local_logic_tests = load_source("../../components/list/test/logic.rs");
    let module_source = load_source("src/list/mod.rs");

    for needle in [
        "list_a11y_i18n_l10n_contract_is_mounted_and_text_is_overridable",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
        "list_semantic_contract_tests_cover_matrix_without_snapshot_only_assertions",
        "list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            local_semantics.contains(needle),
            "list semantics suite should keep contract-first assertion `{needle}`."
        );
    }

    for needle in [
        "resolve_selection_source_state_covers_controlled_and_uncontrolled_matrix",
        "resolve_option_state_derives_selected_focused_and_disabled_bits",
        "list_interaction_source_attr_is_closed_set_for_none_keyboard_and_pointer",
    ] {
        assert!(
            local_logic_tests.contains(needle),
            "list logic regression should keep semantic matrix axis `{needle}`."
        );
    }

    assert!(
        module_source.contains("#[path = \"../test/semantics.rs\"]"),
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
            !local_semantics.contains(forbidden) && !local_logic_tests.contains(forbidden),
            "list semantic suite should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/list/view.rs");
    let local_semantics = load_source("../../components/list/test/semantics.rs");
    let aggregated_semantics = load_source("tests/list_module_semantics.rs");

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
            view_source.contains(marker),
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
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should include `{needle}`."
        );
    }
}

#[test]
fn list_check2_marks_semantics_first_testing_contract_complete() {
    let checklist_source = load_source("../../components/list/check2.md");

    for needle in [
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
            checklist_source.contains(needle),
            "components/list/check2.md should keep semantics-first testing evidence `{needle}`."
        );
    }
}

#[test]
fn list_view_delegates_interaction_and_option_a11y_to_ui_headless() {
    let source = load_source("src/list/view.rs");

    for needle in [
        "use_listbox(ListBoxOptions {",
        "use_focus_ring(FocusRingOptions {",
        "listbox_option_a11y_attrs(ListBoxOptionA11yInput {",
        "role=move || option_a11y().role",
        "aria-selected=move || option_a11y().aria_selected",
        "aria-disabled=move || option_a11y().aria_disabled",
        "data-state=move || option_a11y().data_state",
    ] {
        assert!(
            source.contains(needle),
            "list view should mount ui-headless interaction/a11y contract `{needle}`."
        );
    }
}

#[test]
fn list_motion_contract_is_mapped_in_component_motion_layer() {
    let motion_source = load_source("src/list/motion.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "pub fn resolve_motion(motion: ListMotion) -> (ListMotion, bool)",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
        "pub fn attach_section_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "list motion layer should expose `{needle}`."
        );
    }

    assert!(
        view_source.contains("crate::motion::attach_motion("),
        "list view should bind highlight animation via component motion layer."
    );
    assert!(
        !view_source.contains("attach_active_highlight_motion("),
        "list view should not call active-highlight driver directly."
    );
}

#[test]
fn list_view_exposes_lang_and_dir_via_headless_locale_attrs() {
    let source = load_source("src/list/view.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=list_lang.clone()",
        "dir=list_dir",
    ] {
        assert!(
            source.contains(needle),
            "list view should expose locale contract marker `{needle}`."
        );
    }
}

#[test]
fn list_view_public_props_follow_is_on_default_naming_contract() {
    let source = load_source("src/list/view.rs");

    for needle in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional, default = 0)] default_active_index: usize",
        "#[prop(optional, default = true)] is_active_index_synced_to_selected: bool",
        "#[prop(optional)] is_selected: bool",
        "#[prop(optional)] is_focused: bool",
        "#[prop(optional)] is_selection_indicator_visible: bool",
        "#[prop(optional)] is_divider_visible: bool",
        "#[prop(optional)] is_sticky_heading: bool",
    ] {
        assert!(
            source.contains(needle),
            "list view public prop naming should include `{needle}`."
        );
    }

    for legacy in [
        "selected_index: ReadSignal<Option<usize>>",
        "set_selected_index: WriteSignal<Option<usize>>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = 0)] default_index: usize",
        "#[prop(optional, default = true)] sync_active_index_to_selected: bool",
        "#[prop(optional)] selected: bool",
        "#[prop(optional)] focused: bool",
        "#[prop(optional)] show_selection_indicator: bool",
        "#[prop(optional)] has_divider: bool",
        "#[prop(optional)] sticky_heading: bool",
        "#[prop(optional)] show_divider: bool",
    ] {
        assert!(
            !source.contains(legacy),
            "list view should not keep legacy prop alias `{legacy}`."
        );
    }
}

#[test]
fn list_defaults_are_single_sourced_in_logic_layer() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "pub fn normalize_list_class_name(",
        "pub fn normalize_callbacks(",
        "pub fn normalize_item_count(",
        "pub fn resolve_title_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should own default normalization via `{needle}`.",
        );
    }

    for needle in [
        "let class = logic::normalize_list_class_name(class_name);",
        "logic::item::normalize_callbacks(on_press, on_pointer_move)",
        "let resolved_item_count = logic::section::normalize_item_count(item_count);",
        "let title_text = logic::section::resolve_title_text(title);",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should consume logic default normalization `{needle}`.",
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
            !view_source.contains(forbidden),
            "list view should not keep default fallback branch `{forbidden}`.",
        );
    }
}

#[test]
fn list_state_normalization_is_concentrated_in_logic_layer() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "pub struct ListOptionsAxisInput",
        "pub struct ListOptionStateInput",
        "pub fn normalize_options_axis(",
        "pub fn resolve_option_state(",
        "pub fn is_disabled_index(",
        "pub fn is_interaction_blocked(",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should own typed state normalization via `{needle}`.",
        );
    }

    for needle in [
        "let options_axis = logic::normalize_options_axis(logic::ListOptionsAxisInput {",
        "let option_state = logic::resolve_option_state(logic::ListOptionStateInput {",
        "logic::is_disabled_index(&disabled_indices, index)",
        "let is_interaction_blocked = logic::item::is_interaction_blocked(is_disabled);",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should consume logic-normalized state contract `{needle}`.",
        );
    }

    for forbidden in [
        "let is_disabled = is_disabled || disabled_indices.contains(&index);",
        "if is_disabled {\n                    return;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "list view should not rebuild state rule `{forbidden}`.",
        );
    }
}

#[test]
fn list_discrete_state_axes_are_type_constrained() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "pub use primitives::{ListItemSelectionIndicator, ListSectionHeadingTone};",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ListMotion",
        "#[prop(optional)] heading_tone: logic::ListSectionHeadingTone",
        "#[prop(optional)] motion: ListSectionMotion",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "list discrete axes should stay type-constrained by `{needle}`.",
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
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "list should not model mutually-exclusive state with free-form/optional bool axis `{forbidden}`.",
        );
    }
}

#[test]
fn list_state_primitives_are_consumed_via_logic_without_business_store_binding() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "use ui_state_primitives::list as primitives;",
        "pub type ListState = primitives::ListViewState;",
        "primitives::resolve_view_state(",
        "primitives::resolve_item_state(",
        "primitives::resolve_section_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should consume ui-state-primitives contract via `{needle}`.",
        );
    }

    for needle in [
        "let selection_axis = logic::normalize_selection_axis(logic::ListSelectionAxisInput {",
        "let selected_state = use_controllable_state(",
        "logic::resolve_state(",
        "logic::item::resolve_state(logic::item::ListItemStateInput {",
        "logic::section::resolve_state(logic::section::ListSectionStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should consume logic-mapped state primitives via `{needle}`.",
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
            !view_source.contains(forbidden),
            "list view should not bypass logic boundary or bind business store `{forbidden}`.",
        );
    }
}

#[test]
fn list_theme_contract_is_token_first_and_ui_theme_backed() {
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let root_source = load_source("src/root.rs");
    let tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");
    let css_source = load_source("../ui-theme/src/css.rs");
    let wcag_source = load_source("../ui-theme/tests/wcag_contrast.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");

    for token_var in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-motion-duration-medium",
        "var(--ui-motion-ease-emphasized",
        "var(--ui-font-size-100",
        "var(--ui-line-height-100",
    ] {
        assert!(
            styles_source.contains(token_var),
            "List styles should consume ui-theme token variable `{token_var}`."
        );
    }

    let mut cursor = styles_source.as_str();
    while let Some(start) = cursor.find("var(--") {
        let tail = &cursor[start + 6..];
        let end = tail.find([',', ')']).unwrap_or(tail.len());
        let token = tail[..end].trim();
        assert!(
            token.starts_with("ui-"),
            "List styles should not introduce non-ui token namespace `{token}`."
        );
        cursor = &tail[end..];
    }

    assert!(
        !styles_source.contains("--ui-listbox-"),
        "List styles should not introduce private `--ui-listbox-*` theme token namespace."
    );

    for forbidden in [
        "Theme::",
        "ThemeContext",
        "theme_to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "List component assembly layer should not rebuild theme context; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ThemeContext",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub enum ThemeSystem",
    ] {
        assert!(
            theme_source.contains(needle),
            "ui-theme theme mapping should expose `{needle}`."
        );
    }

    for needle in [
        "Theme mapping happens in `theme.rs`; CSS variable emission happens in `css.rs`.",
        "pub enum TokenScale",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme token source should include `{needle}`."
        );
    }

    for needle in [
        "let system = theme.ctx.system.as_str();",
        "let color = theme.ctx.color.as_str();",
        "let scale = theme.ctx.scale.as_str();",
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css emission should include `{needle}`."
        );
    }

    for needle in [
        "data-theme-color=move || state.get().theme_color_attr",
        "data-theme-system=move || state.get().theme_system_attr",
        "data-theme-scale=move || state.get().theme_scale_attr",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should apply theme axis marker `{needle}`."
        );
    }

    for needle in [
        "for color in [ThemeColor::Light, ThemeColor::Dark, ThemeColor::Oled]",
        "WCAG 2.1 AA contrast failed",
    ] {
        assert!(
            wcag_source.contains(needle),
            "ui-theme WCAG regression should include `{needle}`."
        );
    }

    assert!(
        styling_spec_source.contains("Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量"),
        "styling spec should keep ui-theme token/theme/css SSOT contract."
    );
}

#[test]
fn list_component_styles_are_aggregated_by_css_rs_and_injected_via_ui_root() {
    let component_css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");

    for needle in [
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
    ] {
        assert!(
            component_css_source.contains(needle),
            "ui css aggregation should include list styles marker `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject aggregated component CSS marker `{needle}`."
        );
    }

    for needle in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
    ] {
        assert!(
            styles_source.contains(needle),
            "list styles should remain token-first via `{needle}`."
        );
    }

    assert!(
        !styles_source.contains("--ui-listbox-"),
        "list styles should not define component-private `--ui-listbox-*` token namespace."
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
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "list component should avoid utility-first marker `{forbidden}` in library layer."
        );
    }

    for forbidden in ["css!(", "style!(", "styled::", "StyleSheet::", "emotion::"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "list component should avoid css-in-rust default marker `{forbidden}`."
        );
    }
}

#[test]
fn list_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let checklist_source = load_source("src/list/check2.md");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state_source =
        load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let entrypoints_script_source = load_source("../../scripts/check-ui-entrypoints.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-list\")]",
        "pub use ui_list as list;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep marker `{needle}`."
        );
    }

    for needle in [
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
            css_source.contains(needle),
            "ui css entry should keep marker `{needle}`."
        );
    }

    for needle in [
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
            root_source.contains(needle),
            "UiRoot entry should keep centralized injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn List(", "ui-listbox"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic shared primitive, found `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui should not define `src/overlay_open.rs`."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui should not define `src/presence.rs`."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui should not define `src/a11y.rs`."
    );

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable_state_source.contains(needle)
                || headless_presence_source.contains(needle)
                || headless_a11y_source.contains(needle),
            "headless canonical primitive should keep marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script_source.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "components/list/test/semantics.rs::list_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/list/test/list_module_semantics.rs::list_ui_components_fixed_entry_files_follow_layered_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include fixed-entry evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_component_directory_has_standard_file_layout() {
    let checklist_source = load_source("../../components/list/check2.md");
    let module_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let component_files_script_source =
        load_source("../../scripts/check-ui-component-files.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let list_src_dir = manifest_dir.join("../../components/list/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            list_src_dir.join(required).exists(),
            "components/list/src should contain required file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !list_src_dir.join(forbidden).exists(),
            "components/list simple scope should not introduce `{forbidden}`."
        );
    }

    for needle in [
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
            module_source.contains(needle),
            "mod.rs should keep minimal stable export marker `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "mod render;", "mod spec;"] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export or drift to forbidden module `{forbidden}`."
        );
    }

    for needle in [
        "use ui_state_primitives::list as primitives;",
        "pub fn resolve_accessible_name(",
        "pub fn resolve_state(",
        "pub fn resolve_selection_source_state(",
        "pub fn normalize_options_axis(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "NodeRef<", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain view/headless/dom binding marker `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub const ITEM_CSS: &str = r#\"",
        "pub const SECTION_CSS: &str = r#\"",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep static token-first css marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "use_listbox(", "attach_active_highlight_motion("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not include runtime/interaction marker `{forbidden}`."
        );
    }

    for needle in [
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
            view_source.contains(needle),
            "view.rs should keep structure + headless mounting marker `{needle}`."
        );
    }

    for forbidden in ["resolve_view_state(", "mod render;", "render::"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not drift to forbidden structure marker `{forbidden}`."
        );
    }

    for needle in [
        "pub type ListMotion = ActiveHighlightMotion;",
        "pub fn sanitize_motion(",
        "pub fn resolve_motion(",
        "pub fn attach_motion(",
        "pub type ListSectionMotion = ui_illustrated_message::IllustratedMessageMotion;",
        "pub fn attach_section_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep motion-contract mapping marker `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "use_listbox(",
        "use_focus_ring(",
        "resolve_view_state(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should stay scoped away from view/headless/state-machine marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_component_directory_has_standard_file_layout";
    assert!(
        component_files_script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "components/list/test/semantics.rs::list_component_directory_has_standard_file_layout",
        "components/list/test/list_module_semantics.rs::list_component_directory_has_standard_file_layout",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include component-directory evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/list/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "list/check2.md should not keep unchecked checklist items after sequential verification."
    );
}

#[test]
fn list_check2_marks_async_scope_as_explicit_na() {
    let source = load_source("src/list/check2.md");

    assert!(
        source.contains("N/A：`List` 当前仅本地集合导航与选择，不包含远程请求/异步加载状态。"),
        "list/check2.md should explicitly mark async contract as N/A in current scope."
    );
}

#[test]
fn list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let source = load_source("src/list/check2.md");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "components/list/test/semantics.rs::list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "components/list/test/list_module_semantics.rs::list_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "list/check2.md should keep streaming-definition marker `{needle}`."
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
    let source = load_source("src/list/check2.md");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/list/test/semantics.rs::list_check2_documents_snapshot_as_default_baseline_capability",
        "components/list/test/list_module_semantics.rs::list_check2_documents_snapshot_as_default_baseline_capability",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "list/check2.md should keep snapshot-baseline marker `{needle}`."
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
    let source = load_source("src/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let manifest_source = load_source("src/list/Component.toml");
    let rbi_source = load_source("src/list/list.rbi");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");
    let list_signature = fn_signature_block(&view_source, "List");

    for needle in [
        "#[prop(into)] items: Arc<[String]>",
        "items.iter().cloned().enumerate()",
        "logic::resolve_state(",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle) || list_signature.contains(needle),
            "list snapshot baseline should keep complete-result render marker `{needle}`."
        );
    }

    for needle in [
        "[[capabilities]]\nname = \"snapshot_rendering\"\nenabled = true",
        "name = \"items\"",
        "ty = \"Arc<[String]>\"",
        "default = \"required\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "list manifest should keep snapshot baseline capability marker `{needle}`."
        );
    }

    assert!(
        rbi_source.contains("items: std::sync::Arc<[String]>,"),
        "list RBI should keep complete snapshot input projection for `items`."
    );

    for forbidden in [
        "stream_chunk",
        "token_delta",
        "partial_payload",
        "incremental_patch",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "list baseline snapshot render path should not depend on streaming-only token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list snapshot-stability command."
    );

    for needle in [
        "components/list/test/semantics.rs::list_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "components/list/test/list_module_semantics.rs::list_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "list/check2.md should keep snapshot-baseline evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let source = load_source("src/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let manifest_source = load_source("src/list/Component.toml");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
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
            source.contains(needle),
            "list/check2.md should keep streaming governance marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_attr()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_attr()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should keep streaming-governance semantic marker `{needle}`."
        );
    }

    for needle in [
        "ListAgentStreamSupport::Optional",
        "ListAgentStreamFallback::Snapshot",
        "ListAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should keep streaming-governance typed contract `{needle}`."
        );
    }

    for needle in [
        "name = \"stream_support\"",
        "values = [\"optional\"]",
        "name = \"stream_fallback\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "list manifest should keep streaming-governance marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback";
    assert!(
        streaming_script.contains(script_needle),
        "streaming gate script should include list streaming required/optional governance command."
    );
}

#[test]
fn list_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("../../components/list/check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(marker),
            "components/list/check2.md should keep e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn list_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_list_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let docs_section = list_docs_section(&docs_source);

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
            e2e_source.contains(marker),
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
            "list docs source should keep e2e semantic anchor `{marker}`."
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
            !e2e_source.contains(forbidden),
            "list e2e contract should avoid flaky/text/snapshot selector token `{forbidden}`."
        );
    }
}

#[test]
fn list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_list_contract.spec.mjs");

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
            e2e_source.contains(marker),
            "list e2e ready/settled contract should include `{marker}`."
        );
    }
}

#[test]
fn list_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../components/list/scripts/check-ui-e2e-list.sh");

    for marker in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_e2e_contract_covers_ready_and_settled_conditions_for_list_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "list e2e check script should include `{marker}`."
        );
    }
}

#[test]
fn list_check2_marks_e2e_selector_stability_item_complete() {
    let checklist_source = load_source("../../components/list/check2.md");

    assert!(
        checklist_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "components/list/check2.md should mark e2e selector stability item complete."
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
            checklist_source.contains(marker),
            "components/list/check2.md should include e2e selector stability evidence marker `{marker}`."
        );
    }
}

#[test]
fn list_component_does_not_introduce_spec_rs_for_simple_scope() {
    let module_source = load_source("src/list/mod.rs");
    let readme_source = load_source("src/list/README.md");
    let checklist_source = load_source("../../components/list/check2.md");
    let component_spec_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above CARGO_MANIFEST_DIR"))
        .join("components/list/src/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module_source.contains(forbidden),
            "list module should not expose `spec.rs` surface `{forbidden}` in current scope."
        );
    }

    assert!(
        !component_spec_path.exists(),
        "list simple component scope should not ship `{component_spec_path:?}`."
    );

    assert!(
        readme_source.contains("## 组件结构"),
        "list README should keep component-level documentation instead of moving simple scope to `spec.rs`."
    );

    assert!(
        checklist_source.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "components/list/check2.md should mark `spec.rs` governance item as completed."
    );
}

#[test]
fn list_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let checklist_source = load_source("../../components/list/check2.md");
    let manifest_source = load_source("src/list/Component.toml");
    let rbi_source = load_source("src/list/list.rbi");
    let component_files_script =
        load_source("../../scripts/check-ui-component-files.sh");
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above CARGO_MANIFEST_DIR"));
    let list_src_dir = workspace_root.join("components/list/src");

    assert!(
        list_src_dir.join("Component.toml").exists(),
        "list component should provide `{}` manifest for context compression.",
        list_src_dir.join("Component.toml").display()
    );
    assert!(
        list_src_dir.join("list.rbi").exists(),
        "list component should provide `{}` interface projection.",
        list_src_dir.join("list.rbi").display()
    );

    for needle in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"List\"",
        "crate = \"ui-list\"",
        "[[capabilities]]\nname = \"context_compression_manifest\"\nenabled = true",
        "[[capabilities]]\nname = \"rbi_signature_projection\"\nenabled = true",
    ] {
        assert!(
            manifest_source.contains(needle),
            "list Component.toml should include `{needle}`."
        );
    }

    for needle in [
        "pub type ListState = ui_state_primitives::list::ListViewState;",
        "pub type ListMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn List(",
        "pub fn ListItem(",
        "pub fn ListSection(",
    ] {
        assert!(
            rbi_source.contains(needle),
            "list RBI projection should include `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        component_files_script.contains(script_needle),
        "component-files gate script should include list manifest/rbi command."
    );

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/list/src/Component.toml",
        "components/list/src/list.rbi",
        "components/list/test/semantics.rs::list_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "components/list/test/list_module_semantics.rs::list_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include manifest/rbi evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/list/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include agent-contract governance marker `{needle}`."
        );
    }
}

#[test]
fn list_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let manifest_source = load_source("src/list/Component.toml");
    let rbi_source = load_source("src/list/list.rbi");
    let contract_hygiene_script =
        load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
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
            logic_source.contains(needle),
            "list logic should keep typed agent-contract marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_attr()",
        "data-ui-intent=move || agent_contract.get().intent.as_attr()",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should mount schemaized agent-contract field `{needle}`."
        );
    }

    for needle in [
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
            manifest_source.contains(needle),
            "list Component.toml should include schemaized agent-contract marker `{needle}`."
        );
    }

    for needle in [
        "pub const LIST_AGENT_SCHEMA: &str;",
        "pub enum ListAgentSchemaVersion",
        "pub struct ListAgentContract",
        "pub fn resolve_agent_contract(input: ListAgentContractInput) -> ListAgentContract;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "list RBI projection should include typed agent-contract marker `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-schema-version=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
    ] {
        assert!(
            !view_source.contains(forbidden),
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
    let checklist_source = load_source("../../components/list/check2.md");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let protocol_source = load_source("src/list/protocol.rs");
    let manifest_source = load_source("src/list/Component.toml");
    let contract_hygiene_script =
        load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
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
            manifest_source.contains(needle),
            "list Component.toml should include whitelist guard marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "list render path should stay script-injection free and reject `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene script should include list whitelist-safe agent-contract command."
    );

    for needle in [
        "components/list/test/semantics.rs::list_check2_documents_agent_contract_schema_governance_rules",
        "components/list/test/semantics.rs::list_agent_contract_is_schema_typed_and_machine_readable",
        "components/list/test/semantics.rs::list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "components/list/test/list_module_semantics.rs::list_check2_documents_agent_contract_schema_governance_rules",
        "components/list/test/list_module_semantics.rs::list_agent_contract_is_schema_typed_and_machine_readable",
        "components/list/test/list_module_semantics.rs::list_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include list agent-contract evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_visual_desire_baseline_is_documented_for_component_scope() {
    let checklist_source = load_source("../../components/list/check2.md");
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let readme_source = load_source("src/list/README.md");
    let styles_source = load_source("src/list/styles.rs");

    assert!(
        checklist_source.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "components/list/check2.md should mark visual-desire governance item as completed."
    );

    for needle in [
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "data-slot=\"list-showcase\"",
        "data-slot=\"list-workbench\"",
        "test_css_source=workbench_test_css",
    ] {
        assert!(
            list_section.contains(needle),
            "list docs page should keep visual baseline showcase/workbench marker `{needle}`."
        );
    }

    for needle in [
        "default + disabled option",
        "unsynced active index",
        "disabled root",
        "empty list",
    ] {
        assert!(
            list_section.contains(needle),
            "list docs showcase should keep visual contrast scenario marker `{needle}`."
        );
    }

    for needle in [
        "## 展示 (Display)",
        "## Config (Workbench Settings)",
        "## CSS Test (Scoped CSS)",
        "展示区包含多场景对比",
    ] {
        assert!(
            readme_source.contains(needle),
            "list README should keep visual baseline documentation marker `{needle}`."
        );
    }

    for needle in [
        ".ui-listbox--focus-visible",
        ".ui-listbox__option[data-selected=\"true\"]",
        ".ui-listbox__option[data-disabled=\"true\"]",
        ".ui-listbox-item[data-focused=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "list styles should keep visual hierarchy/feedback marker `{needle}`."
        );
    }
}

#[test]
fn list_tree_shaking_contract_uses_component_feature_gates() {
    let checklist_source = load_source("../../components/list/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        checklist_source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "components/list/check2.md should mark tree-shaking governance item as completed."
    );

    for needle in [
        "component-list = [",
        "\"component-active_highlight\"",
        "\"component-illustrated_message\"",
        "\"dep:ui-list\"",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui feature graph should include list feature dependency marker `{needle}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-list\")]\npub use ui_list as list;"),
        "ui crate root should gate list export behind `component-list` feature."
    );

    for needle in [
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css aggregation should keep feature-gated list marker `{needle}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"all-components\")]\npub use all_components::*;"),
        "all-components export should remain feature-gated and not leak into minimal feature builds."
    );
}

#[test]
fn list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
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
            script_source.contains(needle),
            "tree-shaking check script should include list marker `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn list_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let checklist_source = load_source("../../components/list/check2.md");

    assert!(
        checklist_source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "list check2 should keep tree-shaking first-class ability item checked.",
    );
    assert!(
        checklist_source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "list check2 should mark tree-shaking feature-pruning item complete.",
    );

    for needle in [
        "list_tree_shaking_contract_uses_feature_gates_and_no_unconditional_registry_path",
        "list_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "list_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-list,inject-css",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            checklist_source.contains(needle),
            "list check2 tree-shaking section should reference `{needle}`."
        );
    }
}

#[test]
fn list_type_system_and_semantic_markers_form_machine_readable_contract() {
    let checklist_source = load_source("../../components/list/check2.md");
    let component_semantics_source = load_source("../../components/list/test/semantics.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    assert!(
        checklist_source
            .contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "components/list/check2.md should mark typed-state + semantic-marker governance item as completed."
    );

    for needle in [
        "list_discrete_state_axes_are_type_constrained",
        "list_state_normalization_is_concentrated_in_logic_layer",
        "list_state_observability_contract_uses_stable_data_and_aria_markers",
    ] {
        assert!(
            component_semantics_source.contains(needle),
            "component semantics regression should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ListOptionStateInput",
        "pub struct ListSelectionSourceStateInput",
        "pub struct ListSelectionSourceState",
        "pub enum ListInteractionSource",
        "pub fn normalize_options_axis(",
        "pub fn resolve_option_state(",
        "pub fn resolve_selection_source_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic typed-state contract should include `{needle}`."
        );
    }

    for needle in [
        "data-state=move || option_a11y().data_state",
        "data-selection-mode=selection_sources.selection_mode_attr",
        "data-selection-value-source=selection_sources.selection_value_source_attr",
        "data-selection-change-source=selection_sources.selection_change_source_attr",
        "data-interaction-source=move || interaction_source.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "list view machine-readable semantic marker contract should include `{needle}`."
        );
    }
}

#[test]
fn list_focus_stack_and_gc_contract_is_not_applicable_in_current_scope() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    assert!(
        checklist_source.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "components/list/check2.md should mark focus-stack governance item as completed."
    );

    for needle in [
        "use_focus_ring(FocusRingOptions { is_disabled });",
        "on:focus=move |_| focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| focus_ring.handlers.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "list should keep local focus-ring hook marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list current scope should not implement overlay focus-stack contract marker `{forbidden}`."
        );
    }
}

#[test]
fn list_escape_hatches_foreign_zone_contract_is_not_applicable_in_current_scope() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let logic_source = load_source("src/list/logic.rs");
    let motion_source = load_source("src/list/motion.rs");

    assert!(
        checklist_source.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "components/list/check2.md should mark escape-hatches governance item as completed."
    );

    for needle in [
        "use_listbox(ListBoxOptions {",
        "logic::resolve_state(",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || motion_source.contains(needle),
            "list should keep native headless/logic/motion integration marker `{needle}`."
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list current scope should not include foreign-zone third-party imperative marker `{forbidden}`."
        );
    }
}

#[test]
fn list_hydration_discontinuity_contract_uses_deterministic_id_provider_path() {
    let checklist_source = load_source("../../components/list/check2.md");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let root_source = load_source("src/root.rs");
    let id_provider_source = load_source("../ui-headless/src/id_provider.rs");

    assert!(
        checklist_source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "components/list/check2.md should mark hydration-discontinuity governance item as completed."
    );

    for needle in [
        "#[prop(optional, into)] id_base: Option<String>,",
        "use_ui_id_provider().map(|provider| provider.next_prefixed_id(logic::DEFAULT_ID_BASE))",
        "let id_base = logic::normalize_id_base(id_base);",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should consume deterministic IdProvider path `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-list\";",
        "pub fn normalize_id_base(value: Option<String>) -> String",
        "primitives::normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should normalize deterministic id-base contract `{needle}`."
        );
    }

    assert!(
        root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should keep deterministic id-seed injection path."
    );

    for needle in [
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "headless id-provider contract should expose `{needle}`."
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
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list should not introduce non-deterministic hydration id source `{forbidden}`."
        );
    }
}

#[test]
fn list_ssr_cross_platform_contract_keeps_non_wasm_safe_and_cfg_explicit() {
    let checklist_source = load_source("../../components/list/check2.md");
    let module_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let styles_source = load_source("src/list/styles.rs");
    let motion_source = load_source("src/list/motion.rs");
    let active_highlight_motion_source =
        load_source("../ui-visual-primitive/src/active_highlight.rs");
    let illustrated_message_motion_source =
        load_source("../../components/illustrated-message/src/motion.rs");

    assert!(
        checklist_source
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "components/list/check2.md should mark SSR/cross-platform governance item as completed."
    );

    for needle in [
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include compile-only evidence marker `{needle}`."
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
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list component layer should not reference browser-only API `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_motion_source.contains(needle),
            "active-highlight motion primitive should keep explicit platform cfg marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            illustrated_message_motion_source.contains(needle),
            "illustrated-message motion primitive should keep explicit platform cfg marker `{needle}`."
        );
    }
}

#[test]
fn list_ui_headless_web_ssr_mutual_exclusion_contract_is_preserved() {
    let checklist_source = load_source("../../components/list/check2.md");
    let module_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let styles_source = load_source("src/list/styles.rs");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");

    assert!(
        checklist_source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "components/list/check2.md should mark ui-headless web/ssr mutual-exclusion governance item as completed."
    );

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib_source.contains(needle),
            "ui-headless should keep mutual-exclusion guard marker `{needle}`."
        );
    }

    for needle in [
        "use_listbox(ListBoxOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_ui_id_provider().map(|provider| provider.next_prefixed_id(logic::DEFAULT_ID_BASE))",
    ] {
        assert!(
            view_source.contains(needle),
            "list should keep ui-headless integration marker `{needle}`."
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
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "list component layer should not redefine headless feature-mutex contract `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include ui-headless feature-path evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_ui_motion_non_wasm_noop_contract_is_preserved() {
    let checklist_source = load_source("../../components/list/check2.md");
    let module_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let styles_source = load_source("src/list/styles.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");

    assert!(
        checklist_source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "components/list/check2.md should mark ui-motion non-wasm noop governance item as completed."
    );

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should keep noop/stub marker `{needle}`."
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "pub fn attach_section_motion(",
        "sanitize_section_motion(motion)",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle) || view_source.contains(needle),
            "list motion mapping should keep safe attach contract `{needle}`."
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
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "list component layer should not assume browser animation handles or panic path `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include ui-motion compile evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable() {
    let checklist_source = load_source("../../components/list/check2.md");
    let module_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let styles_source = load_source("src/list/styles.rs");
    let motion_source = load_source("src/list/motion.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let active_highlight_motion_source =
        load_source("../ui-visual-primitive/src/active_highlight.rs");
    let illustrated_message_motion_source =
        load_source("../../components/illustrated-message/src/motion.rs");

    assert!(
        checklist_source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "components/list/check2.md should mark reduced-motion/SSR/wasm governance item as completed."
    );

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion short-circuit marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
        "height: var(--ui-active-highlight-h, 0px);",
        "transform: translateY(var(--ui-active-highlight-y, 0px));",
        "opacity: var(--ui-active-highlight-o, 0);",
    ] {
        assert!(
            active_highlight_motion_source.contains(needle),
            "active-highlight primitive should keep SSR/wasm parity marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            illustrated_message_motion_source.contains(needle),
            "illustrated-message primitive should keep explicit platform branch marker `{needle}`."
        );
    }

    for needle in [
        "role=aria.attrs.role",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-slot=\"listbox\"",
        "data-state=move || option_a11y().data_state",
        "data-selected=move || option_a11y().data_selected",
        "data-focused=move || option_a11y().data_focused",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should keep cross-platform semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "list component layer should not split semantics by platform marker `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui --no-default-features --features component-list,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-list,inject-css",
        "cargo test -p ui-list list_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable -- --nocapture",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include reduced-motion/SSR/wasm evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let styles_source = load_source("src/list/styles.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "components/list/test/semantics.rs::list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "components/list/test/list_module_semantics.rs::list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include list motion-contract evidence marker `{needle}`."
        );
    }

    for needle in [
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
            motion_source.contains(needle),
            "list motion source should keep component contract marker `{needle}`."
        );
    }

    for needle in [
        "crate::motion::attach_motion(",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should mount motion contract via `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion short-circuit marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should keep safe no-op marker `{needle}`."
        );
    }

    assert!(
        styles_source.contains(
            "var(--ui-motion-duration-medium, var(--ui-fallback-text-field-motion-duration))"
        ),
        "list styles should keep ui-motion tokenized transition contract in component scope."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platforms_script_source.contains(script_needle),
        "platform gate should include `{script_needle}`."
    );
}

#[test]
fn list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates() {
    let checklist_source = load_source("../../components/list/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should keep blocking governance command `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "\"list\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell should keep list perf budget/probe marker `{needle}`."
        );
    }

    assert!(
        todo_source.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "performance governance should keep explicit render_count automation follow-up in TODO plan."
    );

    for needle in [
        "on:keydown=on_key_down",
        "on:pointerdown=move |_| {",
        "on:pointermove=move |_| aria.handlers.on_option_pointer_move.run(index)",
        "crate::motion::attach_motion(",
        "Effect::new(move |_| {",
    ] {
        assert!(
            view_source.contains(needle) || motion_source.contains(needle),
            "list perf attribution should keep interaction/render/motion path marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "`scripts/check-ui-performance.sh` 已纳入 `list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates` 阻断命令",
        "`apps/docs-app/src/pages/components/shell.rs` 在 `component_page_perf_budget` 为 `\"list\"` 提供 `UiPerfBudget`（mount/update/heap）并由 `UiPerfProbe` 输出 `data-perf-*`",
        "`docs/plan/TODO.md` 保留 `render_count` 自动化补齐项（当前以可重复 perf probe 基线替代精确计数）",
        "回归锁定：`components/list/test/semantics.rs::list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates` 与 `components/list/test/list_module_semantics.rs::list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates`。",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should record performance governance evidence `{needle}`."
        );
    }
}

#[test]
fn list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let local_semantics = include_str!("../../../components/list/test/semantics.rs");
    let aggregated_semantics = load_source("tests/list_module_semantics.rs");
    let view_source = load_source("src/list/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

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
            view_source.contains(marker),
            "list view should keep semantic/focus-flow marker `{marker}`."
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
fn list_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_performance_governance_contract_is_budgeted_traceable_and_blocking_via_global_gates",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn list_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("../../components/list/check2.md");

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
            check2_source.contains(marker),
            "list check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn list_view_macro_complexity_is_split_into_semantic_subrenders() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        checklist_source.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "components/list/check2.md should mark view-macro complexity governance item as completed."
    );

    for needle in [
        "fn render_list_option(",
        "render_list_option(",
        ".map(|(index, label)| {",
        "collect_view()",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should keep semantic subrender split marker `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 4,
        "list view macro complexity should stay bounded after semantic split; expected <= 4, found {view_macro_count}."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        view_macro_script_source.contains(script_needle),
        "view-macro gate script should include list complexity check command."
    );

    for needle in [
        "render_list_option",
        "`view.rs` 已将 option 行渲染从主 `List` `view!` 中下沉到局部函数 `render_list_option(...)`",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_macro_complexity_is_split_into_semantic_subrenders",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include view-macro complexity evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_view_functional_split_prefers_plain_functions_over_local_components() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        checklist_source.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "components/list/check2.md should mark function-first split governance item as completed."
    );

    for needle in [
        "fn render_list_option(",
        ") -> impl IntoView {",
        "render_list_option(",
        "#[component]\npub fn List(",
        "#[component]\npub fn ListItem(",
        "#[component]\npub fn ListSection(",
    ] {
        assert!(
            view_source.contains(needle),
            "list view should keep function-first split marker `{needle}`."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 3,
        "list view should keep exactly three public component boundaries; found {component_count}."
    );

    for forbidden in [
        "#[component]\nfn render_list_option(",
        "#[component]\nfn render_option",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "list should not escalate lightweight local fragments into component `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        view_macro_script_source.contains(script_needle),
        "view-macro gate script should include list function-first command."
    );

    for needle in [
        "render_list_option",
        "仅保留 3 个公共 `#[component]` 边界（`List`/`ListItem`/`ListSection`）",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_view_functional_split_prefers_plain_functions_over_local_components",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include function-first split evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_static_fragments_are_constantized_with_stable_a11y_markers() {
    let checklist_source = load_source("../../components/list/check2.md");
    let view_source = load_source("src/list/view.rs");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        checklist_source.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "components/list/check2.md should mark static fragment constantization governance item as completed."
    );

    for needle in [
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
            view_source.contains(needle),
            "list view should keep static fragment constantization marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_static_fragments_are_constantized_with_stable_a11y_markers";
    assert!(
        view_macro_script_source.contains(script_needle),
        "view-macro gate script should include list static fragment constantization command."
    );

    for needle in [
        "LISTBOX_HIGHLIGHT_CLASS",
        "LIST_ITEM_DIVIDER_CLASS",
        "LIST_SECTION_DIVIDER_CLASS",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_static_fragments_are_constantized_with_stable_a11y_markers",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should include static fragment constantization evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_inner_html_usage_is_explicitly_na_and_guarded() {
    for rel_path in [
        "src/list/mod.rs",
        "src/list/logic.rs",
        "src/list/styles.rs",
        "src/list/view.rs",
        "src/list/motion.rs",
    ] {
        let source = load_source(rel_path);
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
                "list source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
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

    let checklist_source = load_source("../../components/list/check2.md");
    for needle in [
        "`inner_html` 使用约束",
        "零注入面",
        "list_inner_html_usage_is_explicitly_na_and_guarded_locally",
        "list_inner_html_usage_is_explicitly_na_and_guarded",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep inner_html evidence marker `{needle}`."
        );
    }

    let script_source = load_source("../../scripts/check-ui-inner-html.sh");
    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should enforce `{script_needle}`."
    );
}

#[test]
fn list_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let list_cargo = load_source("../../components/list/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let mod_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let checklist_source = load_source("../../components/list/check2.md");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    for needle in ["[features]", "default = []"] {
        assert!(
            list_cargo.contains(needle),
            "list crate feature boundary should include `{needle}`."
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing"] {
        assert!(
            !list_cargo.contains(forbidden),
            "list crate should not leak wasm-debug feature `{forbidden}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui should keep shared wasm-debug feature marker `{needle}`."
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

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui root should keep shared wasm-debug isolation marker `{needle}`."
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
            "docs app should keep dev-only debug overlay entry marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "events.into_iter().rev().take(40)",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace/debug-overlay contract should keep marker `{needle}`."
        );
    }

    for needle in [
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
            view_source.contains(needle),
            "list should keep state/source and interaction markers for replayability via `{needle}`."
        );
    }

    for needle in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "data-slot=\"list-workbench\"",
        "\"selected: \"",
        "set_workbench_selected.set(next)",
    ] {
        assert!(
            list_section.contains(needle),
            "list docs section should keep minimal replay path marker `{needle}`."
        );
    }

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        styles_source.as_str(),
        view_source.as_str(),
        motion_source.as_str(),
    ] {
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
        script_source.contains(script_needle),
        "wasm-debug check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "本组件判定：N/A（组件级不自建 wasm 调试/回放管线）",
        "components/list/test/semantics.rs::list_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "components/list/test/list_module_semantics.rs::list_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep wasm-debug evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let readme_source = load_source("src/list/README.md");
    let checklist_source = load_source("../../components/list/check2.md");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"list-workbench-controls\"",
        "data-slot=\"list-workbench-canvas\"",
        "data-slot=\"list-workbench\"",
    ] {
        assert!(
            list_section.contains(needle),
            "list docs section should keep DX workbench marker `{needle}`."
        );
    }

    for needle in [
        "set_workbench_sync_active.set(event_target_checked(&ev))",
        "set_workbench_disable_last.set(event_target_checked(&ev))",
        "set_workbench_root_disabled.set(event_target_checked(&ev))",
        "set_workbench_custom_class.set(event_target_checked(&ev))",
        "set_workbench_selected.set(next)",
        "\"selected: \"",
    ] {
        assert!(
            list_section.contains(needle),
            "list workbench should keep context-preserving interaction marker `{needle}`."
        );
    }

    for needle in [
        "## Config (Workbench Settings)",
        "## CSS Test (Scoped CSS)",
        "Workbench 使用统一 `Playground controls` 面板调节：",
        "重点是让语义状态（selection/disabled/data-*）在同一画布连续观察，降低回归定位成本。",
        "CSS Test 面板用于局部覆盖与回放",
    ] {
        assert!(
            readme_source.contains(needle),
            "list README should keep DX contract marker `{needle}`."
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
        script_source.contains(script_needle),
        "dx check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "本组件已提供 Workbench 隔离画布（含 settings/code/css-test）",
        "可选状态保留在 List scope 判定为 N/A（不引入本地持久化存储）",
        "components/list/test/semantics.rs::list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "components/list/test/list_module_semantics.rs::list_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep DX evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let readme_source = load_source("src/list/README.md");
    let checklist_source = load_source("../../components/list/check2.md");
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
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
            list_section.contains(needle),
            "list docs section should keep copy-paste-ready marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"list-source-first\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy starter\"",
        "data-slot=\"list-source-paths\"",
        "data-slot=\"list-source-prerequisites\"",
    ] {
        assert!(
            list_section.contains(needle),
            "list docs section should keep source-first marker `{needle}`."
        );
    }

    for needle in [
        "## 状态矩阵（受控 / 非受控）",
        "## Streaming/Snapshot Display",
        "## Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
    ] {
        assert!(
            readme_source.contains(needle),
            "list README should keep docs-as-product marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        script_source.contains(script_needle),
        "dx check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/list/test/semantics.rs::list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "components/list/test/list_module_semantics.rs::list_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep docs-as-product evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles_source = load_source("src/list/styles.rs");
    let checklist_source = load_source("../../components/list/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
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
            styles_source.contains(needle),
            "list styles should keep defensive fallback-chain marker `{needle}`."
        );
    }

    for line in styles_source
        .lines()
        .filter(|line| line.contains("var(--ui-"))
    {
        assert!(
            line.contains("var(--ui-fallback-"),
            "list styles should use two-level ui-theme fallback chain; offending line: `{line}`."
        );
    }

    for forbidden in ["#000", "#fff", " 8px", " 12px"] {
        assert!(
            !styles_source.contains(forbidden),
            "list styles should not include hardcoded `{forbidden}` terminals in component scope."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/list/test/semantics.rs::list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "components/list/test/list_module_semantics.rs::list_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep defensive-variable evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules() {
    let css_source = load_source("src/css.rs");
    let view_source = load_source("src/list/view.rs");
    let checklist_source = load_source("../../components/list/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-list\")]",
        "out.push_str(crate::list::styles::CSS);",
        "out.push_str(crate::list::styles::ITEM_CSS);",
        "out.push_str(crate::list::styles::SECTION_CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css aggregation should keep list cascade-layer marker `{needle}`."
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
            !view_source.contains(forbidden),
            "list view should reject plain inline style injection marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "已核验（聚合层）：`crates/ui/src/css.rs::push_components_css` 使用 `out.push_str(\"\\n@layer ui {\\n\")` 包裹组件样式并在末尾闭合",
        "已核验（运行时样式边界）：`components/list/src/view.rs` 不含 `style=`/`style:\\\"top`/`style:\\\"left` 等普通内联样式写法",
        "N/A（list，运行时数值注入）：当前 `List/ListItem/ListSection` 无运行时动态样式写入路径，后续若引入仅允许 CSS 自定义变量注入（`style:--ui-*`）",
        "components/list/test/semantics.rs::list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules",
        "components/list/test/list_module_semantics.rs::list_cascade_layer_contract_is_aggregated_in_ui_layer_and_rejects_plain_inline_style_rules",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep cascade-layer evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade() {
    let check2_source = load_source("../../components/list/check2.md");
    let manifest_source = load_source("../../components/list/src/Component.toml");
    let rbi_source = load_source("../../components/list/src/list.rbi");
    let protocol_source = load_source("src/list/protocol.rs");
    let mod_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let styles_source = load_source("src/list/styles.rs");
    let motion_source = load_source("src/list/motion.rs");

    for needle in [
        "pub enum ListComponentSchemaVersion {",
        "V1,",
        "pub struct ListComponentSpec {",
        "pub schema_version: ListComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "list protocol should keep v1 schema marker `{needle}`."
        );
    }

    for needle in ["schema_version = \"1\"", "values = [\"v1\"]"] {
        assert!(
            manifest_source.contains(needle),
            "list Component.toml should keep v1 contract marker `{needle}`."
        );
    }

    for needle in ["pub enum ListAgentSchemaVersion {", "V1,"] {
        assert!(
            rbi_source.contains(needle),
            "list RBI should keep v1 contract marker `{needle}`."
        );
    }

    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
        protocol_source.as_str(),
        manifest_source.as_str(),
        rbi_source.as_str(),
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

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `List` 变更未引入跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/list/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn list_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    let marker = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let mod_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");
    let styles_source = load_source("src/list/styles.rs");
    let motion_source = load_source("src/list/motion.rs");
    let protocol_source = load_source("src/list/protocol.rs");
    let checklist_source = load_source("../../components/list/check2.md");
    let ui_components_cargo = load_source("Cargo.toml");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");
    let button_view_source = load_source("../../components/button/src/view.rs");

    for needle in [
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
            protocol_source.contains(needle),
            "list protocol should keep structured serde schema contract marker `{needle}`."
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
            !protocol_source.contains(forbidden),
            "list protocol should avoid ad-hoc serde/migration helper token `{forbidden}`."
        );
    }

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(needle) || button_view_source.contains(needle),
            "engineering baseline should keep canonical tracing semantics marker `{needle}`."
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

    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
        protocol_source.as_str(),
    ]
    .join("\n");
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

    for source in [
        mod_source.as_str(),
        logic_source.as_str(),
        view_source.as_str(),
        styles_source.as_str(),
        motion_source.as_str(),
    ] {
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
        !mod_source.contains("web_sys"),
        "list public module boundary should not leak web_sys types."
    );

    let script_needle = "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        engineering_script.contains(script_needle),
        "engineering check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "components/list/src/protocol.rs",
        "components/list/test/semantics.rs::list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "components/list/test/list_module_semantics.rs::list_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep engineering evidence marker `{needle}`."
        );
    }
}

#[test]
fn list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("src/list/mod.rs");
    let logic_source = load_source("src/list/logic.rs");
    let styles_source = load_source("src/list/styles.rs");
    let view_source = load_source("src/list/view.rs");
    let motion_source = load_source("src/list/motion.rs");
    let protocol_source = load_source("src/list/protocol.rs");
    let combined = [
        mod_source.as_str(),
        logic_source.as_str(),
        styles_source.as_str(),
        view_source.as_str(),
        motion_source.as_str(),
        protocol_source.as_str(),
    ]
    .join("\n");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "list non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/list/logic.rs");
    let view_source = load_source("src/list/view.rs");

    for needle in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(\"ui-listbox-item\")];",
        "Cow::Borrowed(\"ui-listbox-item--custom-class\")",
        "Cow::Borrowed(\"ui-listbox-section\")",
        "Cow::Borrowed(state.heading_tone_class)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic_source.contains(needle),
            "list logic should keep Cow-based string hotspot mitigation marker `{needle}`."
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
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "list string hotspot contract should avoid `{forbidden}`."
        );
    }
}

#[test]
fn list_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(needle),
            "rust-hygiene gate script should enforce `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test list_module_semantics --no-default-features --features component-list,inject-css list_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should include `{needle}`."
        );
    }
}

#[test]
fn list_check2_marks_rust_hygiene_contract_complete() {
    let checklist_source = load_source("../../components/list/check2.md");

    for needle in [
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
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            checklist_source.contains(needle),
            "components/list/check2.md should keep rust-hygiene evidence marker `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn list_feature_graph_declares_required_motion_dependencies() {
    let cargo_toml = load_source("Cargo.toml");

    assert!(
        cargo_toml.contains(
            "component-list = [\"component-active_highlight\", \"component-illustrated_message\"]"
        ),
        "ui feature graph should declare list -> active_highlight/illustrated_message dependencies for minimal-feature builds."
    );
}
