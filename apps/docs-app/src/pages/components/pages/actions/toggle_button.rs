use super::*;

pub(crate) fn toggle_button() -> AnyView {
    let showcase_node_ref: NodeRef<html::Button> = NodeRef::new();
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    let showcase_code = Signal::derive(move || {
        r#"<ToggleButton
  default_pressed=true
  motion=ToggleButtonMotion {
    hover_scale: 1.06,
    tap_scale: 0.95,
    ..ToggleButtonMotion::default()
  }
  class_name="docs-toggle-button-custom".to_string()
  aria_label="Mute notifications".to_string()
  node_ref=NodeRef::new()
>
  "Mute"
</ToggleButton>"#
            .to_string()
    });

    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let motion_options = vec!["default".to_string(), "custom".to_string()];

    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(2_usize));
    let (motion_index, set_motion_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (default_pressed, set_default_pressed) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ToggleButtonMotion {
            hover_scale: 1.06,
            tap_scale: 0.95,
            ..ToggleButtonMotion::default()
        },
        _ => ToggleButtonMotion::default(),
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-toggle-button-custom".to_string()
        } else {
            String::new()
        }
    });
    let aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Toggle docs sample".to_string()
        } else {
            String::new()
        }
    });

    let (selected, set_selected) = signal(false);
    let selected_signal: Signal<bool> = Signal::derive(move || selected.get());
    let (last_change, set_last_change) = signal("none".to_string());
    let on_toggle_change = Callback::new(move |next: bool| {
        set_selected.set(next);
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let workbench_code = Signal::derive(move || {
        let lines = vec![
            "let (selected, set_selected) = signal(false);".to_string(),
            "let selected_signal: Signal<bool> = Signal::derive(move || selected.get());"
                .to_string(),
            "let on_toggle_change = Callback::new(move |next| set_selected.set(next));".to_string(),
            String::new(),
            "<ToggleButton".to_string(),
            "  is_pressed=selected_signal".to_string(),
            format!("  default_pressed={}", default_pressed.get()),
            format!("  is_disabled={}", disabled.get()),
            format!("  variant=ToggleButtonVariant::{:?}", variant.get()),
            format!("  size=ToggleButtonSize::{:?}", size.get()),
            format!("  motion={:?}", motion.get()),
            "  on_pressed_change=on_toggle_change".to_string(),
            format!("  class_name={:?}", class_name.get()),
            format!("  aria_label={:?}", aria_label.get()),
            "  node_ref=NodeRef::new()".to_string(),
            ">".to_string(),
            "  \"Toggle\"".to_string(),
            "</ToggleButton>".to_string(),
        ];
        lines.join("\n")
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToggleButtonWorkbenchConfig {{\n  is_pressed: {},\n  default_pressed: {:?},\n  is_disabled: {},\n  variant: {:?},\n  size: {:?},\n  motion: {:?},\n  on_pressed_change: {:?},\n  class_name: {:?},\n  aria_label: {:?},\n  node_ref: \"bound\",\n}}",
            selected.get(),
            Some(default_pressed.get()),
            disabled.get(),
            variant.get(),
            size.get(),
            motion.get(),
            last_change.get(),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            if aria_label.get().is_empty() {
                None::<String>
            } else {
                Some(aria_label.get())
            },
        )
    });

    let (notifications, set_notifications) = signal(true);
    let (disabled_selected, set_disabled_selected) = signal(true);
    let (disabled_unselected, set_disabled_unselected) = signal(false);
    let notifications_signal: Signal<bool> = Signal::derive(move || notifications.get());
    let disabled_selected_signal: Signal<bool> = Signal::derive(move || disabled_selected.get());
    let disabled_unselected_signal: Signal<bool> =
        Signal::derive(move || disabled_unselected.get());
    let on_notifications_change = Callback::new(move |next: bool| set_notifications.set(next));
    let on_disabled_selected_change =
        Callback::new(move |next: bool| set_disabled_selected.set(next));
    let on_disabled_unselected_change =
        Callback::new(move |next: bool| set_disabled_unselected.set(next));
    let states_code = Signal::derive(move || {
        r#"<ToggleButton
  is_pressed=notifications_signal
  on_pressed_change=on_notifications_change
  variant=ToggleButtonVariant::Accent
  size=ToggleButtonSize::L
>
  "Notifications"
</ToggleButton>
<ToggleButton is_pressed=disabled_selected_signal on_pressed_change=on_disabled_selected_change is_disabled=true>
  "Disabled on"
</ToggleButton>
<ToggleButton is_pressed=disabled_unselected_signal on_pressed_change=on_disabled_unselected_change is_disabled=true>
  "Disabled off"
</ToggleButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleButton"
            slug="toggle-button"
            group="Actions"
            description="Pressable toggle state with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
            >
                <div class="docs-row">
                    <ToggleButton
                        default_pressed=true
                        motion=ToggleButtonMotion {
                            hover_scale: 1.06,
                            tap_scale: 0.95,
                            ..ToggleButtonMotion::default()
                        }
                        class_name="docs-toggle-button-custom".to_string()
                        aria_label="Mute notifications".to_string()
                        node_ref=showcase_node_ref
                    >
                        "Mute"
                    </ToggleButton>
                </div>
            </Playground>

            <Playground
                title="Workbench (Controlled + on_pressed_change)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton size".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButton motion".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=default_pressed set_checked=set_default_pressed>
                            "Default pressed"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>"Custom aria label"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();
                    let disabled = disabled.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <ToggleButton
                                    is_pressed=selected_signal
                                    default_pressed=default_pressed.get()
                                    motion=motion.get()
                                    on_pressed_change=on_toggle_change
                                    variant=variant
                                    size=size
                                    is_disabled=disabled
                                    class_name=class_name.get()
                                    aria_label=aria_label.get()
                                    node_ref=workbench_node_ref
                                >
                                    "Toggle"
                                </ToggleButton>
                                <span class="ui-muted">
                                    "selected: "
                                    {move || selected.get()}
                                </span>
                            </div>
                            <span class="ui-muted">"last on_pressed_change: " {move || last_change.get()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Variant + Size + Disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ToggleButton
                            is_pressed=notifications_signal
                            on_pressed_change=on_notifications_change
                            variant=ToggleButtonVariant::Accent
                            size=ToggleButtonSize::L
                        >
                            "Notifications"
                        </ToggleButton>
                        <span class="ui-muted">
                            "notifications: "
                            {move || notifications.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ToggleButton
                            is_pressed=disabled_selected_signal
                            on_pressed_change=on_disabled_selected_change
                            is_disabled=true
                        >
                            "Disabled on"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=disabled_unselected_signal
                            on_pressed_change=on_disabled_unselected_change
                            is_disabled=true
                        >
                            "Disabled off"
                        </ToggleButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
