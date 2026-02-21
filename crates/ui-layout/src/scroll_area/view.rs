use crate::scroll_area::{
    ScrollAreaMotion,
    logic::{self, ScrollAreaDisableInput, ScrollAreaRootInput},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, CommonStrings, ScrollAreaOptions, use_scroll_area, use_ui_i18n};

const SLOT_SCROLL_AREA: &str = "scroll-area";
const SLOT_SCROLL_AREA_VIEWPORT: &str = "scroll-area-viewport";
const CLASS_SCROLL_AREA_VIEWPORT: &str = "ui-scroll-area__viewport";
const BOOL_TRUE: &str = "true";
const MOTION_SOURCE_CUSTOM: &str = "custom";

#[component]
pub fn ScrollArea(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation,
    #[prop(optional)] max_height_px: Option<u32>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] motion: ScrollAreaMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();

    let root = logic::normalize_root_state(ScrollAreaRootInput {
        class_name,
        aria_label,
        fallback_aria_label: common.scroll_area_aria_label.as_ref().into(),
        orientation,
        max_height_px,
        disabled: ScrollAreaDisableInput { is_disabled },
    });
    let state = root.state;
    let class = logic::compose_class_name(root.class_name, state);
    let semantics = use_scroll_area(ScrollAreaOptions {
        state,
        aria_label: root.aria_label,
        lang,
        dir,
    });
    let motion = motion::sanitize_motion(motion);
    let motion_source = motion::source_attr(motion);
    let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));
    let has_custom_motion = motion_source == MOTION_SOURCE_CUSTOM;
    let agent_contract = logic::resolve_agent_contract(state, root.disabled_source_attr);

    let viewport_ref: NodeRef<html::Div> = NodeRef::new();

    #[cfg(target_arch = "wasm32")]
    let set_max_height = {
        let viewport_ref = viewport_ref;
        let max_height_px = StoredValue::new(state.max_height_px);
        move || {
            use leptos::wasm_bindgen::JsCast;

            let Some(px) = max_height_px.get_value() else {
                return;
            };

            let Some(div) = viewport_ref.get_untracked() else {
                return;
            };

            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();
            let max_height = format!("{px}px");
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-scroll-area-max-h",
                max_height.as_str()
            );
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let set_max_height = { || {} };

    Effect::new(move |_| {
        drop(viewport_ref.get());
        set_max_height();
    });

    view! {
        <div
            class=class
            style=inline_style.get_value().unwrap_or_default()
            data-slot=SLOT_SCROLL_AREA
            data-motion-source=motion_source
            data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)
            data-orientation=semantics.root_attrs.data_orientation
            data-disabled=semantics.root_attrs.data_disabled
            data-disabled-source=root.disabled_source_attr.as_attr()
            data-max-height=semantics.root_attrs.data_max_height
            data-aria-source=semantics.root_attrs.data_aria_source
            data-class-source=semantics.root_attrs.data_class_source
            data-custom-class=semantics.root_attrs.data_custom_class
            data-ui-schema=agent_contract.schema_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-stream-mode=agent_contract.stream_mode_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=agent_contract.action_attr
            data-ui-state=agent_contract.state_attr
            data-ui-source=agent_contract.source_attr
            role=semantics.root_attrs.role
            aria-label=semantics.root_attrs.aria_label
            lang=semantics.root_attrs.lang
            dir=semantics.root_attrs.dir
        >
            <div
                class=CLASS_SCROLL_AREA_VIEWPORT
                node_ref=viewport_ref
                data-slot=SLOT_SCROLL_AREA_VIEWPORT
                tabindex=semantics.viewport_attrs.tabindex
                aria-disabled=semantics.viewport_attrs.aria_disabled
            >
                {children()}
            </div>
        </div>
    }
}
