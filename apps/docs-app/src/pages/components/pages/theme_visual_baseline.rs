use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Button, ButtonVariant, Input, OnPress, Overlay, SegmentedControl, SegmentedControlSize, Switch,
};
use ui_layout::{
    Card, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, Heading, HeadingLevel,
};

pub(super) fn theme_visual_baseline() -> AnyView {
    let (email, set_email) = signal("design@rust-ui.dev".to_string());

    let (overlay_open_raw, set_overlay_open_raw) = signal(true);
    let overlay_open: Signal<bool> = Signal::derive(move || overlay_open_raw.get());

    let (overlay_present, set_overlay_present) = signal(overlay_open.get_untracked());
    Effect::new(move |_| {
        if overlay_open.get() {
            set_overlay_present.set(true);
        }
    });

    let open_overlay: OnPress = Callback::new(move |_| set_overlay_open_raw.set(true));
    let close_overlay: OnPress = Callback::new(move |_| set_overlay_open_raw.set(false));
    let on_exit_complete = Callback::new(move |_| set_overlay_present.set(false));

    let showcase_code = Signal::derive(move || {
        r#"let (email, set_email) = signal("design@rust-ui.dev".to_string());
let (overlay_open_raw, set_overlay_open_raw) = signal(true);
let overlay_open: Signal<bool> = Signal::derive(move || overlay_open_raw.get());

<Button variant=ButtonVariant::Accent>"Primary"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>

<Input
  id="theme-baseline-input".to_string()
  value=email
  set_value=set_email
  label="Email".to_string()
  placeholder="design@rust-ui.dev".to_string()
  is_clearable=true
/>

<Overlay
  open=overlay_open
  on_close=Callback::new(move |_| set_overlay_open_raw.set(false))
>
  <div class="docs-card">"Overlay visual layer"</div>
</Overlay>"#
            .to_string()
    });

    let workbench_variant_options = vec![
        "Accent".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
    ];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0usize));
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ButtonVariant::Secondary,
            2 => ButtonVariant::Ghost,
            _ => ButtonVariant::Accent,
        });

    let (workbench_email, set_workbench_email) = signal("baseline@rust-ui.dev".to_string());
    let (workbench_clearable, set_workbench_clearable) = signal(true);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_compact_layout, set_workbench_compact_layout) = signal(false);
    let (workbench_show_disabled, set_workbench_show_disabled) = signal(true);

    let (workbench_overlay_open_raw, set_workbench_overlay_open_raw) = signal(false);
    let workbench_overlay_open: Signal<bool> =
        Signal::derive(move || workbench_overlay_open_raw.get());
    let (workbench_overlay_present, set_workbench_overlay_present) = signal(false);
    Effect::new(move |_| {
        if workbench_overlay_open.get() {
            set_workbench_overlay_present.set(true);
        }
    });

    let (workbench_press_count, set_workbench_press_count) = signal(0u32);
    let (workbench_overlay_close_count, set_workbench_overlay_close_count) = signal(0u32);

    let workbench_primary_press: OnPress =
        Callback::new(move |_| set_workbench_press_count.update(|count| *count += 1));
    let workbench_open_overlay: OnPress =
        Callback::new(move |_| set_workbench_overlay_open_raw.set(true));
    let workbench_close_overlay: OnPress = Callback::new(move |_| {
        set_workbench_overlay_open_raw.set(false);
        set_workbench_overlay_close_count.update(|count| *count += 1);
    });
    let workbench_on_exit_complete =
        Callback::new(move |_| set_workbench_overlay_present.set(false));

    let workbench_code = Signal::derive(move || {
        let variant_name = match workbench_variant.get() {
            ButtonVariant::Secondary => "Secondary",
            ButtonVariant::Ghost => "Ghost",
            _ => "Accent",
        };

        format!(
            "<Button variant=ButtonVariant::{variant_name} on_press=on_primary_press>\\n  \"Save theme\"\\n</Button>\\n<Button variant=ButtonVariant::Outline on_press=open_overlay>\\n  \"Preview overlay\"\\n</Button>\\n<Input\\n  id=\"docs-theme-visual-workbench-input\".to_string()\\n  value=workbench_email\\n  set_value=set_workbench_email\\n  label=\"Email\".to_string()\\n  placeholder=\"baseline@rust-ui.dev\".to_string()\\n  is_clearable={}\\n  invalid=Signal::derive(|| {})\\n/>\\n<Overlay\\n  open=Signal::derive(|| {})\\n  on_close=on_overlay_close\\n  on_exit_complete=on_exit_complete\\n>\\n  <Card class_name=\"docs-stack\".to_string()>\\n    \"Overlay depth preview\"\\n  </Card>\\n</Overlay>",
            workbench_clearable.get(),
            workbench_invalid.get(),
            workbench_overlay_open_raw.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant_name = match workbench_variant.get() {
            ButtonVariant::Secondary => "Secondary",
            ButtonVariant::Ghost => "Ghost",
            _ => "Accent",
        };
        let safe_email = workbench_email.get().replace('"', "\\\\\"");

        format!(
            "{{\\n  button_variant: \"{variant_name}\",\\n  input_clearable: {},\\n  input_invalid: {},\\n  compact_layout: {},\\n  show_disabled_button: {},\\n  overlay_open: {},\\n  on_primary_press_feedback: \"press_count={}\",\\n  on_overlay_close_feedback: \"close_count={}\",\\n  email_value: \"{}\"\\n}}",
            workbench_clearable.get(),
            workbench_invalid.get(),
            workbench_compact_layout.get(),
            workbench_show_disabled.get(),
            workbench_overlay_open_raw.get(),
            workbench_press_count.get(),
            workbench_overlay_close_count.get(),
            safe_email,
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Card class_name="docs-stack".to_string()>
  <Heading level=HeadingLevel::H4>"Calm"</Heading>
  <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
  <Input
    id="docs-theme-matrix-calm".to_string()
    value=signal("calm@rust-ui.dev".to_string()).0
    set_value=signal("calm@rust-ui.dev".to_string()).1
    label="Email".to_string()
  />
</Card>
<Card class_name="docs-stack".to_string()>
  <Heading level=HeadingLevel::H4>"Action"</Heading>
  <Button variant=ButtonVariant::Accent>"Primary"</Button>
  <Input
    id="docs-theme-matrix-action".to_string()
    value=signal("action@rust-ui.dev".to_string()).0
    set_value=signal("action@rust-ui.dev".to_string()).1
    label="Email".to_string()
    is_clearable=true
  />
</Card>
<Card class_name="docs-stack".to_string()>
  <Heading level=HeadingLevel::H4>"Alert"</Heading>
  <Button variant=ButtonVariant::Ghost>"Review"</Button>
  <Input
    id="docs-theme-matrix-alert".to_string()
    value=signal("alert@rust-ui.dev".to_string()).0
    set_value=signal("alert@rust-ui.dev".to_string()).1
    label="Email".to_string()
    invalid=Signal::derive(|| true)
  />
</Card>"#
            .to_string()
    });

    let (matrix_calm_email, set_matrix_calm_email) = signal("calm@rust-ui.dev".to_string());
    let (matrix_action_email, set_matrix_action_email) = signal("action@rust-ui.dev".to_string());
    let (matrix_alert_email, set_matrix_alert_email) = signal("alert@rust-ui.dev".to_string());

    view! {
        <ComponentPage
            title="ThemeVisualBaseline"
            slug="theme-visual-baseline"
            group="Layout"
            description="Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots."
        >
            <Playground
                title="Default Theme Visual Baseline"
                description="Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus)."
                code_signal=showcase_code
            >
                <div data-slot="theme-visual-baseline">
                    <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="docs-stack".to_string()>
                        <div data-slot="theme-visual-baseline-surface">
                            <Card class_name="docs-stack".to_string()>
                                <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="docs-stack docs-stack--tight".to_string()>
                                    <Heading level=HeadingLevel::H3>"Visual Baseline"</Heading>
                                    <p class="ui-muted">
                                        "Default theme should feel trustworthy at first glance: clear hierarchy, natural contrast, and explicit interaction feedback."
                                    </p>
                                </Flex>

                                <div data-slot="theme-visual-baseline-button">
                                    <Flex
                                        align=FlexAlign::Center
                                        gap=FlexGap::Sm
                                        wrap=ui_layout::FlexWrap::Wrap
                                        class_name="docs-row".to_string()
                                    >
                                        <Button variant=ButtonVariant::Accent>"Primary Action"</Button>
                                        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                                        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                                        <Button is_disabled=true>"Disabled"</Button>
                                    </Flex>
                                </div>

                                <div data-slot="theme-visual-baseline-input">
                                    <Input
                                        id="docs-theme-visual-baseline-input".to_string()
                                        value=email
                                        set_value=set_email
                                        label="Email".to_string()
                                        placeholder="design@rust-ui.dev".to_string()
                                        is_clearable=true
                                    />
                                </div>

                                <span class="ui-muted">"input: " {move || email.get()}</span>
                            </Card>
                        </div>

                        <Flex
                            align=FlexAlign::Center
                            gap=FlexGap::Sm
                            wrap=ui_layout::FlexWrap::Wrap
                            class_name="docs-row".to_string()
                        >
                            <Button on_press=open_overlay>"Open Overlay Baseline"</Button>
                            <Button variant=ButtonVariant::Outline on_press=close_overlay>
                                "Close Overlay Baseline"
                            </Button>
                            <span class="ui-muted">"overlay open: " {move || overlay_open_raw.get()}</span>
                        </Flex>
                    </Flex>
                </div>

                <Show when=move || overlay_present.get()>
                    <Overlay
                        open=overlay_open
                        on_close=close_overlay
                        aria_labelledby="docs-theme-visual-overlay-title".to_string()
                        aria_describedby="docs-theme-visual-overlay-desc".to_string()
                        class_name="docs-theme-visual-overlay".to_string()
                        on_exit_complete=on_exit_complete
                    >
                        <div data-slot="theme-visual-baseline-overlay">
                            <Card class_name="docs-stack".to_string()>
                                <h4 id="docs-theme-visual-overlay-title">"Overlay Depth"</h4>
                                <p id="docs-theme-visual-overlay-desc" class="ui-muted">
                                    "Overlay layers must preserve background separation, text readability, and interaction focus."
                                </p>
                                <Flex
                                    justify=FlexJustify::End
                                    align=FlexAlign::Center
                                    gap=FlexGap::Sm
                                    class_name="docs-row docs-row--end".to_string()
                                >
                                    <Button variant=ButtonVariant::Secondary on_press=close_overlay>
                                        "Close"
                                    </Button>
                                </Flex>
                            </Card>
                        </div>
                    </Overlay>
                </Show>
            </Playground>

            <Playground
                title="Workbench Theme Visual Baseline"
                description="Config panel with live feedback: button callback count, overlay close count, and real-time config/code output."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="theme-visual-workbench-controls">
                        <div class="docs-search__label">"Primary button variant"</div>
                        <SegmentedControl
                            id_base="docs-theme-visual-workbench-variant".to_string()
                            options=workbench_variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Theme visual baseline primary variant".to_string()
                        />
                        <Switch checked=workbench_clearable set_checked=set_workbench_clearable>
                            "Input clearable"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Input invalid"
                        </Switch>
                        <Switch
                            checked=workbench_compact_layout
                            set_checked=set_workbench_compact_layout
                        >
                            "Compact layout"
                        </Switch>
                        <Switch checked=workbench_show_disabled set_checked=set_workbench_show_disabled>
                            "Show disabled button"
                        </Switch>
                        <Switch
                            checked=workbench_overlay_open_raw
                            set_checked=set_workbench_overlay_open_raw
                        >
                            "Overlay open"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let variant = workbench_variant.get();
                    let compact_layout = workbench_compact_layout.get();
                    let panel_class = if compact_layout {
                        "docs-stack docs-stack--tight"
                    } else {
                        "docs-stack"
                    };
                    let input_invalid_signal: Signal<bool> = Signal::derive(move || workbench_invalid.get());

                    view! {
                        <div class=panel_class data-slot="theme-visual-workbench">
                            <Card class_name="docs-stack".to_string()>
                                <Heading level=HeadingLevel::H3>"Workbench Preview"</Heading>
                                <p class="ui-muted">
                                    "Tune baseline decisions while watching real callback feedback and config output."
                                </p>

                                <Flex
                                    align=FlexAlign::Center
                                    gap=FlexGap::Sm
                                    wrap=ui_layout::FlexWrap::Wrap
                                    class_name="docs-row".to_string()
                                >
                                    <Button variant=variant on_press=workbench_primary_press>
                                        "Save Theme"
                                    </Button>
                                    <Button variant=ButtonVariant::Outline on_press=workbench_open_overlay>
                                        "Preview Overlay"
                                    </Button>
                                    <Show when=move || workbench_show_disabled.get()>
                                        <Button is_disabled=true>"Disabled"</Button>
                                    </Show>
                                </Flex>

                                <Input
                                    id="docs-theme-visual-workbench-input".to_string()
                                    value=workbench_email
                                    set_value=set_workbench_email
                                    label="Email".to_string()
                                    placeholder="baseline@rust-ui.dev".to_string()
                                    is_clearable=workbench_clearable.get()
                                    invalid=input_invalid_signal
                                />

                                <div class="ui-muted" data-slot="theme-visual-workbench-feedback">
                                    "press_count=" {move || workbench_press_count.get()}
                                    " | overlay_close_count=" {move || workbench_overlay_close_count.get()}
                                    " | overlay_open=" {move || workbench_overlay_open_raw.get()}
                                </div>
                            </Card>

                            <Show when=move || workbench_overlay_present.get()>
                                <Overlay
                                    open=workbench_overlay_open
                                    on_close=workbench_close_overlay
                                    aria_labelledby="docs-theme-visual-workbench-overlay-title".to_string()
                                    aria_describedby="docs-theme-visual-workbench-overlay-desc".to_string()
                                    class_name="docs-theme-visual-overlay".to_string()
                                    on_exit_complete=workbench_on_exit_complete
                                >
                                    <div data-slot="theme-visual-workbench-overlay">
                                        <Card class_name="docs-stack".to_string()>
                                            <h4 id="docs-theme-visual-workbench-overlay-title">
                                                "Workbench Overlay"
                                            </h4>
                                            <p id="docs-theme-visual-workbench-overlay-desc" class="ui-muted">
                                                "Close actions are counted in feedback to prove callback wiring."
                                            </p>
                                            <Flex
                                                justify=FlexJustify::End
                                                align=FlexAlign::Center
                                                gap=FlexGap::Sm
                                                class_name="docs-row docs-row--end".to_string()
                                            >
                                                <Button
                                                    variant=ButtonVariant::Secondary
                                                    on_press=workbench_close_overlay
                                                >
                                                    "Close"
                                                </Button>
                                            </Flex>
                                        </Card>
                                    </div>
                                </Overlay>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Calm / Action / Alert)" code_signal=matrix_code>
                <div data-slot="theme-visual-matrix">
                    <Flex
                        direction=FlexDirection::Column
                        gap=FlexGap::Sm
                        class_name="docs-stack docs-stack--tight".to_string()
                    >
                    <Card class_name="docs-stack".to_string()>
                        <Heading level=HeadingLevel::H4>"Calm"</Heading>
                        <p class="ui-muted">"Secondary emphasis, default form state."</p>
                        <Flex align=FlexAlign::Center gap=FlexGap::Sm class_name="docs-row".to_string()>
                            <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                            <Input
                                id="docs-theme-visual-matrix-calm".to_string()
                                value=matrix_calm_email
                                set_value=set_matrix_calm_email
                                label="Email".to_string()
                            />
                        </Flex>
                    </Card>

                    <Card class_name="docs-stack".to_string()>
                        <Heading level=HeadingLevel::H4>"Action"</Heading>
                        <p class="ui-muted">"Primary emphasis with clearable input."</p>
                        <Flex align=FlexAlign::Center gap=FlexGap::Sm class_name="docs-row".to_string()>
                            <Button variant=ButtonVariant::Accent>"Primary"</Button>
                            <Input
                                id="docs-theme-visual-matrix-action".to_string()
                                value=matrix_action_email
                                set_value=set_matrix_action_email
                                label="Email".to_string()
                                is_clearable=true
                            />
                        </Flex>
                    </Card>

                    <Card class_name="docs-stack".to_string()>
                        <Heading level=HeadingLevel::H4>"Alert"</Heading>
                        <p class="ui-muted">"Ghost action paired with invalid form feedback."</p>
                        <Flex align=FlexAlign::Center gap=FlexGap::Sm class_name="docs-row".to_string()>
                            <Button variant=ButtonVariant::Ghost>"Review"</Button>
                            <Input
                                id="docs-theme-visual-matrix-alert".to_string()
                                value=matrix_alert_email
                                set_value=set_matrix_alert_email
                                label="Email".to_string()
                                invalid=Signal::derive(|| true)
                            />
                        </Flex>
                    </Card>
                    </Flex>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
