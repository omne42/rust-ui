use super::{CheckboxMotion, CheckboxSize, CheckboxVariant, logic, motion};
use leptos::{html, prelude::*};
use ui_headless::{
    CheckboxOptions, FocusRingOptions, HoverOptions, OnPress, use_checkbox, use_focus_ring,
    use_hover,
};

#[component]
pub fn Checkbox(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<bool>>,
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional)] size: CheckboxSize,
    #[prop(optional)] motion: CheckboxMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    children: Children,
) -> impl IntoView {
    let toggle: OnPress = Callback::new(move |_| {
        let next = !checked.get_untracked();
        set_checked.set(next);
        if let Some(on_change) = on_change {
            on_change.run(next);
        }
    });

    let aria = use_checkbox(CheckboxOptions {
        is_disabled: disabled,
        is_checked: checked,
        on_press: Some(toggle),
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    motion::attach_root_motion(
        node_ref,
        hover.is_hovered,
        aria.is_pressed,
        disabled,
        motion,
    );

    let indicator_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_indicator_motion(indicator_ref, checked, motion);

    let state = Memo::new(move |_| {
        logic::resolve_state(
            checked.get(),
            disabled,
            aria.is_pressed.get(),
            hover.is_hovered.get(),
            focus_ring.is_focused.get(),
            focus_ring.is_focus_visible.get(),
        )
    });

    let base_class = format!("ui-checkbox {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <button
            type="button"
            node_ref=node_ref
            class=class
            class:ui-checkbox--focus-visible=move || state.get().is_focus_visible
            disabled=disabled
            data-slot="checkbox"
            data-state=move || state.get().data_state()
            data-checked=move || state.get().is_checked.then_some("true")
            data-unchecked=move || state.get().is_unchecked.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-hovered=move || state.get().is_hovered.then_some("true")
            data-pressed=move || state.get().is_pressed.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-focus-visible=move || state.get().is_focus_visible.then_some("true")
            data-motion-source=if motion == CheckboxMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != CheckboxMotion::default()).then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-checked=move || aria.attrs.aria_checked.get()
            aria-label=aria_label
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
        >
            <span class="ui-checkbox__box" data-slot="checkbox-box">
                <span node_ref=indicator_ref class="ui-checkbox__indicator" data-slot="checkbox-indicator">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke_width="3.5"
                        stroke="currentColor"
                    >
                        <path
                            stroke_linecap="round"
                            stroke_linejoin="round"
                            d="M4.5 12.75l6 6 9-13.5"
                        />
                    </svg>
                </span>
            </span>
            <span class="ui-checkbox__label" data-slot="checkbox-label">
                {children()}
            </span>
        </button>
    }
}

#[cfg(feature = "component-checkbox_group")]
#[component]
pub fn CheckboxGroup(
    id: String,
    label: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let ids = logic::resolve_checkbox_group_ids(&id);
    let legend_id = StoredValue::new(ids.legend_id);

    let label = StoredValue::new(logic::normalize_checkbox_group_label(label));
    let description = logic::normalize_checkbox_group_optional_text(description);
    let error = logic::normalize_checkbox_group_optional_text(error);

    let has_description = description.is_some();
    let has_error = error.is_some();

    let aria = logic::use_checkbox_group(logic::CheckboxGroupOptions {
        id: id.clone(),
        has_description,
        has_error,
        aria_describedby,
        is_invalid: invalid,
        is_required: required,
    });

    let state = Memo::new(move |_| {
        logic::resolve_checkbox_group_state(
            disabled,
            invalid.get(),
            required.get(),
            has_description,
            has_error,
        )
    });

    let base_class = "ui-checkbox-group".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <fieldset
            id=id
            class=class
            class:ui-checkbox-group--invalid=move || state.get().is_invalid
            class:ui-checkbox-group--required=move || state.get().is_required
            disabled=disabled
            aria-labelledby=legend_id.get_value()
            aria-describedby=move || aria.fieldset.aria_describedby.get()
            aria-invalid=move || aria.fieldset.aria_invalid.get()
            aria-required=move || aria.fieldset.aria_required.get()
            data-slot="checkbox-group"
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-enabled=move || state.get().is_enabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-valid=move || state.get().is_valid.then_some("true")
            data-required=move || state.get().is_required.then_some("true")
            data-optional=move || state.get().is_optional.then_some("true")
            data-has-description=move || state.get().has_description.then_some("true")
            data-has-error=move || state.get().has_error.then_some("true")
            data-shows-error=move || state.get().shows_error.then_some("true")
            data-has-messages=move || state.get().has_messages.then_some("true")
        >
            <legend
                class="ui-checkbox-group__label"
                id=legend_id.get_value()
                data-slot="checkbox-group-label"
            >
                {label.get_value()}
            </legend>

            <div class="ui-checkbox-group__list" data-slot="checkbox-group-list">
                {children()}
            </div>

            {description.map(|description| {
                let description_id = StoredValue::new(aria.description.id.clone());
                let description = StoredValue::new(description);
                view! {
                    <div
                        class="ui-checkbox-group__description"
                        id=description_id.get_value()
                        data-slot="checkbox-group-description"
                    >
                        {description.get_value()}
                    </div>
                }
            })}

            {error.map(|error| {
                let error_id = StoredValue::new(aria.error.id.clone());
                let error = StoredValue::new(error);
                view! {
                    <Show when=move || state.get().shows_error>
                        <div
                            class="ui-checkbox-group__error"
                            id=error_id.get_value()
                            data-slot="checkbox-group-error"
                        >
                            {error.get_value()}
                        </div>
                    </Show>
                }
            })}
        </fieldset>
    }
}
