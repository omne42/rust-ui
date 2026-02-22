use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    A11yDirection, CheckboxField, CheckboxFieldIndicatorPlacement, CheckboxFieldTone, FormField,
    FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldTone, Legend, LegendTone,
    SegmentedControl, SegmentedControlSize, Switch,
};

const CHECKBOX_FIELD_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::*;";
const LEGEND_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::*;";

pub(super) fn checkbox_field() -> AnyView {
    let (newsletter, set_newsletter) = signal(true);
    let (terms, set_terms) = signal(false);
    let (read_only, set_read_only) = signal(true);
    let (comparison_checked, set_comparison_checked) = signal(true);
    let (interactive_checked, set_interactive_checked) = signal(true);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_invalid, set_interactive_invalid) = signal(false);
    let (interactive_show_description, set_interactive_show_description) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_rtl_dir, set_interactive_rtl_dir) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
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
    let interactive_dir = Signal::derive(move || {
        if interactive_rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let interactive_motion = Signal::derive(move || {
        if interactive_custom_motion.get() {
            ui::checkbox_field::CheckboxFieldMotion {
                enabled: true,
                transition_ms: 260,
                indicator_scale_pct: 114,
            }
        } else {
            ui::checkbox_field::CheckboxFieldMotion::default()
        }
    });

    let interactive_code = Signal::derive(move || {
        let checked = interactive_checked.get();
        let disabled = interactive_disabled.get();
        let invalid = interactive_invalid.get();
        let show_description = interactive_show_description.get();
        let custom_class = interactive_custom_class.get();
        let tone = interactive_tone.get();
        let placement = interactive_placement.get();
        let dir = interactive_dir.get();
        let motion = interactive_motion.get();

        let mut lines = vec![
            "let (checked, set_checked) = signal(true);".to_string(),
            "".to_string(),
            "<CheckboxField".to_string(),
            "  is_checked=checked".to_string(),
            "  on_checked_change=set_checked".to_string(),
            "  id_base=\"docs-checkbox-field-interactive\".into()".to_string(),
            "  label=\"Weekly release digest\".into()".to_string(),
            "  aria_label=\"Weekly release digest preference\".into()".to_string(),
            "  lang=\"en-US\".into()".to_string(),
            format!("  dir=A11yDirection::{dir:?}"),
        ];

        if show_description {
            lines
                .push("  description=\"Receive product updates every Friday.\".into()".to_string());
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
            lines.push("  is_disabled=true".to_string());
        }
        if invalid {
            lines.push("  is_invalid=true".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-checkbox-field-custom\".into()".to_string());
        }
        if motion != ui::checkbox_field::CheckboxFieldMotion::default() {
            lines.push(format!(
                "  motion=ui::checkbox_field::CheckboxFieldMotion {{ enabled: {}, transition_ms: {}, indicator_scale_pct: {} }}",
                motion.enabled,
                motion.transition_ms,
                motion.indicator_scale_pct
            ));
        } else {
            lines.push("  motion=ui::checkbox_field::CheckboxFieldMotion::default()".to_string());
        }
        if !checked {
            lines.push("  // current state: unchecked".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/checkbox_field/styles.rs */\n{}",
            ui::checkbox_field::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        let dir = interactive_dir.get();
        let motion = interactive_motion.get();
        format!(
            "CheckboxFieldActualConfig {{\n  is_checked: Some({checked}),\n  checked: Some({checked}),\n  on_checked_change: Some(\"set_interactive_checked\"),\n  set_checked: Some(\"set_interactive_checked\"),\n  default_checked: Some(false),\n  is_disabled: Some({is_disabled}),\n  disabled: {is_disabled},\n  is_invalid: Some({is_invalid}),\n  invalid: {is_invalid},\n  id_base: Some(\"docs-checkbox-field-interactive\"),\n  label: Some(\"Weekly release digest\"),\n  description: {description},\n  aria_label: Some(\"Weekly release digest preference\"),\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::{dir:?}),\n  tone: {tone:?},\n  indicator_placement: {indicator_placement:?},\n  class_name: {class_name},\n  motion: CheckboxFieldMotion {{ enabled: {motion_enabled}, transition_ms: {motion_transition_ms}, indicator_scale_pct: {motion_scale_pct} }},\n}}",
            checked = interactive_checked.get(),
            is_disabled = interactive_disabled.get(),
            is_invalid = interactive_invalid.get(),
            description = if interactive_show_description.get() {
                "Some(\"Receive product updates every Friday.\")"
            } else {
                "None"
            },
            tone = interactive_tone.get(),
            indicator_placement = interactive_placement.get(),
            class_name = if interactive_custom_class.get() {
                "Some(\"docs-checkbox-field-custom\")"
            } else {
                "None"
            },
            dir = dir,
            motion_enabled = motion.enabled,
            motion_transition_ms = motion.transition_ms,
            motion_scale_pct = motion.indicator_scale_pct,
        )
    });

    let hello_code = Signal::derive(move || {
        r#"<CheckboxField label="Accept terms of service".to_string() />"#.to_string()
    });

    let code = Signal::derive(move || {
        r#"let (newsletter, set_newsletter) = signal(true);

<CheckboxField
  is_checked=newsletter
  on_checked_change=set_newsletter
  id_base="newsletter-checkbox-field".to_string()
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (terms, set_terms) = signal(false);

<CheckboxField
  is_checked=terms
  on_checked_change=set_terms
  id_base="terms-checkbox-field".to_string()
  label="Accept terms of service".to_string()
  description="Required before continuing checkout.".to_string()
  indicator_placement=CheckboxFieldIndicatorPlacement::End
  tone=CheckboxFieldTone::Quiet
  is_invalid=true
  class_name="docs-checkbox-field-custom".to_string()
/>"#
        .to_string()
    });

    let comparison_code = Signal::derive(move || {
        let checked = comparison_checked.get();
        format!(
            r#"let (checked, set_checked) = signal({checked});

<CheckboxField
  is_checked=checked
  on_checked_change=set_checked
  id_base="docs-checkbox-field-controlled".to_string()
  label="Product release digest".to_string()
  description="Parent signal stays as the single source of truth.".to_string()
/>

<CheckboxField
  default_checked=true
  id_base="docs-checkbox-field-uncontrolled".to_string()
  label="Security bulletin alerts".to_string()
  description="Uncontrolled path keeps primitive-owned state after default init.".to_string()
  indicator_placement=CheckboxFieldIndicatorPlacement::End
/>"#
        )
    });

    view! {
        <ComponentPage
            title="CheckboxField"
            slug="checkbox-field"
            group="Forms"
            description="baseline-style checkbox field primitive with centralized tone/indicator/state derivation and stable slot/data-state markers."
        >
            <Playground
                title="Hello World（默认路径）"
                code_signal=hello_code
                code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()
            >
                <CheckboxField label="Accept terms of service".to_string() />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit props and inspect actual config/state contracts."
                code_signal=interactive_code
                code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()
                test_css_source=interactive_test_css
                test_source_path="crates/ui/src/checkbox_field/styles.rs".to_string()
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
                        <Switch checked=interactive_rtl_dir set_checked=set_interactive_rtl_dir>
                            "RTL direction"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let disabled = interactive_disabled.get();
                    let invalid = interactive_invalid.get();
                    let tone = interactive_tone.get();
                    let placement = interactive_placement.get();
                    let dir = interactive_dir.get();
                    let motion = interactive_motion.get();
                    let show_description = interactive_show_description.get();
                    let custom_class = interactive_custom_class.get();

                    if show_description {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <CheckboxField
                                    is_checked=interactive_checked
                                    on_checked_change=set_interactive_checked
                                    id_base="docs-checkbox-field-interactive".to_string()
                                    label="Weekly release digest".to_string()
                                    description="Receive product updates every Friday.".to_string()
                                    aria_label="Weekly release digest preference".to_string()
                                    lang="en-US".to_string()
                                    dir=dir
                                    tone=tone
                                    indicator_placement=placement
                                    is_disabled=disabled
                                    is_invalid=invalid
                                    motion=motion
                                    class_name=if custom_class {
                                        "docs-checkbox-field-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                                <span class="ui-muted">
                                    "checked: " {move || interactive_checked.get()}
                                </span>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <div class="docs-stack docs-stack--tight">
                                <CheckboxField
                                    is_checked=interactive_checked
                                    on_checked_change=set_interactive_checked
                                    id_base="docs-checkbox-field-interactive".to_string()
                                    label="Weekly release digest".to_string()
                                    aria_label="Weekly release digest preference".to_string()
                                    lang="en-US".to_string()
                                    dir=dir
                                    tone=tone
                                    indicator_placement=placement
                                    is_disabled=disabled
                                    is_invalid=invalid
                                    motion=motion
                                    class_name=if custom_class {
                                        "docs-checkbox-field-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                />
                                <span class="ui-muted">
                                    "checked: " {move || interactive_checked.get()}
                                </span>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Playground>

            <Playground
                title="Controlled + Description"
                code_signal=code
                code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack">
                    <CheckboxField
                        is_checked=newsletter
                        on_checked_change=set_newsletter
                        id_base="docs-checkbox-field-newsletter".to_string()
                        label="Subscribe to product updates".to_string()
                        description="Receive release notes and occasional best-practice tips.".to_string()
                    />
                    <span class="ui-muted">
                        "newsletter: " {move || newsletter.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Indicator End + Quiet + Invalid/Disabled"
                code_signal=states_code
                code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack">
                    <CheckboxField
                        is_checked=terms
                        on_checked_change=set_terms
                        id_base="docs-checkbox-field-terms".to_string()
                        label="Accept terms of service".to_string()
                        description="Required before continuing checkout.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        tone=CheckboxFieldTone::Quiet
                        is_invalid=true
                        class_name="docs-checkbox-field-custom".to_string()
                    />

                    <CheckboxField
                        is_checked=read_only
                        on_checked_change=set_read_only
                        id_base="docs-checkbox-field-read-only".to_string()
                        label="Enable maintenance window alerts".to_string()
                        description="Read-only preference inherited from organization policy.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        is_disabled=true
                        aria_label="Maintenance alerts (read only)".to_string()
                    />

                    <span class="ui-muted">
                        "terms: " {move || terms.get()}
                        " · read-only: " {move || read_only.get()}
                    </span>
                    <div class="ui-muted" data-slot="checkbox-field-state-matrix-note">
                        "State matrix: controlled checked/unchecked, invalid, disabled, indicator placement and tone."
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Default (Comparison)"
                code_signal=comparison_code
                code_imports=CHECKBOX_FIELD_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack">
                    <CheckboxField
                        is_checked=comparison_checked
                        on_checked_change=set_comparison_checked
                        id_base="docs-checkbox-field-controlled".to_string()
                        label="Product release digest".to_string()
                        description="Parent signal stays as the single source of truth.".to_string()
                    />

                    <CheckboxField
                        default_checked=true
                        id_base="docs-checkbox-field-uncontrolled".to_string()
                        label="Security bulletin alerts".to_string()
                        description="Uncontrolled path keeps primitive-owned state after default init.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                    />

                    <Switch checked=comparison_checked set_checked=set_comparison_checked>
                        "Controlled checked (parent signal)"
                    </Switch>

                    <div class="ui-muted" data-slot="checkbox-field-controlled-uncontrolled-note">
                        "Controlled path uses is_checked + on_checked_change; default path starts from default_checked."
                    </div>
                </div>
            </Playground>

            <p class="ui-muted" data-slot="checkbox-field-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="checkbox-field-streaming-modes">
                "Snapshot mode renders verified full output for checkbox-field semantics."
            </p>
            <p class="ui-muted" data-slot="checkbox-field-copy-ready">
                "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*."
            </p>
            <p class="ui-muted" data-slot="checkbox-field-source-paths">
                "Source paths: components/checkbox-field/src/mod.rs, components/checkbox-field/src/logic.rs, components/checkbox-field/src/view.rs, components/checkbox-field/src/styles.rs."
            </p>
            <p class="ui-muted" data-slot="checkbox-field-source-prerequisites">
                "Feature prerequisites: component-checkbox_field (inject-css optional for runtime style injection)."
            </p>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn form_field() -> AnyView {
    let (marketing, set_marketing) = signal(true);
    let (tos, set_tos) = signal(false);
    let (comparison_selected, set_comparison_selected) = signal(true);
    let (interactive_selected, set_interactive_selected) = signal(true);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_invalid, set_interactive_invalid) = signal(false);
    let (interactive_show_description, set_interactive_show_description) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_rtl_dir, set_interactive_rtl_dir) = signal(false);
    let tone_options = vec!["Default".to_string(), "Quiet".to_string()];
    let indicator_variant_options = vec!["Switch".to_string(), "Checkbox".to_string()];
    let placement_options = vec!["Start".to_string(), "End".to_string()];
    let (interactive_tone_index, set_interactive_tone_index) = signal(Some(0_usize));
    let (interactive_variant_index, set_interactive_variant_index) = signal(Some(0_usize));
    let (interactive_placement_index, set_interactive_placement_index) = signal(Some(1_usize));
    let on_marketing_selected_change = Callback::new(move |next| set_marketing.set(next));
    let on_tos_selected_change = Callback::new(move |next| set_tos.set(next));
    let on_comparison_selected_change =
        Callback::new(move |next| set_comparison_selected.set(next));
    let on_interactive_selected_change =
        Callback::new(move |next| set_interactive_selected.set(next));

    let interactive_tone =
        Signal::derive(move || match interactive_tone_index.get().unwrap_or(0) {
            1 => FormFieldTone::Quiet,
            _ => FormFieldTone::Default,
        });
    let interactive_indicator_variant =
        Signal::derive(move || match interactive_variant_index.get().unwrap_or(0) {
            1 => FormFieldIndicatorVariant::Checkbox,
            _ => FormFieldIndicatorVariant::Switch,
        });
    let interactive_indicator_placement = Signal::derive(move || match interactive_placement_index
        .get()
        .unwrap_or(1)
    {
        0 => FormFieldIndicatorPlacement::Start,
        _ => FormFieldIndicatorPlacement::End,
    });
    let interactive_dir = Signal::derive(move || {
        if interactive_rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<FormField label="Accept terms of service".to_string() />"#.to_string()
    });

    let code = Signal::derive(move || {
        r#"let (marketing, set_marketing) = signal(true);
let on_marketing_selected_change = Callback::new(move |next| set_marketing.set(next));

<FormField
  is_selected=marketing.into()
  on_selected_change=on_marketing_selected_change
  id_base="marketing-form-field".to_string()
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
  indicator_placement=FormFieldIndicatorPlacement::Start
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (tos, set_tos) = signal(false);
let on_tos_selected_change = Callback::new(move |next| set_tos.set(next));

<FormField
  is_selected=tos.into()
  on_selected_change=on_tos_selected_change
  id_base="terms-form-field".to_string()
  label="Accept terms of service".to_string()
  description="Required before continuing checkout.".to_string()
  indicator_variant=FormFieldIndicatorVariant::Checkbox
  indicator_placement=FormFieldIndicatorPlacement::End
  tone=FormFieldTone::Quiet
  is_invalid=true
  error_message="Please accept terms to continue.".to_string()
  class_name="docs-form-field-custom".to_string()
/>"#
        .to_string()
    });

    let comparison_code = Signal::derive(move || {
        let selected = comparison_selected.get();
        format!(
            r#"use leptos::prelude::*;
use ui::*;

let (selected, set_selected) = signal({selected});
let on_selected_change = Callback::new(move |next| set_selected.set(next));

<FormField
  is_selected=selected.into()
  on_selected_change=on_selected_change
  id_base="controlled-form-field".to_string()
  label="Marketing notifications".to_string()
  description="Parent signal stays as the single source of truth.".to_string()
/>

<FormField
  default_selected=true
  id_base="default-form-field".to_string()
  label="Audit log alerts".to_string()
  description="Uncontrolled path keeps internal primitive state after init.".to_string()
/>"#
        )
    });

    let interactive_code = Signal::derive(move || {
        let selected = interactive_selected.get();
        let disabled = interactive_disabled.get();
        let invalid = interactive_invalid.get();
        let show_description = interactive_show_description.get();
        let custom_class = interactive_custom_class.get();
        let tone = interactive_tone.get();
        let indicator_variant = interactive_indicator_variant.get();
        let indicator_placement = interactive_indicator_placement.get();
        let dir = interactive_dir.get();

        let mut lines = vec![
            format!("let (selected, set_selected) = signal({selected});"),
            "let on_selected_change = Callback::new(move |next| set_selected.set(next));"
                .to_string(),
            "".to_string(),
            "<FormField".to_string(),
            "  is_selected=selected.into()".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            "  id_base=\"docs-form-field-workbench\".to_string()".to_string(),
            "  label=\"Notification consent\".to_string()".to_string(),
            "  aria_label=\"Notification consent selector\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir=A11yDirection::{dir:?}"),
        ];

        if show_description {
            lines.push(
                "  description=\"Allow weekly release updates and product notices.\".to_string()"
                    .to_string(),
            );
        }
        if tone != FormFieldTone::Default {
            lines.push(format!("  tone=FormFieldTone::{tone:?}"));
        }
        if indicator_variant != FormFieldIndicatorVariant::Switch {
            lines.push(format!(
                "  indicator_variant=FormFieldIndicatorVariant::{indicator_variant:?}"
            ));
        }
        if indicator_placement != FormFieldIndicatorPlacement::End {
            lines.push(format!(
                "  indicator_placement=FormFieldIndicatorPlacement::{indicator_placement:?}"
            ));
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if invalid {
            lines.push("  is_invalid=true".to_string());
            lines.push("  error_message=\"Selection is required\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-form-field-custom\".to_string()".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });

    let interactive_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/field_form/form_field/styles.rs */\n{}",
            ui::field_form::form_field::styles::CSS
        )
    });

    let interactive_actual_config = Signal::derive(move || {
        let dir = interactive_dir.get();
        format!(
            "FormFieldActualConfig {{\n  is_selected: Some({is_selected}),\n  default_selected: Some(false),\n  on_selected_change: Some(\"on_interactive_selected_change\"),\n  is_disabled: {is_disabled},\n  is_invalid: {is_invalid},\n  tone: {tone:?},\n  indicator_variant: {indicator_variant:?},\n  indicator_placement: {indicator_placement:?},\n  id_base: Some(\"docs-form-field-workbench\"),\n  label: Some(\"Notification consent\"),\n  description: {description},\n  error_message: {error_message},\n  aria_label: Some(\"Notification consent selector\"),\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::{dir:?}),\n  class_name: {class_name},\n}}",
            is_selected = interactive_selected.get(),
            is_disabled = interactive_disabled.get(),
            is_invalid = interactive_invalid.get(),
            tone = interactive_tone.get(),
            indicator_variant = interactive_indicator_variant.get(),
            indicator_placement = interactive_indicator_placement.get(),
            description = if interactive_show_description.get() {
                "Some(\"Allow weekly release updates and product notices.\")"
            } else {
                "None"
            },
            error_message = if interactive_invalid.get() {
                "Some(\"Selection is required\")"
            } else {
                "None"
            },
            dir = dir,
            class_name = if interactive_custom_class.get() {
                "Some(\"docs-form-field-custom\")"
            } else {
                "None"
            }
        )
    });

    view! {
        <ComponentPage
            title="FormField"
            slug="form-field"
            group="Forms"
            description="baseline-style form field primitive that composes switch/checkbox indicators with centralized tone/placement/message state derivation and stable slot/data-state markers."
        >
            <Playground title="Hello World（默认路径）" code_signal=hello_code>
                <FormField label="Accept terms of service".to_string() />
            </Playground>

            <Playground
                title="FormField Workbench (Display + Config + Code + CSS Test)"
                description="Interactive playground: adjust props/state and inspect live preview + generated config."
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui/src/field_form/form_field/styles.rs".to_string()
                test_config_signal=interactive_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="form-field-workbench-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-form-field-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=interactive_tone_index
                            set_selected_index=set_interactive_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="FormField tone".to_string()
                        />

                        <div class="docs-search__label">"Indicator variant"</div>
                        <SegmentedControl
                            id_base="docs-form-field-workbench-indicator-variant".to_string()
                            options=indicator_variant_options.clone()
                            selected_index=interactive_variant_index
                            set_selected_index=set_interactive_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="FormField indicator variant".to_string()
                        />

                        <div class="docs-search__label">"Indicator placement"</div>
                        <SegmentedControl
                            id_base="docs-form-field-workbench-indicator-placement".to_string()
                            options=placement_options.clone()
                            selected_index=interactive_placement_index
                            set_selected_index=set_interactive_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="FormField indicator placement".to_string()
                        />

                        <Switch checked=interactive_selected set_checked=set_interactive_selected>
                            "Selected"
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
                        <Switch checked=interactive_rtl_dir set_checked=set_interactive_rtl_dir>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let disabled = interactive_disabled.get();
                    let invalid = interactive_invalid.get();
                    let tone = interactive_tone.get();
                    let indicator_variant = interactive_indicator_variant.get();
                    let indicator_placement = interactive_indicator_placement.get();
                    let show_description = interactive_show_description.get();
                    let custom_class = interactive_custom_class.get();
                    let dir = interactive_dir.get();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="form-field-workbench-compare">
                            <FormField
                                is_selected=Signal::derive(move || interactive_selected.get())
                                on_selected_change=on_interactive_selected_change
                                id_base="docs-form-field-workbench".to_string()
                                label="Notification consent".to_string()
                                aria_label="Notification consent selector".to_string()
                                description=if show_description {
                                    "Allow weekly release updates and product notices.".to_string()
                                } else {
                                    String::new()
                                }
                                indicator_variant=indicator_variant
                                indicator_placement=indicator_placement
                                tone=tone
                                is_disabled=disabled
                                is_invalid=invalid
                                lang="en-US".to_string()
                                dir=dir
                                error_message=if invalid {
                                    "Selection is required".to_string()
                                } else {
                                    String::new()
                                }
                                class_name=if custom_class {
                                    "docs-form-field-custom".to_string()
                                } else {
                                    String::new()
                                }
                            />
                            <span class="ui-muted">
                                "selected: " {move || interactive_selected.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Switch Indicator + Description" code_signal=code>
                <div class="docs-stack">
                    <FormField
                        is_selected=Signal::derive(move || marketing.get())
                        on_selected_change=on_marketing_selected_change
                        id_base="docs-form-field-marketing".to_string()
                        label="Subscribe to product updates".to_string()
                        description="Receive release notes and occasional best-practice tips.".to_string()
                        indicator_placement=FormFieldIndicatorPlacement::Start
                    />
                    <span class="ui-muted">
                        "marketing: " {move || marketing.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Checkbox Indicator + Quiet + Invalid/Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <FormField
                        is_selected=Signal::derive(move || tos.get())
                        on_selected_change=on_tos_selected_change
                        id_base="docs-form-field-tos".to_string()
                        label="Accept terms of service".to_string()
                        description="Required before continuing checkout.".to_string()
                        indicator_variant=FormFieldIndicatorVariant::Checkbox
                        indicator_placement=FormFieldIndicatorPlacement::End
                        tone=FormFieldTone::Quiet
                        is_invalid=true
                        error_message="Please accept terms to continue.".to_string()
                        class_name="docs-form-field-custom".to_string()
                    />

                    <FormField
                        default_selected=true
                        id_base="docs-form-field-read-only".to_string()
                        label="Maintenance window alerts".to_string()
                        description="Read-only preference inherited from organization policy.".to_string()
                        indicator_variant=FormFieldIndicatorVariant::Checkbox
                        indicator_placement=FormFieldIndicatorPlacement::End
                        is_disabled=true
                        aria_label="Maintenance alerts (read only)".to_string()
                    />

                    <span class="ui-muted">
                        "tos: " {move || tos.get()}
                        " · read-only: true"
                    </span>
                    <div class="ui-muted" data-slot="form-field-state-matrix-note">
                        "State matrix: controlled selected/unselected, invalid, disabled, indicator variant and tone."
                    </div>
                </div>
            </Playground>

            <Playground title="Controlled vs Default (Comparison)" code_signal=comparison_code>
                <div class="docs-stack">
                    <FormField
                        is_selected=Signal::derive(move || comparison_selected.get())
                        on_selected_change=on_comparison_selected_change
                        id_base="docs-form-field-controlled".to_string()
                        label="Marketing notifications".to_string()
                        description="Parent signal stays as the single source of truth.".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />

                    <FormField
                        default_selected=true
                        id_base="docs-form-field-uncontrolled".to_string()
                        label="Audit log alerts".to_string()
                        description="Uncontrolled path keeps primitive-owned state after default init.".to_string()
                        indicator_variant=FormFieldIndicatorVariant::Checkbox
                        indicator_placement=FormFieldIndicatorPlacement::End
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />

                    <Switch checked=comparison_selected set_checked=set_comparison_selected>
                        "Controlled selected (parent signal)"
                    </Switch>

                    <div class="ui-muted" data-slot="form-field-controlled-uncontrolled-note">
                        "Controlled path uses is_selected + on_selected_change; default path starts from default_selected."
                    </div>
                </div>
            </Playground>

            <p class="ui-muted" data-slot="form-field-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="form-field-streaming-modes">
                "Snapshot mode renders verified full output for form-field semantics."
            </p>
            <p class="ui-muted" data-slot="form-field-copy-ready">
                "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*."
            </p>
            <p class="ui-muted" data-slot="form-field-source-paths">
                "Source paths: components/form-field/src/mod.rs, components/form-field/src/logic.rs, components/form-field/src/view.rs, components/form-field/src/styles.rs."
            </p>
            <p class="ui-muted" data-slot="form-field-source-prerequisites">
                "Feature prerequisites: component-form_field (inject-css optional for runtime style injection)."
            </p>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn legend() -> AnyView {
    let (controlled_required, set_controlled_required) = signal(true);
    let text_options = vec![
        "Notification settings".to_string(),
        "Billing preferences".to_string(),
    ];
    let tone_options = vec![
        "Default".to_string(),
        "Muted".to_string(),
        "Strong".to_string(),
    ];
    let (workbench_text_index, set_workbench_text_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_indicator, set_workbench_custom_indicator) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_text = Signal::derive(move || {
        if workbench_text_index.get().unwrap_or(0) == 1 {
            "Billing preferences".to_string()
        } else {
            "Notification settings".to_string()
        }
    });
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => LegendTone::Muted,
        2 => LegendTone::Strong,
        _ => LegendTone::Default,
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::legend::LegendMotion {
                duration_ms: 320.0,
                ..ui::legend::LegendMotion::default()
            }
        } else {
            ui::legend::LegendMotion::default()
        }
    });

    let hello_code = Signal::derive(move || {
        r#"use ui::Legend;

<fieldset class=\"docs-stack\"> 
  <Legend text=\"Notification settings\".into() />
</fieldset>"#
            .to_string()
    });

    let required_code = Signal::derive(move || {
        r#"use ui::Legend;

<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Notification settings\".into()
    is_required=true
  />
</fieldset>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"use ui::{Legend, LegendTone};

<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Billing preferences\".into()
    tone=LegendTone::Muted
    is_required=true
    required_indicator=\"(required)\".into()
    class_name=\"docs-legend-custom\".into()
  />
  <Legend
    text=\"Read-only group\".into()
    tone=LegendTone::Strong
    is_disabled=true
  />
</fieldset>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        let is_required = controlled_required.get();
        format!(
            "use ui::{{Legend, LegendTone, Switch}};\n\nlet (is_required, set_is_required) = signal({is_required});\n\n<fieldset class=\"docs-stack\">\n  <Legend text=\"Notification settings\".into() />\n  <Legend\n    text=\"Notification settings\".into()\n    tone=LegendTone::Muted\n    is_required=is_required\n  />\n  <Switch checked=is_required set_checked=set_is_required>\n    \"Controlled required\"\n  </Switch>\n</fieldset>"
        )
    });
    let workbench_code = Signal::derive(move || {
        let text = workbench_text.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let dir = workbench_dir.get();
        let custom_indicator = workbench_custom_indicator.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();
        let lang = if workbench_rtl_dir.get() {
            "ar"
        } else {
            "en-US"
        };
        format!(
            "use ui::{{A11yDirection, Legend, LegendTone}};\n\n<fieldset class=\"docs-stack\">\n  <Legend\n    text={text:?}.to_string()\n    tone=LegendTone::{tone:?}\n    is_required={required}\n    is_disabled={disabled}\n    motion=ui::legend::LegendMotion {{ duration_ms: {duration_ms}, ..ui::legend::LegendMotion::default() }}\n    required_indicator={required_indicator}\n    class_name={class_name}\n    lang={lang:?}.to_string()\n    dir=A11yDirection::{dir:?}\n  />\n</fieldset>",
            duration_ms = motion.duration_ms,
            required_indicator = if custom_indicator {
                "\"(required)\".to_string()"
            } else {
                "String::new()"
            },
            class_name = if custom_class {
                "\"docs-legend-custom\".to_string()"
            } else {
                "String::new()"
            },
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let motion = workbench_motion.get();
        format!(
            "LegendActualConfig {{\n  text: Some({text:?}),\n  tone: {tone:?},\n  is_required: Some({is_required}),\n  is_disabled: Some({is_disabled}),\n  motion: LegendMotion {{ duration_ms: {duration_ms}, spring: \"spring_soft\" }},\n  required_indicator: {required_indicator},\n  class_name: {class_name},\n  lang: Some({lang:?}),\n  dir: Some(A11yDirection::{dir:?}),\n}}",
            text = workbench_text.get(),
            tone = workbench_tone.get(),
            is_required = workbench_required.get(),
            is_disabled = workbench_disabled.get(),
            duration_ms = motion.duration_ms,
            required_indicator = if workbench_custom_indicator.get() {
                "Some(\"(required)\")"
            } else {
                "None"
            },
            class_name = if workbench_custom_class.get() {
                "Some(\"docs-legend-custom\")"
            } else {
                "None"
            },
            lang = if workbench_rtl_dir.get() {
                "ar"
            } else {
                "en-US"
            },
            dir = workbench_dir.get(),
        )
    });
    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/legend/styles.rs */\n{}",
            ui::legend::styles::CSS
        )
    });

    view! {
        <ComponentPage
            title="Legend"
            slug="legend"
            group="Forms"
            description="baseline-compatible fieldset legend primitive with centralized tone/required/disabled contracts and stable slot/data-state markers."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() />
                    <div class="ui-muted">"Default path: only pass text; no state wiring required."</div>
                </fieldset>
            </Playground>

            <Playground
                title="Legend Workbench (Display + Config + Code + CSS Test)"
                description="Adjust every Legend API field and inspect live actual config."
                code_signal=workbench_code
                code_imports=LEGEND_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css
                test_source_path="crates/ui/src/legend/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Text"</div>
                        <SegmentedControl
                            id_base="docs-legend-workbench-text".to_string()
                            options=text_options.clone()
                            selected_index=workbench_text_index
                            set_selected_index=set_workbench_text_index
                            size=SegmentedControlSize::Sm
                            aria_label="Legend text".to_string()
                        />
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-legend-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Legend tone".to_string()
                        />
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch
                            checked=workbench_custom_indicator
                            set_checked=set_workbench_custom_indicator
                        >
                            "Custom indicator"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "RTL direction"
                        </Switch>
                        <Switch
                            checked=workbench_custom_motion
                            set_checked=set_workbench_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                <fieldset class="docs-stack">
                    <Legend
                        text=workbench_text.get()
                        tone=workbench_tone.get()
                        is_required=workbench_required.get()
                        is_disabled=workbench_disabled.get()
                        motion=workbench_motion.get()
                        required_indicator=if workbench_custom_indicator.get() {
                            "(required)".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-legend-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl_dir.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=workbench_dir.get()
                    />
                    <div class="ui-muted">
                        "required: " {move || workbench_required.get()}
                        " · disabled: " {move || workbench_disabled.get()}
                    </div>
                </fieldset>
            </Playground>

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

            <Playground title="Controlled vs Default (Comparison)" code_signal=controlled_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() />
                    <Legend
                        text="Notification settings".to_string()
                        tone=LegendTone::Muted
                        is_required=controlled_required.get()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::legend::LegendMotion::default()
                    />
                    <Switch checked=controlled_required set_checked=set_controlled_required>
                        "Controlled required (parent signal)"
                    </Switch>
                    <div class="ui-muted">
                        "Uncontrolled path keeps default props; controlled path keeps parent signal as source of truth."
                    </div>
                </fieldset>
            </Playground>

            <p class="ui-muted" data-slot="legend-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="legend-streaming-modes">
                "Snapshot mode renders verified full output for legend semantics."
            </p>
            <p class="ui-muted" data-slot="legend-copy-ready">
                "Copy-ready snippets prepend imports automatically: use ui::{Legend, LegendTone, Switch}; source: apps/docs-app/src/pages/components/pages/forms_groups_extra.rs."
            </p>
            <p class="ui-muted" data-slot="legend-source-paths">
                "Source paths: components/legend/src/mod.rs, components/legend/src/logic.rs, components/legend/src/view.rs, components/legend/src/styles.rs, components/legend/src/motion.rs."
            </p>
            <p class="ui-muted" data-slot="legend-source-prerequisites">
                "Feature prerequisites: component-legend (inject-css optional for runtime style injection)."
            </p>
        </ComponentPage>
    }
    .into_any()
}
