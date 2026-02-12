use crate::dropzone::{
    DropzoneStateInput,
    logic::{self},
};
use crate::{DropZone, DropZoneMotion, DroppedFile};
use leptos::prelude::*;

#[component]
pub fn Dropzone(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DropZoneMotion,
    #[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (label, has_custom_label) = logic::resolve_label(label);
    let (aria_label, aria_source_attr) = logic::resolve_aria_label(&label, aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let state = logic::resolve_state(DropzoneStateInput {
        disabled,
        has_custom_label,
        aria_source_attr,
        has_custom_class_name,
        has_custom_motion: motion != DropZoneMotion::default(),
        has_custom_drop_handler: on_drop_files.is_some(),
    });

    let class_name = logic::compose_class_name(class_name, state);
    let on_drop_files = on_drop_files.unwrap_or_else(|| Callback::new(|_: Vec<DroppedFile>| {}));

    view! {
        <div
            class=class_name
            data-slot="dropzone"
            data-state=state.state_attr
            data-label-source=state.label_source_attr
            data-aria-source=state.aria_source_attr
            data-class-source=state.class_source_attr
            data-motion-source=state.motion_source_attr
            data-drop-handler-source=state.drop_handler_source_attr
            data-custom-label=state.has_custom_label.then_some("true")
            data-custom-aria=state.has_custom_aria.then_some("true")
            data-custom-drop-handler=state.has_custom_drop_handler.then_some("true")
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
        >
            <DropZone
                label=label
                aria_label=aria_label
                disabled=disabled
                motion=motion
                on_drop_files=on_drop_files
            >
                {children()}
            </DropZone>
        </div>
    }
}
