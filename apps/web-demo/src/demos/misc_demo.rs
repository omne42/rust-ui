use leptos::prelude::*;
use ui::{
    Alert, AlertVariant, AvatarGroup, AvatarGroupItem, Button, ButtonLoadingPlacement, Chip,
    Skeleton, Spinner,
};
use ui_layout::Card;

#[component]
pub fn MiscDemo() -> impl IntoView {
    let (loading, set_loading) = signal(false);

    let toggle_loading = move |_| set_loading.update(|value| *value = !*value);

    let avatars = vec![
        AvatarGroupItem {
            name: Some("Ada Lovelace".to_string()),
            src: None,
            alt: None,
        },
        AvatarGroupItem {
            name: Some("Grace Hopper".to_string()),
            src: None,
            alt: None,
        },
        AvatarGroupItem {
            name: Some("Linus Torvalds".to_string()),
            src: None,
            alt: None,
        },
        AvatarGroupItem {
            name: Some("Margaret Hamilton".to_string()),
            src: None,
            alt: None,
        },
        AvatarGroupItem {
            name: Some("Alan Turing".to_string()),
            src: None,
            alt: None,
        },
    ];

    view! {
        <section id="misc" class="demo-card">
            <h2>"Card / Alert / Chip / Skeleton / AvatarGroup / Spinner"</h2>
            <p>"Low-coupling primitives; style via tokens (OKLCH)."</p>

            <div class="demo-grid-2">
                <Card>
                    <div class="demo-stack">
                        <div class="demo-kv">"Card container"</div>
                        <Alert
                            variant=AlertVariant::Accent
                            title="Heads up".to_string()
                            description="This alert uses semantic tokens and stays readable in OLED.".to_string()
                        >
                            <Button variant=ui::ButtonVariant::Secondary on_press=Callback::new(toggle_loading)>
                                "Toggle loading"
                            </Button>
                        </Alert>
                        <div class="demo-row">
                            <Chip>"Default"</Chip>
                            <Chip variant=ui::ChipVariant::Accent>"Accent"</Chip>
                            <Chip variant=ui::ChipVariant::Outline>"Outline"</Chip>
                            <Chip
                                variant=ui::ChipVariant::Danger
                                on_dismiss=Callback::new(|_| {})
                                dismiss_aria_label="Remove chip".to_string()
                            >
                                "Dismiss"
                            </Chip>
                        </div>

                        <div class="demo-row">
                            {move || {
                                let is_loading = loading.get();
                                view! {
                                    <Button is_loading=is_loading loading_placement=ButtonLoadingPlacement::Start>
                                        "Start"
                                    </Button>
                                    <Button is_loading=is_loading loading_placement=ButtonLoadingPlacement::End>
                                        "End"
                                    </Button>
                                    <Button is_loading=is_loading loading_placement=ButtonLoadingPlacement::Center>
                                        "Center"
                                    </Button>
                                }
                            }}
                        </div>
                    </div>
                </Card>

                <Card>
                    <div class="demo-stack">
                        <div class="demo-kv">"Skeleton + AvatarGroup + Spinner"</div>
                        <div class="demo-row demo-row--tall">
                            <Skeleton class_name="demo-skeleton-line" />
                            <Skeleton class_name="demo-skeleton-line demo-skeleton-line--sm" />
                            <Skeleton variant=ui::SkeletonVariant::Circle class_name="demo-skeleton-circle" />
                            <Spinner />
                        </div>
                        <AvatarGroup items=avatars max=4 />
                        <Alert
                            variant=AlertVariant::Danger
                            title="Danger".to_string()
                            description="Use for destructive feedback; role=status by default today.".to_string()
                        >
                            <Button variant=ui::ButtonVariant::Destructive>"Retry"</Button>
                        </Alert>
                    </div>
                </Card>
            </div>
        </section>
    }
}
