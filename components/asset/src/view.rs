use super::{AssetMotion, AssetSize, AssetVariant, logic, motion, protocol};
use leptos::prelude::*;
use ui_thumbnail::Thumbnail;

const ICON_VIEW_BOX: &str = "0 0 24 24";
const ICON_STROKE_WIDTH: &str = "1.5";
const ICON_STROKE_LINE_JOIN: &str = "round";

const FILE_PATH_BODY: &str = "M6 2h8l4 4v16H6z";
const FILE_PATH_FOLD: &str = "M14 2v4h4";

const FOLDER_PATH_BODY: &str = "M3 6.5h6l2 2h10v9A2.5 2.5 0 0 1 18.5 20h-13A2.5 2.5 0 0 1 3 17.5z";
const FOLDER_PATH_DIVIDER: &str = "M3 9h18";

const CUSTOM_FALLBACK_PATH_SCENE: &str =
    "M8 14l2.8-2.8a1 1 0 0 1 1.4 0L14 13l2-2a1 1 0 0 1 1.4 0L20 13.6";

fn render_two_path_icon(
    label: StoredValue<String>,
    class_name: &'static str,
    slot: &'static str,
    variant: &'static str,
    first_path: &'static str,
    second_path: &'static str,
) -> AnyView {
    view! {
        <svg
            viewBox=ICON_VIEW_BOX
            role="img"
            aria-label=label.get_value()
            class=class_name
            data-slot=slot
            data-variant=variant
        >
            <path
                d=first_path
                fill="none"
                stroke="currentColor"
                stroke-width=ICON_STROKE_WIDTH
                stroke-linejoin=ICON_STROKE_LINE_JOIN
            ></path>
            <path
                d=second_path
                fill="none"
                stroke="currentColor"
                stroke-width=ICON_STROKE_WIDTH
                stroke-linejoin=ICON_STROKE_LINE_JOIN
            ></path>
        </svg>
    }
    .into_any()
}

fn render_file_icon(label: StoredValue<String>) -> AnyView {
    render_two_path_icon(
        label,
        "ui-asset__icon ui-asset__icon--file",
        "asset-file",
        "file",
        FILE_PATH_BODY,
        FILE_PATH_FOLD,
    )
}

fn render_folder_icon(label: StoredValue<String>) -> AnyView {
    render_two_path_icon(
        label,
        "ui-asset__icon ui-asset__icon--folder",
        "asset-folder",
        "folder",
        FOLDER_PATH_BODY,
        FOLDER_PATH_DIVIDER,
    )
}

fn render_custom_fallback_icon(label: StoredValue<String>) -> AnyView {
    view! {
        <svg
            viewBox=ICON_VIEW_BOX
            role="img"
            aria-label=label.get_value()
            class="ui-asset__icon ui-asset__icon--custom-fallback"
            data-slot="asset-custom"
            data-variant="custom"
        >
            <rect
                x="4"
                y="4"
                width="16"
                height="16"
                fill="none"
                stroke="currentColor"
                stroke-width=ICON_STROKE_WIDTH
                stroke-linejoin=ICON_STROKE_LINE_JOIN
            ></rect>
            <path
                d=CUSTOM_FALLBACK_PATH_SCENE
                fill="none"
                stroke="currentColor"
                stroke-width=ICON_STROKE_WIDTH
                stroke-linejoin=ICON_STROKE_LINE_JOIN
            ></path>
            <circle cx="9" cy="9" r="1.2" fill="currentColor"></circle>
        </svg>
    }
    .into_any()
}

fn render_custom_content(label: StoredValue<String>, children: Option<Children>) -> AnyView {
    if let Some(children) = children {
        view! { {children()} }.into_any()
    } else {
        render_custom_fallback_icon(label)
    }
}

fn resolve_icon_content(
    variant: AssetVariant,
    label: StoredValue<String>,
    children: Option<Children>,
) -> AnyView {
    match variant {
        AssetVariant::File => render_file_icon(label),
        AssetVariant::Folder => render_folder_icon(label),
        AssetVariant::Custom => render_custom_content(label, children),
    }
}

#[component]
pub fn Asset(
    #[prop(optional)] variant: AssetVariant,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] dir: Option<String>,
    #[prop(optional)] size: AssetSize,
    #[prop(optional)] is_selected: bool,
    #[prop(optional)] is_focused: bool,
    #[prop(optional)] motion: AssetMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let motion = motion::sanitize_motion(motion);
    let is_custom_motion = motion != AssetMotion::default();
    let motion_source_attr =
        protocol::AssetMotionSource::from_is_custom(is_custom_motion).as_attr();
    let resolved = logic::resolve_view_state(logic::AssetResolvedInput {
        variant,
        size,
        is_selected,
        is_focused,
        label,
        class_name,
        has_children: children.is_some(),
    });
    let state = resolved.state;
    let class_name = resolved.class_name;
    let label = StoredValue::new(resolved.label);
    let content = resolve_icon_content(variant, label, children);

    view! {
        <Thumbnail
            size=size
            selected=is_selected
            focused=is_focused
            motion=motion
            class_name=class_name
        >
            <div
                class="ui-asset__content"
                lang=lang
                dir=dir
                data-ui-schema=protocol::ASSET_AGENT_SCHEMA
                data-ui-intent=protocol::AssetAgentIntent::Display.as_attr()
                data-ui-action=protocol::AssetAgentAction::StaticRender.as_attr()
                data-ui-state=state.data_state_attr
                data-ui-selection-source=protocol::AssetInteractionSource::ExternalProp.as_attr()
                data-ui-focus-source=protocol::AssetInteractionSource::ExternalProp.as_attr()
                data-ui-label-source=state.label_source_attr
                data-ui-class-source=state.class_source_attr
                data-ui-content-source=state.content_source_attr
                data-ui-motion-source=motion_source_attr
                data-ui-stream-support=protocol::AssetStreamSupport::Optional.as_attr()
                data-ui-stream-fallback=protocol::AssetStreamFallback::Snapshot.as_attr()
                data-ui-output-status=protocol::AssetOutputStatus::Verified.as_attr()
                data-slot="asset"
                data-variant=state.variant_attr
                data-size=state.size_attr
                data-state=state.data_state_attr
                data-selected=if state.selected { "true" } else { "false" }
                data-focused=if state.focused { "true" } else { "false" }
                data-selection-source="external-prop"
                data-focus-source="external-prop"
                data-label-source=state.label_source_attr
                data-class-source=state.class_source_attr
                data-content-source=state.content_source_attr
                data-custom-class=if state.has_custom_class_name {
                    "true"
                } else {
                    "false"
                }
                data-motion-source=motion_source_attr
                data-custom-motion=if motion != AssetMotion::default() {
                    "true"
                } else {
                    "false"
                }
            >
                {content}
            </div>
        </Thumbnail>
    }
}
