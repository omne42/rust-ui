use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{Button, ButtonVariant, HoverCard, HoverCardMotion, Switch};

fn bool_word(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}

pub(super) fn hover_card() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<HoverCard content=move || view! { \"Hover card content\" }>
  <Button variant=ButtonVariant::Secondary>\"Hover trigger\"</Button>
</HoverCard>"#
            .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_top_end, set_workbench_top_end) = signal(false);
    let (workbench_open_delay_ms, set_workbench_open_delay_ms) = signal(180_u64);
    let (workbench_close_delay_ms, set_workbench_close_delay_ms) = signal(220_u64);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let open_workbench = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            HoverCardMotion {
                initial_scale: 0.95,
                offset_y_px: 14.0,
                ..HoverCardMotion::default()
            }
        } else {
            HoverCardMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let placement = if workbench_top_end.get() {
            "ui_headless::PopoverPlacement::TopEnd"
        } else {
            "ui_headless::PopoverPlacement::BottomStart"
        };
        let motion = if workbench_custom_motion.get() {
            "HoverCardMotion { initial_scale: 0.95, offset_y_px: 14.0, ..HoverCardMotion::default() }"
        } else {
            "HoverCardMotion::default()"
        };
        let class_name = if workbench_custom_class_name.get() {
            "docs-hover-card-workbench"
        } else {
            ""
        };
        let id = if workbench_custom_id.get() {
            "docs-hover-card-workbench"
        } else {
            ""
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "rtl"
        } else {
            "ltr"
        };

        vec![
            "<HoverCard".to_string(),
            "  content=move || view! { \"Workbench content\" }".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            format!("  placement={placement}"),
            "  is_open=Signal::derive(move || open_raw.get())".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            format!("  open_delay_ms={}", workbench_open_delay_ms.get()),
            format!("  close_delay_ms={}", workbench_close_delay_ms.get()),
            format!("  motion={motion}"),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
            format!("  id={}.to_string()", rust_string_literal(id)),
            format!("  lang={}.to_string()", rust_string_literal(lang)),
            format!("  dir={}.to_string()", rust_string_literal(dir)),
            ">".to_string(),
            "  <Button variant=ButtonVariant::Secondary>\"Workbench trigger\"</Button>".to_string(),
            "</HoverCard>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let placement = if workbench_top_end.get() {
            "PopoverPlacement::TopEnd"
        } else {
            "PopoverPlacement::BottomStart"
        };
        let motion = if workbench_custom_motion.get() {
            "HoverCardMotion::custom"
        } else {
            "HoverCardMotion::default"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-hover-card-workbench")
        } else {
            None
        };
        let id = if workbench_custom_id.get() {
            Some("docs-hover-card-workbench")
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            Some("rtl")
        } else {
            Some("ltr")
        };

        format!(
            "HoverCardActualConfig {{\n  content: \"Workbench content\",\n  is_disabled: Some({}),\n  disabled: Some({}),\n  placement: {placement},\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  open_delay_ms: Some({}),\n  close_delay_ms: Some({}),\n  motion: {motion},\n  class_name: {class_name:?},\n  id: {id:?},\n  lang: {lang:?},\n  dir: {dir:?},\n}}",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_open_raw.get()),
            workbench_open_change_count.get(),
            workbench_open_delay_ms.get(),
            workbench_close_delay_ms.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<HoverCard content=move || view! { \"Default\" }>
  <Button variant=ButtonVariant::Secondary>\"Default\"</Button>
</HoverCard>
<HoverCard
  placement=ui_headless::PopoverPlacement::TopEnd
  open_delay_ms=220
  close_delay_ms=260
  content=move || view! { \"TopEnd + delayed\" }
>
  <Button variant=ButtonVariant::Secondary>\"Delayed\"</Button>
</HoverCard>
<HoverCard
  is_disabled=true
  disabled=true
  content=move || view! { \"Disabled\" }
>
  <Button variant=ButtonVariant::Secondary>\"Disabled\"</Button>
</HoverCard>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="HoverCard"
            slug="hover-card"
            group="Overlays"
            description="HoverCard playground with full API workbench and matrix comparison."
        >
            <Playground title="Hello World (Default HoverCard)" code_signal=hello_code>
                <div class="docs-row">
                    <HoverCard content=move || view! { "Hover card content" }>
                        <Button variant=ButtonVariant::Secondary>"Hover trigger"</Button>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="hover-card-workbench-controls">
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_top_end set_checked=set_workbench_top_end>
                            "placement top-end"
                        </Switch>
                        <label class="docs-search__label">
                            "open_delay_ms: " {move || workbench_open_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1000"
                                step="20"
                                prop:value=move || workbench_open_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(180);
                                    set_workbench_open_delay_ms.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "close_delay_ms: " {move || workbench_close_delay_ms.get()}
                            <input
                                type="range"
                                min="0"
                                max="1000"
                                step="20"
                                prop:value=move || workbench_close_delay_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev).parse::<u64>().unwrap_or(220);
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
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir rtl"
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
                <div class="docs-stack docs-stack--tight" data-slot="hover-card-workbench-feedback">
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                        " · on_open_change: " {move || workbench_open_change_count.get()}
                    </span>
                </div>
                <HoverCard
                    content=move || view! { "Workbench content" }
                    is_disabled=workbench_is_disabled.get()
                    disabled=workbench_disabled_alias.get()
                    placement=if workbench_top_end.get() {
                        ui_headless::PopoverPlacement::TopEnd
                    } else {
                        ui_headless::PopoverPlacement::BottomStart
                    }
                    is_open=workbench_open
                    open=workbench_open
                    default_open=false
                    on_open_change=on_workbench_open_change
                    open_delay_ms=workbench_open_delay_ms.get()
                    close_delay_ms=workbench_close_delay_ms.get()
                    motion=workbench_motion.get()
                    class_name=if workbench_custom_class_name.get() {
                        "docs-hover-card-workbench".to_string()
                    } else {
                        String::new()
                    }
                    id=if workbench_custom_id.get() {
                        "docs-hover-card-workbench".to_string()
                    } else {
                        String::new()
                    }
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        "rtl".to_string()
                    } else {
                        "ltr".to_string()
                    }
                >
                    <Button variant=ButtonVariant::Secondary>"Workbench trigger"</Button>
                </HoverCard>
            </Playground>

            <Playground title="State Matrix (Default / Delayed / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="hover-card-state-matrix">
                    <HoverCard content=move || view! { "Default" }>
                        <Button variant=ButtonVariant::Secondary>"Default"</Button>
                    </HoverCard>
                    <HoverCard
                        placement=ui_headless::PopoverPlacement::TopEnd
                        open_delay_ms=220
                        close_delay_ms=260
                        content=move || view! { "TopEnd + delayed" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Delayed"</Button>
                    </HoverCard>
                    <HoverCard
                        is_disabled=true
                        disabled=true
                        content=move || view! { "Disabled" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Disabled"</Button>
                    </HoverCard>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
