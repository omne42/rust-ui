use crate::{TagInteractivityMode, TagSize, TagVariant, logic};
use leptos::prelude::*;
use ui_headless::{A11yDirection, OnPress, locale_attrs};
use ui_headless::{ButtonOptions, use_button};

const TAG_CONTENT_CLASS: &str = "ui-tag__content";
const TAG_CONTENT_SLOT: &str = "tag-content";
const TAG_REMOVE_CLASS: &str = "ui-tag__remove";
const TAG_REMOVE_SLOT: &str = "tag-remove-button";
const TAG_REMOVE_GLYPH: &str = "×";

fn render_tag_content(children: Children) -> impl IntoView {
    view! {
        <span class=TAG_CONTENT_CLASS data-slot=TAG_CONTENT_SLOT>
            {children()}
        </span>
    }
}

fn render_remove_button(
    state: logic::TagState,
    agent_source: RwSignal<logic::TagAgentSource>,
    remove_aria_label: StoredValue<String>,
    on_remove: StoredValue<Option<OnPress>>,
) -> impl IntoView {
    let on_press = on_remove.get_value().map(|on_remove| {
        Callback::new(move |_| {
            agent_source.set(logic::TagAgentSource::RemovePointer);
            on_remove.run(());
        })
    });
    let remove_button_aria = use_button(ButtonOptions {
        is_disabled: state.is_disabled,
        on_press,
        ..Default::default()
    });

    view! {
        <Show when=move || state.is_removable>
            <button
                type="button"
                class=TAG_REMOVE_CLASS
                role=remove_button_aria.attrs.role
                tabindex=remove_button_aria.attrs.tabindex
                aria-disabled=remove_button_aria.attrs.aria_disabled
                aria-label=move || remove_aria_label.get_value()
                data-slot=TAG_REMOVE_SLOT
                data-disabled=state.is_disabled.then_some("true")
                data-label-source=state.remove_label_source_attr
                disabled=state.is_disabled
                on:pointerdown=move |_| remove_button_aria.handlers.press.on_pointer_down.run(())
                on:pointerup=move |_| remove_button_aria.handlers.press.on_pointer_up.run(())
                on:pointercancel=move |_| remove_button_aria.handlers.press.on_pointer_cancel.run(())
                on:click=move |_| remove_button_aria.handlers.press.on_click.run(())
                on:keydown=move |ev| {
                    let key = ev.key();
                    if remove_button_aria.handlers.press.on_key_down.run(key) {
                        ev.prevent_default();
                    }
                }
                on:keyup=move |ev| {
                    let key = ev.key();
                    if remove_button_aria.handlers.press.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                }
                on:blur=move |_| {
                    remove_button_aria.handlers.press.on_blur.run(());
                }
            >
                {TAG_REMOVE_GLYPH}
            </button>
        </Show>
    }
}

#[component]
pub fn Tag(
    #[prop(optional)] variant: TagVariant,
    #[prop(optional)] size: TagSize,
    #[prop(optional)] mode: Option<TagInteractivityMode>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] is_removable: Option<bool>,
    #[prop(optional)] on_remove: Option<OnPress>,
    #[prop(optional, into)] remove_aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let normalized = logic::normalize_tag_input(logic::TagNormalizeInput {
        variant,
        size,
        mode,
        is_disabled,
        is_removable,
        has_remove_handler: on_remove.is_some(),
        remove_aria_label,
        class_name,
    });
    let state = normalized.state;
    let agent_source = RwSignal::new(logic::TagAgentSource::Init);
    let agent_contract =
        Signal::derive(move || logic::resolve_agent_contract(state, agent_source.get()));

    let class = logic::compose_class_name(normalized.class_name, state);
    let remove_aria_label = StoredValue::new(normalized.remove_aria_label);
    let on_remove = StoredValue::new(on_remove);
    let locale = locale_attrs(lang, dir);
    let content = render_tag_content(children);
    let remove_button = render_remove_button(state, agent_source, remove_aria_label, on_remove);

    view! {
        <span
            class=class
            lang=locale.lang
            dir=locale.dir
            data-slot="tag"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=state.state_attr
            data-enabled=state.is_enabled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-removable=state.is_removable.then_some("true")
            data-static=state.is_static.then_some("true")
            data-has-remove-handler=state.has_remove_handler.then_some("true")
            data-remove-label-source=state.remove_label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-capability-remove=move || {
                agent_contract.get().capabilities.can_remove.then_some("true")
            }
            data-ui-capability-disable=move || {
                agent_contract.get().capabilities.can_disable.then_some("true")
            }
        >
            {content}
            {remove_button}
        </span>
    }
}
