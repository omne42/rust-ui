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
use ui_components::{Button, OnPress, Theme, UiRoot};
use ui_headless::{provide_focus_visible, provide_overlay_stack};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum DemoSection {
    Architecture,
    Ports,
    NewComponents,
    Extras,
    MoreComponents,
    Button,
    Pagination,
    Avatar,
    Status,
    Misc,
    Typography,
    Disclosure,
    Tooltip,
    Overlay,
    ListBox,
    Menu,
    Select,
    ComboBox,
    Tabs,
    TextField,
    TextArea,
    Radio,
    Divider,
    Forms,
}

const DEMO_SECTIONS: [DemoSection; 24] = [
    DemoSection::Architecture,
    DemoSection::Ports,
    DemoSection::NewComponents,
    DemoSection::Extras,
    DemoSection::MoreComponents,
    DemoSection::Button,
    DemoSection::Pagination,
    DemoSection::Avatar,
    DemoSection::Status,
    DemoSection::Misc,
    DemoSection::Typography,
    DemoSection::Disclosure,
    DemoSection::Tooltip,
    DemoSection::Overlay,
    DemoSection::ListBox,
    DemoSection::Menu,
    DemoSection::Select,
    DemoSection::ComboBox,
    DemoSection::Tabs,
    DemoSection::TextField,
    DemoSection::TextArea,
    DemoSection::Radio,
    DemoSection::Divider,
    DemoSection::Forms,
];

impl DemoSection {
    fn label(self) -> &'static str {
        match self {
            Self::Architecture => "Architecture",
            Self::Ports => "Ports",
            Self::NewComponents => "New components",
            Self::Extras => "Extras",
            Self::MoreComponents => "More components",
            Self::Button => "Button",
            Self::Pagination => "Pagination",
            Self::Avatar => "Avatar",
            Self::Status => "Badge / Spinner",
            Self::Misc => "Card / Alert / Chip / Skeleton",
            Self::Typography => "Link / Code / Progress",
            Self::Disclosure => "Disclosure / Accordion",
            Self::Tooltip => "Tooltip",
            Self::Overlay => "Overlay",
            Self::ListBox => "List",
            Self::Menu => "MenuTrigger",
            Self::Select => "Select",
            Self::ComboBox => "ComboBox",
            Self::Tabs => "Tabs",
            Self::TextField => "TextField",
            Self::TextArea => "TextArea",
            Self::Radio => "RadioGroup",
            Self::Divider => "Divider",
            Self::Forms => "Checkbox / Switch",
        }
    }
}

fn render_demo_section(section: DemoSection) -> AnyView {
    match section {
        DemoSection::Architecture => view! { <ArchitectureDemo /> }.into_any(),
        DemoSection::Ports => view! { <PortsDemo /> }.into_any(),
        DemoSection::NewComponents => view! { <NewComponentsDemo /> }.into_any(),
        DemoSection::Extras => view! { <ExtrasDemo /> }.into_any(),
        DemoSection::MoreComponents => view! { <MoreComponentsDemo /> }.into_any(),
        DemoSection::Button => view! { <ButtonDemo /> }.into_any(),
        DemoSection::Pagination => view! { <PaginationDemo /> }.into_any(),
        DemoSection::Avatar => view! { <AvatarDemo /> }.into_any(),
        DemoSection::Status => view! { <StatusDemo /> }.into_any(),
        DemoSection::Misc => view! { <MiscDemo /> }.into_any(),
        DemoSection::Typography => view! { <TypographyDemo /> }.into_any(),
        DemoSection::Disclosure => view! { <DisclosureDemo /> }.into_any(),
        DemoSection::Tooltip => view! { <TooltipDemo /> }.into_any(),
        DemoSection::Overlay => view! { <OverlayDemo /> }.into_any(),
        DemoSection::ListBox => view! { <ListBoxDemo /> }.into_any(),
        DemoSection::Menu => view! { <MenuDemo /> }.into_any(),
        DemoSection::Select => view! { <SelectDemo /> }.into_any(),
        DemoSection::ComboBox => view! { <ComboBoxDemo /> }.into_any(),
        DemoSection::Tabs => view! { <TabsDemo /> }.into_any(),
        DemoSection::TextField => view! { <TextFieldDemo /> }.into_any(),
        DemoSection::TextArea => view! { <TextAreaDemo /> }.into_any(),
        DemoSection::Radio => view! { <RadioDemo /> }.into_any(),
        DemoSection::Divider => view! { <DividerDemo /> }.into_any(),
        DemoSection::Forms => view! { <FormsDemo /> }.into_any(),
    }
}

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    let (demo_theme, set_demo_theme) = signal(DemoTheme::Light);
    let (active_section, set_active_section) = signal(DemoSection::Architecture);
    let theme = Signal::derive(move || match demo_theme.get() {
        DemoTheme::Light => Theme::light(),
        DemoTheme::Dark => Theme::dark(),
        DemoTheme::Oled => Theme::oled(),
    });
    let toggle_theme: OnPress = Callback::new(move |_| set_demo_theme.update(|t| *t = t.next()));

    view! {
        <UiRoot
            theme=theme
            safe_area=true
            inject_components_css=true
        >
        <div class="demo-shell">
            <header class="demo-header">
                <div>
                    <h1 class="demo-title">"rust-ui"</h1>
                    <div class="demo-subtitle">
                        "Leptos UI primitives: " <code>"ui-state-primitives"</code> " / " <code>"ui-headless"</code> " / " <code>"ui-theme"</code> " / " <code>"ui-components"</code> " · active: " {move || active_section.get().label()}
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
                    <For
                        each=move || DEMO_SECTIONS
                        key=|section| *section as u8
                        children=move |section| {
                            let set_active_section = set_active_section;
                            view! {
                                <button
                                    type="button"
                                    class="demo-nav-link"
                                    class:demo-nav-link--active=move || active_section.get() == section
                                    on:click=move |_| set_active_section.set(section)
                                >
                                    {section.label()}
                                </button>
                            }
                        }
                    />
                </nav>

                <main class="demo-main">
                    {move || render_demo_section(active_section.get())}
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
#[path = "test/main.rs"]
mod tests;
