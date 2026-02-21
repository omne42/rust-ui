use super::{CoachmarkAssetVariant, CoachmarkMotion, CoachmarkVariant, logic, motion};
use crate::OnPress;
use crate::asset::{Asset, AssetSize};
use crate::button::{Button, ButtonVariant};
use crate::contextual_help::ContextualHelp;
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::{A11yDirection, PopoverPlacement};

const COACHMARK_BUTTON_SECONDARY_CLASS: &str =
    "ui-coachmark__button ui-coachmark__button--secondary";
const COACHMARK_BUTTON_PRIMARY_CLASS: &str = "ui-coachmark__button ui-coachmark__button--primary";
const COACHMARK_ASSET_CLASS: &str = "ui-coachmark__asset";
const COACHMARK_UI_STREAM_SUPPORT: &str = "optional";
const COACHMARK_STREAM_MODE_SNAPSHOT: &str = "snapshot";
const DATA_TRUE_ATTR: &str = "true";

fn render_footer_fragment(
    step_label: Option<String>,
    secondary_cta: Option<String>,
    primary_cta: Option<String>,
    on_secondary: OnPress,
    on_primary: OnPress,
    actions: Option<ViewFn>,
) -> AnyView {
    view! {
        <div class="ui-coachmark__footer" data-slot="coachmark-footer">
            {step_label.map(|step_label| {
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
                {secondary_cta.map(|secondary_cta| {
                    view! {
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=on_secondary
                            class_name=COACHMARK_BUTTON_SECONDARY_CLASS.to_string()
                        >
                            {secondary_cta}
                        </Button>
                    }
                })}

                {primary_cta.map(|primary_cta| {
                    view! {
                        <Button
                            variant=ButtonVariant::Accent
                            on_press=on_primary
                            class_name=COACHMARK_BUTTON_PRIMARY_CLASS.to_string()
                        >
                            {primary_cta}
                        </Button>
                    }
                })}

                {actions.map(|actions| {
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
}

fn render_content_fragment(
    state: logic::CoachmarkState,
    agent_contract: logic::CoachmarkAgentContract,
    asset_src: Option<String>,
    asset_alt: String,
    asset_variant: Option<CoachmarkAssetVariant>,
    asset_label: String,
    children: ChildrenFn,
) -> AnyView {
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
            data-has-asset=state.has_asset.then_some(DATA_TRUE_ATTR)
            data-custom-class=state.has_custom_class_name.then_some(DATA_TRUE_ATTR)
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version.as_str()
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
            data-ui-state-source=agent_contract.state_source
            data-ui-action-source=agent_contract.action_source
            data-ui-render-path=agent_contract.render_path
            data-ui-stream-support=COACHMARK_UI_STREAM_SUPPORT
            data-ui-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT
            data-ui-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT
            data-ui-output-status=agent_contract.output_status.as_str()
            data-stream-mode=COACHMARK_STREAM_MODE_SNAPSHOT
            data-stream-fallback=COACHMARK_STREAM_MODE_SNAPSHOT
            data-output-status=agent_contract.output_status.as_str()
        >
            {if let Some(asset_src) = asset_src {
                view! {
                    <Asset size=AssetSize::Size700 class_name=COACHMARK_ASSET_CLASS.to_string()>
                        <img src=asset_src alt=asset_alt />
                    </Asset>
                }
                .into_any()
            } else if let Some(asset_variant) = asset_variant {
                view! {
                    <Asset
                        variant=asset_variant
                        size=AssetSize::Size700
                        label=asset_label
                        class_name=COACHMARK_ASSET_CLASS.to_string()
                    />
                }
                .into_any()
            } else {
                ().into_any()
            }}

            <div class="ui-coachmark__body" data-slot="coachmark-body">
                {children()}
            </div>
        </div>
    }
    .into_any()
}

#[component]
pub fn Coachmark(
    children: ChildrenFn,
    #[prop(optional)] variant: CoachmarkVariant,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
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
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] actions: Option<ViewFn>,
) -> impl IntoView {
    let motion = motion::resolve_motion(motion);
    let has_actions_slot = actions.is_some();
    let view_model = logic::resolve_view_model(logic::CoachmarkViewModelInput {
        variant,
        placement,
        is_disabled,
        disabled,
        is_controlled: open.is_some(),
        aria_label,
        class_name,
        title,
        current_step,
        total_steps,
        primary_cta,
        secondary_cta,
        shortcut_key,
        modifier_keys,
        has_actions_slot,
        asset_variant,
        asset_label,
        asset_src,
        asset_alt,
        lang,
    });

    let is_disabled = view_model.is_disabled;
    let has_footer = view_model.has_footer;
    let state = view_model.state;
    let agent_contract = view_model.agent_contract;

    let on_primary = logic::resolve_on_press(on_primary);
    let on_secondary = logic::resolve_on_press(on_secondary);

    let default_open = logic::resolve_default_open(default_open);
    let on_open_change = logic::resolve_on_open_change(on_open_change);

    let class_name = StoredValue::new(view_model.class_name);
    let trigger_label = StoredValue::new(view_model.trigger_label);
    let heading = StoredValue::new(view_model.heading);

    let children = StoredValue::new(children);
    let primary_cta = StoredValue::new(view_model.primary_cta);
    let secondary_cta = StoredValue::new(view_model.secondary_cta);
    let step_label = StoredValue::new(view_model.step_label);
    let actions = StoredValue::new(actions);
    let on_primary = StoredValue::new(on_primary);
    let on_secondary = StoredValue::new(on_secondary);
    let asset_variant = StoredValue::new(view_model.asset_variant);
    let asset_src = StoredValue::new(view_model.asset_src);
    let asset_label = StoredValue::new(view_model.asset_label);
    let asset_alt = StoredValue::new(view_model.asset_alt);
    let lang = StoredValue::new(view_model.lang.unwrap_or_default());
    let dir = StoredValue::new(dir);

    let footer_view = StoredValue::new(ViewFn::from(move || {
        render_footer_fragment(
            step_label.get_value(),
            secondary_cta.get_value(),
            primary_cta.get_value(),
            on_secondary.get_value(),
            on_primary.get_value(),
            actions.get_value(),
        )
    }));

    let content_view = StoredValue::new(ViewFn::from(move || {
        render_content_fragment(
            state,
            agent_contract,
            asset_src.get_value(),
            asset_alt.get_value(),
            asset_variant.get_value(),
            asset_label.get_value(),
            children.get_value(),
        )
    }));

    let render_contextual_help = |open: Option<Signal<bool>>, has_footer: bool| -> AnyView {
        let dir = dir.get_value();
        let lang = lang.get_value();

        if has_footer {
            match (open, dir) {
                (Some(open), Some(dir)) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
                        dir=dir
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
                .into_any(),
                (Some(open), None) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
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
                .into_any(),
                (None, Some(dir)) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
                        dir=dir
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        footer=move || footer_view.get_value().run()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
                (None, None) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        footer=move || footer_view.get_value().run()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
            }
        } else {
            match (open, dir) {
                (Some(open), Some(dir)) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
                        dir=dir
                        open=open
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
                (Some(open), None) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
                        open=open
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
                (None, Some(dir)) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang.clone()
                        dir=dir
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
                (None, None) => view! {
                    <ContextualHelp
                        variant=variant
                        aria_label=trigger_label.get_value()
                        disabled=is_disabled
                        placement=placement
                        motion=motion
                        lang=lang
                        default_open=default_open
                        on_open_change=on_open_change
                        heading=heading.get_value()
                        class_name=class_name.get_value()
                    >
                        {move || content_view.get_value().run()}
                    </ContextualHelp>
                }
                .into_any(),
            }
        }
    };

    render_contextual_help(open, has_footer)
}
