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
fn preview_link_card_module_exposes_slot_state_motion_contracts() {
    let source = load_source("src/preview_link_card/mod.rs");

    for needle in [
        "pub enum PreviewLinkCardSlot",
        "pub struct PreviewLinkCardPartStateInput",
        "pub struct PreviewLinkCardPartState",
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
            source.contains(needle),
            "preview_link_card module should include `{needle}` contracts."
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
        "pub fn content_attr(has_image: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: String)",
        "pub fn resolve_title(value: Option<String>)",
        "pub fn resolve_description(value: Option<String>)",
        "pub fn resolve_url(value: Option<String>)",
        "pub fn resolve_site_label(",
        "resolved_url: &str",
        "pub fn resolve_image_src(image_src: Option<String>)",
        "pub fn resolve_part_state(input: PreviewLinkCardPartStateInput)",
        "pub fn compose_class_name(",
        "state: PreviewLinkCardPartState",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
        "pub fn should_handle_escape(key: &str, is_open: bool, is_composing: bool)",
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
        "use_hover_card_trigger(HoverCardTriggerOptions",
        "use_popover_position(PopoverPositionOptions",
        "motion::attach_motion(",
        "trigger_aria.state.dismiss.run(())",
        "logic::resolve_part_state(PreviewLinkCardPartStateInput {",
        "logic::compose_class_name(class_name, root_state)",
        "logic::compose_panel_vars(",
        "logic::should_handle_escape(&ev.key(), open_signal.get_untracked(), is_composing)",
        "data-slot=root_state.slot_attr",
        "data-content=root_state.content_attr",
        "data-delay-source=root_state.delay_source_attr",
        "data-site-label-source=root_state.site_label_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
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
        ".ui-preview-link-card__trigger[data-state=\"trigger\"]",
        ".ui-preview-link-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(selector),
            "PreviewLinkCard styles should include `{selector}` as stable selectors."
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
        "ui-components css aggregator should include preview_link_card styles."
    );
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
