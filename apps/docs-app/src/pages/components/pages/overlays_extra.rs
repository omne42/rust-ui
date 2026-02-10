use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    BottomSheet, BottomSheetMotion, Button, ButtonVariant, OnPress, Sonner, SonnerPosition,
    ToastMotion, ToastOptions, ToastStoreOptions, ToastVariant, Toaster, ToasterPosition, Tray,
    TrayMotion, Underlay, provide_toast_store,
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

    let semantic_code = r#"<BottomSheet
  open=open
  id_base="bottom-sheet".to_string()
  title="Update available".to_string()
  description="A newer version with security improvements is ready to install.".to_string()
  on_close=close
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#;

    let detached_code = r#"<BottomSheet
  open=open
  id_base="bottom-sheet-detached".to_string()
  title="Quick actions".to_string()
  detached=true
  bottom_inset_px=16.0
  show_close_button=false
  class_name="docs-bottom-sheet-custom".to_string()
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#;

    let custom_motion_code = r#"<BottomSheet
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
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#;

    view! {
        <ComponentPage
            title="BottomSheet"
            slug="bottom-sheet"
            group="Overlays"
            description="Spectrum/HeroUI-style bottom sheet primitive composed from Sheet with centralized handle/description/footer/detached contracts and stable slot/data-state markers."
        >
            <Playground title="Semantic Bottom Sheet" code=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_sheet>"Open bottom sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get().to_string()}</span>
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

            <Playground title="Detached + Title Only + Custom Class" code=detached_code>
                <div class="docs-row">
                    <Button on_press=open_detached_sheet>"Open detached sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_detached_raw.get().to_string()}</span>
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

            <Playground title="Custom Motion Contract" code=custom_motion_code>
                <div class="docs-row">
                    <Button on_press=open_custom_motion_sheet>"Open custom motion sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_motion_raw.get().to_string()}</span>
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

    let semantic_code = r#"<Tray
  open=open
  id_base="tray".to_string()
  title="Filters".to_string()
  description="Bottom tray with semantic heading + footer actions.".to_string()
  on_close=close
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</Tray>"#;

    let custom_code = r#"<Tray
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
  on_close=close
  on_exit_complete=finish_exit
>
  ...
</Tray>"#;

    view! {
        <ComponentPage
            title="Tray"
            slug="tray"
            group="Overlays"
            description="Spectrum-compatible bottom tray primitive composed from Sheet with centralized description/footer/close/height contracts and stable slot/data-state markers."
        >
            <Playground title="Tray + Footer Actions" code=semantic_code>
                <div class="docs-row">
                    <Button on_press=open_semantic_tray>"Open tray"</Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get().to_string()}</span>
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
                code=custom_code
            >
                <div class="docs-row">
                    <Button on_press=open_custom_tray>"Open fixed tray"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get().to_string()}</span>
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

    let basic_code = r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Sonner store=store.clone() />
store.push_simple("Saved");"#;

    let state_code = r#"<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopCenter
  max_toasts=2
  class_name="docs-sonner-inline".to_string()
/>"#;

    let source_code = r#"<Sonner
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
/>"#;

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
            description="Shadcn/HeroUI-style toast host that composes ToastViewport with position presets, queue limits, and stable Sonner slot/source-state data contracts."
        >
            <Playground title="Portal Queue + Variants" code=basic_code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=push_saved>
                        "Push success"
                    </Button>
                    <Button variant=ButtonVariant::Destructive on_press=push_danger>
                        "Push danger"
                    </Button>
                </div>
                <Sonner store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center + Max Queue" code=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_inline>"Push accent"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                            "Clear"
                        </Button>
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
                code=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_source>"Push marker toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_source>
                            "Clear"
                        </Button>
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toaster() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));

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

    let basic_code = r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Toaster store=store.clone() />
store.push_simple("Synced");"#;

    let state_code = r#"<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopCenter
  max_toasts=2
  class_name="docs-toaster-inline".to_string()
