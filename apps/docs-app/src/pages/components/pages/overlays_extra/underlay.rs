use super::*;

pub(crate) fn underlay() -> AnyView {
    let (showcase_open_raw, set_showcase_open_raw) = signal(false);
    let showcase_open: Signal<bool> = Signal::derive(move || showcase_open_raw.get());
    let (showcase_open_change_count, set_showcase_open_change_count) = signal(0_u32);
    let (showcase_close_count, set_showcase_close_count) = signal(0_u32);

    let open_showcase: OnPress = Callback::new(move |_| set_showcase_open_raw.set(true));
    let on_showcase_open_change = Callback::new(move |next: bool| {
        set_showcase_open_raw.set(next);
        set_showcase_open_change_count.update(|count| *count += 1);
    });
    let on_showcase_close: OnPress = Callback::new(move |_| {
        set_showcase_open_raw.set(false);
        set_showcase_close_count.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Underlay
  id_base="docs-underlay-hello".to_string()
  is_open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  on_close=Callback::new(move |_| set_open_raw.set(false))
/>"#
        .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open_signal: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);

    let (workbench_is_transparent, set_workbench_is_transparent) = signal(false);
    let (workbench_transparent_alias, set_workbench_transparent_alias) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_disable_motion, set_workbench_disable_motion) = signal(false);

    let open_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let on_workbench_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_disable_motion.get() {
            ui::UnderlayMotion::disabled()
        } else {
            ui::UnderlayMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class_name.get() {
            "docs-underlay-workbench"
        } else {
            ""
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_disable_motion.get() {
            "UnderlayMotion::disabled()"
        } else {
            "UnderlayMotion::default()"
        };

        vec![
            "<Underlay".to_string(),
            "  id_base=\"docs-underlay-workbench\".to_string()".to_string(),
            "  is_open=Signal::derive(move || open_raw.get())".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            format!(
                "  is_transparent={}",
                bool_word(workbench_is_transparent.get())
            ),
            format!(
                "  transparent={}",
                bool_word(workbench_transparent_alias.get())
            ),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            format!("  lang={}.to_string()", rust_string_literal(lang)),
            format!("  dir={dir}"),
            format!("  motion={motion}"),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-underlay-workbench")
        } else {
            None
        };
        let motion = if workbench_disable_motion.get() {
            "UnderlayMotion::disabled()"
        } else {
            "UnderlayMotion::default()"
        };
        format!(
            "UnderlayActualConfig {{\n  id_base: \"docs-underlay-workbench\",\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  on_close: \"count={}\",\n  is_transparent: Some({}),\n  transparent: Some({}),\n  is_disabled: Some({}),\n  disabled: Some({}),\n  lang: {lang:?},\n  dir: {dir},\n  motion: {motion},\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_open_raw.get()),
            workbench_open_change_count.get(),
            workbench_close_count.get(),
            bool_word(workbench_is_transparent.get()),
            bool_word(workbench_transparent_alias.get()),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
        )
    });

    let (matrix_default_open_raw, set_matrix_default_open_raw) = signal(false);
    let matrix_default_open: Signal<bool> = Signal::derive(move || matrix_default_open_raw.get());
    let open_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(true));
    let on_matrix_default_open_change =
        Callback::new(move |next: bool| set_matrix_default_open_raw.set(next));
    let close_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(false));

    let (matrix_transparent_open_raw, set_matrix_transparent_open_raw) = signal(false);
    let matrix_transparent_open: Signal<bool> =
        Signal::derive(move || matrix_transparent_open_raw.get());
    let open_matrix_transparent: OnPress =
        Callback::new(move |_| set_matrix_transparent_open_raw.set(true));
    let on_matrix_transparent_open_change =
        Callback::new(move |next: bool| set_matrix_transparent_open_raw.set(next));
    let close_matrix_transparent: OnPress =
        Callback::new(move |_| set_matrix_transparent_open_raw.set(false));

    let matrix_code = Signal::derive(move || {
        r#"<Underlay id_base="underlay-default".to_string() is_open=default_open on_open_change=on_default_change on_close=dismiss_default />
<Underlay id_base="underlay-transparent".to_string() is_open=transparent_open on_open_change=on_transparent_change is_transparent=true transparent=true />
<Underlay id_base="underlay-disabled".to_string() is_open=Signal::derive(|| true) is_disabled=true disabled=true />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Underlay"
            slug="underlay"
            group="Overlays"
            description="Underlay playground with full API workbench and state-matrix comparison."
        >
            <Playground title="Hello World (Default Underlay)" code_signal=hello_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_showcase>"Open underlay"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_showcase_close>
                            "Close"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "open: " {move || showcase_open_raw.get()}
                        " · on_open_change: " {move || showcase_open_change_count.get()}
                        " · on_close: " {move || showcase_close_count.get()}
                    </span>
                </div>
                <Underlay
                    id_base="docs-underlay-hello".to_string()
                    is_open=showcase_open
                    on_open_change=on_showcase_open_change
                    on_close=on_showcase_close
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="underlay-workbench-controls">
                        <Switch
                            checked=workbench_is_transparent
                            set_checked=set_workbench_is_transparent
                        >
                            "is_transparent"
                        </Switch>
                        <Switch
                            checked=workbench_transparent_alias
                            set_checked=set_workbench_transparent_alias
                        >
                            "transparent alias"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <Switch checked=workbench_disable_motion set_checked=set_workbench_disable_motion>
                            "motion disabled"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <Button variant=ButtonVariant::Secondary on_press=open_workbench>
                                "Open"
                            </Button>
                            <Button variant=ButtonVariant::Secondary on_press=close_workbench>
                                "Close"
                            </Button>
                        </div>
                    </div>
                }
            >
                <span class="ui-muted" data-slot="underlay-workbench-feedback">
                    "open: " {move || workbench_open_raw.get()}
                    " · on_open_change: " {move || workbench_open_change_count.get()}
                    " · on_close: " {move || workbench_close_count.get()}
                </span>
                <Underlay
                    id_base="docs-underlay-workbench".to_string()
                    is_open=workbench_open_signal
                    open=workbench_open_signal
                    default_open=false
                    on_open_change=on_workbench_open_change
                    on_close=on_workbench_close
                    is_transparent=workbench_is_transparent.get()
                    transparent=workbench_transparent_alias.get()
                    is_disabled=workbench_is_disabled.get()
                    disabled=workbench_disabled_alias.get()
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                    motion=workbench_motion.get()
                    class_name=if workbench_custom_class_name.get() {
                        "docs-underlay-workbench".to_string()
                    } else {
                        String::new()
                    }
                />
            </Playground>

            <Playground title="State Matrix (Default / Transparent / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="underlay-matrix-controls">
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_default>
                        "Open Default"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_transparent>
                        "Open Transparent"
                    </Button>
                </div>
                <Underlay
                    id_base="docs-underlay-matrix-default".to_string()
                    is_open=matrix_default_open
                    on_open_change=on_matrix_default_open_change
                    on_close=close_matrix_default
                />
                <Underlay
                    id_base="docs-underlay-matrix-transparent".to_string()
                    is_open=matrix_transparent_open
                    on_open_change=on_matrix_transparent_open_change
                    on_close=close_matrix_transparent
                    is_transparent=true
                    transparent=true
                />
                <Underlay
                    id_base="docs-underlay-matrix-disabled".to_string()
                    is_open=Signal::derive(|| true)
                    is_disabled=true
                    disabled=true
                    class_name="docs-underlay-disabled".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
