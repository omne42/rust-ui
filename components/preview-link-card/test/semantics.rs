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
fn preview_link_card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/preview_link_card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "PreviewLinkCard internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn preview_link_card_module_keeps_minimal_exports_and_moves_internal_types_to_logic() {
    let mod_source = load_source("src/preview_link_card/mod.rs");
    let logic_source = load_source("src/preview_link_card/logic.rs");

    for needle in [
        "pub use view::PreviewLinkCard;",
        "pub use motion::PreviewLinkCardMotion;",
        "DEFAULT_TITLE",
        "DEFAULT_DESCRIPTION",
        "DEFAULT_URL",
        "DEFAULT_SITE_LABEL",
        "DEFAULT_OPEN_DELAY_MS",
        "DEFAULT_CLOSE_DELAY_MS",
    ] {
        assert!(
            mod_source.contains(needle),
            "preview_link_card module should include `{needle}` as stable exports."
        );
    }

    for forbidden in [
        "enum PreviewLinkCardSlot",
        "enum PreviewLinkCardStateAttr",
        "enum PreviewLinkCardContentAttr",
        "enum PreviewLinkCardSourceAttr",
        "enum PreviewLinkCardOpenModeAttr",
        "enum PreviewLinkCardSiteLabelSourceAttr",
        "struct PreviewLinkCardPartStateInput",
        "struct PreviewLinkCardPartState",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry internal implementation type `{forbidden}`."
        );
    }

    for needle in [
        "pub(crate) enum PreviewLinkCardSlot",
        "pub(crate) enum PreviewLinkCardStateAttr",
        "pub(crate) enum PreviewLinkCardContentAttr",
        "pub(crate) enum PreviewLinkCardSourceAttr",
        "pub(crate) enum PreviewLinkCardOpenModeAttr",
        "pub(crate) enum PreviewLinkCardSiteLabelSourceAttr",
        "pub(crate) struct PreviewLinkCardPartStateInput",
        "pub(crate) struct PreviewLinkCardPartState",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should own internal type `{needle}`."
        );
    }
}

#[test]
fn preview_link_card_is_exported_from_crate_root() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod preview_link_card;"),
        "crate root should expose `preview_link_card` module."
    );
    assert!(
        source.contains("pub use preview_link_card::{PreviewLinkCard, PreviewLinkCardMotion};"),
        "crate root should re-export PreviewLinkCard contracts."
    );
}

