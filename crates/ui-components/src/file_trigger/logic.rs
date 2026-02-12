#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerStateInput {
    pub disabled: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileTriggerState {
    pub is_disabled: bool,
    pub is_enabled: bool,
    pub state_attr: &'static str,
    pub motion_source_attr: &'static str,
    pub has_custom_motion: bool,
}

pub fn resolve_state(input: FileTriggerStateInput) -> FileTriggerState {
    FileTriggerState {
        is_disabled: input.disabled,
        is_enabled: !input.disabled,
        state_attr: if input.disabled { "disabled" } else { "ready" },
        motion_source_attr: if input.has_custom_motion {
            "custom"
        } else {
            "default"
        },
        has_custom_motion: input.has_custom_motion,
    }
}

pub fn compose_class_name(state: FileTriggerState) -> String {
    let mut classes = vec!["ui-file-trigger".to_string()];

    if state.is_disabled {
        classes.push("ui-file-trigger--disabled".to_string());
    }

    if state.has_custom_motion {
        classes.push("ui-file-trigger--custom-motion".to_string());
    }

    classes.join(" ")
}

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

    #[test]
    fn resolve_state_tracks_disabled_and_motion_sources() {
        let default_state = resolve_state(FileTriggerStateInput {
            disabled: false,
            has_custom_motion: false,
        });

        assert!(default_state.is_enabled);
        assert!(!default_state.is_disabled);
        assert_eq!(default_state.state_attr, "ready");
        assert_eq!(default_state.motion_source_attr, "default");
        assert!(!default_state.has_custom_motion);

        let custom_state = resolve_state(FileTriggerStateInput {
            disabled: true,
            has_custom_motion: true,
        });

        assert!(!custom_state.is_enabled);
        assert!(custom_state.is_disabled);
        assert_eq!(custom_state.state_attr, "disabled");
        assert_eq!(custom_state.motion_source_attr, "custom");
        assert!(custom_state.has_custom_motion);
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(resolve_state(FileTriggerStateInput {
            disabled: true,
            has_custom_motion: true,
        }));

        for token in [
            "ui-file-trigger",
            "ui-file-trigger--disabled",
            "ui-file-trigger--custom-motion",
        ] {
            assert!(
                class_name.contains(token),
                "FileTrigger class name should include `{token}`."
            );
        }
    }
}
