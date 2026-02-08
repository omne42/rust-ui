use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    EmptyState, EmptyStateAlign, EmptyStateTone, Icon, IconSize, IconTone, Keyboard, KeyboardTone,
    LabeledValue, LabeledValueOrientation, LabeledValueTone, Text, TextAlign, TextElement,
    TextTone, TextWeight,
};

pub(super) fn labeled_value() -> AnyView {
    let orientation_code = r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#;

    let custom_code = r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  description="Updated 2 minutes ago".to_string()
  aria_label="Build status".to_string()
  class_name="docs-labeled-value-custom".to_string()
  tone=LabeledValueTone::Strong
/>"#;

    view! {
        <ComponentPage
            title="LabeledValue"
            slug="labeled-value"
            group="Display"
            description="Label-value pair primitive with centralized orientation/tone/source state contracts and Spectrum-style data markers."
        >
            <Playground title="Orientation + Tone" code=orientation_code>
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground title="Description + Custom Aria/Class" code=custom_code>
                <div class="docs-stack">
                    <LabeledValue
                        label="Build".to_string()
                        value="passing".to_string()
                        description="Updated 2 minutes ago".to_string()
                        aria_label="Build status".to_string()
                        class_name="docs-labeled-value-custom".to_string()
                        tone=LabeledValueTone::Strong
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Default
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn keyboard() -> AnyView {
    let tone_code = r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>"#;

    let compact_code = r#"<Keyboard
  compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>"#;

    view! {
        <ComponentPage
            title="Keyboard"
            slug="keyboard"
            group="Display"
            description="Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts."
        >
            <Playground title="Tone" code=tone_code>
                <div class="docs-row">
                    <Keyboard>"⌘K"</Keyboard>
                    <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                </div>
            </Playground>

            <Playground title="Compact + Custom Aria/Class" code=compact_code>
                <div class="docs-row">
                    <Keyboard
                        compact=true
                        aria_label="Open command palette".to_string()
                        class_name="docs-keyboard-custom".to_string()
                    >
                        "Ctrl+Shift+P"
                    </Keyboard>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text() -> AnyView {
    let tone_code = r#"<Text text="Primary body copy".to_string() />
<Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
<Text text="Strong headline".to_string() tone=TextTone::Strong weight=TextWeight::Bold />"#;

    let states_code = r#"<Text
  text="Centered label".to_string()
  align=TextAlign::Center
  element=TextElement::Div
/>
<Text
  text="Long text that truncates when width is constrained by the container around it".to_string()
  truncate=true
  class_name="docs-text-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Text"
            slug="text"
            group="Display"
            description="Typography primitive with centralized tone/alignment/weight/source state contracts and Spectrum-style data markers."
        >
            <Playground title="Tone + Weight Matrix" code=tone_code>
                <div class="docs-stack">
                    <Text text="Primary body copy".to_string() />
                    <Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
                    <Text
                        text="Strong headline".to_string()
                        tone=TextTone::Strong
                        weight=TextWeight::Bold
                    />
                </div>
            </Playground>

            <Playground title="Alignment + Truncate + Element" code=states_code>
                <div class="docs-stack">
                    <Text
                        text="Centered label".to_string()
                        align=TextAlign::Center
                        weight=TextWeight::Semibold
                        element=TextElement::Div
                    />
                    <Text
                        text="Long text that truncates when width is constrained by the container around it".to_string()
                        truncate=true
                        class_name="docs-text-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn icon() -> AnyView {
    let matrix_code = r#"<Icon size=IconSize::Sm tone=IconTone::Default decorative=true>"✓"</Icon>
<Icon size=IconSize::Md tone=IconTone::Muted decorative=true>"⚙"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>"★"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>"⚠"</Icon>"#;

    let states_code = r#"<Icon
  size=IconSize::Md
  tone=IconTone::Accent
  decorative=false
  aria_label="Sync successful".to_string()
>
  "✓"
</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Muted
  disabled=true
  class_name="docs-icon-custom".to_string()
  decorative=true
>
  "⚙"
</Icon>"#;

    view! {
        <ComponentPage
            title="Icon"
            slug="icon"
            group="Display"
            description="Spectrum-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers."
        >
            <Playground title="Size + Tone Matrix" code=matrix_code>
                <div class="docs-row">
                    <Icon size=IconSize::Sm tone=IconTone::Default decorative=true>
                        "✓"
                    </Icon>
                    <Icon size=IconSize::Md tone=IconTone::Muted decorative=true>
                        "⚙"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Accent decorative=true>
                        "★"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Danger decorative=true>
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Accessible + Disabled + Custom Class" code=states_code>
                <div class="docs-row">
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        decorative=false
                        aria_label="Sync successful".to_string()
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Muted
                        disabled=true
                        class_name="docs-icon-custom".to_string()
                        decorative=true
                    >
                        "⚙"
                    </Icon>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn empty_state() -> AnyView {
    let tone_code = r#"<EmptyState
  title="No projects yet".to_string()
  description="Create your first project to unlock dashboards and team workflows.".to_string()
  tone=EmptyStateTone::Default
  icon=move || view! { <span>"📁"</span> }
  actions=move || view! {
    <ui_components::Button>"Create project"</ui_components::Button>
  }
/>
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>"#;

    let state_code = r#"<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  compact=true
  bordered=true
  class_name="docs-empty-state-custom".to_string()
  icon=move || view! { <span>"⏸"</span> }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Review approvals"
    </ui_components::Button>
  }
/>"#;

    view! {
        <ComponentPage
            title="EmptyState"
            slug="empty-state"
            group="Display"
            description="Spectrum/HeroUI-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers."
        >
            <Playground title="Tone + Alignment + Actions" code=tone_code>
                <div class="docs-stack">
                    <EmptyState
                        title="No projects yet".to_string()
                        description="Create your first project to unlock dashboards and team workflows.".to_string()
                        tone=EmptyStateTone::Default
                        icon=move || view! { <span>"📁"</span> }
                        actions=move || {
                            view! {
                                <ui_components::Button>
                                    "Create project"
                                </ui_components::Button>
                            }
                        }
                    />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                </div>
            </Playground>

            <Playground title="Compact + Bordered + Custom Class" code=state_code>
                <EmptyState
                    title="Deployments paused".to_string()
                    description="Approvals are required before resuming this environment.".to_string()
                    tone=EmptyStateTone::Accent
                    compact=true
                    bordered=true
                    class_name="docs-empty-state-custom".to_string()
                    icon=move || view! { <span>"⏸"</span> }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Review approvals"
                            </ui_components::Button>
                        }
                    }
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
