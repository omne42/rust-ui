use crate::logic;
use crate::motion as motion_contract;
use leptos::prelude::*;

fn render_description_block(description: String, description_id: String) -> impl IntoView {
    let description = StoredValue::new(description);
    let description_id = StoredValue::new(description_id);

    view! {
        <div
            class="ui-checkbox-group__description"
            id=description_id.get_value()
            data-slot="checkbox-group-description"
        >
            {description.get_value()}
        </div>
    }
}

fn render_error_block(
    error: String,
    error_id: String,
    view_state: Signal<logic::CheckboxGroupViewState>,
) -> impl IntoView {
    let error = StoredValue::new(error);
    let error_id = StoredValue::new(error_id);

    view! {
        <Show when=move || view_state.get().shows_error>
            <div
                class="ui-checkbox-group__error"
                id=error_id.get_value()
                data-slot="checkbox-group-error"
            >
                {error.get_value()}
            </div>
        </Show>
    }
}

#[component]
pub fn CheckboxGroup(
    id: String,
    label: String,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional, into)] is_invalid: Signal<bool>,
    #[prop(optional, into)] is_required: Signal<bool>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] motion: motion_contract::CheckboxGroupMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<ui_headless::A11yDirection>,
    #[prop(optional, into)] aria_describedby: Signal<Option<String>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let ids = logic::resolve_checkbox_group_ids(&id);
    let legend_id = StoredValue::new(ids.legend_id);

    let content = logic::resolve_checkbox_group_content(label, description, error);
    let label = StoredValue::new(content.label);
    let description = content.description;
    let error = content.error;
    let has_description = content.has_description;
    let has_error = content.has_error;

    let headless = logic::use_checkbox_group(logic::CheckboxGroupOptions {
        id: id.clone(),
        is_disabled,
        has_description,
        has_error,
        aria_describedby,
        is_invalid,
        is_required,
        lang,
        dir,
    });
    let resolved_state = headless.state.resolved;
    let fieldset_aria_describedby = headless.attrs.fieldset.aria_describedby;
    let fieldset_aria_invalid = headless.attrs.fieldset.aria_invalid;
    let fieldset_aria_required = headless.attrs.fieldset.aria_required;
    let fieldset_lang = headless.attrs.fieldset.lang;
    let fieldset_dir = headless.attrs.fieldset.dir;
    let description_attrs = headless.attrs.description;
    let error_attrs = headless.attrs.error;

    let class = logic::resolve_checkbox_group_class_name(class_name);
    let style = motion_contract::attach_motion(None, motion);
    let motion_source = motion_contract::motion_source_attr(motion);
    let view_state =
        Signal::derive(move || logic::resolve_checkbox_group_view_state(resolved_state.get()));
    let agent_contract = Memo::new(move |_| {
        let state = view_state.get();
        logic::resolve_checkbox_group_agent_contract(logic::CheckboxGroupAgentContractInput {
            is_disabled: state.is_disabled,
            is_invalid: state.is_invalid,
            shows_error: state.shows_error,
            state_source: state.state_source,
        })
    });
    let description_view = description
        .map(|description| render_description_block(description, description_attrs.id.clone()));
    let error_view =
        error.map(|error| render_error_block(error, error_attrs.id.clone(), view_state));

    view! {
        <fieldset
            id=id
            class=class
            style=style
            class:ui-checkbox-group--invalid=move || view_state.get().is_invalid
            class:ui-checkbox-group--required=move || view_state.get().is_required
            disabled=is_disabled
            aria-labelledby=legend_id.get_value()
            aria-describedby=move || fieldset_aria_describedby.get()
            aria-invalid=move || fieldset_aria_invalid.get()
            aria-required=move || fieldset_aria_required.get()
            lang=fieldset_lang
            dir=fieldset_dir
            data-slot="checkbox-group"
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_data_attr()
            data-ui-intent=move || agent_contract.get().intent.as_data_attr()
            data-ui-action=move || agent_contract.get().action.as_data_attr()
            data-ui-state=move || agent_contract.get().state.as_data_attr()
            data-ui-source=move || agent_contract.get().source.as_data_attr()
            data-ui-state-source=move || agent_contract.get().state_source.as_data_attr()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_data_attr()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_data_attr()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_data_attr()
            data-ui-output-status=move || agent_contract.get().output_status.as_data_attr()
            data-disabled=move || view_state.get().is_disabled.then_some("true")
            data-enabled=move || view_state.get().is_enabled.then_some("true")
            data-invalid=move || view_state.get().is_invalid.then_some("true")
            data-valid=move || view_state.get().is_valid.then_some("true")
            data-required=move || view_state.get().is_required.then_some("true")
            data-optional=move || view_state.get().is_optional.then_some("true")
            data-has-description=move || view_state.get().has_description.then_some("true")
            data-has-error=move || view_state.get().has_error.then_some("true")
            data-shows-error=move || view_state.get().shows_error.then_some("true")
            data-has-messages=move || view_state.get().has_messages.then_some("true")
            data-state-source=move || view_state.get().state_source.as_data_attr()
            data-motion-source=motion_source
            data-motion-phase=move || view_state.get().motion_phase.as_data_attr()
        >
            <legend
                class="ui-checkbox-group__label"
                id=legend_id.get_value()
                data-slot="checkbox-group-label"
            >
                {label.get_value()}
            </legend>

            <div class="ui-checkbox-group__list" data-slot="checkbox-group-list">
                {children()}
            </div>

            {description_view}
            {error_view}
        </fieldset>
    }
}
