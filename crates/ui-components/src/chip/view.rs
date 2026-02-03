use crate::chip::{ChipSize, ChipVariant};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn Chip(
    #[prop(optional)] variant: ChipVariant,
    #[prop(optional)] size: ChipSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_dismiss: Option<OnPress>,
    #[prop(optional, into)] dismiss_aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let dismiss_aria_label = dismiss_aria_label
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Remove".to_string());

    let base_class = format!("ui-chip {} {}", variant.class_name(), size.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span class=class data-slot="chip" data-disabled=disabled.then_some("true")>
            <span data-slot="chip-content">{children()}</span>
            {on_dismiss.map(|on_dismiss| {
                view! {
                    <button
                        type="button"
                        class="ui-chip__dismiss"
                        aria-label=dismiss_aria_label.clone()
                        data-slot="chip-dismiss"
                        disabled=disabled
                        on:click=move |_| on_dismiss.run(())
                    >
                        "×"
                    </button>
                }
            })}
        </span>
    }
}
