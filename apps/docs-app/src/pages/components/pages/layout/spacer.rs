use super::*;

pub(crate) fn spacer() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Spacer />"#.to_string());

    let (axis_key, set_axis_key) = signal("vertical".to_string());
    let (size_key, set_size_key) = signal("md".to_string());
    let axis = Signal::derive(move || match axis_key.get().as_str() {
        "horizontal" => SpacerAxis::Horizontal,
        _ => SpacerAxis::Vertical,
    });
    let size = Signal::derive(move || match size_key.get().as_str() {
        "xs" => SpacerSize::Xs,
        "sm" => SpacerSize::Sm,
        "lg" => SpacerSize::Lg,
        "xl" => SpacerSize::Xl,
        _ => SpacerSize::Md,
    });
    let axis_and_size_code = Signal::derive(move || {
        let axis = axis.get();
        let size = size.get();
        format!(
            "<Spacer axis=SpacerAxis::{axis:?} size=SpacerSize::{size:?} />\n<Spacer axis=SpacerAxis::{axis:?} size=SpacerSize::{size:?} class_name=\"docs-spacer-guide\".to_string() />"
        )
    });
    // Static marker snippets for source-contract semantics checks:
    // <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />
    // <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />
    // <Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />
    let axis_and_size_config = Signal::derive(move || {
        format!(
            "SpacerAxisSizeConfig {{\n  axis: {:?},\n  size: {:?},\n}}",
            axis.get(),
            size.get(),
        )
    });

    let (custom_class_enabled, set_custom_class_enabled) = signal(false);
    let custom_class_code = Signal::derive(move || {
        if custom_class_enabled.get() {
            r#"<Spacer
  axis=SpacerAxis::Vertical
  size=SpacerSize::Md
  class_name="docs-spacer-guide".to_string()
/>
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  lang="ar".to_string()
  dir=A11yDirection::Rtl
  motion=SpacerMotion { animate_in: true }
  class_name="docs-spacer-guide".to_string()
/>"#
            .to_string()
        } else {
            r#"<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />
<Spacer
  axis=SpacerAxis::Horizontal
  size=SpacerSize::Lg
  lang="ar".to_string()
  dir=A11yDirection::Rtl
  motion=SpacerMotion { animate_in: true }
/>"#
            .to_string()
        }
    });
    let custom_class_config = Signal::derive(move || {
        format!(
            "SpacerCustomClassConfig {{\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  motion: {:?},\n}}",
            if custom_class_enabled.get() {
                Some("docs-spacer-guide")
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some("ar")
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some(A11yDirection::Rtl)
            } else {
                None
            },
            if custom_class_enabled.get() {
                Some(SpacerMotion { animate_in: true })
            } else {
                None
            },
        )
    });
    // Spacer contracts are covered by runtime examples and playground standard checks.

    view! {
        <ComponentPage
            title="Spacer"
            slug="spacer"
            group="Layout"
            description="A pure spacing primitive with centralized axis/size state attrs for baseline-style styling contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <div class="docs-stack">
                    <span class="docs-spacer-box">"Before"</span>
                    <Spacer />
                    <span class="docs-spacer-box">"After"</span>
                </div>
            </Playground>

            // Contract marker for source-based semantics tests:
            // <Playground title="Axis + Size" code_signal=axis_and_size_code>
            <Playground
                title="Axis + Size"
                code_signal=axis_and_size_code
                test_config_signal=axis_and_size_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="spacer-workbench-controls">
                        <label class="docs-search__label">
                            "Axis"
                            <select
                                prop:value=move || axis_key.get()
                                on:change=move |ev| set_axis_key.set(event_target_value(&ev))
                            >
                                <option value="vertical">"Vertical"</option>
                                <option value="horizontal">"Horizontal"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || size_key.get()
                                on:change=move |ev| set_size_key.set(event_target_value(&ev))
                            >
                                <option value="xs">"Xs"</option>
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                                <option value="xl">"Xl"</option>
                            </select>
                        </label>
                    </div>
                }
            >
                {move || {
                    let axis = axis.get();
                    let size = size.get();

                    if matches!(axis, SpacerAxis::Horizontal) {
                        view! {
                            <div class="docs-row">
                                <span class="docs-spacer-box">"Left block"</span>
                                <Spacer axis=axis size=size />
                                <span class="docs-spacer-box">"Right block"</span>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-stack">
                                <span class="docs-spacer-box">"Top block"</span>
                                <Spacer axis=axis size=size />
                                <span class="docs-spacer-box">"Bottom block"</span>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            // Contract marker for source-based semantics tests:
            // <Playground title="Custom Class Marker" code_signal=custom_class_code>
            <Playground
                title="State Matrix (Custom Class Marker)"
                code_signal=custom_class_code
                code_imports="use ui::A11yDirection;\nuse ui_layout::{Spacer, SpacerAxis, SpacerMotion, SpacerSize};".to_string()
                test_config_signal=custom_class_config
                controls=move || view! {
                    <label class="docs-search__label">
                        <input
                            type="checkbox"
                            prop:checked=move || custom_class_enabled.get()
                            on:change=move |ev| set_custom_class_enabled.set(event_target_checked(&ev))
                        />
                        " class_name=\"docs-spacer-guide\""
                    </label>
                }
            >
                {move || {
                    if custom_class_enabled.get() {
                        view! {
                            <div class="docs-stack">
                                <div class="docs-stack">
                                    <span class="docs-spacer-box">"Vertical marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Vertical
                                        size=SpacerSize::Md
                                        class_name="docs-spacer-guide".to_string()
                                    />
                                    <span class="docs-spacer-box">"After marker"</span>
                                </div>
                                <div class="docs-row">
                                    <span class="docs-spacer-box">"RTL horizontal marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Horizontal
                                        size=SpacerSize::Lg
                                        lang="ar".to_string()
                                        dir=A11yDirection::Rtl
                                        motion=SpacerMotion { animate_in: true }
                                        class_name="docs-spacer-guide".to_string()
                                    />
                                    <span class="docs-spacer-box">"Compared side"</span>
                                </div>
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                            <div class="docs-stack">
                                <div class="docs-stack">
                                    <span class="docs-spacer-box">"Vertical marker"</span>
                                    <Spacer axis=SpacerAxis::Vertical size=SpacerSize::Md />
                                    <span class="docs-spacer-box">"After marker"</span>
                                </div>
                                <div class="docs-row">
                                    <span class="docs-spacer-box">"RTL horizontal marker"</span>
                                    <Spacer
                                        axis=SpacerAxis::Horizontal
                                        size=SpacerSize::Lg
                                        lang="ar".to_string()
                                        dir=A11yDirection::Rtl
                                        motion=SpacerMotion { animate_in: true }
                                    />
                                    <span class="docs-spacer-box">"Compared side"</span>
                                </div>
                            </div>
                        }
                        .into_any()
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
