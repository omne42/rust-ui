use crate::logic::{self, ImageRadius, ImageShadow};
use crate::{ImageMotion, motion, protocol};
use leptos::{ev, html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

const BLURRED_CLASS: &str = "ui-image__blurred";
const FALLBACK_CLASS: &str = "ui-image__fallback";
const IMAGE_CLASS: &str = "ui-image__img";
const SKELETON_CLASS: &str = "ui-image__skeleton";
const DECORATIVE_ALT_TEXT: &str = "";

fn render_blurred_layer(src_attr: StoredValue<String>) -> impl IntoView {
    view! {
        <img
            class=BLURRED_CLASS
            data-slot="image-blurred"
            aria-hidden="true"
            src=move || src_attr.get_value()
            alt=DECORATIVE_ALT_TEXT
        />
    }
}

fn render_fallback_layer(
    fallback_src_attr: StoredValue<String>,
    alt: StoredValue<String>,
) -> impl IntoView {
    view! {
        <img
            class=FALLBACK_CLASS
            data-slot="image-fallback"
            src=move || fallback_src_attr.get_value()
            alt=move || alt.get_value()
        />
    }
}

fn render_image_layer(
    src_attr: StoredValue<String>,
    alt: StoredValue<String>,
    on_load: impl Fn(ev::Event) + 'static,
    on_error: impl Fn(ev::ErrorEvent) + 'static,
) -> impl IntoView {
    view! {
        <img
            class=IMAGE_CLASS
            data-slot="image"
            src=move || src_attr.get_value()
            alt=move || alt.get_value()
            on:load=on_load
            on:error=on_error
        />
    }
}

fn render_skeleton_layer() -> impl IntoView {
    view! {
        <div class=SKELETON_CLASS data-slot="image-skeleton" aria-hidden="true"></div>
    }
}

#[component]
pub fn Image(
    #[prop(optional, into)] src: Option<String>,
    alt: String,
    #[prop(optional, into)] fallback_src: Option<String>,
    #[prop(optional)] is_skeleton_disabled: bool,
    #[prop(optional)] is_blurred: bool,
    #[prop(optional)] is_zoomed: bool,
    #[prop(optional)] radius: ImageRadius,
    #[prop(optional)] shadow: ImageShadow,
    #[prop(optional)] motion: ImageMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let motion_source = logic::resolve_motion_source(motion);
    let normalized = logic::normalize_props(logic::ImageNormalizeInput {
        src,
        fallback_src,
        class_name,
        lang,
        radius,
        shadow,
    });
    let locale = locale_attrs(normalized.lang.clone(), dir);
    let alt = StoredValue::new(alt);

    let (status, set_status) = signal(logic::derive_initial_status(normalized.src.as_deref()));
    let (status_source, set_status_source) = signal(logic::ImageStatusSource::Initial);

    let src = StoredValue::new(normalized.src);
    let src_attr = StoredValue::new(normalized.src_attr);
    let fallback_src = StoredValue::new(normalized.fallback_src);
    let fallback_src_attr = StoredValue::new(normalized.fallback_src_attr);

    let view_state = Memo::new(move |_| {
        logic::derive_view_state(logic::ImageViewStateInput {
            src: src.get_value(),
            fallback_src: fallback_src.get_value(),
            status: status.get(),
            is_skeleton_disabled,
            is_blurred,
        })
    });

    let class = normalized.class_name;
    let external_prop_source = protocol::ImageAgentPropSource::ExternalProp.as_attr();

    let wrapper_ref: NodeRef<html::Div> = NodeRef::new();
    let motion_state = motion::use_image_motion(false);
    motion::attach_zoom_motion(
        wrapper_ref,
        is_zoomed,
        motion_state.hover.is_hovered,
        motion,
    );

    let on_load = move |_ev: ev::Event| {
        set_status_source.set(logic::ImageStatusSource::Event);
        set_status.update(|value| {
            *value = logic::apply_status_event(*value, logic::ImageStatusEvent::LoadSucceeded)
        })
    };
    let on_error = move |_ev: ev::ErrorEvent| {
        set_status_source.set(logic::ImageStatusSource::Event);
        set_status.update(|value| {
            *value = logic::apply_status_event(*value, logic::ImageStatusEvent::LoadFailed)
        })
    };

    view! {
        <div
            class=class
            node_ref=wrapper_ref
            data-ui-schema=protocol::IMAGE_AGENT_SCHEMA
            data-ui-intent=protocol::ImageAgentIntent::Display.as_attr()
            data-ui-action=move || protocol::action_from_status_source(status_source.get()).as_attr()
            data-ui-state=move || view_state.get().status_attr
            data-ui-status-source=move || status_source.get().as_attr()
            data-ui-motion-source=motion_source.as_attr()
            data-ui-content-source=move || protocol::content_source_from_view_state(view_state.get()).as_attr()
            data-ui-radius-source=external_prop_source
            data-ui-shadow-source=external_prop_source
            data-ui-stream-support=protocol::ImageStreamSupport::Optional.as_attr()
            data-ui-stream-fallback=protocol::ImageStreamFallback::Snapshot.as_attr()
            data-ui-llm-mode=protocol::ImageLlmRenderMode::Snapshot.as_attr()
            data-ui-output-status=protocol::ImageOutputStatus::Verified.as_attr()
            data-slot="image-wrapper"
            data-state=move || view_state.get().status_attr
            data-loaded=move || view_state.get().is_loaded.then_some("true")
            data-zoomed=is_zoomed.then_some("true")
            data-fallback=move || view_state.get().show_fallback.then_some("true")
            data-skeleton=move || view_state.get().show_skeleton.then_some("true")
            data-blurred=move || view_state.get().show_blurred.then_some("true")
            data-radius=radius.as_attr()
            data-shadow=shadow.as_attr()
            data-status-source=move || status_source.get().as_attr()
            data-motion-source=motion_source.as_attr()
            data-custom-motion=motion_source.is_custom().then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
            on:pointerenter=move |_| motion_state.hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| motion_state.hover.handlers.on_pointer_leave.run(())
        >
            <Show when=move || view_state.get().show_blurred>
                {render_blurred_layer(src_attr)}
            </Show>

            <Show when=move || view_state.get().show_fallback>
                {render_fallback_layer(fallback_src_attr, alt)}
            </Show>

            <Show when=move || view_state.get().show_image>
                {render_image_layer(src_attr, alt, on_load, on_error)}
            </Show>

            <Show when=move || view_state.get().show_skeleton>
                {render_skeleton_layer()}
            </Show>
        </div>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
