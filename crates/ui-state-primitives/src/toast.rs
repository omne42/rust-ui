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
#[path = "test/toast.rs"]
mod tests;
