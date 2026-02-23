use super::*;

pub(crate) fn flip_card() -> AnyView {
    let motion_options = vec![
        "default".to_string(),
        "gentle".to_string(),
        "dramatic".to_string(),
    ];
    let flip_card_imports =
        "use leptos::prelude::*;\nuse ui::{FlipCard, FlipCardMotion};".to_string();
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_default_is_flipped, set_workbench_default_is_flipped) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_flip_on_hover, set_workbench_is_flip_on_hover) = signal(true);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(true);
    let (controlled_is_flipped, set_controlled_is_flipped) = signal(false);
    let (controlled_toggle_count, set_controlled_toggle_count) = signal(0_u32);

    let workbench_motion =
        Signal::derive(move || match workbench_motion_index.get().unwrap_or(0) {
            1 => FlipCardMotion {
                hover_scale: 1.01,
                hover_tilt_deg: 2.0,
                ..FlipCardMotion::default()
            },
            2 => FlipCardMotion {
                hover_scale: 1.06,
                hover_tilt_deg: 7.5,
                ..FlipCardMotion::default()
            },
            _ => FlipCardMotion::default(),
        });

    let workbench_code = Signal::derive(move || {
        let default_is_flipped = workbench_default_is_flipped.get();
        let is_disabled = workbench_is_disabled.get();
        let is_flip_on_hover = workbench_is_flip_on_hover.get();
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let motion_index = workbench_motion_index.get().unwrap_or(0);
        let motion_name = match motion_index {
            1 => "gentle",
            2 => "dramatic",
            _ => "default",
        };
        let motion = workbench_motion.get();

        let mut lines = vec!["<FlipCard".to_string()];
        if default_is_flipped {
            lines.push("  default_is_flipped=true".to_string());
        }
        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if is_flip_on_hover {
            lines.push("  is_flip_on_hover=true".to_string());
        }
        if custom_id {
            lines.push("  id=\"docs-flip-card-workbench\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-flip-card-state\".into()".to_string());
        }
        if motion_index != 0 {
            lines.push(format!(
                "  motion=FlipCardMotion {{ hover_scale: {:.2}, hover_tilt_deg: {:.1}, ..FlipCardMotion::default() }}",
                motion.hover_scale,
                motion.hover_tilt_deg
            ));
        }
        lines.push("  front=move || view! { <div>\"Workbench front\"</div> }".to_string());
        lines.push("  back=move || view! { <div>\"Workbench back\"</div> }".to_string());
        lines.push("/>".to_string());
        lines.push(format!("// motion preset: {motion_name}"));

        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let default_is_flipped = workbench_default_is_flipped.get();
        let is_disabled = workbench_is_disabled.get();
        let is_flip_on_hover = workbench_is_flip_on_hover.get();
        let flip_mode = if is_flip_on_hover {
            "FlipCardFlipMode::Hover"
        } else {
            "FlipCardFlipMode::Toggle"
        };
        let lang = "en-US";
        let dir = A11yDirection::Ltr;
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let class_name = if custom_class {
            "docs-flip-card-state"
        } else {
            ""
        };
        let motion = workbench_motion.get();

        let mut classes = vec![
            "ui-flip-card".to_string(),
            if is_disabled {
                "ui-flip-card--disabled".to_string()
            } else {
                "ui-flip-card--enabled".to_string()
            },
            if default_is_flipped {
                "ui-flip-card--flipped".to_string()
            } else {
                "ui-flip-card--default".to_string()
            },
            if is_flip_on_hover {
                "ui-flip-card--hover".to_string()
            } else {
                "ui-flip-card--toggle".to_string()
            },
        ];
        if custom_class {
            classes.push("ui-flip-card--custom-class".to_string());
            classes.push("docs-flip-card-state".to_string());
        }
        if custom_id {
            classes.push("ui-flip-card--custom-id".to_string());
        }
        if motion != FlipCardMotion::default() {
            classes.push("ui-flip-card--custom-motion".to_string());
        }

        format!(
            "FlipCardActualConfig {{\n  default_is_flipped: {default_is_flipped},\n  default_flipped: {default_is_flipped},\n  flip_mode: {flip_mode},\n  class_name: {:?},\n  front: \"Workbench front\",\n  back: \"Workbench back\",\n  on_is_flipped_change: true,\n  lang: {lang:?},\n  dir: {:?},\n  is_disabled: {is_disabled},\n  is_flip_on_hover: {is_flip_on_hover},\n  custom_id: {custom_id},\n  custom_class: {custom_class},\n  motion: {{ hover_scale: {:.2}, hover_tilt_deg: {:.1} }},\n  class: \"{}\",\n  markers: [\"data-state\", \"data-visible\", \"data-flip-mode\", \"data-motion-source\", \"data-id-source\"],\n}}",
            class_name,
            dir,
            motion.hover_scale,
            motion.hover_tilt_deg,
            classes.join(" ")
        )
    });

    let flip_card_test_css_source = Signal::derive(move || {
        format!(
            "/* components/flip-card/src/styles.rs */\n{}",
            ui::flip_card::styles::CSS
        )
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<FlipCard front=... back=... />
<FlipCard is_flip_on_hover=true front=... back=... />
<FlipCard is_disabled=true front=... back=... />
<FlipCard motion=FlipCardMotion { hover_scale: 1.06, hover_tilt_deg: 7.5, ..FlipCardMotion::default() } front=... back=... />"#.to_string()
    });

    let hello_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Front"</div> }
  back=move || view! { <div>"Back"</div> }
