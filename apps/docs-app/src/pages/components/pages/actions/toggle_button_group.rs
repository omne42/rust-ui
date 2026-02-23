use super::*;

pub(crate) fn toggle_button_group() -> AnyView {
    let showcase_code = Signal::derive(move || {
        r#"<ToggleButtonGroup
  orientation=ToggleButtonGroupOrientation::Horizontal
  is_attached=true
  motion=ToggleButtonGroupMotion { duration_ms: 220.0 }
  aria_label="Text style".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
  class_name="docs-toggle-group-custom".to_string()
>
  <ToggleButton default_pressed=true>"Bold"</ToggleButton>
  <ToggleButton>"Italic"</ToggleButton>
  <ToggleButton>"Underline"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });

    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Outline".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Destructive".to_string(),
    ];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let motion_options = vec!["default".to_string(), "custom".to_string()];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(2_usize));
    let (motion_index, set_motion_index) = signal(Some(0_usize));
    let (lang_index, set_lang_index) = signal(Some(0_usize));
    let (attached, set_attached) = signal(false);
    let (rtl, set_rtl) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);

    let orientation = Signal::derive(move || match orientation_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupOrientation::Vertical,
        _ => ToggleButtonGroupOrientation::Horizontal,
    });
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Accent,
        2 => ToggleButtonVariant::Outline,
        3 => ToggleButtonVariant::Secondary,
        4 => ToggleButtonVariant::Ghost,
        5 => ToggleButtonVariant::Destructive,
        _ => ToggleButtonVariant::Default,
    });
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::S,
        2 => ToggleButtonSize::M,
        3 => ToggleButtonSize::L,
        _ => ToggleButtonSize::Xl,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ToggleButtonGroupMotion { duration_ms: 220.0 },
        _ => ToggleButtonGroupMotion::default(),
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
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-toggle-group-custom".to_string()
        } else {
            String::new()
        }
    });
    let aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Formatting controls".to_string()
        } else {
            String::new()
        }
    });

    let (a, set_a) = signal(false);
    let (b, set_b) = signal(true);
    let (c, set_c) = signal(false);
    let a_signal: Signal<bool> = Signal::derive(move || a.get());
    let b_signal: Signal<bool> = Signal::derive(move || b.get());
    let c_signal: Signal<bool> = Signal::derive(move || c.get());
    let on_a_change = Callback::new(move |next: bool| set_a.set(next));
    let on_b_change = Callback::new(move |next: bool| set_b.set(next));
    let on_c_change = Callback::new(move |next: bool| set_c.set(next));
    let attached_selected_count =
        Signal::derive(move || usize::from(a.get()) + usize::from(b.get()) + usize::from(c.get()));

    let workbench_code = Signal::derive(move || {
        let mut toggle_props = String::new();
        if variant.get() != ToggleButtonVariant::Default {
            toggle_props.push_str(&format!(
                " variant=ToggleButtonVariant::{:?}",
                variant.get()
            ));
        }
        if size.get() != ToggleButtonSize::M {
            toggle_props.push_str(&format!(" size=ToggleButtonSize::{:?}", size.get()));
        }
        let snippet = vec![
            "<ToggleButtonGroup".to_string(),
            format!(
                "  orientation=ToggleButtonGroupOrientation::{:?}",
                orientation.get()
            ),
            format!("  is_attached={}", attached.get()),
            format!("  motion={:?}", motion.get()),
            format!("  aria_label={:?}", aria_label.get()),
            format!("  lang={:?}", lang.get()),
            format!("  dir={:?}", dir.get()),
            format!("  class_name={:?}", class_name.get()),
            ">".to_string(),
            format!(
                "  <ToggleButton is_pressed=bold_signal on_pressed_change=on_bold_change{toggle_props}>\"Bold\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton is_pressed=italic_signal on_pressed_change=on_italic_change{toggle_props}>\"Italic\"</ToggleButton>"
            ),
            format!(
                "  <ToggleButton is_pressed=underline_signal on_pressed_change=on_underline_change{toggle_props}>\"Underline\"</ToggleButton>"
            ),
            "</ToggleButtonGroup>".to_string(),
        ];
        snippet.join("\n")
    });

    let (left, set_left) = signal(true);
    let (center, set_center) = signal(false);
    let (right, set_right) = signal(true);
    let left_signal: Signal<bool> = Signal::derive(move || left.get());
    let center_signal: Signal<bool> = Signal::derive(move || center.get());
    let right_signal: Signal<bool> = Signal::derive(move || right.get());
    let on_left_change = Callback::new(move |next: bool| set_left.set(next));
    let on_center_change = Callback::new(move |next: bool| set_center.set(next));
    let on_right_change = Callback::new(move |next: bool| set_right.set(next));
    let detached_selected_count = Signal::derive(move || {
        usize::from(left.get()) + usize::from(center.get()) + usize::from(right.get())
    });
    let states_code = Signal::derive(move || {
        r#"<ToggleButtonGroup
  orientation=ToggleButtonGroupOrientation::Vertical
  is_attached=false
  aria_label="Alignment controls".to_string()
  lang="en-US".to_string()
  dir=A11yDirection::Ltr
>
  <ToggleButton is_pressed=left_signal on_pressed_change=on_left_change>"Left"</ToggleButton>
  <ToggleButton is_pressed=center_signal on_pressed_change=on_center_change>"Center"</ToggleButton>
  <ToggleButton is_pressed=right_signal on_pressed_change=on_right_change>"Right"</ToggleButton>
</ToggleButtonGroup>"#
            .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToggleButtonGroupWorkbenchConfig {{\n  orientation: {:?},\n  is_attached: {},\n  motion: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n  pressed: {{ bold: {}, italic: {}, underline: {} }},\n  attached_selected_count: {},\n}}",
            orientation.get(),
            attached.get(),
            motion.get(),
            if aria_label.get().is_empty() {
                None::<String>
            } else {
                Some(aria_label.get())
            },
            Some(lang.get()),
            Some(dir.get()),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            a.get(),
            b.get(),
            c.get(),
            attached_selected_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="ToggleButtonGroup"
            slug="toggle-button-group"
            group="Actions"
            description="Layout wrapper with baseline-style root state attrs for orientation, attachment, and accessible labeling."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
            >
                <ToggleButtonGroup
                    orientation=ToggleButtonGroupOrientation::Horizontal
                    is_attached=true
                    motion=ToggleButtonGroupMotion { duration_ms: 220.0 }
                    aria_label="Text style".to_string()
                    lang="en-US".to_string()
                    dir=A11yDirection::Ltr
                    class_name="docs-toggle-group-custom".to_string()
                >
                    <ToggleButton default_pressed=true>"Bold"</ToggleButton>
                    <ToggleButton>"Italic"</ToggleButton>
                    <ToggleButton>"Underline"</ToggleButton>
                </ToggleButtonGroup>
            </Playground>

            <Playground
                title="Workbench (Attached + Locale + Motion)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup orientation".to_string()
                        />

                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup size".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup motion".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-toggle-button-group-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleButtonGroup language".to_string()
                        />

                        <Switch checked=attached set_checked=set_attached>
                            "Attached layout"
                        </Switch>
                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>"Custom aria label"</Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = orientation.get();
                    let attached = attached.get();
                    let variant = variant.get();
                    let size = size.get();

                    view! {
                        <div class="docs-stack">
                            <ToggleButtonGroup
                                orientation=orientation
                                is_attached=attached
                                motion=motion.get()
                                aria_label=aria_label.get()
                                lang=lang.get()
                                dir=dir.get()
                                class_name=class_name.get()
                            >
                                <ToggleButton
                                    is_pressed=a_signal
                                    on_pressed_change=on_a_change
                                    variant=variant
                                    size=size
                                >
                                    "Bold"
                                </ToggleButton>
                                <ToggleButton
                                    is_pressed=b_signal
                                    on_pressed_change=on_b_change
                                    variant=variant
                                    size=size
                                >
                                    "Italic"
                                </ToggleButton>
                                <ToggleButton
                                    is_pressed=c_signal
                                    on_pressed_change=on_c_change
                                    variant=variant
                                    size=size
                                >
                                    "Underline"
                                </ToggleButton>
                            </ToggleButtonGroup>
                            <span class="ui-muted">
                                "attached selected count: "
                                {move || attached_selected_count.get()}
                            </span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Orientation + Attachment + Selection)" code_signal=states_code>
                <div class="docs-stack">
                    <ToggleButtonGroup
                        orientation=ToggleButtonGroupOrientation::Vertical
                        is_attached=false
                        aria_label="Alignment controls".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <ToggleButton
                            is_pressed=left_signal
                            on_pressed_change=on_left_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Left"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=center_signal
                            on_pressed_change=on_center_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Center"
                        </ToggleButton>
                        <ToggleButton
                            is_pressed=right_signal
                            on_pressed_change=on_right_change
                            variant=ToggleButtonVariant::Secondary
                        >
                            "Right"
                        </ToggleButton>
                    </ToggleButtonGroup>
                    <span class="ui-muted">
                        "detached selected count: "
                        {move || detached_selected_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
