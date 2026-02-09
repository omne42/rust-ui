use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
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
        module_source.contains("pub use logic::{CarouselItem, CarouselOrientation};"),
        "carousel module should export `CarouselItem` and `CarouselOrientation`."
    );
    assert!(
        module_source.contains("CarouselMotion"),
        "carousel module should expose a motion alias."
    );
    assert!(
        crate_source.contains(
            "pub use carousel::{Carousel, CarouselItem, CarouselMotion, CarouselOrientation};"
        ),
        "crate root should re-export carousel contracts."
    );
}

#[test]
fn carousel_uses_logic_state_model() {
    let view_source = load_source("src/carousel/view.rs");
    let logic_source = load_source("src/carousel/logic.rs");

    for needle in [
        "pub enum CarouselOrientation",
        "pub struct CarouselItem",
        "pub struct CarouselItemResolved",
        "pub struct CarouselStateInput",
        "pub struct CarouselState",
        "pub fn resolve_items(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn adjacent_enabled_index(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Carousel logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let items = logic::resolve_items(&id_base, items);",
        "let selected_state = overlay_open::use_controllable_state(",
        "let state = Signal::derive(move ||",
        "logic::resolve_state(logic::CarouselStateInput {",
        "let class = Signal::derive(move || logic::compose_class_name(class_name.clone(), state.get()));",
    ] {
        assert!(
            view_source.contains(needle),
            "Carousel view should derive wrapper state through logic helpers; missing `{needle}`."
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
        "orientation: CarouselOrientation",
        "loop_navigation: bool",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should accept `{needle}` for controlled/uncontrolled selection behavior."
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
fn carousel_emits_spectrum_root_state_data_attributes() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "data-slot=\"carousel\"",
        "data-state=move || state.get().data_state_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-item-count=move || state.get().item_count.to_string()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-focused-index=move || state.get().focused_index.map(|index| index.to_string())",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-has-focus=move || state.get().has_focus.then_some(\"true\")",
        "data-has-disabled-items=move || state.get().has_disabled_items.then_some(\"true\")",
        "data-orientation=orientation.attr()",
        "data-loop=loop_navigation.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should set `{needle}` so it can be styled/tested with Spectrum-compatible selectors."
        );
    }
}

#[test]
fn carousel_uses_active_highlight_motion_for_indicators() {
    let source = load_source("src/carousel/view.rs");

    for needle in [
        "use crate::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion};",
        "let indicator_list_ref: NodeRef<leptos::html::Div> = NodeRef::new();",
        "let indicator_highlight_ref: NodeRef<leptos::html::Div> = NodeRef::new();",
        "let (active_index, set_active_index) = signal(selected_index.get_untracked().unwrap_or(0));",
        "attach_active_highlight_motion(",
        "data-slot=\"carousel-indicator-highlight\"",
    ] {
        assert!(
            source.contains(needle),
            "Carousel should compose indicator motion via `{needle}` for HeroUI-level feedback continuity."
        );
    }
}

#[test]
fn carousel_styles_include_orientation_selected_and_empty_markers() {
    let source = load_source("src/carousel/styles.rs");

    for needle in [
        ".ui-carousel {",
        ".ui-carousel__slide[data-selected=\"true\"]",
        ".ui-carousel__indicator[data-selected=\"true\"]",
        ".ui-carousel--vertical",
        ".ui-carousel--empty",
    ] {
        assert!(
            source.contains(needle),
            "Carousel styles should include `{needle}` for stable visual state contracts."
        );
    }
}
