use super::*;

pub(crate) fn color_slider() -> AnyView {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct ColorSliderWorkbenchState {
        channel_index: usize,
        value: f64,
        is_disabled: bool,
        has_custom_track: bool,
        has_custom_class: bool,
        reduced_motion: bool,
        preserve_context: bool,
    }

    impl Default for ColorSliderWorkbenchState {
        fn default() -> Self {
            Self {
                channel_index: 0,
                value: 220.0,
                is_disabled: false,
                has_custom_track: false,
                has_custom_class: false,
                reduced_motion: false,
                preserve_context: true,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    impl ColorSliderWorkbenchState {
        fn parse(raw: &str) -> Option<Self> {
            let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
            if parts.len() != 7 {
                return None;
            }

            let parse_bool = |at: usize| match *parts.get(at)? {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            };

            Some(Self {
                channel_index: parts.get(0)?.parse::<usize>().ok()?.min(3),
                value: parts.get(1)?.parse::<f64>().ok()?,
                is_disabled: parse_bool(2)?,
                has_custom_track: parse_bool(3)?,
                has_custom_class: parse_bool(4)?,
                reduced_motion: parse_bool(5)?,
                preserve_context: parse_bool(6)?,
            })
        }

        fn encode(self) -> String {
            let bool_digit = |value: bool| if value { '1' } else { '0' };
            format!(
                "{},{:.4},{},{},{},{},{}",
                self.channel_index.min(3),
                self.value,
                bool_digit(self.is_disabled),
                bool_digit(self.has_custom_track),
                bool_digit(self.has_custom_class),
                bool_digit(self.reduced_motion),
                bool_digit(self.preserve_context),
            )
        }
    }

    #[cfg(target_arch = "wasm32")]
    const COLOR_SLIDER_WORKBENCH_STORAGE_KEY: &str = "docs:color-slider:workbench:state";

    #[cfg(target_arch = "wasm32")]
    fn load_color_slider_workbench_state() -> Option<ColorSliderWorkbenchState> {
        let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
        let raw = storage
            .get_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY)
            .ok()
            .flatten()?;
        ColorSliderWorkbenchState::parse(&raw)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_color_slider_workbench_state() -> Option<ColorSliderWorkbenchState> {
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn save_color_slider_workbench_state(state: ColorSliderWorkbenchState) {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.set_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY, &state.encode()));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_color_slider_workbench_state(_state: ColorSliderWorkbenchState) {}

    #[cfg(target_arch = "wasm32")]
    fn clear_color_slider_workbench_state() {
        if let Some(storage) =
            web_sys::window().and_then(|window| window.local_storage().ok().flatten())
        {
            drop(storage.remove_item(COLOR_SLIDER_WORKBENCH_STORAGE_KEY));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear_color_slider_workbench_state() {}

    let (hue, set_hue) = signal(220.0_f64);
    let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

    let (alpha, set_alpha) = signal(64.0_f64);
    let on_alpha_change = Callback::new(move |next: f64| set_alpha.set(next));

    let persisted_workbench_state = load_color_slider_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let workbench_channel_options = vec![
        "Hue".to_string(),
        "Saturation".to_string(),
        "Lightness".to_string(),
        "Alpha".to_string(),
    ];
    let (workbench_channel_index, set_workbench_channel_index) =
        signal(Some(initial_workbench_state.channel_index.min(3)));
    let workbench_channel =
        Signal::derive(
            move || match workbench_channel_index.get().unwrap_or(0).min(3) {
                1 => ColorSliderChannel::Saturation,
                2 => ColorSliderChannel::Lightness,
                3 => ColorSliderChannel::Alpha,
                _ => ColorSliderChannel::Hue,
            },
        );
    let (workbench_value, set_workbench_value) = signal(initial_workbench_state.value);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: f64| {
        set_workbench_change_count.update(|count| *count += 1);
        set_workbench_value.set(next);
    });
    let workbench_value_signal: Signal<f64> = workbench_value.into();

    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_track, set_workbench_custom_track) =
        signal(initial_workbench_state.has_custom_track);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.has_custom_class);
    let (workbench_reduced_motion, set_workbench_reduced_motion) =
        signal(initial_workbench_state.reduced_motion);
    let (workbench_preserve_context, set_workbench_preserve_context) =
        signal(initial_workbench_state.preserve_context);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let last_workbench_channel_index =
        RwSignal::new(workbench_channel_index.get_untracked().unwrap_or(0).min(3));
    let reset_workbench_value = set_workbench_value;
    let reset_workbench_disabled = set_workbench_disabled;
    let reset_workbench_custom_track = set_workbench_custom_track;
    let reset_workbench_custom_class = set_workbench_custom_class;
    let reset_workbench_reduced_motion = set_workbench_reduced_motion;
    Effect::new(move |_| {
        let next_channel_index = workbench_channel_index.get().unwrap_or(0).min(3);
        let prev_channel_index = last_workbench_channel_index.get_untracked();
        if next_channel_index == prev_channel_index {
            return;
        }

        last_workbench_channel_index.set(next_channel_index);
        if !workbench_preserve_context.get() {
            let next_channel = match next_channel_index {
                1 => ColorSliderChannel::Saturation,
                2 => ColorSliderChannel::Lightness,
                3 => ColorSliderChannel::Alpha,
                _ => ColorSliderChannel::Hue,
            };
            reset_workbench_value.set(next_channel.default_value());
            reset_workbench_disabled.set(false);
            reset_workbench_custom_track.set(false);
            reset_workbench_custom_class.set(false);
            reset_workbench_reduced_motion.set(false);
        }
    });

    Effect::new(move |_| {
        let state = ColorSliderWorkbenchState {
            channel_index: workbench_channel_index.get().unwrap_or(0).min(3),
            value: workbench_value.get(),
            is_disabled: workbench_disabled.get(),
            has_custom_track: workbench_custom_track.get(),
            has_custom_class: workbench_custom_class.get(),
            reduced_motion: workbench_reduced_motion.get(),
            preserve_context: workbench_preserve_context.get(),
        };

        if workbench_persist_state.get() {
            save_color_slider_workbench_state(state);
        } else {
            clear_color_slider_workbench_state();
        }
    });

    let workbench_code = Signal::derive(move || {
        let channel = workbench_channel.get();
        let channel_literal = match channel {
            ColorSliderChannel::Hue => "ColorSliderChannel::Hue",
            ColorSliderChannel::Saturation => "ColorSliderChannel::Saturation",
            ColorSliderChannel::Lightness => "ColorSliderChannel::Lightness",
            ColorSliderChannel::Alpha => "ColorSliderChannel::Alpha",
            _ => "ColorSliderChannel::Hue",
        };
        let (min, max, step) = match channel {
            ColorSliderChannel::Hue => (0.0, 360.0, 1.0),
            _ => (0.0, 100.0, 1.0),
        };
        let default_value = channel.default_value();
        let aria_label = format!("{} slider", channel.default_label());
        let lang_literal = if workbench_lang_zh.get() {
            "\"zh-CN\".to_string()"
        } else {
            "\"en-US\".to_string()"
        };
        let dir_literal = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        let mut lines = vec![
            format!(
                "let (value, set_value) = signal({:.1}_f64);",
                workbench_value.get()
            ),
            "let on_value_change = Callback::new(move |next: f64| set_value.set(next));"
                .to_string(),
            String::new(),
            "<ColorSlider".to_string(),
            "  id_base=\"docs-color-slider-workbench\".into()".to_string(),
            format!("  channel={channel_literal}"),
            format!("  label=\"{}\".into()", channel.default_label()),
            format!("  aria_label={:?}.into()", aria_label),
            "  value=value.into()".to_string(),
            format!("  default_value={default_value:.1}"),
            "  on_value_change=on_value_change".to_string(),
            format!("  min={min:.1}"),
            format!("  max={max:.1}"),
            format!("  step={step:.1}"),
            "  is_disabled=false".to_string(),
            format!("  disabled={}", workbench_disabled.get()),
            "  show_value_label=true".to_string(),
            format!("  lang={lang_literal}"),
            format!("  dir={dir_literal}"),
        ];

        if workbench_custom_track.get() {
            lines.push("  track_start_color=\"#0f172a\".into()".to_string());
            lines.push("  track_end_color=\"#38bdf8\".into()".to_string());
        }
        if workbench_reduced_motion.get() {
            lines.push("  motion=ColorSliderMotion::disabled()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-color-slider-workbench\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/color-slider/src/styles.rs */\n{}\n\n/* apps/docs-app/dev-overrides.css */\n{}",
            ui::color::slider::styles::CSS,
            include_str!("../../../../../dev-overrides.css"),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let channel = workbench_channel.get();
        let (min, max, step) = match channel {
            ColorSliderChannel::Hue => (0.0, 360.0, 1.0),
            _ => (0.0, 100.0, 1.0),
        };
        let default_value = channel.default_value();
        let value = workbench_value.get();
        let is_disabled = workbench_disabled.get();
        let custom_track = workbench_custom_track.get();
        let custom_class = workbench_custom_class.get();
        let reduced_motion = workbench_reduced_motion.get();
        let preserve_context = workbench_preserve_context.get();
        let persist_state = workbench_persist_state.get();
        format!(
            "ColorSliderWorkbenchConfig {{\n  id_base: \"docs-color-slider-workbench\",\n  channel: {channel:?},\n  label: {:?},\n  aria_label: {:?},\n  value: {value:.2},\n  default_value: Some({default_value:.1}),\n  on_value_change: \"count={} last={value:.2}\",\n  min: {min:.1},\n  max: {max:.1},\n  step: {step:.1},\n  is_disabled: Some(false),\n  disabled: {is_disabled},\n  motion: {:?},\n  show_value_label: true,\n  track_start_color: {:?},\n  track_end_color: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  custom_track: {custom_track},\n  custom_class: {custom_class},\n  reduced_motion: {reduced_motion},\n  preserve_context: {preserve_context},\n  persist_state: {persist_state},\n}}",
            channel.default_label(),
            format!("{} slider", channel.default_label()),
            workbench_change_count.get(),
            if reduced_motion {
                ColorSliderMotion::disabled()
            } else {
                ColorSliderMotion::default()
            },
            if custom_track { Some("#0f172a") } else { None },
            if custom_track { Some("#38bdf8") } else { None },
            if custom_class {
                Some("docs-color-slider-workbench")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
        )
    });

    let reduced_motion = ColorSliderMotion::disabled();

    let hello_code = Signal::derive(move || {
        r##"<ColorSlider
  id_base="docs-color-slider-hello".to_string()
/>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"let (hue, set_hue) = signal(220.0_f64);
let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

<ColorSlider
  id_base="docs-color-slider-hue".to_string()
  channel=ColorSliderChannel::Hue
  label="Hue".to_string()
  value=hue.into()
  on_value_change=on_hue_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"let reduced_motion = ColorSliderMotion::disabled();

<ColorSlider
  id_base="docs-color-slider-alpha".to_string()
  channel=ColorSliderChannel::Alpha
  label="Alpha".to_string()
  value=alpha.into()
  on_value_change=on_alpha_change
  disabled=true
/>
<ColorSlider
  id_base="docs-color-slider-custom".to_string()
  channel=ColorSliderChannel::Blue
  label="Brand blue".to_string()
  default_value=172.0
  track_start_color="#0f172a".to_string()
  track_end_color="#38bdf8".to_string()
  motion=reduced_motion
  class_name="docs-color-slider-custom".to_string()
/>"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-row">
  <ColorSlider
    id_base="docs-color-slider-matrix-hue".to_string()
    channel=ColorSliderChannel::Hue
    label="Hue".to_string()
    default_value=196.0
  />
  <ColorSlider
    id_base="docs-color-slider-matrix-saturation".to_string()
    channel=ColorSliderChannel::Saturation
    label="Saturation".to_string()
    default_value=72.0
  />
  <ColorSlider
    id_base="docs-color-slider-matrix-disabled".to_string()
    channel=ColorSliderChannel::Alpha
    label="Disabled alpha".to_string()
    default_value=40.0
    disabled=true
    motion=ColorSliderMotion::disabled()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled_hue, set_controlled_hue) = signal(220.0_f64);
let on_controlled_hue_change =
  Callback::new(move |next: f64| set_controlled_hue.set(next));

<ColorSlider
  id_base="docs-color-slider-compare-controlled".to_string()
  channel=ColorSliderChannel::Hue
  label="Controlled".to_string()
  value=controlled_hue.into()
  on_value_change=on_controlled_hue_change
/>
<ColorSlider
  id_base="docs-color-slider-compare-uncontrolled".to_string()
  channel=ColorSliderChannel::Hue
  label="Uncontrolled".to_string()
  default_value=180.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// ColorSlider is not a text-reading surface.
// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "ColorSlider docs output mode: snapshot"
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorSlider"
            slug="color-slider"
            group="Forms"
            description="baseline-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable slot/data-state contracts."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::ColorSlider;".to_string()
            >
                <ColorSlider id_base="docs-color-slider-hello".to_string() />
            </Playground>

            // <Playground title="State Matrix" code_signal=state_matrix_code>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ColorSlider, ColorSliderChannel, ColorSliderMotion};".to_string()
            >
                <div class="docs-row" data-slot="color-slider-state-matrix">
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Hue"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-hue".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Hue".to_string()
                            default_value=196.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Ready · Saturation"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-saturation".to_string()
                            channel=ColorSliderChannel::Saturation
                            label="Saturation".to_string()
                            default_value=72.0
                        />
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Disabled · Alpha"</div>
                        <ColorSlider
                            id_base="docs-color-slider-matrix-disabled".to_string()
                            channel=ColorSliderChannel::Alpha
                            label="Disabled alpha".to_string()
                            default_value=40.0
                            disabled=true
                            motion=reduced_motion
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_vs_uncontrolled_code
                code_imports="use leptos::prelude::*;\nuse ui::{ColorSlider, ColorSliderChannel};".to_string()
            >
                <div class="docs-row" data-slot="color-slider-controlled-vs-uncontrolled">
                    <div class="docs-card">
                        <div class="ui-muted">"Controlled"</div>
                        <ColorSlider
                            id_base="docs-color-slider-compare-controlled".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Controlled".to_string()
                            value=hue.into()
                            on_value_change=on_hue_change
                        />
                        <span class="ui-muted">
                            "hue: " {move || format!("{:.0}°", hue.get())}
                        </span>
                    </div>
                    <div class="docs-card">
                        <div class="ui-muted">"Uncontrolled"</div>
                        <ColorSlider
                            id_base="docs-color-slider-compare-uncontrolled".to_string()
                            channel=ColorSliderChannel::Hue
                            label="Uncontrolled".to_string()
                            default_value=180.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Workbench (DX)"
                description="Scoped CSS test panel + dev-overrides.css 热更新路径；默认保留调试上下文，并提供可选持久化。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/color-slider/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-slider-workbench-controls">
                        <div class="docs-search__label">"Channel"</div>
                        <SegmentedControl
                            id_base="docs-color-slider-workbench-channel".to_string()
                            options=workbench_channel_options.clone()
                            selected_index=workbench_channel_index
                            set_selected_index=set_workbench_channel_index
                            size=SegmentedControlSize::Sm
                            aria_label="ColorSlider workbench channel".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_custom_track set_checked=set_workbench_custom_track>
                            "Custom track colors"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                        <Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>
                            "Preserve context on channel change"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let channel = workbench_channel.get();
                    let reduced_motion = workbench_reduced_motion.get();
                    let custom_track = workbench_custom_track.get();
                    let custom_class = workbench_custom_class.get();
                    let is_disabled = workbench_disabled.get();
                    let value = workbench_value.get();
                    let motion = if reduced_motion {
                        ColorSliderMotion::disabled()
                    } else {
                        ColorSliderMotion::default()
                    };

                    let (track_start_color, track_end_color) = if custom_track {
                        ("#0f172a".to_string(), "#38bdf8".to_string())
                    } else {
                        (String::new(), String::new())
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-slider-workbench">
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="color-slider-workbench-canvas">
                                <ColorSlider
                                    id_base="docs-color-slider-workbench".to_string()
                                    channel=channel
                                    label=format!("{} (Workbench)", channel.default_label())
                                    aria_label=format!("{} slider", channel.default_label())
                                    value=workbench_value_signal
                                    default_value=channel.default_value()
                                    on_value_change=on_workbench_value_change
                                    min=0.0
                                    max=if channel == ColorSliderChannel::Hue {
                                        360.0
                                    } else {
                                        100.0
                                    }
                                    step=1.0
                                    show_value_label=true
                                    is_disabled=false
                                    disabled=is_disabled
                                    track_start_color=track_start_color
                                    track_end_color=track_end_color
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
                                    motion=motion
                                    class_name=if custom_class {
                                        "docs-color-slider-workbench".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                            </div>

                            <span class="ui-muted" data-slot="color-slider-workbench-state">
                                "channel: " {channel.as_attr()}
                                " · value: " {format!("{value:.1}")}
                                " · changes: " {workbench_change_count.get()}
                                " · preserve: " {if workbench_preserve_context.get() { "on" } else { "off" }}
                                " · persist: " {if workbench_persist_state.get() { "on" } else { "off" }}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports="use leptos::prelude::*;\nuse ui::{ColorSlider, ColorSliderChannel};".to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-slider-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorSlider is an input surface; docs output mode remains snapshot (`fallback=snapshot`)."
                    </span>
                    <ColorSlider
                        id_base="docs-color-slider-snapshot".to_string()
                        channel=ColorSliderChannel::Hue
                        label="Snapshot fallback".to_string()
                        default_value=188.0
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Hue / Saturation / Disabled Alpha)"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ColorSlider, ColorSliderChannel, ColorSliderMotion};".to_string()
            >
                <div class="docs-row" data-slot="color-slider-state-matrix-final">
                    <ColorSlider
                        id_base="docs-color-slider-matrix-final-hue".to_string()
                        channel=ColorSliderChannel::Hue
                        label="Hue".to_string()
                        aria_label="Hue slider".to_string()
                        default_value=196.0
                        min=0.0
                        max=360.0
                        step=1.0
                        show_value_label=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ColorSlider
                        id_base="docs-color-slider-matrix-final-saturation".to_string()
                        channel=ColorSliderChannel::Saturation
                        label="Saturation".to_string()
                        aria_label="Saturation slider".to_string()
                        default_value=72.0
                        min=0.0
                        max=100.0
                        step=1.0
                        show_value_label=true
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <ColorSlider
                        id_base="docs-color-slider-matrix-final-disabled".to_string()
                        channel=ColorSliderChannel::Alpha
                        label="Disabled alpha".to_string()
                        aria_label="Disabled alpha slider".to_string()
                        default_value=40.0
                        min=0.0
                        max=100.0
                        step=1.0
                        show_value_label=true
                        disabled=true
                        motion=ColorSliderMotion::disabled()
                        class_name="docs-color-slider-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>

            <Playground title="Controlled Hue Channel" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorSlider
                        id_base="docs-color-slider-hue".to_string()
                        channel=ColorSliderChannel::Hue
                        label="Hue".to_string()
                        value=hue.into()
                        on_value_change=on_hue_change
                    />
                    <span class="ui-muted">
                        "hue: " {move || format!("{:.0}°", hue.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled Alpha + Custom Track + Reduced Motion" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorSlider
                        id_base="docs-color-slider-alpha".to_string()
                        channel=ColorSliderChannel::Alpha
                        label="Alpha".to_string()
                        value=alpha.into()
                        on_value_change=on_alpha_change
                        disabled=true
                    />
                    <ColorSlider
                        id_base="docs-color-slider-custom".to_string()
                        channel=ColorSliderChannel::Blue
                        label="Brand blue".to_string()
                        default_value=172.0
                        track_start_color="#0f172a".to_string()
                        track_end_color="#38bdf8".to_string()
                        motion=reduced_motion
                        class_name="docs-color-slider-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-slider-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps snippet imports synchronized with "
                    <code>"code_imports"</code>
                    "."
                </p>
                <p data-slot="color-slider-source-paths">
                    "Source-first path: "
                    <code>"components/color-slider/src/mod.rs"</code>
                    ", "
                    <code>"components/color-slider/src/view.rs"</code>
                    ", "
                    <code>"components/color-slider/src/logic.rs"</code>
                    ", "
                    <code>"components/color-slider/src/styles.rs"</code>
                    ", "
                    <code>"components/color-slider/src/motion.rs"</code>
                    "."
                </p>
                <p data-slot="color-slider-source-prerequisites">
                    "Prerequisites: enable "
                    <code>"component-color_slider"</code>
                    " (and "
                    <code>"inject-css"</code>
                    " when runtime CSS injection is required) so copied snippets compile and render as expected."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
