pub mod markdown;
pub mod pages;
pub mod playground;
pub mod route;

mod command_menu;

use leptos::prelude::*;
use ui_components::{
    Theme, ThemeMode, ThemeToggleButton, UiRoot, provide_focus_visible, provide_overlay_stack,
};

#[component]
pub fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    let (theme_mode, set_theme_mode) = signal(ThemeMode::Light);
    let theme = Signal::derive(move || match theme_mode.get() {
        ThemeMode::Light => Theme::light(),
        ThemeMode::Dark => Theme::dark(),
        ThemeMode::Oled => Theme::oled(),
    });

    let (route, set_route) = route::use_hash_route();

    view! {
        <UiRoot theme=theme safe_area=true>
            <div class="docs-shell">
                <header class="docs-header">
                    <div class="docs-header__title">
                        <h1 class="docs-title">"rust-ui docs"</h1>
                        <div class="docs-subtitle">
                            "Type-driven, layered Leptos UI (core/headless/theme/motion/components)."
                        </div>
                    </div>

                    <div class="docs-header__actions">
                        <command_menu::DocsCommandMenu navigate=set_route />
                        <ThemeToggleButton mode=theme_mode set_mode=set_theme_mode />
                    </div>
                </header>

                <div class="docs-layout">
                    <nav class="docs-nav">
                        <pages::nav::DocsNav route=route navigate=set_route />
                    </nav>

                    <main class="docs-main">
                        {move || pages::route_view(route.get())}
                    </main>
                </div>
            </div>
        </UiRoot>
    }
}

pub fn mount() {
    leptos::mount::mount_to_body(|| view! { <App /> });
}
