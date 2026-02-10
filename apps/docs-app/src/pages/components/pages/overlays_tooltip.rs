use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, Tooltip, TooltipMotion};

pub(super) fn tooltip() -> AnyView {
    let code = r#"<Tooltip content=move || view!{ "Tooltip" }>
  <Button>"Hover"</Button>
</Tooltip>"#;

    let markers_code = r##"<Tooltip
  delay_ms=300
  close_delay_ms=200
  should_close_on_press=false
  class_name="docs-tooltip-state".to_string()
  id="docs-tooltip".to_string()
  motion=TooltipMotion {
    initial_scale: 0.95,
    offset_y_px: 10.0,
    ..TooltipMotion::default()
  }
  content=move || view! {
    <div class="docs-stack docs-stack--tight">
      <div>"Marker contracts"</div>
      <div class="ui-muted">"Inspect data-delay-source/data-trigger-source/data-id-source."</div>
    </div>
  }
>
  <Button variant=ButtonVariant::Secondary>"Inspect tooltip"</Button>
</Tooltip>"##;

    view! {
        <ComponentPage
            title="Tooltip"
            slug="tooltip"
            group="Overlays"
            description="Tooltip with delay/warmup/cooldown and anchor positioning."
        >
            <Playground title="Hover / focus" code=code>
                <div class="docs-row">
                    <Tooltip content=move || view! { "This is a tooltip" }>
                        <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
                    </Tooltip>
                    <Tooltip content=move || view! { "Disabled" } disabled=true>
                        <Button variant=ButtonVariant::Secondary disabled=true>"Disabled"</Button>
                    </Tooltip>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-delay-source`, `data-trigger-source`, `data-press-source`, and `data-id-source` contracts."
                code=markers_code
            >
                <div class="docs-row">
                    <Tooltip
                        delay_ms=300
                        close_delay_ms=200
                        should_close_on_press=false
                        class_name="docs-tooltip-state".to_string()
                        id="docs-tooltip".to_string()
                        motion=TooltipMotion {
                            initial_scale: 0.95,
                            offset_y_px: 10.0,
                            ..TooltipMotion::default()
                        }
                        content=move || view! {
                            <div class="docs-stack docs-stack--tight">
                                <div>"Marker contracts"</div>
                                <div class="ui-muted">
                                    "Inspect data-delay-source/data-trigger-source/data-id-source."
                                </div>
                            </div>
                        }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Inspect tooltip"
                        </Button>
                    </Tooltip>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
