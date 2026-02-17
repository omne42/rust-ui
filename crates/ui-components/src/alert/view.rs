use crate::alert::{
    AlertMotion, AlertVariant,
    logic::{self, AlertStateInput},
    motion as alert_motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn Alert(
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: AlertMotion,
    children: Children,
) -> impl IntoView {
    let motion = crate::alert::motion::sanitize_motion(motion);
    let variant = logic::normalize_variant(variant);
    let title = logic::normalize_optional_text(title);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);
    let locale = locale_attrs(lang, dir);

    let node_ref: NodeRef<html::Section> = NodeRef::new();
    alert_motion::attach_motion(node_ref, motion);

    let state = logic::resolve_state(AlertStateInput {
        variant,
        has_title: title.is_some(),
        has_description: description.is_some(),
        has_actions: true,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    view! {
        <section
            class=class
            node_ref=node_ref
            data-slot="alert"
            data-state=state.state_attr
            data-variant=state.variant_attr
            data-title=state.title_attr
            data-description=state.description_attr
            data-actions=state.actions_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            role=state.role_attr
            aria-live=state.live_attr
            lang=locale.lang
            dir=locale.dir
            data-motion-source=if motion == AlertMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != AlertMotion::default()).then_some("true")
        >
            {title.map(|title| {
                view! {
                    <div class="ui-alert__title" data-slot="alert-title">
                        {title}
                    </div>
                }
            })}
            {description.map(|description| {
                view! {
                    <div class="ui-alert__description" data-slot="alert-description">
                        {description}
                    </div>
                }
            })}
            <div class="ui-alert__actions" data-slot="alert-actions">
                {children()}
            </div>
        </section>
    }
}
