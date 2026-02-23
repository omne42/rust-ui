use super::*;

pub(crate) fn close_button() -> AnyView {
    let close_button_imports =
        "use leptos::prelude::*;\nuse ui::{CloseButton, CloseButtonSize, CloseButtonVariant};"
            .to_string();
    let (variant_index, set_variant_index) = signal(0usize);
    let (size_index, set_size_index) = signal(1usize);
    let (disabled, set_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (submit_type, set_submit_type) = signal(false);
    let (show_child, set_show_child) = signal(false);
    let (press_count, set_press_count) = signal(0usize);

    let workbench_variant = Signal::derive(move || match variant_index.get() {
        1 => CloseButtonVariant::OverBackground,
        _ => CloseButtonVariant::Default,
    });
    let workbench_size = Signal::derive(move || match size_index.get() {
        0 => CloseButtonSize::Sm,
        2 => CloseButtonSize::Xl,
        _ => CloseButtonSize::Md,
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Dismiss popover".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-close-button-custom".to_string()
        } else {
            String::new()
        }
    });

    let on_press = Callback::new(move |_| {
        set_press_count.update(|value| *value += 1);
    });
    let workbench_node_ref = NodeRef::<leptos::html::Button>::new();
    let matrix_node_ref = NodeRef::<leptos::html::Button>::new();

    let showcase_code = Signal::derive(move || r#"<CloseButton />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<CloseButton\n  variant=CloseButtonVariant::{:?}\n  size=CloseButtonSize::{:?}\n  disabled={}\n  aria_label={}\n  class_name={}\n  button_type={}\n  node_ref=node_ref\n  on_press=Some(Callback::new(move |_| {{}}))\n>\n  {}\n</CloseButton>",
            workbench_variant.get(),
            workbench_size.get(),
            bool_word(disabled.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(if submit_type.get() {
                "submit"
            } else {
                "button"
            }),
            if show_child.get() { "\"Dismiss\"" } else { "" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<CloseButton />
<CloseButton variant=CloseButtonVariant::OverBackground size=CloseButtonSize::Sm />
<CloseButton
  variant=CloseButtonVariant::Default
  size=CloseButtonSize::Xl
  disabled=true
  aria_label="Dismiss dialog".to_string()
  class_name="docs-close-button-custom".to_string()
  button_type="submit"
  node_ref=NodeRef::<leptos::html::Button>::new()
  on_press=Some(Callback::new(move |_| {}))
>
  "Dismiss"
</CloseButton>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/button/src/close_button/styles.rs */\n{}",
            ui::close_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "CloseButtonActualConfig {{\n  variant: {:?},\n  size: {:?},\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: {:?},\n  on_press: {:?},\n  children: {:?},\n}}",
            workbench_variant.get(),
            workbench_size.get(),
            disabled.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            if submit_type.get() {
                "submit"
            } else {
                "button"
            },
            Some("NodeRef<html::Button>"),
            Some("Callback<OnPress>"),
            if show_child.get() {
                Some("Dismiss")
            } else {
                None
            },
        )
    });

    view! {
        <ComponentPage
            title="CloseButton"
            slug="close-button"
            group="Actions"
            description="baseline-style close affordance with default icon fallback, centralized variant+size contracts, and stable state/source data markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=close_button_imports.clone()
            >
                <div class="docs-row">
                    <CloseButton />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=close_button_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/button/src/close_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="close-button-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || variant_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_variant_index.set(value.min(1));
                                }
                            }
                        >
                            <option value="0">"Default"</option>
                            <option value="1">"OverBackground"</option>
                        </select>

                        <div class="docs-search__label">"Size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || size_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_size_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Sm"</option>
                            <option value="1">"Md"</option>
                            <option value="2">"Xl"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || disabled.get()
                                on:change=move |event| set_disabled.set(event_target_checked(&event))
                            />
                            <span>"Disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"Custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || submit_type.get()
                                on:change=move |event| set_submit_type.set(event_target_checked(&event))
                            />
                            <span>"button_type=submit"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || show_child.get()
                                on:change=move |event| set_show_child.set(event_target_checked(&event))
                            />
                            <span>"Custom children"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <CloseButton
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        disabled=disabled.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        button_type=if submit_type.get() { "submit" } else { "button" }
                        node_ref=workbench_node_ref
                        on_press=on_press
                    >
                        {move || if show_child.get() { "Dismiss".to_string() } else { String::new() }}
                    </CloseButton>
                    <span class="ui-muted">
                        "on_press count: " {move || press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Size / Disabled Comparison)"
                code_signal=matrix_code
                code_imports=close_button_imports
            >
                <div class="docs-row">
                    <CloseButton />
                    <CloseButton
                        variant=CloseButtonVariant::OverBackground
                        size=CloseButtonSize::Sm
                    />
                    <CloseButton
                        variant=CloseButtonVariant::Default
                        size=CloseButtonSize::Xl
                        disabled=true
                        aria_label="Dismiss dialog".to_string()
                        class_name="docs-close-button-custom".to_string()
                        button_type="submit"
                        node_ref=matrix_node_ref
                        on_press=Callback::new(move |_| {})
                    >
                        "Dismiss"
                    </CloseButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
