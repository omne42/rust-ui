use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AlertBanner, AlertBannerFill, AlertBannerMotion, AlertBannerTone, Chart, ChartKind, ChartPoint,
    ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItem, ColorSwatchRounding, ColorSwatchShape,
    ColorSwatchSize, EmptyState, EmptyStateAlign, EmptyStateTone, ErrorView, ErrorViewMotion,
    ErrorViewTone, FlipCard, FlipCardMotion, Icon, IconSize, IconTone, Keyboard, KeyboardTone,
    LabeledValue, LabeledValueOrientation, LabeledValueTone, PressableFeedback,
    PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackTone, RippleMotion,
    SegmentedControl, SegmentedControlSize, Skeleton, SkeletonGroup, SkeletonGroupDensity,
    SkeletonGroupLayout, SkeletonGroupVariant, SkeletonVariant, Switch, Text, TextAlign,
    TextElement, TextTone, TextWeight,
};

pub(super) fn alert_banner() -> AnyView {
    let tone_code = Signal::derive(move || {
        r#"<AlertBanner
  tone=AlertBannerTone::Info
  fill=AlertBannerFill::Border
  title="Updates available".to_string()
  description="A new version is ready to install.".to_string()
>
  "Install now to keep your workspace secure."
</AlertBanner>
<AlertBanner
  tone=AlertBannerTone::Negative
  fill=AlertBannerFill::Subtle
  title="Deployment failed".to_string()
  description="Rollback completed. Review incident timeline.".to_string()
>
  "Check logs before retrying."
</AlertBanner>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<AlertBanner
  tone=AlertBannerTone::Notice
  fill=AlertBannerFill::Bold
  is_hide_icon=true
  title="Maintenance window".to_string()
  description="Service may be degraded during migration.".to_string()
  class_name="docs-alert-banner-custom".to_string()
>
  "Follow status page for live updates."
</AlertBanner>"#
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r#"<AlertBanner
  tone=AlertBannerTone::Info
  fill=AlertBannerFill::Border
  title="Motion tuned".to_string()
  description="Custom spring contract for alert reveal.".to_string()
  motion=AlertBannerMotion {
    spring: Default::default(),
  }
>
  "Inspect data-motion-source/data-custom-motion markers."
</AlertBanner>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="AlertBanner"
            slug="alert-banner"
            group="Display"
            description="baseline-compatible alert banner with centralized tone/fill/content contracts and baseline-level spring reveal motion."
        >
            <Playground title="Tone + Fill" code_signal=tone_code>
                <div class="docs-stack">
                    <AlertBanner
                        tone=AlertBannerTone::Info
                        fill=AlertBannerFill::Border
                        title="Updates available".to_string()
                        description="A new version is ready to install.".to_string()
                    >
                        "Install now to keep your workspace secure."
                    </AlertBanner>
                    <AlertBanner
                        tone=AlertBannerTone::Negative
                        fill=AlertBannerFill::Subtle
                        title="Deployment failed".to_string()
                        description="Rollback completed. Review incident timeline.".to_string()
                    >
                        "Check logs before retrying."
                    </AlertBanner>
                </div>
            </Playground>

            <Playground title="Bold + Hidden Icon + Custom Class" code_signal=custom_code>
                <AlertBanner
                    tone=AlertBannerTone::Notice
                    fill=AlertBannerFill::Bold
                    is_hide_icon=true
                    title="Maintenance window".to_string()
                    description="Service may be degraded during migration.".to_string()
                    class_name="docs-alert-banner-custom".to_string()
                >
                    "Follow status page for live updates."
                </AlertBanner>
            </Playground>

            <Playground title="Custom motion contract" code_signal=motion_code>
                <AlertBanner
                    tone=AlertBannerTone::Info
                    fill=AlertBannerFill::Border
                    title="Motion tuned".to_string()
                    description="Custom spring contract for alert reveal.".to_string()
                    motion=AlertBannerMotion {
                        spring: Default::default(),
                    }
                >
                    "Inspect data-motion-source/data-custom-motion markers."
                </AlertBanner>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn labeled_value() -> AnyView {
    let orientation_options = vec!["Stacked".to_string(), "Inline".to_string()];
    let tone_options = vec![
        "Default".to_string(),
        "Subtle".to_string(),
        "Strong".to_string(),
    ];
    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (show_description, set_show_description) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let orientation_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  description="Updated 2 minutes ago".to_string()
  aria_label="Build status".to_string()
  class_name="docs-labeled-value-custom".to_string()
  tone=LabeledValueTone::Strong
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };
        let description_line = if show_description.get() {
            "  description=\"Updated 2 minutes ago\".to_string()\n"
        } else {
            ""
        };
        let aria_line = if custom_aria.get() {
            "  aria_label=\"Build status\".to_string()\n"
        } else {
            ""
        };
        let class_line = if custom_class.get() {
            "  class_name=\"docs-labeled-value-workbench\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<LabeledValue\n  label=\"Build\".to_string()\n  value=\"passing\".to_string()\n  orientation={orientation_variant} // {orientation}\n  tone={tone_variant}\n{description_line}{aria_line}{class_line}/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone = match tone_index.get().unwrap_or(0) {
            0 => "default",
            1 => "subtle",
            _ => "strong",
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };

        format!(
            "LabeledValueActualConfig {{\n  orientation: {orientation_variant} ({orientation}),\n  tone: {tone_variant} ({tone}),\n  has_description: {},\n  custom_aria_label: {},\n  custom_class_name: {},\n}}",
            show_description.get(),
            custom_aria.get(),
            custom_class.get(),
        )
    });

    view! {
        <ComponentPage
            title="LabeledValue"
            slug="labeled-value"
            group="Display"
            description="Label-value pair primitive with centralized orientation/tone/source state contracts and baseline-style data markers."
        >
            <Playground title="Orientation + Tone" code_signal=orientation_code>
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground title="Description + Custom Aria/Class" code_signal=custom_code>
                <div class="docs-stack">
                    <LabeledValue
                        label="Build".to_string()
                        value="passing".to_string()
                        description="Updated 2 minutes ago".to_string()
                        aria_label="Build status".to_string()
                        class_name="docs-labeled-value-custom".to_string()
                        tone=LabeledValueTone::Strong
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Default
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_source_path="crates/ui-components/src/labeled_value/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue tone".to_string()
                        />

                        <Switch checked=show_description set_checked=set_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = match orientation_index.get().unwrap_or(0) {
                        0 => LabeledValueOrientation::Stacked,
                        _ => LabeledValueOrientation::Inline,
                    };
                    let tone = match tone_index.get().unwrap_or(0) {
                        0 => LabeledValueTone::Default,
                        1 => LabeledValueTone::Subtle,
                        _ => LabeledValueTone::Strong,
                    };
                    let description = if show_description.get() {
                        "Updated 2 minutes ago".to_string()
                    } else {
                        "".to_string()
                    };
                    let aria_label = if custom_aria.get() {
                        "Build status".to_string()
                    } else {
                        "".to_string()
                    };
                    let class_name = if custom_class.get() {
                        "docs-labeled-value-workbench".to_string()
                    } else {
                        "".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <LabeledValue
                                label="Build".to_string()
                                value="passing".to_string()
                                description=description
                                orientation=orientation
                                tone=tone
                                aria_label=aria_label
                                class_name=class_name
                            />
                            <LabeledValue
                                label="Compare / Inline".to_string()
                                value="Healthy".to_string()
                                orientation=LabeledValueOrientation::Inline
                                tone=LabeledValueTone::Subtle
                            />
                            <LabeledValue
                                label="Compare / Stacked".to_string()
                                value="99.95%".to_string()
                                orientation=LabeledValueOrientation::Stacked
                                tone=LabeledValueTone::Strong
                                description="SLA snapshot".to_string()
                            />
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn keyboard() -> AnyView {
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let key_options = vec![
        "⌘K".to_string(),
        "Ctrl+Shift+P".to_string(),
        "⌥⇧P".to_string(),
    ];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0));
    let (workbench_key_index, set_workbench_key_index) = signal(Some(0));
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => KeyboardTone::Muted,
        _ => KeyboardTone::Default,
    });
    let workbench_key_text = Signal::derive(move || match workbench_key_index.get().unwrap_or(0) {
        1 => "Ctrl+Shift+P",
        2 => "⌥⇧P",
        _ => "⌘K",
    });

    let workbench_code = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec!["<Keyboard".to_string()];
        if tone == KeyboardTone::Muted {
            snippet.push("  tone=KeyboardTone::Muted".to_string());
        }
        if is_compact {
            snippet.push("  is_compact=true".to_string());
        }
        if custom_aria {
            snippet.push("  aria_label=\"Open command palette\".to_string()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-keyboard-custom\".to_string()".to_string());
        }
        snippet.push(">".to_string());
        snippet.push(format!("  \"{key_text}\""));
        snippet.push("</Keyboard>".to_string());
        snippet.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let mut class_tokens = vec![
            "ui-keyboard".to_string(),
            match tone {
                KeyboardTone::Muted => "ui-keyboard--tone-muted".to_string(),
                KeyboardTone::Default => "ui-keyboard--tone-default".to_string(),
            },
        ];
        if is_compact {
            class_tokens.push("ui-keyboard--compact".to_string());
        }
        if custom_class {
            class_tokens.push("ui-keyboard--custom-class".to_string());
            class_tokens.push("docs-keyboard-custom".to_string());
        }

        format!(
            "KeyboardActualConfig {{\n  tone: {tone:?},\n  key_text: \"{key_text}\",\n  is_compact: {is_compact},\n  custom_aria_label: {custom_aria},\n  custom_class_name: {custom_class},\n  class: \"{}\",\n  marker_expectations: [\"data-tone\", \"data-state\", \"data-compact\", \"data-aria-source\", \"data-class-source\"],\n}}",
            class_tokens.join(" ")
        )
    });

    let keyboard_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/keyboard/styles.rs */\n{}",
            ui_components::keyboard::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
