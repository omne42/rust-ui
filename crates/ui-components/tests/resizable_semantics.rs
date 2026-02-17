use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn resizable_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/resizable/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Resizable internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn resizable_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/resizable/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Resizable;"),
        "resizable module should export `Resizable`."
    );
    assert!(
        module_source.contains("pub use logic::ResizableOrientation;"),
        "resizable module should export `ResizableOrientation`."
    );
    assert!(
        crate_source.contains("pub use resizable::{Resizable, ResizableOrientation};"),
        "crate root should re-export Resizable contracts."
    );
}

#[test]
fn resizable_uses_logic_state_model() {
    let logic_source = load_source("src/resizable/logic.rs");
    let view_source = load_source("src/resizable/view.rs");

    for needle in [
        "pub enum ResizableOrientation",
        "pub struct SplitBounds",
        "pub struct ResizableStateInput",
        "pub struct ResizableState",
        "pub fn normalize_bounds(",
        "pub fn normalize_split(",
        "pub fn split_from_drag(",
        "pub fn split_step_for_key(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Resizable logic should include `{needle}`."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "logic::normalize_bounds(min_split_percent, max_split_percent)",
        "logic::normalize_split(default_split_percent, bounds)",
        "logic::resolve_state(ResizableStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::split_from_drag(",
        "logic::split_step_for_key(&key, orientation, event.shift_key())",
    ] {
        assert!(
            view_source.contains(needle),
            "Resizable view should derive behavior via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn resizable_supports_controlled_and_uncontrolled_split_state() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "split_percent: Option<Signal<f64>>",
        "default_split_percent: Option<f64>",
        "on_split_percent_change: Option<Callback<f64>>",
        "overlay_open::use_controllable_state(",
        "is_controlled = split_percent.is_some()",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should support `{needle}` for controllable split state."
        );
    }
}

#[test]
fn resizable_wires_pointer_drag_and_keyboard_contracts() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "on:pointermove=on_pointer_move",
        "on:pointerup=on_pointer_up",
        "on:pointerleave=on_pointer_up",
        "on:pointerdown=on_handle_pointer_down",
        "on:keydown=on_handle_key_down",
        "role=\"separator\"",
        "aria-valuemin=move || format!(\"{:.2}\", state.get().min_split_percent)",
        "aria-valuemax=move || format!(\"{:.2}\", state.get().max_split_percent)",
        "aria-valuenow=move || format!(\"{:.2}\", state.get().split_percent)",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should wire `{needle}` for drag + keyboard + separator semantics."
        );
    }
}

#[test]
fn resizable_emits_baseline_root_state_data_attributes() {
    let source = load_source("src/resizable/view.rs");

    for needle in [
        "data-slot=\"resizable\"",
        "data-orientation=move || state.get().orientation_attr",
        "data-state=move || state.get().state_attr",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-enabled=move || state.get().enabled.then_some(\"true\")",
        "data-dragging=move || state.get().dragging.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(\"true\")",
        "data-handle=move || state.get().handle_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Resizable should expose `{needle}` for stable styling/test contracts."
        );
    }
}

#[test]
fn resizable_styles_include_orientation_and_handle_markers() {
    let source = load_source("src/resizable/styles.rs");

    for needle in [
        ".ui-resizable {",
        ".ui-resizable[data-orientation=\"vertical\"]",
        ".ui-resizable__panel--first",
        ".ui-resizable__handle",
        ".ui-resizable__handle::after",
        ".ui-resizable[data-state=\"dragging\"] .ui-resizable__handle",
        ".ui-resizable--disabled",
        ".ui-resizable--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "Resizable styles should include `{needle}` marker contracts."
        );
    }
}

#[test]
fn resizable_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn resizable() -> AnyView",
        "title=\"Resizable\"",
        "slug=\"resizable\"",
        "<Resizable",
    ] {
        assert!(
            docs.contains(needle),
            "Resizable docs page should contain `{needle}`."
        );
    }
}

#[test]
fn resizable_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn resizable() -> AnyView",
        "title=\"Resizable\"",
        "slug=\"resizable\"",
        "description=\"baseline-compatible panel splitter with controlled/uncontrolled split state, pointer + keyboard resize semantics, and baseline-style state data contracts.\"",
        "<Playground title=\"Horizontal + Handle Grip\" code_signal=horizontal_code>",
        "<Playground title=\"Controlled + Vertical Bounds\" code_signal=vertical_code>",
        "<Resizable",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs should include `{needle}` for resizable primary playground coverage.",
        );
    }
}

#[test]
fn resizable_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Horizontal + Handle Grip\"",
        "orientation=ResizableOrientation::Horizontal",
        "default_split_percent=36.0",
        "with_handle=true",
        "\"Sidebar\"",
        "\"Content\"",
        "title=\"Controlled + Vertical Bounds\"",
        "orientation=ResizableOrientation::Vertical",
        "split_percent=split",
        "on_split_percent_change=on_split_change",
        "min_split_percent=25.0",
        "max_split_percent=80.0",
        "with_handle=true",
        "aria_label=\"Deployment regions split\".to_string()",
        "class_name=\"docs-resizable-custom\".to_string()",
        "\"Header\"",
        "\"Body\"",
        "controlled split:",
        "format!(\"{:.1}%\", split_raw.get())",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for resizable contracts.",
        );
    }
}
