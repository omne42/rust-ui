use crate::selection::{
    Key, SelectedKey, SingleSelectionState, SingleSelectionStateOptions, use_single_selection_state,
};

#[derive(Clone, Default)]
pub struct ListStateOptions {
    pub items: Vec<Key>,
    pub selection: SingleSelectionStateOptions,
}

#[derive(Clone)]
pub struct ListState {
    items: Vec<Key>,
    selection: SingleSelectionState,
}

pub fn use_list_state(options: ListStateOptions) -> ListState {
    ListState {
        items: options.items,
        selection: use_single_selection_state(options.selection),
    }
}

impl ListState {
    pub fn items(&self) -> &[Key] {
        &self.items
    }

    pub fn selection(&self) -> &SingleSelectionState {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut SingleSelectionState {
        &mut self.selection
    }

    pub fn selected_key(&self) -> &SelectedKey {
        self.selection.selected_key()
    }

    pub fn selected_key_str(&self) -> Option<&str> {
        self.selection.selected_key_str()
    }

    pub fn selected_index(&self) -> Option<usize> {
        let selected = self.selection.selected_key_str()?;
        self.items.iter().position(|k| k == selected)
    }

    pub fn select_by_index(&mut self, index: usize) {
        let Some(key) = self.items.get(index).cloned() else {
            return;
        };
        self.selection.set_selected_key(SelectedKey::Key(key));
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let next_index = match self.selected_index() {
            None => 0,
            Some(i) => {
                if i + 1 >= self.items.len() {
                    0
                } else {
                    i + 1
                }
            }
        };

        self.select_by_index(next_index);
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let prev_index = match self.selected_index() {
            None => self.items.len() - 1,
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
        };

        self.select_by_index(prev_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::{OnSingleSelectionChange, SelectedKey};
    use std::sync::{Arc, Mutex};

    fn keys(values: &[&str]) -> Vec<Key> {
        values.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn uncontrolled_select_next_and_prev_wraps() {
        let mut state = use_list_state(ListStateOptions {
            items: keys(&["a", "b", "c"]),
            selection: SingleSelectionStateOptions::default(),
        });

        assert_eq!(state.selected_key_str(), None);

        state.select_next();
        assert_eq!(state.selected_key_str(), Some("a"));

        state.select_next();
        assert_eq!(state.selected_key_str(), Some("b"));

        state.select_prev();
        assert_eq!(state.selected_key_str(), Some("a"));

        state.select_prev();
        assert_eq!(state.selected_key_str(), Some("c"));
    }

    #[test]
    fn controlled_selection_does_not_update_internal() {
        let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
        let called2 = Arc::clone(&called);
        let on_selection_change: OnSingleSelectionChange =
            Arc::new(move |v| *called2.lock().unwrap() = Some(v));

        let mut state = use_list_state(ListStateOptions {
            items: keys(&["a", "b"]),
            selection: SingleSelectionStateOptions {
                selected_key: Some(SelectedKey::key("a")),
                on_selection_change: Some(on_selection_change),
                ..Default::default()
            },
        });

        state.select_next();
        assert_eq!(*called.lock().unwrap(), Some(SelectedKey::key("b")));

        // Still controlled by input until synced.
        assert_eq!(state.selected_key_str(), Some("a"));
    }
}
