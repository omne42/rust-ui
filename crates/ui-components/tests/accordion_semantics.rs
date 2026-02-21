use std::fs;
use std::path::{Path, PathBuf};

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(suffix) = rel_path.strip_prefix("src/accordion/") {
        manifest_dir
            .join("../../components/accordion/src")
            .join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_accordion_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir
        .join("../../components/accordion/test")
        .join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_rust_sources_under(rel_dir: &str) -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, acc: &mut Vec<(PathBuf, String)>) {
        let entries =
            fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir failed for {dir:?}: {e}"));
        for entry in entries {
            let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {dir:?}: {e}"));
            let path = entry.path();
            if path.is_dir() {
                walk(&path, acc);
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
            acc.push((path, source));
        }
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.join(rel_dir);
    let mut out = Vec::new();
    walk(&root, &mut out);
    out
}

#[test]
fn accordion_does_not_expose_logic_motion_or_view_modules() {
    let source = load_source("src/accordion/mod.rs");

    for needle in ["pub mod logic", "pub mod motion", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Accordion's internal modules should stay private; found `{needle}`."
        );
    }
}

#[test]
fn accordion_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "pub fn AccordionItem(",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: bool",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should accept `{needle}` to support controlled/uncontrolled open state."
        );
    }

    for legacy in [
        "#[prop(optional)] open: Option<Signal<BTreeSet<usize>>>",
        "#[prop(optional)] default_open: Option<BTreeSet<usize>>",
        "#[prop(optional)] on_open_change: Option<Callback<BTreeSet<usize>>>",
        "open_indices: Option<Signal<BTreeSet<usize>>>",
        "default_open_indices: Option<BTreeSet<usize>>",
        "on_open_indices_change: Option<Callback<BTreeSet<usize>>>",
        "#[prop(optional)] disabled: bool,",
    ] {
        assert!(
            !source.contains(legacy),
            "Accordion public API should not keep legacy alias `{legacy}`."
        );
    }
}

#[test]
fn accordion_api_keeps_minimal_default_path_without_internal_state_objects() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "pub fn Accordion(",
        "pub fn AccordionItem(",
        "#[prop(optional, into)] id_base: Option<String>,",
        "#[prop(into)] label: String,",
        "#[prop(optional)] key: Option<usize>,",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: bool",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "logic::resolve_id_base(",
        "logic::resolve_item_label(",
        "logic::assign_item_keys(",
        "#[prop(optional)] variant: AccordionVariant,",
        "#[prop(optional)] disallow_empty_selection: bool,",
    ] {
        assert!(
            source.contains(needle),
            "Accordion API should preserve minimal default path while allowing advanced control via `{needle}`."
        );
    }

    for forbidden in ["state: ", "headless_state", "primitive_state"] {
        assert!(
            !source.contains(forbidden),
            "Accordion API should not require internal state object parameter `{forbidden}`."
        );
    }

    for forbidden in ["labels: Vec<String>", "disabled_indices: Vec<usize>"] {
        assert!(
            !source.contains(forbidden),
            "Accordion primary API should avoid parallel-array prop contract `{forbidden}`."
        );
    }
}

#[test]
fn accordion_primary_api_uses_explicit_item_composition_contract() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "pub fn AccordionItem(",
        "collect_accordion_items(children)",
        "children: Children",
        "use_ui_id_provider",
    ] {
        assert!(
            source.contains(needle),
            "Accordion primary API should expose explicit item composition via `{needle}`."
        );
    }

    for forbidden in [
        "labels: Vec<String>",
        "children().nodes",
        "zip(panels)",
        "fn next_id() -> u64",
        "thread_local!",
    ] {
        assert!(
            !source.contains(forbidden),
            "Accordion should not rely on parallel array/slot convention `{forbidden}`."
        );
    }
}

#[test]
fn accordion_collection_registration_protocol_is_context_driven_and_ordered_in_logic() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "struct RegistrationContext",
        "AccordionRegistrationAction::Register",
        "AccordionRegistrationAction::Unregister",
        "collect_accordion_items(children)",
        "logic::resolve_registered_item_keys",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should wire registration protocol through `{needle}`."
        );
    }

    for needle in [
        "pub enum AccordionRegistrationAction",
        "pub struct AccordionRegistrationState",
        "pub fn reduce_registration_actions(",
        "pub fn resolve_registered_item_keys(",
        "pub items_order: Vec<usize>",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should own registration ordering contract via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("HashSet"),
        "Accordion registration/navigation ordering should not rely on HashSet iteration order."
    );
}

