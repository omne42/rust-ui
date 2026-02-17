use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CommonStrings {
    pub loading_aria_label: Arc<str>,
    pub close_aria_label: Arc<str>,
    pub clear_aria_label: Arc<str>,
    pub autocomplete_empty_message: Arc<str>,
    pub icon_button_aria_label: Arc<str>,
    pub avatar_fallback_aria_label: Arc<str>,
    pub avatar_group_aria_label: Arc<str>,
    pub avatar_group_overflow_aria_label_suffix: Arc<str>,
    pub search_input_button_placeholder: Arc<str>,
    pub action_button_group_aria_label: Arc<str>,
    pub action_group_aria_label: Arc<str>,
    pub action_menu_trigger_aria_label: Arc<str>,
    pub share_button_label: Arc<str>,
    pub share_button_group_aria_label: Arc<str>,
    pub share_platform_github_label: Arc<str>,
    pub share_platform_x_label: Arc<str>,
    pub share_platform_facebook_label: Arc<str>,
}

impl Default for CommonStrings {
    fn default() -> Self {
        Self {
            loading_aria_label: "Loading".into(),
            close_aria_label: "Close".into(),
            clear_aria_label: "Clear".into(),
            autocomplete_empty_message: "No matches".into(),
            icon_button_aria_label: "Icon button".into(),
            avatar_fallback_aria_label: "Avatar".into(),
            avatar_group_aria_label: "Avatar group".into(),
            avatar_group_overflow_aria_label_suffix: "more collaborators".into(),
            search_input_button_placeholder: "Search".into(),
            action_button_group_aria_label: "Action button group".into(),
            action_group_aria_label: "Action group".into(),
            action_menu_trigger_aria_label: "More actions".into(),
            share_button_label: "Share".into(),
            share_button_group_aria_label: "Share options".into(),
            share_platform_github_label: "GitHub".into(),
            share_platform_x_label: "X".into(),
            share_platform_facebook_label: "Facebook".into(),
        }
    }
}
