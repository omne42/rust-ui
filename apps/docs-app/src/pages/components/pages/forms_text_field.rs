use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::{html, prelude::*};
use ui::{TextField, text_field::TextFieldMotion};
use ui_headless::A11yDirection;

pub(super) fn text_field() -> AnyView {
    let (workbench_value, set_workbench_value) = signal("release@omne.rs".to_string());
    let (workbench_last_change, set_workbench_last_change) = signal("release@omne.rs".to_string());
    let on_workbench_value_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next.clone());
        set_workbench_value.set(next);
    });
    let workbench_node_ref: NodeRef<html::Input> = NodeRef::new();

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_dense_motion, set_workbench_dense_motion) = signal(false);
    let (workbench_type_key, set_workbench_type_key) = signal("email".to_string());

    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let hello_code = Signal::derive(move || {
        r#"<TextField
  id="profile-name".to_string()
  label="Name".to_string()
  default_value="Linus".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let input_type = match workbench_type_key.get().as_str() {
            "text" => "text",
            "password" => "password",
            _ => "email",
        };
        format!(
            "<TextField\n  id=\"docs-text-field-workbench\".to_string()\n  label=\"Email\".to_string()\n  value=value\n  default_value=\"release@omne.rs\".to_string()\n  on_value_change=on_value_change\n  is_disabled={}\n  is_read_only={}\n  is_required=Signal::derive(move || {})\n  is_invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-text-field-help\".to_string()))\n  description=\"Used for release notifications\".to_string()\n  error=\"Please provide a valid address\".to_string()\n  placeholder=\"release@omne.rs\".to_string()\n  input_type=\"{}\"\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=node_ref\n/>",
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            input_type,
            if workbench_dense_motion.get() {
                "TextFieldMotion { enabled: true, duration_ms: 120 }"
            } else {
                "TextFieldMotion::default()"
            },
            if workbench_custom_class.get() {
                "\"docs-text-field-workbench\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let input_type = match workbench_type_key.get().as_str() {
            "text" => "text",
            "password" => "password",
            _ => "email",
        };
        format!(
            "TextFieldWorkbenchConfig {{\n  id: \"docs-text-field-workbench\",\n  label: \"Email\",\n  value: {},\n  default_value: Some(\"release@omne.rs\"),\n  on_value_change: Some(\"Callback<String>\"),\n  is_disabled: Some({}),\n  is_read_only: Some({}),\n  is_required: Some({}),\n  is_invalid: Some({}),\n  aria_describedby: Some(\"docs-text-field-help\"),\n  description: Some(\"Used for release notifications\"),\n  error: Some(\"Please provide a valid address\"),\n  placeholder: Some(\"release@omne.rs\"),\n  input_type: Some({}),\n  motion: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"docs-text-field-workbench-input\"),\n}}",
            rust_string_literal(&workbench_value.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            rust_string_literal(input_type),
            if workbench_dense_motion.get() {
                "TextFieldMotion::custom"
            } else {
                "TextFieldMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-text-field-workbench\")"
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
        r#"<TextField id="matrix-default".to_string() label="Default".to_string() default_value="Ready".to_string() />
<TextField
  id="matrix-invalid".to_string()
  label="Invalid + Required".to_string()
  value=value
  on_value_change=on_value_change
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(|| true)
  error="Please check this input".to_string()
/>
<TextField
  id="matrix-disabled".to_string()
  label="Disabled".to_string()
  value=value
  on_value_change=on_value_change
  is_disabled=true
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="TextField"
            slug="text-field"
            group="Forms"
            description="Compact text field with explicit value/state control contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <TextField
                    id="docs-text-field-hello".to_string()
                    label="Name".to_string()
                    default_value="Linus".to_string()
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Tune the full API surface and inspect the exact resolved config."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="text-field-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Input type"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| set_workbench_type_key.set(event_target_value(&ev))
                            >
                                <option value="email" selected=move || workbench_type_key.get() == "email">"email"</option>
                                <option value="text" selected=move || workbench_type_key.get() == "text">"text"</option>
                                <option value="password" selected=move || workbench_type_key.get() == "password">"password"</option>
                            </select>
                        </label>
                        <ui::Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</ui::Switch>
                        <ui::Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</ui::Switch>
                        <ui::Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>"Required"</ui::Switch>
                        <ui::Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>"Invalid"</ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</ui::Switch>
                        <ui::Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</ui::Switch>
                        <ui::Switch checked=workbench_dense_motion set_checked=set_workbench_dense_motion>"Custom motion"</ui::Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="text-field-workbench-preview">
                    <TextField
                        id="docs-text-field-workbench".to_string()
                        label="Email".to_string()
                        value=workbench_value
                        default_value="release@omne.rs".to_string()
                        on_value_change=on_workbench_value_change
                        is_disabled=workbench_disabled.get()
                        is_read_only=workbench_read_only.get()
                        is_required=workbench_required
                        is_invalid=workbench_invalid
                        aria_describedby=Signal::derive(move || Some("docs-text-field-help".to_string()))
                        description="Used for release notifications".to_string()
                        error="Please provide a valid address".to_string()
                        placeholder="release@omne.rs".to_string()
                        input_type=match workbench_type_key.get().as_str() {
                            "text" => "text",
                            "password" => "password",
                            _ => "email",
                        }
                        motion=if workbench_dense_motion.get() {
                            TextFieldMotion {
                                enabled: true,
                                duration_ms: 120,
                            }
                        } else {
                            TextFieldMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-text-field-workbench".to_string()
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
                    <span id="docs-text-field-help" class="ui-muted">
                        "on_value_change: " {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <TextField
                        id="docs-text-field-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="Ready".to_string()
                    />
                    <TextField
                        id="docs-text-field-matrix-invalid".to_string()
                        label="Invalid + Required".to_string()
                        value=workbench_value
                        on_value_change=on_workbench_value_change
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(|| true)
                        error="Please check this input".to_string()
                    />
                    <TextField
                        id="docs-text-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        value=workbench_value
                        on_value_change=on_workbench_value_change
                        is_disabled=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
