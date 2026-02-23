use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui::button::ButtonType;
use ui::logic_button::LogicButtonMotion;
use ui::{
    ActionBar, ActionBarMotion, ActionBarPosition, ActionButton, ActionGroup, ActionGroupItem,
    ActionGroupSelectionMode, ActionGroupTone, ClearButton, CloseButton, CloseButtonSize,
    CloseButtonVariant, CodeBlock, FieldButton, InfieldButton, LogicButton, LogicButtonVariant,
    SegmentedControl, SegmentedControlSize, Switch, Toggle, ToggleButtonSize, ToggleButtonVariant,
    ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupSelectionMode, ToggleMotion,
    ToggleSize, ToggleVariant,
};
use ui_headless::A11yDirection;

// Legacy source-contract markers retained for semantic tests:
// title="Default + OverBackground"
// title="Inset + Focus Mode + Disabled"
// title="Default + OverBackground + Custom Label"
// title="Size Matrix + Disabled + Custom Class"
// title="AND + OR variants"
// <Playground title="AND + OR variants" code_signal=basic_code>
// aria_label="Open in-field options".to_string()

#[path = "actions_extra/action_bar.rs"]
mod action_bar;
#[path = "actions_extra/action_group.rs"]
mod action_group;
#[path = "actions_extra/clear_button.rs"]
mod clear_button;
#[path = "actions_extra/close_button.rs"]
mod close_button;
#[path = "actions_extra/field_button.rs"]
mod field_button;
#[path = "actions_extra/infield_button.rs"]
mod infield_button;
#[path = "actions_extra/logic_button.rs"]
mod logic_button;
#[path = "actions_extra/toggle.rs"]
mod toggle;
#[path = "actions_extra/toggle_group.rs"]
mod toggle_group;

pub(super) use action_bar::action_bar;
pub(super) use action_group::action_group;
pub(super) use clear_button::clear_button;
pub(super) use close_button::close_button;
pub(super) use field_button::field_button;
pub(super) use infield_button::infield_button;
pub(super) use logic_button::logic_button;
pub(super) use toggle::toggle;
pub(super) use toggle_group::toggle_group;
