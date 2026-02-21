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
fn toggle_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/toggle_button/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ToggleButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn toggle_button_uses_headless_hooks() {
    let source = load_source("src/button/toggle_button/view.rs");

    for needle in ["use_button", "use_focus_ring", "use_hover"] {
        assert!(
            source.contains(needle),
            "ToggleButton should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn toggle_button_uses_logic_state_model() {
    let view_source = load_source("src/button/toggle_button/view.rs");
    let logic_source = load_source("src/button/toggle_button/logic.rs");

    for needle in [
        "pub struct ToggleButtonState",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn compose_class_name(",
        "pub fn resolve_state(",
        "pub is_selected: bool",
        "pub is_enabled: bool",
        "pub is_pressed: bool",
        "pub is_focus_visible: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "ToggleButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "#[prop(optional)] is_pressed: Option<Signal<bool>>",
        "#[prop(optional)] default_pressed: Option<bool>",
        "let state = Memo::new(move |_|",
        "overlay_open::use_controllable_state(is_pressed, default_pressed, on_pressed_change)",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let class = logic::compose_class_name(class_name, variant, size);",
        "logic::resolve_state(",
        "pressed.get()",
        "state.get().data_state()",
    ] {
        assert!(
            view_source.contains(needle),
            "ToggleButton view should derive root state via logic::resolve_state; missing `{needle}`."
        );
    }

    assert!(
        !view_source.contains("set_selected: WriteSignal<bool>"),
        "ToggleButton should not remain controlled-only via `set_selected` injection."
    );
}

#[test]
fn toggle_button_attaches_motion_driver() {
    let source = load_source("src/button/toggle_button/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "ToggleButton should attach a motion driver to match the repo's motion spec."
    );
}

#[test]
fn toggle_button_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/button/toggle_button/view.rs");

    for attr in [
        "data-slot=\"toggle-button\"",
        "data-state=move || state.get().data_state()",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-unselected=move || state.get().is_unselected.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-enabled=move || state.get().is_enabled.then_some(\"true\")",
        "data-hovered=move || state.get().is_hovered.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || state.get().is_focus_visible.then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(attr),
            "ToggleButton should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn toggle_button_styles_define_scale_css_var() {
    let source = load_source("src/button/toggle_button/styles.rs");

    assert!(
        source.contains("--ui-toggle-button-scale"),
        "ToggleButton styles should define `--ui-toggle-button-scale` so motion can update scale without re-rendering."
    );
}

#[test]
fn toggle_button_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/toggle_button/styles.rs");

    for selector in [
        ".ui-toggle-button[data-motion-source=\"custom\"]",
        ".ui-toggle-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ToggleButton styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn toggle_button_motion_uses_spring_animator() {
    let source = load_source("src/button/toggle_button/motion.rs");

    for needle in [
        "crate::button::motion::attach_motion",
        "as_button_motion(",
        "ButtonMotion",
    ] {
        assert!(
            source.contains(needle),
            "ToggleButton motion should delegate spring runtime to Button motion; missing `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toggle_button_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/toggle_button/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ToggleButtonMotion) -> ToggleButtonMotion",
        "crate::button::motion::sanitize_motion",
        "fn sanitize_spring(",
        "fn as_button_motion(",
        "hover_scale:",
        "tap_scale:",
        "let motion = as_button_motion(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "ToggleButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn toggle_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn toggle_button() -> AnyView",
        "title=\"ToggleButton\"",
        "slug=\"toggle-button\"",
        "description=\"Pressable toggle state with baseline-level spring motion and baseline-style root state attrs.\"",
        "title=\"Controlled + on_pressed_change\"",
        "code_signal=code",
        "title=\"Variant + size + disabled matrix\"",
        "id_base=\"docs-toggle-button-variant\".to_string()",
        "id_base=\"docs-toggle-button-size\".to_string()",
        "aria_label=\"ToggleButton variant\".to_string()",
        "aria_label=\"ToggleButton size\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<ToggleButton",
        "on_pressed_change=on_toggle_change",
        "variant=ToggleButtonVariant::Accent",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for toggle-button coverage.",
        );
    }
}

#[test]
fn toggle_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "let (selected, set_selected) = signal(false);",
        "let selected_signal: Signal<bool> = Signal::derive(move || selected.get());",
        "let on_toggle_change = Callback::new(move |next| set_selected.set(next));",
        "if variant != ToggleButtonVariant::Default {",
        "if size != ToggleButtonSize::M {",
        "if disabled {",
        "code_signal=code",
        "\"selected: \"",
        "\"last on_pressed_change: \" {move || last_change.get()}",
        "is_pressed=Some(selected_signal)",
        "is_pressed=Some(notifications_signal)",
        "on_pressed_change=on_notifications_change",
        "size=ToggleButtonSize::L",
        "\"notifications: \"",
        "is_pressed=Some(disabled_selected_signal)",
        "is_pressed=Some(disabled_unselected_signal)",
        "\"Disabled on\"",
        "\"Disabled off\"",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "toggle-button docs playgrounds should contain `{needle}`.",
        );
    }
}
