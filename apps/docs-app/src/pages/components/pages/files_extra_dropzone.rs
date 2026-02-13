use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{DropZoneMotion, DroppedFile, Dropzone};

pub(super) fn dropzone() -> AnyView {
    let (files, set_files) = signal(Vec::<DroppedFile>::new());
    let on_drop_files = Callback::new(move |next: Vec<DroppedFile>| set_files.set(next));

    let (marker_events, set_marker_events) = signal(0_u32);
    let marker_on_drop_files =
        Callback::new(move |_: Vec<DroppedFile>| set_marker_events.update(|count| *count += 1));

    let basic_code = Signal::derive(move || {
        r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });
<Dropzone label=\"Upload\".to_string() on_drop_files=on_drop_files>
  \"Drop files here\"
</Dropzone>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<Dropzone label=\"Disabled\".to_string() disabled=true>
  \"Dropzone disabled\"
</Dropzone>"#
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let mut marker_motion = DropZoneMotion::default();
marker_motion.hover_scale = 1.02;
marker_motion.drop_scale = 1.01;

<Dropzone
  label=\"Asset upload\".to_string()
  aria_label=\"Asset upload area\".to_string()
  class_name=\"docs-dropzone-state\".to_string()
  motion=marker_motion
  on_drop_files=Callback::new(move |_| { /* marker */ })
>
  <div>\"Inspect root source/state markers\"</div>
</Dropzone>"#
            .to_string()
    });

    let marker_motion = DropZoneMotion {
        hover_scale: 1.02,
        drop_scale: 1.01,
        ..DropZoneMotion::default()
    };

    view! {
        <ComponentPage
            title="Dropzone"
            slug="dropzone"
            group="Files"
            description="Spectrum-compatible Dropzone alias for upstream naming parity, preserving DropZone drag/drop + paste accessibility contracts and HeroUI-level spring interaction motion."
        >
            <Playground title="Drop / paste" code_signal=basic_code>
                <div class="docs-stack">
                    <Dropzone label="Upload".to_string() on_drop_files=on_drop_files>
                        <div class="docs-drop-zone">
                            <div>"Drop files here"</div>
                            <div class="ui-muted">"…or paste an image/file."</div>
                        </div>
                    </Dropzone>

                    <div class="docs-stack docs-stack--tight">
                        {move || {
                            let list = files.get();
                            if list.is_empty() {
                                view! { <div class="ui-muted">"No files received."</div> }.into_any()
                            } else {
                                view! {
                                    <ul class="docs-list">
                                        {list
                                            .into_iter()
                                            .map(|file| {
                                                view! {
                                                    <li>
                                                        <code>{file.name}</code>
                                                        <span class="ui-muted">" ("{file.size}" bytes)"</span>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                }
                                .into_any()
                            }
                        }}
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled" code_signal=disabled_code>
                <Dropzone label="Disabled".to_string() disabled=true>
                    <div class="docs-drop-zone">
                        <div>"Dropzone disabled"</div>
                        <div class="ui-muted">"No pointer or drop interactions"</div>
                    </div>
                </Dropzone>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-label-source`, `data-aria-source`, `data-drop-handler-source`, `data-class-source`, and `data-motion-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Dropzone
                        label="Asset upload".to_string()
                        aria_label="Asset upload area".to_string()
                        class_name="docs-dropzone-state".to_string()
                        motion=marker_motion
                        on_drop_files=marker_on_drop_files
                    >
                        <div class="docs-drop-zone">
                            <div>"Inspect root source/state markers"</div>
                            <div class="ui-muted">"Custom aria/handler/motion markers are explicit."</div>
                        </div>
                    </Dropzone>
                    <span class="ui-muted">"marker drop events: " {move || marker_events.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
