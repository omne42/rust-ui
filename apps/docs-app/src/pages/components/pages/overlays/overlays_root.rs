use super::*;

pub(crate) fn overlays_root() -> AnyView {
    let (workbench_open, set_workbench_open) = signal(true);
    let (workbench_modal, set_workbench_modal) = signal(true);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<OverlaysRoot\n  id_base={}\n  is_open={}\n  is_modal={}\n  aria_label={}\n  lang={}\n  dir={}\n  class_name={}\n>\n  ...\n</OverlaysRoot>",
            if workbench_custom_id.get() {
                "\"docs-overlays-root-workbench\".to_string()"
            } else {
                "String::new()"
            },
            workbench_open.get(),
            workbench_modal.get(),
            if workbench_custom_aria.get() {
                "\"Workbench overlays root\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\".to_string())"
            } else {
                "Some(\"en-US\".to_string())"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-overlays-root-workbench\".to_string())"
            } else {
                "None"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "OverlaysRootActualConfig {{\n  id_base: {},\n  is_open: {},\n  is_modal: {},\n  aria_label: {},\n  lang: {},\n  dir: {},\n  class_name: {},\n}}",
            if workbench_custom_id.get() {
                "Some(\"docs-overlays-root-workbench\")"
            } else {
                "None"
            },
            workbench_open.get(),
            workbench_modal.get(),
            if workbench_custom_aria.get() {
                "Some(\"Workbench overlays root\")"
            } else {
                "None"
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
            if workbench_custom_class.get() {
                "\"docs-overlays-root-workbench\".to_string()"
            } else {
                "String::new()"
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<OverlaysRoot is_open=true is_modal=true />
<OverlaysRoot is_open=true is_modal=false class_name="docs-overlays-root-custom".to_string() />
<OverlaysRoot is_open=false is_modal=true aria_label="Background stack".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="OverlaysRoot"
            slug="overlays-root"
            group="Overlays"
            description="Shared overlay host primitive exposing stable root-state and source markers for grouped overlay stacks."
        >
            <Playground
                title="Hello World (Default Overlays Root)"
                code_signal=Signal::derive(move || {
                    r#"<OverlaysRoot is_open=true is_modal=true>
  <div>"Overlay stack host"</div>
</OverlaysRoot>"#
                        .to_string()
                })
            >
                <OverlaysRoot
                    is_open=true
                    is_modal=true
                >
                    <div class="ui-card docs-stack docs-stack--tight">
                        <strong>"Overlays root container"</strong>
                        <span class="ui-muted">
                            "Inspect data-state / data-layer / data-id-source / data-class-source in DevTools."
                        </span>
                    </div>
                </OverlaysRoot>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="overlays-root-workbench-controls">
                        <Switch checked=workbench_open set_checked=set_workbench_open>
                            "is_open"
                        </Switch>
                        <Switch checked=workbench_modal set_checked=set_workbench_modal>
                            "is_modal"
                        </Switch>
                        <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                            "id_base"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                    </div>
                }
            >
                <OverlaysRoot
                    id_base=if workbench_custom_id.get() {
                        "docs-overlays-root-workbench".to_string()
                    } else {
                        String::new()
                    }
                    is_open=workbench_open.get()
                    is_modal=workbench_modal.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Workbench overlays root".to_string()
                    } else {
                        String::new()
                    }
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
                    class_name=if workbench_custom_class.get() {
                        "docs-overlays-root-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="ui-card docs-stack docs-stack--tight">
                        <strong>"Workbench overlays root"</strong>
                        <span class="ui-muted">
                            "Toggle root open/modal/a11y/source props and inspect actual config."
                        </span>
                    </div>
                </OverlaysRoot>
            </Playground>

            <Playground title="State Matrix (Open / Inline / Background)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="overlays-root-state-matrix">
                    <OverlaysRoot
                        id_base="docs-overlays-root-matrix-open".to_string()
                        is_open=true
                        is_modal=true
                        aria_label="Open modal stack".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <div class="ui-card">"Open + modal"</div>
                    </OverlaysRoot>
                    <OverlaysRoot
                        id_base="docs-overlays-root-matrix-inline".to_string()
                        is_open=true
                        is_modal=false
                        aria_label="Inline stack".to_string()
                        class_name="docs-overlays-root-inline".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <div class="ui-card">"Open + non-modal"</div>
                    </OverlaysRoot>
                    <OverlaysRoot
                        id_base="docs-overlays-root-matrix-background".to_string()
                        is_open=false
                        is_modal=true
                        aria_label="Background stack".to_string()
                        lang="zh-CN".to_string()
                        dir=ui_headless::A11yDirection::Rtl
                    >
                        <div class="ui-card">"Closed + background"</div>
                    </OverlaysRoot>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
