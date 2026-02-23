use super::*;

pub(crate) fn switch_group() -> AnyView {
    let (showcase_marketing, set_showcase_marketing) = signal(true);
    let (showcase_product_updates, set_showcase_product_updates) = signal(false);
    let (showcase_security_alerts, set_showcase_security_alerts) = signal(true);

    let tone_options = vec!["Default".to_string(), "Muted".to_string()];
    let orientation_options = vec!["Vertical".to_string(), "Horizontal".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let workbench_tone = Signal::derive(move || {
        if workbench_tone_index.get().unwrap_or(0) == 1 {
            SwitchGroupTone::Muted
        } else {
            SwitchGroupTone::Default
        }
    });
    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_index.get().unwrap_or(0) == 1 {
            SwitchGroupOrientation::Horizontal
        } else {
            SwitchGroupOrientation::Vertical
        }
    });
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let (workbench_marketing, set_workbench_marketing) = signal(true);
    let (workbench_product_updates, set_workbench_product_updates) = signal(false);
    let (workbench_security_alerts, set_workbench_security_alerts) = signal(true);

    let (matrix_critical_alerts, set_matrix_critical_alerts) = signal(true);
    let (matrix_maintenance_mode, set_matrix_maintenance_mode) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<SwitchGroup
  id_base="notifications".to_string()
  label="Notification channels".to_string()
  description="Choose which channels we can use to contact you.".to_string()
  required=true
>
  <Switch checked=marketing set_checked=set_marketing>"Marketing email"</Switch>
</SwitchGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<SwitchGroup\n  id_base=\"docs-switch-group-workbench\".to_string()\n  label=\"Notification channels\".to_string()\n  description=\"Choose which channels we can use to contact you.\".to_string()\n  error_message={}.to_string()\n  orientation={:?}\n  tone={:?}\n  required={}\n  disabled={}\n  invalid={}\n  aria_label=\"Notification switches\".to_string()\n  class_name={}\n>\n  <Switch checked=marketing set_checked=set_marketing>\"Marketing email\"</Switch>\n  <Switch checked=product_updates set_checked=set_product_updates>\"Product updates\"</Switch>\n  <Switch checked=security_alerts set_checked=set_security_alerts>\"Security alerts\"</Switch>\n</SwitchGroup>",
            rust_string_literal(if workbench_invalid.get() {
                "At least one critical channel must stay enabled."
            } else {
                ""
            }),
            workbench_orientation.get(),
            workbench_tone.get(),
            bool_word(workbench_required.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_invalid.get()),
            if workbench_custom_class.get() {
                "\"docs-switch-group-custom\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SwitchGroupWorkbenchActualConfig {{\n  id_base: \"docs-switch-group-workbench\",\n  label: Some(\"Notification channels\"),\n  description: Some(\"Choose which channels we can use to contact you.\"),\n  error_message: {:?},\n  orientation: {:?},\n  tone: {:?},\n  required: {},\n  disabled: {},\n  invalid: {},\n  aria_label: Some(\"Notification switches\"),\n  class_name: {:?},\n}}",
            if workbench_invalid.get() {
                Some("At least one critical channel must stay enabled.")
            } else {
                None
            },
            workbench_orientation.get(),
            workbench_tone.get(),
            bool_word(workbench_required.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_invalid.get()),
            if workbench_custom_class.get() {
                Some("docs-switch-group-custom")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SwitchGroup id_base="sg-default".to_string() label="Default".to_string() required=true />
<SwitchGroup id_base="sg-horizontal".to_string() label="Horizontal".to_string() orientation=SwitchGroupOrientation::Horizontal tone=SwitchGroupTone::Muted />
<SwitchGroup id_base="sg-disabled".to_string() label="Disabled".to_string() invalid=true disabled=true error_message="At least one critical channel must stay enabled.".to_string() class_name="docs-switch-group-custom".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SwitchGroup"
            slug="switch-group"
            group="Forms"
            description="baseline-style switch grouping primitive with centralized orientation/tone/validation/message-state contracts and stable data markers."
        >
            <Playground title="Hello World (Default SwitchGroup)" code_signal=hello_code>
                <div class="docs-stack">
                    <SwitchGroup
                        id_base="docs-switch-group-hello".to_string()
                        label="Notification channels".to_string()
                        description="Choose which channels we can use to contact you.".to_string()
                        required=true
                        aria_label="Notification switches".to_string()
                    >
                        <Switch checked=showcase_marketing set_checked=set_showcase_marketing>
                            "Marketing email"
                        </Switch>
                        <Switch checked=showcase_product_updates set_checked=set_showcase_product_updates>
                            "Product updates"
                        </Switch>
                        <Switch checked=showcase_security_alerts set_checked=set_showcase_security_alerts>
                            "Security alerts"
                        </Switch>
                    </SwitchGroup>
                    <span class="ui-muted">
                        "marketing="
                        {move || showcase_marketing.get()}
                        " · updates="
                        {move || showcase_product_updates.get()}
                        " · security="
                        {move || showcase_security_alerts.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="switch-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-switch-group-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="SwitchGroup orientation".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-switch-group-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="SwitchGroup tone".to_string()
                        />
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "invalid"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <SwitchGroup
                        id_base="docs-switch-group-workbench".to_string()
                        label="Notification channels".to_string()
                        description="Choose which channels we can use to contact you.".to_string()
                        error_message=if workbench_invalid.get() {
                            "At least one critical channel must stay enabled.".to_string()
                        } else {
                            String::new()
                        }
                        orientation=workbench_orientation.get()
                        tone=workbench_tone.get()
                        required=workbench_required.get()
                        disabled=workbench_disabled.get()
                        invalid=workbench_invalid.get()
                        aria_label="Notification switches".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-switch-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <Switch checked=workbench_marketing set_checked=set_workbench_marketing disabled=workbench_disabled.get()>
                            "Critical alerts"
                        </Switch>
                        <Switch checked=workbench_product_updates set_checked=set_workbench_product_updates disabled=workbench_disabled.get()>
                            "Maintenance mode"
                        </Switch>
                        <Switch checked=workbench_security_alerts set_checked=set_workbench_security_alerts disabled=workbench_disabled.get()>
                            "Security alerts"
                        </Switch>
                    </SwitchGroup>
                    <span class="ui-muted">
                        "marketing="
                        {move || workbench_marketing.get()}
                        " · updates="
                        {move || workbench_product_updates.get()}
                        " · security="
                        {move || workbench_security_alerts.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Horizontal / Disabled)" code_signal=matrix_code>
                <div class="docs-stack">
                    <SwitchGroup
                        id_base="docs-switch-group-matrix-default".to_string()
                        label="Default".to_string()
                        required=true
                    >
                        <Switch checked=showcase_marketing set_checked=set_showcase_marketing>
                            "Marketing email"
                        </Switch>
                    </SwitchGroup>
                    <SwitchGroup
                        id_base="docs-switch-group-matrix-horizontal".to_string()
                        label="Horizontal".to_string()
                        orientation=SwitchGroupOrientation::Horizontal
                        tone=SwitchGroupTone::Muted
                    >
                        <Switch checked=matrix_critical_alerts set_checked=set_matrix_critical_alerts>
                            "Critical alerts"
                        </Switch>
                        <Switch checked=matrix_maintenance_mode set_checked=set_matrix_maintenance_mode>
                            "Maintenance mode"
                        </Switch>
                    </SwitchGroup>
                    <SwitchGroup
                        id_base="docs-switch-group-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        invalid=true
                        disabled=true
                        error_message="At least one critical channel must stay enabled.".to_string()
                        class_name="docs-switch-group-custom".to_string()
                    >
                        <Switch checked=matrix_critical_alerts set_checked=set_matrix_critical_alerts disabled=true>
                            "Critical alerts"
                        </Switch>
                    </SwitchGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
