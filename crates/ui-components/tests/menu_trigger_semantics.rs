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
fn menu_trigger_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/trigger/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "MenuTrigger's internal modules should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_uses_logic_state_model() {
    let view_source = load_source("src/menu/trigger/view.rs");
    let logic_source = load_source("src/menu/trigger/logic.rs");

    for needle in [
        "pub struct MenuTriggerStateInput",
        "pub struct MenuTriggerState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_trigger_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "MenuTrigger logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let disabled_indices = logic::normalize_disabled_indices(disabled_indices, item_count);",
        "let (aria_label, has_custom_aria_label) = logic::resolve_trigger_aria_label(aria_label);",
        "let state = logic::resolve_state(logic::MenuTriggerStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "MenuTrigger view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "motion: MenuTriggerMotion",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn menu_trigger_is_labeled_and_owns_a_menu() {
    let source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "aria_label: Option<String>",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_labelledby=trigger_id.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should wire `{needle}` for baseline-style menu trigger semantics."
        );
    }
}

#[test]
fn menu_trigger_uses_presence_to_allow_exit_motion() {
    let source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "use_presence(open)",
        "motion=motion.popover",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_emits_baseline_style_root_data_attributes() {
    let source = load_source("src/menu/trigger/view.rs");

    for attr in [
        "data-slot=\"menu-trigger\"",
        "data-state=move ||",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-disabled=state.is_trigger_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-placement=state.placement_attr",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-close-on-action=state.close_on_action.then_some(\"true\")",
        "data-keep-open-on-action=state.keep_open_on_action.then_some(\"true\")",
        "data-custom-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-has-disabled-items=state.has_disabled_items.then_some(\"true\")",
        "data-has-item-kinds=state.has_item_kinds.then_some(\"true\")",
        "data-motion-source=if motion == MenuTriggerMotion::default()",
        "data-custom-motion=(motion != MenuTriggerMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "MenuTrigger should set `{attr}` to support baseline-style styling and regression testing."
        );
    }
}

#[test]
fn menu_trigger_supports_arrow_key_opening() {
    let source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "focus_strategy_for_open_key",
        "request_open_change.run(true)",
        "set_open_focus.set(strategy)",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger should support ArrowUp/ArrowDown opening via `{needle}`."
        );
    }
}

#[test]
fn menu_trigger_uses_logic_for_empty_and_disabled_trigger_state() {
    let logic_source = load_source("src/menu/trigger/logic.rs");
    let view_source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "resolve_trigger_disabled",
        "normalize_disabled_indices",
        "MenuOpenFocusStrategy",
    ] {
        assert!(
            logic_source.contains(needle),
            "MenuTrigger logic should centralize `{needle}` semantics."
        );
    }

    for needle in [
        "if trigger_disabled.get_value()",
        "if let Some(strategy) = logic::focus_strategy_for_open_key(&key)",
        "disabled=state.is_trigger_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "MenuTrigger view should consume `{needle}` to keep trigger behavior/state attrs consistent."
        );
    }
}

#[test]
fn menu_trigger_styles_include_disabled_and_persistent_markers() {
    let source = load_source("src/menu/trigger/styles.rs");

    for needle in [
        ".ui-menu-trigger--persistent",
        ".ui-menu-trigger--disabled",
        ".ui-menu-trigger[data-motion-source=\"custom\"]",
        ".ui-menu-trigger[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "MenuTrigger styles should include `{needle}` for stable visual state contracts."
        );
    }
}

#[test]
fn menu_trigger_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/menu/trigger/mod.rs");
    let motion_source = load_source("src/menu/trigger/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::MenuTriggerMotion;",
        "pub struct MenuTriggerMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "MenuTrigger motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn menu_trigger_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/menu/trigger/motion.rs");
    let view_source = load_source("src/menu/trigger/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: MenuTriggerMotion) -> MenuTriggerMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "MenuTrigger motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::menu_trigger::motion::sanitize_motion(motion);"),
        "MenuTrigger view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn menu_trigger_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn menu_trigger() -> AnyView",
        "title=\"MenuTrigger\"",
        "slug=\"menu-trigger\"",
        "description=\"Button-triggered menu surface with baseline state attrs and controlled/uncontrolled close-strategy semantics.\"",
        "<Playground title=\"Default\" code_signal=code>",
        "<Playground title=\"Controlled + persistent open\" code_signal=controlled_code>",
        "<Playground title=\"Disabled + Empty\" code_signal=disabled_code>",
        "<MenuTrigger",
        "close_on_action=false",
        "disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for menu-trigger coverage.",
        );
    }
}

#[test]
fn menu_trigger_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-menu-trigger\".to_string()",
        "items=default_items",
        "item_kinds=vec![",
        "\"Open menu\"",
        "\"last: \"",
        "id_base=\"docs-menu-trigger-controlled\".to_string()",
        "items=controlled_items",
        "disabled_indices=vec![1]",
        "open=controlled_open",
        "on_open_change=on_open_change",
        "\"open: \"",
        "id_base=\"docs-menu-trigger-disabled\".to_string()",
        "items=disabled_items",
        "id_base=\"docs-menu-trigger-empty\".to_string()",
        "items=empty_items",
    ] {
        assert!(
            source.contains(needle),
            "menu-trigger docs playgrounds should contain `{needle}`.",
        );
    }
}
