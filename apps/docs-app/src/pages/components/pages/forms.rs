use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::text_input::text_area::TextAreaMotion;
use ui::{
    Checkbox, CheckboxGroup, CheckboxSize, CheckboxVariant, Form, FormLabelAlign,
    FormLabelPosition, Input, InputGroup, InputOtp, InputSize, InputVariant, NumberField, Radio,
    RadioGroup, RadioGroupOrientation, SearchField, SearchFieldMotion, SegmentedControl,
    SegmentedControlMotion, SegmentedControlOrientation, SegmentedControlSize, Switch, TextArea,
};
use ui_headless::A11yDirection;

pub(super) fn input_group() -> AnyView {
    let (email_user, set_email_user) = signal(String::new());
    let (search_query, set_search_query) = signal(String::new());
    let (workbench_attached, set_workbench_attached) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_start, set_workbench_show_start) = signal(true);
    let (workbench_show_end, set_workbench_show_end) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<InputGroup\n  attached={}\n  disabled={}\n  invalid={}\n  aria_label={}\n  start_content={}\n  end_content={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  <Input ... />\n</InputGroup>",
            workbench_attached.get(),
            workbench_disabled.get(),
            workbench_invalid.get(),
            if workbench_custom_aria.get() {
                "\"Search controls\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_show_start.get() {
                "Some(ViewFn)"
            } else {
                "None"
            },
            if workbench_show_end.get() {
                "Some(ViewFn)"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "\"docs-input-group-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_zh_lang.get() {
                "\"zh-CN\".to_string()"
            } else {
                "\"en-US\".to_string()"
            },
            if workbench_rtl_dir.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "InputGroupActualConfig {{\n  attached: {},\n  disabled: {},\n  invalid: {},\n  aria_label: {},\n  start_content: {},\n  end_content: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n}}",
            workbench_attached.get(),
            workbench_disabled.get(),
            workbench_invalid.get(),
            if workbench_custom_aria.get() {
                "Some(\"Search controls\")"
            } else {
                "None"
            },
            if workbench_show_start.get() {
                "Some(\"addon-start\")"
            } else {
                "None"
            },
            if workbench_show_end.get() {
                "Some(\"addon-end\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-input-group-custom\")"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<InputGroup attached=true disabled=false invalid=false />
<InputGroup attached=false invalid=true start_content=Some(ViewFn) end_content=Some(ViewFn) />
<InputGroup attached=true disabled=true class_name="docs-input-group-custom".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl />"#.to_string()
    });

    view! {
        <ComponentPage
            title="InputGroup"
            slug="input-group"
            group="Forms"
            description="Composes one or more inputs with shared prefix/suffix addons and baseline-style state contracts."
        >
            <Playground
                title="Hello World (Default Input Group)"
                code_signal=Signal::derive(move || {
                    r#"<InputGroup attached=true aria_label="Email input group".to_string()>
  <Input ... />
</InputGroup>"#
                        .to_string()
                })
            >
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="input-group-workbench-controls">
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_attached.get() on:change=move |ev| set_workbench_attached.set(event_target_checked(&ev)) />
                            " attached"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_disabled.get() on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev)) />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_invalid.get() on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev)) />
                            " invalid"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_show_start.get() on:change=move |ev| set_workbench_show_start.set(event_target_checked(&ev)) />
                            " start_content"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_show_end.get() on:change=move |ev| set_workbench_show_end.set(event_target_checked(&ev)) />
                            " end_content"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_aria.get() on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev)) />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_class.get() on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev)) />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_zh_lang.get() on:change=move |ev| set_workbench_zh_lang.set(event_target_checked(&ev)) />
                            " lang zh-CN"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_rtl_dir.get() on:change=move |ev| set_workbench_rtl_dir.set(event_target_checked(&ev)) />
                            " dir RTL"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <InputGroup
                        attached=workbench_attached.get()
                        disabled=workbench_disabled.get()
                        invalid=workbench_invalid.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Search controls".to_string()
                        } else {
                            String::new()
                        }
                        start_content=move || {
                            view! { <Show when=move || workbench_show_start.get()><span>"🔍"</span></Show> }
                        }
                        end_content=move || {
                            view! { <Show when=move || workbench_show_end.get()><span>"⌘K"</span></Show> }
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-input-group-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_zh_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    >
                        <Input
                            id="docs-input-group-workbench".to_string()
                            value=search_query
                            set_value=set_search_query
                            aria_label="Search query".to_string()
                            placeholder="Search docs".to_string()
                            label_hidden=true
                        />
                    </InputGroup>
                </div>
            </Playground>

            <Playground title="State Matrix (Attached / Invalid / Disabled)" code_signal=matrix_code>
                <div class="docs-stack">
                    <InputGroup attached=true aria_label="Default controls".to_string()>
                        <Input
                            id="docs-input-group-matrix-default".to_string()
                            value=email_user
                            set_value=set_email_user
                            aria_label="Default field".to_string()
                            placeholder="Default".to_string()
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

                    <InputGroup
                        attached=false
                        invalid=true
                        start_content=move || view! { <span>"!"</span> }
                        end_content=move || view! { <span>"required"</span> }
                        class_name="docs-input-group-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <Input
                            id="docs-input-group-invalid".to_string()
                            value=search_query
                            set_value=set_search_query
                            aria_label="Invalid field".to_string()
                            placeholder="Invalid".to_string()
                            label_hidden=true
                        />
                    </InputGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn form() -> AnyView {
    let (hello_name, set_hello_name) = signal(String::new());
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
    let (workbench_is_required, set_workbench_is_required) = signal(false);
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
        lines.push("  lang=\"en-US\".into()".to_string());
        lines.push("  dir=Some(A11yDirection::Ltr)".to_string());
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
            "FormActualConfig {{\n  is_required: {is_required},\n  is_disabled: {is_disabled},\n  is_read_only: {is_read_only},\n  label_position: {label_position:?},\n  label_align: {label_align:?},\n  class_name: {},\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  class: \"{class}\",\n  marker_expectations: [\"data-disabled\", \"data-readonly\", \"data-required\", \"data-label-position\", \"data-label-align\", \"data-ui-stream-mode=snapshot\", \"data-ui-streaming-policy=optional\", \"data-ui-streaming-fallback=snapshot\", \"data-ui-output-status=verified\"],\n}}",
            if custom_class {
                "Some(\"docs-form-custom\")"
            } else {
                "None"
            },
        )
    });

    let form_test_css_source = Signal::derive(move || {
        format!(
            "/* components/form/src/styles.rs */\n{}",
            ui::field_form::form::styles::CSS
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

    let hello_code = Signal::derive(move || {
        r#"<Form>
  <Input id="docs-form-hello".to_string() label="Name".to_string() value=hello_name set_value=set_hello_name />
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
                title="Hello World（默认路径）"
                code_signal=hello_code
                description="最小可用路径：直接 `<Form>` 包裹字段，不要求手动接线状态原语。"
            >
                <Form>
                    <Input
                        id="docs-form-hello".to_string()
                        label="Name".to_string()
                        value=hello_name
                        set_value=set_hello_name
                        placeholder="Jane".to_string()
                        size=InputSize::Md
                        variant=InputVariant::Bordered
                    />
                </Form>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=form_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/form/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 label-position/align/required/disabled/read-only/class，并在同一面板查看 code + config + scoped css test（含 streaming/snapshot 语义标记基线）。"
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
                                lang="en-US".to_string()
                                dir=A11yDirection::Ltr
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
                description="状态矩阵覆盖默认/必填/禁用/只读；`Form` 无 value 状态轴，受控/非受控对照在该组件按 N/A 处理。"
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
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_clearable, set_workbench_clearable) = signal(true);
    let (workbench_label_hidden, set_workbench_label_hidden) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_label_placement_index, set_workbench_label_placement_index) =
        signal(Some(0_usize));
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let workbench_variant_options = vec![
        "bordered".to_string(),
        "flat".to_string(),
        "underlined".to_string(),
    ];
    let workbench_label_placement_options = vec!["outside".to_string(), "inside".to_string()];
    let workbench_node_ref = NodeRef::<leptos::html::Input>::new();
    let input_motion_default = ui::text_input::input::InputMotion::default();

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => InputSize::Sm,
        2 => InputSize::Lg,
        _ => InputSize::Md,
    });
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => InputVariant::Flat,
            2 => InputVariant::Underlined,
            _ => InputVariant::Bordered,
        });
    let workbench_label_placement =
        Signal::derive(
            move || match workbench_label_placement_index.get().unwrap_or(0) {
                1 => ui::text_input::input::InputLabelPlacement::Inside,
                _ => ui::text_input::input::InputLabelPlacement::Outside,
            },
        );
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_class.get() {
            ui::text_input::input::InputMotion {
                hover_scale: 1.02,
                tap_scale: 0.97,
                ..ui::text_input::input::InputMotion::default()
            }
        } else {
            input_motion_default
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<Input
  id="name".to_string()
  value=value
  set_value=set_value
/>"#
        .to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<Input\n  id=\"docs-input-workbench\".to_string()\n  value=value\n  set_value=set_value\n  label=\"Name\".to_string()\n  aria_label=\"Name input\".to_string()\n  start_content=move || view! {{ <span>\"@\"</span> }}\n  end_content=move || view! {{ <span>\".com\"</span> }}\n  disabled={}\n  read_only={}\n  required=Signal::derive(move || {})\n  invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-input-help\".to_string()))\n  description=\"Used for playground API coverage\".to_string()\n  error=\"This field is invalid.\".to_string()\n  placeholder=\"Type something…\".to_string()\n  clear_aria_label=\"Clear input\".to_string()\n  input_type=\"text\"\n  is_clearable={}\n  label_hidden={}\n  label_placement=ui::text_input::input::InputLabelPlacement::{:?}\n  size=InputSize::{:?}\n  variant=InputVariant::{:?}\n  motion=ui::text_input::input::InputMotion::default()\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Input>::new()\n/>",
            workbench_disabled.get(),
            workbench_read_only.get(),
            workbench_required.get(),
            workbench_invalid.get(),
            workbench_clearable.get(),
            workbench_label_hidden.get(),
            workbench_label_placement.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_custom_class.get() {
                "\"docs-input-workbench\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_rtl.get() {
                "\"ar\".to_string()"
            } else {
                "\"en\".to_string()"
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "InputWorkbenchConfig {{\n  id: \"docs-input-workbench\",\n  value: {:?},\n  set_value: \"WriteSignal<String>\",\n  label: Some(\"Name\"),\n  aria_label: Some(\"Name input\"),\n  start_content: Some(\"prefix\"),\n  end_content: Some(\"suffix\"),\n  disabled: {},\n  read_only: {},\n  required: {},\n  invalid: {},\n  aria_describedby: Some(\"docs-input-help\"),\n  description: Some(\"Used for playground API coverage\"),\n  error: Some(\"This field is invalid.\"),\n  placeholder: Some(\"Type something…\"),\n  clear_aria_label: Some(\"Clear input\"),\n  input_type: Some(\"text\"),\n  is_clearable: {},\n  label_hidden: {},\n  label_placement: \"{:?}\",\n  size: \"{:?}\",\n  variant: \"{:?}\",\n  motion: \"{:?}\",\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"NodeRef<html::Input>\"),\n}}",
            value.get(),
            workbench_disabled.get(),
            workbench_read_only.get(),
            workbench_required.get(),
            workbench_invalid.get(),
            workbench_clearable.get(),
            workbench_label_hidden.get(),
            workbench_label_placement.get(),
            workbench_size.get(),
            workbench_variant.get(),
            workbench_motion.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-input-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(\"rtl\")"
            } else {
                "Some(\"ltr\")"
            },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Input id="m1".to_string() value=value set_value=set_value label="Default".to_string() />
<Input id="m2".to_string() value=value set_value=set_value label="Invalid + Required".to_string() required=Signal::derive(|| true) invalid=Signal::derive(|| true) />
<Input id="m3".to_string() value=value set_value=set_value label="Disabled + Inside".to_string() disabled=true label_placement=ui::text_input::input::InputLabelPlacement::Inside />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Input"
            slug="input"
            group="Forms"
            description="baseline-style text input with label, description/error, and clear button."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <Input
                    id="docs-input-hello".to_string()
                    value=value
                    set_value=set_value
                />
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="input-workbench-controls">
                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input size".to_string()
                            />
                            <div class="docs-search__label">"Variant"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-variant".to_string()
                                options=workbench_variant_options.clone()
                                selected_index=workbench_variant_index
                                set_selected_index=set_workbench_variant_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input variant".to_string()
                            />
                            <div class="docs-search__label">"Label placement"</div>
                            <SegmentedControl
                                id_base="docs-input-workbench-label-placement".to_string()
                                options=workbench_label_placement_options.clone()
                                selected_index=workbench_label_placement_index
                                set_selected_index=set_workbench_label_placement_index
                                size=SegmentedControlSize::Sm
                                aria_label="Input label placement".to_string()
                            />
                            <Switch checked=workbench_invalid set_checked=set_workbench_invalid>"Invalid"</Switch>
                            <Switch checked=workbench_required set_checked=set_workbench_required>"Required"</Switch>
                            <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                            <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                            <Switch checked=workbench_clearable set_checked=set_workbench_clearable>"Clearable"</Switch>
                            <Switch checked=workbench_label_hidden set_checked=set_workbench_label_hidden>"Label hidden"</Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="input-workbench-preview">
                    <Input
                        id="docs-input-workbench".to_string()
                        value=value
                        set_value=set_value
                        label="Name".to_string()
                        aria_label="Name input".to_string()
                        start_content=move || view! { <span>"@"</span> }
                        end_content=move || view! { <span>".com"</span> }
                        disabled=workbench_disabled.get()
                        read_only=workbench_read_only.get()
                        required=Signal::derive(move || workbench_required.get())
                        invalid=Signal::derive(move || workbench_invalid.get())
                        aria_describedby=Signal::derive(move || Some("docs-input-help".to_string()))
                        description="Used for playground API coverage".to_string()
                        error="This field is invalid.".to_string()
                        placeholder="Type something…".to_string()
                        clear_aria_label="Clear input".to_string()
                        input_type="text"
                        is_clearable=workbench_clearable.get()
                        label_hidden=workbench_label_hidden.get()
                        label_placement=workbench_label_placement.get()
                        size=workbench_size.get()
                        variant=workbench_variant.get()
                        motion=ui::text_input::input::InputMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-input-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                    />
                    <span id="docs-input-help" class="ui-muted">
                        "value: " {move || value.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Invalid / Disabled)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <Input
                        id="docs-input-matrix-default".to_string()
                        value=value
                        set_value=set_value
                        label="Default".to_string()
                    />
                    <Input
                        id="docs-input-matrix-invalid".to_string()
                        value=value
                        set_value=set_value
                        label="Invalid + Required".to_string()
                        required=Signal::derive(|| true)
                        invalid=Signal::derive(|| true)
                    />
                    <Input
                        id="docs-input-matrix-disabled".to_string()
                        value=value
                        set_value=set_value
                        label="Disabled + Inside".to_string()
                        disabled=true
                        label_placement=ui::text_input::input::InputLabelPlacement::Inside
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text_area() -> AnyView {
    let (workbench_value, set_workbench_value) = signal("Shipping notes".to_string());
    let (workbench_last_change, set_workbench_last_change) = signal("Shipping notes".to_string());
    let on_workbench_value_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next.clone());
        set_workbench_value.set(next);
    });
    let workbench_node_ref: NodeRef<leptos::html::Textarea> = NodeRef::new();

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rows, set_workbench_rows) = signal(6_u32);

    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let hello_code = Signal::derive(move || {
        r#"<TextArea
  id="release-summary".to_string()
  label="Summary".to_string()
  default_value="Ready for launch".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<TextArea\n  id=\"docs-text-area-workbench\".to_string()\n  label=\"Release notes\".to_string()\n  value=Signal::derive(move || value.get())\n  default_value=\"Shipping notes\".to_string()\n  on_value_change=on_value_change\n  is_disabled={}\n  is_read_only={}\n  is_required=Signal::derive(move || {})\n  is_invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-text-area-help\".to_string()))\n  description=\"Used in release checklist\".to_string()\n  error=\"Release notes are required\".to_string()\n  placeholder=\"Write release notes…\".to_string()\n  rows=Some({})\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=node_ref\n/>",
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "TextAreaMotion::disabled()"
            } else {
                "TextAreaMotion::default()"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-text-area-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\".to_string())"
            } else {
                "Some(\"en\".to_string())"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "TextAreaWorkbenchConfig {{\n  id: \"docs-text-area-workbench\",\n  label: \"Release notes\",\n  value: {},\n  default_value: Some(\"Shipping notes\"),\n  on_value_change: Some(\"Callback<String>\"),\n  is_disabled: Some({}),\n  is_read_only: Some({}),\n  is_required: Some({}),\n  is_invalid: Some({}),\n  aria_describedby: Some(\"docs-text-area-help\"),\n  description: Some(\"Used in release checklist\"),\n  error: Some(\"Release notes are required\"),\n  placeholder: Some(\"Write release notes…\"),\n  rows: Some({}),\n  motion: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  node_ref: Some(\"docs-text-area-workbench-textarea\"),\n}}",
            rust_string_literal(&workbench_value.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            workbench_rows.get(),
            if workbench_custom_motion.get() {
                "TextAreaMotion::disabled"
            } else {
                "TextAreaMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-text-area-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<TextArea id="matrix-default".to_string() label="Default".to_string() default_value="Ready".to_string() />
<TextArea
  id="matrix-invalid".to_string()
  label="Invalid + Required".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(|| true)
  error="Please add details".to_string()
/>
<TextArea
  id="matrix-disabled".to_string()
  label="Disabled".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_disabled=true
  rows=Some(3)
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="TextArea"
            slug="text-area"
            group="Forms"
            description="Multiline field with controlled/uncontrolled value semantics."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <TextArea
                    id="docs-text-area-hello".to_string()
                    label="Summary".to_string()
                    default_value="Ready for launch".to_string()
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Toggle full TextArea API and inspect actual config."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="text-area-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Rows"</span>
                            <select
                                class="docs-select"
                                on:change=move |ev| {
                                    let parsed = event_target_value(&ev).parse::<u32>().ok().unwrap_or(6);
                                    set_workbench_rows.set(parsed.max(1));
                                }
                            >
                                <option value="3" selected=move || workbench_rows.get() == 3>"3"</option>
                                <option value="6" selected=move || workbench_rows.get() == 6>"6"</option>
                                <option value="10" selected=move || workbench_rows.get() == 10>"10"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>"Required"</Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>"Invalid"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Motion disabled"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="text-area-workbench-preview">
                    <TextArea
                        id="docs-text-area-workbench".to_string()
                        label="Release notes".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        default_value="Shipping notes".to_string()
                        on_value_change=on_workbench_value_change
                        is_disabled=workbench_disabled.get()
                        is_read_only=workbench_read_only.get()
                        is_required=workbench_required
                        is_invalid=workbench_invalid
                        aria_describedby=Signal::derive(move || Some("docs-text-area-help".to_string()))
                        description="Used in release checklist".to_string()
                        error="Release notes are required".to_string()
                        placeholder="Write release notes…".to_string()
                        rows=workbench_rows.get()
                        motion=if workbench_custom_motion.get() {
                            TextAreaMotion::disabled()
                        } else {
                            TextAreaMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-text-area-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                    />
                    <span id="docs-text-area-help" class="ui-muted">
                        "on_value_change: " {move || workbench_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <TextArea
                        id="docs-text-area-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="Ready".to_string()
                    />
                    <TextArea
                        id="docs-text-area-matrix-invalid".to_string()
                        label="Invalid + Required".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(|| true)
                        error="Please add details".to_string()
                    />
                    <TextArea
                        id="docs-text-area-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_disabled=true
                        rows=3
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn search_field() -> AnyView {
    let (workbench_value, set_workbench_value) = signal("rust ui".to_string());
    let (workbench_last_change, set_workbench_last_change) = signal("rust ui".to_string());
    let on_workbench_value_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next.clone());
        set_workbench_value.set(next);
    });
    let (workbench_last_submit, set_workbench_last_submit) = signal("none".to_string());
    let on_workbench_submit =
        Callback::new(move |query: String| set_workbench_last_submit.set(query));
    let (workbench_clear_count, set_workbench_clear_count) = signal(0_u32);
    let on_workbench_clear =
        Callback::new(move |()| set_workbench_clear_count.update(|count| *count += 1));

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_read_only, set_workbench_read_only) = signal(false);
    let (workbench_required_raw, set_workbench_required_raw) = signal(false);
    let (workbench_invalid_raw, set_workbench_invalid_raw) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());

    let hello_code = Signal::derive(move || {
        r#"<SearchField
  id="global-search".to_string()
  label="Search".to_string()
  default_value="rust ui".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<SearchField\n  id=\"docs-search-field-workbench\".to_string()\n  label=\"Search docs\".to_string()\n  value=Signal::derive(move || value.get())\n  default_value=\"rust ui\".to_string()\n  on_value_change=on_value_change\n  is_disabled={}\n  is_read_only={}\n  is_required=Signal::derive(move || {})\n  is_invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || Some(\"docs-search-field-help\".to_string()))\n  description=\"Search over component catalog\".to_string()\n  error=\"Try a narrower query\".to_string()\n  placeholder=\"Search docs…\".to_string()\n  on_submit=on_submit\n  on_clear=on_clear\n  clear_button_aria_label=\"Clear search\".to_string()\n  motion={}\n  class_name={}\n  lang={}\n  dir={}\n/>",
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            if workbench_custom_motion.get() {
                "SearchFieldMotion { hover_scale: 1.2, ..SearchFieldMotion::default() }"
            } else {
                "SearchFieldMotion::default()"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-search-field-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\".to_string())"
            } else {
                "Some(\"en\".to_string())"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SearchFieldWorkbenchConfig {{\n  id: \"docs-search-field-workbench\",\n  label: \"Search docs\",\n  value: {},\n  default_value: Some(\"rust ui\"),\n  on_value_change: Some(\"Callback<String>\"),\n  is_disabled: Some({}),\n  is_read_only: Some({}),\n  is_required: Some({}),\n  is_invalid: Some({}),\n  aria_describedby: Some(\"docs-search-field-help\"),\n  description: Some(\"Search over component catalog\"),\n  error: Some(\"Try a narrower query\"),\n  placeholder: Some(\"Search docs…\"),\n  on_submit: Some(\"Callback<String>\"),\n  on_clear: Some(\"Callback<()>\"),\n  clear_button_aria_label: Some(\"Clear search\"),\n  motion: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n}}",
            rust_string_literal(&workbench_value.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_read_only.get()),
            bool_word(workbench_required_raw.get()),
            bool_word(workbench_invalid_raw.get()),
            if workbench_custom_motion.get() {
                "SearchFieldMotion::custom"
            } else {
                "SearchFieldMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-search-field-workbench\")"
            } else {
                "None"
            },
            if workbench_rtl.get() {
                "Some(\"ar\")"
            } else {
                "Some(\"en\")"
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SearchField id="matrix-default".to_string() label="Default".to_string() default_value="search".to_string() />
<SearchField
  id="matrix-required".to_string()
  label="Required + Invalid".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_required=Signal::derive(|| true)
  is_invalid=Signal::derive(|| true)
  error="Query required".to_string()
/>
<SearchField
  id="matrix-disabled".to_string()
  label="Disabled".to_string()
  value=Signal::derive(move || value.get())
  on_value_change=on_value_change
  is_disabled=true
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SearchField"
            slug="search-field"
            group="Forms"
            description="Search input with clear/submit callbacks and a typed state contract."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <SearchField
                    id="docs-search-field-hello".to_string()
                    label="Search".to_string()
                    default_value="rust ui".to_string()
                />
            </Playground>

            <Playground
                title="Config Workbench"
                description="Covers full SearchField API with visible callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="search-field-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_read_only set_checked=set_workbench_read_only>"Read only"</Switch>
                        <Switch checked=workbench_required_raw set_checked=set_workbench_required_raw>"Required"</Switch>
                        <Switch checked=workbench_invalid_raw set_checked=set_workbench_invalid_raw>"Invalid"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL"</Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>"Custom motion"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="search-field-workbench-preview">
                    <SearchField
                        id="docs-search-field-workbench".to_string()
                        label="Search docs".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        default_value="rust ui".to_string()
                        on_value_change=on_workbench_value_change
                        is_disabled=workbench_disabled.get()
                        is_read_only=workbench_read_only.get()
                        is_required=workbench_required
                        is_invalid=workbench_invalid
                        aria_describedby=Signal::derive(move || Some("docs-search-field-help".to_string()))
                        description="Search over component catalog".to_string()
                        error="Try a narrower query".to_string()
                        placeholder="Search docs…".to_string()
                        on_submit=on_workbench_submit
                        on_clear=on_workbench_clear
                        clear_button_aria_label="Clear search".to_string()
                        motion=if workbench_custom_motion.get() {
                            SearchFieldMotion {
                                hover_scale: 1.2,
                                ..SearchFieldMotion::default()
                            }
                        } else {
                            SearchFieldMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-search-field-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span id="docs-search-field-help" class="ui-muted">
                        "change: " {move || workbench_last_change.get()}
                        " · submit: " {move || workbench_last_submit.get()}
                        " · clear_count: " {move || workbench_clear_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <SearchField
                        id="docs-search-field-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="search".to_string()
                    />
                    <SearchField
                        id="docs-search-field-matrix-required".to_string()
                        label="Required + Invalid".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_required=Signal::derive(|| true)
                        is_invalid=Signal::derive(|| true)
                        error="Query required".to_string()
                    />
                    <SearchField
                        id="docs-search-field-matrix-disabled".to_string()
                        label="Disabled".to_string()
                        value=Signal::derive(move || workbench_value.get())
                        on_value_change=on_workbench_value_change
                        is_disabled=true
                    />
                </div>
            </Playground>
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
    let (workbench_use_external_desc, set_workbench_use_external_desc) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_required: Signal<bool> = Signal::derive(move || workbench_required_raw.get());
    let workbench_invalid: Signal<bool> = Signal::derive(move || workbench_invalid_raw.get());
    let workbench_desc_id = "docs-number-field-workbench-help".to_string();
    let workbench_aria_describedby = {
        let workbench_desc_id = workbench_desc_id.clone();
        Signal::derive(move || {
            if workbench_use_external_desc.get() {
                Some(workbench_desc_id.clone())
            } else {
                None
            }
        })
    };
    let workbench_node_ref = NodeRef::<leptos::html::Input>::new();

    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_change = Callback::new(move |next: i64| {
        set_workbench_last_change.set(next.to_string());
    });

    let workbench_desc_id_for_code = workbench_desc_id.clone();
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
            "  on_change=Callback::new(move |_| {})".to_string(),
            format!(
                "  required=Signal::derive(move || {})",
                bool_word(workbench_required_raw.get())
            ),
            format!(
                "  invalid=Signal::derive(move || {})",
                bool_word(workbench_invalid_raw.get())
            ),
            format!(
                "  aria_describedby=Signal::derive(move || {})",
                if workbench_use_external_desc.get() {
                    format!("Some({})", rust_string_literal(&workbench_desc_id_for_code))
                } else {
                    "None".to_string()
                }
            ),
            format!(
                "  description={}",
                rust_string_literal(if workbench_required_raw.get() {
                    "Required field"
                } else {
                    ""
                })
            ),
            format!(
                "  error={}",
                rust_string_literal(if workbench_invalid_raw.get() {
                    "Out of range"
                } else {
                    ""
                })
            ),
            "  placeholder=\"Enter quantity\".into()".to_string(),
            format!(
                "  class_name={}",
                rust_string_literal(if workbench_custom_class.get() {
                    "docs-number-field-custom"
                } else {
                    ""
                })
            ),
            "  node_ref=NodeRef::<leptos::html::Input>::new()".to_string(),
        ];
        lines.push(format!("  disabled={}", workbench_disabled.get()));
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/text-input/src/number_field/styles.rs */\n{}",
            ui::text_input::number_field::styles::CSS
        )
    });

    let workbench_desc_id_for_config = workbench_desc_id.clone();
    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_required_raw.get() {
            "Required field"
        } else {
            ""
        };
        let error = if workbench_invalid_raw.get() {
            "Out of range"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-number-field-custom"
        } else {
            ""
        };
        format!(
            "NumberFieldWorkbenchConfig {{\n  id: \"docs-number-field-workbench\",\n  label: \"Quantity\",\n  value: {},\n  set_value: \"set_workbench_value\",\n  disabled: {},\n  min: {:?},\n  max: {:?},\n  step: {},\n  on_change: \"set_workbench_last_change\",\n  required: {},\n  invalid: {},\n  aria_describedby: {},\n  description: {:?},\n  error: {:?},\n  placeholder: \"Enter quantity\",\n  class_name: {:?},\n  node_ref: \"workbench_node_ref\",\n  last_change: \"{}\",\n}}",
            workbench_value.get(),
            workbench_disabled.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_step.get(),
            workbench_required_raw.get(),
            workbench_invalid_raw.get(),
            if workbench_use_external_desc.get() {
                format!(
                    "Some({})",
                    rust_string_literal(&workbench_desc_id_for_config)
                )
            } else {
                "None".to_string()
            },
            description,
            error,
            class_name,
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
            <Playground title="Hello World (Default Stepper)" code_signal=code>
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

            <Playground
                title="Workbench (All API + Actual Config)"
                description="Display + Config + Code + CSS Test workbench for number-field semantics and stepping contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/text-input/src/number_field/styles.rs".to_string()
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
                        <Switch
                            checked=workbench_use_external_desc
                            set_checked=set_workbench_use_external_desc
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
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
                        aria_describedby=workbench_aria_describedby
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
                        placeholder="Enter quantity".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-number-field-custom".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                        on_change=on_workbench_change
                    />
                    <Show when=move || workbench_use_external_desc.get()>
                        <div id=workbench_desc_id.clone() class="ui-muted">
                            "External help text wired via aria_describedby."
                        </div>
                    </Show>
                    <span class="ui-muted">
                        "value: "
                        {move || workbench_value.get()}
                        " | last on_change: "
                        {move || workbench_last_change.get()}
                    </span>
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
    let (workbench_use_external_desc, set_workbench_use_external_desc) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let workbench_desc_id = "docs-input-otp-workbench-help".to_string();
    let workbench_aria_describedby = {
        let workbench_desc_id = workbench_desc_id.clone();
        Signal::derive(move || {
            if workbench_use_external_desc.get() {
                Some(workbench_desc_id.clone())
            } else {
                None
            }
        })
    };
    let workbench_node_ref = NodeRef::<leptos::html::Input>::new();
    let (workbench_last_change, set_workbench_last_change) = signal("none".to_string());
    let on_workbench_change = Callback::new(move |next: String| {
        set_workbench_last_change.set(next);
    });
    let (workbench_last_complete, set_workbench_last_complete) = signal("none".to_string());
    let on_workbench_complete =
        Callback::new(move |next: String| set_workbench_last_complete.set(next));

    let workbench_desc_id_for_code = workbench_desc_id.clone();
    let workbench_code = Signal::derive(move || {
        let length = workbench_length.get();
        format!(
            "let (value, set_value) = signal(String::new());\n\n<InputOtp\n  id_base=\"docs-otp-workbench\".into()\n  label=\"One-time code\".into()\n  value=value\n  set_value=set_value\n  length={length}\n  disabled={}\n  on_change=Callback::new(move |_| {{}})\n  on_complete=Callback::new(move |_| {{}})\n  aria_label={}\n  required=Signal::derive(move || {})\n  invalid=Signal::derive(move || {})\n  aria_describedby=Signal::derive(move || {})\n  description={}\n  error={}\n  class_name={}\n  lang={}\n  dir={}\n  node_ref=NodeRef::<leptos::html::Input>::new()\n/>",
            workbench_disabled.get(),
            rust_string_literal(if workbench_custom_aria.get() {
                "Verification code"
            } else {
                ""
            }),
            bool_word(workbench_required.get()),
            bool_word(workbench_invalid.get()),
            if workbench_use_external_desc.get() {
                format!("Some({})", rust_string_literal(&workbench_desc_id_for_code))
            } else {
                "None".to_string()
            },
            rust_string_literal(if workbench_show_description.get() {
                "We sent a code to your device."
            } else {
                ""
            }),
            rust_string_literal(if workbench_show_error.get() {
                "Code does not match."
            } else {
                ""
            }),
            rust_string_literal(if workbench_custom_class.get() {
                "docs-input-otp-workbench"
            } else {
                ""
            }),
            if workbench_lang_zh.get() {
                "Some(\"zh-CN\".to_string())".to_string()
            } else {
                "Some(\"en-US\".to_string())".to_string()
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)".to_string()
            } else {
                "Some(A11yDirection::Ltr)".to_string()
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/text-input/src/input_otp/styles.rs */\n{}",
            ui::text_input::input_otp::styles::CSS
        )
    });

    let workbench_desc_id_for_config = workbench_desc_id.clone();
    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            "We sent a code to your device."
        } else {
            ""
        };
        let error = if workbench_show_error.get() {
            "Code does not match."
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-input-otp-workbench"
        } else {
            ""
        };
        format!(
            "InputOtpWorkbenchConfig {{\n  id_base: \"docs-otp-workbench\",\n  value: {:?},\n  set_value: \"set_workbench_value\",\n  length: {},\n  disabled: {},\n  on_change: \"set_workbench_last_change\",\n  on_complete: \"set_workbench_last_complete\",\n  label: \"One-time code\",\n  aria_label: {:?},\n  required: {},\n  invalid: {},\n  aria_describedby: {},\n  description: {:?},\n  error: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  last_change: {:?},\n  last_complete: {:?},\n}}",
            workbench_value.get(),
            workbench_length.get(),
            workbench_disabled.get(),
            if workbench_custom_aria.get() {
                "Verification code"
            } else {
                ""
            },
            workbench_required.get(),
            workbench_invalid.get(),
            if workbench_use_external_desc.get() {
                format!(
                    "Some({})",
                    rust_string_literal(&workbench_desc_id_for_config)
                )
            } else {
                "None".to_string()
            },
            description,
            error,
            class_name,
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_last_change.get(),
            workbench_last_complete.get(),
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
            <Playground title="Hello World (Default OTP)" code_signal=code>
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
                test_source_path="components/text-input/src/input_otp/styles.rs".to_string()
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
                        <Switch
                            checked=workbench_use_external_desc
                            set_checked=set_workbench_use_external_desc
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
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
                        on_change=on_workbench_change
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
                        aria_describedby=workbench_aria_describedby
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
                        lang=if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        node_ref=workbench_node_ref
                        on_complete=on_workbench_complete
                    />
                    <Show when=move || workbench_use_external_desc.get()>
                        <div id=workbench_desc_id.clone() class="ui-muted">
                            "External helper text attached via aria_describedby."
                        </div>
                    </Show>
                    <span class="ui-muted">"value: " {move || workbench_value.get()}</span>
                    <span class="ui-muted">
                        "last change: "
                        {move || workbench_last_change.get()}
                    </span>
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
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_aria, set_interactive_custom_aria) = signal(false);
    let (interactive_lang_zh, set_interactive_lang_zh) = signal(false);
    let (interactive_rtl_dir, set_interactive_rtl_dir) = signal(false);
    let (interactive_last_change, set_interactive_last_change) = signal("none".to_string());
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
    let interactive_motion = Signal::derive(move || {
        if interactive_custom_motion.get() {
            ui::CheckboxMotion {
                hover_scale: 1.08,
                tap_scale: 0.92,
                ..ui::CheckboxMotion::default()
            }
        } else {
            ui::CheckboxMotion::default()
        }
    });
    let on_interactive_change = Callback::new(move |next: bool| {
        set_interactive_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });
    let (comparison_controlled, set_comparison_controlled) = signal(false);

    let interactive_code = Signal::derive(move || {
        let mut lines = vec![
            "let (is_checked, on_checked_change) = signal(true);".to_string(),
            "".to_string(),
            "<Checkbox".to_string(),
            "  is_checked=is_checked".to_string(),
            "  on_checked_change=on_checked_change".to_string(),
            "  on_change=Callback::new(move |_| {})".to_string(),
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
            lines.push("  is_disabled=true".to_string());
        }
        if interactive_custom_class.get() {
            lines.push("  class_name=\"docs-checkbox-custom\".into()".to_string());
        }
        if interactive_custom_motion.get() {
            lines.push(
                "  motion=CheckboxMotion { hover_scale: 1.08, tap_scale: 0.92, ..CheckboxMotion::default() }"
                    .to_string(),
            );
        }
        if interactive_custom_aria.get() {
            lines.push("  aria_label=\"Accept policy\".into()".to_string());
        }
        lines.push(if interactive_lang_zh.get() {
            "  lang=\"zh-CN\".into()".to_string()
        } else {
            "  lang=\"en-US\".into()".to_string()
        });
        lines.push(if interactive_rtl_dir.get() {
            "  dir=Some(A11yDirection::Rtl)".to_string()
        } else {
            "  dir=Some(A11yDirection::Ltr)".to_string()
        });

        lines.push(">".to_string());
        lines.push("  \"Interactive consent\"".to_string());
        lines.push("</Checkbox>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/checkbox/src/styles.rs */\n{}",
            ui::checkbox::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxActualConfig {{\n  is_checked: {},\n  checked: {},\n  on_checked_change: \"set_interactive_checked\",\n  set_checked: \"set_comparison_controlled\",\n  default_checked: Some(true),\n  is_disabled: {},\n  disabled: true,\n  on_change: \"set_interactive_last_change\",\n  variant: {:?},\n  size: {:?},\n  motion: {},\n  class_name: {},\n  aria_label: {},\n  lang: {},\n  dir: {},\n}}",
            interactive_checked.get(),
            comparison_controlled.get(),
            interactive_disabled.get(),
            interactive_variant.get(),
            interactive_size.get(),
            if interactive_custom_motion.get() {
                "CheckboxMotion(custom hover/tap)"
            } else {
                "CheckboxMotion::default()"
            },
            if interactive_custom_class.get() {
                "\"docs-checkbox-custom\""
            } else {
                "None"
            },
            if interactive_custom_aria.get() {
                "Some(\"Accept policy\")"
            } else {
                "None"
            },
            if interactive_lang_zh.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if interactive_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let hello_world_code = Signal::derive(|| r#"<Checkbox>"Accept terms"</Checkbox>"#.to_string());

    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);

<Checkbox
  is_checked=checked
  on_checked_change=set_checked
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
  is_checked=marketing
  on_checked_change=set_marketing
  variant=CheckboxVariant::Accent
  size=CheckboxSize::Lg
>
  "Email updates"
</Checkbox>
<Checkbox
  is_checked=disabled_checked
  on_checked_change=set_disabled_checked
  is_disabled=true
>
  "Disabled on"
</Checkbox>
<Checkbox
  is_checked=disabled_unchecked
  on_checked_change=set_disabled_unchecked
  is_disabled=true
>
  "Disabled off"
</Checkbox>"#
            .to_string()
    });

    let comparison_code = Signal::derive(move || {
        r#"let (controlled, set_controlled) = signal(false);

<Checkbox
  is_checked=controlled
  on_checked_change=set_controlled
>
  "Controlled"
</Checkbox>
<Checkbox default_checked=Some(true)>
  "Uncontrolled default on"
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
                title="Hello World"
                description="Minimal default path: no state wiring required."
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <Checkbox>"Accept terms"</Checkbox>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit checkbox props and inspect actual state contracts."
                code_signal=interactive_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
                test_css_source=interactive_test_css
                test_source_path="components/checkbox/src/styles.rs".to_string()
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
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                        <Switch checked=interactive_custom_aria set_checked=set_interactive_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=interactive_lang_zh set_checked=set_interactive_lang_zh>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=interactive_rtl_dir set_checked=set_interactive_rtl_dir>
                            "dir RTL"
                        </Switch>
                    </div>
                }
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="checkbox-e2e-interactive-surface"
                    data-e2e-ready="true"
                >
                    <Checkbox
                        is_checked=interactive_checked
                        on_checked_change=set_interactive_checked
                        variant=interactive_variant.get()
                        size=interactive_size.get()
                        is_disabled=interactive_disabled.get()
                        on_change=on_interactive_change
                        motion=interactive_motion.get()
                        class_name=if interactive_custom_class.get() {
                            "docs-checkbox-custom".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if interactive_custom_aria.get() {
                            "Accept policy".to_string()
                        } else {
                            String::new()
                        }
                        lang=if interactive_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if interactive_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    >
                        "Interactive consent"
                    </Checkbox>
                    <span class="ui-muted">
                        "checked: " {move || interactive_checked.get()}
                        " · disabled: " {move || interactive_disabled.get()}
                    </span>
                    <span class="ui-muted">
                        "last on_change: "
                        {move || interactive_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled + on_change"
                code_signal=code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div
                    class="docs-stack"
                    data-slot="checkbox-e2e-controlled-surface"
                    data-e2e-ready="true"
                >
                    <div class="docs-row" data-slot="checkbox-e2e-controlled-row">
                        <div data-slot="checkbox-e2e-controlled-target">
                            <Checkbox
                                is_checked=checked
                                on_checked_change=set_checked
                                on_change=on_accept_change
                            >
                                "Accept terms"
                            </Checkbox>
                        </div>
                        <span class="ui-muted" data-slot="checkbox-e2e-controlled-checked">
                            "checked: " {move || checked.get()}
                        </span>
                    </div>
                    <span class="ui-muted" data-slot="checkbox-e2e-controlled-last-change">
                        "last on_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Variant + Disabled matrix"
                code_signal=states_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div
                    class="docs-stack"
                    data-slot="checkbox-e2e-matrix-surface"
                    data-e2e-ready="true"
                >
                    <div class="docs-row" data-slot="checkbox-e2e-marketing-row">
                        <Checkbox
                            is_checked=marketing
                            on_checked_change=set_marketing
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
                    <div class="docs-row" data-slot="checkbox-e2e-disabled-row">
                        <div data-slot="checkbox-e2e-disabled-on">
                            <Checkbox
                                checked=disabled_checked
                                set_checked=set_disabled_checked
                                disabled=true
                            >
                                "Disabled on"
                            </Checkbox>
                        </div>
                        <div data-slot="checkbox-e2e-disabled-off">
                            <Checkbox
                                is_checked=disabled_unchecked
                                on_checked_change=set_disabled_unchecked
                                is_disabled=true
                            >
                                "Disabled off"
                            </Checkbox>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Comparison)"
                description="受控路径展示外部单一事实来源；非受控路径由 default_checked 初始化后内部管理。"
                code_signal=comparison_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <Checkbox
                            is_checked=comparison_controlled
                            on_checked_change=set_comparison_controlled
                        >
                            "Controlled"
                        </Checkbox>
                        <span class="ui-muted">
                            "controlled: " {move || comparison_controlled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Checkbox checked=comparison_controlled set_checked=set_comparison_controlled>
                            "Checked + set_checked alias"
                        </Checkbox>
                        <span class="ui-muted">
                            "alias-controlled: " {move || comparison_controlled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Checkbox default_checked=true>"Uncontrolled default on"</Checkbox>
                        <span class="ui-muted">"uncontrolled: internal state (default_checked)"</span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="checkbox-streaming-policy">
                <h3>"Streaming / Snapshot"</h3>
                <p>
                    "Checkbox is "
                    <strong>"Streaming Optional; fallback=snapshot."</strong>
                </p>
                <p data-slot="checkbox-streaming-modes">
                    "Snapshot mode renders verified full output for checkbox semantics. Streaming labels are exposed via stable markers (`data-ui-stream-support`, `data-ui-stream-fallback`, `data-ui-output-status`)."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="checkbox-source-first">
                <h3>"Source-first / Copy-ready"</h3>
                <p data-slot="checkbox-copy-ready">
                    "Each playground supports code + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " and include "
                    <code>"use leptos::prelude::*; use ui::*;"</code>
                    "."
                </p>
                <ul data-slot="checkbox-source-paths">
                    <li><code>"components/checkbox/src/view.rs"</code></li>
                    <li><code>"components/checkbox/src/logic.rs"</code></li>
                    <li><code>"components/checkbox/src/styles.rs"</code></li>
                    <li><code>"apps/docs-app/src/pages/components/pages/forms.rs"</code></li>
                </ul>
                <ul data-slot="checkbox-source-prerequisites">
                    <li>
                        <code>"ui"</code>
                        " with feature "
                        <code>"component-checkbox"</code>
                    </li>
                    <li>
                        <code>"inject-css"</code>
                        " enabled in docs acceptance surface"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn checkbox_group() -> AnyView {
    let (hello_apple, set_hello_apple) = signal(false);
    let (hello_banana, set_hello_banana) = signal(true);

    let (apple, set_apple) = signal(false);
    let (banana, set_banana) = signal(true);
    let (mango, set_mango) = signal(false);

    let is_invalid = Signal::derive(move || !(apple.get() || banana.get() || mango.get()));
    let is_required = Signal::derive(|| true);
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

    let hello_code = Signal::derive(move || {
        r#"<CheckboxGroup id="demo".to_string() label="Fruits".to_string()>
  <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
  <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
</CheckboxGroup>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let is_invalid = Signal::derive(move || !(apple.get() || banana.get()));
<CheckboxGroup
  id="demo".to_string()
  label="Fruits".to_string()
  description="Pick at least one".to_string()
  error="At least one required".to_string()
  is_required=Signal::derive(|| true)
  is_invalid=is_invalid
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
            "  is_required=Signal::derive(|| {})",
            interactive_required.get()
        ));
        lines.push(format!(
            "  is_invalid=Signal::derive(|| {})",
            interactive_invalid.get()
        ));
        if interactive_disabled.get() {
            lines.push("  is_disabled=true".to_string());
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
            "/* components/checkbox/src/styles.rs */\n{}\n\n/* components/checkbox-group/src/styles.rs */\n{}",
            ui::checkbox::styles::CSS,
            ui::checkbox_group::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxGroupActualConfig {{\n  label: {:?},\n  is_required: {},\n  is_invalid: {},\n  is_disabled: {},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n  aria_describedby: {:?},\n  class_name: {:?},\n  description: {},\n  error: {},\n  alpha: {},\n  beta: {},\n}}",
            "Release channels",
            interactive_required.get(),
            interactive_invalid.get(),
            interactive_disabled.get(),
            ui::checkbox_group::CheckboxGroupMotion::default(),
            "en-US",
            A11yDirection::Ltr,
            aria_describedby.get(),
            "",
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
  is_disabled=true
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
                title="Hello World（默认路径）"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <CheckboxGroup id="docs-checkbox-group-hello".to_string() label="Fruits".to_string()>
                    <Checkbox checked=hello_apple set_checked=set_hello_apple>"Apple"</Checkbox>
                    <Checkbox checked=hello_banana set_checked=set_hello_banana>"Banana"</Checkbox>
                </CheckboxGroup>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit group is_invalid/is_required state and inspect contracts."
                code_signal=interactive_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
                test_css_source=interactive_test_css
                test_source_path="components/checkbox-group/src/styles.rs".to_string()
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
                    let is_required = Signal::derive(move || interactive_required.get());
                    let is_invalid = Signal::derive(move || interactive_invalid.get());
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <CheckboxGroup
                                id="docs-checkbox-group-interactive".to_string()
                                label="Release channels".to_string()
                                description=description
                                error=error
                                is_required=is_required
                                is_invalid=is_invalid
                                is_disabled=interactive_disabled.get()
                                motion=ui::checkbox_group::CheckboxGroupMotion::default()
                                lang="en-US".to_string()
                                dir=A11yDirection::Ltr
                                class_name="".to_string()
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

            <Playground
                title="Validation + Required"
                code_signal=code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-stack">
                    <CheckboxGroup
                        id="docs-checkbox-group".to_string()
                        label="Fruits".to_string()
                        description="Pick at least one".to_string()
                        error="At least one required".to_string()
                        is_required=is_required
                        is_invalid=is_invalid
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
                        {move || is_invalid.get()}
                    </span>

                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_apple.set(false);
                                set_banana.set(false);
                                set_mango.set(false);
                            })
                        >
                            "Clear selections"
                        </ui::Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Disabled + Optional)"
                code_signal=states_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-row">
                    <div class="docs-stack">
                        <CheckboxGroup
                            id="docs-checkbox-group-disabled".to_string()
                            label="Notifications".to_string()
                            description="Read-only preferences".to_string()
                            is_disabled=true
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

            <section class="docs-card docs-prose" data-slot="checkbox-group-streaming-policy">
                <h3>"Streaming / Snapshot"</h3>
                <p>
                    "CheckboxGroup is "
                    <strong>"Streaming Optional; fallback=snapshot."</strong>
                </p>
                <p data-slot="checkbox-group-streaming-modes">
                    "Snapshot mode renders verified full output for group semantics. Streaming labels are exposed via stable markers (`data-ui-stream-support`, `data-ui-stream-fallback`, `data-ui-output-status`)."
                </p>
                <p data-slot="checkbox-group-controlled-uncontrolled-na">
                    "Controlled vs Uncontrolled contrast is N/A at group level: this component does not own a group value axis (`value/on_value_change/default_value`); child `Checkbox` owns checked state."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="checkbox-group-source-first">
                <h3>"Source-first / Copy-ready"</h3>
                <p data-slot="checkbox-group-copy-ready">
                    "Each playground supports code + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " and include "
                    <code>"use leptos::prelude::*; use ui::*;"</code>
                    "."
                </p>
                <ul data-slot="checkbox-group-source-paths">
                    <li><code>"components/checkbox-group/src/view.rs"</code></li>
                    <li><code>"components/checkbox-group/src/logic.rs"</code></li>
                    <li><code>"components/checkbox-group/src/styles.rs"</code></li>
                    <li><code>"apps/docs-app/src/pages/components/pages/forms.rs"</code></li>
                </ul>
                <ul data-slot="checkbox-group-source-prerequisites">
                    <li>
                        <code>"ui"</code>
                        " with feature "
                        <code>"component-checkbox_group"</code>
                    </li>
                    <li>
                        <code>"inject-css"</code>
                        " enabled in docs acceptance surface"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn switch() -> AnyView {
    use leptos::html;

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

    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_pressed_width, set_workbench_pressed_width) = signal(22_u16);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_checked_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
        set_workbench_change_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Switch\n  checked=checked\n  set_checked=set_checked\n  disabled={}\n  on_checked_change=Callback::new(move |_| {{}})\n  pressed_width_px={}\n  motion=SwitchMotion::default()\n  class_name={}\n  aria_label={}\n  node_ref=NodeRef::new()\n>\n  \"Notifications\"\n</Switch>",
            workbench_disabled.get(),
            workbench_pressed_width.get(),
            if workbench_custom_class.get() {
                "\"docs-switch-custom\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_aria.get() {
                "\"Notifications toggle\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SwitchActualConfig {{\n  checked: {},\n  set_checked: \"bound(set_checked)\",\n  disabled: {},\n  on_checked_change: \"count={}\",\n  pressed_width_px: {},\n  motion: SwitchMotion::default(),\n  class_name: {},\n  aria_label: {},\n  node_ref: \"workbench_node_ref\",\n}}",
            checked.get(),
            workbench_disabled.get(),
            workbench_change_count.get(),
            workbench_pressed_width.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-switch-custom\")"
            } else {
                "None"
            },
            if workbench_custom_aria.get() {
                "Some(\"Notifications toggle\")"
            } else {
                "None"
            }
        )
    });

    view! {
        <ComponentPage
            title="Switch"
            slug="switch"
            group="Forms"
            description="Switch toggle with baseline-level spring thumb motion and baseline-style root state attrs."
        >
            <Playground title="Hello World (Default Switch)" code_signal=code>
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="switch-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            "pressed_width_px (" {move || workbench_pressed_width.get()} ")"
                            <input
                                type="range"
                                min="14"
                                max="32"
                                step="1"
                                prop:value=move || workbench_pressed_width.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(22)
                                        .clamp(14, 32);
                                    set_workbench_pressed_width.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Switch
                        checked=checked
                        set_checked=set_checked
                        disabled=workbench_disabled.get()
                        on_checked_change=on_workbench_checked_change
                        pressed_width_px=f64::from(workbench_pressed_width.get())
                        motion=ui::SwitchMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-switch-custom".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Notifications toggle".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                    >
                        "Notifications"
                    </Switch>
                    <span class="ui-muted">
                        "checked: " {move || checked.get()}
                        " · on_checked_change: " {move || workbench_change_count.get()}
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

    let workbench_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (workbench_selected, set_workbench_selected) = signal(Some(2_usize));
    let workbench_external_label_id = "docs-radio-group-workbench-label".to_string();
    let (workbench_is_horizontal, set_workbench_is_horizontal) = signal(true);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_middle, set_workbench_disable_middle) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let matrix_options = vec![
        "Monthly".to_string(),
        "Quarterly".to_string(),
        "Yearly".to_string(),
    ];
    let (matrix_selected, set_matrix_selected) = signal(Some(2_usize));
    let (matrix_vertical_selected, set_matrix_vertical_selected) = signal(Some(0_usize));
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

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_is_horizontal.get() {
            "RadioGroupOrientation::Horizontal"
        } else {
            "RadioGroupOrientation::Vertical"
        };
        let disabled_indices = if workbench_disable_middle.get() {
            "vec![1_usize]"
        } else {
            "Vec::<usize>::new()"
        };
        let motion = if workbench_custom_motion.get() {
            "RadioMotion { hover_scale: 1.08, tap_scale: 0.94, ..RadioMotion::default() }"
        } else {
            "RadioMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-radio-group-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let lang = if workbench_rtl.get() {
            "\"ar\".to_string()"
        } else {
            "\"en-US\".to_string()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        format!(
            "let (selected, set_selected) = signal(Some(2_usize));\n\n<RadioGroup\n  id_base=\"docs-radio-group-workbench\".to_string()\n  options=vec![\n    \"Monthly\".to_string(),\n    \"Quarterly\".to_string(),\n    \"Yearly\".to_string(),\n  ]\n  selected_index=selected\n  set_selected_index=set_selected\n  is_disabled={}\n  disabled={}\n  disabled_indices={disabled_indices}\n  orientation={orientation}\n  label=\"Billing cycle\".to_string()\n  aria_label=\"Billing cycle options\".to_string()\n  aria_labelledby=\"docs-radio-group-workbench-label\".to_string()\n  lang={lang}\n  dir={dir}\n  motion={motion}\n  class_name={class_name}\n/>",
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
        )
    });

    let workbench_options_for_config = workbench_options.clone();
    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_is_horizontal.get() {
            "horizontal"
        } else {
            "vertical"
        };
        let disabled_indices = if workbench_disable_middle.get() {
            vec![1_usize]
        } else {
            Vec::<usize>::new()
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-radio-group-workbench")
        } else {
            None
        };
        let motion = if workbench_custom_motion.get() {
            "custom"
        } else {
            "default"
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };

        format!(
            "RadioGroupWorkbenchActualConfig {{\n  id_base: \"docs-radio-group-workbench\",\n  options: {:?},\n  selected_index: {:?},\n  set_selected_index: \"bound(set_workbench_selected)\",\n  is_disabled: Some({}),\n  disabled: {},\n  disabled_indices: {:?},\n  orientation: \"{orientation}\",\n  label: Some(\"Billing cycle\"),\n  aria_label: Some(\"Billing cycle options\"),\n  aria_labelledby: Some(\"docs-radio-group-workbench-label\"),\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n  motion: \"{motion}\",\n  class_name: {class_name:?},\n}}",
            workbench_options_for_config.clone(),
            workbench_selected.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            disabled_indices,
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let (billing_selected, set_billing_selected) = signal(Some(2_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<RadioGroup
  id_base="billing".to_string()
  options=vec![
    "Monthly".to_string(),
    "Quarterly".to_string(),
    "Yearly".to_string(),
  ]
  orientation=RadioGroupOrientation::Horizontal
  disabled_indices=vec![1]
  selected_index=billing_selected
  set_selected_index=set_billing_selected
/>
<RadioGroup
  id_base="billing-vertical".to_string()
  options=vec![
    "Monthly".to_string(),
    "Quarterly".to_string(),
    "Yearly".to_string(),
  ]
  orientation=RadioGroupOrientation::Vertical
  is_disabled=true
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
/>"#
        .to_string()
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="radio-group-workbench-controls">
                        <Switch checked=workbench_is_horizontal set_checked=set_workbench_is_horizontal>
                            "Horizontal orientation"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_middle set_checked=set_workbench_disable_middle>
                            "Disable middle option"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <div class="docs-row">
                            <button type="button" on:click=move |_| set_workbench_selected.set(Some(0))>
                                "Select first"
                            </button>
                            <button type="button" on:click=move |_| set_workbench_selected.set(None)>
                                "Clear selection"
                            </button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div id=workbench_external_label_id.clone() class="ui-muted">"Billing cycle"</div>
                    {move || {
                        let orientation = if workbench_is_horizontal.get() {
                            RadioGroupOrientation::Horizontal
                        } else {
                            RadioGroupOrientation::Vertical
                        };
                        let disabled_indices = if workbench_disable_middle.get() {
                            vec![1_usize]
                        } else {
                            Vec::new()
                        };
                        let is_disabled = workbench_is_disabled.get();
                        view! {
                            <RadioGroup
                                id_base="docs-radio-group-workbench".to_string()
                                options=workbench_options.clone()
                                orientation=orientation
                                is_disabled=is_disabled
                                disabled=workbench_disabled.get()
                                disabled_indices=disabled_indices
                                label="Billing cycle".to_string()
                                aria_label="Billing cycle options".to_string()
                                aria_labelledby=workbench_external_label_id.clone()
                                selected_index=workbench_selected
                                set_selected_index=set_workbench_selected
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
                                motion=if workbench_custom_motion.get() {
                                    ui::radio::RadioMotion {
                                        hover_scale: 1.08,
                                        tap_scale: 0.94,
                                        ..ui::radio::RadioMotion::default()
                                    }
                                } else {
                                    ui::radio::RadioMotion::default()
                                }
                                class_name=if workbench_custom_class.get() {
                                    "docs-radio-group-workbench".to_string()
                                } else {
                                    String::new()
                                }
                            />
                        }
                    }}
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · orientation: "
                        {move || if workbench_is_horizontal.get() { "horizontal" } else { "vertical" }}
                        " · is_disabled: "
                        {move || workbench_is_disabled.get()}
                        " · disabled: "
                        {move || workbench_disabled.get()}
                        " · disabled options: "
                        {move || if workbench_disable_middle.get() { "1" } else { "0" }}
                        " · lang/dir: "
                        {move || if workbench_rtl.get() { "ar/rtl" } else { "en-US/ltr" }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Empty)" code_signal=matrix_code>
                <div class="docs-stack">
                    <div id="docs-radio-group-billing-label" class="ui-muted">"Billing cycle"</div>
                    <RadioGroup
                        id_base="docs-radio-group-matrix-horizontal".to_string()
                        options=matrix_options.clone()
                        orientation=RadioGroupOrientation::Horizontal
                        disabled_indices=vec![1_usize]
                        selected_index=matrix_selected
                        set_selected_index=set_matrix_selected
                    />
                    <RadioGroup
                        id_base="docs-radio-group-matrix-vertical".to_string()
                        options=matrix_options
                        orientation=RadioGroupOrientation::Vertical
                        is_disabled=true
                        aria_labelledby="docs-radio-group-billing-label".to_string()
                        selected_index=matrix_vertical_selected
                        set_selected_index=set_matrix_vertical_selected
                    />
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

    let (workbench_default_checked, set_workbench_default_checked) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_checked_change_count, set_workbench_checked_change_count) = signal(0_u32);
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_checked_change = Callback::new(move |next: bool| {
        set_checked.set(next);
        set_workbench_checked_change_count.update(|count| *count += 1);
    });
    let on_workbench_change = Callback::new(move |next: bool| {
        set_checked.set(next);
        set_workbench_change_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Radio\n  id=\"docs-radio-workbench\".to_string()\n  label=\"Workbench\".to_string()\n  is_checked=Signal::derive(move || checked.get())\n  checked=Signal::derive(move || checked.get())\n  default_checked={}\n  is_disabled=Some({})\n  disabled={}\n  motion={}\n  class_name={}\n  on_checked_change=Callback::new(move |_| {{}})\n  on_change=Callback::new(move |_| {{}})\n/>",
            workbench_default_checked.get(),
            workbench_is_disabled.get(),
            workbench_disabled.get(),
            if workbench_custom_motion.get() {
                "RadioMotion { hover_scale: 1.08, tap_scale: 0.94, ..RadioMotion::default() }"
            } else {
                "RadioMotion::default()"
            },
            if workbench_custom_class.get() {
                "\"docs-radio-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "RadioActualConfig {{\n  id: \"docs-radio-workbench\",\n  label: \"Workbench\",\n  is_checked: Some({}),\n  checked: Some({}),\n  default_checked: Some({}),\n  is_disabled: Some({}),\n  disabled: {},\n  motion: {},\n  class_name: {},\n  on_checked_change: \"count={}\",\n  on_change: \"count={}\",\n}}",
            checked.get(),
            checked.get(),
            workbench_default_checked.get(),
            workbench_is_disabled.get(),
            workbench_disabled.get(),
            if workbench_custom_motion.get() {
                "RadioMotion::custom"
            } else {
                "RadioMotion::default"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-radio-custom\")"
            } else {
                "None"
            },
            workbench_checked_change_count.get(),
            workbench_change_count.get(),
        )
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

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="radio-workbench-controls">
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_default_checked.get() on:change=move |ev| set_workbench_default_checked.set(event_target_checked(&ev)) />
                            " default_checked"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_is_disabled.get() on:change=move |ev| set_workbench_is_disabled.set(event_target_checked(&ev)) />
                            " is_disabled"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_disabled.get() on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev)) />
                            " disabled"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_motion.get() on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev)) />
                            " motion"
                        </label>
                        <label class="docs-search__label">
                            <input type="checkbox" prop:checked=move || workbench_custom_class.get() on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev)) />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Radio
                        id="docs-radio-workbench".to_string()
                        label="Workbench".to_string()
                        is_checked=Signal::derive(move || checked.get())
                        checked=Signal::derive(move || checked.get())
                        default_checked=workbench_default_checked.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        motion=if workbench_custom_motion.get() {
                            ui::RadioMotion {
                                hover_scale: 1.08,
                                tap_scale: 0.94,
                                ..ui::RadioMotion::default()
                            }
                        } else {
                            ui::RadioMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-radio-custom".to_string()
                        } else {
                            String::new()
                        }
                        on_checked_change=on_workbench_checked_change
                        on_change=on_workbench_change
                    />
                    <span class="ui-muted">
                        "checked: " {move || checked.get()}
                        " · on_checked_change: " {move || workbench_checked_change_count.get()}
                        " · on_change: " {move || workbench_change_count.get()}
                    </span>
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
    let showcase_options = vec!["Overview".to_string(), "Details".to_string()];
    let (showcase_selected, set_showcase_selected) = signal(Some(0_usize));

    let workbench_options = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "Settings".to_string(),
    ];
    let (workbench_selected, set_workbench_selected) = signal(Some(0_usize));
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_disable_last, set_workbench_disable_last) = signal(true);
    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_small, set_workbench_small) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            let mut spring = SegmentedControlMotion::default().spring;
            spring.stiffness = 180.0;
            spring.damping = 26.0;
            spring.mass = 1.0;
            spring.precision = 0.001;
            SegmentedControlMotion { spring }
        } else {
            SegmentedControlMotion::default()
        }
    });

    let matrix_options = vec![
        "System".to_string(),
        "Manual".to_string(),
        "Hybrid".to_string(),
    ];
    let (matrix_horizontal_selected, set_matrix_horizontal_selected) = signal(Some(1_usize));
    let (matrix_vertical_selected, set_matrix_vertical_selected) = signal(Some(0_usize));
    let (matrix_disabled_selected, set_matrix_disabled_selected) = signal(Some(2_usize));

    let hello_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
<SegmentedControl
  id_base="seg-default".to_string()
  options=vec!["Overview".to_string(), "Details".to_string()]
  selected_index=selected
  set_selected_index=set_selected
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            "SegmentedControlOrientation::Vertical"
        } else {
            "SegmentedControlOrientation::Horizontal"
        };
        let size = if workbench_small.get() {
            "SegmentedControlSize::Sm"
        } else {
            "SegmentedControlSize::Default"
        };
        let disabled_indices = if workbench_disable_last.get() {
            "vec![2_usize]"
        } else {
            "Vec::<usize>::new()"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_custom_motion.get() {
            "SegmentedControlMotion { spring: SegmentedControlMotion::default().spring }"
        } else {
            "SegmentedControlMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-segmented-control-workbench\".to_string()"
        } else {
            "String::new()"
        };

        format!(
            "let (selected, set_selected) = signal(Some(0_usize));\n\n<SegmentedControl\n  id_base=\"docs-segments-workbench\".to_string()\n  options=vec![\"Overview\".to_string(), \"Details\".to_string(), \"Settings\".to_string()]\n  selected_index=selected\n  set_selected_index=set_selected\n  disabled={}\n  disabled_indices={disabled_indices}\n  orientation={orientation}\n  size={size}\n  motion={motion}\n  label=\"Workspace section\".to_string()\n  aria_label=\"Workspace segmented control\".to_string()\n  lang={}.to_string()\n  dir={dir}\n  class_name={class_name}\n/>",
            bool_word(workbench_disabled.get()),
            rust_string_literal(if workbench_rtl.get() { "ar" } else { "en-US" }),
        )
    });

    let workbench_options_for_config = workbench_options.clone();
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SegmentedControlWorkbenchActualConfig {{\n  id_base: \"docs-segments-workbench\",\n  options: {:?},\n  selected_index: {:?},\n  set_selected_index: \"bound(set_workbench_selected)\",\n  disabled: {},\n  disabled_indices: {:?},\n  orientation: {:?},\n  size: {:?},\n  motion: {:?},\n  label: Some(\"Workspace section\"),\n  aria_label: Some(\"Workspace segmented control\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {:?},\n}}",
            workbench_options_for_config.clone(),
            workbench_selected.get(),
            bool_word(workbench_disabled.get()),
            if workbench_disable_last.get() {
                vec![2_usize]
            } else {
                Vec::<usize>::new()
            },
            if workbench_vertical.get() {
                SegmentedControlOrientation::Vertical
            } else {
                SegmentedControlOrientation::Horizontal
            },
            if workbench_small.get() {
                SegmentedControlSize::Sm
            } else {
                SegmentedControlSize::Default
            },
            if workbench_custom_motion.get() {
                "custom"
            } else {
                "default"
            },
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            if workbench_custom_class.get() {
                Some("docs-segmented-control-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<SegmentedControl id_base="seg-horizontal".to_string() options=vec!["System".to_string(), "Manual".to_string(), "Hybrid".to_string()] selected_index=selected set_selected_index=set_selected />
<SegmentedControl id_base="seg-vertical".to_string() options=vec!["System".to_string(), "Manual".to_string(), "Hybrid".to_string()] selected_index=selected_vertical set_selected_index=set_selected_vertical orientation=SegmentedControlOrientation::Vertical size=SegmentedControlSize::Sm disabled_indices=vec![2] />
<SegmentedControl id_base="seg-disabled".to_string() options=vec!["System".to_string(), "Manual".to_string(), "Hybrid".to_string()] selected_index=selected_disabled set_selected_index=set_selected_disabled disabled=true aria_label="Disabled options".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="SegmentedControl"
            slug="segmented-control"
            group="Forms"
            description="Segmented control with baseline-level indicator motion and baseline-style root state attrs."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <SegmentedControl
                    id_base="docs-segments-hello".to_string()
                    options=showcase_options
                    selected_index=showcase_selected
                    set_selected_index=set_showcase_selected
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="segmented-control-workbench-controls">
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_small set_checked=set_workbench_small>
                            "Small size"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_disable_last set_checked=set_workbench_disable_last>
                            "disable last option"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <button
                            type="button"
                            on:click=move |_| set_workbench_selected.set(Some(0_usize))
                        >
                            "Reset selection"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack">
                    <SegmentedControl
                        id_base="docs-segments-workbench".to_string()
                        options=workbench_options.clone()
                        selected_index=workbench_selected
                        set_selected_index=set_workbench_selected
                        disabled=workbench_disabled.get()
                        disabled_indices=if workbench_disable_last.get() {
                            vec![2_usize]
                        } else {
                            Vec::<usize>::new()
                        }
                        orientation=if workbench_vertical.get() {
                            SegmentedControlOrientation::Vertical
                        } else {
                            SegmentedControlOrientation::Horizontal
                        }
                        size=if workbench_small.get() {
                            SegmentedControlSize::Sm
                        } else {
                            SegmentedControlSize::Default
                        }
                        motion=workbench_motion.get()
                        label="Workspace section".to_string()
                        aria_label="Workspace segmented control".to_string()
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
                            "docs-segmented-control-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <SegmentedControl
                        id_base="docs-segments-matrix-horizontal".to_string()
                        options=matrix_options.clone()
                        selected_index=matrix_horizontal_selected
                        set_selected_index=set_matrix_horizontal_selected
                    />
                    <SegmentedControl
                        id_base="docs-segments-matrix-vertical".to_string()
                        options=matrix_options.clone()
                        selected_index=matrix_vertical_selected
                        set_selected_index=set_matrix_vertical_selected
                        orientation=SegmentedControlOrientation::Vertical
                        size=SegmentedControlSize::Sm
                        disabled_indices=vec![2_usize]
                    />
                    <SegmentedControl
                        id_base="docs-segments-matrix-disabled".to_string()
                        options=matrix_options
                        selected_index=matrix_disabled_selected
                        set_selected_index=set_matrix_disabled_selected
                        disabled=true
                        aria_label="Disabled options".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
