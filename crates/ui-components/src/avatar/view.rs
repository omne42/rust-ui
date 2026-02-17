use crate::avatar::logic::{self, AvatarSize};
use leptos::prelude::*;
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::{A11yDirection, image_fallback_attrs, locale_attrs};

#[component]
pub fn Avatar(
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] src: Option<String>,
    #[prop(optional)] size: AvatarSize,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let normalized = logic::normalize_input(name, src, alt, class_name);
    let locale = locale_attrs(logic::normalize_lang(lang), dir);

    let accessibility =
        logic::resolve_accessibility(normalized.name.as_deref(), normalized.alt.as_deref());
    let aria_label_value = if accessibility.label_source == logic::AvatarLabelSource::Fallback {
        common.avatar_fallback_aria_label.as_ref().to_string()
    } else {
        accessibility.aria_label
    };
    let aria_label = StoredValue::new(aria_label_value);
    let img_alt = StoredValue::new(accessibility.img_alt);
    let title = StoredValue::new(accessibility.title);

    let state = logic::resolve_state(logic::AvatarStateInput {
        size,
        has_name: normalized.has_name,
        has_src: normalized.has_src,
        has_alt: normalized.has_alt,
        has_custom_class_name: normalized.has_custom_class_name,
    });

    let class = logic::compose_class_name(normalized.class_name, state);
    let initials = StoredValue::new(logic::resolve_initials(normalized.name.as_deref()));

    let image_src = StoredValue::new(normalized.image_src);
    let img_error = RwSignal::new(false);
    let render_state = Signal::derive(move || {
        logic::resolve_image_render_state(logic::AvatarImageRenderInput {
            has_src: state.has_src,
            has_img_error: img_error.get(),
        })
    });

    view! {
        <span
            class=class
            class:ui-avatar--image=move || render_state.get().mode.shows_image()
            class:ui-avatar--fallback=move || !render_state.get().mode.shows_image()
            data-slot="avatar"
            data-size=state.size_attr
            data-state=move || render_state.get().mode.as_str()
            data-image=move || render_state.get().mode.image_attr()
            data-fallback=move || render_state.get().mode.fallback_attr()
            data-has-name=state.has_name.then_some("true")
            data-has-src=state.has_src.then_some("true")
            data-has-alt=state.has_alt.then_some("true")
            data-label-source=state.label_source.as_str()
            data-custom-class=state.has_custom_class_name.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
            title=move || title.get_value()
            role=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).role
            aria-label=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).aria_label
        >
            <Show
                when=move || render_state.get().mode.shows_image()
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
