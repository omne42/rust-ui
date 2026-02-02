use crate::controlled::{
    use_controlled_state, ControlledOnChange, ControlledState, ControlledStateOptions,
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
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn single_uncontrolled_updates_internal_state() {
        let mut state = use_single_selection_state(SingleSelectionStateOptions {
            default_selected_key: Some(SelectedKey::none()),
            ..Default::default()
        });

        assert!(matches!(state.selected_key(), SelectedKey::None));
        state.set_selected_key(SelectedKey::key("a"));
        assert_eq!(state.selected_key_str(), Some("a"));
        assert!(state.is_selected("a"));
    }

    #[test]
    fn single_controlled_calls_on_change_but_does_not_update_internal() {
        let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
        let called2 = Arc::clone(&called);

        let mut state = use_single_selection_state(SingleSelectionStateOptions {
            selected_key: Some(SelectedKey::key("a")),
            on_selection_change: Some(Arc::new(move |v| *called2.lock().unwrap() = Some(v))),
            ..Default::default()
        });

        state.set_selected_key(SelectedKey::key("b"));
        assert_eq!(*called.lock().unwrap(), Some(SelectedKey::key("b")));
        assert_eq!(state.selected_key_str(), Some("a"));

        state.sync_controlled(Some(SelectedKey::key("b")));
        assert_eq!(state.selected_key_str(), Some("b"));
    }

    #[test]
    fn single_controlled_none_is_representable() {
        let called: Arc<Mutex<Option<SelectedKey>>> = Arc::new(Mutex::new(None));
        let called2 = Arc::clone(&called);

        let mut state = use_single_selection_state(SingleSelectionStateOptions {
            selected_key: Some(SelectedKey::none()),
            on_selection_change: Some(Arc::new(move |v| *called2.lock().unwrap() = Some(v))),
            ..Default::default()
        });

        assert_eq!(state.selected_key_str(), None);
        state.set_selected_key(SelectedKey::key("a"));
        assert_eq!(*called.lock().unwrap(), Some(SelectedKey::key("a")));
        assert_eq!(state.selected_key_str(), None);
    }

    #[test]
    fn multiple_uncontrolled_updates_internal_state() {
        let mut state = use_multiple_selection_state(MultipleSelectionStateOptions::default());

        assert!(!state.is_selected("a"));
        state.insert("a");
        assert!(state.is_selected("a"));
        state.remove("a");
        assert!(!state.is_selected("a"));
    }

    #[test]
    fn multiple_controlled_calls_on_change_but_does_not_update_internal() {
        let called: Arc<Mutex<Option<BTreeSet<Key>>>> = Arc::new(Mutex::new(None));
        let called2 = Arc::clone(&called);

        let mut initial = BTreeSet::new();
        initial.insert("a".to_string());

        let mut state = use_multiple_selection_state(MultipleSelectionStateOptions {
            selected_keys: Some(initial.clone()),
            on_selection_change: Some(Arc::new(move |v| *called2.lock().unwrap() = Some(v))),
            ..Default::default()
        });

        state.insert("b");
        let called_value = called.lock().unwrap().clone().unwrap();
        assert!(called_value.contains("a"));
        assert!(called_value.contains("b"));

        // Internal state remains the controlled input until synced.
        assert!(state.is_selected("a"));
        assert!(!state.is_selected("b"));

        let mut next = BTreeSet::new();
        next.insert("b".to_string());
        state.sync_controlled(Some(next));
        assert!(!state.is_selected("a"));
        assert!(state.is_selected("b"));
    }
}
