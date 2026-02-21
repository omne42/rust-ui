use crate::clear_button::ClearButton;
use crate::color::field::logic::{self};
use crate::color::swatch::ColorSwatch;
use leptos::prelude::*;
use ui_headless::{
    A11yDirection, CommonStrings, locale_attrs, use_controllable_state, use_ui_i18n,
};

#[component]
pub fn ColorField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional)] is_preview_visible: Option<bool>,
    #[prop(optional)] show_preview: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);
    let is_preview_visible = logic::resolve_is_preview_visible(is_preview_visible, show_preview);

    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let clear_label: String = common.clear_aria_label.as_ref().into();
    let clear_label = StoredValue::new(clear_label);

    let default_value = logic::normalize_color_value(default_value);
    let value_state = use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (placeholder, has_custom_placeholder) = logic::normalize_placeholder(placeholder);
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let preview_color = Memo::new(move |_| logic::resolve_preview_color(value.get()));

    let state = Memo::new(move |_| {
        logic::resolve_derived_state(logic::ColorFieldDerivedStateInput {
            is_disabled,
            is_preview_visible,
            value: value.get(),
            preview_color: preview_color.get(),
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let label_id = format!("{id_base}-label");
    let input_id = format!("{id_base}-input");

    let on_input = move |ev| {
        if is_disabled {
            return;
        }

        let next = logic::resolve_next_value(event_target_value(&ev));
        request_value_change.run(next);
    };

    let on_clear = Callback::new(move |_: ()| {
        if is_disabled {
            return;
        }

        request_value_change.run(None);
    });

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot="color-field"
            data-state=move || state.get().visual_state.as_attr()
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-valid=move || state.get().has_valid_value.then_some("true")
            data-invalid=move || logic::is_invalid_state(state.get()).then_some("true")
            data-has-preview=move || state.get().has_preview.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id.clone()
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <label id=label_id.clone() class="ui-color-field__label" data-slot="color-field-label" for=input_id.clone()>
                {label.get_value()}
            </label>

            <div class="ui-color-field__control" data-slot="color-field-control">
                <Show when=move || is_preview_visible>
                    {move || {
                        let swatch = if let Some(color) = preview_color.get() {
                            view! { <ColorSwatch color=color is_decorative=true /> }.into_any()
                        } else {
                            view! { <ColorSwatch is_decorative=true /> }.into_any()
                        };

                        view! {
                            <span class="ui-color-field__preview" data-slot="color-field-preview" aria-hidden="true">
                                {swatch}
                            </span>
                        }
                    }}
                </Show>

                <input
                    id=input_id
                    class="ui-color-field__input"
                    data-slot="color-field-input"
                    type="text"
                    spellcheck="false"
                    disabled=is_disabled
                    aria-invalid=move || logic::is_invalid_state(state.get()).then_some("true")
                    placeholder=placeholder.get_value()
                    prop:value=move || logic::resolve_input_value(value.get())
                    on:input=on_input
                />

                <Show when=move || state.get().has_value>
                    <ClearButton
                        slot_name="color-field-clear"
                        class_name="ui-color-field__clear".to_string()
                        disabled=is_disabled
                        aria_label=clear_label.get_value()
                        on_press=on_clear
                    >
                        {clear_label.get_value()}
                    </ClearButton>
                </Show>
            </div>
        </div>
    }
}
