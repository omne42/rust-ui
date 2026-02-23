use super::*;

pub(crate) fn search_field() -> AnyView {
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