#[test]
fn accordion_slot_projection_contract_covers_lazy_keepalive_eager_and_notify_hidden() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");
    let motion_source = load_source("src/accordion/motion.rs");

    for needle in [
        "#[prop(optional)] slot_projection: AccordionSlotProjection,",
        "#[prop(optional)] on_panel_lifecycle: Option<Callback<AccordionPanelLifecycleEvent>>",
        "logic::should_render_panel_surface(",
        "data-slot-projection=slot_projection.as_str()",
        "data-panel-lifecycle=move || panel_lifecycle.get().as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should expose slot projection lifecycle contract `{needle}`."
        );
    }

    for needle in [
        "pub enum AccordionSlotProjection",
        "Lazy",
        "KeepAlive",
        "Eager",
        "pub enum AccordionPanelLifecycleEvent",
        "NotifyHidden",
        "NotifyShown",
        "pub fn should_render_panel_surface(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should model slot projection/lifecycle contract via `{needle}`."
        );
    }

    for needle in [
        "if slot_projection == logic::AccordionSlotProjection::KeepAlive {",
        "observer.disconnect();",
        "on_panel_lifecycle.run(panel_lifecycle_event_from_hidden(hidden));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Accordion motion should pause keep-alive hidden work and notify lifecycle via `{needle}`."
        );
    }
}

#[test]
fn accordion_uses_headless_hooks() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "use_roving_tabindex",
        "use_press",
        "use_focus_ring",
        "use_hover",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn accordion_attaches_motion_drivers() {
    let source = load_source("src/accordion/view.rs");

    for needle in ["attach_indicator_motion", "attach_panel_motion"] {
        assert!(
            source.contains(needle),
            "Accordion should attach `{needle}` for baseline-style spring motion."
        );
    }
}

#[test]
fn accordion_emits_baseline_style_data_attributes() {
    let source = load_source("src/accordion/view.rs");

    for attr in [
        "data-slot=\"accordion\"",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-open-count=move || state.get().open_count.to_string()",
        "data-all-closed=move || (!state.get().has_open_items).then_some(\"true\")",
        "data-multiple-open=move || state.get().has_multiple_open.then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-open-state-source=open_state_source.as_str()",
        "data-open-init-source=open_init_source.as_str()",
        "data-open-last-change-source=move || open_last_change_source.get().as_str()",
        "data-selection-mode=match selection_mode",
        "data-variant=variant.as_str()",
        "data-motion-source=if motion == AccordionMotion::default()",
        "data-custom-motion=(motion != AccordionMotion::default()).then_some(\"true\")",
        "data-slot=\"accordion-item\"",
        "data-slot=\"accordion-trigger\"",
        "data-slot=\"accordion-label\"",
        "data-slot=\"accordion-indicator\"",
        "data-slot=\"accordion-panel\"",
        "data-slot=\"accordion-panel-surface\"",
        "data-index=index",
        "data-open",
        "data-hovered",
        "data-pressed",
        "data-focused",
        "data-focus-visible",
        "data-disabled",
    ] {
        assert!(
            source.contains(attr),
            "Accordion should set `{attr}` to support baseline-style styling and regression testing."
        );
    }
}

#[test]
fn accordion_uses_logic_state_model() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "pub struct AccordionState",
        "pub fn resolve_state(",
        "pub fn normalize_default_open_for_items(",
        "pub open_count: usize",
        "pub has_disabled_items: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should include `{needle}` for centralized root-state derivation."
        );
    }
    assert!(
        view_source.contains("logic::resolve_state("),
        "Accordion view should derive root state through resolve_state."
    );
    assert!(
        view_source.contains("logic::normalize_default_open_for_items("),
        "Accordion view should delegate default-value normalization to logic.rs."
    );
    assert!(
        !view_source.contains("default_open.unwrap_or_default()"),
        "Accordion view should not set default fallback outside logic.rs."
    );
    assert!(
        view_source.contains("has_disabled_items"),
        "Accordion state derivation should include disabled-item state."
    );
}

#[test]
fn accordion_uses_typed_discrete_mode_from_state_primitives() {
    let source = load_source("src/accordion/logic.rs");

    for needle in [
        "use ui_state_primitives::expansion::{",
        "pub type AccordionSelectionMode = ExpansionMode;",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should keep discrete mode typed through state-primitives via `{needle}`."
        );
    }
}

#[test]
fn accordion_consumes_state_primitive_sources_only() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "use ui_state_primitives::expansion::{",
        "normalize_open_keys",
        "normalize_default_open_keys",
        "toggle_open_key",
        "summarize",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Accordion should consume ui-state-primitives capability `{needle}`."
        );
    }
}

#[test]
fn accordion_has_no_async_loading_protocol_axis() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for async_marker in ["is_loading", "aria-busy", "on_retry", "use_async_action"] {
        assert!(
            !view_source.contains(async_marker) && !logic_source.contains(async_marker),
            "Accordion currently has no async axis; unexpected marker `{async_marker}` found.",
        );
    }
}

#[test]
fn accordion_panels_are_labeled_regions() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "role=\"region\"",
        "aria-expanded",
        "aria-controls",
        "aria-labelledby=trigger_id",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should wire `{needle}` for accessible disclosure semantics."
        );
    }
}

