use super::*;

pub(crate) fn color_editor() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));

    let (format, set_format) = signal(ColorEditorFormat::Hex);
    let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let format_signal: Signal<ColorEditorFormat> = format.into();

    let reduced_motion = ColorSliderMotion::disabled();

    let hello_code = Signal::derive(move || {
        r##"<ColorEditor
  id_base="docs-color-editor-hello".to_string()
/>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));

let (format, set_format) = signal(ColorEditorFormat::Hex);
let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

let selected_color_signal: Signal<Option<String>> = selected_color.into();
let format_signal: Signal<ColorEditorFormat> = format.into();

<ColorEditor
  id_base="docs-color-editor-basic".to_string()
  label="Color editor".to_string()
  selected_color=selected_color_signal
  on_selected_change=on_selected_change
  format=format_signal
  on_format_change=on_format_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorSliderMotion::disabled();

<ColorEditor
  id_base="docs-color-editor-disabled".to_string()
  label="Disabled editor".to_string()
  default_selected_color="#0ea5e9".to_string()
  default_format=ColorEditorFormat::Rgb
  is_alpha_channel_hidden=true
  is_disabled=true
  class_name="docs-color-editor-custom".to_string()
/>

<ColorEditor
  id_base="docs-color-editor-motion".to_string()
  label="Brand editor".to_string()
  default_format=ColorEditorFormat::Hsb
  default_hue=282.0
  default_alpha=64.0
  default_area=(0.46, 0.88)
  motion=reduced_motion
/>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-editor-state-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Ready"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-ready".to_string()
      label="Ready".to_string()
      default_selected_color="#4f46e5".to_string()
      default_format=ColorEditorFormat::Hex
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Empty"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-empty".to_string()
      label="Empty".to_string()
      default_format=ColorEditorFormat::Hsl
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Disabled + Hidden Alpha"</div>
    <ColorEditor
      id_base="docs-color-editor-matrix-disabled".to_string()
      label="Disabled".to_string()
      default_selected_color="#0ea5e9".to_string()
      default_format=ColorEditorFormat::Rgb
      is_alpha_channel_hidden=true
      is_disabled=true
      motion=ColorSliderMotion::disabled()
    />
  </div>
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));

let (format, set_format) = signal(ColorEditorFormat::Hex);
let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

<div class="docs-row">
  <div class="docs-card">
    <ColorEditor
      id_base="docs-color-editor-controlled".to_string()
      label="Controlled".to_string()
      selected_color=selected_color.into()
      on_selected_change=on_selected_change
      format=format.into()
      on_format_change=on_format_change
    />
  </div>
  <div class="docs-card">
    <ColorEditor
      id_base="docs-color-editor-uncontrolled".to_string()
      label="Uncontrolled".to_string()
      default_selected_color="#22c55e".to_string()
      default_format=ColorEditorFormat::Rgb
      default_hue=132.0
      default_alpha=92.0
      default_area=(0.62, 0.18)
    />
  </div>
</div>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorEditor is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorEditor
    id_base="docs-color-editor-snapshot".to_string()
    label="Snapshot fallback".to_string()
    default_selected_color="#334155".to_string()
    default_format=ColorEditorFormat::Hex
  />
