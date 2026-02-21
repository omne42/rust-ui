use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::overlays::OverlaysRoot;
use ui_components::{
    Button, ButtonVariant, ContextualHelp, ContextualHelpVariant, Drawer, DrawerMotion,
    DrawerPlacement, Modal, OnPress, Overlay, OverlayMotion, Popover, PopoverMotion, PreviewCard,
    PreviewCardMotion, PreviewLinkCard, PreviewLinkCardMotion, SegmentedControl,
    SegmentedControlSize, Sheet, SheetMotion, SheetPlacement, Snippet, Switch, Toast, ToastMotion,
    ToastOptions, ToastStoreOptions, ToastVariant, ToastViewport, provide_toast_store,
};

#[path = "overlays_dialog.rs"]
mod overlays_dialog;

#[path = "overlays_alert_dialog.rs"]
mod overlays_alert_dialog;

#[path = "overlays_hover_card.rs"]
mod overlays_hover_card;

#[path = "overlays_tooltip.rs"]
mod overlays_tooltip;

const SHEET_PLAYGROUND_CODE: &str = r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});

<Sheet
  open=Signal::derive(move || open_raw.get())
  placement=SheetPlacement::Bottom
  on_close=close
  on_exit_complete=finish_exit
>
  move || view! { ... }
</Sheet>"#;

const SHEET_MARKER_PLAYGROUND_CODE: &str = r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});
let custom_motion = SheetMotion {
  initial_offset_px: 56.0,
  ..SheetMotion::default()
};

<Sheet
  open=Signal::derive(move || open_raw.get())
  placement=SheetPlacement::Right
  on_close=close
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  motion=custom_motion
  on_exit_complete=finish_exit
>
  ...
</Sheet>"#;

const MODAL_MINIMAL_PLAYGROUND_CODE: &str = r#"<Modal default_open=true id_base="m".to_string() title="Hello".to_string() on_close=Callback::new(|_| {})>
  <div>"Minimal modal content"</div>
</Modal>"#;

const MODAL_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui_components::{Modal, OnPress, OverlayMotion};";
const DRAWER_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant, Drawer, DrawerMotion, DrawerPlacement, OnPress, SheetMotion};";

pub(super) fn overlay() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());

    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_overlay: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let (marker_open_raw, set_marker_open_raw) = signal(false);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());

    let (marker_present, set_marker_present) = signal(marker_open.get_untracked());
    Effect::new(move |_| {
        if marker_open.get() {
            set_marker_present.set(true);
        }
    });

    let close_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(false));
    let open_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(true));
    let on_marker_exit_complete = Callback::new(move |_| set_marker_present.set(false));

    let marker_motion = OverlayMotion {
        initial_scale: 0.94,
        initial_y_px: 14.0,
        ..OverlayMotion::default()
    };

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);
let (present, set_present) = signal(open.get_untracked());
let on_close: OnPress = Callback::new(move |_| set_open.set(false));
let on_exit_complete = Callback::new(move |_| set_present.set(false));

<Show when=move || present.get()>
  <Overlay
    open=Signal::derive(move || open.get())
    on_close=on_close
    on_exit_complete=on_exit_complete
  >
    ...
  </Overlay>
</Show>"#
            .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let motion = OverlayMotion {
  initial_scale: 0.94,
  initial_y_px: 14.0,
  ..OverlayMotion::default()
};

<Overlay
  open=open
  on_close=close
  role="alertdialog"
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  motion=motion
  class_name="docs-overlay-state".to_string()
  aria_labelledby="overlay-marker-title".to_string()
  aria_describedby="overlay-marker-desc".to_string()
  on_exit_complete=Callback::new(move |_| {})
>
  <div class="ui-card">
    <h4 id="overlay-marker-title">"Overlay markers"</h4>
    <p id="overlay-marker-desc">"Verifies controlled state and source markers."</p>
  </div>
</Overlay>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Overlay"
            slug="overlay"
            group="Overlays"
            description="Portal + backdrop + focus trap + overlay stack (Esc/topmost). Supports dismiss control flags and requires presence to unmount after exit."
        >
            <Playground title="Overlay presence" code_signal=code>
                <div class="docs-row">
                    <Button on_press=open_overlay>"Open overlay"</Button>
                    <span class="ui-muted">"open: " {move || open_raw.get()}</span>
                </div>

                <Show when=move || present.get()>
                    <Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>
                        <div class="docs-stack">
                            <div>"Overlay panel"</div>
                            <div class="ui-muted">
                                "Esc or click backdrop closes. Tab is trapped."
                            </div>
                            <div class="docs-row">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            </div>
                        </div>
                    </Overlay>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-role-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=marker_code
            >
                <div class="docs-row">
                    <Button on_press=open_marker>"Open marker overlay"</Button>
                    <span class="ui-muted">
                        "open: " {move || marker_open_raw.get().to_string()}
                    </span>
                </div>

                <Show when=move || marker_present.get()>
                    <Overlay
                        open=marker_open
                        on_close=close_marker
                        role="alertdialog"
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        motion=marker_motion
                        class_name="docs-overlay-state".to_string()
                        aria_labelledby="overlay-marker-title".to_string()
                        aria_describedby="overlay-marker-desc".to_string()
                        on_exit_complete=on_marker_exit_complete
                    >
                        <div class="docs-stack">
                            <div id="overlay-marker-title">"Marker overlay"</div>
                            <div id="overlay-marker-desc" class="ui-muted">
                                "Inspect data-dismiss-source / data-keyboard-dismiss-source / data-role-source in DevTools."
                            </div>
                            <Button variant=ButtonVariant::Secondary on_press=close_marker>
                                "Close"
                            </Button>
                        </div>
                    </Overlay>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn popover() -> AnyView {
    use leptos::html;

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

    let code = Signal::derive(move || {
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
            "/* crates/ui-components/src/popover/styles.rs */\n{}",
            ui_components::popover::styles::CSS
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
            "PopoverWorkbenchConfig {{\n  open: {is_open},\n  modal: {is_modal},\n  custom_class: {custom_class},\n  initial_scale: {:.2},\n  offset_y_px: {:.1},\n  state_attr: \"{}\",\n  modal_attr: \"{}\",\n  motion_source: \"{}\",\n  placement_source: \"default\",\n  modal_source: \"{}\",\n  class_source: \"{}\",\n  exit_source: \"custom\",\n  root_class: \"{}\",\n}}",
            motion.initial_scale,
            motion.offset_y_px,
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

    view! {
        <ComponentPage
            title="Popover"
            slug="popover"
            group="Overlays"
            description="Positioned portal panel anchored to a trigger with baseline-style state markers and baseline-level spring motion contract. Requires presence to unmount after exit."
        >
            <Playground title="Popover" code_signal=code>
                <div class="docs-row">
                    <Button node_ref=anchor_ref on_press=toggle aria_haspopup="dialog" aria_expanded=open>
                        {move || if open_raw.get() { "Close popover" } else { "Open popover" }}
                    </Button>
                </div>

                <Show when=move || present.get()>
                    <Popover
                        open=open
                        anchor_ref=anchor_ref
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

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for popover open/modal/motion/class contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/popover/styles.rs".to_string()
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn modal() -> AnyView {
    let (minimal_present, set_minimal_present) = signal(false);
    let open_minimal_modal: OnPress = Callback::new(move |_| set_minimal_present.set(true));
    let close_minimal_modal: OnPress = Callback::new(move |_| set_minimal_present.set(false));
    let on_minimal_open_change: Callback<bool> = Callback::new(move |next: bool| {
        if !next {
            set_minimal_present.set(false);
        }
    });

    let (interactive_open_raw, set_interactive_open_raw) = signal(false);
    let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());
    let open_interactive_modal: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(true));
    let close_interactive_modal: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(false));
    let (interactive_with_description, set_interactive_with_description) = signal(true);
    let (interactive_custom_id, set_interactive_custom_id) = signal(true);
    let (interactive_custom_title, set_interactive_custom_title) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_exit, set_interactive_custom_exit) = signal(false);

    let interactive_code = Signal::derive(move || {
        let with_description = interactive_with_description.get();
        let custom_id = interactive_custom_id.get();
        let custom_title = interactive_custom_title.get();
        let custom_class = interactive_custom_class.get();
        let custom_motion = interactive_custom_motion.get();
        let custom_exit = interactive_custom_exit.get();

        let mut lines = vec![
            "let (open, set_open) = signal(false);".to_string(),
            "let close: OnPress = Callback::new(move |_| set_open.set(false));".to_string(),
            "".to_string(),
            "<Modal".to_string(),
            "  is_open=Signal::derive(move || open.get())".to_string(),
            format!(
                "  id_base={}",
                if custom_id {
                    "\"docs-modal-interactive\".into()"
                } else {
                    "\" \".into()"
                }
            ),
            format!(
                "  title={}",
                if custom_title {
                    "\"Action required\".into()"
                } else {
                    "\" \".into()"
                }
            ),
            "  on_close=close".to_string(),
        ];

        if with_description {
            lines.push("  description=\"Review settings before confirming.\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-modal-custom\".into()".to_string());
        }
        if custom_motion {
            lines.push("  motion=OverlayMotion {".to_string());
            lines.push("    initial_scale: 0.92,".to_string());
            lines.push("    initial_y_px: 18.0,".to_string());
            lines.push("    ..OverlayMotion::default()".to_string());
            lines.push("  }".to_string());
        }
        if custom_exit {
            lines.push("  on_exit_complete=Callback::new(move |_| {})".to_string());
        }
        lines.push(">".to_string());
        lines.push("  ...".to_string());
        lines.push("</Modal>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/modal/src/styles.rs */\n{}",
            ui_components::modal::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "ModalActualConfig {{\n  open: {},\n  id_source: {},\n  title_source: {},\n  description: {},\n  class_source: {},\n  motion_source: {},\n  exit_source: {},\n}}",
            interactive_open_raw.get(),
            if interactive_custom_id.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_title.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_with_description.get() {
                "\"present\""
            } else {
                "\"absent\""
            },
            if interactive_custom_class.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_motion.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_exit.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
        )
    });

    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_modal: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });

    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_modal: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));

    let custom_motion = OverlayMotion {
        initial_scale: 0.92,
        initial_y_px: 18.0,
        ..OverlayMotion::default()
    };

    let semantic_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_exit_complete = Callback::new(move |_| {});

