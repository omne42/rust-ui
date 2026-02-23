use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OnPress, OverlayMotion,
    SegmentedControl, SegmentedControlSize, Switch,
};

const DIALOG_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant, Dialog, DialogMotion, DialogSize, OverlayMotion};";

#[path = "overlays_dialog/dialog.rs"]
mod dialog;

pub(super) use dialog::dialog;
