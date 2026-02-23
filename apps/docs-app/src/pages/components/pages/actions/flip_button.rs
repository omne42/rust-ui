use super::*;

pub(crate) fn flip_button() -> AnyView {
    let code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Top
  front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<FlipButton from=FlipDirection::Top front=... back=... />
<FlipButton from=FlipDirection::Bottom front=... back=... />
<FlipButton from=FlipDirection::Left front=... back=... />
<FlipButton from=FlipDirection::Right front=... back=... />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<FlipButton
  from=FlipDirection::Left
  class_name="docs-flip-button-custom".to_string()
  front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
  back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
/>"#
        .to_string()
    });

    let persisted_workbench_state = load_flip_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let direction_options = vec![
        "top".to_string(),
        "bottom".to_string(),
        "left".to_string(),
        "right".to_string(),
    ];
    let (interactive_direction_index, set_interactive_direction_index) =
        signal(Some(initial_workbench_state.direction_index));
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move || {
        if workbench_persist_state.get() {
            save_flip_button_workbench_state(FlipButtonWorkbenchState {
                direction_index: interactive_direction_index.get().unwrap_or(0).min(3),
            });
        } else {
            clear_flip_button_workbench_state();
        }
    });

    let interactive_direction =
        Signal::derive(
            move || match interactive_direction_index.get().unwrap_or(0) {
                1 => FlipDirection::Bottom,
                2 => FlipDirection::Left,
                3 => FlipDirection::Right,
                _ => FlipDirection::Top,
            },
        );
    let interactive_direction_label =
        Signal::derive(
            move || match interactive_direction_index.get().unwrap_or(0) {
                1 => "Bottom",
                2 => "Left",
                3 => "Right",
                _ => "Top",
            },
        );
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_rtl, set_interactive_rtl) = signal(false);
    let workbench_node_ref = NodeRef::<html::Div>::new();

    let interactive_code = Signal::derive(move || {
        let direction = interactive_direction_label.get();
        format!(
            "<FlipButton\n  from=FlipDirection::{direction}\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Div>::new()\n  front=move || view! {{ <Button variant=ButtonVariant::Secondary>\"Front\"</Button> }}\n  back=move || view! {{ <Button variant=ButtonVariant::Accent>\"Back\"</Button> }}\n/>",
            if interactive_custom_motion.get() {
                "Some(FlipButtonMotion { spring: ui_motion::spring::SpringConfig { stiffness: 340.0, damping: 22.0, mass: 1.0, ..Default::default() } })"
            } else {
                "Some(FlipButtonMotion::default())"
            },
            if interactive_custom_class.get() {
                "\"docs-flip-button-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if interactive_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if interactive_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        format!(
            "FlipButtonWorkbenchConfig {{\n  from: {:?},\n  motion: {},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  front: \"Front\",\n  back: \"Back\",\n  persist_workbench_state: {},\n}}",
            interactive_direction.get(),
            if interactive_custom_motion.get() {
                "Some(FlipButtonMotion(custom))"
            } else {
                "Some(FlipButtonMotion::default())"
            },
            if interactive_custom_class.get() {
                Some("docs-flip-button-custom")
            } else {
                None
            },
            if interactive_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if interactive_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_persist_state.get(),
        )
    });

    view! {
        <ComponentPage
            title="FlipButton"
            slug="flip-button"
            group="Actions"
            description="baseline-level spring flip surface with centralized direction/interaction/class-source state attrs."
        >
            <p class="ui-muted" data-slot="flip-button-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="flip-button-streaming-modes">
                "Streaming: render while the LLM is still generating. Snapshot: render once output is complete."
            </p>
            <p class="ui-muted" data-slot="flip-button-copy-ready-hint">
                "Copy-ready snippets prepend imports automatically; dependency: ui; source: crates/ui/src/button/flip/view.rs."
            </p>

            <Playground title="Hello World (Default FlipButton)" code_signal=code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_config_signal=interactive_actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="flip-button-workbench-controls">
                        <div class="docs-search__label">"Direction"</div>
                        <SegmentedControl
                            id_base="docs-flip-button-direction".to_string()
                            options=direction_options.clone()
                            selected_index=interactive_direction_index
                            set_selected_index=set_interactive_direction_index
                            size=SegmentedControlSize::Sm
                            aria_label="FlipButton direction".to_string()
                        />
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                        <Switch checked=interactive_custom_motion set_checked=set_interactive_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=interactive_custom_class set_checked=set_interactive_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=interactive_rtl set_checked=set_interactive_rtl>"RTL + ar"</Switch>
                    </div>
                }
            >
                {move || {
                    let persist = workbench_persist_state.get();
                    let direction = interactive_direction.get();
                    let direction_label = interactive_direction_label.get();

                    view! {
                        <div class="docs-stack" data-slot="flip-button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                                ", direction: "
                                {direction_label}
                            </span>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="flip-button-workbench-canvas">
                                <div class="docs-row">
                                    <FlipButton
                                        from=direction
                                        motion=if interactive_custom_motion.get() {
                                            let mut motion = FlipButtonMotion::default();
                                            motion.spring.stiffness = 340.0;
                                            motion.spring.damping = 22.0;
                                            motion.spring.mass = 1.0;
                                            motion
                                        } else {
                                            FlipButtonMotion::default()
                                        }
                                        class_name=if interactive_custom_class.get() {
                                            "docs-flip-button-custom".to_string()
                                        } else {
                                            String::new()
                                        }
                                        lang=if interactive_rtl.get() {
                                            "ar".to_string()
                                        } else {
                                            "en-US".to_string()
                                        }
                                        dir=if interactive_rtl.get() {
                                            A11yDirection::Rtl
                                        } else {
                                            A11yDirection::Ltr
                                        }
                                        node_ref=workbench_node_ref
                                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Front"</Button> }
                                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                                    />
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Direction Comparison)" code_signal=states_code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Top
                        motion=FlipButtonMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        node_ref=NodeRef::new()
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Top"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                    <FlipButton
                        from=FlipDirection::Left
                        motion=FlipButtonMotion::default()
                        class_name="docs-flip-button-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                        node_ref=NodeRef::new()
                        front=move || view! { <Button variant=ButtonVariant::Secondary>"Left"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                    />
                </div>
            </Playground>

            <Playground title="Direction Gallery" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipButton
                            from=FlipDirection::Bottom
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Bottom"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Left
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Left"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                        <FlipButton
                            from=FlipDirection::Right
                            front=move || view! { <Button variant=ButtonVariant::Secondary>"Right"</Button> }
                            back=move || view! { <Button variant=ButtonVariant::Accent>"Back"</Button> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class" code_signal=custom_code>
                <div class="docs-row">
                    <FlipButton
                        from=FlipDirection::Left
                        class_name="docs-flip-button-custom".to_string()
                        front=move || view! { <Button variant=ButtonVariant::Outline>"Inspect"</Button> }
                        back=move || view! { <Button variant=ButtonVariant::Accent>"Inspecting"</Button> }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
