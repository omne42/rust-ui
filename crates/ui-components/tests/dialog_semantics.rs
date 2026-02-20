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
fn dialog_does_not_expose_logic_module() {
    let source = load_source("src/dialog/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Dialog's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
}

#[test]
fn dialog_feature_gate_declares_required_component_dependencies() {
    let source = load_source("Cargo.toml");

    assert!(
        source.contains("component-dialog = [\"component-overlay\", \"component-button\"]",),
        "component-dialog feature must depend on component-overlay + component-button so minimal feature builds stay valid.",
    );
}

#[test]
fn dialog_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/dialog/mod.rs");

    for needle in [
        "pub enum DialogSlot",
        "pub struct DialogPartStateInput",
        "pub struct DialogPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TITLE",
        "DEFAULT_CLOSE_LABEL",
        "DEFAULT_SHOW_CLOSE_BUTTON",
        "DEFAULT_SIZE",
    ] {
        assert!(
            source.contains(needle),
            "dialog::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn dialog_logic_exposes_state_helpers() {
    let source = load_source("src/dialog/logic.rs");

    for needle in [
        "pub enum DialogSize",
        "pub fn as_attr(self) -> &'static str",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn footer_attr(has_footer: bool)",
        "pub fn close_button_attr(show_close_button: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: DialogPartStateInput) -> DialogPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: DialogPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Dialog logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn dialog_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(DialogPartStateInput {",
        "slot: DialogSlot::Root",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-size=root_state.size_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-size-source=root_state.size_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-size=root_state.has_custom_size.then_some(\"true\")",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-close=(root_state.close_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-motion=root_state.has_custom_motion.then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Dialog view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn dialog_wires_aria_ids_and_optional_description_semantics() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "let title_id = format!(\"{id_base}-title\")",
        "aria_labelledby=title_id.clone()",
        "let description_id = format!(\"{id_base}-description\")",
        "if root_state.show_description",
        "aria_describedby=description_id.clone()",
        "<Show when=move || root_state.show_description>",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Dialog should include `{needle}` for stable a11y description wiring."
        );
    }
}

#[test]
fn dialog_close_button_uses_button_icon_only_with_aria_label() {
    let source = load_source("src/dialog/view.rs");

    for needle in [
        "data-slot=close_state.slot_attr",
        "<Button",
        "aria_label=close_label",
    ] {
        assert!(
            source.contains(needle),
            "Dialog close button should be accessible and stable (`{needle}`)."
        );
    }
}

#[test]
fn dialog_styles_include_state_and_source_marker_selectors() {
    let source = load_source("src/dialog/styles.rs");

    for selector in [
        ".ui-dialog[data-motion-source=\"custom\"]",
        ".ui-dialog[data-custom-motion=\"true\"]",
        ".ui-dialog--custom-motion",
        ".ui-dialog[data-size-source=\"custom\"]",
        ".ui-dialog[data-custom-size=\"true\"]",
        ".ui-dialog--custom-size",
        ".ui-dialog[data-id-source=\"custom\"]",
        ".ui-dialog[data-custom-id=\"true\"]",
        ".ui-dialog--custom-id",
        ".ui-dialog[data-title-source=\"custom\"]",
        ".ui-dialog[data-custom-title=\"true\"]",
        ".ui-dialog--custom-title",
        ".ui-dialog[data-description-source=\"custom\"]",
        ".ui-dialog[data-custom-description=\"true\"]",
        ".ui-dialog--custom-description",
        ".ui-dialog[data-close-source=\"custom\"]",
        ".ui-dialog[data-custom-close=\"true\"]",
        ".ui-dialog--custom-close",
        ".ui-dialog[data-exit-source=\"custom\"]",
        ".ui-dialog[data-custom-exit=\"true\"]",
        ".ui-dialog--custom-exit",
        ".ui-dialog--with-description",
        ".ui-dialog[data-state=\"with-description\"]",
        ".ui-dialog--title-only",
        ".ui-dialog[data-close-button=\"hidden\"]",
        ".ui-dialog__title[data-slot=\"dialog-title\"]",
        ".ui-dialog__description[data-slot=\"dialog-description\"]",
        ".ui-dialog__body[data-slot=\"dialog-body\"]",
    ] {
        assert!(
            source.contains(selector),
            "Dialog styles should include `{selector}` as stable state/source marker contracts."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dialog_motion_contract_exposes_default_and_custom_overlay_checks() {
    let source = load_source("src/dialog/motion.rs");

    for needle in [
        "pub struct DialogMotion",
        "pub overlay: crate::overlay::OverlayMotion",
        "fn default_motion_uses_default_overlay_motion_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Dialog motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn dialog_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "State + Source Markers",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
        "<Dialog",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should contain `{needle}`."
        );
    }
}

#[test]
fn dialog_docs_page_exposes_interactive_display_config_code_css_test_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "<Playground",
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"components/dialog/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "<Playground title=\"Scenario Comparison\" code_signal=scenario_code>",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should include `{needle}` for interactive display/config/code/css-test + comparison coverage.",
        );
    }
}

#[test]
fn dialog_readme_covers_display_config_code_css_test_and_comparison_sections() {
    let source = load_source("src/dialog/README.md");

    for needle in [
        "## 展示区（Display）",
        "## Config 区",
        "## Code 区",
        "## CSS Test 区",
        "## 多种情况对比显示",
    ] {
        assert!(
            source.contains(needle),
            "dialog README should include `{needle}` for required documentation structure.",
        );
    }
}

#[test]
fn dialog_e2e_contract_uses_semantic_selectors() {
    let source = load_source("../../e2e/tests/docs_app_dialog_contract.spec.mjs");

    for needle in [
        "docs-app dialog exposes stable role/source markers",
        "docs-app dialog interactive + comparison playgrounds stay contract-stable",
        "/#/components/dialog",
        "Open marker dialog",
        "Open workbench dialog",
        "Open compact comparison",
        "Open motion comparison",
        "data-slot=\"overlay-panel\"",
        "role=\"dialog\"",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
        "Dismiss dialog",
        "Cancel",
        "Close",
        "Open dialog",
        "Escape",
    ] {
        assert!(
            source.contains(needle),
            "dialog e2e contract should include `{needle}` for stable semantic regression coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn dialog_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/dialog/motion.rs");
    let view_source = load_source("src/dialog/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DialogMotion) -> DialogMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "fn sanitize_motion_delegates_to_overlay_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "Dialog motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::dialog::motion::sanitize_motion(motion);"),
        "Dialog view should sanitize motion before forwarding to Overlay.",
    );
}

#[test]
fn dialog_docs_page_locks_custom_motion_marker_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "motion=DialogMotion {",
        "overlay: OverlayMotion {",
        "initial_scale: 0.94",
        "initial_y_px: 14.0",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs page should include `{needle}` for motion/source marker regression stability."
        );
    }
}