#[test]
fn accordion_mounts_headless_disclosure_locale_contract() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "disclosure_trigger_attrs(open, panel_id.clone(), lang.clone(), dir)",
        "lang=disclosure_a11y.lang.clone()",
        "dir=disclosure_a11y.dir",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should mount headless disclosure locale/a11y contract via `{needle}`."
        );
    }
}

#[test]
fn accordion_localizable_copy_is_caller_owned() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "pub fn AccordionItem(",
        "#[prop(into)] label: String,",
        "{label}",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should keep localizable copy controlled by caller or locale contract via `{needle}`."
        );
    }
}

#[test]
fn accordion_open_source_markers_use_closed_enums() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "pub enum AccordionOpenStateSource",
        "pub enum AccordionOpenInitSource",
        "pub enum AccordionOpenChangeSource",
        "pub fn resolve_open_state_source(",
        "pub fn resolve_open_init_source(",
        "Self::Controlled => \"controlled\"",
        "Self::Uncontrolled => \"uncontrolled\"",
        "Self::External => \"external\"",
        "Self::Default => \"default\"",
        "Self::Empty => \"empty\"",
        "Self::ExternalSync => \"external-sync\"",
        "Self::Keyboard => \"keyboard\"",
        "Self::Pointer => \"pointer\"",
        "Self::Programmatic => \"programmatic\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should define closed state-source marker `{needle}`."
        );
    }

    for needle in [
        "data-open-state-source=open_state_source.as_str()",
        "data-open-init-source=open_init_source.as_str()",
        "data-open-last-change-source=move || open_last_change_source.get().as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should expose stable source marker `{needle}`."
        );
    }
}

#[test]
fn accordion_agent_contract_is_schema_typed_and_mounted() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "pub enum AccordionAgentSchemaVersion",
        "pub enum AccordionAgentIntent",
        "pub enum AccordionAgentAction",
        "pub enum AccordionAgentStateAxis",
        "pub enum AccordionAgentOutputStatus",
        "pub struct AccordionAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-toggle=move || {",
        "data-ui-capability-focus-move=move || {",
        "data-ui-capability-external-sync=move || {",
        "data-ui-capability-programmatic-replay=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn accordion_streaming_contract_is_explicit_for_na_and_fallback() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");
    let readme_source = load_source("src/accordion/README.md");

    for needle in [
        "use ui_ai_runtime::use_ai_space_state;",
        "let ai_space_state = StoredValue::new(use_ai_space_state());",
        "data-ui-stream-support=move || {",
        "data-ui-stream-fallback=move || {",
        "data-ui-stream-mode=move || {",
        "data-ui-output-status=move || {",
        "data-ui-fragment-kind=\"accordion-panel\"",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should expose explicit stream contract marker `{needle}`."
        );
    }

    for needle in [
        "AccordionAgentStreamSupport::Unsupported",
        "AccordionAgentStreamFallback::FullSnapshot",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should model stream NA/fallback contract via `{needle}`."
        );
    }

    for needle in [
        "data-ui-stream-support=\"unsupported\"",
        "data-ui-stream-fallback=\"full-snapshot\"",
        "data-ui-output-status",
    ] {
        assert!(
            readme_source.contains(needle),
            "Accordion README should document stream fallback contract `{needle}`."
        );
    }
}

#[test]
fn accordion_styles_define_motion_css_vars() {
    let source = load_source("src/accordion/styles.rs");

    for var in ["--ui-font-size-200", "--ui-component-height-100"] {
        assert!(
            source.contains(var),
            "Accordion styles should consume ui-theme token variable `{var}` for scale-aware sizing."
        );
    }

    for hardcoded in ["font-size: 14px;", "width: 18px;", "height: 18px;"] {
        assert!(
            !source.contains(hardcoded),
            "Accordion styles should avoid hardcoded sizing token `{hardcoded}`."
        );
    }

    for var in [
        "--ui-accordion-indicator-rotation",
        "--ui-accordion-panel-height",
        "--ui-accordion-panel-opacity",
        "--ui-accordion-panel-y",
    ] {
        assert!(
            source.contains(var),
            "Accordion styles should define `{var}` so motion can update without re-rendering."
        );
    }

    for selector in [
        ".ui-accordion[data-motion-source=\"custom\"]",
        ".ui-accordion[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Accordion styles should include `{selector}` as stable custom-motion selectors."
        );
    }

    for var in [
        "--ui-layout-content-1",
        "--ui-layout-content-2",
        "--ui-layout-divider",
        "--ui-layout-foreground",
        "--ui-layout-focus",
    ] {
        assert!(
            source.contains(var),
            "Accordion styles should consume semantic token variable `{var}`."
        );
    }
}

