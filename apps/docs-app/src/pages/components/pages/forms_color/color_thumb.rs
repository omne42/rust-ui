use super::*;

pub(crate) fn color_thumb() -> AnyView {
    #[derive(Clone, Debug, serde::Deserialize)]
    struct ColorThumbWorkbenchSpec {
        color: Option<String>,
        x_percent: Option<f32>,
        y_percent: Option<f32>,
        is_disabled: Option<bool>,
        is_focused: Option<bool>,
        is_dragging: Option<bool>,
        is_loupe_visible: Option<bool>,
        class_name: Option<String>,
    }

    let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";
    let color_thumb_imports = "use leptos::prelude::*;\nuse ui::ColorThumb;".to_string();
    let (workbench_color, set_workbench_color) = signal("#10b981".to_string());
    let (workbench_x_percent, set_workbench_x_percent) = signal(48.0_f32);
    let (workbench_y_percent, set_workbench_y_percent) = signal(46.0_f32);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(true);
    let (workbench_dragging, set_workbench_dragging) = signal(false);
    let (workbench_loupe_visible, set_workbench_loupe_visible) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_replay_count, set_workbench_replay_count) = signal(0_u32);
    let (workbench_spec_input, set_workbench_spec_input) = signal(
        r##"{"color":"#0ea5e9","x_percent":66.0,"y_percent":34.0,"is_focused":true}"##.to_string(),
    );
    let workbench_spec = Signal::derive(move || {
        serde_json::from_str::<ColorThumbWorkbenchSpec>(&workbench_spec_input.get()).ok()
    });

    let hello_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb id_base="docs-color-thumb-hello".to_string() />
</div>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorThumb
    id_base="docs-color-thumb-focused".to_string()
    color="#10b981".to_string()
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
    x_percent=82.0
    y_percent=28.0
  />
</div>"##.to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div style=board_style data-slot="color-thumb-state-matrix">
  <ColorThumb
    id_base="docs-color-thumb-matrix-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-focused".to_string()
    color="#10b981".to_string()
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
    x_percent=82.0
    y_percent=28.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-matrix-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"<div class="docs-stack docs-stack--tight">
  <span class="ui-muted">
    "ColorThumb has no value/default_value/on_value_change axis; both examples are external-props driven."
  </span>
  <div class="docs-row" data-slot="color-thumb-controlled-vs-uncontrolled">
    <div class="docs-card">
      <div class="ui-muted">"External props path A"</div>
      <div style=board_style>
        <ColorThumb
          id_base="docs-color-thumb-controlled-like".to_string()
          color="#10b981".to_string()
          is_focused=true
          x_percent=48.0
          y_percent=46.0
        />
      </div>
    </div>
    <div class="docs-card">
      <div class="ui-muted">"External props path B"</div>
      <div style=board_style>
        <ColorThumb
          id_base="docs-color-thumb-uncontrolled-like".to_string()
          color="#10b981".to_string()
          is_focused=true
          x_percent=48.0
          y_percent=46.0
        />
      </div>
    </div>
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorThumb is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorThumb
    id_base="docs-color-thumb-snapshot".to_string()
    color="#334155".to_string()
    x_percent=44.0
    y_percent=56.0
  />
</div>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"<div style=board_style>
  <ColorThumb
    id_base="docs-color-thumb-source-first".to_string()
    color="#14b8a6".to_string()
    is_focused=true
    x_percent=58.0
    y_percent=38.0
  />
