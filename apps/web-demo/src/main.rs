use leptos::{mount::mount_to_body, prelude::*};
use std::rc::Rc;
use ui_components::Button;
use ui_headless::{provide_focus_visible, OnPress};

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();

    let (count, set_count) = signal(0_i32);
    let on_press: OnPress = Rc::new(move || set_count.update(|n| *n += 1));

    view! {
        <main style="padding: 24px; font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;">
            <h1 style="margin: 0 0 12px 0; font-size: 18px;">"web-demo"</h1>
            <div style="display: flex; gap: 12px; align-items: center;">
                <Button on_press=on_press.clone()>"Press Me"</Button>
                <Button disabled=true>"Disabled"</Button>
                <span>"count: " {count}</span>
            </div>
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
