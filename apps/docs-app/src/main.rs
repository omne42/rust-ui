mod markdown;
mod pages;
mod playground;

use leptos::{mount::mount_to_body, prelude::*};
use pages::{DocPage, page_view};
use ui_components::{Button, OnPress, Theme, UiRoot, provide_focus_visible, provide_overlay_stack};

#[component]
fn App() -> impl IntoView {
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

    let (page, set_page) = signal(DocPage::Welcome);

    view! {
        <UiRoot theme=theme safe_area=true>
            <div class="docs-shell">
                <header class="docs-header">
                    <div>
                        <h1 class="docs-title">"rust-ui docs"</h1>
                        <div class="docs-subtitle">
                            "Type-driven, layered Leptos UI (core/headless/theme/motion/components)."
                        </div>
                    </div>
                    <Button on_press=toggle_theme>
                        {move || {
                            let current = docs_theme.get();
                            format!("Theme: {} → {}", current.label(), current.next().label())
                        }}
                    </Button>
                </header>

                <div class="docs-layout">
                    <nav class="docs-nav">
                        <div class="docs-nav-section">
                            <div class="docs-nav-title">"Docs"</div>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::Welcome { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::Welcome); }
                            >
                                "Welcome"
                            </a>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::Rules { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::Rules); }
                            >
                                "Rules"
                            </a>
                        </div>

                        <div class="docs-nav-section">
                            <div class="docs-nav-title">"Components"</div>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::Button { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::Button); }
                            >
                                "Button"
                            </a>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::TextField { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::TextField); }
                            >
                                "TextField"
                            </a>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::Select { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::Select); }
                            >
                                "Select"
                            </a>
                            <a
                                href="#"
                                data-active=move || if page.get() == DocPage::ComboBox { Some("true") } else { None }
                                on:click=move |ev| { ev.prevent_default(); set_page.set(DocPage::ComboBox); }
                            >
                                "ComboBox"
                            </a>
                        </div>
                    </nav>

                    <main class="docs-main">
                        {move || page_view(page.get())}
                    </main>
                </div>
            </div>
        </UiRoot>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
