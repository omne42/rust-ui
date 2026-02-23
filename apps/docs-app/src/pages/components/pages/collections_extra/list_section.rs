use super::*;

pub(crate) fn list_section() -> AnyView {
    let tone_options = ["Default".to_string(), "Quiet".to_string()];
    let item_count_options = ["0 (empty)".to_string(), "2".to_string(), "4".to_string()];

    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (item_count_index, set_item_count_index) = signal(Some(1_usize));
    let (is_disabled, set_is_disabled) = signal(false);
    let (is_sticky_heading, set_is_sticky_heading) = signal(true);
    let (is_divider_visible, set_is_divider_visible) = signal(true);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);
    let (custom_class, set_custom_class) = signal(false);

    let (primary_selected, set_primary_selected) = signal(true);
    let (secondary_selected, set_secondary_selected) = signal(false);
    let (item_press_count, set_item_press_count) = signal(0_u32);

    let workbench_heading_tone = Signal::derive(move || {
        if tone_index.get().unwrap_or(0) == 1 {
            ListSectionHeadingTone::Quiet
        } else {
            ListSectionHeadingTone::Default
        }
    });
    let workbench_item_count = Signal::derive(move || match item_count_index.get().unwrap_or(1) {
        0 => Some(0_usize),
        2 => Some(4_usize),
        _ => Some(2_usize),
    });
    let workbench_title = Signal::derive(move || match item_count_index.get().unwrap_or(1) {
        0 => "Empty section".to_string(),
        2 => "Large section".to_string(),
        _ => "Preferred regions".to_string(),
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ListSectionMotion {
                initial_y_px: 18.0,
                ..ListSectionMotion::default()
            }
        } else {
            ListSectionMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Workbench list section".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-listbox-section-custom".to_string()
        } else {
            String::new()
        }
    });

    let on_toggle_primary = Callback::new(move |_| {
        set_primary_selected.update(|selected| *selected = !*selected);
        set_item_press_count.update(|count| *count += 1);
    });
    let on_toggle_secondary = Callback::new(move |_| {
        set_secondary_selected.update(|selected| *selected = !*selected);
        set_item_press_count.update(|count| *count += 1);
    });

    let showcase_code = Signal::derive(move || {
        r#"<ListSection title="Preferred regions".to_string() item_count=3>
  <ListItem index=0 is_selected=true is_selection_indicator_visible=true>"US East"</ListItem>
  <ListItem index=1>"EU West"</ListItem>
  <ListItem index=2>"AP South"</ListItem>
</ListSection>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone_expr = match workbench_heading_tone.get() {
            ListSectionHeadingTone::Default => "ListSectionHeadingTone::Default",
            ListSectionHeadingTone::Quiet => "ListSectionHeadingTone::Quiet",
        };
        let motion_expr = if custom_motion.get() {
            "ListSectionMotion { initial_y_px: 18.0, ..ListSectionMotion::default() }"
        } else {
            "ListSectionMotion::default()"
        };
        format!(
            "<ListSection\n  title={}\n  item_count={:?}\n  heading_tone={tone_expr}\n  is_disabled={}\n  is_sticky_heading={}\n  is_divider_visible={}\n  motion={motion_expr}\n  aria_label={}\n  class_name={}\n>\n  <ListItem index=0 is_selected=true>\"Primary\"</ListItem>\n  <ListItem index=1>\"Secondary\"</ListItem>\n</ListSection>",
            rust_string_literal(&workbench_title.get()),
            workbench_item_count.get(),
            bool_word(is_disabled.get()),
            bool_word(is_sticky_heading.get()),
            bool_word(is_divider_visible.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ListSectionActualConfig {{\n  title: {:?},\n  item_count: {:?},\n  heading_tone: {:?},\n  is_disabled: {},\n  is_sticky_heading: {},\n  is_divider_visible: {},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n}}",
            workbench_title.get(),
            workbench_item_count.get(),
            workbench_heading_tone.get(),
            is_disabled.get(),
            is_sticky_heading.get(),
            is_divider_visible.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ListSection title="Default section".to_string() item_count=2 heading_tone=ListSectionHeadingTone::Default>
  <ListItem index=0>"Default item"</ListItem>
  <ListItem index=1>"Second item"</ListItem>
</ListSection>
<ListSection
  title="Quiet sticky section".to_string()
  item_count=2
  heading_tone=ListSectionHeadingTone::Quiet
  is_sticky_heading=true
  is_divider_visible=true
  motion=ListSectionMotion { initial_y_px: 18.0, ..ListSectionMotion::default() }
  class_name="docs-listbox-section-custom".to_string()
>
  <ListItem index=0 is_selected=true>"Quiet item"</ListItem>
  <ListItem index=1 is_disabled=true>"Disabled item"</ListItem>
</ListSection>
<ListSection title="Disabled empty".to_string() item_count=0 is_disabled=true aria_label="Disabled empty list".to_string()>
  <span class="ui-muted">"No options available"</span>
</ListSection>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ListSection"
            slug="list-section"
            group="Collections"
            description="baseline-style list section primitive with centralized heading/item/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
            >
                <ListSection
                    title="Preferred regions".to_string()
                    item_count=3
                >
                    <ListItem index=0 is_selected=true is_selection_indicator_visible=true>
                        "US East"
                    </ListItem>
                    <ListItem index=1>
                        "EU West"
                    </ListItem>
                    <ListItem index=2>
                        "AP South"
                    </ListItem>
                </ListSection>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="list-section-workbench-controls">
                        <div class="docs-search__label">"heading_tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().unwrap_or(0).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(Some(value.min(1)));
                                }
                            }
                        >
                            {tone_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <div class="docs-search__label">"item_count"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || item_count_index.get().unwrap_or(1).to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_item_count_index.set(Some(value.min(2)));
                                }
                            }
                        >
                            {item_count_options
                                .iter()
                                .enumerate()
                                .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                .collect_view()}
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_disabled.get()
                                on:change=move |event| set_is_disabled.set(event_target_checked(&event))
                            />
                            <span>"is_disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_sticky_heading.get()
                                on:change=move |event| set_is_sticky_heading.set(event_target_checked(&event))
                            />
                            <span>"is_sticky_heading"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_divider_visible.get()
                                on:change=move |event| set_is_divider_visible.set(event_target_checked(&event))
                            />
                            <span>"is_divider_visible"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"custom class_name"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ListSection
                        title=workbench_title.get()
                        item_count=workbench_item_count.get().unwrap_or(0)
                        heading_tone=workbench_heading_tone.get()
                        is_disabled=is_disabled.get()
                        is_sticky_heading=is_sticky_heading.get()
                        is_divider_visible=is_divider_visible.get()
                        motion=workbench_motion.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                    >
                        <ListItem
                            index=0
                            is_selected=primary_selected.get()
                            is_focused=true
                            is_selection_indicator_visible=true
                            on_press=on_toggle_primary
                        >
                            "Primary target"
                        </ListItem>
                        <ListItem
                            index=1
                            is_selected=secondary_selected.get()
                            is_divider_visible=true
                            is_selection_indicator_visible=true
                            on_press=on_toggle_secondary
                        >
                            "Secondary target"
                        </ListItem>
                    </ListSection>
                    <span class="ui-muted">
                        "primary_selected: " {move || primary_selected.get()}
                        " · secondary_selected: " {move || secondary_selected.get()}
                        " · item on_press count: " {move || item_press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Disabled / Empty Comparison)"
                code_signal=matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{ListItem, ListSection, ListSectionHeadingTone, ListSectionMotion};".to_string()
                test_source_path="components/list/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <ListSection
                        title="Default section".to_string()
                        item_count=2
                        heading_tone=ListSectionHeadingTone::Default
                    >
                        <ListItem index=0>
                            "Default item"
                        </ListItem>
                        <ListItem index=1>
                            "Second item"
                        </ListItem>
                    </ListSection>
                    <ListSection
                        title="Quiet sticky section".to_string()
                        item_count=2
                        heading_tone=ListSectionHeadingTone::Quiet
                        is_sticky_heading=true
                        is_divider_visible=true
                        motion=ListSectionMotion {
                            initial_y_px: 18.0,
                            ..ListSectionMotion::default()
                        }
                        class_name="docs-listbox-section-custom".to_string()
                    >
                        <ListItem index=0 is_selected=true>
                            "Quiet item"
                        </ListItem>
                        <ListItem index=1 is_disabled=true>
                            "Disabled item"
                        </ListItem>
                    </ListSection>
                    <ListSection
                        title="Disabled empty".to_string()
                        item_count=0
                        is_disabled=true
                        aria_label="Disabled empty list".to_string()
                    >
                        <span class="ui-muted">"No options available"</span>
                    </ListSection>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
