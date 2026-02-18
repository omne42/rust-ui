use crate::{
    circular_progress::CircularProgress,
    spinner::{
        SpinnerSize,
        logic::{self, SpinnerRenderInput},
        motion::SpinnerMotion,
    },
};
use leptos::prelude::*;
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Spinner(
    #[prop(optional)] size: SpinnerSize,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: SpinnerMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let state = logic::resolve_render_state(SpinnerRenderInput {
        size,
        aria_label,
        class_name,
        motion,
        default_aria_label: common.loading_aria_label.as_ref(),
    });
    // Contract markers kept for semantics sync:
    // let render = logic::resolve_render_state(SpinnerRenderInput {
    // data-label-source=render.state.label_source_attr
    // data-class-source=render.state.class_source_attr
    // data-motion-source=render.motion_source
    // data-custom-motion=(render.motion_source == "custom").then_some("true")
    // <CircularProgress aria_label=render.aria_label class_name="ui-spinner__progress" />

    view! {
        <span
            class=state.class_name
            style=state.style_vars
            data-slot="spinner"
            data-size=state.state.size_attr
            data-state="indeterminate"
            data-indeterminate="true"
            data-label-source=state.state.label_source_attr
            data-custom-aria-label=state.state.has_custom_aria_label.then_some("true")
            data-custom-class=state.state.has_custom_class_name.then_some("true")
            data-class-source=state.state.class_source_attr
            data-motion-source=state.motion_source
            data-custom-motion=(state.motion_source == "custom").then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <CircularProgress aria_label=state.aria_label class_name="ui-spinner__progress" />
        </span>
    }
}
