use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, HoverCard, HoverCardMotion};

pub(super) fn hover_card() -> AnyView {
    let code = r##"<HoverCard content=move || view!{ <div>...</div> }>
  <a href="#">"Hover"</a>
</HoverCard>"##;

    let markers_code = r##"<HoverCard
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
</HoverCard>"##;

    let motion_code = r##"let custom_motion = HoverCardMotion {
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
</HoverCard>"##;

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
            <Playground title="HoverCard" code=code>
                <div class="docs-row">
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
                title="State + Source Markers"
                description="Inspect root/trigger/panel contracts: data-state/data-open/data-motion-source/data-delay-source/data-id-source."
                code=markers_code
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

            <Playground title="Custom Motion Contract" code=motion_code>
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
        </ComponentPage>
    }
    .into_any()
}
