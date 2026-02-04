use crate::button::{ButtonSize, ButtonVariant};
use crate::button_flip::{FlipButton, FlipDirection};
use crate::button_share::{
    ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform, logic,
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
    let items = logic::resolve_items(&items);
    let label = label
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Share".to_string());

    let icon_button_size = logic::resolve_icon_button_size(size);

    let items = StoredValue::new(items);
    let label = StoredValue::new(label);
    let on_icon_press = StoredValue::new(on_icon_press);

    let base_class = "ui-share-button".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <div class=class data-slot="share-button">
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
                        <div class="ui-share-button__front">
                            <Button variant=variant size=size>
                                {match icon {
                                    ShareButtonIconPlacement::Prefix => render_icon(),
                                    _ => ().into_any(),
                                }}
                                <span>{move || label.get_value()}</span>
                                {match icon {
                                    ShareButtonIconPlacement::Suffix => render_icon(),
                                    _ => ().into_any(),
                                }}
                            </Button>
                        </div>
                    }
                }
                back=move || {
                    let items = items.get_value();
                    view! {
                        <div class="ui-share-button__back">
                            <ButtonGroup attached=true>
                                {items
                                    .into_iter()
                                    .map(|item| {
                                        let platform = item.platform;
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
                                            }.into_any(),
                                            SharePlatform::X => view! {
                                                <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                    <path d="M18.1 3H21l-6.8 7.8L22 21h-6.3l-4.9-6.2L5.6 21H2.7l7.3-8.4L2 3h6.4l4.4 5.7L18.1 3zm-1.1 16h1.6L7.1 4.8H5.4L17 19z" />
                                                </svg>
                                            }.into_any(),
                                            SharePlatform::Facebook => view! {
                                                <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                                                    <path d="M13.5 8.5V7c0-.7.4-1.1 1.2-1.1h1.2V3h-2.1C11.9 3 11 4 11 5.6v2.9H9v2.8h2V21h2.5v-9.7h2l.3-2.8h-2.3z" />
                                                </svg>
                                            }.into_any(),
                                        };

                                        view! {
                                            <Button
                                                variant=ButtonVariant::Ghost
                                                size=icon_button_size
                                                aria_label=aria_label
                                                on_press=on_press
                                            >
                                                {icon}
                                            </Button>
                                        }
                                    })
                                    .collect_view()}
                            </ButtonGroup>
                        </div>
                    }
                }
            />
        </div>
    }
}
