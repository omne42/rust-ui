use super::*;

pub(crate) fn auto_height() -> AnyView {
    let (animated_open, set_animated_open) = signal(false);
    let (static_open, set_static_open) = signal(false);
    let (workbench_open, set_workbench_open) = signal(true);
    let (workbench_animate, set_workbench_animate) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let animated_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
<Button on_press=...>"Toggle"</Button>
<AutoHeight class_name="docs-auto-height".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#
            .to_string()
    });

    let static_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let motion = AutoHeightMotion {
  animate_height: false,
  ..AutoHeightMotion::default()
};
<AutoHeight motion=motion class_name="docs-auto-height docs-auto-height--static-demo".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let animate = workbench_animate.get();
        let custom_class = workbench_custom_class.get();
        let mut lines = vec!["let (open, set_open) = signal(true);".to_string()];
        lines.push("<AutoHeight".to_string());
        if !animate {
            lines.push("  motion=AutoHeightMotion { animate_height: false, ..AutoHeightMotion::default() }".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-auto-height-workbench\".into()".to_string());
        }
        lines.extend([
            ">".to_string(),
            "  <Show when=move || open.get()>".to_string(),
            "    <div>\"Workbench content\"</div>".to_string(),
            "  </Show>".to_string(),
            "</AutoHeight>".to_string(),
        ]);
        lines.join("\n")
    });

    let auto_height_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/auto_height/styles.rs */\n{}",
            ui_layout::auto_height::styles::CSS
        )
    });

    let auto_height_actual_config = Signal::derive(move || {
        let animate_height = workbench_animate.get();
        let custom_class = workbench_custom_class.get();
        let is_open = workbench_open.get();
        let mut classes = vec!["ui-auto-height".to_string()];

        if animate_height {
            classes.push("ui-auto-height--animated".to_string());
        } else {
            classes.push("ui-auto-height--static".to_string());
            classes.push("ui-auto-height--custom-motion".to_string());
        }

        if custom_class {
            classes.push("ui-auto-height--custom-class".to_string());
            classes.push("docs-auto-height-workbench".to_string());
        }

        format!(
            "AutoHeightActualConfig {{\n  open: {is_open},\n  animate_height: {animate_height},\n  custom_class: {custom_class},\n  motion: AutoHeightMotion {{ animate_height: {animate_height}, ..Default::default() }},\n  class_name: {},\n  data_state: \"{}\",\n  data_motion_source: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "\"docs-auto-height-workbench\""
            } else {
                "\"\""
            },
            if animate_height { "animated" } else { "static" },
            if animate_height { "default" } else { "custom" },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="AutoHeight"
            slug="auto-height"
            group="Layout"
            description="Animates (or snaps) height changes via spring-driven CSS variables with centralized motion/class state attrs."
        >
            <Playground title="Animated Height" code_signal=animated_code>
                <div class="docs-stack">
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_animated_open.update(|v| *v = !*v))
                    >
                        {move || if animated_open.get() { "Collapse" } else { "Expand" }}
                    </ui::Button>

                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <Show when=move || animated_open.get()>
                            <div class="docs-stack">
                                <div>"AutoHeight content"</div>
                                <div class="ui-muted">"ResizeObserver + ui-motion spring."</div>
                                <div class="ui-muted">"Toggle quickly to verify stable interpolation."</div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=auto_height_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/auto_height/styles.rs".to_string()
                test_config_signal=auto_height_actual_config
                description="AutoHeight workbench: 展示区 + config 快照 + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open content"
                        </ui::Switch>
                        <ui::Switch checked=workbench_animate set_checked=set_workbench_animate>
                            "Animate height"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let animate_height = workbench_animate.get();
                    let custom_class = workbench_custom_class.get();
                    let is_open = workbench_open.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Configured AutoHeight"</h3>
                                    </Header>
                                    <AutoHeight
                                        motion=AutoHeightMotion {
                                            animate_height,
                                            ..AutoHeightMotion::default()
                                        }
                                        class_name=if custom_class {
                                            "docs-auto-height-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    >
                                        <Show when=move || is_open>
                                            <div class="docs-stack">
                                                <div>"Configured workbench content"</div>
                                                <div class="ui-muted">"Toggle open/animate/class to compare state markers."</div>
                                            </div>
                                        </Show>
                                    </AutoHeight>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Reference AutoHeight"</h3>
                                    </Header>
                                    <AutoHeight class_name="docs-auto-height".to_string()>
                                        <div class="docs-stack">
                                            <div>"Reference content (always shown)"</div>
                                            <div class="ui-muted">"Baseline animated/default contract."</div>
                                        </div>
                                    </AutoHeight>
                                </View>
                            </div>

                            <div class="ui-muted">
                                {format!(
                                    "comparison: configured(open={}, animate_height={}, custom_class={}) vs reference(default)",
                                    is_open,
                                    animate_height,
                                    custom_class,
                                )}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Animated vs Static + Class)"
                code_signal=static_code
            >
                <div class="docs-row">
                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <div class="docs-stack">
                            <div>"Animated default"</div>
                            <div class="ui-muted">"animate_height=true"</div>
                        </div>
                    </AutoHeight>
                    <AutoHeight
                        motion=AutoHeightMotion {
                            animate_height: false,
                            ..AutoHeightMotion::default()
                        }
                        class_name="docs-auto-height docs-auto-height--static-demo".to_string()
                    >
                        <div class="docs-stack">
                            <div>"Static custom motion"</div>
                            <div class="ui-muted">"animate_height=false + custom class"</div>
                        </div>
                    </AutoHeight>
                </div>
            </Playground>

            <Playground title="Static Motion + Custom Class" code_signal=static_code>
                <div class="docs-stack">
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_static_open.update(|v| *v = !*v))
                    >
                        {move || if static_open.get() { "Hide Static" } else { "Show Static" }}
                    </ui::Button>

                    <AutoHeight
                        motion=AutoHeightMotion {
                            animate_height: false,
                            ..AutoHeightMotion::default()
                        }
                        class_name="docs-auto-height docs-auto-height--static-demo".to_string()
                    >
                        <Show when=move || static_open.get()>
                            <div class="docs-stack">
                                <div>"Static mode content"</div>
                                <div class="ui-muted">"Uses custom motion contract (`animate_height=false`)."</div>
                                <div class="ui-muted">"Useful for reduced-motion or deterministic layout jumps."</div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
