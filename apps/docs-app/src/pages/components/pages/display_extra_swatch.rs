use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Swatch, SwatchBorder, SwatchRounding, SwatchShape, SwatchSize};

pub(super) fn swatch() -> AnyView {
    let size_code = r##"<Swatch color="#ffcc00".to_string() size=SwatchSize::Xs />
<Swatch color="#ffcc00".to_string() size=SwatchSize::S />
<Swatch color="#ffcc00".to_string() size=SwatchSize::M />
<Swatch
  color="#ffcc00".to_string()
  size=SwatchSize::L
  shape=SwatchShape::Rectangle
  rounding=SwatchRounding::Full
  border=SwatchBorder::Light
/>"##;

    let state_code = r##"let (selected, set_selected) = signal(true);
let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));

<Swatch
  color="rgba(38, 99, 235, 0.35)".to_string()
  label="Brand blue".to_string()
  selected=move || selected.get()
  on_selected_change=on_selected_change
/>
<Swatch mixed_value=true size=SwatchSize::L shape=SwatchShape::Rectangle />
<Swatch nothing=true border=SwatchBorder::None rounding=SwatchRounding::Full />
<Swatch color="#111827".to_string() disabled=true />"##;

    let (selected, set_selected) = signal(true);
    let on_selected_change = Callback::new(move |next: bool| set_selected.set(next));

    view! {
        <ComponentPage
            title="Swatch"
            slug="swatch"
            group="Display"
            description="Spectrum-compatible swatch primitive with centralized size/shape/rounding/border/state contracts and HeroUI-grade spring selection motion."
        >
            <Playground title="Size + Shape + Rounding" code=size_code>
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

            <Playground title="Mixed + Nothing + Disabled + Controlled" code=state_code>
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
        </ComponentPage>
    }
    .into_any()
}
