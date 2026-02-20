use super::component_catalog;
use crate::perf_probe::{UiPerfBudget, UiPerfProbe};
use leptos::prelude::*;
use ui_components::Snippet;
use ui_layout::{
    Card, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexWrap, Heading, HeadingLevel,
};

const ACCORDION_README_MD: &str = include_str!("../../../../../components/accordion/src/README.md");
const CHECKBOX_README_MD: &str = include_str!("../../../../../components/checkbox/src/README.md");
const CHECKBOX_GROUP_README_MD: &str =
    include_str!("../../../../../components/checkbox-group/src/README.md");
const CHECKBOX_FIELD_README_MD: &str =
    include_str!("../../../../../components/checkbox-field/src/README.md");
const DATE_PICKER_README_MD: &str =
    include_str!("../../../../../components/text-input/src/date_picker/README.md");
const DROPDOWN_MENU_README_MD: &str =
    include_str!("../../../../../components/menu/src/dropdown_menu/README.md");
const MODAL_README_MD: &str = include_str!("../../../../../components/modal/src/README.md");

fn component_readme_markdown(slug: &str) -> Option<&'static str> {
    match slug {
        "accordion" => Some(ACCORDION_README_MD),
        "checkbox" => Some(CHECKBOX_README_MD),
        "checkbox-group" => Some(CHECKBOX_GROUP_README_MD),
        "checkbox-field" => Some(CHECKBOX_FIELD_README_MD),
        "date-picker" => Some(DATE_PICKER_README_MD),
        "dropdown-menu" => Some(DROPDOWN_MENU_README_MD),
        "modal" => Some(MODAL_README_MD),
        _ => None,
    }
}

fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {
    match slug {
        "button" => UiPerfBudget {
            max_mount_ms: 24.0,
            max_update_ms: Some(8.0),
            max_heap_kb: Some(384.0),
        },
        "input" => UiPerfBudget {
            max_mount_ms: 28.0,
            max_update_ms: Some(10.0),
            max_heap_kb: Some(512.0),
        },
        "accordion" => UiPerfBudget {
            max_mount_ms: 36.0,
            max_update_ms: Some(12.0),
            max_heap_kb: Some(768.0),
        },
        "button-copy" => UiPerfBudget {
            max_mount_ms: 26.0,
            max_update_ms: Some(9.0),
            max_heap_kb: Some(448.0),
        },
        "action-button" => UiPerfBudget {
            max_mount_ms: 28.0,
            max_update_ms: Some(9.0),
            max_heap_kb: Some(448.0),
        },
        "action-button-group" => UiPerfBudget {
            max_mount_ms: 34.0,
            max_update_ms: Some(12.0),
            max_heap_kb: Some(640.0),
        },
        "action-group" => UiPerfBudget {
            max_mount_ms: 38.0,
            max_update_ms: Some(14.0),
            max_heap_kb: Some(768.0),
        },
        "action-bar" => UiPerfBudget {
            max_mount_ms: 34.0,
            max_update_ms: Some(12.0),
            max_heap_kb: Some(640.0),
        },
        "flip-button" => UiPerfBudget {
            max_mount_ms: 30.0,
            max_update_ms: Some(10.0),
            max_heap_kb: Some(512.0),
        },
        "share-button" => UiPerfBudget {
            max_mount_ms: 32.0,
            max_update_ms: Some(11.0),
            max_heap_kb: Some(576.0),
        },
        "swatch" => UiPerfBudget {
            max_mount_ms: 22.0,
            max_update_ms: Some(6.0),
            max_heap_kb: Some(320.0),
        },
        "tag" => UiPerfBudget {
            max_mount_ms: 24.0,
            max_update_ms: Some(8.0),
            max_heap_kb: Some(384.0),
        },
        "tag-group" => UiPerfBudget {
            max_mount_ms: 34.0,
            max_update_ms: Some(12.0),
            max_heap_kb: Some(640.0),
        },
        "tree" => UiPerfBudget {
            max_mount_ms: 42.0,
            max_update_ms: Some(14.0),
            max_heap_kb: Some(896.0),
        },
        "time-field" => UiPerfBudget {
            max_mount_ms: 32.0,
            max_update_ms: Some(11.0),
            max_heap_kb: Some(576.0),
        },
        "slider" => UiPerfBudget {
            max_mount_ms: 30.0,
            max_update_ms: Some(10.0),
            max_heap_kb: Some(512.0),
        },
        "resizable" => UiPerfBudget {
            max_mount_ms: 34.0,
            max_update_ms: Some(12.0),
            max_heap_kb: Some(640.0),
        },
        _ => UiPerfBudget::mount_only(120.0),
    }
}

