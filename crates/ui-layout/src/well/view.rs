use crate::well::{
    WellStrings,
    logic::{self, WellDensity, WellTone},
};
use leptos::prelude::*;
use ui_headless::i18n;
use ui_headless::{A11yDirection, region_attrs};

const SLOT_WELL: &str = "well";
const STATE_INSET: &str = "inset";
const STATE_DEFAULT: &str = "default";
const BOOL_TRUE: &str = "true";

fn inset_state_attr(is_inset: bool) -> &'static str {
    if is_inset { STATE_INSET } else { STATE_DEFAULT }
}

#[component]
pub fn Well(
    #[prop(optional)] tone: Option<WellTone>,
    #[prop(optional)] density: Option<WellDensity>,
    #[prop(optional)] is_inset: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<WellStrings>();

    let normalized = logic::normalize_props(logic::WellNormalizeInput {
        tone,
        density,
        is_inset,
        aria_label,
        fallback_aria_label: strings.aria_label.as_ref().into(),
        class_name,
    });
    let region = region_attrs(normalized.aria_label, lang, dir);
    let role_attr = region.role;
    let aria_label = region.aria_label;
    let locale_lang = region.lang;
    let locale_dir = region.dir;
    let tone_source_attr = normalized.tone_source_attr;
    let density_source_attr = normalized.density_source_attr;
    let inset_source_attr = normalized.inset_source_attr;
    let class_name = StoredValue::new(normalized.class_name);
    let state_input = StoredValue::new(normalized.state_input);

    let state = Signal::derive(move || logic::resolve_state(state_input.get_value()));

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <section
            class=move || class.get()
            data-slot=SLOT_WELL
            data-tone=move || state.get().tone_attr
            data-tone-source=tone_source_attr
            data-density=move || state.get().density_attr
            data-density-source=density_source_attr
            data-state=move || inset_state_attr(state.get().is_inset)
            data-inset=move || state.get().is_inset.then_some(BOOL_TRUE)
            data-inset-source=inset_source_attr
            data-label-source=move || state.get().label_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some(BOOL_TRUE)
            data-class-source=move || state.get().class_source_attr
            role=role_attr
            aria-label=aria_label
            lang=locale_lang.clone()
            dir=locale_dir
        >
            {children()}
        </section>
    }
}
