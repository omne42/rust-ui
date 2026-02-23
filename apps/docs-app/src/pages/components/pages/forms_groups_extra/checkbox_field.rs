use super::*;

pub(crate) fn checkbox_field() -> AnyView {
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
