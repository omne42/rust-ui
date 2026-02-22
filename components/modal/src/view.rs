use crate::OnPress;
use crate::modal::{ModalPartStateInput, ModalSlot, logic, motion as motion_contract};
use crate::overlay::Overlay;
use crate::overlay::OverlayMotion;
use leptos::prelude::*;
use std::borrow::Cow;

fn render_modal_title(
    title: Signal<String>,
    title_id_attr: Signal<String>,
    title_state: crate::modal::ModalPartState,
    title_class: StoredValue<Cow<'static, str>>,
) -> AnyView {
    view! {
        <h2
            class=move || title_class.with_value(|class_name| class_name.clone())
            id=move || title_id_attr.get()
            data-slot=title_state.slot_attr
            data-state=title_state.state_attr
            data-title-source=title_state.title_source_attr
        >
            {move || title.get()}
        </h2>
    }
    .into_any()
}

fn render_modal_description(
    description: Signal<String>,
    description_id_attr: Signal<String>,
    description_state: crate::modal::ModalPartState,
    description_class: StoredValue<Cow<'static, str>>,
) -> AnyView {
    view! {
        <p
            class=move || description_class.with_value(|class_name| class_name.clone())
            id=move || description_id_attr.get()
            data-slot=description_state.slot_attr
            data-state=description_state.state_attr
            data-description-source=description_state.description_source_attr
        >
            {move || description.get()}
        </p>
    }
    .into_any()
}

fn render_modal_body(
    body_state: crate::modal::ModalPartState,
    body_class: StoredValue<Cow<'static, str>>,
    children: StoredValue<ChildrenFn>,
) -> AnyView {
    view! {
        <div
            class=move || body_class.with_value(|class_name| class_name.clone())
            data-slot=body_state.slot_attr
            data-state=body_state.state_attr
        >
            {children.with_value(|children| children())}
        </div>
    }
    .into_any()
}

struct ModalSectionsInput {
    title: Signal<String>,
    title_id_attr: Signal<String>,
    title_state: crate::modal::ModalPartState,
    title_class: StoredValue<Cow<'static, str>>,
    description: Option<Signal<String>>,
    description_id_attr: Signal<String>,
    description_state: crate::modal::ModalPartState,
    description_class: StoredValue<Cow<'static, str>>,
    body_state: crate::modal::ModalPartState,
    body_class: StoredValue<Cow<'static, str>>,
    children: StoredValue<ChildrenFn>,
}

fn render_modal_sections(input: ModalSectionsInput) -> AnyView {
    let description_view = input.description.map(|description| {
        render_modal_description(
            description,
            input.description_id_attr,
            input.description_state,
            input.description_class,
        )
    });

    view! {
        {render_modal_title(input.title, input.title_id_attr, input.title_state, input.title_class)}
        {description_view}
        {render_modal_body(input.body_state, input.body_class, input.children)}
    }
    .into_any()
}

