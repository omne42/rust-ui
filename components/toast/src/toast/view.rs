use crate::OnPress;
use crate::close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};
use crate::toast::{ToastMotion, ToastSlot, ToastStore, logic, motion};
use leptos::{html, portal::Portal, prelude::*};
use ui_headless::{
    A11yDirection, CommonStrings, ToastA11yOptions, locale_attrs,
    use_controllable_open_state_traced, use_toast_a11y, use_ui_i18n,
};

const TOAST_CLOSE_GLYPH: &str = "×";

fn render_toast_description(description: String) -> impl IntoView {
    view! {
        <div class="ui-toast__description" data-slot=ToastSlot::Description.as_attr()>
            {description}
        </div>
    }
}

struct ToastItemRenderInput {
    id: String,
    title: String,
    description: String,
    variant: crate::toast::ToastVariant,
    is_open: Signal<bool>,
    motion: ToastMotion,
    on_close: OnPress,
    on_exit_complete: Callback<()>,
}

fn render_toast_item(input: ToastItemRenderInput) -> impl IntoView {
    view! {
        <Toast
            id=input.id
            title=input.title
            description=input.description
            variant=input.variant
            is_open=input.is_open
            motion=input.motion
            on_close=input.on_close
            on_exit_complete=input.on_exit_complete
        />
    }
}

