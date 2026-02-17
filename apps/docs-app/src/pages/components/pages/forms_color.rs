use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ColorArea, ColorEditor, ColorEditorFormat, ColorField, ColorHandle, ColorLoupe, ColorPicker,
    ColorSlider, ColorSliderChannel, ColorSliderMotion, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorThumb, ColorWheel, ColorWheelMotion,
};

pub(super) fn color_field() -> AnyView {
    let (value, set_value) = signal(Some("#4f46e5".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal(Some("#4f46e5".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

<ColorField
  id_base="docs-color-field-basic".to_string()
  label="Fill color".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorField
  id_base="docs-color-field-invalid".to_string()
  label="Brand color".to_string()
  default_value="javascript:alert(1)".to_string()
  class_name="docs-color-field-custom".to_string()
/>
<ColorField
  id_base="docs-color-field-disabled".to_string()
  label="Accent color".to_string()
  default_value="#0ea5e9".to_string()
  disabled=true
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="baseline-compatible color text field with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable slot/data contracts."
        >
            <Playground title="Controlled Value" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-basic".to_string()
                        label="Fill color".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Invalid + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-invalid".to_string()
                        label="Brand color".to_string()
                        default_value="javascript:alert(1)".to_string()
                        class_name="docs-color-field-custom".to_string()
                    />
                    <ColorField
                        id_base="docs-color-field-disabled".to_string()
                        label="Accent color".to_string()
                        default_value="#0ea5e9".to_string()
                        disabled=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_area() -> AnyView {
    let (value, set_value) = signal((0.6_f32, 0.4_f32));
    let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

    let basic_code = Signal::derive(move || {
        r##"let (value, set_value) = signal((0.6_f32, 0.4_f32));
let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

<ColorArea
  id_base="docs-color-area-basic".to_string()
  label="Saturation / Lightness".to_string()
  value=value.into()
  on_value_change=on_value_change
  preview_color="#7c3aed".to_string()
/>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorArea
  id_base="docs-color-area-disabled".to_string()
  label="Accent area".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorArea"
            slug="color-area"
            group="Forms"
            description="baseline-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Grid Selection" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorArea
                        id_base="docs-color-area-basic".to_string()
                        label="Saturation / Lightness".to_string()
                        value=value.into()
                        on_value_change=on_value_change
                        preview_color="#7c3aed".to_string()
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || {
                            let (x, y) = value.get();
                            format!("({:.2}, {:.2})", x, y)
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Custom Grid + Custom Class" code_signal=states_code>
                <ColorArea
                    id_base="docs-color-area-disabled".to_string()
                    label="Accent area".to_string()
                    default_value=(0.25, 0.85)
                    grid_size=15
                    step=0.05
                    disabled=true
                    class_name="docs-color-area-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_slider() -> AnyView {
    let (hue, set_hue) = signal(220.0_f64);
    let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

    let (alpha, set_alpha) = signal(64.0_f64);
    let on_alpha_change = Callback::new(move |next: f64| set_alpha.set(next));

    let reduced_motion = ColorSliderMotion::disabled();

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

    view! {
        <ComponentPage
            title="ColorSlider"
            slug="color-slider"
            group="Forms"
            description="baseline-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable slot/data-state contracts."
        >
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_wheel() -> AnyView {
    let (value, set_value) = signal(32.0_f64);
    let on_value_change = Callback::new(move |next: f64| set_value.set(next));

    let (disabled_value, set_disabled_value) = signal(248.0_f64);
    let on_disabled_change = Callback::new(move |next: f64| set_disabled_value.set(next));

    let reduced_motion = ColorWheelMotion::disabled();

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
  disabled=true
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

    view! {
        <ComponentPage
            title="ColorWheel"
            slug="color-wheel"
            group="Forms"
            description="baseline-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable slot/data-state contracts."
        >
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
                        disabled=true
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_picker() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));

    let (open, set_open) = signal(false);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));

    let (swatches, _set_swatches) = signal(vec![
        ColorSwatchPickerItem::named("#ef4444", "Red"),
        ColorSwatchPickerItem::named("#f59e0b", "Amber"),
        ColorSwatchPickerItem::named("#10b981", "Emerald"),
        ColorSwatchPickerItem::named("#3b82f6", "Blue"),
        ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
    ]);

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let open_signal: Signal<bool> = open.into();

    let basic_code = Signal::derive(move || {
        r##"let (selected_color, set_selected_color) = signal(Some("#ef4444".to_string()));
let on_selected_change = Callback::new(move |next: Option<String>| set_selected_color.set(next));
let (open, set_open) = signal(false);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));

let (swatches, _set_swatches) = signal(vec![
  ColorSwatchPickerItem::named("#ef4444", "Red"),
  ColorSwatchPickerItem::named("#3b82f6", "Blue"),
]);
let selected_color_signal: Signal<Option<String>> = selected_color.into();
let open_signal: Signal<bool> = open.into();

<ColorPicker
  id_base="docs-color-picker-basic".to_string()
  label="Fill".to_string()
  selected_color=selected_color_signal
  on_selected_change=on_selected_change
  open=open_signal
  on_open_change=on_open_change
>
  <ColorSwatchPicker
    swatches=swatches
    selected_color=selected_color_signal
    on_selected_change=on_selected_change
  />
</ColorPicker>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<ColorPicker
  id_base="docs-color-picker-disabled".to_string()
  label="Disabled".to_string()
  default_selected_color="#0ea5e9".to_string()
  disabled=true
  class_name="docs-color-picker-custom".to_string()
>
  <div class="ui-muted">"Disabled picker content"</div>
</ColorPicker>

<ColorPicker
  id_base="docs-color-picker-open".to_string()
  label="Open by default".to_string()
  default_selected_color="#8b5cf6".to_string()
  default_open=true
>
  <div class="docs-stack docs-stack--tight">
    <span class="ui-muted">"Custom content area"</span>
  </div>
</ColorPicker>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorPicker"
            slug="color-picker"
            group="Forms"
            description="baseline-compatible color picker primitive that composes swatch trigger + popover content with controllable color/open state and stable slot/data-state contracts."
        >
            <Playground title="Controlled Color + Controlled Open" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorPicker
                        id_base="docs-color-picker-basic".to_string()
                        label="Fill".to_string()
                        selected_color=selected_color_signal
                        on_selected_change=on_selected_change
                        open=open_signal
                        on_open_change=on_open_change
                    >
                        <ColorSwatchPicker
                            swatches=swatches
                            selected_color=selected_color_signal
                            on_selected_change=on_selected_change
                        />
                    </ColorPicker>

                    <span class="ui-muted">
                        "selected: " {move || selected_color.get().unwrap_or_else(|| "none".to_string())}
                        " · open: " {move || if open.get() { "true" } else { "false" }}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Default Open + Custom Class" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorPicker
                        id_base="docs-color-picker-disabled".to_string()
                        label="Disabled".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        disabled=true
                        class_name="docs-color-picker-custom".to_string()
                    >
                        <div class="ui-muted">"Disabled picker content"</div>
                    </ColorPicker>

                    <ColorPicker
                        id_base="docs-color-picker-open".to_string()
                        label="Open by default".to_string()
                        default_selected_color="#8b5cf6".to_string()
                        default_open=true
                    >
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Custom content area"</span>
                        </div>
                    </ColorPicker>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_thumb() -> AnyView {
    let board_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";

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
    focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorThumb
    id_base="docs-color-thumb-dragging".to_string()
    color="#3b82f6".to_string()
    dragging=true
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
    disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorThumb
    id_base="docs-color-thumb-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    dragging=true
    show_loupe=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-thumb-custom".to_string()
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorThumb"
            slug="color-thumb"
            group="Forms"
            description="baseline-compatible draggable color thumb primitive with focus/drag/loupe state contracts, sanitized color source handling, and stable slot/data-state markers."
        >
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
                        focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-dragging".to_string()
                        color="#3b82f6".to_string()
                        dragging=true
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
                        disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorThumb
                        id_base="docs-color-thumb-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        dragging=true
                        show_loupe=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-thumb-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_editor() -> AnyView {
    let (selected_color, set_selected_color) = signal(Some("#4f46e5".to_string()));
    let on_selected_change =
        Callback::new(move |next: Option<String>| set_selected_color.set(next));

    let (format, set_format) = signal(ColorEditorFormat::Hex);
    let on_format_change = Callback::new(move |next: ColorEditorFormat| set_format.set(next));

    let selected_color_signal: Signal<Option<String>> = selected_color.into();
    let format_signal: Signal<ColorEditorFormat> = format.into();

    let reduced_motion = ColorSliderMotion::disabled();

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
  hide_alpha_channel=true
  disabled=true
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

    view! {
        <ComponentPage
            title="ColorEditor"
            slug="color-editor"
            group="Forms"
            description="baseline-compatible color editor primitive that composes color area + sliders + field + format switching with controllable color/format state and stable slot/data-state contracts."
        >
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

            <Playground title="Disabled + Alpha Hidden + Reduced Motion" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorEditor
                        id_base="docs-color-editor-disabled".to_string()
                        label="Disabled editor".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        default_format=ColorEditorFormat::Rgb
                        hide_alpha_channel=true
                        disabled=true
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_handle() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem;";

    let basic_code = Signal::derive(move || {
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
    focused=true
    x_percent=52.0
    y_percent=44.0
  />
  <ColorHandle
    id_base="docs-color-handle-dragging".to_string()
    color="#3b82f6".to_string()
    dragging=true
    x_percent=82.0
    y_percent=28.0
  />
</div>"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorHandle
    id_base="docs-color-handle-disabled".to_string()
    color="#a78bfa".to_string()
    disabled=true
    x_percent=30.0
    y_percent=56.0
  />
  <ColorHandle
    id_base="docs-color-handle-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    dragging=true
    show_loupe=false
    x_percent=70.0
    y_percent=40.0
    class_name="docs-color-handle-custom".to_string()
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorHandle"
            slug="color-handle"
            group="Forms"
            description="baseline-compatible draggable color handle primitive with composed thumb/loupe behavior, centralized state derivation, and stable slot/data-state contracts."
        >
            <Playground title="Focused + Dragging + Position" code_signal=basic_code>
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
                        focused=true
                        x_percent=52.0
                        y_percent=44.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-dragging".to_string()
                        color="#3b82f6".to_string()
                        dragging=true
                        x_percent=82.0
                        y_percent=28.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Class + Loupe Off" code_signal=states_code>
                <div style=surface_style>
                    <ColorHandle
                        id_base="docs-color-handle-disabled".to_string()
                        color="#a78bfa".to_string()
                        disabled=true
                        x_percent=30.0
                        y_percent=56.0
                    />
                    <ColorHandle
                        id_base="docs-color-handle-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        dragging=true
                        show_loupe=false
                        x_percent=70.0
                        y_percent=40.0
                        class_name="docs-color-handle-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_loupe() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";

    let basic_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-start".to_string()
    color="#f59e0b".to_string()
    open=true
    x_percent=18.0
    y_percent=74.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-center".to_string()
    color="#10b981".to_string()
    open=true
    x_percent=50.0
    y_percent=48.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-end".to_string()
    color="#3b82f6".to_string()
    open=true
    x_percent=82.0
    y_percent=24.0
  />
</div>"##.to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-disabled".to_string()
    color="#a78bfa".to_string()
    open=true
    disabled=true
    x_percent=32.0
    y_percent=58.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    open=true
    x_percent=72.0
    y_percent=36.0
    aria_label="Accent loupe".to_string()
    class_name="docs-color-loupe-custom".to_string()
  />
</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorLoupe"
            slug="color-loupe"
            group="Forms"
            description="baseline-compatible color loupe overlay primitive with centralized open/disabled/position normalization, checkerboard alpha preview, and stable slot/data-state contracts."
        >
            <Playground title="Open + Position Buckets" code_signal=basic_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-start".to_string()
                        color="#f59e0b".to_string()
                        open=true
                        x_percent=18.0
                        y_percent=74.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-center".to_string()
                        color="#10b981".to_string()
                        open=true
                        x_percent=50.0
                        y_percent=48.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-end".to_string()
                        color="#3b82f6".to_string()
                        open=true
                        x_percent=82.0
                        y_percent=24.0
                    />
                </div>
            </Playground>

            <Playground title="Disabled + Custom Label + Custom Class" code_signal=states_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-disabled".to_string()
                        color="#a78bfa".to_string()
                        open=true
                        disabled=true
                        x_percent=32.0
                        y_percent=58.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        open=true
                        x_percent=72.0
                        y_percent=36.0
                        aria_label="Accent loupe".to_string()
                        class_name="docs-color-loupe-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
