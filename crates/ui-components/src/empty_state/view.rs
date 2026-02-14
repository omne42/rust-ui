use crate::empty_state::{
    EmptyStateStateInput, EmptyStateStrings,
    logic::{self, EmptyStateAlign, EmptyStateTone},
};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::i18n;

#[component]
pub fn EmptyState(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] tone: EmptyStateTone,
    #[prop(optional)] align: EmptyStateAlign,
    #[prop(optional)] compact: bool,
    #[prop(optional)] bordered: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<EmptyStateStrings>();
    let (title, has_custom_title) = logic::normalize_title(title, strings.default_title.as_ref());
    let title = StoredValue::new(title);

    let (description, has_custom_description) =
        logic::normalize_description(description, strings.default_description.as_ref());
    let description = StoredValue::new(description);

    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, strings.default_aria_label.as_ref());

    let has_icon = icon.is_some();
    let has_actions = actions.is_some();

    let icon = icon.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(EmptyStateStateInput {
            tone,
            align,
            compact,
            bordered,
            has_icon,
            has_actions,
            has_custom_title,
            has_custom_description,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            data-slot="empty-state"
            data-tone=move || state.get().tone_attr
            data-align=move || state.get().align_attr
            data-state=move || state.get().data_state_attr
            data-compact=move || state.get().is_compact.then_some("true")
            data-bordered=move || state.get().is_bordered.then_some("true")
            data-icon=move || state.get().has_icon.then_some("true")
            data-actions=move || state.get().has_actions.then_some("true")
            data-title-source=move || state.get().title_source_attr
            data-description-source=move || state.get().description_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            role="status"
            aria-label=aria_label
        >
            {state.get().has_icon.then(|| {
                let icon = icon.expect("checked has_icon");
                view! {
                    <div class="ui-empty-state__icon" data-slot="empty-state-icon" aria-hidden="true">
                        {icon.get_value().run()}
                    </div>
                }
            })}
            <h3 class="ui-empty-state__title" data-slot="empty-state-title">{title.get_value()}</h3>
            <p class="ui-empty-state__description" data-slot="empty-state-description">
                {description.get_value()}
            </p>
            {state.get().has_actions.then(|| {
                let actions = actions.expect("checked has_actions");
                view! {
                    <div class="ui-empty-state__actions" data-slot="empty-state-actions">
                        {actions.get_value().run()}
                    </div>
                }
            })}
        </section>
    }
}
