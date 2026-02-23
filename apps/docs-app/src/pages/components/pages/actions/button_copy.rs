use super::*;

pub(crate) fn button_copy() -> AnyView {
    let persisted_workbench_state = load_button_copy_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or(ButtonCopyWorkbenchState {
        mode_index: 2,
        variant_index: 0,
        size_index: 2,
        text_index: 0,
        feedback_scale: 8,
        feedback_glow: 100,
        is_disabled: false,
    });

    let mode_options = vec![
        "text-only".to_string(),
        "icon-only".to_string(),
        "icon+text".to_string(),
    ];
    let variant_options = vec![
        "secondary".to_string(),
        "outline".to_string(),
        "accent".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let text_options = vec![
        "cargo command".to_string(),
        "docs url".to_string(),
        "token".to_string(),
    ];

    let (mode_index, set_mode_index) = signal(Some(initial_workbench_state.mode_index));
    let mode = Signal::derive(move || match mode_index.get().unwrap_or(2) {
        0 => ButtonCopyMode::TextOnly,
        1 => ButtonCopyMode::IconOnly,
        _ => ButtonCopyMode::IconAndText,
    });

    let (variant_index, set_variant_index) = signal(Some(initial_workbench_state.variant_index));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Outline,
        2 => ButtonVariant::Accent,
        _ => ButtonVariant::Secondary,
    });

    let (size_index, set_size_index) = signal(Some(initial_workbench_state.size_index));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (text_index, set_text_index) = signal(Some(initial_workbench_state.text_index));
    let text = Signal::derive(move || match text_index.get().unwrap_or(0) {
        1 => "https://example.com/docs".to_string(),
        2 => "token=sk-demo-123".to_string(),
        _ => "cargo add ui".to_string(),
    });

    let (feedback_scale, set_feedback_scale) = signal(initial_workbench_state.feedback_scale);
    let (feedback_glow, set_feedback_glow) = signal(initial_workbench_state.feedback_glow);
    let (is_disabled, set_is_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_button_copy_workbench_state(ButtonCopyWorkbenchState {
                mode_index: mode_index.get().unwrap_or(2),
                variant_index: variant_index.get().unwrap_or(0),
                size_index: size_index.get().unwrap_or(0),
                text_index: text_index.get().unwrap_or(0),
                feedback_scale: feedback_scale.get(),
                feedback_glow: feedback_glow.get(),
                is_disabled: is_disabled.get(),
            });
        } else {
            clear_button_copy_workbench_state();
        }
    });

    let workbench_motion = Signal::derive(move || ButtonCopyMotion {
        copied_feedback_scale: f64::from(feedback_scale.get()) / 100.0,
        copied_feedback_glow: f64::from(feedback_glow.get()) / 100.0,
        ..ButtonCopyMotion::default()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/copy/styles.rs */\n{}",
            ui::button::copy::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let mode = mode.get();
        let variant = variant.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let copied_feedback_scale = f64::from(feedback_scale.get()) / 100.0;
        let copied_feedback_glow = f64::from(feedback_glow.get()) / 100.0;
        let text = text.get();

        format!(
            "ButtonCopyWorkbenchConfig {{\n  text: \"{text}\",\n  label: \"Copy value\",\n  copied_label: \"Copied!\",\n  aria_label: {:?},\n  is_disabled: {is_disabled},\n  mode: {mode:?},\n  variant: {variant:?},\n  size: {size:?},\n  motion: ButtonCopyMotion {{ copied_feedback_scale: {copied_feedback_scale:.2}, copied_feedback_glow: {copied_feedback_glow:.2}, ..Default::default() }},\n  class_name: {:?},\n  lang: {:?},\n  dir: {},\n  copied_feedback_scale: {copied_feedback_scale:.2},\n  copied_feedback_glow: {copied_feedback_glow:.2},\n}}",
            if workbench_custom_aria.get() {
                Some("Copy selected text")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-button-copy-custom")
            } else {
                None
            },
            if workbench_rtl.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if workbench_rtl.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
        )
    });

    let hello_world_code =
        Signal::derive(move || r#"<ButtonCopy text="cargo add ui".to_string() />"#.to_string());

    let code = Signal::derive(move || {
        r#"<ButtonCopy
  text="cargo add ui".to_string()
  label="Copy install command".to_string()
  copied_label="Copied!".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ButtonCopy text="https://example.com/docs".to_string() variant=ButtonVariant::Outline />
<ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
<ButtonCopy text="token".to_string() is_disabled=true />"#
            .to_string()
    });

    let modes_code = Signal::derive(move || {
        r#"<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::TextOnly />
<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::IconOnly />
<ButtonCopy text="cargo add ui".to_string() mode=ButtonCopyMode::IconAndText />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let mode = mode.get();
        let variant = variant.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let text = text.get();
        let copied_feedback_scale = f64::from(feedback_scale.get()) / 100.0;
        let copied_feedback_glow = f64::from(feedback_glow.get()) / 100.0;

        format!(
            "<ButtonCopy\n  text=\"{text}\".into()\n  mode=ButtonCopyMode::{mode:?}\n  variant=ButtonVariant::{variant:?}\n  size=ButtonSize::{size:?}\n  is_disabled={is_disabled}\n  motion=ButtonCopyMotion {{\n    copied_feedback_scale: {copied_feedback_scale:.2},\n    copied_feedback_glow: {copied_feedback_glow:.2},\n    ..ButtonCopyMotion::default()\n  }}\n/>"
        )
    });

    view! {
        <ComponentPage
            title="ButtonCopy"
            slug="button-copy"
            group="Actions"
            description="Copy-to-clipboard button with baseline-style disabled/empty semantics and live copied announcements."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="docs-row">
                    <ButtonCopy text="cargo add ui".to_string() />
                </div>
                <span class="ui-muted">"Start simple, then move to advanced controls."</span>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/copy/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="button-copy-workbench-controls">
                        <div class="docs-search__label">"Mode"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-mode".to_string()
                            options=mode_options.clone()
                            selected_index=mode_index
                            set_selected_index=set_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy mode".to_string()
                        />

                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy size".to_string()
                        />

                        <div class="docs-search__label">"Text preset"</div>
                        <SegmentedControl
                            id_base="docs-button-copy-text".to_string()
                            options=text_options.clone()
                            selected_index=text_index
                            set_selected_index=set_text_index
                            size=SegmentedControlSize::Sm
                            aria_label="ButtonCopy text preset".to_string()
                        />

                        <label class="docs-search__label" for="docs-button-copy-feedback-scale">
                            "Feedback scale (" {move || format!("{:.2}", f64::from(feedback_scale.get()) / 100.0)} ")"
                        </label>
                        <input
                            id="docs-button-copy-feedback-scale"
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="25"
                            step="1"
                            prop:value=move || feedback_scale.get().to_string()
                            on:input=move |ev| {
                                let next = event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or(8)
                                    .clamp(0, 25);
                                set_feedback_scale.set(next);
                            }
                        />

                        <label class="docs-search__label" for="docs-button-copy-feedback-glow">
                            "Feedback glow (" {move || format!("{:.2}", f64::from(feedback_glow.get()) / 100.0)} ")"
                        </label>
                        <input
                            id="docs-button-copy-feedback-glow"
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="5"
                            prop:value=move || feedback_glow.get().to_string()
                            on:input=move |ev| {
                                let next = event_target_value(&ev)
                                    .parse::<u16>()
                                    .unwrap_or(100)
                                    .clamp(0, 200);
                                set_feedback_glow.set(next);
                            }
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>"RTL + ar"</Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let mode = mode.get();
                    let variant = variant.get();
                    let size = size.get();
                    let text = text.get();
                    let is_disabled = is_disabled.get();
                    let persist = workbench_persist_state.get();
                    let motion = workbench_motion.get();

                    view! {
                        <div class="docs-stack" data-slot="button-copy-workbench" style="width: min(100%, 420px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>

                            <div class="docs-card docs-stack docs-stack--tight" data-slot="button-copy-workbench-canvas">
                                <ButtonCopy
                                    text=text.clone()
                                    mode=mode
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    motion=motion
                                    label="Copy value".to_string()
                                    copied_label="Copied!".to_string()
                                    aria_label=if workbench_custom_aria.get() {
                                        "Copy selected text".to_string()
                                    } else {
                                        String::new()
                                    }
                                    class_name=if workbench_custom_class.get() {
                                        "docs-button-copy-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    lang=if workbench_rtl.get() {
                                        "ar".to_string()
                                    } else {
                                        "en-US".to_string()
                                    }
                                    dir=if workbench_rtl.get() {
                                        A11yDirection::Rtl
                                    } else {
                                        A11yDirection::Ltr
                                    }
                                />
                                <span class="ui-muted">"text: " {text}</span>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Mode + Disabled Comparison)"
                code_signal=modes_code
            >
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::TextOnly
                        motion=ButtonCopyMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconAndText
                        is_disabled=true
                        class_name="docs-button-copy-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground title="Label + variant" code_signal=code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        label="Copy install command".to_string()
                        copied_label="Copied!".to_string()
                    />
                    <ButtonCopy
                        text="https://github.com/openai".to_string()
                        variant=ButtonVariant::Outline
                        label="Copy URL".to_string()
                        copied_label="URL copied".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Disabled + empty matrix" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ButtonCopy
                            text="https://example.com/docs".to_string()
                            variant=ButtonVariant::Outline
                        />
                        <ButtonCopy text="   ".to_string() label="Nothing to copy".to_string() />
                        <ButtonCopy text="token".to_string() is_disabled=true />
                    </div>
                    <span class="ui-muted">
                        "Blank text and explicit disabled state both force non-copyable semantics."
                    </span>
                </div>
            </Playground>

            <Playground title="Mode matrix" code_signal=modes_code>
                <div class="docs-row">
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::TextOnly
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconOnly
                    />
                    <ButtonCopy
                        text="cargo add ui".to_string()
                        mode=ButtonCopyMode::IconAndText
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