#[test]
fn dialog_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "<Playground title=\"Dialog\" code_signal=code>",
        "<Button on_press=open_dialog>\"Open dialog\"</Button>",
        "id_base=\"docs-dialog\".to_string()",
        "title=\"Dialog title\".to_string()",
        "description=\"Uses Overlay + header/body/footer layout.\".to_string()",
        "<Button variant=ButtonVariant::Secondary on_press=on_close>\"Cancel\"</Button>",
        "<Button on_press=on_close>\"Confirm\"</Button>",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn dialog_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "pub(super) fn dialog() -> AnyView",
        "title=\"Dialog\"",
        "slug=\"dialog\"",
        "description=\"Dialog panel with header/body/footer structure on top of Overlay.\"",
        "<Playground title=\"Dialog\" code_signal=code>",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "data-id-source",
        "data-title-source",
        "data-description-source",
        "data-close-source",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "overlays_dialog docs page should include `{needle}` for dialog primary coverage.",
        );
    }
}

#[test]
fn dialog_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_dialog.rs");

    for needle in [
        "id_base=\"docs-dialog\".to_string()",
        "title=\"Dialog title\".to_string()",
        "description=\"Uses Overlay + header/body/footer layout.\".to_string()",
        "<Button variant=ButtonVariant::Secondary on_press=on_close>\"Cancel\"</Button>",
        "<Button on_press=on_close>\"Confirm\"</Button>",
        "on_exit_complete=on_exit_complete",
        "id_base=\"docs-dialog-marker\".to_string()",
        "title=\"Marker dialog\".to_string()",
        "description=\"Custom size, class, close label, and motion for contract inspection.\"",
        "size=DialogSize::Lg",
        "close_label=\"Dismiss dialog\"",
        "class_name=\"docs-dialog-custom\".to_string()",
        "motion=DialogMotion {",
        "overlay: OverlayMotion {",
        "initial_scale: 0.94",
        "initial_y_px: 14.0",
        "\"Open marker dialog\"",
        "\"open: \"",
    ] {
        assert!(
            source.contains(needle),
            "dialog docs playgrounds should contain `{needle}`.",
        );
    }
}
