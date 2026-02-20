use super::{FieldGroupDensity, FieldGroupOrientation, FieldGroupStateInput, logic};
use leptos::{children::Children, prelude::*};

#[component]
pub fn FieldGroup(
    #[prop(default = FieldGroupOrientation::Vertical)] orientation: FieldGroupOrientation,
    #[prop(default = FieldGroupDensity::Comfortable)] density: FieldGroupDensity,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let id_base = StoredValue::new(logic::normalize_id_base(id_base));

    let label = logic::normalize_optional_text(label);
    let has_label = label.is_some();
    let label = StoredValue::new(label);

    let description = logic::normalize_optional_text(description);
    let has_description = description.is_some();
    let description = StoredValue::new(description);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(FieldGroupStateInput {
            orientation,
            density,
            disabled,
            invalid,
            has_label,
            has_description,
            has_custom_aria_label,
            has_custom_class_name: class_name.get_value().is_some(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let label_id = Memo::new(move |_| format!("{}-label", id_base.get_value()));
    let description_id = Memo::new(move |_| format!("{}-description", id_base.get_value()));

    let aria_labelledby = Signal::derive(move || {
        let state = state.get();
        (!state.has_custom_aria_label && state.has_label).then(|| label_id.get())
    });

    let aria_label_value = Signal::derive(move || {
        let state = state.get();
        if !state.has_custom_aria_label && state.has_label {
            None
        } else {
            Some(aria_label.get_value())
        }
    });

    let aria_describedby =
        Signal::derive(move || state.get().has_description.then(|| description_id.get()));

    view! {
        <div
            id=move || id_base.get_value()
            class=move || class.get()
            data-slot="field-group"
            data-orientation=move || state.get().orientation_attr
            data-density=move || state.get().density_attr
            data-state=move || state.get().state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-invalid=move || state.get().is_invalid.then_some("true")
            data-label=move || state.get().label_attr
            data-description=move || state.get().description_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="group"
            aria-label=move || aria_label_value.get()
            aria-labelledby=move || aria_labelledby.get()
            aria-describedby=move || aria_describedby.get()
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-invalid=move || state.get().is_invalid.then_some("true")
        >
            <Show when=move || state.get().has_label>
                <p id=move || label_id.get() class="ui-field-group__label" data-slot="field-group-label">
                    {move || label.get_value().unwrap_or_default()}
                </p>
            </Show>

            <div class="ui-field-group__content" data-slot="field-group-content">
                {children()}
            </div>

            <Show when=move || state.get().has_description>
                <p
                    id=move || description_id.get()
                    class="ui-field-group__description"
                    data-slot="field-group-description"
                >
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
