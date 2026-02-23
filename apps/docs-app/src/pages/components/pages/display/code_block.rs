use super::*;

pub(crate) fn code_block() -> AnyView {
    fn workbench_template(language: &str) -> &'static str {
        match language {
            "bash" => {
                r#"cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings"#
            }
            "plain" => "CodeBlock workbench template for layout and style inspection.",
            _ => {
                r#"fn deploy(service: &str) -> anyhow::Result<()> {
    tracing::info!(target: "deploy", %service, "starting rollout");
    Ok(())
}"#
            }
        }
    }

    let rust_code = workbench_template("rust");

    let hello_world_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo check -p ui".to_string()
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<CodeBlock
  code=rust_code.into()
  language="rust".to_string()
  label="deploy.rs".to_string()
/>"#
        .to_string()
    });

    let compact_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  is_copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#
        .to_string()
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<CodeBlock code="cargo check -p ui".to_string() />
<CodeBlock
  code="cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings".to_string()
  language="bash".to_string()
  label="ci.sh".to_string()
/>
<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  is_copyable=false
/>
<CodeBlock
  code="   ".to_string()
  is_copyable=false
  class_name="docs-code-block-custom".to_string()
/>"#
        .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"let (controlled_copied, set_controlled_copied) = signal(false);
let controlled_copied_signal = Signal::derive(move || controlled_copied.get());

<CodeBlock
  code="Uncontrolled: internal copied state.".to_string()
  default_copied=true
/>
<CodeBlock
  code="Controlled: copied state from app signal.".to_string()
  is_copied=controlled_copied_signal
  on_copied_change=Callback::new(move |next| set_controlled_copied.set(next))
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<CodeBlock
  code="Snapshot: complete validated output rendered in one pass.".to_string()
  language="plain".to_string()
  output_mode=CodeBlockAgentOutputMode::Snapshot
  output_status=CodeBlockAgentOutputStatus::Validated
/>
<CodeBlock
  code="Streaming: incremental draft output while LLM is generating.".to_string()
  language="plain".to_string()
  output_mode=CodeBlockAgentOutputMode::Streaming
  output_status=CodeBlockAgentOutputStatus::Draft
/>"#
        .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<CodeBlock
  code="cargo test -p ui --test code_block_semantics".to_string()
  language="bash".to_string()
