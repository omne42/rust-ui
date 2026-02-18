pub const DEFAULT_ID_BASE: &str = "ui-tray";
pub const DEFAULT_TITLE: &str = "Tray";
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;
pub const DEFAULT_FIXED_HEIGHT: bool = false;
pub const DEFAULT_DISMISSABLE: bool = true;
pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraySlot {
    Root,
    Header,
    Title,
    Description,
    Body,
    Footer,
    Close,
}

impl TraySlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            TraySlot::Root => "tray",
            TraySlot::Header => "tray-header",
            TraySlot::Title => "tray-title",
            TraySlot::Description => "tray-description",
            TraySlot::Body => "tray-body",
            TraySlot::Footer => "tray-footer",
            TraySlot::Close => "tray-close",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            TraySlot::Root => "ui-tray",
            TraySlot::Header => "ui-tray__header",
            TraySlot::Title => "ui-tray__title",
            TraySlot::Description => "ui-tray__description",
            TraySlot::Body => "ui-tray__body",
            TraySlot::Footer => "ui-tray__footer",
            TraySlot::Close => "ui-tray__close",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayPartStateInput {
    pub slot: TraySlot,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub is_fixed_height: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrayPartState {
    pub slot: TraySlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub size_attr: &'static str,
    pub dismiss_attr: &'static str,
    pub keyboard_dismiss_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub is_fixed_height: bool,
    pub is_dismissable: bool,
    pub is_keyboard_dismiss_disabled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub size_source_attr: &'static str,
    pub dismiss_source_attr: &'static str,
    pub keyboard_dismiss_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

pub fn state_attr(has_description: bool) -> &'static str {
    if has_description {
        "with-description"
    } else {
        "title-only"
    }
}

pub fn description_attr(has_description: bool) -> &'static str {
    if has_description { "present" } else { "absent" }
}

pub fn footer_attr(has_footer: bool) -> &'static str {
    if has_footer { "present" } else { "absent" }
}

pub fn close_button_attr(show_close_button: bool) -> &'static str {
    if show_close_button { "shown" } else { "hidden" }
}

pub fn size_attr(is_fixed_height: bool) -> &'static str {
    if is_fixed_height { "fixed" } else { "auto" }
}

pub fn dismiss_attr(is_dismissable: bool) -> &'static str {
    if is_dismissable {
        "dismissable"
    } else {
        "locked"
    }
}

pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool) -> &'static str {
    if is_keyboard_dismiss_disabled {
        "disabled"
    } else {
        "enabled"
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.to_string()
    } else {
        trimmed.to_string()
    }
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state(input: TrayPartStateInput) -> TrayPartState {
    TrayPartState {
        slot: input.slot,
        slot_attr: input.slot.as_attr(),
        base_class: input.slot.base_class(),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        size_attr: size_attr(input.is_fixed_height),
        dismiss_attr: dismiss_attr(input.is_dismissable),
        keyboard_dismiss_attr: keyboard_dismiss_attr(input.is_keyboard_dismiss_disabled),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        is_fixed_height: input.is_fixed_height,
        is_dismissable: input.is_dismissable,
        is_keyboard_dismiss_disabled: input.is_keyboard_dismiss_disabled,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON),
        size_source_attr: source_attr(input.is_fixed_height != DEFAULT_FIXED_HEIGHT),
        dismiss_source_attr: source_attr(input.is_dismissable != DEFAULT_DISMISSABLE),
        keyboard_dismiss_source_attr: source_attr(
            input.is_keyboard_dismiss_disabled != DEFAULT_KEYBOARD_DISMISS_DISABLED,
        ),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_attrs_and_classes_follow_contract() {
        assert_eq!(TraySlot::Root.as_attr(), "tray");
        assert_eq!(TraySlot::Header.as_attr(), "tray-header");
        assert_eq!(TraySlot::Title.as_attr(), "tray-title");
        assert_eq!(TraySlot::Description.as_attr(), "tray-description");
        assert_eq!(TraySlot::Body.as_attr(), "tray-body");
        assert_eq!(TraySlot::Footer.as_attr(), "tray-footer");
        assert_eq!(TraySlot::Close.as_attr(), "tray-close");

        assert_eq!(TraySlot::Root.base_class(), "ui-tray");
        assert_eq!(TraySlot::Header.base_class(), "ui-tray__header");
        assert_eq!(TraySlot::Title.base_class(), "ui-tray__title");
        assert_eq!(TraySlot::Description.base_class(), "ui-tray__description");
        assert_eq!(TraySlot::Body.base_class(), "ui-tray__body");
        assert_eq!(TraySlot::Footer.base_class(), "ui-tray__footer");
        assert_eq!(TraySlot::Close.base_class(), "ui-tray__close");
    }

    #[test]
    fn state_attrs_follow_contract() {
        assert_eq!(state_attr(true), "with-description");
        assert_eq!(state_attr(false), "title-only");
        assert_eq!(description_attr(true), "present");
        assert_eq!(description_attr(false), "absent");
        assert_eq!(footer_attr(true), "present");
        assert_eq!(footer_attr(false), "absent");
        assert_eq!(close_button_attr(true), "shown");
        assert_eq!(close_button_attr(false), "hidden");
        assert_eq!(size_attr(true), "fixed");
        assert_eq!(size_attr(false), "auto");
        assert_eq!(dismiss_attr(true), "dismissable");
        assert_eq!(dismiss_attr(false), "locked");
        assert_eq!(keyboard_dismiss_attr(false), "enabled");
        assert_eq!(keyboard_dismiss_attr(true), "disabled");
    }

    #[test]
    fn normalize_helpers_trim_and_fallback() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some("  ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-tray ".to_string())),
            Some("docs-tray".to_string())
        );

        assert_eq!(
            normalize_required_text(" Tray ".to_string(), DEFAULT_TITLE),
            "Tray"
        );
        assert_eq!(
            normalize_required_text(" ".to_string(), DEFAULT_TITLE),
            DEFAULT_TITLE
        );

        assert_eq!(normalize_id_base(" docs-tray ".to_string()), "docs-tray");
        assert_eq!(normalize_id_base(" ".to_string()), DEFAULT_ID_BASE);
    }

    #[test]
    fn resolve_state_tracks_source_markers() {
        let state = resolve_state(TrayPartStateInput {
            slot: TraySlot::Root,
            has_description: true,
            has_footer: true,
            show_close_button: false,
            is_fixed_height: true,
            is_dismissable: false,
            is_keyboard_dismiss_disabled: true,
            has_custom_id_base: true,
            has_custom_title: true,
            has_custom_description: true,
            has_custom_class_name: true,
            has_custom_motion: true,
            has_on_exit_complete: true,
        });

        assert_eq!(state.slot_attr, "tray");
        assert_eq!(state.base_class, "ui-tray");
        assert_eq!(state.state_attr, "with-description");
        assert_eq!(state.description_attr, "present");
        assert_eq!(state.footer_attr, "present");
        assert_eq!(state.close_button_attr, "hidden");
        assert_eq!(state.size_attr, "fixed");
        assert_eq!(state.dismiss_attr, "locked");
        assert_eq!(state.keyboard_dismiss_attr, "disabled");
        assert_eq!(state.description_source_attr, "custom");
        assert_eq!(state.footer_source_attr, "custom");
        assert_eq!(state.close_source_attr, "custom");
        assert_eq!(state.size_source_attr, "custom");
        assert_eq!(state.dismiss_source_attr, "custom");
        assert_eq!(state.keyboard_dismiss_source_attr, "custom");
        assert_eq!(state.id_source_attr, "custom");
        assert_eq!(state.title_source_attr, "custom");
        assert_eq!(state.class_source_attr, "custom");
        assert_eq!(state.motion_source_attr, "custom");
        assert_eq!(state.exit_source_attr, "custom");
    }
}
