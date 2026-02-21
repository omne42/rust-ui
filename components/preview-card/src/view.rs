use crate::{PreviewCardMotion, PreviewCardPartStateInput, PreviewCardSlot, logic, motion};
use leptos::{children::ViewFn, ev, html, portal::Portal, prelude::*};
use ui_headless::{
    HoverCardTriggerOptions, PopoverPlacement, PopoverPositionOptions, use_hover_card_trigger,
    use_popover_position,
};

fn next_id() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|cell| {
        let id = cell.get();
        cell.set(id + 1);
        id
    })
}

#[component]
pub fn PreviewCard(
    #[prop(into)] trigger: ViewFn,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] url: Option<String>,
    #[prop(optional, into)] site_label: Option<String>,
    #[prop(optional, into)] image_src: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional, default = logic::DEFAULT_OPEN_DELAY_MS)] open_delay_ms: u64,
    #[prop(optional, default = logic::DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64,
    #[prop(optional)] motion: PreviewCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_motion = motion != PreviewCardMotion::default();
    let has_custom_delays = logic::has_custom_delays(open_delay_ms, close_delay_ms);

    let (id, has_custom_id) = logic::resolve_id(id, format!("ui-preview-card-{}", next_id()));
    let id = StoredValue::new(id);

    let (title, has_custom_title) = logic::resolve_title(title);
    let title = StoredValue::new(title);
    let (description, has_custom_description) = logic::resolve_description(description);
    let description = StoredValue::new(description);
    let (url, has_custom_url) = logic::resolve_url(url);
    let url = StoredValue::new(url);

    let (site_label, site_label_source_attr) =
        logic::resolve_site_label(site_label, &url.get_value());
    let site_label = StoredValue::new(site_label);

    let image_src = logic::resolve_image_src(image_src);
    let has_image = image_src.is_some();
    let image_src = StoredValue::new(image_src);

    let trigger_aria = use_hover_card_trigger(HoverCardTriggerOptions {
        is_disabled: disabled,
        open_delay_ms,
        close_delay_ms,
        ..Default::default()
    });
    let open_signal = trigger_aria.state.is_open;
    let presence = ui_headless::use_presence(open_signal);

    let root_state = logic::resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Root,
        disabled,
        has_image,
        has_custom_class_name: class_name.is_some(),
        has_custom_delays,
        has_custom_id,
        has_custom_title,
        has_custom_description,
        has_custom_url,
        site_label_source_attr,
        has_custom_motion,
    });
    let root_class = logic::compose_class_name(class_name, root_state);

    let trigger_state = logic::resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Trigger,
        disabled,
        has_image,
        has_custom_class_name: false,
        has_custom_delays,
        has_custom_id,
        has_custom_title,
        has_custom_description,
        has_custom_url,
        site_label_source_attr,
        has_custom_motion,
    });
    let trigger_class = logic::compose_class_name(None, trigger_state);

    let panel_state = logic::resolve_part_state(PreviewCardPartStateInput {
        slot: PreviewCardSlot::Panel,
        disabled,
        has_image,
        has_custom_class_name: false,
        has_custom_delays,
        has_custom_id,
        has_custom_title,
        has_custom_description,
        has_custom_url,
        site_label_source_attr,
        has_custom_motion,
    });
    let panel_class = logic::compose_class_name(None, panel_state);
    let panel_class = StoredValue::new(panel_class);

    let anchor_ref: NodeRef<html::Span> = NodeRef::new();
    let panel_ref: NodeRef<html::Div> = NodeRef::new();

    let position = use_popover_position(PopoverPositionOptions {
        anchor_ref,
        panel_ref,
        placement,
        ..Default::default()
    });

    motion::attach_motion(
        panel_ref,
        open_signal,
        position.placement.into(),
        presence.finish_exit,
        motion,
    );

    let trigger = StoredValue::new(trigger);

    let on_key_down = move |ev: ev::KeyboardEvent| {
        #[cfg(target_arch = "wasm32")]
        let is_composing = ev.is_composing();
        #[cfg(not(target_arch = "wasm32"))]
        let is_composing = false;

        if !logic::should_handle_escape(&ev.key(), open_signal.get_untracked(), is_composing) {
            return;
        }

        ev.stop_propagation();
        ev.prevent_default();
        trigger_aria.state.dismiss.run(());
    };

    #[cfg(target_arch = "wasm32")]
    let focus_target = StoredValue::new_local(None::<leptos::web_sys::Element>);

    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        if let Some(target) = focus_target.get_value() {
            drop(target.remove_attribute("aria-describedby"));
        }
    });

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let is_open = open_signal.get();
        let Some(target) = focus_target.get_value() else {
            return;
        };

        let id = id.with_value(|id| id.clone());
        if is_open {
            drop(target.set_attribute("aria-describedby", &id));
        } else {
            drop(target.remove_attribute("aria-describedby"));
        }
    });

    let on_focus_in = move |_ev: ev::FocusEvent| {
        trigger_aria.handlers.on_trigger_focus_in.run(());

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            if let Some(target) = focus_target.get_value() {
                drop(target.remove_attribute("aria-describedby"));
            }

            let Some(target) = _ev.target() else {
                focus_target.set_value(None);
                return;
            };

            let Ok(target) = target.dyn_into::<leptos::web_sys::Element>() else {
                focus_target.set_value(None);
                return;
            };

            if open_signal.get_untracked() {
                let id = id.with_value(|id| id.clone());
                drop(target.set_attribute("aria-describedby", &id));
            }

            focus_target.set_value(Some(target));
        }
    };

    let on_focus_out = move |_ev: ev::FocusEvent| {
        trigger_aria.handlers.on_trigger_focus_out.run(());

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(target) = focus_target.get_value() {
                drop(target.remove_attribute("aria-describedby"));
            }
            focus_target.set_value(None);
        }
    };

    let panel_vars = move || {
        logic::compose_panel_vars(
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get(),
        )
    };

    view! {
        <span
            class=root_class
            data-slot=root_state.slot_attr
            data-state=move || logic::state_attr_for_open(open_signal.get())
            data-open=move || open_signal.get().then_some("true")
            data-closed=move || (!open_signal.get()).then_some("true")
            data-disabled=root_state.is_disabled.then_some("true")
            data-enabled=(!root_state.is_disabled).then_some("true")
            data-content=root_state.content_attr
            data-has-image=root_state.has_image.then_some("true")
            data-class-source=root_state.class_source_attr
            data-delay-source=root_state.delay_source_attr
            data-id-source=root_state.id_source_attr
            data-title-source=root_state.title_source_attr
            data-description-source=root_state.description_source_attr
            data-url-source=root_state.url_source_attr
            data-site-label-source=root_state.site_label_source_attr
            data-motion-source=root_state.motion_source_attr
            data-custom-class=root_state.has_custom_class_name.then_some("true")
            data-custom-delay=root_state.has_custom_delays.then_some("true")
            data-custom-id=root_state.has_custom_id.then_some("true")
            data-custom-title=root_state.has_custom_title.then_some("true")
            data-custom-description=root_state.has_custom_description.then_some("true")
            data-custom-url=root_state.has_custom_url.then_some("true")
            data-custom-motion=root_state.has_custom_motion.then_some("true")
        >
            <span
                class=trigger_class
                data-slot=trigger_state.slot_attr
                data-state=trigger_state.state_attr
                data-disabled=trigger_state.is_disabled.then_some("true")
                data-enabled=(!trigger_state.is_disabled).then_some("true")
                data-class-source=trigger_state.class_source_attr
                data-delay-source=trigger_state.delay_source_attr
                data-id-source=trigger_state.id_source_attr
                data-title-source=trigger_state.title_source_attr
                data-description-source=trigger_state.description_source_attr
                data-url-source=trigger_state.url_source_attr
                data-site-label-source=trigger_state.site_label_source_attr
                data-motion-source=trigger_state.motion_source_attr
                node_ref=anchor_ref
                on:pointerenter=move |_| trigger_aria.handlers.on_trigger_pointer_enter.run(())
                on:pointerleave=move |_| trigger_aria.handlers.on_trigger_pointer_leave.run(())
                on:focusin=on_focus_in
                on:focusout=on_focus_out
                on:keydown=on_key_down
            >
                {move || trigger.with_value(|trigger| trigger.run())}
            </span>

            <Show when=move || presence.is_present.get()>
                <Portal>
                    <div
                        class=move || panel_class.with_value(|class_name| class_name.clone())
                        node_ref=panel_ref
                        id=move || id.with_value(|id| id.clone())
                        role="tooltip"
                        data-ui-overlay-portal=""
                        data-placement=move || position.placement.get().as_str()
                        data-slot=panel_state.slot_attr
                        data-state=panel_state.state_attr
                        data-open=move || open_signal.get().then_some("true")
                        data-closed=move || (!open_signal.get()).then_some("true")
                        data-disabled=panel_state.is_disabled.then_some("true")
                        data-enabled=(!panel_state.is_disabled).then_some("true")
                        data-content=panel_state.content_attr
                        data-has-image=panel_state.has_image.then_some("true")
                        data-class-source=panel_state.class_source_attr
                        data-delay-source=panel_state.delay_source_attr
                        data-id-source=panel_state.id_source_attr
                        data-title-source=panel_state.title_source_attr
                        data-description-source=panel_state.description_source_attr
                        data-url-source=panel_state.url_source_attr
                        data-site-label-source=panel_state.site_label_source_attr
                        data-motion-source=panel_state.motion_source_attr
                        style=panel_vars
                        on:pointerenter=move |_| trigger_aria.handlers.on_panel_pointer_enter.run(())
                        on:pointerleave=move |_| trigger_aria.handlers.on_panel_pointer_leave.run(())
                        on:focusin=move |_| trigger_aria.handlers.on_panel_focus_in.run(())
                        on:focusout=move |_| trigger_aria.handlers.on_panel_focus_out.run(())
                        on:keydown=on_key_down
                    >
                        <Show
                            when=move || image_src.with_value(|value| value.is_some())
                            fallback=move || ().into_any()
                        >
                            {move || {
                                image_src
                                    .with_value(|value| value.clone())
                                    .map(|src| {
                                        view! {
                                            <img
                                                class="ui-preview-card__image"
                                                data-slot="preview-card-image"
                                                src=src
                                                alt=""
                                                loading="lazy"
                                            />
                                        }
                                    })
                            }}
                        </Show>

                        <div class="ui-preview-card__body" data-slot="preview-card-body">
                            <div class="ui-preview-card__title" data-slot="preview-card-title">
                                {move || title.get_value()}
                            </div>
                            <div
                                class="ui-preview-card__description"
                                data-slot="preview-card-description"
                            >
                                {move || description.get_value()}
                            </div>
                            <div class="ui-preview-card__meta" data-slot="preview-card-meta">
                                <span data-slot="preview-card-site-label">{move || site_label.get_value()}</span>
                                <span class="ui-preview-card__meta-link" data-slot="preview-card-url">
                                    {move || url.get_value()}
                                </span>
                            </div>
                        </div>
                    </div>
                </Portal>
            </Show>
        </span>
    }
}
