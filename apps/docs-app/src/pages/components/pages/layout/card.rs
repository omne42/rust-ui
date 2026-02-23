use super::*;

pub(crate) fn card() -> AnyView {
    let (workbench_variant_key, set_workbench_variant_key) = signal("default".to_string());
    let (workbench_padded, set_workbench_padded) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant = Signal::derive(move || match workbench_variant_key.get().as_str() {
        "muted" => CardVariant::Muted,
        "outline" => CardVariant::Outline,
        _ => CardVariant::Default,
    });

    let workbench_code = Signal::derive(move || {
        let variant_key = workbench_variant_key.get();
        let padded = workbench_padded.get();
        let custom_class = workbench_custom_class.get();

        let variant_line = match variant_key.as_str() {
            "muted" => "  variant=CardVariant::Muted\n",
            "outline" => "  variant=CardVariant::Outline\n",
            _ => "",
        };
        let padded_line = if padded { "" } else { "  padded=false\n" };
        let class_line = if custom_class {
            "  class_name=\"docs-card-custom\".into()\n"
        } else {
            ""
        };

        format!(
            "<Card\n{variant_line}{padded_line}{class_line}>\n  <div>\"Workbench content\"</div>\n</Card>"
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/card/styles.rs */\n{}",
            ui_layout::card::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let padded = workbench_padded.get();
        let custom_class = workbench_custom_class.get();
        let class_name = if custom_class {
            Some("docs-card-custom")
        } else {
            None
        };

        let mut classes = vec![
            "ui-card".to_string(),
            variant.class_name().into(),
            if padded {
                "ui-card--padded".to_string()
            } else {
                "ui-card--no-padding".to_string()
            },
        ];
        if custom_class {
            classes.push("docs-card-custom".to_string());
        }

        format!(
            "CardActualConfig {{\n  variant: {variant:?},\n  padded: {padded},\n  custom_class: {custom_class},\n  class_name: {class_name:?},\n  data_variant: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            variant.as_str(),
            if padded { "padded" } else { "flush" },
            classes.join(" "),
        )
    });

    let variants_code = Signal::derive(move || {
        r#"<Card variant=CardVariant::Default>"Default"</Card>
<Card variant=CardVariant::Muted>"Muted"</Card>
<Card variant=CardVariant::Outline>"Outline"</Card>"#
            .to_string()
    });

    let padding_code = Signal::derive(move || {
        r#"<Card padded=true>
  <div>"Padded content"</div>
</Card>
<Card padded=false>
  <div>"Flush content"</div>
</Card>"#
            .to_string()
    });

    let custom_class_code = Signal::derive(move || {
        r#"<Card class_name="docs-card-custom".to_string()>
  <div>"Custom class marker"</div>
</Card>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Card"
            slug="card"
            group="Layout"
            description="A token-styled surface with centralized variant/padding state attrs."
        >
            <Playground title="Variants" code_signal=variants_code>
                <div class="docs-row">
                    <Card variant=CardVariant::Default>
                        <div class="docs-stack">
                            <div>"Default"</div>
                            <div class="ui-muted">"Uses tokens for bg/border/shadow."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Muted>
                        <div class="docs-stack">
                            <div>"Muted"</div>
                            <div class="ui-muted">"Lower-contrast surface."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Outline>
                        <div class="docs-stack">
                            <div>"Outline"</div>
                            <div class="ui-muted">"Border-forward style."</div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for variant, padding, and class-source contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/card/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="card-workbench-controls">
                        <label class="docs-search__label">
                            "Variant"
                            <select
                                prop:value=move || workbench_variant_key.get()
                                on:change=move |ev| set_workbench_variant_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                                <option value="outline">"Outline"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_padded.get()
                                on:change=move |ev| set_workbench_padded.set(event_target_checked(&ev))
                            />
                            " Padded"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| {
                                    set_workbench_custom_class.set(event_target_checked(&ev))
                                }
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="card-workbench-preview">
                    {move || {
                        let variant = workbench_variant.get();
                        let padded = workbench_padded.get();
                        if workbench_custom_class.get() {
                            view! {
                                <Card
                                    variant=variant
                                    padded=padded
                                    class_name="docs-card-custom".to_string()
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench card"</div>
                                        <div class="ui-muted">
                                            "Compare variant/padding/class-source markers in one canvas."
                                        </div>
                                    </div>
                                </Card>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Card variant=variant padded=padded>
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench card"</div>
                                        <div class="ui-muted">
                                            "Compare variant/padding/class-source markers in one canvas."
                                        </div>
                                    </div>
                                </Card>
                            }
                            .into_any()
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Padding / Class Comparison)"
                code_signal=variants_code
            >
                <div class="docs-row">
                    <Card variant=CardVariant::Default>
                        <div>"Default"</div>
                    </Card>
                    <Card variant=CardVariant::Muted padded=false>
                        <div>"Muted + flush"</div>
                    </Card>
                    <Card variant=CardVariant::Outline class_name="docs-card-custom".to_string()>
                        <div>"Outline + custom class"</div>
                    </Card>
                </div>
            </Playground>

            <Playground title="Padding States" code_signal=padding_code>
                <div class="docs-row">
                    <Card padded=true>
                        <div class="docs-stack">
                            <div>"Padded"</div>
                            <div class="ui-muted">"Default spacing"</div>
                        </div>
                    </Card>
                    <Card padded=false>
                        <div class="docs-stack">
                            <div class="docs-row docs-row--tight">
                                <span>"Flush"</span>
                                <span class="ui-muted">"No internal padding"</span>
                            </div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground title="Custom Class" code_signal=custom_class_code>
                <div class="docs-row">
                    <Card class_name="docs-card-custom".to_string()>
                        <div class="docs-stack">
                            <div>"Custom class marker"</div>
                            <div class="ui-muted">"Verifies `data-custom-class` + class merge."</div>
                        </div>
                    </Card>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
