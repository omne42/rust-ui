use crate::{
    CodeVariant,
    logic::{self, CodeViewInput},
};
use leptos::prelude::*;
use ui_headless::a11y::{A11yDirection, locale_attrs};

#[component]
pub fn Code(
    #[prop(optional, into)] variant: Option<CodeVariant>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let locale = locale_attrs(lang, dir);
    let resolved = logic::resolve_view_state(CodeViewInput {
        variant,
        class_name,
    });
    let state = resolved.state;
    let class = resolved.class;

    view! {
        <code
            class=class
            data-slot="code"
            data-variant=state.variant_attr
            data-state=state.state_attr
            data-inline=state.is_inline.then_some("true")
            data-block=state.is_block.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-ui-streaming="optional"
            data-ui-fallback="snapshot"
            data-ui-output-state="verified"
            aria-live="off"
            aria-busy="false"
            lang=locale.lang
            dir=locale.dir
        >
            {children()}
        </code>
    }
}
