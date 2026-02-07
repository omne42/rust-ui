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
    let name = logic::normalize_optional_text(name);
    let src = logic::normalize_optional_text(src);
    let alt = logic::normalize_optional_text(alt);
    let class_name = logic::normalize_optional_text(class_name);

    let accessibility = logic::resolve_accessibility(name.as_deref(), alt.as_deref());
    let aria_label = StoredValue::new(accessibility.aria_label);
    let img_alt = StoredValue::new(accessibility.img_alt);
    let title = StoredValue::new(accessibility.title);

    let state = logic::resolve_state(logic::AvatarStateInput {
        size,
        has_name: name.is_some(),
        has_src: src.is_some(),
        has_alt: alt.is_some(),
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let initials = StoredValue::new(logic::resolve_initials(name.as_deref()));

    let image_src = StoredValue::new(src.unwrap_or_default());
    let has_src = state.has_src;

    let img_error = RwSignal::new(false);
    let show_image = Signal::derive(move || has_src && !img_error.get());

    view! {
        <span
            class=class
            class:ui-avatar--image=move || show_image.get()
            class:ui-avatar--fallback=move || !show_image.get()
            data-slot="avatar"
            data-size=state.size_attr
            data-state=move || if show_image.get() { "image" } else { "fallback" }
            data-image=move || show_image.get().then_some("true")
            data-fallback=move || (!show_image.get()).then_some("true")
            data-has-name=state.has_name.then_some("true")
            data-has-src=state.has_src.then_some("true")
            data-has-alt=state.has_alt.then_some("true")
            data-label-source=state.label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            title=move || title.get_value()
            role=move || (!show_image.get()).then_some("img".to_string())
            aria-label=move || (!show_image.get()).then_some(aria_label.get_value())
        >
            <Show
                when=move || show_image.get()
                fallback=move || {
                    view! {
                        <span
                            class="ui-avatar__initials"
                            data-slot="avatar-initials"
                            aria-hidden="true"
                        >
                            {initials.get_value()}
                        </span>
                    }
                }
            >
                <img
                    class="ui-avatar__img"
                    data-slot="avatar-img"
                    src=image_src.get_value()
                    alt=img_alt.get_value()
                    on:error=move |_| img_error.set(true)
                />
            </Show>
        </span>
    }
}
