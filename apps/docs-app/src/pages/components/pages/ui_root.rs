use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;

pub(super) fn ui_root() -> AnyView {
    let usage_code = Signal::derive(move || {
        r#"use ui_components::{UiRoot, Theme};

let theme = Signal::derive(|| Theme::dark());

<UiRoot theme=theme safe_area=true inject_components_css=true>
  // your app
</UiRoot>"#
            .to_string()
    });

    let contract_code = Signal::derive(move || {
        r#"<UiRoot ...>
  // wrapper attrs:
  // data-slot="ui-root"
  // data-theme-system="spectrum|express|spectrum-two"
  // data-theme-color="light|dark|oled"
  // data-theme-scale="medium|large"
  // data-theme-scheme="light|dark" (CSS color-scheme)
  // data-state="default|safe-area"
  // data-safe-area="true" (optional)
</UiRoot>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="UiRoot"
            slug="ui-root"
            group="Layout"
            description="Provider that injects theme tokens + layered component CSS and exposes stable root state attrs."
        >
            <Playground title="Usage" code_signal=usage_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">
                        "This docs app already mounts a global UiRoot at startup."
                    </div>
                    <div class="docs-ui-root-note">
                        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place."
                    </div>
                    <div class="ui-muted">
                        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells."
                    </div>
                </div>
            </Playground>

            <Playground title="State Contract" code_signal=contract_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">"`data-slot=ui-root` for stable root targeting."</div>
                    <div class="docs-ui-root-note">"`data-theme-system/color/scale` mirror the current ThemeContext axes."</div>
                    <div class="docs-ui-root-note">"`data-theme-scheme` mirrors the CSS `color-scheme` value (`light`/`dark`)."</div>
                    <div class="docs-ui-root-note">"`data-state` + `data-safe-area` describe safe-area mode."</div>
                    <div class="ui-muted">"Use these attrs to write app-level overrides without coupling to internal implementation details."</div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
