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
#[path = "test/list.rs"]
mod tests;
