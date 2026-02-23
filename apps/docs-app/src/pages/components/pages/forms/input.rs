use super::*;

pub(crate) fn input() -> AnyView {
    let (value, set_value) = signal(String::new());
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_clearable, set_workbench_clearable) = signal(true);
    let (workbench_label_hidden, set_workbench_label_hidden) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_label_placement_index, set_workbench_label_placement_index) =
        signal(Some(0_usize));
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let workbench_variant_options = vec![
        "bordered".to_string(),
        "flat".to_string(),
        "underlined".to_string(),
    ];
    let workbench_label_placement_options = vec!["outside".to_string(), "inside".to_string()];
    let workbench_node_ref = NodeRef::<leptos::html::Input>::new();
    let input_motion_default = ui::text_input::input::InputMotion::default();

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => InputSize::Sm,
        2 => InputSize::Lg,
        _ => InputSize::Md,
    });
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => InputVariant::Flat,
            2 => InputVariant::Underlined,
            _ => InputVariant::Bordered,
        });
    let workbench_label_placement =
        Signal::derive(
            move || match workbench_label_placement_index.get().unwrap_or(0) {
                1 => ui::text_input::input::InputLabelPlacement::Inside,
                _ => ui::text_input::input::InputLabelPlacement::Outside,
            },
        );
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_class.get() {
            ui::text_input::input::InputMotion {
                hover_scale: 1.02,
                tap_scale: 0.97,
                ..ui::text_input::input::InputMotion::default()
            }
        } else {
            input_motion_default
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<Input
  id="name".to_string()
  value=value
  set_value=set_value
/>"#
        .to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<Input\n  id=\"docs-input-workbench\".to_string()\n  value=value\n  set_value=set_value\n  label=\"Name\".to_string()\n  aria_label=\"Name input\".to_string()\n  start_content=move || view! {{ <span>\"@\"</span> }}\n  end_content=move || view! {{ <span>\".com\"</span> }}\n  disabled={}\n  read_only={}\n  required=Signal::derive(move || {})\n  invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-input-help\".to_string()))\n  description=\"Used for playground API coverage\".to_string()\n  error=\"This field is invalid.\".to_string()\n  placeholder=\"Type something…\".to_string()\n  clear_aria_label=\"Clear input\".to_string()\n  input_type=\"text\"\n  is_clearable={}\n  label_hidden={}\n  label_placement=ui::text_input::input::InputLabelPlacement::{:?}\n  size=InputSize::{:?}\n  variant=InputVariant::{:?}\n  motion=ui::text_input::input::InputMotion::default()\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Input>::new()\n/>",
            workbench_disabled.get(),
            workbench_read_only.get(),
            workbench_required.get(),
            workbench_invalid.get(),
            workbench_clearable.get(),
            workbench_label_hidden.get(),
            workbench_label_placement.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_custom_class.get() {
                "\"docs-input-workbench\".to_string()"
            } else {
                "String::new()"
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
            },
        )
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "InputWorkbenchConfig {{\n  id: \"docs-input-workbench\",\n  value: {:?},\n  set_value: \"WriteSignal<String>\",\n  label: Some(\"Name\"),\n  aria_label: Some(\"Name input\"),\n  start_content: Some(\"prefix\"),\n  end_content: Some(\"suffix\"),\n  disabled: {},\n  read_only: {},\n  required: {},\n  invalid: {},\n  aria_describedby: Some(\"docs-input-help\"),\n  description: Some(\"Used for playground API coverage\"),\n  error: Some(\"This field is invalid.\"),\n  placeholder: Some(\"Type something…\"),\n  clear_aria_label: Some(\"Clear input\"),\n  input_type: Some(\"text\"),\n  is_clearable: {},\n  label_hidden: {},\n  label_placement: \"{:?}\",\n  size: \"{:?}\",\n  variant: \"{:?}\",\n  motion: \"{:?}\",\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"NodeRef<html::Input>\"),\n}}",
            value.get(),
            workbench_disabled.get(),
            workbench_read_only.get(),
            workbench_required.get(),
            workbench_invalid.get(),
            workbench_clearable.get(),
            workbench_label_hidden.get(),
            workbench_label_placement.get(),
            workbench_size.get(),
            workbench_variant.get(),
            workbench_motion.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-input-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(\"rtl\")"
            } else {
                "Some(\"ltr\")"
            },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Input id="m1".to_string() value=value set_value=set_value label="Default".to_string() />