#[test]
fn preview_link_card_logic_exposes_state_and_source_helpers() {
    let source = load_source("src/preview_link_card/logic.rs");

    for needle in [
        "pub fn state_attr_for_open(is_open: bool)",
        "pub struct OpenStateMarkersInput {",
        "pub struct OpenStateMarkers {",
        "pub fn resolve_open_state_markers(input: OpenStateMarkersInput) -> OpenStateMarkers",
        "pub fn content_attr(has_image: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub struct DelayInput {",
        "pub struct Delays {",
        "pub fn normalize_delays(input: DelayInput) -> Delays",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn resolve_title(value: Option<String>)",
        "pub fn resolve_description(value: Option<String>)",
        "pub fn resolve_url(value: Option<String>)",
        "pub fn resolve_site_label(",
        "resolved_url: &str",
        "pub fn resolve_image_src(image_src: Option<String>)",
        "pub struct OpenStateInput {",
        "pub struct OpenState {",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub struct OpenStateSourceMarkersInput {",
        "pub struct OpenStateSourceMarkers {",
        "pub fn resolve_open_state_source_markers(",
        "pub fn resolve_part_state(input: PreviewLinkCardPartStateInput)",
        "pub fn compose_class_name(",
        "state: PreviewLinkCardPartState",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
    ] {
        assert!(
            source.contains(needle),
            "PreviewLinkCard logic should include `{needle}` for centralized contracts."
        );
    }
}

#[test]
fn preview_link_card_view_uses_hover_trigger_position_and_motion_contracts() {
    let source = load_source("src/preview_link_card/view.rs");

    for needle in [
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_open: Option<Signal<bool>>,",
        "#[prop(optional)] open: Option<Signal<bool>>,",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>,",
        "#[prop(optional)] open_delay_ms: Option<u64>,",
        "#[prop(optional)] close_delay_ms: Option<u64>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "use_hover_card_trigger(HoverCardTriggerOptions",
        "use_hover_card_dismiss(HoverCardDismissOptions",
        "use_hover_card_focus_a11y(HoverCardFocusA11yOptions",
        "use_popover_position(PopoverPositionOptions",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
        "tooltip_panel_attrs(TooltipPanelA11yOptions {",
        "motion::attach_motion(",
        "let open_markers = Memo::new(move |_| {",
        "logic::resolve_open_state_markers(logic::OpenStateMarkersInput {",
        "logic::normalize_delays(logic::DelayInput {",
        "logic::normalize_open_state(logic::OpenStateInput {",
        "logic::resolve_open_state_source_markers(logic::OpenStateSourceMarkersInput {",
        "open,",
        "default_open,",
        "on_open_change,",
        "logic::resolve_part_state(logic::PreviewLinkCardPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(",
        "style=panel_vars",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "data-focus-a11y-managed=focus_a11y.attrs.manages_aria_describedby.then_some(\"true\")",
        "data-slot=root_state.slot_attr",
        "data-content=root_state.content_attr.as_attr()",
        "data-delay-source=root_state.delay_source_attr.as_attr()",
        "data-site-label-source=root_state.site_label_source_attr.as_attr()",
        "data-motion-source=root_state.motion_source_attr.as_attr()",
        "data-open-mode=open_state_source_markers.open_mode_attr.as_attr()",
        "data-open-source=open_state_source_markers.open_source_attr.as_attr()",
        "data-default-open-source=open_state_source_markers.default_open_source_attr.as_attr()",
        "data-open-change-source=open_state_source_markers.open_change_source_attr.as_attr()",
        "data-controlled=matches!(",
        "data-uncontrolled=matches!(",
        "lang=root_lang.clone()",
        "dir=root_dir",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
        "role=move || panel_a11y.get().attrs.role",
        "lang=move || panel_a11y.get().attrs.lang.clone()",
        "dir=move || panel_a11y.get().attrs.dir",
        "on:pointerenter=move |_| trigger_aria.handlers.on_trigger_pointer_enter.run(())",
        "on:pointerleave=move |_| trigger_aria.handlers.on_trigger_pointer_leave.run(())",
        "on:focusin=move |ev| trigger_on_focus_in.run(ev)",
        "on:focusout=move |ev| trigger_on_focus_out.run(ev)",
        "on:keydown=move |ev| trigger_on_key_down.run(ev)",
        "on:pointerenter=move |_| trigger_aria.handlers.on_panel_pointer_enter.run(())",
        "on:pointerleave=move |_| trigger_aria.handlers.on_panel_pointer_leave.run(())",
        "on:focusin=move |_| trigger_aria.handlers.on_panel_focus_in.run(())",
        "on:focusout=move |_| trigger_aria.handlers.on_panel_focus_out.run(())",
        "on:keydown=move |ev| panel_on_key_down.run(ev)",
        "data-slot=\"preview-link-card-image\"",
        "data-slot=\"preview-link-card-title\"",
        "data-slot=\"preview-link-card-description\"",
        "data-slot=\"preview-link-card-site-label\"",
        "data-slot=\"preview-link-card-url\"",
    ] {
        assert!(
            source.contains(needle),
            "PreviewLinkCard view should include `{needle}` for stable overlay/source contracts."
        );
    }

    for forbidden in [
        "#[prop(optional, default = logic::DEFAULT_OPEN_DELAY_MS)] open_delay_ms: u64,",
        "#[prop(optional, default = logic::DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64,",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
        "role=\"tooltip\"",
    ] {
        assert!(
            !source.contains(forbidden),
            "PreviewLinkCard view must not keep duplicated state/default derivations; found `{forbidden}`."
        );
    }
}

#[test]
fn preview_link_card_view_avoids_business_store_and_local_state_primitives() {
    let source = load_source("src/preview_link_card/view.rs");

    for forbidden in [
        "RwSignal::new(",
        "create_rw_signal(",
        "provide_context(",
        "expect_context(",
        "use_context(",
        "create_resource(",
        "create_local_resource(",
    ] {
        assert!(
            !source.contains(forbidden),
            "PreviewLinkCard view must not bind business store or local state primitives; found `{forbidden}`."
        );
    }
}

#[test]
fn preview_link_card_styles_include_state_source_and_content_markers() {
    let source = load_source("src/preview_link_card/styles.rs");

    for selector in [
        ".ui-preview-link-card {",
        ".ui-preview-link-card[data-state=\"open\"]",
        ".ui-preview-link-card[data-content=\"media\"]",
        ".ui-preview-link-card[data-content=\"text\"]",
        ".ui-preview-link-card[data-class-source=\"custom\"]",
        ".ui-preview-link-card[data-delay-source=\"custom\"]",
        ".ui-preview-link-card[data-id-source=\"custom\"]",
        ".ui-preview-link-card[data-title-source=\"custom\"]",
        ".ui-preview-link-card[data-description-source=\"custom\"]",
        ".ui-preview-link-card[data-url-source=\"custom\"]",
        ".ui-preview-link-card[data-motion-source=\"custom\"]",
        ".ui-preview-link-card[data-custom-motion=\"true\"]",
        "opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.72));",
        "var(--ui-tooltip-max-width, var(--ui-fallback-tooltip-max-width, 380px))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width, 280px))",
        "z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index, 1000));",
        "var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y, 8px))",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale, 0.98))",
        ".ui-preview-link-card__trigger[data-state=\"trigger\"]",
        ".ui-preview-link-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(selector),
            "PreviewLinkCard styles should include `{selector}` as stable selectors."
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type("] {
        assert!(
            !source.contains(forbidden),
            "PreviewLinkCard styles must not depend on brittle structural selector `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn preview_link_card_motion_contract_exposes_default_and_customization_checks() {
    let mod_source = load_source("src/preview_link_card/mod.rs");
    let motion_source = load_source("src/preview_link_card/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::PreviewLinkCardMotion;",
        "pub struct PreviewLinkCardMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "PreviewLinkCard motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn preview_link_card_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::preview_link_card::styles::CSS);"),
        "ui css aggregator should include preview_link_card styles."
    );
}

#[test]
fn preview_link_card_motion_keeps_wasm_and_non_wasm_paths() {
    let source = load_source("src/preview_link_card/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "ui_popover::motion::attach_motion_with_config(",
        "Effect::new(move |_| {",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            source.contains(needle),
            "preview-link-card motion should keep wasm/non-wasm semantic path `{needle}`."
        );
    }
}

#[test]
fn preview_link_card_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn preview_link_card() -> AnyView",
        "title=\"PreviewLinkCard\"",
        "slug=\"preview-link-card\"",
        "State + Source Markers",
        "data-title-source",
        "data-description-source",
        "data-url-source",
        "data-site-label-source",
        "data-motion-source",
        "<PreviewLinkCard",
    ] {
        assert!(
            source.contains(needle),
            "PreviewLinkCard docs page should contain `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn preview_link_card_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/preview_link_card/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: PreviewLinkCardMotion) -> PreviewLinkCardMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "drop(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset_ranges()",
    ] {
        assert!(
            motion_source.contains(needle),
            "PreviewLinkCard motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn preview_link_card_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-preview-link-card\".to_string()",
        "open_delay_ms=260",
        "close_delay_ms=240",
        "class_name=\"docs-preview-link-card-state\".to_string()",
        "motion=PreviewLinkCardMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "..PreviewLinkCardMotion::default()",
        "site_label=\"ui-baseline.adobe.com\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "preview-link-card docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn preview_link_card_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn preview_link_card() -> AnyView",
        "title=\"PreviewLinkCard\"",
        "slug=\"preview-link-card\"",
        "description=\"Hover-triggered preview link card with overlay positioning, motion contract, and source markers.\"",
        "<Playground title=\"Preview Snapshot\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "code_signal=markers_code",
        "<Playground title=\"Default Fallbacks\" code_signal=fallback_code>",
        "<PreviewLinkCard",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for preview-link-card primary playground coverage.",
        );
    }
}

#[test]
fn preview_link_card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Preview Snapshot\"",
        "title=\"Rust UI docs\".to_string()",
        "description=\"Preview component behavior and source markers.\".to_string()",
        "url=\"https://github.com/adobe/ui-baseline\".to_string()",
        "image_src=\"https://avatars.githubusercontent.com/u/476009?v=4\".to_string()",
        "title=\"State + Source Markers\"",
        "id=\"docs-preview-link-card\".to_string()",
        "title=\"Custom title\".to_string()",
        "description=\"Custom description for source markers.\".to_string()",
        "url=\"https://ui-baseline.adobe.com\".to_string()",
        "site_label=\"ui-baseline.adobe.com\".to_string()",
        "open_delay_ms=260",
        "close_delay_ms=240",
        "class_name=\"docs-preview-link-card-state\".to_string()",
        "motion=PreviewLinkCardMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "\"Inspect markers\"",
        "title=\"Default Fallbacks\"",
        "\"Uses defaults\"",
        "Falls back to default title/description/url/site-label when not provided.",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for preview-link-card contracts.",
        );
    }
}

