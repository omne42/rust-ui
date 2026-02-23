use super::*;

pub(crate) fn checkbox() -> AnyView {
    let (checked, set_checked) = signal(false);
    let (last_change, set_last_change) = signal("none".to_string());
    let on_accept_change = Callback::new(move |next: bool| {
        set_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });

    let (marketing, set_marketing) = signal(true);
    let (disabled_checked, set_disabled_checked) = signal(true);
    let (disabled_unchecked, set_disabled_unchecked) = signal(false);
    let (interactive_checked, set_interactive_checked) = signal(true);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_aria, set_interactive_custom_aria) = signal(false);
    let (interactive_lang_zh, set_interactive_lang_zh) = signal(false);
    let (interactive_rtl_dir, set_interactive_rtl_dir) = signal(false);
    let (interactive_last_change, set_interactive_last_change) = signal("none".to_string());
    let variant_options = vec!["Default".to_string(), "Accent".to_string()];
    let size_options = vec!["Default".to_string(), "Sm".to_string(), "Lg".to_string()];
    let (interactive_variant_index, set_interactive_variant_index) = signal(Some(0_usize));
    let (interactive_size_index, set_interactive_size_index) = signal(Some(0_usize));
    let interactive_variant =
        Signal::derive(move || match interactive_variant_index.get().unwrap_or(0) {
            1 => CheckboxVariant::Accent,
            _ => CheckboxVariant::Default,
        });
    let interactive_size =
        Signal::derive(move || match interactive_size_index.get().unwrap_or(0) {
            1 => CheckboxSize::Sm,
            2 => CheckboxSize::Lg,
            _ => CheckboxSize::Default,
        });
    let interactive_motion = Signal::derive(move || {
        if interactive_custom_motion.get() {
            ui::CheckboxMotion {
                hover_scale: 1.08,
                tap_scale: 0.92,
                ..ui::CheckboxMotion::default()
            }
        } else {
            ui::CheckboxMotion::default()
        }
    });
    let on_interactive_change = Callback::new(move |next: bool| {
        set_interactive_last_change.set(if next {
            "true".to_string()
        } else {
            "false".to_string()
        });
    });
    let (comparison_controlled, set_comparison_controlled) = signal(false);

    let interactive_code = Signal::derive(move || {
        let mut lines = vec![
            "let (is_checked, on_checked_change) = signal(true);".to_string(),
            "".to_string(),
            "<Checkbox".to_string(),
            "  is_checked=is_checked".to_string(),
            "  on_checked_change=on_checked_change".to_string(),
            "  on_change=Callback::new(move |_| {})".to_string(),
        ];

        if interactive_variant.get() != CheckboxVariant::Default {
            lines.push(format!(
                "  variant=CheckboxVariant::{:?}",
                interactive_variant.get()
            ));
        }
        if interactive_size.get() != CheckboxSize::Default {
            lines.push(format!("  size=CheckboxSize::{:?}", interactive_size.get()));
        }
        if interactive_disabled.get() {
            lines.push("  is_disabled=true".to_string());
        }
        if interactive_custom_class.get() {
            lines.push("  class_name=\"docs-checkbox-custom\".into()".to_string());
        }
        if interactive_custom_motion.get() {
            lines.push(
                "  motion=CheckboxMotion { hover_scale: 1.08, tap_scale: 0.92, ..CheckboxMotion::default() }"
                    .to_string(),
            );
        }
        if interactive_custom_aria.get() {
            lines.push("  aria_label=\"Accept policy\".into()".to_string());
        }
        lines.push(if interactive_lang_zh.get() {
            "  lang=\"zh-CN\".into()".to_string()
        } else {
            "  lang=\"en-US\".into()".to_string()
        });
        lines.push(if interactive_rtl_dir.get() {
            "  dir=Some(A11yDirection::Rtl)".to_string()
        } else {
            "  dir=Some(A11yDirection::Ltr)".to_string()
        });

        lines.push(">".to_string());
        lines.push("  \"Interactive consent\"".to_string());
        lines.push("</Checkbox>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/checkbox/src/styles.rs */\n{}",
            ui::checkbox::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "CheckboxActualConfig {{\n  is_checked: {},\n  checked: {},\n  on_checked_change: \"set_interactive_checked\",\n  set_checked: \"set_comparison_controlled\",\n  default_checked: Some(true),\n  is_disabled: {},\n  disabled: true,\n  on_change: \"set_interactive_last_change\",\n  variant: {:?},\n  size: {:?},\n  motion: {},\n  class_name: {},\n  aria_label: {},\n  lang: {},\n  dir: {},\n}}",
            interactive_checked.get(),
            comparison_controlled.get(),
            interactive_disabled.get(),
            interactive_variant.get(),
            interactive_size.get(),
            if interactive_custom_motion.get() {
                "CheckboxMotion(custom hover/tap)"
            } else {
                "CheckboxMotion::default()"
            },
            if interactive_custom_class.get() {
                "\"docs-checkbox-custom\""
            } else {
                "None"
            },
            if interactive_custom_aria.get() {
                "Some(\"Accept policy\")"
            } else {
                "None"
            },
            if interactive_lang_zh.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if interactive_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let hello_world_code = Signal::derive(|| r#"<Checkbox>"Accept terms"</Checkbox>"#.to_string());

    let code = Signal::derive(move || {
        r#"let (checked, set_checked) = signal(false);

<Checkbox
  is_checked=checked
  on_checked_change=set_checked
  on_change=Callback::new(move |_| {})
>
  "Accept terms"
</Checkbox>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (marketing, set_marketing) = signal(true);
let (disabled_checked, set_disabled_checked) = signal(true);
let (disabled_unchecked, set_disabled_unchecked) = signal(false);

<Checkbox
  is_checked=marketing
  on_checked_change=set_marketing
  variant=CheckboxVariant::Accent
  size=CheckboxSize::Lg
>
  "Email updates"
</Checkbox>
<Checkbox
  is_checked=disabled_checked
  on_checked_change=set_disabled_checked
  is_disabled=true
>
  "Disabled on"
</Checkbox>
<Checkbox
  is_checked=disabled_unchecked
  on_checked_change=set_disabled_unchecked
  is_disabled=true
>
  "Disabled off"
</Checkbox>"#
            .to_string()
    });

    let comparison_code = Signal::derive(move || {
        r#"let (controlled, set_controlled) = signal(false);

<Checkbox
  is_checked=controlled
  on_checked_change=set_controlled
>
  "Controlled"
</Checkbox>
<Checkbox default_checked=Some(true)>
  "Uncontrolled default on"
</Checkbox>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Checkbox"
            slug="checkbox"
            group="Forms"
            description="Pressable checkbox with baseline-level spring indicator and baseline-style root state attrs."
        >
            <Playground
                title="Hello World"
                description="Minimal default path: no state wiring required."
                code_signal=hello_world_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <Checkbox>"Accept terms"</Checkbox>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: edit checkbox props and inspect actual state contracts."
                code_signal=interactive_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
                test_css_source=interactive_test_css
                test_source_path="components/checkbox/src/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-variant".to_string()
                            options=variant_options.clone()
                            selected_index=interactive_variant_index
                            set_selected_index=set_interactive_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Checkbox variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-checkbox-size".to_string()
                            options=size_options.clone()
                            selected_index=interactive_size_index
                            set_selected_index=set_interactive_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Checkbox size".to_string()
                        />

                        <Switch checked=interactive_checked set_checked=set_interactive_checked>
                            "Checked"
                        </Switch>
                        <Switch checked=interactive_disabled set_checked=set_interactive_disabled>
                            "Disabled"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                        <Switch checked=interactive_custom_aria set_checked=set_interactive_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=interactive_lang_zh set_checked=set_interactive_lang_zh>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=interactive_rtl_dir set_checked=set_interactive_rtl_dir>
                            "dir RTL"
                        </Switch>
                    </div>
                }
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="checkbox-e2e-interactive-surface"
                    data-e2e-ready="true"
                >
                    <Checkbox
                        is_checked=interactive_checked
                        on_checked_change=set_interactive_checked
                        variant=interactive_variant.get()
                        size=interactive_size.get()
                        is_disabled=interactive_disabled.get()
                        on_change=on_interactive_change
                        motion=interactive_motion.get()
                        class_name=if interactive_custom_class.get() {
                            "docs-checkbox-custom".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if interactive_custom_aria.get() {
                            "Accept policy".to_string()
                        } else {
                            String::new()
                        }
                        lang=if interactive_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if interactive_rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    >
                        "Interactive consent"
                    </Checkbox>
                    <span class="ui-muted">
                        "checked: " {move || interactive_checked.get()}
                        " · disabled: " {move || interactive_disabled.get()}
                    </span>
                    <span class="ui-muted">
                        "last on_change: "
                        {move || interactive_last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled + on_change"
                code_signal=code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div
                    class="docs-stack"
                    data-slot="checkbox-e2e-controlled-surface"
                    data-e2e-ready="true"
                >
                    <div class="docs-row" data-slot="checkbox-e2e-controlled-row">
                        <div data-slot="checkbox-e2e-controlled-target">
                            <Checkbox
                                is_checked=checked
                                on_checked_change=set_checked
                                on_change=on_accept_change
                            >
                                "Accept terms"
                            </Checkbox>
                        </div>
                        <span class="ui-muted" data-slot="checkbox-e2e-controlled-checked">
                            "checked: " {move || checked.get()}
                        </span>
                    </div>
                    <span class="ui-muted" data-slot="checkbox-e2e-controlled-last-change">
                        "last on_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Variant + Disabled matrix"
                code_signal=states_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div
                    class="docs-stack"
                    data-slot="checkbox-e2e-matrix-surface"
                    data-e2e-ready="true"
                >
                    <div class="docs-row" data-slot="checkbox-e2e-marketing-row">
                        <Checkbox
                            is_checked=marketing
                            on_checked_change=set_marketing
                            variant=CheckboxVariant::Accent
                            size=CheckboxSize::Lg
                        >
                            "Email updates"
                        </Checkbox>
                        <span class="ui-muted">
                            "marketing: "
                            {move || marketing.get()}
                        </span>
                    </div>
                    <div class="docs-row" data-slot="checkbox-e2e-disabled-row">
                        <div data-slot="checkbox-e2e-disabled-on">
                            <Checkbox
                                checked=disabled_checked
                                set_checked=set_disabled_checked
                                disabled=true
                            >
                                "Disabled on"
                            </Checkbox>
                        </div>
                        <div data-slot="checkbox-e2e-disabled-off">
                            <Checkbox
                                is_checked=disabled_unchecked
                                on_checked_change=set_disabled_unchecked
                                is_disabled=true
                            >
                                "Disabled off"
                            </Checkbox>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Comparison)"
                description="受控路径展示外部单一事实来源；非受控路径由 default_checked 初始化后内部管理。"
                code_signal=comparison_code
                code_imports="use leptos::prelude::*;\nuse ui::*;".to_string()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <Checkbox
                            is_checked=comparison_controlled
                            on_checked_change=set_comparison_controlled
                        >
                            "Controlled"
                        </Checkbox>
                        <span class="ui-muted">
                            "controlled: " {move || comparison_controlled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Checkbox checked=comparison_controlled set_checked=set_comparison_controlled>
                            "Checked + set_checked alias"
                        </Checkbox>
                        <span class="ui-muted">
                            "alias-controlled: " {move || comparison_controlled.get()}
                        </span>
                    </div>
                    <div class="docs-row">
                        <Checkbox default_checked=true>"Uncontrolled default on"</Checkbox>
                        <span class="ui-muted">"uncontrolled: internal state (default_checked)"</span>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="checkbox-streaming-policy">
                <h3>"Streaming / Snapshot"</h3>
                <p>
                    "Checkbox is "
                    <strong>"Streaming Optional; fallback=snapshot."</strong>
                </p>
                <p data-slot="checkbox-streaming-modes">
                    "Snapshot mode renders verified full output for checkbox semantics. Streaming labels are exposed via stable markers (`data-ui-stream-support`, `data-ui-stream-fallback`, `data-ui-output-status`)."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="checkbox-source-first">
                <h3>"Source-first / Copy-ready"</h3>
                <p data-slot="checkbox-copy-ready">
                    "Each playground supports code + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " and include "
                    <code>"use leptos::prelude::*; use ui::*;"</code>
                    "."
                </p>
                <ul data-slot="checkbox-source-paths">
                    <li><code>"components/checkbox/src/view.rs"</code></li>
                    <li><code>"components/checkbox/src/logic.rs"</code></li>
                    <li><code>"components/checkbox/src/styles.rs"</code></li>
                    <li><code>"apps/docs-app/src/pages/components/pages/forms.rs"</code></li>
                </ul>
                <ul data-slot="checkbox-source-prerequisites">
                    <li>
                        <code>"ui"</code>
                        " with feature "
                        <code>"component-checkbox"</code>
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
