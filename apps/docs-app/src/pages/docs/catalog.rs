use super::{MarkdownPage, Welcome};
use leptos::prelude::*;

const START_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/起点_也即是目的.md"
));
const RULES_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/RULES_ZH.md"
));
const MVP_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/spec/mvp.md"
));
const STYLING_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/spec/styling.md"
));
const MOTION_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/spec/motion.md"
));
const RESEARCH_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/research/README.md"
));
const BB_UI_WEB_NOTES_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/research/bb_ui-web_notes.md"
));
const ANDROID_SPIKE_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/research/android-spike.md"
));

#[derive(Clone, Copy, Debug)]
pub struct DocPage {
    pub title: &'static str,
    pub route: &'static str,
    pub group: &'static str,
    pub page: fn() -> AnyView,
    pub markdown: Option<&'static str>,
}

pub const CATALOG: &[DocPage] = &[
    DocPage {
        title: "Welcome",
        route: "docs/welcome",
        group: "Docs",
        page: welcome,
        markdown: None,
    },
    DocPage {
        title: "起点（也即是目的）",
        route: "docs/start",
        group: "Docs",
        page: start,
        markdown: Some(START_MD),
    },
    DocPage {
        title: "Rules",
        route: "docs/rules",
        group: "Docs",
        page: rules,
        markdown: Some(RULES_MD),
    },
    DocPage {
        title: "MVP spec",
        route: "docs/spec/mvp",
        group: "Spec",
        page: spec_mvp,
        markdown: Some(MVP_MD),
    },
    DocPage {
        title: "Styling spec",
        route: "docs/spec/styling",
        group: "Spec",
        page: spec_styling,
        markdown: Some(STYLING_MD),
    },
    DocPage {
        title: "Motion spec",
        route: "docs/spec/motion",
        group: "Spec",
        page: spec_motion,
        markdown: Some(MOTION_MD),
    },
    DocPage {
        title: "Research README",
        route: "docs/research",
        group: "Research",
        page: research,
        markdown: Some(RESEARCH_MD),
    },
    DocPage {
        title: "bb/ui-web notes",
        route: "docs/research/bb-ui-web-notes",
        group: "Research",
        page: research_bb_ui_web_notes,
        markdown: Some(BB_UI_WEB_NOTES_MD),
    },
    DocPage {
        title: "Android spike",
        route: "docs/research/android-spike",
        group: "Research",
        page: research_android_spike,
        markdown: Some(ANDROID_SPIKE_MD),
    },
];

pub fn docs_catalog() -> &'static [DocPage] {
    CATALOG
}

pub fn doc_page(route: &str) -> Option<AnyView> {
    docs_catalog()
        .iter()
        .find(|entry| entry.route == route)
        .map(|entry| (entry.page)())
}

fn start() -> AnyView {
    view! { <MarkdownPage markdown=START_MD /> }.into_any()
}

fn rules() -> AnyView {
    view! { <MarkdownPage markdown=RULES_MD /> }.into_any()
}

fn spec_mvp() -> AnyView {
    view! { <MarkdownPage markdown=MVP_MD /> }.into_any()
}

fn spec_styling() -> AnyView {
    view! { <MarkdownPage markdown=STYLING_MD /> }.into_any()
}

fn spec_motion() -> AnyView {
    view! { <MarkdownPage markdown=MOTION_MD /> }.into_any()
}

fn research() -> AnyView {
    view! { <MarkdownPage markdown=RESEARCH_MD /> }.into_any()
}

fn research_bb_ui_web_notes() -> AnyView {
    view! { <MarkdownPage markdown=BB_UI_WEB_NOTES_MD /> }.into_any()
}

fn research_android_spike() -> AnyView {
    view! { <MarkdownPage markdown=ANDROID_SPIKE_MD /> }.into_any()
}

fn welcome() -> AnyView {
    Welcome().into_any()
}
