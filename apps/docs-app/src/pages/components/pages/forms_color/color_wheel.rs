use super::*;

pub(crate) fn color_wheel() -> AnyView {
    #[derive(Clone, Copy, Debug)]
    struct ColorWheelWorkbenchPreset {
        label: &'static str,
        default_value: f64,
    }

    #[derive(Clone, Copy, Debug)]
    struct ColorWheelWorkbenchState {
        preset_index: usize,
        value: f64,
        is_disabled: bool,
        has_custom_class: bool,
        reduced_motion: bool,
        preserve_context: bool,
    }

    impl Default for ColorWheelWorkbenchState {
        fn default() -> Self {
            Self {
                preset_index: 0,
                value: 32.0,
                is_disabled: false,
                has_custom_class: false,
                reduced_motion: false,
                preserve_context: true,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn bool_digit(value: bool) -> u8 {
        if value { 1 } else { 0 }
    }

    #[cfg(target_arch = "wasm32")]
    fn parse_bool(raw: &str) -> Option<bool> {
        match raw.trim() {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        }
    }

    impl ColorWheelWorkbenchState {
        #[cfg(target_arch = "wasm32")]
        fn parse(raw: &str) -> Option<Self> {
            let mut parts = raw.split(',');
            let preset_index = parts.next()?.trim().parse::<usize>().ok()?;
            let value = parts.next()?.trim().parse::<f64>().ok()?;
            let is_disabled = parse_bool(parts.next()?)?;
            let has_custom_class = parse_bool(parts.next()?)?;
            let reduced_motion = parse_bool(parts.next()?)?;
            let preserve_context = parse_bool(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }

            Some(Self {
                preset_index,
                value,
                is_disabled,
                has_custom_class,
                reduced_motion,
                preserve_context,
            })
        }

        #[cfg(target_arch = "wasm32")]
        fn serialize(self) -> String {
            format!(
                "{},{:.2},{},{},{},{}",
                self.preset_index,
                self.value,
                bool_digit(self.is_disabled),
                bool_digit(self.has_custom_class),
                bool_digit(self.reduced_motion),
                bool_digit(self.preserve_context),
            )
        }
    }

    #[cfg(target_arch = "wasm32")]
    const COLOR_WHEEL_WORKBENCH_STORAGE_KEY: &str = "docs:color-wheel:workbench:state";

    #[cfg(target_arch = "wasm32")]
    fn load_color_wheel_workbench_state() -> Option<ColorWheelWorkbenchState> {
        let storage = web_sys::window()?.local_storage().ok().flatten()?;
        let raw = storage
            .get_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY)
            .ok()
            .flatten()?;
        ColorWheelWorkbenchState::parse(&raw)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_color_wheel_workbench_state() -> Option<ColorWheelWorkbenchState> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn save_color_wheel_workbench_state(state: ColorWheelWorkbenchState) {
        if let Some(storage) = web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
        {
            drop(storage.set_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY, &state.serialize()));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_color_wheel_workbench_state(_state: ColorWheelWorkbenchState) {}

    #[cfg(target_arch = "wasm32")]
    fn clear_color_wheel_workbench_state() {
        if let Some(storage) = web_sys::window()
            .and_then(|window| window.local_storage().ok())
            .flatten()
        {
            drop(storage.remove_item(COLOR_WHEEL_WORKBENCH_STORAGE_KEY));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear_color_wheel_workbench_state() {}

    let (value, set_value) = signal(32.0_f64);
    let on_value_change = Callback::new(move |next: f64| set_value.set(next));

    let (disabled_value, set_disabled_value) = signal(248.0_f64);
    let on_disabled_change = Callback::new(move |next: f64| set_disabled_value.set(next));

    let reduced_motion = ColorWheelMotion::disabled();
    let workbench_preset_options = vec![
        "warm".to_string(),
        "cool".to_string(),
        "contrast".to_string(),
    ];
    let persisted_workbench_state = load_color_wheel_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_preset_index, set_workbench_preset_index) =
        signal(Some(initial_workbench_state.preset_index.min(2)));
    let workbench_preset =
        Signal::derive(move || match workbench_preset_index.get().unwrap_or(0) {
            1 => ColorWheelWorkbenchPreset {
                label: "Cool hue",
                default_value: 216.0,
            },
            2 => ColorWheelWorkbenchPreset {
                label: "Contrast hue",
                default_value: 282.0,
            },
            _ => ColorWheelWorkbenchPreset {
                label: "Warm hue",
                default_value: 32.0,
            },
        });
    let (workbench_value, set_workbench_value) = signal(initial_workbench_state.value);
    let on_workbench_value_change = Callback::new(move |next: f64| set_workbench_value.set(next));
    let workbench_value_signal: Signal<f64> = workbench_value.into();
    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.has_custom_class);
    let (workbench_reduced_motion, set_workbench_reduced_motion) =
        signal(initial_workbench_state.reduced_motion);
    let (workbench_preserve_context, set_workbench_preserve_context) =
        signal(initial_workbench_state.preserve_context);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let (workbench_show_value_label, set_workbench_show_value_label) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let workbench_motion = Signal::derive(move || {
        if workbench_reduced_motion.get() {
            ColorWheelMotion::disabled()
        } else {
            ColorWheelMotion::default()
        }
    });

    let last_workbench_preset_index =
        RwSignal::new(workbench_preset_index.get_untracked().unwrap_or(0).min(2));
    let reset_workbench_value = set_workbench_value;
    let reset_workbench_disabled = set_workbench_disabled;
    let reset_workbench_custom_class = set_workbench_custom_class;
    let reset_workbench_reduced_motion = set_workbench_reduced_motion;

    Effect::new(move |_| {
        let next_preset_index = workbench_preset_index.get().unwrap_or(0).min(2);
        let prev_preset_index = last_workbench_preset_index.get_untracked();
        if next_preset_index == prev_preset_index {
            return;
        }

        last_workbench_preset_index.set(next_preset_index);
        if !workbench_preserve_context.get() {
            let preset = workbench_preset.get();
            reset_workbench_value.set(preset.default_value);
            reset_workbench_disabled.set(false);
            reset_workbench_custom_class.set(false);
            reset_workbench_reduced_motion.set(false);
        }
    });

    Effect::new(move |_| {
        let state = ColorWheelWorkbenchState {
            preset_index: workbench_preset_index.get().unwrap_or(0).min(2),
            value: workbench_value.get(),
            is_disabled: workbench_disabled.get(),
            has_custom_class: workbench_custom_class.get(),
            reduced_motion: workbench_reduced_motion.get(),
            preserve_context: workbench_preserve_context.get(),
        };

        if workbench_persist_state.get() {
            save_color_wheel_workbench_state(state);
        } else {
            clear_color_wheel_workbench_state();
        }
    });

    let hello_code = Signal::derive(move || {
        r##"<ColorWheel
  id_base="docs-color-wheel-hello".to_string()
/>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-wheel-state-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Ready · Warm"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-ready".to_string()
      label="Ready warm".to_string()
      default_value=32.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Ready · Cool"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-cool".to_string()
      label="Ready cool".to_string()
      default_value=216.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Disabled · Reduced Motion"</div>
    <ColorWheel
      id_base="docs-color-wheel-matrix-disabled".to_string()
      label="Disabled wheel".to_string()
      default_value=282.0
      is_disabled=true
      motion=ColorWheelMotion::disabled()
    />
  </div>
</div>"##
            .to_string()
    });

    let parameter_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row" data-slot="color-wheel-parameter-matrix">
  <div class="docs-card">
    <div class="ui-muted">"Defaults (logic.rs)"</div>
    <ColorWheel id_base="docs-color-wheel-param-default".to_string() />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Step + custom label"</div>
    <ColorWheel
      id_base="docs-color-wheel-param-step".to_string()
      label="Step 15°".to_string()
      step=15.0
    />
  </div>
  <div class="docs-card">
    <div class="ui-muted">"Hidden value label + custom class"</div>
    <ColorWheel
      id_base="docs-color-wheel-param-hidden-value".to_string()
      default_value=120.0
      is_value_label_visible=false
      aria_label="Hue selector".to_string()
      class_name="docs-color-wheel-custom".to_string()
    />
  </div>
</div>
<p class="ui-muted" data-slot="color-wheel-api-defaults-note">
  "Default API sync: step uses logic::DEFAULT_STEP when omitted; default_value falls back through logic::resolve_default_value; is_disabled defaults to false."
</p>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(32.0_f64);
let on_value_change = Callback::new(move |next: f64| set_value.set(next));

<ColorWheel
  id_base="docs-color-wheel-compare-controlled".to_string()
  label="Controlled".to_string()
  value=value.into()
  on_value_change=on_value_change
/>
<ColorWheel
  id_base="docs-color-wheel-compare-uncontrolled".to_string()
  label="Uncontrolled".to_string()
  default_value=180.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorWheel is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorWheel docs output mode: snapshot"
</div>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let preset = workbench_preset.get();
        let mut lines = vec![
            "let (value, set_value) = signal(32.0_f64);".to_string(),
            "let on_value_change = Callback::new(move |next: f64| set_value.set(next));"
                .to_string(),
            String::new(),
            "<ColorWheel".to_string(),
            "  id_base=\"docs-color-wheel-workbench\".into()".to_string(),
            format!("  label=\"{} (Workbench)\".into()", preset.label),
            "  aria_label=\"Workbench hue selector\".into()".to_string(),
            "  value=value.into()".to_string(),
            "  on_value_change=on_value_change".to_string(),
            format!("  default_value={:.1}", workbench_value.get()),
            "  step=15.0".to_string(),
            format!("  is_disabled={}", workbench_disabled.get()),
            format!(
                "  is_value_label_visible={}",
                bool_word(workbench_show_value_label.get())
            ),
            format!(
                "  show_value_label={}",
                bool_word(workbench_show_value_label.get())
            ),
            format!(
                "  lang={}.into()",
                rust_string_literal(if workbench_lang_zh.get() {
                    "zh-CN"
                } else {
                    "en-US"
                })
            ),
            format!(
                "  dir={}",
                if workbench_rtl_dir.get() {
                    "A11yDirection::Rtl"
                } else {
                    "A11yDirection::Ltr"
                }
            ),
        ];

        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorWheelMotion::disabled()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-wheel-workbench\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        if workbench_custom_class.get() {
            ".docs-color-wheel-workbench {\n  --ui-color-wheel-size: clamp(14rem, 42vw, 17rem);\n  --ui-color-wheel-thumb-size: 1.2rem;\n  --ui-color-wheel-track-shadow: 0 14px 30px color-mix(in oklch, var(--ui-brand), transparent 70%);\n}"
                .to_string()
        } else {
            String::new()
        }
    });

    let workbench_actual_config = Signal::derive(move || {
        let preset = workbench_preset.get();
        let value = workbench_value.get();
        let is_disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();
        let preserve_context = workbench_preserve_context.get();
        let persist_state = workbench_persist_state.get();
        let show_value_label = workbench_show_value_label.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        format!(
            "ColorWheelWorkbenchConfig {{\n  id_base: \"docs-color-wheel-workbench\",\n  label: Some(\"{preset_label} (Workbench)\"),\n  aria_label: Some(\"Workbench hue selector\"),\n  value: Some({value:.2}),\n  default_value: Some({default_value:.2}),\n  on_value_change: Some(\"on_workbench_value_change\"),\n  step: 15.0,\n  is_disabled: Some({is_disabled}),\n  disabled: {is_disabled},\n  motion: {motion:?},\n  is_value_label_visible: Some({show_value_label}),\n  show_value_label: {show_value_label},\n  class_name: {class_name},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  preset_label: \"{preset_label}\",\n  reduced_motion: {reduced_motion},\n  preserve_context: {preserve_context},\n  persist_state: {persist_state},\n}}",
            preset_label = preset.label,
            value = value,
            default_value = preset.default_value,
            is_disabled = is_disabled,
            motion = workbench_motion.get(),
            show_value_label = show_value_label,
            class_name = if custom_class {
                "Some(\"docs-color-wheel-workbench\")"
            } else {
                "None"
            },
            lang = lang,
            dir = dir,
            reduced_motion = reduced_motion,
            preserve_context = preserve_context,
            persist_state = persist_state,
        )
    });

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(32.0_f64);
let on_value_change = Callback::new(move |next: f64| set_value.set(next));

<ColorWheel
  id_base="docs-color-wheel-hue".to_string()
  label="Hue wheel".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorWheelMotion::disabled();

<ColorWheel
  id_base="docs-color-wheel-disabled".to_string()
  label="Disabled wheel".to_string()
  value=disabled_value.into()
  on_value_change=on_disabled_change
  is_disabled=true
/>
<ColorWheel
  id_base="docs-color-wheel-custom".to_string()
  label="Brand hue".to_string()
  default_value=282.0
  motion=reduced_motion
  class_name="docs-color-wheel-custom".to_string()
/>"##
            .to_string()
    });

    let baseline_code = Signal::derive(move || {
        r##"<div
  class="docs-stack docs-stack--tight"
  data-doc-visual-baseline="color-wheel-default-theme"
  data-doc-baseline-shot="color-wheel-default-theme-v1"
>
  <ColorWheel
    id_base="docs-color-wheel-baseline-primary".to_string()
    label="Primary hue".to_string()
    default_value=24.0
  />
  <ColorWheel
    id_base="docs-color-wheel-baseline-depth".to_string()
    label="Contrast depth".to_string()
    default_value=216.0
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorWheel"
            slug="color-wheel"
            group="Forms"
            description="baseline-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable slot/data-state contracts."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorWheel;".to_string()
            >
                <ColorWheel id_base="docs-color-wheel-hello".to_string() />
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="Scoped CSS test panel + workbench 隔离画布；默认保留上下文并支持可选持久化。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-wheel/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-wheel-workbench-controls">
                        <div data-slot="color-wheel-workbench-preset">
                            <div class="docs-search__label">"Preset"</div>
                            <SegmentedControl
                                id_base="docs-color-wheel-workbench-preset".to_string()
                                options=workbench_preset_options.clone()
                                selected_index=workbench_preset_index
                                set_selected_index=set_workbench_preset_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorWheel workbench preset".to_string()
                            />
                        </div>

                        <div data-slot="color-wheel-workbench-toggle-disabled">
                            <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                                "Disabled"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-custom-class">
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-reduced-motion">
                            <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                                "Reduced motion"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-value-label">
                            <Switch
                                checked=workbench_show_value_label
                                set_checked=set_workbench_show_value_label
                            >
                                "Show value label"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-lang">
                            <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                "lang=zh-CN"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-dir">
                            <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                                "dir=rtl"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-preserve-context">
                            <Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>
                                "Preserve context on preset change"
                            </Switch>
                        </div>
                        <div data-slot="color-wheel-workbench-toggle-persist-state">
                            <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                                "Persist workbench state"
                            </Switch>
                        </div>
                    </div>
                }
            >
                {move || {
                    let preset = workbench_preset.get();
                    let value = workbench_value.get();
                    let is_disabled = workbench_disabled.get();
                    let motion = workbench_motion.get();
                    let class_name = if workbench_custom_class.get() {
                        "docs-color-wheel-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-wheel-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-wheel-workbench-canvas">
                                <ColorWheel
                                    id_base="docs-color-wheel-workbench".to_string()
                                    label=format!("{} (Workbench)", preset.label)
                                    value=workbench_value_signal
                                    on_value_change=on_workbench_value_change
                                    default_value=preset.default_value
                                    step=15.0
                                    is_disabled=is_disabled
                                    motion=motion
                                    is_value_label_visible=workbench_show_value_label.get()
                                    show_value_label=workbench_show_value_label.get()
                                    class_name=class_name
                                    aria_label="Workbench hue selector".to_string()
                                    lang=if workbench_lang_zh.get() {
                                        "zh-CN".to_string()
                                    } else {
                                        "en-US".to_string()
                                    }
                                    dir=if workbench_rtl_dir.get() {
                                        A11yDirection::Rtl
                                    } else {
                                        A11yDirection::Ltr
                                    }
                                />
                            </div>

                            <span class="ui-muted" data-slot="color-wheel-workbench-state">
                                "preset: " {preset.label}
                                " · value: " {format!("{value:.1}")}
                                " · preserve: " {if workbench_preserve_context.get() { "on" } else { "off" }}
                                " · persist: " {if workbench_persist_state.get() { "on" } else { "off" }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ColorWheel, ColorWheelMotion};".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Warm"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-ready".to_string()
                            label="Ready warm".to_string()
                            default_value=32.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Cool"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-cool".to_string()
                            label="Ready cool".to_string()
                            default_value=216.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled · Reduced Motion"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-disabled".to_string()
                            label="Disabled wheel".to_string()
                            default_value=282.0
                            is_disabled=true
                            motion=reduced_motion
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Parameter Matrix"
                code_signal=parameter_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorWheel;".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-parameter-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Defaults (logic.rs)"</div>
                        <ColorWheel id_base="docs-color-wheel-param-default".to_string() />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Step + custom label"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-param-step".to_string()
                            label="Step 15°".to_string()
                            step=15.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Hidden value label + custom class"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-param-hidden-value".to_string()
                            default_value=120.0
                            is_value_label_visible=false
                            aria_label="Hue selector".to_string()
                            class_name="docs-color-wheel-custom".to_string()
                        />
                    </div>
                </div>
                <p class="ui-muted" data-slot="color-wheel-api-defaults-note">
                    "Default API sync: step uses logic::DEFAULT_STEP when omitted; default_value falls back through logic::resolve_default_value; is_disabled defaults to false."
                </p>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorWheel;".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-compare-controlled".to_string()
                            label="Controlled".to_string()
                            value=value.into()
                            on_value_change=on_value_change
                        />
                        <span class="ui-muted">
                            "hue: " {move || format!("{:.0}°", value.get())}
                        </span>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-compare-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            default_value=180.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorWheel;".to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-wheel-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorWheel is an input surface; docs output mode remains snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorWheel
                        id_base="docs-color-wheel-snapshot".to_string()
                        label="Snapshot fallback".to_string()
                        default_value=188.0
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Controlled vs Default)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorWheel;".to_string()
            >
                <div class="docs-row" data-slot="color-wheel-controlled-vs-default-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-controlled-v2".to_string()
                            label="Controlled".to_string()
                            value=value.into()
                            on_value_change=on_value_change
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Default"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-default-v2".to_string()
                            label="Default".to_string()
                            default_value=180.0
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled + hidden label"</div>
                        <ColorWheel
                            id_base="docs-color-wheel-matrix-disabled-v2".to_string()
                            label="Disabled".to_string()
                            default_value=282.0
                            is_disabled=true
                            is_value_label_visible=false
                            motion=ColorWheelMotion::disabled()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Default Theme Baseline"
                description="默认主题视觉基线：信息层级、对比层次，以及 hover/active/focus 交互反馈。"
                code_signal=baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-doc-visual-baseline="color-wheel-default-theme"
                    data-doc-baseline-shot="color-wheel-default-theme-v1"
                    data-doc-visual-targets="label,value,hover,active,focus"
                >
                    <ColorWheel
                        id_base="docs-color-wheel-baseline-primary".to_string()
                        label="Primary hue".to_string()
                        default_value=24.0
                    />
                    <ColorWheel
                        id_base="docs-color-wheel-baseline-depth".to_string()
                        label="Contrast depth".to_string()
                        default_value=216.0
                    />
                </div>
            </Playground>

            <Playground title="Controlled Hue Wheel" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorWheel
                        id_base="docs-color-wheel-hue".to_string()
                        label="Hue wheel".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "hue: " {move || format!("{:.0}°", value.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Reduced Motion + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorWheel
                        id_base="docs-color-wheel-disabled".to_string()
                        label="Disabled wheel".to_string()
                        value=disabled_value.into()
                        on_value_change=on_disabled_change
                        is_disabled=true
                    />
                    <ColorWheel
                        id_base="docs-color-wheel-custom".to_string()
                        label="Brand hue".to_string()
                        default_value=282.0
                        motion=reduced_motion
                        class_name="docs-color-wheel-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-wheel-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-wheel-source-paths">
                    "Source-first path: "
                    <code>"components/color-wheel/src/mod.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/view.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/logic.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/styles.rs"</code>
                    ", "
                    <code>"components/color-wheel/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-wheel-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_wheel"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
                <p data-slot="color-wheel-source-first-contract">
                    "Contract: use any ColorWheel playground's "
                    <code>"Show code + Copy"</code>
                    " path for one-click runnable snippets; snippets stay synchronized with current props and imports."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
