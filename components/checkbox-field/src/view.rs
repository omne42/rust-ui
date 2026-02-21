use crate::{
    CheckboxFieldIndicatorPlacement, CheckboxFieldMotion, CheckboxFieldTone, logic,
    motion::{self},
};
use leptos::prelude::*;
use ui_checkbox::Checkbox;
use ui_headless::{A11yDirection, labeled_group_attrs};

fn render_checkbox_field_label(label: StoredValue<String>) -> impl IntoView {
    view! {
        <span class="ui-checkbox-field__label" data-slot="checkbox-field-label">
            {move || label.get_value()}
        </span>
    }
}

fn render_checkbox_field_description(
    render_state: Memo<logic::CheckboxFieldRenderState>,
    description_id: Memo<String>,
    description_text: StoredValue<String>,
) -> impl IntoView {
    view! {
        <Show when=move || render_state.get().state.has_description>
            <p
                id=move || description_id.get()
                class="ui-checkbox-field__description"
                data-slot="checkbox-field-description"
            >
                {move || description_text.get_value()}
            </p>
        </Show>
    }
}

#[component]
pub fn CheckboxField(
    #[prop(optional)] is_checked: Option<ReadSignal<bool>>,
    #[prop(optional)] checked: Option<ReadSignal<bool>>,
    #[prop(optional)] on_checked_change: Option<WriteSignal<bool>>,
    #[prop(optional)] set_checked: Option<WriteSignal<bool>>,
    #[prop(optional)] default_checked: Option<bool>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] is_invalid: Option<bool>,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] tone: CheckboxFieldTone,
    #[prop(optional)] indicator_placement: CheckboxFieldIndicatorPlacement,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: CheckboxFieldMotion,
) -> impl IntoView {
    let resolved_content = logic::resolve_content(logic::CheckboxFieldContentInput {
        id_base,
        label,
        description,
        aria_label,
        class_name,
    });
    let has_description = resolved_content.has_description;
    let has_custom_label = resolved_content.has_custom_label;
    let has_custom_aria_label = resolved_content.has_custom_aria_label;
    let has_custom_class_name = resolved_content.has_custom_class_name;
    let id_base = StoredValue::new(resolved_content.id_base);
    let label = StoredValue::new(resolved_content.label);
    let description_text = StoredValue::new(resolved_content.description_text);
    let checkbox_aria_label = StoredValue::new(resolved_content.aria_label);
    let class_name = StoredValue::new(resolved_content.class_name);

    let checked_control = logic::resolve_checked_control(
        is_checked,
        checked,
        on_checked_change,
        set_checked,
        default_checked,
    );
    let checked = checked_control.checked;
    let on_checked_change = checked_control.on_checked_change;
    let checked_mode = checked_control.mode;
    let checked_prop_source_attr = checked_control.checked_prop_source_attr;
    let checked_change_source_attr = checked_control.checked_change_source_attr;
    let checked_default_source_attr = checked_control.checked_default_source_attr;
    let checked_mode_attr = checked_mode.source_attr();
    let disabled = logic::normalize_is_disabled(is_disabled, disabled);
    let invalid = logic::normalize_is_invalid(is_invalid, invalid);
    let checkbox_affordance = logic::resolve_checkbox_affordance(indicator_placement, invalid);

    let motion = motion::sanitize_motion(motion);
    let motion_source = motion::source_attr(motion);
    let style_vars = StoredValue::new(motion::attach_motion(None, motion));
    let group_a11y = StoredValue::new(labeled_group_attrs(
        checkbox_aria_label.get_value(),
        lang,
        dir,
    ));

    let render_state = Memo::new(move |_| {
        logic::resolve_render_state(logic::CheckboxFieldRenderStateInput {
            checked: checked.get(),
            disabled,
            invalid,
            tone,
            indicator_placement,
            has_description,
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
            class_name: class_name.get_value(),
        })
    });
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::CheckboxFieldAgentContractInput {
            status: render_state.get().state.status,
            checked_mode,
            checked_prop_source_attr,
            checked_change_source_attr,
            checked_default_source_attr,
        })
    });

    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));

    view! {
        <div
            id=move || id_base.get_value()
            class=move || render_state.get().root_class_name
            style=move || style_vars.get_value()
            data-slot="checkbox-field"
            data-state=move || render_state.get().state.state_attr
            data-tone=move || render_state.get().state.tone_attr
            data-indicator-placement=move || render_state.get().state.indicator_placement_attr
            data-checked=move || render_state.get().state.is_checked.then_some("true")
            data-unchecked=move || render_state.get().state.is_unchecked.then_some("true")
            data-disabled=move || render_state.get().state.is_disabled.then_some("true")
            data-invalid=move || render_state.get().state.is_invalid.then_some("true")
            data-description=move || render_state.get().state.description_attr
            data-label-source=move || render_state.get().state.label_source_attr
            data-aria-source=move || render_state.get().state.aria_source_attr
            data-custom-class=move || render_state.get().state.has_custom_class_name.then_some("true")
            data-class-source=move || render_state.get().state.class_source_attr
            data-checked-mode=checked_mode_attr
            data-checked-prop-source=checked_prop_source_attr
            data-checked-change-source=checked_change_source_attr
            data-checked-default-source=checked_default_source_attr
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            role=group_a11y.get_value().role
            aria-label=move || group_a11y.get_value().aria_label
            aria-describedby=move || render_state.get().state.has_description.then(|| description_id.get())
            aria-disabled=move || render_state.get().state.is_disabled.then_some("true")
            aria-invalid=move || render_state.get().state.is_invalid.then_some("true")
            lang=move || group_a11y.get_value().lang
            dir=move || group_a11y.get_value().dir
        >
            {match on_checked_change {
                Some(on_checked_change) => {
                    view! {
                        <Checkbox
                            is_checked=checked
                            on_checked_change=on_checked_change
                            is_disabled=disabled
                            variant=checkbox_affordance.variant
                            class_name=checkbox_affordance.class_name
                            aria_label=checkbox_aria_label.get_value()
                        >
                            {render_checkbox_field_label(label)}
                        </Checkbox>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <Checkbox
                            is_checked=checked
                            is_disabled=disabled
                            variant=checkbox_affordance.variant
                            class_name=checkbox_affordance.class_name
                            aria_label=checkbox_aria_label.get_value()
                        >
                            {render_checkbox_field_label(label)}
                        </Checkbox>
                    }
                        .into_any()
                }
            }}

            {render_checkbox_field_description(
                render_state,
                description_id,
                description_text,
            )}
        </div>
    }
}
