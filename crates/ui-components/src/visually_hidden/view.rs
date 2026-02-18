use leptos::prelude::*;
use ui_headless::a11y::{A11yDirection, locale_attrs};

use super::logic::{VisuallyHiddenLogicInput, compose_class_name, normalize_props};

#[component]
pub fn VisuallyHidden(
    children: Children,
    #[prop(optional, into)] is_focusable: Option<bool>,
    #[prop(optional, into)] focusable: Option<bool>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let state = normalize_props(VisuallyHiddenLogicInput {
        is_focusable,
        focusable,
        class_name,
    });
    let locale = locale_attrs(lang, dir);

    let class = compose_class_name(state.class_name, state.primitive_state);

    view! {
        <span
            class=class
            data-slot="visually-hidden"
            data-focus-mode=state.focus_mode.as_attr()
            data-focus-source=state.focus_prop_source.as_attr()
            data-class-source=state.class_name_source.as_attr()
            data-focusable=state.primitive_state.focusable_attr
            data-custom-class=state.primitive_state.custom_class_attr
            lang=locale.lang
            dir=locale.dir
        >
            {children()}
        </span>
    }
}
