pub mod markdown;
pub mod pages;
pub mod playground;
pub mod route;
pub mod router;
pub mod toc;

mod command_menu;
mod debug_overlay;
mod search_index;

#[cfg(all(target_arch = "wasm32", not(erase_components)))]
compile_error!(
    "WASM builds of this repo require `cfg(erase_components)` to avoid Tachys attribute tuple limits. \
Ensure `.cargo/config.toml` is picked up (workspace root), or set `RUSTFLAGS=\"--cfg erase_components\"`."
);

use leptos::prelude::*;
use ui_components::{
    Button, ButtonSize, ButtonVariant, Sheet, SheetPlacement, Theme, ThemeMode, ThemeToggleButton,
    UiRoot,
};
use ui_headless::{provide_focus_visible, provide_overlay_stack, provide_ui_trace};
use ui_layout::{
    Card, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, Grid, GridColumns, GridGap, Header,
    Heading, HeadingLevel,
};

#[cfg(target_arch = "wasm32")]
fn set_document_title(title: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    document.set_title(title);
}

#[cfg(not(target_arch = "wasm32"))]
fn set_document_title(_title: &str) {}

#[component]
pub fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();
    let debug_overlay_enabled = cfg!(debug_assertions);
    provide_ui_trace(debug_overlay_enabled);

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

    router::provide_docs_router(route, navigate);

    let toc = toc::provide_docs_toc();
    let route_path = Memo::new(move |_| route::route_path(&route.get()).to_string());
    let toc_items = toc.items();

    let full_title = Memo::new(move |_| {
        let base = "rust-ui docs";
        let page = pages::title_for_path(&route_path.get());

        if let Some(section) = route::route_section(&route.get()) {
            let section_title = toc_items.with(|items| {
                items
                    .iter()
                    .find(|item| item.id == section)
                    .map(|item| item.title.clone())
            });

            return match section_title {
                Some(section_title) => format!("{section_title} · {page} · {base}"),
                None => format!("{page} · {base}"),
            };
        }

        if page == "Welcome" {
            base.into()
        } else {
            format!("{page} · {base}")
        }
    });

    Effect::new(move |_| set_document_title(&full_title.get()));

    Effect::new(move |_| {
        _ = route_path.get();
        toc.clear();
    });

    Effect::new(move |_| {
        let current = route.get();
        if let Some(section) = route::route_section(&current) {
            toc.set_active(Some(section.into()));
            route::scroll_to_id(section);
        } else {
            toc.set_active(None);
            route::scroll_to_top();
        }
    });

    view! {
        <UiRoot
            theme=theme
            safe_area=true
            inject_components_css=true
        >
            <Show when=move || debug_overlay_enabled>
                <style>{debug_overlay::CSS}</style>
                <debug_overlay::UiDebugOverlay enabled=true />
            </Show>
            <Flex direction=FlexDirection::Column gap=FlexGap::Md class_name="docs-shell".to_string()>
                <Header bordered=true class_name="docs-header".to_string()>
                    <Flex
                        justify=FlexJustify::SpaceBetween
                        align=FlexAlign::Center
                        gap=FlexGap::Md
                        class_name="docs-header__layout".to_string()
                    >
                        <Flex direction=FlexDirection::Column gap=FlexGap::Xs class_name="docs-header__title".to_string()>
                            <Heading level=HeadingLevel::H1 class_name="docs-title".to_string()>
                                "rust-ui docs"
                            </Heading>
                            <div class="docs-subtitle">
                                "Type-driven, layered Leptos UI (core/headless/theme/motion/components)."
                            </div>
                        </Flex>

                        <Flex align=FlexAlign::Center gap=FlexGap::Sm class_name="docs-header__actions".to_string()>
                        <Button
                            aria_label="Open navigation".to_string()
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconSm
                            is_icon_only=true
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
                        </Button>

                        <command_menu::DocsCommandMenu navigate=navigate />
                        <ThemeToggleButton mode=theme_mode set_mode=set_theme_mode />
                        </Flex>
                    </Flex>
                </Header>

                <Grid columns=GridColumns::Three gap=GridGap::Md class_name="docs-layout".to_string()>
                    <Card class_name="docs-nav".to_string()>
                        <pages::nav::DocsNav route=route navigate=navigate />
                    </Card>

                    <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="docs-main".to_string()>
                        {move || pages::route_view(route_path.get())}
                    </Flex>

                    <Card class_name="docs-toc".to_string()>
                        <toc::DocsTocPanel route=route navigate=navigate />
                    </Card>
                </Grid>

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
                                <Flex direction=FlexDirection::Column gap=FlexGap::Sm class_name="docs-mobile-nav".to_string()>
                                    <Flex
                                        justify=FlexJustify::SpaceBetween
                                        align=FlexAlign::Center
                                        gap=FlexGap::Sm
                                        class_name="docs-mobile-nav__header".to_string()
                                    >
                                        <h2 id="docs-mobile-nav-title" class="docs-mobile-nav__title">
                                            "Navigation"
                                        </h2>
                                    </Flex>
                                    <div class="docs-mobile-nav__body">
                                        <pages::nav::DocsNav route=route navigate=navigate />
                                    </div>
                                </Flex>
                            }
                        }
                    </Sheet>
                </Show>
            </Flex>
        </UiRoot>
    }
}

pub fn mount() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| view! { <App /> });
}