/>"#
        .to_string()
    });
    let code_block_imports = "use leptos::prelude::*;\nuse ui::CodeBlock;".to_string();
    let code_block_stream_imports = "use leptos::prelude::*;\nuse ui::CodeBlock;\nuse ui::code_block::protocol::{CodeBlockAgentOutputMode, CodeBlockAgentOutputStatus};".to_string();

    let language_options = vec!["rust".to_string(), "bash".to_string(), "plain".to_string()];
    let output_mode_options = vec!["snapshot".to_string(), "streaming".to_string()];
    let output_status_options = vec![
        "draft".to_string(),
        "validated".to_string(),
        "ready-to-submit".to_string(),
    ];
    let (workbench_language_index, set_workbench_language_index) = signal(Some(0_usize));
    let (workbench_is_copyable, set_workbench_is_copyable) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_preserve_state, set_workbench_preserve_state) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_output_mode_index, set_workbench_output_mode_index) = signal(Some(0_usize));
    let (workbench_output_status_index, set_workbench_output_status_index) = signal(Some(1_usize));
    let (workbench_code_text, set_workbench_code_text) = signal(rust_code.to_string());
    let (workbench_copied, set_workbench_copied) = signal(false);
    let (controlled_copied, set_controlled_copied) = signal(false);
    let controlled_copied_signal = Signal::derive(move || controlled_copied.get());
    let on_controlled_copied_change =
        Callback::new(move |next: bool| set_controlled_copied.set(next));
    let on_controlled_reset = Callback::new(move |_| set_controlled_copied.set(false));

    let workbench_language_key =
        Signal::derive(move || match workbench_language_index.get().unwrap_or(0) {
            1 => "bash",
            2 => "plain",
            _ => "rust",
        });
    let workbench_language = Signal::derive(move || {
        let key = workbench_language_key.get();
        if key == "plain" {
            String::new()
        } else {
            key.into()
        }
    });
    let workbench_output_mode =
        Signal::derive(
            move || match workbench_output_mode_index.get().unwrap_or(0) {
                1 => ui::code_block::protocol::CodeBlockAgentOutputMode::Streaming,
                _ => ui::code_block::protocol::CodeBlockAgentOutputMode::Snapshot,
            },
        );
    let workbench_output_status =
        Signal::derive(
            move || match workbench_output_status_index.get().unwrap_or(1) {
                0 => ui::code_block::protocol::CodeBlockAgentOutputStatus::Draft,
                2 => ui::code_block::protocol::CodeBlockAgentOutputStatus::ReadyToSubmit,
                _ => ui::code_block::protocol::CodeBlockAgentOutputStatus::Validated,
            },
        );

    Effect::new(move |_| {
        if !workbench_preserve_state.get() {
            let template = workbench_template(workbench_language_key.get());
            set_workbench_code_text.set(template.to_string());
            set_workbench_copied.set(false);
        }
    });

    let workbench_copied_signal = Signal::derive(move || {
        if workbench_preserve_state.get() {
            workbench_copied.get()
        } else {
            false
        }
    });
    let workbench_on_copied_change = Callback::new(move |next: bool| {
        if workbench_preserve_state.get_untracked() {
            set_workbench_copied.set(next);
        }
    });

    let workbench_code = Signal::derive(move || {
        let language_key = workbench_language_key.get();
        let is_copyable = workbench_is_copyable.get();
        let custom_class = workbench_custom_class.get();
        let preserve_state = workbench_preserve_state.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let output_mode = workbench_output_mode.get();
        let output_status = workbench_output_status.get();
        let code_literal = format!("{:?}", workbench_code_text.get());

        let mut lines = vec![
            "<CodeBlock".to_string(),
            format!("  code={code_literal}.to_string()"),
            "  label=\"workbench.rs\".to_string()".to_string(),
            format!("  lang={lang:?}.to_string()"),
            format!("  dir={dir}"),
            "  motion=CodeBlockMotion::default()".to_string(),
        ];
        if language_key != "plain" {
            lines.push(format!("  language=\"{language_key}\".to_string()"));
        }
        if !is_copyable {
            lines.push("  is_copyable=false".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-code-block-custom\".to_string()".to_string());
        }
        if preserve_state {
            lines.push("  is_copied=workbench_copied_signal".to_string());
            lines.push("  copied=workbench_copied_signal".to_string());
            lines.push("  default_copied=false".to_string());
            lines.push(
                "  on_copied_change=Callback::new(move |next| set_workbench_copied.set(next))"
                    .to_string(),
            );
        }
        lines.push(format!(
            "  output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::{output_mode:?}"
        ));
        lines.push(format!(
            "  output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::{output_status:?}"
        ));
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let language_key = workbench_language_key.get();
        let language = if language_key == "plain" {
            "none"
        } else {
            language_key
        };
        let is_copyable = workbench_is_copyable.get();
        let custom_class = workbench_custom_class.get();
        let preserve_state = workbench_preserve_state.get();
        let code = workbench_code_text.get();
        let copied = workbench_copied.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let output_mode = workbench_output_mode.get();
        let output_status = workbench_output_status.get();

        format!(
            "CodeBlockActualConfig {{\n  code: {code:?},\n  label: Some(\"workbench.rs\"),\n  language: {language:?},\n  lang: Some({lang:?}),\n  dir: Some({dir}),\n  is_copyable: Some({is_copyable}),\n  copyable: Some({is_copyable}),\n  is_copied: Some({copied}),\n  copied: Some({copied}),\n  default_copied: Some(false),\n  on_copied_change: Some(\"workbench_on_copied_change\"),\n  output_mode: Some(\"{}\"),\n  output_status: Some(\"{}\"),\n  motion: CodeBlockMotion::default(),\n  class_name: {class_name},\n  preserve_state: {preserve_state},\n  code_lines: {},\n}}\n\nCodeBlockPreviewExpectation {{\n  data-ui-output-mode: \"{}\",\n  data-ui-output-status: \"{}\",\n}}",
            output_mode.as_attr(),
            output_status.as_attr(),
            code.lines().count(),
            output_mode.as_attr(),
            output_status.as_attr(),
            class_name = if custom_class {
                "Some(\"docs-code-block-custom\")"
            } else {
                "None"
            },
        )
    });

    let workbench_test_css = Signal::derive(move || {
        let mut css = format!(
            "/* components/code-block/src/styles.rs */\n{}",
            ui::code_block::styles::CSS
        );

        if workbench_custom_class.get() {
            css.push_str(
                "\n\n/* docs custom override */\n.docs-code-block-custom {\n  --ui-code-block-copy-flash: 0.32;\n  border-color: color-mix(in oklab, var(--ui-border), var(--ui-accent) 38%);\n}\n",
            );
        }

        css
    });

    let on_workbench_load_template = Callback::new(move |_| {
        let template = workbench_template(workbench_language_key.get_untracked());
        set_workbench_code_text.set(template.to_string());
        if !workbench_preserve_state.get_untracked() {
            set_workbench_copied.set(false);
        }
    });
    let on_workbench_reset_copy_state = Callback::new(move |_| set_workbench_copied.set(false));

    view! {
        <ComponentPage
            title="CodeBlock"
            slug="code-block"
            group="Display"
            description="Multiline code surface with centralized header/state attrs and spring-driven copy flash motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock code="cargo check -p ui".to_string() />
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="调样式走 CSS Test 即时反馈；`preserve_state` 可选保持复制状态和编辑上下文，降低重复操作。"
                code_signal=workbench_code
                code_imports=code_block_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="components/code-block/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="code-block-workbench-controls">
                            <SegmentedControl
                                id_base="docs-code-block-workbench-language".to_string()
                                options=language_options.clone()
                                selected_index=workbench_language_index
                                set_selected_index=set_workbench_language_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock language".to_string()
                            />
                            <SegmentedControl
                                id_base="docs-code-block-workbench-output-mode".to_string()
                                options=output_mode_options.clone()
                                selected_index=workbench_output_mode_index
                                set_selected_index=set_workbench_output_mode_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock output mode".to_string()
                            />
                            <SegmentedControl
                                id_base="docs-code-block-workbench-output-status".to_string()
                                options=output_status_options.clone()
                                selected_index=workbench_output_status_index
                                set_selected_index=set_workbench_output_status_index
                                size=SegmentedControlSize::Sm
                                aria_label="CodeBlock output status".to_string()
                            />

                            <div class="docs-row">
                                <Switch checked=workbench_is_copyable set_checked=set_workbench_is_copyable>
                                    "copyable"
                                </Switch>
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "custom class"
                                </Switch>
                                <Switch checked=workbench_preserve_state set_checked=set_workbench_preserve_state>
                                    "preserve state"
                                </Switch>
                                <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                    "lang=zh-CN"
                                </Switch>
                                <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                                    "dir=rtl"
                                </Switch>
                            </div>

                            <div class="docs-row">
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    size=ui::ButtonSize::Sm
                                    on_press=on_workbench_load_template
                                >
                                    "Load template"
                                </ui::Button>
                                <ui::Button
                                    variant=ui::ButtonVariant::Secondary
                                    size=ui::ButtonSize::Sm
                                    on_press=on_workbench_reset_copy_state
                                >
                                    "Reset copied state"
                                </ui::Button>
                            </div>

                            <label class="docs-search__label" for="docs-code-block-workbench-code">
                                "Code"
                            </label>
                            <textarea
                                id="docs-code-block-workbench-code"
                                class="docs-search__input"
                                rows="7"
                                prop:value=move || workbench_code_text.get()
                                on:input=move |ev| set_workbench_code_text.set(event_target_value(&ev))
                            />
                        </div>
                    }
                }
            >
                <div class="docs-stack" data-slot="code-block-workbench-preview">
                    {move || {
                        let code = workbench_code_text.get();
                        let language = workbench_language.get();
                        let is_copyable = workbench_is_copyable.get();
                        let output_mode = workbench_output_mode.get();
                        let output_status = workbench_output_status.get();
                        let class_name = if workbench_custom_class.get() {
                            "docs-code-block-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        };
                        let dir = if workbench_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        };

                        view! {
                            <CodeBlock
                                code
                                label="workbench.rs".to_string()
                                language
                                lang
                                dir=dir
                                is_copyable
                                class_name
                                is_copied=workbench_copied_signal
                                copied=workbench_copied_signal
                                default_copied=false
                                on_copied_change=workbench_on_copied_change
                                output_mode
                                output_status
                                motion=ui::CodeBlockMotion::default()
                            />
                        }
                            .into_any()
                    }}

                    <CodeBlock
                        code="cargo test -p ui --test code_block_semantics".to_string()
                        language="bash".to_string()
                        is_copyable=false
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                description="覆盖 single-line/multiline、header visible/hidden、copyable on/off、empty/custom class 等关键状态轴。"
                code_signal=state_matrix_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-state-matrix-preview">
                    <CodeBlock code="cargo check -p ui".to_string() />
                    <CodeBlock
                        code={r#"cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings"#.to_string()}
                        language="bash".to_string()
                        label="ci.sh".to_string()
                    />
                    <CodeBlock
                        code="cargo test -p ui --test code_block_semantics".to_string()
                        is_copyable=false
                    />
                    <CodeBlock
                        code="   ".to_string()
                        is_copyable=false
                        class_name="docs-code-block-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Header + Copy Motion"
                code_signal=matrix_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code=rust_code.to_string()
                    language="rust".to_string()
                    label="deploy.rs".to_string()
                />
            </Playground>

            <Playground
                title="Compact + No Copy"
                code_signal=compact_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code="cargo test -p ui --test code_block_semantics".to_string()
                    is_copyable=false
                    class_name="docs-code-block-custom".to_string()
                />
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Copied State)"
                description="对照 `default_copied`（非受控）与 `is_copied + on_copied_change`（受控）语义。"
                code_signal=controlled_contrast_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-controlled-preview">
                    <CodeBlock
                        code="Uncontrolled: internal copied state starts from default_copied=true.".to_string()
                        default_copied=true
                    />
                    <CodeBlock
                        code="Controlled: copied state comes from app signal.".to_string()
                        is_copied=controlled_copied_signal
                        on_copied_change=on_controlled_copied_change
                    />
                    <div class="docs-row">
                        <span class="ui-muted">
                            {move || format!("controlled copied: {}", controlled_copied.get())}
                        </span>
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            size=ui::ButtonSize::Sm
                            on_press=on_controlled_reset
                        >
                            "Reset controlled copied"
                        </ui::Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="CodeBlock 默认 Snapshot；如需边生成边显示，可显式启用 Streaming，并保持 output status 连续可读。"
                code_signal=stream_snapshot_code
                code_imports=code_block_stream_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-streaming-preview">
                    <CodeBlock
                        code="Snapshot: complete validated output rendered in one pass.".to_string()
                        language="plain".to_string()
                        output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::Snapshot
                        output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::Validated
                    />
                    <CodeBlock
                        code="Streaming: incremental draft output while LLM is generating.".to_string()
                        language="plain".to_string()
                        output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::Streaming
                        output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::Draft
                    />
                    <p class="ui-muted">
                        "Inspect "
                        <code>"data-ui-output-mode"</code>
                        " and "
                        <code>"data-ui-output-status"</code>
                        " on each root."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="复制按钮输出最小可运行片段，并自动补齐 imports。"
                code_signal=source_first_code
                code_imports=code_block_imports.clone()
            >
                <CodeBlock
                    code="cargo test -p ui --test code_block_semantics".to_string()
                    language="bash".to_string()
                />
            </Playground>

            <Playground
                title="State Matrix (Copy + Output Modes)"
                code_signal=state_matrix_code
                code_imports=code_block_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="code-block-state-matrix-v2">
                    <CodeBlock
                        code="cargo check -p ui".to_string()
                        label="check.sh".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::CodeBlockMotion::default()
                    />
                    <CodeBlock
                        code="cargo fmt --all".to_string()
                        language="bash".to_string()
                        label="fmt.sh".to_string()
                        is_copyable=false
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::CodeBlockMotion::default()
                    />
                    <CodeBlock
                        code="cargo clippy --workspace --all-targets -- -D warnings".to_string()
                        language="bash".to_string()
                        label="clippy.sh".to_string()
                        default_copied=false
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        motion=ui::CodeBlockMotion::default()
                        class_name="docs-code-block-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="code-block-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="code-block-state-rows">
                    <li><code>"data-state"</code>" = single-line | multiline"</li>
                    <li><code>"data-header"</code>" = visible | hidden"</li>
                    <li><code>"data-copyable / data-copied"</code>" = true | none"</li>
                    <li><code>"data-copyable-source"</code>" = default | is_copyable | copyable_legacy"</li>
                    <li><code>"data-copied-source"</code>" = uncontrolled | controlled"</li>
                    <li><code>"data-empty / data-custom-class"</code>" = true | none"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="code-block-parameter-rows">
                    <li><code>"is_copyable"</code>" default = true"</li>
                    <li>
                        <code>"copyable"</code>
                        " = historical alias; normalization priority: "
                        <code>"is_copyable > copyable > true"</code>
                    </li>
                    <li><code>"default_copied"</code>" default = false"</li>
                    <li>
                        <code>"is_copied + on_copied_change"</code>
                        " = controlled copied-state API"
                    </li>
                    <li><code>"output_mode"</code>" default = snapshot"</li>
                    <li><code>"output_status"</code>" default = validated"</li>
                    <li><code>"disabled axis"</code>" = N/A (CodeBlock has no disabled prop)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="code-block-streaming-rows">
                    <li><code>"data-ui-output-mode"</code>" = snapshot | streaming"</li>
                    <li><code>"data-ui-output-status"</code>" = draft | validated | ready-to-submit"</li>
                    <li><code>"default fallback"</code>" = snapshot"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-block-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="code-block-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-code_block"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        ") so copied snippets preserve baseline styles."
                    </li>
                </ul>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::CodeBlock;\n\n<CodeBlock\n  code=\"cargo test -p ui --test code_block_semantics\".to_string()\n  language=\"bash\".to_string()\n/>".to_string()
                    label="Copy code starter".to_string()
                    copyable=true
                    class_name="docs-code-block-source-copy".to_string()
                />
                <ul data-slot="code-block-source-paths">
                    <li><code>"components/code-block/src/mod.rs"</code></li>
                    <li><code>"components/code-block/src/logic.rs"</code></li>
                    <li><code>"components/code-block/src/view.rs"</code></li>
                    <li><code>"components/code-block/src/styles.rs"</code></li>
                    <li><code>"components/code-block/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
