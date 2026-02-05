#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileTriggerFile {
    pub name: String,
    pub size: u64,
    pub mime: String,
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn collect_files_from_input(
    input: &leptos::web_sys::HtmlInputElement,
) -> Vec<FileTriggerFile> {
    let Some(files) = input.files() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for idx in 0..files.length() {
        let Some(file) = files.get(idx) else {
            continue;
        };
        out.push(FileTriggerFile {
            name: file.name(),
            size: file.size().max(0.0) as u64,
            mime: file.type_(),
        });
    }
    out
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn collect_files_from_input(
    _input: &leptos::web_sys::HtmlInputElement,
) -> Vec<FileTriggerFile> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_type_is_send_sync_friendly() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FileTriggerFile>();
    }
}