#[test]
fn preview_link_card_docs_hello_world_keeps_minimal_default_api_path() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays/preview_link_card.rs");

    assert!(
        source.contains("title=\"Hello World (Default PreviewLinkCard)\""),
        "PreviewLinkCard docs must expose a clear Hello World default path."
    );

    let section = source
        .split_once("let hello_code = Signal::derive(move || {")
        .map(|(_, tail)| tail)
        .unwrap_or_else(|| panic!("hello_code section should exist in docs source"));

    let snippet_start = section
        .find("r##\"")
        .map(|idx| idx + 4)
        .unwrap_or_else(|| panic!("hello_code should use a raw string snippet"));
    let snippet_end = section[snippet_start..]
        .find("\"##")
        .map(|idx| snippet_start + idx)
        .unwrap_or_else(|| panic!("hello_code snippet should terminate with raw string marker"));
    let snippet = &section[snippet_start..snippet_end];

    assert!(
        snippet.lines().count() <= 5,
        "Hello World snippet must stay within 5 lines; got {} lines:\n{}",
        snippet.lines().count(),
        snippet
    );
    assert!(
        snippet.contains("<PreviewLinkCard"),
        "Hello World snippet must render PreviewLinkCard directly."
    );
    assert!(
        snippet.contains("trigger="),
        "Hello World snippet should show the minimal required trigger prop."
    );
    for forbidden in [
        "is_open=",
        "open=",
        "default_open=",
        "on_open_change=",
        "state=",
    ] {
        assert!(
            !snippet.contains(forbidden),
            "Hello World snippet must not require advanced state wiring; found `{forbidden}`."
        );
    }
}

