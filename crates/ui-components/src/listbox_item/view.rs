use crate::listbox_item::{
    ListBoxItemStateInput,
    logic::{self, ListBoxItemSelectionIndicator},
};
use leptos::{children::Children, prelude::*};

#[component]
pub fn ListBoxItem(
    children: Children,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] selected: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] show_selection_indicator: bool,
    #[prop(optional)] has_divider: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] on_press: Option<Callback<()>>,
    #[prop(optional)] on_pointer_move: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let on_press = on_press.unwrap_or_else(|| Callback::new(|()| {}));
    let on_pointer_move = on_pointer_move.unwrap_or_else(|| Callback::new(|()| {}));

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::resolve_state(ListBoxItemStateInput {
            selected,
            focused,
            disabled,
            show_selection_indicator,
            has_divider,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let selection_indicator = logic::resolve_selection_indicator(show_selection_indicator);
    let indicator_text = move || selection_indicator.marker(state.get().is_selected);
    let index_text = index.map(|value| value.to_string());

    view! {
        <div
            class=move || class.get()
            id=id
            role="option"
            tabindex=if disabled { Some(-1) } else { Some(0) }
            aria-label=aria_label
            aria-selected=selected.then_some("true")
            aria-disabled=disabled.then_some("true")
            data-slot="listbox-item"
            data-index=index_text
            data-state=move || state.get().data_state_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || (!state.get().is_selected).then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-divider=move || state.get().has_divider.then_some("true")
            data-show-selection-indicator=move || {
                state.get().show_selection_indicator.then_some("true")
            }
            data-selection-indicator=move || state.get().selection_indicator_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            on:pointermove=move |_| {
                if disabled {
                    return;
                }
                on_pointer_move.run(());
            }
            on:click=move |_| {
                if disabled {
                    return;
                }
                on_press.run(());
            }
        >
            <span
                class="ui-listbox-item__indicator"
                data-slot="listbox-item-indicator"
                data-visible=move || {
                    indicator_text().is_some().then_some("true")
                }
            >
                {indicator_text}
            </span>

            <span class="ui-listbox-item__label" data-slot="listbox-item-label">
                {children()}
            </span>

            <Show when=move || selection_indicator != ListBoxItemSelectionIndicator::Hidden>
                <span class="ui-listbox-item__selection-sr" data-slot="listbox-item-selection-sr">
                    {move || if state.get().is_selected { "selected" } else { "not selected" }}
                </span>
            </Show>

            <Show when=move || state.get().has_divider>
                <span class="ui-listbox-item__divider" data-slot="listbox-item-divider"></span>
            </Show>
        </div>
    }
}
