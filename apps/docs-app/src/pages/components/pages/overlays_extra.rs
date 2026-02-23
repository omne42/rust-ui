use super::playground_workbench::{bool_word, push_line_when, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    BottomSheet, BottomSheetMotion, Button, ButtonVariant, OnPress, SegmentedControl,
    SegmentedControlSize, Snippet, Sonner, SonnerPosition, Switch, ToastMotion, ToastOptions,
    ToastStoreOptions, ToastVariant, Toaster, ToasterPosition, Tray, TrayMotion, Underlay,
    provide_toast_store,
};

const BOTTOM_SHEET_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{BottomSheet, BottomSheetMotion, Button, OnPress, SegmentedControl, SegmentedControlSize};";

#[path = "overlays_extra/bottom_sheet.rs"]
mod bottom_sheet;
#[path = "overlays_extra/sonner.rs"]
mod sonner;
#[path = "overlays_extra/toaster.rs"]
mod toaster;
#[path = "overlays_extra/tray.rs"]
mod tray;
#[path = "overlays_extra/underlay.rs"]
mod underlay;

pub(super) use bottom_sheet::bottom_sheet;
pub(super) use sonner::sonner;
pub(super) use toaster::toaster;
pub(super) use tray::tray;
pub(super) use underlay::underlay;
