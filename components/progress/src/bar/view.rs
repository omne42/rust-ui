use crate::bar::{ProgressBarSize, ProgressBarVariant, logic};
use leptos::prelude::*;
use ui_headless::{A11yDirection, ProgressbarA11yOptions, progressbar_attrs};

#[component]
pub fn ProgressBar(
    #[prop(optional)] variant: ProgressBarVariant,
    #[prop(optional)] size: ProgressBarSize,
    #[prop(optional)] value: Option<f64>,
    #[prop(optional)] default_value: Option<f64>,
    #[prop(optional)] on_value_change: Option<Callback<Option<f64>>>,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(optional)] indeterminate: Option<bool>,
    #[prop(optional)] is_indeterminate: bool,
    #[prop(optional, into, default = logic::DEFAULT_ARIA_LABEL.into())] aria_label: String,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_indeterminate = indeterminate.unwrap_or(is_indeterminate);
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(aria_label);
    let mode = logic::normalize_mode(is_indeterminate);
    let value_axis = logic::normalize_value_axis(value, default_value, on_value_change);
    let is_value_controlled = value_axis.is_controlled;
    let has_custom_default_value = value_axis.has_custom_default_value;
    let has_custom_on_value_change = value_axis.has_custom_on_value_change;
    let value_mode_attr = value_axis.mode_attr;
    let value_source_attr = value_axis.value_source_attr;
    let default_value_source_attr = value_axis.default_value_source_attr;
    let value_change_source_attr = value_axis.value_change_source_attr;
    let max = logic::normalize_max(max);

    let state = logic::resolve_state(logic::ProgressBarStateInput {
        variant,
        size,
        value: value_axis.value,
        max,
        indeterminate: mode.is_indeterminate(),
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);
    let semantics = progressbar_attrs(ProgressbarA11yOptions {
        aria_label,
        aria_valuemin: 0.0,
        aria_valuemax: state.max,
        aria_valuenow: state.value,
        aria_valuetext: None,
        is_indeterminate: state.is_indeterminate,
        lang,
        dir,
    });
    let max_attr_value = semantics.attrs.aria_valuemax.clone();
    let value_attr_value = semantics.attrs.aria_valuenow.clone();

    view! {
        <progress
            class=class
            data-slot="progress-bar"
            data-variant=state.variant_attr
            data-size=state.size_attr
            data-state=semantics.attrs.data_state
            data-status-mode=mode.as_str()
            data-indeterminate=semantics.attrs.data_indeterminate
            data-determinate=semantics.attrs.data_determinate
            data-has-value=state.has_value.then_some("true")
            data-label-source=state.label_source_attr
            data-motion-source="default"
            data-custom-aria-label=state.has_custom_aria_label.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-value-mode=value_mode_attr
            data-value-source=value_source_attr
            data-default-value-source=default_value_source_attr
            data-value-change-source=value_change_source_attr
            data-value-controlled=is_value_controlled.then_some("true")
            data-value-uncontrolled=(!is_value_controlled).then_some("true")
            data-custom-default-value=has_custom_default_value.then_some("true")
            data-custom-value-change=has_custom_on_value_change.then_some("true")
            role=semantics.attrs.role
            aria-label=semantics.attrs.aria_label
            aria-valuemin=semantics.attrs.aria_valuemin
            aria-valuemax=semantics.attrs.aria_valuemax
            aria-valuenow=semantics.attrs.aria_valuenow
            aria-valuetext=semantics.attrs.aria_valuetext
            lang=semantics.attrs.lang
            dir=semantics.attrs.dir
            max=max_attr_value
            value=value_attr_value
        ></progress>
    }
}
