use super::{CoachmarkAssetVariant, CoachmarkMotion, CoachmarkVariant, logic};
use crate::OnPress;
use crate::asset::{Asset, AssetSize};
use crate::button::{Button, ButtonVariant};
use crate::contextual_help::ContextualHelp;
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::PopoverPlacement;

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
    let normalized_aria_label = logic::normalize_optional_text(aria_label);
    let normalized_class_name = logic::normalize_optional_text(class_name);
    let normalized_primary_cta = logic::normalize_optional_text(primary_cta);
    let normalized_secondary_cta = logic::normalize_optional_text(secondary_cta);
    let normalized_asset_src = logic::normalize_optional_text(asset_src);
    let normalized_asset_label = logic::normalize_optional_text(asset_label)
        .unwrap_or_else(|| logic::DEFAULT_ASSET_LABEL.into());
    let normalized_asset_alt =
        logic::normalize_optional_text(asset_alt).unwrap_or_else(|| normalized_asset_label.clone());

    let normalized_shortcut_key = logic::normalize_optional_text(shortcut_key);
    let normalized_modifier_keys = logic::normalize_modifier_keys(modifier_keys);
    let has_shortcut = normalized_shortcut_key.is_some();

    let heading = logic::compose_heading(title, normalized_modifier_keys, normalized_shortcut_key);
    let step_label = logic::compose_step_label(current_step, total_steps);

    let has_asset_variant = asset_variant.is_some();
    let has_asset_src = normalized_asset_src.is_some();
    let has_asset = has_asset_variant || has_asset_src;
    let has_actions_slot = actions.is_some();
    let has_footer = step_label.is_some()
        || normalized_primary_cta.is_some()
        || normalized_secondary_cta.is_some()
        || has_actions_slot;

    let state = logic::resolve_state(logic::CoachmarkStateInput {
        variant_attr: variant.as_attr(),
        placement_attr: placement.as_str(),
        disabled,
        is_controlled: open.is_some(),
        has_footer,
        has_asset,
        has_custom_aria_label: normalized_aria_label.is_some(),
        has_custom_class_name: normalized_class_name.is_some(),
        has_shortcut,
        has_primary_cta: normalized_primary_cta.is_some(),
        has_secondary_cta: normalized_secondary_cta.is_some(),
        has_actions_slot,
        has_step_label: step_label.is_some(),
        has_asset_variant,
        has_asset_src,
    });
    let ui_action_attr = if has_footer {
        "navigate-step"
    } else {
        "read-guidance"
    };
    let ui_source_attr = if state.open_mode_attr == "controlled" {
        "external"
    } else {
        "internal"
    };
    let output_status_attr = if disabled { "draft" } else { "verified" };

    let class_name = logic::compose_class_name(normalized_class_name, state);
    let trigger_label =
        normalized_aria_label.unwrap_or_else(|| variant.default_label().to_string());

    let on_primary = on_primary.unwrap_or_else(|| Callback::new(|()| {}));
    let on_secondary = on_secondary.unwrap_or_else(|| Callback::new(|()| {}));

    let default_open = default_open.unwrap_or(false);
    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_: bool| {}));

    let class_name = StoredValue::new(class_name);
    let trigger_label = StoredValue::new(trigger_label);
    let heading = StoredValue::new(heading);

    let children = StoredValue::new(children);
    let primary_cta = StoredValue::new(normalized_primary_cta);
    let secondary_cta = StoredValue::new(normalized_secondary_cta);
    let step_label = StoredValue::new(step_label);
    let actions = StoredValue::new(actions);
    let on_primary = StoredValue::new(on_primary);
    let on_secondary = StoredValue::new(on_secondary);
    let asset_variant = StoredValue::new(asset_variant);
    let asset_src = StoredValue::new(normalized_asset_src);
    let asset_label = StoredValue::new(normalized_asset_label);
    let asset_alt = StoredValue::new(normalized_asset_alt);

    let footer_view = StoredValue::new(ViewFn::from(move || {
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
        .into_any()
    }));

    let content_view = StoredValue::new(ViewFn::from(move || {
        view! {
            <div
                class="ui-coachmark__content"
                data-slot="coachmark-content"
                data-state=state.state_attr
                data-variant=state.variant_attr
                data-placement=state.placement_attr
                data-open-mode=state.open_mode_attr
                data-footer=state.footer_attr
                data-asset=state.asset_attr
                data-cta=state.cta_attr
                data-label-source=state.label_source_attr
                data-class-source=state.class_source_attr
                data-shortcut=state.shortcut_attr
                data-actions=state.actions_attr
                data-steps=state.steps_attr
                data-asset-source=state.asset_source_attr
                data-has-asset=state.has_asset.then_some("true")
                data-custom-class=state.has_custom_class_name.then_some("true")
                data-ui-schema="ui.coachmark.agent-contract.v1"
                data-ui-schema-version="1"
                data-ui-intent="guided-tour"
                data-ui-action=ui_action_attr
                data-ui-state=state.state_attr
                data-ui-source=ui_source_attr
                data-ui-stream-support="optional"
                data-ui-stream-fallback="snapshot"
                data-ui-stream-mode="snapshot"
                data-ui-output-status=output_status_attr
                data-stream-mode="snapshot"
                data-stream-fallback="snapshot"
                data-output-status=output_status_attr
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
        }
        .into_any()
    }));

    if let Some(open) = open {
        if has_footer {
            view! {
                <ContextualHelp
                    variant=variant
                    aria_label=trigger_label.get_value()
                    disabled=disabled
                    placement=placement
                    motion=motion
                    open=open
                    default_open=default_open
                    on_open_change=on_open_change
                    heading=heading.get_value()
                    footer=move || footer_view.get_value().run()
                    class_name=class_name.get_value()
                >
                    {move || content_view.get_value().run()}
                </ContextualHelp>
            }
            .into_any()
        } else {
            view! {
                <ContextualHelp
                    variant=variant
                    aria_label=trigger_label.get_value()
                    disabled=disabled
                    placement=placement
                    motion=motion
                    open=open
                    default_open=default_open
                    on_open_change=on_open_change
                    heading=heading.get_value()
                    class_name=class_name.get_value()
                >
                    {move || content_view.get_value().run()}
                </ContextualHelp>
            }
            .into_any()
        }
    } else if has_footer {
        view! {
            <ContextualHelp
                variant=variant
                aria_label=trigger_label.get_value()
                disabled=disabled
                placement=placement
                motion=motion
                default_open=default_open
                on_open_change=on_open_change
                heading=heading.get_value()
                footer=move || footer_view.get_value().run()
                class_name=class_name.get_value()
            >
                {move || content_view.get_value().run()}
            </ContextualHelp>
        }
        .into_any()
    } else {
        view! {
            <ContextualHelp
                variant=variant
                aria_label=trigger_label.get_value()
                disabled=disabled
                placement=placement
                motion=motion
                default_open=default_open
                on_open_change=on_open_change
                heading=heading.get_value()
                class_name=class_name.get_value()
            >
                {move || content_view.get_value().run()}
            </ContextualHelp>
        }
        .into_any()
    }
}
