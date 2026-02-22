use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{SemanticOverrides, Theme, UiRoot};
use ui_headless::UiI18n;

pub(super) fn ui_root() -> AnyView {
    let (workbench_dark, set_workbench_dark) = signal(false);
    let (workbench_inject_css, set_workbench_inject_css) = signal(true);
    let (workbench_safe_area, set_workbench_safe_area) = signal(false);
    let (workbench_semantic_overrides, set_workbench_semantic_overrides) = signal(false);
    let (workbench_custom_i18n, set_workbench_custom_i18n) = signal(false);

    let workbench_theme = Signal::derive(move || {
        if workbench_dark.get() {
            Theme::dark()
        } else {
            Theme::light()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"let theme = Signal::derive(|| Theme::light());

<UiRoot theme=theme inject_components_css=true safe_area=false>
  <div>"App shell content"</div>
</UiRoot>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<UiRoot\n  theme=Signal::derive(|| Theme::{})\n  inject_components_css={}\n  safe_area={}\n  semantic_overrides={}\n  i18n={}\n>\n  <div>\"Workbench app shell\"</div>\n</UiRoot>",
            if workbench_dark.get() {
                "dark()"
            } else {
                "light()"
            },
            workbench_inject_css.get(),
            workbench_safe_area.get(),
            if workbench_semantic_overrides.get() {
                "Some(SemanticOverrides::default())"
            } else {
                "None"
            },
            "UiI18n::default()",
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "UiRootActualConfig {{\n  theme: {:?},\n  inject_components_css: {},\n  safe_area: {},\n  semantic_overrides: {},\n  i18n: {},\n}}",
            if workbench_dark.get() {
                "dark"
            } else {
                "light"
            },
            workbench_inject_css.get(),
            workbench_safe_area.get(),
            workbench_semantic_overrides.get(),
            workbench_custom_i18n.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"let light_theme = Signal::derive(|| Theme::light());
let dark_theme = Signal::derive(|| Theme::dark());

<UiRoot
  theme=light_theme
  inject_components_css=true
  safe_area=false
  semantic_overrides=None
  i18n=UiI18n::default()
>
  <div>"Light shell"</div>
</UiRoot>
<UiRoot
  theme=dark_theme
  inject_components_css=true
  safe_area=true
  semantic_overrides=Some(SemanticOverrides::default())
  i18n=UiI18n::default()
>
  <div>"Dark safe-area shell"</div>
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
            <Playground title="Default Showcase" code_signal=showcase_code>
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

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="ui-root-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_dark.get()
                                on:change=move |ev| set_workbench_dark.set(event_target_checked(&ev))
                            />
                            " theme=dark"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_inject_css.get()
                                on:change=move |ev| set_workbench_inject_css.set(event_target_checked(&ev))
                            />
                            " inject_components_css"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_safe_area.get()
                                on:change=move |ev| set_workbench_safe_area.set(event_target_checked(&ev))
                            />
                            " safe_area"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_semantic_overrides.get()
                                on:change=move |ev| set_workbench_semantic_overrides.set(event_target_checked(&ev))
                            />
                            " semantic_overrides"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_i18n.get()
                                on:change=move |ev| set_workbench_custom_i18n.set(event_target_checked(&ev))
                            />
                            " i18n"
                        </label>
                    </div>
                }
            >
                {move || {
                    if workbench_semantic_overrides.get() {
                        view! {
                            <UiRoot
                                theme=workbench_theme
                                inject_components_css=workbench_inject_css.get()
                                safe_area=workbench_safe_area.get()
                                semantic_overrides=SemanticOverrides::default()
                                i18n=UiI18n::default()
                            >
                                <div class="docs-stack">
                                    <div class="docs-ui-root-note">"Workbench content inside UiRoot"</div>
                                </div>
                            </UiRoot>
                        }
                        .into_any()
                    } else {
                        view! {
                            <UiRoot
                                theme=workbench_theme
                                inject_components_css=workbench_inject_css.get()
                                safe_area=workbench_safe_area.get()
                                i18n=UiI18n::default()
                            >
                                <div class="docs-stack">
                                    <div class="docs-ui-root-note">"Workbench content inside UiRoot"</div>
                                </div>
                            </UiRoot>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Theme / Safe Area / Overrides Comparison)" code_signal=matrix_code>
                <div class="docs-stack">
                    <div class="docs-ui-root-note">"`data-slot=ui-root` for stable root targeting."</div>
                    <div class="docs-ui-root-note">"`data-theme-system/color/scale` mirror Theme axes."</div>
                    <div class="docs-ui-root-note">"`data-state` + `data-safe-area` describe safe-area mode."</div>
                    <div class="docs-ui-root-note">"semantic_overrides + i18n are passed at root boundary."</div>
                    <div class="ui-muted">"Use state attrs for shell-level styling without coupling internals."</div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
