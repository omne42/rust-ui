use crate::button::{ButtonSize, ButtonVariant};
use crate::button_flip::{FlipButton, FlipDirection};
use crate::button_share::{
    ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,
    logic::{self, ShareButtonStateInput},
};
use crate::{Button, ButtonGroup};
use leptos::prelude::*;

#[component]
pub fn ShareButton(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] icon: ShareButtonIconPlacement,
    #[prop(optional)] from: FlipDirection,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] items: Vec<ShareButtonItem>,
    #[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,
    #[prop(optional)] motion: ShareButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let label = logic::normalize_optional_text(label);
    let class_name = logic::normalize_optional_text(class_name);

    let provided_item_count = items.len();
    let resolved_items = logic::resolve_items(&items);
    let icon_button_size = logic::resolve_icon_button_size(size);

    let state = logic::resolve_state(ShareButtonStateInput {
        provided_item_count,
        resolved_item_count: resolved_items.items.len(),
        uses_default_items: resolved_items.uses_default_items,
        icon_placement: icon,
        has_custom_label: label.is_some(),
        has_custom_class_name: class_name.is_some(),
        has_custom_press_handler: on_icon_press.is_some(),
    });

    let label = label.unwrap_or_else(|| "Share".to_string());
    let class = logic::compose_class_name(class_name, state);

    let items = StoredValue::new(resolved_items.items);
    let label = StoredValue::new(label);
    let on_icon_press = StoredValue::new(on_icon_press);

    view! {
        <div
            class=class
            data-slot="share-button"
            data-state=state.state_attr
            data-provided-count=state.provided_item_count.to_string()
            data-count=state.resolved_item_count.to_string()
            data-has-items=state.has_items.then_some("true")
            data-items-source=state.items_source_attr
            data-icon=state.icon_placement_attr
            data-label-source=state.label_source_attr
            data-handler-source=state.handler_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=if motion == ShareButtonMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != ShareButtonMotion::default()).then_some("true")
        >
            <FlipButton
                from=from
                motion=motion.flip
                front=move || {
                    let render_icon = || {
                        view! {
                            <svg class="ui-share-button__icon" viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M12.8 4.5l2.7 2.7-2.7 2.7"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                                <path
                                    d="M5 10a4 4 0 0 1 4-4h6.5"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                                <path
                                    d="M5 10a4 4 0 0 0 4 4h6.5"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        }
                        .into_any()
                    };

                    view! {
                        <div class="ui-share-button__front" data-slot="share-button-front">
                            <Button variant=variant size=size>
                                {match icon {
                                    ShareButtonIconPlacement::Prefix => {
                                        view! { <span data-slot="share-button-trigger-icon">{render_icon()}</span> }
                                            .into_any()
                                    }
                                    _ => ().into_any(),
                                }}
                                <span data-slot="share-button-label">{move || label.get_value()}</span>
                                {match icon {
                                    ShareButtonIconPlacement::Suffix => {
                                        view! { <span data-slot="share-button-trigger-icon">{render_icon()}</span> }
                                            .into_any()
                                    }
                                    _ => ().into_any(),
                                }}
                            </Button>
                        </div>
                    }
                }
                back=move || {
                    let items = items.get_value();
                    view! {
                        <div class="ui-share-button__back" data-slot="share-button-back">
                            <ButtonGroup attached=true>
                                <div class="ui-share-button__platforms" data-slot="share-button-platforms">
                                    {items
                                        .into_iter()
                                        .map(|item| {
                                            let platform = item.platform;
                                            let platform_attr = platform.as_attr();
                                            let aria_label = item.label;
                                            let on_icon_press = on_icon_press.get_value();
                                            let on_press = Callback::new(move |_| {
                                                if let Some(cb) = on_icon_press {
                                                    cb.run(platform);
                                                }
                                            });

                                            let icon = match platform {
                                                SharePlatform::Github => view! {
                                                    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                        <path d="M12 2a10 10 0 0 0-3.16 19.48c.5.1.68-.2.68-.46v-1.6c-2.78.6-3.37-1.2-3.37-1.2-.45-1.2-1.1-1.52-1.1-1.52-.9-.62.07-.6.07-.6 1 .08 1.53 1.05 1.53 1.05.88 1.56 2.3 1.1 2.86.83.1-.67.34-1.1.62-1.36-2.22-.26-4.56-1.13-4.56-5.06 0-1.12.39-2.04 1.03-2.76-.1-.26-.45-1.3.1-2.7 0 0 .84-.28 2.75 1.02a9.3 9.3 0 0 1 5 0c1.9-1.3 2.74-1.02 2.74-1.02.56 1.4.2 2.44.1 2.7.64.72 1.03 1.64 1.03 2.76 0 3.94-2.35 4.8-4.58 5.05.35.32.67.94.67 1.9v2.83c0 .26.18.56.69.46A10 10 0 0 0 12 2z" />
                                                    </svg>
                                                }
                                                .into_any(),
                                                SharePlatform::X => view! {
                                                    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                        <path d="M18.1 3H21l-6.8 7.8L22 21h-6.3l-4.9-6.2L5.6 21H2.7l7.3-8.4L2 3h6.4l4.4 5.7L18.1 3zm-1.1 16h1.6L7.1 4.8H5.4L17 19z" />
                                                    </svg>
                                                }
                                                .into_any(),
                                                SharePlatform::Facebook => view! {
                                                    <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                        <path d="M13.5 8.5V7c0-.7.4-1.1 1.2-1.1h1.2V3h-2.1C11.9 3 11 4 11 5.6v2.9H9v2.8h2V21h2.5v-9.7h2l.3-2.8h-2.3z" />
                                                    </svg>
                                                }
                                                .into_any(),
                                            };

                                            view! {
                                                <span
                                                    class="ui-share-button__platform"
                                                    data-slot="share-button-platform"
                                                    data-platform=platform_attr
                                                >
                                                    <Button
                                                        variant=ButtonVariant::Ghost
                                                        size=icon_button_size
                                                        aria_label=aria_label
                                                        on_press=on_press
                                                    >
                                                        {icon}
                                                    </Button>
                                                </span>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </ButtonGroup>
                        </div>
                    }
                }
            />
        </div>
    }
}
