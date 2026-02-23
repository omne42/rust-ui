use super::*;

pub(crate) fn toggle() -> AnyView {
    let (showcase_pressed, set_showcase_pressed) = signal(false);
    let showcase_pressed_signal: Signal<bool> = Signal::derive(move || showcase_pressed.get());
    let (showcase_change_runs, set_showcase_change_runs) = signal(0_u32);
    let on_showcase_pressed_change = Callback::new(move |next: bool| {
        set_showcase_pressed.set(next);
        set_showcase_change_runs.update(|count| *count += 1);
    });

    let variant_options = vec![
        "Default".to_string(),
        "Outline".to_string(),
        "Ghost".to_string(),
    ];
    let size_options = vec!["S".to_string(), "M".to_string(), "L".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_default_pressed, set_workbench_default_pressed) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ToggleVariant::Outline,
            2 => ToggleVariant::Ghost,
            _ => ToggleVariant::Default,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ToggleSize::S,
        2 => ToggleSize::L,
        _ => ToggleSize::M,
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ToggleMotion {
                tap_scale: 0.92,
                ..ToggleMotion::default()
            }
        } else {
            ToggleMotion::default()
        }
    });
    let (workbench_pressed, set_workbench_pressed) = signal(false);
    let workbench_pressed_signal: Signal<bool> = Signal::derive(move || workbench_pressed.get());
    let (workbench_change_runs, set_workbench_change_runs) = signal(0_u32);
    let on_workbench_pressed_change = Callback::new(move |next: bool| {
        set_workbench_pressed.set(next);
        set_workbench_change_runs.update(|count| *count += 1);
    });
    let workbench_node_ref = NodeRef::new();

    let hello_code = Signal::derive(move || {
        r#"<Toggle
  default_pressed=false
  aria_label="Toggle bold".to_string()
  on_pressed_change=on_pressed_change
>
  "Bold"
</Toggle>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = match workbench_variant.get() {
            ToggleVariant::Outline => "ToggleVariant::Outline",
            ToggleVariant::Ghost => "ToggleVariant::Ghost",
            _ => "ToggleVariant::Default",
        };
        let size = match workbench_size.get() {
            ToggleSize::S => "ToggleSize::S",
            ToggleSize::L => "ToggleSize::L",
            _ => "ToggleSize::M",
        };
        let motion = if workbench_custom_motion.get() {
            "ToggleMotion { tap_scale: 0.92, ..ToggleMotion::default() }"
        } else {
            "ToggleMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-toggle-workbench"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Toggle workbench"
        } else {
            ""
        };

        [
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            format!(
                "  default_pressed={}",
                bool_word(workbench_default_pressed.get())
            ),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  variant={variant}"),
            format!("  size={size}"),
            format!("  motion={motion}"),
            "  on_pressed_change=on_pressed_change".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            "  node_ref=node_ref".to_string(),
            ">".to_string(),
            "  \"Format\"".to_string(),
            "</Toggle>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-toggle-workbench")
        } else {
            Some("")
        };
        let aria_label = if workbench_custom_aria.get() {
            Some("Toggle workbench")
        } else {
            Some("")
        };

        format!(
            "ToggleActualConfig {{\n  is_pressed: Some(Signal<bool>({})),\n  default_pressed: Some({}),\n  is_disabled: {},\n  variant: {:?},\n  size: {:?},\n  motion: {},\n  on_pressed_change: \"runs={}\",\n  class_name: {class_name:?},\n  aria_label: {aria_label:?},\n  node_ref: Some(\"toggle_node_ref\"),\n}}",
            workbench_pressed.get(),
            bool_word(workbench_default_pressed.get()),
            bool_word(workbench_is_disabled.get()),
            workbench_variant.get(),
            workbench_size.get(),
            if workbench_custom_motion.get() {
                "ToggleMotion::custom"
            } else {
                "ToggleMotion::default"
            },
            workbench_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Toggle default_pressed=false aria_label="Default".to_string()>"Default"</Toggle>
<Toggle default_pressed=true variant=ToggleVariant::Outline size=ToggleSize::S aria_label="Outline pressed".to_string()>"Outline"</Toggle>
<Toggle is_disabled=true variant=ToggleVariant::Ghost size=ToggleSize::L aria_label="Disabled ghost".to_string()>"Disabled"</Toggle>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Toggle"
            slug="toggle"
            group="Actions"
            description="baseline-compatible single toggle primitive with baseline-style press/focus contracts and baseline-level spring press motion."
        >
            <Playground title="Hello World (Default Toggle)" code_signal=hello_code>
                <div class="docs-row">
                    <Toggle
                        default_pressed=false
                        aria_label="Toggle bold".to_string()
                        is_pressed=showcase_pressed_signal
                        on_pressed_change=on_showcase_pressed_change
                    >
                        "Bold"
                    </Toggle>
                    <span class="ui-muted">
                        "pressed: " {move || showcase_pressed.get()}
                        " · on_pressed_change: " {move || showcase_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toggle-workbench-controls">
                        <SegmentedControl
                            id_base="docs-toggle-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Toggle variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-workbench-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Toggle size".to_string()
                        />
                        <Switch checked=workbench_default_pressed set_checked=set_workbench_default_pressed>
                            "default_pressed"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-row">
                    <Toggle
                        is_pressed=workbench_pressed_signal
                        default_pressed=workbench_default_pressed.get()
                        is_disabled=workbench_is_disabled.get()
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        motion=workbench_motion.get()
                        on_pressed_change=on_workbench_pressed_change
                        class_name=if workbench_custom_class.get() {
                            "docs-toggle-workbench".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Toggle workbench".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                    >
                        "Format"
                    </Toggle>
                    <span class="ui-muted">
                        "pressed: " {move || workbench_pressed.get()}
                        " · on_pressed_change: " {move || workbench_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Outline / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <Toggle default_pressed=false aria_label="Default".to_string()>
                        "Default"
                    </Toggle>
                    <Toggle
                        default_pressed=true
                        variant=ToggleVariant::Outline
                        size=ToggleSize::S
                        aria_label="Outline pressed".to_string()
                    >
                        "Outline"
                    </Toggle>
                    <Toggle
                        is_disabled=true
                        variant=ToggleVariant::Ghost
                        size=ToggleSize::L
                        aria_label="Disabled ghost".to_string()
                    >
                        "Disabled"
                    </Toggle>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
