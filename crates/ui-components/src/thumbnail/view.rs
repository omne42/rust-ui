use super::{ThumbnailMotion, ThumbnailSize, logic, motion as thumbnail_motion};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, locale_attrs};

const SLOT_THUMBNAIL: &str = "thumbnail";
const SLOT_THUMBNAIL_FRAME: &str = "thumbnail-frame";
const SLOT_THUMBNAIL_CONTENT: &str = "thumbnail-content";

const CLASS_THUMBNAIL_FRAME: &str = "ui-thumbnail__frame";
const CLASS_THUMBNAIL_CONTENT: &str = "ui-thumbnail__content";

fn render_thumbnail_content(children: Children) -> impl IntoView {
    view! {
        <div class=CLASS_THUMBNAIL_FRAME data-slot=SLOT_THUMBNAIL_FRAME>
            <div class=CLASS_THUMBNAIL_CONTENT data-slot=SLOT_THUMBNAIL_CONTENT>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Thumbnail(
    #[prop(optional)] size: ThumbnailSize,
    #[prop(optional, into)] background: Option<String>,
    #[prop(optional, into)] cover: Option<bool>,
    #[prop(optional, into)] layer: Option<bool>,
    #[prop(optional, into)] selected: Option<bool>,
    #[prop(optional, into)] focused: Option<bool>,
    #[prop(optional)] motion: ThumbnailMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    children: Children,
) -> impl IntoView {
    let locale = locale_attrs(logic::normalize_lang(lang), dir);
    let view_state = logic::resolve_view_state(
        logic::ThumbnailViewStateInput {
            size,
            cover,
            layer,
            selected,
            focused,
            motion_source: logic::resolve_motion_source(motion),
        },
        logic::normalize_input(background, class_name),
    );
    let agent_contract = logic::resolve_agent_contract(&view_state);
    let state = view_state.state;
    let class = view_state.class_name;
    let inline_style = StoredValue::new(Some(view_state.inline_css_vars));
    let motion_source = view_state.motion_source;
    let motion_active = view_state.motion_active;
    let cover_source = view_state.cover_source;
    let layer_source = view_state.layer_source;
    let selected_source = view_state.selected_source;
    let focused_source = view_state.focused_source;
    let background_source = view_state.background_source;
    let class_name_source = view_state.class_name_source;
    let content = render_thumbnail_content(children);

    let node_ref: NodeRef<html::Div> = NodeRef::new();
    let active = Signal::derive(move || motion_active);
    thumbnail_motion::attach_motion(node_ref, active, motion);

    view! {
        <div
            class=class
            node_ref=node_ref
            style=inline_style.get_value().unwrap_or_default()
            lang=locale.lang.clone()
            dir=locale.dir
            data-slot=SLOT_THUMBNAIL
            data-size=state.size_attr
            data-cover=state.cover.then_some("true")
            data-layer=state.layer.then_some("true")
            data-selected=state.selected.then_some("true")
            data-focused=state.focused.then_some("true")
            data-cover-source=cover_source.as_attr()
            data-layer-source=layer_source.as_attr()
            data-selected-source=selected_source.as_attr()
            data-focused-source=focused_source.as_attr()
            data-state=state.data_state.as_attr()
            data-background=state.has_background.then_some("true")
            data-background-source=background_source.as_attr()
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=class_name_source.as_attr()
            data-motion-source=motion_source.as_attr()
            data-custom-motion=motion_source.custom_motion_attr()
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version.as_str()
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
        >
            {content}
        </div>
    }
}
