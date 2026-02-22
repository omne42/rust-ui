use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Button, ButtonVariant, OnPress, Switch, Tooltip, TooltipMotion};

fn bool_word(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

pub(super) fn tooltip() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<Tooltip content=move || view! { \"Default tooltip\" }>
  <Button variant=ButtonVariant::Secondary>\"Hover trigger\"</Button>
</Tooltip>"#
            .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_top_placement, set_workbench_top_placement) = signal(true);
    let (workbench_focus_trigger, set_workbench_focus_trigger) = signal(false);
    let (workbench_should_close_on_press, set_workbench_should_close_on_press) = signal(true);
    let (workbench_delay_ms, set_workbench_delay_ms) = signal(220_u64);
    let (workbench_close_delay_ms, set_workbench_close_delay_ms) = signal(180_u64);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);

    let open_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            TooltipMotion {
                initial_scale: 0.93,
                offset_y_px: 12.0,
                ..TooltipMotion::default()
            }
        } else {
            TooltipMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let placement = if workbench_top_placement.get() {
            "ui_headless::TooltipPlacement::Top"
        } else {
            "ui_headless::TooltipPlacement::Bottom"
        };
        let trigger = if workbench_focus_trigger.get() {
            "ui_headless::TooltipTriggerMode::Focus"
        } else {
            "ui_headless::TooltipTriggerMode::Hover"
        };
        let motion = if workbench_custom_motion.get() {
            "TooltipMotion { initial_scale: 0.93, offset_y_px: 12.0, ..TooltipMotion::default() }"
        } else {
            "TooltipMotion::default()"
        };
        let class_name = if workbench_custom_class_name.get() {
            "docs-tooltip-workbench"
        } else {
            ""
        };
        let id = if workbench_custom_id.get() {
            "docs-tooltip-workbench"
        } else {
            ""
        };

        vec![
            "<Tooltip".to_string(),
            "  content=move || view! { \"Workbench content\" }".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            format!("  placement={placement}"),
            "  is_open=Signal::derive(move || open_raw.get())".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            format!("  delay_ms={}", workbench_delay_ms.get()),
            format!("  close_delay_ms={}", workbench_close_delay_ms.get()),
            format!("  trigger={trigger}"),
            format!(
                "  should_close_on_press={}",
                bool_word(workbench_should_close_on_press.get())
            ),
            format!("  motion={motion}"),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
            format!("  id={}.to_string()", rust_string_literal(id)),
            ">".to_string(),
            "  <Button variant=ButtonVariant::Secondary>\"Workbench trigger\"</Button>".to_string(),
            "</Tooltip>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_top_placement.get() {
            "TooltipPlacement::Top"
        } else {
            "TooltipPlacement::Bottom"
        };
        let trigger = if workbench_focus_trigger.get() {
            "TooltipTriggerMode::Focus"
        } else {
            "TooltipTriggerMode::Hover"
        };
        let motion = if workbench_custom_motion.get() {
            "TooltipMotion::custom"
        } else {
            "TooltipMotion::default"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-tooltip-workbench")
        } else {
            None
        };
        let id = if workbench_custom_id.get() {
            Some("docs-tooltip-workbench")
        } else {
            None
        };

        format!(
            "TooltipActualConfig {{\n  content: \"Workbench content\",\n  is_disabled: Some({}),\n  disabled: {},\n  placement: {placement},\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  delay_ms: {},\n  close_delay_ms: {},\n  trigger: {trigger},\n  should_close_on_press: {},\n  motion: {motion},\n  class_name: {class_name:?},\n  id: {id:?},\n}}",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_open_raw.get()),
            workbench_open_change_count.get(),
            workbench_delay_ms.get(),
            workbench_close_delay_ms.get(),
            bool_word(workbench_should_close_on_press.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Tooltip content=move || view! { \"Default matrix tooltip\" }>
  <Button variant=ButtonVariant::Secondary>\"Default\"</Button>
</Tooltip>
<Tooltip
  placement=ui_headless::TooltipPlacement::Bottom
  trigger=ui_headless::TooltipTriggerMode::Focus
  content=move || view! { \"Bottom + focus trigger\" }
>
  <Button variant=ButtonVariant::Secondary>\"Focus\"</Button>
</Tooltip>
<Tooltip
  is_disabled=Some(true)
  disabled=true
  content=move || view! { \"Disabled tooltip\" }
>
  <Button variant=ButtonVariant::Secondary is_disabled=true>\"Disabled\"</Button>
</Tooltip>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Tooltip"
            slug="tooltip"
            group="Overlays"
            description="Tooltip playground with full API workbench and matrix comparison."
        >
            <Playground title="Hello World (Default Tooltip)" code_signal=hello_code>
                <div class="docs-row">
                    <Tooltip content=move || view! { "Default tooltip" }>
                        <Button variant=ButtonVariant::Secondary>"Hover trigger"</Button>
                    </Tooltip>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tooltip-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_top_placement set_checked=set_workbench_top_placement>
                            "placement top"
                        </Switch>
                        <Switch checked=workbench_focus_trigger set_checked=set_workbench_focus_trigger>
                            "trigger focus"
                        </Switch>
                        <Switch
                            checked=workbench_should_close_on_press
                            set_checked=set_workbench_should_close_on_press
                        >
                            "should_close_on_press"
                        </Switch>
                        <label class="docs-search__label">
                            "delay_ms: " {move || workbench_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1500"
                                step="50"
                                prop:value=move || workbench_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(220);
                                    set_workbench_delay_ms.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "close_delay_ms: " {move || workbench_close_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1000"
                                step="50"
                                prop:value=move || workbench_close_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(180);
                                    set_workbench_close_delay_ms.set(next);
                                }
                            />
                        </label>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <Button variant=ButtonVariant::Secondary on_press=open_workbench>
                                "Open"
                            </Button>
                            <Button variant=ButtonVariant::Secondary on_press=close_workbench>
                                "Close"
                            </Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="tooltip-workbench-feedback">
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                        " · on_open_change: " {move || workbench_open_change_count.get()}
                    </span>
                </div>
                <Tooltip
                    content=move || view! { "Workbench content" }
                    is_disabled=workbench_is_disabled.get()
                    disabled=workbench_disabled_alias.get()
                    placement=if workbench_top_placement.get() {
                        ui_headless::TooltipPlacement::Top
                    } else {
                        ui_headless::TooltipPlacement::Bottom
                    }
                    is_open=workbench_open
                    open=workbench_open
                    default_open=false
                    on_open_change=on_workbench_open_change
                    delay_ms=workbench_delay_ms.get()
                    close_delay_ms=workbench_close_delay_ms.get()
                    trigger=if workbench_focus_trigger.get() {
                        ui_headless::TooltipTriggerMode::Focus
                    } else {
                        ui_headless::TooltipTriggerMode::Hover
                    }
                    should_close_on_press=workbench_should_close_on_press.get()
                    motion=workbench_motion.get()
                    class_name=if workbench_custom_class_name.get() {
                        "docs-tooltip-workbench".to_string()
                    } else {
                        String::new()
                    }
                    id=if workbench_custom_id.get() {
                        "docs-tooltip-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <Button variant=ButtonVariant::Secondary>"Workbench trigger"</Button>
                </Tooltip>
            </Playground>

            <Playground title="State Matrix (Default / Focus / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="tooltip-state-matrix">
                    <Tooltip content=move || view! { "Default matrix tooltip" }>
                        <Button variant=ButtonVariant::Secondary>"Default"</Button>
                    </Tooltip>
                    <Tooltip
                        placement=ui_headless::TooltipPlacement::Bottom
                        trigger=ui_headless::TooltipTriggerMode::Focus
                        content=move || view! { "Bottom + focus trigger" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Focus"</Button>
                    </Tooltip>
                    <Tooltip
                        is_disabled=true
                        disabled=true
                        content=move || view! { "Disabled tooltip" }
                    >
                        <Button variant=ButtonVariant::Secondary is_disabled=true>
                            "Disabled"
                        </Button>
                    </Tooltip>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
