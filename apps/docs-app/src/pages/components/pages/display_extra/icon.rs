use super::*;

pub(crate) fn icon() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Icon>"✓"</Icon>"#.to_string());
    let icon_code_imports =
        "use leptos::prelude::*;\nuse ui::{Icon, IconSize, IconTone};".to_string();

    let matrix_code = Signal::derive(move || {
        r#"<Icon size=IconSize::Sm tone=IconTone::Default is_decorative=true>"✓"</Icon>
<Icon size=IconSize::Md tone=IconTone::Muted is_decorative=true>"⚙"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Accent is_decorative=true>"★"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Danger is_decorative=true>"⚠"</Icon>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Icon
  size=IconSize::Md
  tone=IconTone::Accent
  is_decorative=false
  aria_label="Sync successful".to_string()
>
  "✓"
</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Muted
  is_disabled=true
  class_name="docs-icon-custom".to_string()
  is_decorative=true
>
  "⚙"
</Icon>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<Icon is_decorative=true>"✓"</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Accent
  is_decorative=false
  aria_label="Mapped from upstream app state".to_string()
>
  "★"
</Icon>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Icon
  size=IconSize::Md
  tone=IconTone::Muted
  is_decorative=false
  aria_label="Snapshot mode icon".to_string()
>
  "⏺"
</Icon>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Icon size=IconSize::Sm tone=IconTone::Accent is_decorative=true>
  "✓"
</Icon>"#
            .to_string()
    });

    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_glyph, set_workbench_glyph) = signal("✓".to_string());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_slot, set_workbench_slot) = signal(String::new());
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);
    let (workbench_label, set_workbench_label) = signal("Status icon".to_string());

    let workbench_code = Signal::derive(move || {
        let size = workbench_size_key.get();
        let tone = workbench_tone_key.get();
        let glyph = workbench_glyph.get();
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_class = workbench_custom_class.get();
        let slot = workbench_slot.get();
        let class_line = if custom_class {
            "  class_name=\"docs-icon-custom\".into()\n".to_string()
        } else {
            String::new()
        };
        let slot_line = if slot.trim().is_empty() {
            String::new()
        } else {
            format!("  slot={:?}.into()\n", slot.trim())
        };
        let aria_line = if decorative {
            String::new()
        } else {
            format!("  aria_label=\"{}\".into()\n", workbench_label.get().trim())
        };
        let locale_lines = if workbench_rtl_locale.get() {
            "  lang=\"ar\".into()\n  dir=A11yDirection::Rtl\n".to_string()
        } else {
            "  lang=\"en-US\".into()\n  dir=A11yDirection::Ltr\n".to_string()
        };
        format!(
            "<Icon\n  size=IconSize::{}\n  tone=IconTone::{}\n  is_disabled={disabled}\n  is_decorative={decorative}\n{aria_line}{class_line}{slot_line}{locale_lines}>\n  \"{glyph}\"\n</Icon>",
            match size.as_str() {
                "sm" => "Sm",
                "lg" => "Lg",
                _ => "Md",
            },
            match tone.as_str() {
                "muted" => "Muted",
                "accent" => "Accent",
                "danger" => "Danger",
                _ => "Default",
            },
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/icon/styles.rs */\n{}",
            ui::icon::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size_key = workbench_size_key.get();
        let tone_key = workbench_tone_key.get();
        let size_class = match size_key.as_str() {
            "sm" => "ui-icon--size-sm",
            "lg" => "ui-icon--size-lg",
            _ => "ui-icon--size-md",
        };
        let tone_class = match tone_key.as_str() {
            "muted" => "ui-icon--tone-muted",
            "accent" => "ui-icon--tone-accent",
            "danger" => "ui-icon--tone-danger",
            _ => "ui-icon--tone-default",
        };
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_class = workbench_custom_class.get();
        let class_name = if custom_class {
            "docs-icon-custom".to_string()
        } else {
            String::new()
        };
        let aria_label = if decorative {
            String::new()
        } else {
            workbench_label.get().trim().chars().collect::<String>()
        };
        let slot = workbench_slot.get();
        let lang = if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        };
        let dir = if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let data_state = if disabled {
            "disabled"
        } else if decorative {
            "decorative"
        } else {
            "labeled"
        };

        let mut classes = vec!["ui-icon".to_string(), size_class.into(), tone_class.into()];
        if disabled {
            classes.push("ui-icon--disabled".to_string());
        }
        if decorative {
            classes.push("ui-icon--decorative".to_string());
        }
        if custom_class {
            classes.push("ui-icon--custom-class".to_string());
            classes.push("docs-icon-custom".to_string());
        }

        format!(
            "IconActualConfig {{\n  size: \"{}\",\n  tone: \"{}\",\n  is_disabled: {},\n  is_decorative: {},\n  aria_label: {:?},\n  class_name: {:?},\n  slot: {:?},\n  lang: {:?},\n  dir: {:?},\n  glyph: \"{}\",\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            size_key,
            tone_key,
            disabled,
            decorative,
            aria_label,
            class_name,
            slot,
            lang,
            dir,
            workbench_glyph.get(),
            if decorative { "n/a" } else { "custom" },
            if custom_class { "custom" } else { "default" },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="Icon"
            slug="icon"
            group="Display"
            description="baseline-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers."
        >
            <Playground title="Hello World (Default Path)" code_signal=hello_code>
                <Icon>"✓"</Icon>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels and live icon state controls."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/icon/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="icon-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Tone"
                            <select
                                prop:value=move || workbench_tone_key.get()
                                on:change=move |ev| set_workbench_tone_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                                <option value="accent">"Accent"</option>
                                <option value="danger">"Danger"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Glyph"
                            <select
                                prop:value=move || workbench_glyph.get()
                                on:change=move |ev| set_workbench_glyph.set(event_target_value(&ev))
                            >
                                <option value="✓">"Check"</option>
                                <option value="⚙">"Gear"</option>
                                <option value="★">"Star"</option>
                                <option value="⚠">"Alert"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_decorative.get()
                                on:change=move |ev| set_workbench_decorative.set(event_target_checked(&ev))
                            />
                            " Decorative"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            "Slot"
                            <select
                                prop:value=move || workbench_slot.get()
                                on:change=move |ev| set_workbench_slot.set(event_target_value(&ev))
                            >
                                <option value="">"None"</option>
                                <option value="leading">"leading"</option>
                                <option value="status">"status"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                        <label class="docs-search__label">
                            "Aria label"
                            <input
                                type="text"
                                prop:value=move || workbench_label.get()
                                on:input=move |ev| set_workbench_label.set(event_target_value(&ev))
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="icon-workbench">
                    <span class="ui-muted">
                        "display: baseline vs configured vs disabled contrast"
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"Baseline"</div>
                            <Icon size=IconSize::Md tone=IconTone::Default is_decorative=true>
                                "✓"
                            </Icon>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Configured"</div>
                            {move || {
                                let size = match workbench_size_key.get().as_str() {
                                    "sm" => IconSize::Sm,
                                    "lg" => IconSize::Lg,
                                    _ => IconSize::Md,
                                };
                                let tone = match workbench_tone_key.get().as_str() {
                                    "muted" => IconTone::Muted,
                                    "accent" => IconTone::Accent,
                                    "danger" => IconTone::Danger,
                                    _ => IconTone::Default,
                                };
                                let class_name = if workbench_custom_class.get() {
                                    "docs-icon-custom".to_string()
                                } else {
                                    String::new()
                                };
                                let decorative = workbench_decorative.get();
                                let aria_label = if decorative {
                                    String::new()
                                } else {
                                    workbench_label.get()
                                };
                                let lang = if workbench_rtl_locale.get() {
                                    "ar".to_string()
                                } else {
                                    "en-US".to_string()
                                };
                                let dir = if workbench_rtl_locale.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                };
                                view! {
                                    <Icon
                                        size=size
                                        tone=tone
                                        is_disabled=workbench_disabled.get()
                                        is_decorative=decorative
                                        aria_label=aria_label
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    >
                                        {workbench_glyph.get()}
                                    </Icon>
                                }
                            }}
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Disabled contrast"</div>
                            <Icon size=IconSize::Lg tone=IconTone::Danger is_disabled=true is_decorative=true>
                                "⚠"
                            </Icon>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Accessibility / Slot Comparison)"
                code_signal=matrix_code
                code_imports=icon_code_imports.clone()
            >
                <div class="docs-row">
                    <Icon
                        size=IconSize::Sm
                        tone=IconTone::Default
                        is_decorative=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        is_decorative=false
                        aria_label="Status icon".to_string()
                        class_name="docs-icon-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "★"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Danger
                        is_disabled=true
                        is_decorative=true
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Size + Tone Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Icon size=IconSize::Sm tone=IconTone::Default is_decorative=true>
                        "✓"
                    </Icon>
                    <Icon size=IconSize::Md tone=IconTone::Muted is_decorative=true>
                        "⚙"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Accent is_decorative=true>
                        "★"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Danger is_decorative=true>
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Accessible + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-row">
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        is_decorative=false
                        aria_label="Sync successful".to_string()
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Muted
                        is_disabled=true
                        class_name="docs-icon-custom".to_string()
                        is_decorative=true
                    >
                        "⚙"
                    </Icon>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for Icon)"
                description="Icon has no controllable state axis; compare default rendering with upstream state mapped into plain props."
                code_signal=controlled_contrast_code
                code_imports=icon_code_imports.clone()
            >
                <div class="docs-row">
                    <Icon is_decorative=true>"✓"</Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Accent
                        is_decorative=false
                        aria_label="Mapped from upstream app state".to_string()
                    >
                        "★"
                    </Icon>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Icon is a display leaf: streaming is optional and falls back to snapshot (`data-ui-streaming=optional`, `data-ui-streaming-fallback=snapshot`)."
                code_signal=stream_snapshot_code
                code_imports=icon_code_imports.clone()
            >
                <Icon
                    size=IconSize::Md
                    tone=IconTone::Muted
                    is_decorative=false
                    aria_label="Snapshot mode icon".to_string()
                >
                    "⏺"
                </Icon>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run; requires `ui` dependency in Cargo.toml."
                code_signal=source_first_code
                code_imports=icon_code_imports.clone()
            >
                <Icon size=IconSize::Sm tone=IconTone::Accent is_decorative=true>
                    "✓"
                </Icon>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
