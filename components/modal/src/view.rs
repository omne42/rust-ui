use crate::OnPress;
use crate::modal::{ModalPartStateInput, ModalSlot, logic};
use crate::overlay::Overlay;
use crate::overlay::OverlayMotion;
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
    let has_custom_id_base = !id_base.trim().is_empty();
    let has_custom_title = !title.trim().is_empty();

    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let has_custom_description = description.is_some();
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != OverlayMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();

    let root_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Root,
        has_description: has_custom_description,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let root_class = logic::compose_class_name(class_name, root_state);
    let root_class = StoredValue::new(root_class);

    let title_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Title,
        has_description: has_custom_description,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let title_class = logic::compose_class_name(None, title_state);
    let title_class = StoredValue::new(title_class);

    let description_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Description,
        has_description: has_custom_description,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let description_class = logic::compose_class_name(None, description_state);
    let description_class = StoredValue::new(description_class);

    let body_state = logic::resolve_state(ModalPartStateInput {
        slot: ModalSlot::Body,
        has_description: has_custom_description,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let body_class = logic::compose_class_name(None, body_state);
    let body_class = StoredValue::new(body_class);

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
                    class=move || root_class.with_value(|class_name| class_name.clone())
                    data-slot=root_state.slot_attr
                    data-state=root_state.state_attr
                    data-description=root_state.description_attr
                    data-with-description=root_state.show_description.then_some("true")
                    data-custom-id=root_state.has_custom_id_base.then_some("true")
                    data-custom-title=root_state.has_custom_title.then_some("true")
                    data-custom-description=root_state.has_custom_description.then_some("true")
                    data-custom-class=root_state.has_custom_class_name.then_some("true")
                    data-custom-motion=root_state.has_custom_motion.then_some("true")
                    data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                    data-id-source=root_state.id_source_attr
                    data-title-source=root_state.title_source_attr
                    data-description-source=root_state.description_source_attr
                    data-class-source=root_state.class_source_attr
                    data-motion-source=root_state.motion_source_attr
                    data-exit-source=root_state.exit_source_attr
                >
                    <h2
                        class=move || title_class.with_value(|class_name| class_name.clone())
                        id=move || title_id_attr.get()
                        data-slot=title_state.slot_attr
                        data-state=title_state.state_attr
                        data-title-source=title_state.title_source_attr
                    >
                        {move || title.get()}
                    </h2>
                    <p
                        class=move || {
                            description_class.with_value(|class_name| class_name.clone())
                        }
                        id=move || description_id_attr.get()
                        data-slot=description_state.slot_attr
                        data-state=description_state.state_attr
                        data-description-source=description_state.description_source_attr
                    >
                        {move || description.get()}
                    </p>
                    <div
                        class=move || body_class.with_value(|class_name| class_name.clone())
                        data-slot=body_state.slot_attr
                        data-state=body_state.state_attr
                    >
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
                    class=move || root_class.with_value(|class_name| class_name.clone())
                    data-slot=root_state.slot_attr
                    data-state=root_state.state_attr
                    data-description=root_state.description_attr
                    data-with-description=root_state.show_description.then_some("true")
                    data-custom-id=root_state.has_custom_id_base.then_some("true")
                    data-custom-title=root_state.has_custom_title.then_some("true")
                    data-custom-description=root_state.has_custom_description.then_some("true")
                    data-custom-class=root_state.has_custom_class_name.then_some("true")
                    data-custom-motion=root_state.has_custom_motion.then_some("true")
                    data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                    data-id-source=root_state.id_source_attr
                    data-title-source=root_state.title_source_attr
                    data-description-source=root_state.description_source_attr
                    data-class-source=root_state.class_source_attr
                    data-motion-source=root_state.motion_source_attr
                    data-exit-source=root_state.exit_source_attr
                >
                    <h2
                        class=move || title_class.with_value(|class_name| class_name.clone())
                        id=move || title_id_attr.get()
                        data-slot=title_state.slot_attr
                        data-state=title_state.state_attr
                        data-title-source=title_state.title_source_attr
                    >
                        {move || title.get()}
                    </h2>
                    <div
                        class=move || body_class.with_value(|class_name| class_name.clone())
                        data-slot=body_state.slot_attr
                        data-state=body_state.state_attr
                    >
                        {children()}
                    </div>
                </div>
            </Overlay>
        }
        .into_any()
    }
}