/>"#
        .to_string()
    });

    let basic_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card-toggle".to_string()
  front=move || view! {
    <div class="ui-flip-card__title">"Front"</div>
    <div class="ui-flip-card__description">"Click or press Enter/Space to flip."</div>
  }
  back=move || view! {
    <div class="ui-flip-card__title">"Back"</div>
    <div class="ui-flip-card__description">"Stable state/data markers for docs and tests."</div>
  }
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card"
  class_name="docs-flip-card-state".to_string()
  is_flip_on_hover=true
  motion=FlipCardMotion {
    hover_scale: 1.03,
    hover_tilt_deg: 4.0,
    ..FlipCardMotion::default()
  }
  front=move || view! { <div>"Inspect markers (front)"</div> }
  back=move || view! { <div>"Inspect markers (back)"</div> }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card-disabled".to_string()
  is_disabled=true
  front=move || view! { <div>"Disabled front"</div> }
  back=move || view! { <div>"Disabled back"</div> }
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Uncontrolled front"</div> }
  back=move || view! { <div>"Uncontrolled back"</div> }
/>

let (is_flipped, set_is_flipped) = signal(false);
<FlipCard
  is_flipped=Signal::derive(move || is_flipped.get())
  on_is_flipped_change=Callback::new(move |next| set_is_flipped.set(next))
  front=move || view! { <div>"Controlled front"</div> }
  back=move || view! { <div>"Controlled back"</div> }
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Snapshot baseline"</div> }
  back=move || view! { <div>"Complete config renders in one pass."</div> }
