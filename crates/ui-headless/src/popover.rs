pub fn should_dismiss_popover_on_escape(
    key: &str,
    is_topmost: bool,
    is_composing: bool,
    default_prevented: bool,
) -> bool {
    key == "Escape" && is_topmost && !is_composing && !default_prevented
}

#[cfg(test)]
#[path = "test/popover.rs"]
mod tests;
