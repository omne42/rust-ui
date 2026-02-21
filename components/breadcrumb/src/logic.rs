use std::borrow::Cow;

use crate::protocol::{
    BREADCRUMB_AGENT_SCHEMA_NAME, BreadcrumbAgentAction, BreadcrumbAgentContract,
    BreadcrumbAgentIntent, BreadcrumbAgentOutputStatus, BreadcrumbAgentRenderMode,
    BreadcrumbAgentSchemaVersion, BreadcrumbAgentSource, BreadcrumbAgentState,
    BreadcrumbAgentStreamFallback, BreadcrumbAgentStreamSupport,
};
use ui_state_primitives::breadcrumb as breadcrumb_primitives;
use ui_state_primitives::breadcrumbs as breadcrumbs_primitives;

pub use ui_state_primitives::breadcrumbs::{
    BreadcrumbsItemInput, BreadcrumbsState, BreadcrumbsStateInput,
};

pub const DEFAULT_SEPARATOR: &str = "/";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbRootState {
    pub aria_label: String,
    pub aria_source_attr: &'static str,
    pub class_name: Cow<'static, str>,
    pub class_source_attr: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbSeparatorState {
    pub separator: Cow<'static, str>,
    pub separator_source_attr: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BreadcrumbSourceAttr {
    Custom,
    I18n,
    Default,
    Unknown,
}

fn normalize_optional_str(value: Option<&str>) -> Option<&str> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then_some(trimmed)
    })
}

fn parse_source_attr(value: &str) -> BreadcrumbSourceAttr {
    match value {
        "custom" => BreadcrumbSourceAttr::Custom,
        "i18n" => BreadcrumbSourceAttr::I18n,
        "default" => BreadcrumbSourceAttr::Default,
        _ => BreadcrumbSourceAttr::Unknown,
    }
}

pub fn resolve_root_state(
    aria_label: Option<String>,
    aria_label_fallback: Option<&str>,
    class_name: Option<String>,
) -> BreadcrumbRootState {
    let normalized_aria_label = breadcrumb_primitives::normalize_optional_text(aria_label);
    let normalized_aria_label_fallback = normalize_optional_str(aria_label_fallback);
    let has_custom_aria_label = normalized_aria_label.is_some();
    let has_i18n_aria_label = normalized_aria_label_fallback.is_some();
    let merged_aria_label =
        normalized_aria_label.or_else(|| normalized_aria_label_fallback.map(str::to_string));
    let (aria_label, _) = breadcrumb_primitives::normalize_aria_label(merged_aria_label);
    let normalized_class_name = breadcrumb_primitives::normalize_optional_text(class_name);
    let has_custom_class_name = normalized_class_name.is_some();
    let primitive_state = breadcrumb_primitives::resolve_root_state(
        breadcrumb_primitives::BreadcrumbRootStateInput {
            has_custom_aria_label,
            has_custom_class_name,
        },
    );
    let class_name = if let Some(class_name) = normalized_class_name {
        Cow::Owned(format!("ui-breadcrumb {class_name}"))
    } else {
        Cow::Borrowed("ui-breadcrumb")
    };

    BreadcrumbRootState {
        aria_label,
        aria_source_attr: if has_custom_aria_label {
            "custom"
        } else if has_i18n_aria_label {
            "i18n"
        } else {
            "default"
        },
        class_name,
        class_source_attr: primitive_state.class_source_attr,
    }
}

pub fn resolve_separator(
    separator: Option<String>,
    separator_fallback: &str,
) -> BreadcrumbSeparatorState {
    if let Some(separator) = breadcrumb_primitives::normalize_optional_text(separator) {
        return BreadcrumbSeparatorState {
            separator: Cow::Owned(separator),
            separator_source_attr: "custom",
        };
    }

    if let Some(separator) = normalize_optional_str(Some(separator_fallback)) {
        return BreadcrumbSeparatorState {
            separator: Cow::Owned(separator.into()),
            separator_source_attr: "i18n",
        };
    }

    BreadcrumbSeparatorState {
        separator: Cow::Borrowed(DEFAULT_SEPARATOR),
        separator_source_attr: "default",
    }
}

pub fn resolve_state(items: &[BreadcrumbItem]) -> BreadcrumbsState {
    let item_inputs: Vec<_> = items
        .iter()
        .map(|item| BreadcrumbsItemInput {
            href: item.href.as_deref(),
        })
        .collect();

    breadcrumbs_primitives::resolve_state(BreadcrumbsStateInput {
        items: &item_inputs,
    })
}

pub fn is_current_page(item_index: usize, item_count: usize) -> bool {
    breadcrumbs_primitives::is_last_item(item_index, item_count)
}

pub fn resolve_item_href(
    item: &BreadcrumbItem,
    item_index: usize,
    item_count: usize,
) -> Option<String> {
    breadcrumbs_primitives::resolve_item_href(
        BreadcrumbsItemInput {
            href: item.href.as_deref(),
        },
        item_index,
        item_count,
    )
}

pub fn resolve_agent_state(state: &BreadcrumbsState) -> BreadcrumbAgentState {
    match (
        state.is_empty,
        state.has_links,
        state.has_current_page,
        state.has_items,
    ) {
        (true, _, _, _) => BreadcrumbAgentState::Empty,
        (false, true, true, _) => BreadcrumbAgentState::LinkedTrail,
        (false, true, false, _) => BreadcrumbAgentState::LinksOnly,
        (false, false, true, _) => BreadcrumbAgentState::CurrentPageOnly,
        (false, false, false, true) => BreadcrumbAgentState::ItemOnly,
        (false, false, false, false) => BreadcrumbAgentState::Empty,
    }
}

pub fn resolve_agent_source(
    aria_source_attr: &str,
    class_source_attr: &str,
    separator_source_attr: &str,
) -> BreadcrumbAgentSource {
    let sources = [
        parse_source_attr(aria_source_attr),
        parse_source_attr(class_source_attr),
        parse_source_attr(separator_source_attr),
    ];
    let has_custom = sources.contains(&BreadcrumbSourceAttr::Custom);
    let has_i18n = sources.contains(&BreadcrumbSourceAttr::I18n);
    let has_default = sources.contains(&BreadcrumbSourceAttr::Default);
    let has_unknown = sources.contains(&BreadcrumbSourceAttr::Unknown);

    if has_unknown {
        return BreadcrumbAgentSource::Mixed;
    }
    if has_custom && (has_i18n || has_default) {
        return BreadcrumbAgentSource::Mixed;
    }
    if has_custom {
        return BreadcrumbAgentSource::Customized;
    }
    if has_i18n {
        return BreadcrumbAgentSource::I18nFallback;
    }
    BreadcrumbAgentSource::DefaultOnly
}

pub fn resolve_agent_contract(
    state: &BreadcrumbsState,
    aria_source_attr: &str,
    class_source_attr: &str,
    separator_source_attr: &str,
) -> BreadcrumbAgentContract {
    BreadcrumbAgentContract {
        schema_name: BREADCRUMB_AGENT_SCHEMA_NAME,
        schema_version: BreadcrumbAgentSchemaVersion::V1,
        intent: BreadcrumbAgentIntent::TrailNavigation,
        action: BreadcrumbAgentAction::Navigate,
        state: resolve_agent_state(state),
        source: resolve_agent_source(aria_source_attr, class_source_attr, separator_source_attr),
        render_mode: BreadcrumbAgentRenderMode::Snapshot,
        stream_support: BreadcrumbAgentStreamSupport::Optional,
        stream_fallback: BreadcrumbAgentStreamFallback::Snapshot,
        output_status: BreadcrumbAgentOutputStatus::Verified,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
