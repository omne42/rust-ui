use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{ActionBar, ActionBarMotion, ActionBarPosition, ActionButton};

pub(super) fn action_bar() -> AnyView {
    let (selected_count, set_selected_count) = signal(0_usize);
    let selected_count_signal = Signal::derive(move || selected_count.get());

    let clear_selection = Callback::new(move |_| set_selected_count.set(0));

    let code = r#"let (selected_count, set_selected_count) = signal(3_usize);
let selected_count_signal = Signal::derive(move || selected_count.get());

<ActionBar
  selected_count=selected_count_signal
  on_clear_selection=Callback::new(move |_| set_selected_count.set(0))
>
  <ActionButton>"Delete"</ActionButton>
  <ActionButton is_quiet=true>"Archive"</ActionButton>
</ActionBar>"#;

    let state_code = r#"<ActionBar
  selected_count=Signal::derive(move || selected_count.get())
  position=ActionBarPosition::Top
  force_visible=true
  selection_text="Rows selected".to_string()
  clear_label="Clear all".to_string()
  motion=ActionBarMotion::disabled()
/>"#;

    view! {
        <ComponentPage
            title="ActionBar"
            slug="action-bar"
            group="Actions"
            description="Bulk-action surface with Spectrum-style selection contracts and HeroUI-grade spring visibility motion."
        >
            <Playground title="Selection + clear action" code=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Outline
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui_components::Button>
                        <span class="ui-muted">
                            "selected: " {move || selected_count.get().to_string()}
                        </span>
                    </div>

                    <ActionBar
                        selected_count=selected_count_signal
                        on_clear_selection=clear_selection
                        aria_label="Bulk actions".to_string()
                        class_name="docs-action-bar".to_string()
                    >
                        <ActionButton>"Delete"</ActionButton>
                        <ActionButton is_quiet=true>"Archive"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground title="Top placement + custom text + reduced motion" code=state_code>
                <div class="docs-stack">
                    <ActionBar
                        selected_count=selected_count_signal
                        position=ActionBarPosition::Top
                        force_visible=true
                        selection_text="Rows selected".to_string()
                        clear_label="Clear all".to_string()
                        motion=ActionBarMotion::disabled()
                    >
                        <ActionButton is_quiet=true>"Tag"</ActionButton>
                        <ActionButton is_quiet=true>"Assign"</ActionButton>
                    </ActionBar>
                    <span class="ui-muted">
                        "Top placement + custom labels + motion disabled."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
