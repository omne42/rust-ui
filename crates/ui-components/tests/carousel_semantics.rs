use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn carousel_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/carousel/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Carousel internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn carousel_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/carousel/mod.rs");

    for needle in [
        "pub struct CarouselItem",
        "pub struct CarouselItemResolved",
        "pub enum CarouselOrientation",
        "pub enum CarouselSlot",
        "pub struct CarouselPartStateInput",
        "pub struct CarouselPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_ORIENTATION",
        "DEFAULT_LOOP_NAVIGATION",
        "pub use crate::active_highlight::ActiveHighlightMotion as CarouselMotion;",
    ] {
        assert!(
            source.contains(needle),
            "carousel::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn carousel_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/carousel/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Carousel;"),
        "carousel module should export `Carousel`."
    );
    assert!(
        crate_source.contains(
            "pub use carousel::{Carousel, CarouselItem, CarouselMotion, CarouselOrientation};"
        ),
        "crate root should re-export carousel contracts."
    );
}

#[test]
fn carousel_logic_exposes_state_helpers() {
    let source = load_source("src/carousel/logic.rs");

    for needle in [
        "pub fn state_attr(item_count: usize, has_selection: bool, has_focus: bool)",
        "pub fn item_attr(item_count: usize)",
        "pub fn selected_attr(has_selection: bool)",
        "pub fn focus_attr(has_focus: bool)",
        "pub fn navigation_attr(loop_navigation: bool)",
        "pub fn selection_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_aria_label(value: Option<String>)",
        "pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>)",
        "pub fn adjacent_enabled_index(",
        "pub fn resolve_state(input: CarouselPartStateInput) -> CarouselPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: CarouselPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Carousel logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn carousel_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_items(&id_base.get_value(), items)",
        "logic::resolve_aria_label(aria_label)",
        "logic::resolve_state(CarouselPartStateInput {",
        "slot: CarouselSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-orientation=move || root_state.get().orientation_attr",
        "data-navigation-mode=move || root_state.get().navigation_attr",
        "data-selection-mode=move || root_state.get().selection_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-orientation-source=move || root_state.get().orientation_source_attr",
        "data-loop-navigation-source=move || root_state.get().loop_navigation_source_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-default-selected-index-source=move || root_state.get().default_selected_index_source_attr",
        "data-selected-index-change-source=move || root_state.get().selected_index_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=move || root_state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-orientation=move || root_state.get().has_custom_orientation.then_some(\"true\")",
        "data-custom-loop-navigation=move || {",
        "data-custom-selected-index=move || root_state.get().has_custom_selected_index.then_some(\"true\")",
        "data-custom-default-selected-index=move || {",
        "data-custom-selected-index-change=move || {",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Carousel view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn carousel_supports_controlled_and_uncontrolled_selection_state() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "let has_custom_selected_index = selected_index.is_some()",
        "let has_custom_default_selected_index = default_selected_index.is_some()",
        "overlay_open::use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should support `{needle}` for controlled/uncontrolled selection behavior."
        );
    }
}

#[test]
fn carousel_exposes_keyboard_and_control_contracts() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "on:keydown=on_key_down",
        "let prev_key = orientation.prev_key().to_string();",
        "let next_key = orientation.next_key().to_string();",
        "step_selection.run(-1);",
        "step_selection.run(1);",
        "select_edge.run(false);",
        "select_edge.run(true);",
        "on:click=on_prev",
        "on:click=on_next",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should wire `{needle}` to match control and keyboard semantics."
        );
    }
}

#[test]
fn carousel_uses_active_highlight_motion_for_indicators() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "use crate::active_highlight::{",
        "attach_active_highlight_motion",
        "ActiveHighlightMotion",
        "let indicator_list_ref: NodeRef<html::Div> = NodeRef::new();",
        "let indicator_highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "let (active_index, set_active_index) = signal(",
        "attach_active_highlight_motion(",
        "data-slot=highlight_slot.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should compose indicator motion via `{needle}` for HeroUI-level feedback continuity."
        );
    }
}