#[test]
fn accordion_styles_depend_on_explicit_state_markers() {
    let styles_source = load_source("src/accordion/styles.rs");
    let view_source = load_source("src/accordion/view.rs");

    for explicit_selector in [
        "[data-motion-source=\"custom\"]",
        "[data-custom-motion=\"true\"]",
        "[data-hovered=\"true\"]",
        ".ui-accordion__trigger--focus-visible",
    ] {
        assert!(
            styles_source.contains(explicit_selector),
            "Accordion styles should branch on explicit state marker `{explicit_selector}`."
        );
    }

    for fragile_selector in [":nth-child", ":first-child", ":last-child", "> * > * > *"] {
        assert!(
            !styles_source.contains(fragile_selector),
            "Accordion styles should avoid fragile DOM-structure selector `{fragile_selector}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Accordion view should not push business style logic through inline styles."
    );
}

#[test]
fn accordion_motion_is_spring_driven() {
    let source = load_source("src/accordion/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Accordion motion should use SpringAnimator to match the motion spec."
    );
}

#[test]
fn accordion_motion_defaults_come_from_ui_theme_tokens() {
    let source = load_source("src/accordion/motion.rs");

    for needle in [
        "use ui_theme::default_accordion_motion_tokens;",
        "let tokens = default_accordion_motion_tokens();",
        "stiffness: tokens.spring.stiffness",
        "panel_offset_y_px: tokens.panel_offset_y_px",
    ] {
        assert!(
            source.contains(needle),
            "Accordion motion defaults should come from ui-theme token source `{needle}`."
        );
    }
}

#[test]
fn accordion_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/accordion/motion.rs");
    let tests_source = load_accordion_test_source("motion.rs");
    let combined_source = format!("{source}\n{tests_source}");

    for needle in [
        "pub fn sanitize_motion(motion: AccordionMotion) -> AccordionMotion",
        "fn sanitize_spring(value: SpringConfig) -> SpringConfig",
        "indicator_closed_rotation_deg:",
        "indicator_open_rotation_deg:",
        "panel_offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_rotation_and_offset_ranges()",
    ] {
        assert!(
            combined_source.contains(needle),
            "Accordion motion should include `{needle}` so invalid custom values cannot leak into runtime animation state.",
        );
    }
}

#[test]
fn accordion_motion_respects_reduced_motion_preferences() {
    let source = load_source("src/accordion/motion.rs");

    for needle in [
        "fn prefers_reduced_motion() -> bool",
        "(prefers-reduced-motion: reduce)",
        "if prefers_reduced_motion()",
        "--ui-accordion-indicator-rotation",
        "--ui-accordion-panel-height",
        "--ui-accordion-panel-opacity",
        "--ui-accordion-panel-y",
    ] {
        assert!(
            source.contains(needle),
            "Accordion motion should include `{needle}` for reduced-motion fallback behavior.",
        );
    }
}

#[test]
fn accordion_file_roles_follow_layer_contract() {
    let mod_source = load_source("src/accordion/mod.rs");
    let logic_source = load_source("src/accordion/logic.rs");
    let styles_source = load_source("src/accordion/styles.rs");
    let view_source = load_source("src/accordion/view.rs");
    let motion_source = load_source("src/accordion/motion.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{Accordion, AccordionItem};",
    ] {
        assert!(
            mod_source.contains(needle),
            "Accordion module boundary should include `{needle}`."
        );
    }

    for forbidden in ["view!", "on:click", "aria-"] {
        assert!(
            !logic_source.contains(forbidden),
            "Accordion logic.rs should not contain view/a11y binding detail `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str ="),
        "Accordion styles.rs should remain the static token-first CSS contract."
    );
    assert!(
        view_source.contains("#[component]") && view_source.contains("view!"),
        "Accordion view.rs should own Leptos structure rendering."
    );
    assert!(
        view_source.contains("disclosure_trigger_attrs"),
        "Accordion view.rs should mount headless a11y contract."
    );
    assert!(
        motion_source.contains("SpringAnimator"),
        "Accordion motion.rs should map component semantic state to ui-motion driver."
    );
    assert!(
        !motion_source.contains("view!"),
        "Accordion motion.rs should not render views."
    );
}

#[test]
fn accordion_has_no_spec_rs_contract_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/accordion/spec.rs");
    assert!(
        !spec_path.exists(),
        "Accordion should not add spec.rs unless schema complexity requires it."
    );
}

#[test]
fn accordion_token_first_css_is_injected_through_root_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/accordion/styles.rs");

    for needle in [
        "#[cfg(feature = \"component-accordion\")]",
        "out.push_str(crate::accordion::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Component CSS aggregation should include `{needle}` for accordion."
        );
    }

    for needle in [
        "if inject_components_css.get_value()",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should gate component CSS injection via `{needle}`."
        );
    }

    for token_var in [
        "var(--ui-layout-content-1",
        "var(--ui-layout-divider",
        "var(--ui-layout-foreground",
        "var(--ui-space-",
        "var(--ui-radius-",
    ] {
        assert!(
            styles_source.contains(token_var),
            "Accordion styles should use token-first CSS variable `{token_var}`."
        );
    }
}

