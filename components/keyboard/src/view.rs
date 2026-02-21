use crate::logic::{self, KeyboardRootInput, KeyboardTone};
use leptos::prelude::*;
use ui_headless::{A11yDirection, KeyboardOptions, use_keyboard};

#[component]
pub fn Keyboard(
    #[prop(optional)] tone: Option<KeyboardTone>,
    #[prop(optional, into)] is_compact: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let root_state = Memo::new(move |_| {
        logic::normalize_root_state(KeyboardRootInput {
            tone,
            is_compact,
            aria_label: aria_label.clone(),
            class_name: class_name.clone(),
            lang: lang.clone(),
        })
    });
    let semantics = Memo::new(move |_| {
        use_keyboard(KeyboardOptions {
            state: root_state.get().state,
            aria_label: root_state.get().aria_label.clone(),
            lang: root_state.get().lang.clone(),
            dir,
        })
    });

    view! {
        <kbd
            class=move || root_state.get().class_name.clone()
            data-slot=move || semantics.get().attrs.data_slot
            data-tone=move || semantics.get().attrs.data_tone
            data-state=move || semantics.get().attrs.data_state
            data-compact=move || semantics.get().attrs.data_compact
            data-aria-source=move || semantics.get().attrs.data_aria_source
            data-custom-class=move || semantics.get().attrs.data_custom_class
            data-class-source=move || semantics.get().attrs.data_class_source
            data-ui-schema=move || semantics.get().attrs.data_ui_schema
            data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version
            data-ui-intent=move || semantics.get().attrs.data_ui_intent
            data-ui-action=move || semantics.get().attrs.data_ui_action
            data-ui-state=move || semantics.get().attrs.data_ui_state
            data-ui-source=move || semantics.get().attrs.data_ui_source
            data-ui-output-status=move || semantics.get().attrs.data_ui_output_status
            aria-label=move || semantics.get().attrs.aria_label
            lang=move || semantics.get().attrs.lang
            dir=move || semantics.get().attrs.dir
        >
            {children()}
        </kbd>
    }
}
