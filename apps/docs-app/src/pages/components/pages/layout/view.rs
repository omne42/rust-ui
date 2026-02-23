use super::*;

pub(crate) fn view() -> AnyView {
    let (workbench_accent, set_workbench_accent) = signal(false);
    let (workbench_strong_border, set_workbench_strong_border) = signal(false);
    let (workbench_large_padding, set_workbench_large_padding) = signal(false);
    let (workbench_large_radius, set_workbench_large_radius) = signal(false);
    let (workbench_shadow_enabled, set_workbench_shadow_enabled) = signal(false);
    let (workbench_section, set_workbench_section) = signal(false);
    let (workbench_fluid, set_workbench_fluid) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_background = Signal::derive(move || {
        if workbench_accent.get() {
            ViewBackground::Accent
        } else {
            ViewBackground::Subtle
        }
    });
    let workbench_border = Signal::derive(move || {
        if workbench_strong_border.get() {
            ViewBorder::Strong
        } else {
            ViewBorder::Subtle
        }
    });
    let workbench_padding = Signal::derive(move || {
        if workbench_large_padding.get() {
            ViewPadding::Lg
        } else {
            ViewPadding::Md
        }
    });
    let workbench_radius = Signal::derive(move || {
        if workbench_large_radius.get() {
            ViewRadius::Lg
        } else {
            ViewRadius::Md
        }
    });
    let workbench_shadow = Signal::derive(move || {
        if workbench_shadow_enabled.get() {
            ViewShadow::Md
        } else {
            ViewShadow::None
        }
    });
    let workbench_element = Signal::derive(move || {
        if workbench_section.get() {
            ViewElement::Section
        } else {
            ViewElement::Div
        }
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<View\n  background=ViewBackground::{:?}\n  border=ViewBorder::{:?}\n  padding=ViewPadding::{:?}\n  radius=ViewRadius::{:?}\n  shadow=ViewShadow::{:?}\n  element=ViewElement::{:?}\n  fluid={}\n  aria_label={}\n  class_name={}\n>\n  <div>\"Workbench content\"</div>\n</View>",
            workbench_background.get(),
            workbench_border.get(),
            workbench_padding.get(),
            workbench_radius.get(),
            workbench_shadow.get(),
            workbench_element.get(),
            workbench_fluid.get(),
            if workbench_custom_aria.get() {
                "\"Release notes\".to_string()"
            } else {
                "\"\".to_string()"
            },
            if workbench_custom_class.get() {
                "\"docs-view-custom\".to_string()"
            } else {
                "\"\".to_string()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ViewActualConfig {{\n  background: ViewBackground::{:?},\n  border: ViewBorder::{:?},\n  padding: ViewPadding::{:?},\n  radius: ViewRadius::{:?},\n  shadow: ViewShadow::{:?},\n  element: ViewElement::{:?},\n  fluid: {},\n  aria_label: {},\n  class_name: {},\n}}",
            workbench_background.get(),
            workbench_border.get(),
            workbench_padding.get(),
            workbench_radius.get(),
            workbench_shadow.get(),
            workbench_element.get(),
            workbench_fluid.get(),
            if workbench_custom_aria.get() {
                "Some(\"Release notes\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-view-custom\")"
            } else {
                "None"
            }
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<View background=ViewBackground::Subtle border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md />
<View background=ViewBackground::Accent border=ViewBorder::Strong padding=ViewPadding::Lg radius=ViewRadius::Lg shadow=ViewShadow::Md />
<View element=ViewElement::Section fluid=true aria_label="Release notes".to_string() class_name="docs-view-custom".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="View"
            slug="view"
            group="Layout"
            description="General-purpose baseline-style container with centralized surface token state and stable data markers."
        >
            <Playground
                title="Hello World (Default View)"
                code_signal=Signal::derive(move || {
                    r#"<View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
  <div>"Default container"</div>
</View>"#
                        .to_string()
                })
            >
                <View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Default container"</strong>
                        <span class="ui-muted">"Baseline layout surface for content blocks."</span>
                    </div>
                </View>
            </Playground>

            <Playground title="Element + Fluid + Custom Class"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="view-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_accent.get()
                                on:change=move |ev| set_workbench_accent.set(event_target_checked(&ev))
                            />
                            " background accent"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_strong_border.get()
                                on:change=move |ev| set_workbench_strong_border.set(event_target_checked(&ev))
                            />
                            " border strong"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_large_padding.get()
                                on:change=move |ev| set_workbench_large_padding.set(event_target_checked(&ev))
                            />
                            " padding large"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_large_radius.get()
                                on:change=move |ev| set_workbench_large_radius.set(event_target_checked(&ev))
                            />
                            " radius large"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_shadow_enabled.get()
                                on:change=move |ev| set_workbench_shadow_enabled.set(event_target_checked(&ev))
                            />
                            " shadow md"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_section.get()
                                on:change=move |ev| set_workbench_section.set(event_target_checked(&ev))
                            />
                            " element section"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_fluid.get()
                                on:change=move |ev| set_workbench_fluid.set(event_target_checked(&ev))
                            />
                            " fluid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_aria.get()
                                on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                            />
                            " aria_label"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                    </div>
                }
            >
                <View
                    background=workbench_background.get()
                    border=workbench_border.get()
                    padding=workbench_padding.get()
                    radius=workbench_radius.get()
                    shadow=workbench_shadow.get()
                    element=workbench_element.get()
                    fluid=workbench_fluid.get()
                    aria_label=if workbench_custom_aria.get() {
                        "Release notes".to_string()
                    } else {
                        String::new()
                    }
                    class_name=if workbench_custom_class.get() {
                        "docs-view-custom".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <strong>"Workbench view"</strong>
                        <span class="ui-muted">"Adjust all View props and inspect actual config."</span>
                    </div>
                </View>
            </Playground>

            <Playground title="Surface Tokens" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <View border=ViewBorder::Subtle padding=ViewPadding::Md radius=ViewRadius::Md>
                        <div>"Default"</div>
                    </View>
                    <View
                        background=ViewBackground::Accent
                        border=ViewBorder::Strong
                        padding=ViewPadding::Lg
                        radius=ViewRadius::Lg
                        shadow=ViewShadow::Md
                    >
                        <div>"Accent + elevated"</div>
                    </View>
                    <View
                        element=ViewElement::Section
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                        fluid=true
                        aria_label="Release notes".to_string()
                        class_name="docs-view-custom".to_string()
                    >
                        <div>"Section + fluid"</div>
                    </View>
                    <View
                        element=ViewElement::Span
                        border=ViewBorder::Subtle
                        padding=ViewPadding::Sm
                        radius=ViewRadius::Sm
                    >
                        <span>"Span element"</span>
                    </View>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
