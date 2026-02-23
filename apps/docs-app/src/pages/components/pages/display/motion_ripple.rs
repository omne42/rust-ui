use super::*;

pub(crate) fn motion_ripple() -> AnyView {
    let showcase_ref: NodeRef<html::Span> = NodeRef::new();
    let workbench_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_default_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_unbounded_ref: NodeRef<html::Span> = NodeRef::new();
    let matrix_custom_ref: NodeRef<html::Span> = NodeRef::new();

    let (workbench_is_bounded, set_workbench_is_bounded) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);
    let (workbench_trigger_count, set_workbench_trigger_count) = signal(0_u32);

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            RippleMotion {
                duration_ms: 620,
                ..RippleMotion::default()
            }
        } else {
            RippleMotion::default()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-ripple-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<button class="docs-ripple-surface" type="button">
  <MotionRipple node_ref=showcase_ref />
</button>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<MotionRipple\n  node_ref=workbench_ref\n  is_bounded={}\n  motion=RippleMotion {{ duration_ms: {}, ..RippleMotion::default() }}\n  class_name={}\n  lang={}\n  dir={}\n/>",
            bool_word(workbench_is_bounded.get()),
            workbench_motion.get().duration_ms,
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "MotionRippleActualConfig {{\n  node_ref: \"workbench_ref\",\n  is_bounded: {},\n  motion: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  trigger_count: {},\n}}",
            workbench_is_bounded.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
            workbench_trigger_count.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MotionRipple
  node_ref=matrix_default_ref
  is_bounded=true
  motion=RippleMotion::default()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
/>
<MotionRipple
  node_ref=matrix_unbounded_ref
  is_bounded=false
  motion=RippleMotion { duration_ms: 520, ..RippleMotion::default() }
  class_name="docs-ripple-custom".to_string()
  lang="ar".to_string()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="MotionRipple"
            slug="motion-ripple"
            group="Display"
            description="Ripple overlay with centralized boundary/motion/class source attrs and WAAPI trigger helpers."
        >
            <Playground title="Default Showcase" code_signal=showcase_code>
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(showcase_ref, RippleMotion::default());
                        }
                    >
                        <span class="docs-ripple-label">"Click to trigger ripple"</span>
                        <MotionRipple node_ref=showcase_ref />
                    </button>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="motion-ripple-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_bounded.get()
                                on:change=move |ev| set_workbench_is_bounded.set(event_target_checked(&ev))
                            />
                            " is_bounded"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " custom motion"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface docs-ripple-surface--accent"
                        type="button"
                        on:click=move |_| {
                            set_workbench_trigger_count.update(|count| *count += 1);
                            if workbench_is_bounded.get_untracked() {
                                ui::ripple::trigger_ripple(workbench_ref, workbench_motion.get_untracked());
                            } else {
                                ui::ripple::trigger_ripple_at(
                                    workbench_ref,
                                    workbench_motion.get_untracked(),
                                    22.0,
                                    42.0,
                                );
                            }
                        }
                    >
                        <span class="docs-ripple-label">"Trigger workbench ripple"</span>
                        <MotionRipple
                            node_ref=workbench_ref
                            is_bounded=workbench_is_bounded.get()
                            motion=workbench_motion.get()
                            class_name=workbench_class_name.get()
                            lang=workbench_lang.get()
                            dir=workbench_dir.get()
                        />
                    </button>
                    <span class="ui-muted">
                        {move || format!("trigger_count={}", workbench_trigger_count.get())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Boundary / Motion Comparison)"
                code_signal=matrix_code
                code_imports="use ui::color::area::A11yDirection;\nuse ui::{MotionRipple, RippleMotion};".to_string()
            >
                <div class="docs-row">
                    <button
                        class="docs-ripple-surface"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(matrix_default_ref, RippleMotion::default());
                        }
                    >
                        <span class="docs-ripple-label">"Bounded default"</span>
                        <MotionRipple
                            node_ref=matrix_default_ref
                            is_bounded=true
                            motion=RippleMotion::default()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </button>
                    <button
                        class="docs-ripple-surface docs-ripple-surface--unbounded"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple_at(
                                matrix_unbounded_ref,
                                RippleMotion {
                                    duration_ms: 520,
                                    ..RippleMotion::default()
                                },
                                16.0,
                                50.0,
                            );
                        }
                    >
                        <span class="docs-ripple-label">"Unbounded RTL"</span>
                        <MotionRipple
                            node_ref=matrix_unbounded_ref
                            is_bounded=false
                            motion=RippleMotion {
                                duration_ms: 520,
                                ..RippleMotion::default()
                            }
                            class_name="docs-ripple-custom".to_string()
                            lang="ar".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </button>
                    <button
                        class="docs-ripple-surface docs-ripple-surface--slow"
                        type="button"
                        on:click=move |_| {
                            ui::ripple::trigger_ripple(
                                matrix_custom_ref,
                                RippleMotion {
                                    duration_ms: 880,
                                    ..RippleMotion::default()
                                },
                            );
                        }
                    >
                        <span class="docs-ripple-label">"Slow bounded"</span>
                        <MotionRipple
                            node_ref=matrix_custom_ref
                            is_bounded=true
                            motion=RippleMotion {
                                duration_ms: 880,
                                ..RippleMotion::default()
                            }
                            class_name="docs-ripple-item".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
