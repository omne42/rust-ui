use crate::overlay::OverlayMotion;
use crate::{OnPress, Overlay};
use leptos::prelude::*;

#[component]
pub fn Modal(
    open: Signal<bool>,
    id_base: String,
    title: String,
    on_close: OnPress,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] motion: OverlayMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let title: Signal<String> = title.into();
    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if let Some(description) = description {
        let description: Signal<String> = description.into();
        view! {
            <Overlay
                open=open
                on_close=on_close
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion
                on_exit_complete=on_exit_complete
            >
                <h2 class="ui-modal__title" id=move || title_id_attr.get()>
                    {move || title.get()}
                </h2>
                <p class="ui-modal__description" id=move || description_id_attr.get()>
                    {move || description.get()}
                </p>
                {children()}
            </Overlay>
        }
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close
                aria_labelledby=title_id.clone()
                motion=motion
                on_exit_complete=on_exit_complete
            >
                <h2 class="ui-modal__title" id=move || title_id_attr.get()>
                    {move || title.get()}
                </h2>
                {children()}
            </Overlay>
        }
    }
}
