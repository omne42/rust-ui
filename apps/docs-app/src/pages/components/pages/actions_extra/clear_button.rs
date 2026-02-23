use super::*;

pub(crate) fn clear_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();

    let (workbench_variant_key, set_workbench_variant_key) = signal("default".to_string());
    let (workbench_focus_mode_key, set_workbench_focus_mode_key) = signal("default".to_string());
    let (workbench_button_type_key, set_workbench_button_type_key) = signal("button".to_string());
    let (workbench_inset, set_workbench_inset) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_slot, set_workbench_custom_slot) = signal(false);
    let (workbench_visible_raw, set_workbench_visible_raw) = signal(true);
    let (workbench_disabled_signal_raw, set_workbench_disabled_signal_raw) = signal(false);
    let (workbench_hide_aria, set_workbench_hide_aria) = signal(false);

    let workbench_visible: Signal<bool> = Signal::derive(move || workbench_visible_raw.get());
    let workbench_disabled_signal: Signal<bool> =
        Signal::derive(move || workbench_disabled_signal_raw.get());

    let (press_count, set_press_count) = signal(0_u32);
    let (click_count, set_click_count) = signal(0_u32);
    let (blur_count, set_blur_count) = signal(0_u32);
    let (pointer_down_count, set_pointer_down_count) = signal(0_u32);
    let (pointer_up_count, set_pointer_up_count) = signal(0_u32);
    let (pointer_cancel_count, set_pointer_cancel_count) = signal(0_u32);
    let (pointer_enter_count, set_pointer_enter_count) = signal(0_u32);
    let (pointer_leave_count, set_pointer_leave_count) = signal(0_u32);
    let (last_key_down, set_last_key_down) = signal("none".to_string());
    let (last_key_up, set_last_key_up) = signal("none".to_string());

    let on_press = Callback::new(move |_| set_press_count.update(|count| *count += 1));
    let on_click = Callback::new(move |()| set_click_count.update(|count| *count += 1));
    let on_blur = Callback::new(move |()| set_blur_count.update(|count| *count += 1));
    let on_pointer_down = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_down_count.update(|count| *count += 1)
    });
    let on_pointer_up = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_up_count.update(|count| *count += 1)
    });
    let on_pointer_cancel = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_cancel_count.update(|count| *count += 1)
    });
    let on_pointer_enter = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_enter_count.update(|count| *count += 1)
    });
    let on_pointer_leave = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_leave_count.update(|count| *count += 1)
    });
    let on_key_down = Callback::new(move |key: String| {
        set_last_key_down.set(key);
        false
    });
    let on_key_up = Callback::new(move |key: String| {
        set_last_key_up.set(key);
        false
    });

    let hello_code = Signal::derive(move || {
        r#"<ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_variant_key.get() == "over-background" {
            "ui::ClearButtonVariant::OverBackground"
        } else {
            "ui::ClearButtonVariant::Default"
        };
        let focus_mode = match workbench_focus_mode_key.get().as_str() {
            "prevent" => "ui::ClearButtonFocusMode::Prevent",
            "exclude-tab" => "ui::ClearButtonFocusMode::ExcludeTab",
            _ => "ui::ClearButtonFocusMode::Default",
        };
        format!(
            "<ClearButton\n  variant={variant}\n  inset={}\n  disabled={}\n  focus_mode={focus_mode}\n  slot_name={}\n  aria_label=\"Clear search\".to_string()\n  class_name={}\n  button_type={}\n  node_ref=node_ref\n  on_press=on_press\n  is_visible=Signal::derive(move || {})\n  is_disabled_signal=Signal::derive(move || {})\n  aria_hidden_when_invisible={}\n  on_pointer_down=on_pointer_down\n  on_pointer_up=on_pointer_up\n  on_pointer_cancel=on_pointer_cancel\n  on_pointer_enter=on_pointer_enter\n  on_pointer_leave=on_pointer_leave\n  on_click=on_click\n  on_key_down=on_key_down\n  on_key_up=on_key_up\n  on_blur=on_blur\n>\n  \"×\"\n</ClearButton>",
            workbench_inset.get(),
            workbench_disabled.get(),
            if workbench_custom_slot.get() {
                "\"search-clear\""
            } else {
                "\"clear-button\""
            },
            if workbench_custom_class.get() {
                "\"docs-clear-button-workbench\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_button_type_key.get() == "submit" {
                "\"submit\""
            } else {
                "\"button\""
            },
            workbench_visible_raw.get(),
            workbench_disabled_signal_raw.get(),
            workbench_hide_aria.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = if workbench_variant_key.get() == "over-background" {
            "OverBackground"
        } else {
            "Default"
        };
        let focus_mode = match workbench_focus_mode_key.get().as_str() {
            "prevent" => "Prevent",
            "exclude-tab" => "ExcludeTab",
            _ => "Default",
        };
        let button_type = if workbench_button_type_key.get() == "submit" {
            "submit"
        } else {
            "button"
        };
        let slot_name = if workbench_custom_slot.get() {
            "search-clear"
        } else {
            "clear-button"
        };
        format!(
            "ClearButtonWorkbenchConfig {{\n  variant: {variant},\n  inset: {},\n  disabled: {},\n  focus_mode: {focus_mode},\n  slot_name: \"{slot_name}\",\n  aria_label: Some(\"Clear search\"),\n  class_name: {},\n  button_type: \"{button_type}\",\n  node_ref: Some(\"docs-clear-button-workbench\"),\n  on_press: Some(\"OnPress\"),\n  is_visible: Some({}),\n  is_disabled_signal: Some({}),\n  aria_hidden_when_invisible: {},\n  on_pointer_down: Some(\"Callback<PointerEvent>\"),\n  on_pointer_up: Some(\"Callback<PointerEvent>\"),\n  on_pointer_cancel: Some(\"Callback<PointerEvent>\"),\n  on_pointer_enter: Some(\"Callback<PointerEvent>\"),\n  on_pointer_leave: Some(\"Callback<PointerEvent>\"),\n  on_click: Some(\"Callback<()>\"),\n  on_key_down: Some(\"Callback<String, bool>\"),\n  on_key_up: Some(\"Callback<String, bool>\"),\n  on_blur: Some(\"Callback<()>\"),\n}}",
            workbench_inset.get(),
            workbench_disabled.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-clear-button-workbench\")"
            } else {
                "None"
            },
            workbench_visible_raw.get(),
            workbench_disabled_signal_raw.get(),
            workbench_hide_aria.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ClearButton aria_label="Default clear".to_string()>"×"</ClearButton>
<ClearButton variant=ui::ClearButtonVariant::OverBackground aria_label="Overlay clear".to_string()>"×"</ClearButton>
<ClearButton inset=true focus_mode=ui::ClearButtonFocusMode::Prevent aria_label="Inset prevent".to_string()>"×"</ClearButton>
<ClearButton disabled=true aria_label="Disabled clear".to_string()>"×"</ClearButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ClearButton"
            slug="clear-button"
            group="Actions"
            description="Clear affordance with full pointer/keyboard callback contract."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Toggles every ClearButton API and reports callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="clear-button-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Variant"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_variant_key.set(event_target_value(&ev))>
                                <option value="default" selected=move || workbench_variant_key.get() == "default">"Default"</option>
                                <option value="over-background" selected=move || workbench_variant_key.get() == "over-background">"OverBackground"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Focus mode"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_focus_mode_key.set(event_target_value(&ev))>
                                <option value="default" selected=move || workbench_focus_mode_key.get() == "default">"Default"</option>
                                <option value="prevent" selected=move || workbench_focus_mode_key.get() == "prevent">"Prevent"</option>
                                <option value="exclude-tab" selected=move || workbench_focus_mode_key.get() == "exclude-tab">"ExcludeTab"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Button type"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_button_type_key.set(event_target_value(&ev))>
                                <option value="button" selected=move || workbench_button_type_key.get() == "button">"button"</option>
                                <option value="submit" selected=move || workbench_button_type_key.get() == "submit">"submit"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_inset set_checked=set_workbench_inset>"Inset"</Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_custom_slot set_checked=set_workbench_custom_slot>"Custom slot_name"</Switch>
                        <Switch checked=workbench_visible_raw set_checked=set_workbench_visible_raw>"Visible (is_visible)"</Switch>
                        <Switch checked=workbench_disabled_signal_raw set_checked=set_workbench_disabled_signal_raw>"Disabled signal"</Switch>
                        <Switch checked=workbench_hide_aria set_checked=set_workbench_hide_aria>"aria_hidden_when_invisible"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="clear-button-workbench-preview">
                    <ClearButton
                        variant=if workbench_variant_key.get() == "over-background" {
                            ui::ClearButtonVariant::OverBackground
                        } else {
                            ui::ClearButtonVariant::Default
                        }
                        inset=workbench_inset.get()
                        disabled=workbench_disabled.get()
                        focus_mode=match workbench_focus_mode_key.get().as_str() {
                            "prevent" => ui::ClearButtonFocusMode::Prevent,
                            "exclude-tab" => ui::ClearButtonFocusMode::ExcludeTab,
                            _ => ui::ClearButtonFocusMode::Default,
                        }
                        slot_name=if workbench_custom_slot.get() { "search-clear" } else { "clear-button" }
                        aria_label="Clear search".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-clear-button-workbench".to_string()
                        } else {
                            String::new()
                        }
                        button_type=if workbench_button_type_key.get() == "submit" { "submit" } else { "button" }
                        node_ref=workbench_node_ref
                        on_press=on_press
                        is_visible=workbench_visible
                        is_disabled_signal=workbench_disabled_signal
                        aria_hidden_when_invisible=workbench_hide_aria.get()
                        on_pointer_down=on_pointer_down
                        on_pointer_up=on_pointer_up
                        on_pointer_cancel=on_pointer_cancel
                        on_pointer_enter=on_pointer_enter
                        on_pointer_leave=on_pointer_leave
                        on_click=on_click
                        on_key_down=on_key_down
                        on_key_up=on_key_up
                        on_blur=on_blur
                    >
                        "×"
                    </ClearButton>
                    <span class="ui-muted">
                        "press=" {move || press_count.get()}
                        ", click=" {move || click_count.get()}
                        ", blur=" {move || blur_count.get()}
                    </span>
                    <span class="ui-muted">
                        "pointer: down=" {move || pointer_down_count.get()}
                        ", up=" {move || pointer_up_count.get()}
                        ", cancel=" {move || pointer_cancel_count.get()}
                        ", enter=" {move || pointer_enter_count.get()}
                        ", leave=" {move || pointer_leave_count.get()}
                    </span>
                    <span class="ui-muted">
                        "key: down=" {move || last_key_down.get()}
                        ", up=" {move || last_key_up.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <ClearButton aria_label="Default clear".to_string()>"×"</ClearButton>
                    <ClearButton
                        variant=ui::ClearButtonVariant::OverBackground
                        aria_label="Overlay clear".to_string()
                    >
                        "×"
                    </ClearButton>
                    <ClearButton
                        inset=true
                        focus_mode=ui::ClearButtonFocusMode::Prevent
                        aria_label="Inset prevent".to_string()
                    >
                        "×"
                    </ClearButton>
                    <ClearButton disabled=true aria_label="Disabled clear".to_string()>"×"</ClearButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
