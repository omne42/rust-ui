#[derive(Clone, Debug, Default, PartialEq)]
pub struct IllustratedMessageViewState {
    pub show_illustration: bool,
    pub show_title: bool,
    pub show_description: bool,
    pub show_actions: bool,
}

pub fn resolve_view_state(
    has_illustration: bool,
    title: Option<&str>,
    description: Option<&str>,
    has_actions: bool,
) -> IllustratedMessageViewState {
    let show_title = title.is_some_and(|v| !v.trim().is_empty());
    let show_description = description.is_some_and(|v| !v.trim().is_empty());

    IllustratedMessageViewState {
        show_illustration: has_illustration,
        show_title,
        show_description,
        show_actions: has_actions,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
