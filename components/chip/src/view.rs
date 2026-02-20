use crate::{
    ChipSize, ChipVariant,
    logic::{self, ChipStateInput},
};
use leptos::prelude::*;
use ui_headless::OnPress;

#[component]
pub fn Chip(
    #[prop(optional)] variant: ChipVariant,
    #[prop(optional)] size: ChipSize,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] on_dismiss: Option<OnPress>,
    #[prop(optional, into)] dismiss_aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (dismiss_aria_label, has_custom_dismiss_aria_label) =
        logic::resolve_dismiss_aria_label(dismiss_aria_label);

    let state = logic::resolve_state(ChipStateInput {
        variant,
        size,
        disabled: is_disabled,
        has_dismiss_action: on_dismiss.is_some(),
        has_custom_dismiss_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let dismiss_aria_label = StoredValue::new(dismiss_aria_label);
    let on_dismiss = StoredValue::new(on_dismiss);

    view! {
        <span
            class=class
            data-slot="chip"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=state.state_attr
            data-enabled=state.is_enabled.then_some("true")
            data-disabled=state.is_disabled.then_some("true")
            data-removable=state.has_dismiss_action.then_some("true")
            data-static=state.is_static.then_some("true")
            data-dismiss-label=state.dismiss_label_source_attr
            data-dismiss-label-source=state.dismiss_label_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
        >
            <span class="ui-chip__content" data-slot="chip-content">
                {children()}
            </span>

            <Show when=move || state.has_dismiss_action>
                <button
                    type="button"
                    class="ui-chip__dismiss"
                    aria-label=move || dismiss_aria_label.get_value()
                    data-slot="chip-dismiss"
                    data-disabled=state.is_disabled.then_some("true")
                    data-label-source=state.dismiss_label_source_attr
                    disabled=state.is_disabled
                    on:click=move |_| {
                        if state.is_enabled
                            && let Some(on_dismiss) = on_dismiss.get_value()
                        {
                            on_dismiss.run(());
                        }
                    }
                >
                    "×"
                </button>
            </Show>
        </span>
    }
}
