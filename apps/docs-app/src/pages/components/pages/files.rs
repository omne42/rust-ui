use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{DropZone, DroppedFile, FileTrigger, FileTriggerFile};

pub(super) fn file_trigger() -> AnyView {
    let (files, set_files) = signal(Vec::<FileTriggerFile>::new());
    let on_files = Callback::new(move |next: Vec<FileTriggerFile>| set_files.set(next));

    let code = r#"let on_files = Callback::new(|files: Vec<FileTriggerFile>| { /* ... */ });
<FileTrigger multiple=true on_files=on_files>"Pick files"</FileTrigger>"#;

    view! {
        <ComponentPage
            title="FileTrigger"
            slug="file-trigger"
            group="Files"
            description="A Button that forwards to an invisible <input type=file>."
        >
            <Playground title="Pick files" code=code>
                <div class="docs-stack">
                    <FileTrigger multiple=true on_files=on_files>
                        "Pick files"
                    </FileTrigger>
                    <div class="docs-stack docs-stack--tight">
                        {move || {
                            let list = files.get();
                            if list.is_empty() {
                                view! { <div class="ui-muted">"No files selected."</div> }.into_any()
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn drop_zone() -> AnyView {
    let (files, set_files) = signal(Vec::<DroppedFile>::new());
    let on_drop_files = Callback::new(move |next: Vec<DroppedFile>| set_files.set(next));

    let code = r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });
<DropZone label="Upload".to_string() on_drop_files=on_drop_files>
  "Drop files here"
</DropZone>"#;

    view! {
        <ComponentPage
            title="DropZone"
            slug="drop-zone"
            group="Files"
            description="Drag-and-drop + paste file ingestion with focus handling."
        >
            <Playground title="Drop / paste" code=code>
                <div class="docs-stack">
                    <DropZone label="Upload".to_string() on_drop_files=on_drop_files>
                        <div class="docs-drop-zone">
                            <div>"Drop files here"</div>
                            <div class="ui-muted">"…or paste an image/file."</div>
                        </div>
                    </DropZone>

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
        </ComponentPage>
    }
    .into_any()
}
