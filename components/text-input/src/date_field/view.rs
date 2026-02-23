use crate::clear_button::ClearButton;
use crate::text_input::date_field::{
    DateFieldMotion, DateFieldStateInput, DateFieldStrings,
    logic::{self, DateFieldTone},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{self as headless, A11yDirection, DateFieldOptions, i18n, use_date_field};

#[component]
pub fn DateField(
    id_base: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] tone: DateFieldTone,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] value: Option<Signal<Option<String>>>,
    #[prop(optional)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] year_aria_label: Option<String>,
    #[prop(optional, into)] month_aria_label: Option<String>,
    #[prop(optional, into)] day_aria_label: Option<String>,
    #[prop(optional, into)] clear_label: Option<String>,
    #[prop(optional, into)] clear_aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: DateFieldMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<DateFieldStrings>();

    let is_controlled = value.is_some();
    let has_value_change_handler = on_value_change.is_some();
    let default_value = logic::resolve_default_value(default_value);
    let has_default_value = default_value.is_some();

    let control_mode = logic::DateFieldControlMode::from_is_controlled(is_controlled);
    let value_source =
        logic::DateFieldValueSource::from_control_mode(control_mode, has_default_value);
    let value_change_source =
        logic::DateFieldValueChangeSource::from_has_handler(has_value_change_handler);
    let (interaction_source, set_interaction_source) =
        signal(logic::DateFieldInteractionSource::Programmatic);

    let value_state = headless::use_controllable_state(value, Some(default_value), on_value_change);
    let value = value_state.value;
    let request_value_change = value_state.request_change;

    let (label, has_custom_label) = logic::normalize_label(label, strings.label.as_ref());
    let label = StoredValue::new(label);

    let (placeholder, has_custom_placeholder) =
        logic::normalize_placeholder(placeholder, strings.placeholder.as_ref());
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, strings.aria_label.as_ref());
    let (year_aria_label, _) =
        logic::normalize_year_aria_label(year_aria_label, strings.year_aria_label.as_ref());
    let (month_aria_label, _) =
        logic::normalize_month_aria_label(month_aria_label, strings.month_aria_label.as_ref());
    let (day_aria_label, _) =
        logic::normalize_day_aria_label(day_aria_label, strings.day_aria_label.as_ref());
    let (clear_label, _) = logic::normalize_clear_label(clear_label, strings.clear_label.as_ref());
    let clear_label = StoredValue::new(clear_label);
    let (clear_aria_label, _) =
        logic::normalize_clear_aria_label(clear_aria_label, strings.clear_aria_label.as_ref());

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != DateFieldMotion::default();

    let ids = logic::resolve_ids(&id_base);
    let root_id = ids.root_id;
    let label_id = ids.label_id;
    let year_id = ids.year_id;
    let month_id = ids.month_id;
    let day_id = ids.day_id;

    let value_for_year = value;
    let request_value_change_for_year = request_value_change;
    let set_interaction_source_for_year = set_interaction_source;
    let on_year_input = Callback::new(move |year_input: String| {
        set_interaction_source_for_year.set(logic::DateFieldInteractionSource::YearInput);
        let next = logic::update_year_from_input(value_for_year.get_untracked(), &year_input);
        request_value_change_for_year.run(next);
    });
    let value_for_month = value;
    let request_value_change_for_month = request_value_change;
    let set_interaction_source_for_month = set_interaction_source;
    let on_month_input = Callback::new(move |month_input: String| {
        set_interaction_source_for_month.set(logic::DateFieldInteractionSource::MonthInput);
        let next = logic::update_month_from_input(value_for_month.get_untracked(), &month_input);
        request_value_change_for_month.run(next);
    });
    let value_for_day = value;
    let request_value_change_for_day = request_value_change;
    let set_interaction_source_for_day = set_interaction_source;
    let on_day_input = Callback::new(move |day_input: String| {
        set_interaction_source_for_day.set(logic::DateFieldInteractionSource::DayInput);
        let next = logic::update_day_from_input(value_for_day.get_untracked(), &day_input);
        request_value_change_for_day.run(next);
    });
    let request_value_change_for_clear = request_value_change;
    let set_interaction_source_for_clear = set_interaction_source;
    let on_clear = Callback::new(move |_| {
        set_interaction_source_for_clear.set(logic::DateFieldInteractionSource::ClearButton);
        request_value_change_for_clear.run(None);
    });

    let date_field = use_date_field(DateFieldOptions {
        is_disabled,
        value,
        resolve_parts: Callback::new(logic::resolve_date_parts),
        on_year_input,
        on_month_input,
        on_day_input,
        on_clear,
        aria_label,
        aria_labelledby: (!has_custom_aria_label).then(|| label_id.clone()),
        lang,
        dir,
        year_aria_label,
        month_aria_label,
        day_aria_label,
        clear_aria_label,
    });

    let group_role = date_field.attrs.role;
    let group_aria_label = StoredValue::new(date_field.attrs.aria_label.clone());
    let group_aria_labelledby = StoredValue::new(date_field.attrs.aria_labelledby.clone());
    let group_lang = StoredValue::new(date_field.attrs.lang.clone());
    let group_dir = date_field.attrs.dir;
    let year_aria_label = StoredValue::new(date_field.attrs.year_aria_label.clone());
    let month_aria_label = StoredValue::new(date_field.attrs.month_aria_label.clone());
    let day_aria_label = StoredValue::new(date_field.attrs.day_aria_label.clone());
    let clear_aria_label = StoredValue::new(date_field.attrs.clear_aria_label.clone());
    let on_year_input_handler = date_field.handlers.on_year_input;
    let on_month_input_handler = date_field.handlers.on_month_input;
    let on_day_input_handler = date_field.handlers.on_day_input;
    let on_clear_handler = date_field.handlers.on_clear;
    let parts = date_field.state.parts;
    let has_date_value = date_field.state.has_value;

    let state = Memo::new(move |_| {
        logic::resolve_state(DateFieldStateInput {
            tone,
            disabled: is_disabled,
            has_value: has_date_value.get(),
            has_custom_label,
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let has_value = Signal::derive(move || has_date_value.get());

    let input_placeholders = logic::resolve_input_placeholders(&placeholder.get_value());
    let year_placeholder = StoredValue::new(input_placeholders.0);
    let month_placeholder = StoredValue::new(input_placeholders.1);
    let day_placeholder = StoredValue::new(input_placeholders.2);

    let year_value = Memo::new(move |_| {
        let (year, _, _, has_value) = parts.get();
        if has_value {
            format!("{year:04}")
        } else {
            String::new()
        }
    });

    let month_value = Memo::new(move |_| {
        let (_, month, _, has_value) = parts.get();
        if has_value {
            format!("{month:02}")
        } else {
            String::new()
        }
    });

    let day_value = Memo::new(move |_| {
        let (_, _, day, has_value) = parts.get();
        if has_value {
            format!("{day:02}")
        } else {
            String::new()
        }
    });

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, has_value, motion);

    view! {
        <div
            node_ref=root_ref
            id=root_id
            class=move || class.get()
            data-slot="date-field"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-control-mode=control_mode.as_attr()
            data-value-source=value_source.as_attr()
            data-default-value-source=if has_default_value { "custom" } else { "none" }
            data-value-change-source=value_change_source.as_attr()
            data-interaction-source=move || interaction_source.get().as_attr()
            role=group_role
            aria-label=group_aria_label.get_value()
            aria-labelledby=move || group_aria_labelledby.get_value()
            lang=move || group_lang.get_value()
            dir=group_dir
        >
            <label id=label_id.clone() class="ui-date-field__label" data-slot="date-field-label" for=year_id.clone()>
                {label.get_value()}
            </label>

            <div class="ui-date-field__control" data-slot="date-field-control">
                <input
                    id=year_id.clone()
                    class="ui-date-field__input ui-date-field__input--year"
                    data-slot="date-field-year"
                    type="number"
                    min="1"
                    max="9999"
                    step="1"
                    placeholder=year_placeholder.get_value()
                    prop:value=move || year_value.get()
                    disabled=is_disabled
                    aria-label=year_aria_label.get_value()
                    on:input=move |ev| on_year_input_handler.run(event_target_value(&ev))
                />

                <span class="ui-date-field__separator" data-slot="date-field-separator" aria-hidden="true">
                    "-"
                </span>

                <input
                    id=month_id.clone()
                    class="ui-date-field__input ui-date-field__input--month"
                    data-slot="date-field-month"
                    type="number"
                    min="1"
                    max="12"
                    step="1"
                    placeholder=month_placeholder.get_value()
                    prop:value=move || month_value.get()
                    disabled=is_disabled
                    aria-label=month_aria_label.get_value()
                    on:input=move |ev| on_month_input_handler.run(event_target_value(&ev))
                />

                <span class="ui-date-field__separator" data-slot="date-field-separator" aria-hidden="true">
                    "-"
                </span>

                <input
                    id=day_id
                    class="ui-date-field__input ui-date-field__input--day"
                    data-slot="date-field-day"
                    type="number"
                    min="1"
                    max="31"
                    step="1"
                    placeholder=day_placeholder.get_value()
                    prop:value=move || day_value.get()
                    disabled=is_disabled
                    aria-label=day_aria_label.get_value()
                    on:input=move |ev| on_day_input_handler.run(event_target_value(&ev))
                />

                <Show when=move || has_value.get()>
                    <ClearButton
                        slot_name="date-field-clear"
                        class_name="ui-date-field__clear".to_string()
                        disabled=is_disabled
                        aria_label=clear_aria_label.get_value()
                        on_press=on_clear_handler
                    >
                        {clear_label.get_value()}
                    </ClearButton>
                </Show>
            </div>
        </div>
    }
}
