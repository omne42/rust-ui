pub mod markdown;
pub mod pages;
pub mod playground;
pub mod route;

use leptos::prelude::*;
use ui_components::{Button, OnPress, Theme, UiRoot, provide_focus_visible, provide_overlay_stack};

#[component]
pub fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum DocsTheme {
        Light,
        Dark,
        Oled,
    }

    impl DocsTheme {
        fn next(self) -> Self {
            match self {
                Self::Light => Self::Dark,
                Self::Dark => Self::Oled,
                Self::Oled => Self::Light,
            }
        }

        fn label(self) -> &'static str {
            match self {
                Self::Light => "Light",
                Self::Dark => "Dark",
                Self::Oled => "OLED",
            }
        }
    }

    let (docs_theme, set_docs_theme) = signal(DocsTheme::Light);
    let theme = Signal::derive(move || match docs_theme.get() {
        DocsTheme::Light => Theme::light(),
        DocsTheme::Dark => Theme::dark(),
        DocsTheme::Oled => Theme::oled(),
    });
    let toggle_theme: OnPress = Callback::new(move |_| set_docs_theme.update(|t| *t = t.next()));

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
                        <Button on_press=toggle_theme>
                            {move || {
                                let current = docs_theme.get();
                                format!("Theme: {} → {}", current.label(), current.next().label())
                            }}
                        </Button>
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
