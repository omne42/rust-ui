use super::*;

pub(crate) fn heading() -> AnyView {
    let (workbench_level_key, set_workbench_level_key) = signal("h2".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_truncate, set_workbench_truncate) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_level = Signal::derive(move || match workbench_level_key.get().as_str() {
        "h1" => HeadingLevel::H1,
        "h3" => HeadingLevel::H3,
        "h4" => HeadingLevel::H4,
        "h5" => HeadingLevel::H5,
        "h6" => HeadingLevel::H6,
        _ => HeadingLevel::H2,
    });
    let workbench_tone = Signal::derive(move || match workbench_tone_key.get().as_str() {
        "muted" => HeadingTone::Muted,
        "strong" => HeadingTone::Strong,
        _ => HeadingTone::Default,
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench section heading".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-heading-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code =
        Signal::derive(move || r#"<Heading>"Project Overview"</Heading>"#.to_string());

    let workbench_code = Signal::derive(move || {
        let level_expr = match workbench_level.get() {
            HeadingLevel::H1 => "HeadingLevel::H1",
            HeadingLevel::H2 => "HeadingLevel::H2",
            HeadingLevel::H3 => "HeadingLevel::H3",
            HeadingLevel::H4 => "HeadingLevel::H4",
            HeadingLevel::H5 => "HeadingLevel::H5",
            HeadingLevel::H6 => "HeadingLevel::H6",
        };
        let tone_expr = match workbench_tone.get() {
            HeadingTone::Default => "HeadingTone::Default",
            HeadingTone::Muted => "HeadingTone::Muted",
            HeadingTone::Strong => "HeadingTone::Strong",
        };

        format!(
            "<Heading\n  level={level_expr}\n  tone={tone_expr}\n  truncate={}\n  aria_label={}\n  class_name={}\n>\n  \"Quarterly product status and delivery timeline\"\n</Heading>",
            bool_word(workbench_truncate.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "HeadingActualConfig {{\n  level: {:?},\n  tone: {:?},\n  truncate: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_level.get(),
            workbench_tone.get(),
            workbench_truncate.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Heading level=HeadingLevel::H1>
  "Strategic Dashboard"
</Heading>
<Heading level=HeadingLevel::H3>
  "Team Capacity"
</Heading>
<Heading
  level=HeadingLevel::H5
  tone=HeadingTone::Muted
>
  "Delivery Risks"
</Heading>
<Heading
  level=HeadingLevel::H4
  tone=HeadingTone::Strong
  truncate=true
  aria_label="Truncated heading".to_string()
  class_name="docs-heading-custom".to_string()
>
  "A very long heading that is intentionally truncated for dense dashboard cards"
</Heading>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Heading"
            slug="heading"
            group="Layout"
            description="baseline-style semantic heading (`<h1>`..`<h6>`) with centralized level/tone/truncate contracts."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-stack">
                    <Heading>"Project Overview"</Heading>
                    <p class="ui-muted">
                        "Use Heading to keep section hierarchy readable in cards and dashboards."
                    </p>
                </div>
            </Playground>

            <Playground title="Strong + Truncate + Custom Aria/Class"
                code_signal=workbench_code test_config_signal=workbench_actual_config controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="heading-workbench-controls">
                        <label class="docs-search__label">
                            "Level"
                            <select
                                prop:value=move || workbench_level_key.get()
                                on:change=move |ev| set_workbench_level_key.set(event_target_value(&ev))
                            >
                                <option value="h1">"H1"</option>
                                <option value="h2">"H2"</option>
                                <option value="h3">"H3"</option>
                                <option value="h4">"H4"</option>
                                <option value="h5">"H5"</option>
                                <option value="h6">"H6"</option>
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
                                <option value="strong">"Strong"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_truncate.get()
                                on:change=move |ev| set_workbench_truncate.set(event_target_checked(&ev))
                            />
                            " truncate"
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
                    </div>
                }
            >
                <View border=ViewBorder::Subtle radius=ViewRadius::Md class_name="docs-heading-workbench".to_string()>
                    <Heading
                        level=workbench_level.get()
                        tone=workbench_tone.get()
                        truncate=workbench_truncate.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    >
                        "Quarterly product status and delivery timeline for all active teams"
                    </Heading>
                </View>
            </Playground>

            <Playground title="Heading Levels + Tone" code_signal=matrix_code>
                <div class="docs-stack">
                    <Heading level=HeadingLevel::H1>
                        "Strategic Dashboard"
                    </Heading>
                    <Heading level=HeadingLevel::H3>
                        "Team Capacity"
                    </Heading>
                    <Heading level=HeadingLevel::H5 tone=HeadingTone::Muted>
                        "Delivery Risks"
                    </Heading>
                    <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                        <Heading
                            level=HeadingLevel::H4
                            tone=HeadingTone::Strong
                            truncate=true
                            aria_label="Truncated heading".to_string()
                            class_name="docs-heading-custom".to_string()
                        >
                            "A very long heading that is intentionally truncated for dense dashboard cards"
                        </Heading>
                    </View>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