<Input id="m2".to_string() value=value set_value=set_value label="Invalid + Required".to_string() required=Signal::derive(|| true) invalid=Signal::derive(|| true) />
<Input id="m3".to_string() value=value set_value=set_value label="Disabled + Inside".to_string() disabled=true label_placement=ui::text_input::input::InputLabelPlacement::Inside />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Input"
            slug="input"
            group="Forms"
            description="baseline-style text input with label, description/error, and clear button."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <Input
                    id="docs-input-hello".to_string()
                    value=value
                    set_value=set_value
                />
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="input-workbench-controls">
                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input size".to_string()
                            />
                            <div class="docs-search__label">"Variant"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-variant".to_string()
                                options=workbench_variant_options.clone()
                                selected_index=workbench_variant_index
                                set_selected_index=set_workbench_variant_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input variant".to_string()
                            />
                            <div class="docs-search__label">"Label placement"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-label-placement".to_string()
                                options=workbench_label_placement_options.clone()
                                selected_index=workbench_label_placement_index
                                set_selected_index=set_workbench_label_placement_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input label placement".to_string()
                            />
                            <Switch checked=workbench_invalid set_checked=set_workbench_invalid>"Invalid"</Switch>
                            <Switch checked=workbench_required set_checked=set_workbench_required>"Required"</Switch>
                            <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                            <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                            <Switch checked=workbench_clearable set_checked=set_workbench_clearable>"Clearable"</Switch>
                            <Switch checked=workbench_label_hidden set_checked=set_workbench_label_hidden>"Label hidden"</Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="input-workbench-preview">
                    <Input
                        id="docs-input-workbench".to_string()
                        value=value
                        set_value=set_value
                        label="Name".to_string()
                        aria_label="Name input".to_string()
                        start_content=move || view! { <span>"@"</span> }
                        end_content=move || view! { <span>".com"</span> }
                        disabled=workbench_disabled.get()
                        read_only=workbench_read_only.get()
                        required=Signal::derive(move || workbench_required.get())
                        invalid=Signal::derive(move || workbench_invalid.get())
                        aria_describedby=Signal::derive(move || Some("docs-input-help".to_string()))
                        description="Used for playground API coverage".to_string()
                        error="This field is invalid.".to_string()
                        placeholder="Type something…".to_string()
                        clear_aria_label="Clear input".to_string()
                        input_type="text"
                        is_clearable=workbench_clearable.get()
                        label_hidden=workbench_label_hidden.get()
                        label_placement=workbench_label_placement.get()
                        size=workbench_size.get()
                        variant=workbench_variant.get()
                        motion=ui::text_input::input::InputMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-input-workbench".to_string()
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
                    <span id="docs-input-help" class="ui-muted">
                        "value: " {move || value.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Invalid / Disabled)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <Input
                        id="docs-input-matrix-default".to_string()
                        value=value
                        set_value=set_value
                        label="Default".to_string()
                    />
                    <Input
                        id="docs-input-matrix-invalid".to_string()
                        value=value
                        set_value=set_value
                        label="Invalid + Required".to_string()
                        required=Signal::derive(|| true)
                        invalid=Signal::derive(|| true)
                    />
                    <Input
                        id="docs-input-matrix-disabled".to_string()
                        value=value
                        set_value=set_value
                        label="Disabled + Inside".to_string()
                        disabled=true
                        label_placement=ui::text_input::input::InputLabelPlacement::Inside
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
