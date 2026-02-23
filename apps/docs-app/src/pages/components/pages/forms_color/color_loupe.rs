use super::*;

pub(crate) fn color_loupe() -> AnyView {
    let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm); background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);";
    let color_options = vec![
        "Amber".to_string(),
        "Emerald".to_string(),
        "Sky".to_string(),
        "Alpha".to_string(),
    ];
    let position_options = vec!["Start".to_string(), "Center".to_string(), "End".to_string()];
    let (color_index, set_color_index) = signal(Some(0_usize));
    let (position_index, set_position_index) = signal(Some(1_usize));
    let (is_open, set_is_open) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let hello_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorLoupe id_base="docs-color-loupe-hello".to_string() is_open=true />
</div>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"let surface_style = "position: relative; inline-size: 12rem; block-size: 7rem; border: 1px dashed color-mix(in oklch, var(--ui-border), transparent 24%); border-radius: var(--ui-radius-sm);";

<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-start".to_string()
    color="#f59e0b".to_string()
    is_open=true
    x_percent=18.0
    y_percent=74.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-center".to_string()
    color="#10b981".to_string()
    is_open=true
    x_percent=50.0
    y_percent=48.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-end".to_string()
    color="#3b82f6".to_string()
    is_open=true
    x_percent=82.0
    y_percent=24.0
  />
</div>"##.to_string()
    });

    let states_code = Signal::derive(move || {
        r##"<div style=surface_style>
  <ColorLoupe
    id_base="docs-color-loupe-disabled".to_string()
    color="#a78bfa".to_string()
    is_open=true
    is_disabled=true
    x_percent=32.0
    y_percent=58.0
  />
  <ColorLoupe
    id_base="docs-color-loupe-custom".to_string()
    color="rgba(56, 189, 248, 0.72)".to_string()
    is_open=true
    x_percent=72.0
    y_percent=36.0
    aria_label="Accent loupe".to_string()
    class_name="docs-color-loupe-custom".to_string()
  />
</div>"##
            .to_string()
    });

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r##"// ColorLoupe is snapshot-only: no internal mutable state axis.
// Controlled/uncontrolled state ownership is N/A.
<ColorLoupe
  id_base="docs-color-loupe-controlled".to_string()
  color="#10b981".to_string()
  is_open=true
  x_percent=50.0
  y_percent=48.0
/>
<ColorLoupe id_base="docs-color-loupe-uncontrolled-na".to_string() />"##
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<ColorLoupe
  id_base="docs-color-loupe-matrix-default".to_string()
/>
<ColorLoupe
  id_base="docs-color-loupe-matrix-open".to_string()
  color="#f59e0b".to_string()
  is_open=true
  x_percent=18.0
  y_percent=74.0
/>
<ColorLoupe
  id_base="docs-color-loupe-matrix-disabled".to_string()
  color="#a78bfa".to_string()
  is_open=true
  is_disabled=true
  x_percent=32.0
  y_percent=58.0
