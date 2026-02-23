use super::*;

pub(crate) fn pressable_feedback() -> AnyView {
    let effect_options = [
        "Scale".to_string(),
        "Highlight".to_string(),
        "Ripple".to_string(),
        "HighlightRipple".to_string(),
    ];
    let tone_options = [
        "Default".to_string(),
        "Neutral".to_string(),
        "Accent".to_string(),
    ];

    let (effect_index, set_effect_index) = signal(Some(0_usize));
    let (tone_index, set_tone_index) = signal(Some(2_usize));
    let (is_bounded, set_is_bounded) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (enable_on_press, set_enable_on_press) = signal(true);

    let (press_count, set_press_count) = signal(0_u32);
    let (last_press_feedback, set_last_press_feedback) = signal("none".to_string());

    let workbench_effect = Signal::derive(move || match effect_index.get().unwrap_or(0) {
        1 => PressableFeedbackEffect::Highlight,
        2 => PressableFeedbackEffect::Ripple,
        3 => PressableFeedbackEffect::HighlightRipple,
        _ => PressableFeedbackEffect::Scale,
    });
    let workbench_tone = Signal::derive(move || match tone_index.get().unwrap_or(2) {
        0 => PressableFeedbackTone::Default,
        1 => PressableFeedbackTone::Neutral,
        _ => PressableFeedbackTone::Accent,
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            PressableFeedbackMotion {
                pressed_scale: 0.94,
                highlight_opacity: 0.2,
                ripple: RippleMotion {
                    duration_ms: 720,
                    ..RippleMotion::default()
                },
                ..PressableFeedbackMotion::default()
            }
        } else {
            PressableFeedbackMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Workbench pressable surface".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-pressable-feedback-custom".to_string()
        } else {
            String::new()
        }
    });

    let on_workbench_press = Callback::new(move |_| {
        if !enable_on_press.get_untracked() {
            return;
        }
        set_press_count.update(|count| *count += 1);
        set_last_press_feedback.set(format!("pressed #{}", press_count.get_untracked() + 1));
    });

    let showcase_code = Signal::derive(move || {
        r#"<PressableFeedback>
  <div class="docs-ripple-surface">"Hello feedback"</div>
</PressableFeedback>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let effect_variant = match workbench_effect.get() {
            PressableFeedbackEffect::Scale => "PressableFeedbackEffect::Scale",
            PressableFeedbackEffect::Highlight => "PressableFeedbackEffect::Highlight",
            PressableFeedbackEffect::Ripple => "PressableFeedbackEffect::Ripple",
            PressableFeedbackEffect::HighlightRipple => "PressableFeedbackEffect::HighlightRipple",
        };
        let tone_variant = match workbench_tone.get() {
            PressableFeedbackTone::Default => "PressableFeedbackTone::Default",
            PressableFeedbackTone::Neutral => "PressableFeedbackTone::Neutral",
            PressableFeedbackTone::Accent => "PressableFeedbackTone::Accent",
        };
        let motion_expr = if custom_motion.get() {
            "PressableFeedbackMotion { pressed_scale: 0.94, highlight_opacity: 0.2, ripple: RippleMotion { duration_ms: 720, ..RippleMotion::default() }, ..PressableFeedbackMotion::default() }"
        } else {
            "PressableFeedbackMotion::default()"
        };
        let on_press_expr = if enable_on_press.get() {
            "Some(on_workbench_press)"
        } else {
            "None"
        };

        format!(
            "<PressableFeedback\n  effect={effect_variant}\n  tone={tone_variant}\n  is_bounded={}\n  is_disabled={}\n  motion={motion_expr}\n  aria_label={}\n  class_name={}\n  on_press={on_press_expr}\n>\n  <div class=\"docs-ripple-surface\">\"Interactive surface\"</div>\n</PressableFeedback>",
            bool_word(is_bounded.get()),
            bool_word(is_disabled.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "PressableFeedbackActualConfig {{\n  effect: {:?},\n  tone: {:?},\n  is_bounded: {},\n  is_disabled: {},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  on_press: {},\n}}",
            workbench_effect.get(),
            workbench_tone.get(),
            is_bounded.get(),
            is_disabled.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            if enable_on_press.get() {
                "Some"
            } else {
                "None"
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<PressableFeedback effect=PressableFeedbackEffect::Scale tone=PressableFeedbackTone::Default>
  <div class="docs-ripple-surface docs-ripple-surface--static">"Scale"</div>
</PressableFeedback>
<PressableFeedback effect=PressableFeedbackEffect::Highlight tone=PressableFeedbackTone::Accent is_bounded=true>
  <div class="docs-ripple-surface">"Highlight"</div>
</PressableFeedback>
<PressableFeedback
  effect=PressableFeedbackEffect::HighlightRipple
  tone=PressableFeedbackTone::Neutral
  is_bounded=false
  is_disabled=true
  motion=PressableFeedbackMotion { pressed_scale: 0.94, highlight_opacity: 0.2, ripple: RippleMotion { duration_ms: 720, ..RippleMotion::default() }, ..PressableFeedbackMotion::default() }
  class_name="docs-pressable-feedback-custom".to_string()
>
  <div class="docs-ripple-surface docs-ripple-surface--accent">"Disabled custom"</div>
</PressableFeedback>"#
            .to_string()
    });
    let visual_baseline_code = Signal::derive(move || {
        r#"<div data-visual-baseline="pressable-feedback-default-theme">
  <PressableFeedback tone=PressableFeedbackTone::Default aria_label="Primary action".to_string()>
    <div class="docs-ripple-surface">"Primary Surface"</div>
  </PressableFeedback>
  <PressableFeedback
    effect=PressableFeedbackEffect::Highlight
    tone=PressableFeedbackTone::Accent
    aria_label="Accent action".to_string()
  >
    <div class="docs-ripple-surface docs-ripple-surface--accent">"Accent Surface"</div>
  </PressableFeedback>
  <PressableFeedback is_disabled=true aria_label="Disabled action".to_string()>
    <div class="docs-ripple-surface docs-ripple-surface--static">"Disabled Surface"</div>
  </PressableFeedback>
</div>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="PressableFeedback"
            slug="pressable-feedback"
            group="Display"
            description="baseline-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{PressableFeedback};".to_string()
                test_source_path="components/pressable-feedback/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback>
                        <div class="docs-ripple-surface">
                            "Hello feedback"
                        </div>
                    </PressableFeedback>
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{PressableFeedback, PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackTone, RippleMotion};".to_string()
                test_source_path="components/pressable-feedback/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="pressable-feedback-workbench-controls">
                        <div class="docs-search__label">"Effect"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || effect_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_effect_index.set(Some(value.min(3)));
                                }
                            }
                        >
                            {effect_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"Tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().unwrap_or(2).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {tone_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_bounded.get()
                                on:change=move |event| set_is_bounded.set(event_target_checked(&event))
                            />
                            <span>"is_bounded"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_disabled.get()
                                on:change=move |event| set_is_disabled.set(event_target_checked(&event))
                            />
                            <span>"is_disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || enable_on_press.get()
                                on:change=move |event| set_enable_on_press.set(event_target_checked(&event))
                            />
                            <span>"enable on_press callback"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=workbench_effect.get()
                        tone=workbench_tone.get()
                        is_bounded=is_bounded.get()
                        is_disabled=is_disabled.get()
                        motion=workbench_motion.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        on_press=on_workbench_press
                    >
                        <div class="docs-ripple-surface">
                            "Interactive surface"
                        </div>
                    </PressableFeedback>
                    <span class="ui-muted">
                        "press_count: " {move || press_count.get()}
                        " · last_event: " {move || last_press_feedback.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Effect / Tone / Disabled Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{PressableFeedback, PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackTone, RippleMotion};".to_string()
                test_source_path="components/pressable-feedback/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback effect=PressableFeedbackEffect::Scale tone=PressableFeedbackTone::Default>
                        <div class="docs-ripple-surface docs-ripple-surface--static">
                            "Scale"
                        </div>
                    </PressableFeedback>
                    <PressableFeedback
                        effect=PressableFeedbackEffect::Highlight
                        tone=PressableFeedbackTone::Accent
                        is_bounded=true
                    >
                        <div class="docs-ripple-surface">
                            "Highlight"
                        </div>
                    </PressableFeedback>
                    <PressableFeedback
                        effect=PressableFeedbackEffect::HighlightRipple
                        tone=PressableFeedbackTone::Neutral
                        is_bounded=false
                        is_disabled=true
                        motion=PressableFeedbackMotion {
                            pressed_scale: 0.94,
                            highlight_opacity: 0.2,
                            ripple: RippleMotion {
                                duration_ms: 720,
                                ..RippleMotion::default()
                            },
                            ..PressableFeedbackMotion::default()
                        }
                        class_name="docs-pressable-feedback-custom".to_string()
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--accent">
                            "Disabled custom"
                        </div>
                    </PressableFeedback>
                </div>
            </Playground>

            <Playground
                title="Default Theme Visual Baseline (Visual Desire)"
                description="First-impression baseline for hierarchy, contrast layers, and hover/active/focus cues. Use this section as the screenshot regression anchor."
                code_signal=visual_baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-visual-baseline="pressable-feedback-default-theme"
                >
                    <span class="ui-muted">
                        "HeroUI-quality visual direction baseline for PressableFeedback under default theme."
                    </span>
                    <span class="ui-muted" data-slot="pressable-feedback-visual-baseline-screenshot">
                        "Screenshot baseline anchor: compare hover/active/focus feedback and disabled contrast."
                    </span>
                    <div class="docs-stack docs-stack--tight">
                        <PressableFeedback
                            tone=PressableFeedbackTone::Default
                            aria_label="Primary action".to_string()
                        >
                            <div class="docs-ripple-surface">
                                "Primary Surface"
                            </div>
                        </PressableFeedback>
                        <PressableFeedback
                            effect=PressableFeedbackEffect::Highlight
                            tone=PressableFeedbackTone::Accent
                            aria_label="Accent action".to_string()
                        >
                            <div class="docs-ripple-surface docs-ripple-surface--accent">
                                "Accent Surface"
                            </div>
                        </PressableFeedback>
                        <PressableFeedback is_disabled=true aria_label="Disabled action".to_string()>
                            <div class="docs-ripple-surface docs-ripple-surface--static">
                                "Disabled Surface"
                            </div>
                        </PressableFeedback>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