#[test]
fn accordion_tree_shaking_feature_wiring_is_component_scoped() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-accordion = [\"dep:ui-accordion\"]",
        "#[cfg(feature = \"component-accordion\")]",
        "pub use ui_accordion as accordion;",
        "out.push_str(crate::accordion::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Accordion tree-shaking contract should include `{needle}`."
        );
    }
}

#[test]
fn accordion_public_props_are_documented_in_view() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "/// Public props:",
        "/// - `id_base`",
        "/// - `selection_mode`",
        "/// - `disallow_empty_selection`",
        "/// - `is_disabled`",
        "/// - `motion`",
        "/// - `class_name`",
        "/// - `children`",
        "explicit item composition",
    ] {
        assert!(
            source.contains(needle),
            "Accordion public API docs should include `{needle}` in view.rs rustdoc.",
        );
    }
}

#[test]
fn accordion_has_component_readme() {
    let source = load_source("src/accordion/README.md");

    for needle in [
        "# Accordion",
        "## Architecture Layers",
        "## API (Table)",
        "## Semantics and Accessibility",
        "## Motion and Fallback",
        "prefers-reduced-motion: reduce",
    ] {
        assert!(
            source.contains(needle),
            "Accordion README should include `{needle}`.",
        );
    }
}

#[test]
fn accordion_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn accordion() -> AnyView",
        "title=\"Accordion\"",
        "slug=\"accordion\"",
        "description=\"Multi-panel disclosure with roving tabindex, baseline-level spring motion, and baseline-style root state attrs.\"",
        "<Playground",
        "title=\"Hello World (Uncontrolled)\"",
        "description=\"Zero wiring path: no controlled state and no headless/state-primitives setup needed.\"",
        "code_signal=hello_code",
        "<Playground title=\"Multiple + Controlled\" code_signal=code>",
        "<Playground title=\"Single + Disabled\" code_signal=states_code>",
        "<Accordion",
        "<AccordionItem",
        "selection_mode=AccordionSelectionMode::Multiple",
        "selection_mode=AccordionSelectionMode::Single",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for accordion coverage.",
        );
    }
}

#[test]
fn accordion_docs_hello_world_example_stays_under_five_lines() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    let start = source
        .find("let hello_code = Signal::derive")
        .expect("hello_code signal should exist");
    let snippet = &source[start..];
    let code_start = snippet
        .find("r#\"")
        .expect("hello_code should start with raw string literal")
        + 3;
    let code_end = snippet[code_start..]
        .find("\"#")
        .expect("hello_code should end with raw string literal");
    let code = &snippet[code_start..code_start + code_end];

    let non_empty_lines = code.lines().filter(|line| !line.trim().is_empty()).count();
    assert!(
        non_empty_lines <= 5,
        "Accordion Hello World example must be <= 5 lines, got {non_empty_lines} lines:\n{code}"
    );

    for needle in [
        "<Accordion",
        "<AccordionItem label=\"First\">\"Panel 1\"</AccordionItem>",
        "<AccordionItem label=\"Second\">\"Panel 2\"</AccordionItem>",
    ] {
        assert!(
            code.contains(needle),
            "Accordion Hello World example should include `{needle}`."
        );
    }
}

#[test]
fn accordion_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let (open_multi, set_open_multi) = signal(open_set([0]));",
        "let on_multi_0_change = on_item_open_change(set_open_multi, 0);",
        "id_base=\"docs-accordion\".to_string()",
        "<AccordionItem key=0 label=\"First\" open=open_multi_0 on_open_change=on_multi_0_change>",
        "<AccordionItem key=1 label=\"Second\" open=open_multi_1 on_open_change=on_multi_1_change>",
        "<AccordionItem key=2 label=\"Third\" open=open_multi_2 on_open_change=on_multi_2_change>",
        "\"Panel 1 content\"",
        "\"Panel 2 content\"",
        "\"Panel 3 content\"",
        "\"open: \"",
        "format!(\"{open:?}\")",
        "let (open_single, set_open_single) = signal(open_set([1]));",
        "let on_single_0_change = on_item_open_change(set_open_single, 0);",
        "id_base=\"docs-accordion-single\".to_string()",
        "<AccordionItem key=0 label=\"Overview\" open=open_single_0 on_open_change=on_single_0_change>",
        "<AccordionItem key=1 label=\"Details\" open=open_single_1 on_open_change=on_single_1_change>",
        "<AccordionItem key=2 label=\"History\" open=open_single_2 on_open_change=on_single_2_change is_disabled=true>",
        "\"Overview content\"",
        "\"Details content\"",
        "\"History content\"",
        "\"single open: \"",
        "\"disabled index: 2\"",
        "disallow_empty_selection=true",
    ] {
        assert!(
            source.contains(needle),
            "accordion docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn accordion_docs_api_contract_aligns_with_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "pub fn normalize_default_open_for_items(",
        "pub fn resolve_open_init_source(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should expose default/init normalization contract `{needle}`."
        );
    }

    let start = docs_source
        .find("pub(super) fn accordion() -> AnyView")
        .expect("accordion docs function should exist");
    let tail = &docs_source[start..];
    let end = tail
        .find("pub(super) fn disclosure() -> AnyView")
        .expect("accordion docs section should terminate before disclosure section");
    let accordion_docs = &tail[..end];

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "open=open_multi_0",
        "on_open_change=on_multi_0_change",
        "open=open_single_0",
        "on_open_change=on_single_0_change",
        "selection_mode=AccordionSelectionMode::Single",
        "selection_mode=AccordionSelectionMode::Multiple",
        "<AccordionItem key=2 label=\"History\" open=open_single_2 on_open_change=on_single_2_change is_disabled=true>",
    ] {
        assert!(
            accordion_docs.contains(needle),
            "Accordion docs API/default matrix should include `{needle}`."
        );
    }

    for legacy in [
        "on_open_indices_change=",
        "default_open_indices=",
        "open_indices=",
    ] {
        assert!(
            !accordion_docs.contains(legacy),
            "Accordion docs should not drift to legacy API alias `{legacy}`."
        );
    }
}

