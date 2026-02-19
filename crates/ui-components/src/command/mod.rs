mod logic;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_DISABLED, DEFAULT_EMPTY_LABEL, DEFAULT_ID_BASE, DEFAULT_PLACEHOLDER,
};
pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion as CommandMotion;
pub use view::Command;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandItem {
    pub id: String,
    pub label: String,
    pub keywords: Vec<String>,
    pub shortcut: Option<String>,
    pub disabled: bool,
}

impl CommandItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            keywords: Vec::new(),
            shortcut: None,
            disabled: false,
        }
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGroup {
    pub heading: String,
    pub items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(heading: impl Into<String>, items: Vec<CommandItem>) -> Self {
        Self {
            heading: heading.into(),
            items,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilteredCommandItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilteredCommandGroup {
    pub heading: String,
    pub item_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommandFilterState {
    pub items: Vec<FilteredCommandItem>,
    pub groups: Vec<FilteredCommandGroup>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandSlot {
    Root,
    InputWrap,
    Input,
    List,
    Options,
    Group,
    GroupHeading,
    GroupItems,
    Item,
    ItemLabel,
    Shortcut,
    Empty,
    Highlight,
}

impl CommandSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandSlot::Root => "command",
            CommandSlot::InputWrap => "command-input-wrap",
            CommandSlot::Input => "command-input",
            CommandSlot::List => "command-list",
            CommandSlot::Options => "command-options",
            CommandSlot::Group => "command-group",
            CommandSlot::GroupHeading => "command-group-heading",
            CommandSlot::GroupItems => "command-group-items",
            CommandSlot::Item => "command-item",
            CommandSlot::ItemLabel => "command-item-label",
            CommandSlot::Shortcut => "command-shortcut",
            CommandSlot::Empty => "command-empty",
            CommandSlot::Highlight => "command-highlight",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            CommandSlot::Root => "ui-command",
            CommandSlot::InputWrap => "ui-command__input-wrap",
            CommandSlot::Input => "ui-command__input",
            CommandSlot::List => "ui-command__list",
            CommandSlot::Options => "ui-command__options",
            CommandSlot::Group => "ui-command__group",
            CommandSlot::GroupHeading => "ui-command__group-heading",
            CommandSlot::GroupItems => "ui-command__group-items",
            CommandSlot::Item => "ui-command__option",
            CommandSlot::ItemLabel => "ui-command__item-label",
            CommandSlot::Shortcut => "ui-command__shortcut",
            CommandSlot::Empty => "ui-command__empty",
            CommandSlot::Highlight => "ui-active-highlight",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandPartStateInput {
    pub slot: CommandSlot,
    pub item_count: usize,
    pub group_count: usize,
    pub is_disabled: bool,
    pub has_query: bool,
    pub has_custom_id_base: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_on_action: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandPartState {
    pub slot: CommandSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub group_attr: &'static str,
    pub query_attr: &'static str,
    pub disabled_attr: &'static str,
    pub item_count: usize,
    pub group_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_query: bool,
    pub has_custom_id_base: bool,
    pub has_custom_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_on_action: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub placeholder_source_attr: &'static str,
    pub empty_label_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub disabled_source_attr: &'static str,
    pub action_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
