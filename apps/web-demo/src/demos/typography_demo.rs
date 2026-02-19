use leptos::prelude::*;
use ui_components::{
    BreadcrumbItem, Breadcrumbs, Code, CodeVariant, Kbd, Link, ProgressBar, Snippet,
};
use ui_layout::{Spacer, SpacerAxis, SpacerSize};

#[component]
pub fn TypographyDemo() -> impl IntoView {
    let breadcrumbs = vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            href: Some("#architecture".to_string()),
        },
        BreadcrumbItem {
            label: "Components".to_string(),
            href: Some("#button".to_string()),
        },
        BreadcrumbItem {
            label: "Typography".to_string(),
            href: None,
        },
    ];

    view! {
        <section id="typography" class="demo-card">
            <h2>"Link / Breadcrumbs / Code / Kbd / Spacer / ProgressBar"</h2>
            <p>"Utility primitives inspired by bb/ui-web. All styling is tokens-driven and injected via UiRoot."</p>

            <div class="demo-stack">
                <div class="demo-row">
                    <Link href="https://example.com".to_string() target="_blank">
                        "External Link"
                    </Link>
                    <Link href="https://example.com".to_string() is_disabled=true>
                        "Disabled"
                    </Link>
                </div>

                <Breadcrumbs items=breadcrumbs />

                <div class="demo-row">
                    <Code>"inline code"</Code>
                    <Kbd keys="⌘K".to_string()>"Open"</Kbd>
                </div>

                <Code variant=CodeVariant::Block class_name="demo-code-block".to_string()>
                    {r#"fn main() {
    println!("hello");
}"#}
                </Code>

                <div class="demo-row">
                    <span class="demo-kv">"Spacer:"</span>
                    <span>"A"</span>
                    <Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Sm />
                    <span>"B"</span>
                </div>

                <div class="demo-progress-stack">
                    <ProgressBar value=36.0 />
                    <ProgressBar variant=ui_components::ProgressBarVariant::Accent value=72.0 />
                    <ProgressBar variant=ui_components::ProgressBarVariant::Danger indeterminate=true />
                </div>

                <Snippet
                    label="API".to_string()
                    text="cargo test --workspace".to_string()
                    copyable=true
                />
            </div>
        </section>
    }
}