#[test]
fn accordion_docs_examples_remain_copy_paste_ready() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let accordion_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines",
        "<CodeBlock code=resolved_code.get() />",
        "let hello_code = Signal::derive",
        "<Playground",
        "code_signal=hello_code",
    ] {
        assert!(
            playground_source.contains(needle) || accordion_docs_source.contains(needle),
            "Accordion docs copy-paste-ready contract should include `{needle}`."
        );
    }
}

#[test]
fn accordion_docs_workbench_supports_live_style_tuning_and_optional_persist() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "const ACCORDION_WORKBENCH_STORAGE_KEY: &str = \"docs:accordion:workbench:open\";",
        "fn load_workbench_open() -> Option<BTreeSet<usize>>",
        "fn save_workbench_open(indices: &BTreeSet<usize>)",
        "fn clear_workbench_open()",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
        "data-slot=\"accordion-workbench-controls\"",
        "data-slot=\"accordion-workbench-canvas\"",
        "Persist open state (optional)",
        "--ui-radius-md:",
        "--ui-accordion-trigger-hover-bg:",
        "style=workbench_style",
    ] {
        assert!(
            source.contains(needle),
            "Accordion docs workbench contract should include `{needle}`."
        );
    }
}

#[test]
fn accordion_parameter_strategy_doc_tracks_current_model() {
    let source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "### Accordion 对齐记录（2026-02）",
        "selection_mode",
        "open",
        "on_open_change",
        "default_open",
        "#/components/accordion",
    ] {
        assert!(
            source.contains(needle),
            "Parameter strategy doc should track accordion parameter model via `{needle}`."
        );
    }
}

#[test]
fn docs_app_exposes_default_theme_visual_baseline_page() {
    let catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            catalog_source.contains(needle),
            "docs-app component catalog should include default theme baseline entry `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default Theme Visual Baseline",
        "hover/active/focus",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            page_source.contains(needle),
            "default theme baseline page should include `{needle}`."
        );
    }
}

#[test]
fn docs_app_visual_baseline_has_screenshot_regression_contract() {
    let source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "/#/components/theme-visual-baseline",
        "E2E_VISUAL_BASELINE",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
    ] {
        assert!(
            source.contains(needle),
            "visual baseline e2e contract should include `{needle}`."
        );
    }
}

#[test]
fn ui_headless_has_web_ssr_compile_error_mutex_guard() {
    let source = load_source("../../crates/ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            source.contains(needle),
            "ui-headless should keep feature mutex guard `{needle}`."
        );
    }
}

#[test]
fn accordion_explicitly_gates_browser_calls_for_non_wasm_paths() {
    let view_source = load_source("src/accordion/view.rs");
    let motion_source = load_source("src/accordion/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn focus_trigger(",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should keep explicit wasm/non-wasm focus branching via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "fn prefers_reduced_motion() -> bool",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_indicator_motion(",
        "pub fn attach_panel_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Accordion motion should keep explicit wasm/non-wasm motion branching via `{needle}`."
        );
    }
}

#[test]
fn ui_motion_non_wasm_stub_contract_is_explicit() {
    let source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            source.contains(needle),
            "ui-motion should keep explicit non-wasm stub contract `{needle}`."
        );
    }
}

