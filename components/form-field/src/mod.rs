pub mod checkbox {
    pub use ui_checkbox::{Checkbox, CheckboxVariant};
}

pub mod switch {
    use leptos::ev;
    use leptos::prelude::*;

    #[component]
    pub fn Switch(
        checked: ReadSignal<bool>,
        set_checked: WriteSignal<bool>,
        #[prop(optional)] on_checked_change: Option<Callback<bool>>,
        #[prop(optional)] disabled: bool,
        #[prop(optional, into)] aria_label: Option<String>,
        #[prop(optional, into)] class_name: Option<String>,
        children: Children,
    ) -> impl IntoView {
        let on_checked_change =
            StoredValue::new(on_checked_change.unwrap_or_else(|| Callback::new(|_| {})));

        let class_name = class_name
            .filter(|value| !value.trim().is_empty())
            .map(|value| format!("ui-switch {value}"))
            .unwrap_or_else(|| "ui-switch".to_string());

        view! {
            <label class=class_name data-slot="switch">
                <input
                    type="checkbox"
                    checked=move || checked.get()
                    disabled=disabled
                    aria-label=aria_label
                    on:change=move |ev: ev::Event| {
                        let next = event_target_checked(&ev);
                        set_checked.set(next);
                        on_checked_change.get_value().run(next);
                    }
                />
                <span class="ui-switch__label">{children()}</span>
            </label>
        }
    }
}

pub(crate) mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ERROR_MESSAGE, DEFAULT_LABEL, FormFieldIndicatorPlacement,
    FormFieldIndicatorVariant, FormFieldTone,
};
pub use view::FormField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormFieldStateInput {
    pub selected: bool,
    pub disabled: bool,
    pub invalid: bool,
    pub tone: FormFieldTone,
    pub indicator_variant: FormFieldIndicatorVariant,
    pub indicator_placement: FormFieldIndicatorPlacement,
    pub has_description: bool,
    pub has_error_message: bool,
    pub has_custom_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_error_message: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormFieldState {
    pub is_selected: bool,
    pub is_unselected: bool,
    pub is_disabled: bool,
    pub is_invalid: bool,
    pub tone: FormFieldTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub indicator_variant: FormFieldIndicatorVariant,
    pub indicator_variant_class: &'static str,
    pub indicator_variant_attr: &'static str,
    pub indicator_placement: FormFieldIndicatorPlacement,
    pub indicator_placement_class: &'static str,
    pub indicator_placement_attr: &'static str,
    pub has_description: bool,
    pub has_error_message: bool,
    pub shows_error: bool,
    pub message_kind_attr: &'static str,
    pub state_attr: &'static str,
    pub label_source_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub error_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
