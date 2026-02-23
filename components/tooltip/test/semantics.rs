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
fn tooltip_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/tooltip/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tooltip internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tooltip_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/tooltip/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Tooltip;"),
        "tooltip module should export `Tooltip`."
    );
    assert!(
        module_source.contains("pub use ui_state_primitives::tooltip::{TooltipPartState, TooltipPartStateInput, TooltipSlot};"),
        "tooltip module should expose tooltip state contracts from ui-state-primitives."
    );
    assert!(
        crate_source.contains("pub use ui_tooltip as tooltip;"),
        "crate root should re-export ui-tooltip as `tooltip` module."
    );
    assert!(
        crate_source.contains("pub use tooltip::Tooltip;")
            && crate_source.contains("pub use tooltip::TooltipMotion;"),
        "crate root should re-export `Tooltip` and `TooltipMotion` contracts."
    );
}

#[test]
fn tooltip_logic_exposes_state_helpers() {
    let source = load_source("../../components/tooltip/src/logic.rs");

    for needle in [
        "pub const DEFAULT_DELAY_MS: u64 = tooltip_state::DEFAULT_DELAY_MS;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64 = tooltip_state::DEFAULT_CLOSE_DELAY_MS;",
        "pub const DEFAULT_SHOULD_CLOSE_ON_PRESS: bool = tooltip_state::DEFAULT_SHOULD_CLOSE_ON_PRESS;",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn trigger_attr(trigger: TooltipTriggerMode)",
        "pub fn press_behavior_attr(should_close_on_press: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn has_custom_delays(delay_ms: u64, close_delay_ms: u64)",
        "pub fn resolve_state(input: TooltipPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: TooltipPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64)",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn tooltip_view_uses_logic_state_contracts() {
    let source = load_source("../../components/tooltip/src/view.rs");

    for needle in [
        "pub fn Tooltip(",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput",
        "logic::normalize_open_state(logic::OpenStateInput",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_id(id, format!(\"ui-tooltip-{}\", next_id()))",
        "logic::has_custom_delays(delay_ms, close_delay_ms)",
        "open,",
        "default_open,",
        "on_open_change,",
        "let open_mode_attr = normalized_open_state.open_mode_attr;",
        "let open_source_attr = normalized_open_state.open_source_attr;",
        "let root_state = Memo::new(move |_| {",
        "let panel_state = Memo::new(move |_| {",
        "logic::resolve_state(TooltipPartStateInput {",
        "logic::compose_class_name(class_name, root_state.get_untracked())",
        "logic::compose_panel_vars(position.top_px.get(), position.left_px.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-open=move || root_state.get().is_open.then_some(\"true\")",
        "data-trigger=move || root_state.get().trigger_attr",
        "data-press-behavior=move || root_state.get().press_behavior_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-delay-source=move || root_state.get().delay_source_attr",
        "data-trigger-source=move || root_state.get().trigger_source_attr",
        "data-press-source=move || root_state.get().press_source_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_source_attr",
        "data-default-open-source=default_open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-custom-delay=move || root_state.get().has_custom_delays.then_some(\"true\")",
        "data-custom-trigger=move || root_state.get().has_custom_trigger_mode.then_some(\"true\")",
        "data-custom-press=move || root_state.get().has_custom_press_behavior.then_some(\"true\")",
        "data-custom-id=move || root_state.get().has_custom_id.then_some(\"true\")",
        "data-custom-open=has_custom_open.then_some(\"true\")",
        "data-custom-default-open=has_custom_default_open.then_some(\"true\")",
        "data-custom-open-change=has_custom_on_open_change.then_some(\"true\")",
        "data-slot=move || panel_state.get().slot_attr",
        "data-state=move || panel_state.get().state_attr",
        "data-open=move || panel_state.get().is_open.then_some(\"true\")",
        "data-motion-source=move || panel_state.get().motion_source_attr",
        "data-custom-motion=move || panel_state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-delay=move || panel_state.get().has_custom_delays.then_some(\"true\")",
        "data-custom-trigger=move || panel_state.get().has_custom_trigger_mode.then_some(\"true\")",
        "data-custom-press=move || panel_state.get().has_custom_press_behavior.then_some(\"true\")",
        "data-custom-id=move || panel_state.get().has_custom_id.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn tooltip_api_naming_uses_prefixed_props_without_alias_drift() {
    let source = load_source("../../components/tooltip/src/view.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] is_open: Option<Signal<bool>>,",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional, into)] dir: Option<String>,",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip API should expose standardized prefixed prop `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] open: Option<Signal<bool>>,",
    ] {
        assert!(
            !source.contains(forbidden),
            "Tooltip API should not expose legacy alias `{forbidden}`."
        );
    }
}

#[test]
fn tooltip_uses_headless_trigger_and_position_hooks() {
    let source = load_source("../../components/tooltip/src/view.rs");

    for needle in [
        "use_tooltip_trigger",
        "use_tooltip_position",
        "use_tooltip_focus_a11y",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should use headless `{needle}` hooks for baseline-style behavior and positioning."
        );
    }
}

#[test]
fn tooltip_uses_presence_for_exit_motion_unmounting() {
    let source = load_source("../../components/tooltip/src/view.rs");

    for needle in [
        "use_presence(open)",
        "presence.finish_exit",
        "motion::attach_motion",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip should use presence for motion-friendly unmounting via `{needle}`."
        );
    }
}

#[test]
fn tooltip_manages_aria_describedby_on_the_focused_element() {
    let view_source = load_source("../../components/tooltip/src/view.rs");
    let headless_source = load_source("../ui-headless/src/tooltip.rs");

    assert!(
        view_source.contains("use_tooltip_focus_a11y"),
        "Tooltip view should delegate focused-element aria-describedby semantics to headless."
    );
    assert!(
        headless_source.contains("set_attribute(\"aria-describedby\"")
            && headless_source.contains("remove_attribute(\"aria-describedby\""),
        "Tooltip headless contract should manage `aria-describedby` on the focused element."
    );
}

#[test]
fn tooltip_mounts_a11y_attrs_from_headless_contract() {
    let view_source = load_source("../../components/tooltip/src/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "tooltip_panel_attrs(TooltipPanelA11yOptions",
        "id=move || panel_a11y.get().attrs.id.clone()",
        "role=move || panel_a11y.get().attrs.role",
        "lang=move || panel_a11y.get().attrs.lang.clone()",
        "dir=move || panel_a11y.get().attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Tooltip view should mount headless a11y contract `{needle}`."
        );
    }

    for needle in [
        "pub struct TooltipPanelA11yAttrs",
        "pub struct TooltipPanelA11yHandlers",
        "pub struct TooltipPanelA11yState",
        "pub struct TooltipPanelA11yContract",
        "pub struct TooltipPanelA11yOptions",
        "pub fn tooltip_panel_attrs(options: TooltipPanelA11yOptions)",
    ] {
        assert!(
            headless_a11y_source.contains(needle),
            "ui-headless a11y module should expose tooltip contract `{needle}`."
        );
    }
}

#[test]
fn tooltip_styles_include_state_and_source_marker_contracts() {
    let source = load_source("../../components/tooltip/src/styles.rs");

    for needle in [
        ".ui-tooltip[data-motion-source=\"custom\"]",
        ".ui-tooltip[data-custom-motion=\"true\"]",
        ".ui-tooltip[data-delay-source=\"custom\"]",
        ".ui-tooltip[data-trigger-source=\"custom\"]",
        ".ui-tooltip[data-press-source=\"custom\"]",
        ".ui-tooltip[data-id-source=\"custom\"]",
        ".ui-tooltip[data-state=\"open\"]",
        ".ui-tooltip[data-state=\"closed\"]",
        ".ui-tooltip__panel[data-state=\"panel\"]",
        ".ui-tooltip__panel[data-motion-source=\"custom\"]",
        ".ui-tooltip__panel[data-custom-motion=\"true\"]",
        ".ui-tooltip__panel[data-delay-source=\"custom\"]",
        ".ui-tooltip__panel[data-custom-delay=\"true\"]",
        ".ui-tooltip__panel[data-trigger-source=\"custom\"]",
        ".ui-tooltip__panel[data-custom-trigger=\"true\"]",
        ".ui-tooltip__panel[data-press-source=\"custom\"]",
        ".ui-tooltip__panel[data-custom-press=\"true\"]",
        ".ui-tooltip__panel[data-id-source=\"custom\"]",
        ".ui-tooltip__panel[data-custom-id=\"true\"]",
        "z-index: var(--ui-overlay-z-index, 1000);",
        "padding: var(--ui-space-sm, 8px) var(--ui-space-md, 12px);",
        "font-size: var(--ui-font-size-100, 12px);",
        "max-width: var(--ui-tooltip-max-width, 280px);",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip styles should include `{needle}` for motion/state/source selector contracts."
        );
    }
}

