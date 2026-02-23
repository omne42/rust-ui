use super::*;

pub(crate) fn disclosure() -> AnyView {
    let (open, set_open) = signal(true);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));
    let (workbench_open, set_workbench_open) = signal(true);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let workbench_on_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(true);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));
<Disclosure
  id_base="disc".to_string()
  label="Details".to_string()
  open=open
  on_open_change=on_open_change
>
  <div>"Hidden content"</div>
</Disclosure>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let custom_motion = workbench_custom_motion.get();
        let default_open = workbench_open.get();

        let mut lines = vec![
            "let (open, set_open) = signal(true);".to_string(),
            "let on_open_change = Callback::new(move |next: bool| set_open.set(next));".to_string(),
            "<Disclosure".to_string(),
            "  id_base=\"docs-disclosure-workbench\".into()".to_string(),
            "  label=\"Workbench details\".into()".to_string(),
        ];

        if controlled {
            lines.push("  open=open".to_string());
            lines.push("  on_open_change=on_open_change".to_string());
        } else {
            lines.push(format!("  default_open={default_open}"));
        }
        if disabled {
            lines.push("  disabled=true".to_string());
        }
        if custom_motion {
            lines.push("  motion=DisclosureMotion { open_rotation_deg: 135.0, panel_offset_y_px: 10.0, ..DisclosureMotion::default() }".to_string());
        }

        lines.extend([
            ">".to_string(),
            "  <div>\"Workbench disclosure content\"</div>".to_string(),
            "</Disclosure>".to_string(),
        ]);

        lines.join("\n")
    });

    let disclosure_test_css_source = Signal::derive(move || {
        format!(
            "/* components/disclosure/src/styles.rs */\n{}",
            ui::disclosure::styles::CSS
        )
    });

    let disclosure_actual_config = Signal::derive(move || {
        let controlled = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let custom_motion = workbench_custom_motion.get();
        let open_value = workbench_open.get();

        format!(
            "DisclosureActualConfig {{\n  open: {open_value},\n  disabled: {disabled},\n  control_mode: \"{}\",\n  default_open_source: \"{}\",\n  motion_source: \"{}\",\n  expected_root_attrs: [\"data-open-control-mode\", \"data-default-open-source\", \"data-motion-source\"],\n}}",
            if controlled {
                "controlled"
            } else {
                "uncontrolled"
            },
            if controlled {
                "implicit-default"
            } else {
                "prop"
            },
            if custom_motion { "custom" } else { "default" },
        )
    });

    let states_code = Signal::derive(move || {
        r#"<Disclosure
  id_base="disc-disabled".to_string()
  label="Disabled details".to_string()
  default_open=false
  disabled=true
>
  <div>"Disabled content"</div>
</Disclosure>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Disclosure"
            slug="disclosure"
            group="Collections"
            description="Single disclosure panel with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground title="Controlled" code_signal=code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure".to_string()
                        label="Details".to_string()
                        open=open.into()
                        on_open_change=on_open_change
                    >
                        <div class="docs-stack">
                            <div>"Hidden content"</div>
                            <div class="ui-muted">"Uses the same open-state contract as overlays."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">
                        "open: "
                        {move || open.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure-disabled".to_string()
                        label="Disabled details".to_string()
                        default_open=false
                        disabled=true
                    >
                        <div class="docs-stack">
                            <div>"Disabled content"</div>
                            <div class="ui-muted">"Disabled disclosure keeps trigger non-interactive."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">"disabled: true"</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=disclosure_test_css_source
                test_source_path="components/disclosure/src/styles.rs".to_string()
                test_config_signal=disclosure_actual_config
                description="Disclosure workbench: 对比展示 + config + code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled mode"
                        </ui::Switch>
                        <ui::Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </ui::Switch>
                        <ui::Switch checked=workbench_open set_checked=set_workbench_open>
                            "Open state (for controlled/default_open)"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let controlled = workbench_controlled.get();
                    let disabled = workbench_disabled.get();
                    let custom_motion = workbench_custom_motion.get();
                    let motion = if custom_motion {
                        ui::DisclosureMotion {
                            open_rotation_deg: 135.0,
                            panel_offset_y_px: 10.0,
                            ..ui::DisclosureMotion::default()
                        }
                    } else {
                        ui::DisclosureMotion::default()
                    };

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <div class="docs-card">
                                    <h4>"Configured Disclosure"</h4>
                                    {if controlled {
                                        view! {
                                            <Disclosure
                                                id_base="docs-disclosure-workbench".to_string()
                                                label="Workbench details".to_string()
                                                open=workbench_open.into()
                                                on_open_change=workbench_on_open_change
                                                disabled=disabled
                                                motion=motion
                                            >
                                                <div class="docs-stack">
                                                    <div>"Configured content"</div>
                                                    <div class="ui-muted">"Tracks controlled/uncontrolled + motion source attrs."</div>
                                                </div>
                                            </Disclosure>
                                        }
                                            .into_any()
                                    } else {
                                        view! {
                                            <Disclosure
                                                id_base="docs-disclosure-workbench".to_string()
                                                label="Workbench details".to_string()
                                                default_open=workbench_open.get()
                                                disabled=disabled
                                                motion=motion
                                            >
                                                <div class="docs-stack">
                                                    <div>"Configured content"</div>
                                                    <div class="ui-muted">"Uncontrolled path uses default_open source marker."</div>
                                                </div>
                                            </Disclosure>
                                        }
                                            .into_any()
                                    }}
                                </div>

                                <div class="docs-card">
                                    <h4>"Reference Disclosure"</h4>
                                    <Disclosure
                                        id_base="docs-disclosure-reference".to_string()
                                        label="Reference details".to_string()
                                        default_open=true
                                    >
                                        <div class="docs-stack">
                                            <div>"Reference content"</div>
                                            <div class="ui-muted">"Baseline uncontrolled + default motion."</div>
                                        </div>
                                    </Disclosure>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
