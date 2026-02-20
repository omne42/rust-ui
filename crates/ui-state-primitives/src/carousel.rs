pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    index.filter(|index| *index < item_count)
}

pub fn sanitize_enabled_index(index: Option<usize>, disabled_flags: &[bool]) -> Option<usize> {
    let index = sanitize_index(index, disabled_flags.len())?;
    (!disabled_flags[index]).then_some(index)
}

pub fn first_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().position(|disabled| !disabled)
}

pub fn last_enabled_index(disabled_flags: &[bool]) -> Option<usize> {
    disabled_flags.iter().rposition(|disabled| !disabled)
}

pub fn adjacent_enabled_index(
    disabled_flags: &[bool],
    current_index: usize,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    if disabled_flags.is_empty() || step == 0 {
        return None;
    }

    if should_loop {
        let len = disabled_flags.len() as isize;
        let mut cursor = current_index as isize;

        for _ in 0..disabled_flags.len().saturating_sub(1) {
            cursor = (cursor + step).rem_euclid(len);
            let index = cursor as usize;
            if !disabled_flags[index] {
                return Some(index);
            }
        }

        return None;
    }

    let mut cursor = current_index as isize;
    loop {
        cursor += step;
        if cursor < 0 || cursor >= disabled_flags.len() as isize {
            return None;
        }

        let index = cursor as usize;
        if !disabled_flags[index] {
            return Some(index);
        }
    }
}

pub fn resolve_initial_selected_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

pub fn resolve_initial_focused_index(
    disabled_flags: &[bool],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_enabled_index(selected_index, disabled_flags)
        .or_else(|| first_enabled_index(disabled_flags))
}

#[cfg(test)]
#[path = "test/carousel.rs"]
mod tests;
