use super::*;

pub(crate) fn well() -> AnyView {
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_compact, set_workbench_compact) = signal(false);
    let (workbench_inset, set_workbench_inset) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let workbench_tone = Signal::derive(move || {
        if workbench_strong_tone.get() {
            WellTone::Strong
        } else {
            WellTone::Default
        }
    });
    let workbench_density = Signal::derive(move || {
        if workbench_compact.get() {
            WellDensity::Compact
        } else {
            WellDensity::Comfortable
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Well\n  tone=WellTone::{:?}\n  density=WellDensity::{:?}\n  is_inset={}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n>\n  ...\n</Well>",
            workbench_tone.get(),
            workbench_density.get(),
            workbench_inset.get(),
            if workbench_custom_aria.get() {
                "\"Selection summary\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-well-custom\".to_string()"
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
            "WellActualConfig {{\n  tone: Some(WellTone::{:?}),\n  density: Some(WellDensity::{:?}),\n  is_inset: Some({}),\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n}}",
            workbench_tone.get(),
            workbench_density.get(),
            workbench_inset.get(),
            if workbench_custom_aria.get() {
                "Some(\"Selection summary\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-well-custom\")"
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

    let hello_code = Signal::derive(move || {
        r#"<Well>
  <div>"Default well"</div>
</Well>"#
            .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<Well tone=WellTone::Default>
  <div>"Default"</div>
</Well>
<Well tone=WellTone::Quiet density=WellDensity::Compact>
  <div>"Quiet compact"</div>
</Well>
<Well tone=WellTone::Strong is_inset=true>
  <div>"Strong inset"</div>
</Well>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Well
  aria_label="Selection summary".to_string()
  class_name="docs-well-custom".to_string()
>
  <div>"Custom label + class"</div>
</Well>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Well"
            slug="well"
            group="Layout"
            description="Inset container surface for grouped content with centralized tone/density/label state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <Well>
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Default well"</strong>
                        <span class="ui-muted">"Minimal usage with default tone/density and non-inset state."</span>
                    </div>
                </Well>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="well-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_strong_tone.get()
                                on:change=move |ev| set_workbench_strong_tone.set(event_target_checked(&ev))
                            />
                            " tone strong"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_compact.get()
                                on:change=move |ev| set_workbench_compact.set(event_target_checked(&ev))
                            />
                            " density compact"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_inset.get()
                                on:change=move |ev| set_workbench_inset.set(event_target_checked(&ev))
                            />
                            " is_inset"
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
                                prop:checked=move || workbench_zh_lang.get()
                                on:change=move |ev| set_workbench_zh_lang.set(event_target_checked(&ev))
                            />
                            " lang zh-CN"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_dir.get()
                                on:change=move |ev| set_workbench_rtl_dir.set(event_target_checked(&ev))
                            />
                            " dir RTL"
                        </label>
                    </div>
                }
            >
                <Well
                    tone=workbench_tone.get()
                    density=workbench_density.get()
                    is_inset=workbench_inset.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Selection summary".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-well-custom".to_string()
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
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Workbench well"</strong>
                        <span class="ui-muted">"Tune all Well props and inspect actual config."</span>
                    </div>
                </Well>
            </Playground>

            <Playground title="Tone + Density + Inset" code_signal=tone_code>
                <div class="docs-stack docs-stack--tight">
                    <Well tone=WellTone::Default>
                        <div>"Default"</div>
                    </Well>
                    <Well tone=WellTone::Quiet density=WellDensity::Compact>
                        <div>"Quiet compact"</div>
                    </Well>
                    <Well tone=WellTone::Strong is_inset=true>
                        <div>"Strong inset"</div>
                    </Well>
                </div>
            </Playground>

            <Playground title="Custom Label + Class" code_signal=custom_code>
                <Well
                    aria_label="Selection summary".to_string()
                    class_name="docs-well-custom".to_string()
                >
                    <div>"Custom label + class"</div>
                </Well>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
