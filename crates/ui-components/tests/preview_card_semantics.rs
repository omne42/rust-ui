use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn preview_card_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/preview_card/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "PreviewCard internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn preview_card_module_exposes_slot_state_motion_contracts() {
    let source = load_source("src/preview_card/mod.rs");

    for needle in [
        "pub enum PreviewCardSlot",
        "pub struct PreviewCardPartStateInput",
        "pub struct PreviewCardPartState",
        "pub use view::PreviewCard;",
        "pub use motion::PreviewCardMotion;",
        "DEFAULT_TITLE",
        "DEFAULT_DESCRIPTION",
        "DEFAULT_URL",
        "DEFAULT_SITE_LABEL",
        "DEFAULT_OPEN_DELAY_MS",
        "DEFAULT_CLOSE_DELAY_MS",
    ] {
        assert!(
            source.contains(needle),
            "preview_card module should include `{needle}` contracts."
        );
    }
}

#[test]
fn preview_card_is_exported_from_crate_root() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod preview_card;"),
        "crate root should expose `preview_card` module."
    );
    assert!(
        source.contains("pub use preview_card::{PreviewCard, PreviewCardMotion};"),
        "crate root should re-export PreviewCard contracts."
    );
}

#[test]
fn preview_card_logic_exposes_state_and_source_helpers() {
    let source = load_source("src/preview_card/logic.rs");

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
        "pub fn resolve_part_state(input: PreviewCardPartStateInput)",
        "pub fn compose_class_name(base_class_name: Option<String>, state: PreviewCardPartState)",
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64)",
        "pub fn should_handle_escape(key: &str, is_open: bool, is_composing: bool)",
    ] {
        assert!(
            source.contains(needle),
            "PreviewCard logic should include `{needle}` for centralized contracts."
        );
    }
}

#[test]
fn preview_card_view_uses_hover_trigger_position_and_motion_contracts() {
    let source = load_source("src/preview_card/view.rs");

    for needle in [
        "use_hover_card_trigger(HoverCardTriggerOptions",
        "use_popover_position(PopoverPositionOptions",
        "motion::attach_motion(",
        "trigger_aria.state.dismiss.run(())",
        "logic::resolve_part_state(PreviewCardPartStateInput {",
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
        "data-slot=\"preview-card-image\"",
        "data-slot=\"preview-card-title\"",
        "data-slot=\"preview-card-description\"",
        "data-slot=\"preview-card-site-label\"",
        "data-slot=\"preview-card-url\"",
    ] {
        assert!(
            source.contains(needle),
            "PreviewCard view should include `{needle}` for stable overlay/source contracts."
        );
    }
}

#[test]
fn preview_card_styles_include_state_source_and_content_markers() {
    let source = load_source("src/preview_card/styles.rs");

    for selector in [
        ".ui-preview-card {",
        ".ui-preview-card[data-state=\"open\"]",
        ".ui-preview-card[data-content=\"media\"]",
        ".ui-preview-card[data-content=\"text\"]",
        ".ui-preview-card[data-class-source=\"custom\"]",
        ".ui-preview-card[data-delay-source=\"custom\"]",
        ".ui-preview-card[data-id-source=\"custom\"]",
        ".ui-preview-card[data-title-source=\"custom\"]",
        ".ui-preview-card[data-description-source=\"custom\"]",
        ".ui-preview-card[data-url-source=\"custom\"]",
        ".ui-preview-card[data-motion-source=\"custom\"]",
        ".ui-preview-card[data-custom-motion=\"true\"]",
        ".ui-preview-card__trigger[data-state=\"trigger\"]",
        ".ui-preview-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(selector),
            "PreviewCard styles should include `{selector}` as stable selectors."
        );
    }
}

#[test]
fn preview_card_motion_contract_exposes_default_and_customization_tests() {
    let mod_source = load_source("src/preview_card/mod.rs");
    let motion_source = load_source("src/preview_card/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::PreviewCardMotion;",
        "pub struct PreviewCardMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_y_follows_vertical_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "PreviewCard motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn preview_card_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::preview_card::styles::CSS);"),
        "ui-components css aggregator should include preview_card styles."
    );
}

#[test]
fn preview_card_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn preview_card() -> AnyView",
        "title=\"PreviewCard\"",
        "slug=\"preview-card\"",
        "State + Source Markers",
        "data-title-source",
        "data-description-source",
        "data-url-source",
        "data-site-label-source",
        "data-motion-source",
        "<PreviewCard",
    ] {
        assert!(
            source.contains(needle),
            "PreviewCard docs page should contain `{needle}`."
        );
    }
}

#[test]
fn preview_card_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/preview_card/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: PreviewCardMotion) -> PreviewCardMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_scale:",
        "offset_y_px:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let _ = sanitize_motion(motion);",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_and_offset_ranges()",
    ] {
        assert!(
            motion_source.contains(needle),
            "PreviewCard motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn preview_card_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id=\"docs-preview-card\".to_string()",
        "open_delay_ms=260",
        "close_delay_ms=240",
        "class_name=\"docs-preview-card-state\".to_string()",
        "motion=PreviewCardMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "..PreviewCardMotion::default()",
        "site_label=\"github.com\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "preview-card docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn preview_card_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn preview_card() -> AnyView",
        "title=\"PreviewCard\"",
        "slug=\"preview-card\"",
        "description=\"baseline-compatible link preview popover with hover/focus trigger semantics, source-state markers, and baseline-level spring motion.\"",
        "<Playground title=\"Basic Preview\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "code_signal=markers_code",
        "<Playground title=\"Default Fallbacks\" code_signal=fallback_code>",
        "<PreviewCard",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for preview-card primary playground coverage.",
        );
    }
}

#[test]
fn preview_card_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Basic Preview\"",
        "title=\"UI Baseline\".to_string()",
        "description=\"Design system and component architecture documentation.\".to_string()",
        "url=\"https://ui-baseline.adobe.com\".to_string()",
        "image_src=\"https://ui-baseline.adobe.com/static/logo.png\".to_string()",
        "title=\"State + Source Markers\"",
        "id=\"docs-preview-card\".to_string()",
        "title=\"Custom title\".to_string()",
        "description=\"Custom description for source markers.\".to_string()",
        "url=\"https://github.com/adobe/ui-baseline\".to_string()",
        "site_label=\"github.com\".to_string()",
        "open_delay_ms=260",
        "close_delay_ms=240",
        "class_name=\"docs-preview-card-state\".to_string()",
        "motion=PreviewCardMotion {",
        "initial_scale: 0.95",
        "offset_y_px: 12.0",
        "\"Inspect markers\"",
        "title=\"Default Fallbacks\"",
        "\"Uses defaults\"",
        "Falls back to default title/description/url/site-label when not provided.",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for preview-card contracts.",
        );
    }
}
