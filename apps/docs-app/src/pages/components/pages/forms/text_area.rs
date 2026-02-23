use super::*;

pub(crate) fn text_area() -> AnyView {
    let (workbench_value, set_workbench_value) = signal("Shipping notes".to_string());
    let (workbench_last_change, set_workbench_last_change) = signal("Shipping notes".to_string());
    let on_workbench_value_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next.clone());
        set_workbench_value.set(next);
    });
    let workbench_node_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rows, set_workbench_rows) = signal(6_u32);

    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let hello_code = Signal::derive(move || {
        r#"<TextArea
  id="release-summary".to_string()
  label="Summary".to_string()
  default_value="Ready for launch".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<TextArea\n  id=\"docs-text-area-workbench\".to_string()\n  label=\"Release notes\".to_string()\n  value=Signal::derive(move || value.get())\n  default_value=\"Shipping notes\".to_string()\n  on_value_change=on_value_change\n  is_disabled={}\n  is_read_only={}\n  is_required=Signal::derive(move || {})\n  is_invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-text-area-help\".to_string()))\n  description=\"Used in release checklist\".to_string()\n  error=\"Release notes are required\".to_string()\n  placeholder=\"Write release notes…\".to_string()\n  rows=Some({})\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=node_ref\n/>",
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "TextAreaMotion::disabled()"
            } else {
                "TextAreaMotion::default()"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-text-area-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\".to_string())"
            } else {
                "Some(\"en\".to_string())"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "TextAreaWorkbenchConfig {{\n  id: \"docs-text-area-workbench\",\n  label: \"Release notes\",\n  value: {},\n  default_value: Some(\"Shipping notes\"),\n  on_value_change: Some(\"Callback<String>\"),\n  is_disabled: Some({}),\n  is_read_only: Some({}),\n  is_required: Some({}),\n  is_invalid: Some({}),\n  aria_describedby: Some(\"docs-text-area-help\"),\n  description: Some(\"Used in release checklist\"),\n  error: Some(\"Release notes are required\"),\n  placeholder: Some(\"Write release notes…\"),\n  rows: Some({}),\n  motion: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"docs-text-area-workbench-textarea\"),\n}}",
            rust_string_literal(&workbench_value.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "TextAreaMotion::disabled"
            } else {
                "TextAreaMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-text-area-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<TextArea id="matrix-default".to_string() label="Default".to_string() default_value="Ready".to_string() />
<TextArea
  id="matrix-invalid".to_string()
  label="Invalid + Required".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(|| true)
  error="Please add details".to_string()
/>
<TextArea
  id="matrix-disabled".to_string()
  label="Disabled".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_disabled=true
  rows=Some(3)
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="TextArea"
            slug="text-area"
            group="Forms"
            description="Multiline field with controlled/uncontrolled value semantics."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <TextArea
                    id="docs-text-area-hello".to_string()
                    label="Summary".to_string()
                    default_value="Ready for launch".to_string()
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Toggle full TextArea API and inspect actual config."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="text-area-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Rows"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| {
                                    let parsed = event_target_value(&ev).parse::<u32>().ok().unwrap_or(6);
                                    set_workbench_rows.set(parsed.max(1));
                                }
                            >
                                <option value="3" selected=move || workbench_rows.get() == 3>"3"</option>
                                <option value="6" selected=move || workbench_rows.get() == 6>"6"</option>
                                <option value="10" selected=move || workbench_rows.get() == 10>"10"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>"Required"</Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>"Invalid"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Motion disabled"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="text-area-workbench-preview">
                    <TextArea
                        id="docs-text-area-workbench".to_string()
                        label="Release notes".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        default_value="Shipping notes".to_string()
                        on_value_change=on_workbench_value_change
                        is_disabled=workbench_disabled.get()
                        is_read_only=workbench_read_only.get()
                        is_required=workbench_required
                        is_invalid=workbench_invalid
                        aria_describedby=Signal::derive(move || Some("docs-text-area-help".to_string()))
                        description="Used in release checklist".to_string()
                        error="Release notes are required".to_string()
                        placeholder="Write release notes…".to_string()
                        rows=workbench_rows.get()
                        motion=if workbench_custom_motion.get() {
                            TextAreaMotion::disabled()
                        } else {
                            TextAreaMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-text-area-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                    />
                    <span id="docs-text-area-help" class="ui-muted">
                        "on_value_change: " {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <TextArea
                        id="docs-text-area-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="Ready".to_string()
                    />
                    <TextArea
                        id="docs-text-area-matrix-invalid".to_string()
                        label="Invalid + Required".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(|| true)
                        error="Please add details".to_string()
                    />
                    <TextArea
                        id="docs-text-area-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_disabled=true
                        rows=3
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
