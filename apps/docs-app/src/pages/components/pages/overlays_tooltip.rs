use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, Tooltip, TooltipMotion};
use ui_layout::{Flex, FlexAlign, FlexDirection, FlexGap, FlexWrap};

pub(super) fn tooltip() -> AnyView {
    let code = Signal::derive(move || {
        r#"<Tooltip content=move || view!{ "Tooltip" }>
  <Button>"Hover"</Button>
</Tooltip>"#
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r##"<Tooltip
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
</Tooltip>"##
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r##"let custom_motion = TooltipMotion {
  initial_scale: 0.92,
  offset_y_px: 14.0,
  ..TooltipMotion::default()
};

<Tooltip
  motion=custom_motion
  content=move || view! { "Custom spring + placement offset" }
>
  <Button variant=ButtonVariant::Secondary>"Custom motion"</Button>
</Tooltip>
<Tooltip
  motion=TooltipMotion::default()
  content=move || view! { "Default motion" }
>
  <Button variant=ButtonVariant::Secondary>"Default motion"</Button>
</Tooltip>"##
            .to_string()
    });

    let custom_motion = TooltipMotion {
        initial_scale: 0.92,
        offset_y_px: 14.0,
        ..TooltipMotion::default()
    };

    view! {
        <ComponentPage
            title="Tooltip"
            slug="tooltip"
            group="Overlays"
            description="Tooltip with delay/warmup/cooldown and anchor positioning."
        >
            <Playground title="Hover / focus" code_signal=code>
                <Flex
                    align=FlexAlign::Center
                    gap=FlexGap::Sm
                    wrap=FlexWrap::Wrap
                    class_name="docs-row".to_string()
                >
                    <Tooltip content=move || view! { "This is a tooltip" }>
                        <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
                    </Tooltip>
                    <Tooltip content=move || view! { "Disabled" } disabled=true>
                        <Button variant=ButtonVariant::Secondary is_disabled=true>"Disabled"</Button>
                    </Tooltip>
                </Flex>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-delay-source`, `data-trigger-source`, `data-press-source`, and `data-id-source` contracts."
                code_signal=markers_code
            >
                <Flex
                    align=FlexAlign::Center
                    gap=FlexGap::Sm
                    wrap=FlexWrap::Wrap
                    class_name="docs-row".to_string()
                >
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
                            <Flex
                                direction=FlexDirection::Column
                                gap=FlexGap::Xs
                                class_name="docs-stack docs-stack--tight".to_string()
                            >
                                <div>"Marker contracts"</div>
                                <div class="ui-muted">
                                    "Inspect data-delay-source/data-trigger-source/data-id-source."
                                </div>
                            </Flex>
                        }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Inspect tooltip"
                        </Button>
                    </Tooltip>
                </Flex>
            </Playground>

            <Playground title="Custom Motion Contract" code_signal=motion_code>
                <Flex
                    align=FlexAlign::Center
                    gap=FlexGap::Sm
                    wrap=FlexWrap::Wrap
                    class_name="docs-row".to_string()
                >
                    <Tooltip
                        motion=custom_motion
                        content=move || view! { "Custom spring + placement offset" }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Custom motion"
                        </Button>
                    </Tooltip>
                    <Tooltip
                        motion=TooltipMotion::default()
                        content=move || view! { "Default motion" }
                    >
                        <Button variant=ButtonVariant::Secondary>
                            "Default motion"
                        </Button>
                    </Tooltip>
                </Flex>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
