use super::*;

pub(crate) fn button() -> AnyView {
    let persisted_workbench_state = load_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let variant_options = vec![
        "solid".to_string(),
        "faded".to_string(),
        "bordered".to_string(),
        "light".to_string(),
        "flat".to_string(),
        "ghost".to_string(),
        "shadow".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(initial_workbench_state.variant_index));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Faded,
        2 => ButtonVariant::Bordered,
        3 => ButtonVariant::Light,
        4 => ButtonVariant::Flat,
        5 => ButtonVariant::Ghost,
        6 => ButtonVariant::Shadow,
        _ => ButtonVariant::Solid,
    });

    let color_options = vec![
        "default".to_string(),
        "primary".to_string(),
        "secondary".to_string(),
        "success".to_string(),
        "warning".to_string(),
        "danger".to_string(),
    ];
    let (color_index, set_color_index) = signal(Some(initial_workbench_state.color_index));
    let color = Signal::derive(move || match color_index.get().unwrap_or(1) {
        0 => ButtonColor::Default,
        2 => ButtonColor::Secondary,
        3 => ButtonColor::Success,
        4 => ButtonColor::Warning,
        5 => ButtonColor::Danger,
        _ => ButtonColor::Primary,
    });

    let radius_options = vec![
        "full".to_string(),
        "lg".to_string(),
        "md".to_string(),
        "sm".to_string(),
        "none".to_string(),
    ];
    let (radius_index, set_radius_index) = signal(Some(initial_workbench_state.radius_index));
    let radius = Signal::derive(move || match radius_index.get().unwrap_or(2) {
        0 => ButtonRadius::Full,
        1 => ButtonRadius::Lg,
        3 => ButtonRadius::Sm,
        4 => ButtonRadius::None,
        _ => ButtonRadius::Md,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(initial_workbench_state.size_index));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (is_disabled, set_is_disabled) = signal(initial_workbench_state.is_disabled);
    let (loading, set_loading) = signal(initial_workbench_state.is_loading);
    let loading_placement_options =
        vec!["Start".to_string(), "End".to_string(), "Center".to_string()];
    let (loading_placement_index, set_loading_placement_index) =
        signal(Some(initial_workbench_state.loading_placement_index));
    let loading_placement =
        Signal::derive(move || match loading_placement_index.get().unwrap_or(0) {
            1 => ButtonLoadingPlacement::End,
            2 => ButtonLoadingPlacement::Center,
            _ => ButtonLoadingPlacement::Start,
        });
    let (icon_only, set_icon_only) = signal(initial_workbench_state.is_icon_only);
    let (is_full_width, set_is_full_width) = signal(initial_workbench_state.is_full_width);
    let (show_start, set_show_start) = signal(initial_workbench_state.show_start);
    let (show_end, set_show_end) = signal(initial_workbench_state.show_end);
    let (spec_schema_enabled, set_spec_schema_enabled) = signal(false);
    let (spec_requires_confirmation, set_spec_requires_confirmation) = signal(false);
    let spec_schema_json = Signal::derive(move || {
        if !spec_schema_enabled.get() {
            return None;
        }

        Some(
            ButtonSchema::new(
                "docs-button-workbench",
                ButtonIntent::Primary,
                "button.press",
            )
            .requires_confirmation(spec_requires_confirmation.get())
            .to_json(),
        )
    });
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_button_workbench_state(ButtonWorkbenchState {
                variant_index: variant_index.get().unwrap_or(0),
                color_index: color_index.get().unwrap_or(1),
                radius_index: radius_index.get().unwrap_or(2),
                size_index: size_index.get().unwrap_or(2),
                loading_placement_index: loading_placement_index.get().unwrap_or(0),
                is_disabled: is_disabled.get(),
                is_loading: loading.get(),
                is_icon_only: icon_only.get(),
                is_full_width: is_full_width.get(),
                show_start: show_start.get(),
                show_end: show_end.get(),
            });
        } else {
            clear_button_workbench_state();
        }
    });

    let hello_code = Signal::derive(move || r#"<Button>"Button"</Button>"#.to_string());
    let button_imports = "use leptos::prelude::*;\nuse ui::{Button, ButtonColor, ButtonLoadingPlacement, ButtonRadius, ButtonSize, ButtonVariant};".to_string();

    let code = Signal::derive(move || {
        let variant = variant.get();
        let color = color.get();
        let radius = radius.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let loading = loading.get();
        let loading_placement = loading_placement.get();
        let icon_only = icon_only.get();
        let is_full_width = is_full_width.get();
        let show_start = show_start.get();
        let show_end = show_end.get();
        let schema_json = spec_schema_json.get();

        let mut snippet = vec!["<Button".to_string()];

        if color != ButtonColor::Primary {
            snippet.push(format!("  color=ButtonColor::{color:?}"));
        }
        if variant != ButtonVariant::Solid {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if radius != ButtonRadius::Md {
            snippet.push(format!("  radius=ButtonRadius::{radius:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if is_disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if loading {
            snippet.push("  is_loading=true".to_string());
            if loading_placement != ButtonLoadingPlacement::Start {
                snippet.push(format!(
                    "  loading_placement=ButtonLoadingPlacement::{loading_placement:?}"
                ));
            }
        }
        if icon_only {
            snippet.push("  is_icon_only=true".to_string());
            snippet.push("  aria_label=\"Button\".into()".to_string());
        }
        if is_full_width {
            snippet.push("  is_full_width=true".to_string());
        }
        if show_start {
            snippet.push("  start_content=move || view! { <span>\"★\"</span> }".to_string());
        }
        if show_end {
            snippet.push("  end_content=move || view! { <span>\"→\"</span> }".to_string());
        }
        if let Some(schema_json) = schema_json {
            snippet.push(format!(
                "  schema_json=Some(r#\"{schema_json}\"#.to_string())"
            ));
        }

        snippet.extend([
            ">".to_string(),
            if icon_only {
                "  \"★\"".to_string()
            } else {
                "  \"Button\"".to_string()
            },
            "</Button>".to_string(),
        ]);

        snippet.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/styles.rs */\n{}",
            ui::button::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let variant = variant.get();
        let color = color.get();
        let radius = radius.get();
        let size = size.get();
        let is_disabled = is_disabled.get();
        let is_loading = loading.get();
        let loading_placement = loading_placement.get();
        let icon_only = icon_only.get();
        let is_full_width = is_full_width.get();
        let show_start = show_start.get();
        let show_end = show_end.get();
        let schema_json = spec_schema_json.get();

        let mut classes = vec![
            "ui-button".to_string(),
            variant.class_name().into(),
            color.class_name().into(),
            radius.class_name().into(),
            size.class_name().into(),
            format!("ui-button--loading-{}", loading_placement.as_attr()),
        ];

        if icon_only {
            classes.push("ui-button--icon-only".to_string());
        }
        if is_full_width {
            classes.push("ui-button--full-width".to_string());
        }
        if is_loading {
            classes.push("ui-button--loading".to_string());
        }
        if show_start {
            classes.push("ui-button--has-start".to_string());
        }
        if show_end {
            classes.push("ui-button--has-end".to_string());
        }

        format!(
            "ButtonActualConfig {{\n  color: {color:?},\n  variant: {variant:?},\n  radius: {radius:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  is_loading: {is_loading},\n  loading_placement: {loading_placement:?},\n  is_icon_only: {icon_only},\n  aria_label: {},\n  is_full_width: {is_full_width},\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n  schema_json: {schema_json:?},\n  class_name: {:?},\n  on_press: {:?},\n  class: \"{}\",\n}}",
            if icon_only {
                "Some(\"Button\")"
            } else {
                "None"
            },
            None::<String>,
            "Callback<MouseEvent>",
            classes.join(" ")
        )
    });

    let colors_code = Signal::derive(move || {
        r#"<Button color="default">"Default"</Button>
<Button color="primary">"Primary"</Button>
<Button color="secondary">"Secondary"</Button>
<Button color="success">"Success"</Button>
<Button color="warning">"Warning"</Button>
<Button color="danger">"Danger"</Button>"#
            .to_string()
    });

    let radius_code = Signal::derive(move || {
        r#"<Button radius="full" color="default">"Full"</Button>
<Button radius="lg" color="default">"Large"</Button>
<Button radius="md" color="default">"Medium"</Button>
<Button radius="sm" color="default">"Small"</Button>
<Button radius="none" color="default">"None"</Button>"#
            .to_string()
    });

    let sizes_code = Signal::derive(move || {
        r#"<Button size="xs">"XS"</Button>
<Button size="s">"S"</Button>
<Button size="m">"M"</Button>
<Button size="l">"L"</Button>
<Button size="xl">"XL"</Button>"#
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Button id="docs-button-matrix-idle".to_string()>"Idle"</Button>
<Button
  id="docs-button-matrix-loading".to_string()
  is_loading=true
  loading_placement=ButtonLoadingPlacement::Start
>
  "Loading"
</Button>
<Button id="docs-button-matrix-disabled".to_string() is_disabled=true>"Disabled"</Button>
<Button
  id="docs-button-matrix-icon-only".to_string()
  aria_label="Icon only".to_string()
>
  "★"
</Button>"#
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"// N/A: Button has no value/open selection axis.
// Use explicit props/callbacks and keep loading/disabled state in caller.
<Button id="docs-button-controlled-like".to_string() is_loading=true>"Parent-managed loading"</Button>
<Button id="docs-button-uncontrolled-like".to_string()>"No internal state axis"</Button>"#
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r#"// Button is not a long-form reading surface.
// Streaming is optional; docs fallback remains snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  "Button docs output mode: snapshot"
</div>
<Button id="docs-button-snapshot".to_string()>"Snapshot"</Button>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Button
  id="docs-button-source-first".to_string()
  color=ButtonColor::Primary
  variant=ButtonVariant::Solid
>
  "Build"
</Button>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Button"
            slug="button"
            group="Actions"
            description="Variants + sizes with spring hover/tap motion."
        >
            <Playground title="Hello World" code_signal=hello_code code_imports=button_imports.clone()>
                <div class="docs-row">
                    <Button
                        class_name="docs-button-showcase".to_string()
                        on_press=Callback::new(move |_| {})
                    >
                        "Button"
                    </Button>
                </div>
            </Playground>

            <Playground
                title="Variants & sizes"
                code_signal=code
                code_imports=button_imports.clone()
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/styles.rs".to_string()
                test_config_signal=actual_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button variant".to_string()
                        />

                        <div class="docs-search__label">"Color"</div>
                        <SegmentedControl
                            id_base="docs-button-color".to_string()
                            options=color_options.clone()
                            selected_index=color_index
                            set_selected_index=set_color_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button color".to_string()
                        />

                        <div class="docs-search__label">"Radius"</div>
                        <SegmentedControl
                            id_base="docs-button-radius".to_string()
                            options=radius_options.clone()
                            selected_index=radius_index
                            set_selected_index=set_radius_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button radius".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button size".to_string()
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        <Switch checked=loading set_checked=set_loading>"Loading"</Switch>
                        <div class="docs-search__label">"Loading placement"</div>
                        <SegmentedControl
                            id_base="docs-button-loading-placement".to_string()
                            options=loading_placement_options.clone()
                            selected_index=loading_placement_index
                            set_selected_index=set_loading_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="Button loading placement".to_string()
                        />
                        <Switch checked=icon_only set_checked=set_icon_only>"Icon only"</Switch>
                        <Switch checked=is_full_width set_checked=set_is_full_width>"Full width"</Switch>
                        <Switch checked=show_start set_checked=set_show_start>"Start slot"</Switch>
                        <Switch checked=show_end set_checked=set_show_end>"End slot"</Switch>
                        <Switch checked=spec_schema_enabled set_checked=set_spec_schema_enabled>
                            "Use AI spec payload"
                        </Switch>
                        <Switch checked=spec_requires_confirmation set_checked=set_spec_requires_confirmation>
                            "Spec requires confirmation"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let color = color.get();
                    let radius = radius.get();
                    let size = size.get();
                    let is_disabled = is_disabled.get();
                    let is_loading = loading.get();
                    let loading_placement = loading_placement.get();
                    let icon_only = icon_only.get();
                    let is_full_width = is_full_width.get();
                    let show_start = show_start.get();
                    let show_end = show_end.get();
                    let schema_json = spec_schema_json.get();
                    let persist = workbench_persist_state.get();

                    view! {
                        <div class="docs-stack" data-slot="button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-stack docs-stack--tight" data-slot="button-interactive-spec-preview">
                                <span class="ui-muted" data-slot="button-interactive-spec-input">
                                    "spec-input: "
                                    {if schema_json.is_some() { "schema_json" } else { "off" }}
                                </span>
                                <code data-slot="button-interactive-spec-json">
                                    {schema_json
                                        .clone()
                                        .unwrap_or_else(|| "none".to_string())}
                                </code>
                            </div>
                            <div class="docs-card" data-slot="button-workbench-canvas">
                                <div
                                    class="docs-row"
                                    style=if is_full_width {
                                        "width: 100%;"
                                    } else {
                                        "width: fit-content; margin-inline: auto;"
                                    }
                                >
                                    {match (show_start, show_end) {
                                        (true, true) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (true, false) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (false, true) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                        (false, false) => view! {
                                            <Button
                                                color=color
                                                variant=variant
                                                radius=radius
                                                size=size
                                                is_disabled=is_disabled
                                                is_loading=is_loading
                                                loading_placement=loading_placement
                                                is_full_width=is_full_width
                                                schema_json=schema_json.clone().unwrap_or_default()
                                                aria_label=if icon_only { "Button".to_string() } else { String::new() }
                                            >
                                                {if icon_only { "★" } else { "Button" }}
                                            </Button>
                                        }
                                            .into_any(),
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=button_imports.clone()
            >
                <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                    <Button id="docs-button-matrix-idle".to_string()>"Idle"</Button>
                    <Button
                        id="docs-button-matrix-loading".to_string()
                        is_loading=true
                        loading_placement=ButtonLoadingPlacement::Start
                    >
                        "Loading"
                    </Button>
                    <Button id="docs-button-matrix-disabled".to_string() is_disabled=true>
                        "Disabled"
                    </Button>
                    <Button
                        id="docs-button-matrix-icon-only".to_string()
                        aria_label="Icon only".to_string()
                    >
                        "★"
                    </Button>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                code_signal=controlled_vs_uncontrolled_code
                code_imports=button_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "N/A: Button has no value/open axis. Caller-managed props/callbacks remain the only state boundary."
                    </span>
                    <div class="docs-row docs-row--wrap" style="gap: var(--ui-space-lg); align-items: flex-start;">
                        <Button id="docs-button-controlled-like".to_string() is_loading=true>
                            "Parent-managed loading"
                        </Button>
                        <Button id="docs-button-uncontrolled-like".to_string()>
                            "No internal state axis"
                        </Button>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                code_signal=output_mode_code
                code_imports=button_imports.clone()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="button-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "Button is not a text-reading surface; docs output stays snapshot (`fallback=snapshot`)."
                    </span>
                    <Button id="docs-button-snapshot".to_string()>"Snapshot"</Button>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                code_signal=source_first_code
                code_imports=button_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted" data-slot="button-source-first-contract">
                        "Copy-ready snippet auto-prepends imports. Feature gate: component-button + inject-css."
                    </span>
                    <Button
                        id="docs-button-source-first".to_string()
                        color=ButtonColor::Primary
                        variant=ButtonVariant::Solid
                    >
                        "Build"
                    </Button>
                    <p class="ui-muted" data-slot="button-source-paths">
                        "Source: components/button/src/view.rs and crates/ui/src/button/view.rs."
                    </p>
                </div>
            </Playground>

            <Playground title="Colors" code_signal=colors_code>
                <div class="docs-row">
                    <Button color="default">"Default"</Button>
                    <Button color="primary">"Primary"</Button>
                    <Button color="secondary">"Secondary"</Button>
                    <Button color="success">"Success"</Button>
                    <Button color="warning">"Warning"</Button>
                    <Button color="danger">"Danger"</Button>
                </div>
            </Playground>

            <Playground title="Radius" code_signal=radius_code>
                <div class="docs-row">
                    <Button radius="full" color="default">"Full"</Button>
                    <Button radius="lg" color="default">"Large"</Button>
                    <Button radius="md" color="default">"Medium"</Button>
                    <Button radius="sm" color="default">"Small"</Button>
                    <Button radius="none" color="default">"None"</Button>
                </div>
            </Playground>

            <Playground title="Sizes" code_signal=sizes_code>
                <div class="docs-row">
                    <Button size="xs">"XS"</Button>
                    <Button size="s">"S"</Button>
                    <Button size="m">"M"</Button>
                    <Button size="l">"L"</Button>
                    <Button size="xl">"XL"</Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
