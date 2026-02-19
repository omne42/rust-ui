use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, Input, OnPress, Overlay};

pub(super) fn theme_visual_baseline() -> AnyView {
    let (email, set_email) = signal("design@rust-ui.dev".to_string());

    let (overlay_open_raw, set_overlay_open_raw) = signal(true);
    let overlay_open: Signal<bool> = Signal::derive(move || overlay_open_raw.get());

    let (overlay_present, set_overlay_present) = signal(overlay_open.get_untracked());
    Effect::new(move |_| {
        if overlay_open.get() {
            set_overlay_present.set(true);
        }
    });

    let open_overlay: OnPress = Callback::new(move |_| set_overlay_open_raw.set(true));
    let close_overlay: OnPress = Callback::new(move |_| set_overlay_open_raw.set(false));
    let on_exit_complete = Callback::new(move |_| set_overlay_present.set(false));

    let code = Signal::derive(move || {
        r#"let (email, set_email) = signal("design@rust-ui.dev".to_string());
let (overlay_open_raw, set_overlay_open_raw) = signal(true);
let overlay_open: Signal<bool> = Signal::derive(move || overlay_open_raw.get());

<Button variant=ButtonVariant::Accent>"Primary"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>

<Input
  id="theme-baseline-input".to_string()
  value=email
  set_value=set_email
  label="Email".to_string()
  placeholder="design@rust-ui.dev".to_string()
  is_clearable=true
/>

<Overlay
  open=overlay_open
  on_close=Callback::new(move |_| set_overlay_open_raw.set(false))
>
  <div class="docs-card">"Overlay visual layer"</div>
</Overlay>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ThemeVisualBaseline"
            slug="theme-visual-baseline"
            group="Layout"
            description="Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots."
        >
            <Playground
                title="Default Theme Visual Baseline"
                description="Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus)."
                code_signal=code
            >
                <div class="docs-stack" data-slot="theme-visual-baseline">
                    <section class="docs-card docs-stack" data-slot="theme-visual-baseline-surface">
                        <div class="docs-stack docs-stack--tight">
                            <h3>"Visual Baseline"</h3>
                            <p class="ui-muted">
                                "Default theme should feel trustworthy at first glance: clear hierarchy, natural contrast, and explicit interaction feedback."
                            </p>
                        </div>

                        <div class="docs-row" data-slot="theme-visual-baseline-button">
                            <Button variant=ButtonVariant::Accent>"Primary Action"</Button>
                            <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                            <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                            <Button is_disabled=true>"Disabled"</Button>
                        </div>

                        <div data-slot="theme-visual-baseline-input">
                            <Input
                                id="docs-theme-visual-baseline-input".to_string()
                                value=email
                                set_value=set_email
                                label="Email".to_string()
                                placeholder="design@rust-ui.dev".to_string()
                                is_clearable=true
                            />
                        </div>

                        <span class="ui-muted">"input: " {move || email.get()}</span>
                    </section>

                    <div class="docs-row">
                        <Button on_press=open_overlay>"Open Overlay Baseline"</Button>
                        <Button variant=ButtonVariant::Outline on_press=close_overlay>
                            "Close Overlay Baseline"
                        </Button>
                        <span class="ui-muted">"overlay open: " {move || overlay_open_raw.get()}</span>
                    </div>
                </div>

                <Show when=move || overlay_present.get()>
                    <Overlay
                        open=overlay_open
                        on_close=close_overlay
                        aria_labelledby="docs-theme-visual-overlay-title".to_string()
                        aria_describedby="docs-theme-visual-overlay-desc".to_string()
                        class_name="docs-theme-visual-overlay".to_string()
                        on_exit_complete=on_exit_complete
                    >
                        <div class="docs-card docs-stack" data-slot="theme-visual-baseline-overlay">
                            <h4 id="docs-theme-visual-overlay-title">"Overlay Depth"</h4>
                            <p id="docs-theme-visual-overlay-desc" class="ui-muted">
                                "Overlay layers must preserve background separation, text readability, and interaction focus."
                            </p>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_overlay>
                                    "Close"
                                </Button>
                            </div>
                        </div>
                    </Overlay>
                </Show>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
