mod actions;
mod actions_extra;
mod actions_extra_icon_button;
mod actions_extra_picker_button;
mod collections;
mod collections_breadcrumb;
mod collections_breadcrumb_catalog;
mod collections_breadcrumb_primitives;
mod collections_command;
mod collections_core_catalog;
mod collections_extra;
mod collections_groups;
mod collections_item_catalog;
mod collections_item_primitives;
mod display;
mod display_extra;
mod display_extra_asset;
mod display_extra_empty;
mod display_extra_empty_catalog;
mod display_extra_icons;
mod display_extra_icons_catalog;
mod display_extra_icons_ui;
mod display_extra_icons_workflow;
mod display_extra_iconset;
mod display_extra_swatch;
mod display_extra_thumbnail;
mod files;
mod forms;
mod forms_color;
mod forms_extra;
mod forms_extra_field_label;
mod forms_extra_visually_hidden;
mod forms_groups;
mod forms_groups_extra;
mod forms_native;
mod forms_text_field;
mod layout;
mod layout_extra;
mod layout_extra_direction;
mod overlays;
mod overlays_extra;
mod overlays_extra_coachmark;
mod theme_visual_baseline;
mod ui_root;

use self::{
    actions as a, actions_extra as ax, actions_extra_icon_button as axib,
    actions_extra_picker_button as apb, forms_extra_field_label as fxl, layout_extra as lx,
    overlays as ov,
};
use super::ComponentDoc;

macro_rules! component_doc {
    ($name:literal, $slug:literal, $group:literal, $page:path) => {
        ComponentDoc {
            name: $name,
            slug: $slug,
            group: $group,
            page: $page,
        }
    };
}

