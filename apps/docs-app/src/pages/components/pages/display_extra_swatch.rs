use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::color::area::A11yDirection;
use ui::{
    SegmentedControl, SegmentedControlSize, Swatch, SwatchBorder, SwatchMotion, SwatchRounding,
    SwatchShape, SwatchSize, Switch,
};

pub(super) fn swatch() -> AnyView {
    let (workbench_selected_raw, set_workbench_selected_raw) = signal(true);
    let workbench_selected: Signal<bool> = Signal::derive(move || workbench_selected_raw.get());
    let (workbench_on_selected_change_runs, set_workbench_on_selected_change_runs) = signal(0_u32);
    let on_selected_change = Callback::new(move |next: bool| {
        set_workbench_selected_raw.set(next);
        set_workbench_on_selected_change_runs.update(|count| *count += 1);
    });

    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let size_options = vec![
        "Xs".to_string(),
        "S".to_string(),
        "M".to_string(),
        "L".to_string(),
    ];
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => SwatchSize::Xs,
        1 => SwatchSize::S,
        3 => SwatchSize::L,
        _ => SwatchSize::M,
    });

    let (workbench_border_index, set_workbench_border_index) = signal(Some(1_usize));
    let border_options = vec![
        "None".to_string(),
        "Light".to_string(),
        "Default".to_string(),
    ];
    let workbench_border =
        Signal::derive(move || match workbench_border_index.get().unwrap_or(1) {
            0 => SwatchBorder::None,
            2 => SwatchBorder::Default,
            _ => SwatchBorder::Light,
        });

    let (workbench_rounding_index, set_workbench_rounding_index) = signal(Some(1_usize));
    let rounding_options = vec![
        "None".to_string(),
        "Default".to_string(),
        "Full".to_string(),
    ];
    let workbench_rounding =
        Signal::derive(move || match workbench_rounding_index.get().unwrap_or(1) {
            0 => SwatchRounding::None,
            2 => SwatchRounding::Full,
            _ => SwatchRounding::Default,
        });

    let (workbench_shape_index, set_workbench_shape_index) = signal(Some(0_usize));
    let shape_options = vec!["Square".to_string(), "Rectangle".to_string()];
    let workbench_shape = Signal::derive(move || match workbench_shape_index.get().unwrap_or(0) {
        1 => SwatchShape::Rectangle,
        _ => SwatchShape::Square,
    });

    let (workbench_is_nothing, set_workbench_is_nothing) = signal(false);
    let (workbench_is_mixed, set_workbench_is_mixed) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_decorative, set_workbench_is_decorative) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_reduced_motion, set_workbench_reduced_motion) = signal(false);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_color, set_workbench_custom_color) = signal(false);

    let hello_world_code = Signal::derive(move || {
        "<Swatch color=\"#ffcc00\".to_string() label=\"Brand\".to_string() />".to_string()
    });

    let workbench_code = Signal::derive(move || {
        let color = if workbench_custom_color.get() {
            "rgba(38, 99, 235, 0.35)"
        } else {
            "#ffcc00"
        };
        let label = if workbench_custom_label.get() {
            "Brand blue"
        } else {
            "Brand"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-swatch-custom"
        } else {
            ""
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_reduced_motion.get() {
            "SwatchMotion::disabled()"
        } else {
            "SwatchMotion::default()"
        };

        [
            "<Swatch".to_string(),
            format!("  color={}", rust_string_literal(color)),
            format!("  label={}", rust_string_literal(label)),
            format!("  size={:?}", workbench_size.get()),
            format!("  border={:?}", workbench_border.get()),
            format!("  rounding={:?}", workbench_rounding.get()),
            format!("  shape={:?}", workbench_shape.get()),
            format!("  is_nothing={}", bool_word(workbench_is_nothing.get())),
            format!("  is_mixed_value={}", bool_word(workbench_is_mixed.get())),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!(
                "  is_decorative={}",
                bool_word(workbench_is_decorative.get())
            ),
            "  selected=workbench_selected".to_string(),
            "  default_selected=true".to_string(),
            "  on_selected_change=on_selected_change".to_string(),
            "  aria_label=\"Color swatch\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            format!("  motion={motion}"),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let color = if workbench_custom_color.get() {
            Some("rgba(38, 99, 235, 0.35)")
        } else {
            Some("#ffcc00")
        };
        let label = if workbench_custom_label.get() {
            Some("Brand blue")
        } else {
            Some("Brand")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-swatch-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if workbench_reduced_motion.get() {
            SwatchMotion::disabled()
        } else {
            SwatchMotion::default()
        };

        format!(
            "SwatchActualConfig {{\n  color: {:?},\n  label: {:?},\n  size: {:?},\n  border: {:?},\n  rounding: {:?},\n  shape: {:?},\n  is_nothing: {},\n  is_mixed_value: {},\n  is_disabled: {},\n  is_decorative: {},\n  selected: Some({}),\n  default_selected: Some(true),\n  on_selected_change: \"runs={}\",\n  aria_label: Some(\"Color swatch\"),\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n  class_name: {class_name:?},\n  motion: {motion:?},\n}}",
            color,
            label,
            workbench_size.get(),
            workbench_border.get(),
            workbench_rounding.get(),
            workbench_shape.get(),
            bool_word(workbench_is_nothing.get()),
            bool_word(workbench_is_mixed.get()),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_decorative.get()),
            bool_word(workbench_selected_raw.get()),
            workbench_on_selected_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r##"<Swatch color="#ffcc00".to_string() label="Default".to_string() />
<Swatch color="rgba(38, 99, 235, 0.35)".to_string() label="Mixed".to_string() is_mixed_value=true shape=SwatchShape::Rectangle size=SwatchSize::L />
<Swatch is_nothing=true is_disabled=true border=SwatchBorder::None rounding=SwatchRounding::Full motion=SwatchMotion::disabled() />"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="Swatch"
            slug="swatch"
            group="Display"
            description="Swatch playground with full API workbench coverage and visible callback feedback."
        >
            <Playground title="Hello World (Default Swatch)" code_signal=hello_world_code>
                <Swatch color="#ffcc00".to_string() label="Brand".to_string() />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="swatch-workbench-controls">
                        <SegmentedControl
                            id_base="docs-swatch-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Swatch size".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-swatch-border".to_string()
                            options=border_options.clone()
                            selected_index=workbench_border_index
                            set_selected_index=set_workbench_border_index
                            size=SegmentedControlSize::Sm
                            aria_label="Swatch border".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-swatch-rounding".to_string()
                            options=rounding_options.clone()
                            selected_index=workbench_rounding_index
                            set_selected_index=set_workbench_rounding_index
                            size=SegmentedControlSize::Sm
                            aria_label="Swatch rounding".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-swatch-shape".to_string()
                            options=shape_options.clone()
                            selected_index=workbench_shape_index
                            set_selected_index=set_workbench_shape_index
                            size=SegmentedControlSize::Sm
                            aria_label="Swatch shape".to_string()
                        />
                        <Switch checked=workbench_is_nothing set_checked=set_workbench_is_nothing>
                            "is_nothing"
                        </Switch>
                        <Switch checked=workbench_is_mixed set_checked=set_workbench_is_mixed>
                            "is_mixed_value"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_decorative set_checked=set_workbench_is_decorative>
                            "is_decorative"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "Custom label"
                        </Switch>
                        <Switch checked=workbench_custom_color set_checked=set_workbench_custom_color>
                            "Custom color"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                        <Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="swatch-workbench-preview">
                    <Swatch
                        color=if workbench_custom_color.get() {
                            "rgba(38, 99, 235, 0.35)".to_string()
                        } else {
                            "#ffcc00".to_string()
                        }
                        label=if workbench_custom_label.get() {
                            "Brand blue".to_string()
                        } else {
                            "Brand".to_string()
                        }
                        size=workbench_size.get()
                        border=workbench_border.get()
                        rounding=workbench_rounding.get()
                        shape=workbench_shape.get()
                        is_nothing=workbench_is_nothing.get()
                        is_mixed_value=workbench_is_mixed.get()
                        is_disabled=workbench_is_disabled.get()
                        is_decorative=workbench_is_decorative.get()
                        selected=workbench_selected
                        default_selected=true
                        on_selected_change=on_selected_change
                        aria_label="Color swatch".to_string()
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-swatch-custom".to_string()
                        } else {
                            String::new()
                        }
                        motion=if workbench_reduced_motion.get() {
                            SwatchMotion::disabled()
                        } else {
                            SwatchMotion::default()
                        }
                    />
                    <span class="ui-muted" data-slot="swatch-workbench-feedback">
                        "selected: " {move || workbench_selected_raw.get()}
                        " · on_selected_change: " {move || workbench_on_selected_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Mixed / Nothing)" code_signal=matrix_code>
                <div class="docs-row" data-slot="swatch-state-matrix">
                    <Swatch color="#ffcc00".to_string() label="Default".to_string() />
                    <Swatch
                        color="rgba(38, 99, 235, 0.35)".to_string()
                        label="Mixed".to_string()
                        is_mixed_value=true
                        shape=SwatchShape::Rectangle
                        size=SwatchSize::L
                    />
                    <Swatch
                        is_nothing=true
                        is_disabled=true
                        border=SwatchBorder::None
                        rounding=SwatchRounding::Full
                        motion=SwatchMotion::disabled()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
