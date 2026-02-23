use super::*;

pub(crate) fn popover() -> AnyView {
    use leptos::html;

    let minimal_anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());

    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let toggle: OnPress = Callback::new(move |_| set_open_raw.update(|v| *v = !*v));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let custom_anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let (custom_open_raw, set_custom_open_raw) = signal(false);
    let custom_open: Signal<bool> = Signal::derive(move || custom_open_raw.get());

    let (custom_present, set_custom_present) = signal(custom_open.get_untracked());
    Effect::new(move |_| {
        if custom_open.get() {
            set_custom_present.set(true);
        }
    });

    let close_custom: OnPress = Callback::new(move |_| set_custom_open_raw.set(false));
    let toggle_custom: OnPress = Callback::new(move |_| set_custom_open_raw.update(|v| *v = !*v));
    let on_custom_exit_complete = Callback::new(move |_| set_custom_present.set(false));

    let custom_motion = PopoverMotion {
        initial_scale: 0.95,
        offset_y_px: 12.0,
        ..PopoverMotion::default()
    };

    let minimal_code = Signal::derive(move || {
        r#"let anchor_ref: NodeRef<html::Button> = NodeRef::new();
<Button node_ref=anchor_ref>"Anchor"</Button>
<Popover anchor_ref=anchor_ref default_open=true>
  {move || view! { <div>"Popover content"</div> }}
</Popover>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let anchor_ref: NodeRef<html::Button> = NodeRef::new();
let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let (present, set_present) = signal(open.get_untracked());
let toggle: OnPress = Callback::new(move |_| set_open_raw.update(|value| *value = !*value));
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| set_present.set(false));

<Button node_ref=anchor_ref on_press=toggle>"Open"</Button>
<Show when=present>
  <Popover open=open anchor_ref=anchor_ref on_close=close on_exit_complete=finish_exit>
    ...
  </Popover>
</Show>"#
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r#"let anchor_ref: NodeRef<html::Button> = NodeRef::new();
let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});
let custom_motion = PopoverMotion {
  initial_scale: 0.95,
  offset_y_px: 12.0,
  ..PopoverMotion::default()
};

<Popover
  open=Signal::derive(move || open_raw.get())
  anchor_ref=anchor_ref
  on_close=close
  motion=custom_motion
  is_modal=false
  class_name="docs-popover-state".to_string()
  on_exit_complete=finish_exit
>
  ...