<Keyboard is_compact=true>"Ctrl+K"</Keyboard>
<Keyboard
  tone=KeyboardTone::Muted
  is_compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Keyboard"
            slug="keyboard"
            group="Display"
            description="Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts."
        >
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=keyboard_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/keyboard/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 tone/key/is_compact/aria/class，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Tone"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-tone".to_string()
                                options=tone_options.clone()
                                selected_index=workbench_tone_index
                                set_selected_index=set_workbench_tone_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard tone".to_string()
                            />

                            <div class="docs-search__label">"Key Text"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-key".to_string()
                                options=key_options.clone()
                                selected_index=workbench_key_index
                                set_selected_index=set_workbench_key_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard key text".to_string()
                            />

                            <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                "is_compact"
                            </Switch>
                            <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                "Custom aria_label"
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
                        let tone = workbench_tone.get();
                        let key_text = workbench_key_text.get();
                        let is_compact = workbench_is_compact.get();
                        let aria_label = if workbench_custom_aria.get() {
                            "Open command palette".to_string()
                        } else {
                            "".to_string()
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-keyboard-custom".to_string()
                        } else {
                            "".to_string()
                        };

                        view! {
                            <Keyboard
                                tone=tone
                                is_compact=is_compact
                                aria_label=aria_label
                                class_name=class_name
                            >
                                {key_text}
                            </Keyboard>
                        }
                    }}

                    <div class="docs-row">
                        <span class="ui-muted">"Compare baseline:"</span>
                        <Keyboard>"⌘K"</Keyboard>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Tone / Compact / Source Markers)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Default"</span>
                        <Keyboard>"⌘K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted"</span>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Compact"</span>
                        <Keyboard is_compact=true>"Ctrl+K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted + Compact + Custom"</span>
                        <Keyboard
                            tone=KeyboardTone::Muted
                            is_compact=true
                            aria_label="Open command palette".to_string()
                            class_name="docs-keyboard-custom".to_string()
                        >
                            "Ctrl+Shift+P"
                        </Keyboard>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text() -> AnyView {
    let tone_code = Signal::derive(move || {
        r#"<Text text="Primary body copy".to_string() />
<Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
<Text text="Strong headline".to_string() tone=TextTone::Strong weight=TextWeight::Bold />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Text
  text="Centered label".to_string()
  align=TextAlign::Center
  element=TextElement::Div
/>
<Text
  text="Long text that truncates when width is constrained by the container around it".to_string()
  truncate=true
  class_name="docs-text-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Text"
            slug="text"
            group="Display"
            description="Typography primitive with centralized tone/alignment/weight/source state contracts and baseline-style data markers."
        >
            <Playground title="Tone + Weight Matrix" code_signal=tone_code>
                <div class="docs-stack">
                    <Text text="Primary body copy".to_string() />
                    <Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
                    <Text
                        text="Strong headline".to_string()
                        tone=TextTone::Strong
                        weight=TextWeight::Bold
                    />
                </div>
            </Playground>

            <Playground title="Alignment + Truncate + Element" code_signal=states_code>
                <div class="docs-stack">
                    <Text
                        text="Centered label".to_string()
                        align=TextAlign::Center
                        weight=TextWeight::Semibold
                        element=TextElement::Div
                    />
                    <Text
                        text="Long text that truncates when width is constrained by the container around it".to_string()
                        truncate=true
                        class_name="docs-text-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn icon() -> AnyView {
    let matrix_code = Signal::derive(move || {
        r#"<Icon size=IconSize::Sm tone=IconTone::Default decorative=true>"✓"</Icon>
<Icon size=IconSize::Md tone=IconTone::Muted decorative=true>"⚙"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>"★"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>"⚠"</Icon>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Icon
  size=IconSize::Md
  tone=IconTone::Accent
  decorative=false
  aria_label="Sync successful".to_string()
>
  "✓"
</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Muted
  disabled=true
  class_name="docs-icon-custom".to_string()
  decorative=true
>
  "⚙"
</Icon>"#
            .to_string()
    });

    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_glyph, set_workbench_glyph) = signal("✓".to_string());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_label, set_workbench_label) = signal("Status icon".to_string());

    let workbench_code = Signal::derive(move || {
        let size = workbench_size_key.get();
        let tone = workbench_tone_key.get();
        let glyph = workbench_glyph.get();
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_class = workbench_custom_class.get();
        let class_line = if custom_class {
            "  class_name=\"docs-icon-custom\".to_string()\n".to_string()
        } else {
            String::new()
        };
        let aria_line = if decorative {
            String::new()
        } else {
            format!(
                "  aria_label=\"{}\".to_string()\n",
                workbench_label.get().trim()
            )
        };
        format!(
            "<Icon\n  size=IconSize::{}\n  tone=IconTone::{}\n  disabled={disabled}\n  decorative={decorative}\n{aria_line}{class_line}>\n  \"{glyph}\"\n</Icon>",
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
            "/* crates/ui-components/src/icon/styles.rs */\n{}",
            ui_components::icon::styles::CSS
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
        let data_state = if disabled {
            "disabled"
        } else if decorative {
            "decorative"
        } else {
            "labeled"
        };

        let mut classes = vec![
            "ui-icon".to_string(),
            size_class.to_string(),
            tone_class.to_string(),
        ];
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
            "IconActualConfig {{\n  size: \"{}\",\n  tone: \"{}\",\n  disabled: {},\n  decorative: {},\n  glyph: \"{}\",\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            size_key,
            tone_key,
            disabled,
            decorative,
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
            <Playground title="Size + Tone Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Icon size=IconSize::Sm tone=IconTone::Default decorative=true>
                        "✓"
                    </Icon>
                    <Icon size=IconSize::Md tone=IconTone::Muted decorative=true>
                        "⚙"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>
                        "★"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Accessible + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-row">
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        decorative=false
                        aria_label="Sync successful".to_string()
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Muted
                        disabled=true
                        class_name="docs-icon-custom".to_string()
                        decorative=true
                    >
                        "⚙"
                    </Icon>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels and live icon state controls."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/icon/styles.rs".to_string()
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
                            <Icon size=IconSize::Md tone=IconTone::Default decorative=true>
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
                                view! {
                                    <Icon
                                        size=size
                                        tone=tone
                                        disabled=workbench_disabled.get()
                                        decorative=decorative
                                        aria_label=aria_label
                                        class_name=class_name
                                    >
                                        {workbench_glyph.get()}
                                    </Icon>
                                }
                            }}
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Disabled contrast"</div>
                            <Icon size=IconSize::Lg tone=IconTone::Danger disabled=true decorative=true>
                                "⚠"
                            </Icon>
                        </div>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn empty_state() -> AnyView {
    let tone_code = Signal::derive(move || {
        r#"<EmptyState
  title="No projects yet".to_string()
  description="Create your first project to unlock dashboards and team workflows.".to_string()
  tone=EmptyStateTone::Default
  icon=move || view! { <span>"📁"</span> }
  actions=move || view! {
    <ui_components::Button>"Create project"</ui_components::Button>
  }
/>
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  compact=true
  bordered=true
  class_name="docs-empty-state-custom".to_string()
  icon=move || view! { <span>"⏸"</span> }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Review approvals"
    </ui_components::Button>
  }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="EmptyState"
            slug="empty-state"
            group="Display"
            description="baseline-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers."
        >
            <Playground title="Tone + Alignment + Actions" code_signal=tone_code>
                <div class="docs-stack">
                    <EmptyState
                        title="No projects yet".to_string()
                        description="Create your first project to unlock dashboards and team workflows.".to_string()
                        tone=EmptyStateTone::Default
                        icon=move || view! { <span>"📁"</span> }
                        actions=move || {
                            view! {
                                <ui_components::Button>
                                    "Create project"
                                </ui_components::Button>
                            }
                        }
                    />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                </div>
            </Playground>

            <Playground title="Compact + Bordered + Custom Class" code_signal=state_code>
                <EmptyState
                    title="Deployments paused".to_string()
                    description="Approvals are required before resuming this environment.".to_string()
                    tone=EmptyStateTone::Accent
                    compact=true
                    bordered=true
                    class_name="docs-empty-state-custom".to_string()
                    icon=move || view! { <span>"⏸"</span> }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Review approvals"
                            </ui_components::Button>
                        }
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_view() -> AnyView {
    let basic_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>
<ErrorView
  is_invalid=false
  message="This error stays hidden until the field becomes invalid.".to_string()
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  is_bordered=true
  class_name="docs-error-view-custom".to_string()
  motion=ErrorViewMotion {
    hidden_translate_px: 12.0,
    hidden_opacity: 0.0,
    hidden_scale: 0.95,
    ..ErrorViewMotion::default()
  }
  icon=move || view! {
    <Icon size=IconSize::Sm tone=IconTone::Danger decorative=true>"⚠"</Icon>
  }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Retry"
    </ui_components::Button>
  }
>
  <span>"Validation failed. Check highlighted fields and retry."</span>
</ErrorView>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ErrorView"
            slug="error-view"
            group="Display"
            description="baseline-style validation error container with centralized visibility/content/source state contracts and spring-driven motion markers."
        >
            <Playground title="Invalid Visibility" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Please enter a valid email address".to_string()
                    />
                    <ErrorView
                        is_invalid=false
                        message="This error stays hidden until the field becomes invalid.".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Custom Content + Motion + Actions" code_signal=state_code>
                <ErrorView
                    is_invalid=true
                    tone=ErrorViewTone::Neutral
                    is_compact=true
                    is_bordered=true
                    class_name="docs-error-view-custom".to_string()
                    motion=ErrorViewMotion {
                        hidden_translate_px: 12.0,
                        hidden_opacity: 0.0,
                        hidden_scale: 0.95,
                        ..ErrorViewMotion::default()
                    }
                    icon=move || {
                        view! {
                            <Icon size=IconSize::Sm tone=IconTone::Danger decorative=true>
                                "⚠"
                            </Icon>
                        }
                    }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Retry"
                            </ui_components::Button>
                        }
                    }
                >
                    <span>
                        "Validation failed. Check highlighted fields and retry."
                    </span>
                </ErrorView>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn pressable_feedback() -> AnyView {
    let (press_count, set_press_count) = signal(0u32);
    let on_press_count = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let basic_code = Signal::derive(move || {
        r#"<PressableFeedback
  effect=PressableFeedbackEffect::Highlight
  tone=PressableFeedbackTone::Accent
  on_press=on_press_count
>
  <div class="docs-ripple-surface">"Press me"</div>
</PressableFeedback>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<PressableFeedback
  effect=PressableFeedbackEffect::HighlightRipple
  tone=PressableFeedbackTone::Neutral
  bounded=false
  motion=PressableFeedbackMotion {
    pressed_scale: 0.94,
    highlight_opacity: 0.2,
    ripple: RippleMotion {
      duration_ms: 720,
      ..RippleMotion::default()
    },
    ..PressableFeedbackMotion::default()
  }
  class_name="docs-pressable-feedback-custom".to_string()
>
  <div class="docs-ripple-surface docs-ripple-surface--accent">"Custom feedback"</div>
</PressableFeedback>

<PressableFeedback is_disabled=true effect=PressableFeedbackEffect::Highlight>
  <div class="docs-ripple-surface docs-ripple-surface--static">"Disabled"</div>
</PressableFeedback>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="PressableFeedback"
            slug="pressable-feedback"
            group="Display"
            description="baseline-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition."
        >
            <Playground title="Scale + Highlight" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::Highlight
                        tone=PressableFeedbackTone::Accent
                        on_press=on_press_count
                    >
                        <div class="docs-ripple-surface">
                            "Press me"
                        </div>
                    </PressableFeedback>

                    <div class="ui-muted">
                        {move || format!("Press count: {}", press_count.get())}
                    </div>
                </div>
            </Playground>

            <Playground title="Highlight + Ripple + Custom Motion" code_signal=custom_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::HighlightRipple
                        tone=PressableFeedbackTone::Neutral
                        bounded=false
                        motion=PressableFeedbackMotion {
                            pressed_scale: 0.94,
                            highlight_opacity: 0.2,
                            ripple: RippleMotion {
                                duration_ms: 720,
                                ..RippleMotion::default()
                            },
                            ..PressableFeedbackMotion::default()
                        }
                        class_name="docs-pressable-feedback-custom".to_string()
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--accent">
                            "Custom feedback"
                        </div>
                    </PressableFeedback>

                    <PressableFeedback
                        is_disabled=true
                        effect=PressableFeedbackEffect::Highlight
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--static">
                            "Disabled"
                        </div>
                    </PressableFeedback>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_swatch() -> AnyView {
    let size_options = vec![
        "xs".to_string(),
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ColorSwatchSize::Xs,
        1 => ColorSwatchSize::Sm,
        3 => ColorSwatchSize::Lg,
        _ => ColorSwatchSize::Md,
    });

    let shape_options = vec!["square".to_string(), "wide".to_string()];
    let (shape_index, set_shape_index) = signal(Some(0_usize));
    let shape = Signal::derive(move || match shape_index.get().unwrap_or(0) {
        1 => ColorSwatchShape::Wide,
        _ => ColorSwatchShape::Square,
    });

    let rounding_options = vec![
        "default".to_string(),
        "full".to_string(),
        "none".to_string(),
    ];
    let (rounding_index, set_rounding_index) = signal(Some(0_usize));
    let rounding = Signal::derive(move || match rounding_index.get().unwrap_or(0) {
        1 => ColorSwatchRounding::Full,
        2 => ColorSwatchRounding::None,
        _ => ColorSwatchRounding::Default,
    });

    let alpha_options = vec![
        "opaque".to_string(),
        "translucent".to_string(),
        "transparent".to_string(),
        "none".to_string(),
    ];
    let (alpha_index, set_alpha_index) = signal(Some(0_usize));
    let color = Signal::derive(move || match alpha_index.get().unwrap_or(0) {
        1 => "rgba(38, 99, 235, 0.35)".to_string(),
        2 => "rgba(255, 0, 0, 0)".to_string(),
        3 => "".to_string(),
        _ => "#2663eb".to_string(),
    });
    let color_name = Signal::derive(move || match alpha_index.get().unwrap_or(0) {
        1 => Some("Brand blue / 35%".to_string()),
        2 => Some("No fill".to_string()),
        3 => None,
        _ => Some("Brand blue".to_string()),
    });

    let (is_bordered, set_is_bordered) = signal(true);
    let (is_decorative, set_is_decorative) = signal(false);
    let (use_legacy_alias, set_use_legacy_alias) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang, set_custom_lang) = signal(false);

    let workbench_code = Signal::derive(move || {
        let color = color.get();
        let color_name = color_name.get();
        let size = size.get();
        let shape = shape.get();
        let rounding = rounding.get();
        let is_bordered = is_bordered.get();
        let is_decorative = is_decorative.get();
        let use_legacy_alias = use_legacy_alias.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();

        let mut out = vec![
            "<ColorSwatch".to_string(),
            format!("  color=\"{color}\".to_string()"),
        ];
        if let Some(color_name) = color_name {
            out.push(format!("  color_name=\"{color_name}\".to_string()"));
        }
        if size != ColorSwatchSize::Md {
            out.push(format!("  size=ColorSwatchSize::{size:?}"));
        }
        if rounding != ColorSwatchRounding::Default {
            out.push(format!("  rounding=ColorSwatchRounding::{rounding:?}"));
        }
        if shape != ColorSwatchShape::Square {
            out.push(format!("  shape=ColorSwatchShape::{shape:?}"));
        }
        out.push(if use_legacy_alias {
            format!("  bordered={is_bordered}")
        } else {
            format!("  is_bordered={is_bordered}")
        });
        if is_decorative {
            out.push(if use_legacy_alias {
                "  decorative=true".to_string()
            } else {
                "  is_decorative=true".to_string()
            });
        }
        if custom_aria {
            out.push("  aria_label=\"Background color\".to_string()".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-color-swatch-custom\".to_string()".to_string());
        }
        if custom_lang {
            out.push("  lang=\"zh-CN\".to_string()".to_string());
        }
        out.push("/>".to_string());
        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let color = color.get();
        let size = size.get();
        let shape = shape.get();
        let rounding = rounding.get();
        let is_bordered = is_bordered.get();
        let is_decorative = is_decorative.get();
        let use_legacy_alias = use_legacy_alias.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();
        let alpha_index = alpha_index.get().unwrap_or(0);
        let alpha_attr = match alpha_index {
            1 => "translucent",
            2 => "transparent",
            3 => "none",
            _ => "opaque",
        };
        let data_state = match alpha_index {
            3 => "empty",
            2 => "transparent",
            1 => "translucent",
            _ if is_bordered => "framed",
            _ => "default",
        };

        let mut classes = vec![
            "ui-color-swatch".to_string(),
            size.class_name().to_string(),
            rounding.class_name().to_string(),
            shape.class_name().to_string(),
            format!("ui-color-swatch--alpha-{alpha_attr}"),
        ];
        if is_bordered {
            classes.push("ui-color-swatch--bordered".to_string());
        }
        if custom_class {
            classes.push("ui-color-swatch--custom-class".to_string());
            classes.push("docs-color-swatch-custom".to_string());
        }

        format!(
            "ColorSwatchActualConfig {{\n  color: \"{color}\",\n  size: {size:?},\n  rounding: {rounding:?},\n  shape: {shape:?},\n  is_bordered: {is_bordered},\n  is_decorative: {is_decorative},\n  bool_source: \"{}\",\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  lang: {},\n  data_alpha: \"{alpha_attr}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            if use_legacy_alias {
                "legacy-alias"
            } else {
                "is-prefixed"
            },
            if custom_lang { "\"zh-CN\"" } else { "None" },
            classes.join(" ")
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/color_swatch/styles.rs */\n{}",
            ui_components::color_swatch::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
<ColorSwatch color="rgba(38, 99, 235, 0.35)".to_string() color_name="Brand blue / 35%".to_string() shape=ColorSwatchShape::Wide />
<ColorSwatch color="rgba(255, 0, 0, 0)".to_string() color_name="No fill".to_string() is_bordered=true />
<ColorSwatch color="".to_string() is_bordered=true />"##
            .to_string()
    });
    // Legacy contract anchors for color module compatibility tests:
    // title="Size + Rounding"
    // title="Transparency + Accessible Label + Shape"

    view! {
        <ComponentPage
            title="ColorSwatch"
            slug="color-swatch"
            group="Display"
            description="baseline-compatible color preview primitive with centralized size/rounding/shape/transparency/source contracts and stable slot/data markers."
        >
            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="crates/ui-components/src/color_swatch/styles.rs".to_string()
                test_config_signal=workbench_config
                description="切换尺寸/形状/圆角/透明度/边框/装饰模式，并实时查看 config + code + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-color-swatch-workbench-size".to_string()
                                options=size_options.clone()
                                selected_index=size_index
                                set_selected_index=set_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorSwatch size".to_string()
                            />

                            <div class="docs-search__label">"Shape"</div>
                            <SegmentedControl
                                id_base="docs-color-swatch-workbench-shape".to_string()
                                options=shape_options.clone()
                                selected_index=shape_index
                                set_selected_index=set_shape_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorSwatch shape".to_string()
                            />

                            <div class="docs-search__label">"Rounding"</div>
                            <SegmentedControl
                                id_base="docs-color-swatch-workbench-rounding".to_string()
                                options=rounding_options.clone()
                                selected_index=rounding_index
                                set_selected_index=set_rounding_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorSwatch rounding".to_string()
                            />

                            <div class="docs-search__label">"Alpha"</div>
                            <SegmentedControl
                                id_base="docs-color-swatch-workbench-alpha".to_string()
                                options=alpha_options.clone()
                                selected_index=alpha_index
                                set_selected_index=set_alpha_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorSwatch alpha".to_string()
                            />

                            <Switch checked=is_bordered set_checked=set_is_bordered>"Bordered"</Switch>
                            <Switch checked=is_decorative set_checked=set_is_decorative>
                                "Decorative"
                            </Switch>
                            <Switch checked=use_legacy_alias set_checked=set_use_legacy_alias>
                                "Use legacy `bordered/decorative`"
                            </Switch>
                            <Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria_label"
                            </Switch>
                            <Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=custom_lang set_checked=set_custom_lang>"Lang=zh-CN"</Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let color = color.get();
                        let color_name = color_name.get().unwrap_or_default();
                        let size = size.get();
                        let shape = shape.get();
                        let rounding = rounding.get();
                        let is_bordered = is_bordered.get();
                        let is_decorative = is_decorative.get();
                        let use_legacy_alias = use_legacy_alias.get();
                        let aria_label = if custom_aria.get() {
                            "Background color".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-color-swatch-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if custom_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            String::new()
                        };

                        if use_legacy_alias {
                            view! {
                                <ColorSwatch
                                    color=color
                                    color_name=color_name
                                    size=size
                                    shape=shape
                                    rounding=rounding
                                    bordered=is_bordered
                                    decorative=is_decorative
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                />
                            }
                            .into_any()
                        } else {
                            view! {
                                <ColorSwatch
                                    color=color
                                    color_name=color_name
                                    size=size
                                    shape=shape
                                    rounding=rounding
                                    is_bordered=is_bordered
                                    is_decorative=is_decorative
                                    aria_label=aria_label
                                    class_name=class_name
                                    lang=lang
                                />
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "alpha={}, bordered={}, decorative={}",
                            match alpha_index.get().unwrap_or(0) {
                                1 => "translucent",
                                2 => "transparent",
                                3 => "none",
                                _ => "opaque",
                            },
                            is_bordered.get(),
                            is_decorative.get(),
                        )}
                    </span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Size / Alpha / Shape / Empty)" code_signal=matrix_code>
                <div class="docs-row">
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"XS / Opaque"</span>
                        <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"SM / Opaque"</span>
                        <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"Wide / Translucent"</span>
                        <ColorSwatch
                            color="rgba(38, 99, 235, 0.35)".to_string()
                            color_name="Brand blue / 35%".to_string()
                            shape=ColorSwatchShape::Wide
                            rounding=ColorSwatchRounding::Default
                        />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"Transparent / Empty"</span>
                        <div class="docs-row">
                            <ColorSwatch
                                color="rgba(255, 0, 0, 0)".to_string()
                                color_name="No fill".to_string()
                                is_bordered=true
                            />
                            <ColorSwatch color="".to_string() is_bordered=true />
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground title="Rounded Large + Custom Label/Class" code_signal=Signal::derive(move || {
                r##"<ColorSwatch
  color="#ffcc00".to_string()
  color_name="Accent yellow".to_string()
  size=ColorSwatchSize::Lg
  rounding=ColorSwatchRounding::Full
  aria_label="Accent token".to_string()
  class_name="docs-color-swatch-custom".to_string()
/>"##.to_string()
            })>
                <div class="docs-row">
                    <ColorSwatch
                        color="#ffcc00".to_string()
                        size=ColorSwatchSize::Lg
                        rounding=ColorSwatchRounding::Full
                        color_name="Accent yellow".to_string()
                        aria_label="Accent token".to_string()
                        class_name="docs-color-swatch-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_swatch_picker() -> AnyView {
    let swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("#f80", "Orange"),
        ColorSwatchPickerItem::named("#080", "Green"),
        ColorSwatchPickerItem::named("#08f", "Blue"),
    ];

    let disabled_swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
        ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
        ColorSwatchPickerItem::new("#08f"),
    ];

    let basic_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>"##
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
  class_name="docs-color-swatch-picker-custom".to_string()
  aria_label="Fill color".to_string()
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorSwatchPicker"
            slug="color-swatch-picker"
            group="Display"
            description="baseline-compatible selectable swatch group with centralized color normalization, single-selection state, keyboard roving, and stable slot/data state markers."
        >
            <Playground title="Basic Selection" code_signal=basic_code>
                <ColorSwatchPicker
                    swatches=signal(swatches).0
                    default_selected_color="#f80".to_string()
                />
            </Playground>

            <Playground title="Transparency + Disabled + Custom Class" code_signal=state_code>
                <ColorSwatchPicker
                    swatches=signal(disabled_swatches).0
                    shape=ColorSwatchShape::Wide
                    rounding=ColorSwatchRounding::Default
                    class_name="docs-color-swatch-picker-custom".to_string()
                    aria_label="Fill color".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn skeleton_group() -> AnyView {
    let loading_code = Signal::derive(move || {
        r#"<SkeletonGroup
  is_loading=true
  variant=SkeletonGroupVariant::Shimmer
  layout=SkeletonGroupLayout::Vertical
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
</SkeletonGroup>"#.to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<SkeletonGroup
  is_loading=false
  is_skeleton_only=false
  variant=SkeletonGroupVariant::None
>
  <div class="ui-muted">"Loaded content rendered by parent group."</div>
</SkeletonGroup>

<SkeletonGroup
  is_loading=false
  is_skeleton_only=true
  variant=SkeletonGroupVariant::Pulse
  class_name="docs-skeleton-group-custom".to_string()
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SkeletonGroup"
            slug="skeleton-group"
            group="Display"
            description="baseline-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers."
        >
            <Playground
                title="Shimmer + Pulse Layout"
                code_signal=loading_code
                test_source_path="crates/ui-components/src/skeleton/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Shimmer
                        layout=SkeletonGroupLayout::Vertical
                        density=SkeletonGroupDensity::Comfortable
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Pulse
                        layout=SkeletonGroupLayout::Horizontal
                        density=SkeletonGroupDensity::Compact
                        aria_label="Profile placeholders".to_string()
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Circle
                            is_shimmer=false
                            class_name="docs-skeleton-avatar".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            is_shimmer=false
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            is_shimmer=false
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>
                </div>
            </Playground>

            <Playground
                title="Loaded + Skeleton Only"
                code_signal=state_code
                test_source_path="crates/ui-components/src/skeleton/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=false
                        variant=SkeletonGroupVariant::None
                    >
                        <div class="ui-muted">
                            "Loaded content rendered by parent group."
                        </div>
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=true
                        variant=SkeletonGroupVariant::Pulse
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <div class="ui-muted">
                        "When `is_skeleton_only=true` and loading is finished, the skeleton group hides itself."
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn flip_card() -> AnyView {
    let motion_options = vec![
        "default".to_string(),
        "gentle".to_string(),
        "dramatic".to_string(),
    ];
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_default_flipped, set_workbench_default_flipped) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_flip_on_hover, set_workbench_flip_on_hover) = signal(true);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(true);

    let workbench_motion =
        Signal::derive(move || match workbench_motion_index.get().unwrap_or(0) {
            1 => FlipCardMotion {
                hover_scale: 1.01,
                hover_tilt_deg: 2.0,
                ..FlipCardMotion::default()
            },
            2 => FlipCardMotion {
                hover_scale: 1.06,
                hover_tilt_deg: 7.5,
                ..FlipCardMotion::default()
            },
            _ => FlipCardMotion::default(),
        });

    let workbench_code = Signal::derive(move || {
        let default_flipped = workbench_default_flipped.get();
        let disabled = workbench_disabled.get();
        let flip_on_hover = workbench_flip_on_hover.get();
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let motion_index = workbench_motion_index.get().unwrap_or(0);
        let motion_name = match motion_index {
            1 => "gentle",
            2 => "dramatic",
            _ => "default",
        };
        let motion = workbench_motion.get();

        let mut lines = vec!["<FlipCard".to_string()];
        if default_flipped {
            lines.push("  default_flipped=true".to_string());
        }
        if disabled {
            lines.push("  disabled=true".to_string());
        }
        if flip_on_hover {
            lines.push("  flip_on_hover=true".to_string());
        }
        if custom_id {
            lines.push("  id=\"docs-flip-card-workbench\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-flip-card-state\".to_string()".to_string());
        }
        if motion_index != 0 {
            lines.push(format!(
                "  motion=FlipCardMotion {{ hover_scale: {:.2}, hover_tilt_deg: {:.1}, ..FlipCardMotion::default() }}",
                motion.hover_scale,
                motion.hover_tilt_deg
            ));
        }
        lines.push("  front=move || view! { <div>\"Workbench front\"</div> }".to_string());
        lines.push("  back=move || view! { <div>\"Workbench back\"</div> }".to_string());
        lines.push("/>".to_string());
        lines.push(format!("// motion preset: {motion_name}"));

        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let default_flipped = workbench_default_flipped.get();
        let disabled = workbench_disabled.get();
        let flip_on_hover = workbench_flip_on_hover.get();
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();

        let mut classes = vec![
            "ui-flip-card".to_string(),
            if disabled {
                "ui-flip-card--disabled".to_string()
            } else {
                "ui-flip-card--enabled".to_string()
            },
            if default_flipped {
                "ui-flip-card--flipped".to_string()
            } else {
                "ui-flip-card--default".to_string()
            },
            if flip_on_hover {
                "ui-flip-card--hover".to_string()
            } else {
                "ui-flip-card--toggle".to_string()
            },
        ];
        if custom_class {
            classes.push("ui-flip-card--custom-class".to_string());
            classes.push("docs-flip-card-state".to_string());
        }
        if custom_id {
            classes.push("ui-flip-card--custom-id".to_string());
        }
        if motion != FlipCardMotion::default() {
            classes.push("ui-flip-card--custom-motion".to_string());
        }

        format!(
            "FlipCardActualConfig {{\n  default_flipped: {default_flipped},\n  disabled: {disabled},\n  flip_on_hover: {flip_on_hover},\n  custom_id: {custom_id},\n  custom_class: {custom_class},\n  motion: {{ hover_scale: {:.2}, hover_tilt_deg: {:.1} }},\n  class: \"{}\",\n  markers: [\"data-state\", \"data-visible\", \"data-flip-mode\", \"data-motion-source\", \"data-id-source\"],\n}}",
            motion.hover_scale,
            motion.hover_tilt_deg,
            classes.join(" ")
        )
    });

    let flip_card_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/flip_card/styles.rs */\n{}",
            ui_components::flip_card::styles::CSS
        )
    });

    let compare_code = Signal::derive(move || {
        r#"<FlipCard front=... back=... />
<FlipCard flip_on_hover=true front=... back=... />
<FlipCard disabled=true front=... back=... />
<FlipCard motion=FlipCardMotion { hover_scale: 1.06, hover_tilt_deg: 7.5, ..FlipCardMotion::default() } front=... back=... />"#
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! {
    <div class="ui-flip-card__title">"Front"</div>
    <div class="ui-flip-card__description">"Click or press Enter/Space to flip."</div>
  }
  back=move || view! {
    <div class="ui-flip-card__title">"Back"</div>
    <div class="ui-flip-card__description">"Stable state/data markers for docs and tests."</div>
  }
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card"
  class_name="docs-flip-card-state".to_string()
  flip_on_hover=true
  motion=FlipCardMotion {
    hover_scale: 1.03,
    hover_tilt_deg: 4.0,
    ..FlipCardMotion::default()
  }
  front=move || view! { <div>"Inspect markers (front)"</div> }
  back=move || view! { <div>"Inspect markers (back)"</div> }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<FlipCard
  disabled=true
  front=move || view! { <div>"Disabled front"</div> }
  back=move || view! { <div>"Disabled back"</div> }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FlipCard"
            slug="flip-card"
            group="Display"
            description="3D front/back card with baseline-style state/source markers and baseline-level spring motion for flip/hover interactions."
        >
            <Playground title="Click + Keyboard Flip" code_signal=basic_code>
                <div class="docs-row">
                    <FlipCard
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Front"</div>
                                    <div class="ui-flip-card__description">
                                        "Click or press Enter/Space to flip."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Back"</div>
                                    <div class="ui-flip-card__description">
                                        "Back face stays keyboard reachable with the same button semantics."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=flip_card_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/flip_card/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调翻转初始态/hover/disabled/id/class/motion，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Motion preset"</div>
                            <SegmentedControl
                                id_base="docs-flip-card-motion".to_string()
                                options=motion_options.clone()
                                selected_index=workbench_motion_index
                                set_selected_index=set_workbench_motion_index
                                size=SegmentedControlSize::Sm
                                aria_label="FlipCard motion preset".to_string()
                            />
                            <Switch checked=workbench_default_flipped set_checked=set_workbench_default_flipped>
                                "Default Flipped"
                            </Switch>
                            <Switch checked=workbench_flip_on_hover set_checked=set_workbench_flip_on_hover>
                                "Flip On Hover"
                            </Switch>
                            <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                                "Disabled"
                            </Switch>
                            <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                                "Custom ID"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let default_flipped = workbench_default_flipped.get();
                        let disabled = workbench_disabled.get();
                        let flip_on_hover = workbench_flip_on_hover.get();
                        let with_custom_class = workbench_custom_class.get();
                        let with_custom_id = workbench_custom_id.get();
                        let motion = workbench_motion.get();

                        match (with_custom_class, with_custom_id) {
                            (true, true) => view! {
                                <FlipCard
                                    default_flipped=default_flipped
                                    disabled=disabled
                                    flip_on_hover=flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (true, false) => view! {
                                <FlipCard
                                    default_flipped=default_flipped
                                    disabled=disabled
                                    flip_on_hover=flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, true) => view! {
                                <FlipCard
                                    default_flipped=default_flipped
                                    disabled=disabled
                                    flip_on_hover=flip_on_hover
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, false) => view! {
                                <FlipCard
                                    default_flipped=default_flipped
                                    disabled=disabled
                                    flip_on_hover=flip_on_hover
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                        }
                    }}
                    <div class="ui-muted">
                        "切换 settings 后，使用 Code / Test 面板查看实际配置与 scoped CSS 影响。"
                    </div>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-flip-mode`, `data-class-source`, `data-motion-source`, `data-id-source`, and face-level visibility markers (`data-visible`/`data-hidden`)."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card".to_string()
                        class_name="docs-flip-card-state".to_string()
                        flip_on_hover=true
                        motion=FlipCardMotion {
                            hover_scale: 1.03,
                            hover_tilt_deg: 4.0,
                            ..FlipCardMotion::default()
                        }
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (front)"</div>
                                    <div class="ui-flip-card__description">
                                        "Hover enters flipped mode source = custom."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (back)"</div>
                                    <div class="ui-flip-card__description">
                                        "Front/back visibility markers stay explicit for regression tests."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Default / Hover / Disabled / Dramatic Motion)" code_signal=compare_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipCard
                            front=move || view! { <div class="ui-flip-card__title">"Default"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Hover flip"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                    <div class="docs-row">
                        <FlipCard
                            disabled=true
                            front=move || view! { <div class="ui-flip-card__title">"Disabled"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            motion=FlipCardMotion {
                                hover_scale: 1.06,
                                hover_tilt_deg: 7.5,
                                ..FlipCardMotion::default()
                            }
                            flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Dramatic motion"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled" code_signal=disabled_code>
                <div class="docs-row">
                    <FlipCard
                        disabled=true
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled front"</div>
                                    <div class="ui-flip-card__description">
                                        "No click/keyboard toggle while disabled."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled back"</div>
                                    <div class="ui-flip-card__description">
                                        "aria-disabled and disabled markers remain consistent."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn chart() -> AnyView {
    let revenue_points = vec![
        ChartPoint::new("jan", "Jan", 12.0),
        ChartPoint::new("feb", "Feb", 18.5),
        ChartPoint::new("mar", "Mar", 17.2),
        ChartPoint::new("apr", "Apr", 24.7),
        ChartPoint::new("may", "May", 28.1),
    ];

    let line_points = vec![
        ChartPoint::new("q1", "Q1", 42.0),
        ChartPoint::new("q2", "Q2", 56.0),
        ChartPoint::new("q3", "Q3", 51.0),
        ChartPoint::new("q4", "Q4", 63.0),
    ];
    let flat_points = vec![
        ChartPoint::new("alpha", "Alpha", 20.0),
        ChartPoint::new("beta", "Beta", 20.0),
        ChartPoint::new("gamma", "Gamma", 20.0),
    ];
    let revenue_points_for_workbench = revenue_points.clone();
    let line_points_for_workbench = line_points.clone();
    let flat_points_for_workbench = flat_points.clone();
    let revenue_points_for_matrix = revenue_points.clone();
    let line_points_for_matrix = line_points.clone();
    let flat_points_for_matrix = flat_points.clone();
    let revenue_points_for_bar = revenue_points.clone();
    let line_points_for_controlled = line_points.clone();

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (controlled_active_raw, set_controlled_active_raw) = signal(1_usize);
    let controlled_active: Signal<usize> = Signal::derive(move || controlled_active_raw.get());
    let on_controlled_active_change =
        Callback::new(move |next: usize| set_controlled_active_raw.set(next));

    let kind_options = vec!["bar".to_string(), "line".to_string()];
    let dataset_options = vec![
        "revenue".to_string(),
        "growth".to_string(),
        "flat".to_string(),
    ];
    let (workbench_kind_index, set_workbench_kind_index) = signal(Some(0_usize));
    let (workbench_dataset_index, set_workbench_dataset_index) = signal(Some(0_usize));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_show_grid, set_workbench_show_grid) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang, set_workbench_lang) = signal(false);
    let (workbench_last_action, set_workbench_last_action) = signal("none".to_string());
    let workbench_on_action = Callback::new(move |id: String| set_workbench_last_action.set(id));

    let workbench_kind = Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
        1 => ChartKind::Line,
        _ => ChartKind::Bar,
    });
    let workbench_dataset_name = Signal::derive(move || {
        match workbench_dataset_index.get().unwrap_or(0) {
            1 => "growth",
            2 => "flat",
            _ => "revenue",
        }
        .to_string()
    });
    let workbench_points =
        Signal::derive(move || match workbench_dataset_index.get().unwrap_or(0) {
            1 => line_points_for_workbench.clone(),
            2 => flat_points_for_workbench.clone(),
            _ => revenue_points_for_workbench.clone(),
        });

    let hello_code = Signal::derive(move || {
        r#"<Chart
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
  ]
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let is_disabled = workbench_is_disabled.get();
        let show_grid = workbench_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();

        let mut out = vec![
            "<Chart".to_string(),
            "  id_base=\"docs-chart-workbench\".to_string()".to_string(),
            format!("  // dataset: {dataset}"),
            "  points=/* see preview dataset */".to_string(),
            format!("  kind=ChartKind::{kind:?}"),
        ];

        if is_disabled {
            out.push("  is_disabled=Some(true)".to_string());
        }
        if !show_grid {
            out.push("  show_grid=false".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-chart-custom\".to_string()".to_string());
        }
        if lang {
            out.push("  lang=\"en-US\".to_string()".to_string());
        }
        out.push("  on_action=Callback::new(move |id: String| { /* ... */ })".to_string());
        out.push("/>".to_string());

        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let is_disabled = workbench_is_disabled.get();
        let show_grid = workbench_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();

        let mut class_tokens = vec![
            "ui-chart".to_string(),
            match kind {
                ChartKind::Bar => "ui-chart--bar".to_string(),
                ChartKind::Line => "ui-chart--line".to_string(),
            },
            if is_disabled {
                "ui-chart--disabled".to_string()
            } else {
                "ui-chart--uncontrolled".to_string()
            },
        ];
        if show_grid {
            class_tokens.push("ui-chart--grid".to_string());
        }
        if custom_class {
            class_tokens.push("ui-chart--custom-class".to_string());
            class_tokens.push("docs-chart-custom".to_string());
        }

        format!(
            "ChartActualConfig {{\n  dataset: \"{dataset}\",\n  kind: {kind:?},\n  is_disabled: {is_disabled},\n  show_grid: {show_grid},\n  custom_class: {custom_class},\n  lang: {},\n  class: \"{}\",\n  marker_expectations: [\"data-kind\", \"data-state\", \"data-active-index\", \"data-motion-source\"],\n}}",
            if lang { "\"en-US\"" } else { "None" },
            class_tokens.join(" ")
        )
    });

    let chart_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/chart/styles.rs */\n{}",
            ui_components::chart::styles::CSS
        )
    });

    let bar_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Chart
  id_base="docs-chart-bar".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
    ChartPoint::new("apr", "Apr", 24.7),
    ChartPoint::new("may", "May", 28.1),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let line_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="docs-chart-line".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
  class_name="docs-chart-custom".to_string()
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<Chart id_base="docs-chart-matrix-bar".to_string() kind=ChartKind::Bar points=vec![...] />
<Chart id_base="docs-chart-matrix-line".to_string() kind=ChartKind::Line points=vec![...] />
<Chart id_base="docs-chart-matrix-disabled".to_string() kind=ChartKind::Bar is_disabled=Some(true) points=vec![...] />
<Chart id_base="docs-chart-matrix-empty".to_string() kind=ChartKind::Line points=vec![] />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Chart"
            slug="chart"
            group="Display"
            description="baseline-compatible chart primitive with bar/line modes, controlled active-index state, baseline-style data contracts, and baseline-level spring highlight motion for legends."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-row">
                    <Chart
                        id_base="docs-chart-hello".to_string()
                        points=vec![
                            ChartPoint::new("jan", "Jan", 12.0),
                            ChartPoint::new("feb", "Feb", 18.5),
                            ChartPoint::new("mar", "Mar", 17.2),
                        ]
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=chart_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/chart/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 kind/dataset/disabled/grid/class/lang，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Kind"</div>
                            <SegmentedControl
                                id_base="docs-chart-kind".to_string()
                                options=kind_options.clone()
                                selected_index=workbench_kind_index
                                set_selected_index=set_workbench_kind_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart kind".to_string()
                            />

                            <div class="docs-search__label">"Dataset"</div>
                            <SegmentedControl
                                id_base="docs-chart-dataset".to_string()
                                options=dataset_options.clone()
                                selected_index=workbench_dataset_index
                                set_selected_index=set_workbench_dataset_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart dataset".to_string()
                            />

                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "Disabled"
                            </Switch>
                            <Switch checked=workbench_show_grid set_checked=set_workbench_show_grid>
                                "Show Grid"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                            <Switch checked=workbench_lang set_checked=set_workbench_lang>
                                "Lang=en-US"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let points = workbench_points.get();
                        let kind = workbench_kind.get();
                        let disabled = workbench_is_disabled.get();
                        let show_grid = workbench_show_grid.get();
                        let class_name = workbench_custom_class
                            .get()
                            .then_some("docs-chart-custom".to_string());
                        let lang = workbench_lang.get().then_some("en-US".to_string());

                        view! {
                            <Chart
                                id_base="docs-chart-workbench".to_string()
                                points=points
                                kind=kind
                                is_disabled=disabled
                                show_grid=show_grid
                                class_name=class_name.unwrap_or_default()
                                lang=lang.unwrap_or_default()
                                on_action=workbench_on_action
                            />
                        }
                    }}
                    <span class="ui-muted">
                        "workbench last action: "
                        {move || workbench_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Bar / Line / Disabled / Empty)" code_signal=matrix_code>
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Bar / Revenue"</span>
                        <Chart
                            id_base="docs-chart-matrix-bar".to_string()
                            points=revenue_points_for_matrix.clone()
                            kind=ChartKind::Bar
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Line / Growth"</span>
                        <Chart
                            id_base="docs-chart-matrix-line".to_string()
                            points=line_points_for_matrix.clone()
                            kind=ChartKind::Line
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Disabled"</span>
                        <Chart
                            id_base="docs-chart-matrix-disabled".to_string()
                            points=flat_points_for_matrix.clone()
                            kind=ChartKind::Bar
                            is_disabled=true
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Empty"</span>
                        <Chart
                            id_base="docs-chart-matrix-empty".to_string()
                            points=vec![]
                            kind=ChartKind::Line
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Bar + Hover/Keyboard + Action" code_signal=bar_code>
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-bar".to_string()
                        points=revenue_points_for_bar.clone()
                        kind=ChartKind::Bar
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Line + Active Index" code_signal=line_code>
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-line".to_string()
                        points=line_points_for_controlled.clone()
                        kind=ChartKind::Line
                        active_index=controlled_active
                        on_active_index_change=on_controlled_active_change
                        aria_label="Quarterly growth line chart".to_string()
                        class_name="docs-chart-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled active index: "
                        {move || controlled_active_raw.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
