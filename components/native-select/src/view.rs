use crate::{
    NativeSelectOption,
    logic::{self, NativeSelectSize, NativeSelectStateParams},
};
use leptos::{ev, prelude::*};
use ui_headless::{
    self as overlay_open, A11yDirection, NativeSelectOptions, resolve_native_select_change_index,
    use_native_select,
};

const NATIVE_SELECT_INDICATOR_SYMBOL: &str = "▾";

fn render_placeholder_option(placeholder_label: String, is_required: bool) -> impl IntoView {
    view! {
        <option value="" disabled=is_required>
            {placeholder_label}
        </option>
    }
}

fn render_native_select_option(option: crate::NativeSelectOptionResolved) -> impl IntoView {
    view! {
        <option id=option.id value=option.value disabled=option.disabled>
            {option.label}
        </option>
    }
}

fn render_static_indicator() -> impl IntoView {
    view! {
        <span
            class="ui-native-select__indicator"
            data-slot="native-select-indicator"
            aria-hidden="true"
        >
            {NATIVE_SELECT_INDICATOR_SYMBOL}
        </span>
    }
}

#[component]
pub fn NativeSelect(
    id_base: String,
    options: Vec<NativeSelectOption>,
    #[prop(optional)] selected_index: Option<Signal<Option<usize>>>,
    #[prop(optional)] default_selected_index: Option<usize>,
    #[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] size: NativeSelectSize,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_base = StoredValue::new(id_base);
    let options = StoredValue::new(logic::normalize_options(options));
    let is_controlled = selected_index.is_some();
    let has_default_selected_index = default_selected_index.is_some();
    let selection_source_attr = if is_controlled {
        "external"
    } else if has_default_selected_index {
        "default"
    } else {
        "internal"
    };
    let default_selected_index = logic::normalize_default_selected_index(default_selected_index);

    let selected_state = overlay_open::use_controllable_state(
        selected_index,
        default_selected_index,
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
    let lang = StoredValue::new(logic::normalize_optional_text(lang));

    let resolved_options =
        Signal::derive(move || logic::resolve_options(&id_base.get_value(), &options.get_value()));

    let (selection_change_source_attr, set_selection_change_source_attr) = signal("initial");
    let (pending_user_change, set_pending_user_change) = signal(false);
    let (last_selected_index, set_last_selected_index) = signal(selected_index.get_untracked());

    Effect::new(move |_| {
        let current_selected_index = selected_index.get();
        let previous_selected_index = last_selected_index.get_untracked();

        if current_selected_index != previous_selected_index {
            if pending_user_change.get_untracked() {
                set_selection_change_source_attr.set("user");
                set_pending_user_change.set(false);
            } else if is_controlled {
                set_selection_change_source_attr.set("external");
            } else {
                set_selection_change_source_attr.set("internal");
            }
            set_last_selected_index.set(current_selected_index);
        }
    });

    Effect::new(move |_| {
        if let Some(next_selected_index) =
            logic::resolve_selected_index_correction(selected_index.get(), &resolved_options.get())
        {
            set_selection_change_source_attr.set("sync-effect");
            request_selected_index_change.run(next_selected_index);
        }
    });

    let resolved_states = Signal::derive(move || {
        logic::resolve_states_for_render(NativeSelectStateParams {
            size,
            is_disabled,
            is_invalid,
            is_required,
            has_placeholder,
            selected_index: selected_index.get(),
            options: &resolved_options.get(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });
    let state = Signal::derive(move || resolved_states.get().component);
    let semantics = Signal::derive(move || {
        let primitive_state = resolved_states.get().primitive;
        use_native_select(NativeSelectOptions {
            state: primitive_state,
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir,
        })
    });
    let agent_contract = Signal::derive(move || {
        let current_state = state.get();
        logic::resolve_agent_contract(logic::NativeSelectAgentContractInput {
            state: &current_state,
            selection_source_attr,
            change_source_attr: selection_change_source_attr.get(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), &state.get()));

    let selected_value =
        Signal::derive(move || logic::resolve_control_value(state.get().selected_value.as_deref()));
    let output_status = Signal::derive(move || logic::resolve_output_status(&state.get()));

    let on_change = move |ev: ev::Event| {
        let next_value = event_target_value(&ev);
        let next_index =
            resolve_native_select_change_index(&next_value, &resolved_options.get_untracked());

        set_pending_user_change.set(true);
        set_selection_change_source_attr.set("user");
        request_selected_index_change.run(next_index);
    };

    let render_placeholder = move || {
        // Contract marker for docs/tests: {placeholder}
        placeholder
            .get_value()
            .map(|placeholder_label| render_placeholder_option(placeholder_label, is_required))
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
            data-selection-mode=if is_controlled {
                "controlled"
            } else {
                "uncontrolled"
            }
            data-selection-source=if is_controlled {
                "external"
            } else if has_default_selected_index {
                "default"
            } else {
                "internal"
            }
            data-change-source=move || selection_change_source_attr.get()
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version
            data-ui-intent=move || agent_contract.get().intent.as_attr()
            data-ui-action=move || agent_contract.get().action.as_attr()
            data-ui-state=move || agent_contract.get().state.as_attr()
            data-ui-source=move || agent_contract.get().source.as_attr()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()
            data-streaming-mode="optional"
            data-streaming-fallback="snapshot"
            data-output-status=move || output_status.get().as_attr()
        >
            <select
                id=move || format!("{}-control", id_base.get_value())
                class="ui-native-select__control"
                data-slot=move || semantics.get().attrs.data_slot
                data-aria-source=move || semantics.get().attrs.data_aria_source
                name=name.get_value()
                aria-label=move || semantics.get().attrs.aria_label
                disabled=move || semantics.get().attrs.disabled
                required=move || semantics.get().attrs.required
                aria-invalid=move || semantics.get().attrs.aria_invalid
                lang=move || semantics.get().attrs.lang
                dir=move || semantics.get().attrs.dir
                on:change=on_change
                prop:value=move || selected_value.get()
            >
                {render_placeholder}

                <For
                    each=move || resolved_options.get()
                    key=|option| option.id.clone()
                    children=render_native_select_option
                />
            </select>

            {render_static_indicator()}
        </div>
    }
}
