#[cfg(feature = "inject-css")]
pub fn push_components_css(out: &mut String) {
    out.push_str("\n@layer ui {\n");
    #[cfg(feature = "component-active_highlight")]
    out.push_str(crate::active_highlight::CSS);
    #[cfg(feature = "component-avatar")]
    out.push_str(crate::avatar::styles::CSS);
    #[cfg(feature = "component-avatar_group")]
    out.push_str(crate::avatar::styles::AVATAR_GROUP_CSS);
    #[cfg(feature = "component-action_bar")]
    out.push_str(crate::action_bar::styles::CSS);
    #[cfg(feature = "component-button")]
    out.push_str(crate::button::styles::CSS);
    #[cfg(feature = "component-clear_button")]
    out.push_str(crate::clear_button::styles::CSS);
    #[cfg(feature = "component-close_button")]
    out.push_str(crate::close_button::styles::CSS);
    #[cfg(feature = "component-action_button_group")]
    out.push_str(crate::button::action::styles::ACTION_BUTTON_GROUP_CSS);
    #[cfg(feature = "component-action_group")]
    out.push_str(crate::button::action::styles::ACTION_GROUP_CSS);
    #[cfg(feature = "component-button_copy")]
    out.push_str(crate::button::copy::styles::CSS);
    #[cfg(feature = "component-button_search_input")]
    out.push_str(crate::button::search_input::styles::CSS);
    #[cfg(feature = "component-button_flip")]
    out.push_str(crate::button::flip::styles::CSS);
    #[cfg(feature = "component-button_share")]
    out.push_str(crate::button::share::styles::CSS);
    #[cfg(feature = "component-button_group")]
    out.push_str(crate::button::styles::BUTTON_GROUP_CSS);
    #[cfg(feature = "component-button_theme_toggle")]
    out.push_str(crate::button::theme_toggle::styles::CSS);
    #[cfg(feature = "component-ripple")]
    out.push_str(crate::ripple::styles::CSS);
    #[cfg(feature = "component-toggle_button")]
    out.push_str(crate::button::toggle_button::styles::CSS);
    #[cfg(feature = "component-toggle_button_group")]
    out.push_str(crate::button::toggle_button::styles::TOGGLE_BUTTON_GROUP_CSS);
    #[cfg(feature = "component-toggle_group")]
    out.push_str(crate::button::toggle::styles::TOGGLE_GROUP_CSS);
    #[cfg(feature = "component-badge")]
    out.push_str(crate::badge::styles::CSS);
    #[cfg(feature = "component-status_light")]
    out.push_str(crate::status_light::styles::CSS);
    #[cfg(feature = "component-checkbox")]
    out.push_str(crate::checkbox::styles::CSS);
    #[cfg(feature = "component-checkbox_field")]
    out.push_str(crate::checkbox_field::styles::CSS);
    #[cfg(feature = "component-checkbox_group")]
    out.push_str(crate::checkbox::styles::CHECKBOX_GROUP_CSS);
    #[cfg(feature = "component-switch")]
    out.push_str(crate::switch::styles::CSS);
    #[cfg(feature = "component-switch_group")]
    out.push_str(crate::switch::group::styles::CSS);
    #[cfg(feature = "component-circular_progress")]
    out.push_str(crate::circular_progress::styles::CSS);
    #[cfg(feature = "component-spinner")]
    out.push_str(crate::spinner::styles::CSS);
    #[cfg(feature = "component-calendar")]
    out.push_str(crate::calendar::styles::CSS);
    #[cfg(feature = "component-carousel")]
    out.push_str(crate::carousel::styles::CSS);
    #[cfg(feature = "component-alert")]
    out.push_str(crate::alert::styles::CSS);
    #[cfg(feature = "component-alert_banner")]
    out.push_str(crate::alert_banner::styles::CSS);
    #[cfg(feature = "component-inline_alert")]
    out.push_str(crate::inline_alert::styles::CSS);
    #[cfg(feature = "component-chip")]
    out.push_str(crate::chip::styles::CSS);
    #[cfg(feature = "component-chart")]
    out.push_str(crate::chart::styles::CSS);
    #[cfg(feature = "component-collapsible")]
    out.push_str(crate::collapsible::styles::CSS);
    #[cfg(feature = "component-command")]
    out.push_str(crate::command::styles::CSS);
    #[cfg(feature = "component-command_dialog")]
    out.push_str(crate::command_dialog::styles::CSS);
    #[cfg(feature = "component-color_area")]
    out.push_str(crate::color::area::styles::CSS);
    #[cfg(feature = "component-color_field")]
    out.push_str(crate::color::field::styles::CSS);
    #[cfg(feature = "component-color_handle")]
    out.push_str(crate::color::handle::styles::CSS);
    #[cfg(feature = "component-color_loupe")]
    out.push_str(crate::color::loupe::styles::CSS);
    #[cfg(feature = "component-color_editor")]
    out.push_str(crate::color::editor::styles::CSS);
    #[cfg(feature = "component-color_picker")]
    out.push_str(crate::color::picker::styles::CSS);
    #[cfg(feature = "component-color_slider")]
    out.push_str(crate::color::slider::styles::CSS);
    #[cfg(feature = "component-color_thumb")]
    out.push_str(crate::color::thumb::styles::CSS);
    #[cfg(feature = "component-color_wheel")]
    out.push_str(crate::color::wheel::styles::CSS);
    #[cfg(feature = "component-color_swatch")]
    out.push_str(crate::color::swatch::styles::CSS);
    #[cfg(feature = "component-color_swatch_picker")]
    out.push_str(crate::color::swatch_picker::styles::CSS);
    #[cfg(feature = "component-swatch")]
    out.push_str(crate::swatch::styles::CSS);
    #[cfg(feature = "component-icon")]
    out.push_str(crate::icon::styles::CSS);
    #[cfg(feature = "component-iconset")]
    out.push_str(crate::iconset::styles::CSS);
    #[cfg(feature = "component-icons")]
    out.push_str(crate::icons::styles::CSS);
    #[cfg(feature = "component-icons_ui")]
    out.push_str(crate::icons_ui::styles::CSS);
    #[cfg(feature = "component-icons_workflow")]
    out.push_str(crate::icons_workflow::styles::CSS);
    #[cfg(feature = "component-tag")]
    out.push_str(crate::tag::styles::CSS);
    #[cfg(feature = "component-asset")]
    out.push_str(crate::asset::styles::CSS);
    #[cfg(feature = "component-image")]
    out.push_str(crate::image::styles::CSS);
    #[cfg(feature = "component-thumbnail")]
    out.push_str(crate::thumbnail::styles::CSS);
    #[cfg(feature = "component-infield_button")]
    out.push_str(crate::infield_button::styles::CSS);
    #[cfg(feature = "component-tag_group")]
    out.push_str(crate::tag::group::styles::CSS);
    #[cfg(feature = "component-table")]
    out.push_str(crate::table::styles::CSS);
    #[cfg(feature = "component-pagination")]
    out.push_str(crate::pagination::styles::CSS);
    #[cfg(feature = "component-skeleton")]
    out.push_str(crate::skeleton::styles::CSS);
    #[cfg(feature = "component-skeleton_group")]
    out.push_str(crate::skeleton::group::styles::CSS);
    #[cfg(feature = "component-link")]
    out.push_str(crate::link::styles::CSS);
    #[cfg(feature = "component-legend")]
    out.push_str(crate::legend::styles::CSS);
    #[cfg(feature = "component-breadcrumb")]
    out.push_str(crate::breadcrumb::styles::CSS);
    #[cfg(feature = "component-breadcrumbs")]
    out.push_str(crate::breadcrumbs::styles::CSS);
    #[cfg(feature = "component-code")]
    out.push_str(crate::code::styles::CSS);
    #[cfg(feature = "component-code_block")]
    out.push_str(crate::code_block::styles::CSS);
    #[cfg(feature = "component-snippet")]
    out.push_str(crate::snippet::styles::CSS);
    #[cfg(feature = "component-text")]
    out.push_str(crate::text_input::text::styles::CSS);
    #[cfg(feature = "component-visually_hidden")]
    out.push_str(crate::visually_hidden::CSS);
    #[cfg(feature = "component-label")]
    out.push_str(crate::label::styles::CSS);
    #[cfg(feature = "component-labeled_value")]
    out.push_str(crate::labeled_value::styles::CSS);
    #[cfg(feature = "component-kbd")]
    out.push_str(crate::kbd::styles::CSS);
    #[cfg(feature = "component-progress_bar")]
    out.push_str(crate::progress_bar::styles::CSS);
    #[cfg(feature = "component-progress")]
    out.push_str(crate::progress::styles::CSS);
    #[cfg(feature = "component-progress_circle")]
    out.push_str(crate::progress_circle::styles::CSS);
    #[cfg(feature = "component-pressable_feedback")]
    out.push_str(crate::pressable_feedback::styles::CSS);
    #[cfg(feature = "component-link_button")]
    out.push_str(crate::link_button::styles::CSS);
    #[cfg(feature = "component-logic_button")]
    out.push_str(crate::logic_button::styles::CSS);
    #[cfg(feature = "component-autocomplete")]
    out.push_str(crate::autocomplete::styles::CSS);
    #[cfg(feature = "component-combo_box")]
    out.push_str(crate::combo_box::styles::CSS);
    #[cfg(feature = "component-text_field")]
    out.push_str(crate::text_field::styles::CSS);
    #[cfg(feature = "component-date_field")]
    out.push_str(crate::text_input::date_field::styles::CSS);
    #[cfg(feature = "component-date_input_group")]
    out.push_str(crate::text_input::date_input_group::styles::CSS);
    #[cfg(feature = "component-time_field")]
    out.push_str(crate::time_field::styles::CSS);
    #[cfg(feature = "component-date_range_picker")]
    out.push_str(crate::text_input::date_range_picker::styles::CSS);
    #[cfg(feature = "component-date_picker")]
    out.push_str(crate::text_input::date_picker::styles::CSS);
    #[cfg(feature = "component-search_field")]
    out.push_str(crate::search_field::styles::CSS);
    #[cfg(feature = "component-text_area")]
    out.push_str(crate::text_input::text_area::styles::CSS);
    #[cfg(feature = "component-textarea")]
    out.push_str(crate::textarea::styles::CSS);
    #[cfg(feature = "component-number_field")]
    out.push_str(crate::text_input::number_field::styles::CSS);
    #[cfg(feature = "component-number")]
    out.push_str(crate::number::styles::CSS);
    #[cfg(feature = "component-slider")]
    out.push_str(crate::slider::styles::CSS);
    #[cfg(feature = "component-input_group")]
    out.push_str(crate::text_input::input::group::styles::CSS);
    #[cfg(feature = "component-input")]
    out.push_str(crate::text_input::input::styles::CSS);
    #[cfg(feature = "component-input_otp")]
    out.push_str(crate::text_input::input_otp::styles::CSS);
    #[cfg(feature = "component-file_trigger")]
    out.push_str(crate::file_trigger::styles::CSS);
    #[cfg(feature = "component-flip_card")]
    out.push_str(crate::flip_card::styles::CSS);
    #[cfg(feature = "component-drop_zone")]
    out.push_str(crate::drop_zone::styles::CSS);
    #[cfg(feature = "component-empty_state")]
    out.push_str(crate::empty_state::styles::CSS);
    #[cfg(feature = "component-empty")]
    out.push_str(crate::empty::styles::CSS);
    #[cfg(feature = "component-error_view")]
    out.push_str(crate::error_view::styles::CSS);
    #[cfg(feature = "component-field")]
    out.push_str(crate::field_form::field::styles::CSS);
    #[cfg(feature = "component-button")]
    out.push_str(crate::button::field::styles::CSS);
    #[cfg(feature = "component-picker_button")]
    out.push_str(crate::picker_button::styles::CSS);
    #[cfg(feature = "component-field_error")]
    out.push_str(crate::field_form::field_error::styles::CSS);
    #[cfg(feature = "component-field_group")]
    out.push_str(crate::field_form::field::group::styles::CSS);
    #[cfg(feature = "component-field_label")]
    out.push_str(crate::field_form::field_label::styles::CSS);
    #[cfg(feature = "component-error_message")]
    out.push_str(crate::error_message::styles::CSS);
    #[cfg(feature = "component-fieldset")]
    out.push_str(crate::field_form::fieldset::styles::CSS);
    #[cfg(feature = "component-form")]
    out.push_str(crate::field_form::form::styles::CSS);
    #[cfg(feature = "component-form_field")]
    out.push_str(crate::field_form::form_field::styles::CSS);
    #[cfg(feature = "component-segmented_control")]
    out.push_str(crate::segmented_control::styles::CSS);
    #[cfg(feature = "component-radio")]
    out.push_str(crate::radio::styles::CSS);
    #[cfg(feature = "component-tabs")]
    out.push_str(crate::tabs::styles::CSS);
    #[cfg(feature = "component-step_list")]
    out.push_str(crate::step_list::styles::CSS);
    #[cfg(feature = "component-accordion")]
    out.push_str(crate::accordion::styles::CSS);
    #[cfg(feature = "component-disclosure")]
    out.push_str(crate::disclosure::styles::CSS);
    #[cfg(feature = "component-disclosure_group")]
    out.push_str(crate::disclosure::group::styles::CSS);
    #[cfg(feature = "component-overlay")]
    out.push_str(crate::overlay::styles::CSS);
    #[cfg(feature = "component-overlays")]
    out.push_str(crate::overlays::styles::CSS);
    #[cfg(feature = "component-underlay")]
    out.push_str(crate::underlay::styles::CSS);
    #[cfg(feature = "component-popover")]
    out.push_str(crate::popover::styles::CSS);
    #[cfg(feature = "component-preview_card")]
    out.push_str(crate::preview_card::styles::CSS);
    #[cfg(feature = "component-preview_link_card")]
    out.push_str(crate::preview_link_card::styles::CSS);
    #[cfg(feature = "component-tooltip")]
    out.push_str(crate::tooltip::styles::CSS);
    #[cfg(feature = "component-contextual_help")]
    out.push_str(crate::contextual_help::styles::CSS);
    #[cfg(feature = "component-coachmark")]
    out.push_str(crate::coachmark::styles::CSS);
    #[cfg(feature = "component-hover_card")]
    out.push_str(crate::hover_card::styles::CSS);
    #[cfg(feature = "component-modal")]
    out.push_str(crate::modal::styles::CSS);
    #[cfg(feature = "component-dialog")]
    out.push_str(crate::dialog::styles::CSS);
    #[cfg(feature = "component-alert_dialog")]
    out.push_str(crate::alert_dialog::styles::CSS);
    #[cfg(feature = "component-sheet")]
    out.push_str(crate::sheet::styles::CSS);
    #[cfg(feature = "component-sidebar")]
    out.push_str(crate::sidebar::styles::CSS);
    #[cfg(feature = "component-sidebar_content")]
    out.push_str(crate::sidebar_content::styles::CSS);
    #[cfg(feature = "component-sidebar_inset")]
    out.push_str(crate::sidebar_inset::styles::CSS);
    #[cfg(feature = "component-sidebar_rail")]
    out.push_str(crate::sidebar_rail::styles::CSS);
    #[cfg(feature = "component-sidebar_trigger")]
    out.push_str(crate::sidebar_trigger::styles::CSS);
    #[cfg(feature = "component-sidebar_footer")]
    out.push_str(crate::sidebar_footer::styles::CSS);
    #[cfg(feature = "component-sidebar_header")]
    out.push_str(crate::sidebar_header::styles::CSS);
    #[cfg(feature = "component-sidebar_group")]
    out.push_str(crate::sidebar::group::styles::CSS);
    #[cfg(feature = "component-sidebar_menu")]
    out.push_str(crate::sidebar_menu::styles::CSS);
    #[cfg(feature = "component-sidebar_menu_action")]
    out.push_str(crate::sidebar_menu_action::styles::CSS);
    #[cfg(feature = "component-sidebar_menu_badge")]
    out.push_str(crate::sidebar_menu_badge::styles::CSS);
    #[cfg(feature = "component-bottom_sheet")]
    out.push_str(crate::bottom_sheet::styles::CSS);
    #[cfg(feature = "component-tray")]
    out.push_str(crate::tray::styles::CSS);
    #[cfg(feature = "component-drawer")]
    out.push_str(crate::drawer::styles::CSS);
    #[cfg(feature = "component-meter")]
    out.push_str(crate::meter::styles::CSS);
    #[cfg(feature = "component-illustrated_message")]
    out.push_str(crate::illustrated_message::styles::CSS);
    #[cfg(feature = "component-toast")]
    out.push_str(crate::toast::styles::CSS);
    #[cfg(feature = "component-sonner")]
    out.push_str(crate::sonner::styles::CSS);
    #[cfg(feature = "component-toaster")]
    out.push_str(crate::toaster::styles::CSS);
    #[cfg(feature = "component-toggle")]
    out.push_str(crate::button::toggle::styles::CSS);
    #[cfg(feature = "component-list")]
    {
        out.push_str(crate::list::styles::CSS);
        out.push_str(crate::list::styles::ITEM_CSS);
        out.push_str(crate::list::styles::SECTION_CSS);
    }
    #[cfg(feature = "component-menu")]
    out.push_str(crate::menu::styles::CSS);
    #[cfg(feature = "component-menu_item")]
    out.push_str(crate::menu::item::styles::CSS);
    #[cfg(feature = "component-menu_section")]
    out.push_str(crate::menu::section::styles::CSS);
    #[cfg(feature = "component-select")]
    out.push_str(crate::select::styles::CSS);
    #[cfg(feature = "component-native_select")]
    out.push_str(crate::native_select::styles::CSS);
    #[cfg(feature = "component-dropdown")]
    out.push_str(crate::dropdown::styles::CSS);
    #[cfg(feature = "component-dropdown_menu")]
    out.push_str(crate::dropdown_menu::styles::CSS);
    #[cfg(feature = "component-context_menu")]
    out.push_str(crate::context_menu::styles::CSS);
    #[cfg(feature = "component-action_menu")]
    out.push_str(crate::action_menu::styles::CSS);
    #[cfg(feature = "component-menu_trigger")]
    out.push_str(crate::menu_trigger::styles::CSS);
    #[cfg(feature = "component-menubar")]
    out.push_str(crate::menubar::styles::CSS);
    #[cfg(feature = "component-navigation_menu")]
    out.push_str(crate::navigation_menu::styles::CSS);
    #[cfg(feature = "component-tree")]
    out.push_str(crate::tree::styles::CSS);
    #[cfg(feature = "component-direction")]
    out.push_str(crate::direction::styles::CSS);
    #[cfg(feature = "component-description")]
    out.push_str(crate::field_form::description::styles::CSS);
    #[cfg(feature = "component-help_text")]
    out.push_str(crate::field_form::help_text::styles::CSS);
    #[cfg(feature = "component-item")]
    out.push_str(crate::item::styles::CSS);
    #[cfg(feature = "component-keyboard")]
    out.push_str(crate::keyboard::styles::CSS);
    out.push_str("\n}\n");
}

#[cfg(not(feature = "inject-css"))]
pub fn push_components_css(_out: &mut String) {}
