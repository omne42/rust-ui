use crate::modal::logic::{self, ModalStateInput};
use crate::overlay::OverlayMotion;
use crate::{OnPress, Overlay};
use leptos::prelude::*;

#[component]
pub fn Modal(
    open: Signal<bool>,
    id_base: String,
    title: String,
    on_close: OnPress,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] motion: OverlayMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, "Modal");
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(ModalStateInput {
        has_description: description.is_some(),
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);
    let class = StoredValue::new(class);

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
                <div
                    class=move || class.get_value()
                    data-slot="modal"
                    data-state=state.state_attr
                    data-description=state.description_attr
                    data-with-description=state.show_description.then_some("true")
                    data-custom-class=state.has_custom_class_name.then_some("true")
                >
                    <h2 class="ui-modal__title" id=move || title_id_attr.get() data-slot="modal-title">
                        {move || title.get()}
                    </h2>
                    <p
                        class="ui-modal__description"
                        id=move || description_id_attr.get()
                        data-slot="modal-description"
                    >
                        {move || description.get()}
                    </p>
                    <div class="ui-modal__body" data-slot="modal-body">
                        {children()}
                    </div>
                </div>
            </Overlay>
        }
        .into_any()
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close
                aria_labelledby=title_id.clone()
                motion=motion
                on_exit_complete=on_exit_complete
            >
                <div
                    class=move || class.get_value()
                    data-slot="modal"
                    data-state=state.state_attr
                    data-description=state.description_attr
                    data-with-description=state.show_description.then_some("true")
                    data-custom-class=state.has_custom_class_name.then_some("true")
                >
                    <h2 class="ui-modal__title" id=move || title_id_attr.get() data-slot="modal-title">
                        {move || title.get()}
                    </h2>
                    <div class="ui-modal__body" data-slot="modal-body">
                        {children()}
                    </div>
                </div>
            </Overlay>
        }
        .into_any()
    }
}
