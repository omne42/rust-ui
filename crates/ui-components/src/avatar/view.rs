use crate::avatar::logic::{self, AvatarSize};
use leptos::prelude::*;

#[component]
pub fn Avatar(
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] src: Option<String>,
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let name = name.and_then(|name| {
        let name = name.trim().to_string();
        (!name.is_empty()).then_some(name)
    });
    let src = src.and_then(|src| {
        let src = src.trim().to_string();
        (!src.is_empty()).then_some(src)
    });
    let alt = alt.and_then(|alt| {
        let alt = alt.trim().to_string();
        (!alt.is_empty()).then_some(alt)
    });

    let aria_label = alt
        .clone()
        .or_else(|| name.clone())
        .unwrap_or_else(|| "Avatar".to_string());
    let img_alt = alt.clone().or_else(|| name.clone()).unwrap_or_default();
    let title = name.clone();
    let initials = name
        .as_deref()
        .and_then(logic::initials_from_name)
        .unwrap_or_else(|| "?".to_string());

    let base_class = format!("ui-avatar {}", size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let img_error = RwSignal::new(false);

    let src_for_show = src.clone();
    let show_image = Signal::derive(move || src_for_show.is_some() && !img_error.get());

    view! {
        <span
            class=class
            class:ui-avatar--image=move || show_image.get()
            class:ui-avatar--fallback=move || !show_image.get()
            data-slot="avatar"
            title=title
            role=move || (!show_image.get()).then_some("img".to_string())
            aria-label=move || (!show_image.get()).then_some(aria_label.clone())
        >
            <Show
                when=move || show_image.get()
                fallback=move || view! {
                    <span
                        class="ui-avatar__initials"
                        data-slot="avatar-initials"
                        aria-hidden="true"
                    >
                        {initials.clone()}
                    </span>
                }
            >
                <img
                    class="ui-avatar__img"
                    data-slot="avatar-img"
                    src=src.clone().unwrap_or_default()
                    alt=img_alt.clone()
                    on:error=move |_| img_error.set(true)
                />
            </Show>
        </span>
    }
}
