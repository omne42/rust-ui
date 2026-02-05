use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AutoHeight, Card, CardVariant, Divider, DividerOrientation, ScrollShadow, Separator,
    SeparatorOrientation, Spacer, SpacerAxis, SpacerSize,
};

pub(super) fn card() -> AnyView {
    let code = r#"<Card variant=CardVariant::Default>
  <div>"Card content"</div>
</Card>"#;

    view! {
        <ComponentPage
            title="Card"
            slug="card"
            group="Layout"
            description="A token-styled surface with variant support."
        >
            <Playground title="Variants" code=code>
                <div class="docs-row">
                    <Card variant=CardVariant::Default>
                        <div class="docs-stack">
                            <div>"Default"</div>
                            <div class="ui-muted">"Uses tokens for bg/border/shadow."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Muted>
                        <div class="docs-stack">
                            <div>"Muted"</div>
                            <div class="ui-muted">"Lower-contrast surface."</div>
                        </div>
                    </Card>
                    <Card variant=CardVariant::Outline>
                        <div class="docs-stack">
                            <div>"Outline"</div>
                            <div class="ui-muted">"Border-forward style."</div>
                        </div>
                    </Card>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn divider() -> AnyView {
    let code = r#"<Divider />
<Divider orientation=DividerOrientation::Vertical />"#;

    view! {
        <ComponentPage
            title="Divider"
            slug="divider"
            group="Layout"
            description="Simple separator line with aria-orientation."
        >
            <Playground title="Horizontal & vertical" code=code>
                <div class="docs-stack">
                    <div>"Above"</div>
                    <Divider />
                    <div>"Below"</div>
                    <div class="docs-row">
                        <span>"Left"</span>
                        <Divider orientation=DividerOrientation::Vertical />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn separator() -> AnyView {
    let code = r#"<Separator />
<Separator orientation=SeparatorOrientation::Vertical />"#;

    view! {
        <ComponentPage
            title="Separator"
            slug="separator"
            group="Layout"
            description="Animated separator (motion-enabled) with decorative mode."
        >
            <Playground title="Separator" code=code>
                <div class="docs-stack">
                    <div class="docs-stack docs-stack--tight">
                        <span>"Above"</span>
                        <Separator />
                        <span>"Below"</span>
                    </div>
                    <div class="docs-row">
                        <span>"Left"</span>
                        <Separator orientation=SeparatorOrientation::Vertical />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn spacer() -> AnyView {
    let code = r#"<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />"#;

    view! {
        <ComponentPage
            title="Spacer"
            slug="spacer"
            group="Layout"
            description="A fixed-size gap element (aria-hidden)."
        >
            <Playground title="Spacing" code=code>
                <div class="docs-stack">
                    <div>"Above"</div>
                    <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />
                    <div>"Below"</div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn scroll_shadow() -> AnyView {
    let code = r#"<ScrollShadow max_height_px=160>
  {rows}
</ScrollShadow>"#;

    view! {
        <ComponentPage
            title="ScrollShadow"
            slug="scroll-shadow"
            group="Layout"
            description="Adds top/bottom shadow indicators for scrollable content."
        >
            <Playground title="Scrollable" code=code>
                <ScrollShadow max_height_px=160>
                    <div class="docs-stack">
                        {(1..=24)
                            .map(|idx| view! { <div class="ui-muted">{format!("Row {idx}")}</div> })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn auto_height() -> AnyView {
    let (open, set_open) = signal(false);
    let code = r#"let (open, set_open) = signal(false);
<Button on_press=...>"Toggle"</Button>
<AutoHeight>
  <Show when=open>...</Show>
</AutoHeight>"#;

    view! {
        <ComponentPage
            title="AutoHeight"
            slug="auto-height"
            group="Layout"
            description="Animates height changes using a spring-driven CSS variable contract."
        >
            <Playground title="Expand / collapse" code=code>
                <div class="docs-stack">
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_open.update(|v| *v = !*v))
                    >
                        {move || if open.get() { "Collapse" } else { "Expand" }}
                    </ui_components::Button>

                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <Show when=move || open.get()>
                            <div class="docs-stack">
                                <div>"AutoHeight content"</div>
                                <div class="ui-muted">
                                    "ResizeObserver + ui-motion spring."
                                </div>
                                <div class="ui-muted">
                                    "Toggle quickly to verify stability."
                                </div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn ui_root() -> AnyView {
    let code = r#"use ui_components::{UiRoot, Theme};

<UiRoot theme=Theme::dark() safe_area=true>
  // your app
</UiRoot>"#;

    view! {
        <ComponentPage
            title="UiRoot"
            slug="ui-root"
            group="Layout"
            description="The provider that injects tokens + component CSS. This docs app already runs inside UiRoot."
        >
            <Playground title="Usage" code=code>
                <div class="docs-stack">
                    <div>"UiRoot is already mounted at the root of this docs app."</div>
                    <div class="ui-muted">
                        "It injects theme tokens + component CSS via a single <style> tag."
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
