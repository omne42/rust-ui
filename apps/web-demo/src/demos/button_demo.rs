use leptos::prelude::*;
use ui_components::{Button, ButtonSize, ButtonVariant, IconButton, OnPress};

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (count, set_count) = signal(0_i32);
    let on_press: OnPress = Callback::new(move |_| set_count.update(|n| *n += 1));

    view! {
        <section id="button" class="demo-card">
            <h2>"Button"</h2>
            <p>"Pointer + keyboard press handling, disabled semantics, focus-visible outline."</p>
            <div class="demo-row">
                <Button on_press=on_press>"Press Me"</Button>
                <Button disabled=true>"Disabled"</Button>
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
                <IconButton aria_label="Star button">"★"</IconButton>
                <IconButton size=ButtonSize::IconSm aria_label="Small star button">"☆"</IconButton>
                <IconButton size=ButtonSize::IconLg aria_label="Large star button">"✦"</IconButton>
            </div>
        </section>
    }
}
