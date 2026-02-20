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
fn overlay_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/overlay/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Overlay internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn overlay_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/overlay/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Overlay;"),
        "overlay module should export `Overlay`."
    );
    assert!(
        module_source.contains("pub struct OverlayPartStateInput"),
        "overlay module should expose `OverlayPartStateInput` contract."
    );
    assert!(
        crate_source.contains("pub use overlay::Overlay;")
            && crate_source.contains("pub use overlay::OverlayMotion;"),
        "crate root should re-export `Overlay` and `OverlayMotion` contracts."
    );
}

#[test]
fn overlay_logic_exposes_state_helpers() {
    let source = load_source("src/overlay/logic.rs");

    for needle in [
        "pub const DEFAULT_ROLE: &str = \"dialog\";",
        "pub const DEFAULT_DISMISSABLE: bool = true;",
        "pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn dismiss_attr(is_dismissable: bool)",
        "pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: OverlayPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: OverlayPartState)",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            source.contains(needle),
            "Overlay logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn overlay_escape_respects_default_prevented_composition_and_keyboard_flag() {
    let source = load_source("src/overlay/view.rs");

    for needle in [
        "default_prevented",
        "is_composing",
        "logic::should_close_on_escape(",
        "is_keyboard_dismiss_disabled",
        "stop_propagation()",
    ] {
        assert!(
            source.contains(needle),
            "Overlay should include `{needle}` for stable Escape-dismiss behavior."
        );
    }
}

#[test]
fn overlay_view_uses_logic_state_contracts() {
    let source = load_source("src/overlay/view.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_optional_text(aria_labelledby)",
        "logic::normalize_optional_text(aria_describedby)",
        "logic::resolve_state(OverlayPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-role-source=root_state.role_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-role=root_state.has_custom_role.then_some(\"true\")",
        "data-custom-aria-labelledby=root_state.has_custom_aria_labelledby.then_some(\"true\")",
        "data-custom-aria-describedby=root_state.has_custom_aria_describedby.then_some(\"true\")",
        "data-custom-class=root_state.has_custom_class_name.then_some(\"true\")",
        "data-slot=backdrop_state.slot_attr",
        "data-slot=panel_state.slot_attr",
        "data-role=role",
    ] {
        assert!(
            source.contains(needle),
            "Overlay view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn overlay_styles_include_state_and_source_markers() {
    let source = load_source("src/overlay/styles.rs");

    for selector in [
        ".ui-overlay[data-motion-source=\"custom\"]",
        ".ui-overlay[data-custom-motion=\"true\"]",
        ".ui-overlay--custom-role",
        ".ui-overlay[data-role-source=\"custom\"]",
        ".ui-overlay--custom-aria-labelledby",
        ".ui-overlay[data-custom-aria-labelledby=\"true\"]",
        ".ui-overlay[data-aria-labelledby-source=\"custom\"]",
        ".ui-overlay--custom-aria-describedby",
        ".ui-overlay[data-custom-aria-describedby=\"true\"]",
        ".ui-overlay[data-aria-describedby-source=\"custom\"]",
        ".ui-overlay[data-dismiss-source=\"custom\"]",
        ".ui-overlay[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-overlay[data-exit-source=\"custom\"]",
        ".ui-overlay[data-dismissable=\"true\"] .ui-overlay__backdrop",
        ".ui-overlay[data-keyboard-dismiss-disabled=\"true\"] .ui-overlay__panel",
        ".ui-overlay__backdrop[data-state=\"backdrop\"]",
        ".ui-overlay__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(selector),
            "Overlay styles should include `{selector}` as stable state/source contracts."
        );
    }

    for needle in [
        ".ui-overlay[data-state=\"open\"]",
        ".ui-overlay[data-open=\"true\"]",
        ".ui-overlay[data-state=\"closed\"]",
        ".ui-overlay[data-closed=\"true\"]",
        "pointer-events: none;",
    ] {
        assert!(
            source.contains(needle),
            "Overlay styles should include `{needle}` to avoid closed backdrops intercepting clicks."
        );
    }
}

#[test]
fn overlay_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::overlay::styles::CSS);"),
        "ui-components css aggregator should include overlay styles."
    );
}

