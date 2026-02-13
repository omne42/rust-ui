use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ButtonMotion, DropZone, DropZoneMotion, DroppedFile, FileTrigger, FileTriggerFile,
    FileTriggerMotion,
};

pub(super) fn file_trigger() -> AnyView {
    let (files, set_files) = signal(Vec::<FileTriggerFile>::new());
    let on_files = Callback::new(move |next: Vec<FileTriggerFile>| set_files.set(next));

    let (custom_files, set_custom_files) = signal(Vec::<FileTriggerFile>::new());
    let on_custom_files =
        Callback::new(move |next: Vec<FileTriggerFile>| set_custom_files.set(next));

    let code = Signal::derive(move || {
        r#"let on_files = Callback::new(|files: Vec<FileTriggerFile>| { /* ... */ });
<FileTrigger multiple=true on_files=on_files>"Pick files"</FileTrigger>"#
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r#"let on_files = Callback::new(|files: Vec<FileTriggerFile>| { /* ... */ });

<FileTrigger
  multiple=true
  motion=FileTriggerMotion {
    trigger: ButtonMotion {
      hover_scale: 1.04,
      tap_scale: 0.94,
      ..ButtonMotion::default()
    }
  }
  on_files=on_files
>
  "Pick files (custom motion)"
</FileTrigger>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="FileTrigger"
            slug="file-trigger"
            group="Files"
            description="A Button that forwards to an invisible <input type=file>."
        >
            <Playground title="Pick files" code_signal=code>
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

            <Playground title="Pick files with custom motion" code_signal=motion_code>
                <div class="docs-stack">
                    <FileTrigger
                        multiple=true
                        motion=FileTriggerMotion {
                            trigger: ButtonMotion {
                                hover_scale: 1.04,
                                tap_scale: 0.94,
                                ..ButtonMotion::default()
                            },
                        }
                        on_files=on_custom_files
                    >
                        "Pick files (custom motion)"
                    </FileTrigger>
                    <div class="docs-stack docs-stack--tight">
                        {move || {
                            let list = custom_files.get();
                            if list.is_empty() {
                                view! {
                                    <div class="ui-muted">"No files selected (custom motion example)."</div>
                                }
                                .into_any()
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

    let code = Signal::derive(move || {
        r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });
<DropZone label="Upload".to_string() on_drop_files=on_drop_files>
  "Drop files here"
</DropZone>"#
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });

<DropZone
  label="Upload".to_string()
  motion=DropZoneMotion {
    hover_scale: 1.015,
    drop_scale: 1.03,
    hover_highlight: 0.42,
    ..DropZoneMotion::default()
  }
  on_drop_files=on_drop_files
>
  "Drop files here"
</DropZone>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DropZone"
            slug="drop-zone"
            group="Files"
            description="Drag-and-drop + paste file ingestion with focus handling."
        >
            <Playground title="Drop / paste" code_signal=code>
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

            <Playground title="Drop / paste with custom motion" code_signal=motion_code>
                <div class="docs-stack">
                    <DropZone
                        label="Upload (custom motion)".to_string()
                        motion=DropZoneMotion {
                            hover_scale: 1.015,
                            drop_scale: 1.03,
                            hover_highlight: 0.42,
                            ..DropZoneMotion::default()
                        }
                        on_drop_files=on_drop_files
                    >
                        <div class="docs-drop-zone">
                            <div>"Drop files here"</div>
                            <div class="ui-muted">"Custom spring scale + highlight tuning."</div>
                        </div>
                    </DropZone>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
