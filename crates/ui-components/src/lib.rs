//! `ui-components` — Leptos components that compose ui-state-primitives + ui-headless + ui-theme.

#[cfg(feature = "component-button")]
macro_rules! wasm_debug_proxy {
    ($feature:literal, $debug:block, $release:block $(,)?) => {{
        #[cfg(all(feature = $feature, debug_assertions, target_arch = "wasm32"))]
        {
            $debug
        }
        #[cfg(not(all(feature = $feature, debug_assertions, target_arch = "wasm32")))]
        {
            $release
        }
    }};
}

#[cfg(feature = "component-button")]
pub(crate) use wasm_debug_proxy;

mod css;

#[cfg(feature = "component-accordion")]
pub use ui_accordion as accordion;
#[cfg(feature = "component-action_bar")]
pub use ui_action_bar as action_bar;
#[cfg(feature = "component-underlay")]
pub use ui_ai_runtime as ai_runtime;
#[cfg(feature = "component-underlay")]
pub use ui_underlay as underlay;
#[cfg(feature = "component-action_menu")]
#[path = "../../../components/menu/src/action_menu/mod.rs"]
pub mod action_menu;
#[cfg(feature = "component-alert")]
pub use ui_alert as alert;
#[cfg(feature = "component-alert_dialog")]
#[path = "../../../components/alert-dialog/src/mod.rs"]
pub mod alert_dialog;
#[cfg(feature = "component-asset")]
pub use ui_asset as asset;
#[cfg(feature = "component-autocomplete")]
pub use ui_autocomplete as autocomplete;
#[cfg(feature = "component-avatar")]
pub use ui_avatar as avatar;
#[cfg(feature = "component-avatar_group")]
pub use ui_avatar_group as avatar_group;
#[cfg(feature = "component-badge")]
pub use ui_badge as badge;
#[cfg(feature = "component-bottom_sheet")]
#[path = "../../../components/bottom-sheet/src/mod.rs"]
pub mod bottom_sheet;
#[cfg(feature = "component-breadcrumb")]
pub use ui_breadcrumb as breadcrumb;
#[cfg(feature = "component-button")]
#[path = "../../../components/button/src/mod.rs"]
pub mod button;
#[cfg(feature = "component-calendar")]
pub use ui_calendar as calendar;
#[cfg(feature = "component-carousel")]
pub use ui_carousel as carousel;
#[cfg(feature = "component-chart")]
pub use ui_chart as chart;
#[cfg(feature = "component-checkbox")]
pub use ui_checkbox as checkbox;
#[cfg(feature = "component-checkbox_field")]
pub use ui_checkbox_field as checkbox_field;
#[cfg(feature = "component-checkbox_group")]
pub use ui_checkbox_group as checkbox_group;
#[cfg(feature = "component-chip")]
pub use ui_chip as chip;
#[cfg(feature = "component-circular_progress")]
pub use ui_circular_progress as circular_progress;
#[cfg(feature = "component-clear_button")]
#[path = "../../../components/button/src/clear_button/mod.rs"]
pub mod clear_button;
#[cfg(feature = "component-close_button")]
#[path = "../../../components/button/src/close_button/mod.rs"]
pub mod close_button;
#[cfg(feature = "component-coachmark")]
#[path = "../../../components/coachmark/src/mod.rs"]
pub mod coachmark;
#[cfg(feature = "component-code")]
pub use ui_code as code;
#[cfg(feature = "component-code_block")]
pub use ui_code_block as code_block;
#[cfg(feature = "component-collapsible")]
pub use ui_collapsible as collapsible;
#[cfg(feature = "component-color_area")]
pub use ui_color_area as color_area;
#[cfg(feature = "component-description")]
pub use ui_description as description;
#[cfg(any(
    feature = "component-color_area",
    feature = "component-color_editor",
    feature = "component-color_field",
    feature = "component-color_handle",
    feature = "component-color_loupe",
    feature = "component-color_picker",
    feature = "component-color_slider",
    feature = "component-color_swatch",
    feature = "component-color_swatch_picker",
    feature = "component-color_thumb",
    feature = "component-color_wheel",
    feature = "component-swatch"
))]
pub mod color {
    #[cfg(feature = "component-color_area")]
    pub use crate::color_area as area;
    #[cfg(feature = "component-color_editor")]
    pub use crate::color_editor as editor;
    #[cfg(feature = "component-color_field")]
    pub use crate::color_field as field;
    #[cfg(feature = "component-color_handle")]
    pub use crate::color_handle as handle;
    #[cfg(feature = "component-color_loupe")]
    pub use crate::color_loupe as loupe;
    #[cfg(feature = "component-color_picker")]
    pub use crate::color_picker as picker;
    #[cfg(feature = "component-color_slider")]
    pub use crate::color_slider as slider;
    #[cfg(feature = "component-color_swatch")]
    pub use crate::color_swatch as swatch;
    #[cfg(feature = "component-color_swatch_picker")]
    pub use crate::color_swatch_picker as swatch_picker;
    #[cfg(feature = "component-color_thumb")]
    pub use crate::color_thumb as thumb;
    #[cfg(feature = "component-color_wheel")]
    pub use crate::color_wheel as wheel;
    #[cfg(feature = "component-swatch")]
    pub use crate::swatch as swatch_core;
}
#[cfg(feature = "component-color_editor")]
#[path = "../../../components/color-editor/src/mod.rs"]
pub mod color_editor;
#[cfg(feature = "component-color_field")]
#[path = "../../../components/color-field/src/mod.rs"]
pub mod color_field;
#[cfg(feature = "component-color_handle")]
#[path = "../../../components/color-handle/src/mod.rs"]
pub mod color_handle;
#[cfg(feature = "component-color_loupe")]
#[path = "../../../components/color-loupe/src/mod.rs"]
pub mod color_loupe;
#[cfg(feature = "component-color_picker")]
#[path = "../../../components/color-picker/src/mod.rs"]
pub mod color_picker;
#[cfg(feature = "component-color_slider")]
pub use ui_color_slider as color_slider;
#[cfg(feature = "component-color_swatch")]
#[path = "../../../components/color-swatch/src/mod.rs"]
pub mod color_swatch;
#[cfg(feature = "component-color_swatch_picker")]
#[path = "../../../components/color-swatch-picker/src/mod.rs"]
pub mod color_swatch_picker;
#[cfg(feature = "component-color_thumb")]
#[path = "../../../components/color-thumb/src/mod.rs"]
pub mod color_thumb;
#[cfg(feature = "component-color_wheel")]
pub use ui_color_wheel as color_wheel;
#[cfg(feature = "component-combo_box")]
pub use ui_combo_box as combo_box;
#[cfg(feature = "component-command")]
pub use ui_command as command;
#[cfg(feature = "component-command_dialog")]
#[path = "../../../components/command-dialog/src/mod.rs"]
pub mod command_dialog;
#[cfg(feature = "component-context_menu")]
#[path = "../../../components/menu/src/context_menu/mod.rs"]
pub mod context_menu;
#[cfg(feature = "component-contextual_help")]
#[path = "../../../components/contextual-help/src/mod.rs"]
pub mod contextual_help;
#[cfg(feature = "component-dialog")]
#[path = "../../../components/dialog/src/mod.rs"]
pub mod dialog;
#[cfg(feature = "component-direction")]
pub use ui_direction as direction;
#[cfg(feature = "component-disclosure")]
pub use ui_disclosure as disclosure;
#[cfg(feature = "component-drawer")]
pub use ui_drawer as drawer;
#[cfg(feature = "component-drop_zone")]
pub use ui_drop_zone as drop_zone;
#[cfg(feature = "component-dropdown")]
#[path = "../../../components/menu/src/dropdown/mod.rs"]
pub mod dropdown;
#[cfg(feature = "component-dropdown_menu")]
#[path = "../../../components/menu/src/dropdown_menu/mod.rs"]
pub mod dropdown_menu;
#[cfg(feature = "component-empty")]
pub use ui_empty as empty;
#[cfg(feature = "component-empty_state")]
pub use ui_empty_state as empty_state;
#[cfg(feature = "component-error_message")]
pub use ui_error_message as error_message;
#[cfg(feature = "component-error_view")]
pub use ui_error_view as error_view;
#[cfg(feature = "component-example_theme")]
pub use ui_example_theme as example_theme;
#[cfg(feature = "component-form")]
#[path = "../../../components/form/src/mod.rs"]
mod field_form_form;
#[cfg(feature = "component-form_field")]
#[path = "../../../components/form-field/src/mod.rs"]
mod field_form_form_field;
#[cfg(any(
    feature = "component-description",
    feature = "component-field",
    feature = "component-field_error",
    feature = "component-field_label",
    feature = "component-fieldset",
    feature = "component-form",
    feature = "component-form_field",
    feature = "component-help_text"
))]
pub mod field_form {
    #[cfg(feature = "component-description")]
    pub use crate::description;
    #[cfg(feature = "component-field")]
    pub use ui_field as field;
    #[cfg(feature = "component-field_error")]
    pub use ui_field_error as field_error;
    #[cfg(feature = "component-field_label")]
    pub use ui_field_label as field_label;
    #[cfg(feature = "component-fieldset")]
    pub use ui_fieldset as fieldset;
    #[cfg(feature = "component-form")]
    pub mod form {
        pub use crate::field_form_form::*;
    }
    #[cfg(feature = "component-form_field")]
    pub mod form_field {
        pub use crate::field_form_form_field::*;
    }
    #[cfg(feature = "component-help_text")]
    pub use ui_help_text as help_text;
}
#[cfg(feature = "component-file_trigger")]
#[path = "../../../components/file-trigger/src/mod.rs"]
pub mod file_trigger;
#[cfg(feature = "component-flip_card")]
pub use ui_flip_card as flip_card;
#[cfg(feature = "component-hover_card")]
pub use ui_hover_card as hover_card;
#[cfg(feature = "component-icon")]
pub use ui_icon as icon;
#[cfg(feature = "component-icons")]
pub use ui_icon::icons;
#[cfg(feature = "component-icons_ui")]
pub use ui_icon::icons_ui;
#[cfg(feature = "component-icons_workflow")]
pub use ui_icon::icons_workflow;
#[cfg(feature = "component-iconset")]
pub use ui_icon::iconset;
#[cfg(feature = "component-illustrated_message")]
pub use ui_illustrated_message as illustrated_message;
#[cfg(feature = "component-image")]
pub use ui_image as image;
#[cfg(feature = "component-infield_button")]
#[path = "../../../components/button/src/infield_button/mod.rs"]
pub mod infield_button;
#[cfg(feature = "component-item")]
pub use ui_item as item;
#[cfg(feature = "component-kbd")]
pub use ui_kbd as kbd;
#[cfg(feature = "component-keyboard")]
pub use ui_keyboard as keyboard;
#[cfg(feature = "component-label")]
#[path = "../../../components/label/src/mod.rs"]
pub mod label;
#[cfg(feature = "component-labeled_value")]
pub use ui_labeled_value as labeled_value;
#[cfg(feature = "component-legend")]
pub use ui_legend as legend;
#[cfg(feature = "component-link")]
pub use ui_link as link;
#[cfg(feature = "component-link_button")]
#[path = "../../../components/button/src/link_button/mod.rs"]
pub mod link_button;
#[cfg(feature = "component-list")]
pub use ui_list as list;
#[cfg(feature = "component-logic_button")]
#[path = "../../../components/button/src/logic_button/mod.rs"]
pub mod logic_button;
#[cfg(feature = "component-menu")]
#[path = "../../../components/menu/src/mod.rs"]
pub mod menu;
#[cfg(feature = "component-menu_trigger")]
#[path = "../../../components/menu/src/trigger/mod.rs"]
pub mod menu_trigger;
#[cfg(feature = "component-menubar")]
#[path = "../../../components/menu/src/menubar/mod.rs"]
pub mod menubar;
#[cfg(feature = "component-meter")]
pub use ui_meter as meter;
#[cfg(feature = "component-modal")]
#[path = "../../../components/modal/src/mod.rs"]
pub mod modal;
#[cfg(feature = "component-native_select")]
pub use ui_native_select as native_select;
#[cfg(feature = "component-navigation_menu")]
#[path = "../../../components/menu/src/navigation_menu/mod.rs"]
pub mod navigation_menu;
#[cfg(feature = "component-overlay")]
#[path = "../../../components/overlay/src/mod.rs"]
pub mod overlay;
#[cfg(feature = "component-overlays")]
#[path = "../../../components/overlays/src/mod.rs"]
pub mod overlays;
#[cfg(feature = "component-pagination")]
pub use ui_pagination as pagination;
#[cfg(feature = "component-picker_button")]
#[path = "../../../components/button/src/picker_button/mod.rs"]
pub mod picker_button;
#[cfg(feature = "component-popover")]
pub use ui_popover as popover;
#[cfg(feature = "component-pressable_feedback")]
pub use ui_pressable_feedback as pressable_feedback;
#[cfg(feature = "component-preview_card")]
pub use ui_preview_card as preview_card;
#[cfg(feature = "component-preview_link_card")]
#[path = "../../../components/preview-link-card/src/mod.rs"]
pub mod preview_link_card;
#[cfg(feature = "component-progress")]
pub use ui_progress as progress;
#[cfg(feature = "component-progress_bar")]
pub use ui_progress::bar as progress_bar;
#[cfg(feature = "component-progress_circle")]
pub use ui_progress::circle as progress_circle;
#[cfg(feature = "component-radio")]
pub use ui_radio as radio;
#[cfg(feature = "component-ripple")]
pub use ui_ripple as ripple;
pub mod root;
#[cfg(feature = "component-search_field")]
#[path = "../../../components/text-input/src/search_field/mod.rs"]
pub mod search_field;
#[cfg(feature = "component-segmented_control")]
pub use ui_segmented_control as segmented_control;
#[cfg(feature = "component-select")]
#[path = "../../../components/select/src/mod.rs"]
pub mod select;
#[cfg(feature = "component-sheet")]
pub use ui_sheet as sheet;
#[cfg(feature = "component-sidebar")]
#[path = "../../../components/sidebar/src/mod.rs"]
pub mod sidebar;
#[cfg(feature = "component-sidebar_content")]
#[path = "../../../components/sidebar/src/content/mod.rs"]
pub mod sidebar_content;
#[cfg(feature = "component-sidebar_footer")]
#[path = "../../../components/sidebar/src/footer/mod.rs"]
pub mod sidebar_footer;
#[cfg(feature = "component-sidebar_header")]
#[path = "../../../components/sidebar/src/header/mod.rs"]
pub mod sidebar_header;
#[cfg(feature = "component-sidebar_inset")]
#[path = "../../../components/sidebar/src/inset/mod.rs"]
pub mod sidebar_inset;
#[cfg(feature = "component-sidebar_menu")]
#[path = "../../../components/sidebar/src/menu/mod.rs"]
pub mod sidebar_menu;
#[cfg(feature = "component-sidebar_menu_action")]
#[path = "../../../components/sidebar/src/menu_action/mod.rs"]
pub mod sidebar_menu_action;
#[cfg(feature = "component-sidebar_menu_badge")]
#[path = "../../../components/sidebar/src/menu_badge/mod.rs"]
pub mod sidebar_menu_badge;
#[cfg(feature = "component-sidebar_rail")]
#[path = "../../../components/sidebar/src/rail/mod.rs"]
pub mod sidebar_rail;
#[cfg(feature = "component-sidebar_trigger")]
#[path = "../../../components/sidebar/src/trigger/mod.rs"]
pub mod sidebar_trigger;
#[cfg(feature = "component-skeleton")]
pub use ui_skeleton as skeleton;
#[cfg(feature = "component-slider")]
pub use ui_slider as slider;
#[cfg(feature = "component-snippet")]
pub use ui_snippet as snippet;
#[cfg(feature = "component-spinner")]
pub use ui_spinner as spinner;
#[cfg(feature = "component-status_light")]
pub use ui_status_light as status_light;
#[cfg(feature = "component-step_list")]
pub use ui_step_list as step_list;
#[cfg(feature = "component-sonner")]
pub use ui_toast::sonner;
#[cfg(feature = "component-swatch")]
#[path = "../../../components/swatch/src/mod.rs"]
pub mod swatch;
#[cfg(feature = "component-switch")]
#[path = "../../../components/switch/src/mod.rs"]
pub mod switch;
#[cfg(feature = "component-table")]
pub use ui_table as table;
#[cfg(feature = "component-tabs")]
pub use ui_tabs as tabs;
#[cfg(feature = "component-tag")]
pub use ui_tag as tag;
#[cfg(feature = "component-text")]
pub use ui_text as text;
#[cfg(feature = "component-text_field")]
#[path = "../../../components/text-input/src/text_field/mod.rs"]
pub mod text_field;
#[cfg(any(
    feature = "component-date_field",
    feature = "component-date_input_group",
    feature = "component-date_picker",
    feature = "component-date_range_picker",
    feature = "component-input",
    feature = "component-input_otp",
    feature = "component-number",
    feature = "component-number_field",
    feature = "component-search_field",
    feature = "component-text",
    feature = "component-text_area",
    feature = "component-text_field",
    feature = "component-textarea",
    feature = "component-time_field"
))]
#[path = "../../../components/text-input/src/mod.rs"]
pub mod text_input;
#[cfg(feature = "component-textarea")]
#[path = "../../../components/text-input/src/textarea/mod.rs"]
pub mod textarea;
#[cfg(feature = "component-time_field")]
#[path = "../../../components/text-input/src/time_field/mod.rs"]
pub mod time_field;
#[cfg(feature = "component-number")]
pub use text_input::number;
#[cfg(feature = "component-theme_dark")]
#[path = "../../../components/theme-dark/src/mod.rs"]
pub mod theme_dark;
#[cfg(feature = "component-theme_default")]
#[path = "../../../components/theme-default/src/mod.rs"]
pub mod theme_default;
#[cfg(feature = "component-theme_express")]
#[path = "../../../components/theme-express/src/mod.rs"]
pub mod theme_express;
#[cfg(feature = "component-theme_light")]
#[path = "../../../components/theme-light/src/mod.rs"]
pub mod theme_light;
#[cfg(feature = "component-accordion")]
pub use accordion::{
    AccordionStreamingItem, AccordionStreamingProjection, project_streaming_accordion_markup,
};
#[cfg(feature = "component-underlay")]
pub use ai_runtime::{AiOutputStatus, AiRenderMode, AiSpace, AiSpaceState, use_ai_space_state};
#[cfg(feature = "component-color_picker")]
pub use color_picker::{ColorPicker, ColorPickerMotion};
pub use root::UiRoot;
#[cfg(feature = "component-swatch")]
pub use swatch::{Swatch, SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize};
#[cfg(feature = "component-number")]
pub use text_input::number::{
    NumberFormatOptions, SlidingNumber, SlidingNumberMotion, StaticNumber,
};
#[cfg(feature = "component-textarea")]
pub use textarea::Textarea;
pub use ui_headless::{MenuItemKind, OnPress};
pub use ui_theme::Theme;
pub use ui_theme::{SemanticOverrides, SemanticVariable};
#[cfg(feature = "component-thumbnail")]
pub use ui_thumbnail as thumbnail;
#[cfg(feature = "component-toast")]
pub use ui_toast::toast;
#[cfg(feature = "component-toaster")]
pub use ui_toast::toaster;
#[cfg(feature = "component-tooltip")]
pub use ui_tooltip as tooltip;
#[cfg(feature = "component-tray")]
pub use ui_tray as tray;
#[cfg(feature = "component-tree")]
pub use ui_tree as tree;
#[cfg(feature = "component-visually_hidden")]
pub use ui_visually_hidden as visually_hidden;

