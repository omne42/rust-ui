use super::{ThumbnailMotion, ThumbnailSize, logic, motion as thumbnail_motion};
use leptos::{html, prelude::*};

#[component]
pub fn Thumbnail(
    #[prop(optional)] size: ThumbnailSize,
    #[prop(optional, into)] background: Option<String>,
    #[prop(optional)] cover: bool,
    #[prop(optional)] layer: bool,
    #[prop(optional)] selected: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] motion: ThumbnailMotion,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let background = logic::sanitize_background(background);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(logic::ThumbnailStateInput {
        size,
        cover,
        layer,
        selected,
        focused,
        has_background: background.is_some(),
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    let node_ref: NodeRef<html::Div> = NodeRef::new();
    let active = Signal::derive(move || selected || focused);
    thumbnail_motion::attach_motion(node_ref, active, motion);

    let motion_source = if motion == ThumbnailMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != ThumbnailMotion::default()).then_some("true");

    view! {
        <div
            class=class
            node_ref=node_ref
            style=logic::compose_inline_style(background.as_deref()).unwrap_or_default()
            data-slot="thumbnail"
            data-size=state.size_attr
            data-cover=state.cover.then_some("true")
            data-layer=state.layer.then_some("true")
            data-selected=state.selected.then_some("true")
            data-focused=state.focused.then_some("true")
            data-state=state.data_state_attr
            data-background=state.has_background.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
        >
            <div class="ui-thumbnail__frame" data-slot="thumbnail-frame">
                <div class="ui-thumbnail__content" data-slot="thumbnail-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}
