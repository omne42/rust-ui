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
        });
    });

    let on_item_focus = Callback::new(move |index: usize| set_active_index.set(index));

    let on_key_down = {
        let is_disabled = options.is_disabled;
        let should_loop = options.should_loop;
        let orientation = options.orientation;
        let item_count = options.item_count;
        Callback::new(move |key: String| -> bool {
            if is_disabled {
                return false;
            }

            let count = item_count.get_untracked();
            if count == 0 {
                return false;
            }

            let mut next = active_index.get_untracked();

            match key.as_str() {
                "Home" => next = 0,
                "End" => next = count.saturating_sub(1),
                "ArrowLeft"
                    if matches!(
                        orientation,
                        RovingOrientation::Horizontal | RovingOrientation::Both
                    ) =>
                {
                    if next == 0 {
                        if should_loop {
                            next = count.saturating_sub(1);
                        }
                    } else {
                        next -= 1;
                    }
                }
                "ArrowRight"
                    if matches!(
                        orientation,
                        RovingOrientation::Horizontal | RovingOrientation::Both
                    ) =>
                {
                    if next + 1 >= count {
                        if should_loop {
                            next = 0;
                        }
                    } else {
                        next += 1;
                    }
                }
                "ArrowUp"
                    if matches!(
                        orientation,
                        RovingOrientation::Vertical | RovingOrientation::Both
                    ) =>
                {
                    if next == 0 {
                        if should_loop {
                            next = count.saturating_sub(1);
                        }
                    } else {
                        next -= 1;
                    }
                }
                "ArrowDown"
                    if matches!(
                        orientation,
                        RovingOrientation::Vertical | RovingOrientation::Both
                    ) =>
                {
                    if next + 1 >= count {
                        if should_loop {
                            next = 0;
                        }
                    } else {
                        next += 1;
                    }
                }
                _ => return false,
            }

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
