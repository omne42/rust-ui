use crate::{
    DateInputGroupMotion,
    logic::{self, DateInputGroupVariant},
    motion as date_input_group_motion,
};
use leptos::{children::ViewFn, html, prelude::*};
use ui_headless::{A11yDirection, labeled_group_attrs};

fn render_prefix_slot(prefix: StoredValue<ViewFn>) -> impl IntoView {
    view! {
        <div class="ui-date-input-group__prefix" data-slot="date-input-group-prefix">
            {prefix.get_value().run()}
        </div>
    }
}

fn render_suffix_slot(suffix: StoredValue<ViewFn>) -> impl IntoView {
    view! {
        <div class="ui-date-input-group__suffix" data-slot="date-input-group-suffix">
            {suffix.get_value().run()}
        </div>
    }
}

#[component]
pub fn DateInputGroup(
    #[prop(optional)] is_full_width: bool,
    #[prop(optional)] variant: DateInputGroupVariant,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_invalid: bool,
    #[prop(optional)] is_segmented: bool,
    #[prop(optional)] motion: DateInputGroupMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] prefix: Option<ViewFn>,
    #[prop(optional, into)] suffix: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let group_a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));

    let prefix = prefix.map(StoredValue::new);
    let suffix = suffix.map(StoredValue::new);

    let state = Memo::new(move |_| {
        logic::derive_state(logic::DateInputGroupStateDeriveInput {
            variant,
            width: logic::resolve_width(is_full_width),
            status: logic::resolve_status(is_disabled, is_invalid),
            is_segmented,
            has_prefix: prefix.is_some(),
            has_suffix: suffix.is_some(),
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let is_custom_motion = motion != DateInputGroupMotion::default();
    let (motion_source_attr, custom_motion_attr) = logic::resolve_motion_source_attrs(motion);
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(logic::DateInputGroupAgentContractInput {
            render_state: state.get(),
            is_custom_motion,
        })
    });
    let node_ref: NodeRef<html::Div> = NodeRef::new();
    date_input_group_motion::attach_motion(node_ref, motion);

    view! {
        <div
            class=move || class.get()
            node_ref=node_ref
            data-slot="date-input-group"
            data-variant=move || state.get().variant_attr
            data-width=move || state.get().width_attr
            data-state=move || state.get().data_state_attr
            data-full-width=move || state.get().is_full_width.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-segmented=move || state.get().is_segmented.then_some("true")
            data-has-prefix=move || state.get().has_prefix.then_some("true")
            data-has-suffix=move || state.get().has_suffix.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=motion_source_attr
            data-custom-motion=custom_motion_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-state-source=move || agent_contract.get().state_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-aria-source=move || agent_contract.get().aria_source
            data-ui-class-source=move || agent_contract.get().class_source
            data-ui-config-policy=move || agent_contract.get().config_policy
            role=move || group_a11y.get_value().role
            aria-label=move || group_a11y.get_value().aria_label
            lang=move || group_a11y.get_value().lang
            dir=move || group_a11y.get_value().dir
            aria-disabled=move || state.get().is_disabled.then_some("true")
        >
            {prefix.map(render_prefix_slot)}

            <div class="ui-date-input-group__input" data-slot="date-input-group-input">
                <div class="ui-date-input-group__segment" data-slot="date-input-group-segment">
                    {children()}
                </div>
            </div>

            {suffix.map(render_suffix_slot)}
        </div>
    }
}
