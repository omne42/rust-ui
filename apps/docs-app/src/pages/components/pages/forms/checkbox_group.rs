use super::*;

pub(crate) fn checkbox_group() -> AnyView {
    let (hello_apple, set_hello_apple) = signal(false);
    let (hello_banana, set_hello_banana) = signal(true);

    let (apple, set_apple) = signal(false);
    let (banana, set_banana) = signal(true);
    let (mango, set_mango) = signal(false);

    let is_invalid = Signal::derive(move || !(apple.get() || banana.get() || mango.get()));
    let is_required = Signal::derive(|| true);
    let external_desc_id = "docs-checkbox-group-extra".to_string();
    let aria_describedby = Signal::derive(move || Some(external_desc_id.clone()));

    let (disabled_a, set_disabled_a) = signal(true);
    let (disabled_b, set_disabled_b) = signal(false);

    let (optional_email, set_optional_email) = signal(false);
    let (optional_sms, set_optional_sms) = signal(true);
    let optional_selected_count =
        Signal::derive(move || usize::from(optional_email.get()) + usize::from(optional_sms.get()));
    let (interactive_alpha, set_interactive_alpha) = signal(true);
    let (interactive_beta, set_interactive_beta) = signal(false);
    let (interactive_required, set_interactive_required) = signal(true);
    let (interactive_invalid, set_interactive_invalid) = signal(false);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_description, set_interactive_description) = signal(true);
    let (interactive_error, set_interactive_error) = signal(true);

    let hello_code = Signal::derive(move || {
        r#"<CheckboxGroup id="demo".to_string() label="Fruits".to_string()>
  <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
  <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
</CheckboxGroup>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let is_invalid = Signal::derive(move || !(apple.get() || banana.get()));
<CheckboxGroup
  id="demo".to_string()
  label="Fruits".to_string()
  description="Pick at least one".to_string()
  error="At least one required".to_string()
  is_required=Signal::derive(|| true)
  is_invalid=is_invalid
>
  <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
  <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
</CheckboxGroup>"#
            .to_string()
    });

    let interactive_code = Signal::derive(move || {
        let mut lines = vec![
            "let (alpha, set_alpha) = signal(true);".to_string(),
            "let (beta, set_beta) = signal(false);".to_string(),
            "".to_string(),
            "<CheckboxGroup".to_string(),
            "  id=\"docs-checkbox-group-interactive\".into()".to_string(),
            "  label=\"Release channels\".into()".to_string(),
        ];

        if interactive_description.get() {
            lines.push("  description=\"Choose at least one channel.\".into()".to_string());
        }
        if interactive_error.get() {
            lines.push("  error=\"At least one channel is required.\".into()".to_string());
        }
        lines.push(format!(
            "  is_required=Signal::derive(|| {})",
            interactive_required.get()
        ));
        lines.push(format!(
            "  is_invalid=Signal::derive(|| {})",
            interactive_invalid.get()
        ));
        if interactive_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }

        lines.push(">".to_string());
        lines.push(
            "  <Checkbox checked=alpha set_checked=set_alpha>\"Email\"</Checkbox>".to_string(),
        );
        lines.push("  <Checkbox checked=beta set_checked=set_beta>\"SMS\"</Checkbox>".to_string());
        lines.push("</CheckboxGroup>".to_string());

        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/checkbox/src/styles.rs */\n{}\n\n/* components/checkbox-group/src/styles.rs */\n{}",
            ui::checkbox::styles::CSS,
            ui::checkbox_group::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxGroupActualConfig {{\n  label: {:?},\n  is_required: {},\n  is_invalid: {},\n  is_disabled: {},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n  aria_describedby: {:?},\n  class_name: {:?},\n  description: {},\n  error: {},\n  alpha: {},\n  beta: {},\n}}",
            "Release channels",
            interactive_required.get(),
            interactive_invalid.get(),
            interactive_disabled.get(),
            ui::checkbox_group::CheckboxGroupMotion::default(),
            "en-US",
            A11yDirection::Ltr,
            aria_describedby.get(),
            "",
            if interactive_description.get() {
                "present"
            } else {
                "absent"
            },
            if interactive_error.get() {
                "present"
            } else {
                "absent"
            },
            interactive_alpha.get(),
            interactive_beta.get()
        )
    });

    let states_code = Signal::derive(move || {
        r#"<CheckboxGroup
  id="disabled".to_string()
  label="Notifications".to_string()
  is_disabled=true
>
  <Checkbox ...>"Email"</Checkbox>
  <Checkbox ...>"SMS"</Checkbox>
</CheckboxGroup>
<CheckboxGroup
  id="optional".to_string()
  label="Delivery channels".to_string()
  description="Optional selection".to_string()
>
  <Checkbox ...>"Email"</Checkbox>
  <Checkbox ...>"SMS"</Checkbox>
</CheckboxGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="CheckboxGroup"
            slug="checkbox-group"
            group="Forms"
            description="Fieldset wrapper with normalized labels, validation semantics, and baseline-style root state attrs."
        >
            <Playground
                title="Hello World（默认路径）"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <CheckboxGroup id="docs-checkbox-group-hello".to_string() label="Fruits".to_string()>
                    <Checkbox checked=hello_apple set_checked=set_hello_apple>"Apple"</Checkbox>
                    <Checkbox checked=hello_banana set_checked=set_hello_banana>"Banana"</Checkbox>
                </CheckboxGroup>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit group is_invalid/is_required state and inspect contracts."
                code_signal=interactive_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
                test_css_source=interactive_test_css
                test_source_path="components/checkbox-group/src/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch checked=interactive_required set_checked=set_interactive_required>
                            "Required"
                        </Switch>
                        <Switch checked=interactive_invalid set_checked=set_interactive_invalid>
                            "Invalid"
                        </Switch>
                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=interactive_description set_checked=set_interactive_description>
                            "Description"
                        </Switch>
                        <Switch checked=interactive_error set_checked=set_interactive_error>
                            "Error message"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let description = if interactive_description.get() {
                        "Choose at least one channel.".to_string()
                    } else {
                        String::new()
                    };
                    let error = if interactive_error.get() {
                        "At least one channel is required.".to_string()
                    } else {
                        String::new()
                    };
                    let is_required = Signal::derive(move || interactive_required.get());
                    let is_invalid = Signal::derive(move || interactive_invalid.get());
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <CheckboxGroup
                                id="docs-checkbox-group-interactive".to_string()
                                label="Release channels".to_string()
                                description=description
                                error=error
                                is_required=is_required
                                is_invalid=is_invalid
                                is_disabled=interactive_disabled.get()
                                motion=ui::checkbox_group::CheckboxGroupMotion::default()
                                lang="en-US".to_string()
                                dir=A11yDirection::Ltr
                                class_name="".to_string()
                            >
                                <Checkbox checked=interactive_alpha set_checked=set_interactive_alpha>
                                    "Email"
                                </Checkbox>
                                <Checkbox checked=interactive_beta set_checked=set_interactive_beta>
                                    "SMS"
                                </Checkbox>
                            </CheckboxGroup>
                            <span class="ui-muted">
                                "selected count: "
                                {move || {
                                    (usize::from(interactive_alpha.get()) + usize::from(interactive_beta.get()))
                                        .to_string()
                                }}
                            </span>
                        </div>
                    }
                        .into_any()
                }}
            </Playground>

            <Playground
                title="Validation + Required"
                code_signal=code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-stack">
                    <CheckboxGroup
                        id="docs-checkbox-group".to_string()
                        label="Fruits".to_string()
                        description="Pick at least one".to_string()
                        error="At least one required".to_string()
                        is_required=is_required
                        is_invalid=is_invalid
                        aria_describedby=aria_describedby
                    >
                        <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
                        <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
                        <Checkbox checked=mango set_checked=set_mango>"Mango"</Checkbox>
                    </CheckboxGroup>

                    <div id="docs-checkbox-group-extra" class="ui-muted">
                        "Tip: combine with an external description via aria-describedby."
                    </div>

                    <span class="ui-muted">
                        "selected: "
                        {move || {
                            let mut picked = Vec::new();
                            if apple.get() {
                                picked.push("Apple");
                            }
                            if banana.get() {
                                picked.push("Banana");
                            }
                            if mango.get() {
                                picked.push("Mango");
                            }
                            if picked.is_empty() {
                                "None".to_string()
                            } else {
                                picked.join(", ")
                            }
                        }}
                        " · invalid: "
                        {move || is_invalid.get()}
                    </span>

                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_apple.set(false);
                                set_banana.set(false);
                                set_mango.set(false);
                            })
                        >
                            "Clear selections"
                        </ui::Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Disabled + Optional)"
                code_signal=states_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-row">
                    <div class="docs-stack">
                        <CheckboxGroup
                            id="docs-checkbox-group-disabled".to_string()
                            label="Notifications".to_string()
                            description="Read-only preferences".to_string()
                            is_disabled=true
                        >
                            <Checkbox checked=disabled_a set_checked=set_disabled_a>"Email"</Checkbox>
                            <Checkbox checked=disabled_b set_checked=set_disabled_b>"SMS"</Checkbox>
                        </CheckboxGroup>
                        <span class="ui-muted">"disabled: true"</span>
                    </div>

                    <div class="docs-stack">
                        <CheckboxGroup
                            id="docs-checkbox-group-optional".to_string()
                            label="Delivery channels".to_string()
                            description="Optional selection (required = false)".to_string()
                        >
                            <Checkbox checked=optional_email set_checked=set_optional_email>
                                "Email"
                            </Checkbox>
                            <Checkbox checked=optional_sms set_checked=set_optional_sms>"SMS"</Checkbox>
                        </CheckboxGroup>
                        <span class="ui-muted">
                            "optional selected count: "
                            {move || optional_selected_count.get()}
                        </span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="checkbox-group-streaming-policy">
                <h3>"Streaming / Snapshot"</h3>
                <p>
                    "CheckboxGroup is "
                    <strong>"Streaming Optional; fallback=snapshot."</strong>
                </p>
                <p data-slot="checkbox-group-streaming-modes">
                    "Snapshot mode renders verified full output for group semantics. Streaming labels are exposed via stable markers (`data-ui-stream-support`, `data-ui-stream-fallback`, `data-ui-output-status`)."
                </p>
                <p data-slot="checkbox-group-controlled-uncontrolled-na">
                    "Controlled vs Uncontrolled contrast is N/A at group level: this component does not own a group value axis (`value/on_value_change/default_value`); child `Checkbox` owns checked state."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="checkbox-group-source-first">
                <h3>"Source-first / Copy-ready"</h3>
                <p data-slot="checkbox-group-copy-ready">
                    "Each playground supports code + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " and include "
                    <code>"use leptos::prelude::*; use ui::*;"</code>
                    "."
                </p>
                <ul data-slot="checkbox-group-source-paths">
                    <li><code>"components/checkbox-group/src/view.rs"</code></li>
                    <li><code>"components/checkbox-group/src/logic.rs"</code></li>
                    <li><code>"components/checkbox-group/src/styles.rs"</code></li>
                    <li><code>"apps/docs-app/src/pages/components/pages/forms.rs"</code></li>
                </ul>
                <ul data-slot="checkbox-group-source-prerequisites">
                    <li>
                        <code>"ui"</code>
                        " with feature "
                        <code>"component-checkbox_group"</code>
                    </li>
                    <li>
                        <code>"inject-css"</code>
                        " enabled in docs acceptance surface"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
