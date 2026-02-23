use super::*;

pub(crate) fn button_group() -> AnyView {
    let (left_count, set_left_count) = signal(0_usize);
    let (middle_count, set_middle_count) = signal(0_usize);
    let (right_count, set_right_count) = signal(0_usize);
    let on_left: OnPress = Callback::new(move |_| set_left_count.update(|count| *count += 1));
    let on_middle: OnPress = Callback::new(move |_| set_middle_count.update(|count| *count += 1));
    let on_right: OnPress = Callback::new(move |_| set_right_count.update(|count| *count += 1));

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_attached, set_workbench_attached) = signal(true);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_usize);
    let on_workbench_press: OnPress =
        Callback::new(move |_| set_workbench_press_count.update(|count| *count += 1));
    let workbench_node_ref = NodeRef::<html::Div>::new();

    let workbench_orientation = Signal::derive(move || {
        if workbench_vertical.get() {
            ButtonGroupOrientation::Vertical
        } else {
            ButtonGroupOrientation::Horizontal
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::button::ButtonGroupMotion {
                enter_scale: 0.96,
                ..ui::button::ButtonGroupMotion::default()
            }
        } else {
            ui::button::ButtonGroupMotion::default()
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ButtonGroup is_attached=true>
  <Button variant=ButtonVariant::Secondary>"Left"</Button>
  <Button variant=ButtonVariant::Secondary>"Middle"</Button>
  <Button variant=ButtonVariant::Secondary>"Right"</Button>
</ButtonGroup>"#
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<ButtonGroup\n  orientation=ButtonGroupOrientation::{:?}\n  is_attached={}\n  motion={}\n  node_ref=NodeRef::<leptos::html::Div>::new()\n  aria_label={}\n  lang={}\n  dir={}\n  class_name={}\n>\n  <Button on_press=on_press>\"Left\"</Button>\n  <Button on_press=on_press>\"Center\"</Button>\n  <Button on_press=on_press>\"Right\"</Button>\n</ButtonGroup>",
            workbench_orientation.get(),
            workbench_attached.get(),
            if workbench_custom_motion.get() {
                "ButtonGroupMotion { enter_scale: 0.96, ..ButtonGroupMotion::default() }"
            } else {
                "ButtonGroupMotion::default()"
            },
            if workbench_custom_label.get() {
                "\"Action buttons\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                "\"docs-button-group-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ButtonGroupActualConfig {{\n  orientation: {:?},\n  is_attached: {},\n  motion: {},\n  node_ref: \"workbench_node_ref\",\n  aria_label: {:?},\n  lang: {:?},\n  dir: {},\n  class_name: {:?},\n  press_count: {},\n}}",
            workbench_orientation.get(),
            workbench_attached.get(),
            if workbench_custom_motion.get() {
                "ButtonGroupMotion(custom)"
            } else {
                "ButtonGroupMotion::default()"
            },
            if workbench_custom_label.get() {
                Some("Action buttons")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                Some("docs-button-group-custom")
            } else {
                None
            },
            workbench_press_count.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ButtonGroup orientation=ButtonGroupOrientation::Horizontal is_attached=true>
  <Button>"Left"</Button><Button>"Center"</Button><Button>"Right"</Button>
</ButtonGroup>
<ButtonGroup orientation=ButtonGroupOrientation::Vertical is_attached=false aria_label="Doc actions".to_string()>
  <Button>"Top"</Button><Button>"Middle"</Button><Button>"Bottom"</Button>
</ButtonGroup>"#
            .to_string()
    });
    let matrix_node_ref_horizontal = NodeRef::<html::Div>::new();
    let matrix_node_ref_vertical = NodeRef::<html::Div>::new();

    view! {
        <ComponentPage
            title="ButtonGroup"
            slug="button-group"
            group="Actions"
            description="Groups Buttons with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground title="Hello World (Default ButtonGroup)" code_signal=hello_code>
                <div class="docs-stack">
                    <ButtonGroup is_attached=true orientation=ButtonGroupOrientation::Horizontal>
                        <Button variant=ButtonVariant::Secondary on_press=on_left>
                            "Left"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_middle>
                            "Middle"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_right>
                            "Right"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">
                        "left/middle/right clicks: "
                        {move || format!(
                            "{}/{}/{}",
                            left_count.get(),
                            middle_count.get(),
                            right_count.get()
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_attached set_checked=set_workbench_attached>
                            "is_attached"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL + ar"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ButtonGroup
                        orientation=workbench_orientation.get()
                        is_attached=workbench_attached.get()
                        motion=workbench_motion.get()
                        node_ref=workbench_node_ref
                        aria_label=if workbench_custom_label.get() {
                            "Action buttons".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-button-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Left"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Center"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_workbench_press>
                            "Right"
                        </Button>
                    </ButtonGroup>
                    <span class="ui-muted">"workbench presses: " {move || workbench_press_count.get()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix (Orientation + Attachment)" code_signal=matrix_code>
                <div class="docs-stack">
                    <ButtonGroup
                        is_attached=true
                        orientation=ButtonGroupOrientation::Horizontal
                        motion=ui::button::ButtonGroupMotion::default()
                        node_ref=matrix_node_ref_horizontal
                        aria_label="Primary actions".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <Button variant=ButtonVariant::Secondary>"Left"</Button>
                        <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                        <Button variant=ButtonVariant::Secondary>"Right"</Button>
                    </ButtonGroup>
                    <ButtonGroup
                        is_attached=false
                        orientation=ButtonGroupOrientation::Vertical
                        motion=ui::button::ButtonGroupMotion::default()
                        node_ref=matrix_node_ref_vertical
                        aria_label="Document actions".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                        class_name="docs-button-group-custom".to_string()
                    >
                        <Button variant=ButtonVariant::Outline>"Top"</Button>
                        <Button variant=ButtonVariant::Outline is_disabled=true>"Disabled"</Button>
                        <Button variant=ButtonVariant::Outline>"Bottom"</Button>
                    </ButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
