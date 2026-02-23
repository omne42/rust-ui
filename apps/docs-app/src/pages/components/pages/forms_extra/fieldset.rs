use super::*;

pub(crate) fn fieldset() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<Fieldset legend="Channels".to_string()>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<Fieldset
  legend="Notification channels".to_string()
  description="Pick every channel you want to receive release updates from.".to_string()
  is_required=true
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let invalid_code = Signal::derive(move || {
        r#"<Fieldset
  orientation=FieldsetOrientation::Horizontal
  tone=FieldsetTone::Muted
  is_invalid=true
  error_message="Pick at least one channel".to_string()
  class_name="docs-fieldset-custom".to_string()
  actions=move || view! {
    <ui::Button variant=ui::ButtonVariant::Secondary size=ui::ButtonSize::Sm>
      "Manage channels"
    </ui::Button>
  }
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"let (controlled_invalid, set_controlled_invalid) = signal(true);

// Uncontrolled-style: initialize once with default_is_invalid.
<Fieldset
  legend="Uncontrolled snapshot".to_string()
  default_is_required=true
  default_is_disabled=false
  default_is_invalid=true
  error_message="Uncontrolled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>

// Controlled-style: external signal is the single source of truth.
<Fieldset
  legend="Controlled snapshot".to_string()
  default_is_required=false
  on_is_required_change=Callback::new(move |_next| {})
  default_is_disabled=false
  on_is_disabled_change=Callback::new(move |_next| {})
  is_invalid=Signal::derive(move || controlled_invalid.get())
  on_is_invalid_change=Callback::new(move |next| set_controlled_invalid.set(next))
  motion=FieldsetMotion::default()
  error_message="Controlled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Streaming Optional: Fieldset is not a正文阅读面; fallback remains snapshot.
<Fieldset
  legend="Streaming validation snapshot".to_string()
  is_invalid=true
  error_message="Streaming fallback=snapshot: waiting for final validation".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>"#
            .to_string()
    });

    let orientation_options = vec!["vertical".to_string(), "horizontal".to_string()];
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let locale_options = vec!["en-US".to_string(), "zh-CN".to_string(), "ar".to_string()];

    let (controlled_invalid, set_controlled_invalid) = signal(true);
    let controlled_invalid_signal = Signal::derive(move || controlled_invalid.get());
    let on_controlled_invalid_change =
        Callback::new(move |next: bool| set_controlled_invalid.set(next));

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_locale_index, set_workbench_locale_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_actions, set_workbench_show_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_orientation =
        Signal::derive(
            move || match workbench_orientation_index.get().unwrap_or(0) {
                1 => FieldsetOrientation::Horizontal,
                _ => FieldsetOrientation::Vertical,
            },
        );
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => FieldsetTone::Muted,
        _ => FieldsetTone::Default,
    });

    let workbench_code = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => Some("zh-CN"),
            2 => Some("ar"),
            _ => None,
        };

        let mut lines = vec!["<Fieldset".to_string()];
        if orientation != FieldsetOrientation::Vertical {
            lines.push(format!(
                "  orientation=FieldsetOrientation::{orientation:?}"
            ));
        }
        if tone != FieldsetTone::Default {
            lines.push(format!("  tone=FieldsetTone::{tone:?}"));
        }
        if required {
            lines.push("  is_required=true".to_string());
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if invalid {
            lines.push("  is_invalid=true".to_string());
            lines.push("  error_message=\"Pick at least one channel\".into()".to_string());
        } else if show_description {
            lines.push("  description=\"Choose channels for release updates.\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-fieldset-custom\".into()".to_string());
        }
        if let Some(lang) = lang {
            lines.push(format!("  lang=\"{lang}\".into()"));
        }
        if rtl {
            lines.push("  dir=ui_headless::A11yDirection::Rtl".to_string());
        }
        if show_actions {
            lines.push(
                "  actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary size=ui::ButtonSize::Sm>\"Manage\"</ui::Button> }".to_string(),
            );
        }
        lines.extend([
            "  legend=\"Notification channels\".into()".to_string(),
            ">".to_string(),
            "  <label><input type=\"checkbox\" /> \"Email\"</label>".to_string(),
            "  <label><input type=\"checkbox\" /> \"SMS\"</label>".to_string(),
            "</Fieldset>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/field_form/fieldset/styles.rs */\n{}",
            ui::field_form::fieldset::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => "zh-CN",
            2 => "ar",
            _ => "default",
        };

        let mut class = vec![
            "ui-fieldset".to_string(),
            orientation.class_name().into(),
            tone.class_name().into(),
        ];
        if required {
            class.push("ui-fieldset--required".to_string());
        }
        if disabled {
            class.push("ui-fieldset--disabled".to_string());
        }
        if invalid {
            class.push("ui-fieldset--invalid".to_string());
        }
        if custom_class {
            class.push("ui-fieldset--custom-class".to_string());
            class.push("docs-fieldset-custom".to_string());
        }

        let message_kind = if invalid {
            "error"
        } else if show_description {
            "description"
        } else {
            "none"
        };

        format!(
            "FieldsetActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  is_required: {required},\n  default_is_required: {},\n  on_is_required_change: {:?},\n  is_disabled: {disabled},\n  default_is_disabled: {},\n  on_is_disabled_change: {:?},\n  is_invalid: {invalid},\n  default_is_invalid: {},\n  on_is_invalid_change: {:?},\n  legend: {:?},\n  error_message: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  has_description: {},\n  has_actions: {show_actions},\n  class_source: \"{}\",\n  message_kind: \"{message_kind}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
            false,
            Some("Callback<bool>"),
            false,
            Some("Callback<bool>"),
            false,
            Some("Callback<bool>"),
            Some("Notification channels"),
            if invalid {
                Some("Pick at least one channel")
            } else {
                None
            },
            Some("Notification channel group"),
            if custom_class {
                Some("docs-fieldset-custom")
            } else {
                None
            },
            ui::field_form::fieldset::FieldsetMotion::default(),
            show_description && !invalid,
            if custom_class { "custom" } else { "default" },
            if rtl { "rtl" } else { "auto" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Fieldset"
            slug="fieldset"
            group="Forms"
            description="baseline-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <Fieldset legend="Channels".to_string()>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground
                title="Fieldset Workbench (Display + Config + Code + CSS Test)"
                description="展示 / config / code / css test 一体化工作台，并提供多场景对比。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui/src/field_form/fieldset/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="fieldset-workbench-controls">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset tone".to_string()
                        />

                        <div class="docs-search__label">"Locale"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-locale".to_string()
                            options=locale_options.clone()
                            selected_index=workbench_locale_index
                            set_selected_index=set_workbench_locale_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset locale".to_string()
                        />

                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </Switch>
                        <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                            "Actions"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = workbench_orientation.get();
                    let tone = workbench_tone.get();
                    let locale_index = workbench_locale_index.get().unwrap_or(0);
                    let required = workbench_required.get();
                    let disabled = workbench_disabled.get();
                    let invalid = workbench_invalid.get();
                    let show_description = workbench_show_description.get();
                    let show_actions = workbench_show_actions.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();
                    let lang = match locale_index {
                        1 => "zh-CN".to_string(),
                        2 => "ar".to_string(),
                        _ => String::new(),
                    };
                    let description = if show_description && !invalid {
                        "Choose channels for release updates.".to_string()
                    } else {
                        String::new()
                    };
                    let error_message = if invalid {
                        "Pick at least one channel".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-fieldset-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="fieldset-workbench-compare">
                            <div class="docs-search__label">"Baseline"</div>
                            <Fieldset legend="Notification channels".to_string()>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"Email"</span>
                                </label>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"SMS"</span>
                                </label>
                            </Fieldset>

                            <div class="docs-search__label">"Configured"</div>
                            {if show_actions {
                                view! {
                            <Fieldset
                                orientation=orientation
                                tone=tone
                                is_required=required
                                is_disabled=disabled
                                is_invalid=invalid
                                legend="Notification channels".to_string()
                                description=description.clone()
                                error_message=error_message.clone()
                                class_name=class_name.clone()
                                aria_label="Notification channel group".to_string()
                                lang=lang.clone()
                                dir=dir
                                motion=ui::field_form::fieldset::FieldsetMotion::default()
                                actions=move || {
                                    view! {
                                        <ui::Button
                                                    variant=ui::ButtonVariant::Secondary
                                                    size=ui::ButtonSize::Sm
                                                >
                                                    "Manage"
                                                </ui::Button>
                                            }
                                        }
                                    >
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            } else {
                                view! {
                            <Fieldset
                                orientation=orientation
                                tone=tone
                                is_required=required
                                is_disabled=disabled
                                is_invalid=invalid
                                legend="Notification channels".to_string()
                                description=description
                                error_message=error_message
                                class_name=class_name
                                aria_label="Notification channel group".to_string()
                                lang=lang
                                dir=dir
                                motion=ui::field_form::fieldset::FieldsetMotion::default()
                            >
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            }}

                            <div class="docs-search__label">"Scenario compare"</div>
                            <div class="docs-stack docs-stack--tight">
                                <Fieldset
                                    legend="Required vertical".to_string()
                                    is_required=true
                                    description="Required + description".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"Email"</span>
                                    </label>
                                </Fieldset>
                                <Fieldset
                                    legend="Invalid horizontal".to_string()
                                    orientation=FieldsetOrientation::Horizontal
                                    tone=FieldsetTone::Muted
                                    is_invalid=true
                                    error_message="At least one option is required".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"SMS"</span>
                                    </label>
                                </Fieldset>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Required / Invalid / Disabled)"
                code_signal=invalid_code
            >
                <div class="docs-stack docs-stack--tight" data-slot="fieldset-state-matrix">
                    <Fieldset
                        legend="Required vertical".to_string()
                        is_required=true
                        description="Required + description".to_string()
                        aria_label="Required group".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"Email"</span>
                        </label>
                    </Fieldset>
                    <Fieldset
                        legend="Invalid horizontal".to_string()
                        orientation=FieldsetOrientation::Horizontal
                        tone=FieldsetTone::Muted
                        is_invalid=true
                        error_message="At least one option is required".to_string()
                        aria_label="Invalid group".to_string()
                        class_name="docs-fieldset-custom".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"SMS"</span>
                        </label>
                    </Fieldset>
                    <Fieldset
                        legend="Disabled".to_string()
                        is_disabled=true
                        default_is_required=true
                        default_is_disabled=true
                        on_is_required_change=Callback::new(move |_next| {})
                        on_is_disabled_change=Callback::new(move |_next| {})
                        aria_label="Disabled group".to_string()
                        motion=ui::field_form::fieldset::FieldsetMotion::default()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" disabled />
                            <span>"Push"</span>
                        </label>
                    </Fieldset>
                </div>
            </Playground>

            <Playground title="Legend + Description" code_signal=default_code>
                <Fieldset
                    legend="Notification channels".to_string()
                    description="Pick every channel you want to receive release updates from.".to_string()
                    is_required=true
                    aria_label="Notification channel group".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Push"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground title="Horizontal + Invalid + Actions" code_signal=invalid_code>
                <Fieldset
                    orientation=FieldsetOrientation::Horizontal
                    tone=FieldsetTone::Muted
                    is_invalid=true
                    error_message="Pick at least one channel".to_string()
                    class_name="docs-fieldset-custom".to_string()
                    actions=move || {
                        view! {
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                size=ui::ButtonSize::Sm
                            >
                                "Manage channels"
                            </ui::Button>
                        }
                    }
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Snapshot Contrast)"
                description="受控/非受控对照：默认值只初始化一次，受控值由外部 signal 驱动。"
                code_signal=controlled_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight" data-slot="fieldset-controlled-uncontrolled">
                    <div class="docs-search__label">"Uncontrolled snapshot"</div>
                    <Fieldset
                        legend="Uncontrolled snapshot".to_string()
                        default_is_invalid=true
                        error_message="Uncontrolled snapshot: pick at least one channel".to_string()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"Email"</span>
                        </label>
                    </Fieldset>

                    <div class="docs-search__label">"Controlled snapshot"</div>
                    <ui::Button
                        variant=ui::ButtonVariant::Secondary
                        size=ui::ButtonSize::Sm
                        on_press=Callback::new(move |_| {
                            set_controlled_invalid.update(|value| *value = !*value);
                        })
                    >
                        {move || if controlled_invalid.get() { "Set controlled valid" } else { "Set controlled invalid" }}
                    </ui::Button>
                    <Fieldset
                        legend="Controlled snapshot".to_string()
                        is_invalid=controlled_invalid_signal.get()
                        on_is_invalid_change=on_controlled_invalid_change
                        error_message="Controlled snapshot: pick at least one channel".to_string()
                    >
                        <label class="docs-choice-row">
                            <input type="checkbox" />
                            <span>"SMS"</span>
                        </label>
                    </Fieldset>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Fieldset 不是正文阅读面；文档展示 snapshot 输出与 fallback=snapshot 契约。"
                code_signal=stream_snapshot_code
            >
                <Fieldset
                    legend="Streaming validation snapshot".to_string()
                    is_invalid=true
                    error_message="Streaming fallback=snapshot: waiting for final validation".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                </Fieldset>
                <div class="docs-subtitle">
                    "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-stream-mode."
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="fieldset-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="fieldset-source-first-contract">
                    "Use any Fieldset Playground's "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p data-slot="fieldset-source-first-dependency-baseline">
                    "Dependency baseline (Cargo.toml): "
                    <code>
                        "ui = { default-features = false, features = [\"component-fieldset\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text=r#"components/fieldset/src/mod.rs
components/fieldset/src/logic.rs
components/fieldset/src/view.rs
components/fieldset/src/styles.rs
components/fieldset/src/motion.rs
crates/ui/src/field_form/fieldset/{mod,logic,view,styles,motion}.rs
apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset"#.to_string()
                    copyable=true
                    class_name="docs-fieldset-source-copy".to_string()
                />
                <ul data-slot="fieldset-source-prerequisites">
                    <li><code>"component-fieldset"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
