use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_clears_on_escape_when_clearable_and_not_empty() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("key != \"Escape\""),
        "Input should treat Escape as a clear shortcut when `is_clearable` is enabled."
    );
    assert!(
        source.contains("!is_clearable"),
        "Input Escape handling should gate on `is_clearable` to avoid surprising clears."
    );
    assert!(
        source.contains("is_empty.get_untracked()"),
        "Input should only clear on Escape when a value is present (otherwise let Escape propagate)."
    );
    assert!(
        source.contains("set_value.set(String::new())"),
        "Input should clear its value on Escape."
    );
}

#[test]
fn input_escape_clear_stops_propagation() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "Input should stop Escape propagation when clearing (Spectrum parity: Escape clears without dismissing parent overlays)."
    );
}

#[test]
fn input_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "Input clear button should be excluded from tab order to avoid extra Tab stops."
    );
}

#[test]
fn input_clear_button_is_presence_safe() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("data-visible"),
        "Input should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || view_state.get().show_clear"),
        "Input should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn input_attaches_clear_motion_driver() {
    let source = load_source("src/input/view.rs");

    assert!(
        source.contains("attach_clear_button_motion"),
        "Input should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn input_styles_include_motion_marker_contracts() {
    let source = load_source("src/input/styles.rs");

    for selector in [
        ".ui-input[data-motion-source=\"custom\"]",
        ".ui-input[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Input styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn input_styles_define_clear_motion_css_vars() {
    let source = load_source("src/input/styles.rs");

    assert!(
        source.contains("--ui-input-clear-opacity"),
        "Input styles should define `--ui-input-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-input-clear-scale"),
        "Input styles should define `--ui-input-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn input_motion_uses_spring_animator() {
    let source = load_source("src/input/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Input motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn input_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/input/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-read-only",
        "data-required",
        "data-motion-source",
        "data-custom-motion",
    ] {
        assert!(
            source.contains(attr),
            "Input should set `{attr}` to support Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn input_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/input/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: InputMotion) -> InputMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "hidden_scale:",
        "hover_scale:",
        "tap_scale:",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values()",
    ] {
        assert!(
            source.contains(needle),
            "Input motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn input_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn input() -> AnyView",
        "title=\"Input\"",
        "slug=\"input\"",
        "description=\"Spectrum-style text input with label, description/error, and clear button.\"",
        "<Playground title=\"Clearable + validation\" code_signal=code>",
        "<Input",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for input primary playground coverage.",
        );
    }
}

#[test]
fn input_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Clearable + validation\"",
        "id=\"docs-input\".to_string()",
        "label=\"Name\".to_string()",
        "is_clearable=true",
        "invalid=Signal::derive(move || invalid.get())",
        "description=\"Try toggling invalid.\".to_string()",
        "error=\"This field is invalid.\".to_string()",
        "size=InputSize::Md",
        "variant=InputVariant::Bordered",
        "on_press=Callback::new(move |_| set_invalid.update(|v| *v = !*v))",
    ] {
        assert!(
            source.contains(needle),
            "input docs playgrounds should contain `{needle}`.",
        );
    }
}
