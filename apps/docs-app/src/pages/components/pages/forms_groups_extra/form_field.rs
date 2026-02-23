use super::*;

pub(crate) fn form_field() -> AnyView {
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
