use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Checkbox, CheckboxGroup, Form, FormLabelAlign, FormLabelPosition, Input, InputOtp, InputSize,
    InputVariant, NumberField, Radio, RadioGroup, RadioGroupOrientation, SearchField,
    SegmentedControl, SegmentedControlSize, Switch, TextArea, TextField,
};

pub(super) fn form() -> AnyView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());

    let code = r#"<Form required=true label_position=FormLabelPosition::Left>
  <Input id="name".to_string() label="Name".to_string() ... />
  <Input id="email".to_string() label="Email".to_string() ... />
</Form>"#;

    view! {
        <ComponentPage
            title="Form"
            slug="form"
            group="Forms"
            description="A context provider for form-wide disabled/required/label layout."
        >
            <Playground title="Label layout context" code=code>
                <Form
                    required=true
                    label_position=FormLabelPosition::Left
                    label_align=FormLabelAlign::End
                >
                    <div class="docs-stack">
                        <Input
                            id="docs-form-name".to_string()
                            label="Name".to_string()
                            value=name
                            set_value=set_name
                            placeholder="Jane".to_string()
                            size=InputSize::Md
                            variant=InputVariant::Bordered
                        />
                        <Input
                            id="docs-form-email".to_string()
                            label="Email".to_string()
                            value=email
                            set_value=set_email
                            placeholder="jane@example.com".to_string()
                            size=InputSize::Md
                            variant=InputVariant::Bordered
                        />
                    </div>
                </Form>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn input() -> AnyView {
    let (value, set_value) = signal(String::new());
    let (invalid, set_invalid) = signal(false);

    let code = r#"let (value, set_value) = signal(String::new());
<Input id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder="Type something…".to_string()
  is_clearable=true
/>"#;

    view! {
        <ComponentPage
            title="Input"
            slug="input"
            group="Forms"
            description="Spectrum-style text input with label, description/error, and clear button."
        >
            <Playground title="Clearable + validation" code=code>
                <div class="docs-stack">
                    <Input
                        id="docs-input".to_string()
                        label="Name".to_string()
                        value=value
                        set_value=set_value
                        placeholder="Type something…".to_string()
                        is_clearable=true
                        invalid=Signal::derive(move || invalid.get())
                        description="Try toggling invalid.".to_string()
                        error="This field is invalid.".to_string()
                        size=InputSize::Md
                        variant=InputVariant::Bordered
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_invalid.update(|v| *v = !*v))
                        >
                            {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                        </ui_components::Button>
                        <span class="ui-muted">"value: " {move || value.get()}</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text_field() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = r#"let (value, set_value) = signal(String::new());
<TextField id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder="Jane".to_string()
/>"#;

    view! {
        <ComponentPage
            title="TextField"
            slug="text-field"
            group="Forms"
            description="A compact field wrapper built on headless text field semantics."
        >
            <Playground title="Label + placeholder" code=code>
                <TextField
                    id="docs-text-field".to_string()
                    label="Name".to_string()
                    value=value
                    set_value=set_value
                    placeholder="Jane".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text_area() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = r#"let (value, set_value) = signal(String::new());
<TextArea id="about".to_string()
  label="About".to_string()
  value=value
  set_value=set_value
/>"#;

    view! {
        <ComponentPage
            title="TextArea"
            slug="text-area"
            group="Forms"
            description="Multiline text field with Spectrum-style semantics."
        >
            <Playground title="Multiline" code=code>
                <TextArea
                    id="docs-text-area".to_string()
                    label="About".to_string()
                    value=value
                    set_value=set_value
                    placeholder="Write something…".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn search_field() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = r#"let (value, set_value) = signal(String::new());
<SearchField id="search".to_string()
  label="Search".to_string()
  value=value
  set_value=set_value
/>"#;

    view! {
        <ComponentPage
            title="SearchField"
            slug="search-field"
            group="Forms"
            description="Search input with clear action and keyboard-friendly semantics."
        >
            <Playground title="Search" code=code>
                <SearchField
                    id="docs-search-field".to_string()
                    label="Search".to_string()
                    value=value
                    set_value=set_value
                    placeholder="Search…".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn number_field() -> AnyView {
    let (value, set_value) = signal(42_i64);
    let code = r#"let (value, set_value) = signal(42_i64);
<NumberField id="qty".to_string()
  label="Quantity".to_string()
  value=value
  set_value=set_value
  min=0
  max=100
/>"#;

    view! {
        <ComponentPage
            title="NumberField"
            slug="number-field"
            group="Forms"
            description="Numeric input with steppers and keyboard control."
        >
            <Playground title="Stepper" code=code>
                <div class="docs-row">
                    <NumberField
                        id="docs-number-field".to_string()
                        label="Quantity".to_string()
                        value=value
                        set_value=set_value
                        min=0
                        max=100
                    />
                    <span class="ui-muted">"value: " {move || value.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn input_otp() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = r#"let (value, set_value) = signal(String::new());
<InputOtp id_base="otp".to_string()
  label="One-time code".to_string()
  value=value
  set_value=set_value
  length=6
/>"#;

    view! {
        <ComponentPage
            title="InputOtp"
            slug="input-otp"
            group="Forms"
            description="HeroUI-style OTP input with a single hidden input and slot chrome."
        >
            <Playground title="OTP" code=code>
                <div class="docs-stack">
                    <InputOtp
                        id_base="docs-otp".to_string()
                        label="One-time code".to_string()
                        value=value
                        set_value=set_value
                        length=6
                    />
                    <span class="ui-muted">"value: " {move || value.get()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn checkbox() -> AnyView {
    let (checked, set_checked) = signal(false);
    let code = r#"let (checked, set_checked) = signal(false);
<Checkbox checked=checked set_checked=set_checked>"Accept"</Checkbox>"#;

    view! {
        <ComponentPage
            title="Checkbox"
            slug="checkbox"
            group="Forms"
            description="Pressable checkbox with focus-visible ring and spring indicator."
        >
            <Playground title="Checkbox" code=code>
                <div class="docs-row">
                    <Checkbox checked=checked set_checked=set_checked>"Accept"</Checkbox>
                    <span class="ui-muted">"checked: " {move || checked.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn checkbox_group() -> AnyView {
    let (apple, set_apple) = signal(false);
    let (banana, set_banana) = signal(true);
    let (mango, set_mango) = signal(false);

    let invalid = Signal::derive(move || !(apple.get() || banana.get() || mango.get()));
    let required = Signal::derive(|| true);
    let external_desc_id = "docs-checkbox-group-extra".to_string();
    let aria_describedby = Signal::derive(move || Some(external_desc_id.clone()));

    let (disabled_a, set_disabled_a) = signal(true);
    let (disabled_b, set_disabled_b) = signal(false);

    let (optional_email, set_optional_email) = signal(false);
    let (optional_sms, set_optional_sms) = signal(true);
    let optional_selected_count =
        Signal::derive(move || usize::from(optional_email.get()) + usize::from(optional_sms.get()));

    let code = r#"let invalid = Signal::derive(move || !(apple.get() || banana.get()));
<CheckboxGroup
  id="demo".to_string()
  label="Fruits".to_string()
  description="Pick at least one".to_string()
  error="At least one required".to_string()
  required=Signal::derive(|| true)
  invalid=invalid
>
  <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
  <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
</CheckboxGroup>"#;

    let states_code = r#"<CheckboxGroup
  id="disabled".to_string()
  label="Notifications".to_string()
  disabled=true
>
  <Checkbox ...>"Email"</Checkbox>
  <Checkbox ...>"SMS"</Checkbox>
</CheckboxGroup>
<CheckboxGroup
  id="optional".to_string()
  label="Delivery channels".to_string()
  description="Optional selection".to_string()
>
  <Checkbox ...>"Email"</Checkbox>
  <Checkbox ...>"SMS"</Checkbox>
</CheckboxGroup>"#;

    view! {
        <ComponentPage
            title="CheckboxGroup"
            slug="checkbox-group"
            group="Forms"
            description="Fieldset wrapper with normalized labels, validation semantics, and Spectrum-style root state attrs."
        >
            <Playground title="Validation + Required" code=code>
                <div class="docs-stack">
                    <CheckboxGroup
                        id="docs-checkbox-group".to_string()
                        label="Fruits".to_string()
                        description="Pick at least one".to_string()
                        error="At least one required".to_string()
                        required=required
                        invalid=invalid
                        aria_describedby=aria_describedby
                    >
                        <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
                        <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
                        <Checkbox checked=mango set_checked=set_mango>"Mango"</Checkbox>
                    </CheckboxGroup>

                    <div id="docs-checkbox-group-extra" class="ui-muted">
                        "Tip: combine with an external description via aria-describedby."
                    </div>

                    <span class="ui-muted">
                        "selected: "
                        {move || {
                            let mut picked = Vec::new();
                            if apple.get() {
                                picked.push("Apple");
                            }
                            if banana.get() {
                                picked.push("Banana");
                            }
                            if mango.get() {
                                picked.push("Mango");
                            }
                            if picked.is_empty() {
                                "None".to_string()
                            } else {
                                picked.join(", ")
                            }
                        }}
                        " · invalid: "
                        {move || invalid.get().to_string()}
                    </span>

                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_apple.set(false);
                                set_banana.set(false);
                                set_mango.set(false);
                            })
                        >
                            "Clear selections"
                        </ui_components::Button>
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Optional" code=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <CheckboxGroup
                            id="docs-checkbox-group-disabled".to_string()
                            label="Notifications".to_string()
                            description="Read-only preferences".to_string()
                            disabled=true
                        >
                            <Checkbox checked=disabled_a set_checked=set_disabled_a>"Email"</Checkbox>
                            <Checkbox checked=disabled_b set_checked=set_disabled_b>"SMS"</Checkbox>
                        </CheckboxGroup>
                        <span class="ui-muted">"disabled: true"</span>
                    </div>

                    <div class="docs-stack">
                        <CheckboxGroup
                            id="docs-checkbox-group-optional".to_string()
                            label="Delivery channels".to_string()
                            description="Optional selection (required = false)".to_string()
                        >
                            <Checkbox checked=optional_email set_checked=set_optional_email>
                                "Email"
                            </Checkbox>
                            <Checkbox checked=optional_sms set_checked=set_optional_sms>"SMS"</Checkbox>
                        </CheckboxGroup>
                        <span class="ui-muted">
                            "optional selected count: "
                            {move || optional_selected_count.get().to_string()}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn switch() -> AnyView {
    let (checked, set_checked) = signal(true);
    let code = r#"let (checked, set_checked) = signal(true);
<Switch checked=checked set_checked=set_checked>"Notifications"</Switch>"#;

    view! {
        <ComponentPage
            title="Switch"
            slug="switch"
            group="Forms"
            description="Switch toggle with spring-driven thumb motion."
        >
            <Playground title="Switch" code=code>
                <div class="docs-row">
                    <Switch checked=checked set_checked=set_checked>"Notifications"</Switch>
                    <span class="ui-muted">"checked: " {move || checked.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn radio_group() -> AnyView {
    let options = vec![
        "Small".to_string(),
        "Medium".to_string(),
        "Large".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));

    let billing_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (billing_selected, set_billing_selected) = signal(Some(2_usize));
    let external_label_id = "docs-radio-group-billing-label".to_string();

    let empty_options = Vec::<String>::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = r#"let (selected, set_selected) = signal(Some(1_usize));
<RadioGroup id_base="size".to_string()
  options=options
  label="Size".to_string()
  selected_index=selected
  set_selected_index=set_selected
/>"#;

    let states_code = r#"let (billing_selected, set_billing_selected) = signal(Some(2_usize));
<RadioGroup
  id_base="billing".to_string()
  options=billing_options
  orientation=RadioGroupOrientation::Horizontal
  disabled_indices=vec![1]
  aria_labelledby="docs-radio-group-billing-label".to_string()
  selected_index=billing_selected
  set_selected_index=set_billing_selected
/>

<RadioGroup
  id_base="empty".to_string()
  options=Vec::<String>::new()
  disabled=true
  aria_label="No options available".to_string()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
/>"#;

    view! {
        <ComponentPage
            title="RadioGroup"
            slug="radio-group"
            group="Forms"
            description="Roving tabindex radiogroup with Spectrum-style labeling, orientation, and disabled semantics."
        >
            <Playground title="Label + Selection" code=code>
                <div class="docs-stack">
                    <RadioGroup
                        id_base="docs-radio-group".to_string()
                        options=options
                        label="Size".to_string()
                        selected_index=selected
                        set_selected_index=set_selected
                    />
                    <span class="ui-muted">
                        "selected: " {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Horizontal + Disabled + Empty" code=states_code>
                <div class="docs-stack">
                    <div id=external_label_id.clone() class="ui-muted">"Billing cycle"</div>
                    <RadioGroup
                        id_base="docs-radio-group-billing".to_string()
                        options=billing_options
                        orientation=RadioGroupOrientation::Horizontal
                        disabled_indices=vec![1]
                        aria_labelledby=external_label_id.clone()
                        selected_index=billing_selected
                        set_selected_index=set_billing_selected
                    />
                    <span class="ui-muted">
                        "billing: " {move || billing_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>

                    <RadioGroup
                        id_base="docs-radio-group-empty".to_string()
                        options=empty_options
                        disabled=true
                        aria_label="No options available".to_string()
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                    />
                    <span class="ui-muted">
                        "empty selected: " {move || empty_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn radio() -> AnyView {
    let (checked, set_checked) = signal(false);
    let on_change = Callback::new(move |next: bool| set_checked.set(next));
    let code = r#"let (checked, set_checked) = signal(false);
<Radio id="r1".to_string() label="Standalone".to_string() checked=checked.into() on_change=Some(on_change) />"#;

    view! {
        <ComponentPage
            title="Radio"
            slug="radio"
            group="Forms"
            description="Standalone radio button (use RadioGroup for semantics)."
        >
            <Playground title="Standalone" code=code>
                <div class="docs-row">
                    <Radio
                        id="docs-radio".to_string()
                        label="Standalone".to_string()
                        checked=checked.into()
                        on_change=on_change
                    />
                    <span class="ui-muted">"checked: " {move || checked.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn segmented_control() -> AnyView {
    let options = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Settings".to_string(),
    ];
    let (selected, set_selected) = signal(Some(0_usize));
    let code = r#"let (selected, set_selected) = signal(Some(0_usize));
<SegmentedControl id_base="seg".to_string()
  options=options
  selected_index=selected
  set_selected_index=set_selected
/>"#;

    view! {
        <ComponentPage
            title="SegmentedControl"
            slug="segmented-control"
            group="Forms"
            description="Segmented control with spring active indicator motion."
        >
            <Playground title="Segments" code=code>
                <div class="docs-stack">
                    <SegmentedControl
                        id_base="docs-segments".to_string()
                        options=options
                        selected_index=selected
                        set_selected_index=set_selected
                        size=SegmentedControlSize::Default
                    />
                    <span class="ui-muted">
                        "selected: " {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
