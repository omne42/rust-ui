use crate::{
    EmptyStateMotion, EmptyStateStrings,
    logic::{self, EmptyStateAlign, EmptyStateTone},
    motion,
};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::i18n;
use ui_headless::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};

fn render_icon_slot(icon: Option<StoredValue<ViewFn>>) -> impl IntoView {
    icon.map(|icon| {
        view! {
            <div class="ui-empty-state__icon" data-slot="empty-state-icon" aria-hidden="true">
                {icon.get_value().run()}
            </div>
        }
    })
}

fn render_actions_slot(actions: Option<StoredValue<ViewFn>>) -> impl IntoView {
    actions.map(|actions| {
        view! {
            <div class="ui-empty-state__actions" data-slot="empty-state-actions">
                {actions.get_value().run()}
            </div>
        }
    })
}

#[component]
pub fn EmptyState(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] tone: EmptyStateTone,
    #[prop(optional)] align: EmptyStateAlign,
    #[prop(optional)] is_compact: bool,
    #[prop(optional)] is_bordered: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: EmptyStateMotion,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional, into)] actions: Option<ViewFn>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<EmptyStateStrings>();
    let logic::EmptyStateResolvedDefaults {
        title,
        description,
        aria_label,
        class_name,
        has_custom_title,
        has_custom_description,
        has_custom_aria_label,
        has_custom_class_name,
    } = logic::resolve_defaults(
        title,
        description,
        aria_label,
        class_name,
        strings.default_title.as_ref(),
        strings.default_description.as_ref(),
        strings.default_aria_label.as_ref(),
    );
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let live_region = live_region_attrs(LiveRegionPriority::Polite);

    let has_icon = icon.is_some();
    let has_actions = actions.is_some();

    let icon = icon.map(StoredValue::new);
    let actions = actions.map(StoredValue::new);

    let class_name = StoredValue::new(class_name);
    let motion = motion::sanitize_motion(motion);

    let state = Memo::new(move |_| {
        logic::resolve_render_state(logic::EmptyStateRenderStateInput {
            tone,
            align,
            is_compact,
            is_bordered,
            has_icon,
            has_actions,
            has_custom_title,
            has_custom_description,
            has_custom_aria_label,
            has_custom_class_name,
            motion,
        })
    });

    let class =
        Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get().state));
    let root_ref = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    view! {
        <section
            node_ref=root_ref
            class=move || class.get()
            data-slot="empty-state"
            data-tone=move || state.get().state.tone_attr
            data-align=move || state.get().state.align_attr
            data-state=move || state.get().state.data_state_attr
            data-compact=move || state.get().state.is_compact.then_some("true")
            data-bordered=move || state.get().state.is_bordered.then_some("true")
            data-icon=move || state.get().state.has_icon.then_some("true")
            data-actions=move || state.get().state.has_actions.then_some("true")
            data-title-source=move || state.get().state.title_source_attr
            data-description-source=move || state.get().state.description_source_attr
            data-aria-source=move || state.get().state.aria_source_attr
            data-custom-class=move || state.get().state.has_custom_class_name.then_some("true")
            data-class-source=move || state.get().state.class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-custom-motion=move || state.get().has_custom_motion.then_some("true")
            data-ui-schema=move || state.get().agent_contract.schema_name
            data-ui-schema-version=move || state.get().agent_contract.schema_version
            data-ui-intent=move || state.get().agent_contract.intent.as_attr()
            data-ui-action=move || state.get().agent_contract.action.as_attr()
            data-ui-state=move || state.get().agent_contract.state
            data-ui-source=move || state.get().agent_contract.source.as_attr()
            data-ui-streaming=move || state.get().agent_contract.streaming_support.as_attr()
            data-ui-render-mode=move || state.get().agent_contract.render_mode.as_attr()
            data-ui-fallback=move || state.get().agent_contract.fallback_mode.as_attr()
            data-ui-output-status=move || state.get().agent_contract.output_status.as_attr()
            role=live_region.role
            aria-live=live_region.aria_live
            aria-label=aria_label
            lang=locale.lang.clone()
            dir=locale.dir
        >
            {render_icon_slot(icon)}
            <h3 class="ui-empty-state__title" data-slot="empty-state-title">{title.get_value()}</h3>
            <p class="ui-empty-state__description" data-slot="empty-state-description">
                {description.get_value()}
            </p>
            {render_actions_slot(actions)}
        </section>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
