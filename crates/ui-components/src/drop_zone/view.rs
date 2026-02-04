use crate::drop_zone::{DropZoneMotion, DroppedFile};
use leptos::{ev, prelude::*};

#[component]
pub fn DropZone(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DropZoneMotion,
    #[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>,
    children: Children,
) -> impl IntoView {
    let _ = motion;

    let (drag_over, set_drag_over) = signal(false);
    let on_drop_files = StoredValue::new(on_drop_files);

    let on_drag_enter = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();
        set_drag_over.set(true);
    };

    let on_drag_over = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();
    };

    let on_drag_leave = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();
        set_drag_over.set(false);
    };

    let on_drop = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();
        set_drag_over.set(false);
        let files = crate::drop_zone::logic::collect_files_from_drag_event(&ev);
        if let Some(cb) = on_drop_files.get_value() {
            cb.run(files);
        }
    };

    view! {
        <div class="ui-drop-zone" data-slot="drop-zone">
            {label.filter(|value| !value.trim().is_empty()).map(|label| view! {
                <div class="ui-drop-zone__label" data-slot="drop-zone-label">{label}</div>
            })}

            <div
                class="ui-drop-zone__zone"
                data-slot="drop-zone-zone"
                data-drag-over=move || if drag_over.get() { Some("true") } else { None }
                data-disabled=disabled.then_some("true")
                tabindex=if disabled { -1 } else { 0 }
                on:dragenter=on_drag_enter
                on:dragover=on_drag_over
                on:dragleave=on_drag_leave
                on:drop=on_drop
            >
                {children()}
            </div>
        </div>
    }
}
