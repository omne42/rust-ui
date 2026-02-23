use super::*;

pub(crate) fn separator() -> AnyView {
    let (workbench_vertical, set_workbench_vertical) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(false);
    let (workbench_hr, set_workbench_hr) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_locale, set_workbench_rtl_locale) = signal(false);

    let workbench_orientation = Signal::derive(move || {
        if workbench_vertical.get() {
            SeparatorOrientation::Vertical
        } else {
            SeparatorOrientation::Horizontal
        }
    });
    let workbench_element_type = Signal::derive(move || {
        if workbench_hr.get() {
            SeparatorElementType::Hr
        } else {
            SeparatorElementType::Div
        }
    });
    let workbench_lang = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_locale.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let workbench_motion = Signal::derive(move || SeparatorMotion {
        animate_in: workbench_custom_motion.get(),
    });
    let workbench_class_name = Signal::derive(move || {
        if workbench_custom_class.get() {
            if workbench_vertical.get() {
                "docs-separator-rail docs-separator-custom".to_string()
            } else {
                "docs-separator-custom".to_string()
            }
        } else {
            String::new()
        }
    });

    let semantic_code = Signal::derive(move || {
        r#"<Separator />
<Separator element_type=SeparatorElementType::Hr />
<Separator orientation=SeparatorOrientation::Vertical class_name="docs-separator-rail".to_string() />"#.to_string()
    });

    let decorative_code = Signal::derive(move || {
        r#"<Separator is_decorative=true class_name="docs-separator-custom".to_string() />
<Separator
  orientation=SeparatorOrientation::Vertical
  is_decorative=true
  class_name="docs-separator-rail docs-separator-custom".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Separator\n  orientation=SeparatorOrientation::{:?}\n  is_decorative={}\n  element_type=SeparatorElementType::{:?}\n  lang={}\n  dir={}\n  motion=ui_layout::SeparatorMotion {{ animate_in: {} }}\n  class_name={}\n/>",
            workbench_orientation.get(),
            bool_word(workbench_decorative.get()),
            workbench_element_type.get(),
            rust_string_literal(&workbench_lang.get()),
            if matches!(workbench_dir.get(), A11yDirection::Rtl) {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
            bool_word(workbench_custom_motion.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SeparatorActualConfig {{\n  orientation: {:?},\n  is_decorative: {},\n  element_type: {:?},\n  lang: {:?},\n  dir: {:?},\n  motion: {:?},\n  class_name: {:?},\n}}",
            workbench_orientation.get(),
            workbench_decorative.get(),
            workbench_element_type.get(),
            workbench_lang.get(),
            workbench_dir.get(),
            workbench_motion.get(),
            workbench_class_name.get(),
        )
    });

    // Separator semantic markers are covered by runtime examples and test snapshots.

    view! {
        <ComponentPage
            title="Separator"
            slug="separator"
            group="Layout"
            description="Spring-enabled separator with centralized orientation/element/decorative state attrs."
        >
            <Playground title="Semantic + Element Type" code_signal=semantic_code>
                <div class="docs-row">
                    <Separator />
                    <Separator element_type=SeparatorElementType::Hr />
                    <Separator
                        orientation=SeparatorOrientation::Vertical
                        class_name="docs-separator-rail".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (All API Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="separator-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_vertical.get()
                                on:change=move |ev| set_workbench_vertical.set(event_target_checked(&ev))
                            />
                            " orientation=Vertical"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_decorative.get()
                                on:change=move |ev| set_workbench_decorative.set(event_target_checked(&ev))
                            />
                            " is_decorative"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_hr.get()
                                on:change=move |ev| set_workbench_hr.set(event_target_checked(&ev))
                            />
                            " element_type=Hr"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " motion.animate_in"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " class_name"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_rtl_locale.get()
                                on:change=move |ev| set_workbench_rtl_locale.set(event_target_checked(&ev))
                            />
                            " lang/dir Arabic"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <Separator
                        orientation=workbench_orientation.get()
                        is_decorative=workbench_decorative.get()
                        element_type=workbench_element_type.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                        motion=workbench_motion.get()
                        class_name=workbench_class_name.get()
                    />
                </div>
            </Playground>

            <Playground title="Decorative + Custom Class" code_signal=decorative_code>
                <div class="docs-stack docs-stack--tight">
                    <Separator
                        is_decorative=true
                        class_name="docs-separator-custom".to_string()
                    />
                    <Separator
                        orientation=SeparatorOrientation::Vertical
                        is_decorative=true
                        class_name="docs-separator-rail docs-separator-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
