use super::{IconsGlyph, IconsTone};
use crate::{IconsUi, IconsUiSize, IconsWorkflow, IconsWorkflowSize};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconsSet {
    #[default]
    Ui,
    Workflow,
}

impl IconsSet {
    pub fn as_attr(self) -> &'static str {
        match self {
            IconsSet::Ui => "ui",
            IconsSet::Workflow => "workflow",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconsScale {
    #[default]
    Medium,
    Large,
}

impl IconsScale {
    pub fn as_attr(self) -> &'static str {
        match self {
            IconsScale::Medium => "medium",
            IconsScale::Large => "large",
        }
    }

    pub fn as_ui_size(self) -> IconsUiSize {
        match self {
            IconsScale::Medium => IconsUiSize::Md,
            IconsScale::Large => IconsUiSize::Lg,
        }
    }

    pub fn as_workflow_size(self) -> IconsWorkflowSize {
        match self {
            IconsScale::Medium => IconsWorkflowSize::Md,
            IconsScale::Large => IconsWorkflowSize::Lg,
        }
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn parse_set_from_name(name: &str) -> Option<IconsSet> {
    let (prefix, _) = name.trim().split_once(':')?;

    match prefix {
        "workflow" => Some(IconsSet::Workflow),
        "ui" => Some(IconsSet::Ui),
        _ => None,
    }
}

fn normalize_name(name: String, set: IconsSet) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return format!("{}:help", set.as_attr());
    }

    if trimmed.contains(':') {
        return trimmed.to_string();
    }

    format!("{}:{trimmed}", set.as_attr())
}

#[component]
pub fn Icons(
    #[prop(into)] name: String,
    #[prop(optional)] set: IconsSet,
    #[prop(optional)] scale: IconsScale,
    #[prop(optional)] tone: IconsTone,
    #[prop(optional)] disabled: bool,
    #[prop(default = true)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] glyphs: Vec<IconsGlyph>,
) -> impl IntoView {
    let resolved_set = parse_set_from_name(&name).unwrap_or(set);
    let normalized_name = normalize_name(name, resolved_set);
    let aria_label = normalize_optional_text(aria_label).unwrap_or_default();
    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-icons {class_name}"))
        .unwrap_or_else(|| "ui-icons".to_string());

    let content = match resolved_set {
        IconsSet::Ui => view! {
            <IconsUi
                icon=normalized_name
                size=scale.as_ui_size()
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=aria_label
                class_name=class_name
                glyphs=glyphs
            />
        }
        .into_any(),
        IconsSet::Workflow => view! {
            <IconsWorkflow
                icon=normalized_name
                size=scale.as_workflow_size()
                tone=tone
                disabled=disabled
                decorative=decorative
                aria_label=aria_label
                class_name=class_name
                glyphs=glyphs
            />
        }
        .into_any(),
    };

    view! {
        <span
            data-slot="icons"
            data-set=resolved_set.as_attr()
            data-scale=scale.as_attr()
            data-disabled=disabled.then_some("true")
        >
            {content}
        </span>
    }
}
