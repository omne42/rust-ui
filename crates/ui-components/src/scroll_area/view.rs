use crate::scroll_area::logic::{self, ScrollAreaStateInput};
use leptos::{html, prelude::*};

#[component]
pub fn ScrollArea(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation,
    #[prop(optional)] max_height_px: Option<u32>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let state = logic::resolve_state(ScrollAreaStateInput {
        orientation,
        disabled,
        max_height_px,
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    let viewport_ref: NodeRef<html::Div> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    let set_max_height = {
        let viewport_ref = viewport_ref;
        let max_height_px = StoredValue::new(state.max_height_px);
        move || {
            use leptos::wasm_bindgen::JsCast;

            let Some(px) = max_height_px.get_value() else {
                return;
            };

            let Some(div) = viewport_ref.get_untracked() else {
                return;
            };

            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();
            let _ = style.set_property("--ui-scroll-area-max-h", &format!("{px}px"));
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let set_max_height = {
        let _ = state.max_height_px;
        || {}
    };

    Effect::new(move |_| {
        let _ = viewport_ref.get();
        set_max_height();
    });

    view! {
        <div
            class=class
            data-slot="scroll-area"
            data-orientation=state.orientation_attr
            data-disabled=state.disabled.then_some("true")
            data-max-height=state.max_height_attr
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            role="region"
            aria-label=aria_label
        >
            <div
                class="ui-scroll-area__viewport"
                node_ref=viewport_ref
                data-slot="scroll-area-viewport"
                tabindex=if state.disabled { -1 } else { 0 }
                aria-disabled=state.disabled.then_some("true")
            >
                {children()}
            </div>
        </div>
    }
}
