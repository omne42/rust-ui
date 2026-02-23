use super::*;

pub(crate) fn input_group() -> AnyView {
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
