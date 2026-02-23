use super::*;

pub(crate) fn color_swatch_picker() -> AnyView {
    let color_swatch_picker_imports = "use leptos::prelude::*;\nuse ui::{ColorSwatchPicker, ColorSwatchPickerItem, ColorSwatchRounding, ColorSwatchShape};".to_string();
    let swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("#f80", "Orange"),
        ColorSwatchPickerItem::named("#080", "Green"),
        ColorSwatchPickerItem::named("#08f", "Blue"),
    ];
    let swatches_for_basic = swatches.clone();
    let swatches_for_matrix = swatches.clone();
    let swatches_for_matrix_after = swatches.clone();
    let swatches_for_controlled = swatches.clone();
    let swatches_for_matrix_final = swatches.clone();
    let swatches_for_controlled_matrix = swatches.clone();
    let swatches_for_stream = swatches.clone();
    let swatches_for_source = swatches.clone();

    let disabled_swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
        ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
        ColorSwatchPickerItem::new("#08f"),
    ];
    let disabled_swatches_for_state = disabled_swatches.clone();
    let disabled_swatches_for_matrix = disabled_swatches.clone();
    let disabled_swatches_for_matrix_after = disabled_swatches.clone();
    let disabled_swatches_for_matrix_final = disabled_swatches.clone();
    let (controlled_selected_color, set_controlled_selected_color) =
        signal(Some("#A00".to_string()));

    let hello_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![ColorSwatchPickerItem::named("#f80", "Orange")]).0
/>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>"##
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
  class_name="docs-color-swatch-picker-custom".to_string()
  aria_label="Fill color".to_string()
/>"##
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>

<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
/>"##
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>

<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  selected_color=selected_signal
  on_selected_change=on_selected_change
/>"##
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
  ]).0
  aria_label="Fill color".to_string()
/>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
  class_name="docs-color-swatch-picker-custom".to_string()
