use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    A11yDirection, CheckboxField, CheckboxFieldIndicatorPlacement, CheckboxFieldTone, FormField,
    FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldTone, Legend, LegendTone,
    SegmentedControl, SegmentedControlSize, Switch,
};

const CHECKBOX_FIELD_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::*;";
const LEGEND_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::*;";

#[path = "forms_groups_extra/checkbox_field.rs"]
mod checkbox_field;
#[path = "forms_groups_extra/form_field.rs"]
mod form_field;
#[path = "forms_groups_extra/legend.rs"]
mod legend;

pub(super) use checkbox_field::checkbox_field;
pub(super) use form_field::form_field;
pub(super) use legend::legend;
