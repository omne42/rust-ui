use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AiRenderMode {
    #[default]
    Snapshot,
    Streaming,
}

impl AiRenderMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Streaming => "streaming",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AiOutputStatus {
    Draft,
    #[default]
    Verified,
    Submittable,
}

impl AiOutputStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Verified => "verified",
            Self::Submittable => "submittable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AiSpaceState {
    pub mode: AiRenderMode,
    pub output_status: AiOutputStatus,
}

pub type AiSpaceStateSignal = Signal<AiSpaceState>;

pub fn use_ai_space_state() -> Option<AiSpaceStateSignal> {
    use_context::<AiSpaceStateSignal>()
}

#[component]
pub fn AiSpace(
    children: Children,
    #[prop(into)] mode: Signal<AiRenderMode>,
    #[prop(into)] output_status: Signal<AiOutputStatus>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("ui-ai-space {value}"))
        .unwrap_or_else(|| "ui-ai-space".to_string());

    let state = Signal::derive(move || AiSpaceState {
        mode: mode.get(),
        output_status: output_status.get(),
    });
    provide_context(state);

    view! {
        <section
            class=class
            data-slot="ai-space"
            data-ui-ai-space="true"
            data-ui-stream-mode=move || mode.get().as_str()
            data-ui-output-status=move || output_status.get().as_str()
        >
            {children()}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_render_mode_attr_values_are_stable() {
        assert_eq!(AiRenderMode::Snapshot.as_str(), "snapshot");
        assert_eq!(AiRenderMode::Streaming.as_str(), "streaming");
    }

    #[test]
    fn ai_output_status_attr_values_are_stable() {
        assert_eq!(AiOutputStatus::Draft.as_str(), "draft");
        assert_eq!(AiOutputStatus::Verified.as_str(), "verified");
        assert_eq!(AiOutputStatus::Submittable.as_str(), "submittable");
    }
}
