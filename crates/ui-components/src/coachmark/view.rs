use super::{CoachmarkAssetVariant, CoachmarkMotion, CoachmarkVariant};
use crate::{Asset, AssetSize, Button, ButtonVariant, ContextualHelp, OnPress};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::PopoverPlacement;

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn compose_heading(
    title: Option<String>,
    modifier_keys: Vec<String>,
    shortcut_key: Option<String>,
) -> String {
    let title = normalize_optional_text(title).unwrap_or_else(|| "Coachmark".to_string());

    let mut keys = modifier_keys
        .into_iter()
        .filter_map(|key| normalize_optional_text(Some(key)))
        .collect::<Vec<_>>();

    if let Some(shortcut_key) = normalize_optional_text(shortcut_key) {
        keys.push(shortcut_key);
    }

    if keys.is_empty() {
        title
    } else {
        format!("{title} ({})", keys.join(" + "))
    }
}

fn compose_step_label(current_step: Option<usize>, total_steps: Option<usize>) -> Option<String> {
    match (current_step, total_steps) {
        (Some(current_step), Some(total_steps)) if total_steps > 1 => {
            Some(format!("{current_step} of {total_steps}"))
        }
        _ => None,
    }
}

#[component]
pub fn Coachmark(
    children: ChildrenFn,
    #[prop(optional)] variant: CoachmarkVariant,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] motion: CoachmarkMotion,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] current_step: Option<usize>,
    #[prop(optional)] total_steps: Option<usize>,
    #[prop(optional, into)] primary_cta: Option<String>,
    #[prop(optional, into)] secondary_cta: Option<String>,
    #[prop(optional)] on_primary: Option<OnPress>,
    #[prop(optional)] on_secondary: Option<OnPress>,
    #[prop(optional, into)] shortcut_key: Option<String>,
    #[prop(optional)] modifier_keys: Vec<String>,
    #[prop(optional)] asset_variant: Option<CoachmarkAssetVariant>,
    #[prop(optional, into)] asset_label: Option<String>,
    #[prop(optional, into)] asset_src: Option<String>,
    #[prop(optional, into)] asset_alt: Option<String>,
    #[prop(optional, into)] actions: Option<ViewFn>,
) -> impl IntoView {
    let heading = compose_heading(title, modifier_keys, shortcut_key);
    let aria_label = aria_label.unwrap_or_default();
    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-coachmark {class_name}"))
        .unwrap_or_else(|| "ui-coachmark".to_string());

    let step_label = compose_step_label(current_step, total_steps);
    let primary_cta = normalize_optional_text(primary_cta);
    let secondary_cta = normalize_optional_text(secondary_cta);

    let on_primary = on_primary.unwrap_or_else(|| Callback::new(|()| {}));
    let on_secondary = on_secondary.unwrap_or_else(|| Callback::new(|()| {}));

    let asset_src = normalize_optional_text(asset_src);
    let asset_label =
        normalize_optional_text(asset_label).unwrap_or_else(|| "Coachmark asset".to_string());
    let asset_alt = normalize_optional_text(asset_alt).unwrap_or_else(|| asset_label.clone());

    let has_asset = asset_variant.is_some() || asset_src.is_some();

    let default_open = default_open.unwrap_or(false);
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    let children = StoredValue::new(children);
    let primary_cta = StoredValue::new(primary_cta);
    let secondary_cta = StoredValue::new(secondary_cta);
    let step_label = StoredValue::new(step_label);
    let actions = StoredValue::new(actions);
    let on_primary = StoredValue::new(on_primary);
    let on_secondary = StoredValue::new(on_secondary);
    let asset_variant = StoredValue::new(asset_variant);
    let asset_src = StoredValue::new(asset_src);
    let asset_label = StoredValue::new(asset_label);
    let asset_alt = StoredValue::new(asset_alt);

    let has_footer = step_label.get_value().is_some()
        || primary_cta.get_value().is_some()
        || secondary_cta.get_value().is_some()
        || actions.get_value().is_some();

    if let Some(open) = open {
        if has_footer {
            view! {
                <ContextualHelp
                    variant=variant
                    aria_label=aria_label
                    disabled=disabled
                    placement=placement
                    motion=motion
                    open=open
                    default_open=default_open
                    on_open_change=on_open_change
                    heading=heading
                    footer=move || {
                        view! {
                            <div class="ui-coachmark__footer" data-slot="coachmark-footer">
                                {step_label.get_value().map(|step_label| {
                                    view! {
                                        <span
                                            class="ui-coachmark__steps ui-muted"
                                            data-slot="coachmark-steps"
                                            role="status"
                                            aria-live="polite"
                                        >
                                            {step_label}
                                        </span>
                                    }
                                })}
                                <div class="ui-coachmark__actions" data-slot="coachmark-actions">
                                    {secondary_cta.get_value().map(|secondary_cta| {
                                        view! {
                                            <Button
                                                variant=ButtonVariant::Secondary
                                                on_press=on_secondary.get_value()
                                                class_name="ui-coachmark__button ui-coachmark__button--secondary".to_string()
                                            >
                                                {secondary_cta}
                                            </Button>
                                        }
                                    })}
                                    {primary_cta.get_value().map(|primary_cta| {
                                        view! {
                                            <Button
                                                variant=ButtonVariant::Accent
                                                on_press=on_primary.get_value()
                                                class_name="ui-coachmark__button ui-coachmark__button--primary".to_string()
                                            >
                                                {primary_cta}
                                            </Button>
                                        }
                                    })}
                                    {actions.get_value().map(|actions| {
                                        view! {
                                            <span class="ui-coachmark__actions-extra" data-slot="coachmark-actions-extra">
                                                {actions.run()}
                                            </span>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    }
                    class_name=class_name
                >
                    <div
                        class="ui-coachmark__content"
                        data-slot="coachmark-content"
                        data-has-asset=has_asset.then_some("true")
                    >
                        {move || {
                            if let Some(asset_src) = asset_src.get_value() {
                                view! {
                                    <Asset size=AssetSize::Size700 class_name="ui-coachmark__asset".to_string()>
                                        <img src=asset_src alt=asset_alt.get_value() />
                                    </Asset>
                                }
                                    .into_any()
                            } else if let Some(asset_variant) = asset_variant.get_value() {
                                view! {
                                    <Asset
                                        variant=asset_variant
                                        size=AssetSize::Size700
                                        label=asset_label.get_value()
                                        class_name="ui-coachmark__asset".to_string()
                                    />
                                }
                                    .into_any()
                            } else {
                                ().into_any()
                            }
                        }}

                        <div class="ui-coachmark__body" data-slot="coachmark-body">
                            {children.get_value()()}
                        </div>
                    </div>
                </ContextualHelp>
            }
            .into_any()
        } else {
            view! {
                <ContextualHelp
                    variant=variant
                    aria_label=aria_label
                    disabled=disabled
                    placement=placement
                    motion=motion
                    open=open
                    default_open=default_open
                    on_open_change=on_open_change
                    heading=heading
                    class_name=class_name
                >
                    <div
                        class="ui-coachmark__content"
                        data-slot="coachmark-content"
                        data-has-asset=has_asset.then_some("true")
                    >
                        {move || {
                            if let Some(asset_src) = asset_src.get_value() {
                                view! {
                                    <Asset size=AssetSize::Size700 class_name="ui-coachmark__asset".to_string()>
                                        <img src=asset_src alt=asset_alt.get_value() />
                                    </Asset>
                                }
                                    .into_any()
                            } else if let Some(asset_variant) = asset_variant.get_value() {
                                view! {
                                    <Asset
                                        variant=asset_variant
                                        size=AssetSize::Size700
                                        label=asset_label.get_value()
                                        class_name="ui-coachmark__asset".to_string()
                                    />
                                }
                                    .into_any()
                            } else {
                                ().into_any()
                            }
                        }}

                        <div class="ui-coachmark__body" data-slot="coachmark-body">
                            {children.get_value()()}
                        </div>
                    </div>
                </ContextualHelp>
            }
            .into_any()
        }
    } else if has_footer {
        view! {
            <ContextualHelp
                variant=variant
                aria_label=aria_label
                disabled=disabled
                placement=placement
                motion=motion
                default_open=default_open
                on_open_change=on_open_change
                heading=heading
                footer=move || {
                    view! {
                        <div class="ui-coachmark__footer" data-slot="coachmark-footer">
                            {step_label.get_value().map(|step_label| {
                                view! {
                                    <span
                                        class="ui-coachmark__steps ui-muted"
                                        data-slot="coachmark-steps"
                                        role="status"
                                        aria-live="polite"
                                    >
                                        {step_label}
                                    </span>
                                }
                            })}
                            <div class="ui-coachmark__actions" data-slot="coachmark-actions">
                                {secondary_cta.get_value().map(|secondary_cta| {
                                    view! {
                                        <Button
                                            variant=ButtonVariant::Secondary
                                            on_press=on_secondary.get_value()
                                            class_name="ui-coachmark__button ui-coachmark__button--secondary".to_string()
                                        >
                                            {secondary_cta}
                                        </Button>
                                    }
                                })}
                                {primary_cta.get_value().map(|primary_cta| {
                                    view! {
                                        <Button
                                            variant=ButtonVariant::Accent
                                            on_press=on_primary.get_value()
                                            class_name="ui-coachmark__button ui-coachmark__button--primary".to_string()
                                        >
                                            {primary_cta}
                                        </Button>
                                    }
                                })}
                                {actions.get_value().map(|actions| {
                                    view! {
                                        <span class="ui-coachmark__actions-extra" data-slot="coachmark-actions-extra">
                                            {actions.run()}
                                        </span>
                                    }
                                })}
                            </div>
                        </div>
                    }
                }
                class_name=class_name
            >
                <div
                    class="ui-coachmark__content"
                    data-slot="coachmark-content"
                    data-has-asset=has_asset.then_some("true")
                >
                    {move || {
                        if let Some(asset_src) = asset_src.get_value() {
                            view! {
                                <Asset size=AssetSize::Size700 class_name="ui-coachmark__asset".to_string()>
                                    <img src=asset_src alt=asset_alt.get_value() />
                                </Asset>
                            }
                                .into_any()
                        } else if let Some(asset_variant) = asset_variant.get_value() {
                            view! {
                                <Asset
                                    variant=asset_variant
                                    size=AssetSize::Size700
                                    label=asset_label.get_value()
                                    class_name="ui-coachmark__asset".to_string()
                                />
                            }
                                .into_any()
                        } else {
                            ().into_any()
                        }
                    }}

                    <div class="ui-coachmark__body" data-slot="coachmark-body">
                        {children.get_value()()}
                    </div>
                </div>
            </ContextualHelp>
        }
        .into_any()
    } else {
        view! {
            <ContextualHelp
                variant=variant
                aria_label=aria_label
                disabled=disabled
                placement=placement
                motion=motion
                default_open=default_open
                on_open_change=on_open_change
                heading=heading
                class_name=class_name
            >
                <div
                    class="ui-coachmark__content"
                    data-slot="coachmark-content"
                    data-has-asset=has_asset.then_some("true")
                >
                    {move || {
                        if let Some(asset_src) = asset_src.get_value() {
                            view! {
                                <Asset size=AssetSize::Size700 class_name="ui-coachmark__asset".to_string()>
                                    <img src=asset_src alt=asset_alt.get_value() />
                                </Asset>
                            }
                                .into_any()
                        } else if let Some(asset_variant) = asset_variant.get_value() {
                            view! {
                                <Asset
                                    variant=asset_variant
                                    size=AssetSize::Size700
                                    label=asset_label.get_value()
                                    class_name="ui-coachmark__asset".to_string()
                                />
                            }
                                .into_any()
                        } else {
                            ().into_any()
                        }
                    }}

                    <div class="ui-coachmark__body" data-slot="coachmark-body">
                        {children.get_value()()}
                    </div>
                </div>
            </ContextualHelp>
        }
        .into_any()
    }
}
