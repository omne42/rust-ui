use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{ButtonMotion, ButtonSize, ButtonVariant, IconButton, OnPress};

pub(super) fn icon_button() -> AnyView {
    let _ = super::actions::icon_button as fn() -> AnyView;
    let (close_count, set_close_count) = signal(0_usize);
    let (search_count, set_search_count) = signal(0_usize);

    let on_close: OnPress = Callback::new(move |_| {
        set_close_count.update(|count| *count += 1);
    });
    let on_search: OnPress = Callback::new(move |_| {
        set_search_count.update(|count| *count += 1);
    });

    let (marker_count, set_marker_count) = signal(0_usize);
    let marker_press: OnPress = Callback::new(move |_| {
        set_marker_count.update(|count| *count += 1);
    });

    let code = r#"let (presses, set_presses) = signal(0_usize);
let on_press = Callback::new(move |_| set_presses.update(|count| *count += 1));
<IconButton aria_label="Close dialog".to_string() variant=ButtonVariant::Ghost on_press=on_press>
  <span aria-hidden="true">"✕"</span>
</IconButton>"#;

    let states_code = r#"<IconButton aria_label="Search".to_string() size=ButtonSize::IconSm>
  <span aria-hidden="true">"⌕"</span>
</IconButton>
<IconButton aria_label="Search".to_string() size=ButtonSize::Icon>
  <span aria-hidden="true">"⌕"</span>
</IconButton>
<IconButton aria_label="Search".to_string() size=ButtonSize::IconLg disabled=true>
  <span aria-hidden="true">"⌕"</span>
</IconButton>"#;

    let markers_code = r#"<IconButton
  aria_label="Inspect icon trigger".to_string()
  variant=ButtonVariant::Secondary
  size=ButtonSize::Lg
  motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }
  class_name="docs-icon-button-state".to_string()
  on_press=Callback::new(move |_| { /* marker */ })
>
  <span aria-hidden="true">"⌕"</span>
</IconButton>"#;

    view! {
        <ComponentPage
            title="IconButton"
            slug="icon-button"
            group="Actions"
            description="Spectrum-compatible IconButton alias with centralized aria/size/handler source contracts and HeroUI-level motion behavior via Button composition."
        >
            <Playground title="on_press + variants" code=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <IconButton
                            aria_label="Close dialog".to_string()
                            variant=ButtonVariant::Ghost
                            on_press=on_close
                        >
                            <span aria-hidden="true">"✕"</span>
                        </IconButton>
                        <IconButton
                            aria_label="Search".to_string()
                            variant=ButtonVariant::Secondary
                            on_press=on_search
                        >
                            <span aria-hidden="true">"⌕"</span>
                        </IconButton>
                    </div>
                    <span class="ui-muted">
                        "close/search presses: "
                        {move || format!("{}/{}", close_count.get(), search_count.get())}
                    </span>
                </div>
            </Playground>

            <Playground title="Size + disabled matrix" code=states_code>
                <div class="docs-row">
                    <IconButton aria_label="Search small".to_string() size=ButtonSize::IconSm>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton aria_label="Search default".to_string() size=ButtonSize::Icon>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton aria_label="Search large".to_string() size=ButtonSize::IconLg>
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <IconButton
                        aria_label="Close disabled".to_string()
                        variant=ButtonVariant::Ghost
                        disabled=true
                    >
                        <span aria-hidden="true">"✕"</span>
                    </IconButton>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-size-mode`, `data-handler-source`, `data-label-source`, `data-class-source`, and `data-motion-source`."
                code=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <IconButton
                        aria_label="Inspect icon trigger".to_string()
                        variant=ButtonVariant::Secondary
                        size=ButtonSize::Lg
                        motion=ButtonMotion { hover_scale: 1.0, tap_scale: 1.0, ..ButtonMotion::default() }
                        class_name="docs-icon-button-state".to_string()
                        on_press=marker_press
                    >
                        <span aria-hidden="true">"⌕"</span>
                    </IconButton>
                    <span class="ui-muted">"presses: " {move || marker_count.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
