use crate::scroll_shadow::logic::{
    self, ScrollShadowSemanticInput, ScrollShadowStateInput, compute_scroll_shadow_edges,
};
use leptos::{ev, html, prelude::*};
use std::rc::Rc;

#[component]
pub fn ScrollShadow(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] max_height_px: Option<u32>,
    children: Children,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let state = logic::resolve_state(ScrollShadowStateInput {
        max_height_px,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let viewport_ref: NodeRef<html::Div> = NodeRef::new();
    let (edge_state, set_edge_state) = signal(logic::ScrollShadowEdgeState::None);
    let semantic_state = Memo::new(move |_| {
        logic::resolve_semantic_state(ScrollShadowSemanticInput {
            edge_state: edge_state.get(),
        })
    });

    let update = Rc::new(move || {
        let Some(div) = viewport_ref.get_untracked() else {
            return;
        };
        let scroll_top = div.scroll_top() as f64;
        let client_height = div.client_height() as f64;
        let scroll_height = div.scroll_height() as f64;
        let edges = compute_scroll_shadow_edges(scroll_top, client_height, scroll_height);
        let next_state = logic::resolve_edge_state(edges.top, edges.bottom);
        if edge_state.get_untracked() != next_state {
            set_edge_state.set(next_state);
        }
    });

    #[cfg(target_arch = "wasm32")]
    let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);
    #[cfg(target_arch = "wasm32")]
    let resize_closure = StoredValue::new_local(
        None::<
            leptos::wasm_bindgen::closure::Closure<
                dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver),
            >,
        >,
    );

    Effect::new({
        let update = Rc::clone(&update);
        move |_| {
            drop(viewport_ref.get());
            update.as_ref()();

            #[cfg(target_arch = "wasm32")]
            {
                if resize_observer.get_value().is_some() {
                    return;
                }

                use leptos::wasm_bindgen::{JsCast, closure::Closure};

                let Some(div) = viewport_ref.get_untracked() else {
                    return;
                };
                let element: leptos::web_sys::Element = div.unchecked_into();

                let update_for_observer = Rc::clone(&update);
                let closure = Closure::wrap(Box::new(
                    move |_: js_sys::Array, _: leptos::web_sys::ResizeObserver| {
                        update_for_observer.as_ref()();
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, leptos::web_sys::ResizeObserver)>);

                if let Ok(observer) =
                    leptos::web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                {
                    observer.observe(&element);
                    resize_observer.set_value(Some(observer));
                    resize_closure.set_value(Some(closure));
                }

                let resize_observer_for_cleanup = resize_observer;
                let resize_closure_for_cleanup = resize_closure;
                on_cleanup(move || {
                    if let Some(observer) = resize_observer_for_cleanup.get_value() {
                        observer.disconnect();
                    }
                    resize_observer_for_cleanup.set_value(None);
                    resize_closure_for_cleanup.set_value(None);
                });
            }
        }
    });

    let on_scroll = {
        let update = Rc::clone(&update);
        move |_event: ev::Event| update.as_ref()()
    };

    view! {
        <div
            class=class
            class:ui-scroll-shadow--shadow-top=move || semantic_state.get().shadow_top_attr.is_some()
            class:ui-scroll-shadow--shadow-bottom=move || {
                semantic_state.get().shadow_bottom_attr.is_some()
            }
            class:ui-scroll-shadow--scrollable=move || semantic_state.get().is_scrollable
            data-slot="scroll-shadow"
            data-max-height=state.max_height_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-state=move || semantic_state.get().edge_state_attr
            data-scrollable=move || semantic_state.get().scrollable_attr
            data-shadow-top=move || semantic_state.get().shadow_top_attr
            data-shadow-bottom=move || semantic_state.get().shadow_bottom_attr
        >
            <div
                class="ui-scroll-shadow__viewport"
                node_ref=viewport_ref
                data-slot="scroll-shadow-viewport"
                style=logic::compose_inline_style(state.max_height_px)
                on:scroll=on_scroll
            >
                {children()}
            </div>
        </div>
    }
}
