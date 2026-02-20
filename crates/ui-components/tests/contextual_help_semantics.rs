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
fn contextual_help_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/contextual-help/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ContextualHelp internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn contextual_help_uses_logic_state_model() {
    let view_source = load_source("../../components/contextual-help/src/view.rs");
    let logic_source = load_source("../../components/contextual-help/src/logic.rs");

    for needle in [
        "pub struct ContextualHelpStateInput",
        "pub struct ContextualHelpState",
        "pub fn normalize_optional_text(",
        "pub fn resolve_trigger_aria_label(",
        "pub fn resolve_id(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "class_source_attr",
        "motion_source_attr",
        "has_custom_motion",
    ] {
        assert!(
            logic_source.contains(needle),
            "ContextualHelp logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(heading)",
        "logic::resolve_trigger_aria_label(variant, aria_label)",
        "logic::resolve_id(id, format!(\"ui-contextual-help-{}\", next_id()))",
        "logic::resolve_state(ContextualHelpStateInput {",
        "logic::compose_class_name(class_name, state)",
        "motion: ContextualHelpMotion",
        "motion=motion.popover",
    ] {
        assert!(
            view_source.contains(needle),
            "ContextualHelp view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn contextual_help_uses_controllable_open_and_presence() {
    let source = load_source("../../components/contextual-help/src/view.rs");

    for needle in [
        "overlay_open::use_controllable_open_state_traced",
        "use_presence(open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "ContextualHelp should keep open/presence contracts (`{needle}`)."
        );
    }
}

#[test]
fn contextual_help_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/contextual-help/src/view.rs");

    for attr in [
        "data-slot=\"contextual-help\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-placement=state.placement_attr",
        "data-heading=state.heading_attr",
        "data-footer=state.footer_attr",
        "data-open-mode=state.open_mode_attr",
        "data-label-source=state.label_source_attr",
        "data-id-source=state.id_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-slot=\"contextual-help-panel\"",
        "data-slot=\"contextual-help-content\"",
    ] {
        assert!(
            source.contains(attr),
            "ContextualHelp should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn contextual_help_panel_preserves_non_modal_dialog_semantics() {
    let source = load_source("../../components/contextual-help/src/view.rs");

    for needle in [
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "aria-label=panel_aria_label.get_value()",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "aria-describedby=panel_aria_describedby.get_value()",
        "is_modal=false",
    ] {
        assert!(
            source.contains(needle),
            "ContextualHelp panel should preserve dialog semantics (`{needle}`)."
        );
    }
}

#[test]
fn contextual_help_styles_include_state_marker_contracts() {
    let source = load_source("../../components/contextual-help/src/styles.rs");

    for selector in [
        ".ui-contextual-help--variant-info",
        ".ui-contextual-help[data-variant=\"help\"]",
        ".ui-contextual-help--placement-top-end",
        ".ui-contextual-help[data-state=\"disabled\"]",
        ".ui-contextual-help[data-heading=\"absent\"]",
        ".ui-contextual-help[data-footer=\"present\"]",
        ".ui-contextual-help[data-class-source=\"custom\"]",
        ".ui-contextual-help--custom-class",
        ".ui-contextual-help[data-motion-source=\"custom\"]",
        ".ui-contextual-help--custom-motion",
        ".ui-contextual-help[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ContextualHelp styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn contextual_help_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("../../components/contextual-help/src/mod.rs");
    let motion_source = load_source("../../components/contextual-help/src/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ContextualHelpMotion;",
        "pub struct ContextualHelpMotion",
        "pub popover: crate::popover::PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ContextualHelp motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn contextual_help_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/contextual-help/src/motion.rs");
    let view_source = load_source("../../components/contextual-help/src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ContextualHelpMotion) -> ContextualHelpMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ContextualHelp motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source
            .contains("let motion = crate::contextual_help::motion::sanitize_motion(motion);"),
        "ContextualHelp view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn contextual_help_docs_page_covers_primary_playgrounds() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn contextual_help() -> AnyView",
        "title=\"ContextualHelp\"",
        "slug=\"contextual-help\"",
        "description=\"Non-modal popover help trigger with centralized variant/placement/heading/footer state attrs.\"",
        "<Playground title=\"Help Variant + Slots\" code_signal=semantic_code>",
        "<Playground title=\"Info Variant + Controlled\" code_signal=controlled_code>",
        "<ContextualHelp",
        "ContextualHelpVariant::Info",
        "open=controlled_open",
        "on_open_change=on_controlled_open_change",
    ] {
        assert!(
            docs.contains(needle),
            "overlays docs page should include `{needle}` for contextual_help primary coverage.",
        );
    }
}

#[test]
fn contextual_help_docs_playgrounds_lock_state_matrix_contract_values() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "heading=\"Contextual help\".to_string()",
        "footer=move || view! { \"Popover-based\" }",
        "\"Uses Button + Popover + spring motion.\"",
        "\"Works in Light/Dark/OLED via tokens.\"",
        "let (controlled_open_raw, set_controlled_open_raw) = signal(false);",
        "let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());",
        "variant=ContextualHelpVariant::Info",
        "open=controlled_open",
        "on_open_change=on_controlled_open_change",
        "aria_label=\"More info\".to_string()",
        "class_name=\"docs-contextual-help-custom\".to_string()",
        "\"Toggle controlled help\"",
        "\"open: \"",
        "\"Controlled mode keeps parent state as the source of truth.\"",
    ] {
        assert!(
            docs.contains(needle),
            "contextual_help docs playgrounds should contain `{needle}`.",
        );
    }
}