</Popover>"#
            .to_string()
    });

    let workbench_anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });
    let workbench_close: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let workbench_toggle: OnPress =
        Callback::new(move |_| set_workbench_open_raw.update(|value| *value = !*value));
    let workbench_on_exit_complete = Callback::new(move |_| set_workbench_present.set(false));

    let (workbench_modal, set_workbench_modal) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_scale_pct, set_workbench_scale_pct) = signal(98_u16);
    let (workbench_offset_px, set_workbench_offset_px) = signal(6_u16);

    let workbench_motion = Signal::derive(move || PopoverMotion {
        initial_scale: f64::from(workbench_scale_pct.get()) / 100.0,
        offset_y_px: f64::from(workbench_offset_px.get()),
        ..PopoverMotion::default()
    });

    let workbench_code = Signal::derive(move || {
        let is_modal = workbench_modal.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();

        let mut lines = vec![
            "let anchor_ref: NodeRef<html::Button> = NodeRef::new();".to_string(),
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let close: OnPress = Callback::new(move |_| set_open_raw.set(false));".to_string(),
            "let on_exit_complete = Callback::new(move |_| {});".to_string(),
            "let custom_motion = PopoverMotion {".to_string(),
            format!("  initial_scale: {:.2},", motion.initial_scale),
            format!("  offset_y_px: {:.1},", motion.offset_y_px),
            "  ..PopoverMotion::default()".to_string(),
            "};".to_string(),
            "".to_string(),
            "<Popover".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  anchor_ref=anchor_ref".to_string(),
            "  on_close=close".to_string(),
            "  placement=PopoverPlacement::BottomStart".to_string(),
            "  motion=custom_motion".to_string(),
        ];
        if !is_modal {
            lines.push("  is_modal=false".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-popover-workbench\".into()".to_string());
        }
        lines.push("  on_exit_complete=on_exit_complete".to_string());
        lines.push(">".to_string());
        lines.push("  ...".to_string());
        lines.push("</Popover>".to_string());
        lines.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/popover/styles.rs */\n{}",
            ui::popover::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let is_modal = workbench_modal.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();
        let has_custom_motion = motion != PopoverMotion::default();
        let is_open = workbench_open_raw.get();

        let mut root_class = vec!["ui-popover".to_string()];
        if has_custom_motion {
            root_class.push("ui-popover--custom-motion".to_string());
        }
        if !is_modal {
            root_class.push("ui-popover--non-modal".to_string());
            root_class.push("ui-popover--custom-modal".to_string());
        }
        root_class.push("ui-popover--custom-exit".to_string());
        if custom_class {
            root_class.push("ui-popover--custom-class".to_string());
            root_class.push("docs-popover-workbench".to_string());
        }

        format!(
            "PopoverWorkbenchConfig {{\n  open: {is_open},\n  anchor_ref: \"workbench_anchor_ref\",\n  on_close: \"workbench_close\",\n  placement: PopoverPlacement::BottomStart,\n  motion: PopoverMotion {{ initial_scale: {:.2}, offset_y_px: {:.1}, ..PopoverMotion::default() }},\n  is_modal: {is_modal},\n  class_name: {},\n  on_exit_complete: \"workbench_on_exit_complete\",\n  modal: {is_modal},\n  custom_class: {custom_class},\n  state_attr: \"{}\",\n  modal_attr: \"{}\",\n  motion_source: \"{}\",\n  placement_source: \"default\",\n  modal_source: \"{}\",\n  class_source: \"{}\",\n  exit_source: \"custom\",\n  root_class: \"{}\",\n}}",
            motion.initial_scale,
            motion.offset_y_px,
            if custom_class {
                "Some(\"docs-popover-workbench\")"
            } else {
                "None"
            },
            if is_open { "open" } else { "closed" },
            if is_modal { "modal" } else { "non-modal" },
            if has_custom_motion {
                "custom"
            } else {
                "default"
            },
            if is_modal { "default" } else { "custom" },
            if custom_class { "custom" } else { "default" },
            root_class.join(" "),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Popover open=default_open anchor_ref=default_anchor on_close=dismiss_default on_exit_complete=on_exit_default />
<Popover open=custom_open anchor_ref=custom_anchor placement=PopoverPlacement::TopEnd on_close=dismiss_custom is_modal=false motion=custom_motion class_name="docs-popover-state".to_string() on_exit_complete=on_exit_custom />
<Popover open=workbench_open anchor_ref=workbench_anchor placement=PopoverPlacement::BottomStart on_close=dismiss_workbench motion=workbench_motion is_modal=workbench_modal on_exit_complete=on_exit_workbench />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Popover"
            slug="popover"
            group="Overlays"
            description="Positioned portal panel anchored to a trigger with baseline-style state markers and baseline-level spring motion contract. Requires presence to unmount after exit."
        >
            <Playground title="Hello World (Minimal API)" code_signal=minimal_code>
                <div class="docs-row">
                    <Button node_ref=minimal_anchor_ref>"Anchor"</Button>
                </div>

                <Popover anchor_ref=minimal_anchor_ref default_open=true>
                    <div class="docs-stack docs-stack--tight">
                        <div>"Popover content"</div>
                    </div>
                </Popover>
            </Playground>

            <Playground title="Controlled + Presence (Advanced)" code_signal=controlled_code>
                <div class="docs-row">
                    <Button node_ref=anchor_ref on_press=toggle aria_haspopup="dialog" aria_expanded=open>
                        {move || if open_raw.get() { "Close popover" } else { "Open popover" }}
                    </Button>
                </div>

                <Show when=move || present.get()>
                    <Popover
                        open=open
                        anchor_ref=anchor_ref
                        placement=ui_headless::PopoverPlacement::BottomStart
                        on_close=on_close
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Popover content"</div>
                            <div class="ui-muted">"Positioned via anchor rect + CSS vars."</div>
                            <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                        </div>
                    </Popover>
                </Show>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for popover open/modal/motion/class contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/popover/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" attr:data-slot="popover-workbench-controls">
                        <label class="docs-search__label">
                            "Initial scale (" {move || format!("{:.2}", f64::from(workbench_scale_pct.get()) / 100.0)} ")"
                            <input
                                type="range"
                                min="70"
                                max="120"
                                step="1"
                                prop:value=move || workbench_scale_pct.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(98)
                                        .clamp(70, 120);
                                    set_workbench_scale_pct.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            "Offset px (" {move || workbench_offset_px.get()} ")"
                            <input
                                type="range"
                                min="0"
                                max="48"
                                step="1"
                                prop:value=move || workbench_offset_px.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(6)
                                        .clamp(0, 48);
                                    set_workbench_offset_px.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_modal.get()
                                on:change=move |ev| set_workbench_modal.set(event_target_checked(&ev))
                            />
                            " Modal"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| {
                                    set_workbench_custom_class.set(event_target_checked(&ev))
                                }
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <Button
                        node_ref=workbench_anchor_ref
                        on_press=workbench_toggle
                        aria_haspopup="dialog"
                        aria_expanded=workbench_open
                    >
                        {move || {
                            if workbench_open_raw.get() {
                                "Close workbench popover"
                            } else {
                                "Open workbench popover"
                            }
                        }}
                    </Button>
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                    </span>
                </div>

                <Show when=move || workbench_present.get()>
                    {move || {
                        let motion = workbench_motion.get();
                        let is_modal = workbench_modal.get();
                        if workbench_custom_class.get() {
                            view! {
                                <Popover
                                    open=workbench_open
                                    anchor_ref=workbench_anchor_ref
                                    placement=ui_headless::PopoverPlacement::BottomStart
                                    on_close=workbench_close
                                    motion=motion
                                    is_modal=is_modal
                                    class_name="docs-popover-workbench".to_string()
                                    on_exit_complete=workbench_on_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench popover content"</div>
                                        <div class="ui-muted">
                                            "Tune modal + motion + class source and inspect config/test panels."
                                        </div>
                                        <Button variant=ButtonVariant::Secondary on_press=workbench_close>
                                            "Close"
                                        </Button>
                                    </div>
                                </Popover>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Popover
                                    open=workbench_open
                                    anchor_ref=workbench_anchor_ref
                                    placement=ui_headless::PopoverPlacement::BottomStart
                                    on_close=workbench_close
                                    motion=motion
                                    is_modal=is_modal
                                    on_exit_complete=workbench_on_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"Workbench popover content"</div>
                                        <div class="ui-muted">
                                            "Tune modal + motion + class source and inspect config/test panels."
                                        </div>
                                        <Button variant=ButtonVariant::Secondary on_press=workbench_close>
                                            "Close"
                                        </Button>
                                    </div>
                                </Popover>
                            }
                            .into_any()
                        }
                    }}
                </Show>
            </Playground>

            <Playground
                title="State Matrix (Default / TopEnd / Workbench)"
                code_signal=matrix_code
            >
                <div class="docs-stack docs-stack--tight" data-slot="popover-state-matrix">
                    <div class="docs-row">
                        <Button node_ref=anchor_ref on_press=toggle>
                            "Default popover"
                        </Button>
                        <Button node_ref=custom_anchor_ref on_press=toggle_custom>
                            "TopEnd non-modal"
                        </Button>
                        <Button node_ref=workbench_anchor_ref on_press=workbench_toggle>
                            "Workbench variant"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "default/open: " {move || open_raw.get()}
                        " · custom/open: " {move || custom_open_raw.get()}
                        " · workbench/open: " {move || workbench_open_raw.get()}
                    </span>

                    <Show when=move || present.get()>
                        <Popover
                            open=open
                            anchor_ref=anchor_ref
                            placement=ui_headless::PopoverPlacement::BottomStart
                            on_close=on_close
                            on_exit_complete=on_exit_complete
                        >
                            <div class="docs-stack docs-stack--tight">
                                <strong>"Default"</strong>
                                <div class="ui-muted">"BottomStart + modal"</div>
                            </div>
                        </Popover>
                    </Show>
                    <Show when=move || custom_present.get()>
                        <Popover
                            open=custom_open
                            anchor_ref=custom_anchor_ref
                            placement=ui_headless::PopoverPlacement::TopEnd
                            on_close=close_custom
                            motion=custom_motion
                            is_modal=false
                            class_name="docs-popover-state".to_string()
                            on_exit_complete=on_custom_exit_complete
                        >
                            <div class="docs-stack docs-stack--tight">
                                <strong>"TopEnd / Non-modal"</strong>
                                <div class="ui-muted">"custom motion + class"</div>
                            </div>
                        </Popover>
                    </Show>
                    <Show when=move || workbench_present.get()>
                        <Popover
                            open=workbench_open
                            anchor_ref=workbench_anchor_ref
                            placement=ui_headless::PopoverPlacement::BottomStart
                            on_close=workbench_close
                            motion=workbench_motion.get()
                            is_modal=workbench_modal.get()
                            on_exit_complete=workbench_on_exit_complete
                        >
                            <div class="docs-stack docs-stack--tight">
                                <strong>"Workbench"</strong>
                                <div class="ui-muted">"switch modal + motion via workbench controls"</div>
                            </div>
                        </Popover>
                    </Show>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-modal`, `data-motion-source`, `data-placement-source`, `data-modal-source`, and `data-exit-source` contracts."
                code_signal=motion_code
            >
                <div class="docs-row">
                    <Button
                        node_ref=custom_anchor_ref
                        on_press=toggle_custom
                        aria_haspopup="dialog"
                        aria_expanded=custom_open
                    >
                        {move || {
                            if custom_open_raw.get() {
                                "Close custom popover"
                            } else {
                                "Open custom popover"
                            }
                        }}
                    </Button>
                </div>

                <Show when=move || custom_present.get()>
                    <Popover
                        open=custom_open
                        anchor_ref=custom_anchor_ref
                        placement=ui_headless::PopoverPlacement::TopEnd
                        on_close=close_custom
                        motion=custom_motion
                        is_modal=false
                        class_name="docs-popover-state".to_string()
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Custom spring-like popover motion"</div>
                            <div class="ui-muted">
                                "Inspect `data-modal-source`/`data-placement-source` while tuning PopoverMotion."
                            </div>
                            <Button variant=ButtonVariant::Secondary on_press=close_custom>
                                "Close"
                            </Button>
                        </div>
                    </Popover>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
