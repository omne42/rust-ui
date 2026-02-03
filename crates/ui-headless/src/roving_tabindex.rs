use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RovingOrientation {
    Horizontal,
    Vertical,
    #[default]
    Both,
}

#[derive(Clone, Copy)]
pub struct RovingTabIndexOptions {
    pub is_disabled: bool,
    pub default_index: usize,
    pub should_loop: bool,
    pub orientation: RovingOrientation,
    pub item_count: ReadSignal<usize>,
    pub is_item_disabled: Option<Callback<usize, bool>>,
}

#[derive(Clone)]
pub struct RovingTabIndexHandlers {
    pub on_key_down: Callback<String, bool>,
    pub on_item_focus: Callback<usize>,
}

#[derive(Clone)]
pub struct RovingTabIndexState {
    pub active_index: ReadSignal<usize>,
    pub handlers: RovingTabIndexHandlers,
}

pub fn use_roving_tabindex(options: RovingTabIndexOptions) -> RovingTabIndexState {
    let (active_index, set_active_index) = signal(options.default_index);

    // Clamp active index when the item count changes.
    Effect::new(move |_| {
        let count = options.item_count.get();
        if count == 0 {
            set_active_index.set(0);
            return;
        }
        set_active_index.update(|i| {
            if *i >= count {
                *i = count.saturating_sub(1);
            }
            if let Some(is_item_disabled) = options.is_item_disabled {
                if is_item_disabled.run(*i) {
                    if let Some(first_enabled) = (0..count).find(|&idx| !is_item_disabled.run(idx))
                    {
                        *i = first_enabled;
                    }
                }
            }
        });
    });

    let on_item_focus = Callback::new(move |index: usize| {
        if options.is_disabled {
            return;
        }
        if let Some(is_item_disabled) = options.is_item_disabled {
            if is_item_disabled.run(index) {
                return;
            }
        }
        set_active_index.set(index);
    });

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let should_loop = options.should_loop;
        let orientation = options.orientation;
        let item_count = options.item_count;
        let is_item_disabled = options.is_item_disabled;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            let count = item_count.get_untracked();
            if count == 0 {
                return false;
            }

            let current = active_index.get_untracked();

            let advance = |index: usize, delta: i32| -> Option<usize> {
                match delta {
                    1 => {
                        if index + 1 >= count {
                            should_loop.then_some(0)
                        } else {
                            Some(index + 1)
                        }
                    }
                    -1 => {
                        if index == 0 {
                            should_loop.then_some(count.saturating_sub(1))
                        } else {
                            Some(index - 1)
                        }
                    }
                    _ => None,
                }
            };

            let next_enabled = |delta: i32| -> Option<usize> {
                let is_item_disabled = is_item_disabled?;
                let mut cursor = current;
                for _ in 0..count {
                    let candidate = advance(cursor, delta)?;
                    if !is_item_disabled.run(candidate) {
                        return Some(candidate);
                    }
                    cursor = candidate;
                }
                None
            };

            let next = match key.as_str() {
                "Home" => {
                    if let Some(is_item_disabled) = is_item_disabled {
                        (0..count)
                            .find(|&idx| !is_item_disabled.run(idx))
                            .unwrap_or(current)
                    } else {
                        0
                    }
                }
                "End" => {
                    if let Some(is_item_disabled) = is_item_disabled {
                        (0..count)
                            .rev()
                            .find(|&idx| !is_item_disabled.run(idx))
                            .unwrap_or(current)
                    } else {
                        count.saturating_sub(1)
                    }
                }
                "ArrowLeft"
                    if matches!(
                        orientation,
                        RovingOrientation::Horizontal | RovingOrientation::Both
                    ) =>
                {
                    if is_item_disabled.is_some() {
                        next_enabled(-1).unwrap_or(current)
                    } else {
                        advance(current, -1).unwrap_or(current)
                    }
                }
                "ArrowRight"
                    if matches!(
                        orientation,
                        RovingOrientation::Horizontal | RovingOrientation::Both
                    ) =>
                {
                    if is_item_disabled.is_some() {
                        next_enabled(1).unwrap_or(current)
                    } else {
                        advance(current, 1).unwrap_or(current)
                    }
                }
                "ArrowUp"
                    if matches!(
                        orientation,
                        RovingOrientation::Vertical | RovingOrientation::Both
                    ) =>
                {
                    if is_item_disabled.is_some() {
                        next_enabled(-1).unwrap_or(current)
                    } else {
                        advance(current, -1).unwrap_or(current)
                    }
                }
                "ArrowDown"
                    if matches!(
                        orientation,
                        RovingOrientation::Vertical | RovingOrientation::Both
                    ) =>
                {
                    if is_item_disabled.is_some() {
                        next_enabled(1).unwrap_or(current)
                    } else {
                        advance(current, 1).unwrap_or(current)
                    }
                }
                _ => return false,
            };

            set_active_index.set(next);
            true
        })
    };

    RovingTabIndexState {
        active_index,
        handlers: RovingTabIndexHandlers {
            on_key_down,
            on_item_focus,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_executor() {
        let _ = any_spawner::Executor::init_futures_executor();
    }

    #[test]
    fn default_index_skips_disabled_items() {
        init_executor();
        let (count, _set_count) = signal(3_usize);
        let roving = use_roving_tabindex(RovingTabIndexOptions {
            is_disabled: false,
            default_index: 0,
            should_loop: true,
            orientation: RovingOrientation::Vertical,
            item_count: count,
            is_item_disabled: Some(Callback::new(|index: usize| index == 0)),
        });

        any_spawner::Executor::poll_local();
        assert_eq!(roving.active_index.get_untracked(), 1);
    }

    #[test]
    fn arrow_navigation_skips_disabled_items_and_loops() {
        init_executor();
        let (count, _set_count) = signal(3_usize);
        let roving = use_roving_tabindex(RovingTabIndexOptions {
            is_disabled: false,
            default_index: 0,
            should_loop: true,
            orientation: RovingOrientation::Vertical,
            item_count: count,
            is_item_disabled: Some(Callback::new(|index: usize| index == 1)),
        });

        any_spawner::Executor::poll_local();
        assert_eq!(roving.active_index.get_untracked(), 0);

        roving.handlers.on_key_down.run("ArrowDown".to_string());
        assert_eq!(roving.active_index.get_untracked(), 2);

        roving.handlers.on_key_down.run("ArrowDown".to_string());
        assert_eq!(roving.active_index.get_untracked(), 0);

        roving.handlers.on_key_down.run("ArrowUp".to_string());
        assert_eq!(roving.active_index.get_untracked(), 2);
    }

    #[test]
    fn focus_ignores_disabled_items() {
        init_executor();
        let (count, _set_count) = signal(3_usize);
        let roving = use_roving_tabindex(RovingTabIndexOptions {
            is_disabled: false,
            default_index: 0,
            should_loop: true,
            orientation: RovingOrientation::Vertical,
            item_count: count,
            is_item_disabled: Some(Callback::new(|index: usize| index == 2)),
        });

        any_spawner::Executor::poll_local();
        roving.handlers.on_item_focus.run(2);
        assert_eq!(roving.active_index.get_untracked(), 0);
    }

    #[test]
    fn home_and_end_select_first_and_last_enabled() {
        init_executor();
        let (count, _set_count) = signal(4_usize);
        let roving = use_roving_tabindex(RovingTabIndexOptions {
            is_disabled: false,
            default_index: 0,
            should_loop: true,
            orientation: RovingOrientation::Vertical,
            item_count: count,
            is_item_disabled: Some(Callback::new(|index: usize| matches!(index, 0 | 3))),
        });

        any_spawner::Executor::poll_local();
        roving.handlers.on_key_down.run("Home".to_string());
        assert_eq!(roving.active_index.get_untracked(), 1);

        roving.handlers.on_key_down.run("End".to_string());
        assert_eq!(roving.active_index.get_untracked(), 2);
    }
}
