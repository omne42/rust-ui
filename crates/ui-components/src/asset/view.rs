use super::{AssetMotion, AssetSize, AssetVariant};
use crate::Thumbnail;
use leptos::prelude::*;

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn resolve_label(label: Option<String>, variant: AssetVariant) -> String {
    if let Some(label) = normalize_optional_text(label) {
        return label;
    }

    match variant {
        AssetVariant::File => "File".to_string(),
        AssetVariant::Folder => "Folder".to_string(),
        AssetVariant::Custom => "Asset".to_string(),
    }
}

#[component]
pub fn Asset(
    #[prop(optional)] variant: AssetVariant,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] size: AssetSize,
    #[prop(optional)] selected: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] motion: AssetMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let label = resolve_label(label, variant);
    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-asset {class_name}"))
        .unwrap_or_else(|| "ui-asset".to_string());

    let content: AnyView = match variant {
        AssetVariant::File => view! {
            <svg
                viewBox="0 0 24 24"
                role="img"
                aria-label=label
                data-slot="asset-file"
                data-variant="file"
            >
                <path
                    d="M6 2h8l4 4v16H6z"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linejoin="round"
                ></path>
                <path
                    d="M14 2v4h4"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linejoin="round"
                ></path>
            </svg>
        }
        .into_any(),
        AssetVariant::Folder => view! {
            <svg
                viewBox="0 0 24 24"
                role="img"
                aria-label=label
                data-slot="asset-folder"
                data-variant="folder"
            >
                <path
                    d="M3 6.5h6l2 2h10v9A2.5 2.5 0 0 1 18.5 20h-13A2.5 2.5 0 0 1 3 17.5z"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linejoin="round"
                ></path>
                <path
                    d="M3 9h18"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linejoin="round"
                ></path>
            </svg>
        }
        .into_any(),
        AssetVariant::Custom => {
            if let Some(children) = children {
                view! { {children()} }.into_any()
            } else {
                view! {
                    <svg
                        viewBox="0 0 24 24"
                        role="img"
                        aria-label=label
                        data-slot="asset-custom"
                        data-variant="custom"
                    >
                        <rect
                            x="4"
                            y="4"
                            width="16"
                            height="16"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linejoin="round"
                        ></rect>
                        <path
                            d="M8 14l2.8-2.8a1 1 0 0 1 1.4 0L14 13l2-2a1 1 0 0 1 1.4 0L20 13.6"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linejoin="round"
                        ></path>
                        <circle cx="9" cy="9" r="1.2" fill="currentColor"></circle>
                    </svg>
                }
                .into_any()
            }
        }
    };

    view! {
        <Thumbnail
            size=size
            selected=selected
            focused=focused
            motion=motion
            class_name=class_name
        >
            <div
                data-slot="asset"
                data-variant=variant.as_attr()
                data-selected=selected.then_some("true")
                data-focused=focused.then_some("true")
            >
                {content}
            </div>
        </Thumbnail>
    }
}
