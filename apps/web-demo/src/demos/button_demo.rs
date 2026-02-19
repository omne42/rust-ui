use leptos::prelude::*;
use ui_components::{
    Button, ButtonGroup, ButtonSize, ButtonVariant, LinkButton, OnPress, ToggleButton,
    ToggleButtonGroup, ToggleButtonVariant,
};

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (count, set_count) = signal(0_i32);
    let on_press: OnPress = Callback::new(move |_| set_count.update(|n| *n += 1));

    let (bold, set_bold) = signal(false);
    let (italic, set_italic) = signal(true);
    let (underline, set_underline) = signal(false);

    view! {
        <section id="button" class="demo-card">
            <h2>"Button"</h2>
            <p>"Pointer + keyboard press handling, disabled semantics, focus-visible outline."</p>
            <div class="demo-row">
                <Button on_press=on_press>"Press Me"</Button>
                <Button is_disabled=true>"Disabled"</Button>
                <span class="demo-kv">"count: " {count}</span>
            </div>
            <div class="demo-divider"></div>
            <div class="demo-row">
                <Button>"Default"</Button>
                <Button variant=ButtonVariant::Accent>"Accent"</Button>
                <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                <Button variant=ButtonVariant::Outline>"Outline"</Button>
                <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                <Button variant=ButtonVariant::Link>"Link"</Button>
                <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
            </div>
            <div class="demo-row">
                <Button size=ButtonSize::Sm>"Small"</Button>
                <Button size=ButtonSize::Default>"Default"</Button>
                <Button size=ButtonSize::Lg>"Large"</Button>
                <Button is_icon_only=true aria_label="Star button" size=ButtonSize::Icon>"★"</Button>
                <Button is_icon_only=true size=ButtonSize::IconSm aria_label="Small star button">"☆"</Button>
                <Button is_icon_only=true size=ButtonSize::IconLg aria_label="Large star button">"✦"</Button>
            </div>

            <div class="demo-divider"></div>
            <div class="demo-row">
                <ButtonGroup attached=true aria_label="Attached button group">
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Left"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Middle"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Right"</Button>
                </ButtonGroup>

                <LinkButton
                    href="https://example.com".to_string()
                    target="_blank"
                    variant=ButtonVariant::Outline
                >
                    "LinkButton"
                </LinkButton>
            </div>

            <div class="demo-divider"></div>
            <div class="demo-row">
                <ToggleButtonGroup attached=true aria_label="Formatting toggles">
                    <ToggleButton
                        selected=bold
                        set_selected=set_bold
                        variant=ToggleButtonVariant::Outline
                        size=ui_components::ToggleButtonSize::Sm
                    >
                        "Bold"
                    </ToggleButton>
                    <ToggleButton
                        selected=italic
                        set_selected=set_italic
                        variant=ToggleButtonVariant::Outline
                        size=ui_components::ToggleButtonSize::Sm
                    >
                        "Italic"
                    </ToggleButton>
                    <ToggleButton
                        selected=underline
                        set_selected=set_underline
                        variant=ToggleButtonVariant::Outline
                        size=ui_components::ToggleButtonSize::Sm
                    >
                        "Underline"
                    </ToggleButton>
                </ToggleButtonGroup>
                <span class="demo-kv">
                    "selected: "
                    {move || format!("bold={}, italic={}, underline={}", bold.get(), italic.get(), underline.get())}
                </span>
            </div>
        </section>
    }
}
