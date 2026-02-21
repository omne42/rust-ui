mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_DISABLED, DEFAULT_EMPTY_LABEL, DEFAULT_ID_BASE, DEFAULT_PLACEHOLDER,
};
pub use motion::CommandMotion;
pub use ui_state_primitives::command::{
    CommandFilterState, CommandGroup, CommandItem, FilteredCommandGroup, FilteredCommandItem,
};
pub use view::Command;

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
pub enum CommandRootStateAttr {
    DisabledEmpty,
    Disabled,
    QueryEmpty,
    Empty,
    QueryResults,
    Default,
}

impl CommandRootStateAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandRootStateAttr::DisabledEmpty => "disabled-empty",
            CommandRootStateAttr::Disabled => "disabled",
            CommandRootStateAttr::QueryEmpty => "query-empty",
            CommandRootStateAttr::Empty => "empty",
            CommandRootStateAttr::QueryResults => "query-results",
            CommandRootStateAttr::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandCollectionAttr {
    Empty,
    Populated,
}

impl CommandCollectionAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandCollectionAttr::Empty => "empty",
            CommandCollectionAttr::Populated => "populated",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandQueryAttr {
    Present,
    Absent,
}

impl CommandQueryAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandQueryAttr::Present => "present",
            CommandQueryAttr::Absent => "absent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandDisabledAttr {
    Disabled,
    Enabled,
}

impl CommandDisabledAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandDisabledAttr::Disabled => "disabled",
            CommandDisabledAttr::Enabled => "enabled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandSourceAttr {
    Custom,
    I18n,
    Default,
}

impl CommandSourceAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandSourceAttr::Custom => "custom",
            CommandSourceAttr::I18n => "i18n",
            CommandSourceAttr::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandQueryControlAttr {
    Controlled,
    Uncontrolled,
}

impl CommandQueryControlAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandQueryControlAttr::Controlled => "controlled",
            CommandQueryControlAttr::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandQueryDefaultSourceAttr {
    Provided,
    Empty,
}

impl CommandQueryDefaultSourceAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandQueryDefaultSourceAttr::Provided => "provided",
            CommandQueryDefaultSourceAttr::Empty => "empty",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandQueryChangeSourceAttr {
    Provided,
    None,
}

impl CommandQueryChangeSourceAttr {
    pub fn as_attr(self) -> &'static str {
        match self {
            CommandQueryChangeSourceAttr::Provided => "provided",
            CommandQueryChangeSourceAttr::None => "none",
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
    pub has_i18n_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_i18n_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_i18n_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_on_action: bool,
    pub has_custom_motion: bool,
    pub is_query_controlled: bool,
    pub has_custom_default_query: bool,
    pub has_custom_query_change_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandPartState {
    pub slot: CommandSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: CommandRootStateAttr,
    pub item_attr: CommandCollectionAttr,
    pub group_attr: CommandCollectionAttr,
    pub query_attr: CommandQueryAttr,
    pub disabled_attr: CommandDisabledAttr,
    pub item_count: usize,
    pub group_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub has_query: bool,
    pub has_custom_id_base: bool,
    pub has_custom_placeholder: bool,
    pub has_i18n_placeholder: bool,
    pub has_custom_empty_label: bool,
    pub has_i18n_empty_label: bool,
    pub has_custom_aria_label: bool,
    pub has_i18n_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_disabled: bool,
    pub has_custom_on_action: bool,
    pub has_custom_motion: bool,
    pub is_query_controlled: bool,
    pub has_custom_default_query: bool,
    pub has_custom_query_change_handler: bool,
    pub id_source_attr: CommandSourceAttr,
    pub placeholder_source_attr: CommandSourceAttr,
    pub empty_label_source_attr: CommandSourceAttr,
    pub aria_label_source_attr: CommandSourceAttr,
    pub class_source_attr: CommandSourceAttr,
    pub disabled_source_attr: CommandSourceAttr,
    pub action_source_attr: CommandSourceAttr,
    pub motion_source_attr: CommandSourceAttr,
    pub query_control_attr: CommandQueryControlAttr,
    pub query_default_source_attr: CommandQueryDefaultSourceAttr,
    pub query_change_source_attr: CommandQueryChangeSourceAttr,
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
