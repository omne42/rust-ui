use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    ColorArea, ColorField, ColorSlider, ColorSliderChannel, ColorSliderMotion, ColorWheel,
    ColorWheelMotion,
};

pub(super) fn color_field() -> AnyView {
    let (value, set_value) = signal(Some("#4f46e5".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

    let basic_code = r##"let (value, set_value) = signal(Some("#4f46e5".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

<ColorField
  id_base="docs-color-field-basic".to_string()
  label="Fill color".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##;

    let states_code = r##"<ColorField
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
/>"##;

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="Spectrum-compatible color text field with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable slot/data contracts."
        >
            <Playground title="Controlled Value" code=basic_code>
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

            <Playground title="Invalid + Disabled + Custom Class" code=states_code>
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

    let basic_code = r##"let (value, set_value) = signal((0.6_f32, 0.4_f32));
let on_value_change = Callback::new(move |next: (f32, f32)| set_value.set(next));

<ColorArea
  id_base="docs-color-area-basic".to_string()
  label="Saturation / Lightness".to_string()
  value=value.into()
  on_value_change=on_value_change
  preview_color="#7c3aed".to_string()
/>"##;

    let states_code = r##"<ColorArea
  id_base="docs-color-area-disabled".to_string()
  label="Accent area".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  disabled=true
  class_name="docs-color-area-custom".to_string()
/>"##;

    view! {
        <ComponentPage
            title="ColorArea"
            slug="color-area"
            group="Forms"
            description="Spectrum-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Grid Selection" code=basic_code>
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

            <Playground title="Disabled + Custom Grid + Custom Class" code=states_code>
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

    let basic_code = r##"let (hue, set_hue) = signal(220.0_f64);
let on_hue_change = Callback::new(move |next: f64| set_hue.set(next));

<ColorSlider
  id_base="docs-color-slider-hue".to_string()
  channel=ColorSliderChannel::Hue
  label="Hue".to_string()
  value=hue.into()
  on_value_change=on_hue_change
/>"##;

    let states_code = r##"let reduced_motion = ColorSliderMotion::disabled();

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
/>"##;

    view! {
        <ComponentPage
            title="ColorSlider"
            slug="color-slider"
            group="Forms"
            description="Spectrum-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Hue Channel" code=basic_code>
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

            <Playground title="Disabled Alpha + Custom Track + Reduced Motion" code=states_code>
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

    let basic_code = r##"let (value, set_value) = signal(32.0_f64);
let on_value_change = Callback::new(move |next: f64| set_value.set(next));

<ColorWheel
  id_base="docs-color-wheel-hue".to_string()
  label="Hue wheel".to_string()
  value=value.into()
  on_value_change=on_value_change
/>"##;

    let states_code = r##"let reduced_motion = ColorWheelMotion::disabled();

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
/>"##;

    view! {
        <ComponentPage
            title="ColorWheel"
            slug="color-wheel"
            group="Forms"
            description="Spectrum-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable slot/data-state contracts."
        >
            <Playground title="Controlled Hue Wheel" code=basic_code>
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

            <Playground title="Disabled + Reduced Motion + Custom Class" code=states_code>
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
