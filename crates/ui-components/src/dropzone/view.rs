use crate::{DropZone, DropZoneMotion, DroppedFile};
use leptos::prelude::*;

#[component]
pub fn Dropzone(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DropZoneMotion,
    #[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>,
    children: Children,
) -> impl IntoView {
    let label = label.unwrap_or_default();
    let aria_label = aria_label.unwrap_or_default();
    let on_drop_files = on_drop_files.unwrap_or_else(|| Callback::new(|_: Vec<DroppedFile>| {}));

    view! {
        <DropZone
            label=label
            aria_label=aria_label
            disabled=disabled
            motion=motion
            on_drop_files=on_drop_files
        >
            {children()}
        </DropZone>
    }
}
