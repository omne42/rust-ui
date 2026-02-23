use super::*;

pub(crate) fn progress_bar() -> AnyView {
    let variant_options = [
        "Default".to_string(),
        "Accent".to_string(),
        "Danger".to_string(),
    ];
    let size_options = ["Sm".to_string(), "Md".to_string(), "Lg".to_string()];
    let max_options = ["100".to_string(), "200".to_string()];

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(0_usize));
    let (workbench_indeterminate, set_workbench_indeterminate) = signal(false);
    let (workbench_value_raw, set_workbench_value_raw) = signal(64.0_f64);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ProgressBarVariant::Accent,
            2 => ProgressBarVariant::Danger,
            _ => ProgressBarVariant::Default,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ProgressBarSize::Sm,
        2 => ProgressBarSize::Lg,
        _ => ProgressBarSize::Md,
    });
    let workbench_max = Signal::derive(move || {
        if workbench_max_index.get().unwrap_or(0) == 1 {
            200.0_f64
        } else {
            100.0_f64
        }
    });
    let workbench_value =
        Signal::derive(move || workbench_value_raw.get().clamp(0.0, workbench_max.get()));
    let workbench_aria_label = Signal::derive(move || {
        if workbench_custom_aria.get() {
            "Workbench progress bar".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            "docs-progress-bar-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<ProgressBar
  variant=ProgressBarVariant::Default
  size=ProgressBarSize::Md
  value=42.0
  max=100.0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant_expr = match workbench_variant.get() {
            ProgressBarVariant::Default => "ProgressBarVariant::Default",
            ProgressBarVariant::Accent => "ProgressBarVariant::Accent",
            ProgressBarVariant::Danger => "ProgressBarVariant::Danger",
        };
        let size_expr = match workbench_size.get() {
            ProgressBarSize::Sm => "ProgressBarSize::Sm",
            ProgressBarSize::Md => "ProgressBarSize::Md",
            ProgressBarSize::Lg => "ProgressBarSize::Lg",
        };

        format!(
            "<ProgressBar\n  variant={variant_expr}\n  size={size_expr}\n  value={:.1}\n  max={:.1}\n  indeterminate={}\n  aria_label={}\n  class_name={}\n/>",
            workbench_value.get(),
            workbench_max.get(),
            bool_word(workbench_indeterminate.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ProgressBarActualConfig {{\n  variant: {:?},\n  size: {:?},\n  value: {:.1},\n  max: {:.1},\n  indeterminate: {},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_variant.get(),
            workbench_size.get(),
            workbench_value.get(),
            workbench_max.get(),
            workbench_indeterminate.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Sm value=24.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Accent size=ProgressBarSize::Md value=72.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Danger size=ProgressBarSize::Lg value=54.0 max=100.0 />
<ProgressBar variant=ProgressBarVariant::Default size=ProgressBarSize::Md indeterminate=true />"#
            .to_string()
    });
    let custom_code = Signal::derive(move || {
        r#"<ProgressBar
  variant=ProgressBarVariant::Accent
  size=ProgressBarSize::Md
  value=64.0
  max=f64::NAN
  aria_label="Upload completion".to_string()
  class_name="docs-progress-bar-custom".to_string()
/>
<ProgressBar
  variant=ProgressBarVariant::Default
  size=ProgressBarSize::Sm
  value=18.0
  max=100.0
  aria_label="   ".to_string()
  class_name="docs-progress-bar-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ProgressBar"
            slug="progress-bar"
            group="Display"
            description="Native <progress> element with centralized variant/size/state source attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <ProgressBar
                    variant=ProgressBarVariant::Default
                    size=ProgressBarSize::Md
                    value=42.0
                    max=100.0
                />
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="progress-bar-workbench-controls">
                        <div class="docs-search__label">"variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_variant_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_variant_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {variant_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || workbench_size_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_workbench_size_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {size_options
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

                        <div class="docs-search__label">"value"</div>
                        <input
                            class="docs-search__input"
                            type="range"
                            min="0"
                            max="200"
                            step="1"
                            prop:value=move || format!("{:.0}", workbench_value_raw.get())
                            on:input=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<f64>() {
                                    set_workbench_value_raw.set(value);
                                }
                            }
                        />

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_indeterminate.get()
                                on:change=move |event| set_workbench_indeterminate.set(event_target_checked(&event))
                            />
                            <span>"indeterminate"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |event| set_workbench_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |event| set_workbench_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        value=workbench_value.get()
                        max=workbench_max.get()
                        indeterminate=workbench_indeterminate.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", workbench_value.get())}
                        " · max: " {move || format!("{:.1}", workbench_max.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Variant + Size Matrix"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Sm
                        value=24.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Accent
                        size=ProgressBarSize::Md
                        value=72.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Danger
                        size=ProgressBarSize::Lg
                        value=54.0
                        max=100.0
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Md
                        indeterminate=true
                    />
                </div>
            </Playground>

            <Playground title="Custom Label + Class"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::{ProgressBar, ProgressBarSize, ProgressBarVariant};".to_string()
                test_source_path="components/progress/src/bar/view.rs".to_string()
            >
                <div class="docs-stack">
                    <ProgressBar
                        variant=ProgressBarVariant::Accent
                        size=ProgressBarSize::Md
                        value=64.0
                        max=f64::NAN
                        aria_label="Upload completion".to_string()
                        class_name="docs-progress-bar-custom".to_string()
                    />
                    <ProgressBar
                        variant=ProgressBarVariant::Default
                        size=ProgressBarSize::Sm
                        value=18.0
                        max=100.0
                        aria_label="   ".to_string()
                        class_name="docs-progress-bar-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