#[test]
fn accordion_view_macro_complexity_is_split_by_semantic_fragments() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "fn compose_root_class_name(class_name: Option<String>) -> String",
        "fn render_item_label(label: String) -> impl IntoView",
        "fn render_item_indicator(indicator_ref: NodeRef<html::Span>) -> impl IntoView",
        "struct AccordionPanelRenderInput",
        "fn render_item_panel<",
        "let label_view = render_item_label(label);",
        "let indicator_view = render_item_indicator(indicator_ref);",
        "let panel_view = render_item_panel(AccordionPanelRenderInput",
        "{label_view}",
        "{indicator_view}",
        "{panel_view}",
    ] {
        assert!(
            source.contains(needle),
            "Accordion view complexity should be split through semantic helper fragments via `{needle}`."
        );
    }

    let component_count = source.matches("#[component]").count();
    assert_eq!(
        component_count, 2,
        "Accordion view should keep function-style split helpers; only Accordion + AccordionItem components are allowed."
    );
}

#[test]
fn accordion_static_fragments_are_constantized_for_stable_rendering() {
    let source = load_source("src/accordion/view.rs");

    for needle in [
        "const ACCORDION_BASE_CLASS: &str = \"ui-accordion\";",
        "const ACCORDION_INDICATOR_GLYPH: &str = \"›\";",
        "{ACCORDION_INDICATOR_GLYPH}",
    ] {
        assert!(
            source.contains(needle),
            "Accordion should centralize static fragments with constants via `{needle}`."
        );
    }
}

#[test]
fn accordion_disallows_inner_html_in_runtime_markup_paths() {
    for rel_path in [
        "src/accordion/view.rs",
        "src/accordion/logic.rs",
        "src/accordion/motion.rs",
        "src/accordion/styles.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Accordion must not inject runtime HTML; found `{forbidden}` in `{rel_path}`."
            );
        }
    }
}

#[test]
fn accordion_wasm_debug_contract_is_feature_gated_and_dev_only() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/accordion/view.rs");

    assert!(
        cargo_source.contains("accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]"),
        "Accordion wasm debug should be an opt-in feature gated behind component-accordion.",
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("]\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("accordion-wasm-debug"),
        "Debug feature must not be included in all-components/default production paths."
    );

    for needle in [
        "feature = \"accordion-wasm-debug\"",
        "debug_assertions",
        "target_arch = \"wasm32\"",
        "Accordion Debug (wasm dev)",
        "data-slot=\"accordion-debug-entry\"",
        "data-slot=\"accordion-debug-event\"",
        "data-slot=\"accordion-debug-replay\"",
        "data-debug-source=source",
        "data-debug-before=before_text.clone()",
        "data-debug-after=after_text.clone()",
        "data-debug-timestamp-ms=timestamp_text.clone()",
        "request_open_change.run((",
        "AccordionOpenChangeSource::Programmatic",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion wasm debug contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] debug"),
        "Accordion public API should not expose debug props."
    );
}

#[test]
fn accordion_engineering_contract_uses_tracing_and_avoids_runtime_leaks() {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");
    let motion_source = load_source("src/accordion/motion.rs");
    let mod_source = load_source("src/accordion/mod.rs");

    assert!(
        cargo_source.contains("accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]"),
        "Engineering contract should keep tracing behind accordion-wasm-debug feature."
    );

    for needle in [
        "tracing::event!(",
        "target: \"ui_components::accordion::state_change\"",
        "source = source.as_str()",
        "before = %format_indices(before)",
        "after = %format_indices(after)",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion tracing contract should include `{needle}`."
        );
    }

    for source in [&view_source, &logic_source, &motion_source, &mod_source] {
        for forbidden in ["tokio", "async_std", "async-std", "serde::"] {
            assert!(
                !source.contains(forbidden),
                "Accordion engineering contract should not leak runtime/serde/platform detail `{forbidden}`."
            );
        }
    }
    assert!(
        !mod_source.contains("web_sys"),
        "Accordion public module boundary should not leak web_sys types."
    );
}

#[test]
fn status_primitives_remain_dom_and_style_free() {
    let sources = load_rust_sources_under("../../crates/ui-state-primitives/src");

    for (path, source) in sources {
        for forbidden in [
            "leptos", "web_sys", "web-sys", "view!", "class=", "style=", "aria-",
        ] {
            assert!(
                !source.contains(forbidden),
                "ui-state-primitives must stay DOM/style free; found `{forbidden}` in `{path:?}`."
            );
        }
    }
}

#[test]
fn ui_headless_remains_semantic_without_visual_or_motion_orchestration() {
    let sources = load_rust_sources_under("../../crates/ui-headless/src");

    for (path, source) in sources {
        for forbidden in [
            ".ui-",
            "@keyframes",
            "class=",
            "style=",
            "SpringAnimator",
            "animate(",
            "transition:",
        ] {
            assert!(
                !source.contains(forbidden),
                "ui-headless should not host visual/motion orchestration `{forbidden}` in `{path:?}`."
            );
        }
    }
}

