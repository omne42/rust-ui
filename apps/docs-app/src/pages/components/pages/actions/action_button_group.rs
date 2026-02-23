use super::*;

pub(crate) fn action_button_group() -> AnyView {
    let (showcase_count, set_showcase_count) = signal(0_u32);
    let on_showcase_press: OnPress = Callback::new(move |_| {
        set_showcase_count.update(|count| *count += 1);
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let density_options = vec!["Regular".to_string(), "Compact".to_string()];
    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];

    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_density_index, set_workbench_density_index) = signal(Some(0_usize));
    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_justified, set_workbench_justified) = signal(false);
    let (workbench_quiet, set_workbench_quiet) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let on_workbench_press: OnPress = Callback::new(move |_| {
        set_workbench_press_count.update(|count| *count += 1);
    });

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::S,
        3 => ActionButtonSize::L,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let workbench_density = Signal::derive(move || {
        if workbench_density_index.get().unwrap_or(0) == 1 {
            ActionButtonGroupDensity::Compact
        } else {
            ActionButtonGroupDensity::Regular
        }
    });
    let workbench_orientation = Signal::derive(move || {
        if workbench_orientation_index.get().unwrap_or(0) == 1 {
            ActionButtonGroupOrientation::Vertical
        } else {
            ActionButtonGroupOrientation::Horizontal
        }
    });

    let hello_code = Signal::derive(move || {
        r#"<ActionButtonGroup
  size=ActionButtonSize::S
  density=ActionButtonGroupDensity::Compact
  orientation=ActionButtonGroupOrientation::Horizontal
  is_quiet=true
>
  <ActionButton on_press=Callback::new(move |_| {})>"One"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Two"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Three"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ActionButtonGroup\n  size={:?}\n  density={:?}\n  orientation={:?}\n  is_justified={}\n  is_quiet={}\n  is_disabled={}\n  motion=ActionButtonGroupMotion::default()\n  aria_label=\"Action group workbench\".to_string()\n  lang={}.to_string()\n  dir={}\n  class_name={}\n>\n  <ActionButton on_press=on_press>\"Primary\"</ActionButton>\n  <ActionButton on_press=on_press>\"Secondary\"</ActionButton>\n  <ActionButton on_press=on_press>\"Danger\"</ActionButton>\n</ActionButtonGroup>",
            workbench_size.get(),
            workbench_density.get(),
            workbench_orientation.get(),
            workbench_justified.get(),
            workbench_quiet.get(),
            workbench_disabled.get(),
            if workbench_lang_zh.get() {
                "\"zh-CN\""
            } else {
                "\"en-US\""
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            if workbench_custom_class.get() {
                "\"docs-action-button-group-workbench\".to_string()"
            } else {
                "String::new()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ActionButtonGroupActualConfig {{\n  size: {:?},\n  density: {:?},\n  orientation: {:?},\n  is_justified: {},\n  is_quiet: {},\n  is_disabled: {},\n  motion: ActionButtonGroupMotion::default(),\n  aria_label: Some(\"Action group workbench\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  class_name: {:?},\n}}",
            workbench_size.get(),
            workbench_density.get(),
            workbench_orientation.get(),
            workbench_justified.get(),
            workbench_quiet.get(),
            workbench_disabled.get(),
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            if workbench_custom_class.get() {
                Some("docs-action-button-group-workbench")
            } else {
                None
            },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionButtonGroup size=ActionButtonSize::M density=ActionButtonGroupDensity::Regular orientation=ActionButtonGroupOrientation::Horizontal aria_label="Default".to_string()>
  <ActionButton on_press=Callback::new(move |_| {})>"A"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"B"</ActionButton>
</ActionButtonGroup>
<ActionButtonGroup size=ActionButtonSize::S density=ActionButtonGroupDensity::Compact orientation=ActionButtonGroupOrientation::Vertical is_justified=true is_quiet=true class_name="docs-action-button-group-workbench".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl aria_label="Vertical".to_string()>
  <ActionButton on_press=Callback::new(move |_| {})>"Top"</ActionButton>
  <ActionButton on_press=Callback::new(move |_| {})>"Bottom"</ActionButton>
</ActionButtonGroup>
<ActionButtonGroup size=ActionButtonSize::M is_disabled=true motion=ActionButtonGroupMotion::default() aria_label="Disabled".to_string()>
  <ActionButton>"Disabled"</ActionButton>
  <ActionButton>"Group"</ActionButton>
</ActionButtonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ActionButtonGroup"
            slug="action-button-group"
            group="Actions"
            description="Toolbar-style action clusters with full API workbench coverage."
        >
            <Playground title="Hello World (Default + compact)" code_signal=hello_code>
                <div class="docs-stack">
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Horizontal
                        is_quiet=true
                        aria_label="Quick actions".to_string()
                    >
                        <ActionButton on_press=on_showcase_press>"One"</ActionButton>
                        <ActionButton on_press=on_showcase_press>"Two"</ActionButton>
                        <ActionButton on_press=on_showcase_press>"Three"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "pressed: "
                        {move || showcase_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-button-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-action-button-group-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup size".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-action-button-group-density".to_string()
                            options=density_options.clone()
                            selected_index=workbench_density_index
                            set_selected_index=set_workbench_density_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup density".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-action-button-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButtonGroup orientation".to_string()
                        />
                        <Switch checked=workbench_justified set_checked=set_workbench_justified>
                            "is_justified"
                        </Switch>
                        <Switch checked=workbench_quiet set_checked=set_workbench_quiet>
                            "is_quiet"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <ActionButtonGroup
                        size=workbench_size.get()
                        density=workbench_density.get()
                        orientation=workbench_orientation.get()
                        is_justified=workbench_justified.get()
                        is_quiet=workbench_quiet.get()
                        is_disabled=workbench_disabled.get()
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Action group workbench".to_string()
                        lang=if workbench_lang_zh.get() { "zh-CN".to_string() } else { "en-US".to_string() }
                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                        class_name=if workbench_custom_class.get() {
                            "docs-action-button-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <ActionButton on_press=on_workbench_press>"Primary"</ActionButton>
                        <ActionButton on_press=on_workbench_press>"Secondary"</ActionButton>
                        <ActionButton on_press=on_workbench_press>"Danger"</ActionButton>
                    </ActionButtonGroup>
                    <span class="ui-muted">
                        "workbench on_press count: "
                        {move || workbench_press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Horizontal / Vertical / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionButtonGroup
                        size=ActionButtonSize::M
                        density=ActionButtonGroupDensity::Regular
                        orientation=ActionButtonGroupOrientation::Horizontal
                        aria_label="Default group".to_string()
                    >
                        <ActionButton>"A"</ActionButton>
                        <ActionButton>"B"</ActionButton>
                    </ActionButtonGroup>
                    <ActionButtonGroup
                        size=ActionButtonSize::S
                        density=ActionButtonGroupDensity::Compact
                        orientation=ActionButtonGroupOrientation::Vertical
                        is_justified=true
                        is_quiet=true
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Vertical group".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        class_name="docs-action-button-group-workbench".to_string()
                    >
                        <ActionButton>"Top"</ActionButton>
                        <ActionButton>"Bottom"</ActionButton>
                    </ActionButtonGroup>
                    <ActionButtonGroup
                        size=ActionButtonSize::M
                        is_disabled=true
                        motion=ActionButtonGroupMotion::default()
                        aria_label="Disabled group".to_string()
                    >
                        <ActionButton>"Disabled"</ActionButton>
                        <ActionButton>"Group"</ActionButton>
                    </ActionButtonGroup>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
