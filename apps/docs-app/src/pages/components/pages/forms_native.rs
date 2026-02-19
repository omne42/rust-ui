use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{NativeSelect, NativeSelectOption, NativeSelectSize};

pub(super) fn native_select() -> AnyView {
    let options = vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
        NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
    ];
    let (selected_raw, set_selected_raw) = signal(None::<usize>);
    let selected_signal: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());
    let on_selected_change = Callback::new(move |next: Option<usize>| set_selected_raw.set(next));

    let required_options = vec![
        NativeSelectOption::new("staging", "Staging"),
        NativeSelectOption::new("production", "Production"),
        NativeSelectOption::new("canary", "Canary"),
    ];

    let disabled_options = vec![
        NativeSelectOption::new("legacy", "Legacy").disabled(true),
        NativeSelectOption::new("frozen", "Frozen").disabled(true),
    ];

    let code = Signal::derive(move || {
        r#"let (selected_raw, set_selected_raw) = signal(None::<usize>);
let selected: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());

<NativeSelect
  id_base="mode".to_string()
  options=vec![
    NativeSelectOption::new("system", "System"),
    NativeSelectOption::new("manual", "Manual"),
    NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
  ]
  selected_index=selected
  on_selected_index_change=Callback::new(move |next| set_selected_raw.set(next))
  placeholder="Choose mode".to_string()
  name="mode".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<NativeSelect
  id_base="deployment".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  default_selected_index=1
  required=true
  invalid=true
  size=NativeSelectSize::Lg
  aria_label="Deployment strategy".to_string()
/>
<NativeSelect
  id_base="disabled".to_string()
  options=vec![
    NativeSelectOption::new("legacy", "Legacy").disabled(true),
    NativeSelectOption::new("frozen", "Frozen").disabled(true),
  ]
  disabled=true
  placeholder="Disabled select".to_string()
  size=NativeSelectSize::Sm
/>"#
        .to_string()
    });
    let workbench_options = StoredValue::new(vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
        NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
    ]);
    let workbench_size_options = vec!["Sm".to_string(), "Md".to_string(), "Lg".to_string()];
    let workbench_selected_options = vec![
        "None".to_string(),
        "System".to_string(),
        "Manual".to_string(),
    ];
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_selected_mode_index, set_workbench_selected_mode_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_placeholder, set_workbench_placeholder) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_show_compare, set_workbench_show_compare) = signal(true);

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => NativeSelectSize::Sm,
        2 => NativeSelectSize::Lg,
        _ => NativeSelectSize::Md,
    });
    let workbench_selected_index =
        Signal::derive(
            move || match workbench_selected_mode_index.get().unwrap_or(0) {
                1 => Some(0),
                2 => Some(1),
                _ => None,
            },
        );
    let on_workbench_selected_index_change = Callback::new(move |next: Option<usize>| {
        let mapped = match next {
            Some(0) => Some(1),
            Some(1) => Some(2),
            _ => Some(0),
        };
        set_workbench_selected_mode_index.set(mapped);
    });
    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let placeholder = workbench_placeholder.get();
        let custom_class = workbench_custom_class.get();
        let selected_mode = workbench_selected_mode_index.get().unwrap_or(0);
        let selected_line = match selected_mode {
            1 => "selected_index=Signal::derive(|| Some(0usize))".to_string(),
            2 => "selected_index=Signal::derive(|| Some(1usize))".to_string(),
            _ => "selected_index=Signal::derive(|| None::<usize>)".to_string(),
        };

        let mut lines = vec![
            "<NativeSelect".to_string(),
            "  id_base=\"docs-native-select-workbench\".into()".to_string(),
            "  options=vec![".to_string(),
            "    NativeSelectOption::new(\"system\", \"System\"),".to_string(),
            "    NativeSelectOption::new(\"manual\", \"Manual\"),".to_string(),
            "    NativeSelectOption::new(\"hybrid\", \"Hybrid\").disabled(true),".to_string(),
            "  ]".to_string(),
            format!("  {selected_line}"),
            format!("  required={required}"),
            format!("  invalid={invalid}"),
            format!("  disabled={disabled}"),
            format!("  size=NativeSelectSize::{size:?}"),
        ];
        if placeholder {
            lines.push("  placeholder=\"Choose mode\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-native-select-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/native_select/styles.rs */\n{}",
            ui_components::native_select::styles::CSS
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let selected_index = workbench_selected_index.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let placeholder = workbench_placeholder.get();
        let custom_class = workbench_custom_class.get();
        format!(
            "NativeSelectActualConfig {{\n  size: NativeSelectSize::{size:?},\n  selected_index: {selected_index:?},\n  required: {required},\n  invalid: {invalid},\n  disabled: {disabled},\n  has_placeholder: {placeholder},\n  has_custom_class_name: {custom_class},\n}}"
        )
    });

    view! {
        <ComponentPage
            title="NativeSelect"
            slug="native-select"
            group="Forms"
            description="baseline-style native `<select>` wrapper with controllable selection, root `data-*` contracts, and stable option normalization."
        >
            <Playground title="Controlled + Placeholder" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-controlled".to_string()
                        options=options
                        selected_index=selected_signal
                        on_selected_index_change=on_selected_change
                        placeholder="Choose mode".to_string()
                        name="mode".to_string()
                        aria_label="Mode".to_string()
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Required + Invalid + Disabled" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-required".to_string()
                        options=required_options
                        default_selected_index=1
                        required=true
                        invalid=true
                        size=NativeSelectSize::Lg
                        aria_label="Deployment strategy".to_string()
                        class_name="docs-native-select-custom".to_string()
                    />

                    <NativeSelect
                        id_base="docs-native-select-disabled".to_string()
                        options=disabled_options
                        disabled=true
                        placeholder="Disabled select".to_string()
                        size=NativeSelectSize::Sm
                        aria_label="Disabled native select".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-components/src/native_select/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；支持 controlled selection / required / invalid / disabled 的对比展示。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"配置区 · Size"</div>
                        <ui_components::SegmentedControl
                            id_base="docs-native-select-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=ui_components::SegmentedControlSize::Sm
                            aria_label="NativeSelect size".to_string()
                        />
                        <div class="docs-search__label">"配置区 · Selected"</div>
                        <ui_components::SegmentedControl
                            id_base="docs-native-select-workbench-selected".to_string()
                            options=workbench_selected_options.clone()
                            selected_index=workbench_selected_mode_index
                            set_selected_index=set_workbench_selected_mode_index
                            size=ui_components::SegmentedControlSize::Sm
                            aria_label="NativeSelect selected index".to_string()
                        />
                        <ui_components::Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_placeholder set_checked=set_workbench_placeholder>
                            "Placeholder"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui_components::Switch>
                        <ui_components::Switch checked=workbench_show_compare set_checked=set_workbench_show_compare>
                            "Show compare matrix"
                        </ui_components::Switch>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let selected = workbench_selected_index.get();
                    let required = workbench_required.get();
                    let invalid = workbench_invalid.get();
                    let disabled = workbench_disabled.get();
                    let placeholder = if workbench_placeholder.get() {
                        "Choose mode".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-native-select-custom".to_string()
                    } else {
                        String::new()
                    };
                    let show_compare = workbench_show_compare.get();
                    let on_selected_index_change = on_workbench_selected_index_change;

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <NativeSelect
                                id_base="docs-native-select-workbench-primary".to_string()
                                options=workbench_options.get_value()
                                selected_index=Signal::derive(move || selected)
                                on_selected_index_change=on_selected_index_change
                                required=required
                                invalid=invalid
                                disabled=disabled
                                size=size
                                placeholder=placeholder
                                aria_label="Native select workbench".to_string()
                                class_name=class_name
                            />

                            <Show when=move || show_compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <NativeSelect
                                        id_base="docs-native-select-workbench-compare-required".to_string()
                                        options=workbench_options.get_value()
                                        default_selected_index=1
                                        required=true
                                        invalid=true
                                        size=NativeSelectSize::Lg
                                        placeholder="Required + invalid".to_string()
                                        aria_label="Required invalid compare".to_string()
                                    />
                                    <NativeSelect
                                        id_base="docs-native-select-workbench-compare-disabled".to_string()
                                        options=vec![
                                            NativeSelectOption::new("legacy", "Legacy").disabled(true),
                                            NativeSelectOption::new("frozen", "Frozen").disabled(true),
                                        ]
                                        disabled=true
                                        size=NativeSelectSize::Sm
                                        placeholder="Disabled select".to_string()
                                        aria_label="Disabled compare".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