#[test]
fn preview_link_card_tree_shaking_contract_is_feature_gated_and_css_prunable() {
    let ui_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-preview_link_card = []",
        "\"component-preview_link_card\"",
    ] {
        assert!(
            ui_cargo.contains(needle),
            "ui Cargo features should register preview-link-card tree-shaking gate `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-preview_link_card\")]",
        "pub mod preview_link_card;",
        "#[path = \"../../../components/preview-link-card/src/mod.rs\"]",
    ] {
        assert!(
            ui_lib.contains(needle),
            "ui lib should gate preview-link-card module export with feature `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-preview_link_card\")]",
        "out.push_str(crate::preview_link_card::styles::CSS);",
    ] {
        assert!(
            ui_css.contains(needle),
            "ui css aggregator should gate preview-link-card CSS with feature `{needle}`."
        );
    }

    assert!(
        web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "web-demo should depend on ui with web-demo-components feature bundle."
    );
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not pull all-components implicitly for tree-shaking checks."
    );

    for needle in [
        "PREVIEW_LINK_CARD_MIN_FEATURES=\"component-preview_link_card,inject-css\"",
        "[tree-shaking] preview-link-card minimal feature tree",
        "feature \"component-preview_link_card\" (command-line)",
        "preview-link-card minimal wasm check",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$PREVIEW_LINK_CARD_MIN_FEATURES\"",
        "WEB_DEMO_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p web-demo)\"",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should include `{needle}` for feature isolation contract."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}
