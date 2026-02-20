use super::{
    FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldStateInput, FormFieldTone,
    logic,
};
use crate::checkbox::{Checkbox, CheckboxVariant};
use crate::switch::Switch;
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn FormField(
    selected: ReadSignal<bool>,
    set_selected: WriteSignal<bool>,
    #[prop(optional)] on_selected_change: Option<Callback<bool>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] tone: FormFieldTone,
    #[prop(optional)] indicator_variant: FormFieldIndicatorVariant,
    #[prop(optional)] indicator_placement: FormFieldIndicatorPlacement,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error_message: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = StoredValue::new(logic::normalize_id_base(id_base));

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let (error_message, has_custom_error_message) =
        logic::normalize_error_message(error_message, invalid);
    let has_error_message = error_message.is_some();
    let error_message = StoredValue::new(error_message);

    let (control_aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let control_aria_label = StoredValue::new(control_aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let on_selected_change =
        StoredValue::new(on_selected_change.unwrap_or_else(|| Callback::new(|_| {})));

    let state = Memo::new(move |_| {
        logic::resolve_state(FormFieldStateInput {
            selected: selected.get(),
            disabled,
            invalid,
            tone,
            indicator_variant,
            indicator_placement,
            has_description,
            has_error_message,
            has_custom_label,
            has_custom_aria_label,
            has_custom_error_message,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));
    let error_id = Memo::new(move |_| format!("{}-error", id_base.get_value()));

    let describedby = Signal::derive(move || {
        let state = state.get();
        let mut ids = Vec::new();

        if state.has_description {
            ids.push(description_id.get());
        }

        if state.shows_error {
            ids.push(error_id.get());
        }

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    });

    let control_class = StoredValue::new("ui-form-field__control".to_string());
    let locale = locale_attrs(lang, dir);

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="form-field"
            data-state=move || state.get().state_attr
            data-tone=move || state.get().tone_attr
            data-indicator-variant=move || state.get().indicator_variant_attr
            data-indicator-placement=move || state.get().indicator_placement_attr
            data-selected=move || state.get().is_selected.then_some("true")
            data-unselected=move || state.get().is_unselected.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-message-kind=move || state.get().message_kind_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-error-source=move || state.get().error_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=move || control_aria_label.get_value()
            aria-describedby=move || describedby.get()
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <Show when=move || state.get().indicator_placement == FormFieldIndicatorPlacement::Start>
                <div class="ui-form-field__indicator" data-slot="form-field-indicator">
                    {match indicator_variant {
                        FormFieldIndicatorVariant::Switch => {
                            let on_selected_change = on_selected_change.get_value();
                            view! {
                                <Switch
                                    checked=selected
                                    set_checked=set_selected
                                    on_checked_change=on_selected_change
                                    disabled=disabled
                                    aria_label=control_aria_label.get_value()
                                    class_name=control_class.get_value()
                                >
                                    "toggle"
                                </Switch>
                            }
                            .into_any()
                        }
                        FormFieldIndicatorVariant::Checkbox => {
                            let checkbox_variant = if invalid {
                                CheckboxVariant::Accent
                            } else {
                                CheckboxVariant::Default
                            };

                            let on_selected_change = on_selected_change.get_value();
                            view! {
                                <Checkbox
                                    checked=selected
                                    set_checked=set_selected
                                    on_change=on_selected_change
                                    disabled=disabled
                                    variant=checkbox_variant
                                    aria_label=control_aria_label.get_value()
                                    class_name=control_class.get_value()
                                >
                                    "toggle"
                                </Checkbox>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Show>

            <div class="ui-form-field__content" data-slot="form-field-content">
                <p class="ui-form-field__label" data-slot="form-field-label">
                    {move || label.get_value()}
                </p>

                <Show when=move || state.get().message_kind_attr == "description">
                    <p
                        id=move || description_id.get()
                        class="ui-form-field__description"
                        data-slot="form-field-description"
                    >
                        {move || description.get_value().unwrap_or_default()}
                    </p>
                </Show>

                <Show when=move || state.get().message_kind_attr == "error">
                    <p
                        id=move || error_id.get()
                        class="ui-form-field__error"
                        data-slot="form-field-error"
                        role="alert"
                    >
                        {move || error_message.get_value().unwrap_or_default()}
                    </p>
                </Show>
            </div>

            <Show when=move || state.get().indicator_placement == FormFieldIndicatorPlacement::End>
                <div class="ui-form-field__indicator" data-slot="form-field-indicator">
                    {match indicator_variant {
                        FormFieldIndicatorVariant::Switch => {
                            let on_selected_change = on_selected_change.get_value();
                            view! {
                                <Switch
                                    checked=selected
                                    set_checked=set_selected
                                    on_checked_change=on_selected_change
                                    disabled=disabled
                                    aria_label=control_aria_label.get_value()
                                    class_name=control_class.get_value()
                                >
                                    "toggle"
                                </Switch>
                            }
                            .into_any()
                        }
                        FormFieldIndicatorVariant::Checkbox => {
                            let checkbox_variant = if invalid {
                                CheckboxVariant::Accent
                            } else {
                                CheckboxVariant::Default
                            };

                            let on_selected_change = on_selected_change.get_value();
                            view! {
                                <Checkbox
                                    checked=selected
                                    set_checked=set_selected
                                    on_change=on_selected_change
                                    disabled=disabled
                                    variant=checkbox_variant
                                    aria_label=control_aria_label.get_value()
                                    class_name=control_class.get_value()
                                >
                                    "toggle"
                                </Checkbox>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
