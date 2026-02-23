use super::*;

pub(crate) fn field_group() -> AnyView {
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
