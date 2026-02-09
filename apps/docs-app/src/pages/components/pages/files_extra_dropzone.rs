use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{DroppedFile, Dropzone};

pub(super) fn dropzone() -> AnyView {
    let (files, set_files) = signal(Vec::<DroppedFile>::new());
    let on_drop_files = Callback::new(move |next: Vec<DroppedFile>| set_files.set(next));

    let basic_code = r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });
<Dropzone label=\"Upload\".to_string() on_drop_files=on_drop_files>
  \"Drop files here\"
</Dropzone>"#;

    let disabled_code = r#"<Dropzone label=\"Disabled\".to_string() disabled=true>
  \"Dropzone disabled\"
</Dropzone>"#;

    view! {
        <ComponentPage
            title="Dropzone"
            slug="dropzone"
            group="Files"
            description="Spectrum-compatible Dropzone alias for upstream naming parity, preserving DropZone drag/drop + paste accessibility contracts and HeroUI-level spring interaction motion."
        >
            <Playground title="Drop / paste" code=basic_code>
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

            <Playground title="Disabled" code=disabled_code>
                <Dropzone label="Disabled".to_string() disabled=true>
                    <div class="docs-drop-zone">
                        <div>"Dropzone disabled"</div>
                        <div class="ui-muted">"No pointer or drop interactions"</div>
                    </div>
                </Dropzone>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
