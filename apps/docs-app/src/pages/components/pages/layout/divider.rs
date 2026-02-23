use super::*;

pub(crate) fn divider() -> AnyView {
    let orientations_code = Signal::derive(move || {
        r#"<Divider />
<Divider orientation=DividerOrientation::Vertical class_name="docs-divider-rail".to_string() />"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<Divider class_name="docs-divider-custom".to_string() />
<Divider
  orientation=DividerOrientation::Vertical
  class_name="docs-divider-custom docs-divider-rail".to_string()
/>"#
        .to_string()
    });

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<Divider\n  orientation=DividerOrientation::{}\n  motion=DividerMotion {{ animate_in: {} }}\n  class_name={}\n/>",
            if workbench_vertical.get() {
                "Vertical"
            } else {
                "Horizontal"
            },
            workbench_custom_motion.get(),
            if workbench_custom_class.get() {
                "\"docs-divider-custom docs-divider-rail\".into()"
            } else {
                "\"\".into()"
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/divider/styles.rs */\n{}",
            ui_layout::divider::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            DividerOrientation::Vertical
        } else {
            DividerOrientation::Horizontal
        };
        let custom_motion = workbench_custom_motion.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec!["ui-divider".to_string(), orientation.class_name().into()];
        if custom_class {
            classes.push("docs-divider-custom".to_string());
            if matches!(orientation, DividerOrientation::Vertical) {
                classes.push("docs-divider-rail".to_string());
            }
        }

        format!(
            "DividerActualConfig {{\n  orientation: {orientation:?},\n  aria_orientation: {:?},\n  custom_class: {custom_class},\n  class_name: {},\n  lang: {:?},\n  dir: {:?},\n  custom_motion: {custom_motion},\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            orientation.aria_orientation(),
            if custom_class {
                "\"docs-divider-custom\""
            } else {
                "\"\""
            },
            if matches!(orientation, DividerOrientation::Vertical) {
                "ar"
            } else {
                "en-US"
            },
            if matches!(orientation, DividerOrientation::Vertical) {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
            if custom_motion { "custom" } else { "default" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Divider"
            slug="divider"
            group="Layout"
            description="A separator primitive with centralized orientation state attrs and baseline-style styling markers."
        >
            <Playground title="Orientation" code_signal=orientations_code>
                <div class="docs-stack">
                    <div>"Above"</div>
                    <Divider />
                    <div>"Below"</div>
                    <div class="docs-row">
                        <span>"Left"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-rail".to_string()
                        />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/divider/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区对比 default/workbench；Config 控制 orientation/motion/class，Code 与 CSS Test 用于契约回归。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="divider-config-controls">
                        <button
                            type="button"
                            data-action="toggle-orientation-config"
                            on:click=move |_| {
                                set_workbench_vertical.update(|value| *value = !*value);
                            }
                        >
                            "Toggle orientation"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-class-config"
                            on:click=move |_| {
                                set_workbench_custom_class.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom class"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-motion-config"
                            on:click=move |_| {
                                set_workbench_custom_motion.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom motion"
                        </button>
                        <p class="ui-muted" data-slot="divider-config-summary">
                            {move || {
                                format!(
                                    "config: orientation={} class={} motion={}",
                                    if workbench_vertical.get() {
                                        "vertical"
                                    } else {
                                        "horizontal"
                                    },
                                    if workbench_custom_class.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    },
                                    if workbench_custom_motion.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"default"</span>
                            <span>"Above"</span>
                            <Divider />
                            <span>"Below"</span>
                        </div>

                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"workbench"</span>
                            {move || {
                                if workbench_vertical.get() {
                                    view! {
                                        <div class="docs-row">
                                            <span>"Start"</span>
                                            <Divider
                                                orientation=DividerOrientation::Vertical
                                                motion=if workbench_custom_motion.get() {
                                                    ui_layout::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_layout::divider::motion::DividerMotion::default()
                                                }
                                                class_name=if workbench_custom_class.get() {
                                                    "docs-divider-custom docs-divider-rail".to_string()
                                                } else {
                                                    "".to_string()
                                                }
                                            />
                                            <span>"End"</span>
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="docs-stack docs-stack--tight">
                                            <span>"Start"</span>
                                            <Divider
                                                orientation=DividerOrientation::Horizontal
                                                motion=if workbench_custom_motion.get() {
                                                    ui_layout::divider::motion::DividerMotion {
                                                        animate_in: true,
                                                    }
                                                } else {
                                                    ui_layout::divider::motion::DividerMotion::default()
                                                }
                                                class_name=if workbench_custom_class.get() {
                                                    "docs-divider-custom".to_string()
                                                } else {
                                                    "".to_string()
                                                }
                                            />
                                            <span>"End"</span>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                    <span class="ui-muted" data-slot="divider-workbench-summary">
                        {move || {
                            format!(
                                "orientation={} class={} motion={}",
                                if workbench_vertical.get() {
                                    "vertical"
                                } else {
                                    "horizontal"
                                },
                                if workbench_custom_class.get() {
                                    "custom"
                                } else {
                                    "default"
                                },
                                if workbench_custom_motion.get() {
                                    "custom"
                                } else {
                                    "default"
                                }
                            )
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Orientation / Locale / Motion Comparison)"
                code_signal=custom_class_code
                code_imports="use ui_headless::A11yDirection;\nuse ui_layout::{Divider, DividerMotion, DividerOrientation};".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Divider
                        orientation=DividerOrientation::Horizontal
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <div class="docs-row">
                        <span>"RTL"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            motion=DividerMotion {
                                animate_in: true,
                            }
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                        <span>"Rail"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class Marker" code_signal=custom_class_code>
                <div class="docs-stack">
                    <span>"Custom horizontal divider"</span>
                    <Divider
                        class_name="docs-divider-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <div class="docs-row">
                        <span>"Start"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                        <span>"End"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
