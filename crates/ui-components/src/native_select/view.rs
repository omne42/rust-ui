use crate::native_select::{
    NativeSelectOption, NativeSelectStateInput,
    logic::{self, NativeSelectSize},
};
use leptos::{ev, prelude::*};
use ui_headless as overlay_open;

#[component]
pub fn NativeSelect(
    id_base: String,
    options: Vec<NativeSelectOption>,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] size: NativeSelectSize,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = StoredValue::new(id_base);
    let options = StoredValue::new(logic::normalize_options(options));

    let selected_state = overlay_open::use_controllable_state(
        selected_index,
        default_selected_index.map(Some),
        on_selected_index_change,
    );
    let selected_index = selected_state.value;
    let request_selected_index_change = selected_state.request_change;

    let placeholder = logic::normalize_placeholder(placeholder);
    let has_placeholder = placeholder.is_some();
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let name = StoredValue::new(logic::normalize_optional_text(name));

    let resolved_options =
        Signal::derive(move || logic::resolve_options(&id_base.get_value(), &options.get_value()));

    Effect::new(move |_| {
        let sanitized =
            logic::sanitize_selected_index(selected_index.get(), &resolved_options.get());
        if sanitized != selected_index.get_untracked() {
            request_selected_index_change.run(sanitized);
        }
    });

    let state = Signal::derive(move || {
        logic::resolve_state(
            NativeSelectStateInput {
                disabled,
                invalid,
                required,
                has_placeholder,
                selected_index: selected_index.get(),
                options: &resolved_options.get(),
                has_custom_aria_label,
                has_custom_class_name,
            },
            size,
        )
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), &state.get()));

    let selected_value = Signal::derive(move || state.get().selected_value.unwrap_or_default());

    let on_change = move |ev: ev::Event| {
        let next_value = event_target_value(&ev);
        let next_index = if next_value.is_empty() {
            None
        } else {
            logic::find_index_by_value(&next_value, &resolved_options.get_untracked())
        };

        request_selected_index_change.run(next_index);
    };

    view! {
        <div
            id=move || format!("{}-root", id_base.get_value())
            class=move || class.get()
            data-slot="native-select"
            data-state=move || state.get().data_state_attr
            data-size=move || state.get().size_attr
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-options=move || state.get().has_options.then_some("true")
            data-option-count=move || state.get().option_count.to_string()
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-selected-value=move || state.get().selected_value.clone()
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-disabled=move || state.get().control_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-required=move || state.get().is_required.then_some("true")
            data-has-placeholder=move || state.get().has_placeholder.then_some("true")
            data-has-disabled-options=move || state.get().has_disabled_options.then_some("true")
            data-has-enabled-options=move || state.get().has_enabled_options.then_some("true")
            data-disabled-option-count=move || state.get().disabled_option_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-class-source=move || state.get().class_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
        >
            <select
                id=move || format!("{}-control", id_base.get_value())
                class="ui-native-select__control"
                data-slot="native-select-control"
                name=name.get_value()
                aria-label=aria_label.get_value()
                disabled=move || state.get().control_disabled
                required=required
                aria-invalid=move || state.get().is_invalid.then_some("true")
                on:change=on_change
                prop:value=move || selected_value.get()
            >
                {move || {
                    placeholder
                        .get_value()
                        .map(|placeholder| {
                            view! {
                                <option value="" disabled=required>
                                    {placeholder}
                                </option>
                            }
                        })
                }}

                <For
                    each=move || resolved_options.get()
                    key=|option| option.id.clone()
                    children=move |option| {
                        view! {
                            <option id=option.id value=option.value disabled=option.disabled>
                                {option.label}
                            </option>
                        }
                    }
                />
            </select>

            <span
                class="ui-native-select__indicator"
                data-slot="native-select-indicator"
                aria-hidden="true"
            >
                "▾"
            </span>
        </div>
    }
}
