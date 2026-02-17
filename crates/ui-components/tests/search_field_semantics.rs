use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn search_field_clears_on_escape_when_not_empty() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("key != \"Escape\""),
        "SearchField should treat Escape as a clear shortcut (baseline parity)."
    );
    assert!(
        source.contains("value.get_untracked().is_empty()"),
        "SearchField should only clear on Escape when a value is present (otherwise let Escape propagate)."
    );
    assert!(
        source.contains("set_value.set(String::new())"),
        "SearchField should clear its value on Escape."
    );
}

#[test]
fn search_field_escape_clear_stops_propagation() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("stop_propagation()"),
        "SearchField should stop Escape propagation when clearing (baseline parity: Escape clears without dismissing parent overlays)."
    );
}

#[test]
fn search_field_clear_button_is_excluded_from_tab_order() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("tabindex=\"-1\""),
        "SearchField clear button should be excluded from tab order like UI Baseline."
    );
}

#[test]
fn search_field_clear_button_is_presence_safe() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("data-visible"),
        "SearchField should keep the clear button in the DOM and toggle visibility via data attributes."
    );
    assert!(
        !source.contains("Show when=move || state.show_clear_button.get()"),
        "SearchField should not unmount the clear button abruptly; use CSS/data attributes to allow motion."
    );
}

#[test]
fn search_field_attaches_clear_motion_driver() {
    let source = load_source("src/search_field/view.rs");

    assert!(
        source.contains("attach_clear_motion"),
        "SearchField should attach a motion driver for clear button micro-interactions."
    );
}

#[test]
fn search_field_styles_define_clear_motion_css_vars() {
    let source = load_source("src/search_field/styles.rs");

    assert!(
        source.contains("--ui-search-field-clear-opacity"),
        "SearchField styles should define `--ui-search-field-clear-opacity` for motion-driven reveal."
    );
    assert!(
        source.contains("--ui-search-field-clear-scale"),
        "SearchField styles should define `--ui-search-field-clear-scale` for motion-driven micro-interactions."
    );
}

#[test]
fn search_field_motion_uses_spring_animator() {
    let source = load_source("src/search_field/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SearchField motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn search_field_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/search_field/view.rs");

    for attr in [
        "data-focused",
        "data-focus-visible",
        "data-invalid",
        "data-disabled",
        "data-read-only",
        "data-required",
    ] {
        assert!(
            source.contains(attr),
            "SearchField should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn search_field_styles_respect_prefers_reduced_motion() {
    let source = load_source("src/search_field/styles.rs");

    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "SearchField styles should respect prefers-reduced-motion to avoid forced transitions."
    );
    assert!(
        source.contains("transition: none;"),
        "SearchField styles should disable transitions under prefers-reduced-motion."
    );
}

#[test]
fn search_field_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/search_field/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SearchFieldMotion) -> SearchFieldMotion",
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
            "SearchField motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn search_field_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn search_field() -> AnyView",
        "title=\"SearchField\"",
        "slug=\"search-field\"",
        "<Playground title=\"Search\" code_signal=code>",
        "<SearchField",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for search-field primary playground coverage.",
        );
    }
}

#[test]
fn search_field_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"Search\"",
        "id=\"docs-search-field\".to_string()",
        "label=\"Search\".to_string()",
        "placeholder=\"Search…\".to_string()",
        "value=value",
        "set_value=set_value",
    ] {
        assert!(
            source.contains(needle),
            "forms docs playground should contain `{needle}` for search-field contracts.",
        );
    }
}
