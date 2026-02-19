use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::text_input::text_area::TextAreaMotion;
use ui_components::{
    Checkbox, CheckboxGroup, CheckboxSize, CheckboxVariant, Form, FormLabelAlign,
    FormLabelPosition, Input, InputGroup, InputOtp, InputSize, InputVariant, NumberField, Radio,
    RadioGroup, RadioGroupOrientation, SearchField, SegmentedControl, SegmentedControlOrientation,
    SegmentedControlSize, Switch, TextArea,
};

pub(super) fn input_group() -> AnyView {
    let (email_user, set_email_user) = signal(String::new());
    let (search_query, set_search_query) = signal(String::new());

    let code = Signal::derive(move || {
        r#"let (email_user, set_email_user) = signal(String::new());
<InputGroup
  aria_label="Email input group".to_string()
  start_content=move || view! { <span>"@"</span> }
  end_content=move || view! { <span>".com"</span> }
>
  <Input
    id="email-user".to_string()
    value=email_user
    set_value=set_email_user
    aria_label="Email user".to_string()
    placeholder="username".to_string()
    label_hidden=true
  />
</InputGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (search_query, set_search_query) = signal(String::new());
let (disabled_value, set_disabled_value) = signal(String::new());

<InputGroup
  attached=false
  aria_label="Search controls".to_string()
  start_content=move || view! { <span>"🔍"</span> }
  end_content=move || view! { <span>"⌘K"</span> }
>
  <Input
    id="search-query".to_string()
    value=search_query
    set_value=set_search_query
    aria_label="Search query".to_string()
    placeholder="Search docs".to_string()
    label_hidden=true
  />
</InputGroup>
<InputGroup disabled=true aria_label="Disabled controls".to_string()>
  <Input
    id="disabled-group-input".to_string()
    value=disabled_value
    set_value=set_disabled_value
    aria_label="Disabled field".to_string()
    placeholder="Disabled".to_string()
    label_hidden=true
    disabled=true
  />
</InputGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="InputGroup"
            slug="input-group"
            group="Forms"
            description="Composes one or more inputs with shared prefix/suffix addons and baseline-style state contracts."
        >
            <Playground title="Attached Addons" code_signal=code>
                <div class="docs-stack">
                    <InputGroup
                        aria_label="Email input group".to_string()
                        start_content=move || view! { <span>"@"</span> }
                        end_content=move || view! { <span>".com"</span> }
                    >
                        <Input
                            id="docs-input-group-email".to_string()
                            value=email_user
                            set_value=set_email_user
                            aria_label="Email user".to_string()
                            placeholder="username".to_string()
                            label_hidden=true
                        />
                    </InputGroup>
                    <span class="ui-muted">"email: " {move || email_user.get()}</span>
                </div>
            </Playground>

            <Playground title="Detached + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <InputGroup
                        attached=false
                        aria_label="Search controls".to_string()
                        start_content=move || view! { <span>"🔍"</span> }
                        end_content=move || view! { <span>"⌘K"</span> }
                    >
                        <Input
                            id="docs-input-group-search".to_string()
                            value=search_query
                            set_value=set_search_query
                            aria_label="Search query".to_string()
                            placeholder="Search docs".to_string()
                            label_hidden=true
                        />
                    </InputGroup>

                    <InputGroup disabled=true aria_label="Disabled controls".to_string()>
                        <Input
                            id="docs-input-group-disabled".to_string()
                            value=email_user
                            set_value=set_email_user
                            aria_label="Disabled field".to_string()
                            placeholder="Disabled".to_string()
                            label_hidden=true
                            disabled=true
                        />
                    </InputGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn form() -> AnyView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (matrix_default_name, set_matrix_default_name) = signal(String::new());
    let (matrix_required_email, set_matrix_required_email) = signal(String::new());
    let (matrix_disabled_name, set_matrix_disabled_name) = signal(String::new());
    let (matrix_readonly_token, set_matrix_readonly_token) = signal("token_123".to_string());

    let label_position_options = vec!["top".to_string(), "left".to_string()];
    let label_align_options = vec!["start".to_string(), "end".to_string()];
    let (workbench_label_position_index, set_workbench_label_position_index) = signal(Some(0));
    let (workbench_label_align_index, set_workbench_label_align_index) = signal(Some(0));
    let (workbench_is_required, set_workbench_is_required) = signal(true);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_read_only, set_workbench_is_read_only) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_label_position =
        Signal::derive(
            move || match workbench_label_position_index.get().unwrap_or(0) {
                1 => FormLabelPosition::Left,
                _ => FormLabelPosition::Top,
            },
        );
    let workbench_label_align =
        Signal::derive(
            move || match workbench_label_align_index.get().unwrap_or(0) {
                1 => FormLabelAlign::End,
                _ => FormLabelAlign::Start,
            },
        );

    let workbench_code = Signal::derive(move || {
        let label_position = workbench_label_position.get();
        let label_align = workbench_label_align.get();
        let is_required = workbench_is_required.get();
        let is_disabled = workbench_is_disabled.get();
        let is_read_only = workbench_is_read_only.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec!["<Form".to_string()];
        if is_required {
            lines.push("  is_required=true".to_string());
        }
        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if is_read_only {
            lines.push("  is_read_only=true".to_string());
        }
        if label_position != FormLabelPosition::Top {
            lines.push(format!(
                "  label_position=FormLabelPosition::{label_position:?}"
            ));
        }
        if label_align != FormLabelAlign::Start {
            lines.push(format!("  label_align=FormLabelAlign::{label_align:?}"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-form-custom\".into()".to_string());
        }
        lines.push(">".to_string());
        lines.push("  <Input id=\"name\" label=\"Name\" ... />".to_string());
        lines.push("  <Input id=\"email\" label=\"Email\" ... />".to_string());
        lines.push("</Form>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let label_position = workbench_label_position.get();
        let label_align = workbench_label_align.get();
        let is_required = workbench_is_required.get();
        let is_disabled = workbench_is_disabled.get();
        let is_read_only = workbench_is_read_only.get();
        let custom_class = workbench_custom_class.get();
        let class = if custom_class {
            "ui-form docs-form-custom"
        } else {
            "ui-form"
        };

        format!(
            "FormActualConfig {{\n  is_required: {is_required},\n  is_disabled: {is_disabled},\n  is_read_only: {is_read_only},\n  label_position: {label_position:?},\n  label_align: {label_align:?},\n  class: \"{class}\",\n  marker_expectations: [\"data-disabled\", \"data-readonly\", \"data-required\", \"data-label-position\", \"data-label-align\"],\n}}"
        )
    });

    let form_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/field_form/form/styles.rs */\n{}",
            ui_components::field_form::form::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Form>
  <Input id="m1-name".to_string() label="Name".to_string() ... />
</Form>
<Form is_required=true label_position=FormLabelPosition::Left label_align=FormLabelAlign::End>
  <Input id="m2-name".to_string() label="Name".to_string() ... />
</Form>
<Form is_disabled=true>
  <Input id="m3-name".to_string() label="Name".to_string() ... />
</Form>
<Form is_read_only=true class_name="docs-form-custom".to_string()>
  <Input id="m4-name".to_string() label="Name".to_string() ... />
</Form>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Form"
            slug="form"
            group="Forms"
            description="A context provider for form-wide disabled/required/label layout."
        >
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=form_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/field_form/form/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 label-position/align/required/disabled/read-only/class，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Label Position"</div>
                            <SegmentedControl
                                id_base="docs-form-label-position".to_string()
                                options=label_position_options.clone()
                                selected_index=workbench_label_position_index
                                set_selected_index=set_workbench_label_position_index
                                size=SegmentedControlSize::Sm
                                aria_label="Form label position".to_string()
                                orientation=SegmentedControlOrientation::Horizontal
                            />

                            <div class="docs-search__label">"Label Align"</div>
                            <SegmentedControl
                                id_base="docs-form-label-align".to_string()
                                options=label_align_options.clone()
                                selected_index=workbench_label_align_index
                                set_selected_index=set_workbench_label_align_index
                                size=SegmentedControlSize::Sm
                                aria_label="Form label align".to_string()
                                orientation=SegmentedControlOrientation::Horizontal
                            />

                            <Switch checked=workbench_is_required set_checked=set_workbench_is_required>
                                "is_required"
                            </Switch>
                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "is_disabled"
                            </Switch>
                            <Switch checked=workbench_is_read_only set_checked=set_workbench_is_read_only>
                                "is_read_only"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class_name"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let label_position = workbench_label_position.get();
                        let label_align = workbench_label_align.get();
                        let is_required = workbench_is_required.get();
                        let is_disabled = workbench_is_disabled.get();
                        let is_read_only = workbench_is_read_only.get();
                        let class_name = if workbench_custom_class.get() {
                            "docs-form-custom".to_string()
                        } else {
                            "".to_string()
                        };

                        view! {
                            <Form
                                is_required=is_required
                                is_disabled=is_disabled
                                is_read_only=is_read_only
                                label_position=label_position
                                label_align=label_align
                                class_name=class_name
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
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Default / Required / Disabled / ReadOnly)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 240px;">
                        <span class="ui-muted">"Default"</span>
                        <Form>
                            <Input
                                id="docs-form-matrix-default".to_string()
                                label="Name".to_string()
                                value=matrix_default_name
                                set_value=set_matrix_default_name
                                placeholder="Jane".to_string()
                                size=InputSize::Md
                                variant=InputVariant::Bordered
                            />
                        </Form>
                    </div>

                    <div class="docs-card" style="flex: 1 1 240px;">
                        <span class="ui-muted">"Required + Left/End"</span>
                        <Form
                            is_required=true
                            label_position=FormLabelPosition::Left
                            label_align=FormLabelAlign::End
                        >
                            <Input
                                id="docs-form-matrix-required".to_string()
                                label="Email".to_string()
                                value=matrix_required_email
                                set_value=set_matrix_required_email
                                placeholder="jane@example.com".to_string()
                                size=InputSize::Md
                                variant=InputVariant::Bordered
                            />
                        </Form>
                    </div>

                    <div class="docs-card" style="flex: 1 1 240px;">
                        <span class="ui-muted">"Disabled"</span>
                        <Form is_disabled=true>
                            <Input
                                id="docs-form-matrix-disabled".to_string()
                                label="Name".to_string()
                                value=matrix_disabled_name
                                set_value=set_matrix_disabled_name
                                placeholder="Disabled".to_string()
                                size=InputSize::Md
                                variant=InputVariant::Bordered
                            />
                        </Form>
                    </div>

                    <div class="docs-card" style="flex: 1 1 240px;">
                        <span class="ui-muted">"ReadOnly + Custom"</span>
                        <Form is_read_only=true class_name="docs-form-custom".to_string()>
                            <Input
                                id="docs-form-matrix-readonly".to_string()
                                label="Token".to_string()
                                value=matrix_readonly_token
                                set_value=set_matrix_readonly_token
                                placeholder="Read only".to_string()
                                size=InputSize::Md
                                variant=InputVariant::Bordered
                            />
                        </Form>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn input() -> AnyView {
    let (value, set_value) = signal(String::new());
    let (invalid, set_invalid) = signal(false);

    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
<Input id="name".to_string()
  label="Name".to_string()
  value=value
  set_value=set_value
  placeholder="Type something…".to_string()
  is_clearable=true
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Input"
            slug="input"
            group="Forms"
            description="baseline-style text input with label, description/error, and clear button."
        >
            <Playground title="Clearable + validation" code_signal=code>
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

pub(super) fn text_area() -> AnyView {
    let (value, set_value) = signal("Shipping notes".to_string());
    let (invalid, set_invalid) = signal(false);
    let on_value_change = Callback::new(move |next: String| set_value.set(next));

    let hello_code = Signal::derive(move || {
        r#"<TextArea
  id="summary".to_string()
  label="Summary".to_string()
  default_value="Ready for launch".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (value, set_value) = signal("Shipping notes".to_string());
let (invalid, set_invalid) = signal(false);
let on_value_change = Callback::new(move |next: String| set_value.set(next));

<TextArea
  id="docs-text-area-markers".to_string()
  label="Release notes".to_string()
  value=Signal::derive(move || value.get())
  default_value="Shipping notes".to_string()
  on_value_change=on_value_change
  is_required=Signal::derive(move || true)
  is_invalid=Signal::derive(move || invalid.get())
  description="Inspect source/state marker contracts".to_string()
  error="Release notes are required".to_string()
  placeholder="Write release notes…".to_string()
  motion=TextAreaMotion::disabled()
  rows=6
  class_name="docs-text-area-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TextArea"
            slug="text-area"
            group="Forms"
            description="Multiline text field with baseline-style semantics and explicit state/source marker contracts."
        >
            <Playground
                title="Hello World"
                description="Minimal usage: id + label + default_value."
                code_signal=hello_code
            >
                <TextArea
                    id="docs-text-area-hello".to_string()
                    label="Summary".to_string()
                    default_value="Ready for launch".to_string()
                />
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-value-control-mode`, `data-default-value-source`, `data-value-change-source`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-rows-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <TextArea
                        id="docs-text-area-markers".to_string()
                        label="Release notes".to_string()
                        value=Signal::derive(move || value.get())
                        default_value="Shipping notes".to_string()
                        on_value_change=on_value_change
                        is_required=Signal::derive(move || true)
                        is_invalid=Signal::derive(move || invalid.get())
                        description="Inspect source/state marker contracts".to_string()
                        error="Release notes are required".to_string()
                        placeholder="Write release notes…".to_string()
                        motion=TextAreaMotion::disabled()
                        rows=6
                        class_name="docs-text-area-state".to_string()
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                    >
                        {move || if invalid.get() { "Clear marker invalid" } else { "Mark marker invalid" }}
                    </ui_components::Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn search_field() -> AnyView {
    let (marker_value, set_marker_value) = signal("rust ui".to_string());
    let on_marker_value_change = Callback::new(move |next: String| set_marker_value.set(next));
    let (marker_invalid, set_marker_invalid) = signal(false);
    let (marker_read_only, set_marker_read_only) = signal(false);
    let (marker_disabled, set_marker_disabled) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<SearchField
  id="search".to_string()
  label="Search".to_string()
  default_value="rust ui".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (marker_value, set_marker_value) = signal("rust ui".to_string());
let on_value_change = Callback::new(move |next: String| set_marker_value.set(next));
let (marker_invalid, set_marker_invalid) = signal(false);
let (marker_read_only, set_marker_read_only) = signal(false);
let (marker_disabled, set_marker_disabled) = signal(false);

<SearchField
  id="docs-search-field-markers".to_string()
  label="Search".to_string()
  value=marker_value
  on_value_change=on_value_change
  default_value="rust ui".to_string()
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(move || marker_invalid.get())
  is_read_only=marker_read_only.get()
  is_disabled=marker_disabled.get()
  placeholder="Search docs…".to_string()
  class_name="docs-search-field-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="SearchField"
            slug="search-field"
            group="Forms"
            description="A search input built on typed state primitives + headless semantics, with explicit state/source markers."
        >
            <Playground
                title="Hello World"
                description="Minimal usage: id + label + default_value."
                code_signal=hello_code
            >
                <SearchField
                    id="docs-search-field-hello".to_string()
                    label="Search".to_string()
                    default_value="rust ui".to_string()
                />
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-ui-*`, and `data-value-*` while toggling invalid/read-only/disabled."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <SearchField
                        id="docs-search-field-markers".to_string()
                        label="Search".to_string()
                        value=marker_value
                        on_value_change=on_marker_value_change
                        default_value="rust ui".to_string()
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(move || marker_invalid.get())
                        is_read_only=marker_read_only.get()
                        is_disabled=marker_disabled.get()
                        placeholder="Search docs…".to_string()
                        class_name="docs-search-field-state".to_string()
                    />

                    <div class="docs-row" data-slot="search-field-marker-controls">
                        <div data-slot="search-field-toggle-invalid">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_invalid.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_invalid.get() {
                                    "Clear marker invalid"
                                } else {
                                    "Mark marker invalid"
                                }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="search-field-toggle-readonly">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_read_only.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_read_only.get() {
                                    "Set editable"
                                } else {
                                    "Set read only"
                                }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="search-field-toggle-disabled">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>

                    <span class="ui-muted" data-slot="search-field-marker-summary">
                        "value: " {move || marker_value.get()}
                        " · invalid: " {move || marker_invalid.get()}
                        " · read-only: " {move || marker_read_only.get()}
                        " · disabled: " {move || marker_disabled.get()}
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="search-field-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="search-field-api-rows">
                    <li>
                        <code>"id: String"</code>
                        " required"
                    </li>
                    <li>
                        <code>"label: String"</code>
                        " required"
                    </li>
                    <li>
                        <code>"value + on_value_change + default_value"</code>
                        " controlled/uncontrolled value axis"
                    </li>
                    <li>
                        <code>"is_disabled / is_read_only / is_required / is_invalid"</code>
                        " unified prefixed accessibility states"
                    </li>
                    <li>
                        <code>"on_submit / on_clear / clear_button_aria_label"</code>
                        " search action contract + clear-label source chain"
                    </li>
                    <li>
                        <code>"lang / dir"</code>
                        " locale passthrough to headless contract"
                    </li>
                    <li>
                        <code>"motion: SearchFieldMotion"</code>
                        " default = SearchFieldMotion::default()"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="search-field-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="search-field-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = ready | invalid | readonly | disabled"
                    </li>
                    <li>
                        <code>"data-value"</code>
                        " = empty | filled"
                    </li>
                    <li>
                        <code>"data-requirement"</code>
                        " = required | optional"
                    </li>
                    <li>
                        <code>"data-value-control-mode / data-default-value-source / data-value-change-source"</code>
                        " = controlled|uncontrolled + default|custom + on_value_change|none"
                    </li>
                    <li>
                        <code>"data-clear-label-source / data-class-source"</code>
                        " = prop|i18n|default and default|custom"
                    </li>
                    <li>
                        <code>"data-ui-schema / data-ui-intent / data-ui-action-model / data-ui-state-axis / data-ui-source-axis"</code>
                        " typed Agent Contract markers"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="search-field-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ui_components::Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<SearchField\n  id=\"search\".into()\n  label=\"Search\".into()\n  default_value=\"rust ui\".into()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-search-field-source-copy".to_string()
                />
                <ul data-slot="search-field-source-paths">
                    <li><code>"crates/ui-components/src/text_input/search_field/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_input/search_field/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_input/search_field/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_input/search_field/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/text_input/search_field/motion.rs"</code></li>
                    <li><code>"crates/ui-state-primitives/src/search_field.rs"</code></li>
                    <li><code>"crates/ui-headless/src/search_field.rs"</code></li>
                </ul>
                <ul data-slot="search-field-source-prerequisites">
                    <li><code>"component-search_field"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn number_field() -> AnyView {
    let (value, set_value) = signal(42_i64);
    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(42_i64);
<NumberField id="qty".to_string()
  label="Quantity".to_string()
  value=value
  set_value=set_value
  min=0
  max=100
/>"#
        .to_string()
    });

    let (required_value, set_required_value) = signal(7_i64);
    let required_flag: Signal<bool> = Signal::derive(|| true);

    let (invalid_value, set_invalid_value) = signal(120_i64);
    let invalid_flag: Signal<bool> = Signal::derive(|| true);

    let (disabled_value, set_disabled_value) = signal(18_i64);

    let states_code = Signal::derive(move || {
        r#"<NumberField id="qty-default".to_string() label="Default".to_string() value=value set_value=set_value min=0 max=100 />
<NumberField id="qty-required".to_string() label="Required".to_string() value=required_value set_value=set_required_value min=0 max=20 required=Signal::derive(|| true) description=Some("Required field".to_string()) />
<NumberField id="qty-invalid".to_string() label="Invalid".to_string() value=invalid_value set_value=set_invalid_value min=0 max=100 invalid=Signal::derive(|| true) error=Some("Out of range".to_string()) />
<NumberField id="qty-disabled".to_string() label="Disabled".to_string() value=disabled_value set_value=set_disabled_value min=0 max=100 disabled=true />"#.to_string()
    });

    let bounds_options = vec![
        "0..100".to_string(),
        "0..10".to_string(),
        "-20..20".to_string(),
    ];
    let (bounds_index, set_bounds_index) = signal(Some(1_usize));
    let workbench_min = Signal::derive(move || match bounds_index.get().unwrap_or(1) {
        1 => 0_i64,
        2 => -20_i64,
        _ => 0_i64,
    });
    let workbench_max = Signal::derive(move || match bounds_index.get().unwrap_or(1) {
        1 => 10_i64,
        2 => 20_i64,
        _ => 100_i64,
    });

    let step_options = vec!["1".to_string(), "5".to_string(), "10".to_string()];
    let (step_index, set_step_index) = signal(Some(0_usize));
    let workbench_step = Signal::derive(move || match step_index.get().unwrap_or(0) {
        1 => 5_i64,
        2 => 10_i64,
        _ => 1_i64,
    });

    let (workbench_value, set_workbench_value) = signal(12_i64);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_change = Callback::new(move |next: i64| {
        set_workbench_last_change.set(next.to_string());
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<NumberField".to_string(),
            "  id=\"docs-number-field-workbench\".into()".to_string(),
            "  label=\"Quantity\".into()".to_string(),
            "  value=value".to_string(),
            "  set_value=set_value".to_string(),
            format!("  min={}", workbench_min.get()),
            format!("  max={}", workbench_max.get()),
            format!("  step={}", workbench_step.get()),
        ];
        if workbench_disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if workbench_required_raw.get() {
            lines.push("  required=Signal::derive(|| true)".to_string());
            lines.push("  description=\"Required field\".into()".to_string());
        }
        if workbench_invalid_raw.get() {
            lines.push("  invalid=Signal::derive(|| true)".to_string());
            lines.push("  error=\"Out of range\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/text_input/number_field/styles.rs */\n{}",
            ui_components::text_input::number_field::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "NumberFieldWorkbenchConfig {{\n  value: {},\n  min: {:?},\n  max: {:?},\n  step: {},\n  disabled: {},\n  required: {},\n  invalid: {},\n  last_change: \"{}\",\n}}",
            workbench_value.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
            workbench_disabled.get(),
            workbench_required_raw.get(),
            workbench_invalid_raw.get(),
            workbench_last_change.get()
        )
    });

    view! {
        <ComponentPage
            title="NumberField"
            slug="number-field"
            group="Forms"
            description="Numeric input with steppers and keyboard control."
        >
            <Playground title="Stepper" code_signal=code>
                <div class="docs-row">
                    <NumberField
                        id="docs-number-field".to_string()
                        label="Quantity".to_string()
                        value=value
                        set_value=set_value
                        min=0
                        max=100
                    />
                    <span class="ui-muted">"value: " {move || value.get()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <NumberField
                            id="docs-number-field-default".to_string()
                            label="Default".to_string()
                            value=value
                            set_value=set_value
                            min=0
                            max=100
                        />
                        <NumberField
                            id="docs-number-field-required".to_string()
                            label="Required".to_string()
                            value=required_value
                            set_value=set_required_value
                            min=0
                            max=20
                            required=required_flag
                            description="Required field".to_string()
                        />
                    </div>
                    <div class="docs-row">
                        <NumberField
                            id="docs-number-field-invalid".to_string()
                            label="Invalid".to_string()
                            value=invalid_value
                            set_value=set_invalid_value
                            min=0
                            max=100
                            invalid=invalid_flag
                            error="Out of range".to_string()
                        />
                        <NumberField
                            id="docs-number-field-disabled".to_string()
                            label="Disabled".to_string()
                            value=disabled_value
                            set_value=set_disabled_value
                            min=0
                            max=100
                            disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test workbench for number-field semantics and stepping contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/text_input/number_field/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="number-field-workbench-controls">
                        <div class="docs-search__label">"Bounds"</div>
                        <SegmentedControl
                            id_base="docs-number-field-workbench-bounds".to_string()
                            options=bounds_options.clone()
                            selected_index=bounds_index
                            set_selected_index=set_bounds_index
                            size=SegmentedControlSize::Sm
                            aria_label="NumberField bounds".to_string()
                        />

                        <div class="docs-search__label">"Step"</div>
                        <SegmentedControl
                            id_base="docs-number-field-workbench-step".to_string()
                            options=step_options.clone()
                            selected_index=step_index
                            set_selected_index=set_step_index
                            size=SegmentedControlSize::Sm
                            aria_label="NumberField step".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>
                            "Invalid"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" style="width: min(100%, 360px);">
                    <NumberField
                        id="docs-number-field-workbench".to_string()
                        label="Quantity".to_string()
                        value=workbench_value
                        set_value=set_workbench_value
                        min=workbench_min.get()
                        max=workbench_max.get()
                        step=workbench_step.get()
                        disabled=workbench_disabled.get()
                        required=workbench_required
                        invalid=workbench_invalid
                        description=if workbench_required_raw.get() {
                            "Required field".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_invalid_raw.get() {
                            "Out of range".to_string()
                        } else {
                            String::new()
                        }
                        on_change=on_workbench_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || workbench_value.get()}
                        " | last on_change: "
                        {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn input_otp() -> AnyView {
    let (value, set_value) = signal(String::new());
    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(String::new());
<InputOtp id_base="otp".to_string()
  label="One-time code".to_string()
  value=value
  set_value=set_value
  length=6
/>"#
        .to_string()
    });

    let length_options = vec!["4".to_string(), "6".to_string(), "8".to_string()];
    let (workbench_length_index, set_workbench_length_index) = signal(Some(1_usize));
    let workbench_length =
        Signal::derive(move || match workbench_length_index.get().unwrap_or(1) {
            0 => 4_usize,
            2 => 8_usize,
            _ => 6_usize,
        });
    let (workbench_value, set_workbench_value) = signal(String::new());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_error, set_workbench_show_error) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_last_complete, set_workbench_last_complete) = signal("none".to_string());
    let on_workbench_complete =
        Callback::new(move |next: String| set_workbench_last_complete.set(next));

    let workbench_code = Signal::derive(move || {
        let length = workbench_length.get();
        let disabled_line = if workbench_disabled.get() {
            "  disabled=true\n"
        } else {
            ""
        };
        let required_line = if workbench_required.get() {
            "  required=Signal::derive(move || true)\n"
        } else {
            ""
        };
        let invalid_line = if workbench_invalid.get() {
            "  invalid=Signal::derive(move || true)\n"
        } else {
            ""
        };
        let description_line = if workbench_show_description.get() {
            "  description=\"We sent a code to your device.\".into()\n"
        } else {
            ""
        };
        let error_line = if workbench_show_error.get() {
            "  error=\"Code does not match.\".into()\n"
        } else {
            ""
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-input-otp-workbench\".into()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Verification code\".into()\n"
        } else {
            ""
        };

        format!(
            "let (value, set_value) = signal(String::new());\n\n<InputOtp\n  id_base=\"docs-otp-workbench\".into()\n  label=\"One-time code\".into()\n  value=value\n  set_value=set_value\n  length={length}\n{disabled_line}{required_line}{invalid_line}{description_line}{error_line}{class_line}{aria_line}/>"
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/text_input/input_otp/styles.rs */\n{}",
            ui_components::text_input::input_otp::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let length = workbench_length.get();
        let disabled = workbench_disabled.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_error = workbench_show_error.get();
        let custom_class = workbench_custom_class.get();
        let custom_aria = workbench_custom_aria.get();
        let value = workbench_value.get();

        format!(
            "InputOtpWorkbenchConfig {{\n  length: {length},\n  value: \"{value}\",\n  disabled: {disabled},\n  required: {required},\n  invalid: {invalid},\n  show_description: {show_description},\n  show_error: {show_error},\n  custom_class: {custom_class},\n  custom_aria_label: {custom_aria},\n}}"
        )
    });

    let (compare_default, set_compare_default) = signal(String::new());
    let (compare_disabled, set_compare_disabled) = signal("2468".to_string());
    let (compare_invalid, set_compare_invalid) = signal("12".to_string());
    let state_compare_code = Signal::derive(move || {
        r#"<InputOtp id_base="otp-default".to_string() value=default_value set_value=set_default_value length=6 />
<InputOtp id_base="otp-disabled".to_string() value=disabled_value set_value=set_disabled_value length=4 disabled=true />
<InputOtp
  id_base="otp-invalid".to_string()
  value=invalid_value
  set_value=set_invalid_value
  length=6
  invalid=Signal::derive(move || true)
  error="Code does not match.".to_string()
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="InputOtp"
            slug="input-otp"
            group="Forms"
            description="baseline-style OTP input with a single hidden input and slot chrome."
        >
            <Playground title="OTP" code_signal=code>
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

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-components/src/text_input/input_otp/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Length"</div>
                        <SegmentedControl
                            id_base="docs-input-otp-workbench-length".to_string()
                            options=length_options.clone()
                            selected_index=workbench_length_index
                            set_selected_index=set_workbench_length_index
                            size=SegmentedControlSize::Sm
                            aria_label="InputOtp workbench length".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </Switch>
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "Show description"
                        </Switch>
                        <Switch checked=workbench_show_error set_checked=set_workbench_show_error>
                            "Show error"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="input-otp-workbench">
                    <InputOtp
                        id_base="docs-otp-workbench".to_string()
                        label="One-time code".to_string()
                        value=workbench_value
                        set_value=set_workbench_value
                        length=workbench_length.get()
                        disabled=workbench_disabled.get()
                        required=workbench_required
                        invalid=workbench_invalid
                        description=if workbench_show_description.get() {
                            "We sent a code to your device.".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_show_error.get() {
                            "Code does not match.".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-input-otp-workbench".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Verification code".to_string()
                        } else {
                            String::new()
                        }
                        on_complete=on_workbench_complete
                    />
                    <span class="ui-muted">"value: " {move || workbench_value.get()}</span>
                    <span class="ui-muted">
                        "last complete: "
                        {move || workbench_last_complete.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Comparison" code_signal=state_compare_code>
                <div class="docs-stack docs-stack--tight" data-slot="input-otp-state-compare">
                    <div class="ui-muted">"Default"</div>
                    <InputOtp
                        id_base="docs-otp-compare-default".to_string()
                        label="Default OTP".to_string()
                        value=compare_default
                        set_value=set_compare_default
                        length=6
                    />
                    <span class="ui-muted">"value: " {move || compare_default.get()}</span>

                    <div class="ui-muted">"Disabled (prefilled)"</div>
                    <InputOtp
                        id_base="docs-otp-compare-disabled".to_string()
                        label="Disabled OTP".to_string()
                        value=compare_disabled
                        set_value=set_compare_disabled
                        length=4
                        disabled=true
                    />

                    <div class="ui-muted">"Invalid + error"</div>
                    <InputOtp
                        id_base="docs-otp-compare-invalid".to_string()
                        label="Invalid OTP".to_string()
                        value=compare_invalid
                        set_value=set_compare_invalid
                        length=6
                        invalid=Signal::derive(move || true)
                        error="Code does not match.".to_string()
                    />
                    <span class="ui-muted">"value: " {move || compare_invalid.get()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn checkbox() -> AnyView {
    let (checked, set_checked) = signal(false);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_accept_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let (marketing, set_marketing) = signal(true);
    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);
    let (interactive_checked, set_interactive_checked) = signal(true);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let variant_options = vec!["Default".to_string(), "Accent".to_string()];
    let size_options = vec!["Default".to_string(), "Sm".to_string(), "Lg".to_string()];
    let (interactive_variant_index, set_interactive_variant_index) = signal(Some(0_usize));
    let (interactive_size_index, set_interactive_size_index) = signal(Some(0_usize));
    let interactive_variant =
        Signal::derive(move || match interactive_variant_index.get().unwrap_or(0) {
            1 => CheckboxVariant::Accent,
            _ => CheckboxVariant::Default,
        });
    let interactive_size =
        Signal::derive(move || match interactive_size_index.get().unwrap_or(0) {
            1 => CheckboxSize::Sm,
            2 => CheckboxSize::Lg,
            _ => CheckboxSize::Default,
        });

    let interactive_code = Signal::derive(move || {
        let mut lines = vec![
            "let (checked, set_checked) = signal(true);".to_string(),
            "".to_string(),
            "<Checkbox".to_string(),
            "  checked=checked".to_string(),
            "  set_checked=set_checked".to_string(),
        ];

        if interactive_variant.get() != CheckboxVariant::Default {
            lines.push(format!(
                "  variant=CheckboxVariant::{:?}",
                interactive_variant.get()
            ));
        }
        if interactive_size.get() != CheckboxSize::Default {
            lines.push(format!("  size=CheckboxSize::{:?}", interactive_size.get()));
        }
        if interactive_disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if interactive_custom_class.get() {
            lines.push("  class_name=\"docs-checkbox-custom\".into()".to_string());
        }

        lines.push(">".to_string());
        lines.push("  \"Interactive consent\"".to_string());
        lines.push("</Checkbox>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/checkbox/styles.rs */\n{}",
            ui_components::checkbox::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxActualConfig {{\n  checked: {},\n  disabled: {},\n  variant: {:?},\n  size: {:?},\n  class_name: {},\n}}",
            interactive_checked.get(),
            interactive_disabled.get(),
            interactive_variant.get(),
            interactive_size.get(),
            if interactive_custom_class.get() {
                "\"docs-checkbox-custom\""
            } else {
                "None"
            }
        )
    });

    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);

<Checkbox
  checked=checked
  set_checked=set_checked
  on_change=Callback::new(move |_| {})
>
  "Accept terms"
</Checkbox>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (marketing, set_marketing) = signal(true);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Checkbox
  checked=marketing
  set_checked=set_marketing
  variant=CheckboxVariant::Accent
  size=CheckboxSize::Lg
>
  "Email updates"
</Checkbox>
<Checkbox checked=disabled_checked set_checked=set_disabled_checked disabled=true>
  "Disabled on"
</Checkbox>
<Checkbox checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>
  "Disabled off"
</Checkbox>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Checkbox"
            slug="checkbox"
            group="Forms"
            description="Pressable checkbox with baseline-level spring indicator and baseline-style root state attrs."
        >
            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit checkbox props and inspect actual state contracts."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui-components/src/checkbox/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-variant".to_string()
                            options=variant_options.clone()
                            selected_index=interactive_variant_index
                            set_selected_index=set_interactive_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Checkbox variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-size".to_string()
                            options=size_options.clone()
                            selected_index=interactive_size_index
                            set_selected_index=set_interactive_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Checkbox size".to_string()
                        />

                        <Switch checked=interactive_checked set_checked=set_interactive_checked>
                            "Checked"
                        </Switch>
                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Checkbox
                        checked=interactive_checked
                        set_checked=set_interactive_checked
                        variant=interactive_variant.get()
                        size=interactive_size.get()
                        disabled=interactive_disabled.get()
                        class_name=if interactive_custom_class.get() {
                            "docs-checkbox-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        "Interactive consent"
                    </Checkbox>
                    <span class="ui-muted">
                        "checked: " {move || interactive_checked.get()}
                        " · disabled: " {move || interactive_disabled.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + on_change" code_signal=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Checkbox
                            checked=checked
                            set_checked=set_checked
                            on_change=on_accept_change
                        >
                            "Accept terms"
                        </Checkbox>
                        <span class="ui-muted">"checked: " {move || checked.get()}</span>
                    </div>
                    <span class="ui-muted">"last on_change: " {move || last_change.get()}</span>
                </div>
            </Playground>

            <Playground title="Variant + Disabled matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Checkbox
                            checked=marketing
                            set_checked=set_marketing
                            variant=CheckboxVariant::Accent
                            size=CheckboxSize::Lg
                        >
                            "Email updates"
                        </Checkbox>
                        <span class="ui-muted">
                            "marketing: "
                            {move || marketing.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Checkbox
                            checked=disabled_checked
                            set_checked=set_disabled_checked
                            disabled=true
                        >
                            "Disabled on"
                        </Checkbox>
                        <Checkbox
                            checked=disabled_unchecked
                            set_checked=set_disabled_unchecked
                            disabled=true
                        >
                            "Disabled off"
                        </Checkbox>
                    </div>
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
    let (interactive_alpha, set_interactive_alpha) = signal(true);
    let (interactive_beta, set_interactive_beta) = signal(false);
    let (interactive_required, set_interactive_required) = signal(true);
    let (interactive_invalid, set_interactive_invalid) = signal(false);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_description, set_interactive_description) = signal(true);
    let (interactive_error, set_interactive_error) = signal(true);

    let code = Signal::derive(move || {
        r#"let invalid = Signal::derive(move || !(apple.get() || banana.get()));
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
</CheckboxGroup>"#
            .to_string()
    });

    let interactive_code = Signal::derive(move || {
        let mut lines = vec![
            "let (alpha, set_alpha) = signal(true);".to_string(),
            "let (beta, set_beta) = signal(false);".to_string(),
            "".to_string(),
            "<CheckboxGroup".to_string(),
            "  id=\"docs-checkbox-group-interactive\".into()".to_string(),
            "  label=\"Release channels\".into()".to_string(),
        ];

        if interactive_description.get() {
            lines.push("  description=\"Choose at least one channel.\".into()".to_string());
        }
        if interactive_error.get() {
            lines.push("  error=\"At least one channel is required.\".into()".to_string());
        }
        lines.push(format!(
            "  required=Signal::derive(|| {})",
            interactive_required.get()
        ));
        lines.push(format!(
            "  invalid=Signal::derive(|| {})",
            interactive_invalid.get()
        ));
        if interactive_disabled.get() {
            lines.push("  disabled=true".to_string());
        }

        lines.push(">".to_string());
        lines.push(
            "  <Checkbox checked=alpha set_checked=set_alpha>\"Email\"</Checkbox>".to_string(),
        );
        lines.push("  <Checkbox checked=beta set_checked=set_beta>\"SMS\"</Checkbox>".to_string());
        lines.push("</CheckboxGroup>".to_string());

        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/checkbox/styles.rs */\n{}\n\n/* checkbox group */\n{}",
            ui_components::checkbox::styles::CSS,
            ui_components::checkbox::styles::CHECKBOX_GROUP_CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxGroupActualConfig {{\n  required: {},\n  invalid: {},\n  disabled: {},\n  description: {},\n  error: {},\n  alpha: {},\n  beta: {},\n}}",
            interactive_required.get(),
            interactive_invalid.get(),
            interactive_disabled.get(),
            if interactive_description.get() {
                "present"
            } else {
                "absent"
            },
            if interactive_error.get() {
                "present"
            } else {
                "absent"
            },
            interactive_alpha.get(),
            interactive_beta.get()
        )
    });

    let states_code = Signal::derive(move || {
        r#"<CheckboxGroup
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
</CheckboxGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="CheckboxGroup"
            slug="checkbox-group"
            group="Forms"
            description="Fieldset wrapper with normalized labels, validation semantics, and baseline-style root state attrs."
        >
            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit group validation/required state and inspect contracts."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui-components/src/checkbox/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=interactive_required set_checked=set_interactive_required>
                            "Required"
                        </Switch>
                        <Switch checked=interactive_invalid set_checked=set_interactive_invalid>
                            "Invalid"
                        </Switch>
                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=interactive_description set_checked=set_interactive_description>
                            "Description"
                        </Switch>
                        <Switch checked=interactive_error set_checked=set_interactive_error>
                            "Error message"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let description = if interactive_description.get() {
                        "Choose at least one channel.".to_string()
                    } else {
                        String::new()
                    };
                    let error = if interactive_error.get() {
                        "At least one channel is required.".to_string()
                    } else {
                        String::new()
                    };
                    let required = Signal::derive(move || interactive_required.get());
                    let invalid = Signal::derive(move || interactive_invalid.get());
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <CheckboxGroup
                                id="docs-checkbox-group-interactive".to_string()
                                label="Release channels".to_string()
                                description=description
                                error=error
                                required=required
                                invalid=invalid
                                disabled=interactive_disabled.get()
                            >
                                <Checkbox checked=interactive_alpha set_checked=set_interactive_alpha>
                                    "Email"
                                </Checkbox>
                                <Checkbox checked=interactive_beta set_checked=set_interactive_beta>
                                    "SMS"
                                </Checkbox>
                            </CheckboxGroup>
                            <span class="ui-muted">
                                "selected count: "
                                {move || {
                                    (usize::from(interactive_alpha.get()) + usize::from(interactive_beta.get()))
                                        .to_string()
                                }}
                            </span>
                        </div>
                    }
                        .into_any()
                }}
            </Playground>

            <Playground title="Validation + Required" code_signal=code>
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
                        {move || invalid.get()}
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

            <Playground title="Disabled + Optional" code_signal=states_code>
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
                            {move || optional_selected_count.get()}
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

    let (system_enabled, set_system_enabled) = signal(true);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_system_checked_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);

    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(true);

<Switch
  checked=checked
  set_checked=set_checked
  on_checked_change=Callback::new(move |_| {})
>
  "Notifications"
</Switch>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (system_enabled, set_system_enabled) = signal(true);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Switch checked=system_enabled set_checked=set_system_enabled>
  "System alerts"
</Switch>
<Switch checked=disabled_checked set_checked=set_disabled_checked disabled=true>
  "Disabled on"
</Switch>
<Switch checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>
  "Disabled off"
</Switch>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Switch"
            slug="switch"
            group="Forms"
            description="Switch toggle with baseline-level spring thumb motion and baseline-style root state attrs."
        >
            <Playground title="Controlled + on_checked_change" code_signal=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Switch
                            checked=checked
                            set_checked=set_checked
                            on_checked_change=on_system_checked_change
                        >
                            "Notifications"
                        </Switch>
                        <span class="ui-muted">"checked: " {move || checked.get()}</span>
                    </div>
                    <span class="ui-muted">
                        "last on_checked_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Switch checked=system_enabled set_checked=set_system_enabled>
                            "System alerts"
                        </Switch>
                        <span class="ui-muted">
                            "system enabled: "
                            {move || system_enabled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Switch checked=disabled_checked set_checked=set_disabled_checked disabled=true>
                            "Disabled on"
                        </Switch>
                        <Switch checked=disabled_unchecked set_checked=set_disabled_unchecked disabled=true>
                            "Disabled off"
                        </Switch>
                    </div>
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
    let has_selection = Signal::derive(move || selected.get().is_some());

    let billing_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (billing_selected, set_billing_selected) = signal(Some(2_usize));
    let external_label_id = "docs-radio-group-billing-label".to_string();
    let (billing_is_horizontal, set_billing_is_horizontal) = signal(true);
    let (billing_group_disabled, set_billing_group_disabled) = signal(false);
    let (billing_disable_middle, set_billing_disable_middle) = signal(true);

    let empty_options = Vec::<String>::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));

<RadioGroup
  id_base="size".to_string()
  options=vec![
    "Small".to_string(),
    "Medium".to_string(),
    "Large".to_string(),
  ]
  label="Size".to_string()
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"let (billing_selected, set_billing_selected) = signal(Some(2_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);
let (is_horizontal, set_is_horizontal) = signal(true);
let (is_group_disabled, set_is_group_disabled) = signal(false);
let (disable_middle, set_disable_middle) = signal(true);

let orientation = Signal::derive(move || {
  if is_horizontal.get() {
    RadioGroupOrientation::Horizontal
  } else {
    RadioGroupOrientation::Vertical
  }
});
let disabled_indices = Signal::derive(move || if disable_middle.get() { vec![1] } else { Vec::new() });

<RadioGroup
  id_base="billing".to_string()
  options=vec![
    "Monthly".to_string(),
    "Quarterly".to_string(),
    "Yearly".to_string(),
  ]
  orientation=orientation.get()
  is_disabled=is_group_disabled.get()
  disabled_indices=disabled_indices.get()
  aria_labelledby="docs-radio-group-billing-label".to_string()
  selected_index=billing_selected
  set_selected_index=set_billing_selected
/>
<RadioGroup
  id_base="empty".to_string()
  options=Vec::<String>::new()
  is_disabled=true
  aria_label="No options available".to_string()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="RadioGroup"
            slug="radio-group"
            group="Forms"
            description="Roving tabindex radiogroup with baseline-level spring motion and baseline-style root state attrs."
        >
            <Playground title="Hello World（默认路径）" code_signal=code>
                <div class="docs-stack">
                    <RadioGroup
                        id_base="docs-radio-group".to_string()
                        options=options
                        label="Size".to_string()
                        selected_index=selected
                        set_selected_index=set_selected
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Interactive Matrix（方向/禁用/状态）" code_signal=matrix_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <button
                            class="ui-button"
                            type="button"
                            on:click=move |_| {
                                set_billing_is_horizontal.update(|value| *value = !*value)
                            }
                        >
                            {move || {
                                if billing_is_horizontal.get() {
                                    "Orientation: Horizontal"
                                } else {
                                    "Orientation: Vertical"
                                }
                            }}
                        </button>
                        <button
                            class="ui-button"
                            type="button"
                            on:click=move |_| {
                                set_billing_group_disabled.update(|value| *value = !*value)
                            }
                        >
                            {move || {
                                if billing_group_disabled.get() {
                                    "Group: Disabled"
                                } else {
                                    "Group: Enabled"
                                }
                            }}
                        </button>
                        <button
                            class="ui-button"
                            type="button"
                            on:click=move |_| {
                                set_billing_disable_middle.update(|value| *value = !*value)
                            }
                        >
                            {move || {
                                if billing_disable_middle.get() {
                                    "Middle option: Disabled"
                                } else {
                                    "Middle option: Enabled"
                                }
                            }}
                        </button>
                    </div>
                    <div id=external_label_id.clone() class="ui-muted">"Billing cycle"</div>
                    {move || {
                        let orientation = if billing_is_horizontal.get() {
                            RadioGroupOrientation::Horizontal
                        } else {
                            RadioGroupOrientation::Vertical
                        };
                        let disabled_indices = if billing_disable_middle.get() {
                            vec![1_usize]
                        } else {
                            Vec::new()
                        };
                        let is_disabled = billing_group_disabled.get();
                        view! {
                            <RadioGroup
                                id_base="docs-radio-group-billing".to_string()
                                options=billing_options.clone()
                                orientation=orientation
                                is_disabled=is_disabled
                                disabled_indices=disabled_indices
                                aria_labelledby=external_label_id.clone()
                                selected_index=billing_selected
                                set_selected_index=set_billing_selected
                            />
                        }
                    }}
                    <span class="ui-muted">
                        "billing: "
                        {move || billing_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · orientation: "
                        {move || if billing_is_horizontal.get() { "horizontal" } else { "vertical" }}
                        " · group disabled: "
                        {move || billing_group_disabled.get()}
                        " · disabled options: "
                        {move || if billing_disable_middle.get() { "1" } else { "0" }}
                    </span>

                    <RadioGroup
                        id_base="docs-radio-group-empty".to_string()
                        options=empty_options
                        is_disabled=true
                        aria_label="No options available".to_string()
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                    />
                    <span class="ui-muted">
                        "empty selected: "
                        {move || empty_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn radio() -> AnyView {
    let (checked, set_checked) = signal(false);
    let on_checked_change = Callback::new(move |next: bool| set_checked.set(next));
    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);
    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);

<Radio
  id="r1".to_string()
  label="Standalone".to_string()
  is_checked=Signal::derive(move || checked.get())
  on_checked_change=Callback::new(move |next: bool| set_checked.set(next))
/>"#
        .to_string()
    });
    let matrix_code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Radio
  id="r1".to_string()
  label="Standalone".to_string()
  is_checked=Signal::derive(move || checked.get())
  on_checked_change=Callback::new(move |next: bool| set_checked.set(next))
/>
<Radio
  id="r2".to_string()
  label="Disabled on".to_string()
  is_checked=Signal::derive(move || disabled_checked.get())
  on_checked_change=Callback::new(move |next: bool| set_disabled_checked.set(next))
  is_disabled=true
/>
<Radio
  id="r3".to_string()
  label="Disabled off".to_string()
  is_checked=Signal::derive(move || disabled_unchecked.get())
  on_checked_change=Callback::new(move |next: bool| set_disabled_unchecked.set(next))
  is_disabled=true
/>
<Radio
  id="r4".to_string()
  label="Uncontrolled default on".to_string()
  default_checked=true
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Radio"
            slug="radio"
            group="Forms"
            description="Standalone radio button (use RadioGroup for semantics)."
        >
            <Playground title="Hello World（默认路径）" code_signal=code>
                <div class="docs-row">
                    <Radio
                        id="docs-radio".to_string()
                        label="Standalone".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        on_checked_change=on_checked_change
                    />
                    <span class="ui-muted">"checked: " {move || checked.get()}</span>
                </div>
            </Playground>

            <Playground title="状态矩阵（受控 + disabled）" code_signal=matrix_code>
                <div class="docs-row">
                    <Radio
                        id="docs-radio-controlled".to_string()
                        label="Controlled".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        on_checked_change=on_checked_change
                    />
                    <Radio
                        id="docs-radio-disabled-on".to_string()
                        label="Disabled on".to_string()
                        is_checked=Signal::derive(move || disabled_checked.get())
                        on_checked_change=Callback::new(move |next: bool| set_disabled_checked.set(next))
                        is_disabled=true
                    />
                    <Radio
                        id="docs-radio-disabled-off".to_string()
                        label="Disabled off".to_string()
                        is_checked=Signal::derive(move || disabled_unchecked.get())
                        on_checked_change=Callback::new(move |next: bool| set_disabled_unchecked.set(next))
                        is_disabled=true
                    />
                    <Radio
                        id="docs-radio-uncontrolled-default".to_string()
                        label="Uncontrolled default on".to_string()
                        default_checked=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn segmented_control() -> AnyView {
    let hello_options = vec!["Overview".to_string(), "Details".to_string()];
    let (hello_selected, set_hello_selected) = signal(Some(0_usize));
    let options = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Settings".to_string(),
    ];
    let (selected, set_selected) = signal(Some(0_usize));
    let has_selection = Signal::derive(move || selected.get().is_some());

    let vertical_options = vec![
        "System".to_string(),
        "Manual".to_string(),
        "Hybrid".to_string(),
    ];
    let vertical_disabled_indices = vec![2_usize];
    let (vertical_selected, set_vertical_selected) = signal(Some(1_usize));

    let empty_options = Vec::<String>::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let interactive_options = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Settings".to_string(),
    ];
    let (interactive_selected, set_interactive_selected) = signal(Some(0_usize));
    let (interactive_vertical, set_interactive_vertical) = signal(false);
    let (interactive_small, set_interactive_small) = signal(false);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_disable_last, set_interactive_disable_last) = signal(true);

    let hello_code = Signal::derive(move || {
        r#"let (value, set_value) = signal(Some(0_usize));
<SegmentedControl id_base="seg".to_string() options=vec!["Overview".to_string(), "Details".to_string()] selected_index=value set_selected_index=set_value />"#.to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));

<SegmentedControl
  id_base="seg".to_string()
  options=vec![
    "Overview".to_string(),
    "Details".to_string(),
    "Settings".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![2]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (vertical_selected, set_vertical_selected) = signal(Some(1_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<SegmentedControl
  id_base="seg-vertical".to_string()
  options=vec![
    "System".to_string(),
    "Manual".to_string(),
    "Hybrid".to_string(),
  ]
  selected_index=vertical_selected
  set_selected_index=set_vertical_selected
  orientation=SegmentedControlOrientation::Vertical
  size=SegmentedControlSize::Sm
  disabled_indices=vec![2]
/>
<SegmentedControl
  id_base="seg-empty".to_string()
  options=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  disabled=true
  aria_label="No options".to_string()
/>"#
        .to_string()
    });

    let interactive_code = Signal::derive(move || {
        r#"let (value, set_value) = signal(Some(0_usize));
let (is_vertical, set_is_vertical) = signal(false);
let (is_small, set_is_small) = signal(false);

<SegmentedControl
  id_base="seg-interactive".to_string()
  options=vec!["Overview".to_string(), "Details".to_string(), "Settings".to_string()]
  selected_index=value
  set_selected_index=set_value
  orientation=if is_vertical.get() { SegmentedControlOrientation::Vertical } else { SegmentedControlOrientation::Horizontal }
  size=if is_small.get() { SegmentedControlSize::Sm } else { SegmentedControlSize::Default }
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="SegmentedControl"
            slug="segmented-control"
            group="Forms"
            description="Segmented control with baseline-level indicator motion and baseline-style root state attrs."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <SegmentedControl
                    id_base="docs-segments-hello".to_string()
                    options=hello_options
                    selected_index=hello_selected
                    set_selected_index=set_hello_selected
                />
            </Playground>

            <Playground title="Selection + Root State" code_signal=code>
                <div class="docs-stack">
                    <SegmentedControl
                        id_base="docs-segments".to_string()
                        options=options
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![2]
                        size=SegmentedControlSize::Default
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get()}
                        " · disabled options: 1"
                    </span>
                </div>
            </Playground>

            <Playground title="Interactive Playground (Props + State)"
                description="Toggle orientation/size/disabled rules and observe semantic markers."
                code_signal=interactive_code>
                <div class="docs-stack">
                    <SegmentedControl
                        id_base="docs-segments-interactive".to_string()
                        options=interactive_options
                        selected_index=interactive_selected
                        set_selected_index=set_interactive_selected
                        orientation=if interactive_vertical.get() {
                            SegmentedControlOrientation::Vertical
                        } else {
                            SegmentedControlOrientation::Horizontal
                        }
                        size=if interactive_small.get() {
                            SegmentedControlSize::Sm
                        } else {
                            SegmentedControlSize::Default
                        }
                        disabled=interactive_disabled.get()
                        disabled_indices=if interactive_disable_last.get() {
                            vec![2_usize]
                        } else {
                            Vec::new()
                        }
                    />
                    <div class="docs-row" data-slot="segmented-control-marker-controls">
                        <div data-slot="segmented-control-toggle-orientation">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_interactive_vertical.update(|value| *value = !*value);
                                })
                            >
                                {move || if interactive_vertical.get() {
                                    "Orientation vertical"
                                } else {
                                    "Orientation horizontal"
                                }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="segmented-control-toggle-size">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_interactive_small.update(|value| *value = !*value);
                                })
                            >
                                {move || if interactive_small.get() { "Size sm" } else { "Size default" }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="segmented-control-toggle-disabled">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_interactive_disabled.update(|value| *value = !*value);
                                })
                            >
                                {move || if interactive_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="segmented-control-toggle-disabled-last">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_interactive_disable_last.update(|value| *value = !*value);
                                })
                            >
                                {move || if interactive_disable_last.get() {
                                    "Disable last on"
                                } else {
                                    "Disable last off"
                                }}
                            </ui_components::Button>
                        </div>
                        <div data-slot="segmented-control-reset-selection">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_interactive_selected.set(Some(0_usize));
                                })
                            >
                                "Reset selection"
                            </ui_components::Button>
                        </div>
                    </div>
                    <span class="ui-muted" data-slot="segmented-control-marker-summary">
                        "selected: "
                        {move || interactive_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · vertical: " {move || interactive_vertical.get()}
                        " · size: " {move || if interactive_small.get() { "sm" } else { "default" }}
                        " · disabled: " {move || interactive_disabled.get()}
                        " · disable last: " {move || interactive_disable_last.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Vertical + Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <SegmentedControl
                            id_base="docs-segments-vertical".to_string()
                            options=vertical_options
                            selected_index=vertical_selected
                            set_selected_index=set_vertical_selected
                            orientation=SegmentedControlOrientation::Vertical
                            size=SegmentedControlSize::Sm
                            disabled_indices=vertical_disabled_indices
                        />
                        <span class="ui-muted">
                            "vertical selected: "
                            {move || vertical_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <SegmentedControl
                            id_base="docs-segments-empty".to_string()
                            options=empty_options
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            disabled=true
                            aria_label="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="segmented-control-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ui_components::Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\nlet (value, set_value) = signal(Some(0_usize));\n<SegmentedControl id_base=\"seg\".into() options=vec![\"Overview\".into(), \"Details\".into()] selected_index=value set_selected_index=set_value />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-segmented-control-source-copy".to_string()
                />
                <ul data-slot="segmented-control-source-paths">
                    <li><code>"crates/ui-components/src/segmented_control/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/segmented_control/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/segmented_control/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/segmented_control/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/segmented_control/motion.rs"</code></li>
                </ul>
                <ul data-slot="segmented-control-source-prerequisites">
                    <li><code>"component-segmented_control"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
