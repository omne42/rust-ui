#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DroppedFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

#[cfg(target_arch = "wasm32")]
pub fn collect_files_from_drag_event(ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
    let Some(dt) = ev.data_transfer() else {
        return Vec::new();
    };
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

#[cfg(not(target_arch = "wasm32"))]
pub fn collect_files_from_drag_event(_ev: &leptos::ev::DragEvent) -> Vec<DroppedFile> {
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
}