/>"##
            .to_string()
    });

    let workbench_size_options = vec!["Sm".to_string(), "Md".to_string(), "Lg".to_string()];
    let workbench_shape_options = vec!["Default".to_string(), "Wide".to_string()];
    let workbench_rounding_options = vec!["Default".to_string(), "Full".to_string()];
    let workbench_selected_options = vec![
        "None".to_string(),
        "Red".to_string(),
        "Orange".to_string(),
        "Green".to_string(),
        "Blue".to_string(),
    ];
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_shape_index, set_workbench_shape_index) = signal(Some(0_usize));
    let (workbench_rounding_index, set_workbench_rounding_index) = signal(Some(0_usize));
    let (workbench_selected_index, set_workbench_selected_index) = signal(Some(2_usize));
    let (workbench_use_controlled, set_workbench_use_controlled) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_bordered, set_workbench_is_bordered) = signal(true);
    let (workbench_use_disabled_palette, set_workbench_use_disabled_palette) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_last_selected, set_workbench_last_selected) = signal(Some("#f80".to_string()));

    let workbench_swatches_base = swatches.clone();
    let workbench_swatches_disabled = disabled_swatches.clone();
    let (workbench_swatches, set_workbench_swatches) =
        signal(if workbench_use_disabled_palette.get_untracked() {
            workbench_swatches_disabled.clone()
        } else {
            workbench_swatches_base.clone()
        });
    Effect::new(move |_| {
        let next = if workbench_use_disabled_palette.get() {
            workbench_swatches_disabled.clone()
        } else {
            workbench_swatches_base.clone()
        };
        set_workbench_swatches.set(next);
    });

    let workbench_selected_color =
        Signal::derive(move || match workbench_selected_index.get().unwrap_or(2) {
            1 => Some("#A00".to_string()),
            2 => Some("#f80".to_string()),
            3 => Some("#080".to_string()),
            4 => Some("#08f".to_string()),
            _ => None,
        });

    let workbench_code = Signal::derive(move || {
        let size_variant = match workbench_size_index.get().unwrap_or(1) {
            0 => "ColorSwatchSize::Sm",
            2 => "ColorSwatchSize::Lg",
            _ => "ColorSwatchSize::Md",
        };
        let shape_variant = match workbench_shape_index.get().unwrap_or(0) {
            1 => "ColorSwatchShape::Wide",
            _ => "ColorSwatchShape::Square",
        };
        let rounding_variant = match workbench_rounding_index.get().unwrap_or(0) {
            1 => "ColorSwatchRounding::Full",
            _ => "ColorSwatchRounding::Default",
        };
        let swatch_vector = if workbench_use_disabled_palette.get() {
            r##"signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0"##
        } else {
            r##"signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0"##
        };
        let selected_color = match workbench_selected_index.get().unwrap_or(2) {
            1 => Some("#A00"),
            2 => Some("#f80"),
            3 => Some("#080"),
            4 => Some("#08f"),
            _ => None,
        };
        let selection_lines = if workbench_use_controlled.get() {
            "  selected_color=selected_signal\n  on_selected_change=on_selected_change\n"
                .to_string()
        } else {
            selected_color
                .map(|color| format!("  default_selected_color=\"{color}\".to_string()\n"))
                .unwrap_or_default()
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-color-swatch-picker-custom\".to_string()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Workbench fill color\".to_string()\n"
        } else {
            ""
        };
        let lang_line = if workbench_lang_zh.get() {
            "  lang=\"zh-CN\".to_string()\n"
        } else {
            ""
        };
        let dir_line = if workbench_rtl.get() {
            "  dir=ui_headless::A11yDirection::Rtl\n"
        } else {
            "  dir=ui_headless::A11yDirection::Ltr\n"
        };
        format!(
            "<ColorSwatchPicker\n  swatches={swatch_vector}\n  is_disabled={}\n  size={size_variant}\n  rounding={rounding_variant}\n  shape={shape_variant}\n  is_bordered={}\n{selection_lines}  id_base=\"docs-color-swatch-picker-workbench\".to_string()\n{aria_line}{class_line}{lang_line}{dir_line}  motion=ui::ColorSwatchPickerMotion::default()\n/>",
            workbench_is_disabled.get(),
            workbench_is_bordered.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected_color.get();
        let default_selected = if workbench_use_controlled.get() {
            None
        } else {
            selected.clone()
        };
        format!(
            "ColorSwatchPickerActualConfig {{\n  swatches: {:?},\n  is_disabled: {},\n  size: {:?},\n  rounding: {:?},\n  shape: {:?},\n  is_bordered: {},\n  selected_color: {:?},\n  default_selected_color: {:?},\n  on_selected_change: \"updates(last_selected)\",\n  id_base: Some(\"docs-color-swatch-picker-workbench\"),\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n  motion: ColorSwatchPickerMotion::default(),\n}}",
            if workbench_use_disabled_palette.get() {
                vec![
                    "#A00".to_string(),
                    "rgba(14, 116, 144, 0.4)".to_string(),
                    "rgba(255, 0, 0, 0)".to_string(),
                    "#08f".to_string(),
                ]
            } else {
                vec![
                    "#A00".to_string(),
                    "#f80".to_string(),
                    "#080".to_string(),
                    "#08f".to_string(),
                ]
            },
            workbench_is_disabled.get(),
            match workbench_size_index.get().unwrap_or(1) {
                0 => ColorSwatchSize::Sm,
                2 => ColorSwatchSize::Lg,
                _ => ColorSwatchSize::Md,
            },
            if workbench_rounding_index.get().unwrap_or(0) == 1 {
                ColorSwatchRounding::Full
            } else {
                ColorSwatchRounding::Default
            },
            if workbench_shape_index.get().unwrap_or(0) == 1 {
                ColorSwatchShape::Wide
            } else {
                ColorSwatchShape::Square
            },
            workbench_is_bordered.get(),
            if workbench_use_controlled.get() {
                selected.clone()
            } else {
                None
            },
            default_selected,
            if workbench_custom_aria.get() {
                Some("Workbench fill color")
            } else {
                None
            },
            if workbench_custom_class.get() {
                Some("docs-color-swatch-picker-custom")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
        )
    });

    view! {
        <ComponentPage
            title="ColorSwatchPicker"
            slug="color-swatch-picker"
            group="Display"
            description="baseline-compatible selectable swatch group with centralized color normalization, single-selection state, keyboard roving, and stable slot/data state markers."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(vec![ColorSwatchPickerItem::named("#f80", "Orange")]).0
                />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Interactive acceptance canvas: adjust props/state, observe selection feedback, and replay keyboard flow."
                code_signal=workbench_code
                code_imports=color_swatch_picker_imports.clone()
                test_source_path="components/color-swatch-picker/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-workbench-controls">
                            <div data-slot="color-swatch-picker-workbench-size-control">
                                <div class="docs-search__label">"Size"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-size".to_string()
                                    options=workbench_size_options.clone()
                                    selected_index=workbench_size_index
                                    set_selected_index=set_workbench_size_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker size".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-shape-control">
                                <div class="docs-search__label">"Shape"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-shape".to_string()
                                    options=workbench_shape_options.clone()
                                    selected_index=workbench_shape_index
                                    set_selected_index=set_workbench_shape_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker shape".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-rounding-control">
                                <div class="docs-search__label">"Rounding"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-rounding".to_string()
                                    options=workbench_rounding_options.clone()
                                    selected_index=workbench_rounding_index
                                    set_selected_index=set_workbench_rounding_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker rounding".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-selection-control">
                                <div class="docs-search__label">"Selected color"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-selection".to_string()
                                    options=workbench_selected_options.clone()
                                    selected_index=workbench_selected_index
                                    set_selected_index=set_workbench_selected_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker selected color".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-mode-switch">
                                <Switch checked=workbench_use_controlled set_checked=set_workbench_use_controlled>
                                    "Controlled mode"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-disabled-switch">
                                <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                    "Disabled"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-bordered-switch">
                                <Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered>
                                    "Bordered"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-palette-switch">
                                <Switch
                                    checked=workbench_use_disabled_palette
                                    set_checked=set_workbench_use_disabled_palette
                                >
                                    "Use disabled/transparent palette"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-custom-class-switch">
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "Custom class"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-custom-aria-switch">
                                <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                    "Custom aria_label"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-lang-switch">
                                <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                    "Lang=zh-CN"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-dir-switch">
                                <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                    "dir=rtl"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-workbench-canvas">
                    {move || {
                        let size = match workbench_size_index.get().unwrap_or(1) {
                            0 => ColorSwatchSize::Sm,
                            2 => ColorSwatchSize::Lg,
                            _ => ColorSwatchSize::Md,
                        };
                        let shape = match workbench_shape_index.get().unwrap_or(0) {
                            1 => ColorSwatchShape::Wide,
                            _ => ColorSwatchShape::Square,
                        };
                        let rounding = match workbench_rounding_index.get().unwrap_or(0) {
                            1 => ColorSwatchRounding::Full,
                            _ => ColorSwatchRounding::Default,
                        };
                        let default_selected_color = match workbench_selected_index.get().unwrap_or(2)
                        {
                            1 => "#A00".to_string(),
                            2 => "#f80".to_string(),
                            3 => "#080".to_string(),
                            4 => "#08f".to_string(),
                            _ => String::new(),
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-color-swatch-picker-custom".to_string()
                        } else {
                            String::new()
                        };
                        let aria_label = if workbench_custom_aria.get() {
                            "Workbench fill color".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        };
                        let dir = if workbench_rtl.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        };

                        if workbench_use_controlled.get() {
                            view! {
                                <ColorSwatchPicker
                                    swatches=workbench_swatches
                                    id_base="docs-color-swatch-picker-workbench".to_string()
                                    size=size
                                    selected_color=workbench_selected_color
                                    on_selected_change=Callback::new(move |next: Option<String>| {
                                        set_workbench_last_selected.set(next.clone());
                                        let next_index = match next.as_deref() {
                                            Some("#A00") => 1,
                                            Some("#f80") => 2,
                                            Some("#080") => 3,
                                            Some("#08f") => 4,
                                            _ => 0,
                                        };
                                        set_workbench_selected_index.set(Some(next_index));
                                    })
                                    is_disabled=workbench_is_disabled.get()
                                    is_bordered=workbench_is_bordered.get()
                                    shape=shape
                                    rounding=rounding
                                    class_name=class_name
                                    aria_label=aria_label
                                    lang=lang
                                    dir=dir
                                    motion=ColorSwatchPickerMotion::default()
                                />
                            }
                            .into_any()
                        } else {
                            view! {
                                <ColorSwatchPicker
                                    swatches=workbench_swatches
                                    id_base="docs-color-swatch-picker-workbench".to_string()
                                    size=size
                                    default_selected_color=default_selected_color
                                    on_selected_change=Callback::new(move |next| {
                                        set_workbench_last_selected.set(next);
                                    })
                                    is_disabled=workbench_is_disabled.get()
                                    is_bordered=workbench_is_bordered.get()
                                    shape=shape
                                    rounding=rounding
                                    class_name=class_name
                                    aria_label=aria_label
                                    lang=lang
                                    dir=dir
                                    motion=ColorSwatchPickerMotion::default()
                                />
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted" data-slot="color-swatch-picker-workbench-feedback">
                        {move || {
                            format!(
                                "mode={}, palette={}, last_selected={}, disabled={}, bordered={}",
                                if workbench_use_controlled.get() {
                                    "controlled"
                                } else {
                                    "uncontrolled"
                                },
                                if workbench_use_disabled_palette.get() {
                                    "disabled+transparent"
                                } else {
                                    "base"
                                },
                                workbench_last_selected
                                    .get()
                                    .unwrap_or_else(|| "none".to_string()),
                                workbench_is_disabled.get(),
                                workbench_is_bordered.get(),
                            )
                        }}
                    </span>
                    <ol class="ui-muted" data-slot="color-swatch-picker-workbench-replay">
                        <li>"Replay path: focus Orange swatch, press ArrowRight, observe selected marker change."</li>
                        <li>"Toggle Controlled mode and repeat ArrowRight to verify controlled callback sync."</li>
                        <li>"Enable disabled palette and Disabled switch to verify blocked interaction branch."</li>
                    </ol>
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=matrix_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-state-matrix">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_matrix_after.clone()).0
                        default_selected_color="#f80".to_string()
                        id_base="docs-color-swatch-picker-matrix-default".to_string()
                        size=ColorSwatchSize::Md
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        motion=ColorSwatchPickerMotion::default()
                    />
                    <ColorSwatchPicker
                        swatches=signal(disabled_swatches_for_matrix.clone()).0
                        shape=ColorSwatchShape::Wide
                        rounding=ColorSwatchRounding::Default
                        id_base="docs-color-swatch-picker-matrix-disabled".to_string()
                        is_disabled=true
                        class_name="docs-color-swatch-picker-custom".to_string()
                        aria_label="Fill color".to_string()
                        lang="zh-CN".to_string()
                        dir=ui_headless::A11yDirection::Rtl
                        motion=ColorSwatchPickerMotion::default()
                    />
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_controlled_matrix.clone()).0
                        id_base="docs-color-swatch-picker-matrix-controlled".to_string()
                        size=ColorSwatchSize::Lg
                        selected_color=controlled_selected_color
                        on_selected_change=Callback::new(move |next| {
                            set_controlled_selected_color.set(next);
                        })
                        aria_label="Controlled swatch picker".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        motion=ColorSwatchPickerMotion::default()
                    />
                </div>
            </Playground>

            <Playground
                title="Basic Selection"
                code_signal=basic_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(swatches_for_basic).0
                    default_selected_color="#f80".to_string()
                />
            </Playground>

            // title="State Matrix"
            // swatches=signal(swatches_for_matrix).0
            // swatches=signal(disabled_swatches_for_matrix).0
            <Playground
                title="Transparency + Disabled + Custom Class"
                code_signal=state_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(disabled_swatches_for_state).0
                    shape=ColorSwatchShape::Wide
                    rounding=ColorSwatchRounding::Default
                    class_name="docs-color-swatch-picker-custom".to_string()
                    aria_label="Fill color".to_string()
                />
            </Playground>

            <Playground
                title="Variant Gallery"
                code_signal=matrix_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_matrix_final).0
                        default_selected_color="#f80".to_string()
                    />
                    <ColorSwatchPicker
                        swatches=signal(disabled_swatches_for_matrix_final).0
                        shape=ColorSwatchShape::Wide
                        rounding=ColorSwatchRounding::Default
                        class_name="docs-color-swatch-picker-custom".to_string()
                        aria_label="Fill color".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                code_signal=controlled_contrast_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_controlled.clone()).0
                        default_selected_color="#f80".to_string()
                    />
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_controlled.clone()).0
                        selected_color=controlled_selected_color
                        on_selected_change=Callback::new(move |next| {
                            set_controlled_selected_color.set(next);
                        })
                        aria_label="Controlled swatch picker".to_string()
                    />
                    <span class="ui-muted">
                        {move || {
                            format!(
                                "controlled selected_color={}",
                                controlled_selected_color
                                    .get()
                                    .unwrap_or_else(|| "none".to_string())
                            )
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="ColorSwatchPicker is streaming-optional. Marker contract remains `data-ui-stream-support=unsupported` + `data-ui-stream-fallback=snapshot`."
                code_signal=stream_snapshot_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_stream).0
                        default_selected_color="#f80".to_string()
                        aria_label="Fill color".to_string()
                    />
                    <span class="ui-muted">
                        "effective markers: data-ui-stream-support=unsupported data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Selection / Disabled / Shape Comparison)"
                code_signal=matrix_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-state-matrix-after-workbench">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_matrix.clone()).0
                        default_selected_color="#f80".to_string()
                        id_base="docs-color-swatch-picker-matrix-after-default".to_string()
                        size=ColorSwatchSize::Md
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                        motion=ColorSwatchPickerMotion::default()
                    />
                    <ColorSwatchPicker
                        swatches=signal(disabled_swatches_for_matrix_after.clone()).0
                        id_base="docs-color-swatch-picker-matrix-after-disabled".to_string()
                        shape=ColorSwatchShape::Wide
                        rounding=ColorSwatchRounding::Default
                        is_disabled=true
                        class_name="docs-color-swatch-picker-custom".to_string()
                        aria_label="Fill color".to_string()
                        lang="zh-CN".to_string()
                        dir=ui_headless::A11yDirection::Rtl
                        motion=ColorSwatchPickerMotion::default()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Show code + copy returns runnable snippet with imports injected by apps/docs-app/src/playground.rs::compose_copy_ready_code."
                code_signal=source_first_code
                code_imports=color_swatch_picker_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-copy-ready">
                    <h3>"Source-first / Copy-Paste Ready"</h3>
                    <span class="ui-muted">
                        "Playground copy action injects missing imports through "
                        <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                        "."
                    </span>
                    <span class="ui-muted" data-slot="color-swatch-picker-source-prerequisites">
                        "Dependency baseline (Cargo.toml): "
                        <code>
                            "ui = { default-features = false, features = [\"component-color_swatch_picker\", \"inject-css\"] }"
                        </code>
                    </span>
                    <ul class="ui-muted" data-slot="color-swatch-picker-source-paths">
                        <li><code>"components/color-swatch-picker/src/mod.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/logic.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/view.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/styles.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/motion.rs"</code></li>
                    </ul>
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_source).0
                        default_selected_color="#f80".to_string()
                        class_name="docs-color-swatch-picker-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
