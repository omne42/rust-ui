use crate::controlled::{
    ControlledOnChange, ControlledState, ControlledStateOptions, use_controlled_state,
};
use std::collections::BTreeSet;

pub type Key = String;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SelectedKey {
    #[default]
    None,
    Key(Key),
}

impl SelectedKey {
    pub fn none() -> Self {
        Self::None
    }

    pub fn key(key: impl Into<Key>) -> Self {
        Self::Key(key.into())
    }

    pub fn as_deref(&self) -> Option<&str> {
        match self {
            SelectedKey::None => None,
            SelectedKey::Key(key) => Some(key.as_str()),
        }
    }

    pub fn is_selected(&self, key: &str) -> bool {
        match self {
            SelectedKey::None => false,
            SelectedKey::Key(selected) => selected == key,
        }
    }
}

pub type OnSingleSelectionChange = ControlledOnChange<SelectedKey>;

#[derive(Clone)]
pub struct SingleSelectionState {
    selected_key: ControlledState<SelectedKey>,
}

#[derive(Clone, Default)]
pub struct SingleSelectionStateOptions {
    pub selected_key: Option<SelectedKey>,
    pub default_selected_key: Option<SelectedKey>,
    pub on_selection_change: Option<OnSingleSelectionChange>,
}

pub fn use_single_selection_state(options: SingleSelectionStateOptions) -> SingleSelectionState {
    SingleSelectionState {
        selected_key: use_controlled_state(
            SelectedKey::None,
            ControlledStateOptions {
                value: options.selected_key,
                default_value: options.default_selected_key,
                on_change: options.on_selection_change,
            },
        ),
    }
}

impl SingleSelectionState {
    pub fn selected_key(&self) -> &SelectedKey {
        self.selected_key.value()
    }

    pub fn selected_key_str(&self) -> Option<&str> {
        self.selected_key.value().as_deref()
    }

    pub fn default_selected_key(&self) -> &SelectedKey {
        self.selected_key.default_value()
    }

    pub fn is_controlled(&self) -> bool {
        self.selected_key.is_controlled()
    }

    pub fn sync_controlled(&mut self, selected_key: Option<SelectedKey>) {
        self.selected_key.sync_controlled(selected_key);
    }

    pub fn is_selected(&self, key: &str) -> bool {
        self.selected_key.value().is_selected(key)
    }

    pub fn clear(&mut self) {
        self.set_selected_key(SelectedKey::None);
    }

    pub fn set_selected_key(&mut self, selected_key: SelectedKey) {
        self.selected_key.set_value(selected_key);
    }
}

pub type OnMultipleSelectionChange = ControlledOnChange<BTreeSet<Key>>;

#[derive(Clone)]
pub struct MultipleSelectionState {
    selected_keys: ControlledState<BTreeSet<Key>>,
}

#[derive(Clone, Default)]
pub struct MultipleSelectionStateOptions {
    pub selected_keys: Option<BTreeSet<Key>>,
    pub default_selected_keys: Option<BTreeSet<Key>>,
    pub on_selection_change: Option<OnMultipleSelectionChange>,
}

pub fn use_multiple_selection_state(
    options: MultipleSelectionStateOptions,
) -> MultipleSelectionState {
    MultipleSelectionState {
        selected_keys: use_controlled_state(
            BTreeSet::new(),
            ControlledStateOptions {
                value: options.selected_keys,
                default_value: options.default_selected_keys,
                on_change: options.on_selection_change,
            },
        ),
    }
}

impl MultipleSelectionState {
    pub fn selected_keys(&self) -> &BTreeSet<Key> {
        self.selected_keys.value()
    }

    pub fn default_selected_keys(&self) -> &BTreeSet<Key> {
        self.selected_keys.default_value()
    }

    pub fn is_controlled(&self) -> bool {
        self.selected_keys.is_controlled()
    }

    pub fn sync_controlled(&mut self, selected_keys: Option<BTreeSet<Key>>) {
        self.selected_keys.sync_controlled(selected_keys);
    }

    pub fn is_selected(&self, key: &str) -> bool {
        self.selected_keys.value().contains(key)
    }

    pub fn clear(&mut self) {
        self.set_selected_keys(BTreeSet::new());
    }

    pub fn set_selected_keys(&mut self, selected_keys: BTreeSet<Key>) {
        self.selected_keys.set_value(selected_keys);
    }

    pub fn insert(&mut self, key: impl Into<Key>) {
        let key = key.into();
        let mut next = self.selected_keys.value().clone();
        next.insert(key);
        self.selected_keys.set_value(next);
    }

    pub fn remove(&mut self, key: &str) {
        let mut next = self.selected_keys.value().clone();
        next.remove(key);
        self.selected_keys.set_value(next);
    }

    pub fn toggle(&mut self, key: impl Into<Key>) {
        let key = key.into();
        let mut next = self.selected_keys.value().clone();
        if next.contains(&key) {
            next.remove(&key);
        } else {
            next.insert(key);
        }
        self.selected_keys.set_value(next);
    }
}

#[cfg(test)]
#[path = "test/selection.rs"]
mod tests;
