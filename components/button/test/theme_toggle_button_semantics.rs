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
fn theme_toggle_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/theme_toggle/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ThemeToggleButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn theme_toggle_button_uses_logic_state_model() {
    let view_source = load_source("src/button/theme_toggle/view.rs");
    let logic_source = load_source("src/button/theme_toggle/logic.rs");

    for needle in [
        "pub struct ThemeToggleState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_modes(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub current_mode_attr: &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "ThemeToggleButton logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let modes = logic::normalize_modes(modes);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);",
        "let has_custom_motion = motion != ThemeToggleMotion::default();",
        "let motion_source_attr = if has_custom_motion {",
        "let class = logic::compose_class_name(",
        "let state = Memo::new(move |_| {",
    ] {
        assert!(
            view_source.contains(needle),
            "ThemeToggleButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn theme_toggle_button_uses_button_wrapper_contract() {
    let source = load_source("src/button/theme_toggle/view.rs");

    for needle in [
        "<Button",
        "aria_label=aria_label",
        "class_name=class",
        "variant=variant",
        "size=size",
        "is_icon_only=true",
        "disabled=disabled",
        "on_press=on_press",
    ] {
        assert!(
            source.contains(needle),
            "ThemeToggleButton should forward `{needle}` to Button for consistent semantics."
        );
    }
}

#[test]
fn theme_toggle_button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/theme_toggle/view.rs");

    for needle in [
        "class=\"ui-theme-toggle-button-shell\"",
        "data-slot=\"theme-toggle-button\"",
        "data-slot=\"theme-toggle-icon\"",
        "data-state=move || if state.get().is_disabled { \"disabled\" } else { \"enabled\" }",
        "data-current-mode=move || state.get().current_mode_attr",
        "data-next-mode=move || state.get().next_mode_attr",
        "data-mode-count=move || state.get().mode_count.to_string()",
        "data-custom-modes=move || state.get().has_custom_modes.then_some(\"true\")",
        "data-aria-source=move || {",
        "data-custom-aria-label=move || state.get().has_custom_aria_label.then_some(\"true\")",
        "data-class-source=move || {",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ThemeToggleButton should expose `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn theme_toggle_button_does_not_ignore_motion_contract() {
    let source = load_source("src/button/theme_toggle/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "ThemeToggleButton should honor `ThemeToggleMotion` rather than ignoring it."
    );
}

#[test]
fn theme_toggle_button_attaches_motion_driver() {
    let source = load_source("src/button/theme_toggle/view.rs");

    assert!(
        source.contains("attach_motion"),
        "ThemeToggleButton should attach its motion driver to deliver icon micro-interactions."
    );
}

#[test]
fn theme_toggle_button_styles_define_css_vars_for_motion() {
    let source = load_source("src/button/theme_toggle/styles.rs");

    for name in ["--ui-theme-toggle-rotate", "--ui-theme-toggle-scale"] {
        assert!(
            source.contains(name),
            "ThemeToggleButton styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn theme_toggle_motion_uses_spring_animator() {
    let source = load_source("src/button/theme_toggle/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "ThemeToggleMotion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn theme_toggle_button_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/theme_toggle/styles.rs");

    for selector in [
        ".ui-theme-toggle-button-shell[data-motion-source=\"custom\"]",
        ".ui-theme-toggle-button-shell[data-custom-motion=\"true\"]",
        ".ui-theme-toggle-button__icon[data-motion-source=\"custom\"]",
        ".ui-theme-toggle-button__icon[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ThemeToggleButton styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn theme_toggle_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/button/theme_toggle/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ThemeToggleMotion) -> ThemeToggleMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "rotate_deg:",
        "scale_down:",
        "scale_settle_delay_ms:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_and_clamps_values()",
        "fn sanitize_motion_keeps_valid_values()",
    ] {
        assert!(
            source.contains(needle),
            "ThemeToggle motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn theme_toggle_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn theme_toggle_button() -> AnyView",
        "title=\"ThemeToggleButton\"",
        "slug=\"theme-toggle-button\"",
        "description=\"Icon-only theme toggle with baseline-level spring motion and baseline-style mode state attrs.\"",
        "<Playground",
        "title=\"Default cycle\"",
        "code_signal=code",
        "id_base=\"docs-theme-toggle-mode\".to_string()",
        "aria_label=\"ThemeToggle start mode\".to_string()",
        "<Switch checked=disabled set_checked=set_disabled>\"Disabled\"</Switch>",
        "<Switch checked=two_mode_cycle set_checked=set_two_mode_cycle>",
        "<Switch checked=custom_aria_label set_checked=set_custom_aria_label>",
        "title=\"Custom modes + disabled\"",
        "<ThemeToggleButton",
    ] {
        assert!(
            source.contains(needle),
            "actions theme_toggle_button docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn theme_toggle_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Default cycle\"",
        "code_signal=code",
        "let mode_options = vec![",
        "Effect::new(move |_| {",
        "let modes = if two_mode_cycle {",
        "if custom_aria_label {",
        "modes=modes",
        "aria_label=\"Switch UI mode\".to_string()",
        "title=\"Custom modes + disabled\"",
        "modes=custom_modes.clone()",
        "aria_label=\"Switch UI mode\".to_string()",
        "<ThemeToggleButton mode=mode set_mode=set_mode is_disabled=true />",
        "\"disabled toggle should remain inert\"",
    ] {
        assert!(
            source.contains(needle),
            "actions theme_toggle_button docs playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
