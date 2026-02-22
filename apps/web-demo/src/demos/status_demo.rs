use leptos::prelude::*;
use ui::{Badge, BadgeVariant, CircularProgress, StatusLight, StatusLightVariant};

#[component]
pub fn StatusDemo() -> impl IntoView {
    view! {
        <section id="status" class="demo-card">
            <h2>"Badge / CircularProgress"</h2>
            <p>"Pure visual components (tokens-driven). Spinner respects prefers-reduced-motion."</p>

            <div class="demo-row">
                <Badge>"Default"</Badge>
                <Badge variant=BadgeVariant::Accent>"Accent"</Badge>
                <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
            </div>

            <div class="demo-row">
                <CircularProgress />
                <CircularProgress size_px=16.0 thickness_px=2.0 />
                <CircularProgress size_px=28.0 thickness_px=3.0 />
            </div>

            <div class="demo-divider"></div>
            <div class="demo-row">
                <StatusLight>"Idle"</StatusLight>
                <StatusLight variant=StatusLightVariant::Accent>"Online"</StatusLight>
                <StatusLight variant=StatusLightVariant::Danger>"Error"</StatusLight>
            </div>
        </section>
    }
}
