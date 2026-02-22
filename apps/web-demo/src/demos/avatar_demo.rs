use leptos::prelude::*;
use ui::{Avatar, AvatarSize};

const DEMO_AVATAR_SVG: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='96' height='96'><defs><linearGradient id='g' x1='0' x2='1' y1='0' y2='1'><stop offset='0' stop-color='%23467bff'/><stop offset='1' stop-color='%23a855f7'/></linearGradient></defs><rect width='96' height='96' rx='48' fill='url(%23g)'/><text x='48' y='56' text-anchor='middle' font-family='system-ui,Segoe UI,Roboto' font-size='34' fill='white'>A</text></svg>";

#[component]
pub fn AvatarDemo() -> impl IntoView {
    view! {
        <section id="avatar" class="demo-card">
            <h2>"Avatar"</h2>
            <p>"Image avatar with initials fallback, sizes via class-based tokens."</p>

            <div class="demo-row">
                <Avatar name="Ada Lovelace" size=AvatarSize::Sm />
                <Avatar name="Ada Lovelace" size=AvatarSize::Md />
                <Avatar name="Ada Lovelace" size=AvatarSize::Lg />
                <Avatar name="Image Avatar" src=DEMO_AVATAR_SVG size=AvatarSize::Lg />
            </div>
        </section>
    }
}
