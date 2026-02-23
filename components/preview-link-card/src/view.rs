use crate::preview_link_card::{PreviewLinkCardMotion, logic, motion};
use leptos::{children::ViewFn, html, portal::Portal, prelude::*};
use ui_headless::a11y::{
    A11yDirection, TooltipPanelA11yOptions, locale_attrs, tooltip_panel_attrs,
};
use ui_headless::{
    HoverCardDismissOptions, HoverCardFocusA11yOptions, HoverCardTriggerOptions, PopoverPlacement,
    PopoverPositionOptions, use_hover_card_dismiss, use_hover_card_focus_a11y,
    use_hover_card_trigger, use_popover_position,
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
pub fn PreviewLinkCard(
    #[prop(into)] trigger: ViewFn,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] url: Option<String>,
    #[prop(optional, into)] site_label: Option<String>,
    #[prop(optional, into)] image_src: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] open_delay_ms: Option<u64>,
    #[prop(optional)] close_delay_ms: Option<u64>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: PreviewLinkCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_motion = motion != PreviewLinkCardMotion::default();
    let normalized_delays = logic::normalize_delays(logic::DelayInput {
        open_delay_ms,
        close_delay_ms,
    });
    let open_delay_ms = normalized_delays.open_delay_ms;
    let close_delay_ms = normalized_delays.close_delay_ms;
    let has_custom_delays = normalized_delays.has_custom_delays;

    let (id, has_custom_id) = logic::resolve_id(id, format!("ui-preview-link-card-{}", next_id()));
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
    let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let open = normalized_open_state.open;
    let default_open = normalized_open_state.default_open;
    let on_open_change = normalized_open_state.on_open_change;
    let open_state_source_markers =
        logic::resolve_open_state_source_markers(logic::OpenStateSourceMarkersInput {
            is_controlled: normalized_open_state.is_controlled,
            has_open_prop: open.is_some(),
            has_default_open: default_open.is_some(),
            has_on_open_change: on_open_change.is_some(),
        });

    let trigger_aria = use_hover_card_trigger(HoverCardTriggerOptions {
        is_disabled,
        open_delay_ms,
        close_delay_ms,
        open,
        default_open,
        on_open_change,
    });
    let open_signal = trigger_aria.state.is_open;
    let open_markers = Memo::new(move |_| {
        logic::resolve_open_state_markers(logic::OpenStateMarkersInput {
            is_open: open_signal.get(),
        })
    });
    let presence = ui_headless::use_presence(open_signal);
    let dismiss_a11y = use_hover_card_dismiss(HoverCardDismissOptions {
        is_open: open_signal,
        dismiss: trigger_aria.state.dismiss,
    });
    let focus_a11y = use_hover_card_focus_a11y(HoverCardFocusA11yOptions {
        hover_card_id: id,
        is_open: open_signal,
        on_focus_in: trigger_aria.handlers.on_trigger_focus_in,
        on_focus_out: trigger_aria.handlers.on_trigger_focus_out,
    });
    let trigger_on_key_down = dismiss_a11y.handlers.on_key_down;
    let panel_on_key_down = trigger_on_key_down;
    let trigger_on_focus_in = focus_a11y.handlers.on_focus_in;
    let trigger_on_focus_out = focus_a11y.handlers.on_focus_out;

    let root_state = logic::resolve_part_state(logic::PreviewLinkCardPartStateInput {
        slot: logic::PreviewLinkCardSlot::Root,
        disabled: is_disabled,
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

    let trigger_state = logic::resolve_part_state(logic::PreviewLinkCardPartStateInput {
        slot: logic::PreviewLinkCardSlot::Trigger,
        disabled: is_disabled,
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

    let panel_state = logic::resolve_part_state(logic::PreviewLinkCardPartStateInput {
        slot: logic::PreviewLinkCardSlot::Panel,
        disabled: is_disabled,
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

    let panel_vars = move || {
        logic::compose_panel_vars(
            position.top_px.get(),
            position.left_px.get(),
            position.anchor_width_px.get(),
        )
    };
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let root_lang = locale.lang.clone();
    let root_dir = locale.dir;
    let panel_lang = locale.lang.clone();
    let panel_a11y = Memo::new(move |_| {
        tooltip_panel_attrs(TooltipPanelA11yOptions {
            tooltip_id: id.with_value(|id| id.clone()),
            is_open: open_signal.get(),
            lang: panel_lang.clone(),
            dir,
        })
    });

    view! {
        <span
            class=root_class
            lang=root_lang.clone()
            dir=root_dir
            data-slot=root_state.slot_attr
            data-state=move || open_markers.get().state_attr.as_attr()
            data-open=move || open_markers.get().open_attr
            data-closed=move || open_markers.get().closed_attr
            data-disabled=root_state.is_disabled.then_some("true")
            data-enabled=(!root_state.is_disabled).then_some("true")
            data-content=root_state.content_attr.as_attr()
            data-has-image=root_state.has_image.then_some("true")
            data-class-source=root_state.class_source_attr.as_attr()
            data-delay-source=root_state.delay_source_attr.as_attr()
            data-id-source=root_state.id_source_attr.as_attr()
            data-title-source=root_state.title_source_attr.as_attr()
            data-description-source=root_state.description_source_attr.as_attr()
            data-url-source=root_state.url_source_attr.as_attr()
            data-site-label-source=root_state.site_label_source_attr.as_attr()
            data-motion-source=root_state.motion_source_attr.as_attr()
            data-open-mode=open_state_source_markers.open_mode_attr.as_attr()
            data-open-source=open_state_source_markers.open_source_attr.as_attr()
            data-default-open-source=open_state_source_markers.default_open_source_attr.as_attr()
            data-open-change-source=open_state_source_markers.open_change_source_attr.as_attr()
            data-controlled=matches!(
                open_state_source_markers.open_mode_attr,
                logic::PreviewLinkCardOpenModeAttr::Controlled
            )
            .then_some("true")
            data-uncontrolled=matches!(
                open_state_source_markers.open_mode_attr,
                logic::PreviewLinkCardOpenModeAttr::Uncontrolled
            )
            .then_some("true")
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
                data-state=trigger_state.state_attr.as_attr()
                data-disabled=trigger_state.is_disabled.then_some("true")
                data-enabled=(!trigger_state.is_disabled).then_some("true")
                data-class-source=trigger_state.class_source_attr.as_attr()
                data-delay-source=trigger_state.delay_source_attr.as_attr()
                data-id-source=trigger_state.id_source_attr.as_attr()
                data-title-source=trigger_state.title_source_attr.as_attr()
                data-description-source=trigger_state.description_source_attr.as_attr()
                data-url-source=trigger_state.url_source_attr.as_attr()
                data-site-label-source=trigger_state.site_label_source_attr.as_attr()
                data-motion-source=trigger_state.motion_source_attr.as_attr()
                data-open-mode=open_state_source_markers.open_mode_attr.as_attr()
                data-open-source=open_state_source_markers.open_source_attr.as_attr()
                data-default-open-source=open_state_source_markers.default_open_source_attr.as_attr()
                data-open-change-source=open_state_source_markers.open_change_source_attr.as_attr()
                data-focus-a11y-managed=focus_a11y.attrs.manages_aria_describedby.then_some("true")
                aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts
                node_ref=anchor_ref
                on:pointerenter=move |_| trigger_aria.handlers.on_trigger_pointer_enter.run(())
                on:pointerleave=move |_| trigger_aria.handlers.on_trigger_pointer_leave.run(())
                on:focusin=move |ev| trigger_on_focus_in.run(ev)
                on:focusout=move |ev| trigger_on_focus_out.run(ev)
                on:keydown=move |ev| trigger_on_key_down.run(ev)
            >
                {move || trigger.with_value(|trigger| trigger.run())}
            </span>

            <Show when=move || presence.is_present.get()>
                <Portal>
                    <div
                        class=move || panel_class.with_value(|class_name| class_name.clone())
                        node_ref=panel_ref
                        id=move || panel_a11y.get().attrs.id.clone()
                        role=move || panel_a11y.get().attrs.role
                        lang=move || panel_a11y.get().attrs.lang.clone()
                        dir=move || panel_a11y.get().attrs.dir
                        data-ui-overlay-portal=""
                        data-placement=move || position.placement.get().as_str()
                        data-slot=panel_state.slot_attr
                        data-state=panel_state.state_attr.as_attr()
                        data-open=move || open_markers.get().open_attr
                        data-closed=move || open_markers.get().closed_attr
                        data-disabled=panel_state.is_disabled.then_some("true")
                        data-enabled=(!panel_state.is_disabled).then_some("true")
                        data-content=panel_state.content_attr.as_attr()
                        data-has-image=panel_state.has_image.then_some("true")
                        data-class-source=panel_state.class_source_attr.as_attr()
                        data-delay-source=panel_state.delay_source_attr.as_attr()
                        data-id-source=panel_state.id_source_attr.as_attr()
                        data-title-source=panel_state.title_source_attr.as_attr()
                        data-description-source=panel_state.description_source_attr.as_attr()
                        data-url-source=panel_state.url_source_attr.as_attr()
                        data-site-label-source=panel_state.site_label_source_attr.as_attr()
                        data-motion-source=panel_state.motion_source_attr.as_attr()
                        data-open-mode=open_state_source_markers.open_mode_attr.as_attr()
                        data-open-source=open_state_source_markers.open_source_attr.as_attr()
                        data-default-open-source=open_state_source_markers.default_open_source_attr.as_attr()
                        data-open-change-source=open_state_source_markers.open_change_source_attr.as_attr()
                        aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts
                        style=panel_vars
                        on:pointerenter=move |_| trigger_aria.handlers.on_panel_pointer_enter.run(())
                        on:pointerleave=move |_| trigger_aria.handlers.on_panel_pointer_leave.run(())
                        on:focusin=move |_| trigger_aria.handlers.on_panel_focus_in.run(())
                        on:focusout=move |_| trigger_aria.handlers.on_panel_focus_out.run(())
                        on:keydown=move |ev| panel_on_key_down.run(ev)
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
                                                class="ui-preview-link-card__image"
                                                data-slot="preview-link-card-image"
                                                src=src
                                                alt=""
                                                loading="lazy"
                                            />
                                        }
                                    })
                            }}
                        </Show>

                        <div class="ui-preview-link-card__body" data-slot="preview-link-card-body">
                            <div class="ui-preview-link-card__title" data-slot="preview-link-card-title">
                                {move || title.get_value()}
                            </div>
                            <div
                                class="ui-preview-link-card__description"
                                data-slot="preview-link-card-description"
                            >
                                {move || description.get_value()}
                            </div>
                            <div class="ui-preview-link-card__meta" data-slot="preview-link-card-meta">
                                <span data-slot="preview-link-card-site-label">{move || site_label.get_value()}</span>
                                <span class="ui-preview-link-card__meta-link" data-slot="preview-link-card-url">
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
