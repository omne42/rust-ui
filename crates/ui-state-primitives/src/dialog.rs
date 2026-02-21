#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DialogSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl DialogSize {
    pub fn class_name(self) -> &'static str {
        match self {
            DialogSize::Sm => "ui-dialog--size-sm",
            DialogSize::Md => "ui-dialog--size-md",
            DialogSize::Lg => "ui-dialog--size-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            DialogSize::Sm => "sm",
            DialogSize::Md => "md",
            DialogSize::Lg => "lg",
        }
    }
}

pub const DEFAULT_ID_BASE: &str = "ui-dialog";
pub const DEFAULT_TITLE: &str = "Dialog";
pub const DEFAULT_CLOSE_LABEL: &str = "Close";
pub const DEFAULT_SHOW_CLOSE_BUTTON: bool = true;
pub const DEFAULT_SIZE: DialogSize = DialogSize::Md;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogOpenMode {
    Controlled,
    Uncontrolled,
}

impl DialogOpenMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogOpenStateContractInput {
    pub has_is_open_prop: bool,
    pub has_open_prop: bool,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogOpenStateContract {
    pub mode: DialogOpenMode,
    pub open_prop_source_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
}

pub fn resolve_open_state_contract(input: DialogOpenStateContractInput) -> DialogOpenStateContract {
    let mode = if input.has_is_open_prop || input.has_open_prop {
        DialogOpenMode::Controlled
    } else {
        DialogOpenMode::Uncontrolled
    };
    let open_prop_source_attr = if input.has_is_open_prop {
        "is_open"
    } else if input.has_open_prop {
        "open"
    } else {
        "none"
    };
    let open_source_attr = if matches!(mode, DialogOpenMode::Controlled) {
        "controlled"
    } else if input.has_default_open {
        "default"
    } else {
        "implicit-default"
    };
    let open_change_source_attr = if input.has_open_change_handler {
        "custom"
    } else {
        "none"
    };

    DialogOpenStateContract {
        mode,
        open_prop_source_attr,
        open_mode_attr: mode.as_attr(),
        open_source_attr,
        open_change_source_attr,
    }
}

pub fn can_request_close(mode: DialogOpenMode, has_open_change_handler: bool) -> bool {
    matches!(mode, DialogOpenMode::Uncontrolled) || has_open_change_handler
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogCloseButtonVisibility {
    Visible,
    Hidden,
}

impl DialogCloseButtonVisibility {
    pub fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogCloseButtonPropSource {
    IsCloseButtonVisible,
    ShowCloseButton,
}

impl DialogCloseButtonPropSource {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::IsCloseButtonVisible => "is_close_button_visible",
            Self::ShowCloseButton => "show_close_button",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogCloseButtonContractInput {
    pub is_close_button_visible: bool,
    pub show_close_button: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogCloseButtonContract {
    pub visibility: DialogCloseButtonVisibility,
    pub prop_source: DialogCloseButtonPropSource,
}

pub fn resolve_close_button_contract(
    input: DialogCloseButtonContractInput,
) -> DialogCloseButtonContract {
    let visibility = if input
        .show_close_button
        .unwrap_or(input.is_close_button_visible)
    {
        DialogCloseButtonVisibility::Visible
    } else {
        DialogCloseButtonVisibility::Hidden
    };
    let prop_source = if input.show_close_button.is_some() {
        DialogCloseButtonPropSource::ShowCloseButton
    } else {
        DialogCloseButtonPropSource::IsCloseButtonVisible
    };

    DialogCloseButtonContract {
        visibility,
        prop_source,
    }
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

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_required_text(value: String, fallback: &'static str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        fallback.into()
    } else {
        trimmed.into()
    }
}

pub fn normalize_id_base(value: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        DEFAULT_ID_BASE.into()
    } else {
        trimmed.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogStateCoreInput {
    pub size: DialogSize,
    pub has_description: bool,
    pub has_footer: bool,
    pub show_close_button: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_close_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialogStateCore {
    pub size: DialogSize,
    pub size_attr: &'static str,
    pub size_class: &'static str,
    pub state_attr: &'static str,
    pub description_attr: &'static str,
    pub footer_attr: &'static str,
    pub close_button_attr: &'static str,
    pub show_description: bool,
    pub show_footer: bool,
    pub show_close_button: bool,
    pub has_custom_size: bool,
    pub has_custom_id_base: bool,
    pub has_custom_title: bool,
    pub has_custom_description: bool,
    pub has_custom_close_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_motion: bool,
    pub has_on_exit_complete: bool,
    pub size_source_attr: &'static str,
    pub description_source_attr: &'static str,
    pub footer_source_attr: &'static str,
    pub close_source_attr: &'static str,
    pub id_source_attr: &'static str,
    pub title_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub exit_source_attr: &'static str,
}

fn source_attr(is_custom: bool) -> &'static str {
    if is_custom { "custom" } else { "default" }
}

pub fn resolve_state_core(input: DialogStateCoreInput) -> DialogStateCore {
    let has_custom_size = input.size != DEFAULT_SIZE;
    let has_custom_close =
        input.has_custom_close_label || input.show_close_button != DEFAULT_SHOW_CLOSE_BUTTON;

    DialogStateCore {
        size: input.size,
        size_attr: input.size.as_attr(),
        size_class: input.size.class_name(),
        state_attr: state_attr(input.has_description),
        description_attr: description_attr(input.has_description),
        footer_attr: footer_attr(input.has_footer),
        close_button_attr: close_button_attr(input.show_close_button),
        show_description: input.has_description,
        show_footer: input.has_footer,
        show_close_button: input.show_close_button,
        has_custom_size,
        has_custom_id_base: input.has_custom_id_base,
        has_custom_title: input.has_custom_title,
        has_custom_description: input.has_custom_description,
        has_custom_close_label: input.has_custom_close_label,
        has_custom_class_name: input.has_custom_class_name,
        has_custom_motion: input.has_custom_motion,
        has_on_exit_complete: input.has_on_exit_complete,
        size_source_attr: source_attr(has_custom_size),
        description_source_attr: source_attr(input.has_custom_description),
        footer_source_attr: source_attr(input.has_footer),
        close_source_attr: source_attr(has_custom_close),
        id_source_attr: source_attr(input.has_custom_id_base),
        title_source_attr: source_attr(input.has_custom_title),
        class_source_attr: source_attr(input.has_custom_class_name),
        motion_source_attr: source_attr(input.has_custom_motion),
        exit_source_attr: source_attr(input.has_on_exit_complete),
    }
}

#[cfg(test)]
#[path = "test/dialog.rs"]
mod tests;
