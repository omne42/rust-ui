pub const DEFAULT_MAX_TOASTS: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastRecord<T> {
    pub id: String,
    pub payload: T,
    pub is_open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastStateOptions {
    pub max_toasts: usize,
}

impl Default for ToastStateOptions {
    fn default() -> Self {
        Self {
            max_toasts: DEFAULT_MAX_TOASTS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastMutationKind {
    Pushed,
    OverflowClosed,
    Dismissed,
    Cleared,
    Removed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastMutation {
    pub id: String,
    pub kind: ToastMutationKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastState<T> {
    max_toasts: usize,
    toasts: Vec<ToastRecord<T>>,
}

pub fn normalize_max_toasts(max_toasts: usize) -> usize {
    max_toasts.max(1)
}

impl<T> ToastState<T> {
    pub fn new(options: ToastStateOptions) -> Self {
        Self {
            max_toasts: normalize_max_toasts(options.max_toasts),
            toasts: Vec::new(),
        }
    }

    pub fn from_records(options: ToastStateOptions, records: Vec<ToastRecord<T>>) -> Self {
        Self {
            max_toasts: normalize_max_toasts(options.max_toasts),
            toasts: records,
        }
    }

    pub fn max_toasts(&self) -> usize {
        self.max_toasts
    }

    pub fn toasts(&self) -> &[ToastRecord<T>] {
        &self.toasts
    }

    pub fn into_records(self) -> Vec<ToastRecord<T>> {
        self.toasts
    }

    pub fn push(&mut self, id: String, payload: T) -> Vec<ToastMutation> {
        let mut mutations = vec![ToastMutation {
            id: id.clone(),
            kind: ToastMutationKind::Pushed,
        }];

        self.toasts.push(ToastRecord {
            id,
            payload,
            is_open: true,
        });

        let overflow = self.toasts.len().saturating_sub(self.max_toasts);
        for _ in 0..overflow {
            if self.toasts.is_empty() {
                break;
            }

            if self.toasts[0].is_open {
                self.toasts[0].is_open = false;
                mutations.push(ToastMutation {
                    id: self.toasts[0].id.clone(),
                    kind: ToastMutationKind::OverflowClosed,
                });
            }

            let oldest = self.toasts.remove(0);
            self.toasts.push(oldest);
        }

        mutations
    }

    pub fn dismiss(&mut self, id: &str) -> Option<ToastMutation> {
        let id = id.trim();
        if id.is_empty() {
            return None;
        }

        let toast = self.toasts.iter_mut().find(|toast| toast.id == id)?;
        if !toast.is_open {
            return None;
        }

        toast.is_open = false;
        Some(ToastMutation {
            id: toast.id.clone(),
            kind: ToastMutationKind::Dismissed,
        })
    }

    pub fn clear(&mut self) -> Vec<ToastMutation> {
        let mut mutations = Vec::new();

        for toast in &mut self.toasts {
            if !toast.is_open {
                continue;
            }

            toast.is_open = false;
            mutations.push(ToastMutation {
                id: toast.id.clone(),
                kind: ToastMutationKind::Cleared,
            });
        }

        mutations
    }

    pub fn remove(&mut self, id: &str) -> Option<ToastMutation> {
        let id = id.trim();
        if id.is_empty() {
            return None;
        }

        let index = self.toasts.iter().position(|toast| toast.id == id)?;
        let removed = self.toasts.remove(index);

        Some(ToastMutation {
            id: removed.id,
            kind: ToastMutationKind::Removed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(text: &str) -> String {
        text.to_string()
    }

    #[test]
    fn max_toasts_is_normalized_to_one() {
        let state = ToastState::<String>::new(ToastStateOptions { max_toasts: 0 });
        assert_eq!(state.max_toasts(), 1);
    }

    #[test]
    fn push_overflow_closes_oldest_and_rotates_to_end() {
        let mut state = ToastState::new(ToastStateOptions { max_toasts: 2 });

        state.push("one".to_string(), payload("One"));
        state.push("two".to_string(), payload("Two"));
        let mutations = state.push("three".to_string(), payload("Three"));

        assert_eq!(state.toasts().len(), 3);
        assert_eq!(state.toasts()[0].id, "two");
        assert_eq!(state.toasts()[1].id, "three");
        assert_eq!(state.toasts()[2].id, "one");

        assert!(state.toasts()[0].is_open);
        assert!(state.toasts()[1].is_open);
        assert!(!state.toasts()[2].is_open);

        assert!(
            mutations
                .iter()
                .any(|m| { m.id == "three" && m.kind == ToastMutationKind::Pushed })
        );
        assert!(
            mutations
                .iter()
                .any(|m| { m.id == "one" && m.kind == ToastMutationKind::OverflowClosed })
        );
    }

    #[test]
    fn dismiss_closes_open_toast_once() {
        let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
        state.push("one".to_string(), payload("One"));

        let first = state.dismiss("one");
        let second = state.dismiss("one");

        assert_eq!(
            first,
            Some(ToastMutation {
                id: "one".to_string(),
                kind: ToastMutationKind::Dismissed,
            })
        );
        assert_eq!(second, None);
    }

    #[test]
    fn clear_closes_only_open_toasts() {
        let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
        state.push("one".to_string(), payload("One"));
        state.push("two".to_string(), payload("Two"));
        state.dismiss("one");

        let mutations = state.clear();

        assert_eq!(mutations.len(), 1);
        assert_eq!(mutations[0].id, "two");
        assert_eq!(mutations[0].kind, ToastMutationKind::Cleared);
    }

    #[test]
    fn remove_drops_toast_by_id() {
        let mut state = ToastState::new(ToastStateOptions { max_toasts: 3 });
        state.push("one".to_string(), payload("One"));
        state.push("two".to_string(), payload("Two"));

        let removed = state.remove("one");

        assert_eq!(
            removed,
            Some(ToastMutation {
                id: "one".to_string(),
                kind: ToastMutationKind::Removed,
            })
        );
        assert_eq!(state.toasts().len(), 1);
        assert_eq!(state.toasts()[0].id, "two");
    }
}
