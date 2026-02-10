use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Picker, PopoverMotion, SelectMotion};

pub(super) fn picker() -> AnyView {
    let items = vec![
        "North America".to_string(),
        "Europe".to_string(),
        "Asia Pacific".to_string(),
        "South America".to_string(),
    ];

    let items_basic = items.clone();
    let items_controlled = items.clone();
    let items_markers = items;

    let (selected, set_selected) = signal(Some(1_usize));

    let (selected_controlled, set_selected_controlled) = signal(Some(0_usize));
    let (open_raw, set_open_raw) = signal(false);
    let open: Signal<bool> = Signal::derive(move || open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_open_raw.set(next));

    let (marker_selected, set_marker_selected) = signal(Some(2_usize));
    let (marker_open_raw, set_marker_open_raw) = signal(true);
    let marker_open: Signal<bool> = Signal::derive(move || marker_open_raw.get());
    let marker_on_open_change = Callback::new(move |next: bool| set_marker_open_raw.set(next));

    let basic_code = r#"let (selected, set_selected) = signal(Some(1_usize));
<Picker
  id_base=\"region\".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  placeholder=\"Choose region\".to_string()
/>"#;

    let controlled_code = r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Picker
  id_base=\"region-controlled\".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  disabled_indices=vec![3]
/>"#;

    let markers_code = r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Picker
  id_base=\"region-markers\".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  open=open
  default_open=true
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  disabled_indices=vec![1]
  motion=SelectMotion {
    popover: PopoverMotion {
      initial_scale: 1.0,
      offset_y_px: 0.0,
      ..PopoverMotion::default()
    }
  }
  placeholder=\"Pick region\".to_string()
  class_name=\"docs-picker-state\".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Picker"
            slug="picker"
            group="Collections"
            description="Spectrum-compatible Picker alias for upstream naming parity, preserving Select accessibility/state contracts and HeroUI-level trigger/listbox interaction behavior."
        >
            <Playground title="Basic Selection" code=basic_code>
                <div class="docs-stack">
                    <Picker
                        id_base="docs-picker-basic".to_string()
                        items=items_basic
                        selected_index=selected
                        set_selected_index=set_selected
                        placeholder="Choose region".to_string()
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open + Disabled Option" code=controlled_code>
                <div class="docs-stack">
                    <Picker
                        id_base="docs-picker-controlled".to_string()
                        items=items_controlled
                        selected_index=selected_controlled
                        set_selected_index=set_selected_controlled
                        open=open
                        on_open_change=on_open_change
                        disabled_indices=vec![3]
                    />
                    <span class="ui-muted">"open: " {move || open_raw.get().to_string()}</span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-selection`, `data-disabled-options`, `data-open-mode`, `data-initial-open`, `data-placeholder-source`, `data-handler-source`, `data-placement-source`, and `data-motion-source`."
                code=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Picker
                        id_base="docs-picker-markers".to_string()
                        items=items_markers
                        selected_index=marker_selected
                        set_selected_index=set_marker_selected
                        open=marker_open
                        default_open=true
                        on_open_change=marker_on_open_change
                        disabled_indices=vec![1]
                        motion=SelectMotion {
    popover: PopoverMotion {
      initial_scale: 1.0,
      offset_y_px: 0.0,
      ..PopoverMotion::default()
    }
  }
                        placeholder="Pick region".to_string()
                        class_name="docs-picker-state".to_string()
                    />
                    <span class="ui-muted">"open: " {move || marker_open_raw.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
