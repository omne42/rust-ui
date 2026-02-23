use super::*;

pub(crate) fn radio() -> AnyView {
    let (checked, set_checked) = signal(false);
    let on_checked_change = Callback::new(move |next: bool| set_checked.set(next));
    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);
    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);

<Radio id="r1".to_string() label="Standalone".to_string() is_checked=Signal::derive(move || checked.get()) on_checked_change=Callback::new(move |next: bool| set_checked.set(next)) />"#
        .to_string()
    });
    let matrix_code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Radio
  id="r1".to_string()
  label="Standalone".to_string()
  is_checked=Signal::derive(move || checked.get())
  on_checked_change=Callback::new(move |next: bool| set_checked.set(next))
/>
<Radio
  id="r2".to_string()
  label="Disabled on".to_string()
  is_checked=Signal::derive(move || disabled_checked.get())
  on_checked_change=Callback::new(move |next: bool| set_disabled_checked.set(next))
  is_disabled=true
/>
<Radio
  id="r3".to_string()
  label="Disabled off".to_string()
  is_checked=Signal::derive(move || disabled_unchecked.get())
  on_checked_change=Callback::new(move |next: bool| set_disabled_unchecked.set(next))
  is_disabled=true
/>
<Radio
  id="r4".to_string()
  label="Uncontrolled default on".to_string()
  default_checked=true
/>"#
        .to_string()
    });

    let (workbench_default_checked, set_workbench_default_checked) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_checked_change_count, set_workbench_checked_change_count) = signal(0_u32);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_checked_change = Callback::new(move |next: bool| {
        set_checked.set(next);
        set_workbench_checked_change_count.update(|count| *count += 1);
    });
    let on_workbench_change = Callback::new(move |next: bool| {
        set_checked.set(next);
        set_workbench_change_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Radio\n  id=\"docs-radio-workbench\".to_string()\n  label=\"Workbench\".to_string()\n  is_checked=Signal::derive(move || checked.get())\n  checked=Signal::derive(move || checked.get())\n  default_checked={}\n  is_disabled=Some({})\n  disabled={}\n  motion={}\n  class_name={}\n  on_checked_change=Callback::new(move |_| {{}})\n  on_change=Callback::new(move |_| {{}})\n/>",
            workbench_default_checked.get(),
            workbench_is_disabled.get(),
            workbench_disabled.get(),
            if workbench_custom_motion.get() {
                "RadioMotion { hover_scale: 1.08, tap_scale: 0.94, ..RadioMotion::default() }"
            } else {
                "RadioMotion::default()"
            },
            if workbench_custom_class.get() {
                "\"docs-radio-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "RadioActualConfig {{\n  id: \"docs-radio-workbench\",\n  label: \"Workbench\",\n  is_checked: Some({}),\n  checked: Some({}),\n  default_checked: Some({}),\n  is_disabled: Some({}),\n  disabled: {},\n  motion: {},\n  class_name: {},\n  on_checked_change: \"count={}\",\n  on_change: \"count={}\",\n}}",
            checked.get(),
            checked.get(),
            workbench_default_checked.get(),
            workbench_is_disabled.get(),
            workbench_disabled.get(),
            if workbench_custom_motion.get() {
                "RadioMotion::custom"
            } else {
                "RadioMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-radio-custom\")"
            } else {
                "None"
            },
            workbench_checked_change_count.get(),
            workbench_change_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="Radio"
            slug="radio"
            group="Forms"
            description="Standalone radio button (use RadioGroup for semantics)."
        >
            <Playground title="Hello World（默认路径）" code_signal=code>
                <div class="docs-row">
                    <Radio
                        id="docs-radio".to_string()
                        label="Standalone".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        on_checked_change=on_checked_change
                    />
                    <span class="ui-muted">"checked: " {move || checked.get()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="radio-workbench-controls">
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_default_checked.get() on:change=move |ev| set_workbench_default_checked.set(event_target_checked(&ev)) />
                            " default_checked"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_is_disabled.get() on:change=move |ev| set_workbench_is_disabled.set(event_target_checked(&ev)) />
                            " is_disabled"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_disabled.get() on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev)) />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_motion.get() on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev)) />
                            " motion"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_class.get() on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev)) />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Radio
                        id="docs-radio-workbench".to_string()
                        label="Workbench".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        checked=Signal::derive(move || checked.get())
                        default_checked=workbench_default_checked.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        motion=if workbench_custom_motion.get() {
                            ui::RadioMotion {
                                hover_scale: 1.08,
                                tap_scale: 0.94,
                                ..ui::RadioMotion::default()
                            }
                        } else {
                            ui::RadioMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-radio-custom".to_string()
                        } else {
                            String::new()
                        }
                        on_checked_change=on_workbench_checked_change
                        on_change=on_workbench_change
                    />
                    <span class="ui-muted">
                        "checked: " {move || checked.get()}
                        " · on_checked_change: " {move || workbench_checked_change_count.get()}
                        " · on_change: " {move || workbench_change_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="状态矩阵（受控 + disabled）" code_signal=matrix_code>
                <div class="docs-row">
                    <Radio
                        id="docs-radio-controlled".to_string()
                        label="Controlled".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        on_checked_change=on_checked_change
                    />
                    <Radio
                        id="docs-radio-disabled-on".to_string()
                        label="Disabled on".to_string()
                        is_checked=Signal::derive(move || disabled_checked.get())
                        on_checked_change=Callback::new(move |next: bool| set_disabled_checked.set(next))
                        is_disabled=true
                    />
                    <Radio
                        id="docs-radio-disabled-off".to_string()
                        label="Disabled off".to_string()
                        is_checked=Signal::derive(move || disabled_unchecked.get())
                        on_checked_change=Callback::new(move |next: bool| set_disabled_unchecked.set(next))
                        is_disabled=true
                    />
                    <Radio
                        id="docs-radio-uncontrolled-default".to_string()
                        label="Uncontrolled default on".to_string()
                        default_checked=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
