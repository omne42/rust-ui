use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::overlays::OverlaysRoot;
use ui::{
    Button, ButtonVariant, ContextualHelp, ContextualHelpVariant, Drawer, DrawerMotion,
    DrawerPlacement, Modal, OnPress, Overlay, OverlayMotion, Popover, PopoverMotion, PreviewCard,
    PreviewCardMotion, PreviewLinkCard, PreviewLinkCardMotion, SegmentedControl,
    SegmentedControlSize, Sheet, SheetMotion, SheetPlacement, Snippet, Switch, Toast, ToastMotion,
    ToastOptions, ToastStoreOptions, ToastVariant, ToastViewport, provide_toast_store,
};

#[path = "overlays_dialog.rs"]
mod overlays_dialog;

#[path = "overlays_alert_dialog.rs"]
mod overlays_alert_dialog;

#[path = "overlays_hover_card.rs"]
mod overlays_hover_card;

#[path = "overlays_tooltip.rs"]
mod overlays_tooltip;

const MODAL_MINIMAL_PLAYGROUND_CODE: &str = r#"<Modal default_open=true id_base="m".to_string() title="Hello".to_string() on_close=Callback::new(|_| {})>
  <div>"Minimal modal content"</div>
</Modal>"#;

const MODAL_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Modal, OnPress, OverlayMotion};";
const DRAWER_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant, Drawer, DrawerMotion, DrawerPlacement, OnPress, SheetMotion};";

// Legacy source-contract markers retained for overlays semantic suites:
// title="Overlay presence"
// <Button on_press=open_custom_modal>
// ui = { workspace = true, default-features = false, features = ["component-modal", "inject-css"] }
// ui = { workspace = true, default-features = false, features = ["component-drawer", "inject-css"] }

#[path = "overlays/alert_dialog.rs"]
mod alert_dialog;
#[path = "overlays/contextual_help.rs"]
mod contextual_help;
#[path = "overlays/dialog.rs"]
mod dialog;
#[path = "overlays/drawer.rs"]
mod drawer;
#[path = "overlays/hover_card.rs"]
mod hover_card;
#[path = "overlays/modal.rs"]
mod modal;
#[path = "overlays/overlay.rs"]
mod overlay;
#[path = "overlays/overlays_root.rs"]
mod overlays_root;
#[path = "overlays/popover.rs"]
mod popover;
#[path = "overlays/preview_card.rs"]
mod preview_card;
#[path = "overlays/preview_link_card.rs"]
mod preview_link_card;
#[path = "overlays/sheet.rs"]
mod sheet;
#[path = "overlays/toast.rs"]
mod toast;
#[path = "overlays/toast_viewport.rs"]
mod toast_viewport;
#[path = "overlays/tooltip.rs"]
mod tooltip;

pub(super) use alert_dialog::alert_dialog;
pub(super) use contextual_help::contextual_help;
pub(super) use dialog::dialog;
pub(super) use drawer::drawer;
pub(super) use hover_card::hover_card;
pub(super) use modal::modal;
pub(super) use overlay::overlay;
pub(super) use overlays_root::overlays_root;
pub(super) use popover::popover;
pub(super) use preview_card::preview_card;
pub(super) use preview_link_card::preview_link_card;
pub(super) use sheet::sheet;
pub(super) use toast::toast;
pub(super) use toast_viewport::toast_viewport;
pub(super) use tooltip::tooltip;
