use super::{IconsWorkflowSize, IconsWorkflowTone, IconsetGlyph};
use crate::Iconset;
use leptos::prelude::*;

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn normalize_icon_reference(icon: String) -> String {
    let icon = icon.trim();
    if icon.is_empty() {
        return "workflow:help".to_string();
    }

    if icon.contains(':') {
        return icon.to_string();
    }

    format!("workflow:{icon}")
}

fn default_workflow_glyphs() -> Vec<IconsetGlyph> {
    vec![
        IconsetGlyph::new("workflow:help", "?").with_aria_label("Workflow Help"),
        IconsetGlyph::new("workflow:success", "✓").with_aria_label("Workflow Success"),
        IconsetGlyph::new("workflow:warning", "⚠").with_aria_label("Workflow Warning"),
        IconsetGlyph::new("workflow:info", "ℹ").with_aria_label("Workflow Info"),
        IconsetGlyph::new("workflow:settings", "⚙").with_aria_label("Workflow Settings"),
    ]
}

#[component]
pub fn IconsWorkflow(
    #[prop(into)] icon: String,
    #[prop(optional)] size: IconsWorkflowSize,
    #[prop(optional)] tone: IconsWorkflowTone,
    #[prop(optional)] disabled: bool,
    #[prop(default = true)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] glyphs: Vec<IconsetGlyph>,
) -> impl IntoView {
    let icon = normalize_icon_reference(icon);
    let icon_reference = icon.clone();

    let mut registry = default_workflow_glyphs();
    registry.extend(glyphs);

    let aria_label = normalize_optional_text(aria_label).unwrap_or_default();

    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-icons-workflow {class_name}"))
        .unwrap_or_else(|| "ui-icons-workflow".to_string());

    view! {
        <span
            data-slot="icons-workflow"
            data-icon-reference=icon_reference
            data-disabled=disabled.then_some("true")
        >
            <Iconset
                icon=icon
                iconset="workflow".to_string()
                glyphs=registry
                size=size
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=aria_label
                class_name=class_name
            />
        </span>
    }
}
