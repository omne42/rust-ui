use crate::date_picker::{
    DatePickerMotion, DatePickerStateInput, DatePickerStrings,
    logic::{self, DatePickerTone},
};
use crate::{Button, Calendar, CalendarFirstWeekday, CalendarTone, OnPress, Popover};
use leptos::{html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::PopoverPlacement;
use ui_headless::i18n;
use ui_headless::use_presence;

#[component]
pub fn DatePicker(
    id_base: String,
    year: i32,
    month: u8,
    #[prop(optional)] tone: DatePickerTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] selected_day: Option<Signal<Option<u8>>>,
    #[prop(optional)] default_selected_day: Option<u8>,
    #[prop(optional)] on_selected_day_change: Option<Callback<Option<u8>>>,
    #[prop(optional)] first_weekday: CalendarFirstWeekday,
    #[prop(optional)] show_outside_days: bool,
    #[prop(optional)] popover_placement: PopoverPlacement,
    #[prop(optional)] motion: DatePickerMotion,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<DatePickerStrings>();
    let calendar_aria_label = strings.calendar_aria_label.as_ref().to_string();
    let calendar_aria_label = StoredValue::new(calendar_aria_label);
    let normalized_month = logic::normalize_month(month);

    let open_state = overlay_open::use_controllable_open_state_traced(
        "date-picker",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let default_selected_day =
        logic::normalize_selected_day(default_selected_day, year, normalized_month);
    let selected_state = overlay_open::use_controllable_state(
        selected_day,
        Some(default_selected_day),
        on_selected_day_change,
    );
    let selected_day = selected_state.value;
    let request_selected_day_change = selected_state.request_change;

    let (placeholder, has_custom_placeholder) = logic::normalize_placeholder(placeholder);
    let placeholder = StoredValue::new(placeholder);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);

    let motion = crate::date_picker::motion::sanitize_motion(motion);
    let has_custom_motion = motion != DatePickerMotion::default();
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let ids = logic::resolve_ids(&id_base);
    let trigger_id = StoredValue::new(ids.trigger_id);
    let panel_id = StoredValue::new(ids.panel_id);

    let state = Memo::new(move |_| {
        logic::resolve_state(DatePickerStateInput {
            year,
            month: normalized_month,
            selected_day: selected_day.get(),
            tone,
            disabled,
            open: open.get(),
            has_custom_placeholder,
            has_custom_aria_label,
            has_custom_class_name,
            has_custom_motion,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let trigger_label = Memo::new(move |_| {
        logic::resolve_trigger_label(
            year,
            normalized_month,
            state.get().selected_day,
            &placeholder.get_value(),
        )
    });

    let calendar_tone = match tone {
        DatePickerTone::Default => CalendarTone::Default,
        DatePickerTone::Quiet => CalendarTone::Quiet,
        DatePickerTone::Strong => CalendarTone::Strong,
    };

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let presence = use_presence(open);

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if disabled {
            return;
        }
        request_open_change.run(!open.get_untracked());
    });

    let on_day_press: Callback<u8> = Callback::new(move |day| {
        if disabled {
            return;
        }
        request_selected_day_change.run(Some(day));
        request_open_change.run(false);
    });

    let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));
    let on_day_press = StoredValue::new(on_day_press);

    let aria_controls = ui_headless::aria_controls_when_open(open, panel_id.get_value());

    view! {
        <div
            class=move || class.get()
            data-slot="date-picker"
            data-tone=move || state.get().tone_attr
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-closed=move || state.get().is_closed.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-value=move || state.get().has_value.then_some("true")
            data-selected-day=move || state.get().selected_day.map(|day| day.to_string())
            data-year=move || state.get().year.to_string()
            data-month=move || state.get().month.to_string()
            data-placeholder-source=move || state.get().placeholder_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            role="group"
            aria-label=aria_label
        >
            <div class="ui-date-picker__trigger-wrap" data-slot="date-picker-trigger">
                <Button
                    id=trigger_id.get_value()
                    node_ref=anchor_ref
                    on_press=on_trigger_press
                    disabled=disabled
                    aria_haspopup="dialog"
                    aria_expanded=open
                    aria_controls_signal=aria_controls
                    class_name="ui-date-picker__trigger".to_string()
                >
                    {move || trigger_label.get()}
                </Button>
            </div>

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=popover_placement
                    motion=motion.popover
                    is_modal=false
                    on_exit_complete=presence.finish_exit
                >
                    <div
                        id=panel_id.get_value()
                        class="ui-date-picker__panel"
                        data-slot="date-picker-panel"
                        role="dialog"
                        aria-labelledby=trigger_id.get_value()
                    >
                        {move || {
                            let selected_day = selected_day.get();
                            view! {
                                    <Calendar
                                    year=year
                                    month=normalized_month
                                    tone=calendar_tone
                                    first_weekday=first_weekday
                                        show_outside_days=show_outside_days
                                        selected_day=selected_day
                                        on_day_press=Some(on_day_press.get_value())
                                        aria_label=calendar_aria_label.get_value()
                                        class_name="ui-date-picker__calendar".to_string()
                                    />
                            }
                        }}
                    </div>
                </Popover>
            </Show>
        </div>
    }
}