</div>"##
            .to_string()
    });

    let workbench_format_options = vec![
        "hex".to_string(),
        "rgb".to_string(),
        "hsl".to_string(),
        "hsb".to_string(),
    ];
    let (workbench_format_index, set_workbench_format_index) = signal(Some(0usize));
    let workbench_format =
        Signal::derive(move || match workbench_format_index.get().unwrap_or(0) {
            1 => ColorEditorFormat::Rgb,
            2 => ColorEditorFormat::Hsl,
            3 => ColorEditorFormat::Hsb,
            _ => ColorEditorFormat::Hex,
        });
    let (workbench_selected_color, set_workbench_selected_color) =
        signal(Some("#4f46e5".to_string()));
    let (workbench_selected_change_count, set_workbench_selected_change_count) = signal(0_u32);
    let (workbench_format_change_count, set_workbench_format_change_count) = signal(0_u32);
    let on_workbench_format_change = Callback::new(move |next: ColorEditorFormat| {
        set_workbench_format_index.set(Some(match next {
            ColorEditorFormat::Hex => 0,
            ColorEditorFormat::Rgb => 1,
            ColorEditorFormat::Hsl => 2,
            ColorEditorFormat::Hsb => 3,
        }));
        set_workbench_format_change_count.update(|count| *count += 1);
    });
    let on_workbench_selected_change = Callback::new(move |next: Option<String>| {
        set_workbench_selected_color.set(next);
        set_workbench_selected_change_count.update(|count| *count += 1);
    });
    let workbench_selected_color_signal: Signal<Option<String>> = workbench_selected_color.into();
    let workbench_format_signal: Signal<ColorEditorFormat> = workbench_format;

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_hide_alpha, set_workbench_hide_alpha) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let workbench_code = Signal::derive(move || {
        let format = workbench_format.get();
        let format_literal = match format {
            ColorEditorFormat::Hex => "ColorEditorFormat::Hex",
            ColorEditorFormat::Rgb => "ColorEditorFormat::Rgb",
            ColorEditorFormat::Hsl => "ColorEditorFormat::Hsl",
            ColorEditorFormat::Hsb => "ColorEditorFormat::Hsb",
        };

        let mut lines = vec![
            "let (selected_color, set_selected_color) = signal(Some(\"#4f46e5\".into()));".to_string(),
            "let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));".to_string(),
            String::new(),
            format!("let (format, set_format) = signal({format_literal});"),
            "let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));".to_string(),
            String::new(),
            "<ColorEditor".to_string(),
            "  id_base=\"docs-color-editor-workbench\".into()".to_string(),
            "  selected_color=selected_color.into()".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            "  format=format.into()".to_string(),
            "  on_format_change=on_format_change".to_string(),
            "  aria_label=\"Brand palette editor\".into()".to_string(),
            "  default_selected_color=\"#4f46e5\".into()".to_string(),
            "  default_format=ColorEditorFormat::Hex".to_string(),
            "  default_hue=258.0".to_string(),
            "  default_alpha=88.0".to_string(),
            "  default_area=(0.55_f32, 0.22_f32)".to_string(),
            "  area_label=\"Saturation and brightness\".into()".to_string(),
            "  area_aria_label=\"Drag to change saturation and brightness\".into()".to_string(),
            "  hue_label=\"Hue\".into()".to_string(),
            "  alpha_label=\"Alpha\".into()".to_string(),
            "  value_label=\"Color value\".into()".to_string(),
            "  format_aria_label=\"Color format selector\".into()".to_string(),
            "  preview_color=\"#4f46e5\".into()".to_string(),
            "  lang=\"en-US\".into()".to_string(),
            "  dir=A11yDirection::Ltr".to_string(),
        ];

        if workbench_custom_label.get() {
            lines.push("  label=\"Brand color workspace\".into()".to_string());
        }
        if workbench_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if workbench_hide_alpha.get() {
            lines.push("  is_alpha_channel_hidden=true".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-editor-workbench\".into()".to_string());
        }
        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorSliderMotion::disabled()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/color-editor/src/styles.rs */\n{}",
            ui::color::editor::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let format = workbench_format.get();
        let is_disabled = workbench_disabled.get();
        let is_alpha_channel_hidden = workbench_hide_alpha.get();
        let has_selection = workbench_selected_color.get().is_some();
        let custom_label = workbench_custom_label.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();
        let selected_value = workbench_selected_color.get();
        let selected_value_text = selected_value
            .as_ref()
            .map_or_else(|| "None".to_string(), |value| format!("Some({value:?})"));
        let callback_runs = workbench_selected_change_count.get();
        let format_callback_runs = workbench_format_change_count.get();

        let data_state = if is_disabled {
            "disabled"
        } else if has_selection {
            "ready"
        } else {
            "empty"
        };
        let data_alpha = if is_alpha_channel_hidden {
            "hidden"
        } else {
            "visible"
        };
        let data_motion_source = if reduced_motion { "custom" } else { "default" };
        let data_label_source = if custom_label { "custom" } else { "default" };
        let data_class_source = if custom_class { "custom" } else { "default" };

        let mut classes = vec!["ui-color-editor".to_string(), format.class_name().into()];
        if is_disabled {
            classes.push("ui-color-editor--disabled".to_string());
        }
        if is_alpha_channel_hidden {
            classes.push("ui-color-editor--alpha-hidden".to_string());
        }
        if custom_class {
            classes.push("ui-color-editor--custom-class".to_string());
            classes.push("docs-color-editor-workbench".to_string());
        }

        format!(
            "ColorEditorActualConfig {{\n  id_base: \"docs-color-editor-workbench\",\n  label: {:?},\n  aria_label: \"Brand palette editor\",\n  is_disabled: {is_disabled},\n  selected_color: {selected_value_text},\n  default_selected_color: Some(\"#4f46e5\"),\n  on_selected_change: \"callback_runs={callback_runs}\",\n  format: {format:?},\n  default_format: ColorEditorFormat::Hex,\n  on_format_change: \"callback_runs={format_callback_runs}\",\n  is_alpha_channel_hidden: {is_alpha_channel_hidden},\n  default_hue: Some(258.0),\n  default_alpha: Some(88.0),\n  default_area: Some((0.55_f32, 0.22_f32)),\n  area_label: \"Saturation and brightness\",\n  area_aria_label: \"Drag to change saturation and brightness\",\n  hue_label: \"Hue\",\n  alpha_label: \"Alpha\",\n  value_label: \"Color value\",\n  format_aria_label: \"Color format selector\",\n  preview_color: Some(\"#4f46e5\"),\n  motion: {:?},\n  class_name: {:?},\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  has_selection: {has_selection},\n  custom_label: {custom_label},\n  custom_class: {custom_class},\n  reduced_motion: {reduced_motion},\n  data_state: \"{data_state}\",\n  data_alpha: \"{data_alpha}\",\n  data_motion_source: \"{data_motion_source}\",\n  data_label_source: \"{data_label_source}\",\n  data_class_source: \"{data_class_source}\",\n  class: \"{}\",\n}}",
            if custom_label {
                "Brand color workspace"
            } else {
                "Color editor"
            },
            if reduced_motion {
                ColorSliderMotion::disabled()
            } else {
                ColorSliderMotion::default()
            },
            if custom_class {
                Some("docs-color-editor-workbench")
            } else {
                None
            },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="ColorEditor"
            slug="color-editor"
            group="Forms"
            description="baseline-compatible color editor primitive that composes color area + sliders + field + format switching with controllable color/format state and stable slot/data-state contracts."
        >
            <Playground title="Hello World（默认路径）" code_signal=hello_code>
                <ColorEditor id_base="docs-color-editor-hello".to_string() />
            </Playground>

            // Playground title="State Matrix" code_signal=state_matrix_code

            <Playground
                title="State Matrix (Disabled / Motion / Hidden Alpha)"
                code_signal=states_code
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorEditor
                        id_base="docs-color-editor-disabled".to_string()
                        label="Disabled editor".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        default_format=ColorEditorFormat::Rgb
                        is_alpha_channel_hidden=true
                        is_disabled=true
                        class_name="docs-color-editor-custom".to_string()
                    />

                    <ColorEditor
                        id_base="docs-color-editor-motion".to_string()
                        label="Brand editor".to_string()
                        default_format=ColorEditorFormat::Hsb
                        default_hue=282.0
                        default_alpha=64.0
                        default_area=(0.46, 0.88)
                        motion=reduced_motion
                    />
                </div>
            </Playground>

            <Playground title="State Gallery" code_signal=state_matrix_code>
                <div class="docs-row" data-slot="color-editor-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-ready".to_string()
                            label="Ready".to_string()
                            default_selected_color="#4f46e5".to_string()
                            default_format=ColorEditorFormat::Hex
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Empty"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-empty".to_string()
                            label="Empty".to_string()
                            default_format=ColorEditorFormat::Hsl
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled + Hidden Alpha"</div>
                        <ColorEditor
                            id_base="docs-color-editor-matrix-disabled".to_string()
                            label="Disabled".to_string()
                            default_selected_color="#0ea5e9".to_string()
                            default_format=ColorEditorFormat::Rgb
                            is_alpha_channel_hidden=true
                            is_disabled=true
                            motion=ColorSliderMotion::disabled()
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled vs Uncontrolled" code_signal=controlled_vs_uncontrolled_code>
                <div class="docs-row" data-slot="color-editor-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorEditor
                            id_base="docs-color-editor-controlled".to_string()
                            label="Controlled".to_string()
                            selected_color=selected_color_signal
                            on_selected_change=on_selected_change
                            format=format_signal
                            on_format_change=on_format_change
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorEditor
                            id_base="docs-color-editor-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_selected_color="#22c55e".to_string()
                            default_format=ColorEditorFormat::Rgb
                            default_hue=132.0
                            default_alpha=92.0
                            default_area=(0.62, 0.18)
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-editor/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Format"</div>
                        <SegmentedControl
                            id_base="docs-color-editor-workbench-format".to_string()
                            options=workbench_format_options.clone()
                            selected_index=workbench_format_index
                            set_selected_index=set_workbench_format_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorEditor workbench format".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>
                            "Hide alpha channel"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let reduced_motion = if workbench_reduced_motion.get() {
                        ColorSliderMotion::disabled()
                    } else {
                        ColorSliderMotion::default()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row" style="align-items: flex-start;">
                                <div class="docs-card" style="flex: 1 1 28rem; min-width: 20rem;">
                                    <div class="ui-muted">"Workbench"</div>
                                    <ColorEditor
                                        id_base="docs-color-editor-workbench".to_string()
                                        label=if workbench_custom_label.get() {
                                            "Brand color workspace".to_string()
                                        } else {
                                            "Color editor".to_string()
                                        }
                                        aria_label="Brand palette editor".to_string()
                                        selected_color=workbench_selected_color_signal
                                        on_selected_change=on_workbench_selected_change
                                        default_selected_color="#4f46e5".to_string()
                                        format=workbench_format_signal
                                        default_format=ColorEditorFormat::Hex
                                        on_format_change=on_workbench_format_change
                                        is_alpha_channel_hidden=workbench_hide_alpha.get()
                                        is_disabled=workbench_disabled.get()
                                        default_hue=258.0
                                        default_alpha=88.0
                                        default_area=(0.55_f32, 0.22_f32)
                                        area_label="Saturation and brightness".to_string()
                                        area_aria_label="Drag to change saturation and brightness".to_string()
                                        hue_label="Hue".to_string()
                                        alpha_label="Alpha".to_string()
                                        value_label="Color value".to_string()
                                        format_aria_label="Color format selector".to_string()
                                        preview_color="#4f46e5".to_string()
                                        class_name=if workbench_custom_class.get() {
                                            "docs-color-editor-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                        lang="en-US".to_string()
                                        dir=A11yDirection::Ltr
                                        motion=reduced_motion
                                    />
                                </div>

                                <div class="docs-card" style="flex: 1 1 28rem; min-width: 20rem;">
                                    <div class="ui-muted">"Comparison (Disabled + Alpha Hidden)"</div>
                                    <ColorEditor
                                        id_base="docs-color-editor-workbench-compare".to_string()
                                        label="Comparison".to_string()
                                        default_selected_color="#0ea5e9".to_string()
                                        default_format=ColorEditorFormat::Rgb
                                        is_alpha_channel_hidden=true
                                        is_disabled=true
                                        motion=ColorSliderMotion::disabled()
                                    />
                                </div>
                            </div>

                            <span class="ui-muted">
                                "value: "
                                {move || {
                                    workbench_selected_color
                                        .get()
                                        .unwrap_or_else(|| "none".to_string())
                                }}
                                " · format: "
                                {move || workbench_format.get().as_attr()}
                                " · on_selected_change: "
                                {move || workbench_selected_change_count.get()}
                                " · on_format_change: "
                                {move || workbench_format_change_count.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-editor-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorEditor is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorEditor
                        id_base="docs-color-editor-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_selected_color="#334155".to_string()
                        default_format=ColorEditorFormat::Hex
                    />
                </div>
            </Playground>

            <Playground title="Controlled Color + Controlled Format" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorEditor
                        id_base="docs-color-editor-basic".to_string()
                        label="Color editor".to_string()
                        selected_color=selected_color_signal
                        on_selected_change=on_selected_change
                        format=format_signal
                        on_format_change=on_format_change
                    />

                    <span class="ui-muted">
                        "value: "
                        {move || selected_color.get().unwrap_or_else(|| "none".to_string())}
                        " · format: "
                        {move || format.get().as_attr()}
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-editor-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-editor/src/mod.rs"</code>
                    ", "
                    <code>"components/color-editor/src/view.rs"</code>
                    ", "
                    <code>"components/color-editor/src/logic.rs"</code>
                    ", "
                    <code>"components/color-editor/src/styles.rs"</code>
                    ", "
                    <code>"components/color-editor/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-editor-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_editor"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
