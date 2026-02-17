use super::super::flip::{FlipButton, FlipDirection};
use super::{
    ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,
    logic::{self, ShareButtonStateInput},
};
use crate::button::group::ButtonGroup;
use crate::button::{Button, ButtonSize, ButtonVariant};
use leptos::prelude::*;
use ui_headless::{A11yDirection, CommonStrings, labeled_group_attrs, use_ui_i18n};

fn render_trigger_icon() -> AnyView {
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
}

fn render_platform_icon(platform: SharePlatform) -> AnyView {
    let icon_path = match platform {
        SharePlatform::Github => {
            "M12 2a10 10 0 0 0-3.16 19.48c.5.1.68-.2.68-.46v-1.6c-2.78.6-3.37-1.2-3.37-1.2-.45-1.2-1.1-1.52-1.1-1.52-.9-.62.07-.6.07-.6 1 .08 1.53 1.05 1.53 1.05.88 1.56 2.3 1.1 2.86.83.1-.67.34-1.1.62-1.36-2.22-.26-4.56-1.13-4.56-5.06 0-1.12.39-2.04 1.03-2.76-.1-.26-.45-1.3.1-2.7 0 0 .84-.28 2.75 1.02a9.3 9.3 0 0 1 5 0c1.9-1.3 2.74-1.02 2.74-1.02.56 1.4.2 2.44.1 2.7.64.72 1.03 1.64 1.03 2.76 0 3.94-2.35 4.8-4.58 5.05.35.32.67.94.67 1.9v2.83c0 .26.18.56.69.46A10 10 0 0 0 12 2z"
        }
        SharePlatform::X => {
            "M18.1 3H21l-6.8 7.8L22 21h-6.3l-4.9-6.2L5.6 21H2.7l7.3-8.4L2 3h6.4l4.4 5.7L18.1 3zm-1.1 16h1.6L7.1 4.8H5.4L17 19z"
        }
        SharePlatform::Facebook => {
            "M13.5 8.5V7c0-.7.4-1.1 1.2-1.1h1.2V3h-2.1C11.9 3 11 4 11 5.6v2.9H9v2.8h2V21h2.5v-9.7h2l.3-2.8h-2.3z"
        }
    };

    view! {
        <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d=icon_path />
        </svg>
    }
    .into_any()
}

fn render_front_button(
    icon: ShareButtonIconPlacement,
    variant: ButtonVariant,
    size: ButtonSize,
    label: StoredValue<String>,
) -> AnyView {
    match icon {
        ShareButtonIconPlacement::Prefix => view! {
            <Button
                variant=variant
                size=size
                start_content=move || view! {
                    <span data-slot="share-button-trigger-icon">
                        {render_trigger_icon()}
                    </span>
                }
            >
                <span data-slot="share-button-label">{move || label.get_value()}</span>
            </Button>
        }
        .into_any(),
        ShareButtonIconPlacement::Suffix => view! {
            <Button
                variant=variant
                size=size
                end_content=move || view! {
                    <span data-slot="share-button-trigger-icon">
                        {render_trigger_icon()}
                    </span>
                }
            >
                <span data-slot="share-button-label">{move || label.get_value()}</span>
            </Button>
        }
        .into_any(),
        ShareButtonIconPlacement::None => view! {
            <Button variant=variant size=size>
                <span data-slot="share-button-label">{move || label.get_value()}</span>
            </Button>
        }
        .into_any(),
    }
}

fn render_front_panel(
    icon: ShareButtonIconPlacement,
    variant: ButtonVariant,
    size: ButtonSize,
    label: StoredValue<String>,
) -> impl IntoView {
    let front_button = render_front_button(icon, variant, size, label);
    view! {
        <div class="ui-share-button__front" data-slot="share-button-front">
            {front_button}
        </div>
    }
}

fn render_platform_button(
    item: ShareButtonItem,
    icon_button_size: ButtonSize,
    on_icon_press: StoredValue<Option<Callback<SharePlatform>>>,
) -> impl IntoView {
    let platform = item.platform;
    let platform_attr = platform.as_attr();
    let aria_label = item.label;
    let on_icon_press = on_icon_press.get_value();
    let on_press = Callback::new(move |_| {
        if let Some(cb) = on_icon_press {
            cb.run(platform);
        }
    });
    let icon = render_platform_icon(platform);

    view! {
        <span class="ui-share-button__platform" data-slot="share-button-platform" data-platform=platform_attr>
            <Button
                variant=ButtonVariant::Ghost
                size=icon_button_size
                is_icon_only=true
                aria_label=aria_label
                on_press=on_press
            >
                {icon}
            </Button>
        </span>
    }
}

fn render_back_panel(
    items: Vec<ShareButtonItem>,
    icon_button_size: ButtonSize,
    on_icon_press: StoredValue<Option<Callback<SharePlatform>>>,
) -> impl IntoView {
    view! {
        <div class="ui-share-button__back" data-slot="share-button-back">
            <ButtonGroup attached=true>
                <div class="ui-share-button__platforms" data-slot="share-button-platforms">
                    {items
                        .into_iter()
                        .map(|item| render_platform_button(item, icon_button_size, on_icon_press))
                        .collect_view()}
                </div>
            </ButtonGroup>
        </div>
    }
}

#[component]
pub fn ShareButton(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] icon: ShareButtonIconPlacement,
    #[prop(optional)] from: FlipDirection,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] items: Vec<ShareButtonItem>,
    #[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,
    #[prop(optional)] motion: ShareButtonMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let motion = super::motion::sanitize_motion(motion);
    let normalized_label = logic::normalize_optional_text(label);
    let normalized_group_aria_label = logic::normalize_optional_text(aria_label);
    let class_name = logic::normalize_optional_text(class_name);

    let provided_item_count = items.len();
    let resolved_items = logic::resolve_items_with_fallback(
        &items,
        logic::SharePlatformLabels {
            github: common.share_platform_github_label.as_ref(),
            x: common.share_platform_x_label.as_ref(),
            facebook: common.share_platform_facebook_label.as_ref(),
        },
    );
    let icon_button_size = logic::resolve_icon_button_size(size);

    let state = logic::resolve_state(ShareButtonStateInput {
        provided_item_count,
        resolved_item_count: resolved_items.items.len(),
        uses_default_items: resolved_items.uses_default_items,
        icon_placement: icon,
        has_custom_label: normalized_label.is_some(),
        has_custom_class_name: class_name.is_some(),
        has_custom_press_handler: on_icon_press.is_some(),
    });

    let label =
        logic::resolve_label_with_fallback(normalized_label, common.share_button_label.as_ref());
    let group_aria_label = logic::resolve_label_with_fallback(
        normalized_group_aria_label.clone(),
        common.share_button_group_aria_label.as_ref(),
    );
    let group_a11y = labeled_group_attrs(group_aria_label, lang, dir);
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
            data-group-label-source=if normalized_group_aria_label.is_some() {
                "custom"
            } else {
                "i18n"
            }
            role=group_a11y.role
            aria-label=group_a11y.aria_label.clone()
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
        >
            <FlipButton
                from=from
                motion=motion.flip
                front=move || {
                    render_front_panel(icon, variant, size, label)
                }
                back=move || {
                    let items = items.get_value();
                    render_back_panel(items, icon_button_size, on_icon_press)
                }
            />
        </div>
    }
}
