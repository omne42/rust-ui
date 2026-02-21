use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, HoverCard, HoverCardMotion, OnPress};

const HOVER_CARD_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant, HoverCard, HoverCardMotion, OnPress};";

pub(super) fn hover_card() -> AnyView {
    let code = Signal::derive(move || {
        r##"<HoverCard content=move || view!{ <div>...</div> }>
  <a href="#">"Hover"</a>
</HoverCard>"##
            .to_string()
    });

    let hello_world_code = Signal::derive(move || {
        r##"<HoverCard content=move || view! { "Hello World" }>
  <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
</HoverCard>"##
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r##"<HoverCard
  open_delay_ms=220
  close_delay_ms=260
  class_name="docs-hover-card-state".to_string()
  id="docs-hover-card".to_string()
  motion=HoverCardMotion {
    initial_scale: 0.96,
    offset_y_px: 14.0,
    ..HoverCardMotion::default()
  }
  content=move || view! {
    <div class="docs-stack docs-stack--tight">
      <div>"State + source markers"</div>
      <div class="ui-muted">"Inspect data-delay-source and data-id-source on root/trigger/panel."</div>
    </div>
  }
>
  <Button variant=ButtonVariant::Secondary>"Inspect markers"</Button>
</HoverCard>"##.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div attr:data-slot="hover-card-state-matrix">
  <HoverCard content=move || view! { "Default state" }>
    <Button variant=ButtonVariant::Secondary>"Default"</Button>
  </HoverCard>
  <HoverCard open_delay_ms=220 close_delay_ms=260 content=move || view! { "Delayed state" }>
    <Button variant=ButtonVariant::Secondary>"Delayed"</Button>
  </HoverCard>
  <HoverCard is_disabled=true content=move || view! { "Disabled state" }>
    <Button variant=ButtonVariant::Secondary>"Disabled"</Button>
  </HoverCard>
</div>"##
            .to_string()
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change: Callback<bool> =
        Callback::new(move |next| set_compare_controlled_open_raw.set(next));
    let (compare_uncontrolled_events_raw, set_compare_uncontrolled_events_raw) = signal(0u32);
    let on_compare_uncontrolled_open_change: Callback<bool> =
        Callback::new(move |_| set_compare_uncontrolled_events_raw.update(|count| *count += 1));

    let controlled_uncontrolled_code = Signal::derive(move || {
        r##"let (controlled_open_raw, set_controlled_open_raw) = signal(false);
let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
let on_controlled_open_change: Callback<bool> =
  Callback::new(move |next| set_controlled_open_raw.set(next));
let on_uncontrolled_open_change: Callback<bool> = Callback::new(move |_| {});

<HoverCard
  is_open=controlled_open
  on_open_change=on_controlled_open_change
  content=move || view! { "Controlled" }
>
  <Button variant=ButtonVariant::Secondary>"Controlled trigger"</Button>
</HoverCard>
<HoverCard
  default_open=true
  on_open_change=on_uncontrolled_open_change
  content=move || view! { "Uncontrolled" }
>
  <Button variant=ButtonVariant::Secondary>"Uncontrolled trigger"</Button>
</HoverCard>"##
            .to_string()
    });

    let streaming_snapshot_code = Signal::derive(move || {
        r##"<div attr:data-slot="hover-card-streaming-snapshot">
  <div>"requested mode: streaming (optional)"</div>
  <div>"fallback=snapshot"</div>
  <div>"requested output status: draft -> verified"</div>
  <div>"effective component status: data-ui-output-status=verified"</div>
</div>"##
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r##"let custom_motion = HoverCardMotion {
  initial_scale: 0.93,
  offset_y_px: 18.0,
  ..HoverCardMotion::default()
};

<HoverCard
  motion=custom_motion
  content=move || view! { "Custom spring + offset motion" }
>
  <Button variant=ButtonVariant::Secondary>"Custom motion"</Button>
</HoverCard>
<HoverCard
  motion=HoverCardMotion::default()
  content=move || view! { "Default motion" }
>
  <Button variant=ButtonVariant::Secondary>"Default motion"</Button>
</HoverCard>"##
            .to_string()
    });

    let (interactive_open_raw, set_interactive_open_raw) = signal(false);
    let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());
    let open_interactive_hover_card: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(true));
    let close_interactive_hover_card: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(false));
    let on_interactive_open_change: Callback<bool> =
        Callback::new(move |next| set_interactive_open_raw.set(next));

    let interactive_code = Signal::derive(move || {
        r##"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let on_open_change: Callback<bool> = Callback::new(move |next| set_open_raw.set(next));
let open_hover_card: OnPress = Callback::new(move |_| set_open_raw.set(true));
let close_hover_card: OnPress = Callback::new(move |_| set_open_raw.set(false));

<div attr:data-slot="hover-card-e2e-controls">
  <Button attr:data-slot="hover-card-e2e-open" on_press=open_hover_card>
    "Open interactive hover card"
  </Button>
  <Button attr:data-slot="hover-card-e2e-close" variant=ButtonVariant::Secondary on_press=close_hover_card>
    "Close interactive hover card"
  </Button>
</div>

<div attr:data-slot="hover-card-e2e-canvas">
  <HoverCard
    is_open=open
    on_open_change=on_open_change
    id="docs-hover-card-interactive".to_string()
    content=move || view! { "Interactive content" }
  >
    <Button attr:data-slot="hover-card-e2e-trigger" variant=ButtonVariant::Secondary>
      "Interactive trigger"
    </Button>
  </HoverCard>
</div>"##
            .to_string()
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/hover-card/src/styles.rs */\n{}",
            ui_components::hover_card::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        format!(
            "HoverCardActualConfig {{\n  open: {},\n  mode: \"controlled\",\n  value_source: \"external\",\n  intent_source: \"interaction\",\n}}",
            interactive_open_raw.get()
        )
    });

    let custom_motion = HoverCardMotion {
        initial_scale: 0.93,
        offset_y_px: 18.0,
        ..HoverCardMotion::default()
    };

    view! {
        <ComponentPage
            title="HoverCard"
            slug="hover-card"
            group="Overlays"
            description="Hover/focus triggered card with open/close delays."
        >
            <Playground title="HoverCard" code_signal=code>
                <div class="docs-row" data-visual-baseline="hover-card-default-theme">
                    <HoverCard content=move || view! {
                        <div class="docs-stack">
                            <div>"HoverCard content"</div>
                            <div class="ui-muted">"Moves with placement + spring enter/exit."</div>
                        </div>
                    }>
                        <a href="#" class="ui-muted" on:click=move |ev| ev.prevent_default()>
                            "Hover me"
                        </a>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="Hello World (Minimal Path)"
                code_signal=hello_world_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <HoverCard content=move || view! { "Hello World" }>
                        <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root/trigger/panel contracts: data-state/data-open/data-motion-source/data-delay-source/data-id-source."
                code_signal=markers_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <HoverCard
                        open_delay_ms=220
                        close_delay_ms=260
                        class_name="docs-hover-card-state".to_string()
                        id="docs-hover-card".to_string()
                        motion=HoverCardMotion {
                            initial_scale: 0.96,
                            offset_y_px: 14.0,
                            ..HoverCardMotion::default()
                        }
                        content=move || view! {
                            <div class="docs-stack docs-stack--tight">
                                <div>"State + source markers"</div>
                                <div class="ui-muted">
                                    "Inspect data-delay-source and data-id-source on root/trigger/panel."
                                </div>
                            </div>
                        }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Inspect markers"
                        </Button>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                description="Default/delayed/disabled branches for semantic matrix regression."
                code_signal=state_matrix_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="hover-card-state-matrix">
                    <HoverCard content=move || view! { "Default state" }>
                        <Button variant=ButtonVariant::Secondary>"Default"</Button>
                    </HoverCard>
                    <HoverCard
                        open_delay_ms=220
                        close_delay_ms=260
                        content=move || view! { "Delayed state" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Delayed"</Button>
                    </HoverCard>
                    <HoverCard is_disabled=true content=move || view! { "Disabled state" }>
                        <Button variant=ButtonVariant::Secondary>"Disabled"</Button>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_uncontrolled_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="hover-card-controlled-uncontrolled">
                    <HoverCard
                        is_open=compare_controlled_open
                        on_open_change=on_compare_controlled_open_change
                        content=move || view! { "Controlled content" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Controlled trigger"</Button>
                    </HoverCard>
                    <HoverCard
                        default_open=true
                        on_open_change=on_compare_uncontrolled_open_change
                        content=move || view! { "Uncontrolled content" }
                    >
                        <Button variant=ButtonVariant::Secondary>"Uncontrolled trigger"</Button>
                    </HoverCard>
                    <span class="ui-muted">
                        "controlled open: " {move || compare_controlled_open_raw.get()}
                    </span>
                    <span class="ui-muted">
                        "uncontrolled events: " {move || compare_uncontrolled_events_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="HoverCard is streaming-optional and falls back to snapshot while preserving semantic markers."
                code_signal=streaming_snapshot_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="hover-card-streaming-snapshot">
                    <div>"requested mode: streaming (optional)"</div>
                    <div>"fallback=snapshot"</div>
                    <div>"requested output status: draft -> verified"</div>
                    <div>"effective component status: data-ui-output-status=verified"</div>
                </div>
            </Playground>

            <Playground
                title="Custom Motion Contract"
                code_signal=motion_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <HoverCard
                        motion=custom_motion
                        content=move || view! { "Custom spring + offset motion" }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Custom motion"
                        </Button>
                    </HoverCard>
                    <HoverCard
                        motion=HoverCardMotion::default()
                        content=move || view! { "Default motion" }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Default motion"
                        </Button>
                    </HoverCard>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + CSS Test: keep controlled-open context while tuning styles."
                code_signal=interactive_code
                code_imports=HOVER_CARD_DOC_IMPORTS.to_string()
                test_css_source=interactive_test_css
                test_source_path="components/hover-card/src/styles.rs".to_string()
                test_config_signal=interactive_config
            >
                <div class="docs-row">
                    <div class="docs-row" attr:data-slot="hover-card-e2e-controls">
                        <Button attr:data-slot="hover-card-e2e-open" on_press=open_interactive_hover_card>
                            "Open interactive hover card"
                        </Button>
                        <Button
                            attr:data-slot="hover-card-e2e-close"
                            variant=ButtonVariant::Secondary
                            on_press=close_interactive_hover_card
                        >
                            "Close interactive hover card"
                        </Button>
                        <span class="ui-muted">"open: " {move || interactive_open_raw.get()}</span>
                    </div>

                    <div class="docs-row" attr:data-slot="hover-card-e2e-canvas">
                        <HoverCard
                            is_open=interactive_open
                            on_open_change=on_interactive_open_change
                            id="docs-hover-card-interactive".to_string()
                            content=move || view! {
                                <div class="docs-stack docs-stack--tight">
                                    <div>"Interactive content"</div>
                                    <div class="ui-muted">
                                        "Inspect root markers in DevTools while keeping context."
                                    </div>
                                </div>
                            }
                        >
                            <Button attr:data-slot="hover-card-e2e-trigger" variant=ButtonVariant::Secondary>
                                "Interactive trigger"
                            </Button>
                        </HoverCard>
                    </div>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" attr:data-slot="hover-card-defaults-contract">
                <h3>"API + Defaults Contract"</h3>
                <p class="ui-muted">
                    "Defaults are normalized in "
                    <code>"components/hover-card/src/logic.rs"</code>
                    "."
                </p>
                <ul class="docs-list docs-list--tight">
                    <li>
                        <code>"open_delay_ms"</code>
                        " default: "
                        <code>"DEFAULT_OPEN_DELAY_MS (140)"</code>
                    </li>
                    <li>
                        <code>"close_delay_ms"</code>
                        " default: "
                        <code>"DEFAULT_CLOSE_DELAY_MS (180)"</code>
                    </li>
                    <li>
                        <code>"is_disabled"</code>
                        " default: "
                        <code>"false"</code>
                        " (via "
                        <code>"resolve_is_disabled"</code>
                        ")."
                    </li>
                    <li>
                        <code>"is_open/default_open/on_open_change"</code>
                        " map to controlled/uncontrolled open-axis."
                    </li>
                </ul>
            </div>

            <div class="docs-stack docs-stack--tight" attr:data-slot="hover-card-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " to copy runnable snippets with "
                    <code>"HOVER_CARD_DOC_IMPORTS"</code>
                    ". Imports are completed by "
                    <code>"compose_copy_ready_code"</code>
                    " in playground pipeline."
                </p>
                <ul class="docs-list docs-list--tight" attr:data-slot="hover-card-source-paths">
                    <li><code>"components/hover-card/src/mod.rs"</code></li>
                    <li><code>"components/hover-card/src/logic.rs"</code></li>
                    <li><code>"components/hover-card/src/view.rs"</code></li>
                    <li><code>"components/hover-card/src/styles.rs"</code></li>
                    <li><code>"components/hover-card/src/motion.rs"</code></li>
                </ul>
                <p class="ui-muted">
                    "Required package features: "
                    <code>"component-hover_card"</code>
                    " + "
                    <code>"inject-css"</code>
                    "."
                </p>
            </div>
        </ComponentPage>
    }
    .into_any()
}
