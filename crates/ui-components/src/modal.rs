use crate::{OnPress, Overlay};
use leptos::prelude::*;

#[component]
pub fn Modal(
    id_base: String,
    title: String,
    on_close: OnPress,
    #[prop(optional)] description: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let title: Signal<String> = title.into();
    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();
    if let Some(description) = description {
        let description: Signal<String> = description.into();
        view! {
            <Overlay
                on_close=on_close
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
            >
                <h2 id=move || title_id_attr.get() style="margin: 0 0 8px 0; font-size: 16px;">
                    {move || title.get()}
                </h2>
                <p id=move || description_id_attr.get() style="margin: 0 0 12px 0; line-height: 1.4;">
                    {move || description.get()}
                </p>
                {children()}
            </Overlay>
        }
    } else {
        view! {
            <Overlay on_close=on_close aria_labelledby=title_id.clone()>
                <h2 id=move || title_id_attr.get() style="margin: 0 0 8px 0; font-size: 16px;">
                    {move || title.get()}
                </h2>
                {children()}
            </Overlay>
        }
    }
}