</div>"##
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let motion_line = "    motion=ColorThumbMotion::default()\n";
        let lang_line = if workbench_lang_zh.get() {
            "    lang=\"zh-CN\".to_string()\n"
        } else {
            "    lang=\"en-US\".to_string()\n"
        };
        let dir_line = if workbench_rtl.get() {
            "    dir=A11yDirection::Rtl\n"
        } else {
            "    dir=A11yDirection::Ltr\n"
        };
        let class_name_line = if workbench_custom_class.get() {
            "    class_name=\"docs-color-thumb-workbench\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<div style=board_style>\n  <ColorThumb\n    id_base=\"docs-color-thumb-workbench\".to_string()\n    color=\"{}\".to_string()\n    is_disabled={}\n    is_focused={}\n    is_dragging={}\n    is_loupe_visible={}\n    x_percent={:.1}\n    y_percent={:.1}\n    aria_label=\"Color thumb\".to_string()\n    aria_value_text={:?}.to_string()\n{}{}{}{}  />\n</div>\n\n// replay_count={}; spec_valid={}",
            workbench_color.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_loupe_visible.get(),
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            format!(
                "{} @ ({:.1}%, {:.1}%)",
                workbench_color.get(),
                workbench_x_percent.get(),
                workbench_y_percent.get()
            ),
            motion_line,
            lang_line,
            dir_line,
            class_name_line,
            workbench_replay_count.get(),
            if workbench_spec.get().is_some() {
                "true"
            } else {
                "false"
            },
        )
    });
    let workbench_test_css_source = Signal::derive(move || {
        r#"
:scope .docs-color-thumb-workbench[data-state="focused"] .ui-color-thumb__handle {
  box-shadow: 0 0 0 2px color-mix(in oklch, var(--ui-accent), white 24%);
}

:scope .docs-color-thumb-workbench[data-state="dragging"] .ui-color-thumb__loupe {
  transform: translateY(-0.25rem);
}
"#
        .trim()
        .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ColorThumbWorkbenchConfig {{\n  id_base: \"docs-color-thumb-workbench\",\n  color: Some({:?}),\n  is_disabled: {},\n  is_focused: {},\n  is_dragging: {},\n  x_percent: Some({:.1}),\n  y_percent: Some({:.1}),\n  is_loupe_visible: Some({}),\n  motion: {:?},\n  aria_label: Some(\"Color thumb\"),\n  aria_value_text: Some({:?}),\n  class_name: {:?},\n  lang: Some({:?}),\n  dir: Some({:?}),\n  replay_count: {},\n  spec_valid: {},\n}}",
            workbench_color.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_loupe_visible.get(),
            ColorThumbMotion::default(),
            format!(
                "{} @ ({:.1}%, {:.1}%)",
                workbench_color.get(),
                workbench_x_percent.get(),
                workbench_y_percent.get()
            ),
            if workbench_custom_class.get() {
                Some("docs-color-thumb-workbench")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            workbench_replay_count.get(),
            workbench_spec.get().is_some(),
        )
    });

    view! {
        <ComponentPage
            title="ColorThumb"
            slug="color-thumb"
            group="Forms"
            description="baseline-compatible draggable color thumb primitive with focus/drag/loupe state contracts, sanitized color source handling, and stable slot/data-state markers."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div style=board_style>
                    <ColorThumb id_base="docs-color-thumb-hello".to_string() />
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="在线 props/状态调节 + key-flow 回放 + Spec JSON 联动预览；作为可重复验收面。"
                code_signal=workbench_code
                code_imports=color_thumb_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-thumb/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench-controls">
                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-color-label">
                            "Color"
                        </label>
                        <input
                            type="text"
                            data-slot="color-thumb-workbench-input-color"
                            prop:value=move || workbench_color.get()
                            on:input=move |ev| set_workbench_color.set(event_target_value(&ev))
                        />

                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-x-label">
                            "X Percent"
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="1"
                            data-slot="color-thumb-workbench-input-x"
                            prop:value=move || workbench_x_percent.get().to_string()
                            on:input=move |ev| {
                                if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                    set_workbench_x_percent.set(next.clamp(0.0, 100.0));
                                }
                            }
                        />

                        <label class="docs-search__label" data-slot="color-thumb-workbench-input-y-label">
                            "Y Percent"
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            step="1"
                            data-slot="color-thumb-workbench-input-y"
                            prop:value=move || workbench_y_percent.get().to_string()
                            on:input=move |ev| {
                                if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                    set_workbench_y_percent.set(next.clamp(0.0, 100.0));
                                }
                            }
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_focused set_checked=set_workbench_focused>
                            "Focused"
                        </Switch>
                        <Switch checked=workbench_dragging set_checked=set_workbench_dragging>
                            "Dragging"
                        </Switch>
                        <Switch checked=workbench_loupe_visible set_checked=set_workbench_loupe_visible>
                            "Show Loupe"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>

                        <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench-replay-controls">
                            <button
                                type="button"
                                data-slot="color-thumb-workbench-replay-idle"
                                on:click=move |_| {
                                    set_workbench_disabled.set(false);
                                    set_workbench_focused.set(false);
                                    set_workbench_dragging.set(false);
                                    set_workbench_loupe_visible.set(true);
                                    set_workbench_x_percent.set(44.0);
                                    set_workbench_y_percent.set(56.0);
                                    set_workbench_replay_count.update(|count| *count += 1);
                                }
                            >
                                "Replay Idle Path"
                            </button>
                            <button
                                type="button"
                                data-slot="color-thumb-workbench-replay-drag"
                                on:click=move |_| {
                                    set_workbench_disabled.set(false);
                                    set_workbench_focused.set(true);
                                    set_workbench_dragging.set(true);
                                    set_workbench_loupe_visible.set(false);
                                    set_workbench_x_percent.set(78.0);
                                    set_workbench_y_percent.set(26.0);
                                    set_workbench_replay_count.update(|count| *count += 1);
                                }
                            >
                                "Replay Drag Path"
                            </button>
                        </div>

                        <label class="docs-search__label" data-slot="color-thumb-workbench-spec-input-label">
                            "Spec JSON"
                        </label>
                        <textarea
                            class="playground__test-editor"
                            data-slot="color-thumb-workbench-spec-input"
                            prop:value=move || workbench_spec_input.get()
                            on:input=move |ev| set_workbench_spec_input.set(event_target_value(&ev))
                        ></textarea>
                    </div>
                }
            >
                {move || {
                    let parsed_spec = workbench_spec.get();
                    let spec_status = if parsed_spec.is_some() {
                        "ok"
                    } else {
                        "invalid-json"
                    };
                    let spec_color = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.color.clone())
                        .unwrap_or_else(|| "#0ea5e9".to_string());
                    let spec_x = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.x_percent)
                        .unwrap_or(66.0);
                    let spec_y = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.y_percent)
                        .unwrap_or(34.0);
                    let spec_disabled = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_disabled)
                        .unwrap_or(false);
                    let spec_focused = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_focused)
                        .unwrap_or(true);
                    let spec_dragging = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_dragging)
                        .unwrap_or(false);
                    let spec_loupe_visible = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.is_loupe_visible)
                        .unwrap_or(true);
                    let spec_class_name = parsed_spec
                        .as_ref()
                        .and_then(|spec| spec.class_name.clone())
                        .unwrap_or_default();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-thumb-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-thumb-workbench-canvas">
                                <div style=board_style>
                                    <ColorThumb
                                        id_base="docs-color-thumb-workbench".to_string()
                                        color=workbench_color.get()
                                        is_disabled=workbench_disabled.get()
                                        is_focused=workbench_focused.get()
                                        is_dragging=workbench_dragging.get()
                                        is_loupe_visible=workbench_loupe_visible.get()
                                        x_percent=workbench_x_percent.get()
                                        y_percent=workbench_y_percent.get()
                                        motion=if workbench_reduced_motion.get() {
                                            ColorThumbMotion::disabled()
                                        } else {
                                            ColorThumbMotion::default()
                                        }
                                        aria_label="Color thumb".to_string()
                                        aria_value_text=format!(
                                            "{} @ ({:.1}%, {:.1}%)",
                                            workbench_color.get(),
                                            workbench_x_percent.get(),
                                            workbench_y_percent.get()
                                        )
                                        lang=if workbench_lang_zh.get() {
                                            "zh-CN".to_string()
                                        } else {
                                            "en-US".to_string()
                                        }
                                        dir=if workbench_rtl.get() {
                                            A11yDirection::Rtl
                                        } else {
                                            A11yDirection::Ltr
                                        }
                                        class_name=if workbench_custom_class.get() {
                                            "docs-color-thumb-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                </div>
                            </div>

                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-thumb-workbench-spec-preview">
                                <div class="ui-muted">
                                    "Spec input -> preview"
                                </div>
                                <div style=board_style>
                                    <ColorThumb
                                        id_base="docs-color-thumb-workbench-spec".to_string()
                                        color=spec_color
                                        is_disabled=spec_disabled
                                        is_focused=spec_focused
                                        is_dragging=spec_dragging
                                        is_loupe_visible=spec_loupe_visible
                                        x_percent=spec_x
                                        y_percent=spec_y
                                        motion=ColorThumbMotion::default()
                                        aria_label="Spec color thumb".to_string()
                                        aria_value_text=format!(
                                            "{} @ ({:.1}%, {:.1}%)",
                                            workbench_color.get(),
                                            spec_x,
                                            spec_y
                                        )
                                        lang=if workbench_lang_zh.get() {
                                            "zh-CN".to_string()
                                        } else {
                                            "en-US".to_string()
                                        }
                                        dir=if workbench_rtl.get() {
                                            A11yDirection::Rtl
                                        } else {
                                            A11yDirection::Ltr
                                        }
                                        class_name=spec_class_name
                                    />
                                </div>
                                <span class="ui-muted" data-slot="color-thumb-workbench-spec-state">
                                    "spec: " {spec_status}
                                </span>
                            </div>

                            <span class="ui-muted" data-slot="color-thumb-workbench-state">
                                "x: " {format!("{:.1}", workbench_x_percent.get())}
                                " · y: " {format!("{:.1}", workbench_y_percent.get())}
                                " · state: " {if workbench_disabled.get() {
                                    "disabled"
                                } else if workbench_dragging.get() {
                                    "dragging"
                                } else if workbench_focused.get() {
                                    "focused"
                                } else {
                                    "idle"
                                }}
                                " · replay: " {workbench_replay_count.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Focused + Dragging + Position" code_signal=basic_code>
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=color_thumb_imports.clone()
            >
                <div style=board_style data-slot="color-thumb-state-matrix">
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=color_thumb_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorThumb has no value/default_value/on_value_change axis; both examples are external-props driven."
                    </span>
                    <div class="docs-row" data-slot="color-thumb-controlled-vs-uncontrolled">
                        <div class="docs-card">
                            <div class="ui-muted">"External props path A"</div>
                            <div style=board_style>
                                <ColorThumb
                                    id_base="docs-color-thumb-controlled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"External props path B"</div>
                            <div style=board_style>
                                <ColorThumb
                                    id_base="docs-color-thumb-uncontrolled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports=color_thumb_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-thumb-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorThumb is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=board_style>
                        <ColorThumb
                            id_base="docs-color-thumb-snapshot".to_string()
                            color="#334155".to_string()
                            x_percent=44.0
                            y_percent=56.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=color_thumb_imports.clone()
            >
                <div style=board_style data-slot="color-thumb-state-matrix-final">
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-final-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                        motion=ColorThumbMotion::default()
                        aria_label="Idle thumb".to_string()
                        aria_value_text="amber @ (22%, 72%)".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-final-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                        motion=ColorThumbMotion::default()
                        aria_label="Focused thumb".to_string()
                        aria_value_text="emerald @ (52%, 44%)".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-final-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                        motion=ColorThumbMotion::default()
                        aria_label="Dragging thumb".to_string()
                        aria_value_text="blue @ (82%, 28%)".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-final-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                        motion=ColorThumbMotion::default()
                        aria_label="Disabled thumb".to_string()
                        aria_value_text="violet @ (30%, 56%)".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-matrix-final-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        motion=ColorThumbMotion::default()
                        aria_label="Custom thumb".to_string()
                        aria_value_text="cyan @ (70%, 40%)".to_string()
                        class_name="docs-color-thumb-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=color_thumb_imports.clone()
            >
                <div style=board_style>
                    <ColorThumb
                        id_base="docs-color-thumb-source-first".to_string()
                        color="#14b8a6".to_string()
                        is_focused=true
                        x_percent=58.0
                        y_percent=38.0
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-thumb-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-thumb-source-first-copy-action">
                    "Source-first usage stays copy-ready via "
                    <code>"Show code + Copy"</code>
                    ", and copied snippets are expected to run without manual import patching."
                </p>
                <p data-slot="color-thumb-source-paths">
                    "Source-first path: "
                    <code>"components/color-thumb/src/mod.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/view.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/logic.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/styles.rs"</code>
                    ", "
                    <code>"components/color-thumb/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-thumb-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_thumb"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
