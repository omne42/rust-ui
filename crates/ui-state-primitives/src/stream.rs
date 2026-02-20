//! Streaming primitives that are platform-agnostic and recoverable.
//!
//! This module intentionally avoids any async runtime coupling. It models
//! monotonically checkpointed state so callers can resume and verify updates.

pub type UiCheckpoint = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiStreamError {
    NonMonotonicCheckpoint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiStreamSnapshot<T> {
    pub checkpoint: UiCheckpoint,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiStream<T> {
    checkpoint: UiCheckpoint,
    value: Option<T>,
}

impl<T> Default for UiStream<T> {
    fn default() -> Self {
        Self {
            checkpoint: 0,
            value: None,
        }
    }
}

impl<T> UiStream<T> {
    pub fn checkpoint(&self) -> UiCheckpoint {
        self.checkpoint
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn snapshot(&self) -> Option<UiStreamSnapshot<&T>> {
        self.value.as_ref().map(|value| UiStreamSnapshot {
            checkpoint: self.checkpoint,
            value,
        })
    }

    pub fn resume(snapshot: UiStreamSnapshot<T>) -> Self {
        Self {
            checkpoint: snapshot.checkpoint,
            value: Some(snapshot.value),
        }
    }

    pub fn apply(&mut self, snapshot: UiStreamSnapshot<T>) -> Result<(), UiStreamError> {
        if snapshot.checkpoint < self.checkpoint {
            return Err(UiStreamError::NonMonotonicCheckpoint);
        }

        self.checkpoint = snapshot.checkpoint;
        self.value = Some(snapshot.value);
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/stream.rs"]
mod tests;
