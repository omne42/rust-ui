use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_empty_state_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/empty-state").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_empty_state_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-empty_state\")]")
            && lib_source.contains("pub use ui_empty_state as empty_state;"),
        "ui-components should re-export the external ui-empty-state crate as `empty_state`.",
    );
    assert!(
        cargo_source.contains("component-empty_state = [\"dep:ui-empty-state\"]"),
        "component-empty_state feature should depend on dep:ui-empty-state after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-empty-state = { path = \"../../components/empty-state\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-empty-state dependency.",
    );
}

#[test]
fn empty_state_does_not_expose_logic_or_view_modules() {
    let source = load_empty_state_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "EmptyState internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn empty_state_uses_logic_state_model() {
    let logic_source = load_empty_state_component_source("src/logic.rs");
    let view_source = load_empty_state_component_source("src/view.rs");
    let primitive_source =
        load_ui_components_source("../../crates/ui-state-primitives/src/empty_state.rs");

    for needle in [
        "pub use ui_state_primitives::empty_state::{",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_DESCRIPTION",
        "DEFAULT_TITLE",
        "EmptyStateAlign",
        "EmptyStateState",
        "EmptyStateStateInput",
        "EmptyStateTone",
        "compose_class_name",
        "normalize_aria_label",
        "normalize_description",
        "normalize_optional_text",
        "normalize_title",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "EmptyState logic should re-export primitive contract `{needle}`."
        );
    }

    for needle in [
        "pub enum EmptyStateTone",
        "pub enum EmptyStateAlign",
        "pub struct EmptyStateStateInput",
        "pub struct EmptyStateState",
        "pub const DEFAULT_TITLE: &str = \"Nothing to show\";",
        "pub const DEFAULT_DESCRIPTION: &str = \"Try adjusting filters or refreshing data.\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Empty state\";",
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
            primitive_source.contains(needle),
            "EmptyState primitives should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<EmptyStateStrings>()",
        "logic::normalize_title(title, strings.default_title.as_ref())",
        "logic::normalize_description(description, strings.default_description.as_ref())",
        "logic::normalize_aria_label(aria_label, strings.default_aria_label.as_ref())",
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "logic::resolve_state(EmptyStateStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "let motion = motion::sanitize_motion(motion);",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "EmptyState view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn empty_state_emits_baseline_style_state_data_attributes() {
    let source = load_empty_state_component_source("src/view.rs");

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
        "data-motion-source=motion_source",
        "data-custom-motion=(motion_source == \"custom\").then_some(\"true\")",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(attr),
            "EmptyState should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn empty_state_styles_include_tone_align_and_markers() {
    let source = load_empty_state_component_source("src/styles.rs");

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
        ".ui-empty-state[data-motion-source=\"custom\"]",
        ".ui-empty-state[data-custom-motion=\"true\"]",
        "--ui-empty-state-enter",
        "prefers-reduced-motion: reduce",
    ] {
        assert!(
            source.contains(selector),
            "EmptyState styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn empty_state_docs_page_covers_primary_playgrounds() {
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );

    for needle in [
        "pub(super) fn empty_state() -> AnyView",
        "title=\"EmptyState\"",
        "slug=\"empty-state\"",
        "description=\"baseline-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers.\"",
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
    let source = load_ui_components_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    );

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
