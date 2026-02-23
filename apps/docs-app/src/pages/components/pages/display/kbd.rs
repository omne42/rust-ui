use super::*;

pub(crate) fn kbd() -> AnyView {
    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_keys, set_workbench_keys) = signal("Ctrl".to_string());
    let (workbench_label, set_workbench_label) = signal("K".to_string());
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_size = Signal::derive(move || match workbench_size_key.get().as_str() {
        "sm" => KbdSize::Sm,
        _ => KbdSize::Md,
    });

    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let keys = workbench_keys.get();
        let label = workbench_label.get();
        let custom_class = workbench_custom_class.get();
        let keys_trimmed = keys.trim();
        let label_trimmed = label.trim();

        let mut lines = vec!["<Kbd".to_string()];
        if size != KbdSize::Md {
            lines.push(format!("  size=KbdSize::{size:?}"));
        }
        if !keys_trimmed.is_empty() {
            lines.push(format!("  keys={keys_trimmed:?}.into()"));
        }
        if custom_class {
            lines.push("  class_name=\"docs-kbd-custom\".into()".to_string());
        }
        lines.push(">".to_string());
        lines.push(format!(
            "  {:?}",
            if label_trimmed.is_empty() {
                "K"
            } else {
                &label_trimmed
            }
        ));
        lines.push("</Kbd>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* components/kbd/src/styles.rs */\n{}",
            ui::kbd::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let keys = workbench_keys.get();
        let label = workbench_label.get();
        let custom_class = workbench_custom_class.get();
        let has_keys = !keys.trim().is_empty();

        let mut classes = vec![
            "ui-kbd".to_string(),
            size.class_name().into(),
            if has_keys {
                "ui-kbd--state-with-keys".to_string()
            } else {
                "ui-kbd--state-label-only".to_string()
            },
        ];
        if custom_class {
            classes.push("ui-kbd--custom-class".to_string());
            classes.push("docs-kbd-custom".to_string());
        }

        format!(
            "KbdActualConfig {{\n  size: {size:?},\n  keys: {:?},\n  label: {:?},\n  class_name: {:?},\n  custom_class: {custom_class},\n  data_size: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            keys.trim(),
            label.trim(),
            if custom_class {
                Some("docs-kbd-custom")
            } else {
                None
            },
            size.as_attr(),
            if has_keys { "with-keys" } else { "label-only" },
            classes.join(" "),
        )
    });

    let hello_world_code =
        Signal::derive(move || r#"<Kbd keys="Ctrl".to_string()>"K"</Kbd>"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
<Kbd size=KbdSize::Md>"Esc"</Kbd>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let label = Signal::derive(move || "K".to_string());

<Kbd keys="Ctrl".to_string()>"K"</Kbd>
<Kbd keys="Ctrl".to_string()>{label.get()}</Kbd>
// N/A: Kbd has no controlled/uncontrolled runtime axis (`value/on_value_change/default_value`)."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Kbd keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm>"Esc"</Kbd>
// Streaming Optional -> fallback=snapshot for Kbd display leaf."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Sm keys="Shift".to_string() class_name="docs-kbd-custom".to_string()>
    "Tab"
</Kbd>"#
            .to_string()
    });

    let kbd_imports = "use leptos::prelude::*;\nuse ui::{Kbd, KbdSize};".to_string();

    let custom_code = Signal::derive(move || {
        r#"<Kbd size=KbdSize::Md class_name="docs-kbd-custom".to_string()>"Esc"</Kbd>
<Kbd size=KbdSize::Sm keys="Shift".to_string() class_name="docs-kbd-custom".to_string()>"Tab"</Kbd>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Kbd"
            slug="kbd"
            group="Display"
            description="Keyboard keycap with centralized size/keys state attrs and optional custom-class contract."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for size/keys/class contracts."
                code_signal=workbench_code
                code_imports=kbd_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/kbd/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="kbd-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="md">"Md"</option>
                                <option value="sm">"Sm"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Keys"
                            <input
                                class="docs-search__input"
                                prop:value=move || workbench_keys.get()
                                on:input=move |ev| set_workbench_keys.set(event_target_value(&ev))
                                placeholder="Ctrl"
                            />
                        </label>
                        <label class="docs-search__label">
                            "Label"
                            <input
                                class="docs-search__input"
                                prop:value=move || workbench_label.get()
                                on:input=move |ev| set_workbench_label.set(event_target_value(&ev))
                                placeholder="K"
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let keys = workbench_keys.get();
                    let label = workbench_label.get();
                    let label = {
                        let trimmed = label.trim();
                        if trimmed.is_empty() {
                            "K".to_string()
                        } else {
                            trimmed.to_string()
                        }
                    };
                    let keys = {
                        let trimmed = keys.trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed.to_string())
                        }
                    };
                    let has_keys = keys.is_some();

                    if workbench_custom_class.get() && has_keys {
                        let keys_text = keys.unwrap_or_default();
                        view! {
                            <div class="docs-row">
                                <Kbd
                                    size=size
                                    keys=keys_text
                                    class_name="docs-kbd-custom".to_string()
                                >
                                    {label}
                                </Kbd>
                            </div>
                        }
                        .into_any()
                    } else if workbench_custom_class.get() {
                        view! {
                            <div class="docs-row">
                                <Kbd size=size class_name="docs-kbd-custom".to_string()>
                                    {label}
                                </Kbd>
                            </div>
                        }
                        .into_any()
                    } else if has_keys {
                        let keys_text = keys.unwrap_or_default();
                        view! {
                            <div class="docs-row">
                                <Kbd size=size keys=keys_text>{label}</Kbd>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-row">
                                <Kbd size=size>{label}</Kbd>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Size + Keys + Label-only)"
                code_signal=state_matrix_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string()>"P"</Kbd>
                    <Kbd size=KbdSize::Md keys="Alt".to_string()>"Enter"</Kbd>
                    <Kbd size=KbdSize::Md>"Esc"</Kbd>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Kbd has no internal controlled/uncontrolled axis; compare default static props with app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                        <Kbd size=KbdSize::Sm>"Esc"</Kbd>
                    </div>
                    <p class="ui-muted">
                        "N/A: Kbd is snapshot-only display leaf without `value/on_value_change/default_value` state axis."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Custom Class + Label Only"
                code_signal=custom_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd size=KbdSize::Md class_name="docs-kbd-custom".to_string()>"Esc"</Kbd>
                    <Kbd
                        size=KbdSize::Sm
                        keys="Shift".to_string()
                        class_name="docs-kbd-custom".to_string()
                    >
                        "Tab"
                    </Kbd>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Kbd defaults to snapshot rendering; streaming path is optional and falls back to snapshot semantics."
                code_signal=stream_snapshot_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="kbd-streaming-snapshot">
                    <div class="docs-row">
                        <Kbd keys="Ctrl".to_string()>"K"</Kbd>
                        <Kbd size=KbdSize::Sm>"Esc"</Kbd>
                    </div>
                    <p class="ui-muted" data-slot="kbd-streaming-hint">
                        "Streaming Optional -> fallback=snapshot; keep output-state semantic continuity at upstream layer."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Playground copy action injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=kbd_imports.clone()
            >
                <div class="docs-row">
                    <Kbd
                        size=KbdSize::Sm
                        keys="Shift".to_string()
                        class_name="docs-kbd-custom".to_string()
                    >
                        "Tab"
                    </Kbd>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Size / Keys / Class Comparison)"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Kbd, KbdSize};".to_string()
            >
                <div class="docs-row">
                    <Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
                    <Kbd size=KbdSize::Sm keys="⌘".to_string() class_name="docs-kbd-custom".to_string()>
                        "P"
                    </Kbd>
                    <Kbd size=KbdSize::Md>"Esc"</Kbd>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="kbd-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="kbd-state-rows">
                    <li><code>"data-size / data-state"</code>" = sm|md / with-keys|label-only"</li>
                    <li><code>"data-keys"</code>" = true | none"</li>
                    <li><code>"data-custom-class"</code>" = true | none"</li>
                    <li><code>"control mode"</code>" = N/A (Kbd has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Kbd has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="kbd-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="kbd-parameter-rows">
                    <li><code>"size: Option&lt;KbdSize&gt;"</code>" default = None -> Md (`logic.rs`: `normalize_size -> unwrap_or_default()`)"</li>
                    <li><code>"keys: Option&lt;String&gt;"</code>" default = None; blank string trims to None (`normalize_optional_text`)"</li>
                    <li><code>"class_name: Option&lt;String&gt;"</code>" default = None; blank string trims to None (`normalize_optional_text`)"</li>
                    <li><code>"children: Children"</code>" required; renders label content inside `<span data-slot=\"kbd-label\">`"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="kbd-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    ", and keeps starter code aligned with the current Kbd prop surface."
                </p>
                <ul data-slot="kbd-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-kbd"</code>
                        " for package-mode consumption."
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
                    text="use leptos::prelude::*;\nuse ui::{Kbd, KbdSize};\n\n<Kbd size=KbdSize::Sm keys=\"Shift\".to_string() class_name=\"docs-kbd-custom\".to_string()>\n  \"Tab\"\n</Kbd>".to_string()
                    label="Copy Kbd starter".to_string()
                    copyable=true
                    class_name="docs-kbd-source-copy".to_string()
                />
                <ul data-slot="kbd-source-paths">
                    <li><code>"components/kbd/src/mod.rs"</code></li>
                    <li><code>"components/kbd/src/logic.rs"</code></li>
                    <li><code>"components/kbd/src/view.rs"</code></li>
                    <li><code>"components/kbd/src/styles.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
