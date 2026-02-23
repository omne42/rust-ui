use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    A11yDirection, DateField, DateFieldTone, DateInputGroup, DateInputGroupVariant, Field,
    FieldGroup, FieldGroupDensity, FieldGroupOrientation, SegmentedControl, SegmentedControlSize,
    Switch, SwitchGroup, SwitchGroupOrientation, SwitchGroupTone, TimeField, TimeFieldTone,
};

#[path = "forms_groups/date_input_group.rs"]
mod date_input_group;
#[path = "forms_groups/field_group.rs"]
mod field_group;
#[path = "forms_groups/switch_group.rs"]
mod switch_group;

pub(super) use date_input_group::date_input_group;
pub(super) use field_group::field_group;
pub(super) use switch_group::switch_group;