#[test]
fn carousel_styles_include_state_and_source_markers() {
    let source = load_source("src/carousel/styles.rs");

    for needle in [
        ".ui-carousel {",
        ".ui-carousel--selected",
        ".ui-carousel[data-state=\"selected\"]",
        ".ui-carousel--loop",
        ".ui-carousel[data-navigation-mode=\"loop\"]",
        ".ui-carousel[data-selection-mode=\"controlled\"]",
        ".ui-carousel[data-id-source=\"custom\"]",
        ".ui-carousel[data-custom-id=\"true\"]",
        ".ui-carousel--custom-id",
        ".ui-carousel[data-aria-label-source=\"custom\"]",
        ".ui-carousel[data-custom-aria-label=\"true\"]",
        ".ui-carousel--custom-aria-label",
        ".ui-carousel[data-class-source=\"custom\"]",
        ".ui-carousel[data-custom-class=\"true\"]",
        ".ui-carousel--custom-class",
        ".ui-carousel[data-orientation-source=\"custom\"]",
        ".ui-carousel[data-custom-orientation=\"true\"]",
        ".ui-carousel--custom-orientation",
        ".ui-carousel[data-loop-navigation-source=\"custom\"]",
        ".ui-carousel[data-custom-loop-navigation=\"true\"]",
        ".ui-carousel--custom-loop-navigation",
        ".ui-carousel[data-selected-index-source=\"custom\"]",
        ".ui-carousel[data-custom-selected-index=\"true\"]",
        ".ui-carousel--custom-selected-index",
        ".ui-carousel[data-default-selected-index-source=\"custom\"]",
        ".ui-carousel[data-custom-default-selected-index=\"true\"]",
        ".ui-carousel--custom-default-selected-index",
        ".ui-carousel[data-selected-index-change-source=\"custom\"]",
        ".ui-carousel[data-custom-selected-index-change=\"true\"]",
        ".ui-carousel--custom-selected-index-change",
        ".ui-carousel[data-motion-source=\"custom\"]",
        ".ui-carousel--custom-motion",
        ".ui-carousel[data-custom-motion=\"true\"]",
        ".ui-carousel__slide[data-state=\"selected\"]",
    ] {
        assert!(
            source.contains(needle),
            "Carousel styles should include `{needle}` as stable state/source contracts."
        );
    }
}

#[test]
fn carousel_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn carousel() -> AnyView",
        "title=\"Carousel\"",
        "slug=\"carousel\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-orientation-source",
        "data-loop-navigation-source",
        "data-selected-index-source",
        "data-selected-index-change-source",
        "data-motion-source",
        "<Carousel",
    ] {
        assert!(
            source.contains(needle),
            "Carousel docs page should contain `{needle}`."
        );
    }
}

#[test]
fn carousel_docs_controlled_state_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Controlled + Vertical + No Loop\"",
        "id_base=\"docs-carousel-vertical\".to_string()",
        "selected_index=controlled_selected",
        "on_selected_index_change=on_controlled_selected_change",
        "orientation=CarouselOrientation::Vertical",
        "loop_navigation=false",
        "aria_label=\"Feature carousel\".to_string()",
        "class_name=\"docs-carousel-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "Carousel docs controlled-state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn carousel_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-carousel-markers\".to_string()",
        "selected_index=marker_selected",
        "default_selected_index=0",
        "on_selected_index_change=on_marker_selected_change",
        "orientation=CarouselOrientation::Vertical",
        "loop_navigation=false",
        "aria_label=\"Workspace spotlight\".to_string()",
        "class_name=\"docs-carousel-custom\".to_string()",
        "let mut marker_motion = ui_components::CarouselMotion::default();",
        "marker_motion.spring.stiffness = 250.0",
        "marker_motion.spring.damping = 22.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-aria-label-source / data-orientation-source / data-loop-navigation-source / data-selected-index-source / data-selected-index-change-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "Carousel docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn carousel_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn carousel() -> AnyView",
        "title=\"Carousel\"",
        "slug=\"carousel\"",
        "title=\"Default + Indicator Motion\"",
        "title=\"Controlled + Vertical + No Loop\"",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            source.contains(needle),
            "carousel docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn carousel_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "<Playground title=\"Default + Indicator Motion\" code_signal=code>",
        "id_base=\"docs-carousel-default\".to_string()",
        "default_selected_index=1",
        "<Playground title=\"Controlled + Vertical + No Loop\" code_signal=states_code>",
        "id_base=\"docs-carousel-vertical\".to_string()",
        "orientation=CarouselOrientation::Vertical",
        "loop_navigation=false",
        "aria_label=\"Feature carousel\".to_string()",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "id_base=\"docs-carousel-markers\".to_string()",
        "default_selected_index=0",
        "aria_label=\"Workspace spotlight\".to_string()",
        "class_name=\"docs-carousel-custom\".to_string()",
        "marker_motion.spring.stiffness = 250.0",
        "marker_motion.spring.damping = 22.0",
    ] {
        assert!(
            source.contains(needle),
            "carousel docs playground should contain `{needle}`.",
        );
    }
}