/>
<FlipCard
  is_flip_on_hover=true
  front=move || view! { <div>"Streaming optional"</div> }
  back=move || view! { <div>"Fallback stays snapshot for FlipCard."</div> }
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<FlipCard
  is_flip_on_hover=true
  front=move || view! { <div>"Copy-ready front"</div> }
  back=move || view! { <div>"Copy-ready back"</div> }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FlipCard"
            slug="flip-card"
            group="Display"
            description="3D front/back card with baseline-style state/source markers and baseline-level spring motion for flip/hover interactions."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        default_flipped=false
                        flip_mode=ui::flip_card::FlipCardFlipMode::Toggle
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        front=move || view! { <div class="ui-flip-card__title">"Front"</div> }
                        back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=flip_card_imports.clone()
                test_css_source=flip_card_test_css_source
                test_source_path="components/flip-card/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调翻转初始态/hover/disabled/id/class/motion，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Motion preset"</div>
                            <SegmentedControl
                                id_base="docs-flip-card-motion".to_string()
                                options=motion_options.clone()
                                selected_index=workbench_motion_index
                                set_selected_index=set_workbench_motion_index
                                size=SegmentedControlSize::Sm
                                aria_label="FlipCard motion preset".to_string()
                            />
                            <Switch checked=workbench_default_is_flipped set_checked=set_workbench_default_is_flipped>
                                "Default Flipped"
                            </Switch>
                            <Switch checked=workbench_is_flip_on_hover set_checked=set_workbench_is_flip_on_hover>
                                "Flip On Hover"
                            </Switch>
                            <div data-slot="chart-workbench-toggle-disabled">
                                <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                    "Disabled"
                                </Switch>
                            </div>
                            <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                                "Custom ID"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let default_is_flipped = workbench_default_is_flipped.get();
                        let is_disabled = workbench_is_disabled.get();
                        let is_flip_on_hover = workbench_is_flip_on_hover.get();
                        let with_custom_class = workbench_custom_class.get();
                        let with_custom_id = workbench_custom_id.get();
                        let motion = workbench_motion.get();

                        match (with_custom_class, with_custom_id) {
                            (true, true) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (true, false) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, true) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, false) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                        }
                    }}
                    <div class="ui-muted">
                        "切换 settings 后，使用 Code / Test 面板查看实际配置与 scoped CSS 影响。"
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Hover / Disabled / Dramatic Motion)"
                code_signal=state_matrix_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipCard
                            front=move || view! { <div class="ui-flip-card__title">"Default"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            is_flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Hover flip"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                    <div class="docs-row">
                        <FlipCard
                            is_disabled=true
                            front=move || view! { <div class="ui-flip-card__title">"Disabled"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            motion=FlipCardMotion {
                                hover_scale: 1.06,
                                hover_tilt_deg: 7.5,
                                ..FlipCardMotion::default()
                            }
                            is_flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Dramatic motion"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Click + Keyboard Flip"
                code_signal=basic_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card-toggle".to_string()
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Front"</div>
                                    <div class="ui-flip-card__description">
                                        "Click or press Enter/Space to flip."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Back"</div>
                                    <div class="ui-flip-card__description">
                                        "Back face stays keyboard reachable with the same button semantics."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-flip-mode`, `data-class-source`, `data-motion-source`, `data-id-source`, and face-level visibility markers (`data-visible`/`data-hidden`)."
                code_signal=markers_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card".to_string()
                        class_name="docs-flip-card-state".to_string()
                        is_flip_on_hover=true
                        motion=FlipCardMotion {
                            hover_scale: 1.03,
                            hover_tilt_deg: 4.0,
                            ..FlipCardMotion::default()
                        }
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (front)"</div>
                                    <div class="ui-flip-card__description">
                                        "Hover enters flipped mode source = custom."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (back)"</div>
                                    <div class="ui-flip-card__description">
                                        "Front/back visibility markers stay explicit for regression tests."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                description="Compare default uncontrolled usage against externally managed `is_flipped + on_is_flipped_change`."
                code_signal=controlled_contrast_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <FlipCard
                            front=move || view! { <div class="ui-flip-card__title">"Uncontrolled front"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Uncontrolled back"</div> }
                        />
                        <FlipCard
                            is_flipped=Signal::derive(move || controlled_is_flipped.get())
                            on_is_flipped_change=Callback::new(move |next| {
                                set_controlled_is_flipped.set(next);
                                set_controlled_toggle_count.update(|count| *count += 1);
                            })
                            front=move || view! { <div class="ui-flip-card__title">"Controlled front"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Controlled back"</div> }
                        />
                    </div>
                    <div class="ui-muted">
                        {move || {
                            format!(
                                "Controlled state: is_flipped={}, on_is_flipped_change calls={}",
                                controlled_is_flipped.get(),
                                controlled_toggle_count.get(),
                            )
                        }}
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="flip-card-parameter-matrix">
                <h3>"Parameter Matrix (API + Defaults)"</h3>
                <ul data-slot="flip-card-parameter-rows">
                    <li>
                        <code>"is_flipped"</code>
                        " = controlled axis (default: uncontrolled when omitted)"
                    </li>
                    <li>
                        <code>"default_is_flipped"</code>
                        " / "
                        <code>"default_flipped"</code>
                        " = default priority "
                        <code>"default_is_flipped > default_flipped > DEFAULT_FLIPPED(false)"</code>
                    </li>
                    <li>
                        <code>"on_is_flipped_change"</code>
                        " = optional callback (default: none)"
                    </li>
                    <li>
                        <code>"is_disabled"</code>
                        " / "
                        <code>"disabled"</code>
                        " = default priority "
                        <code>"is_disabled > disabled > DEFAULT_DISABLED(false)"</code>
                    </li>
                    <li>
                        <code>"flip_mode"</code>
                        " / "
                        <code>"is_flip_on_hover"</code>
                        " / "
                        <code>"flip_on_hover"</code>
                        " = default priority "
                        <code>"flip_mode > is_flip_on_hover > flip_on_hover > DEFAULT_HOVER_FLIP(false)"</code>
                    </li>
                    <li>
                        <code>"id"</code>
                        " = custom id or "
                        <code>"IdProvider::next_prefixed_id(DEFAULT_ID_PREFIX)"</code>
                    </li>
                    <li>
                        <code>"class_name"</code>
                        " = optional custom class (default: none)"
                    </li>
                    <li>
                        <code>"motion"</code>
                        " = "
                        <code>"FlipCardMotion::default()"</code>
                    </li>
                </ul>
            </section>

            <Playground
                title="Streaming / Snapshot Contract"
                description="FlipCard is not an LLM body reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        front=move || view! { <div class="ui-flip-card__title">"Snapshot baseline"</div> }
                        back=move || view! { <div class="ui-flip-card__description">"Complete config renders in one pass."</div> }
                    />
                    <FlipCard
                        is_flip_on_hover=true
                        front=move || view! { <div class="ui-flip-card__title">"Streaming optional"</div> }
                        back=move || view! { <div class="ui-flip-card__description">"Fallback stays snapshot for FlipCard."</div> }
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=flip_card_imports.clone()
            >
                <FlipCard
                    is_flip_on_hover=true
                    front=move || view! { <div class="ui-flip-card__title">"Copy-ready front"</div> }
                    back=move || view! { <div class="ui-flip-card__title">"Copy-ready back"</div> }
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="flip-card-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                <p>
                    "Open "
                    <code>"Show code"</code>
                    " in any playground, then use the code block copy action. Copied snippets are auto-normalized by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " so required imports are included."
                </p>
                <p>"Real component sources:"</p>
                <ul data-slot="flip-card-source-paths">
                    <li><code>"components/flip-card/src/mod.rs"</code></li>
                    <li><code>"components/flip-card/src/logic.rs"</code></li>
                    <li><code>"components/flip-card/src/view.rs"</code></li>
                    <li><code>"components/flip-card/src/styles.rs"</code></li>
                    <li><code>"components/flip-card/src/motion.rs"</code></li>
                </ul>
                <p>"Dependency baseline (Cargo.toml):"</p>
                <pre data-slot="flip-card-source-first-deps">
                    <code>
                        "[dependencies]\nui = { default-features = false, features = [\"component-flip_card\", \"inject-css\"] }\n# Mount under UiRoot to inject base/theme/components CSS."
                    </code>
                </pre>
            </section>

            <Playground
                title="Disabled"
                code_signal=disabled_code
                code_imports=flip_card_imports
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card-disabled".to_string()
                        is_disabled=true
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled front"</div>
                                    <div class="ui-flip-card__description">
                                        "No click/keyboard toggle while disabled."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled back"</div>
                                    <div class="ui-flip-card__description">
                                        "aria-disabled and disabled markers remain consistent."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
