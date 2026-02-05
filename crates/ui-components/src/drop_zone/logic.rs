#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DroppedFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DragDepth {
    depth: usize,
}

impl DragDepth {
    pub fn enter(self) -> Self {
        Self {
            depth: self.depth.saturating_add(1),
        }
    }

    pub fn leave(self) -> Self {
        Self {
            depth: self.depth.saturating_sub(1),
        }
    }

    pub fn reset(self) -> Self {
        Self { depth: 0 }
    }

    pub fn is_active(self) -> bool {
        self.depth > 0
    }
}

#[cfg(target_arch = "wasm32")]
fn collect_files_from_data_transfer(dt: &leptos::web_sys::DataTransfer) -> Vec<DroppedFile> {
    let Some(files) = dt.files() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for idx in 0..files.length() {
        let Some(file) = files.get(idx) else {
            continue;
        };
        out.push(DroppedFile {
            name: file.name(),
            size: file.size().max(0.0) as u64,
            mime: file.type_(),
        });
    }
    out
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_drag_event(ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
    let Some(dt) = ev.data_transfer() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_clipboard_event(
    ev: &leptos::ev::ClipboardEvent,
) -> Vec<DroppedFile> {
    let Some(dt) = ev.clipboard_data() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_drag_event(_ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_clipboard_event(
    _ev: &leptos::ev::ClipboardEvent,
) -> Vec<DroppedFile> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropped_file_is_send_sync_friendly() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<DroppedFile>();
    }

    #[test]
    fn drag_depth_tracks_nested_drag_enter_leave_without_flicker() {
        let state = DragDepth::default();
        assert!(!state.is_active());

        let state = state.enter();
        assert!(state.is_active());

        let state = state.enter();
        assert!(state.is_active());

        let state = state.leave();
        assert!(state.is_active());

        let state = state.leave();
        assert!(!state.is_active());

        let state = state.leave();
        assert!(!state.is_active());
    }

    #[test]
    fn drag_depth_reset_always_clears() {
        let state = DragDepth::default().enter().enter();
        assert!(state.is_active());

        let state = state.reset();
        assert!(!state.is_active());
    }
}
