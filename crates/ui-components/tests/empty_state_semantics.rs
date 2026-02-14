use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn empty_state_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/empty_state/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "EmptyState internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_state_uses_logic_state_model() {
    let logic_source = load_source("src/empty_state/logic.rs");
    let view_source = load_source("src/empty_state/view.rs");

    for needle in [
        "pub enum EmptyStateTone",
        "pub enum EmptyStateAlign",
        "pub fn normalize_optional_text(",
        "pub fn normalize_title(",
        "pub fn normalize_description(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "title_source_attr",
        "description_source_attr",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "EmptyState logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<EmptyStateStrings>()",
        "logic::normalize_title(title, strings.default_title.as_ref())",
        "logic::normalize_description(description, strings.default_description.as_ref())",
        "logic::normalize_aria_label(aria_label, strings.default_aria_label.as_ref())",
        "logic::resolve_state(EmptyStateStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "EmptyState view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn empty_state_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/empty_state/view.rs");

    for attr in [
        "data-slot=\"empty-state\"",
        "data-slot=\"empty-state-icon\"",
        "data-slot=\"empty-state-title\"",
        "data-slot=\"empty-state-description\"",
        "data-slot=\"empty-state-actions\"",
        "data-tone=move || state.get().tone_attr",
        "data-align=move || state.get().align_attr",
        "data-state=move || state.get().data_state_attr",
        "data-compact=move || state.get().is_compact.then_some(\"true\")",
        "data-bordered=move || state.get().is_bordered.then_some(\"true\")",
        "data-icon=move || state.get().has_icon.then_some(\"true\")",
        "data-actions=move || state.get().has_actions.then_some(\"true\")",
        "data-title-source=move || state.get().title_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "EmptyState should expose `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn empty_state_styles_include_tone_align_and_markers() {
    let source = load_source("src/empty_state/styles.rs");

    for selector in [
        ".ui-empty-state--tone-default",
        ".ui-empty-state[data-tone=\"default\"]",
        ".ui-empty-state--tone-muted",
        ".ui-empty-state[data-tone=\"muted\"]",
        ".ui-empty-state--tone-accent",
        ".ui-empty-state[data-tone=\"accent\"]",
        ".ui-empty-state--align-start",
        ".ui-empty-state[data-align=\"start\"]",
        ".ui-empty-state--align-center",
        ".ui-empty-state[data-align=\"center\"]",
        ".ui-empty-state--compact",
        ".ui-empty-state[data-compact=\"true\"]",
        ".ui-empty-state--bordered",
        ".ui-empty-state[data-bordered=\"true\"]",
        ".ui-empty-state--custom-class",
        ".ui-empty-state[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "EmptyState styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn empty_state_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn empty_state() -> AnyView",
        "title=\"EmptyState\"",
        "slug=\"empty-state\"",
        "description=\"Spectrum/HeroUI-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers.\"",
        "<Playground title=\"Tone + Alignment + Actions\" code_signal=tone_code>",
        "<Playground title=\"Compact + Bordered + Custom Class\" code_signal=state_code>",
        "<EmptyState",
    ] {
        assert!(
            source.contains(needle),
            "display_extra docs page should include `{needle}` for empty_state primary playground coverage.",
        );
    }
}

#[test]
fn empty_state_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Tone + Alignment + Actions\"",
        "title=\"No projects yet\".to_string()",
        "description=\"Create your first project to unlock dashboards and team workflows.\".to_string()",
        "tone=EmptyStateTone::Default",
        "icon=move || view! { <span>\"📁\"</span> }",
        "tone=EmptyStateTone::Muted",
        "align=EmptyStateAlign::Center",
        "title=\"Compact + Bordered + Custom Class\"",
        "title=\"Deployments paused\".to_string()",
        "description=\"Approvals are required before resuming this environment.\".to_string()",
        "tone=EmptyStateTone::Accent",
        "compact=true",
        "bordered=true",
        "class_name=\"docs-empty-state-custom\".to_string()",
        "icon=move || view! { <span>\"⏸\"</span> }",
        "variant=ui_components::ButtonVariant::Secondary",
        "\"Review approvals\"",
    ] {
        assert!(
            source.contains(needle),
            "empty_state docs playgrounds should contain `{needle}`.",
        );
    }
}
