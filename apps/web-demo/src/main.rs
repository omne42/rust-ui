mod demos;

#[cfg(all(target_arch = "wasm32", not(erase_components)))]
compile_error!(
    "WASM builds of this repo require `cfg(erase_components)` to avoid Tachys attribute tuple limits. \
Ensure `.cargo/config.toml` is picked up (workspace root), or set `RUSTFLAGS=\"--cfg erase_components\"`."
);

use demos::{
    ArchitectureDemo, AvatarDemo, ButtonDemo, ComboBoxDemo, DisclosureDemo, DividerDemo,
    ExtrasDemo, FormsDemo, ListBoxDemo, MenuDemo, MiscDemo, MoreComponentsDemo, NewComponentsDemo,
    OverlayDemo, PaginationDemo, PortsDemo, RadioDemo, SelectDemo, StatusDemo, TabsDemo,
    TextAreaDemo, TextFieldDemo, TooltipDemo, TypographyDemo,
};
use leptos::{mount::mount_to_body, prelude::*};
use ui_components::{Button, OnPress, Theme, UiRoot, provide_focus_visible, provide_overlay_stack};

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

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

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
                    <a href="#ports">"Ports"</a>
                    <a href="#new-components">"New components"</a>
                    <a href="#extras">"Extras"</a>
                    <a href="#more-components">"More components"</a>
                    <a href="#button">"Button"</a>
                    <a href="#pagination">"Pagination"</a>
                    <a href="#avatar">"Avatar"</a>
                    <a href="#status">"Badge / Spinner"</a>
                    <a href="#misc">"Card / Alert / Chip / Skeleton"</a>
                    <a href="#typography">"Link / Code / Progress"</a>
                    <a href="#disclosure">"Disclosure / Accordion"</a>
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
                    <PortsDemo />
                    <NewComponentsDemo />
                    <ExtrasDemo />
                    <MoreComponentsDemo />
                    <ButtonDemo />
                    <PaginationDemo />
                    <AvatarDemo />
                    <StatusDemo />
                    <MiscDemo />
                    <TypographyDemo />
                    <DisclosureDemo />
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
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App /> })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_theme_cycle_is_stable() {
        assert_eq!(DemoTheme::Light.next(), DemoTheme::Dark);
        assert_eq!(DemoTheme::Dark.next(), DemoTheme::Oled);
        assert_eq!(DemoTheme::Oled.next(), DemoTheme::Light);
        assert_eq!(DemoTheme::Light.label(), "Light");
        assert_eq!(DemoTheme::Dark.label(), "Dark");
        assert_eq!(DemoTheme::Oled.label(), "OLED");
    }
}
