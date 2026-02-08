use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Calendar, CalendarFirstWeekday, CalendarTone, DateField, DateFieldTone, DatePicker,
    DatePickerTone, DateRangePicker, DateRangePickerTone, Description, DescriptionElement,
    DescriptionTone, ErrorMessage, ErrorMessageElement, ErrorMessageTone, Field, FieldError,
    FieldErrorTone, FieldOrientation, FieldTone, Fieldset, FieldsetOrientation, FieldsetTone,
    HelpText, HelpTextTone, Label, LabelEmphasis, Slider, SliderMotion, TimeField, TimeFieldTone,
};

pub(super) fn field_error() -> AnyView {
    let default_code = r#"<FieldError
  visible=true
  message="Email is required".to_string()
/>
<FieldError
  visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  visible=true
  tone=FieldErrorTone::Negative
  show_icon=true
  message="Two-factor code is invalid".to_string()
/>
"#;

    let hidden_code = r#"<FieldError
  visible=false
  message="This text should not render when hidden".to_string()
/>
<FieldError
  visible=true
  disabled=true
  show_icon=true
  class_name="docs-field-error-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="FieldError"
            slug="field-error"
            group="Forms"
            description="Spectrum/HeroUI-style field error primitive with centralized visibility/tone/message normalization and stable data contracts."
        >
            <Playground title="Visible + Tone" code=default_code>
                <div class="docs-stack">
                    <FieldError
                        visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        visible=true
                        tone=FieldErrorTone::Negative
                        show_icon=true
                        message="Two-factor code is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Hidden + Disabled + Custom Class" code=hidden_code>
                <div class="docs-stack">
                    <FieldError
                        visible=false
                        message="This text should not render when hidden".to_string()
                    />
                    <FieldError
                        visible=true
                        disabled=true
                        show_icon=true
                        class_name="docs-field-error-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_message() -> AnyView {
    let tone_code = r#"<ErrorMessage text="Invalid email address".to_string() />
<ErrorMessage
  text="Username contains unsupported characters.".to_string()
  tone=ErrorMessageTone::Neutral
/>
<ErrorMessage
  text="Verification code expired, request a new one.".to_string()
  tone=ErrorMessageTone::Negative
/>"#;

    let state_code = r#"<ErrorMessage
  text="A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.".to_string()
  truncate=true
  class_name="docs-error-message-custom".to_string()
/>
<ErrorMessage
  text="This error remains visible but marked as disabled for read-only states.".to_string()
  disabled=true
  element=ErrorMessageElement::Div
  aria_label="Disabled error message".to_string()
/>"#;

    view! {
        <ComponentPage
            title="ErrorMessage"
            slug="error-message"
            group="Forms"
            description="Spectrum/HeroUI-style inline error primitive with centralized tone/disabled/truncate/source normalization and stable slot/data contracts."
        >
            <Playground title="Tone Variants" code=tone_code>
                <div class="docs-stack">
                    <ErrorMessage
                        text="Invalid email address".to_string()
                        aria_label="Email error".to_string()
                    />
                    <ErrorMessage
                        text="Username contains unsupported characters.".to_string()
                        tone=ErrorMessageTone::Neutral
                    />
                    <ErrorMessage
                        text="Verification code expired, request a new one.".to_string()
                        tone=ErrorMessageTone::Negative
                    />
                </div>
            </Playground>

            <Playground title="Truncate + Disabled + Element + Custom Class" code=state_code>
                <div class="docs-stack docs-error-message-limit">
                    <ErrorMessage
                        text="A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.".to_string()
                        truncate=true
                        class_name="docs-error-message-custom".to_string()
                    />
                    <ErrorMessage
                        text="This error remains visible but marked as disabled for read-only states.".to_string()
                        disabled=true
                        element=ErrorMessageElement::Div
                        aria_label="Disabled error message".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn description() -> AnyView {
    let tone_code = r#"<Description
  text="This appears below the field as guidance.".to_string()
  tone=DescriptionTone::Default
/>
<Description
  text="Two-factor code expired. Request a new one.".to_string()
  tone=DescriptionTone::Negative
/>"#;

    let truncate_code = r#"<Description
  text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
  element=DescriptionElement::Span
  truncate=true
  class_name="docs-description-custom".to_string()
/>
<Description
  text="Disabled helper text".to_string()
  disabled=true
  tone=DescriptionTone::Muted
/>"#;

    view! {
        <ComponentPage
            title="Description"
            slug="description"
            group="Forms"
            description="Spectrum/HeroUI-style form description primitive with centralized tone/state/source contracts and stable slot semantics."
        >
            <Playground title="Tone Variants" code=tone_code>
                <div class="docs-stack">
                    <Description
                        text="This appears below the field as guidance.".to_string()
                        tone=DescriptionTone::Default
                        aria_label="Name helper".to_string()
                    />
                    <Description
                        text="Optional details are only visible to admins.".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <Description
                        text="Two-factor code expired. Request a new one.".to_string()
                        tone=DescriptionTone::Negative
                    />
                </div>
            </Playground>

            <Playground title="Truncate + Element + Disabled" code=truncate_code>
                <div class="docs-stack docs-description-limit">
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        truncate=true
                        class_name="docs-description-custom".to_string()
                    />
                    <Description
                        text="Disabled helper text".to_string()
                        disabled=true
                        tone=DescriptionTone::Muted
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn fieldset() -> AnyView {
    let default_code = r#"<Fieldset
  legend="Notification channels".to_string()
  description="Pick every channel you want to receive release updates from.".to_string()
  required=true
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#;

    let invalid_code = r#"<Fieldset
  orientation=FieldsetOrientation::Horizontal
  tone=FieldsetTone::Muted
  invalid=true
  error_message="Pick at least one channel".to_string()
  class_name="docs-fieldset-custom".to_string()
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary size=ui_components::ButtonSize::Sm>
      "Manage channels"
    </ui_components::Button>
  }
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#;

    view! {
        <ComponentPage
            title="Fieldset"
            slug="fieldset"
            group="Forms"
            description="Spectrum/HeroUI-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts."
        >
            <Playground title="Legend + Description" code=default_code>
                <Fieldset
                    legend="Notification channels".to_string()
                    description="Pick every channel you want to receive release updates from.".to_string()
                    required=true
                    aria_label="Notification channel group".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Push"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground title="Horizontal + Invalid + Actions" code=invalid_code>
                <Fieldset
                    orientation=FieldsetOrientation::Horizontal
                    tone=FieldsetTone::Muted
                    invalid=true
                    error_message="Pick at least one channel".to_string()
                    class_name="docs-fieldset-custom".to_string()
                    actions=move || {
                        view! {
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                size=ui_components::ButtonSize::Sm
                            >
                                "Manage channels"
                            </ui_components::Button>
                        }
                    }
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                </Fieldset>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

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

pub(super) fn field() -> AnyView {
    let required_code = r#"<Field
  label="Email".to_string()
  required=true
  description="We'll only use this for release notes.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>"#;

    let invalid_code = r#"<Field
  orientation=FieldOrientation::Horizontal
  tone=FieldTone::Muted
  invalid=true
  error_message="A valid email is required".to_string()
  class_name="docs-field-custom".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#;

    view! {
        <ComponentPage
            title="Field"
            slug="field"
            group="Forms"
            description="Form field wrapper with centralized orientation/tone/validation/message-state modeling and stable data contracts."
        >
            <Playground title="Required + Description" code=required_code>
                <Field
                    label="Email".to_string()
                    required=true
                    description="We'll only use this for release notes.".to_string()
                    aria_label="Email field".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </Field>
            </Playground>

            <Playground title="Horizontal + Invalid + Custom Class" code=invalid_code>
                <Field
                    orientation=FieldOrientation::Horizontal
                    tone=FieldTone::Muted
                    invalid=true
                    error_message="A valid email is required".to_string()
                    class_name="docs-field-custom".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="owner@company.com"
                    />
                </Field>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn help_text() -> AnyView {
    let description_code = r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#;

    let error_code = r#"<HelpText
  invalid=true
  show_error_icon=true
  error_message="Password does not meet complexity requirements.".to_string()
  class_name="docs-help-text-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="HelpText"
            slug="help-text"
            group="Forms"
            description="Spectrum-style form assistance primitive that resolves description vs error message and tone/icon state through centralized logic contracts."
        >
            <Playground title="Description (Neutral)" code=description_code>
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Invalid + Error Icon" code=error_code>
                <div class="docs-stack">
                    <HelpText
                        invalid=true
                        show_error_icon=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        disabled=true
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

pub(super) fn time_field() -> AnyView {
    let (value, set_value) = signal(Some("09:30".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });

    let code = r#"let (value, set_value) = signal(Some("09:30".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_value.set(next);
});

<TimeField
  id_base="meeting-time".to_string()
  label="Meeting time".to_string()
  value=value
  on_value_change=on_value_change
  minute_step=15
/>"#;

    let states_code = r#"<TimeField
  id_base="ship-window".to_string()
  label="Ship window".to_string()
  tone=TimeFieldTone::Strong
  minute_step=5
  default_value="18:45".to_string()
  placeholder="hour:minute".to_string()
  class_name="docs-time-field-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="TimeField"
            slug="time-field"
            group="Forms"
            description="Time entry field with centralized hour/minute normalization and Spectrum-style state/source data contracts."
        >
            <Playground title="Controlled + Step 15" code=code>
                <div class="docs-stack">
                    <TimeField
                        id_base="docs-time-field-controlled".to_string()
                        label="Meeting time".to_string()
                        value=value
                        on_value_change=on_value_change
                        minute_step=15
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Custom Placeholder" code=states_code>
                <TimeField
                    id_base="docs-time-field-strong".to_string()
                    label="Ship window".to_string()
                    tone=TimeFieldTone::Strong
                    minute_step=5
                    default_value="18:45".to_string()
                    placeholder="hour:minute".to_string()
                    class_name="docs-time-field-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_range_picker() -> AnyView {
    let (start_day, set_start_day) = signal(Some(8_u8));
    let (end_day, set_end_day) = signal(Some(19_u8));

    let on_start_day_change = Callback::new(move |next: Option<u8>| {
        set_start_day.set(next);
    });

    let on_end_day_change = Callback::new(move |next: Option<u8>| {
        set_end_day.set(next);
    });

    let code = r#"let (start_day, set_start_day) = signal(Some(8_u8));
let (end_day, set_end_day) = signal(Some(19_u8));

<DateRangePicker
  id_base="release-window".to_string()
  start_year=2026
  start_month=6
  end_year=2026
  end_month=6
  start_day=start_day
  end_day=end_day
  on_start_day_change=Callback::new(move |next| set_start_day.set(next))
  on_end_day_change=Callback::new(move |next| set_end_day.set(next))
/>"#;

    let states_code = r#"<DateRangePicker
  id_base="ship-window".to_string()
  start_year=2026
  start_month=7
  end_year=2026
  end_month=7
  default_start_day=20
  default_end_day=12
  tone=DateRangePickerTone::Strong
  class_name="docs-date-range-picker-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="DateRangePicker"
            slug="date-range-picker"
            group="Forms"
            description="Two DatePicker composition with centralized range validity/value-shape derivation and Spectrum-style state/source contracts."
        >
            <Playground title="Controlled + Shared Month" code=code>
                <div class="docs-stack">
                    <DateRangePicker
                        id_base="docs-date-range-picker-controlled".to_string()
                        start_year=2026
                        start_month=6
                        end_year=2026
                        end_month=6
                        start_day=start_day
                        end_day=end_day
                        on_start_day_change=on_start_day_change
                        on_end_day_change=on_end_day_change
                    />

                    <span class="ui-muted">
                        "start: " {move || start_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                        " · end: " {move || end_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Invalid Range Hint" code=states_code>
                <DateRangePicker
                    id_base="docs-date-range-picker-strong".to_string()
                    start_year=2026
                    start_month=7
                    end_year=2026
                    end_month=7
                    default_start_day=20
                    default_end_day=12
                    tone=DateRangePickerTone::Strong
                    class_name="docs-date-range-picker-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_field() -> AnyView {
    let (value, set_value) = signal(Some("2026-03-14".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });

    let code = r#"let (value, set_value) = signal(Some("2026-03-14".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_value.set(next);
});

<DateField
  id_base="invoice-date".to_string()
  label="Invoice date".to_string()
  value=value
  on_value_change=on_value_change
/>"#;

    let states_code = r#"<DateField
  id_base="ship-date".to_string()
  label="Ship date".to_string()
  tone=DateFieldTone::Strong
  default_value="2026-07-22".to_string()
  placeholder="year-month-day".to_string()
  class_name="docs-date-field-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="DateField"
            slug="date-field"
            group="Forms"
            description="Segmented date entry field with centralized year/month/day normalization and Spectrum-style state/source contracts."
        >
            <Playground title="Controlled Value" code=code>
                <div class="docs-stack">
                    <DateField
                        id_base="docs-date-field-controlled".to_string()
                        label="Invoice date".to_string()
                        value=value
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "value: " {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Custom Placeholder" code=states_code>
                <DateField
                    id_base="docs-date-field-strong".to_string()
                    label="Ship date".to_string()
                    tone=DateFieldTone::Strong
                    default_value="2026-07-22".to_string()
                    placeholder="year-month-day".to_string()
                    class_name="docs-date-field-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
