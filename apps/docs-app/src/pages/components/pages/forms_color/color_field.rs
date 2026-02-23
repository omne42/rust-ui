use super::*;

pub(crate) fn color_field() -> AnyView {
    let (showcase_value, set_showcase_value) = signal(Some("#4f46e5".to_string()));
    let on_showcase_value_change =
        Callback::new(move |next: Option<String>| set_showcase_value.set(next));

    let (workbench_value, set_workbench_value) = signal(Some("#22c55e".to_string()));
    let (workbench_change_count, set_workbench_change_count) = signal(0_u32);
    let on_workbench_value_change = Callback::new(move |next: Option<String>| {
        set_workbench_change_count.update(|count| *count += 1);
        set_workbench_value.set(next);
    });

    let (workbench_placeholder, set_workbench_placeholder) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_preview_visible, set_workbench_preview_visible) = signal(true);
    let (workbench_show_preview, set_workbench_show_preview) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r##"<ColorField
  id_base="docs-color-field-hello".to_string()
  label="Brand color".to_string()
  default_value="#4f46e5".to_string()
/>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ColorField\n  id_base=\"docs-color-field-workbench\".to_string()\n  label=\"Brand color\".to_string()\n  placeholder={}.to_string()\n  is_disabled={}\n  disabled={}\n  value=value.into()\n  default_value=\"#0ea5e9\".to_string()\n  on_value_change=on_value_change\n  is_preview_visible={}\n  show_preview={}\n  aria_label=\"Brand color input\".to_string()\n  class_name={}\n  lang={}.to_string()\n  dir={}\n/>",
            rust_string_literal(if workbench_placeholder.get() {
                "Use #RRGGBB"
            } else {
                ""
            }),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            bool_word(workbench_preview_visible.get()),
            bool_word(workbench_show_preview.get()),
            if workbench_custom_class.get() {
                "\"docs-color-field-custom\".to_string()"
            } else {
                "String::new()"
            },
            rust_string_literal(if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            }),
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ColorFieldActualConfig {{\n  id_base: \"docs-color-field-workbench\",\n  label: Some(\"Brand color\"),\n  placeholder: {:?},\n  is_disabled: Some({}),\n  disabled: Some({}),\n  value: {:?},\n  default_value: Some(\"#0ea5e9\"),\n  on_value_change: \"count={}\",\n  is_preview_visible: Some({}),\n  show_preview: Some({}),\n  aria_label: Some(\"Brand color input\"),\n  class_name: {:?},\n  lang: Some({:?}),\n  dir: Some({:?}),\n}}",
            if workbench_placeholder.get() {
                Some("Use #RRGGBB")
            } else {
                None
            },
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
            workbench_value.get(),
            workbench_change_count.get(),
            bool_word(workbench_preview_visible.get()),
            bool_word(workbench_show_preview.get()),
            if workbench_custom_class.get() {
                Some("docs-color-field-custom")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<ColorField id_base="cf-default".to_string() label="Default".to_string() default_value="#4f46e5".to_string() />
<ColorField id_base="cf-placeholder".to_string() label="Placeholder".to_string() placeholder="Use #RRGGBB".to_string() is_preview_visible=true show_preview=true />
<ColorField id_base="cf-disabled".to_string() label="Disabled".to_string() is_disabled=true disabled=true class_name="docs-color-field-custom".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl />"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorField"
            slug="color-field"
            group="Forms"
            description="ColorField playground with full API workbench coverage and visible callback feedback."
        >
            <Playground title="Hello World (Default ColorField)" code_signal=hello_code>
                <div class="docs-stack docs-stack--tight">
                    <ColorField
                        id_base="docs-color-field-hello".to_string()
                        label="Brand color".to_string()
                        default_value="#4f46e5".to_string()
                        value=showcase_value.into()
                        on_value_change=on_showcase_value_change
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || showcase_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-field-workbench-controls">
                        <Switch checked=workbench_placeholder set_checked=set_workbench_placeholder>
                            "placeholder"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch checked=workbench_preview_visible set_checked=set_workbench_preview_visible>
                            "is_preview_visible"
                        </Switch>
                        <Switch checked=workbench_show_preview set_checked=set_workbench_show_preview>
                            "show_preview"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-field-workbench">
                    <ColorField
                        id_base="docs-color-field-workbench".to_string()
                        label="Brand color".to_string()
                        placeholder=if workbench_placeholder.get() {
                            "Use #RRGGBB".to_string()
                        } else {
                            String::new()
                        }
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled_alias.get()
                        value=workbench_value.into()
                        default_value="#0ea5e9".to_string()
                        on_value_change=on_workbench_value_change
                        is_preview_visible=workbench_preview_visible.get()
                        show_preview=workbench_show_preview.get()
                        aria_label="Brand color input".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-color-field-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted">
                        "changes="
                        {move || workbench_change_count.get()}
                        " · value="
                        {move || workbench_value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Placeholder / Disabled RTL)" code_signal=matrix_code>
                <div class="docs-row">
                    <ColorField
                        id_base="docs-color-field-matrix-default".to_string()
                        label="Default".to_string()
                        default_value="#4f46e5".to_string()
                    />
                    <ColorField
                        id_base="docs-color-field-matrix-placeholder".to_string()
                        label="Placeholder".to_string()
                        placeholder="Use #RRGGBB".to_string()
                        is_preview_visible=true
                        show_preview=true
                    />
                    <ColorField
                        id_base="docs-color-field-matrix-disabled".to_string()
                        label="Disabled RTL".to_string()
                        is_disabled=true
                        disabled=true
                        class_name="docs-color-field-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
