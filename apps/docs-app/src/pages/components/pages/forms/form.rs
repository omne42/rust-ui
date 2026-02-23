use super::*;

pub(crate) fn form() -> AnyView {
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
