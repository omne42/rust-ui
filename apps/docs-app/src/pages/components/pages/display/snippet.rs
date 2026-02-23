use super::*;

pub(crate) fn snippet() -> AnyView {
    let (workbench_copied_raw, set_workbench_copied_raw) = signal(false);
    let workbench_copied_signal: Signal<bool> = Signal::derive(move || workbench_copied_raw.get());
    let (workbench_on_copied_change_runs, set_workbench_on_copied_change_runs) = signal(0_u32);
    let on_copied_change = Callback::new(move |next: bool| {
        set_workbench_copied_raw.set(next);
        set_workbench_on_copied_change_runs.update(|count| *count += 1);
    });

    let (workbench_copyable, set_workbench_copyable) = signal(true);
    let (workbench_multiline, set_workbench_multiline) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<Snippet
  text="cargo fmt --all".to_string()
  label="Command".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let text = if workbench_multiline.get() {
            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
        } else {
            "cargo fmt --all"
        };
        let label = if workbench_custom_label.get() {
            "CI command"
        } else {
            "Command"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-snippet-custom"
        } else {
            ""
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_reduced_motion.get() {
            "SnippetMotion::disabled()"
        } else {
            "SnippetMotion::default()"
        };

        [
            "<Snippet".to_string(),
            format!("  text={}", rust_string_literal(text)),
            format!("  label={}", rust_string_literal(label)),
            format!("  is_copyable={}", bool_word(workbench_copyable.get())),
            format!("  copyable={}", bool_word(workbench_copyable.get())),
            "  copy_label=\"Copy\".to_string()".to_string(),
            "  copied_label=\"Copied\".to_string()".to_string(),
            "  copy_aria_label=\"Copy snippet\".to_string()".to_string(),
            "  copy_error_label=\"Copy failed\".to_string()".to_string(),
            "  is_copied=workbench_copied_signal".to_string(),
            "  copied=workbench_copied_signal".to_string(),
            "  default_copied=false".to_string(),
            "  on_copied_change=on_copied_change".to_string(),
            format!("  motion={motion}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let text = if workbench_multiline.get() {
            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
        } else {
            "cargo fmt --all"
        };
        let label = if workbench_custom_label.get() {
            Some("CI command")
        } else {
            Some("Command")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-snippet-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if workbench_reduced_motion.get() {
            SnippetMotion::disabled()
        } else {
            SnippetMotion::default()
        };

        format!(
            "SnippetActualConfig {{\n  text: {},\n  label: {label:?},\n  is_copyable: Some({}),\n  copyable: Some({}),\n  copy_label: Some(\"Copy\"),\n  copied_label: Some(\"Copied\"),\n  copy_aria_label: Some(\"Copy snippet\"),\n  copy_error_label: Some(\"Copy failed\"),\n  is_copied: Some({}),\n  copied: Some({}),\n  default_copied: Some(false),\n  on_copied_change: \"runs={}\",\n  motion: {motion:?},\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n}}",
            rust_string_literal(text),
            bool_word(workbench_copyable.get()),
            bool_word(workbench_copyable.get()),
            bool_word(workbench_copied_raw.get()),
            bool_word(workbench_copied_raw.get()),
            workbench_on_copied_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Snippet text="cargo fmt --all".to_string() label="Default".to_string() is_copyable=true />
<Snippet text="cargo test -p ui --test snippet_semantics".to_string() label="Static".to_string() is_copyable=false class_name="docs-snippet-custom".to_string() />
<Snippet text="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string() label="Multiline".to_string() copyable=true motion=SnippetMotion::disabled() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Snippet"
            slug="snippet"
            group="Display"
            description="Snippet playground with full API workbench and visible copy callback feedback."
        >
            <Playground title="Hello World (Copyable Snippet)" code_signal=hello_code>
                <Snippet text="cargo fmt --all".to_string() label="Command".to_string() />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="snippet-workbench-controls">
                        <Switch checked=workbench_copyable set_checked=set_workbench_copyable>
                            "Copy enabled"
                        </Switch>
                        <Switch checked=workbench_multiline set_checked=set_workbench_multiline>
                            "Multiline text"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom label"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_workbench_copied_raw.update(|value| *value = !*value)
                            })
                        >
                            "Toggle copied signal"
                        </ui::Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="snippet-workbench-preview">
                    <Snippet
                        text=if workbench_multiline.get() {
                            "cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"
                                .to_string()
                        } else {
                            "cargo fmt --all".to_string()
                        }
                        label=if workbench_custom_label.get() {
                            "CI command".to_string()
                        } else {
                            "Command".to_string()
                        }
                        is_copyable=workbench_copyable.get()
                        copyable=workbench_copyable.get()
                        copy_label="Copy".to_string()
                        copied_label="Copied".to_string()
                        copy_aria_label="Copy snippet".to_string()
                        copy_error_label="Copy failed".to_string()
                        is_copied=workbench_copied_signal
                        copied=workbench_copied_signal
                        default_copied=false
                        on_copied_change=on_copied_change
                        motion=if workbench_reduced_motion.get() {
                            SnippetMotion::disabled()
                        } else {
                            SnippetMotion::default()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-snippet-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="snippet-workbench-feedback">
                        "copied: " {move || workbench_copied_raw.get()}
                        " · on_copied_change: " {move || workbench_on_copied_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Static / Multiline)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight" data-slot="snippet-state-matrix">
                    <Snippet
                        text="cargo fmt --all".to_string()
                        label="Default".to_string()
                        is_copyable=true
                    />
                    <Snippet
                        text="cargo test -p ui --test snippet_semantics".to_string()
                        label="Static".to_string()
                        is_copyable=false
                        class_name="docs-snippet-custom".to_string()
                    />
                    <Snippet
                        text="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string()
                        label="Multiline".to_string()
                        copyable=true
                        motion=SnippetMotion::disabled()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
