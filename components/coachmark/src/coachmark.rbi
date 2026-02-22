pub type CoachmarkVariant = ui::contextual_help::ContextualHelpVariant;
pub type CoachmarkAssetVariant = ui::asset::AssetVariant;
pub type OnPress = ui::OnPress;
pub type PopoverPlacement = ui_headless::PopoverPlacement;
pub type A11yDirection = ui_headless::A11yDirection;
pub type PopoverMotion = ui::popover::PopoverMotion;

pub struct CoachmarkMotion {
    pub popover: PopoverMotion,
}

pub fn Coachmark(
    children: leptos::children::ChildrenFn,
    variant: CoachmarkVariant,
    aria_label: Option<String>,
    is_disabled: Option<bool>,
    disabled: Option<bool>,
    placement: PopoverPlacement,
    motion: CoachmarkMotion,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    title: Option<String>,
    class_name: Option<String>,
    current_step: Option<usize>,
    total_steps: Option<usize>,
    primary_cta: Option<String>,
    secondary_cta: Option<String>,
    on_primary: Option<OnPress>,
    on_secondary: Option<OnPress>,
    shortcut_key: Option<String>,
    modifier_keys: Vec<String>,
    asset_variant: Option<CoachmarkAssetVariant>,
    asset_label: Option<String>,
    asset_src: Option<String>,
    asset_alt: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
    actions: Option<leptos::children::ViewFn>,
) -> impl leptos::prelude::IntoView;
