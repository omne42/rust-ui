use leptos::{html, prelude::*};
use ui_layout::Card;

#[component]
pub fn MarkdownPage(markdown: &'static str) -> impl IntoView {
    let crate::markdown::MarkdownDoc {
        html: rendered_html,
        toc: toc_items,
        ..
    } = crate::markdown::render_markdown(markdown);

    #[cfg(target_arch = "wasm32")]
    let toc_ids = toc_items
        .iter()
        .map(|item| item.id.clone())
        .collect::<Vec<_>>();

    if let Some(toc) = crate::toc::use_docs_toc() {
        toc.set_items(toc_items);
    }

    let html = StoredValue::new(rendered_html);

    let container_ref: NodeRef<html::Div> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    {
        use gloo_events::EventListener;
        use leptos::wasm_bindgen::JsCast;

        let router = crate::router::use_docs_router();
        if let Some(router) = router {
            let toc_ids = StoredValue::new(toc_ids);

            let listener = StoredValue::new_local(None::<EventListener>);

            Effect::new(move |_| {
                let Some(div) = container_ref.get() else {
                    return;
                };
                if listener.with_value(|value| value.is_some()) {
                    return;
                }

                let Some(window) = web_sys::window() else {
                    return;
                };
                let Some(document) = window.document() else {
                    return;
                };

                toc_ids.with_value(|ids| {
                    for id in ids {
                        let Some(heading) = document.get_element_by_id(id) else {
                            continue;
                        };
                        let Ok(existing) = heading.query_selector(":scope > .docs-heading-anchor")
                        else {
                            continue;
                        };
                        if existing.is_some() {
                            continue;
                        }

                        let anchor = r#"<button type="button" class="docs-heading-anchor" aria-label="Link to section">#</button>"#;
                        drop(heading.insert_adjacent_html("afterbegin", anchor));                    }
                });

                let element: leptos::web_sys::Element = div.unchecked_into();
                let listener_router = router;
                listener.set_value(Some(EventListener::new(&element, "click", move |event| {
                    let Some(target) = event.target() else {
                        return;
                    };
                    let Ok(target) = target.dyn_into::<leptos::web_sys::Element>() else {
                        return;
                    };

                    let Ok(Some(anchor)) = target.closest(".docs-heading-anchor") else {
                        return;
                    };
                    let Ok(Some(heading)) = anchor.closest("h2[id], h3[id]") else {
                        return;
                    };
                    let id = heading.id();
                    if id.trim().is_empty() {
                        return;
                    }

                    event.prevent_default();
                    event.stop_propagation();

                    let next = crate::route::route_with_section(
                        &listener_router.route.get_untracked(),
                        &id,
                    );
                    listener_router.navigate.run(next);
                })));
            });

            on_cleanup(move || listener.set_value(None));
        }
    }

    view! {
        <Card class_name="docs-prose".to_string()>
            <div node_ref=container_ref inner_html=move || html.get_value()></div>
        </Card>
    }
}
