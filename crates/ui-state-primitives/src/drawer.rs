pub const DEFAULT_ID_BASE: &str = "ui-drawer";
pub const DEFAULT_TITLE: &str = "Drawer";
pub const DEFAULT_OPEN: bool = false;
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerOpenMode {
    Controlled,
    Uncontrolled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerOpenConfigInput {
    pub has_open: bool,
    pub default_open: Option<bool>,
    pub has_on_open_change: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerOpenConfig {
    pub mode: DrawerOpenMode,
    pub default_open: bool,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
}

pub fn resolve_open_config(input: DrawerOpenConfigInput) -> DrawerOpenConfig {
    let mode = if input.has_open {
        DrawerOpenMode::Controlled
    } else {
        DrawerOpenMode::Uncontrolled
    };

    DrawerOpenConfig {
        mode,
        default_open: input.default_open.unwrap_or(DEFAULT_OPEN),
        has_default_open: input.default_open.is_some(),
        has_open_change_handler: input.has_on_open_change,
    }
}

pub fn can_request_open_change(mode: DrawerOpenMode, has_open_change_handler: bool) -> bool {
    mode == DrawerOpenMode::Uncontrolled || has_open_change_handler
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerVisibility {
    Visible,
    Hidden,
}

impl DrawerVisibility {
    pub fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

pub fn resolve_close_button_visibility(is_close_button_visible: Option<bool>) -> DrawerVisibility {
    if is_close_button_visible.unwrap_or(DEFAULT_SHOW_CLOSE_BUTTON) {
        DrawerVisibility::Visible
    } else {
        DrawerVisibility::Hidden
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DrawerPlacement {
    Bottom,
    Left,
    #[default]
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerSlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl DrawerSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            DrawerSlot::Root => "drawer",
            DrawerSlot::Header => "drawer-header",
            DrawerSlot::Title => "drawer-title",
            DrawerSlot::Description => "drawer-description",
            DrawerSlot::Body => "drawer-body",
            DrawerSlot::Footer => "drawer-footer",
            DrawerSlot::Close => "drawer-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            DrawerSlot::Root => "ui-drawer",
            DrawerSlot::Header => "ui-drawer__header",
            DrawerSlot::Title => "ui-drawer__title",
            DrawerSlot::Description => "ui-drawer__description",
            DrawerSlot::Body => "ui-drawer__body",
            DrawerSlot::Footer => "ui-drawer__footer",
            DrawerSlot::Close => "ui-drawer__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartStateInput {
    pub slot: DrawerSlot,
    pub placement: DrawerPlacement,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawerPartState {
    pub slot: DrawerSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub placement_attr: &'static str,
    pub placement_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub placement_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub fn state_attr(has_description: bool) -> &'static str {
    if has_description {
        "with-description"
    } else {
        "title-only"
    }
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn footer_attr(has_footer: bool) -> &'static str {
    if has_footer { "present" } else { "absent" }
}

pub fn close_button_attr(show_close_button: bool) -> &'static str {
    if show_close_button { "shown" } else { "hidden" }
}

pub fn placement_class(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "ui-drawer--placement-bottom",
        DrawerPlacement::Left => "ui-drawer--placement-left",
        DrawerPlacement::Right => "ui-drawer--placement-right",
    }
}

pub fn placement_attr(placement: DrawerPlacement) -> &'static str {
    match placement {
        DrawerPlacement::Bottom => "bottom",
        DrawerPlacement::Left => "left",
        DrawerPlacement::Right => "right",
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        trimmed.into()
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: DrawerPartStateInput) -> DrawerPartState {
    let has_custom_placement = input.placement != DrawerPlacement::default();

    DrawerPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        placement_attr: placement_attr(input.placement),
        placement_class: placement_class(input.placement),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        placement_source_attr: source_attr(has_custom_placement),
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

#[cfg(test)]
#[path = "test/drawer.rs"]
mod tests;