#[component]
pub fn ComponentPage(
    title: &'static str,
    slug: &'static str,
    group: &'static str,
    #[prop(optional)] description: &'static str,
    children: Children,
) -> impl IntoView {
    let description = (!description.trim().is_empty()).then_some(description);

    let (prev, next) = {
        let catalog = component_catalog();
        let current = catalog.iter().position(|doc| doc.slug == slug);
        current
            .map(|idx| {
                let prev = idx
                    .checked_sub(1)
                    .and_then(|prev_idx| catalog.get(prev_idx).copied());
                let next = catalog.get(idx.saturating_add(1)).copied();
                (prev, next)
            })
            .unwrap_or((None, None))
    };

    let import_text = format!("use ui_components::{title};");
    let perf_budget = component_page_perf_budget(slug);
    let perf_name = format!("ComponentPage::{slug}");
    let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);

    view! {
        <Card class_name="docs-prose docs-page-header".to_string()>
            <Flex
                justify=FlexJustify::SpaceBetween
                align=FlexAlign::Start
                gap=FlexGap::Md
                class_name="docs-page-header__top".to_string()
            >
                <Flex direction=FlexDirection::Column gap=FlexGap::Xs>
                    <Heading level=HeadingLevel::H2 class_name="docs-page-title".to_string()>
                        {title}
                    </Heading>
                    {description.map(|description| view! {
                        <p class="docs-page-description">{description}</p>
                    })}
                </Flex>
                <Flex
                    wrap=FlexWrap::Wrap
                    justify=FlexJustify::End
                    align=FlexAlign::Center
                    gap=FlexGap::Sm
                    class_name="docs-page-header__actions".to_string()
                >
                    {prev.map(|doc| view! {
                        <a class="docs-page-nav" href=format!("#/components/{}", doc.slug)>
                            "← " {doc.name}
                        </a>
                    })}
                    {next.map(|doc| view! {
                        <a class="docs-page-nav" href=format!("#/components/{}", doc.slug)>
                            {doc.name} " →"
                        </a>
                    })}
                    <a class="docs-page-back" href="#/components">
                        "All components"
                    </a>
                </Flex>
            </Flex>
            <Flex
                justify=FlexJustify::SpaceBetween
                align=FlexAlign::Center
                gap=FlexGap::Sm
                class_name="docs-page-meta".to_string()
            >
                <Flex align=FlexAlign::Center gap=FlexGap::Sm class_name="docs-page-meta__left".to_string()>
                    <span class="docs-page-group">{group}</span>
                    <code class="docs-page-slug">{slug}</code>
                </Flex>
                <Snippet
                    text=import_text
                    label="Import".to_string()
                    copyable=true
                    class_name="docs-page-import".to_string()
                />
            </Flex>
        </Card>

        {readme_html.map(|html| view! {
            <Card class_name="docs-prose".to_string()>
                <div data-slot="component-readme" inner_html=html></div>
            </Card>
        })}

        <UiPerfProbe name=perf_name budget=perf_budget>
            <div class="docs-component-root" data-slot=slug data-component=slug>
                {children()}
            </div>
        </UiPerfProbe>
    }
}
