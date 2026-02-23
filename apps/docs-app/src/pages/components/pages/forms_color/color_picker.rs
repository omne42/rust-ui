use super::*;

pub(crate) fn color_picker() -> AnyView {
    let (swatches, _set_swatches) = signal(vec![
        ColorSwatchPickerItem::named("#ef4444", "Red"),
        ColorSwatchPickerItem::named("#f59e0b", "Amber"),
        ColorSwatchPickerItem::named("#10b981", "Emerald"),
        ColorSwatchPickerItem::named("#3b82f6", "Blue"),
        ColorSwatchPickerItem::named("#8b5cf6", "Violet"),
    ]);

    let (workbench_color, set_workbench_color) = signal(Some("#ef4444".to_string()));
    let workbench_value: Signal<Option<String>> = Signal::derive(move || workbench_color.get());
    let workbench_selected_color: Signal<Option<String>> =
        Signal::derive(move || workbench_color.get());

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());

    let (last_value_change, set_last_value_change) = signal("none".to_string());
    let (last_selected_change, set_last_selected_change) = signal("none".to_string());
    let (last_open_change, set_last_open_change) = signal("none".to_string());
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_last_value_change.set(next.clone().unwrap_or_else(|| "none".to_string()));
        set_workbench_color.set(next);
    });
    let on_selected_change = Callback::new(move |next: Option<String>| {
        set_last_selected_change.set(next.clone().unwrap_or_else(|| "none".to_string()));
        set_workbench_color.set(next);
    });
    let on_open_change = Callback::new(move |next: bool| {
        set_last_open_change.set(if next { "true" } else { "false" }.to_string());
        set_workbench_open_raw.set(next);
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_swatch_bordered, set_workbench_swatch_bordered) = signal(true);

    let (placement_key, set_placement_key) = signal("bottom-start".to_string());
    let workbench_placement = Signal::derive(move || match placement_key.get().as_str() {
        "top-start" => ui_headless::PopoverPlacement::TopStart,
        "bottom-end" => ui_headless::PopoverPlacement::BottomEnd,
        _ => ui_headless::PopoverPlacement::BottomStart,
    });

    let (swatch_size_key, set_swatch_size_key) = signal("md".to_string());
    let workbench_swatch_size = Signal::derive(move || match swatch_size_key.get().as_str() {
        "xs" => ui::ColorSwatchSize::Xs,
        "sm" => ui::ColorSwatchSize::Sm,
        "lg" => ui::ColorSwatchSize::Lg,
        _ => ui::ColorSwatchSize::Md,
    });

    let (swatch_rounding_key, set_swatch_rounding_key) = signal("default".to_string());
    let workbench_swatch_rounding =
        Signal::derive(move || match swatch_rounding_key.get().as_str() {
            "none" => ui::ColorSwatchRounding::None,
            "full" => ui::ColorSwatchRounding::Full,
            _ => ui::ColorSwatchRounding::Default,
        });

    let (swatch_shape_key, set_swatch_shape_key) = signal("square".to_string());
    let workbench_swatch_shape = Signal::derive(move || match swatch_shape_key.get().as_str() {
        "wide" => ui::ColorSwatchShape::Wide,
        _ => ui::ColorSwatchShape::Square,
    });

    let hello_code = Signal::derive(move || {
        r##"<ColorPicker id_base="docs-color-picker-hello".to_string()>
  <div class="ui-muted">"Choose a brand color"</div>
</ColorPicker>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ColorPicker\n  id_base=\"docs-color-picker-workbench\".to_string()\n  label=\"Brand color\".to_string()\n  aria_label=\"Brand color picker\".to_string()\n  lang={}\n  dir={}\n  is_disabled={}\n  disabled={}\n  value=value\n  default_value=\"#ef4444\".to_string()\n  on_value_change=on_value_change\n  selected_color=selected_color\n  default_selected_color=\"#ef4444\".to_string()\n  on_selected_change=on_selected_change\n  open=open\n  default_open=false\n  on_open_change=on_open_change\n  motion={}\n  placement={}\n  swatch_size={}\n  swatch_rounding={}\n  swatch_shape={}\n  swatch_bordered={}\n  class_name={}\n>\n  <ColorSwatchPicker swatches=swatches selected_color=selected_color on_selected_change=on_selected_change />\n</ColorPicker>",
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "ui_headless::A11yDirection::Rtl"
            } else {
                "ui_headless::A11yDirection::Ltr"
            },
            workbench_disabled.get(),
            workbench_disabled.get(),
            if workbench_custom_motion.get() {
                "ui::ColorPickerMotion { popover: ui::PopoverMotion { initial_scale: 0.92, offset_y_px: 10.0, ..ui::PopoverMotion::default() } }"
            } else {
                "ui::ColorPickerMotion::default()"
            },
            match workbench_placement.get() {
                ui_headless::PopoverPlacement::TopStart => {
                    "ui_headless::PopoverPlacement::TopStart"
                }
                ui_headless::PopoverPlacement::BottomEnd => {
                    "ui_headless::PopoverPlacement::BottomEnd"
                }
                _ => "ui_headless::PopoverPlacement::BottomStart",
            },
            match workbench_swatch_size.get() {
                ui::ColorSwatchSize::Xs => "ui::ColorSwatchSize::Xs",
                ui::ColorSwatchSize::Sm => "ui::ColorSwatchSize::Sm",
                ui::ColorSwatchSize::Lg => "ui::ColorSwatchSize::Lg",
                _ => "ui::ColorSwatchSize::Md",
            },
            match workbench_swatch_rounding.get() {
                ui::ColorSwatchRounding::None => "ui::ColorSwatchRounding::None",
                ui::ColorSwatchRounding::Full => "ui::ColorSwatchRounding::Full",
                _ => "ui::ColorSwatchRounding::Default",
            },
            match workbench_swatch_shape.get() {
                ui::ColorSwatchShape::Wide => "ui::ColorSwatchShape::Wide",
                _ => "ui::ColorSwatchShape::Square",
            },
            workbench_swatch_bordered.get(),
            if workbench_custom_class.get() {
                "\"docs-color-picker-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ColorPickerWorkbenchConfig {{\n  id_base: \"docs-color-picker-workbench\",\n  label: Some(\"Brand color\"),\n  aria_label: Some(\"Brand color picker\"),\n  lang: {},\n  dir: {},\n  is_disabled: {},\n  disabled: Some({}),\n  value: {},\n  default_value: Some(\"#ef4444\"),\n  on_value_change: Some(\"Callback<Option<String>>\"),\n  selected_color: {},\n  default_selected_color: Some(\"#ef4444\"),\n  on_selected_change: Some(\"Callback<Option<String>>\"),\n  open: {},\n  default_open: Some(false),\n  on_open_change: Some(\"Callback<bool>\"),\n  motion: {},\n  placement: {},\n  swatch_size: {},\n  swatch_rounding: {},\n  swatch_shape: {},\n  swatch_bordered: {},\n  class_name: {},\n}}",
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_disabled.get(),
            workbench_disabled.get(),
            match workbench_color.get() {
                Some(value) => format!("Some({value:?})"),
                None => "None".to_string(),
            },
            match workbench_color.get() {
                Some(value) => format!("Some({value:?})"),
                None => "None".to_string(),
            },
            workbench_open_raw.get(),
            if workbench_custom_motion.get() {
                "ColorPickerMotion::custom"
            } else {
                "ColorPickerMotion::default"
            },
            match workbench_placement.get() {
                ui_headless::PopoverPlacement::TopStart => "TopStart",
                ui_headless::PopoverPlacement::BottomEnd => "BottomEnd",
                _ => "BottomStart",
            },
            match workbench_swatch_size.get() {
                ui::ColorSwatchSize::Xs => "Xs",
                ui::ColorSwatchSize::Sm => "Sm",
                ui::ColorSwatchSize::Lg => "Lg",
                _ => "Md",
            },
            match workbench_swatch_rounding.get() {
                ui::ColorSwatchRounding::None => "None",
                ui::ColorSwatchRounding::Full => "Full",
                _ => "Default",
            },
            match workbench_swatch_shape.get() {
                ui::ColorSwatchShape::Wide => "Wide",
                _ => "Square",
            },
            workbench_swatch_bordered.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-color-picker-workbench\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<ColorPicker id_base="docs-color-picker-matrix-default".to_string() label="Default".to_string() default_selected_color="#3b82f6".to_string()>
  <div class="ui-muted">"Default"</div>
</ColorPicker>
<ColorPicker id_base="docs-color-picker-matrix-open".to_string() label="Open".to_string() default_selected_color="#8b5cf6".to_string() default_open=true>
  <div class="ui-muted">"Open by default"</div>
</ColorPicker>
<ColorPicker id_base="docs-color-picker-matrix-disabled".to_string() label="Disabled".to_string() default_selected_color="#0ea5e9".to_string() is_disabled=true>
  <div class="ui-muted">"Disabled"</div>
</ColorPicker>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorPicker"
            slug="color-picker"
            group="Forms"
            description="Color picker with controlled value/open contracts and swatch composition."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ColorPicker id_base="docs-color-picker-hello".to_string()>
                    <div class="ui-muted">"Choose a brand color"</div>
                </ColorPicker>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full ColorPicker API and shows callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-picker-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Placement"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_placement_key.set(event_target_value(&ev))
                            >
                                <option value="bottom-start" selected=move || placement_key.get() == "bottom-start">"BottomStart"</option>
                                <option value="top-start" selected=move || placement_key.get() == "top-start">"TopStart"</option>
                                <option value="bottom-end" selected=move || placement_key.get() == "bottom-end">"BottomEnd"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Swatch size"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_swatch_size_key.set(event_target_value(&ev))
                            >
                                <option value="xs" selected=move || swatch_size_key.get() == "xs">"Xs"</option>
                                <option value="sm" selected=move || swatch_size_key.get() == "sm">"Sm"</option>
                                <option value="md" selected=move || swatch_size_key.get() == "md">"Md"</option>
                                <option value="lg" selected=move || swatch_size_key.get() == "lg">"Lg"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Rounding"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_swatch_rounding_key.set(event_target_value(&ev))
                            >
                                <option value="default" selected=move || swatch_rounding_key.get() == "default">"Default"</option>
                                <option value="none" selected=move || swatch_rounding_key.get() == "none">"None"</option>
                                <option value="full" selected=move || swatch_rounding_key.get() == "full">"Full"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Shape"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_swatch_shape_key.set(event_target_value(&ev))
                            >
                                <option value="square" selected=move || swatch_shape_key.get() == "square">"Square"</option>
                                <option value="wide" selected=move || swatch_shape_key.get() == "wide">"Wide"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                        <Switch checked=workbench_swatch_bordered set_checked=set_workbench_swatch_bordered>"Swatch bordered"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-picker-workbench-preview">
                    <ColorPicker
                        id_base="docs-color-picker-workbench".to_string()
                        label="Brand color".to_string()
                        aria_label="Brand color picker".to_string()
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        is_disabled=workbench_disabled.get()
                        disabled=workbench_disabled.get()
                        value=workbench_value
                        default_value="#ef4444".to_string()
                        on_value_change=on_value_change
                        selected_color=workbench_selected_color
                        default_selected_color="#ef4444".to_string()
                        on_selected_change=on_selected_change
                        open=workbench_open
                        default_open=false
                        on_open_change=on_open_change
                        motion=if workbench_custom_motion.get() {
                            ui::ColorPickerMotion {
                                popover: ui::PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 10.0,
                                    ..ui::PopoverMotion::default()
                                },
                            }
                        } else {
                            ui::ColorPickerMotion::default()
                        }
                        placement=workbench_placement.get()
                        swatch_size=workbench_swatch_size.get()
                        swatch_rounding=workbench_swatch_rounding.get()
                        swatch_shape=workbench_swatch_shape.get()
                        swatch_bordered=workbench_swatch_bordered.get()
                        class_name=if workbench_custom_class.get() {
                            "docs-color-picker-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <ColorSwatchPicker
                            swatches=swatches
                            selected_color=workbench_selected_color
                            on_selected_change=on_selected_change
                        />
                    </ColorPicker>
                    <span class="ui-muted">
                        "selected=" {move || workbench_color.get().unwrap_or_else(|| "none".to_string())}
                        " · open=" {move || workbench_open_raw.get()}
                    </span>
                    <span class="ui-muted">
                        "on_value_change=" {move || last_value_change.get()}
                        " · on_selected_change=" {move || last_selected_change.get()}
                        " · on_open_change=" {move || last_open_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row" data-slot="color-picker-state-matrix">
                    <ColorPicker
                        id_base="docs-color-picker-matrix-default".to_string()
                        label="Default".to_string()
                        default_selected_color="#3b82f6".to_string()
                    >
                        <div class="ui-muted">"Default"</div>
                    </ColorPicker>
                    <ColorPicker
                        id_base="docs-color-picker-matrix-open".to_string()
                        label="Open".to_string()
                        default_selected_color="#8b5cf6".to_string()
                        default_open=true
                    >
                        <div class="ui-muted">"Open by default"</div>
                    </ColorPicker>
                    <ColorPicker
                        id_base="docs-color-picker-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        default_selected_color="#0ea5e9".to_string()
                        is_disabled=true
                    >
                        <div class="ui-muted">"Disabled"</div>
                    </ColorPicker>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