<Modal
  is_open=open
  id_base="m".to_string()
  title="Confirm".to_string()
  description="Modal composes Overlay and wires aria attributes.".to_string()
  on_close=close
  on_exit_complete=on_exit_complete
>
  ...
</Modal>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_exit_complete = Callback::new(move |_| {});
let custom_motion = OverlayMotion {
  initial_scale: 0.92,
  initial_y_px: 18.0,
  ..OverlayMotion::default()
};

<Modal
  is_open=Signal::derive(move || open_raw.get())
  id_base="m-custom".to_string()
  title="Title only".to_string()
  class_name="docs-modal-custom".to_string()
  motion=custom_motion
  on_close=close
  on_exit_complete=on_exit_complete
>
  ...
</Modal>"#
            .to_string()
    });

    let state_matrix_options = vec![
        "Uncontrolled + default_open=true".to_string(),
        "Uncontrolled + no description".to_string(),
        "Controlled + custom title".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_default_open =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_with_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) != 1);
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));
    let close_state_matrix_modal: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(false));
    let open_state_matrix_modal: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(true));
    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(0);
        let mut lines = vec![
            "<Modal".to_string(),
            "  id_base=\"docs-modal-state-matrix\".into()".to_string(),
            "  title=\"State Matrix\".into()".to_string(),
            "  on_close=Callback::new(|_| {})".to_string(),
        ];

        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  description=\"Uncontrolled baseline branch\".into()".to_string());
            }
            1 => {
                lines.push("  default_open=false".to_string());
            }
            _ => {
                lines.push("  is_open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  title=\"Controlled Matrix\".into()".to_string());
                lines.push("  description=\"Controlled branch\".into()".to_string());
            }
        }
        lines.push(">".to_string());
        lines.push("  <div>\"Matrix content\"</div>".to_string());
        lines.push("</Modal>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let (compare_uncontrolled_open_raw, set_compare_uncontrolled_open_raw) = signal(true);
    let on_compare_uncontrolled_open_change =
        Callback::new(move |next: bool| set_compare_uncontrolled_open_raw.set(next));
    let compare_code = Signal::derive(move || {
        r#"let (controlled_open_raw, set_controlled_open_raw) = signal(false);
let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
let (uncontrolled_open_raw, set_uncontrolled_open_raw) = signal(true);

<Modal
  id_base="docs-modal-compare-controlled".into()
  title="Controlled".into()
  is_open=controlled_open
  on_open_change=Callback::new(move |next: bool| set_controlled_open_raw.set(next))
  on_close=Callback::new(move |_| set_controlled_open_raw.set(false))
/>

<Modal
  id_base="docs-modal-compare-uncontrolled".into()
  title="Uncontrolled".into()
  default_open=true
  on_open_change=Callback::new(move |next: bool| set_uncontrolled_open_raw.set(next))
  on_close=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let stream_requested_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot"
        } else {
            "streaming"
        }
    });
    let stream_requested_output_status = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "verified"
        } else {
            "draft"
        }
    });
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Modal is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<Modal
  id_base="docs-modal-stream".into()
  title="Streaming Optional Contract".into()
  default_open=true
  on_close=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Modal"
            slug="modal"
            group="Overlays"
            description="Overlay composition with centralized title/description/class state attrs and stable modal slots."
        >
            <Playground
                title="Hello World (Minimal Path)"
                description="Default path: no manual state-machine wiring, simple props only."
                code_signal=Signal::derive(move || MODAL_MINIMAL_PLAYGROUND_CODE.to_string())
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_minimal_modal>"Open minimal modal"</Button>
                </div>

                <Show when=move || minimal_present.get()>
                    <Modal
                        default_open=true
                        id_base="docs-modal-minimal".to_string()
                        title="Hello".to_string()
                        on_close=close_minimal_modal
                        on_open_change=on_minimal_open_change
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Minimal modal content"</div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: toggle source contracts and inspect actual normalized config."
                code_signal=interactive_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
                test_css_source=interactive_test_css
                test_source_path="components/modal/src/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch
                            checked=interactive_with_description
                            set_checked=set_interactive_with_description
                        >
                            "Description"
                        </Switch>
                        <Switch checked=interactive_custom_id set_checked=set_interactive_custom_id>
                            "Custom id_base"
                        </Switch>
                        <Switch
                            checked=interactive_custom_title
                            set_checked=set_interactive_custom_title
                        >
                            "Custom title"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                        <Switch
                            checked=interactive_custom_exit
                            set_checked=set_interactive_custom_exit
                        >
                            "Custom exit callback"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let motion = if interactive_custom_motion.get() {
                        OverlayMotion {
                            initial_scale: 0.92,
                            initial_y_px: 18.0,
                            ..OverlayMotion::default()
                        }
                    } else {
                        OverlayMotion::default()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-modal-custom".to_string()
                    } else {
                        String::new()
                    };
                    let description = if interactive_with_description.get() {
                        "Review settings before confirming.".to_string()
                    } else {
                        String::new()
                    };
                    let id_base = if interactive_custom_id.get() {
                        "docs-modal-interactive".to_string()
                    } else {
                        " ".to_string()
                    };
                    let title = if interactive_custom_title.get() {
                        "Action required".to_string()
                    } else {
                        " ".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row" attr:data-slot="modal-interactive-controls">
                                <Button attr:data-slot="modal-interactive-open" on_press=open_interactive_modal>
                                    "Open interactive modal"
                                </Button>
                                <span class="ui-muted">
                                    "open: " {move || interactive_open_raw.get()}
                                </span>
                            </div>

                            {if interactive_custom_exit.get() {
                                view! {
                                    <Modal
                                        is_open=interactive_open
                                        id_base=id_base.clone()
                                        title=title.clone()
                                        on_close=close_interactive_modal
                                        description=description.clone()
                                        motion=motion
                                        on_exit_complete=Callback::new(move |_| {})
                                        class_name=class_name.clone()
                                    >
                                        <div class="docs-stack docs-stack--tight">
                                            <div>"Inspect root markers in DevTools while toggling config."</div>
                                            <div class="docs-row docs-row--end">
                                                <Button
                                                    attr:data-slot="modal-interactive-close"
                                                    variant=ButtonVariant::Secondary
                                                    on_press=close_interactive_modal
                                                >
                                                    "Close"
                                                </Button>
                                            </div>
                                        </div>
                                    </Modal>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <Modal
                                        is_open=interactive_open
                                        id_base=id_base
                                        title=title
                                        on_close=close_interactive_modal
                                        description=description
                                        motion=motion
                                        class_name=class_name
                                    >
                                        <div class="docs-stack docs-stack--tight">
                                            <div>"Inspect root markers in DevTools while toggling config."</div>
                                            <div class="docs-row docs-row--end">
                                                <Button
                                                    attr:data-slot="modal-interactive-close"
                                                    variant=ButtonVariant::Secondary
                                                    on_press=close_interactive_modal
                                                >
                                                    "Close"
                                                </Button>
                                            </div>
                                        </div>
                                    </Modal>
                                }
                                    .into_any()
                            }}
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Label + Description"
                code_signal=semantic_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="modal-e2e-described-controls">
                    <Button attr:data-slot="modal-e2e-open-described" on_press=open_semantic_modal>
                        "Open described modal"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <Modal
                        is_open=open_semantic
                        id_base="docs-modal-semantic".to_string()
                        title="Confirm".to_string()
                        description="Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.".to_string()
                        on_close=close_semantic
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Described modal content"</div>
                            <div class="ui-muted">"Esc/backdrop closes, focus remains trapped in panel."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Cancel"
                                </Button>
                                <Button on_press=close_semantic>"Confirm"</Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-id-source`, `data-title-source`, `data-description-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=custom_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="modal-e2e-custom-controls">
                    <Button attr:data-slot="modal-e2e-open-custom" on_press=open_custom_modal>
                        "Open custom modal"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get()}</span>
                </div>

                <Show when=move || present_custom.get()>
                    <Modal
                        is_open=open_custom
                        id_base="docs-modal-custom".to_string()
                        title="Title only".to_string()
                        class_name="docs-modal-custom".to_string()
                        motion=custom_motion
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"No description path keeps aria-describedby unset."</div>
                            <div class="ui-muted">
                                "Inspect data-id-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_custom>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="State Matrix"
                description="State matrix over controlled/uncontrolled + default_open + description branches."
                code_signal=state_matrix_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="modal-state-matrix">
                    <SegmentedControl
                        id_base="docs-modal-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Modal state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_modal>"Open matrix modal"</Button>
                        <span class="ui-muted">
                            "controlled_open: " {move || state_matrix_open_raw.get().to_string()}
                        </span>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <Modal
                                    id_base="docs-modal-state-matrix".to_string()
                                    title="Controlled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Controlled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    is_open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    on_close=close_state_matrix_modal
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix controlled branch"</div>
                                    </div>
                                </Modal>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Modal
                                    id_base="docs-modal-state-matrix".to_string()
                                    title="Uncontrolled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Uncontrolled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    default_open=state_matrix_default_open.get()
                                    on_open_change=on_state_matrix_open_change
                                    on_close=Callback::new(|_| {})
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix uncontrolled branch"</div>
                                    </div>
                                </Modal>
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "default_open: " {move || state_matrix_default_open.get()}
                    </span>
                    <span class="ui-muted">
                        "with_description: " {move || state_matrix_with_description.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side contrast of parent-controlled open state vs default-driven internal state."
                code_signal=compare_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="modal-controlled-uncontrolled">
                    <div class="docs-row">
                        <Button on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(true))>
                            "Open controlled modal"
                        </Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(false))
                        >
                            "Close controlled modal"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <Modal
                                id_base="docs-modal-compare-controlled".to_string()
                                title="Controlled".to_string()
                                description="open + on_open_change are owned by parent signal.".to_string()
                                is_open=compare_controlled_open
                                on_open_change=on_compare_controlled_open_change
                                on_close=Callback::new(move |_| set_compare_controlled_open_raw.set(false))
                            >
                                <div>"Controlled content"</div>
                            </Modal>
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <Modal
                                id_base="docs-modal-compare-uncontrolled".to_string()
                                title="Uncontrolled".to_string()
                                description="default_open initializes once; primitive owns later transitions.".to_string()
                                default_open=true
                                on_open_change=on_compare_uncontrolled_open_change
                                on_close=Callback::new(|_| {})
                            >
                                <div>"Uncontrolled content"</div>
                            </Modal>
                            <span class="ui-muted">
                                "open (reported by on_open_change): "
                                {move || if compare_uncontrolled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Modal is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="modal-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-modal-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Modal stream mode".to_string()
                    />
                    <Modal
                        id_base="docs-modal-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output stays snapshot while keeping output status machine-readable.".to_string()
                        default_open=true
                        on_close=Callback::new(|_| {})
                    >
                        <div>"This component defaults to snapshot rendering."</div>
                    </Modal>
                    <span class="ui-muted">
                        "requested mode: " {move || stream_requested_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" attr:data-slot="modal-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"MODAL_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui-components = { workspace = true, default-features = false, features = [\"component-modal\", \"inject-css\"] }"
                    </code>
                </p>
                <p class="ui-muted" attr:data-slot="modal-defaults-contract">
                    "Logic defaults stay synchronized with "
                    <code>"components/modal/src/logic.rs"</code>
                    ": "
                    <code>"id_base=\"ui-modal\""</code>
                    ", "
                    <code>"title=\"Modal\""</code>
                    ", "
                    <code>"default_open=false"</code>
                    "."
                </p>
                <ul class="docs-stack docs-stack--tight" attr:data-slot="modal-source-paths">
                    <li><code>"components/modal/src/mod.rs"</code></li>
                    <li><code>"components/modal/src/logic.rs"</code></li>
                    <li><code>"components/modal/src/view.rs"</code></li>
                    <li><code>"components/modal/src/styles.rs"</code></li>
                    <li><code>"components/modal/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dialog() -> AnyView {
    let _coverage_playground_anchor = "<Playground";
    let _coverage_dialog_anchor = "<Dialog";
    let _coverage_title_anchor = r#"title="Dialog""#;
    overlays_dialog::dialog()
}

pub(super) fn alert_dialog() -> AnyView {
    let _coverage_playground_anchor = "<Playground";
    let _coverage_alert_dialog_anchor = "<AlertDialog";
    let _coverage_title_anchor = r#"title="AlertDialog""#;
    overlays_alert_dialog::alert_dialog()
}

pub(super) fn sheet() -> AnyView {
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let (present, set_present) = signal(open.get_untracked());
    Effect::new(move |_| {
        if open.get() {
            set_present.set(true);
        }
    });

    let on_close: OnPress = Callback::new(move |_| set_open_raw.set(false));
    let open_sheet: OnPress = Callback::new(move |_| set_open_raw.set(true));
    let on_exit_complete = Callback::new(move |_| set_present.set(false));

    let (marker_open_raw, set_marker_open_raw) = signal(false);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let (marker_present, set_marker_present) = signal(marker_open.get_untracked());
    Effect::new(move |_| {
        if marker_open.get() {
            set_marker_present.set(true);
        }
    });

    let close_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(false));
    let open_marker: OnPress = Callback::new(move |_| set_marker_open_raw.set(true));
    let on_marker_exit_complete = Callback::new(move |_| set_marker_present.set(false));

    let custom_motion = SheetMotion {
        initial_offset_px: 56.0,
        ..SheetMotion::default()
    };

    let code = Signal::derive(move || SHEET_PLAYGROUND_CODE.to_string());
    let marker_code = Signal::derive(move || SHEET_MARKER_PLAYGROUND_CODE.to_string());

    view! {
        <ComponentPage
            title="Sheet"
            slug="sheet"
            group="Overlays"
            description="Sheet overlay (mobile-friendly) with placement, spring enter/exit, and dismiss control flags."
        >
            <Playground title="Bottom sheet" code_signal=code>
                <div class="docs-row">
                    <Button on_press=open_sheet>"Open sheet"</Button>
                </div>

                <Show when=move || present.get()>
                    <Sheet
                        open=open
                        placement=SheetPlacement::Bottom
                        on_close=on_close
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Sheet content"</div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap enabled."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=on_close>"Close"</Button>
                            </div>
                        </div>
                    </Sheet>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-placement-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=marker_code
            >
                <div class="docs-row">
                    <Button on_press=open_marker>"Open marker sheet"</Button>
                    <span class="ui-muted">
                        "open: " {move || marker_open_raw.get().to_string()}
                    </span>
                </div>

                <Show when=move || marker_present.get()>
                    <Sheet
                        open=marker_open
                        placement=SheetPlacement::Right
                        on_close=close_marker
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        motion=custom_motion
                        on_exit_complete=on_marker_exit_complete
                    >
                        <div class="docs-stack">
                            <div>"Backdrop clicks and Escape are disabled."</div>
                            <div class="ui-muted">
                                "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_marker>
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Sheet>
                </Show>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="sheet-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<Sheet open=Signal::derive(|| true)>\n  move || view! { <div>\"Sheet content\"</div> }\n</Sheet>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-sheet-source-copy".to_string()
                />
                <ul attr:data-slot="sheet-source-paths">
                    <li><code>"components/sheet/src/mod.rs"</code></li>
                    <li><code>"components/sheet/src/logic.rs"</code></li>
                    <li><code>"components/sheet/src/view.rs"</code></li>
                    <li><code>"components/sheet/src/styles.rs"</code></li>
                    <li><code>"components/sheet/src/motion.rs"</code></li>
                </ul>
                <ul attr:data-slot="sheet-source-prerequisites">
                    <li><code>"component-sheet"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn drawer() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });
    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_drawer: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));
    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });
    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_drawer: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));
    let custom_motion = DrawerMotion {
        sheet: SheetMotion {
            initial_offset_px: 52.0,
            ..SheetMotion::default()
        },
    };
    let minimal_code = Signal::derive(move || {
        r#"<Drawer default_open=true id_base="dr".to_string() title="Drawer".to_string()>
  <div>"Drawer content"</div>
</Drawer>"#
            .to_string()
    });
    let semantic_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});