/>"#;

    let source_code = r#"<Toaster
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
/>"#;

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
            description="Shadcn-compatible toast host that composes Sonner/ToastViewport with centralized slot/queue/position/store source-state contracts and HeroUI-level spring motion handoff."
        >
            <Playground title="Portal Queue Host" code=basic_code>
                <div class="docs-row">
                    <Button variant=ButtonVariant::Secondary on_press=push_saved>
                        "Push success"
                    </Button>
                    <Button on_press=push_warning>
                        "Push accent"
                    </Button>
                </div>
                <Toaster store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center Host" code=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_inline>"Push inline toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                            "Clear"
                        </Button>
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
                code=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_source>"Push source toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_source>
                            "Clear"
                        </Button>
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn underlay() -> AnyView {
    let (open_scrim_raw, set_open_scrim_raw) = signal(false);
    let open_scrim: Signal<bool> = Signal::derive(move || open_scrim_raw.get());

    let close_scrim: OnPress = Callback::new(move |_| set_open_scrim_raw.set(false));
    let open_scrim_underlay: OnPress = Callback::new(move |_| set_open_scrim_raw.set(true));

    let (open_transparent_raw, set_open_transparent_raw) = signal(false);
    let open_transparent: Signal<bool> = Signal::derive(move || open_transparent_raw.get());
    let disabled_open: Signal<bool> = Signal::derive(|| true);

    let close_transparent: OnPress = Callback::new(move |_| set_open_transparent_raw.set(false));
    let open_transparent_underlay: OnPress =
        Callback::new(move |_| set_open_transparent_raw.set(true));

    let (open_source_raw, set_open_source_raw) = signal(false);
    let open_source: Signal<bool> = Signal::derive(move || open_source_raw.get());

    let close_source: OnPress = Callback::new(move |_| set_open_source_raw.set(false));
    let open_source_underlay: OnPress = Callback::new(move |_| set_open_source_raw.set(true));

    let code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());

<Underlay
  id_base="docs-underlay-basic".to_string()
  open=open_signal
  on_close=Callback::new(move |_| set_open.set(false))
/>"#;

    let state_code = r#"<Underlay
  id_base="docs-underlay-transparent".to_string()
  open=open_signal
  transparent=true
  class_name="docs-underlay-custom".to_string()
  on_close=close
/>
<Underlay
  id_base="docs-underlay-disabled".to_string()
  open=Signal::derive(|| true)
  disabled=true
/>"#;

    let source_code = r#"<Underlay
  id_base="docs-underlay-source".to_string()
  open=open_signal
  transparent=true
  class_name="docs-underlay-source".to_string()
  on_close=close
/>
<Underlay
  id_base="docs-underlay-source-disabled".to_string()
  open=Signal::derive(|| true)
  disabled=true
  class_name="docs-underlay-disabled-source".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Underlay"
            slug="underlay"
            group="Overlays"
            description="Spectrum-compatible full-viewport underlay primitive with centralized open/transparent/disabled/close source-state derivation and stable slot/data-state markers."
        >
            <Playground title="Scrim + Click To Close" code=code>
                <div class="docs-row">
                    <Button on_press=open_scrim_underlay>
                        {move || if open_scrim_raw.get() { "Underlay open" } else { "Open underlay" }}
                    </Button>
                    <span class="ui-muted">"open: " {move || open_scrim_raw.get().to_string()}</span>
                </div>

                <Underlay
                    id_base="docs-underlay-basic".to_string()
                    open=open_scrim
                    on_close=close_scrim
                />
            </Playground>

            <Playground title="Transparent + Disabled + Custom Class" code=state_code>
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
                        "transparent open: " {move || open_transparent_raw.get().to_string()}
                    </span>
                </div>

                <Underlay
                    id_base="docs-underlay-transparent".to_string()
                    open=open_transparent
                    transparent=true
                    class_name="docs-underlay-custom".to_string()
                    on_close=close_transparent
                />

                <Underlay
                    id_base="docs-underlay-disabled".to_string()
                    open=disabled_open
                    disabled=true
                    class_name="docs-underlay-disabled".to_string()
                />
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-tone`, `data-close-mode`, `data-transparent-source`, `data-disabled-source`, `data-close-source`, and `data-class-source` contracts."
                code=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_source_underlay>"Open source underlay"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_source>
                            "Close"
                        </Button>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-transparent-source / data-disabled-source / data-close-source / data-class-source in DevTools."
                    </div>

                    <Underlay
                        id_base="docs-underlay-source".to_string()
                        open=open_source
                        transparent=true
                        class_name="docs-underlay-source".to_string()
                        on_close=close_source
                    />

                    <Underlay
                        id_base="docs-underlay-source-disabled".to_string()
                        open=Signal::derive(|| true)
                        disabled=true
                        class_name="docs-underlay-disabled-source".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
