use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Calendar, CalendarFirstWeekday, CalendarTone, DatePicker, DatePickerTone, Label, LabelEmphasis,
    Slider, SliderMotion,
};

pub(super) fn label() -> AnyView {
    let emphasis_code = r#"<Label text="Name".to_string() for_id="name".to_string() required=true />
<Label text="Hint".to_string() emphasis=LabelEmphasis::Subtle />
<Label text="Critical".to_string() emphasis=LabelEmphasis::Strong required=true />"#;

    let custom_code = r#"<Label
  text="Assignee".to_string()
  for_id="assignee".to_string()
  required=true
  required_indicator="(required)".to_string()
  class_name="docs-label-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Label"
            slug="label"
            group="Forms"
            description="Form label primitive with centralized required/emphasis/source state contracts."
        >
            <Playground title="Emphasis + Required" code=emphasis_code>
                <div class="docs-stack">
                    <Label text="Name".to_string() for_id="docs-label-name".to_string() required=true />
                    <input id="docs-label-name" class="docs-search__input" type="text" placeholder="Type name" />

                    <Label text="Hint".to_string() emphasis=LabelEmphasis::Subtle />
                    <Label text="Critical".to_string() emphasis=LabelEmphasis::Strong required=true />
                </div>
            </Playground>

            <Playground title="Custom Indicator + Class" code=custom_code>
                <div class="docs-stack">
                    <Label
                        text="Assignee".to_string()
                        for_id="docs-label-assignee".to_string()
                        required=true
                        required_indicator="(required)".to_string()
                        class_name="docs-label-custom".to_string()
                    />
                    <input
                        id="docs-label-assignee"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn slider() -> AnyView {
    let (value, set_value) = signal(36.0_f64);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_change = Callback::new(move |next: f64| {
        set_last_change.set(format!("{next:.1}"));
    });

    let (disabled_value, set_disabled_value) = signal(68.0_f64);

    let code = r#"let (value, set_value) = signal(36.0_f64);
let on_change = Callback::new(move |next: f64| {
  logging::log!("slider changed: {next}");
});
<Slider
  id="volume".to_string()
  label="Volume".to_string()
  value=value
  set_value=set_value
  min=0.0
  max=100.0
  step=1.0
  on_change=Some(on_change)
/>"#;

    let states_code = r#"<Slider
  id="slider-disabled".to_string()
  label="Disabled".to_string()
  value=disabled_value
  set_value=set_disabled_value
  disabled=true
/>
<Slider
  id="slider-fine".to_string()
  label="Fine Step".to_string()
  value=value
  set_value=set_value
  min=0.0
  max=1.0
  step=0.05
  motion=SliderMotion::disabled()
/>"#;

    let fine_motion = SliderMotion::disabled();

    view! {
        <ComponentPage
            title="Slider"
            slug="slider"
            group="Forms"
            description="Range slider with spring-driven fill/thumb motion and Spectrum-style state data contracts."
        >
            <Playground title="Controlled + on_change" code=code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-volume".to_string()
                        label="Volume".to_string()
                        value=value
                        set_value=set_value
                        min=0.0
                        max=100.0
                        step=1.0
                        on_change=on_change
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", value.get())}
                        " · last on_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Fine Step" code=states_code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-disabled".to_string()
                        label="Disabled".to_string()
                        value=disabled_value
                        set_value=set_disabled_value
                        disabled=true
                    />
                    <Slider
                        id="docs-slider-fine".to_string()
                        label="Fine Step".to_string()
                        value=value
                        set_value=set_value
                        min=0.0
                        max=1.0
                        step=0.05
                        motion=fine_motion
                        class_name="docs-slider--fine".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn calendar() -> AnyView {
    let code = r#"<Calendar
  year=2026
  month=1
  selected_day=Some(6)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#;

    let states_code = r#"<Calendar
  year=2026
  month=2
  selected_day=Some(14)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  class_name="docs-calendar-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Calendar"
            slug="calendar"
            group="Forms"
            description="Month-grid calendar with centralized date normalization and Spectrum-style tone/weekday/source state contracts."
        >
            <Playground title="Default + Outside Days" code=code>
                <Calendar
                    year=2026
                    month=1
                    selected_day=Some(6)
                    tone=CalendarTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground title="Monday First + Strong Tone" code=states_code>
                <Calendar
                    year=2026
                    month=2
                    selected_day=Some(14)
                    tone=CalendarTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    class_name="docs-calendar-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_picker() -> AnyView {
    let code = r#"<DatePicker
  id_base="release-date".to_string()
  year=2026
  month=3
  default_selected_day=12
  tone=DatePickerTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#;

    let states_code = r#"<DatePicker
  id_base="ship-date".to_string()
  year=2026
  month=4
  default_selected_day=21
  tone=DatePickerTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  placeholder="Pick ship date".to_string()
  class_name="docs-date-picker-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="DatePicker"
            slug="date-picker"
            group="Forms"
            description="Date picker trigger + popover calendar with centralized open/value/source state contracts and HeroUI-grade popover motion handoff."
        >
            <Playground title="Default + Outside Days" code=code>
                <DatePicker
                    id_base="docs-date-picker-release".to_string()
                    year=2026
                    month=3
                    default_selected_day=12
                    tone=DatePickerTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground title="Monday First + Strong Tone" code=states_code>
                <DatePicker
                    id_base="docs-date-picker-ship".to_string()
                    year=2026
                    month=4
                    default_selected_day=21
                    tone=DatePickerTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    placeholder="Pick ship date".to_string()
                    class_name="docs-date-picker-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