#[component]
pub fn Toast(
    title: String,
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] variant: crate::toast::ToastVariant,
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] on_close: Option<OnPress>,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] close_aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let motion = crate::toast::motion::sanitize_motion(motion);
    let normalized = logic::normalize_props(logic::ToastNormalizeInput {
        title,
        id,
        description,
        class_name,
        motion,
    });
    let requested_open = is_open.or(open);
    let open_config =
        logic::resolve_open_state_config(requested_open, default_open, on_open_change);
    let open_state_markers = logic::resolve_open_state_markers(&open_config);
    let is_controlled = open_config.is_controlled;
    let has_custom_default_open = open_config.has_custom_default_open;
    let has_custom_on_open_change = open_config.has_custom_on_open_change;
    let open_source_attr = open_config.open_source_attr;
    let controlled_open = open_config.controlled_open;
    let default_open = open_config.default_open;
    let on_open_change = open_config.on_open_change;

    let open_state =
        use_controllable_open_state_traced("toast", controlled_open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let has_custom_id = normalized.has_custom_id;
    let has_description = normalized.has_description;
    let has_custom_description = normalized.has_custom_description;
    let has_custom_class_name = normalized.has_custom_class_name;
    let has_custom_motion = normalized.has_custom_motion;

    let title = normalized.title;
    let id = StoredValue::new(normalized.id);
    let description = StoredValue::new(normalized.description);
    let class_name = StoredValue::new(normalized.class_name);

    let callbacks = logic::resolve_callbacks_config(on_close, on_exit_complete);
    let has_custom_on_close = callbacks.has_custom_on_close;
    let has_custom_on_exit_complete = callbacks.has_custom_on_exit_complete;
    let on_close = StoredValue::new(callbacks.on_close);
    let close_toast = Callback::new(move |_| {
        on_close.get_value().run(());
        request_open_change.run(false);
    });
    let on_exit_complete = callbacks.on_exit_complete;

    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let close_aria_label =
        logic::resolve_close_aria_label(close_aria_label, common.close_aria_label.as_ref());
    let live_region_priority = logic::resolve_live_region_priority(variant);
    let toast_a11y = use_toast_a11y(ToastA11yOptions {
        is_open: open,
        priority: live_region_priority,
        lang: logic::normalize_optional_text(lang),
        dir,
        on_dismiss_request: close_toast,
    });
    let toast_role = toast_a11y.attrs.role;
    let toast_aria_live = toast_a11y.attrs.aria_live;
    let toast_aria_atomic = toast_a11y.attrs.aria_atomic;
    let toast_aria_keyshortcuts = toast_a11y.attrs.aria_keyshortcuts;
    let toast_lang = toast_a11y.attrs.lang;
    let toast_dir = toast_a11y.attrs.dir;
    let toast_on_key_down = toast_a11y.handlers.on_key_down;
    let agent_contract = logic::toast_agent_contract();

    let state = Memo::new(move |_| {
        logic::resolve_toast_part_state(logic::ToastStateDerivationInput {
            variant,
            is_open: open.get(),
            has_description,
            has_custom_id,
            has_custom_description,
            has_custom_class_name,
            has_custom_motion,
            has_custom_on_close,
            has_custom_on_exit_complete,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, open, on_exit_complete, motion);

    view! {
        <div
            class=move || class.get()
            data-ui-schema=agent_contract.schema_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action-model=agent_contract.action_model_attr
            data-ui-state-axis=agent_contract.state_axis_attr
            data-ui-source-axis=agent_contract.source_axis_attr
            data-slot=move || state.get().slot_attr
            data-id=move || id.get_value()
            data-state=move || state.get().state_attr
            data-variant=move || state.get().variant_attr
            data-description=move || state.get().description_attr
            data-open=move || state.get().open_attr
            data-close-mode=move || state.get().close_mode_attr
            data-id-source=move || state.get().id_source_attr
            data-description-source=move || state.get().description_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-close-source=move || state.get().close_source_attr
            data-exit-source=move || state.get().exit_source_attr
            data-control-mode=open_state_markers.control_mode_attr
            data-open-source=open_source_attr
            data-default-open-source=open_state_markers.default_open_source_attr
            data-open-change-source=open_state_markers.open_change_source_attr
            data-custom-id=move || state.get().has_custom_id.then_some("true")
            data-custom-description=move || state.get().has_custom_description.then_some("true")
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-custom-close=move || state.get().has_custom_on_close.then_some("true")
            data-custom-exit=move || state.get().has_custom_on_exit_complete.then_some("true")
            data-controlled=is_controlled.then_some("true")
            data-uncontrolled=(!is_controlled).then_some("true")
            data-custom-default-open=has_custom_default_open.then_some("true")
            data-custom-open-change=has_custom_on_open_change.then_some("true")
            node_ref=root_ref
            role=toast_role
            aria-live=toast_aria_live
            aria-atomic=toast_aria_atomic
            aria-keyshortcuts=toast_aria_keyshortcuts
            lang=toast_lang
            dir=toast_dir
            on:keydown=move |ev| toast_on_key_down.run(ev)
        >
            <div class="ui-toast__content" data-slot=ToastSlot::Content.as_attr()>
                <div class="ui-toast__title" data-slot=ToastSlot::Title.as_attr()>{title}</div>
                {move || {
                    description.get_value().map(|description| {
                        render_toast_description(description)
                    })
                }}
            </div>
            <span data-slot=ToastSlot::Close.as_attr()>
                <CloseButton
                    variant=CloseButtonVariant::Default
                    size=CloseButtonSize::Md
                    class_name="ui-toast__close".to_string()
                    aria_label=close_aria_label
                    on_press=close_toast
                >
                    {TOAST_CLOSE_GLYPH}
                </CloseButton>
            </span>
        </div>
    }
}

#[component]
pub fn ToastViewport(
    #[prop(optional)] motion: ToastMotion,
    #[prop(optional, default = logic::DEFAULT_VIEWPORT_PORTAL)] is_portal: bool,
    #[prop(optional, default = logic::DEFAULT_VIEWPORT_MAX_TOASTS)] max_toasts: usize,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] store: Option<ToastStore>,
) -> impl IntoView {
    let motion = crate::toast::motion::sanitize_motion(motion);
    let normalized = logic::normalize_viewport_props(logic::ToastViewportNormalizeInput {
        is_portal,
        max_toasts,
        class_name,
        motion,
    });
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);

    let viewport_config = normalized.viewport;
    let (store, store_source) =
        logic::resolve_viewport_store(store, normalized.normalized_max_toasts);

    let viewport_state =
        logic::resolve_toast_viewport_state(logic::ToastViewportStateDerivationInput {
            is_portal: viewport_config.is_portal,
            max_toasts: viewport_config.max_toasts,
            has_custom_portal: viewport_config.has_custom_portal,
            has_custom_max_toasts: viewport_config.has_custom_max_toasts,
            has_custom_class_name: normalized.has_custom_class_name,
            has_custom_motion: normalized.has_custom_motion,
            store_source,
        });

    let viewport_class_name =
        logic::compose_viewport_class_name(normalized.class_name, viewport_state);
    let viewport_class_name = StoredValue::new(viewport_class_name);
    let viewport_state = StoredValue::new(viewport_state);
    let viewport_agent_contract = logic::toast_viewport_agent_contract();

    let store = StoredValue::new(store);
    let items = Signal::derive(move || store.get_value().toasts().get());
    let viewport_lang = locale.lang.clone();
    let viewport_dir = locale.dir;

    let render_item = move |toast: crate::toast::ToastInstance| {
        let id = toast.id.clone();
        let title = toast.title.clone();
        let description = toast.description.clone();
        let variant = toast.variant;
        let is_open = {
            let store = store.get_value();
            let id = id.clone();
            Signal::derive(move || logic::resolve_instance_open(&store.toasts().get(), &id))
        };

        let on_exit_complete = {
            let store = store.get_value();
            let id = id.clone();
            Callback::new(move |_| store.remove(&id))
        };

        let on_close = {
            let store = store.get_value();
            let id = id.clone();
            Callback::new(move |_| store.dismiss.run(id.clone()))
        };

        render_toast_item(ToastItemRenderInput {
            id,
            title,
            description: logic::resolve_instance_description(description),
            variant,
            is_open,
            motion,
            on_close,
            on_exit_complete,
        })
    };

    let render_viewport_root = move || {
        view! {
            <div
                class=move || viewport_class_name.get_value()
                data-ui-schema=viewport_agent_contract.schema_attr
                data-ui-intent=viewport_agent_contract.intent_attr
                data-ui-action-model=viewport_agent_contract.action_model_attr
                data-ui-state-axis=viewport_agent_contract.state_axis_attr
                data-ui-source-axis=viewport_agent_contract.source_axis_attr
                data-ui-overlay-portal=move || viewport_state.get_value().portal.then_some("")
                data-slot=move || viewport_state.get_value().slot_attr
                data-state=move || viewport_state.get_value().state_attr
                data-queue=move || viewport_state.get_value().queue_attr
                data-portal=move || viewport_state.get_value().portal_attr
                data-max-toasts=move || viewport_state.get_value().max_toasts.to_string()
                data-portal-source=move || viewport_state.get_value().portal_source_attr
                data-max-toasts-source=move || viewport_state.get_value().max_toasts_source_attr
                data-class-source=move || viewport_state.get_value().class_source_attr
                data-motion-source=move || viewport_state.get_value().motion_source_attr
                data-store-source=move || viewport_state.get_value().store_source_attr
                data-custom-portal=move || viewport_state.get_value().has_custom_portal.then_some("true")
                data-custom-max-toasts=move || viewport_state.get_value().has_custom_max_toasts.then_some("true")
                data-custom-class=move || viewport_state.get_value().has_custom_class_name.then_some("true")
                data-custom-motion=move || viewport_state.get_value().has_custom_motion.then_some("true")
                lang=viewport_lang.clone()
                dir=viewport_dir
            >
                <For each=move || items.get() key=|toast| toast.id.clone() children=render_item />
            </div>
        }
    };

    if viewport_state.get_value().portal {
        view! { <Portal>{render_viewport_root()}</Portal> }.into_any()
    } else {
        render_viewport_root().into_any()
    }
}
