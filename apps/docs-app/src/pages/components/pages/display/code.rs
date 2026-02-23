use super::*;

pub(crate) fn code() -> AnyView {
    let hello_world_code =
        Signal::derive(move || r#"<Code>"cargo check -p ui"</Code>"#.to_string());

    let variants_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline>"cargo test -p ui"</Code>
<Code variant=CodeVariant::Block>
  "cargo fmt --all\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings"
</Code>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>"--deny warnings"</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui --test code_semantics\ncargo test -p ui"
</Code>"#.to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"<Code>"Default path: no controlled/uncontrolled state axis."</Code>
<Code variant=CodeVariant::Block>
  "Controlled-like usage lives in app state only: map upstream state to variant/class_name props."
</Code>"#
            .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline>
  "Snapshot: complete validated output rendered in one pass."
</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "Streaming Optional -> fallback=snapshot; inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-output-state=verified."
</Code>"#
            .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
  "cargo test -p ui --test code_semantics"
</Code>"#
            .to_string()
    });
    let variant_options = vec!["Inline".to_string(), "Block".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (custom_class, set_custom_class) = signal(false);
    let (long_content, set_long_content) = signal(false);
    let (show_compare, set_show_compare) = signal(true);
    let (custom_lang, set_custom_lang) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

    let active_variant = Signal::derive(move || {
        if variant_index.get().unwrap_or(0) == 1 {
            CodeVariant::Block
        } else {
            CodeVariant::Inline
        }
    });
    let active_content = Signal::derive(move || {
        if long_content.get() {
            "cargo fmt --all\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings"
                .to_string()
        } else {
            "cargo test -p ui --test code_semantics".to_string()
        }
    });
    let interactive_code = Signal::derive(move || {
        let variant = active_variant.get();
        let content = active_content.get();
        let class_line = if custom_class.get() {
            " class_name=\"docs-code-custom\".into()".to_string()
        } else {
            "".to_string()
        };
        let lang_line = if custom_lang.get() {
            "\n  lang=\"zh-CN\".into()"
        } else {
            ""
        };
        let dir_line = if rtl_dir.get() {
            "\n  dir=A11yDirection::Rtl"
        } else {
            "\n  dir=A11yDirection::Ltr"
        };
        format!(
            "<Code variant=CodeVariant::{variant:?}{class_line}{lang_line}{dir_line}>\n  {content:?}\n</Code>"
        )
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/code/src/styles.rs */\n{}",
            ui::code::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let variant = active_variant.get();
        let has_custom_class = custom_class.get();
        let show_compare = show_compare.get();
        let content_mode = if long_content.get() { "long" } else { "short" };
        let class_name = if has_custom_class {
            "docs-code-custom"
        } else {
            "(none)"
        };
        let lang = if custom_lang.get() { "zh-CN" } else { "en-US" };
        let dir = if rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        format!(
            "CodeActualConfig {{\n  variant: CodeVariant::{variant:?},\n  content_mode: \"{content_mode}\",\n  has_custom_class_name: {has_custom_class},\n  class_name: \"{class_name}\",\n  lang: {:?},\n  dir: {:?},\n  show_compare: {show_compare},\n}}",
            lang, dir
        )
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<Code variant=CodeVariant::Inline lang="en-US".to_string() dir=A11yDirection::Ltr>"cargo test -p ui"</Code>
<Code variant=CodeVariant::Block class_name="docs-code-custom".to_string() lang="en-US".to_string() dir=A11yDirection::Ltr>
  "cargo fmt --all\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings"
</Code>
<Code variant=CodeVariant::Inline lang="zh-CN".to_string() dir=A11yDirection::Rtl>"--deny warnings"</Code>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Code"
            slug="code"
            group="Display"
            description="Inline/Block code surface with centralized variant state attrs and optional custom-class contract."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_world_code>
                <Code>"cargo check -p ui"</Code>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="components/code/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；包含 inline/block 与 custom class 的对比展示。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="code-workbench-controls">
                        <div class="docs-search__label">"配置区 · Variant"</div>
                        <ui::SegmentedControl
                            id_base="docs-code-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=ui::SegmentedControlSize::Sm
                            aria_label="Code variant".to_string()
                        />
                        <ui::Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </ui::Switch>
                        <ui::Switch checked=long_content set_checked=set_long_content>
                            "Long content"
                        </ui::Switch>
                        <ui::Switch checked=show_compare set_checked=set_show_compare>
                            "Show compare matrix"
                        </ui::Switch>
                        <ui::Switch checked=custom_lang set_checked=set_custom_lang>
                            "Lang=zh-CN"
                        </ui::Switch>
                        <ui::Switch checked=rtl_dir set_checked=set_rtl_dir>
                            "dir=rtl"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let variant = active_variant.get();
                    let content = active_content.get();
                    let class_name = if custom_class.get() {
                        "docs-code-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if custom_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    };
                    let dir = if rtl_dir.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };
                    let compare = show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="code-workbench-preview">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="code-workbench-primary">
                                <span class="ui-muted">
                                    {format!("variant={variant:?}, custom_class={}", custom_class.get())}
                                </span>
                                // <Code variant=variant class_name=class_name.clone()>
                                <Code
                                    variant=variant
                                    class_name=class_name.clone()
                                    lang=lang.clone()
                                    dir=dir
                                >
                                    {content}
                                </Code>
                            </div>

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight" data-slot="code-workbench-compare">
                                    <div class="docs-row">
                                        <span>"Inline: "</span>
                                        <Code
                                            variant=CodeVariant::Inline
                                            class_name=class_name.clone()
                                            lang=lang.clone()
                                            dir=dir
                                        >
                                            "cargo test -p ui"
                                        </Code>
                                    </div>
                                    <Code
                                        variant=CodeVariant::Block
                                        class_name=class_name.clone()
                                        lang=lang.clone()
                                        dir=dir
                                    >
                                        {r#"cargo fmt --all
cargo clippy -p ui -p docs-app --all-targets -- -D warnings"#}
                                    </Code>
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Variant / Class / Locale Comparison)"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};\nuse ui::color::area::A11yDirection;".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Code variant=CodeVariant::Inline lang="en-US".to_string() dir=A11yDirection::Ltr>
                            "cargo test -p ui"
                        </Code>
                    </div>
                    <Code
                        variant=CodeVariant::Block
                        class_name="docs-code-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        {r#"cargo fmt --all
cargo clippy -p ui -p docs-app --all-targets -- -D warnings"#}
                    </Code>
                    <div class="docs-row">
                        <Code variant=CodeVariant::Inline lang="zh-CN".to_string() dir=A11yDirection::Rtl>
                            "--deny warnings"
                        </Code>
                    </div>
                </div>
            </Playground>

            <Playground title="Variant Matrix" code_signal=variants_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <span>"Run "</span>
                        <Code variant=CodeVariant::Inline>"cargo test -p ui"</Code>
                        <span>" before opening a PR."</span>
                    </div>
                    <Code variant=CodeVariant::Block>
                        {r#"cargo fmt --all
cargo clippy -p ui -p docs-app --all-targets -- -D warnings"#}
                    </Code>
                </div>
            </Playground>

            <Playground title="Custom Class + Block" code_signal=custom_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <span>"CI flags: "</span>
                        <Code variant=CodeVariant::Inline class_name="docs-code-custom".to_string()>
                            "--deny warnings"
                        </Code>
                    </div>
                    <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                        {r#"cargo test -p ui --test code_semantics
cargo test -p ui"#}
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Code has no internal controlled/uncontrolled axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <div class="docs-stack">
                    <Code>"Default path: no controlled/uncontrolled state axis."</Code>
                    <Code variant=CodeVariant::Block>
                        "Controlled-like usage lives in app state only: map upstream state to variant/class_name props."
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Code is a display leaf: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <div class="docs-stack">
                    <Code variant=CodeVariant::Inline>
                        "Snapshot: complete validated output rendered in one pass."
                    </Code>
                    <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                        "Streaming Optional -> fallback=snapshot; inspect data-ui-streaming=optional, data-ui-fallback=snapshot, data-ui-output-state=verified."
                    </Code>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};".to_string()
            >
                <Code variant=CodeVariant::Block class_name="docs-code-custom".to_string()>
                    "cargo test -p ui --test code_semantics"
                </Code>
            </Playground>

            <section class="docs-card docs-prose" data-slot="code-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="code-state-rows">
                    <li><code>"data-variant / data-state"</code>" = inline | block"</li>
                    <li><code>"data-inline / data-block"</code>" = true | none"</li>
                    <li><code>"data-custom-class"</code>" = true | none"</li>
                    <li><code>"control mode"</code>" = N/A (Code has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Code has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="code-parameter-rows">
                    <li><code>"variant: Option&lt;CodeVariant&gt;"</code>" default = None -> normalize to inline (`logic.rs`: `variant.unwrap_or_default()`)"</li>
                    <li><code>"class_name: Option&lt;String&gt;"</code>" default = None -> `normalize_optional_text` trims blank/empty to None"</li>
                    <li><code>"lang: Option&lt;String&gt;, dir: Option&lt;A11yDirection&gt;"</code>" default = None -> locale inherited via `locale_attrs`"</li>
                    <li><code>"children: Children"</code>" required; component renders caller-provided code content only"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="code-streaming-rows">
                    <li><code>"data-ui-streaming"</code>" = optional"</li>
                    <li><code>"data-ui-fallback"</code>" = snapshot"</li>
                    <li><code>"data-ui-output-state"</code>" = verified"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="code-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="code-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-code"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{Code, CodeVariant};\n\n<Code variant=CodeVariant::Block>\n  \"cargo test -p ui --test code_semantics\"\n</Code>".to_string()
                    label="Copy code starter".to_string()
                    copyable=true
                    class_name="docs-code-source-copy".to_string()
                />
                <ul data-slot="code-source-paths">
                    <li><code>"components/code/src/mod.rs"</code></li>
                    <li><code>"components/code/src/logic.rs"</code></li>
                    <li><code>"components/code/src/view.rs"</code></li>
                    <li><code>"components/code/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
