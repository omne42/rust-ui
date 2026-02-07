use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    AutoHeight, AutoHeightMotion, Card, CardVariant, Divider, DividerOrientation, ScrollShadow,
    Separator, SeparatorOrientation, Spacer, SpacerAxis, SpacerSize,
};

pub(super) fn card() -> AnyView {
    let variants_code = r#"<Card variant=CardVariant::Default>"Default"</Card>
<Card variant=CardVariant::Muted>"Muted"</Card>
<Card variant=CardVariant::Outline>"Outline"</Card>"#;

    let padding_code = r#"<Card padded=true>
  <div>"Padded content"</div>
</Card>
<Card padded=false>
  <div>"Flush content"</div>
</Card>"#;

    let custom_class_code = r#"<Card class_name="docs-card-custom".to_string()>
  <div>"Custom class marker"</div>
</Card>"#;

    view! {
        <ComponentPage
            title="Card"
            slug="card"
            group="Layout"
            description="A token-styled surface with centralized variant/padding state attrs."
        >
            <Playground title="Variants" code=variants_code>
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

            <Playground title="Padding States" code=padding_code>
                <div class="docs-row">
                    <Card padded=true>
                        <div class="docs-stack">
                            <div>"Padded"</div>
                            <div class="ui-muted">"Default spacing"</div>
                        </div>
                    </Card>
                    <Card padded=false>
                        <div class="docs-stack">
                            <div class="docs-row docs-row--tight">
                                <span>"Flush"</span>
                                <span class="ui-muted">"No internal padding"</span>
                            </div>
                        </div>
                    </Card>
                </div>
            </Playground>

            <Playground title="Custom Class" code=custom_class_code>
                <div class="docs-row">
                    <Card class_name="docs-card-custom".to_string()>
                        <div class="docs-stack">
                            <div>"Custom class marker"</div>
                            <div class="ui-muted">"Verifies `data-custom-class` + class merge."</div>
                        </div>
                    </Card>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn divider() -> AnyView {
    let orientations_code = r#"<Divider />
<Divider orientation=DividerOrientation::Vertical class_name="docs-divider-rail".to_string() />"#;

    let custom_class_code = r#"<Divider class_name="docs-divider-custom".to_string() />
<Divider
  orientation=DividerOrientation::Vertical
  class_name="docs-divider-custom docs-divider-rail".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Divider"
            slug="divider"
            group="Layout"
            description="A separator primitive with centralized orientation state attrs and Spectrum-style styling markers."
        >
            <Playground title="Orientation" code=orientations_code>
                <div class="docs-stack">
                    <div>"Above"</div>
                    <Divider />
                    <div>"Below"</div>
                    <div class="docs-row">
                        <span>"Left"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-rail".to_string()
                        />
                        <span>"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class Marker" code=custom_class_code>
                <div class="docs-stack">
                    <span>"Custom horizontal divider"</span>
                    <Divider class_name="docs-divider-custom".to_string() />
                    <div class="docs-row">
                        <span>"Start"</span>
                        <Divider
                            orientation=DividerOrientation::Vertical
                            class_name="docs-divider-custom docs-divider-rail".to_string()
                        />
                        <span>"End"</span>
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
    let axis_and_size_code = r#"<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
<Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />"#;

    let custom_class_code = r#"<Spacer
  axis=SpacerAxis::Vertical
  size=SpacerSize::Md
  class_name="docs-spacer-guide".to_string()
/>
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  class_name="docs-spacer-guide".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Spacer"
            slug="spacer"
            group="Layout"
            description="A pure spacing primitive with centralized axis/size state attrs for Spectrum-style styling contracts."
        >
            <Playground title="Axis + Size" code=axis_and_size_code>
                <div class="docs-stack">
                    <div class="docs-stack">
                        <span class="docs-spacer-box">"Top"</span>
                        <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
                        <span class="docs-spacer-box">"Small gap"</span>
                        <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
                        <span class="docs-spacer-box">"Large gap"</span>
                    </div>

                    <div class="docs-row">
                        <span class="docs-spacer-box">"Left"</span>
                        <Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />
                        <span class="docs-spacer-box">"Right"</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class Marker" code=custom_class_code>
                <div class="docs-stack">
                    <span class="docs-spacer-box">"Custom vertical spacer"</span>
                    <Spacer
                        axis=SpacerAxis::Vertical
                        size=SpacerSize::Md
                        class_name="docs-spacer-guide".to_string()
                    />
                    <span class="docs-spacer-box">"Marker visible via custom class"</span>

                    <div class="docs-row">
                        <span class="docs-spacer-box">"Start"</span>
                        <Spacer
                            axis=SpacerAxis::Horizontal
                            size=SpacerSize::Lg
                            class_name="docs-spacer-guide".to_string()
                        />
                        <span class="docs-spacer-box">"End"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn scroll_shadow() -> AnyView {
    let default_code = r#"<ScrollShadow max_height_px=160>
  {rows}
</ScrollShadow>"#;

    let custom_class_code = r#"<ScrollShadow
  max_height_px=120
  class_name="docs-scroll-shadow-custom".to_string()
>
  {rows}
</ScrollShadow>"#;

    view! {
        <ComponentPage
            title="ScrollShadow"
            slug="scroll-shadow"
            group="Layout"
            description="Adds top/bottom shadow indicators with centralized edge/max-height state attrs."
        >
            <Playground title="Default Scrollable" code=default_code>
                <ScrollShadow max_height_px=160>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=20)
                            .map(|idx| {
                                view! { <div class="docs-scroll-shadow-item">{format!("Activity {idx}")}</div> }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>

            <Playground title="Custom Height + Class" code=custom_class_code>
                <ScrollShadow max_height_px=120 class_name="docs-scroll-shadow-custom".to_string()>
                    <div class="docs-stack docs-stack--tight">
                        {(1..=16)
                            .map(|idx| {
                                view! {
                                    <div class="docs-scroll-shadow-item">
                                        {format!("Notification {idx}")}
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </ScrollShadow>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn auto_height() -> AnyView {
    let (animated_open, set_animated_open) = signal(false);
    let (static_open, set_static_open) = signal(false);

    let animated_code = r#"let (open, set_open) = signal(false);
<Button on_press=...>"Toggle"</Button>
<AutoHeight class_name="docs-auto-height".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#;

    let static_code = r#"let (open, set_open) = signal(false);
let motion = AutoHeightMotion {
  animate_height: false,
  ..AutoHeightMotion::default()
};
<AutoHeight motion=motion class_name="docs-auto-height docs-auto-height--static-demo".to_string()>
  <Show when=open>...</Show>
</AutoHeight>"#;

    view! {
        <ComponentPage
            title="AutoHeight"
            slug="auto-height"
            group="Layout"
            description="Animates (or snaps) height changes via spring-driven CSS variables with centralized motion/class state attrs."
        >
            <Playground title="Animated Height" code=animated_code>
                <div class="docs-stack">
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_animated_open.update(|v| *v = !*v))
                    >
                        {move || if animated_open.get() { "Collapse" } else { "Expand" }}
                    </ui_components::Button>

                    <AutoHeight class_name="docs-auto-height".to_string()>
                        <Show when=move || animated_open.get()>
                            <div class="docs-stack">
                                <div>"AutoHeight content"</div>
                                <div class="ui-muted">"ResizeObserver + ui-motion spring."</div>
                                <div class="ui-muted">"Toggle quickly to verify stable interpolation."</div>
                            </div>
                        </Show>
                    </AutoHeight>
                </div>
            </Playground>

            <Playground title="Static Motion + Custom Class" code=static_code>
                <div class="docs-stack">
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| set_static_open.update(|v| *v = !*v))
                    >
                        {move || if static_open.get() { "Hide Static" } else { "Show Static" }}
                    </ui_components::Button>

                    <AutoHeight
                        motion=AutoHeightMotion {
                            animate_height: false,
                            ..AutoHeightMotion::default()
                        }
                        class_name="docs-auto-height docs-auto-height--static-demo".to_string()
                    >
                        <Show when=move || static_open.get()>
                            <div class="docs-stack">
                                <div>"Static mode content"</div>
                                <div class="ui-muted">"Uses custom motion contract (`animate_height=false`)."</div>
                                <div class="ui-muted">"Useful for reduced-motion or deterministic layout jumps."</div>
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
    let usage_code = r#"use ui_components::{UiRoot, Theme};

let theme = Signal::derive(|| Theme::dark());

<UiRoot theme=theme safe_area=true>
  // your app
</UiRoot>"#;

    let contract_code = r#"<UiRoot ...>
  // wrapper attrs:
  // data-slot="ui-root"
  // data-theme-scheme="light|dark"
  // data-state="default|safe-area"
  // data-safe-area="true" (optional)
</UiRoot>"#;

    view! {
        <ComponentPage
            title="UiRoot"
            slug="ui-root"
            group="Layout"
            description="Provider that injects theme tokens + layered component CSS and exposes stable root state attrs."
        >
            <Playground title="Usage" code=usage_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">
                        "This docs app already mounts a global UiRoot at startup."
                    </div>
                    <div class="docs-ui-root-note">
                        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place."
                    </div>
                    <div class="ui-muted">
                        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells."
                    </div>
                </div>
            </Playground>

            <Playground title="State Contract" code=contract_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">"`data-slot=ui-root` for stable root targeting."</div>
                    <div class="docs-ui-root-note">"`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`)."</div>
                    <div class="docs-ui-root-note">"`data-state` + `data-safe-area` describe safe-area mode."</div>
                    <div class="ui-muted">"Use these attrs to write app-level overrides without coupling to internal implementation details."</div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