/>"##
            .to_string()
    });

    let output_mode_code = Signal::derive(move || {
        r##"// Streaming is optional; fallback is snapshot.
<div
  data-ui-streaming="optional"
  data-ui-fallback="snapshot"
  data-ui-output-state="snapshot"
>
  <ColorLoupe
    id_base="docs-color-loupe-snapshot".to_string()
    is_open=true
    output_state=ColorLoupeOutputState::Verified
  />
</div>"##
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let (color, color_label) = match color_index.get().unwrap_or(0) {
            0 => ("#f59e0b", "amber"),
            1 => ("#10b981", "emerald"),
            2 => ("#3b82f6", "sky"),
            _ => ("rgba(56, 189, 248, 0.72)", "alpha"),
        };
        let (x_percent, y_percent, position_label) = match position_index.get().unwrap_or(0) {
            0 => (18.0_f32, 74.0_f32, "start"),
            1 => (50.0_f32, 48.0_f32, "center"),
            _ => (82.0_f32, 24.0_f32, "end"),
        };
        let is_open_value = is_open.get();
        let is_disabled_value = is_disabled.get();
        let aria_label = if custom_aria.get() {
            "Workbench loupe"
        } else {
            ""
        };
        let class_name = if custom_class.get() {
            "docs-color-loupe-workbench"
        } else {
            ""
        };

        format!(
            "<ColorLoupe\n  id_base=\"docs-color-loupe-workbench\".into()\n  color=\"{color}\".into() // {color_label}\n  is_open={is_open_value}\n  is_disabled={is_disabled_value}\n  x_percent={x_percent}\n  y_percent={y_percent} // {position_label}\n  output_state=ColorLoupeOutputState::Verified\n  lang=\"en-US\".into()\n  dir=A11yDirection::Ltr\n  aria_label=\"{aria_label}\".into()\n  class_name=\"{class_name}\".into()\n/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let (color, color_label) = match color_index.get().unwrap_or(0) {
            0 => ("#f59e0b", "amber"),
            1 => ("#10b981", "emerald"),
            2 => ("#3b82f6", "sky"),
            _ => ("rgba(56, 189, 248, 0.72)", "alpha"),
        };
        let (x_percent, y_percent, position_label) = match position_index.get().unwrap_or(0) {
            0 => (18.0_f32, 74.0_f32, "start"),
            1 => (50.0_f32, 48.0_f32, "center"),
            _ => (82.0_f32, 24.0_f32, "end"),
        };
        let is_open_value = is_open.get();
        let is_disabled_value = is_disabled.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let aria_label = if custom_aria { "Workbench loupe" } else { "" };
        let class_name = if custom_class {
            "docs-color-loupe-workbench"
        } else {
            ""
        };

        format!(
            "ColorLoupeActualConfig {{\n  id_base: \"docs-color-loupe-workbench-main\",\n  color: \"{color}\" ({color_label}),\n  position: \"{position_label}\" ({x_percent:.1}, {y_percent:.1}),\n  is_open: {is_open_value},\n  is_disabled: {is_disabled_value},\n  output_state: ColorLoupeOutputState::Verified,\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  aria_label: \"{aria_label}\",\n  class_name: \"{class_name}\",\n}}"
        )
    });

    view! {
        <ComponentPage
            title="ColorLoupe"
            slug="color-loupe"
            group="Forms"
            description="baseline-compatible color loupe overlay primitive with centralized is_open/is_disabled/position normalization, checkerboard alpha preview, and stable slot/data-state contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div style=surface_style>
                    <ColorLoupe id_base="docs-color-loupe-hello".to_string() is_open=true />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_source_path="components/color-loupe/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="color-loupe-workbench-controls">
                        <div data-slot="color-loupe-workbench-color">
                            <div class="docs-search__label">"Color"</div>
                            <SegmentedControl
                                id_base="docs-color-loupe-workbench-color".to_string()
                                options=color_options.clone()
                                selected_index=color_index
                                set_selected_index=set_color_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorLoupe workbench color".to_string()
                            />
                        </div>

                        <div data-slot="color-loupe-workbench-position">
                            <div class="docs-search__label">"Position bucket"</div>
                            <SegmentedControl
                                id_base="docs-color-loupe-workbench-position".to_string()
                                options=position_options.clone()
                                selected_index=position_index
                                set_selected_index=set_position_index
                                size=SegmentedControlSize::Sm
                                aria_label="ColorLoupe workbench position".to_string()
                            />
                        </div>

                        <div data-slot="color-loupe-workbench-open">
                            <Switch checked=is_open set_checked=set_is_open>"Open"</Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-disabled">
                            <Switch checked=is_disabled set_checked=set_is_disabled>"Disabled"</Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-custom-aria">
                            <Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria_label"
                            </Switch>
                        </div>
                        <div data-slot="color-loupe-workbench-custom-class">
                            <Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </Switch>
                        </div>
                    </div>
                }
            >
                {move || {
                    let color = match color_index.get().unwrap_or(0) {
                        0 => "#f59e0b".to_string(),
                        1 => "#10b981".to_string(),
                        2 => "#3b82f6".to_string(),
                        _ => "rgba(56, 189, 248, 0.72)".to_string(),
                    };
                    let (x_percent, y_percent) = match position_index.get().unwrap_or(0) {
                        0 => (18.0_f32, 74.0_f32),
                        1 => (50.0_f32, 48.0_f32),
                        _ => (82.0_f32, 24.0_f32),
                    };
                    let is_open_value = is_open.get();
                    let is_disabled_value = is_disabled.get();
                    let aria_label = if custom_aria.get() {
                        "Workbench loupe".to_string()
                    } else {
                        "".to_string()
                    };
                    let class_name = if custom_class.get() {
                        "docs-color-loupe-workbench".to_string()
                    } else {
                        "".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-loupe-workbench-canvas">
                            <div style=surface_style data-slot="color-loupe-workbench-surface">
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-main".to_string()
                                    color=color
                                    is_open=is_open_value
                                    is_disabled=is_disabled_value
                                    x_percent=x_percent
                                    y_percent=y_percent
                                    output_state=ColorLoupeOutputState::Verified
                                    lang="en-US".to_string()
                                    dir=A11yDirection::Ltr
                                    aria_label=aria_label
                                    class_name=class_name
                                />
                                <ColorLoupe
                                    id_base="docs-color-loupe-workbench-compare".to_string()
                                    color="#3b82f6".to_string()
                                    is_open=true
                                    x_percent=82.0
                                    y_percent=24.0
                                    output_state=ColorLoupeOutputState::Verified
                                    lang="en-US".to_string()
                                    dir=A11yDirection::Ltr
                                    aria_label="Comparison loupe".to_string()
                                />
                            </div>
                            <span class="ui-muted">
                                "左侧可调，右侧固定对照（blue/end/is_open）。"
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Default / Open / Disabled)" code_signal=state_matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">"default / open / disabled"</span>
                    <div style=surface_style>
                        <ColorLoupe id_base="docs-color-loupe-matrix-default-after-workbench".to_string() />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-open-after-workbench".to_string()
                            color="#f59e0b".to_string()
                            is_open=true
                            x_percent=18.0
                            y_percent=74.0
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-disabled-after-workbench".to_string()
                            color="#a78bfa".to_string()
                            is_open=true
                            is_disabled=true
                            x_percent=32.0
                            y_percent=58.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Open + Position Buckets" code_signal=basic_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-start".to_string()
                        color="#f59e0b".to_string()
                        is_open=true
                        x_percent=18.0
                        y_percent=74.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-center".to_string()
                        color="#10b981".to_string()
                        is_open=true
                        x_percent=50.0
                        y_percent=48.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-end".to_string()
                        color="#3b82f6".to_string()
                        is_open=true
                        x_percent=82.0
                        y_percent=24.0
                    />
                </div>
            </Playground>

            // title="State Matrix"
            <Playground title="Disabled + Custom Label + Custom Class" code_signal=states_code>
                <div style=surface_style>
                    <ColorLoupe
                        id_base="docs-color-loupe-disabled".to_string()
                        color="#a78bfa".to_string()
                        is_open=true
                        is_disabled=true
                        x_percent=32.0
                        y_percent=58.0
                    />
                    <ColorLoupe
                        id_base="docs-color-loupe-custom".to_string()
                        color="rgba(56, 189, 248, 0.72)".to_string()
                        is_open=true
                        x_percent=72.0
                        y_percent=36.0
                        aria_label="Accent loupe".to_string()
                        class_name="docs-color-loupe-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled（N/A）"
                code_signal=controlled_vs_uncontrolled_code
            >
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">
                        "ColorLoupe 是 snapshot 展示组件，不持有内部可变状态轴；受控/非受控切换语义 N/A。"
                    </span>
                    <div style=surface_style>
                        <ColorLoupe
                            id_base="docs-color-loupe-controlled".to_string()
                            color="#10b981".to_string()
                            is_open=true
                            x_percent=50.0
                            y_percent=48.0
                        />
                        <ColorLoupe id_base="docs-color-loupe-uncontrolled-na".to_string() />
                    </div>
                </div>
            </Playground>

            <Playground title="Baseline States" code_signal=state_matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <span class="ui-muted">"default / open / disabled"</span>
                    <div style=surface_style>
                        <ColorLoupe id_base="docs-color-loupe-matrix-default".to_string() />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-open".to_string()
                            color="#f59e0b".to_string()
                            is_open=true
                            x_percent=18.0
                            y_percent=74.0
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-matrix-disabled".to_string()
                            color="#a78bfa".to_string()
                            is_open=true
                            is_disabled=true
                            x_percent=32.0
                            y_percent=58.0
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Streaming Optional / Snapshot" code_signal=output_mode_code>
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="color-loupe-output-mode"
                    data-ui-streaming="optional"
                    data-ui-fallback="snapshot"
                    data-ui-output-state="snapshot"
                >
                    <span class="ui-muted">
                        "ColorLoupe is not a text-reading surface; docs output mode stays snapshot (`fallback=snapshot`)."
                    </span>
                    <div style=surface_style>
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-draft".to_string()
                            color="#f59e0b".to_string()
                            is_open=true
                            x_percent=18.0
                            y_percent=74.0
                            output_state=ColorLoupeOutputState::Draft
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-verified".to_string()
                            color="#10b981".to_string()
                            is_open=true
                            x_percent=50.0
                            y_percent=48.0
                            output_state=ColorLoupeOutputState::Verified
                        />
                        <ColorLoupe
                            id_base="docs-color-loupe-snapshot-committable".to_string()
                            color="#3b82f6".to_string()
                            is_open=true
                            x_percent=82.0
                            y_percent=24.0
                            output_state=ColorLoupeOutputState::Committable
                        />
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="color-loupe-copy-ready">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Dependency prerequisites: enable "
                    <code>"ui features: component-color_loupe + inject-css"</code>
                    " and render inside "
                    <code>"UiRoot"</code>
                    " so copied snippets keep theme vars/components css injection."
                </p>
                <p>
                    "Source-first path: "
                    <code>"components/color-loupe/src/view.rs"</code>
                    ", "
                    <code>"components/color-loupe/src/logic.rs"</code>
                    ", "
                    <code>"components/color-loupe/src/styles.rs"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
