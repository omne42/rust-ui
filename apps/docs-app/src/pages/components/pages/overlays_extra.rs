use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, OnPress, Tray, Underlay};

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
  is_fixed_height=true
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

            <Playground title="Fixed Height + Title Only + Custom Class" code=custom_code>
                <div class="docs-row">
                    <Button on_press=open_custom_tray>"Open fixed tray"</Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get().to_string()}</span>
                </div>

                <Show when=move || present_custom.get()>
                    <Tray
                        open=open_custom
                        id_base="docs-tray-fixed".to_string()
                        title="Fixed tray".to_string()
                        is_fixed_height=true
                        show_close_button=false
                        class_name="docs-tray-custom".to_string()
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only path keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">"Custom class validates merge + state attrs."</div>
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

    view! {
        <ComponentPage
            title="Underlay"
            slug="underlay"
            group="Overlays"
            description="Spectrum-compatible full-viewport underlay primitive with centralized open/transparent/disabled state derivation, close-interaction contracts, and stable slot/data-state markers."
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
        </ComponentPage>
    }
    .into_any()
}