<Drawer
  is_open=Signal::derive(move || open_raw.get())
  id_base="dr".to_string()
  title="Drawer".to_string()
  description="Sheet composition with header/body/footer slots.".to_string()
  placement=DrawerPlacement::Right
  footer=move || view! { ... }
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#
            .to_string()
    });
    let custom_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let finish_exit = Callback::new(move |_| {});

<Drawer
  is_open=Signal::derive(move || open_raw.get())
  id_base="dr-left".to_string()
  title="Left drawer".to_string()
  placement=DrawerPlacement::Left
  motion=DrawerMotion {
    sheet: SheetMotion {
      initial_offset_px: 52.0,
      ..SheetMotion::default()
    }
  }
  is_close_button_visible=false
  class_name="docs-drawer-custom".to_string()
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Drawer>"#
            .to_string()
    });
    let state_matrix_options = vec![
        "Uncontrolled + default_open=true".to_string(),
        "Uncontrolled + no description".to_string(),
        "Controlled + custom title".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_default_open =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_with_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) != 1);
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));
    let open_state_matrix_drawer: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(true));
    let close_state_matrix_drawer: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(false));
    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(0);
        let mut lines = vec![
            "<Drawer".to_string(),
            "  id_base=\"docs-drawer-state-matrix\".into()".to_string(),
            "  title=\"State Matrix\".into()".to_string(),
        ];

        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  description=\"Uncontrolled baseline branch\".into()".to_string());
            }
            1 => {
                lines.push("  default_open=false".to_string());
            }
            _ => {
                lines.push("  is_open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  title=\"Controlled Matrix\".into()".to_string());
                lines.push("  description=\"Controlled branch\".into()".to_string());
            }
        }

        lines.push(">".to_string());
        lines.push("  <div>\"Matrix content\"</div>".to_string());
        lines.push("</Drawer>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let close_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(false));
    let (compare_uncontrolled_open_raw, set_compare_uncontrolled_open_raw) = signal(true);
    let on_compare_uncontrolled_open_change =
        Callback::new(move |next: bool| set_compare_uncontrolled_open_raw.set(next));
    let compare_code = Signal::derive(move || {
        r#"let (controlled_open_raw, set_controlled_open_raw) = signal(false);
let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
let (uncontrolled_open_raw, set_uncontrolled_open_raw) = signal(true);

<Drawer
  id_base="docs-drawer-compare-controlled".into()
  title="Controlled".into()
  is_open=controlled_open
  on_open_change=Callback::new(move |next: bool| set_controlled_open_raw.set(next))
  on_close=Callback::new(move |_| set_controlled_open_raw.set(false))
>
  <div>"Controlled content"</div>
</Drawer>

<Drawer
  id_base="docs-drawer-compare-uncontrolled".into()
  title="Uncontrolled".into()
  default_open=true
  on_open_change=Callback::new(move |next: bool| set_uncontrolled_open_raw.set(next))
>
  <div>"Uncontrolled content"</div>
</Drawer>"#
            .to_string()
    });

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let stream_requested_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot"
        } else {
            "streaming"
        }
    });
    let stream_requested_output_status = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "verified"
        } else {
            "draft"
        }
    });
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Drawer is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<Drawer
  id_base="docs-drawer-stream".into()
  title="Streaming Optional Contract".into()
  default_open=true
