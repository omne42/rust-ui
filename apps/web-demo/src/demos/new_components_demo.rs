use leptos::{html, prelude::*};
use ui::{
    Button, InputOtp, Meter, MotionRipple, NumberField, OnPress, Progress, ProgressCircle,
    RippleMotion, SegmentedControl, SegmentedControlSize, Sheet, SheetPlacement, SlidingNumber,
    StaticNumber,
};
use ui_layout::{AutoHeight, ScrollShadow, Separator, SeparatorOrientation};

#[component]
pub fn NewComponentsDemo() -> impl IntoView {
    // Meter + NumberField (value driver)
    let (meter_value, set_meter_value) = signal(42_i64);
    let meter_progress = Signal::derive(move || Some(meter_value.get() as f64));

    // InputOtp
    let (otp_value, set_otp_value) = signal(String::new());

    // SegmentedControl
    let options = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Settings".to_string(),
    ];
    let (seg_selected, set_seg_selected) = signal(Some(0_usize));

    // Sheet presence (keep mounted while exit animates)
    let (sheet_open, set_sheet_open) = signal(false);
    let is_sheet_open = Signal::derive(move || sheet_open.get());
    let (is_sheet_present, set_sheet_present) = signal(is_sheet_open.get_untracked());
    Effect::new(move |_| {
        if is_sheet_open.get() {
            set_sheet_present.set(true);
        }
    });
    let close_sheet: OnPress = Callback::new(move |_| set_sheet_open.set(false));
    let open_sheet: OnPress = Callback::new(move |_| set_sheet_open.set(true));
    let on_sheet_exit_complete: Callback<()> = Callback::new(move |_| set_sheet_present.set(false));

    // Progress + ProgressCircle reuse meter_value
    let progress_value = Signal::derive(move || Some(meter_value.get() as f64));

    // SlidingNumber
    let (sliding_value, set_sliding_value) = signal(12345.67_f64);

    // AutoHeight
    let (auto_height_open, set_auto_height_open) = signal(false);

    // Ripple demo surface
    let ripple_ref: NodeRef<html::Span> = NodeRef::new();
    let on_ripple_click = move |_| {
        ui::ripple::trigger_ripple(ripple_ref, RippleMotion::default());
    };

    view! {
        <>
            <Show when=move || is_sheet_present.get()>
                <Sheet
                    open=is_sheet_open
                    placement=SheetPlacement::Bottom
                    on_close=close_sheet
                    on_exit_complete=on_sheet_exit_complete
                >
                    move || {
                        view! {
                            <div class="demo-stack">
                                <div class="demo-kv">"Sheet content"</div>
                                <p class="demo-kv">
                                    "Esc or click backdrop closes. Uses spring motion + focus trap."
                                </p>
                                <div class="demo-row demo-row--end">
                                    <Button on_press=close_sheet>"Close"</Button>
                                </div>
                            </div>
                        }
                    }
                </Sheet>
            </Show>

            <section id="new-components" class="demo-card">
                <h2>"New components"</h2>
                <p>"InputOtp / NumberField / Meter / SegmentedControl / ScrollShadow / Sheet / Separator / Progress / MotionRipple / AutoHeight / Number"</p>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"Meter + NumberField"</div>
                        <Meter
                            id="demo-meter".to_string()
                            label="Completion".to_string()
                            value=meter_progress
                            min=0.0
                            max=100.0
                        />
                        <NumberField
                            id="demo-meter-number".to_string()
                            label="Value".to_string()
                            value=meter_value
                            set_value=set_meter_value
                            min=0
                            max=100
                        />
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"InputOtp"</div>
                        <InputOtp
                            id_base="demo-otp".to_string()
                            label="One-time code".to_string()
                            value=otp_value
                            set_value=set_otp_value
                            length=6
                        />
                        <div class="demo-kv">
                            "value: " {move || otp_value.get()}
                        </div>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"SegmentedControl"</div>
                        <SegmentedControl
                            id_base="demo-segments".to_string()
                            options=options.clone()
                            selected_index=seg_selected
                            set_selected_index=set_seg_selected
                            size=SegmentedControlSize::Default
                        />
                        <div class="demo-kv">
                            "selected: " {move || seg_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        </div>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"ScrollShadow"</div>
                        <ScrollShadow max_height_px=160>
                            <div class="demo-stack">
                                {(1..=24).map(|idx| view! { <div class="demo-kv">{format!("Row {idx}")}</div> }).collect_view()}
                            </div>
                        </ScrollShadow>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"Separator"</div>
                        <div class="demo-stack demo-stack--tight">
                            <span>"Above"</span>
                            <Separator />
                            <span>"Below"</span>
                        </div>
                        <div class="demo-row demo-row--tall">
                            <span>"Left"</span>
                            <Separator orientation=SeparatorOrientation::Vertical />
                            <span>"Right"</span>
                        </div>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"Progress / ProgressCircle"</div>
                        <Progress aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
                        <div class="demo-row">
                            <ProgressCircle aria_label="Determinate".to_string() value=progress_value min=0.0 max=100.0 />
                            <ProgressCircle aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
                        </div>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-row">
                    <Button on_press=open_sheet>"Open Sheet"</Button>
                    <span class="demo-kv">"open: " {move || sheet_open.get()}</span>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"MotionRipple"</div>
                        <button class="demo-ripple-surface" on:click=on_ripple_click type="button">
                            <span class="demo-ripple-label">"Click for ripple"</span>
                            <MotionRipple node_ref=ripple_ref class_name="demo-ripple-item".to_string() />
                        </button>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"AutoHeight"</div>
                        <Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_auto_height_open.update(|v| *v = !*v))
                        >
                            {move || if auto_height_open.get() { "Collapse" } else { "Expand" }}
                        </Button>
                        <AutoHeight class_name="demo-auto-height".to_string()>
                            <Show when=move || auto_height_open.get()>
                                <div class="demo-kv">
                                    "Animates height changes via spring-driven CSS variables (ResizeObserver + ui-motion spring)."
                                </div>
                                <div class="demo-kv">
                                    "This content toggles without layout jitter thanks to overflow handling."
                                </div>
                            </Show>
                        </AutoHeight>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"StaticNumber"</div>
                        <div class="demo-row">
                            <StaticNumber
                                number=12345.678
                                decimal_places=2
                                thousand_separator=",".to_string()
                            />
                            <StaticNumber number=-1.5 decimal_separator=",".to_string() />
                        </div>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"SlidingNumber"</div>
                        <div class="demo-row">
                            <Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_sliding_value.update(|v| *v -= 1.0))
                            >
                                "-1"
                            </Button>
                            <Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| set_sliding_value.update(|v| *v += 1.0))
                            >
                                "+1"
                            </Button>
                            <span class="demo-kv">"value:"</span>
                            <SlidingNumber
                                number=sliding_value
                                decimal_places=2
                                thousand_separator=",".to_string()
                            />
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
