mod actions;
mod actions_extra;
mod actions_extra_icon_button;
mod actions_extra_picker_button;
mod collections;
mod collections_breadcrumb;
mod collections_breadcrumb_catalog;
mod collections_breadcrumb_shadcn;
mod collections_command;
mod collections_core_catalog;
mod collections_extra;
mod collections_extra_combobox;
mod collections_extra_picker;
mod collections_extra_tags;
mod collections_extra_top_nav;
mod collections_groups;
mod collections_item_catalog;
mod collections_item_shadcn;
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
mod files_extra_dropzone;
mod forms;
mod forms_color;
mod forms_extra;
mod forms_extra_field_label;
mod forms_extra_search;
mod forms_extra_textfield;
mod forms_extra_visually_hidden;
mod forms_groups;
mod forms_groups_extra;
mod forms_native;
mod layout;
mod layout_extra;
mod layout_extra_direction;
mod layout_extra_sidenav;
mod layout_extra_split_view;
mod overlays;
mod overlays_extra;
mod overlays_extra_coachmark;

use self::{
    actions as a, actions_extra as ax, actions_extra_icon_button as axib,
    actions_extra_picker_button as apb, collections_extra_combobox as cxb,
    collections_extra_picker as cxp, collections_extra_tags as cxt,
    collections_extra_top_nav as cxtn, files_extra_dropzone as fdz, forms_extra_field_label as fxl,
    forms_extra_search as fxs, forms_extra_textfield as fxt, layout_extra as lx,
    layout_extra_sidenav as lxs, layout_extra_split_view as lxv, overlays as ov,
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
    ComponentDoc {
        name: "LogicButton",
        slug: "logic-button",
        group: "Actions",
        page: actions_extra::logic_button,
    },
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
    ComponentDoc {
        name: "SearchInputButton",
        slug: "search-input-button",
        group: "Actions",
        page: actions::search_input_button,
    },
    ComponentDoc {
        name: "ShareButton",
        slug: "share-button",
        group: "Actions",
        page: actions::share_button,
    },
    ComponentDoc {
        name: "ThemeToggleButton",
        slug: "theme-toggle-button",
        group: "Actions",
        page: actions::theme_toggle_button,
    },
    ComponentDoc {
        name: "ToggleButton",
        slug: "toggle-button",
        group: "Actions",
        page: actions::toggle_button,
    },
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
    ComponentDoc {
        name: "Legend",
        slug: "legend",
        group: "Forms",
        page: forms_groups_extra::legend,
    },
    ComponentDoc {
        name: "Description",
        slug: "description",
        group: "Forms",
        page: forms_extra::description,
    },
    ComponentDoc {
        name: "FieldError",
        slug: "field-error",
        group: "Forms",
        page: forms_extra::field_error,
    },
    ComponentDoc {
        name: "ErrorMessage",
        slug: "error-message",
        group: "Forms",
        page: forms_extra::error_message,
    },
    ComponentDoc {
        name: "Field",
        slug: "field",
        group: "Forms",
        page: forms_extra::field,
    },
    ComponentDoc {
        name: "Fieldset",
        slug: "fieldset",
        group: "Forms",
        page: forms_extra::fieldset,
    },
    ComponentDoc {
        name: "FieldGroup",
        slug: "field-group",
        group: "Forms",
        page: forms_groups::field_group,
    },
    ComponentDoc {
        name: "HelpText",
        slug: "help-text",
        group: "Forms",
        page: forms_extra::help_text,
    },
    ComponentDoc {
        name: "InputGroup",
        slug: "input-group",
        group: "Forms",
        page: forms::input_group,
    },
    ComponentDoc {
        name: "NativeSelect",
        slug: "native-select",
        group: "Forms",
        page: forms_native::native_select,
    },
    ComponentDoc {
        name: "DateInputGroup",
        slug: "date-input-group",
        group: "Forms",
        page: forms_groups::date_input_group,
    },
    ComponentDoc {
        name: "TextField",
        slug: "text-field",
        group: "Forms",
        page: forms::text_field,
    },
    component_doc!("Textfield", "textfield", "Forms", fxt::textfield),
    ComponentDoc {
        name: "TextArea",
        slug: "text-area",
        group: "Forms",
        page: forms::text_area,
    },
    ComponentDoc {
        name: "Textarea",
        slug: "textarea",
        group: "Forms",
        page: forms_extra::textarea,
    },
    ComponentDoc {
        name: "SearchField",
        slug: "search-field",
        group: "Forms",
        page: forms::search_field,
    },
    component_doc!("Search", "search", "Forms", fxs::search),
    ComponentDoc {
        name: "ColorField",
        slug: "color-field",
        group: "Forms",
        page: forms_color::color_field,
    },
    ComponentDoc {
        name: "ColorEditor",
        slug: "color-editor",
        group: "Forms",
        page: forms_color::color_editor,
    },
    ComponentDoc {
        name: "ColorArea",
        slug: "color-area",
        group: "Forms",
        page: forms_color::color_area,
    },
    ComponentDoc {
        name: "ColorSlider",
        slug: "color-slider",
        group: "Forms",
        page: forms_color::color_slider,
    },
    ComponentDoc {
        name: "ColorWheel",
        slug: "color-wheel",
        group: "Forms",
        page: forms_color::color_wheel,
    },
    ComponentDoc {
        name: "ColorThumb",
        slug: "color-thumb",
        group: "Forms",
        page: forms_color::color_thumb,
    },
    ComponentDoc {
        name: "ColorHandle",
        slug: "color-handle",
        group: "Forms",
        page: forms_color::color_handle,
    },
    ComponentDoc {
        name: "ColorLoupe",
        slug: "color-loupe",
        group: "Forms",
        page: forms_color::color_loupe,
    },
    ComponentDoc {
        name: "ColorPicker",
        slug: "color-picker",
        group: "Forms",
        page: forms_color::color_picker,
    },
    ComponentDoc {
        name: "NumberField",
        slug: "number-field",
        group: "Forms",
        page: forms::number_field,
    },
    ComponentDoc {
        name: "InputOtp",
        slug: "input-otp",
        group: "Forms",
        page: forms::input_otp,
    },
    ComponentDoc {
        name: "Checkbox",
        slug: "checkbox",
        group: "Forms",
        page: forms::checkbox,
    },
    ComponentDoc {
        name: "CheckboxField",
        slug: "checkbox-field",
        group: "Forms",
        page: forms_groups_extra::checkbox_field,
    },
    ComponentDoc {
        name: "FormField",
        slug: "form-field",
        group: "Forms",
        page: forms_groups_extra::form_field,
    },
    ComponentDoc {
        name: "CheckboxGroup",
        slug: "checkbox-group",
        group: "Forms",
        page: forms::checkbox_group,
    },
    ComponentDoc {
        name: "Switch",
        slug: "switch",
        group: "Forms",
        page: forms::switch,
    },
    ComponentDoc {
        name: "SwitchGroup",
        slug: "switch-group",
        group: "Forms",
        page: forms_groups::switch_group,
    },
    ComponentDoc {
        name: "RadioGroup",
        slug: "radio-group",
        group: "Forms",
        page: forms::radio_group,
    },
    ComponentDoc {
        name: "Radio",
        slug: "radio",
        group: "Forms",
        page: forms::radio,
    },
    ComponentDoc {
        name: "SegmentedControl",
        slug: "segmented-control",
        group: "Forms",
        page: forms::segmented_control,
    },
    ComponentDoc {
        name: "Slider",
        slug: "slider",
        group: "Forms",
        page: forms_extra::slider,
    },
    ComponentDoc {
        name: "Calendar",
        slug: "calendar",
        group: "Forms",
        page: forms_extra::calendar,
    },
    ComponentDoc {
        name: "DateField",
        slug: "date-field",
        group: "Forms",
        page: forms_extra::date_field,
    },
    ComponentDoc {
        name: "DatePicker",
        slug: "date-picker",
        group: "Forms",
        page: forms_extra::date_picker,
    },
    ComponentDoc {
        name: "DateRangePicker",
        slug: "date-range-picker",
        group: "Forms",
        page: forms_extra::date_range_picker,
    },
    ComponentDoc {
        name: "TimeField",
        slug: "time-field",
        group: "Forms",
        page: forms_extra::time_field,
    },
    ComponentDoc {
        name: "Card",
        slug: "card",
        group: "Layout",
        page: layout::card,
    },
    ComponentDoc {
        name: "View",
        slug: "view",
        group: "Layout",
        page: layout::view,
    },
    ComponentDoc {
        name: "Flex",
        slug: "flex",
        group: "Layout",
        page: layout::flex,
    },
    ComponentDoc {
        name: "Grid",
        slug: "grid",
        group: "Layout",
        page: layout_extra::grid,
    },
    ComponentDoc {
        name: "AspectRatio",
        slug: "aspect-ratio",
        group: "Layout",
        page: layout_extra::aspect_ratio,
    },
    ComponentDoc {
        name: "Content",
        slug: "content",
        group: "Layout",
        page: layout::content,
    },
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
    component_doc!("SplitView", "split-view", "Layout", lxv::split_view),
    layout_extra_direction::DIRECTION_PROVIDER_DOC,
    component_doc!("Sidebar", "sidebar", "Layout", lx::sidebar),
    component_doc!("Sidenav", "sidenav", "Layout", lxs::sidenav),
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
    component_doc!("UiRoot", "ui-root", "Layout", layout::ui_root),
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
    ComponentDoc {
        name: "Chip",
        slug: "chip",
        group: "Display",
        page: display::chip,
    },
    component_doc!("Chart", "chart", "Display", display_extra::chart),
    ComponentDoc {
        name: "Skeleton",
        slug: "skeleton",
        group: "Display",
        page: display::skeleton,
    },
    ComponentDoc {
        name: "SkeletonGroup",
        slug: "skeleton-group",
        group: "Display",
        page: display_extra::skeleton_group,
    },
    ComponentDoc {
        name: "CircularProgress",
        slug: "circular-progress",
        group: "Display",
        page: display::circular_progress,
    },
    ComponentDoc {
        name: "Spinner",
        slug: "spinner",
        group: "Display",
        page: display::spinner,
    },
    ComponentDoc {
        name: "Progress",
        slug: "progress",
        group: "Display",
        page: display::progress,
    },
    ComponentDoc {
        name: "ProgressBar",
        slug: "progress-bar",
        group: "Display",
        page: display::progress_bar,
    },
    ComponentDoc {
        name: "ProgressCircle",
        slug: "progress-circle",
        group: "Display",
        page: display::progress_circle,
    },
    ComponentDoc {
        name: "Meter",
        slug: "meter",
        group: "Display",
        page: display::meter,
    },
    ComponentDoc {
        name: "Code",
        slug: "code",
        group: "Display",
        page: display::code,
    },
    ComponentDoc {
        name: "CodeBlock",
        slug: "code-block",
        group: "Display",
        page: display::code_block,
    },
    ComponentDoc {
        name: "Kbd",
        slug: "kbd",
        group: "Display",
        page: display::kbd,
    },
    ComponentDoc {
        name: "Keyboard",
        slug: "keyboard",
        group: "Display",
        page: display_extra::keyboard,
    },
    ComponentDoc {
        name: "Snippet",
        slug: "snippet",
        group: "Display",
        page: display::snippet,
    },
    ComponentDoc {
        name: "Link",
        slug: "link",
        group: "Display",
        page: display::link,
    },
    ComponentDoc {
        name: "LabeledValue",
        slug: "labeled-value",
        group: "Display",
        page: display_extra::labeled_value,
    },
    ComponentDoc {
        name: "Text",
        slug: "text",
        group: "Display",
        page: display_extra::text,
    },
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
    ComponentDoc {
        name: "FileTrigger",
        slug: "file-trigger",
        group: "Files",
        page: files::file_trigger,
    },
    ComponentDoc {
        name: "DropZone",
        slug: "drop-zone",
        group: "Files",
        page: files::drop_zone,
    },
    component_doc!("Dropzone", "dropzone", "Files", fdz::dropzone),
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
    collections_core_catalog::DISCLOSURE_DOC,
    ComponentDoc {
        name: "Collapsible",
        slug: "collapsible",
        group: "Collections",
        page: collections_groups::collapsible,
    },
    ComponentDoc {
        name: "DisclosureGroup",
        slug: "disclosure-group",
        group: "Collections",
        page: collections_extra::disclosure_group,
    },
    collections_core_catalog::TABS_DOC,
    ComponentDoc {
        name: "ListBox",
        slug: "listbox",
        group: "Collections",
        page: collections::list_box,
    },
    ComponentDoc {
        name: "ListBoxItem",
        slug: "listbox-item",
        group: "Collections",
        page: collections_extra::listbox_item,
    },
    ComponentDoc {
        name: "ListBoxSection",
        slug: "listbox-section",
        group: "Collections",
        page: collections_extra::listbox_section,
    },
    ComponentDoc {
        name: "Menu",
        slug: "menu",
        group: "Collections",
        page: collections::menu,
    },
    ComponentDoc {
        name: "MenuItem",
        slug: "menu-item",
        group: "Collections",
        page: collections_extra::menu_item,
    },
    ComponentDoc {
        name: "MenuSection",
        slug: "menu-section",
        group: "Collections",
        page: collections_extra::menu_section,
    },
    ComponentDoc {
        name: "MenuTrigger",
        slug: "menu-trigger",
        group: "Collections",
        page: collections::menu_trigger,
    },
    ComponentDoc {
        name: "Carousel",
        slug: "carousel",
        group: "Collections",
        page: collections_command::carousel,
    },
    ComponentDoc {
        name: "Menubar",
        slug: "menubar",
        group: "Collections",
        page: collections_command::menubar,
    },
    ComponentDoc {
        name: "NavigationMenu",
        slug: "navigation-menu",
        group: "Collections",
        page: collections_command::navigation_menu,
    },
    component_doc!("TopNav", "top-nav", "Collections", cxtn::top_nav),
    ComponentDoc {
        name: "Dropdown",
        slug: "dropdown",
        group: "Collections",
        page: collections_extra::dropdown,
    },
    ComponentDoc {
        name: "Select",
        slug: "select",
        group: "Collections",
        page: collections::select,
    },
    component_doc!("Picker", "picker", "Collections", cxp::picker),
    component_doc!(
        "ComboBox",
        "combo-box",
        "Collections",
        collections::combo_box
    ),
    component_doc!("Combobox", "combobox", "Collections", cxb::combobox),
    component_doc!(
        "Autocomplete",
        "autocomplete",
        "Collections",
        collections::autocomplete
    ),
    ComponentDoc {
        name: "Command",
        slug: "command",
        group: "Collections",
        page: collections_command::command,
    },
    ComponentDoc {
        name: "CommandDialog",
        slug: "command-dialog",
        group: "Collections",
        page: collections_command::command_dialog,
    },
    ComponentDoc {
        name: "ContextMenu",
        slug: "context-menu",
        group: "Collections",
        page: collections_command::context_menu,
    },
    ComponentDoc {
        name: "DropdownMenu",
        slug: "dropdown-menu",
        group: "Collections",
        page: collections::dropdown_menu,
    },
    ComponentDoc {
        name: "Pagination",
        slug: "pagination",
        group: "Collections",
        page: collections::pagination,
    },
    ComponentDoc {
        name: "Tag",
        slug: "tag",
        group: "Collections",
        page: collections_groups::tag,
    },
    ComponentDoc {
        name: "TagGroup",
        slug: "tag-group",
        group: "Collections",
        page: collections::tag_group,
    },
    component_doc!("Tags", "tags", "Collections", cxt::tags),
    ComponentDoc {
        name: "StepList",
        slug: "step-list",
        group: "Collections",
        page: collections_extra::step_list,
    },
    ComponentDoc {
        name: "Table",
        slug: "table",
        group: "Collections",
        page: collections_extra::table,
    },
    ComponentDoc {
        name: "Tree",
        slug: "tree",
        group: "Collections",
        page: collections_extra::tree,
    },
    ComponentDoc {
        name: "Overlay",
        slug: "overlay",
        group: "Overlays",
        page: overlays::overlay,
    },
    ComponentDoc {
        name: "Underlay",
        slug: "underlay",
        group: "Overlays",
        page: overlays_extra::underlay,
    },
    ComponentDoc {
        name: "Popover",
        slug: "popover",
        group: "Overlays",
        page: overlays::popover,
    },
    ComponentDoc {
        name: "PreviewCard",
        slug: "preview-card",
        group: "Overlays",
        page: overlays::preview_card,
    },
    ComponentDoc {
        name: "PreviewLinkCard",
        slug: "preview-link-card",
        group: "Overlays",
        page: overlays::preview_link_card,
    },
    ComponentDoc {
        name: "Modal",
        slug: "modal",
        group: "Overlays",
        page: overlays::modal,
    },
    ComponentDoc {
        name: "Dialog",
        slug: "dialog",
        group: "Overlays",
        page: overlays::dialog,
    },
    ComponentDoc {
        name: "AlertDialog",
        slug: "alert-dialog",
        group: "Overlays",
        page: overlays::alert_dialog,
    },
    ComponentDoc {
        name: "Sheet",
        slug: "sheet",
        group: "Overlays",
        page: overlays::sheet,
    },
    ComponentDoc {
        name: "BottomSheet",
        slug: "bottom-sheet",
        group: "Overlays",
        page: overlays_extra::bottom_sheet,
    },
    ComponentDoc {
        name: "Tray",
        slug: "tray",
        group: "Overlays",
        page: overlays_extra::tray,
    },
    ComponentDoc {
        name: "Drawer",
        slug: "drawer",
        group: "Overlays",
        page: overlays::drawer,
    },
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
