use crate::logic::{self, ImageRadius, ImageShadow, ImageStatus};
use crate::{ImageMotion, motion};
use leptos::{ev, html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Image(
    #[prop(optional, into)] src: Option<String>,
    alt: String,
    #[prop(optional, into)] fallback_src: Option<String>,
    #[prop(optional)] disable_skeleton: bool,
    #[prop(optional)] is_blurred: bool,
    #[prop(optional)] is_zoomed: bool,
    #[prop(optional)] radius: ImageRadius,
    #[prop(optional)] shadow: ImageShadow,
    #[prop(optional)] motion: ImageMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Img>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let motion_source = if motion == ImageMotion::default() {
        "default"
    } else {
        "custom"
    };
    let src = logic::normalize_optional_text(src);
    let fallback_src = logic::normalize_optional_text(fallback_src);
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let alt = StoredValue::new(alt);

    let (status, set_status) = signal(if src.is_some() {
        ImageStatus::Loading
    } else {
        ImageStatus::Idle
    });

    let src = StoredValue::new(src);
    let fallback_src = StoredValue::new(fallback_src);

    let view_state = Memo::new(move |_| {
        logic::resolve_view_state(
            src.get_value().as_deref(),
            fallback_src.get_value().as_deref(),
            status.get(),
            disable_skeleton,
            is_blurred,
        )
    });

    let base_class = format!("ui-image {} {}", radius.class_name(), shadow.class_name());
    let class = logic::normalize_optional_text(class_name)
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let wrapper_ref: NodeRef<html::Div> = NodeRef::new();
    let motion_state = motion::use_image_motion(false);
    motion::attach_zoom_motion(
        wrapper_ref,
        is_zoomed,
        motion_state.hover.is_hovered,
        motion,
    );

    let on_load = move |_ev: ev::Event| set_status.set(ImageStatus::Loaded);
    let on_error = move |_ev: ev::ErrorEvent| set_status.set(ImageStatus::Error);

    view! {
        <div
            class=class
            node_ref=wrapper_ref
            data-slot="image-wrapper"
            data-state=move || view_state.get().status_attr
            data-loaded=move || view_state.get().is_loaded.then_some("true")
            data-zoomed=is_zoomed.then_some("true")
            data-fallback=move || view_state.get().show_fallback.then_some("true")
            data-skeleton=move || view_state.get().show_skeleton.then_some("true")
            data-blurred=move || view_state.get().show_blurred.then_some("true")
            data-radius=radius.as_attr()
            data-shadow=shadow.as_attr()
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
            on:pointerenter=move |_| motion_state.hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| motion_state.hover.handlers.on_pointer_leave.run(())
        >
            <Show when=move || view_state.get().show_blurred>
                <img
                    class="ui-image__blurred"
                    data-slot="image-blurred"
                    aria-hidden="true"
                    src=move || src.get_value().unwrap_or_default()
                    alt=""
                />
            </Show>

            <Show when=move || view_state.get().show_fallback>
                <img
                    class="ui-image__fallback"
                    data-slot="image-fallback"
                    src=move || fallback_src.get_value().unwrap_or_default()
                    alt=move || alt.get_value()
                />
            </Show>

            <Show when=move || view_state.get().show_image>
                <img
                    class="ui-image__img"
                    data-slot="image"
                    node_ref=node_ref
                    src=move || src.get_value().unwrap_or_default()
                    alt=move || alt.get_value()
                    on:load=on_load
                    on:error=on_error
                />
            </Show>

            <Show when=move || view_state.get().show_skeleton>
                <div
                    class="ui-image__skeleton"
                    data-slot="image-skeleton"
                    aria-hidden="true"
                ></div>
            </Show>
        </div>
    }
}