#[component]
pub fn Modal(
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    id_base: String,
    title: String,
    on_close: OnPress,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] motion: OverlayMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<ui_headless::A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let open_state = logic::normalize_open_state(logic::ModalOpenStateInput {
        is_open,
        default_open,
        on_open_change,
    });
    let children = StoredValue::new(children);
    let open_contract = logic::resolve_open_contract(&open_state);
    let open_state = ui_headless::use_controllable_open_state_traced(
        "modal",
        open_state.open,
        Some(open_state.default_open),
        open_state.on_open_change,
    );
    let is_open = open_state.open;
    let request_open_change = open_state.request_open_change;
    let close_action: OnPress = Callback::new(move |_| {
        request_open_change.run(false);
        on_close.run(());
    });
    let close_action = StoredValue::new(close_action);

    let content_state = logic::resolve_content_state(logic::ModalContentStateInput {
        id_base,
        title,
        description,
        class_name,
    });
    let motion = motion_contract::normalize_motion(motion);
    let has_custom_motion = motion_contract::is_custom_motion(motion);
    let has_on_exit_complete = on_exit_complete.is_some();

    let root_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Root,
        description_state: content_state.description_state,
        has_custom_id_base: content_state.has_custom_id_base,
        has_custom_title: content_state.has_custom_title,
        has_custom_description: content_state.has_custom_description,
        has_custom_class_name: content_state.has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let root_class = logic::compose_class_name(content_state.class_name, root_state);
    let root_class = StoredValue::new(root_class);

    let title_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Title,
        description_state: content_state.description_state,
        has_custom_id_base: content_state.has_custom_id_base,
        has_custom_title: content_state.has_custom_title,
        has_custom_description: content_state.has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let title_class = logic::compose_class_name(None, title_state);
    let title_class = StoredValue::new(title_class);

    let description_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Description,
        description_state: content_state.description_state,
        has_custom_id_base: content_state.has_custom_id_base,
        has_custom_title: content_state.has_custom_title,
        has_custom_description: content_state.has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let description_class = logic::compose_class_name(None, description_state);
    let description_class = StoredValue::new(description_class);

    let body_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Body,
        description_state: content_state.description_state,
        has_custom_id_base: content_state.has_custom_id_base,
        has_custom_title: content_state.has_custom_title,
        has_custom_description: content_state.has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let body_class = logic::compose_class_name(None, body_state);
    let body_class = StoredValue::new(body_class);

    let id_base = content_state.id_base;
    let title: Signal<String> = content_state.title.into();
    let description: Option<Signal<String>> = content_state.description.map(Into::into);
    let description_for_sections = StoredValue::new(description);
    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let dialog_a11y = ui_headless::overlay_dialog_attrs(
        Some(title_id.clone()),
        description.as_ref().map(|_| description_id.clone()),
        lang,
        dir,
    );
    let dialog_aria_labelledby = dialog_a11y.aria_labelledby.unwrap_or_default();
    let dialog_aria_describedby = dialog_a11y.aria_describedby.unwrap_or_default();
    let dialog_lang = StoredValue::new(dialog_a11y.lang);
    let dialog_dir = dialog_a11y.dir;
    let on_overlay_close: OnPress =
        Callback::new(move |_| close_action.with_value(|callback| callback.run(())));

    let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::ModalAgentContractInput {
            is_open: is_open.get(),
            open_mode: open_contract.mode,
            has_description: root_state.description_state.shows_description(),
        })
    });
    view! {
        <Overlay
            open=is_open
            on_close=on_overlay_close
            aria_labelledby=dialog_aria_labelledby
            aria_describedby=dialog_aria_describedby
            motion=motion
            on_exit_complete=on_exit_complete
        >
            <div
                class=move || root_class.with_value(|class_name| class_name.clone())
                lang=move || dialog_lang.get_value()
                dir=dialog_dir
                data-slot=root_state.slot_attr
                data-state=root_state.state_attr
                data-description=root_state.description_attr
                data-with-description=root_state
                    .description_state
                    .shows_description()
                    .then_some("true")
                data-custom-id=root_state.has_custom_id_base.then_some("true")
                data-custom-title=root_state.has_custom_title.then_some("true")
                data-custom-description=root_state.has_custom_description.then_some("true")
                data-custom-class=root_state.has_custom_class_name.then_some("true")
                data-custom-motion=root_state.has_custom_motion.then_some("true")
                data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                data-id-source=root_state.id_source_attr
                data-title-source=root_state.title_source_attr
                data-description-source=root_state.description_source_attr
                data-class-source=root_state.class_source_attr
                data-motion-source=root_state.motion_source_attr
                data-exit-source=root_state.exit_source_attr
                data-open=move || is_open.get().then_some("true")
                data-closed=move || (!is_open.get()).then_some("true")
                data-open-mode=open_contract.mode.as_attr()
                data-open-source=open_contract.open_source.as_attr()
                data-open-change-source=open_contract.open_change_source.as_attr()
                data-open-prop-source=open_contract.open_prop_source.as_attr()
                data-controlled=(open_contract.mode == logic::ModalOpenMode::Controlled)
                    .then_some("true")
                data-uncontrolled=(open_contract.mode == logic::ModalOpenMode::Uncontrolled)
                    .then_some("true")
                data-custom-default-open=open_contract.has_custom_default_open.then_some("true")
                data-custom-open-change=open_contract.has_custom_on_open_change.then_some("true")
                data-ui-schema=move || agent_contract.get().schema_name
                data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
                data-ui-intent=move || agent_contract.get().intent.as_str()
                data-ui-action=move || agent_contract.get().action.as_str()
                data-ui-state=move || agent_contract.get().state.as_str()
                data-ui-source=move || agent_contract.get().source.as_str()
                data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
                data-ui-output-status=move || agent_contract.get().output_status.as_str()
                data-ui-capability-description=move || {
                    agent_contract.get().capabilities.has_description.then_some("true")
                }
                data-ui-capability-open=move || {
                    agent_contract.get().capabilities.can_open.then_some("true")
                }
                data-ui-capability-close=move || {
                    agent_contract.get().capabilities.can_close.then_some("true")
                }
            >
                {move || {
                    render_modal_sections(ModalSectionsInput {
                        title,
                        title_id_attr,
                        title_state,
                        title_class,
                        description: description_for_sections.get_value(),
                        description_id_attr,
                        description_state,
                        description_class,
                        body_state,
                        body_class,
                        children,
                    })
                }}
            </div>
        </Overlay>
    }
    .into_any()
}
