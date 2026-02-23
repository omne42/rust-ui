use super::*;

pub(crate) fn theme_toggle_button() -> AnyView {
    let (mode, set_mode) = signal(ThemeMode::Light);

    let mode_options = vec!["Light".to_string(), "Dark".to_string(), "OLED".to_string()];
    let (mode_index, set_mode_index) = signal(Some(0_usize));
    Effect::new(move |_| {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        set_mode.set(mode);
    });

    let (disabled, set_disabled) = signal(false);
    let (two_mode_cycle, set_two_mode_cycle) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let variant_options = vec!["Ghost".to_string(), "Outline".to_string()];
    let icon_size_options = vec!["IconSm".to_string(), "IconLg".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Outline,
        _ => ButtonVariant::Ghost,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(0) {
        1 => ButtonSize::IconLg,
        _ => ButtonSize::IconSm,
    });
    let motion = Signal::derive(move || {
        if custom_motion.get() {
            ThemeToggleMotion {
                rotate_deg: 270.0,
                ..ThemeToggleMotion::default()
            }
        } else {
            ThemeToggleMotion::default()
        }
    });

    let code = Signal::derive(move || {
        let mode = match mode_index.get().unwrap_or(0) {
            1 => ThemeMode::Dark,
            2 => ThemeMode::Oled,
            _ => ThemeMode::Light,
        };
        let disabled = disabled.get();
        let two_mode_cycle = two_mode_cycle.get();
        let custom_aria_label = custom_aria_label.get();
        let custom_class_name = custom_class_name.get();
        let variant = variant.get();
        let size = size.get();
        let motion = motion.get();

        let mut snippet = vec![
            format!("let (mode, set_mode) = signal(ThemeMode::{mode:?});"),
            String::new(),
            "<ThemeToggleButton".to_string(),
            "  mode=mode".to_string(),
            "  set_mode=set_mode".to_string(),
            format!("  variant=ButtonVariant::{variant:?}"),
            format!("  size=ButtonSize::{size:?}"),
            format!("  motion={motion:?}"),
        ];

        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if two_mode_cycle {
            snippet.push("  modes=vec![ThemeMode::Dark, ThemeMode::Light]".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Switch UI mode\".into()".to_string());
        }
        if custom_class_name {
            snippet.push("  class_name=\"docs-theme-toggle-custom\".into()".to_string());
        }

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
    let custom_modes = vec![ThemeMode::Dark, ThemeMode::Light];
    let showcase_code = Signal::derive(move || {
        r#"let (mode, set_mode) = signal(ThemeMode::Light);
<ThemeToggleButton mode=mode set_mode=set_mode />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (custom_mode, set_custom_mode) = signal(ThemeMode::Dark);
let (mode, set_mode) = signal(ThemeMode::System);

<ThemeToggleButton
  mode=custom_mode
  set_mode=set_custom_mode
  modes=vec![ThemeMode::Dark, ThemeMode::Light]
  aria_label="Switch UI mode".to_string()
/>
<ThemeToggleButton mode=mode set_mode=set_mode is_disabled=true />"#
            .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        let available_modes = if two_mode_cycle.get() {
            vec!["Dark", "Light"]
        } else {
            vec!["Light", "Dark", "Oled"]
        };
        format!(
            "ThemeToggleButtonWorkbenchConfig {{\n  mode: {:?},\n  set_mode: {:?},\n  is_disabled: {},\n  custom_aria_label: {},\n  two_mode_cycle: {},\n  modes: {:?},\n  variant: {:?},\n  size: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            mode.get(),
            "write_signal",
            disabled.get(),
            custom_aria_label.get(),
            two_mode_cycle.get(),
            available_modes,
            variant.get(),
            size.get(),
            motion.get(),
            if custom_class_name.get() {
                "docs-theme-toggle-custom"
            } else {
                ""
            },
        )
    });

    view! {
        <ComponentPage
            title="ThemeToggleButton"
            slug="theme-toggle-button"
            group="Actions"
            description="Icon-only theme toggle with baseline-level spring motion and baseline-style mode state attrs."
        >
            <Playground title="Hello World (Default API)" code_signal=showcase_code>
                <div class="docs-row">
                    <ThemeToggleButton mode=mode set_mode=set_mode />
                </div>
            </Playground>

            <Playground
                title="Default cycle"
                code_signal=code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Start mode"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle start mode".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=two_mode_cycle set_checked=set_two_mode_cycle>
                            "Two-mode cycle (dark/light)"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class_name set_checked=set_custom_class_name>
                            "Custom class"
                        </Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle variant".to_string()
                        />
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-theme-toggle-size".to_string()
                            options=icon_size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ThemeToggle size".to_string()
                        />
                    </div>
                }
            >
                {move || {
                    let disabled = disabled.get();
                    let two_mode_cycle = two_mode_cycle.get();
                    let custom_aria_label = custom_aria_label.get();
                    let modes = if two_mode_cycle {
                        vec![ThemeMode::Dark, ThemeMode::Light]
                    } else {
                        vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::Oled]
                    };

                    view! {
                        <div class="docs-row">
                            {if custom_aria_label {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        is_disabled=disabled
                                        modes=modes.clone()
                                        aria_label="Switch UI mode".to_string()
                                        variant=variant.get()
                                        size=size.get()
                                        motion=motion.get()
                                        class_name=if custom_class_name.get() {
                                            "docs-theme-toggle-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ThemeToggleButton
                                        mode=mode
                                        set_mode=set_mode
                                        is_disabled=disabled
                                        modes=modes
                                        variant=variant.get()
                                        size=size.get()
                                        motion=motion.get()
                                        class_name=if custom_class_name.get() {
                                            "docs-theme-toggle-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                    />
                                }
                                    .into_any()
                            }}
                            <span class="ui-muted">"mode: " {move || format!("{:?}", mode.get())}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Custom Modes + Disabled Comparison)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ThemeToggleButton
                            mode=custom_mode
                            set_mode=set_custom_mode
                            modes=custom_modes.clone()
                            aria_label="Switch UI mode".to_string()
                            variant=ButtonVariant::Outline
                            size=ButtonSize::IconLg
                            motion=ThemeToggleMotion {
                                rotate_deg: 270.0,
                                ..ThemeToggleMotion::default()
                            }
                            class_name="docs-theme-toggle-custom".to_string()
                        />
                        <span class="ui-muted">
                            "custom mode: " {move || format!("{:?}", custom_mode.get())}
                        </span>
                    </div>
                    <div class="docs-row">
                        <ThemeToggleButton mode=mode set_mode=set_mode is_disabled=true />
                        <span class="ui-muted">"disabled toggle should remain inert"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