#[test]
fn tooltip_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::tooltip::styles::CSS);"),
        "ui css aggregator should include tooltip styles."
    );
}

#[test]
fn tooltip_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "pub(super) fn tooltip() -> AnyView",
        "title=\"Tooltip\"",
        "slug=\"tooltip\"",
        "State + Source Markers",
        "data-delay-source",
        "<Tooltip",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs page should contain `{needle}`."
        );
    }
}

#[test]
fn tooltip_docs_page_exposes_stable_e2e_slots() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_tooltip_contract.spec.mjs");

    for needle in [
        "attr:data-slot=\"tooltip-e2e-open\"",
        "attr:data-slot=\"tooltip-e2e-close\"",
        "attr:data-slot=\"tooltip-e2e-trigger\"",
        "data-slot=\"tooltip-workbench-controls\"",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs page should expose stable e2e selector `{needle}`."
        );
    }

    for needle in [
        "[data-slot=\"tooltip-e2e-open\"]",
        "[data-slot=\"tooltip-e2e-close\"]",
        "[data-slot=\"tooltip-panel\"][id=\"docs-tooltip-workbench\"]",
        "toHaveAttribute(\"data-open-mode\", \"controlled\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "tooltip e2e contract should assert `{needle}`."
        );
    }
}

#[test]
fn tooltip_motion_delegates_triplet_driver_to_ui_motion() {
    let source = load_source("../../components/tooltip/src/motion.rs");

    for needle in [
        "ui_motion::spring::SpringAnimatorTriplet",
        "ui_motion::spring::SpringAnimatorTriplet::new(",
        "springs.clear_on_rest();",
        "springs.set_on_rest_second(move || on_exit_complete.run());",
        "springs.set_targets([0.0, motion.initial_scale, offset_y]);",
    ] {
        assert!(
            source.contains(needle),
            "tooltip motion should delegate spring triplet driver to ui-motion via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tooltip_motion_contract_exposes_default_and_custom_test_coverage() {
    let source = load_source("../../components/tooltip/src/motion.rs");

    for needle in [
        "pub struct TooltipMotion",
        "use ui_theme::default_overlay_layout_tokens;",
        "ui_motion::spring::sanitize_config(value, default)",
        "fn default_motion_uses_soft_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip motion contract should include `{needle}` for baseline-style regression coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tooltip_motion_sanitizes_custom_contract_values() {
    let source = load_source("../../components/tooltip/src/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TooltipMotion) -> TooltipMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "drop(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset_ranges()",
    ] {
        assert!(
            source.contains(needle),
            "Tooltip motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn tooltip_docs_page_includes_custom_motion_contract_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion = TooltipMotion {",
        "initial_scale: 0.92",
        "offset_y_px: 14.0",
        "motion=custom_motion",
        "motion=TooltipMotion::default()",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs page should include `{needle}` for custom motion demos."
        );
    }
}

#[test]
fn tooltip_docs_state_source_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "delay_ms=300",
        "close_delay_ms=200",
        "should_close_on_press=false",
        "class_name=\"docs-tooltip-state\".to_string()",
        "id=\"docs-tooltip\".to_string()",
        "motion=TooltipMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 10.0",
        "Inspect data-delay-source/data-trigger-source/data-id-source.",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs state/source playground should contain `{needle}`."
        );
    }
}

#[test]
fn tooltip_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_tooltip.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion = TooltipMotion {",
        "initial_scale: 0.92",
        "offset_y_px: 14.0",
        "motion=custom_motion",
        "motion=TooltipMotion::default()",
        "content=move || view! { \"Custom spring + placement offset\" }",
        "content=move || view! { \"Default motion\" }",
    ] {
        assert!(
            source.contains(needle),
            "tooltip docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn tooltip_docs_page_covers_primary_playgrounds() {
    tooltip_docs_page_contains_state_source_playground();
}

#[test]
fn tooltip_docs_playgrounds_lock_state_matrix_contract_values() {
    tooltip_docs_state_source_playground_locks_contract_values();
    tooltip_docs_custom_motion_playground_locks_contract_values();
}
