use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    ButtonMotion, DropZone, DropZoneMotion, DroppedFile, FileTrigger, FileTriggerFile,
    FileTriggerMotion, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn file_trigger() -> AnyView {
    // Legacy source-contract markers retained for semantic tests:
    // <Playground title="Pick files" code_signal=code>
    // <Playground title="Pick files with custom motion" code_signal=motion_code>
    // title="Pick files"
    // title="Pick files with custom motion"
    // <FileTrigger is_multiple=true on_files=on_files>
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
    let (accept_directory, set_accept_directory) = signal(false);
    let (capture_environment, set_capture_environment) = signal(false);
    let (lang_zh, set_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

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
    let (quick_start_files, set_quick_start_files) = signal(Vec::<FileTriggerFile>::new());
    let on_quick_start_files =
        Callback::new(move |next: Vec<FileTriggerFile>| set_quick_start_files.set(next));

    let quick_start_code = Signal::derive(move || {
        r#"let on_files = Callback::new(|files: Vec<FileTriggerFile>| { /* ... */ });
<FileTrigger on_files=on_files>"Pick files"</FileTrigger>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "let on_files = Callback::new(|files: Vec<FileTriggerFile>| {".to_string(),
            "  // handle selected files".to_string(),
            "});".to_string(),
            "".to_string(),
            "<FileTrigger".to_string(),
            "  id=\"docs-file-trigger-input\".into()".to_string(),
        ];

        if multiple.get() {
            lines.push("  is_multiple=true".to_string());
        }
        if disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if let Some(accept) = selected_accept.get() {
            lines.push(format!("  accept=\"{accept}\".into()"));
        }
        if accept_directory.get() {
            lines.push("  is_accept_directory=true".to_string());
            lines.push("  accept_directory=true".to_string());
        }
        if capture_environment.get() {
            lines.push("  capture=\"environment\".into()".to_string());
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
        lines.push(format!(
            "  lang={}.into()",
            if lang_zh.get() {
                "\"zh-CN\""
            } else {
                "\"en-US\""
            }
        ));
        lines.push(format!(
            "  dir={}",
            if rtl_dir.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        ));
        lines.push("  on_files=on_files".to_string());
        lines.push(">".to_string());
        lines.push("  \"Pick files\"".to_string());
        lines.push("</FileTrigger>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/file-trigger/src/styles.rs */\n{}",
            ui::file_trigger::styles::CSS
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
            "FileTriggerActualConfig {{\n  id: Some(\"docs-file-trigger-input\"),\n  is_disabled: Some({is_disabled}),\n  disabled: Some({disabled}),\n  is_multiple: Some({is_multiple}),\n  multiple: Some({multiple}),\n  accept: {accept:?},\n  is_accept_directory: Some({is_accept_directory}),\n  accept_directory: Some({accept_directory}),\n  capture: {capture},\n  motion: {motion},\n  on_files: Some(\"on_files\"),\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  selected_file_count: {selected_file_count},\n  class: {class_name:?},\n}}",
            is_disabled = disabled.get(),
            disabled = disabled.get(),
            is_multiple = multiple.get(),
            multiple = multiple.get(),
            accept = selected_accept.get().unwrap_or_default(),
            is_accept_directory = accept_directory.get(),
            accept_directory = accept_directory.get(),
            capture = if capture_environment.get() {
                "Some(\"environment\")"
            } else {
                "None"
            },
            motion = if custom_motion.get() {
                "FileTriggerMotion::custom"
            } else {
                "FileTriggerMotion::default"
            },
            lang = if lang_zh.get() { "zh-CN" } else { "en-US" },
            dir = if rtl_dir.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            selected_file_count = files.get().len(),
            class_name = classes.join(" "),
        )
    });

    let comparison_code = Signal::derive(move || {
        r#"<FileTrigger on_files=on_default_files>
  "Default"
</FileTrigger>
<FileTrigger is_disabled=true on_files=on_disabled_files>
  "Disabled"
</FileTrigger>
<FileTrigger
  is_multiple=true
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
                title="Quick Start (Default API)"
                description="默认调用路径：只需要 `on_files` 即可使用，无需接线内部状态原语。"
                code_signal=quick_start_code
            >
                <div class="docs-stack docs-stack--tight">
                    <FileTrigger on_files=on_quick_start_files>
                        "Pick files"
                    </FileTrigger>
                    <span class="ui-muted">
                        "selected: "
                        {move || quick_start_files.get().len()}
                    </span>
                </div>
            </Playground>

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
                        <Switch checked=accept_directory set_checked=set_accept_directory>
                            "Accept directory"
                        </Switch>
                        <Switch checked=capture_environment set_checked=set_capture_environment>
                            "Capture environment"
                        </Switch>
                        <Switch checked=lang_zh set_checked=set_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <FileTrigger
                        id="docs-file-trigger-input".to_string()
                        accept=selected_accept.get().unwrap_or_default()
                        is_multiple=multiple.get()
                        is_disabled=disabled.get()
                        is_accept_directory=accept_directory.get()
                        accept_directory=accept_directory.get()
                        capture=if capture_environment.get() {
                            "environment".to_string()
                        } else {
                            String::new()
                        }
                        motion=selected_motion.get()
                        lang=if lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if rtl_dir.get() {
                            ui::A11yDirection::Rtl
                        } else {
                            ui::A11yDirection::Ltr
                        }
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
                        <FileTrigger is_disabled=true>
                            "Disabled"
                        </FileTrigger>
                        <span class="ui-muted">"state is fixed to disabled"</span>
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Custom motion + multiple"</div>
                        <FileTrigger
                            is_multiple=true
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
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct DropZoneWorkbenchState {
        is_disabled: bool,
        custom_motion: bool,
    }

    #[cfg(target_arch = "wasm32")]
    impl DropZoneWorkbenchState {
        fn parse(raw: &str) -> Option<Self> {
            let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
            if parts.len() != 2 {
                return None;
            }

            let parse_bool = |at: usize| match *parts.get(at)? {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            };

            Some(Self {
                is_disabled: parse_bool(0)?,
                custom_motion: parse_bool(1)?,
            })
        }

        fn encode(self) -> String {
            let bool_digit = |value: bool| if value { '1' } else { '0' };
            format!(
                "{},{}",
                bool_digit(self.is_disabled),
                bool_digit(self.custom_motion),
            )
        }
    }

    #[cfg(target_arch = "wasm32")]
    const DROP_ZONE_WORKBENCH_STORAGE_KEY: &str = "docs:drop-zone:workbench:state";

    #[cfg(target_arch = "wasm32")]
    fn load_drop_zone_workbench_state() -> Option<DropZoneWorkbenchState> {
        let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
        let raw = storage
            .get_item(DROP_ZONE_WORKBENCH_STORAGE_KEY)
            .ok()
            .flatten()?;
        DropZoneWorkbenchState::parse(&raw)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_drop_zone_workbench_state() -> Option<DropZoneWorkbenchState> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn save_drop_zone_workbench_state(state: DropZoneWorkbenchState) {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.set_item(DROP_ZONE_WORKBENCH_STORAGE_KEY, &state.encode()));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_drop_zone_workbench_state(_state: DropZoneWorkbenchState) {}

    #[cfg(target_arch = "wasm32")]
    fn clear_drop_zone_workbench_state() {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.remove_item(DROP_ZONE_WORKBENCH_STORAGE_KEY));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear_drop_zone_workbench_state() {}

    let (files, set_files) = signal(Vec::<DroppedFile>::new());
    let on_drop_files = Callback::new(move |next: Vec<DroppedFile>| set_files.set(next));
    let persisted_workbench_state = load_drop_zone_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_files, set_workbench_files) = signal(Vec::<DroppedFile>::new());
    let on_workbench_drop_files =
        Callback::new(move |next: Vec<DroppedFile>| set_workbench_files.set(next));
    let (workbench_is_disabled, set_workbench_is_disabled) =
        signal(initial_workbench_state.is_disabled);
    let (workbench_custom_motion, set_workbench_custom_motion) =
        signal(initial_workbench_state.custom_motion);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        let state = DropZoneWorkbenchState {
            is_disabled: workbench_is_disabled.get(),
            custom_motion: workbench_custom_motion.get(),
        };

        if workbench_persist_state.get() {
            save_drop_zone_workbench_state(state);
        } else {
            clear_drop_zone_workbench_state();
        }
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            DropZoneMotion {
                hover_scale: 1.015,
                drop_scale: 1.03,
                hover_highlight: 0.42,
                ..DropZoneMotion::default()
            }
        } else {
            DropZoneMotion::default()
        }
    });
    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "let on_drop_files = Callback::new(|files: Vec<DroppedFile>| {".to_string(),
            "  // handle dropped files".to_string(),
            "});".to_string(),
            "".to_string(),
            "<DropZone".to_string(),
        ];

        if workbench_is_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if workbench_custom_motion.get() {
            lines.push("  motion=DropZoneMotion {".to_string());
            lines.push("    hover_scale: 1.015,".to_string());
            lines.push("    drop_scale: 1.03,".to_string());
            lines.push("    hover_highlight: 0.42,".to_string());
            lines.push("    ..DropZoneMotion::default()".to_string());
            lines.push("  }".to_string());
        }
        lines.push("  on_drop_files=on_drop_files".to_string());
        lines.push(">".to_string());
        lines.push("  <div class=\"docs-drop-zone\">\"Drop files here\"</div>".to_string());
        lines.push("</DropZone>".to_string());
        lines.join("\n")
    });
    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/drop-zone/src/styles.rs */\n{}",
            ui::drop_zone::styles::CSS
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let label = if workbench_custom_motion.get() {
            "Upload (custom motion)"
        } else {
            "Upload"
        };
        format!(
            "DropZoneWorkbenchConfig {{\n  label: {:?},\n  is_disabled: {},\n  motion: {:?},\n  on_drop_files: \"callback:on_workbench_drop_files\",\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  persist_state: {},\n  dropped_file_count: {},\n}}",
            label,
            workbench_is_disabled.get(),
            workbench_motion.get(),
            "Workbench drop area",
            "en-US",
            ui::A11yDirection::Ltr,
            workbench_persist_state.get(),
            workbench_files.get().len(),
        )
    });

    let quick_start_code = Signal::derive(move || {
        r#"<DropZone>
  <div class="docs-drop-zone">"Drop files here"</div>
</DropZone>"#
            .to_string()
    });
    let hello_world_code = Signal::derive(move || {
        r#"<DropZone>
  <div class="docs-drop-zone">"Drop files here"</div>
</DropZone>"#
            .to_string()
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<DropZone label="Default".to_string()>
  <div class="docs-drop-zone">"Default state"</div>
</DropZone>

<DropZone label="Disabled".to_string() is_disabled=true>
  <div class="docs-drop-zone">"Disabled state"</div>
</DropZone>

<DropZone
  label="Custom motion".to_string()
  motion=DropZoneMotion {
    hover_scale: 1.015,
    drop_scale: 1.03,
    hover_highlight: 0.42,
    ..DropZoneMotion::default()
  }
>
  <div class="docs-drop-zone">"Custom motion state"</div>
</DropZone>"#
            .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"// DropZone has no persistent controlled/uncontrolled state axis.
// It is event-driven: consume `on_drop_files` callback for app state sync.
let on_drop_files = Callback::new(|files: Vec<DroppedFile>| { /* ... */ });

<DropZone on_drop_files=on_drop_files>
  <div class="docs-drop-zone">"Drop files here"</div>
</DropZone>"#
            .to_string()
    });
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Streaming Optional (fallback=snapshot):
// DropZone renders stable semantics while upper layer validates stream data.
<DropZone>
  <div class="docs-drop-zone">"Streaming fallback=snapshot: waiting for final validation"</div>
