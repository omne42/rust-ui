pub mod markdown;
pub mod pages;
pub mod playground;
pub mod route;
pub mod toc;

mod command_menu;
mod search_index;

use leptos::prelude::*;
use ui_components::{
    ButtonSize, ButtonVariant, IconButton, Sheet, SheetPlacement, Theme, ThemeMode,
    ThemeToggleButton, UiRoot, provide_focus_visible, provide_overlay_stack,
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

    let (nav_open, set_nav_open) = signal(false);
    let open_nav = Callback::new(move |_| set_nav_open.set(true));
    let close_nav = Callback::new(move |_| set_nav_open.set(false));
    let is_nav_open = Signal::derive(move || nav_open.get());
    let (nav_present, set_nav_present) = signal(is_nav_open.get_untracked());
    Effect::new(move |_| {
        if is_nav_open.get() {
            set_nav_present.set(true);
        }
    });
    let on_nav_exit_complete: Callback<()> = Callback::new(move |_| set_nav_present.set(false));

    let navigate = Callback::new(move |next: String| {
        set_route.run(next);
        set_nav_open.set(false);
    });

    let toc = toc::provide_docs_toc();
    let route_path = Memo::new(move |_| route::route_path(&route.get()).to_string());

    Effect::new(move |_| {
        _ = route_path.get();
        toc.clear();
    });

    Effect::new(move |_| {
        let current = route.get();
        if let Some(section) = route::route_section(&current) {
            route::scroll_to_id(section);
        }
    });

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
                        <IconButton
                            aria_label="Open navigation".to_string()
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconSm
                            class_name="docs-header__nav-toggle".to_string()
                            on_press=open_nav
                        >
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M4 6h12M4 10h12M4 14h12"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                    stroke_linecap="round"
                                />
                            </svg>
                        </IconButton>

                        <command_menu::DocsCommandMenu navigate=navigate />
                        <ThemeToggleButton mode=theme_mode set_mode=set_theme_mode />
                    </div>
                </header>

                <div class="docs-layout">
                    <nav class="docs-nav">
                        <pages::nav::DocsNav route=route navigate=navigate />
                    </nav>

                    <main class="docs-main">
                        {move || pages::route_view(route_path.get())}
                    </main>

                    <aside class="docs-toc">
                        <toc::DocsTocPanel route=route navigate=navigate />
                    </aside>
                </div>

                <Show when=move || nav_present.get()>
                    <Sheet
                        open=is_nav_open
                        on_close=close_nav
                        placement=SheetPlacement::Left
                        aria_labelledby="docs-mobile-nav-title".to_string()
                        on_exit_complete=on_nav_exit_complete
                    >
                        move || {
                            view! {
                                <div class="docs-mobile-nav">
                                    <div class="docs-mobile-nav__header">
                                        <h2 id="docs-mobile-nav-title" class="docs-mobile-nav__title">
                                            "Navigation"
                                        </h2>
                                    </div>
                                    <div class="docs-mobile-nav__body">
                                        <pages::nav::DocsNav route=route navigate=navigate />
                                    </div>
                                </div>
                            }
                        }
                    </Sheet>
                </Show>
            </div>
        </UiRoot>
    }
}

pub fn mount() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <App /> });
}
