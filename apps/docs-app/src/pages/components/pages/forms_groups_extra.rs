use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    CheckboxField, CheckboxFieldIndicatorPlacement, CheckboxFieldTone, FormField,
    FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldTone, Legend, LegendTone,
    SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn checkbox_field() -> AnyView {
    let (newsletter, set_newsletter) = signal(true);
    let (terms, set_terms) = signal(false);
    let (read_only, set_read_only) = signal(true);
    let (interactive_checked, set_interactive_checked) = signal(true);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_invalid, set_interactive_invalid) = signal(false);
    let (interactive_show_description, set_interactive_show_description) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let tone_options = vec!["Default".to_string(), "Quiet".to_string()];
    let placement_options = vec!["Start".to_string(), "End".to_string()];
    let (interactive_tone_index, set_interactive_tone_index) = signal(Some(0_usize));
    let (interactive_placement_index, set_interactive_placement_index) = signal(Some(0_usize));

    let interactive_tone =
        Signal::derive(move || match interactive_tone_index.get().unwrap_or(0) {
            1 => CheckboxFieldTone::Quiet,
            _ => CheckboxFieldTone::Default,
        });
    let interactive_placement =
        Signal::derive(
            move || match interactive_placement_index.get().unwrap_or(0) {
                1 => CheckboxFieldIndicatorPlacement::End,
                _ => CheckboxFieldIndicatorPlacement::Start,
            },
        );

    let interactive_code = Signal::derive(move || {
        let checked = interactive_checked.get();
        let disabled = interactive_disabled.get();
        let invalid = interactive_invalid.get();
        let show_description = interactive_show_description.get();
        let custom_class = interactive_custom_class.get();
        let tone = interactive_tone.get();
        let placement = interactive_placement.get();

        let mut lines = vec![
            "let (checked, set_checked) = signal(true);".to_string(),
            "".to_string(),
            "<CheckboxField".to_string(),
            "  checked=checked".to_string(),
            "  set_checked=set_checked".to_string(),
            "  id_base=\"docs-checkbox-field-interactive\".to_string()".to_string(),
            "  label=\"Weekly release digest\".to_string()".to_string(),
        ];

        if show_description {
            lines.push(
                "  description=\"Receive product updates every Friday.\".to_string()".to_string(),
            );
        }
        if tone != CheckboxFieldTone::Default {
            lines.push(format!("  tone=CheckboxFieldTone::{tone:?}"));
        }
        if placement != CheckboxFieldIndicatorPlacement::Start {
            lines.push(format!(
                "  indicator_placement=CheckboxFieldIndicatorPlacement::{placement:?}"
            ));
        }
        if disabled {
            lines.push("  disabled=true".to_string());
        }
        if invalid {
            lines.push("  invalid=true".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-checkbox-field-custom\".to_string()".to_string());
        }
        if !checked {
            lines.push("  // current state: unchecked".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/checkbox_field/styles.rs */\n{}",
            ui_components::checkbox_field::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxFieldActualConfig {{\n  checked: {},\n  disabled: {},\n  invalid: {},\n  tone: {:?},\n  indicator_placement: {:?},\n  description: {},\n  class_name: {},\n}}",
            interactive_checked.get(),
            interactive_disabled.get(),
            interactive_invalid.get(),
            interactive_tone.get(),
            interactive_placement.get(),
            if interactive_show_description.get() {
                "present"
            } else {
                "absent"
            },
            if interactive_custom_class.get() {
                "\"docs-checkbox-field-custom\""
            } else {
                "None"
            }
        )
    });

    let code = Signal::derive(move || {
        r#"let (newsletter, set_newsletter) = signal(true);

<CheckboxField
  checked=newsletter
  set_checked=set_newsletter
  id_base="newsletter-checkbox-field".to_string()
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (terms, set_terms) = signal(false);

<CheckboxField
  checked=terms
  set_checked=set_terms
  id_base="terms-checkbox-field".to_string()
  label="Accept terms of service".to_string()
  description="Required before continuing checkout.".to_string()
  indicator_placement=CheckboxFieldIndicatorPlacement::End
  tone=CheckboxFieldTone::Quiet
  invalid=true
  class_name="docs-checkbox-field-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="CheckboxField"
            slug="checkbox-field"
            group="Forms"
            description="baseline-style checkbox field primitive with centralized tone/indicator/state derivation and stable slot/data-state markers."
        >
            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit props and inspect actual config/state contracts."
                code_signal=interactive_code
                test_css_source=interactive_test_css
                test_source_path="crates/ui-components/src/checkbox_field/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-field-tone".to_string()
                            options=tone_options.clone()
                            selected_index=interactive_tone_index
                            set_selected_index=set_interactive_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="CheckboxField tone".to_string()
                        />

                        <div class="docs-search__label">"Indicator placement"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-field-placement".to_string()
                            options=placement_options.clone()
                            selected_index=interactive_placement_index
                            set_selected_index=set_interactive_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="CheckboxField indicator placement".to_string()
                        />

                        <Switch checked=interactive_checked set_checked=set_interactive_checked>
                            "Checked"
                        </Switch>
                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=interactive_invalid set_checked=set_interactive_invalid>
                            "Invalid"
                        </Switch>
                        <Switch
                            checked=interactive_show_description
                            set_checked=set_interactive_show_description
                        >
                            "Show description"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let disabled = interactive_disabled.get();
                    let invalid = interactive_invalid.get();
                    let tone = interactive_tone.get();
                    let placement = interactive_placement.get();
                    let show_description = interactive_show_description.get();
                    let custom_class = interactive_custom_class.get();

                    if show_description {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <CheckboxField
                                    checked=interactive_checked
                                    set_checked=set_interactive_checked
                                    id_base="docs-checkbox-field-interactive".to_string()
                                    label="Weekly release digest".to_string()
                                    description="Receive product updates every Friday.".to_string()
                                    tone=tone
                                    indicator_placement=placement
                                    disabled=disabled
                                    invalid=invalid
                                    class_name=if custom_class {
                                        "docs-checkbox-field-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                                <span class="ui-muted">
                                    "checked: " {move || interactive_checked.get().to_string()}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <CheckboxField
                                    checked=interactive_checked
                                    set_checked=set_interactive_checked
                                    id_base="docs-checkbox-field-interactive".to_string()
                                    label="Weekly release digest".to_string()
                                    tone=tone
                                    indicator_placement=placement
                                    disabled=disabled
                                    invalid=invalid
                                    class_name=if custom_class {
                                        "docs-checkbox-field-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                                <span class="ui-muted">
                                    "checked: " {move || interactive_checked.get().to_string()}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground title="Controlled + Description" code_signal=code>
                <div class="docs-stack">
                    <CheckboxField
                        checked=newsletter
                        set_checked=set_newsletter
                        id_base="docs-checkbox-field-newsletter".to_string()
                        label="Subscribe to product updates".to_string()
                        description="Receive release notes and occasional best-practice tips.".to_string()
                    />
                    <span class="ui-muted">
                        "newsletter: " {move || newsletter.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Indicator End + Quiet + Invalid/Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <CheckboxField
                        checked=terms
                        set_checked=set_terms
                        id_base="docs-checkbox-field-terms".to_string()
                        label="Accept terms of service".to_string()
                        description="Required before continuing checkout.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        tone=CheckboxFieldTone::Quiet
                        invalid=true
                        class_name="docs-checkbox-field-custom".to_string()
                    />

                    <CheckboxField
                        checked=read_only
                        set_checked=set_read_only
                        id_base="docs-checkbox-field-read-only".to_string()
                        label="Enable maintenance window alerts".to_string()
                        description="Read-only preference inherited from organization policy.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        disabled=true
                        aria_label="Maintenance alerts (read only)".to_string()
                    />

                    <span class="ui-muted">
                        "terms: " {move || terms.get().to_string()}
                        " · read-only: " {move || read_only.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn form_field() -> AnyView {
    let (marketing, set_marketing) = signal(true);
    let (tos, set_tos) = signal(false);
    let (read_only, set_read_only) = signal(true);

    let code = Signal::derive(move || {
        r#"let (marketing, set_marketing) = signal(true);

<FormField
  selected=marketing
  set_selected=set_marketing
  id_base="marketing-form-field".to_string()
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
  indicator_placement=FormFieldIndicatorPlacement::Start
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (tos, set_tos) = signal(false);

<FormField
  selected=tos
  set_selected=set_tos
  id_base="terms-form-field".to_string()
  label="Accept terms of service".to_string()
  description="Required before continuing checkout.".to_string()
  indicator_variant=FormFieldIndicatorVariant::Checkbox
  indicator_placement=FormFieldIndicatorPlacement::End
  tone=FormFieldTone::Quiet
  invalid=true
  error_message="Please accept terms to continue.".to_string()
  class_name="docs-form-field-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FormField"
            slug="form-field"
            group="Forms"
            description="baseline-style form field primitive that composes switch/checkbox indicators with centralized tone/placement/message state derivation and stable slot/data-state markers."
        >
            <Playground title="Switch Indicator + Description" code_signal=code>
                <div class="docs-stack">
                    <FormField
                        selected=marketing
                        set_selected=set_marketing
                        id_base="docs-form-field-marketing".to_string()
                        label="Subscribe to product updates".to_string()
                        description="Receive release notes and occasional best-practice tips.".to_string()
                        indicator_placement=FormFieldIndicatorPlacement::Start
                    />
                    <span class="ui-muted">
                        "marketing: " {move || marketing.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Checkbox Indicator + Quiet + Invalid/Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <FormField
                        selected=tos
                        set_selected=set_tos
                        id_base="docs-form-field-tos".to_string()
                        label="Accept terms of service".to_string()
                        description="Required before continuing checkout.".to_string()
                        indicator_variant=FormFieldIndicatorVariant::Checkbox
                        indicator_placement=FormFieldIndicatorPlacement::End
                        tone=FormFieldTone::Quiet
                        invalid=true
                        error_message="Please accept terms to continue.".to_string()
                        class_name="docs-form-field-custom".to_string()
                    />

                    <FormField
                        selected=read_only
                        set_selected=set_read_only
                        id_base="docs-form-field-read-only".to_string()
                        label="Maintenance window alerts".to_string()
                        description="Read-only preference inherited from organization policy.".to_string()
                        indicator_variant=FormFieldIndicatorVariant::Checkbox
                        indicator_placement=FormFieldIndicatorPlacement::End
                        disabled=true
                        aria_label="Maintenance alerts (read only)".to_string()
                    />

                    <span class="ui-muted">
                        "tos: " {move || tos.get().to_string()}
                        " · read-only: " {move || read_only.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn legend() -> AnyView {
    let required_code = Signal::derive(move || {
        r#"<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Notification settings\".to_string()
    is_required=true
  />
</fieldset>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Billing preferences\".to_string()
    tone=LegendTone::Muted
    is_required=true
    required_indicator=\"(required)\".to_string()
    class_name=\"docs-legend-custom\".to_string()
  />
  <Legend
    text=\"Read-only group\".to_string()
    tone=LegendTone::Strong
    is_disabled=true
  />
</fieldset>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Legend"
            slug="legend"
            group="Forms"
            description="baseline-compatible fieldset legend primitive with centralized tone/required/disabled contracts and stable slot/data-state markers."
        >
            <Playground title="Required Legend" code_signal=required_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() is_required=true />
                    <div class="ui-muted">
                        "Legend stays semantic inside fieldset and exposes required marker contracts."
                    </div>
                </fieldset>
            </Playground>

            <Playground title="Tone + Custom Indicator + Disabled" code_signal=states_code>
                <fieldset class="docs-stack">
                    <Legend
                        text="Billing preferences".to_string()
                        tone=LegendTone::Muted
                        is_required=true
                        required_indicator="(required)".to_string()
                        class_name="docs-legend-custom".to_string()
                    />

                    <Legend
                        text="Read-only group".to_string()
                        tone=LegendTone::Strong
                        is_disabled=true
                    />
                </fieldset>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
