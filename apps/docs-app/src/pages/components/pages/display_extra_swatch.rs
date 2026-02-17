use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Swatch, SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize};

pub(super) fn swatch() -> AnyView {
    let size_code = Signal::derive(move || {
        r##"<Swatch color="#ffcc00".to_string() size=SwatchSize::Xs />
<Swatch color="#ffcc00".to_string() size=SwatchSize::S />
<Swatch color="#ffcc00".to_string() size=SwatchSize::M />
<Swatch
  color="#ffcc00".to_string()
  size=SwatchSize::L
  shape=SwatchShape::Rectangle
  rounding=SwatchRounding::Full
  border=SwatchBorder::Light
/>"##
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r##"let (selected, set_selected) = signal(true);
let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));

<Swatch
  color="rgba(38, 99, 235, 0.35)".to_string()
  label="Brand blue".to_string()
  selected=move || selected.get()
  on_selected_change=on_selected_change
/>
<Swatch mixed_value=true size=SwatchSize::L shape=SwatchShape::Rectangle />
<Swatch nothing=true border=SwatchBorder::None rounding=SwatchRounding::Full />
<Swatch color="#111827".to_string() disabled=true />"##
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r##"let custom_motion = SwatchMotion {
  selected_scale: 1.12,
  selected_ring_opacity: 0.92,
  ..SwatchMotion::default()
};

<Swatch
  color="#7c3aed".to_string()
  label="Featured motion".to_string()
  selected=Signal::derive(|| true)
  motion=custom_motion
/>
<Swatch
  color="#7c3aed".to_string()
  label="Reduced motion".to_string()
  selected=Signal::derive(|| true)
  motion=SwatchMotion::disabled()
/>"##
            .to_string()
    });

    let (selected, set_selected) = signal(true);
    let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));

    let custom_motion = SwatchMotion {
        selected_scale: 1.12,
        selected_ring_opacity: 0.92,
        ..SwatchMotion::default()
    };

    view! {
        <ComponentPage
            title="Swatch"
            slug="swatch"
            group="Display"
            description="baseline-compatible swatch primitive with centralized size/shape/rounding/border/state contracts and baseline-level spring selection motion."
        >
            <Playground title="Size + Shape + Rounding" code_signal=size_code>
                <div class="docs-row">
                    <Swatch color="#ffcc00".to_string() size=SwatchSize::Xs />
                    <Swatch color="#ffcc00".to_string() size=SwatchSize::S />
                    <Swatch color="#ffcc00".to_string() size=SwatchSize::M />
                    <Swatch
                        color="#ffcc00".to_string()
                        size=SwatchSize::L
                        shape=SwatchShape::Rectangle
                        rounding=SwatchRounding::Full
                        border=SwatchBorder::Light
                    />
                </div>
            </Playground>

            <Playground title="Mixed + Nothing + Disabled + Controlled" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <Swatch
                        color="rgba(38, 99, 235, 0.35)".to_string()
                        label="Brand blue".to_string()
                        selected=move || selected.get()
                        on_selected_change=on_selected_change
                    />
                    <div class="ui-muted">
                        {move || format!("Selected: {}", selected.get())}
                    </div>
                    <Swatch mixed_value=true size=SwatchSize::L shape=SwatchShape::Rectangle />
                    <Swatch
                        nothing=true
                        border=SwatchBorder::None
                        rounding=SwatchRounding::Full
                    />
                    <Swatch color="#111827".to_string() disabled=true />
                </div>
            </Playground>

            <Playground title="Custom Motion Contract" code_signal=motion_code>
                <div class="docs-row">
                    <Swatch
                        color="#7c3aed".to_string()
                        label="Featured motion".to_string()
                        selected=Signal::derive(|| true)
                        motion=custom_motion
                    />
                    <Swatch
                        color="#7c3aed".to_string()
                        label="Reduced motion".to_string()
                        selected=Signal::derive(|| true)
                        motion=SwatchMotion::disabled()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