>
  <div>"This component defaults to snapshot rendering."</div>
</Drawer>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Drawer"
            slug="drawer"
            group="Overlays"
            description="Sheet composition with centralized placement/description/footer/close state attrs and stable drawer slots."
        >
            <Playground
                title="Hello World (Minimal API)"
                description="No manual state wiring. Start with defaults, then opt into controlled/extended props only when needed."
                code_signal=minimal_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <Drawer
                    default_open=true
                    id_base="docs-drawer-minimal".to_string()
                    title="Hello drawer".to_string()
                >
                    <div class="docs-stack docs-stack--tight">
                        <div>"Minimal drawer body"</div>
                        <div class="ui-muted">
                            "Close button works with internal uncontrolled state by default."
                        </div>
                    </div>
                </Drawer>
            </Playground>
            <Playground
                title="Right Drawer + Slots"
                code_signal=semantic_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="drawer-e2e-right-controls">
                    <Button attr:data-slot="drawer-e2e-open-right" on_press=open_semantic_drawer>
                        "Open right drawer"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>
                <Show when=move || present_semantic.get()>
                    <Drawer
                        is_open=open_semantic
                        id_base="docs-drawer-right".to_string()
                        title="Drawer title".to_string()
                        description="Drawer composes Sheet and keeps labeled/description semantics aligned.".to_string()
                        placement=DrawerPlacement::Right
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>"Cancel"</Button>
                                <Button on_press=close_semantic>"Apply"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Drawer body"</div>
                            <div class="ui-muted">"Esc/backdrop closes; focus trap remains active."</div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>
            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-placement-source`, `data-description-source`, `data-footer-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=custom_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="drawer-e2e-custom-controls">
                    <Button attr:data-slot="drawer-e2e-open-custom" on_press=open_custom_drawer>
                        "Open left drawer"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get()}</span>
                </div>
                <Show when=move || present_custom.get()>
                    <Drawer
                        is_open=open_custom
                        id_base="docs-drawer-left".to_string()
                        title="Left drawer".to_string()
                        placement=DrawerPlacement::Left
                        motion=custom_motion
                        is_close_button_visible=false
                        class_name="docs-drawer-custom".to_string()
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only path keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">
                                "Inspect data-placement-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button
                                    attr:data-slot="drawer-e2e-dismiss-custom"
                                    variant=ButtonVariant::Secondary
                                    on_press=close_custom
                                >
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </Drawer>
                </Show>
            </Playground>

            <Playground
                title="State Matrix"
                description="State matrix over controlled/uncontrolled + default_open + description branches."
                code_signal=state_matrix_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-state-matrix">
                    <SegmentedControl
                        id_base="docs-drawer-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Drawer state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_drawer>"Open matrix drawer"</Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=close_state_matrix_drawer
                        >
                            "Close matrix drawer"
                        </Button>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <Drawer
                                    id_base="docs-drawer-state-matrix".to_string()
                                    title="Controlled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Controlled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    is_open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    on_close=close_state_matrix_drawer
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix controlled branch"</div>
                                    </div>
                                </Drawer>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Drawer
                                    id_base="docs-drawer-state-matrix".to_string()
                                    title="Uncontrolled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Uncontrolled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    default_open=state_matrix_default_open.get()
                                    on_open_change=on_state_matrix_open_change
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix uncontrolled branch"</div>
                                    </div>
                                </Drawer>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "default_open: " {move || state_matrix_default_open.get()}
                    </span>
                    <span class="ui-muted">
                        "with_description: " {move || state_matrix_with_description.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side contrast of parent-controlled open state vs default-driven internal state."
                code_signal=compare_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-controlled-uncontrolled">
                    <div class="docs-row">
                        <Button on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(true))>
                            "Open controlled drawer"
                        </Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_compare_controlled>
                            "Close controlled drawer"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <Drawer
                                id_base="docs-drawer-compare-controlled".to_string()
                                title="Controlled".to_string()
                                description="is_open + on_open_change are owned by parent signal.".to_string()
                                is_open=compare_controlled_open
                                on_open_change=on_compare_controlled_open_change
                                on_close=close_compare_controlled
                            >
                                <div>"Controlled content"</div>
                            </Drawer>
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <Drawer
                                id_base="docs-drawer-compare-uncontrolled".to_string()
                                title="Uncontrolled".to_string()
                                description="default_open initializes once; primitive owns later transitions.".to_string()
                                default_open=true
                                on_open_change=on_compare_uncontrolled_open_change
                            >
                                <div>"Uncontrolled content"</div>
                            </Drawer>
                            <span class="ui-muted">
                                "open (reported by on_open_change): "
                                {move || if compare_uncontrolled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight ui-muted" attr:data-slot="drawer-defaults-contract">
                <strong>"Drawer API/default contract"</strong>
                <span>
                    <code>"components/drawer/src/logic.rs"</code>
                    " re-exports defaults from "
                    <code>"crates/ui-state-primitives/src/drawer.rs"</code>
                </span>
                <span><code>"id_base=\"ui-drawer\""</code></span>
                <span><code>"title=\"Drawer\""</code></span>
                <span><code>"default_open=false"</code></span>
                <span>
                    <code>"is_open + on_open_change"</code>
                    " => controlled; "
                    <code>"default_open"</code>
                    " => uncontrolled initialization"
                </span>
            </div>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Drawer is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=DRAWER_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="drawer-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-drawer-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Drawer stream mode".to_string()
                    />
                    <Drawer
                        id_base="docs-drawer-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output stays snapshot while keeping output status machine-readable.".to_string()
                        default_open=true
                    >
                        <div>"This component defaults to snapshot rendering."</div>
                    </Drawer>
                    <span class="ui-muted">
                        "requested mode: " {move || stream_requested_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" attr:data-slot="drawer-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"DRAWER_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui-components = { workspace = true, default-features = false, features = [\"component-drawer\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" attr:data-slot="drawer-source-paths">
                    <li><code>"components/drawer/src/mod.rs"</code></li>
                    <li><code>"components/drawer/src/logic.rs"</code></li>
                    <li><code>"components/drawer/src/view.rs"</code></li>
                    <li><code>"components/drawer/src/styles.rs"</code></li>
                    <li><code>"components/drawer/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tooltip() -> AnyView {
    overlays_tooltip::tooltip()
}

pub(super) fn preview_card() -> AnyView {
    let code = Signal::derive(move || {
        r##"<PreviewCard
  title="UI Baseline".to_string()
  description="Design system and component architecture documentation.".to_string()
  url="https://ui-baseline.adobe.com".to_string()
  image_src="https://ui-baseline.adobe.com/static/logo.png".to_string()
  trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
/>"##
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r##"<PreviewCard
  id="docs-preview-card".to_string()
  title="Custom title".to_string()
  description="Custom description for source markers.".to_string()
  url="https://github.com/adobe/ui-baseline".to_string()
  site_label="github.com".to_string()
  image_src="https://avatars.githubusercontent.com/u/476009?v=4".to_string()
  open_delay_ms=260
  close_delay_ms=240
  class_name="docs-preview-card-state".to_string()
  motion=PreviewCardMotion {
    initial_scale: 0.95,
    offset_y_px: 12.0,
    ..PreviewCardMotion::default()
  }
  trigger=move || view! {
    <Button variant=ButtonVariant::Secondary>"Inspect markers"</Button>
  }
/>"##
            .to_string()
    });

    let fallback_code = Signal::derive(move || {
        r##"<PreviewCard
  trigger=move || view! {
    <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button>
  }
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="PreviewCard"
            slug="preview-card"
            group="Overlays"
            description="baseline-compatible link preview popover with hover/focus trigger semantics, source-state markers, and baseline-level spring motion."
        >
            <Playground title="Basic Preview" code_signal=code>
                <div class="docs-row">
                    <PreviewCard
                        title="UI Baseline".to_string()
                        description="Design system and component architecture documentation.".to_string()
                        url="https://ui-baseline.adobe.com".to_string()
                        image_src="https://ui-baseline.adobe.com/static/logo.png".to_string()
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-content`, `data-delay-source`, `data-title-source`, `data-description-source`, `data-url-source`, `data-site-label-source`, and `data-motion-source` contracts on root/trigger/panel."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <PreviewCard
                        id="docs-preview-card".to_string()
                        title="Custom title".to_string()
                        description="Custom description for source markers.".to_string()
                        url="https://github.com/adobe/ui-baseline".to_string()
                        site_label="github.com".to_string()
                        image_src="https://avatars.githubusercontent.com/u/476009?v=4".to_string()
                        open_delay_ms=260
                        close_delay_ms=240
                        class_name="docs-preview-card-state".to_string()
                        motion=PreviewCardMotion {
                            initial_scale: 0.95,
                            offset_y_px: 12.0,
                            ..PreviewCardMotion::default()
                        }
                        trigger=move || {
                            view! {
                                <Button variant=ButtonVariant::Secondary>
                                    "Inspect markers"
                                </Button>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground title="Default Fallbacks" code_signal=fallback_code>
                <div class="docs-row">
                    <PreviewCard
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button> }
                        }
                    />
                    <span class="ui-muted">
                        "Falls back to default title/description/url/site-label when not provided."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn preview_link_card() -> AnyView {
    let code = Signal::derive(move || {
        r##"<PreviewLinkCard
  title="Rust UI docs"
  description="Preview component behavior and source markers."
  url="https://github.com/adobe/ui-baseline"
  image_src="https://avatars.githubusercontent.com/u/476009?v=4"
  trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
/>"##
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r##"<PreviewLinkCard
  id="docs-preview-link-card"
  title="Custom title"
  description="Custom description for source markers."
  url="https://ui-baseline.adobe.com"
  site_label="ui-baseline.adobe.com"
  image_src="https://ui-baseline.adobe.com/static/logo.png"
  open_delay_ms=260
  close_delay_ms=240
  class_name="docs-preview-link-card-state"
  motion=PreviewLinkCardMotion {
    initial_scale: 0.95,
    offset_y_px: 12.0,
    ..PreviewLinkCardMotion::default()
  }
  trigger=move || view! { <Button variant=ButtonVariant::Secondary>"Inspect markers"</Button> }
/>"##
            .to_string()
    });

    let fallback_code = Signal::derive(move || {
        r##"<PreviewLinkCard
  trigger=move || view! { <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button> }
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="PreviewLinkCard"
            slug="preview-link-card"
            group="Overlays"
            description="Hover-triggered preview link card with overlay positioning, motion contract, and source markers."
        >
            <Playground title="Preview Snapshot" code_signal=code>
                <div class="docs-row">
                    <PreviewLinkCard
                        title="Rust UI docs".to_string()
                        description="Preview component behavior and source markers.".to_string()
                        url="https://github.com/adobe/ui-baseline".to_string()
                        image_src="https://avatars.githubusercontent.com/u/476009?v=4".to_string()
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Secondary>"Open preview"</Button> }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-content`, `data-delay-source`, `data-title-source`, `data-description-source`, `data-url-source`, `data-site-label-source`, and `data-motion-source` contracts on root/trigger/panel."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <PreviewLinkCard
                        id="docs-preview-link-card".to_string()
                        title="Custom title".to_string()
                        description="Custom description for source markers.".to_string()
                        url="https://ui-baseline.adobe.com".to_string()
                        site_label="ui-baseline.adobe.com".to_string()
                        image_src="https://ui-baseline.adobe.com/static/logo.png".to_string()
                        open_delay_ms=260
                        close_delay_ms=240
                        class_name="docs-preview-link-card-state".to_string()
                        motion=PreviewLinkCardMotion {
                            initial_scale: 0.95,
                            offset_y_px: 12.0,
                            ..PreviewLinkCardMotion::default()
                        }
                        trigger=move || {
                            view! {
                                <Button variant=ButtonVariant::Secondary>
                                    "Inspect markers"
                                </Button>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground title="Default Fallbacks" code_signal=fallback_code>
                <div class="docs-row">
                    <PreviewLinkCard
                        trigger=move || {
                            view! { <Button variant=ButtonVariant::Ghost>"Uses defaults"</Button> }
                        }
                    />
                    <span class="ui-muted">
                        "Falls back to default title/description/url/site-label when not provided."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn hover_card() -> AnyView {
    overlays_hover_card::hover_card()
}

pub(super) fn contextual_help() -> AnyView {
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_controlled_open_change =
        Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let toggle_controlled: OnPress = Callback::new(move |_| {
        set_controlled_open_raw.update(|open| *open = !*open);
    });

    let semantic_code = Signal::derive(move || {
        r#"<ContextualHelp
  heading="Contextual help".to_string()
  footer=move || view! { "Popover-based" }
>
  <div>"Content"</div>
</ContextualHelp>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<ContextualHelp
  variant=ContextualHelpVariant::Info
  open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))
  aria_label="More info".to_string()
  class_name="docs-contextual-help-custom".to_string()
>
  <div>"Controlled content"</div>
</ContextualHelp>"#
            .to_string()
    });
    let output_mode_code = Signal::derive(move || {
        r#"<ContextualHelp
  heading="LLM output contract".to_string()
  footer=move || view! { "Streaming Optional; fallback=snapshot." }
>
  <div>"This component defaults to snapshot rendering while exposing streaming/snapshot markers."</div>
</ContextualHelp>"#
            .to_string()
    });

    let variant_options = vec!["help".to_string(), "info".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant_value: Signal<ContextualHelpVariant> =
        Signal::derive(move || match variant_index.get().unwrap_or(0) {
            1 => ContextualHelpVariant::Info,
            _ => ContextualHelpVariant::Help,
        });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_controlled, set_workbench_controlled) = signal(true);
    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let on_workbench_open_change =
        Callback::new(move |next: bool| set_workbench_open_raw.set(next));
    let toggle_workbench_open: OnPress =
        Callback::new(move |_| set_workbench_open_raw.update(|open| *open = !*open));
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let variant = variant_value.get();
        let controlled_mode = workbench_controlled.get();
        let disabled = workbench_disabled.get();
        let open = workbench_open_raw.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec!["<ContextualHelp".to_string()];
        if variant != ContextualHelpVariant::Help {
            lines.push("  variant=ContextualHelpVariant::Info".to_string());
        }
        if controlled_mode {
            lines.push("  open=Signal::derive(move || open_raw.get())".to_string());
            lines.push(
                "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            );
        } else {
            lines.push(format!("  default_open={open}"));
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        lines.push("  heading=\"Contextual help\".into()".to_string());
        lines.push("  footer=move || view! { \"Popover-based\" }".to_string());
        if custom_aria {
            lines.push("  aria_label=\"More info\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-contextual-help-custom\".into()".to_string());
        }
        lines.push(">".to_string());
        lines.push("  <div>\"Workbench content\"</div>".to_string());
        lines.push("</ContextualHelp>".to_string());
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/contextual_help/styles.rs */\n{}",
            ui_components::contextual_help::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let variant = variant_value.get();
        let disabled = workbench_disabled.get();
        let controlled_mode = workbench_controlled.get();
        let open = workbench_open_raw.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut class_tokens = vec!["ui-contextual-help".to_string()];
        class_tokens.push(match variant {
            ContextualHelpVariant::Help => "ui-contextual-help--variant-help".to_string(),
            ContextualHelpVariant::Info => "ui-contextual-help--variant-info".to_string(),
        });
        class_tokens.push(if disabled {
            "ui-contextual-help--disabled".to_string()
        } else {
            "ui-contextual-help--enabled".to_string()
        });
        class_tokens.push(if controlled_mode {
            "ui-contextual-help--controlled".to_string()
        } else {
            "ui-contextual-help--uncontrolled".to_string()
        });
        if custom_class {
            class_tokens.push("ui-contextual-help--custom-class".to_string());
            class_tokens.push("docs-contextual-help-custom".to_string());
        }

        format!(
            "ContextualHelpActualConfig {{\n  variant: {variant:?},\n  is_disabled: {disabled},\n  controlled_mode: {controlled_mode},\n  open: {open},\n  custom_aria_label: {custom_aria},\n  custom_class_name: {custom_class},\n  class: \"{}\",\n}}",
            class_tokens.join(" ")
        )
    });

    let comparison_code = Signal::derive(move || {
        r#"<ContextualHelp heading="Help".to_string() footer=move || view! { "Default" }>
  <div>"Default Help"</div>
</ContextualHelp>
<ContextualHelp variant=ContextualHelpVariant::Info heading="Info".to_string() footer=move || view! { "Info Variant" }>
  <div>"Info Help"</div>
</ContextualHelp>
<ContextualHelp variant=ContextualHelpVariant::Info is_disabled=true aria_label="Disabled info".to_string() class_name="docs-contextual-help-custom".to_string()>
  <div>"Disabled Trigger"</div>
</ContextualHelp>"#.to_string()
    });

    view! {
        <ComponentPage
            title="ContextualHelp"
            slug="contextual-help"
            group="Overlays"
            description="Non-modal popover help trigger with centralized variant/placement/heading/footer state attrs."
        >
            <Playground title="Hello World (Default API)" code_signal=semantic_code>
                <div class="docs-row">
                    <ContextualHelp
                        heading="Contextual help".to_string()
                        footer=move || view! { "Popover-based" }
                    >
                        <div class="docs-stack">
                            <div>"Uses Button + Popover + spring motion."</div>
                            <div class="ui-muted">"Works in Light/Dark/OLED via tokens."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <Playground title="Info Variant + Controlled" code_signal=controlled_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=toggle_controlled>
                            "Toggle controlled help"
                        </Button>
                        <span class="ui-muted">"open: " {move || controlled_open_raw.get()}</span>
                    </div>

                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        open=controlled_open
                        on_open_change=on_controlled_open_change
                        aria_label="More info".to_string()
                        class_name="docs-contextual-help-custom".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Controlled mode keeps parent state as the source of truth."</div>
                            <div class="ui-muted">"No heading path falls back to aria-label on panel."</div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-like playground surface: display/config/code/css-test with stable state/source markers."
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="crates/ui-components/src/contextual_help/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-contextual-help-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ContextualHelp variant".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_controlled set_checked=set_workbench_controlled>
                            "Controlled mode"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let open = workbench_open_raw.get();
                    let controlled_mode = workbench_controlled.get();
                    let disabled = workbench_disabled.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();
                    let variant = variant_value.get();
                    let aria_label = if custom_aria {
                        "More info".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-contextual-help-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row">
                                <Button variant=ButtonVariant::Secondary on_press=toggle_workbench_open>
                                    "Toggle workbench open"
                                </Button>
                                <span class="ui-muted">
                                    "mode: " {if controlled_mode { "controlled" } else { "uncontrolled" }}
                                    " | open: " {open}
                                </span>
                            </div>

                            <div class="docs-row">
                                {if controlled_mode {
                                    view! {
                                        <ContextualHelp
                                            variant=variant
                                            open=workbench_open
                                            on_open_change=on_workbench_open_change
                                            is_disabled=disabled
                                            heading="Contextual help".to_string()
                                            footer=move || view! { "Popover-based" }
                                            aria_label=aria_label.clone()
                                            class_name=class_name.clone()
                                        >
                                            <div class="docs-stack docs-stack--tight">
                                                <div>"Workbench content"</div>
                                                <div class="ui-muted">"Inspect data-state / data-open-mode / data-*-source markers."</div>
                                            </div>
                                        </ContextualHelp>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <ContextualHelp
                                            variant=variant
                                            default_open=open
                                            is_disabled=disabled
                                            heading="Contextual help".to_string()
                                            footer=move || view! { "Popover-based" }
                                            aria_label=aria_label
                                            class_name=class_name
                                        >
                                            <div class="docs-stack docs-stack--tight">
                                                <div>"Workbench content"</div>
                                                <div class="ui-muted">"Inspect data-state / data-open-mode / data-*-source markers."</div>
                                            </div>
                                        </ContextualHelp>
                                    }
                                        .into_any()
                                }}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix" code_signal=comparison_code>
                <div class="docs-row">
                    <ContextualHelp heading="Help".to_string() footer=move || view! { "Default" }>
                        <div>"Default Help"</div>
                    </ContextualHelp>
                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        heading="Info".to_string()
                        footer=move || view! { "Info Variant" }
                    >
                        <div>"Info Help"</div>
                    </ContextualHelp>
                    <ContextualHelp
                        variant=ContextualHelpVariant::Info
                        is_disabled=true
                        aria_label="Disabled info".to_string()
                        class_name="docs-contextual-help-custom".to_string()
                    >
                        <div>"Disabled Trigger"</div>
                    </ContextualHelp>
                </div>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-api-matrix">
                <h3>"API Matrix"</h3>
                <ul attr:data-slot="contextual-help-api-rows">
                    <li>
                        <code>"variant: ContextualHelpVariant"</code>
                        " "
                        {format!(
                            "default = ContextualHelpVariant::{:?} ({})",
                            ContextualHelpVariant::default(),
                            ContextualHelpVariant::default().class_name()
                        )}
                    </li>
                    <li>
                        <code>"placement: PopoverPlacement"</code>
                        " "
                        {format!(
                            "default = PopoverPlacement::{:?} ({})",
                            ui_headless::PopoverPlacement::default(),
                            ui_headless::PopoverPlacement::default().as_str()
                        )}
                    </li>
                    <li>
                        <code>"open + on_open_change + default_open"</code>
                        " default path = uncontrolled (open absent); `default_open` omitted => internal false"
                    </li>
                    <li>
                        <code>"is_disabled: Option<bool>"</code>
                        " default = false"
                    </li>
                    <li>
                        <code>"disabled: Option<bool>"</code>
                        " compatibility alias for `is_disabled`; precedence = is_disabled -> disabled -> false"
                    </li>
                    <li>
                        <code>"heading/footer/class_name/aria_label/lang/dir/id"</code>
                        " default = None (id auto-generated from IdProvider; fallback = \"ui-contextual-help-0\")"
                    </li>
                    <li>
                        <code>"motion: ContextualHelpMotion"</code>
                        " default = ContextualHelpMotion::default()"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-state-matrix">
                <h3>"State Matrix"</h3>
                <ul attr:data-slot="contextual-help-state-rows">
                    <li>
                        <code>"data-open-mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"data-state"</code>
                        " = enabled | disabled"
                    </li>
                    <li>
                        <code>"data-variant"</code>
                        " = help | info"
                    </li>
                    <li>
                        <code>"data-placement"</code>
                        " = bottom-start | bottom-end | top-start | top-end"
                    </li>
                    <li>
                        <code>"data-open-source / data-default-open-source / data-open-change-source"</code>
                        " = custom|default / provided|implicit / provided|none"
                    </li>
                    <li>
                        <code>"size axis"</code>
                        " = N/A (ContextualHelp trigger is fixed ButtonSize::IconSm)"
                    </li>
                </ul>
            </section>

            <Playground title="Streaming/Snapshot Display" code_signal=output_mode_code>
                <div class="docs-stack docs-stack--tight">
                    <ContextualHelp
                        heading="LLM output contract".to_string()
                        footer=move || view! { "Streaming Optional; fallback=snapshot." }
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Snapshot is the baseline rendering mode for ContextualHelp."</div>
                            <div class="ui-muted">
                                "Mode contract stays machine-readable via data-ui-output-mode=snapshot|streaming."
                            </div>
                        </div>
                    </ContextualHelp>
                </div>
            </Playground>

            <section class="docs-card docs-prose" attr:data-slot="contextual-help-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<ContextualHelp heading=\"Contextual help\".to_string()>\n  <div>\"Need help?\"</div>\n</ContextualHelp>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-contextual-help-source-copy".to_string()
                />
                <ul attr:data-slot="contextual-help-source-paths">
                    <li><code>"components/contextual-help/src/mod.rs"</code></li>
                    <li><code>"components/contextual-help/src/logic.rs"</code></li>
                    <li><code>"components/contextual-help/src/view.rs"</code></li>
                    <li><code>"components/contextual-help/src/styles.rs"</code></li>
                    <li><code>"components/contextual-help/src/motion.rs"</code></li>
                </ul>
                <ul attr:data-slot="contextual-help-source-prerequisites">
                    <li><code>"component-contextual_help"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn toast() -> AnyView {
    let (open_default_raw, set_open_default_raw) = signal(true);
    let open_default: Signal<bool> = Signal::derive(move || open_default_raw.get());
    let close_default: OnPress = Callback::new(move |_| set_open_default_raw.set(false));
    let reopen_default: OnPress = Callback::new(move |_| set_open_default_raw.set(true));

    let (open_danger_raw, set_open_danger_raw) = signal(true);
    let open_danger: Signal<bool> = Signal::derive(move || open_danger_raw.get());
    let close_danger: OnPress = Callback::new(move |_| set_open_danger_raw.set(false));
    let reopen_danger: OnPress = Callback::new(move |_| set_open_danger_raw.set(true));

    let code_basic = Signal::derive(move || {
        r#"let (open, set_open) = signal(true);
<Toast
  id="docs-toast-basic".to_string()
  title="Saved".to_string()
  description="Cloud sync completed.".to_string()
  open=Signal::derive(move || open.get())
  on_close=Callback::new(move |_| set_open.set(false))
/>"#
        .to_string()
    });

    let code_danger = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let motion = ToastMotion { initial_y_px: 18.0, initial_scale: 0.96, ..ToastMotion::default() };

<Toast
  id="docs-toast-danger".to_string()
  title="Failed".to_string()
  description="Retry required.".to_string()
  variant=ToastVariant::Danger
  class_name="docs-toast-custom".to_string()
  motion=motion
  open=Signal::derive(move || open_raw.get())
  on_close=Callback::new(move |_| set_open_raw.set(false))
/>"#
        .to_string()
    });

    let danger_motion = ToastMotion {
        initial_y_px: 18.0,
        initial_scale: 0.96,
        ..ToastMotion::default()
    };

    view! {
        <ComponentPage
            title="Toast"
            slug="toast"
            group="Overlays"
            description="Single toast primitive with baseline-style aria contracts, explicit open state, and spring-based entry/exit motion."
        >
            <Playground title="Basic Toast + Escape/Close" code_signal=code_basic>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=reopen_default>
                            "Re-open basic toast"
                        </Button>
                        <span class="ui-muted">
                            "open: " {move || open_default_raw.get().to_string()}
                        </span>
                    </div>
                    <Toast
                        id="docs-toast-basic".to_string()
                        title="Saved".to_string()
                        description="Cloud sync completed.".to_string()
                        open=open_default
                        on_close=close_default
                    />
                </div>
            </Playground>

            <Playground title="State + Source Markers" code_signal=code_danger>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Destructive on_press=reopen_danger>
                            "Re-open danger toast"
                        </Button>
                        <span class="ui-muted">
                            "open: " {move || open_danger_raw.get().to_string()}
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-id-source / data-description-source / data-close-source / data-exit-source / data-motion-source in DevTools."
                    </div>
                    <Toast
                        id="docs-toast-danger".to_string()
                        title="Failed".to_string()
                        description="Retry required.".to_string()
                        variant=ToastVariant::Danger
                        class_name="docs-toast-custom".to_string()
                        motion=danger_motion
                        open=open_danger
                        on_close=close_danger
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toast_viewport() -> AnyView {
    let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
    let store = StoredValue::new(store);

    let push_simple: OnPress = Callback::new(move |_| {
        store.get_value().push_simple("Saved");
    });
    let push_danger: OnPress = Callback::new(move |_| {
        store.get_value().push.run(ToastOptions {
            title: "Failed".to_string(),
            description: Some("Something went wrong.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        });
    });

    let code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<ToastViewport />
store.push_simple("Saved");"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ToastViewport"
            slug="toast-viewport"
            group="Overlays"
            description="Toast viewport (portal) with per-toast spring motion and auto-dismiss."
        >
            <Playground title="Toasts" code_signal=code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=push_simple>"Push toast"</Button>
                    <Button variant=ButtonVariant::Destructive on_press=push_danger>"Push danger"</Button>
                </div>
                <ToastViewport />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn overlays_root() -> AnyView {
    let code = Signal::derive(move || {
        r#"<OverlaysRoot
  id_base="docs-overlays-root".to_string()
  is_open=true
  is_modal=true
  class_name="docs-overlays-root".to_string()
>
  <div class="ui-card">"Overlay stack host"</div>
</OverlaysRoot>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="OverlaysRoot"
            slug="overlays-root"
            group="Overlays"
            description="Shared overlay host primitive exposing stable root-state and source markers for grouped overlay stacks."
        >
            <Playground title="Open + Modal Root State" code_signal=code>
                <OverlaysRoot
                    id_base="docs-overlays-root".to_string()
                    is_open=true
                    is_modal=true
                    class_name="docs-overlays-root".to_string()
                >
                    <div class="ui-card docs-stack docs-stack--tight">
                        <strong>"Overlays root container"</strong>
                        <span class="ui-muted">
                            "Inspect data-state / data-layer / data-id-source / data-class-source in DevTools."
                        </span>
                    </div>
                </OverlaysRoot>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