#[cfg(feature = "inject-css")]
// Intentionally exposed as a thin wrapper below to keep the public API stable while
// allowing internal CSS aggregation to evolve.
#[cfg(all(feature = "web-demo-components", not(feature = "all-components")))]
mod web_demo_components {
    use crate::*;
    pub use accordion::{
        Accordion, AccordionItem, AccordionMotion, AccordionSelectionMode, AccordionVariant,
        open_set,
    };
    pub use action_menu::{ActionMenu, ActionMenuItemSpec, ActionMenuMotion};
    pub use alert::{Alert, AlertFill, AlertLayout, AlertMotion, AlertTone, AlertVariant};
    pub use alert_dialog::{
        AlertDialog, AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogVariant,
    };
    pub use autocomplete::{Autocomplete, AutocompleteMotion};
    pub use avatar::{Avatar, AvatarSize};
    pub use avatar_group::{AvatarGroup, AvatarGroupItem};
    pub use badge::{Badge, BadgeVariant};
    pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
    pub use button::action::{
        ActionButton, ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize,
        ActionButtonType,
    };
    pub use button::action::{
        ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupMotion,
        ActionButtonGroupOrientation,
    };
    pub use button::copy::ButtonCopyMode;
    pub use button::copy::{ButtonCopy, ButtonCopyMotion, ButtonCopyStrings};
    pub use button::flip::{FlipButton, FlipButtonMotion, FlipDirection};
    pub use button::search_input::{SearchInputButton, SearchInputButtonMotion};
    pub use button::share::{
        ShareButton, ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,
    };
    pub use button::theme_toggle::{ThemeMode, ThemeToggleButton, ThemeToggleMotion};
    pub use button::toggle_button::{
        ToggleButton, ToggleButtonMotion, ToggleButtonSize, ToggleButtonVariant,
    };
    pub use button::{
        Button, ButtonColor, ButtonLoadingPlacement, ButtonMotion, ButtonRadius, ButtonSize,
        ButtonVariant,
    };
    pub use button::{ButtonGroup, ButtonGroupOrientation};
    pub use button::{ToggleButtonGroup, ToggleButtonGroupOrientation};
    pub use checkbox::{Checkbox, CheckboxSize, CheckboxVariant};
    pub use checkbox_group::CheckboxGroup;
    pub use chip::{Chip, ChipSize, ChipVariant};
    pub use circular_progress::CircularProgress;
    pub use code::{Code, CodeVariant};
    pub use code_block::{CodeBlock, CodeBlockMotion};
    pub use combo_box::{ComboBox, ComboBoxMotion};
    pub use contextual_help::{ContextualHelp, ContextualHelpMotion, ContextualHelpVariant};
    pub use dialog::{Dialog, DialogMotion, DialogSize};
    pub use disclosure::{Disclosure, DisclosureMotion};
    pub use drawer::{Drawer, DrawerMotion, DrawerPlacement};
    pub use drop_zone::{DropZone, DropZoneMotion, DroppedFile};
    pub use dropdown_menu::{DropdownMenu, DropdownMenuMotion};
    pub use field_form::form::{Form, FormLabelAlign, FormLabelPosition, use_form_context};
    pub use file_trigger::{FileTrigger, FileTriggerFile, FileTriggerMotion};
    pub use hover_card::{HoverCard, HoverCardMotion};
    pub use illustrated_message::{
        IllustratedMessage, IllustratedMessageMotion, IllustratedMessageOrientation,
    };
    pub use image::{Image, ImageMotion, ImageRadius, ImageShadow};
    pub use kbd::{Kbd, KbdSize};
    pub use link::Link;
    pub use link_button::LinkButton;
    pub use list::List;
    pub use menu::{Menu, MenuItemSpec};
    pub use menu_trigger::{MenuTrigger, MenuTriggerMotion};
    pub use meter::{Meter, MeterMotion, MeterSize, MeterVariant};
    pub use modal::Modal;
    pub use overlay::{Overlay, OverlayMotion};
    pub use pagination::Pagination;
    pub use popover::{Popover, PopoverMotion};
    pub use progress::{Progress, ProgressMotion, ProgressRange};
    pub use progress_bar::{ProgressBar, ProgressBarSize, ProgressBarVariant};
    pub use progress_circle::{ProgressCircle, ProgressCircleMotion};
    pub use radio::{Radio, RadioGroup, RadioGroupOrientation, RadioMotion};
    pub use ripple::{MotionRipple, RippleMotion};
    pub use segmented_control::{
        SegmentedControl, SegmentedControlMotion, SegmentedControlOrientation, SegmentedControlSize,
    };
    pub use select::{Select, SelectMotion};
    pub use sheet::{Sheet, SheetMotion, SheetPlacement};
    pub use skeleton::{Skeleton, SkeletonVariant};
    pub use snippet::Snippet;
    pub use spinner::{Spinner, SpinnerSize};
    pub use status_light::{StatusLight, StatusLightRole, StatusLightVariant};
    pub use switch::{Switch, SwitchMotion};
    pub use tabs::{Tabs, TabsKeyboardActivation, TabsMotion};
    pub use tag::group::{Tag, TagGroup};
    pub use tag::{TagSize, TagVariant};
    pub use text_input::input::{Input, InputLabelPlacement, InputMotion, InputSize, InputVariant};
    pub use text_input::input_otp::InputOtp;
    pub use text_input::number_field::NumberField;
    pub use text_input::search_field::{SearchField, SearchFieldMotion};
    pub use text_input::text_area::TextArea;
    pub use text_input::text_field::TextField;
    pub use toast::{
        Toast, ToastMotion, ToastOptions, ToastStoreOptions, ToastVariant, ToastViewport,
        provide_toast_store,
    };
    pub use tooltip::{Tooltip, TooltipMotion};
    pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion;
}

