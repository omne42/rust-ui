mod demos;

use demos::{
    ArchitectureDemo, AvatarDemo, ButtonDemo, ComboBoxDemo, DividerDemo, FormsDemo, ListBoxDemo,
    MenuDemo, OverlayDemo, RadioDemo, SelectDemo, StatusDemo, TabsDemo, TextAreaDemo,
    TextFieldDemo, TooltipDemo,
};
use leptos::{mount::mount_to_body, prelude::*};
use ui_components::{Button, OnPress, Theme, UiRoot, provide_focus_visible, provide_overlay_stack};

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum DemoTheme {
        Light,
        Dark,
        Oled,
    }

    impl DemoTheme {
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

    let (demo_theme, set_demo_theme) = signal(DemoTheme::Light);
    let theme = Signal::derive(move || match demo_theme.get() {
        DemoTheme::Light => Theme::light(),
        DemoTheme::Dark => Theme::dark(),
        DemoTheme::Oled => Theme::oled(),
    });
    let toggle_theme: OnPress = Callback::new(move |_| set_demo_theme.update(|t| *t = t.next()));

    view! {
        <UiRoot theme=theme safe_area=true>
        <div class="demo-shell">
            <header class="demo-header">
                <div>
                    <h1 class="demo-title">"rust-ui"</h1>
                    <div class="demo-subtitle">
                        "Leptos UI primitives: " <code>"ui-core"</code> " / " <code>"ui-headless"</code> " / " <code>"ui-theme"</code> " / " <code>"ui-components"</code>
                    </div>
                </div>
                <Button on_press=toggle_theme>
                    {move || {
                        let current = demo_theme.get();
                        format!("Theme: {} → {}", current.label(), current.next().label())
                    }}
                </Button>
            </header>

            <div class="demo-layout">
                <nav class="demo-nav">
                    <div class="demo-nav-title">"On this page"</div>
                    <a href="#architecture">"Architecture"</a>
                    <a href="#button">"Button"</a>
                    <a href="#avatar">"Avatar"</a>
                    <a href="#status">"Badge / Spinner"</a>
                    <a href="#tooltip">"Tooltip"</a>
                    <a href="#overlay">"Overlay"</a>
                    <a href="#listbox">"ListBox"</a>
                    <a href="#menu">"MenuTrigger"</a>
                    <a href="#select">"Select"</a>
                    <a href="#combo-box">"ComboBox"</a>
                    <a href="#tabs">"Tabs"</a>
                    <a href="#text-field">"TextField"</a>
                    <a href="#text-area">"TextArea"</a>
                    <a href="#radio">"RadioGroup"</a>
                    <a href="#divider">"Divider"</a>
                    <a href="#forms">"Checkbox / Switch"</a>
                </nav>

                <main class="demo-main">
                    <ArchitectureDemo />
                    <ButtonDemo />
                    <AvatarDemo />
                    <StatusDemo />
                    <TooltipDemo />
                    <OverlayDemo />
                    <ListBoxDemo />
                    <MenuDemo />
                    <SelectDemo />
                    <ComboBoxDemo />
                    <TabsDemo />
                    <TextFieldDemo />
                    <TextAreaDemo />
                    <RadioDemo />
                    <DividerDemo />
                    <FormsDemo />
                </main>
            </div>
        </div>
        </UiRoot>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
