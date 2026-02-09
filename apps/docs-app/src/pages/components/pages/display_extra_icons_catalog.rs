use crate::pages::components::ComponentDoc;

pub(super) const ICONSET_DOC: ComponentDoc = ComponentDoc {
    name: "Iconset",
    slug: "iconset",
    group: "Display",
    page: super::display_extra_iconset::iconset,
};

pub(super) const ICONS_UI_DOC: ComponentDoc = ComponentDoc {
    name: "IconsUi",
    slug: "icons-ui",
    group: "Display",
    page: super::display_extra_icons_ui::icons_ui,
};

pub(super) const ICONS_WORKFLOW_DOC: ComponentDoc = ComponentDoc {
    name: "IconsWorkflow",
    slug: "icons-workflow",
    group: "Display",
    page: super::display_extra_icons_workflow::icons_workflow,
};
