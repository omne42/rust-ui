use super::*;

pub(crate) fn logic_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let variant_options = vec!["AND".to_string(), "OR".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let button_type_options = vec![
        "button".to_string(),
        "submit".to_string(),
        "reset".to_string(),
    ];
    let (button_type_index, set_button_type_index) = signal(Some(0_usize));
    let button_type = Signal::derive(move || match button_type_index.get().unwrap_or(0) {
        1 => "submit",
        2 => "reset",
        _ => "button",
    });
    let (press_count, set_press_count) = signal(0_usize);

    let variant = Signal::derive(move || {
        if variant_index.get().unwrap_or(0) == 1 {
            LogicButtonVariant::Or
        } else {
            LogicButtonVariant::And
        }
    });

    let motion = Signal::derive(move || {
        if custom_motion.get() {
            LogicButtonMotion {
                transition_ms: 240,
                press_scale_pct: 92,
            }
        } else {
            LogicButtonMotion::default()
        }
    });

    let on_press = Callback::new(move |_| {
        set_press_count.update(|next| *next += 1);
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<LogicButton".to_string()];
        lines.push(format!("  variant=LogicButtonVariant::{:?}", variant.get()));
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if custom_motion.get() {
            lines.push(
                "  motion=LogicButtonMotion { transition_ms: 240, press_scale_pct: 92 }"
                    .to_string(),
            );
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-logic-button-custom\".into()".to_string());
        }
        if custom_aria.get() {
            lines.push("  aria_label=\"Logic operator\".into()".to_string());
        }
        if button_type.get() != "button" {
            lines.push(format!("  button_type={:?}", button_type.get()));
        }
        lines.push("  node_ref=NodeRef::new()".to_string());
        lines.push("  on_press=Callback::new(move |_| {})".to_string());
        lines.push(">".to_string());
        lines.push("  \"Logic\"".to_string());
        lines.push("</LogicButton>".to_string());
        lines.join("\n")
    });

    let comparison_code = Signal::derive(move || {
        r#"<LogicButton variant=LogicButtonVariant::And>"AND"</LogicButton>
<LogicButton variant=LogicButtonVariant::Or>"OR"</LogicButton>
<LogicButton variant=LogicButtonVariant::And class_name="docs-logic-button-custom".to_string()>"Custom"</LogicButton>
<LogicButton variant=LogicButtonVariant::Or disabled=true>"Disabled"</LogicButton>"#.to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/logic_button/styles.rs */\n{}",
            ui::logic_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "LogicButtonWorkbenchConfig {{\n  variant: \"{:?}\",\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: \"bound\",\n  on_press: {:?},\n  custom_motion: {},\n  press_count: {},\n}}",
            variant.get(),
            disabled.get(),
            if custom_aria.get() {
                "Logic operator"
            } else {
                ""
            },
            if custom_class.get() {
                "docs-logic-button-custom"
            } else {
                ""
            },
            button_type.get(),
            "handler",
            custom_motion.get(),
            press_count.get()
        )
    });

    view! {
        <ComponentPage
            title="LogicButton"
            slug="logic-button"
            group="Actions"
            description="baseline-style boolean operator button with centralized variant normalization, headless press/hover/focus behavior, and stable state/source data contracts."
        >
            <Playground title="Hello World (Default API)" code_signal=comparison_code>
                <div class="docs-row">
                    <LogicButton on_press=on_press>"AND"</LogicButton>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/logic_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-logic-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="LogicButton variant".to_string()
                        />
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria label"
                        </Switch>
                        <div class="docs-search__label">"Button type"</div>
                        <SegmentedControl
                            id_base="docs-logic-button-button-type".to_string()
                            options=button_type_options.clone()
                            selected_index=button_type_index
                            set_selected_index=set_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="LogicButton button type".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <LogicButton
                        variant=variant.get()
                        disabled=disabled.get()
                        motion=motion.get()
                        aria_label=if custom_aria.get() {
                            "Logic operator".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if custom_class.get() {
                            "docs-logic-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        button_type=button_type.get()
                        node_ref=workbench_node_ref
                        on_press=on_press
                    >
                        {if variant.get() == LogicButtonVariant::And {
                            "AND"
                        } else {
                            "OR"
                        }}
                    </LogicButton>
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (AND / OR / Custom / Disabled)" code_signal=comparison_code>
                <div class="docs-row">
                    <LogicButton variant=LogicButtonVariant::And>"AND"</LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or>"OR"</LogicButton>
                    <LogicButton
                        variant=LogicButtonVariant::And
                        class_name="docs-logic-button-custom".to_string()
                    >
                        "Custom"
                    </LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or disabled=true>"Disabled"</LogicButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
