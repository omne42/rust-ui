use super::*;

pub(crate) fn switch() -> AnyView {
    use leptos::html;

    let (checked, set_checked) = signal(true);

    let (system_enabled, set_system_enabled) = signal(true);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_system_checked_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);

    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(true);

<Switch
  checked=checked
  set_checked=set_checked
  on_checked_change=Callback::new(move |_| {})
>
  "Notifications"
</Switch>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (system_enabled, set_system_enabled) = signal(true);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Switch checked=system_enabled set_checked=set_system_enabled>
  "System alerts"
</Switch>
<Switch checked=disabled_checked set_checked=set_disabled_checked disabled=true>
  "Disabled on"
</Switch>
<Switch checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>
  "Disabled off"
</Switch>"#
            .to_string()
    });

    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_pressed_width, set_workbench_pressed_width) = signal(22_u16);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_checked_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
        set_workbench_change_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Switch\n  checked=checked\n  set_checked=set_checked\n  disabled={}\n  on_checked_change=Callback::new(move |_| {{}})\n  pressed_width_px={}\n  motion=SwitchMotion::default()\n  class_name={}\n  aria_label={}\n  node_ref=NodeRef::new()\n>\n  \"Notifications\"\n</Switch>",
            workbench_disabled.get(),
            workbench_pressed_width.get(),
            if workbench_custom_class.get() {
                "\"docs-switch-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_aria.get() {
                "\"Notifications toggle\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SwitchActualConfig {{\n  checked: {},\n  set_checked: \"bound(set_checked)\",\n  disabled: {},\n  on_checked_change: \"count={}\",\n  pressed_width_px: {},\n  motion: SwitchMotion::default(),\n  class_name: {},\n  aria_label: {},\n  node_ref: \"workbench_node_ref\",\n}}",
            checked.get(),
            workbench_disabled.get(),
            workbench_change_count.get(),
            workbench_pressed_width.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-switch-custom\")"
            } else {
                "None"
            },
            if workbench_custom_aria.get() {
                "Some(\"Notifications toggle\")"
            } else {
                "None"
            }
        )
    });

    view! {
        <ComponentPage
            title="Switch"
            slug="switch"
            group="Forms"
            description="Switch toggle with baseline-level spring thumb motion and baseline-style root state attrs."
        >
            <Playground title="Hello World (Default Switch)" code_signal=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Switch
                            checked=checked
                            set_checked=set_checked
                            on_checked_change=on_system_checked_change
                        >
                            "Notifications"
                        </Switch>
                        <span class="ui-muted">"checked: " {move || checked.get()}</span>
                    </div>
                    <span class="ui-muted">
                        "last on_checked_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="switch-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            "pressed_width_px (" {move || workbench_pressed_width.get()} ")"
                            <input
                                type="range"
                                min="14"
                                max="32"
                                step="1"
                                prop:value=move || workbench_pressed_width.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(22)
                                        .clamp(14, 32);
                                    set_workbench_pressed_width.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Switch
                        checked=checked
                        set_checked=set_checked
                        disabled=workbench_disabled.get()
                        on_checked_change=on_workbench_checked_change
                        pressed_width_px=f64::from(workbench_pressed_width.get())
                        motion=ui::SwitchMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-switch-custom".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Notifications toggle".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                    >
                        "Notifications"
                    </Switch>
                    <span class="ui-muted">
                        "checked: " {move || checked.get()}
                        " · on_checked_change: " {move || workbench_change_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Switch checked=system_enabled set_checked=set_system_enabled>
                            "System alerts"
                        </Switch>
                        <span class="ui-muted">
                            "system enabled: "
                            {move || system_enabled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Switch checked=disabled_checked set_checked=set_disabled_checked disabled=true>
                            "Disabled on"
                        </Switch>
                        <Switch checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>
                            "Disabled off"
                        </Switch>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
