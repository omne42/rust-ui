use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{SegmentedControl, SegmentedControlSize, Switch, VisuallyHidden};
use ui_headless::A11yDirection;

pub(super) fn visually_hidden() -> AnyView {
    let focus_mode_options = vec![
        "default".to_string(),
        "is_focusable".to_string(),
        "focusable".to_string(),
    ];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (focus_mode_index, set_focus_mode_index) = signal(Some(0_usize));
    let (lang_index, set_lang_index) = signal(Some(0_usize));
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let is_focusable = Signal::derive(move || focus_mode_index.get().unwrap_or(0) == 1);
    let focusable = Signal::derive(move || focus_mode_index.get().unwrap_or(0) == 2);
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-visually-hidden-custom".to_string()
        } else {
            String::new()
        }
    });
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let hello_world_code = Signal::derive(move || {
        r##"<VisuallyHidden>"Open account settings"</VisuallyHidden>"##.to_string()
    });
    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<VisuallyHidden".to_string()];
        if is_focusable.get() {
            lines.push("  is_focusable=true".to_string());
        }
        if focusable.get() {
            lines.push("  focusable=true".to_string());
        }
        if !class_name.get().is_empty() {
            lines.push(format!("  class_name={:?}.to_string()", class_name.get()));
        }
        lines.push(format!("  lang={:?}.to_string()", lang.get()));
        lines.push(format!("  dir=A11yDirection::{:?}", dir.get()));
        lines.push(">".to_string());
        lines.push("  \"Skip to account settings\"".to_string());
        lines.push("</VisuallyHidden>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "VisuallyHiddenWorkbenchConfig {{\n  is_focusable: {:?},\n  focusable: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            is_focusable.get(),
            focusable.get(),
            class_name.get(),
            lang.get(),
            dir.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r##"<VisuallyHidden>"Accessible label"</VisuallyHidden>
<VisuallyHidden is_focusable=true lang="en-US".to_string() dir=A11yDirection::Ltr>
  <a href="#docs-visually-hidden-target">"Skip to details"</a>
</VisuallyHidden>
<VisuallyHidden focusable=true class_name="docs-visually-hidden-custom".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl>
  <button type="button">"跳转"</button>
</VisuallyHidden>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="VisuallyHidden"
            slug="visually-hidden"
            group="Forms"
            description="@a11y-baseline/visually-hidden compatible utility for screen-reader-only content and focusable skip-link workflows."
        >
            <Playground title="Hello World (Default API)" code_signal=hello_world_code>
                <div class="docs-stack">
                    <VisuallyHidden>"Open account settings"</VisuallyHidden>
                    <p>
                        "Default usage is a single semantic wrapper without extra state wiring."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Workbench (Focus + Locale + Class)"
                code_signal=workbench_code
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Focus prop"</div>
                        <SegmentedControl
                            id_base="docs-visually-hidden-focus-mode".to_string()
                            options=focus_mode_options.clone()
                            selected_index=focus_mode_index
                            set_selected_index=set_focus_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="VisuallyHidden focus mode".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-visually-hidden-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="VisuallyHidden language".to_string()
                        />

                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <VisuallyHidden
                        is_focusable=is_focusable.get()
                        focusable=focusable.get()
                        class_name=class_name.get()
                        lang=lang.get()
                        dir=dir.get()
                    >
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

            <Playground title="State Matrix (Default / is_focusable / focusable)" code_signal=matrix_code>
                <div class="docs-stack">
                    <VisuallyHidden>"Accessible label"</VisuallyHidden>
                    <VisuallyHidden
                        is_focusable=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <a href="#docs-visually-hidden-target">"Skip to details"</a>
                    </VisuallyHidden>
                    <VisuallyHidden
                        focusable=true
                        class_name="docs-visually-hidden-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <button type="button">"跳转"</button>
                    </VisuallyHidden>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