#[test]
fn overlay_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn overlay() -> AnyView",
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "State + Source Markers",
        "data-dismiss-source",
        "<Overlay",
    ] {
        assert!(
            source.contains(needle),
            "overlay docs page should contain `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn overlay_motion_contract_exposes_default_and_customization_checks() {
    let mod_source = load_source("src/overlay/mod.rs");
    let motion_source = load_source("src/overlay/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::OverlayMotion;",
        "pub struct OverlayMotion",
        "fn default_motion_uses_flip3d_spring_contract()",
        "fn supports_custom_overlay_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "Overlay motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn overlay_motion_contract_sanitizes_custom_values() {
    let source = load_source("src/overlay/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: OverlayMotion) -> OverlayMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "initial_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_y_offset_ranges()",
    ] {
        assert!(
            source.contains(needle),
            "Overlay motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn overlay_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "<Playground title=\"Overlay presence\" code_signal=code>",
        "<Button on_press=open_overlay>\"Open overlay\"</Button>",
        "<Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>",
        "Esc or click backdrop closes. Tab is trapped.",
        "<Button variant=ButtonVariant::Secondary on_press=on_close>\"Close\"</Button>",
    ] {
        assert!(
            source.contains(needle),
            "overlay docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn overlay_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "<Playground",
        "title=\"State + Source Markers\"",
        "<Button on_press=open_marker>\"Open marker overlay\"</Button>",
        "role=\"alertdialog\"",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=marker_motion",
        "class_name=\"docs-overlay-state\".to_string()",
        "aria_labelledby=\"overlay-marker-title\".to_string()",
        "aria_describedby=\"overlay-marker-desc\".to_string()",
        "on_exit_complete=on_marker_exit_complete",
        "initial_scale: 0.94,",
        "initial_y_px: 14.0,",
    ] {
        assert!(
            source.contains(needle),
            "overlay docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn overlay_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn overlay() -> AnyView",
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "description=\"Portal + backdrop + focus trap + overlay stack (Esc/topmost). Supports dismiss control flags and requires presence to unmount after exit.\"",
        "<Playground title=\"Overlay presence\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "code_signal=marker_code",
        "<Overlay",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for overlay primary playground coverage.",
        );
    }
}

#[test]
fn overlay_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Overlay presence\"",
        "<Button on_press=open_overlay>\"Open overlay\"</Button>",
        "<Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>",
        "Esc or click backdrop closes. Tab is trapped.",
        "title=\"State + Source Markers\"",
        "<Button on_press=open_marker>\"Open marker overlay\"</Button>",
        "role=\"alertdialog\"",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=marker_motion",
        "class_name=\"docs-overlay-state\".to_string()",
        "aria_labelledby=\"overlay-marker-title\".to_string()",
        "aria_describedby=\"overlay-marker-desc\".to_string()",
        "on_exit_complete=on_marker_exit_complete",
        "initial_scale: 0.94,",
        "initial_y_px: 14.0,",
    ] {
        assert!(
            source.contains(needle),
            "overlay docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn overlay_merge_gate_verdicts_are_explicit_and_fully_completed() {
    let check2_source = load_source("src/overlay/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "overlay merge-gate checklist should include `{needle}`."
        );
    }
}

#[test]
fn overlay_checklist_sections_one_to_nine_have_no_unchecked_items() {
    let check2_source = load_source("src/overlay/check2.md");

    for needle in [
        "### 1. 大骨架（架构边界与层职责）",
        "### 2. 小骨架（API 设计检查 + 状态管理检查）",
        "### 3. 实现细节（A11y / i18n-l10n / 可观测 / 样式与动效）",
        "### 4. SSR / 跨平台 / WASM / 性能 / 工程能力",
        "### 5. 文件落点检查（必须提及）",
        "### 6. AI 原生能力（Agent Contract + 流式）",
        "### 7. 测试与文档（验证闭环）",
        "### 8. 明确禁止的反模式",
        "### 9. 合并门禁（最终裁决）",
    ] {
        assert!(
            check2_source.contains(needle),
            "overlay checklist should include section `{needle}`."
        );
    }

    assert!(
        !check2_source.contains("- [ ]"),
        "overlay checklist must not keep unchecked items after completion."
    );
}