#[cfg(feature = "inject-css")]
// Intentionally exposed as a thin wrapper below to keep the public API stable while
// allowing internal CSS aggregation to evolve.
#[cfg(feature = "all-components")]
mod all_components {
    use crate::*;
    pub use accordion::{
        Accordion, AccordionItem, AccordionMotion, AccordionSelectionMode, AccordionVariant,
        open_set,
    };
    pub use action_bar::{ActionBar, ActionBarMotion, ActionBarPosition};
    pub use action_menu::{ActionMenu, ActionMenuItemSpec, ActionMenuMotion};
    pub use alert::{Alert, AlertFill, AlertLayout, AlertMotion, AlertTone, AlertVariant};
    pub use alert_dialog::{
        AlertDialog, AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogVariant,
    };
    pub use asset::{Asset, AssetMotion, AssetSize, AssetVariant};
    pub use autocomplete::{Autocomplete, AutocompleteMotion};
    pub use avatar::{Avatar, AvatarSize};
    pub use avatar_group::{AvatarGroup, AvatarGroupItem};
    pub use badge::Badge;
    pub use badge::BadgeVariant;
    pub use bottom_sheet::{BottomSheet, BottomSheetMotion};
    pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
    pub use button::Button;
    pub use button::action::{
        ActionButton, ActionButtonLoadingPlacement, ActionButtonMotion, ActionButtonSize,
        ActionButtonType,
    };
    pub use button::action::{
        ActionButtonGroup, ActionButtonGroupDensity, ActionButtonGroupOrientation,
    };
    pub use button::action::{
        ActionGroup, ActionGroupItem, ActionGroupSelectionMode, ActionGroupTone,
    };
    pub use button::copy::ButtonCopyMode;
    pub use button::copy::{ButtonCopy, ButtonCopyMotion, ButtonCopyStrings};
    pub use button::field::FieldButton;
    pub use button::flip::{FlipButton, FlipButtonMotion, FlipDirection};
    pub use button::search_input::{SearchInputButton, SearchInputButtonMotion};
    pub use button::share::{
        ShareButton, ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,
    };
    pub use button::theme_toggle::{ThemeMode, ThemeToggleButton, ThemeToggleMotion};
    pub use button::toggle::{Toggle, ToggleMotion, ToggleSize, ToggleVariant};
    pub use button::toggle_button::{
        ToggleButton, ToggleButtonMotion, ToggleButtonSize, ToggleButtonVariant,
    };
    pub use button::{
        ButtonA11y, ButtonAction, ButtonColor, ButtonIntent, ButtonLoadingPlacement, ButtonMotion,
        ButtonRadius, ButtonSchema, ButtonSize, ButtonSpec, ButtonText, ButtonVariant,
    };
    pub use button::{ButtonGroup, ButtonGroupOrientation};
    pub use button::{ToggleButtonGroup, ToggleButtonGroupOrientation};
    pub use button::{
        ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupSelectionMode,
    };
    pub use calendar::{
        Calendar, CalendarFirstWeekday, CalendarGridCell, CalendarMotion, CalendarTone,
    };
    pub use carousel::{
        Carousel, CarouselItem, CarouselMotion, CarouselOrientation, CarouselStrings,
    };
    pub use chart::{Chart, ChartKind, ChartMotion, ChartPoint};
    pub use checkbox::{Checkbox, CheckboxMotion, CheckboxSize, CheckboxVariant};
    pub use checkbox_field::{CheckboxField, CheckboxFieldIndicatorPlacement, CheckboxFieldTone};
    pub use checkbox_group::CheckboxGroup;
    pub use chip::{Chip, ChipSize, ChipVariant};
    pub use circular_progress::CircularProgress;
    pub use clear_button::{ClearButton, ClearButtonFocusMode, ClearButtonVariant};
    pub use close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};
    pub use coachmark::{Coachmark, CoachmarkAssetVariant, CoachmarkMotion, CoachmarkVariant};
    pub use code::{Code, CodeVariant};
    pub use code_block::{CodeBlock, CodeBlockMotion};
    pub use collapsible::{Collapsible, CollapsibleMotion};
    pub use color::area::ColorArea;
    pub use color::editor::{ColorEditor, ColorEditorFormat};
    pub use color::field::ColorField;
    pub use color::handle::ColorHandle;
    pub use color::loupe::ColorLoupe;
    pub use color::slider::{ColorSlider, ColorSliderChannel, ColorSliderMotion};
    pub use color::swatch::{
        ColorSwatch, ColorSwatchAlpha, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize,
    };
    pub use color::swatch_picker::{ColorSwatchPicker, ColorSwatchPickerItem};
    pub use color::thumb::ColorThumb;
    pub use color::wheel::{ColorWheel, ColorWheelMotion};
    pub use combo_box::{ComboBox, ComboBoxMotion};
    pub use command::{Command, CommandGroup, CommandItem, CommandMotion};
    pub use command_dialog::CommandDialog;
    pub use context_menu::{ContextMenu, ContextMenuMotion};
    pub use contextual_help::{ContextualHelp, ContextualHelpMotion, ContextualHelpVariant};
    pub use description::{Description, DescriptionElement, DescriptionTone};
    pub use dialog::{Dialog, DialogMotion, DialogSize};
    pub use direction::{DirectionMode, DirectionProvider};
    pub use disclosure::Disclosure;
    pub use disclosure::DisclosureMotion;
    pub use disclosure::group::{DisclosureGroup, DisclosureGroupSelectionMode};
    pub use drawer::{Drawer, DrawerMotion, DrawerPlacement};
    pub use drop_zone::{DropZone, DropZoneMotion, DroppedFile};
    pub use dropdown::{Dropdown, DropdownMotion};
    pub use dropdown_menu::{DropdownMenu, DropdownMenuMotion};
    pub use empty::{
        Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant,
        EmptyTitle,
    };
    pub use empty_state::{EmptyState, EmptyStateAlign, EmptyStateTone};
    pub use error_message::{
        ErrorMessage, ErrorMessageElement, ErrorMessageMotion, ErrorMessageTone,
    };
    pub use error_view::{ErrorView, ErrorViewMotion, ErrorViewTone};
    pub use field_form::field::group::{FieldGroup, FieldGroupDensity, FieldGroupOrientation};
    pub use field_form::field::{Field, FieldOrientation, FieldTone};
    pub use field_form::field_error::{FieldError, FieldErrorTone};
    pub use field_form::field_label::{FieldLabel, FieldLabelTone};
    pub use field_form::fieldset::{Fieldset, FieldsetOrientation, FieldsetTone};
    pub use field_form::form::{Form, FormLabelAlign, FormLabelPosition, use_form_context};
    pub use field_form::form_field::{
        FormField, FormFieldIndicatorPlacement, FormFieldIndicatorVariant, FormFieldTone,
    };
    pub use field_form::help_text::{HelpText, HelpTextTone};
    pub use file_trigger::{FileTrigger, FileTriggerFile, FileTriggerMotion};
    pub use flip_card::{FlipCard, FlipCardMotion};
    pub use hover_card::{HoverCard, HoverCardMotion};
    pub use icon::{Icon, IconSize, IconTone};
    pub use icons::{Icons, IconsGlyph, IconsScale, IconsSet, IconsTone};
    pub use icons_ui::{IconsUi, IconsUiSize, IconsUiTone};
    pub use icons_workflow::{IconsWorkflow, IconsWorkflowSize, IconsWorkflowTone};
    pub use iconset::{Iconset, IconsetGlyph, IconsetSize, IconsetTone};
    pub use illustrated_message::{
        IllustratedMessage, IllustratedMessageMotion, IllustratedMessageOrientation,
    };
    pub use image::{Image, ImageMotion, ImageRadius, ImageShadow};
    pub use infield_button::InfieldButton;
    pub use item::{
        Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader,
        ItemMedia, ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
    };
    pub use kbd::{Kbd, KbdSize};
    pub use keyboard::{Keyboard, KeyboardTone};
    pub use label::{Label, LabelEmphasis, LabelMotion};
    pub use labeled_value::{LabeledValue, LabeledValueOrientation, LabeledValueTone};
    pub use legend::{Legend, LegendTone};
    pub use link::Link;
    pub use link_button::LinkButton;
    pub use list::{
        List, ListItem, ListItemSelectionIndicator, ListMotion, ListSection,
        ListSectionHeadingTone, ListSectionMotion,
    };
    pub use logic_button::{LogicButton, LogicButtonVariant};
    pub use menu::item::MenuItem;
    pub use menu::section::{MenuSection, MenuSectionHeadingTone};
    pub use menu::{Menu, MenuItemSpec};
    pub use menu_trigger::{MenuTrigger, MenuTriggerMotion};
    pub use menubar::{Menubar, MenubarMenu, MenubarMotion};
    pub use meter::{Meter, MeterMotion, MeterSize, MeterVariant};
    pub use modal::Modal;
    pub use native_select::{NativeSelect, NativeSelectOption, NativeSelectSize};
    pub use navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuMotion};
    pub use overlay::Overlay;
    pub use overlay::OverlayMotion;
    pub use pagination::Pagination;
    pub use picker_button::PickerButton;
    pub use popover::Popover;
    pub use popover::PopoverMotion;
    pub use pressable_feedback::{
        PressableFeedback, PressableFeedbackEffect, PressableFeedbackMotion, PressableFeedbackTone,
    };
    pub use preview_card::{PreviewCard, PreviewCardMotion};
    pub use preview_link_card::{PreviewLinkCard, PreviewLinkCardMotion};
    pub use progress::{Progress, ProgressMotion, ProgressRange};
    pub use progress_bar::{ProgressBar, ProgressBarSize, ProgressBarVariant};
    pub use progress_circle::{ProgressCircle, ProgressCircleMotion};
    pub use radio::{Radio, RadioGroup, RadioGroupOrientation, RadioMotion};
    pub use ripple::{MotionRipple, RippleMotion};
    pub use segmented_control::{
        SegmentedControl, SegmentedControlMotion, SegmentedControlOrientation, SegmentedControlSize,
    };
    pub use select::{Select, SelectMotion};
    pub use sheet::{Sheet, SheetMotion, SheetPlacement};
    pub use sidebar::group::SidebarGroup;
    pub use sidebar::{Sidebar, SidebarCollapsible, SidebarSide, SidebarVariant};
    pub use sidebar_content::SidebarContent;
    pub use sidebar_footer::SidebarFooter;
    pub use sidebar_header::SidebarHeader;
    pub use sidebar_inset::SidebarInset;
    pub use sidebar_menu::{SidebarMenu, SidebarMenuItem, SidebarMenuMotion, SidebarMenuSubItem};
    pub use sidebar_menu_action::SidebarMenuAction;
    pub use sidebar_menu_action::SidebarMenuActionMotion;
    pub use sidebar_menu_badge::SidebarMenuBadge;
    pub use sidebar_rail::SidebarRail;
    pub use sidebar_trigger::SidebarTrigger;
    pub use skeleton::group::{
        SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant,
    };
    pub use skeleton::{Skeleton, SkeletonVariant};
    pub use slider::{Slider, SliderMotion};
    pub use snippet::Snippet;
    pub use sonner::{Sonner, SonnerPosition};
    pub use spinner::{Spinner, SpinnerSize};
    pub use status_light::{StatusLight, StatusLightRole, StatusLightVariant};
    pub use step_list::{StepList, StepListItem, StepListOrientation, StepListSize};
    pub use switch::Switch;
    pub use switch::SwitchMotion;
    pub use switch::group::{SwitchGroup, SwitchGroupOrientation, SwitchGroupTone};
    pub use table::{
        Table, TableCellAlign, TableColumn, TableDensity, TableLayout, TableRow, TableVariant,
    };
    pub use tabs::{Tabs, TabsKeyboardActivation, TabsMotion};
    pub use tag::group::{Tag, TagGroup};
    pub use tag::{TagSize, TagVariant};
    pub use text::{Text, TextAlign, TextElement, TextTone, TextWeight};
    pub use text_input::date_field::{DateField, DateFieldIds, DateFieldTone};
    pub use text_input::date_input_group::{DateInputGroup, DateInputGroupVariant};
    pub use text_input::date_picker::{
        DatePicker, DatePickerIds, DatePickerMotion, DatePickerTone,
    };
    pub use text_input::date_range_picker::{DateRangePicker, DateRangePickerTone};
    pub use text_input::input::group::InputGroup;
    pub use text_input::input::{Input, InputLabelPlacement, InputMotion, InputSize, InputVariant};
    pub use text_input::input_otp::InputOtp;
    pub use text_input::number_field::NumberField;
    pub use text_input::search_field::{SearchField, SearchFieldMotion};
    pub use text_input::text_area::TextArea;
    pub use text_input::text_field::TextField;
    pub use text_input::time_field::{TimeField, TimeFieldIds, TimeFieldTone};
    pub use thumbnail::{Thumbnail, ThumbnailMotion, ThumbnailSize};
    pub use toast::{
        Toast, ToastMotion, ToastOptions, ToastStoreOptions, ToastVariant, ToastViewport,
        provide_toast_store,
    };
    pub use toaster::{Toaster, ToasterPosition};
    pub use tooltip::Tooltip;
    pub use tooltip::TooltipMotion;
    pub use tray::{Tray, TrayMotion};
    pub use tree::{Tree, TreeDensity, TreeMotion, TreeNode, TreeTone};
    pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion;
    pub use underlay::{Underlay, UnderlayMotion};
    pub use visually_hidden::VisuallyHidden;
}
#[cfg(all(feature = "web-demo-components", not(feature = "all-components")))]
pub use web_demo_components::*;

#[cfg(feature = "all-components")]
pub use all_components::*;

#[cfg(feature = "inject-css")]
#[doc(hidden)]
pub fn push_components_css(out: &mut String) {
    css::push_components_css(out);
}
