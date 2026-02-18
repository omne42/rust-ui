use crate::labeled_value::{
    LabeledValueOrientation, LabeledValueStateInput,
    logic::{self, LabeledValueTone},
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, LabeledValueOptions, use_labeled_value};

#[component]
pub fn LabeledValue(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] orientation: LabeledValueOrientation,
    #[prop(optional)] tone: LabeledValueTone,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: super::motion::LabeledValueMotion,
) -> impl IntoView {
    let (label, has_custom_label) = logic::normalize_label_text(label);
    let (value, has_custom_value) = logic::normalize_value_text(value);
    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);
    let lang = StoredValue::new(logic::normalize_optional_text(lang));

    let state = Signal::derive(move || {
        logic::resolve_state(LabeledValueStateInput {
            orientation,
            tone,
            has_custom_label,
            has_custom_value,
            has_description,
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let semantics = Signal::derive(move || {
        use_labeled_value(LabeledValueOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            lang: lang.get_value(),
            dir,
        })
    });
    let motion = super::motion::sanitize_motion(motion);
    let motion_source = if motion == super::motion::LabeledValueMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != super::motion::LabeledValueMotion::default()).then_some("true");
    let node_ref: NodeRef<html::Div> = NodeRef::new();
    super::motion::attach_motion(
        node_ref,
        Signal::derive(move || state.get().has_description),
        motion,
    );

    view! {
        <div
            class=move || class.get()
            node_ref=node_ref
            data-slot="labeled-value"
            data-orientation=move || semantics.get().attrs.data_orientation
            data-tone=move || semantics.get().attrs.data_tone
            data-state=move || semantics.get().attrs.data_state
            data-has-description=move || semantics.get().attrs.data_has_description
            data-label-source=move || semantics.get().attrs.data_label_source
            data-value-source=move || semantics.get().attrs.data_value_source
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=move || semantics.get().attrs.role
            aria-label=move || semantics.get().attrs.aria_label
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
        >
            <span class="ui-labeled-value__label" data-slot="labeled-value-label">
                {label}
            </span>
            <span class="ui-labeled-value__value" data-slot="labeled-value-value">
                {value}
            </span>
            {description.map(|description| {
                view! {
                    <span class="ui-labeled-value__description" data-slot="labeled-value-description">
                        {description}
                    </span>
                }
            })}
        </div>
    }
}
