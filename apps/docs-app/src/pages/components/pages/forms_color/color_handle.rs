use super::*;

pub(crate) fn color_handle() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

    let hello_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-hello".to_string()
    color="#f59e0b".to_string()
  />
</div>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-idle".to_string()
    color="#f59e0b".to_string()
    x_percent=22.0
    y_percent=72.0
  />
  <ColorHandle
    id_base="docs-color-handle-focused".to_string()
    color="#10b981".to_string()
    is_focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorHandle
    id_base="docs-color-handle-dragging".to_string()
    color="#3b82f6".to_string()
    is_dragging=true
    x_percent=82.0
    y_percent=28.0
  />
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-handle-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"// ColorHandle does not expose a value/default_value/on_value_change axis.
// Controlled vs uncontrolled is N/A; both examples are external-props driven.
let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

<div class="docs-row">
  <div style=surface_style>
    <ColorHandle
      id_base="docs-color-handle-controlled-like".to_string()
      color="#10b981".to_string()
      is_focused=true
      x_percent=48.0
      y_percent=46.0
    />
  </div>
  <div style=surface_style>
    <ColorHandle
      id_base="docs-color-handle-uncontrolled-like".to_string()
      color="#10b981".to_string()
      is_focused=true
      x_percent=48.0
      y_percent=46.0
    />
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorHandle is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorHandle docs output mode: snapshot"
</div>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    is_disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_dragging=true
    is_loupe_visible=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-handle-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let (workbench_color_key, set_workbench_color_key) = signal("amber".to_string());
    let (workbench_x_percent, set_workbench_x_percent) = signal(68_f32);
    let (workbench_y_percent, set_workbench_y_percent) = signal(36_f32);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_focused, set_workbench_focused) = signal(true);
    let (workbench_dragging, set_workbench_dragging) = signal(false);
    let (workbench_show_loupe, set_workbench_show_loupe) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_motion_ms, set_workbench_motion_ms) = signal(180_u16);

    let workbench_code = Signal::derive(move || {
        let color = match workbench_color_key.get().as_str() {
            "mint" => "#10b981",
            "sky" => "#0ea5e9",
            "violet" => "#8b5cf6",
            _ => "#f59e0b",
        };
        let class_name_line = if workbench_custom_class.get() {
            "  class_name=\"docs-color-handle-custom\".into()\n".to_string()
        } else {
            String::new()
        };
        format!(
            "<ColorHandle\n  id_base=\"docs-color-handle-workbench\".into()\n  color=\"{color}\".into()\n  x_percent={:.1}\n  y_percent={:.1}\n  is_disabled={}\n  is_focused={}\n  is_dragging={}\n  is_loupe_visible={}\n  motion=ColorHandleMotion {{ duration_ms: {} }}\n{class_name_line}/>",
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_disabled.get(),
            workbench_focused.get(),
            workbench_dragging.get(),
            workbench_show_loupe.get(),
            workbench_motion_ms.get(),
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/color-handle/src/styles.rs */\n{}",
            ui::color::handle::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let color = match workbench_color_key.get().as_str() {
            "mint" => "#10b981",
            "sky" => "#0ea5e9",
            "violet" => "#8b5cf6",
            _ => "#f59e0b",
        };
        let is_disabled = workbench_disabled.get();
        let is_focused = workbench_focused.get();
        let is_dragging = workbench_dragging.get();
        let is_loupe_visible = workbench_show_loupe.get();
        let has_custom_class = workbench_custom_class.get();
        let class_name = if has_custom_class {
            Some("docs-color-handle-custom")
        } else {
            None
        };
        let state = if is_disabled {
            "disabled"
        } else if is_dragging {
            "dragging"
        } else if is_focused {
            "focused"
        } else {
            "color"
        };
        let loupe_visible = !is_disabled && is_loupe_visible && is_dragging;
        let mut classes = vec!["ui-color-handle".to_string()];
        if is_disabled {
            classes.push("ui-color-handle--disabled".to_string());
        }
        if is_focused {
            classes.push("ui-color-handle--focused".to_string());
        }
        if is_dragging {
            classes.push("ui-color-handle--dragging".to_string());
        }
        if has_custom_class {
            classes.push("ui-color-handle--custom-class".to_string());
            classes.push("docs-color-handle-custom".to_string());
        }
        format!(
            "ColorHandleActualConfig {{\n  id_base: \"docs-color-handle-workbench\",\n  color: {:?},\n  aria_label: Some(\"Workbench color handle\"),\n  lang: Some(\"en-US\"),\n  dir: Some(\"ltr\"),\n  class_name: {:?},\n  motion: ColorHandleMotion {{ duration_ms: {} }},\n  state: \"{state}\",\n  x_percent: {:.1},\n  y_percent: {:.1},\n  is_disabled: {is_disabled},\n  is_focused: {is_focused},\n  is_dragging: {is_dragging},\n  is_loupe_visible: {is_loupe_visible},\n  loupe_visible: {loupe_visible},\n  motion_duration_ms: {},\n  class: \"{}\",\n}}",
            color,
            class_name,
            workbench_motion_ms.get(),
            workbench_x_percent.get(),
            workbench_y_percent.get(),
            workbench_motion_ms.get(),
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="ColorHandle"
            slug="color-handle"
            group="Forms"
            description="baseline-compatible draggable color handle primitive with composed thumb/loupe behavior, centralized state derivation, and stable slot/data-state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-hello".to_string()
                        color="#f59e0b".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Parameter Matrix Workbench (Display + Config + Code + CSS Test)"
                description="参数矩阵 + 状态矩阵联动工作台：交互配置、copy-ready code、scoped CSS test。"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-handle/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div
                        class="docs-stack docs-stack--tight"
                        data-slot="color-handle-workbench-controls"
                        data-parameter-matrix="color-handle"
                    >
                        <label class="docs-search__label">
                            "Color"
                            <select
                                data-slot="color-handle-workbench-color"
                                prop:value=move || workbench_color_key.get()
                                on:change=move |ev| set_workbench_color_key.set(event_target_value(&ev))
                            >
                                <option value="amber">"Amber"</option>
                                <option value="mint">"Mint"</option>
                                <option value="sky">"Sky"</option>
                                <option value="violet">"Violet"</option>
                            </select>
                        </label>

                        <label class="docs-search__label">
                            "X position "
                            <input
                                data-slot="color-handle-workbench-x"
                                type="range"
                                min="0"
                                max="100"
                                prop:value=move || format!("{:.0}", workbench_x_percent.get())
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                        set_workbench_x_percent.set(next.clamp(0.0, 100.0));
                                    }
                                }
                            />
                        </label>

                        <label class="docs-search__label">
                            "Y position "
                            <input
                                data-slot="color-handle-workbench-y"
                                type="range"
                                min="0"
                                max="100"
                                prop:value=move || format!("{:.0}", workbench_y_percent.get())
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<f32>() {
                                        set_workbench_y_percent.set(next.clamp(0.0, 100.0));
                                    }
                                }
                            />
                        </label>

                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-disabled"
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-focused"
                                type="checkbox"
                                prop:checked=move || workbench_focused.get()
                                on:change=move |ev| set_workbench_focused.set(event_target_checked(&ev))
                            />
                            " Focused"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-dragging"
                                type="checkbox"
                                prop:checked=move || workbench_dragging.get()
                                on:change=move |ev| set_workbench_dragging.set(event_target_checked(&ev))
                            />
                            " Dragging"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-show-loupe"
                                type="checkbox"
                                prop:checked=move || workbench_show_loupe.get()
                                on:change=move |ev| set_workbench_show_loupe.set(event_target_checked(&ev))
                            />
                            " Show loupe"
                        </label>
                        <label class="docs-search__label">
                            <input
                                data-slot="color-handle-workbench-custom-class"
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            "Motion (ms) "
                            <input
                                data-slot="color-handle-workbench-motion"
                                type="range"
                                min="60"
                                max="600"
                                prop:value=move || workbench_motion_ms.get().to_string()
                                on:input=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<u16>() {
                                        set_workbench_motion_ms.set(next.clamp(60, 600));
                                    }
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-handle-workbench">
                    <span class="ui-muted" data-slot="color-handle-api-defaults">
                        "API defaults: is_loupe_visible=true, x_percent=50.0, y_percent=50.0, motion=ColorHandleMotion::default()."
                    </span>
                    <span class="ui-muted">
                        "display: baseline vs configured"
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"Baseline"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-baseline".to_string()
                                    color="#f59e0b".to_string()
                                    x_percent=44.0
                                    y_percent=56.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Configured"</div>
                            <div style=surface_style>
                                {move || {
                                    let color = match workbench_color_key.get().as_str() {
                                        "mint" => "#10b981",
                                        "sky" => "#0ea5e9",
                                        "violet" => "#8b5cf6",
                                        _ => "#f59e0b",
                                    };
                                    let class_name = if workbench_custom_class.get() {
                                        "docs-color-handle-custom".to_string()
                                    } else {
                                        String::new()
                                    };
                                    let motion = ColorHandleMotion {
                                        duration_ms: workbench_motion_ms.get(),
                                    };
                                    view! {
                                        <ColorHandle
                                            id_base="docs-color-handle-workbench".to_string()
                                            color=color.to_string()
                                            aria_label="Workbench color handle".to_string()
                                            lang="en-US".to_string()
                                            dir=A11yDirection::Ltr
                                            is_disabled=workbench_disabled.get()
                                            is_focused=workbench_focused.get()
                                            is_dragging=workbench_dragging.get()
                                            is_loupe_visible=workbench_show_loupe.get()
                                            x_percent=workbench_x_percent.get()
                                            y_percent=workbench_y_percent.get()
                                            class_name=class_name
                                            motion=motion
                                        />
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Disabled / Focus / Drag Comparison)"
                code_signal=state_matrix_code
            >
                <div style=surface_style data-slot="color-handle-state-matrix-after-workbench">
                    <ColorHandle
                        id_base="docs-color-handle-idle-after-workbench".to_string()
                        color="#f59e0b".to_string()
                        aria_label="Idle color handle".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-focused-after-workbench".to_string()
                        color="#10b981".to_string()
                        aria_label="Focused color handle".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-dragging-after-workbench".to_string()
                        color="#3b82f6".to_string()
                        aria_label="Dragging color handle".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                        class_name="docs-color-handle-custom".to_string()
                        motion=ColorHandleMotion { duration_ms: 240 }
                    />
                </div>
            </Playground>

            <Playground title="State Variants" code_signal=state_matrix_code>
                <div style=surface_style data-slot="color-handle-state-matrix">
                    <ColorHandle
                        id_base="docs-color-handle-idle".to_string()
                        color="#f59e0b".to_string()
                        x_percent=22.0
                        y_percent=72.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-focused".to_string()
                        color="#10b981".to_string()
                        is_focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-dragging".to_string()
                        color="#3b82f6".to_string()
                        is_dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorHandle has no value/default_value/on_value_change axis; both examples are external-props driven."
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"External props path A"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-controlled-like".to_string()
                                    color="#10b981".to_string()
                                    is_focused=true
                                    x_percent=48.0
                                    y_percent=46.0
                                />
                            </div>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"External props path B"</div>
                            <div style=surface_style>
                                <ColorHandle
                                    id_base="docs-color-handle-uncontrolled-like".to_string()
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

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-handle-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorHandle is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=surface_style>
                        <ColorHandle
                            id_base="docs-color-handle-snapshot".to_string()
                            color="#334155".to_string()
                            x_percent=44.0
                            y_percent=56.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_dragging=true
                        is_loupe_visible=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-handle-copy-ready">
                <h3>"Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-handle/src/view.rs"</code>
                    ", "
                    <code>"components/color-handle/src/logic.rs"</code>
                    ", "
                    <code>"components/color-handle/src/styles.rs"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
