use super::*;

pub(crate) fn circular_progress() -> AnyView {
    let hello_world_code = Signal::derive(move || r#"<CircularProgress />"#.to_string());

    let matrix_code = Signal::derive(move || {
        r#"<CircularProgress aria_label="Loading".to_string() />
<CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
<CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
<CircularProgress aria_label="Syncing mail".to_string() size_px=30.0 thickness_px=4.0 />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<CircularProgress
  aria_label="Background refresh".to_string()
  size_px=28.0
  thickness_px=3.5
  class_name="docs-circular-progress-custom".to_string()
/>
<CircularProgress aria_label="   ".to_string() class_name="docs-circular-progress-custom".to_string() />"#.to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let upstream_label = "Syncing mail".to_string();

<CircularProgress />
<CircularProgress aria_label=upstream_label size_px=24.0 />
// CircularProgress has no controlled/uncontrolled runtime axis.
// App state maps directly to props; no value/on_change/default triplet."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<CircularProgress aria_label="Snapshot".to_string() />
// Streaming Optional; fallback=snapshot.
// CircularProgress renders complete validated snapshot output with stable semantic attrs."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<CircularProgress
  aria_label="Syncing mailbox".to_string()
  size_px=24.0
  thickness_px=3.0
/>"#
        .to_string()
    });

    let (workbench_size_px, set_workbench_size_px) = signal(None::<f64>);
    let (workbench_thickness_px, set_workbench_thickness_px) = signal(None::<f64>);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let size_px = workbench_size_px.get();
        let thickness_px = workbench_thickness_px.get();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let mut lines = vec!["<CircularProgress".to_string()];
        if custom_label {
            lines.push("  aria_label=\"Workbench sync\".to_string()".to_string());
        }
        if let Some(size_px) = size_px {
            lines.push(format!("  size_px={size_px}"));
        }
        if let Some(thickness_px) = thickness_px {
            lines.push(format!("  thickness_px={thickness_px}"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-circular-progress-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let aria_label = if workbench_custom_label.get() {
            "Workbench sync"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-circular-progress-custom"
        } else {
            ""
        };
        let lang = if workbench_rtl.get() { "ar" } else { "" };

        format!(
            "CircularProgressWorkbenchConfig {{\n  aria_label: {:?},\n  size_px: {:?},\n  thickness_px: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  size_source: {:?},\n  thickness_source: {:?},\n  label_source: {:?},\n  class_source: {:?},\n}}",
            aria_label,
            workbench_size_px.get(),
            workbench_thickness_px.get(),
            class_name,
            lang,
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
            if workbench_size_px.get().is_some() {
                "custom"
            } else {
                "default"
            },
            if workbench_thickness_px.get().is_some() {
                "custom"
            } else {
                "default"
            },
            if workbench_custom_label.get() {
                "custom"
            } else {
                "default"
            },
            if workbench_custom_class.get() {
                "custom"
            } else {
                "default"
            },
        )
    });

    view! {
        <ComponentPage
            title="CircularProgress"
            slug="circular-progress"
            group="Display"
            description="Indeterminate circular progress with centralized size/thickness/label source attrs."
        >
            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props / State / Preview)"
                description="在线调整 props（size/thickness/label/class/lang/dir）并实时预览语义标记变化；组件本身无内部受控状态轴。"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::color::area::A11yDirection;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="circular-progress-workbench-controls">
                            <div class="docs-search__label">"Size"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-size-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-default"
                                    on:click=move |_| set_workbench_size_px.set(None)
                                >
                                    "Default"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-24"
                                    on:click=move |_| set_workbench_size_px.set(Some(24.0))
                                >
                                    "24"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-size-32"
                                    on:click=move |_| set_workbench_size_px.set(Some(32.0))
                                >
                                    "32"
                                </button>
                            </div>

                            <div class="docs-search__label">"Thickness"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-thickness-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-default"
                                    on:click=move |_| set_workbench_thickness_px.set(None)
                                >
                                    "Default"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-3"
                                    on:click=move |_| set_workbench_thickness_px.set(Some(3.0))
                                >
                                    "3"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-thickness-4"
                                    on:click=move |_| set_workbench_thickness_px.set(Some(4.0))
                                >
                                    "4"
                                </button>
                            </div>

                            <div class="docs-search__label">"Label source"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-label-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-label-default"
                                    on:click=move |_| set_workbench_custom_label.set(false)
                                >
                                    "Default label"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-label-custom"
                                    on:click=move |_| set_workbench_custom_label.set(true)
                                >
                                    "Custom label"
                                </button>
                            </div>

                            <div class="docs-search__label">"Class source"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-class-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-class-default"
                                    on:click=move |_| set_workbench_custom_class.set(false)
                                >
                                    "Default class"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-class-custom"
                                    on:click=move |_| set_workbench_custom_class.set(true)
                                >
                                    "Custom class"
                                </button>
                            </div>

                            <div class="docs-search__label">"Direction"</div>
                            <div class="docs-row" data-slot="circular-progress-workbench-dir-controls">
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-dir-ltr"
                                    on:click=move |_| set_workbench_rtl.set(false)
                                >
                                    "LTR"
                                </button>
                                <button
                                    type="button"
                                    data-slot="circular-progress-workbench-dir-rtl"
                                    on:click=move |_| set_workbench_rtl.set(true)
                                >
                                    "RTL"
                                </button>
                            </div>
                        </div>
                    }
                }
            >
                {move || {
                    let size_px = workbench_size_px.get();
                    let thickness_px = workbench_thickness_px.get();
                    let custom_label = workbench_custom_label.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let aria_label = if custom_label {
                        "Workbench sync".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-circular-progress-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    let size_source = if size_px.is_some() { "custom" } else { "default" };
                    let thickness_source = if thickness_px.is_some() {
                        "custom"
                    } else {
                        "default"
                    };
                    let label_source = if custom_label { "custom" } else { "default" };
                    let class_source = if custom_class { "custom" } else { "default" };
                    let dir_label = if rtl { "rtl" } else { "ltr" };
                    let configured_progress = match (size_px, thickness_px) {
                        (Some(size_px), Some(thickness_px)) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                size_px=size_px
                                thickness_px=thickness_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (Some(size_px), None) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                size_px=size_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (None, Some(thickness_px)) => view! {
                            <CircularProgress
                                aria_label=aria_label.clone()
                                thickness_px=thickness_px
                                class_name=class_name.clone()
                                lang=lang.clone()
                                dir=dir
                            />
                        }
                        .into_any(),
                        (None, None) => view! {
                            <CircularProgress
                                aria_label=aria_label
                                class_name=class_name
                                lang=lang
                                dir=dir
                            />
                        }
                        .into_any(),
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="circular-progress-workbench-preview">
                            <p class="ui-muted" data-slot="circular-progress-workbench-state">
                                {format!(
                                    "size_source={size_source}; thickness_source={thickness_source}; label_source={label_source}; class_source={class_source}; dir={dir_label}"
                                )}
                            </p>
                            {configured_progress}
                        </div>
                    }
                }}
            </Playground>

            // <Playground title="Custom Label + Class" code_signal=custom_code>
            <Playground
                title="State Matrix (Size / Thickness / Locale Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::color::area::A11yDirection;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress aria_label="Loading".to_string() />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                    <CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
                    <CircularProgress
                        aria_label="Syncing mail".to_string()
                        size_px=30.0
                        thickness_px=4.0
                        class_name="docs-circular-progress-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground
                title="Size + Thickness Matrix"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress aria_label="Loading".to_string() />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                    <CircularProgress aria_label="Syncing mail".to_string() thickness_px=3.0 />
                    <CircularProgress
                        aria_label="Syncing mail".to_string()
                        size_px=30.0
                        thickness_px=4.0
                    />
                </div>
            </Playground>

            <Playground
                title="Custom Label + Class"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress
                        aria_label="Background refresh".to_string()
                        size_px=28.0
                        thickness_px=3.5
                        class_name="docs-circular-progress-custom".to_string()
                    />
                    <CircularProgress
                        aria_label="   ".to_string()
                        class_name="docs-circular-progress-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="CircularProgress has no internal controlled/uncontrolled axis; compare default usage with app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress />
                    <CircularProgress aria_label="Syncing mail".to_string() size_px=24.0 />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="CircularProgress is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="circular-progress-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="circular-progress-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/circular-progress/src/view.rs."
                    </p>
                    <CircularProgress aria_label="Snapshot".to_string() />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports="use leptos::prelude::*;\nuse ui::CircularProgress;"
                    .to_string()
                test_source_path="components/circular-progress/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <CircularProgress
                        aria_label="Syncing mailbox".to_string()
                        size_px=24.0
                        thickness_px=3.0
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="circular-progress-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="circular-progress-source-first-contract">
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="circular-progress-source-prerequisites">
                    <li>
                        "Dependency prerequisites: enable "
                        <code>"component-circular_progress"</code>
                        " + "
                        <code>"inject-css"</code>
                        " in package mode."
                    </li>
                    <li>
                        "Runtime style prerequisite: mount "
                        <code>"UiRoot"</code>
                        " (or equivalent CSS injection path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy circular-progress starter".to_string()
                    copyable=true
                    class_name="docs-circular-progress-source-copy".to_string()
                />
                <p>"Source paths:"</p>
                <ul data-slot="circular-progress-source-paths">
                    <li><code>"components/circular-progress/src/mod.rs"</code></li>
                    <li><code>"components/circular-progress/src/logic.rs"</code></li>
                    <li><code>"components/circular-progress/src/view.rs"</code></li>
                    <li><code>"components/circular-progress/src/styles.rs"</code></li>
                </ul>
                <p class="ui-muted" data-slot="circular-progress-source-sync-note">
                    "Sync note: snippet text is sourced from "
                    <code>"source_first_code"</code>
                    " and mirrors "
                    <code>"components/circular-progress/src/view.rs"</code>
                    " API usage; update docs snippet and source implementation together to avoid drift."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="circular-progress-docs-sync-matrix">
                <h3>"State Matrix"</h3>
                <ul>
                    <li><code>"data-state"</code>" = indeterminate（固定快照态）"</li>
                    <li>
                        <code>"data-size-source / data-thickness-source / data-label-source / data-class-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A（CircularProgress 无内部受控/非受控状态轴）"
                    </li>
                </ul>

                <h3>"Parameter Matrix"</h3>
                <ul>
                    <li>
                        <code>"aria_label: Option&lt;String&gt;"</code>
                        " default = None；`logic.rs::resolve_component_contract` 归一到 i18n `loading_aria_label`，空白回退 `DEFAULT_ARIA_LABEL`"
                    </li>
                    <li>
                        <code>"size_px / thickness_px: Option&lt;f64&gt;"</code>
                        " default = None；仅 finite 且 > 0 时生效，否则回落 default source"
                    </li>
                    <li>
                        <code>"class_name / lang: Option&lt;String&gt;"</code>
                        " default = None；空白字符串经 `normalize_optional_text` 归一为 None"
                    </li>
                    <li>
                        <code>"dir: Option&lt;A11yDirection&gt;"</code>
                        " default = None（继承 `UiRoot` locale direction 上下文）"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
