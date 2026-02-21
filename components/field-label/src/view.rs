use crate::logic::{self, FieldLabelLogicInput, FieldLabelTone};
use leptos::prelude::*;
use ui_headless::{A11yDirection, FieldLabelOptions, use_field_label};

fn render_text_slot(text: String) -> impl IntoView {
    view! {
        <span class="ui-field-label__text" data-slot="field-label-text">
            {text}
        </span>
    }
}

fn render_required_slot(
    required_indicator: String,
    semantics: Memo<ui_headless::FieldLabelContract>,
) -> impl IntoView {
    view! {
        <Show when=move || semantics.get().state.is_required>
            <span
                class="ui-field-label__required"
                data-slot="field-label-required"
                aria-hidden="true"
            >
                {required_indicator.clone()}
            </span>
        </Show>
    }
}

#[component]
pub fn FieldLabel(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] tone: FieldLabelTone,
    #[prop(optional, into)] required_indicator: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let view_model = logic::derive_view_model(
        FieldLabelLogicInput {
            tone,
            is_required,
            is_disabled,
        },
        text,
        required_indicator,
        aria_label,
        for_id,
        class_name,
    );
    let logic::FieldLabelViewModel {
        text,
        required_indicator,
        aria_label,
        for_id,
        class_name,
        state,
    } = view_model;
    let class_name = StoredValue::new(class_name);
    let state = StoredValue::new(state);
    let class =
        Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get_value()));
    let semantics = Memo::new(move |_| {
        use_field_label(FieldLabelOptions {
            state: state.get_value(),
            aria_label: aria_label.clone(),
            lang: lang.clone(),
            dir,
        })
    });

    view! {
        <label
            class=move || class.get()
            for=for_id
            aria-label=move || semantics.get().attrs.aria_label
            aria-disabled=move || semantics.get().attrs.aria_disabled
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
            data-ui-schema=logic::FIELD_LABEL_AGENT_SCHEMA
            data-ui-intent=logic::FieldLabelAgentIntent::Label.as_attr()
            data-ui-action=logic::FieldLabelAgentAction::SnapshotRender.as_attr()
            data-ui-streaming=logic::FieldLabelAgentStreaming::Optional.as_attr()
            data-ui-fallback=logic::FieldLabelAgentFallback::Snapshot.as_attr()
            data-ui-output-state=logic::FieldLabelAgentOutputState::Verified.as_attr()
            data-slot="field-label"
            data-tone=move || semantics.get().attrs.data_tone
            data-state=move || semantics.get().attrs.data_state
            data-required=move || semantics.get().attrs.data_required
            data-disabled=move || semantics.get().attrs.data_disabled
            data-has-for=move || semantics.get().attrs.data_has_for
            data-text-source=move || semantics.get().attrs.data_text_source
            data-indicator-source=move || semantics.get().attrs.data_indicator_source
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
        >
            {render_text_slot(text)}
            {render_required_slot(required_indicator, semantics)}
        </label>
    }
}
