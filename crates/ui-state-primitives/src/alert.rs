#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateCoreInput {
    pub has_title: bool,
    pub has_description: bool,
    pub has_actions: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlertStateCore {
    pub has_title: bool,
    pub title_attr: &'static str,
    pub has_description: bool,
    pub description_attr: &'static str,
    pub has_actions: bool,
    pub actions_attr: &'static str,
    pub state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn resolve_state_core(input: AlertStateCoreInput) -> AlertStateCore {
    let title_attr = if input.has_title { "present" } else { "absent" };
    let description_attr = if input.has_description {
        "present"
    } else {
        "absent"
    };
    let actions_attr = if input.has_actions {
        "present"
    } else {
        "absent"
    };
    let state_attr = if input.has_title && input.has_description {
        "detailed"
    } else {
        "compact"
    };
    AlertStateCore {
        has_title: input.has_title,
        title_attr,
        has_description: input.has_description,
        description_attr,
        has_actions: input.has_actions,
        actions_attr,
        state_attr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_optional_text_trims_and_filters_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some(" docs-alert ".to_string())),
            Some("docs-alert".to_string())
        );
    }

    #[test]
    fn resolve_state_core_tracks_content_flags() {
        let detailed = resolve_state_core(AlertStateCoreInput {
            has_title: true,
            has_description: true,
            has_actions: false,
        });
        assert!(detailed.has_title);
        assert_eq!(detailed.title_attr, "present");
        assert!(detailed.has_description);
        assert_eq!(detailed.description_attr, "present");
        assert!(!detailed.has_actions);
        assert_eq!(detailed.actions_attr, "absent");
        assert_eq!(detailed.state_attr, "detailed");

        let compact = resolve_state_core(AlertStateCoreInput {
            has_title: false,
            has_description: true,
            has_actions: true,
        });
        assert!(!compact.has_title);
        assert_eq!(compact.title_attr, "absent");
        assert!(compact.has_description);
        assert_eq!(compact.description_attr, "present");
        assert!(compact.has_actions);
        assert_eq!(compact.actions_attr, "present");
        assert_eq!(compact.state_attr, "compact");
    }
}
