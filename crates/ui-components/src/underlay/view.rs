use crate::{OnPress, underlay::UnderlayStateInput};
use leptos::prelude::*;

use crate::underlay::logic;

#[component]
pub fn Underlay(
    id_base: String,
    open: Signal<bool>,
    #[prop(optional)] transparent: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_close: Option<OnPress>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let on_close = StoredValue::new(on_close);
    let has_on_close = on_close.get_value().is_some();

    let state = Memo::new(move |_| {
        logic::resolve_state(UnderlayStateInput {
            open: open.get(),
            transparent,
            disabled,
            has_on_close,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let on_click = move |_| {
        let state = state.get();
        if !state.is_interactive {
            return;
        }

        if let Some(on_close) = on_close.get_value() {
            on_close.run(());
        }
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="presentation"
            aria-hidden="true"
            tabindex=-1
            on:click=on_click
            data-slot="underlay"
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-transparent=move || state.get().is_transparent.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-interactive=move || state.get().is_interactive.then_some("true")
            data-tone=move || state.get().tone_attr
            data-close-mode=move || state.get().close_mode_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        ></div>
    }
}
