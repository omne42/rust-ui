use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::VisuallyHidden;

pub(super) fn visually_hidden() -> AnyView {
    let icon_code = Signal::derive(move || {
        r##"<button type="button" aria-haspopup="menu">
  <span aria-hidden="true">⚙</span>
  <VisuallyHidden>"Open account settings"</VisuallyHidden>
</button>"##
            .to_string()
    });

    let focusable_code = Signal::derive(move || {
        r##"<VisuallyHidden focusable=true>
  <a href="#docs-visually-hidden-target">"Skip to details"</a>
</VisuallyHidden>
<div id="docs-visually-hidden-target" tabindex="-1">"Details section"</div>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="VisuallyHidden"
            slug="visually-hidden"
            group="Forms"
            description="@react-aria/visually-hidden compatible utility for screen-reader-only content and focusable skip-link workflows."
        >
            <Playground title="Icon Button Accessible Label" code_signal=icon_code>
                <div class="docs-stack">
                    <button type="button" aria-haspopup="menu">
                        <span aria-hidden="true">"⚙"</span>
                        <VisuallyHidden>"Open account settings"</VisuallyHidden>
                    </button>
                    <p>
                        "The icon stays visual-only while the hidden text provides an accessible name."
                    </p>
                </div>
            </Playground>

            <Playground title="Focusable Skip Link" code_signal=focusable_code>
                <div class="docs-stack">
                    <VisuallyHidden focusable=true>
                        <a href="#docs-visually-hidden-target">"Skip to details"</a>
                    </VisuallyHidden>
                    <p>
                        "Use keyboard Tab to reveal and focus the skip link."
                    </p>
                    <div id="docs-visually-hidden-target" tabindex="-1">
                        "Details section"
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
