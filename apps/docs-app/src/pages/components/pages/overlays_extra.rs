use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AiOutputStatus, AiRenderMode, AiSpace, BottomSheet, BottomSheetMotion, Button, ButtonVariant,
    OnPress, Snippet, Sonner, SonnerPosition, ToastMotion, ToastOptions, ToastStoreOptions,
    ToastVariant, Toaster, ToasterPosition, Tray, TrayMotion, Underlay, provide_toast_store,
};

pub(super) fn bottom_sheet() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_sheet: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_detached_raw, set_open_detached_raw) = signal(false);
    let open_detached: Signal<bool> = Signal::derive(move || open_detached_raw.get());
    let (present_detached, set_present_detached) = signal(open_detached.get_untracked());
    Effect::new(move |_| {
        if open_detached.get() {
            set_present_detached.set(true);
        }
    });

    let close_detached: OnPress = Callback::new(move |_| set_open_detached_raw.set(false));
    let open_detached_sheet: OnPress = Callback::new(move |_| set_open_detached_raw.set(true));
    let on_detached_exit_complete = Callback::new(move |_| set_present_detached.set(false));

    let (open_custom_motion_raw, set_open_custom_motion_raw) = signal(false);
    let open_custom_motion: Signal<bool> = Signal::derive(move || open_custom_motion_raw.get());
    let (present_custom_motion, set_present_custom_motion) =
        signal(open_custom_motion.get_untracked());
    Effect::new(move |_| {
        if open_custom_motion.get() {
            set_present_custom_motion.set(true);
        }
    });

    let close_custom_motion: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(false));
    let open_custom_motion_sheet: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(true));
    let on_custom_motion_exit_complete =
        Callback::new(move |_| set_present_custom_motion.set(false));

    let semantic_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet".to_string()
  title="Update available".to_string()
  description="A newer version with security improvements is ready to install.".to_string()
  on_close=Callback::new(move |_| {})
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let detached_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-detached".to_string()
  title="Quick actions".to_string()
  detached=true
  bottom_inset_px=16.0
  show_close_button=false
  class_name="docs-bottom-sheet-custom".to_string()
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let custom_motion_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-motion".to_string()
  title="Motion tuned".to_string()
  description="Custom sheet motion demonstrates data-motion-source contract.".to_string()
  motion=BottomSheetMotion {
    sheet: ui_components::SheetMotion {
      initial_offset_px: 64.0,
      ..ui_components::SheetMotion::default()
    }
  }
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="BottomSheet"
            slug="bottom-sheet"
            group="Overlays"
            description="baseline-style bottom sheet primitive composed from Sheet with centralized handle/description/footer/detached contracts and stable slot/data-state markers."
        >
            <Playground title="Semantic Bottom Sheet" code_signal=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_sheet>"Open bottom sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <BottomSheet
                        open=open_semantic
                        id_base="docs-bottom-sheet-semantic".to_string()
                        title="Update available".to_string()
                        description="A newer version with security improvements is ready to install.".to_string()
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Later"
                                </Button>
                                <Button on_press=close_semantic>"Install"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>
                                "Bottom sheet uses Sheet(bottom) + spring motion with stable a11y wiring."
                            </div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap remains active."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground title="Detached + Title Only + Custom Class" code_signal=detached_code>
                <div class="docs-row">
                    <Button on_press=open_detached_sheet>"Open detached sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_detached_raw.get()}</span>
                </div>

                <Show when=move || present_detached.get()>
                    <BottomSheet
                        open=open_detached
                        id_base="docs-bottom-sheet-detached".to_string()
                        title="Quick actions".to_string()
                        detached=true
                        bottom_inset_px=16.0
                        show_close_button=false
                        class_name="docs-bottom-sheet-custom".to_string()
                        on_close=close_detached
                        on_exit_complete=on_detached_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only mode keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">"Detached mode applies inset and rounded surface styling."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_detached>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground title="Custom Motion Contract" code_signal=custom_motion_code>
                <div class="docs-row">
                    <Button on_press=open_custom_motion_sheet>"Open custom motion sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_motion_raw.get()}</span>
                </div>

                <Show when=move || present_custom_motion.get()>
                    <BottomSheet
                        open=open_custom_motion
                        id_base="docs-bottom-sheet-motion".to_string()
                        title="Motion tuned".to_string()
                        description="Custom sheet motion flips data-motion-source to custom.".to_string()
                        motion=BottomSheetMotion {
                            sheet: ui_components::SheetMotion {
                                initial_offset_px: 64.0,
                                ..ui_components::SheetMotion::default()
                            },
                        }
                        on_close=close_custom_motion
                        on_exit_complete=on_custom_motion_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Motion contract switches to custom markers for style/debug hooks."</div>
                            <div class="ui-muted">"Use devtools to inspect data-motion-source/custom-motion."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tray() -> AnyView {
    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_tray: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
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
    let open_custom_tray: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));

    let custom_motion = TrayMotion {
        sheet: ui_components::SheetMotion {
            initial_offset_px: 46.0,
            ..ui_components::SheetMotion::default()
        },
    };

    let semantic_code = Signal::derive(move || {
        r#"<Tray
  open=open
  id_base="tray".to_string()
  title="Filters".to_string()
  description="Bottom tray with semantic heading + footer actions.".to_string()
  on_close=Callback::new(move |_| {})
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</Tray>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Tray
  open=open
  id_base="tray-fixed".to_string()
  title="Fixed tray".to_string()
  motion=TrayMotion {
    sheet: ui_components::SheetMotion {
      initial_offset_px: 46.0,
      ..ui_components::SheetMotion::default()
    }
  }
  is_fixed_height=true
  is_dismissable=false
  is_keyboard_dismiss_disabled=true
  show_close_button=false
  class_name="docs-tray-custom".to_string()
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</Tray>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Tray"
            slug="tray"
            group="Overlays"
            description="baseline-compatible bottom tray primitive composed from Sheet with centralized description/footer/close/height contracts and stable slot/data-state markers."
        >
            <Playground title="Tray + Footer Actions" code_signal=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_tray>"Open tray"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <Tray
                        open=open_semantic
                        id_base="docs-tray-semantic".to_string()
                        title="Filters".to_string()
                        description="Tray composes Sheet with title/description wiring and footer action slots.".to_string()
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>"Reset"</Button>
                                <Button on_press=close_semantic>"Apply"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Tray body content"</div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap remains active."</div>
                        </div>
                    </Tray>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-size-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=custom_code
            >
                <div class="docs-row">
                    <Button on_press=open_custom_tray>"Open fixed tray"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get()}</span>
                </div>

                <Show when=move || present_custom.get()>
                    <Tray
                        open=open_custom
                        id_base="docs-tray-fixed".to_string()
                        title="Fixed tray".to_string()
                        motion=custom_motion
                        is_fixed_height=true
                        is_dismissable=false
                        is_keyboard_dismiss_disabled=true
                        show_close_button=false
                        class_name="docs-tray-custom".to_string()
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only path keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">
                                "Inspect data-size-source / data-dismiss-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_custom>"Dismiss"</Button>
                            </div>
                        </div>
                    </Tray>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sonner() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));
    let hello_world_code = Signal::derive(move || r#"<Sonner />"#.to_string());

    let push_saved: OnPress = Callback::new(move |_| {
        portal_store.get_value().push_simple("Saved");
    });
    let push_danger: OnPress = Callback::new(move |_| {
        portal_store.get_value().push.run(ToastOptions {
            title: "Publish failed".to_string(),
            description: Some("Check network and retry.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        });
    });

    let push_inline: OnPress = Callback::new(move |_| {
        inline_store.get_value().push.run(ToastOptions {
            title: "Undo available".to_string(),
            description: Some("Item moved to archive.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5000),
        });
    });
    let clear_inline: OnPress = Callback::new(move |_| inline_store.get_value().clear.run(()));

    let push_source: OnPress = Callback::new(move |_| {
        source_store.get_value().push.run(ToastOptions {
            title: "Migration complete".to_string(),
            description: Some("Source markers are stable.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(5500),
        });
    });
    let clear_source: OnPress = Callback::new(move |_| source_store.get_value().clear.run(()));

    let basic_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Sonner store=store.clone() />
store.push_simple("Saved");"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopCenter
  max_toasts=2
  class_name="docs-sonner-inline".to_string()
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopLeft
  max_toasts=4
  aria_label="Status updates".to_string()
  class_name="docs-sonner-source".to_string()
  motion=ToastMotion {
    initial_y_px: 22.0,
    initial_scale: 0.94,
    ..ToastMotion::default()
  }
/>"#
        .to_string()
    });

    let custom_motion = ToastMotion {
        initial_y_px: 22.0,
        initial_scale: 0.94,
        ..ToastMotion::default()
    };

    view! {
        <ComponentPage
            title="Sonner"
            slug="sonner"
            group="Overlays"
            description="baseline-style toast host that composes ToastViewport with position presets, queue limits, and stable Sonner slot/source-state data contracts."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="ui-muted">
                    "Default path mounts a notification host with sensible defaults and no manual state wiring."
                </div>
                <Sonner />
            </Playground>

            <Playground title="Portal Queue + Variants" code_signal=basic_code>
                <div class="docs-row" data-slot="sonner-portal-controls">
                    <span data-slot="sonner-portal-push-success">
                        <Button variant=ButtonVariant::Secondary on_press=push_saved>
                            "Push success"
                        </Button>
                    </span>
                    <span data-slot="sonner-portal-push-danger">
                        <Button variant=ButtonVariant::Destructive on_press=push_danger>
                            "Push danger"
                        </Button>
                    </span>
                </div>
                <Sonner store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center + Max Queue" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="sonner-inline-controls">
                        <span data-slot="sonner-inline-push">
                            <Button on_press=push_inline>"Push accent"</Button>
                        </span>
                        <span data-slot="sonner-inline-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <Sonner
                        store=inline_store.get_value()
                        portal=false
                        position=SonnerPosition::TopCenter
                        max_toasts=2
                        class_name="docs-sonner-inline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-queue`, `data-position-source`, `data-portal-source`, `data-max-toasts-source`, `data-store-source`, and `data-motion-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="sonner-source-controls">
                        <span data-slot="sonner-source-push">
                            <Button on_press=push_source>"Push marker toast"</Button>
                        </span>
                        <span data-slot="sonner-source-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_source>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools."
                    </div>
                    <Sonner
                        store=source_store.get_value()
                        portal=false
                        position=SonnerPosition::TopLeft
                        max_toasts=4
                        aria_label="Status updates".to_string()
                        class_name="docs-sonner-source".to_string()
                        motion=custom_motion
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="sonner-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="sonner-api-rows">
                    <li>
                        <code>"position: SonnerPosition"</code>
                        " "
                        {format!(
                            "default = SonnerPosition::{:?} ({})",
                            SonnerPosition::default(),
                            SonnerPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"portal: bool"</code>
                        " "
                        {format!("default = {}", ui_components::sonner::DEFAULT_PORTAL)}
                    </li>
                    <li>
                        <code>"max_toasts: usize"</code>
                        " "
                        {format!("default = {}", ui_components::sonner::DEFAULT_MAX_TOASTS)}
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui_components::sonner::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"class_name: Option<String>"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"lang: Option<String>, dir: Option<A11yDirection>"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ToastMotion"</code>
                        " default = ToastMotion::default()"
                    </li>
                    <li>
                        <code>"store: Option<ToastStore>"</code>
                        " default path = provided -> context -> local"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="sonner-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="sonner-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = portal | inline"
                    </li>
                    <li>
                        <code>"data-queue"</code>
                        " = single | bounded | extended"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top-left | top-center | top-right | bottom-left | bottom-center | bottom-right"
                    </li>
                    <li>
                        <code>"data-store-source"</code>
                        " = provided | context | local"
                    </li>
                    <li>
                        <code>"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A (Sonner is host config, no controlled/uncontrolled runtime axis)"
                    </li>
                </ul>
            </section>

        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toaster() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));

    let hello_world_code = Signal::derive(move || r#"<Toaster />"#.to_string());

    let push_saved: OnPress = Callback::new(move |_| {
        portal_store.get_value().push_simple("Synced");
    });
    let push_warning: OnPress = Callback::new(move |_| {
        portal_store.get_value().push.run(ToastOptions {
            title: "Action required".to_string(),
            description: Some("Session expires soon.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5000),
        });
    });

    let push_inline: OnPress = Callback::new(move |_| {
        inline_store.get_value().push.run(ToastOptions {
            title: "Draft restored".to_string(),
            description: Some("Recovered from autosave.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(4500),
        });
    });
    let clear_inline: OnPress = Callback::new(move |_| inline_store.get_value().clear.run(()));

    let push_source: OnPress = Callback::new(move |_| {
        source_store.get_value().push.run(ToastOptions {
            title: "Promotion deployed".to_string(),
            description: Some("Store + source markers stay inspectable.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(5200),
        });
    });
    let clear_source: OnPress = Callback::new(move |_| source_store.get_value().clear.run(()));

    let basic_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Toaster store=store.clone() />
store.push_simple("Synced");"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopCenter
  max_toasts=2
  class_name="docs-toaster-inline".to_string()
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopLeft
  max_toasts=4
  aria_label="Alert stream".to_string()
  class_name="docs-toaster-source".to_string()
  motion=ToastMotion {
    initial_y_px: 20.0,
    initial_scale: 0.95,
    ..ToastMotion::default()
  }
/>"#
        .to_string()
    });

    let custom_motion = ToastMotion {
        initial_y_px: 20.0,
        initial_scale: 0.95,
        ..ToastMotion::default()
    };

    view! {
        <ComponentPage
            title="Toaster"
            slug="toaster"
            group="Overlays"
            description="baseline-compatible toast host that composes Sonner/ToastViewport with centralized slot/queue/position/store source-state contracts and baseline-level spring motion handoff."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="ui-muted">
                    "Default path only mounts host; no state primitive wiring or custom store binding required."
                </div>
                <Toaster />
            </Playground>

            <Playground title="Portal Queue Host" code_signal=basic_code>
                <div class="docs-row" data-slot="toaster-portal-controls">
                    <span data-slot="toaster-portal-push-success">
                        <Button variant=ButtonVariant::Secondary on_press=push_saved>
                            "Push success"
                        </Button>
                    </span>
                    <span data-slot="toaster-portal-push-accent">
                        <Button on_press=push_warning>
                            "Push accent"
                        </Button>
                    </span>
                </div>
                <Toaster store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center Host" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-inline-controls">
                        <span data-slot="toaster-inline-push">
                            <Button on_press=push_inline>"Push inline toast"</Button>
                        </span>
                        <span data-slot="toaster-inline-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <Toaster
                        store=inline_store.get_value()
                        portal=false
                        position=ToasterPosition::TopCenter
                        max_toasts=2
                        class_name="docs-toaster-inline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-queue`, `data-position-source`, `data-portal-source`, `data-max-toasts-source`, `data-store-source`, and `data-motion-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-source-controls">
                        <span data-slot="toaster-source-push">
                            <Button on_press=push_source>"Push source toast"</Button>
                        </span>
                        <span data-slot="toaster-source-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_source>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools."
                    </div>
                    <Toaster
                        store=source_store.get_value()
                        portal=false
                        position=ToasterPosition::TopLeft
                        max_toasts=4
                        aria_label="Alert stream".to_string()
                        class_name="docs-toaster-source".to_string()
                        motion=custom_motion
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="toaster-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="toaster-api-rows">
                    <li>
                        <code>"position: ToasterPosition"</code>
                        " "
                        {format!(
                            "default = ToasterPosition::{:?} ({})",
                            ToasterPosition::default(),
                            ToasterPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"portal: bool"</code>
                        " "
                        {format!("default = {}", ui_components::toaster::DEFAULT_PORTAL)}
                    </li>
                    <li>
                        <code>"max_toasts: usize"</code>
                        " "
                        {format!(
                            "default = {}",
                            ui_components::toaster::DEFAULT_MAX_TOASTS
                        )}
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui_components::toaster::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"class_name: Option<String>"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"lang: Option<String>, dir: Option<A11yDirection>"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ToastMotion"</code>
                        " default = ToastMotion::default()"
                    </li>
                    <li>
                        <code>"store: Option<ToastStore>"</code>
                        " default path = provided -> context -> local"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="toaster-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = portal | inline"
                    </li>
                    <li>
                        <code>"data-queue"</code>
                        " = single | bounded | extended"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top-left | top-center | top-right | bottom-left | bottom-center | bottom-right"
                    </li>
                    <li>
                        <code>"data-store-source"</code>
                        " = provided | context | local"
                    </li>
                    <li>
                        <code>"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A (Toaster is host config, no controlled/uncontrolled runtime axis)"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<Toaster />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-toaster-source-copy".to_string()
                />
                <ul data-slot="toaster-source-paths">
                    <li><code>"crates/ui-components/src/toaster/mod.rs"</code></li>
                    <li><code>"crates/ui-components/src/toaster/logic.rs"</code></li>
                    <li><code>"crates/ui-components/src/toaster/view.rs"</code></li>
                    <li><code>"crates/ui-components/src/toaster/styles.rs"</code></li>
                    <li><code>"crates/ui-components/src/toaster/motion.rs"</code></li>
                </ul>
                <ul data-slot="toaster-source-prerequisites">
                    <li><code>"component-toaster"</code></li>
                    <li><code>"component-toast"</code></li>
                    <li><code>"component-sonner"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn underlay() -> AnyView {
    let (open_scrim_raw, set_open_scrim_raw) = signal(false);
    let open_scrim: Signal<bool> = Signal::derive(move || open_scrim_raw.get());

    let close_scrim: OnPress = Callback::new(move |_| set_open_scrim_raw.set(false));
    let on_scrim_open_change = Callback::new(move |next: bool| set_open_scrim_raw.set(next));
    let open_scrim_underlay: OnPress = Callback::new(move |_| set_open_scrim_raw.set(true));

    let (open_transparent_raw, set_open_transparent_raw) = signal(false);
    let open_transparent: Signal<bool> = Signal::derive(move || open_transparent_raw.get());
    let disabled_open: Signal<bool> = Signal::derive(|| true);

    let close_transparent: OnPress = Callback::new(move |_| set_open_transparent_raw.set(false));
    let on_transparent_open_change =
        Callback::new(move |next: bool| set_open_transparent_raw.set(next));
    let open_transparent_underlay: OnPress =
        Callback::new(move |_| set_open_transparent_raw.set(true));

    let (open_source_raw, set_open_source_raw) = signal(false);
    let open_source: Signal<bool> = Signal::derive(move || open_source_raw.get());

    let close_source: OnPress = Callback::new(move |_| set_open_source_raw.set(false));
    let on_source_open_change = Callback::new(move |next: bool| set_open_source_raw.set(next));
    let open_source_underlay: OnPress = Callback::new(move |_| set_open_source_raw.set(true));

    let (open_ai_raw, set_open_ai_raw) = signal(false);
    let open_ai: Signal<bool> = Signal::derive(move || open_ai_raw.get());
    let close_ai: OnPress = Callback::new(move |_| set_open_ai_raw.set(false));
    let on_ai_open_change = Callback::new(move |next: bool| set_open_ai_raw.set(next));
    let open_ai_underlay: OnPress = Callback::new(move |_| set_open_ai_raw.set(true));

    let (ai_mode_raw, set_ai_mode_raw) = signal(AiRenderMode::Snapshot);
    let ai_mode: Signal<AiRenderMode> = Signal::derive(move || ai_mode_raw.get());
    let toggle_ai_mode: OnPress = Callback::new(move |_| {
        set_ai_mode_raw.update(|mode| {
            *mode = match *mode {
                AiRenderMode::Snapshot => AiRenderMode::Streaming,
                AiRenderMode::Streaming => AiRenderMode::Snapshot,
            };
        });
    });

    let (ai_output_status_raw, set_ai_output_status_raw) = signal(AiOutputStatus::Verified);
    let ai_output_status: Signal<AiOutputStatus> =
        Signal::derive(move || ai_output_status_raw.get());
    let cycle_ai_output_status: OnPress = Callback::new(move |_| {
        set_ai_output_status_raw.update(|status| {
            *status = match *status {
                AiOutputStatus::Draft => AiOutputStatus::Verified,
                AiOutputStatus::Verified => AiOutputStatus::Submittable,
                AiOutputStatus::Submittable => AiOutputStatus::Draft,
            };
        });
    });

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<Underlay
  id_base="docs-underlay-basic".to_string()
  is_open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<Underlay
  id_base="docs-underlay-transparent".to_string()
  is_open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  is_transparent=true
  class_name="docs-underlay-custom".to_string()
/>
<Underlay
  id_base="docs-underlay-disabled".to_string()
  is_open=Signal::derive(|| true)
  is_disabled=true
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);

<Underlay
  id_base="docs-underlay-source".to_string()
  is_open=Signal::derive(move || open_raw.get())
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  is_transparent=true
  class_name="docs-underlay-source".to_string()
/>
<Underlay
  id_base="docs-underlay-source-disabled".to_string()
  is_open=Signal::derive(|| true)
  is_disabled=true
  class_name="docs-underlay-disabled-source".to_string()
/>"#
        .to_string()
    });

    let ai_stream_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let (mode_raw, set_mode_raw) = signal(AiRenderMode::Snapshot);
let (status_raw, set_status_raw) = signal(AiOutputStatus::Verified);

<AiSpace
  mode=Signal::derive(move || mode_raw.get())
  output_status=Signal::derive(move || status_raw.get())
>
  <Underlay
    id_base="docs-underlay-ai".to_string()
    is_open=Signal::derive(move || open_raw.get())
    on_open_change=Callback::new(move |next| set_open_raw.set(next))
    on_close=Callback::new(move |_| set_open_raw.set(false))
  />
</AiSpace>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Underlay"
            slug="underlay"
            group="Overlays"
            description="baseline-compatible full-viewport underlay primitive with centralized open/transparent/disabled/close source-state derivation and stable slot/data-state markers."
        >
            <Playground title="Scrim + Click To Close" code_signal=code>
                <div class="docs-row">
                    <Button on_press=open_scrim_underlay>
                        {move || if open_scrim_raw.get() { "Underlay open" } else { "Open underlay" }}
                    </Button>
                    <span class="ui-muted">"open: " {move || open_scrim_raw.get()}</span>
                </div>

                <Underlay
                    id_base="docs-underlay-basic".to_string()
                    is_open=open_scrim
                    on_open_change=on_scrim_open_change
                    on_close=close_scrim
                />
            </Playground>

            <Playground title="Transparent + Disabled + Custom Class" code_signal=state_code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=open_transparent_underlay>
                        {move || {
                            if open_transparent_raw.get() {
                                "Transparent underlay open"
                            } else {
                                "Open transparent underlay"
                            }
                        }}
                    </Button>
                    <span class="ui-muted">
                        "transparent open: " {move || open_transparent_raw.get()}
                    </span>
                </div>

                <Underlay
                    id_base="docs-underlay-transparent".to_string()
                    is_open=open_transparent
                    on_open_change=on_transparent_open_change
                    is_transparent=true
                    class_name="docs-underlay-custom".to_string()
                    on_close=close_transparent
                />

                <Underlay
                    id_base="docs-underlay-disabled".to_string()
                    is_open=disabled_open
                    is_disabled=true
                    class_name="docs-underlay-disabled".to_string()
                />
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-tone`, `data-close-mode`, `data-transparent-source`, `data-disabled-source`, `data-close-source`, and `data-class-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_source_underlay>"Open source underlay"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_source>
                            "Close"
                        </Button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-open-mode / data-open-source / data-open-change-source / data-transparent-source / data-disabled-source / data-close-source / data-class-source in DevTools."
                    </div>

                    <Underlay
                        id_base="docs-underlay-source".to_string()
                        is_open=open_source
                        on_open_change=on_source_open_change
                        is_transparent=true
                        class_name="docs-underlay-source".to_string()
                        on_close=close_source
                    />

                    <Underlay
                        id_base="docs-underlay-source-disabled".to_string()
                        is_open=Signal::derive(|| true)
                        is_disabled=true
                        class_name="docs-underlay-disabled-source".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="LLM Render Modes (Snapshot + Streaming)"
                description="`Underlay` is not a text-reader surface, so streaming is optional with explicit `fallback=snapshot`; output status stays observable via `data-ui-output-status`."
                code_signal=ai_stream_code
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/underlay/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="underlay-ai-demo">
                    <div class="docs-row" data-slot="underlay-ai-controls">
                        <button type="button" data-action="open" on:click=move |_| open_ai_underlay.run(())>
                            "Open AI underlay"
                        </button>
                        <button type="button" data-action="close" on:click=move |_| close_ai.run(())>
                            "Close"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-mode"
                            on:click=move |_| toggle_ai_mode.run(())
                        >
                            "Toggle mode"
                        </button>
                        <button
                            type="button"
                            data-action="cycle-status"
                            on:click=move |_| cycle_ai_output_status.run(())
                        >
                            "Cycle status"
                        </button>
                    </div>
                    <span class="ui-muted" data-slot="underlay-ai-runtime">
                        "mode: " {move || ai_mode_raw.get().as_str()} " | status: "
                        {move || ai_output_status_raw.get().as_str()} " | open: "
                        {move || open_ai_raw.get()}
                    </span>
                    <div class="ui-muted">
                        "Inspect data-ui-stream-support / data-ui-stream-fallback / data-ui-stream-mode / data-ui-output-status on the underlay root."
                    </div>
                    <div class="ui-muted">
                        "Copy-ready snippets auto-include `use leptos::prelude::*; use ui_components::*;` (requires `ui-components` with `component-underlay`)."
                    </div>
                    <AiSpace mode=ai_mode output_status=ai_output_status>
                        <Underlay
                            id_base="docs-underlay-ai".to_string()
                            is_open=open_ai
                            on_open_change=on_ai_open_change
                            on_close=close_ai
                        />
                    </AiSpace>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