pub(super) const CATALOG: &[ComponentDoc] = &[
    component_doc!("Button", "button", "Actions", actions::button),
    component_doc!("ActionBar", "action-bar", "Actions", ax::action_bar),
    component_doc!("ActionButton", "action-button", "Actions", a::action_button),
    component_doc!(
        "FieldButton",
        "field-button",
        "Actions",
        actions_extra::field_button
    ),
    component_doc!(
        "PickerButton",
        "picker-button",
        "Actions",
        apb::picker_button
    ),
    component_doc!(
        "InfieldButton",
        "infield-button",
        "Actions",
        actions_extra::infield_button
    ),
    component_doc!("ClearButton", "clear-button", "Actions", ax::clear_button),
    component_doc!("CloseButton", "close-button", "Actions", ax::close_button),
    component_doc!(
        "LogicButton",
        "logic-button",
        "Actions",
        actions_extra::logic_button
    ),
    component_doc!(
        "ActionButtonGroup",
        "action-button-group",
        "Actions",
        actions::action_button_group
    ),
    component_doc!(
        "ActionGroup",
        "action-group",
        "Actions",
        actions_extra::action_group
    ),
    component_doc!("ActionMenu", "action-menu", "Actions", actions::action_menu),
    component_doc!("ButtonCopy", "button-copy", "Actions", actions::button_copy),
    component_doc!(
        "ButtonGroup",
        "button-group",
        "Actions",
        actions::button_group
    ),
    component_doc!("FlipButton", "flip-button", "Actions", actions::flip_button),
    component_doc!("IconButton", "icon-button", "Actions", axib::icon_button),
    component_doc!("LinkButton", "link-button", "Actions", actions::link_button),
    component_doc!(
        "SearchInputButton",
        "search-input-button",
        "Actions",
        actions::search_input_button
    ),
    component_doc!(
        "ShareButton",
        "share-button",
        "Actions",
        actions::share_button
    ),
    component_doc!(
        "ThemeToggleButton",
        "theme-toggle-button",
        "Actions",
        actions::theme_toggle_button
    ),
    component_doc!(
        "ToggleButton",
        "toggle-button",
        "Actions",
        actions::toggle_button
    ),
    component_doc!("Toggle", "toggle", "Actions", actions_extra::toggle),
    component_doc!(
        "ToggleButtonGroup",
        "toggle-button-group",
        "Actions",
        a::toggle_button_group
    ),
    component_doc!(
        "ToggleGroup",
        "toggle-group",
        "Actions",
        actions_extra::toggle_group
    ),
    component_doc!("Form", "form", "Forms", forms::form),
    component_doc!("Input", "input", "Forms", forms::input),
    component_doc!("Label", "label", "Forms", forms_extra::label),
    component_doc!("FieldLabel", "field-label", "Forms", fxl::field_label),
    component_doc!(
        "VisuallyHidden",
        "visually-hidden",
        "Forms",
        forms_extra_visually_hidden::visually_hidden
    ),
    component_doc!("Legend", "legend", "Forms", forms_groups_extra::legend),
    component_doc!(
        "Description",
        "description",
        "Forms",
        forms_extra::description
    ),
    component_doc!(
        "FieldError",
        "field-error",
        "Forms",
        forms_extra::field_error
    ),
    component_doc!(
        "ErrorMessage",
        "error-message",
        "Forms",
        forms_extra::error_message
    ),
    component_doc!("Field", "field", "Forms", forms_extra::field),
    component_doc!("Fieldset", "fieldset", "Forms", forms_extra::fieldset),
    component_doc!(
        "FieldGroup",
        "field-group",
        "Forms",
        forms_groups::field_group
    ),
    component_doc!("HelpText", "help-text", "Forms", forms_extra::help_text),
    component_doc!("InputGroup", "input-group", "Forms", forms::input_group),
    component_doc!(
        "NativeSelect",
        "native-select",
        "Forms",
        forms_native::native_select
    ),
    component_doc!(
        "DateInputGroup",
        "date-input-group",
        "Forms",
        forms_groups::date_input_group
    ),
    component_doc!(
        "TextField",
        "text-field",
        "Forms",
        forms_text_field::text_field
    ),
    component_doc!("TextArea", "text-area", "Forms", forms::text_area),
    component_doc!("Textarea", "textarea", "Forms", forms_extra::textarea),
    component_doc!("SearchField", "search-field", "Forms", forms::search_field),
    component_doc!(
        "ColorField",
        "color-field",
        "Forms",
        forms_color::color_field
    ),
    component_doc!(
        "ColorEditor",
        "color-editor",
        "Forms",
        forms_color::color_editor
    ),
    component_doc!("ColorArea", "color-area", "Forms", forms_color::color_area),
    component_doc!(
        "ColorSlider",
        "color-slider",
        "Forms",
        forms_color::color_slider
    ),
    component_doc!(
        "ColorWheel",
        "color-wheel",
        "Forms",
        forms_color::color_wheel
    ),
    component_doc!(
        "ColorThumb",
        "color-thumb",
        "Forms",
        forms_color::color_thumb
    ),
    component_doc!(
        "ColorHandle",
        "color-handle",
        "Forms",
        forms_color::color_handle
    ),
    component_doc!(
        "ColorLoupe",
        "color-loupe",
        "Forms",
        forms_color::color_loupe
    ),
    component_doc!(
        "ColorPicker",
        "color-picker",
        "Forms",
        forms_color::color_picker
    ),
    component_doc!("NumberField", "number-field", "Forms", forms::number_field),
    component_doc!("InputOtp", "input-otp", "Forms", forms::input_otp),
    component_doc!("Checkbox", "checkbox", "Forms", forms::checkbox),
    component_doc!(
        "CheckboxField",
        "checkbox-field",
        "Forms",
        forms_groups_extra::checkbox_field
    ),
    component_doc!(
        "FormField",
        "form-field",
        "Forms",
        forms_groups_extra::form_field
    ),
    component_doc!(
        "CheckboxGroup",
        "checkbox-group",
        "Forms",
        forms::checkbox_group
    ),
    component_doc!("Switch", "switch", "Forms", forms::switch),
    component_doc!(
        "SwitchGroup",
        "switch-group",
        "Forms",
        forms_groups::switch_group
    ),
    component_doc!("RadioGroup", "radio-group", "Forms", forms::radio_group),
    component_doc!("Radio", "radio", "Forms", forms::radio),
    component_doc!(
        "SegmentedControl",
        "segmented-control",
        "Forms",
        forms::segmented_control
    ),
    component_doc!("Slider", "slider", "Forms", forms_extra::slider),
    component_doc!("Calendar", "calendar", "Forms", forms_extra::calendar),
    component_doc!("DateField", "date-field", "Forms", forms_extra::date_field),
    component_doc!(
        "DatePicker",
        "date-picker",
        "Forms",
        forms_extra::date_picker
    ),
    component_doc!(
        "DateRangePicker",
        "date-range-picker",
        "Forms",
        forms_extra::date_range_picker
    ),
    component_doc!("TimeField", "time-field", "Forms", forms_extra::time_field),
    component_doc!("Card", "card", "Layout", layout::card),
    component_doc!("View", "view", "Layout", layout::view),
    component_doc!("Flex", "flex", "Layout", layout::flex),
    component_doc!("Grid", "grid", "Layout", layout_extra::grid),
    component_doc!(
        "AspectRatio",
        "aspect-ratio",
        "Layout",
        layout_extra::aspect_ratio
    ),
    component_doc!("Content", "content", "Layout", layout::content),
    component_doc!("Header", "header", "Layout", layout::header),
    component_doc!("Footer", "footer", "Layout", layout::footer),
    component_doc!("Heading", "heading", "Layout", layout::heading),
    component_doc!("Divider", "divider", "Layout", layout::divider),
    component_doc!("Separator", "separator", "Layout", layout::separator),
    component_doc!("Spacer", "spacer", "Layout", layout::spacer),
    component_doc!("Well", "well", "Layout", layout::well),
    component_doc!("Surface", "surface", "Layout", layout_extra::surface),
    layout_extra::SCROLL_AREA_DOC,
    component_doc!("Resizable", "resizable", "Layout", layout_extra::resizable),
    layout_extra_direction::DIRECTION_PROVIDER_DOC,
    component_doc!("Sidebar", "sidebar", "Layout", lx::sidebar),
    component_doc!(
        "SidebarHeader",
        "sidebar-header",
        "Layout",
        lx::sidebar_header
    ),
    component_doc!("SidebarRail", "sidebar-rail", "Layout", lx::sidebar_rail),
    component_doc!(
        "SidebarTrigger",
        "sidebar-trigger",
        "Layout",
        lx::sidebar_trigger
    ),
    component_doc!(
        "SidebarContent",
        "sidebar-content",
        "Layout",
        lx::sidebar_content
    ),
    component_doc!(
        "SidebarFooter",
        "sidebar-footer",
        "Layout",
        lx::sidebar_footer
    ),
    component_doc!("SidebarInset", "sidebar-inset", "Layout", lx::sidebar_inset),
    component_doc!("SidebarGroup", "sidebar-group", "Layout", lx::sidebar_group),
    component_doc!("SidebarMenu", "sidebar-menu", "Layout", lx::sidebar_menu),
    component_doc!(
        "SidebarMenuAction",
        "sidebar-menu-action",
        "Layout",
        lx::sidebar_menu_action
    ),
    component_doc!(
        "SidebarMenuBadge",
        "sidebar-menu-badge",
        "Layout",
        lx::sidebar_menu_badge
    ),
    component_doc!(
        "ScrollShadow",
        "scroll-shadow",
        "Layout",
        layout::scroll_shadow
    ),
    component_doc!("AutoHeight", "auto-height", "Layout", layout::auto_height),
    component_doc!("UiRoot", "ui-root", "Layout", ui_root::ui_root),
    component_doc!(
        "ThemeVisualBaseline",
        "theme-visual-baseline",
        "Layout",
        theme_visual_baseline::theme_visual_baseline
    ),
    component_doc!("Alert", "alert", "Display", display::alert),
    component_doc!(
        "AlertBanner",
        "alert-banner",
        "Display",
        display_extra::alert_banner
    ),
    component_doc!(
        "InlineAlert",
        "inline-alert",
        "Display",
        display::inline_alert
    ),
    component_doc!("Badge", "badge", "Display", display::badge),
    component_doc!(
        "StatusLight",
        "status-light",
        "Display",
        display::status_light
    ),
    component_doc!("Chip", "chip", "Display", display::chip),
    component_doc!("Chart", "chart", "Display", display_extra::chart),
    component_doc!("Skeleton", "skeleton", "Display", display::skeleton),
    component_doc!(
        "SkeletonGroup",
        "skeleton-group",
        "Display",
        display_extra::skeleton_group
    ),
    component_doc!(
        "CircularProgress",
        "circular-progress",
        "Display",
        display::circular_progress
    ),
    component_doc!("Spinner", "spinner", "Display", display::spinner),
    component_doc!("Progress", "progress", "Display", display::progress),
    component_doc!(
        "ProgressBar",
        "progress-bar",
        "Display",
        display::progress_bar
    ),
    component_doc!(
        "ProgressCircle",
        "progress-circle",
        "Display",
        display::progress_circle
    ),
    component_doc!("Meter", "meter", "Display", display::meter),
    component_doc!("Code", "code", "Display", display::code),
    component_doc!("CodeBlock", "code-block", "Display", display::code_block),
    component_doc!("Kbd", "kbd", "Display", display::kbd),
    component_doc!("Keyboard", "keyboard", "Display", display_extra::keyboard),
    component_doc!("Snippet", "snippet", "Display", display::snippet),
    component_doc!("Link", "link", "Display", display::link),
    component_doc!(
        "LabeledValue",
        "labeled-value",
        "Display",
        display_extra::labeled_value
    ),
    component_doc!("Text", "text", "Display", display_extra::text),
    component_doc!("Avatar", "avatar", "Display", display::avatar),
    component_doc!(
        "AvatarGroup",
        "avatar-group",
        "Display",
        display::avatar_group
    ),
    component_doc!("Image", "image", "Display", display::image),
    component_doc!("Asset", "asset", "Display", display_extra_asset::asset),
    component_doc!("FlipCard", "flip-card", "Display", display_extra::flip_card),
    component_doc!(
        "Thumbnail",
        "thumbnail",
        "Display",
        display_extra_thumbnail::thumbnail
    ),
    component_doc!(
        "ColorSwatch",
        "color-swatch",
        "Display",
        display_extra::color_swatch
    ),
    component_doc!("Swatch", "swatch", "Display", display_extra_swatch::swatch),
    component_doc!(
        "ColorSwatchPicker",
        "color-swatch-picker",
        "Display",
        display_extra::color_swatch_picker
    ),
    component_doc!("Icon", "icon", "Display", display_extra::icon),
    display_extra_empty_catalog::EMPTY_DOC,
    display_extra_empty_catalog::EMPTY_HEADER_DOC,
    display_extra_empty_catalog::EMPTY_MEDIA_DOC,
    display_extra_empty_catalog::EMPTY_TITLE_DOC,
    display_extra_empty_catalog::EMPTY_DESCRIPTION_DOC,
    display_extra_empty_catalog::EMPTY_CONTENT_DOC,
    display_extra_icons_catalog::ICONS_DOC,
    display_extra_icons_catalog::ICONSET_DOC,
    display_extra_icons_catalog::ICONS_UI_DOC,
    display_extra_icons_catalog::ICONS_WORKFLOW_DOC,
    component_doc!(
        "IllustratedMessage",
        "illustrated-message",
        "Display",
        display::illustrated_message
    ),
    component_doc!(
        "EmptyState",
        "empty-state",
        "Display",
        display_extra::empty_state
    ),
    component_doc!(
        "ErrorView",
        "error-view",
        "Display",
        display_extra::error_view
    ),
    component_doc!(
        "PressableFeedback",
        "pressable-feedback",
        "Display",
        display_extra::pressable_feedback
    ),
    component_doc!(
        "MotionRipple",
        "motion-ripple",
        "Display",
        display::motion_ripple
    ),
    component_doc!(
        "StaticNumber",
        "static-number",
        "Display",
        display::static_number
    ),
    component_doc!(
        "SlidingNumber",
        "sliding-number",
        "Display",
        display::sliding_number
    ),
    component_doc!("FileTrigger", "file-trigger", "Files", files::file_trigger),
    component_doc!("DropZone", "drop-zone", "Files", files::drop_zone),
    collections_breadcrumb_catalog::BREADCRUMB_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_LIST_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_ITEM_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_LINK_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_PAGE_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_SEPARATOR_DOC,
    collections_breadcrumb_catalog::BREADCRUMB_ELLIPSIS_DOC,
    collections_item_catalog::ITEM_DOC,
    collections_item_catalog::ITEM_GROUP_DOC,
    collections_item_catalog::ITEM_SEPARATOR_DOC,
    collections_item_catalog::ITEM_MEDIA_DOC,
    collections_item_catalog::ITEM_CONTENT_DOC,
    collections_item_catalog::ITEM_TITLE_DOC,
    collections_item_catalog::ITEM_DESCRIPTION_DOC,
    collections_item_catalog::ITEM_ACTIONS_DOC,
    collections_item_catalog::ITEM_HEADER_DOC,
    collections_item_catalog::ITEM_FOOTER_DOC,
    component_doc!(
        "Breadcrumbs",
        "breadcrumbs",
        "Collections",
        collections::breadcrumbs
    ),
    collections_core_catalog::ACCORDION_DOC,
    component_doc!(
        "AccordionItem",
        "accordion-item",
        "Collections",
        collections::accordion
    ),
    collections_core_catalog::AI_SPACE_DOC,
    collections_core_catalog::DISCLOSURE_DOC,
    component_doc!(
        "Collapsible",
        "collapsible",
        "Collections",
        collections_groups::collapsible
    ),
    component_doc!(
        "DisclosureGroup",
        "disclosure-group",
        "Collections",
        collections_extra::disclosure_group
    ),
    collections_core_catalog::TABS_DOC,
    component_doc!("List", "list", "Collections", collections::list),
    component_doc!(
        "ListItem",
        "list-item",
        "Collections",
        collections_extra::list_item
    ),
    component_doc!(
        "ListSection",
        "list-section",
        "Collections",
        collections_extra::list_section
    ),
    component_doc!("Menu", "menu", "Collections", collections::menu),
    component_doc!(
        "MenuItem",
        "menu-item",
        "Collections",
        collections_extra::menu_item
    ),
    component_doc!(
        "MenuSection",
        "menu-section",
        "Collections",
        collections_extra::menu_section
    ),
    component_doc!(
        "MenuTrigger",
        "menu-trigger",
        "Collections",
        collections::menu_trigger
    ),
    component_doc!(
        "Carousel",
        "carousel",
        "Collections",
        collections_command::carousel
    ),
    component_doc!(
        "Menubar",
        "menubar",
        "Collections",
        collections_command::menubar
    ),
    component_doc!(
        "NavigationMenu",
        "navigation-menu",
        "Collections",
        collections_command::navigation_menu
    ),
    component_doc!(
        "Dropdown",
        "dropdown",
        "Collections",
        collections_extra::dropdown
    ),
    component_doc!("Select", "select", "Collections", collections::select),
    component_doc!(
        "ComboBox",
        "combo-box",
        "Collections",
        collections::combo_box
    ),
    component_doc!(
        "Autocomplete",
        "autocomplete",
        "Collections",
        collections::autocomplete
    ),
    component_doc!(
        "Command",
        "command",
        "Collections",
        collections_command::command
    ),
    component_doc!(
        "CommandDialog",
        "command-dialog",
        "Collections",
        collections_command::command_dialog
    ),
    component_doc!(
        "ContextMenu",
        "context-menu",
        "Collections",
        collections_command::context_menu
    ),
    component_doc!(
        "DropdownMenu",
        "dropdown-menu",
        "Collections",
        collections::dropdown_menu
    ),
    component_doc!(
        "Pagination",
        "pagination",
        "Collections",
        collections::pagination
    ),
    component_doc!("Tag", "tag", "Collections", collections_groups::tag),
    component_doc!(
        "TagGroup",
        "tag-group",
        "Collections",
        collections::tag_group
    ),
    component_doc!(
        "StepList",
        "step-list",
        "Collections",
        collections_extra::step_list
    ),
    component_doc!("Table", "table", "Collections", collections_extra::table),
    component_doc!("Tree", "tree", "Collections", collections_extra::tree),
    component_doc!("Overlay", "overlay", "Overlays", overlays::overlay),
    component_doc!("Underlay", "underlay", "Overlays", overlays_extra::underlay),
    component_doc!("Popover", "popover", "Overlays", overlays::popover),
    component_doc!(
        "PreviewCard",
        "preview-card",
        "Overlays",
        overlays::preview_card
    ),
    component_doc!(
        "PreviewLinkCard",
        "preview-link-card",
        "Overlays",
        overlays::preview_link_card
    ),
    component_doc!("Modal", "modal", "Overlays", overlays::modal),
    component_doc!("Dialog", "dialog", "Overlays", overlays::dialog),
    component_doc!(
        "AlertDialog",
        "alert-dialog",
        "Overlays",
        overlays::alert_dialog
    ),
    component_doc!("Sheet", "sheet", "Overlays", overlays::sheet),
    component_doc!(
        "BottomSheet",
        "bottom-sheet",
        "Overlays",
        overlays_extra::bottom_sheet
    ),
    component_doc!("Tray", "tray", "Overlays", overlays_extra::tray),
    component_doc!("Drawer", "drawer", "Overlays", overlays::drawer),
    component_doc!(
        "OverlaysRoot",
        "overlays-root",
        "Overlays",
        overlays::overlays_root
    ),
    component_doc!("Tooltip", "tooltip", "Overlays", overlays::tooltip),
    component_doc!("HoverCard", "hover-card", "Overlays", overlays::hover_card),
    component_doc!(
        "ContextualHelp",
        "contextual-help",
        "Overlays",
        overlays::contextual_help
    ),
    component_doc!(
        "Coachmark",
        "coachmark",
        "Overlays",
        overlays_extra_coachmark::coachmark
    ),
    component_doc!("Toast", "toast", "Overlays", overlays::toast),
    component_doc!("Sonner", "sonner", "Overlays", overlays_extra::sonner),
    component_doc!("Toaster", "toaster", "Overlays", overlays_extra::toaster),
    component_doc!(
        "ToastViewport",
        "toast-viewport",
        "Overlays",
        ov::toast_viewport
    ),
];
