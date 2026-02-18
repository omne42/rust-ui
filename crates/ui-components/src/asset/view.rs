use super::{AssetMotion, AssetSize, AssetVariant, logic, motion};
use crate::thumbnail::Thumbnail;
use leptos::prelude::*;

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
    let motion = motion::sanitize_motion(motion);
    let motion_source_attr = if motion == AssetMotion::default() {
        "default"
    } else {
        "custom"
    };

    let normalized_label = logic::normalize_optional_text(label);
    let has_custom_label = normalized_label.is_some();
    let label = logic::resolve_label(normalized_label, variant);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_content = children.is_some() && variant == AssetVariant::Custom;

    let state = logic::resolve_state(logic::AssetStateInput {
        variant,
        size,
        selected,
        focused,
        has_custom_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_content,
    });

    let class_name = logic::compose_class_name(class_name, state);

    let label = StoredValue::new(label);

    let content: AnyView = match variant {
        AssetVariant::File => view! {
            <svg
                viewBox="0 0 24 24"
                role="img"
                aria-label=label.get_value()
                class="ui-asset__icon ui-asset__icon--file"
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
                aria-label=label.get_value()
                class="ui-asset__icon ui-asset__icon--folder"
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
                        aria-label=label.get_value()
                        class="ui-asset__icon ui-asset__icon--custom-fallback"
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
                class="ui-asset__content"
                data-slot="asset"
                data-variant=state.variant_attr
                data-size=state.size_attr
                data-state=state.data_state_attr
                data-selected=state.selected.then_some("true")
                data-focused=state.focused.then_some("true")
                data-label-source=state.label_source_attr
                data-class-source=state.class_source_attr
                data-content-source=state.content_source_attr
                data-custom-class=state.has_custom_class_name.then_some("true")
                data-motion-source=motion_source_attr
                data-custom-motion=(motion != AssetMotion::default()).then_some("true")
            >
                {content}
            </div>
        </Thumbnail>
    }
}