#[test]
fn accordion_view_consumes_logic_outputs_without_hiding_state_machine_rules() {
    let view_source = load_source("src/accordion/view.rs");
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "logic::resolve_state(",
        "logic::resolve_agent_contract(",
        "logic::normalize_default_open_for_items(",
        "logic::toggle_open_for_items(",
    ] {
        assert!(
            view_source.contains(needle),
            "Accordion view should consume logic output `{needle}`."
        );
    }

    for forbidden in [
        "pub enum AccordionOpen",
        "pub struct AccordionState",
        "pub fn resolve_state(",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Accordion view must not define core state machine contract `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub struct AccordionState"),
        "AccordionState contract should remain in logic.rs."
    );
}

#[test]
fn accordion_has_no_temporary_patch_markers_or_contract_bypasses() {
    for rel_path in [
        "src/accordion/mod.rs",
        "src/accordion/logic.rs",
        "src/accordion/view.rs",
        "src/accordion/motion.rs",
        "src/accordion/styles.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["TODO", "FIXME", "HACK", "TEMP", "workaround"] {
            assert!(
                !source.contains(forbidden),
                "Accordion files should not carry temporary bypass marker `{forbidden}` in `{rel_path}`."
            );
        }
    }
}

#[test]
fn accordion_reuses_state_primitives_instead_of_reimplementing_reusable_rules() {
    let logic_source = load_source("src/accordion/logic.rs");

    for needle in [
        "normalize_open_keys",
        "normalize_default_open_keys",
        "toggle_open_key",
        "pub fn normalize_open_for_items(",
        "pub fn toggle_open_for_items(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Accordion logic should route state invariants through ui-state-primitives `{needle}`."
        );
    }

    assert!(
        logic_source.contains("disallow_empty_selection"),
        "Accordion logic should include disallow-empty selection guard."
    );
}

#[test]
fn ui_components_entry_file_locations_follow_contract() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let _active_highlight = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-accordion\")]",
        "pub use ui_accordion as accordion;",
        "pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-accordion\")]",
        "out.push_str(crate::accordion::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css entry should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "ui-components root entry should keep `{needle}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        let path = manifest_dir.join(forbidden);
        assert!(
            !path.exists(),
            "ui-components should not host `{forbidden}`; capability belongs to ui-headless."
        );
    }
}

#[test]
fn accordion_directory_standard_files_follow_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let accordion_dir = manifest_dir.join("../../components/accordion/src");
    let legacy_accordion_dir = manifest_dir.join("src/accordion");

    assert!(
        !legacy_accordion_dir.exists(),
        "Legacy ui-components accordion directory should be removed after split."
    );

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = accordion_dir.join(required);
        assert!(
            path.exists(),
            "Accordion directory should include `{required}`."
        );
    }

    let mod_source = load_source("src/accordion/mod.rs");
    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{Accordion, AccordionItem};",
    ] {
        assert!(
            mod_source.contains(needle),
            "Accordion module entry should keep `{needle}`."
        );
    }
}

#[test]
fn docs_perf_probe_budgets_are_wired_for_component_pages() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "use crate::perf_probe::{UiPerfBudget, UiPerfProbe};",
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"accordion\" => UiPerfBudget {",
        "UiPerfBudget::mount_only(120.0)",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should define budgeted perf probe via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-observability",
        "data-perf-violation",
    ] {
        assert!(
            coverage_source.contains(needle),
            "e2e coverage should assert perf contract marker `{needle}`."
        );
    }
}

#[test]
fn accordion_e2e_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_accordion.spec.mjs");

    for needle in [
        "page.goto(\"/#/components/accordion\")",
        "const accordions = page.locator(\"[data-slot=\\\"accordion\\\"]\");",
        "[data-slot=\\\"accordion-trigger\\\"][data-index=\\\"0\\\"]",
        "[data-slot=\\\"accordion-panel\\\"][data-index=\\\"1\\\"]",
        "toHaveAttribute(\"data-has-items\", \"true\")",
        "\"data-ui-schema\"",
        "\"ui.accordion.agent-contract\"",
        "\"data-ui-stream-support\", \"unsupported\"",
        "toHaveAttribute(\"data-open\", \"true\")",
        "not.toHaveAttribute(\"data-open\", \"true\")",
        "(toggle-pointer|external-sync)",
        "(draft|verified|submittable)",
        "await page.keyboard.press(\"ArrowDown\")",
        "await page.keyboard.press(\"Space\")",
    ] {
        assert!(
            source.contains(needle),
            "Accordion e2e contract should include semantic selector/wait marker `{needle}`."
        );
    }

    assert!(
        !source.contains("waitForTimeout("),
        "Accordion e2e should avoid fixed sleep waits."
    );
}

#[test]
fn perf_render_count_follow_up_is_tracked_in_plan() {
    let source = load_source("../../docs/plan/TODO.md");
    assert!(
        source.contains("render_count"),
        "perf governance should keep explicit follow-up task for render_count automation."
    );
}
