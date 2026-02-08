use crate::menu_section::{
    MenuSectionStateInput,
    logic::{self, MenuSectionHeadingTone},
};
use leptos::{children::Children, prelude::*};

#[component]
pub fn MenuSection(
    children: Children,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] item_count: Option<usize>,
    #[prop(optional)] heading_tone: MenuSectionHeadingTone,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] sticky_heading: bool,
    #[prop(optional)] show_divider: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let title = logic::normalize_optional_text(title);
    let has_title = title.is_some();
    let title = StoredValue::new(title);

    let resolved_item_count = item_count.unwrap_or(1);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::resolve_state(MenuSectionStateInput {
            heading_tone,
            item_count: resolved_item_count,
            disabled,
            sticky_heading,
            show_divider,
            has_title,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            role="group"
            aria-label=aria_label
            aria-disabled=disabled.then_some("true")
            data-slot="menu-section"
            data-tone=move || state.get().heading_tone_attr
            data-state=move || state.get().data_state_attr
            data-item-count=move || state.get().item_count.to_string()
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-has-title=move || state.get().has_title.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-sticky-heading=move || state.get().is_sticky_heading.then_some("true")
            data-divided=move || state.get().has_divider.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-title-source=move || state.get().title_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <Show when=move || state.get().has_title>
                <header
                    class="ui-menu-section__header"
                    data-slot="menu-section-header"
                    data-sticky=move || state.get().is_sticky_heading.then_some("true")
                >
                    {move || title.get_value().unwrap_or_default()}
                </header>
            </Show>

            <div class="ui-menu-section__items" data-slot="menu-section-items">
                {children()}
            </div>

            <Show when=move || state.get().has_divider>
                <div class="ui-menu-section__divider" data-slot="menu-section-divider"></div>
            </Show>
        </section>
    }
}
