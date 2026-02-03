use crate::scroll_shadow::logic::compute_scroll_shadow_edges;
use leptos::{ev, html, prelude::*};

#[component]
pub fn ScrollShadow(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] max_height_px: Option<u32>,
    children: Children,
) -> impl IntoView {
    let viewport_ref: NodeRef<html::Div> = NodeRef::new();
    let (shadow_top, set_shadow_top) = signal(false);
    let (shadow_bottom, set_shadow_bottom) = signal(false);

    let update = move || {
        let Some(div) = viewport_ref.get_untracked() else {
            return;
        };
        let scroll_top = div.scroll_top() as f64;
        let client_height = div.client_height() as f64;
        let scroll_height = div.scroll_height() as f64;
        let edges = compute_scroll_shadow_edges(scroll_top, client_height, scroll_height);
        set_shadow_top.set(edges.top);
        set_shadow_bottom.set(edges.bottom);
    };

    Effect::new(move |_| {
        let _ = viewport_ref.get();
        update();
    });

    let on_scroll = move |_ev: ev::Event| update();

    let base_class = "ui-scroll-shadow".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    #[cfg(target_arch = "wasm32")]
    let set_max_height = {
        let viewport_ref = viewport_ref;
        let max_height_px = StoredValue::new(max_height_px);
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
            let _ = style.set_property("--ui-scroll-shadow-max-h", &format!("{px}px"));
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let set_max_height = {
        let _ = max_height_px;
        || {}
    };

    Effect::new(move |_| {
        let _ = viewport_ref.get();
        set_max_height();
    });

    view! {
        <div
            class=class
            data-slot="scroll-shadow"
            data-shadow-top=move || if shadow_top.get() { Some("true") } else { None }
            data-shadow-bottom=move || if shadow_bottom.get() { Some("true") } else { None }
        >
            <div
                class="ui-scroll-shadow__viewport"
                node_ref=viewport_ref
                data-slot="scroll-shadow-viewport"
                on:scroll=on_scroll
            >
                {children()}
            </div>
        </div>
    }
}
