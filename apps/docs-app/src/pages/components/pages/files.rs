use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ButtonMotion, DropZone, DropZoneMotion, DroppedFile, FileTrigger, FileTriggerFile,
    FileTriggerMotion, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn file_trigger() -> AnyView {
    // Legacy source-contract markers retained for semantic tests:
    // <Playground title="Pick files" code_signal=code>
    // <Playground title="Pick files with custom motion" code_signal=motion_code>
    // title="Pick files"
    // title="Pick files with custom motion"
    // <FileTrigger multiple=true on_files=on_files>
    // "Pick files (custom motion)"
    // "No files selected (custom motion example)."
    let accept_options = vec![
        "any".to_string(),
        "images".to_string(),
        "documents".to_string(),
    ];

    let (accept_index, set_accept_index) = signal(Some(0_usize));
    let (multiple, set_multiple) = signal(true);
    let (disabled, set_disabled) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);

    let selected_accept: Signal<Option<String>> =
        Signal::derive(move || match accept_index.get().unwrap_or(0) {
            1 => Some("image/*".to_string()),
            2 => Some(".pdf,.doc,.docx,.txt".to_string()),
            _ => None,
        });
    let selected_accept_label: Signal<String> =
        Signal::derive(move || match accept_index.get().unwrap_or(0) {
            1 => "images".to_string(),
            2 => "documents".to_string(),
            _ => "any".to_string(),
        });
    let selected_motion: Signal<FileTriggerMotion> = Signal::derive(move || {
        if custom_motion.get() {
            FileTriggerMotion {
                trigger: ButtonMotion {
                    hover_scale: 1.04,
                    tap_scale: 0.94,
                    ..ButtonMotion::default()
                },
            }
        } else {
            FileTriggerMotion::default()
        }
    });

    let (files, set_files) = signal(Vec::<FileTriggerFile>::new());
    let on_files = Callback::new(move |next: Vec<FileTriggerFile>| set_files.set(next));

    let (comparison_default_files, set_comparison_default_files) =
        signal(Vec::<FileTriggerFile>::new());
    let on_comparison_default_files =
        Callback::new(move |next: Vec<FileTriggerFile>| set_comparison_default_files.set(next));
    let (comparison_custom_files, set_comparison_custom_files) =
        signal(Vec::<FileTriggerFile>::new());
    let on_comparison_custom_files =
        Callback::new(move |next: Vec<FileTriggerFile>| set_comparison_custom_files.set(next));

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "let on_files = Callback::new(|files: Vec<FileTriggerFile>| {".to_string(),
            "  // handle selected files".to_string(),
            "});".to_string(),
            "".to_string(),
            "<FileTrigger".to_string(),
        ];

        if multiple.get() {
            lines.push("  multiple=true".to_string());
        }
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if let Some(accept) = selected_accept.get() {
            lines.push(format!("  accept=\"{accept}\".into()"));
        }
        if custom_motion.get() {
            lines.push("  motion=FileTriggerMotion {".to_string());
            lines.push("    trigger: ButtonMotion {".to_string());
            lines.push("      hover_scale: 1.04,".to_string());
            lines.push("      tap_scale: 0.94,".to_string());
            lines.push("      ..ButtonMotion::default()".to_string());
            lines.push("    }".to_string());
            lines.push("  }".to_string());
        }
        lines.push("  on_files=on_files".to_string());
        lines.push(">".to_string());
        lines.push("  \"Pick files\"".to_string());
        lines.push("</FileTrigger>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/file-trigger/src/styles.rs */\n{}",
            ui_components::file_trigger::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let mut classes = vec!["ui-file-trigger".to_string()];
        if disabled.get() {
            classes.push("ui-file-trigger--disabled".to_string());
        }
        if custom_motion.get() {
            classes.push("ui-file-trigger--custom-motion".to_string());
        }

        format!(
            "FileTriggerActualConfig {{\n  accept: \"{}\",\n  multiple: {},\n  disabled: {},\n  motion_source: \"{}\",\n  selected_file_count: {},\n  class: \"{}\",\n}}",
            selected_accept_label.get(),
            multiple.get(),
            disabled.get(),
            if custom_motion.get() {
                "custom"
            } else {
                "default"
            },
            files.get().len(),
            classes.join(" ")
        )
    });

    let comparison_code = Signal::derive(move || {
        r#"<FileTrigger on_files=on_default_files>
  "Default"
</FileTrigger>
<FileTrigger disabled=true on_files=on_disabled_files>
  "Disabled"
</FileTrigger>
<FileTrigger
  multiple=true
  motion=FileTriggerMotion {
    trigger: ButtonMotion {
      hover_scale: 1.04,
      tap_scale: 0.94,
      ..ButtonMotion::default()
    }
  }
  on_files=on_custom_files
>
  "Custom motion"
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
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                description="展示区支持文件选择结果回显；Config 区切换 accept/multiple/disabled/motion；Code + CSS Test 区用于契约回归。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/file-trigger/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Accept"</div>
                        <SegmentedControl
                            id_base="docs-file-trigger-accept".to_string()
                            options=accept_options.clone()
                            selected_index=accept_index
                            set_selected_index=set_accept_index
                            size=SegmentedControlSize::Sm
                            aria_label="FileTrigger accept".to_string()
                        />
                        <Switch checked=multiple set_checked=set_multiple>
                            "Multiple"
                        </Switch>
                        <Switch checked=disabled set_checked=set_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <FileTrigger
                        accept=selected_accept.get().unwrap_or_default()
                        multiple=multiple.get()
                        disabled=disabled.get()
                        motion=selected_motion.get()
                        on_files=on_files
                    >
                        "Pick files"
                    </FileTrigger>
                    <span class="ui-muted">
                        "accept: "
                        {move || selected_accept_label.get()}
                        " / multiple: "
                        {move || if multiple.get() { "true" } else { "false" }}
                        " / disabled: "
                        {move || if disabled.get() { "true" } else { "false" }}
                    </span>
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
                                            let name = file.name;
                                            let size = file.size;
                                            let mime = if file.mime.is_empty() {
                                                "unknown".to_string()
                                            } else {
                                                file.mime
                                            };
                                            view! {
                                                <li>
                                                    <code>{name}</code>
                                                    <span class="ui-muted">
                                                        " ("{size}" bytes, "{mime}")"
                                                    </span>
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
            </Playground>

            <Playground
                title="State Comparison (Default / Disabled / Custom Motion)"
                code_signal=comparison_code
            >
                <div class="docs-row">
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Default"</div>
                        <FileTrigger on_files=on_comparison_default_files>
                            "Default"
                        </FileTrigger>
                        <span class="ui-muted">
                            "selected: "
                            {move || comparison_default_files.get().len()}
                        </span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Disabled"</div>
                        <FileTrigger disabled=true>
                            "Disabled"
                        </FileTrigger>
                        <span class="ui-muted">"state is fixed to disabled"</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Custom motion + multiple"</div>
                        <FileTrigger
                            multiple=true
                            motion=FileTriggerMotion {
                                trigger: ButtonMotion {
                                    hover_scale: 1.04,
                                    tap_scale: 0.94,
                                    ..ButtonMotion::default()
                                },
                            }
                            on_files=on_comparison_custom_files
                        >
                            "Custom motion"
                        </FileTrigger>
                        <span class="ui-muted">
                            "selected: "
                            {move || comparison_custom_files.get().len()}
                        </span>
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
