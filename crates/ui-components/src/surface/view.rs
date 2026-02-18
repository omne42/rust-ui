use crate::surface::logic::{self, SurfaceElevation, SurfaceTone};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, SurfaceOptions, use_surface};

#[component]
pub fn Surface(
    #[prop(optional)] tone: SurfaceTone,
    #[prop(optional)] elevation: SurfaceElevation,
    #[prop(optional)] is_bordered: Option<bool>,
    #[prop(optional)] bordered: bool,
    #[prop(optional)] is_padded: Option<bool>,
    #[prop(optional, default = true)] padded: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: super::motion::SurfaceMotion,
    children: Children,
) -> impl IntoView {
    let root = logic::normalize_root_state(logic::SurfaceRootInput {
        tone,
        elevation,
        control: logic::SurfaceControlInput {
            is_bordered,
            bordered,
            is_padded,
            padded,
        },
        aria_label,
        class_name,
    });

    let bordered_source_attr = root.bordered_source_attr;
    let padded_source_attr = root.padded_source_attr;
    let state = root.state;
    let class = logic::compose_class_name(root.class_name.clone(), state);
    let semantics = use_surface(SurfaceOptions {
        state,
        aria_label: root.aria_label,
        lang,
        dir,
    });
    let motion = super::motion::sanitize_motion(motion);
    let motion_source = if motion == super::motion::SurfaceMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != super::motion::SurfaceMotion::default()).then_some("true");
    let node_ref: NodeRef<html::Section> = NodeRef::new();
    super::motion::attach_motion(node_ref, motion);

    view! {
        <section
            class=class
            node_ref=node_ref
            data-slot="surface"
            data-tone=semantics.attrs.data_tone
            data-elevation=semantics.attrs.data_elevation
            data-state=semantics.attrs.data_state
            data-bordered=semantics.attrs.data_bordered
            data-padded=semantics.attrs.data_padded
            data-plain=semantics.attrs.data_plain
            data-aria-source=semantics.attrs.data_aria_source
            data-custom-class=semantics.attrs.data_custom_class
            data-class-source=semantics.attrs.data_class_source
            data-bordered-source=bordered_source_attr
            data-padded-source=padded_source_attr
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=semantics.attrs.role
            aria-label=semantics.attrs.aria_label
            lang=semantics.attrs.lang
            dir=semantics.attrs.dir
        >
            {children()}
        </section>
    }
}
