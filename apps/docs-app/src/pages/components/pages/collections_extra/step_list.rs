use super::*;

pub(crate) fn step_list() -> AnyView {
    let steps = vec![
        StepListItem::new("account", "Account").described("Create account and verify email"),
        StepListItem::new("shipping", "Shipping").described("Choose shipping address"),
        StepListItem::new("payment", "Payment").described("Add payment method"),
        StepListItem::new("review", "Review").described("Confirm and place order"),
    ];

    let steps_with_disabled = vec![
        StepListItem::new("plan", "Plan").described("Pick your subscription tier"),
        StepListItem::new("profile", "Profile").described("Fill organization details"),
        StepListItem::new("billing", "Billing")
            .described("Billing is locked until profile is approved")
            .disabled(true),
        StepListItem::new("launch", "Launch").described("Start using the workspace"),
    ];

    let (selected_index, set_selected_index) = signal(Some(1_usize));
    let selected_index_signal: Signal<Option<usize>> = selected_index.into();
    let (on_selected_index_change_runs, set_on_selected_index_change_runs) = signal(0_u32);
    let on_selected_index_change = Callback::new(move |next: Option<usize>| {
        set_selected_index.set(next);
        set_on_selected_index_change_runs.update(|count| *count += 1);
    });

    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_emphasized, set_workbench_emphasized) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let workbench_size_options = vec!["S".to_string(), "M".to_string(), "L".to_string()];
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => StepListSize::S,
        2 => StepListSize::L,
        _ => StepListSize::M,
    });
    let workbench_steps = steps.clone();
    let showcase_steps = steps.clone();

    let hello_code = Signal::derive(move || {
        r#"<StepList
  id_base="docs-step-list-hello".to_string()
  steps=signal(steps).0
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            "StepListOrientation::Vertical"
        } else {
            "StepListOrientation::Horizontal"
        };
        let size = match workbench_size.get() {
            StepListSize::S => "StepListSize::S",
            StepListSize::M => "StepListSize::M",
            StepListSize::L => "StepListSize::L",
            StepListSize::Xl => "StepListSize::Xl",
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-step-list-custom"
        } else {
            ""
        };

        [
            "<StepList".to_string(),
            "  steps=signal(steps).0".to_string(),
            format!("  orientation={orientation}"),
            format!("  size={size}"),
            format!("  is_emphasized={}", bool_word(workbench_emphasized.get())),
            format!("  is_disabled={}", bool_word(workbench_disabled.get())),
            "  selected_index=selected_index_signal".to_string(),
            "  default_selected_index=1".to_string(),
            "  completed_indices=vec![0]".to_string(),
            "  on_selected_index_change=on_selected_index_change".to_string(),
            "  id_base=\"docs-step-list-workbench\".to_string()".to_string(),
            "  aria_label=\"Checkout progress\".to_string()".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = if workbench_vertical.get() {
            StepListOrientation::Vertical
        } else {
            StepListOrientation::Horizontal
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-step-list-custom")
        } else {
            None
        };

        format!(
            "StepListActualConfig {{\n  steps: \"sample_steps(len=4)\",\n  orientation: {orientation:?},\n  size: {:?},\n  is_emphasized: {},\n  is_disabled: {},\n  selected_index: {:?},\n  default_selected_index: Some(1),\n  completed_indices: {:?},\n  on_selected_index_change: \"runs={}\",\n  id_base: Some(\"docs-step-list-workbench\"),\n  aria_label: Some(\"Checkout progress\"),\n  class_name: {class_name:?},\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n}}",
            workbench_size.get(),
            bool_word(workbench_emphasized.get()),
            bool_word(workbench_disabled.get()),
            selected_index.get(),
            vec![0_usize],
            on_selected_index_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<StepList id_base="step-default".to_string() steps=signal(steps).0 default_selected_index=1 />
<StepList id_base="step-vertical".to_string() steps=signal(steps).0 orientation=StepListOrientation::Vertical size=StepListSize::L is_emphasized=true />
<StepList id_base="step-disabled".to_string() steps=signal(disabled_steps).0 is_disabled=true default_selected_index=2 />"#.to_string()
    });

    view! {
        <ComponentPage
            title="StepList"
            slug="step-list"
            group="Collections"
            description="baseline-compatible step progression primitive with centralized orientation/size/status normalization and stable slot + data-state contracts."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <StepList
                    id_base="docs-step-list-hello".to_string()
                    steps=signal(showcase_steps).0
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="step-list-workbench-controls">
                        <SegmentedControl
                            id_base="docs-step-list-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="StepList size".to_string()
                        />
                        <Switch checked=workbench_vertical set_checked=set_workbench_vertical>
                            "Vertical orientation"
                        </Switch>
                        <Switch checked=workbench_emphasized set_checked=set_workbench_emphasized>
                            "Emphasized"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="step-list-workbench-preview">
                    <StepList
                        steps=signal(workbench_steps).0
                        orientation=if workbench_vertical.get() {
                            StepListOrientation::Vertical
                        } else {
                            StepListOrientation::Horizontal
                        }
                        size=workbench_size.get()
                        is_emphasized=workbench_emphasized.get()
                        is_disabled=workbench_disabled.get()
                        selected_index=selected_index_signal
                        default_selected_index=1
                        completed_indices=vec![0]
                        on_selected_index_change=on_selected_index_change
                        id_base="docs-step-list-workbench".to_string()
                        aria_label="Checkout progress".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-step-list-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted" data-slot="step-list-workbench-feedback">
                        "selected index: "
                        {move || selected_index.get().map_or("none".to_string(), |it| it.to_string())}
                        " · on_selected_index_change: " {move || on_selected_index_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="step-list-state-matrix">
                    <StepList
                        id_base="docs-step-list-matrix-default".to_string()
                        steps=signal(steps.clone()).0
                        default_selected_index=1
                    />
                    <StepList
                        id_base="docs-step-list-matrix-vertical".to_string()
                        steps=signal(steps.clone()).0
                        orientation=StepListOrientation::Vertical
                        size=StepListSize::L
                        is_emphasized=true
                        completed_indices=vec![0, 1]
                    />
                    <StepList
                        id_base="docs-step-list-matrix-disabled".to_string()
                        steps=signal(steps_with_disabled).0
                        is_disabled=true
                        default_selected_index=2
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