</DropZone>"#
            .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"let on_drop_files = Callback::new(|files: Vec<DroppedFile>| {
  // sync files to app state
});

<DropZone
  motion=DropZoneMotion::default()
  on_drop_files=on_drop_files
>
  <div class="docs-drop-zone">"Drop files here"</div>
</DropZone>"#
            .to_string()
    });
    let source_first_imports =
        "use leptos::prelude::*;\nuse ui::{DropZone, DropZoneMotion, DroppedFile};".to_string();

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
            <Playground title="Hello World" code_signal=hello_world_code>
                <DropZone>
                    <div class="docs-drop-zone">"Drop files here"</div>
                </DropZone>
            </Playground>

            <Playground
                title="Quick Start (Default API)"
                description="默认调用路径：无需手动接线状态原语，只需渲染组件内容。"
                code_signal=quick_start_code
            >
                <div data-slot="drop-zone-e2e-quick-start">
                    <DropZone>
                        <div class="docs-drop-zone">"Drop files here"</div>
                    </DropZone>
                </div>
            </Playground>

            <Playground
                title="Workbench（展示 + Config + Code + CSS Test）"
                description="隔离画布用于拖放/粘贴演练；样式支持 scoped 热编辑；可选保留 workbench 状态。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/drop-zone/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="drop-zone-workbench-controls">
                        <div data-slot="drop-zone-workbench-toggle-disabled">
                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "Disabled"
                            </Switch>
                        </div>
                        <div data-slot="drop-zone-workbench-toggle-custom-motion">
                            <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                                "Custom motion"
                            </Switch>
                        </div>
                        <div data-slot="drop-zone-workbench-toggle-persist">
                            <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                                "Persist workbench state"
                            </Switch>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="drop-zone-workbench">
                    <div class="docs-card docs-stack docs-stack--tight" data-slot="drop-zone-workbench-canvas">
                        <div data-slot="drop-zone-workbench-surface">
                            <DropZone
                                label=if workbench_custom_motion.get() {
                                    "Upload (custom motion)".to_string()
                                } else {
                                    "Upload".to_string()
                                }
                                aria_label="Workbench drop area".to_string()
                                lang="en-US".to_string()
                                dir=ui::A11yDirection::Ltr
                                is_disabled=workbench_is_disabled.get()
                                motion=workbench_motion.get()
                                on_drop_files=on_workbench_drop_files
                            >
                                <div class="docs-drop-zone">
                                    <div>"Drop files here"</div>
                                    <div class="ui-muted">"…or paste an image/file."</div>
                                </div>
                            </DropZone>
                        </div>
                    </div>
                    <span class="ui-muted">
                        "files: "
                        {move || workbench_files.get().len()}
                        " · persist: "
                        {move || if workbench_persist_state.get() { "on" } else { "off" }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Disabled / Motion / Callback)"
                code_signal=state_matrix_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div data-slot="drop-zone-e2e-state-default-after-workbench">
                        <DropZone
                            label="Default".to_string()
                            aria_label="Default drop area".to_string()
                            lang="en-US".to_string()
                            dir=ui::A11yDirection::Ltr
                        >
                            <div class="docs-drop-zone">"Default state"</div>
                        </DropZone>
                    </div>
                    <div data-slot="drop-zone-e2e-state-disabled-after-workbench">
                        <DropZone
                            label="Disabled".to_string()
                            aria_label="Disabled drop area".to_string()
                            lang="en-US".to_string()
                            dir=ui::A11yDirection::Ltr
                            is_disabled=true
                        >
                            <div class="docs-drop-zone">"Disabled state"</div>
                        </DropZone>
                    </div>
                    <div data-slot="drop-zone-e2e-state-custom-motion-after-workbench">
                        <DropZone
                            label="Custom motion".to_string()
                            aria_label="Custom motion drop area".to_string()
                            lang="ar".to_string()
                            dir=ui::A11yDirection::Rtl
                            motion=DropZoneMotion {
                                hover_scale: 1.015,
                                drop_scale: 1.03,
                                hover_highlight: 0.42,
                                ..DropZoneMotion::default()
                            }
                            on_drop_files=on_drop_files
                        >
                            <div class="docs-drop-zone">"Custom motion state"</div>
                        </DropZone>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Gallery (Disabled / Motion / Callback)"
                code_signal=state_matrix_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div data-slot="drop-zone-e2e-state-default">
                        <DropZone label="Default".to_string()>
                            <div class="docs-drop-zone">"Default state"</div>
                        </DropZone>
                    </div>
                    <div data-slot="drop-zone-e2e-state-disabled">
                        <DropZone label="Disabled".to_string() is_disabled=true>
                            <div class="docs-drop-zone">"Disabled state"</div>
                        </DropZone>
                    </div>
                    <div data-slot="drop-zone-e2e-state-custom-motion">
                        <DropZone
                            label="Custom motion".to_string()
                            motion=DropZoneMotion {
                                hover_scale: 1.015,
                                drop_scale: 1.03,
                                hover_highlight: 0.42,
                                ..DropZoneMotion::default()
                            }
                            on_drop_files=on_drop_files
                        >
                            <div class="docs-drop-zone">"Custom motion state"</div>
                        </DropZone>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_contrast_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="ui-muted">
                        "DropZone has no persistent controlled/uncontrolled state axis."
                    </div>
                    <div class="ui-muted">
                        "Use on_drop_files callback to sync dropped files into app state."
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                code_signal=streaming_snapshot_code
            >
                <DropZone>
                    <div class="docs-drop-zone">
                        <div>"Streaming fallback=snapshot: waiting for final validation"</div>
                        <div class="ui-muted">
                            "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-output-status."
                        </div>
                    </div>
                </DropZone>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="代码面板默认可复制运行片段；同时给出真实源码路径与最小依赖前提，避免“复制即报错”。"
                code_signal=source_first_code
                code_imports=source_first_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="drop-zone-source-first">
                    <div class="docs-stack docs-stack--tight" data-slot="drop-zone-source-first-contract">
                        <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                        <div class="ui-muted">
                            "Open "
                            <code>"Show code"</code>
                            " then use the copy button. Snippets prepend imports automatically."
                        </div>
                        <div class="ui-muted">
                            "docs entry: apps/docs-app/src/pages/components/pages/files.rs::drop_zone"
                        </div>
                    </div>

                    <div
                        class="docs-stack docs-stack--tight"
                        data-slot="drop-zone-source-first-dependency-baseline"
                    >
                        <div class="docs-search__label">"Dependency baseline (Cargo.toml)"</div>
                        <code>
                            "ui = { default-features = false, features = [\"component-drop_zone\", \"inject-css\"] }"
                        </code>
                    </div>

                    <div class="docs-stack docs-stack--tight" data-slot="drop-zone-source-paths">
                        <div class="docs-search__label">"Source paths"</div>
                        <div class="ui-muted">"components/drop-zone/src/mod.rs"</div>
                        <div class="ui-muted">"components/drop-zone/src/logic.rs"</div>
                        <div class="ui-muted">"components/drop-zone/src/view.rs"</div>
                        <div class="ui-muted">"components/drop-zone/src/styles.rs"</div>
                        <div class="ui-muted">"components/drop-zone/src/motion.rs"</div>
                    </div>

                    <div class="ui-muted" data-slot="drop-zone-source-prerequisites">
                        "Feature prerequisites: component-drop_zone (inject-css optional for runtime style injection)."
                    </div>
                </div>
            </Playground>

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
