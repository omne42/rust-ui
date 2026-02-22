use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    A11yDirection, DateField, DateFieldTone, DateInputGroup, DateInputGroupVariant, Field,
    FieldGroup, FieldGroupDensity, FieldGroupOrientation, SegmentedControl, SegmentedControlSize,
    Switch, SwitchGroup, SwitchGroupOrientation, SwitchGroupTone, TimeField, TimeFieldTone,
};

pub(super) fn switch_group() -> AnyView {
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

pub(super) fn field_group() -> AnyView {
    let orientation_options = vec!["Vertical".to_string(), "Horizontal".to_string()];
    let density_options = vec!["Comfortable".to_string(), "Compact".to_string()];

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_density_index, set_workbench_density_index) = signal(Some(0_usize));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_is_invalid, set_workbench_is_invalid) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_index.get().unwrap_or(0) == 1 {
            FieldGroupOrientation::Horizontal
        } else {
            FieldGroupOrientation::Vertical
        }
    });
    let workbench_density = Signal::derive(move || {
        if workbench_density_index.get().unwrap_or(0) == 1 {
            FieldGroupDensity::Compact
        } else {
            FieldGroupDensity::Comfortable
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<FieldGroup
  id_base="account-fields".to_string()
  label="Account details".to_string()
  description="Group related fields to keep form scanning predictable.".to_string()
>
  <Field label="Name".to_string()>
    <input class="docs-search__input" type="text" placeholder="Ada Lovelace" />
  </Field>
</FieldGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<FieldGroup\n  orientation={:?}\n  density={:?}\n  is_disabled={}\n  disabled={}\n  is_invalid={}\n  invalid={}\n  id_base=\"docs-field-group-workbench\".to_string()\n  label=\"Account details\".to_string()\n  description=\"Group related fields to keep form scanning predictable.\".to_string()\n  aria_label=\"Account field cluster\".to_string()\n  lang={}.to_string()\n  dir={}\n  class_name={}\n>\n  <Field label=\"Name\".to_string()>\n    <input class=\"docs-search__input\" type=\"text\" placeholder=\"Ada Lovelace\" />\n  </Field>\n</FieldGroup>",
            workbench_orientation.get(),
            workbench_density.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_invalid.get()),
            rust_string_literal(if workbench_rtl.get() { "ar" } else { "en-US" }),
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            if workbench_custom_class.get() {
                "\"docs-field-group-workbench\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "FieldGroupWorkbenchActualConfig {{\n  orientation: {:?},\n  density: {:?},\n  is_disabled: {},\n  disabled: {},\n  is_invalid: {},\n  invalid: {},\n  id_base: Some(\"docs-field-group-workbench\"),\n  label: Some(\"Account details\"),\n  description: Some(\"Group related fields to keep form scanning predictable.\"),\n  aria_label: Some(\"Account field cluster\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {:?},\n}}",
            workbench_orientation.get(),
            workbench_density.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_invalid.get()),
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            if workbench_custom_class.get() {
                Some("docs-field-group-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<FieldGroup id_base="fg-default".to_string() label="Default".to_string() />
<FieldGroup id_base="fg-horizontal".to_string() orientation=FieldGroupOrientation::Horizontal density=FieldGroupDensity::Compact is_invalid=true invalid=true />
<FieldGroup id_base="fg-disabled".to_string() is_disabled=true disabled=true aria_label="Disabled cluster".to_string() class_name="docs-field-group-custom".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="FieldGroup"
            slug="field-group"
            group="Forms"
            description="baseline-compatible field clustering primitive with centralized orientation/density/aria/class-state contracts and stable slot + data markers."
        >
            <Playground title="Hello World (Default FieldGroup)" code_signal=hello_code>
                <FieldGroup
                    id_base="docs-field-group-account".to_string()
                    label="Account details".to_string()
                    description="Group related fields to keep form scanning predictable.".to_string()
                >
                    <Field label="Name".to_string()>
                        <input
                            class="docs-search__input"
                            type="text"
                            placeholder="Ada Lovelace"
                        />
                    </Field>

                    <Field label="Email".to_string()>
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="ada@example.com"
                        />
                    </Field>
                </FieldGroup>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-field-group-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="FieldGroup orientation".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-field-group-workbench-density".to_string()
                            options=density_options.clone()
                            selected_index=workbench_density_index
                            set_selected_index=set_workbench_density_index
                            size=SegmentedControlSize::Sm
                            aria_label="FieldGroup density".to_string()
                        />
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_is_invalid set_checked=set_workbench_is_invalid>
                            "is_invalid"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "invalid"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <FieldGroup
                    orientation=workbench_orientation.get()
                    density=workbench_density.get()
                    is_disabled=workbench_is_disabled.get()
                    disabled=workbench_disabled.get()
                    is_invalid=workbench_is_invalid.get()
                    invalid=workbench_invalid.get()
                    id_base="docs-field-group-workbench".to_string()
                    label="Account details".to_string()
                    description="Group related fields to keep form scanning predictable.".to_string()
                    aria_label="Account field cluster".to_string()
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
                        "docs-field-group-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <Field
                        label="Name".to_string()
                        invalid=workbench_invalid.get()
                        disabled=workbench_disabled.get()
                        error_message="Name is required".to_string()
                    >
                        <input class="docs-search__input" type="text" placeholder="Ada Lovelace" />
                    </Field>

                    <Field label="Email".to_string() disabled=workbench_disabled.get()>
                        <input class="docs-search__input" type="email" placeholder="ada@example.com" />
                    </Field>
                </FieldGroup>
            </Playground>

            <Playground title="State Matrix (Default / Horizontal / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <FieldGroup
                        id_base="docs-field-group-matrix-default".to_string()
                        label="Default".to_string()
                        description="Standard vertical cluster".to_string()
                    >
                        <Field label="Name".to_string()>
                            <input class="docs-search__input" type="text" placeholder="Name" />
                        </Field>
                    </FieldGroup>
                    <FieldGroup
                        id_base="docs-field-group-matrix-horizontal".to_string()
                        orientation=FieldGroupOrientation::Horizontal
                        density=FieldGroupDensity::Compact
                        is_invalid=true
                        invalid=true
                        label="Horizontal".to_string()
                    >
                        <Field label="VAT".to_string() invalid=true>
                            <input class="docs-search__input" type="text" placeholder="VAT" />
                        </Field>
                    </FieldGroup>
                    <FieldGroup
                        id_base="docs-field-group-matrix-disabled".to_string()
                        is_disabled=true
                        disabled=true
                        aria_label="Disabled cluster".to_string()
                        class_name="docs-field-group-custom".to_string()
                    >
                        <Field label="Purchase Order".to_string() disabled=true>
                            <input class="docs-search__input" type="text" disabled />
                        </Field>
                    </FieldGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_input_group() -> AnyView {
    let date_input_group_imports = "use leptos::prelude::*;\nuse ui::{DateField, DateFieldTone, DateInputGroup, DateInputGroupVariant, TimeField, TimeFieldTone};".to_string();

    let (invoice_date, set_invoice_date) = signal(Some("2026-03-14".to_string()));
    let on_invoice_date_change = Callback::new(move |next: Option<String>| {
        set_invoice_date.set(next);
    });

    let (ship_window, set_ship_window) = signal(Some("18:30".to_string()));
    let on_ship_window_change = Callback::new(move |next: Option<String>| {
        set_ship_window.set(next);
    });

    let (controlled_date, set_controlled_date) = signal(Some("2026-04-01".to_string()));
    let on_controlled_date_change = Callback::new(move |next: Option<String>| {
        set_controlled_date.set(next);
    });

    let (requested_stream_mode, set_requested_stream_mode) = signal("streaming".to_string());
    let (requested_output_status, set_requested_output_status) = signal("draft".to_string());

    let variant_options = vec!["Primary".to_string(), "Secondary".to_string()];
    let dir_options = vec!["LTR".to_string(), "RTL".to_string()];
    let motion_options = vec!["Default".to_string(), "Snappy".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_dir_index, set_workbench_dir_index) = signal(Some(0_usize));
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_is_full_width, set_workbench_is_full_width) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_invalid, set_workbench_is_invalid) = signal(false);
    let (workbench_is_segmented, set_workbench_is_segmented) = signal(true);
    let (workbench_with_lang, set_workbench_with_lang) = signal(true);
    let (workbench_with_prefix, set_workbench_with_prefix) = signal(true);
    let (workbench_with_suffix, set_workbench_with_suffix) = signal(true);
    let (workbench_with_class, set_workbench_with_class) = signal(false);
    let (workbench_value, set_workbench_value) = signal(Some("2026-04-10".to_string()));
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_change = Callback::new(move |next: Option<String>| {
        set_workbench_change_count.update(|count| *count += 1);
        set_workbench_last_change.set(next.clone().unwrap_or_else(|| "none".to_string()));
        set_workbench_value.set(next);
    });

    let workbench_variant = Signal::derive(move || {
        if workbench_variant_index.get().unwrap_or(0) == 1 {
            DateInputGroupVariant::Secondary
        } else {
            DateInputGroupVariant::Primary
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_dir_index.get().unwrap_or(0) == 1 {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_motion_index.get().unwrap_or(0) == 1 {
            let mut spring =
                ui::text_input::date_input_group::DateInputGroupMotion::default().spring;
            spring.stiffness = 260.0;
            spring.damping = 26.0;
            ui::text_input::date_input_group::DateInputGroupMotion {
                spring,
                enter_scale: 1.03,
            }
        } else {
            ui::text_input::date_input_group::DateInputGroupMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<DateInputGroup\n  is_full_width={}\n  variant={:?}\n  is_disabled={}\n  is_invalid={}\n  is_segmented={}\n  motion=DateInputGroupMotion {{ enter_scale: {}, ..Default::default() }}\n  aria_label=\"Workbench date group\".to_string()\n  lang={}\n  dir={:?}\n  prefix={}\n  suffix={}\n  class_name={}\n>\n  <DateField id_base=\"workbench-date\".to_string() label=\"Invoice date\".to_string() value=workbench_value on_value_change=on_workbench_change />\n</DateInputGroup>",
            bool_word(workbench_is_full_width.get()),
            workbench_variant.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_is_segmented.get()),
            workbench_motion.get().enter_scale,
            if workbench_with_lang.get() {
                "\"en-US\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
            workbench_dir.get(),
            if workbench_with_prefix.get() {
                "prefix_slot".to_string()
            } else {
                "empty_prefix".to_string()
            },
            if workbench_with_suffix.get() {
                "suffix_slot".to_string()
            } else {
                "empty_suffix".to_string()
            },
            if workbench_with_class.get() {
                "\"docs-date-input-group-custom\".to_string()".to_string()
            } else {
                "\"\".to_string()".to_string()
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "DateInputGroupWorkbenchActualConfig {{\n  is_full_width: {},\n  variant: {:?},\n  is_disabled: {},\n  is_invalid: {},\n  is_segmented: {},\n  motion: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  prefix: {:?},\n  suffix: {:?},\n  class_name: {:?},\n}}",
            bool_word(workbench_is_full_width.get()),
            workbench_variant.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_is_segmented.get()),
            workbench_motion.get(),
            Some("Workbench date group"),
            if workbench_with_lang.get() {
                Some("en-US")
            } else {
                None
            },
            workbench_dir.get(),
            if workbench_with_prefix.get() {
                Some("calendar-icon")
            } else {
                None
            },
            if workbench_with_suffix.get() {
                Some("timezone-tag")
            } else {
                None
            },
            if workbench_with_class.get() {
                Some("docs-date-input-group-custom")
            } else {
                None
            },
        )
    });

    let hello_code = Signal::derive(move || {
        r#"<DateInputGroup>
  <DateField id_base="hello-date".to_string() />
</DateInputGroup>"#
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"let (invoice_date, set_invoice_date) = signal(Some("2026-03-14".to_string()));
let on_invoice_date_change = Callback::new(move |next: Option<String>| {
  set_invoice_date.set(next);
});

let (ship_window, set_ship_window) = signal(Some("18:30".to_string()));
let on_ship_window_change = Callback::new(move |next: Option<String>| {
  set_ship_window.set(next);
});

<div class="docs-stack">
  <DateInputGroup>
    <DateField id_base="matrix-default-date".to_string() />
  </DateInputGroup>

<DateInputGroup
  aria_label="Invoice date controls".to_string()
  is_segmented=true
  prefix=move || view! { <span>"📅"</span> }
  suffix=move || view! { <span>"UTC+0"</span> }
>
  <DateField
    id_base="invoice-date".to_string()
    label="Invoice date".to_string()
    tone=DateFieldTone::Quiet
    value=invoice_date
    on_value_change=on_invoice_date_change
  />
</DateInputGroup>

<DateInputGroup
  is_full_width=true
  variant=DateInputGroupVariant::Secondary
  is_invalid=true
  is_segmented=true
  aria_label="Ship window controls".to_string()
  class_name="docs-date-input-group-custom".to_string()
  prefix=move || view! { <span>"🕒"</span> }
  suffix=move || view! { <span>"5m"</span> }
>
  <TimeField
    id_base="ship-window".to_string()
    label="Ship window".to_string()
    tone=TimeFieldTone::Strong
    minute_step=5
    value=ship_window
    on_value_change=on_ship_window_change
  />
</DateInputGroup>
</div>"#
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"let (controlled_date, set_controlled_date) = signal(Some("2026-04-01".to_string()));
let on_controlled_date_change = Callback::new(move |next: Option<String>| {
  set_controlled_date.set(next);
});

<div class="docs-stack">
  <DateInputGroup aria_label="Controlled date field".to_string()>
    <DateField
      id_base="controlled-date".to_string()
      label="Controlled".to_string()
      tone=DateFieldTone::Quiet
      value=controlled_date
      on_value_change=on_controlled_date_change
    />
  </DateInputGroup>

  <DateInputGroup aria_label="Uncontrolled date field".to_string()>
    <DateField
      id_base="uncontrolled-date".to_string()
      label="Uncontrolled".to_string()
      tone=DateFieldTone::Quiet
      default_value=Some("2026-04-09".to_string())
    />
  </DateInputGroup>
</div>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<DateInputGroup aria_label="LLM date result".to_string() is_segmented=true>
  <DateField id_base="streaming-contract-date".to_string() />
</DateInputGroup>

// requested mode: streaming
// requested output status: draft
// effective component markers: data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified"#.to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"// Cargo.toml dependency baseline:
// ui-components = { default-features = false, features = ["component-date_input_group", "inject-css"] }
// Source paths:
// components/date-input-group/src/{mod,logic,view,styles,motion}.rs
// apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group

<DateInputGroup is_segmented=true>
  <DateField
    id_base="source-first-date".to_string()
    label="Invoice date".to_string()
    tone=DateFieldTone::Quiet
    default_value=Some("2026-03-14".to_string())
  />
</DateInputGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DateInputGroup"
            slug="date-input-group"
            group="Forms"
            description="baseline-style date-input grouping primitive with centralized variant/width/prefix-suffix state contracts and segmented slot markers."
        >
            <Playground
                title="Hello World (Default API)"
                description="Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines."
                code_signal=hello_code
                code_imports=date_input_group_imports.clone()
            >
                <DateInputGroup>
                    <DateField id_base="docs-date-input-group-hello".to_string() />
                </DateInputGroup>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                code_imports=date_input_group_imports.clone()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-input-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-date-input-group-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateInputGroup variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-input-group-workbench-dir".to_string()
                            options=dir_options.clone()
                            selected_index=workbench_dir_index
                            set_selected_index=set_workbench_dir_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateInputGroup dir".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-date-input-group-workbench-motion".to_string()
                            options=motion_options.clone()
                            selected_index=workbench_motion_index
                            set_selected_index=set_workbench_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="DateInputGroup motion".to_string()
                        />
                        <Switch checked=workbench_is_full_width set_checked=set_workbench_is_full_width>
                            "is_full_width"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_invalid set_checked=set_workbench_is_invalid>
                            "is_invalid"
                        </Switch>
                        <Switch checked=workbench_is_segmented set_checked=set_workbench_is_segmented>
                            "is_segmented"
                        </Switch>
                        <Switch checked=workbench_with_lang set_checked=set_workbench_with_lang>
                            "lang"
                        </Switch>
                        <Switch checked=workbench_with_prefix set_checked=set_workbench_with_prefix>
                            "prefix"
                        </Switch>
                        <Switch checked=workbench_with_suffix set_checked=set_workbench_with_suffix>
                            "suffix"
                        </Switch>
                        <Switch checked=workbench_with_class set_checked=set_workbench_with_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <DateInputGroup
                        is_full_width=workbench_is_full_width.get()
                        variant=workbench_variant.get()
                        is_disabled=workbench_is_disabled.get()
                        is_invalid=workbench_is_invalid.get()
                        is_segmented=workbench_is_segmented.get()
                        motion=workbench_motion.get()
                        aria_label="Workbench date group".to_string()
                        lang=if workbench_with_lang.get() {
                            "en-US".to_string()
                        } else {
                            String::new()
                        }
                        dir=workbench_dir.get()
                        prefix=move || {
                            if workbench_with_prefix.get() {
                                view! { <span>"📅"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }
                        suffix=move || {
                            if workbench_with_suffix.get() {
                                view! { <span>"UTC"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }
                        class_name=if workbench_with_class.get() {
                            "docs-date-input-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <DateField
                            id_base="docs-date-input-group-workbench".to_string()
                            label="Invoice date".to_string()
                            tone=DateFieldTone::Quiet
                            value=workbench_value
                            on_value_change=on_workbench_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "value: " {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                    <span class="ui-muted">
                        "on_value_change count="
                        {move || workbench_change_count.get()}
                        " · last="
                        {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Prefix-Suffix / Secondary+Invalid)"
                code_signal=state_matrix_code
                code_imports=date_input_group_imports.clone()
            >
                <div class="docs-stack" data-slot="date-input-group-state-matrix">
                    <DateInputGroup>
                        <DateField id_base="docs-date-input-group-matrix-default".to_string() />
                    </DateInputGroup>
                    <DateInputGroup
                        aria_label="Invoice date controls".to_string()
                        is_segmented=true
                        prefix=move || view! { <span>"📅"</span> }
                        suffix=move || view! { <span>"UTC+0"</span> }
                    >
                        <DateField
                            id_base="docs-date-input-group-invoice".to_string()
                            label="Invoice date".to_string()
                            tone=DateFieldTone::Quiet
                            value=invoice_date
                            on_value_change=on_invoice_date_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "invoice date: "
                        {move || invoice_date.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                    <DateInputGroup
                        is_full_width=true
                        variant=DateInputGroupVariant::Secondary
                        is_invalid=true
                        is_segmented=true
                        aria_label="Ship window controls".to_string()
                        class_name="docs-date-input-group-custom".to_string()
                        prefix=move || view! { <span>"🕒"</span> }
                        suffix=move || view! { <span>"5m"</span> }
                    >
                        <TimeField
                            id_base="docs-date-input-group-time".to_string()
                            label="Ship window".to_string()
                            tone=TimeFieldTone::Strong
                            minute_step=5
                            value=ship_window
                            on_value_change=on_ship_window_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "ship window: "
                        {move || ship_window.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Child Field Axis)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=date_input_group_imports.clone()
            >
                <div class="docs-stack" data-slot="date-input-group-controlled-matrix">
                    <DateInputGroup aria_label="Controlled date field".to_string()>
                        <DateField
                            id_base="docs-date-input-group-controlled".to_string()
                            label="Controlled".to_string()
                            tone=DateFieldTone::Quiet
                            value=controlled_date
                            on_value_change=on_controlled_date_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "controlled date: "
                        {move || controlled_date.get().unwrap_or_else(|| "none".to_string())}
                    </span>

                    <DateInputGroup aria_label="Uncontrolled date field".to_string()>
                        <DateField
                            id_base="docs-date-input-group-uncontrolled".to_string()
                            label="Uncontrolled".to_string()
                            tone=DateFieldTone::Quiet
                            default_value="2026-04-09".to_string()
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "uncontrolled DateField uses default_value and internal state after mount."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Streaming is optional; fallback stays snapshot."
                code_signal=stream_snapshot_code
                code_imports=date_input_group_imports.clone()
            >
                <div class="docs-stack" data-slot="date-input-group-streaming-contract">
                    <label class="docs-subtitle">
                        "requested mode: "
                        <select
                            class="docs-select"
                            data-slot="date-input-group-requested-stream-mode"
                            prop:value=move || requested_stream_mode.get()
                            on:change=move |ev| set_requested_stream_mode.set(event_target_value(&ev))
                        >
                            <option value="streaming">"streaming"</option>
                            <option value="snapshot">"snapshot"</option>
                        </select>
                    </label>

                    <label class="docs-subtitle">
                        "requested output status: "
                        <select
                            class="docs-select"
                            data-slot="date-input-group-requested-output-status"
                            prop:value=move || requested_output_status.get()
                            on:change=move |ev| set_requested_output_status.set(event_target_value(&ev))
                        >
                            <option value="draft">"draft"</option>
                            <option value="verified">"verified"</option>
                        </select>
                    </label>

                    <DateInputGroup aria_label="LLM date result".to_string() is_segmented=true>
                        <DateField id_base="docs-date-input-group-streaming".to_string() />
                    </DateInputGroup>

                    <span class="ui-muted" data-slot="date-input-group-streaming-requested-state">
                        "requested mode: "
                        {move || requested_stream_mode.get()}
                        " · requested output status: "
                        {move || requested_output_status.get()}
                    </span>
                    <span class="ui-muted" data-slot="date-input-group-streaming-effective-state">
                        "effective component markers: data-ui-stream-mode=snapshot data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=date_input_group_imports
            >
                <div class="docs-stack" data-slot="date-input-group-source-first">
                    <DateInputGroup is_segmented=true>
                        <DateField
                            id_base="docs-date-input-group-source-first".to_string()
                            label="Invoice date".to_string()
                            tone=DateFieldTone::Quiet
                            default_value="2026-03-14".to_string()
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "Copy action auto-injects missing imports for direct run."
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="date-input-group-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="date-input-group-source-first-contract">
                    "Use "
                    <code>"Show code"</code>
                    " + copy from the Source-first Playground. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p data-slot="date-input-group-source-first-dependency-baseline">
                    "Dependency baseline (Cargo.toml): "
                    <code>
                        "ui-components = { default-features = false, features = [\"component-date_input_group\", \"inject-css\"] }"
                    </code>
                </p>
                <ul data-slot="date-input-group-source-prerequisites">
                    <li><code>"component-date_input_group"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
                <ul data-slot="date-input-group-source-paths">
                    <li><code>"components/date-input-group/src/mod.rs"</code></li>
                    <li><code>"components/date-input-group/src/logic.rs"</code></li>
                    <li><code>"components/date-input-group/src/view.rs"</code></li>
                    <li><code>"components/date-input-group/src/styles.rs"</code></li>
                    <li><code>"components/date-input-group/src/motion.rs"</code></li>
                    <li><code>"apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
