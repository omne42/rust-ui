use std::borrow::Cow;

use super::{FieldGroupDensity, FieldGroupOrientation, FieldGroupStateInput, logic};
use leptos::{children::Children, prelude::*};
use ui_headless::{A11yDirection, FieldGroupOptions, use_field_group, use_ui_id_provider};

fn render_group_label(
    has_label: Signal<bool>,
    label_id: Memo<String>,
    label_text: StoredValue<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <Show when=move || has_label.get()>
            <p id=move || label_id.get() class="ui-field-group__label" data-slot="field-group-label">
                {move || label_text.get_value()}
            </p>
        </Show>
    }
}

fn render_group_description(
    has_description: Signal<bool>,
    description_id: Memo<String>,
    description_text: StoredValue<Cow<'static, str>>,
) -> impl IntoView {
    view! {
        <Show when=move || has_description.get()>
            <p
                id=move || description_id.get()
                class="ui-field-group__description"
                data-slot="field-group-description"
            >
                {move || description_text.get_value()}
            </p>
        </Show>
    }
}

#[component]
pub fn FieldGroup(
    #[prop(default = FieldGroupOrientation::Vertical)] orientation: FieldGroupOrientation,
    #[prop(default = FieldGroupDensity::Comfortable)] density: FieldGroupDensity,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] is_invalid: Option<bool>,
    #[prop(optional)] invalid: Option<bool>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let disabled_source = logic::resolve_disabled_source(is_disabled, disabled);
    let disabled_source_attr = disabled_source.as_data_attr();
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);
    let invalid_source = logic::resolve_invalid_source(is_invalid, invalid);
    let invalid_source_attr = invalid_source.as_data_attr();
    let is_invalid = logic::resolve_is_invalid(is_invalid, invalid);
    let id_base = id_base.or_else(|| {
        use_ui_id_provider().map(|id_provider| id_provider.next_prefixed_id(logic::DEFAULT_ID_BASE))
    });

    let content = logic::resolve_content(logic::FieldGroupContentInput {
        id_base,
        label,
        description,
        aria_label,
        lang,
        class_name,
    });
    let has_label = content.has_label;
    let has_description = content.has_description;
    let has_custom_aria_label = content.has_custom_aria_label;
    let has_custom_class_name = content.has_custom_class_name;

    let id_base = StoredValue::new(content.id_base.into_owned());
    let label_text = StoredValue::new(content.label_text);
    let description_text = StoredValue::new(content.description_text);
    let aria_label = StoredValue::new(content.aria_label.into_owned());
    let lang = StoredValue::new(content.lang.map(std::borrow::Cow::into_owned));
    let class_name = StoredValue::new(content.class_name.map(std::borrow::Cow::into_owned));

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldGroupStateInput {
            orientation,
            density,
            disabled: is_disabled,
            invalid: is_invalid,
            has_label,
            has_description,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let has_group_label = Signal::derive(move || state.get().has_label);
    let has_group_description = Signal::derive(move || state.get().has_description);

    let label_id = Memo::new(move |_| format!("{}-label", id_base.get_value()));
    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));
    let headless = Memo::new(move |_| {
        use_field_group(FieldGroupOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            label_id: state.get().has_label.then(|| label_id.get()),
            description_id: state.get().has_description.then(|| description_id.get()),
            lang: lang.get_value(),
            dir,
        })
    });
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(state.get(), disabled_source, invalid_source)
    });

    let aria_labelledby = Signal::derive(move || headless.get().attrs.aria_labelledby);
    let aria_label_value = Signal::derive(move || headless.get().attrs.aria_label);
    let aria_describedby = Signal::derive(move || headless.get().attrs.aria_describedby);

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="field-group"
            data-orientation=move || headless.get().attrs.data_orientation
            data-density=move || headless.get().attrs.data_density
            data-state=move || headless.get().attrs.data_state
            data-disabled=move || headless.get().attrs.data_disabled
            data-invalid=move || headless.get().attrs.data_invalid
            data-disabled-source=disabled_source_attr
            data-invalid-source=invalid_source_attr
            data-label=move || headless.get().attrs.data_label
            data-description=move || headless.get().attrs.data_description
            data-aria-source=move || headless.get().attrs.data_aria_source
            data-custom-class=move || headless.get().attrs.data_custom_class
            data-class-source=move || headless.get().attrs.data_class_source
            data-ui-schema=move || agent_contract.get().schema
            data-ui-schema-version=move || agent_contract.get().schema_version
            data-ui-intent=move || agent_contract.get().intent
            data-ui-action=move || agent_contract.get().action
            data-ui-state=move || agent_contract.get().state
            data-ui-source=move || agent_contract.get().source
            data-ui-source-disabled=move || agent_contract.get().source_disabled
            data-ui-source-invalid=move || agent_contract.get().source_invalid
            data-ui-source-aria=move || agent_contract.get().source_aria
            data-ui-source-class=move || agent_contract.get().source_class
            data-ui-stream-mode=move || agent_contract.get().stream_mode
            data-ui-stream-support=move || agent_contract.get().stream_support
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback
            data-ui-output-mode=move || agent_contract.get().output_mode
            data-ui-output-status=move || agent_contract.get().output_status
            role=move || headless.get().attrs.role
            aria-label=move || aria_label_value.get()
            aria-labelledby=move || aria_labelledby.get()
            aria-describedby=move || aria_describedby.get()
            aria-disabled=move || headless.get().attrs.aria_disabled
            aria-invalid=move || headless.get().attrs.aria_invalid
            lang=move || headless.get().attrs.lang
            dir=move || headless.get().attrs.dir
        >
            {render_group_label(has_group_label, label_id, label_text)}

            <div class="ui-field-group__content" data-slot="field-group-content">
                {children()}
            </div>

            {render_group_description(has_group_description, description_id, description_text)}
        </div>
    }
}
