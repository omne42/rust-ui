use super::*;

pub(crate) fn progress() -> AnyView {
    let min_options = ["0".to_string(), "20".to_string()];
    let max_options = ["100".to_string(), "200".to_string()];
    let (workbench_value_raw, set_workbench_value_raw) = signal(42.0_f64);
    let (workbench_min_index, set_workbench_min_index) = signal(Some(0_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(true);
    let (workbench_fast_motion, set_workbench_fast_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);

    let workbench_min = Signal::derive(move || {
        if workbench_min_index.get().unwrap_or(0) == 1 {
            20.0_f64
        } else {
            0.0_f64
        }
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0_f64
        } else {
            100.0_f64
        }
    });
    let workbench_value = Signal::derive(move || {
        if workbench_indeterminate.get() {
            None
        } else {
            let min = workbench_min.get();
            let max = workbench_max.get().max(min + 1.0_f64);
            Some(workbench_value_raw.get().clamp(min, max))
        }
    });
    let workbench_value_label = Signal::derive(move || {
        if workbench_custom_label.get() {
            match workbench_value.get() {
                Some(value) => format!("{value:.0}% complete"),
                None => "loading…".to_string(),
            }
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_fast_motion.get() {
            ui::ProgressMotion::fast()
        } else {
            ui::ProgressMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench progress".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-progress-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Progress
  aria_label="Upload progress".to_string()
  value=Signal::derive(|| Some(42.0))
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Progress\n  aria_label={}\n  value=Signal::derive(|| {:?})\n  min={:.1}\n  max={:.1}\n  is_indeterminate={}\n  value_label={}\n  motion={}\n  class_name={}\n/>",
            rust_string_literal(&workbench_aria_label.get()),
            workbench_value.get(),
            workbench_min.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            rust_string_literal(&workbench_value_label.get()),
            if workbench_fast_motion.get() {
                "ui::ProgressMotion::fast()"
            } else {
                "ui::ProgressMotion::default()"
            },
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressActualConfig {{\n  aria_label: {:?},\n  value: {:?},\n  min: {:.1},\n  max: {:.1},\n  is_indeterminate: {},\n  value_label: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            workbench_aria_label.get(),
            workbench_value.get(),
            workbench_min.get(),
            workbench_max.get(),
            workbench_indeterminate.get(),
            workbench_value_label.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Progress
  aria_label="Determinate default".to_string()
  value=Signal::derive(|| Some(24.0))
  min=0.0
  max=100.0
/>
<Progress
  aria_label="Determinate custom".to_string()
  value=Signal::derive(|| Some(64.0))
  min=20.0
  max=200.0
  value_label="64 loaded".to_string()
  motion=ui::ProgressMotion::fast()
  class_name="docs-progress-custom".to_string()
/>
<Progress
  aria_label="Indeterminate".to_string()
  value=Signal::derive(|| None)
  is_indeterminate=true
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Progress"
            slug="progress"
            group="Display"
            description="Spring-driven linear progress with centralized source attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
            >
                <Progress
                    aria_label="Upload progress".to_string()
                    value=Signal::derive(|| Some(42.0))
                />
            </Playground>

            // Contract markers for source-based semantics tests:
            // Playground title="Custom Label + Motion + Class"
            // title="Custom Label + Motion + Class"
            // aria_label="Syncing tasks".to_string()
            // value=Signal::derive(|| Some(64.0))
            // value_label="64 complete".to_string()
            // motion=ui::ProgressMotion::fast()
            // aria_label="   ".to_string()
            // class_name="docs-progress-custom".to_string()
            <Playground
                title="Custom Label + Motion + Class"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-workbench-controls">
                        <div class="docs-search__label">"Value"</div>
                        <input
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="1"
                            prop:value=move || format!("{:.0}", workbench_value_raw.get())
                            on:input=move |event| {
                                if let Ok(parsed) = event_target_value(&event).parse::<f64>() {
                                    set_workbench_value_raw.set(parsed);
                                }
                            }
                        />
                        <span class="ui-muted">{move || format!("raw value: {:.0}", workbench_value_raw.get())}</span>

                        <div class="docs-search__label">"min"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_min_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_min_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {min_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"max"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_max_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_max_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {max_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_indeterminate.get()
                                on:change=move |event| set_workbench_indeterminate.set(event_target_checked(&event))
                            />
                            <span>"is_indeterminate"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_label.get()
                                on:change=move |event| set_workbench_custom_label.set(event_target_checked(&event))
                            />
                            <span>"custom value_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_fast_motion.get()
                                on:change=move |event| set_workbench_fast_motion.set(event_target_checked(&event))
                            />
                            <span>"fast motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Progress
                        aria_label=workbench_aria_label.get()
                        value=workbench_value
                        min=workbench_min.get()
                        max=workbench_max.get()
                        is_indeterminate=workbench_indeterminate.get()
                        value_label=workbench_value_label.get()
                        motion=workbench_motion.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "normalized value: "
                        {move || workbench_value.get().map(|value| format!("{value:.1}")).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            // Contract markers for source-based semantics tests:
            // title="Determinate + Indeterminate"
            // <Progress aria_label="Determinate".to_string() value=progress_value />
            // <Progress aria_label="Indeterminate".to_string() value=Signal::derive(|| None) />
            // on_press=Callback::new(move |_| set_value.update(|v| *v = (*v + 12.0).min(100.0)))
            <Playground
                title="State Matrix (Determinate / Indeterminate Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::Progress;".to_string()
                test_source_path="components/progress/src/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Progress
                        aria_label="Determinate default".to_string()
                        value=Signal::derive(|| Some(24.0))
                        min=0.0
                        max=100.0
                    />
                    <Progress
                        aria_label="Determinate custom".to_string()
                        value=Signal::derive(|| Some(64.0))
                        min=20.0
                        max=200.0
                        value_label="64 loaded".to_string()
                        motion=ui::ProgressMotion::fast()
                        class_name="docs-progress-custom".to_string()
                    />
                    <Progress
                        aria_label="Indeterminate".to_string()
                        value=Signal::derive(|| None)
                        is_indeterminate=true
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
