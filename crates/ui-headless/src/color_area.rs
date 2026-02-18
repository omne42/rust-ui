use crate::a11y::{A11yDirection, labeled_group_attrs};
use leptos::prelude::*;
use ui_state_primitives::color_area::{
    ColorAreaState, clamp_value, move_value_by_delta, parse_axis_percent, value_from_cell,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorAreaRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_labelledby: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub tabindex: i32,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_grid_size: String,
    pub data_value_x: String,
    pub data_value_y: String,
    pub data_selected_col: String,
    pub data_selected_row: String,
    pub data_has_preview: Option<&'static str>,
    pub data_label_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_class_source: &'static str,
    pub data_x_axis_source: &'static str,
    pub data_y_axis_source: &'static str,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaGridAttrs {
    pub role: &'static str,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaAxisAttrs {
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorAreaKeyboardInput {
    pub key: String,
    pub current_value: (f32, f32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorAreaKeyboardResult {
    pub next_value: (f32, f32),
    pub prevent_default: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorAreaHandlers {
    pub on_key_down: Callback<ColorAreaKeyboardInput, Option<ColorAreaKeyboardResult>>,
    pub parse_axis_input: Callback<String, Option<f32>>,
    pub cell_to_value: Callback<(usize, usize, usize), (f32, f32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorAreaSemanticState {
    pub is_disabled: bool,
    pub value_x_percent: u8,
    pub value_y_percent: u8,
    pub selected_col: usize,
    pub selected_row: usize,
    pub grid_size: usize,
    pub label_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub x_axis_source: &'static str,
    pub y_axis_source: &'static str,
}

#[derive(Clone, Debug)]
pub struct ColorAreaContract {
    pub root_attrs: ColorAreaRootAttrs,
    pub grid_attrs: ColorAreaGridAttrs,
    pub axis_attrs: ColorAreaAxisAttrs,
    pub handlers: ColorAreaHandlers,
    pub state: ColorAreaSemanticState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorAreaOptions {
    pub state: ColorAreaState,
    pub aria_label: String,
    pub label_id: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_color_area(options: ColorAreaOptions) -> ColorAreaContract {
    let group = labeled_group_attrs(options.aria_label, options.lang, options.dir);
    let state = options.state;

    ColorAreaContract {
        root_attrs: ColorAreaRootAttrs {
            role: group.role,
            aria_label: group.aria_label,
            aria_labelledby: options.label_id,
            lang: group.lang,
            dir: group.dir,
            tabindex: if state.is_disabled { -1 } else { 0 },
            data_state: state.data_state_attr.as_attr(),
            data_disabled: state.is_disabled.then_some("true"),
            data_grid_size: state.grid_size.to_string(),
            data_value_x: state.value_x_percent.to_string(),
            data_value_y: state.value_y_percent.to_string(),
            data_selected_col: state.selected_col.to_string(),
            data_selected_row: state.selected_row.to_string(),
            data_has_preview: state.has_preview_color.then_some("true"),
            data_label_source: state.label_source_attr.as_attr(),
            data_aria_source: state.aria_source_attr.as_attr(),
            data_class_source: state.class_source_attr.as_attr(),
            data_x_axis_source: state.x_axis_source_attr.as_attr(),
            data_y_axis_source: state.y_axis_source_attr.as_attr(),
            data_custom_class: state.has_custom_class_name.then_some("true"),
        },
        grid_attrs: ColorAreaGridAttrs {
            role: "grid",
            aria_disabled: state.is_disabled.then_some("true"),
        },
        axis_attrs: ColorAreaAxisAttrs {
            aria_disabled: state.is_disabled.then_some("true"),
        },
        handlers: ColorAreaHandlers {
            on_key_down: Callback::new(move |input: ColorAreaKeyboardInput| {
                if state.is_disabled {
                    return None;
                }

                let next = match input.key.as_str() {
                    "ArrowLeft" => Some(move_value_by_delta(
                        input.current_value,
                        -1.0,
                        0.0,
                        state.step,
                    )),
                    "ArrowRight" => Some(move_value_by_delta(
                        input.current_value,
                        1.0,
                        0.0,
                        state.step,
                    )),
                    "ArrowUp" => Some(move_value_by_delta(
                        input.current_value,
                        0.0,
                        1.0,
                        state.step,
                    )),
                    "ArrowDown" => Some(move_value_by_delta(
                        input.current_value,
                        0.0,
                        -1.0,
                        state.step,
                    )),
                    "Home" => Some((0.0, 0.0)),
                    "End" => Some((1.0, 1.0)),
                    _ => None,
                };

                next.map(|next_value| ColorAreaKeyboardResult {
                    next_value: clamp_value(next_value),
                    prevent_default: true,
                })
            }),
            parse_axis_input: Callback::new(|raw: String| parse_axis_percent(raw.as_str())),
            cell_to_value: Callback::new(|(col, row, grid_size)| {
                value_from_cell(col, row, grid_size)
            }),
        },
        state: ColorAreaSemanticState {
            is_disabled: state.is_disabled,
            value_x_percent: state.value_x_percent,
            value_y_percent: state.value_y_percent,
            selected_col: state.selected_col,
            selected_row: state.selected_row,
            grid_size: state.grid_size,
            label_source: state.label_source_attr.as_attr(),
            aria_source: state.aria_source_attr.as_attr(),
            class_source: state.class_source_attr.as_attr(),
            x_axis_source: state.x_axis_source_attr.as_attr(),
            y_axis_source: state.y_axis_source_attr.as_attr(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::color_area::{ColorAreaStateInput, resolve_state};

    #[test]
    fn use_color_area_maps_locale_and_semantic_markers() {
        let state = resolve_state(ColorAreaStateInput {
            disabled: true,
            step: 0.1,
            value: (0.35, 0.8),
            grid_size: 11,
            has_preview_color: true,
            has_custom_label: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            has_custom_x_axis_label: false,
            has_custom_y_axis_label: true,
        });

        let contract = use_color_area(ColorAreaOptions {
            state,
            aria_label: " Color area region ".to_string(),
            label_id: "color-area-label".to_string(),
            lang: Some(" zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.root_attrs.role, "group");
        assert_eq!(contract.root_attrs.aria_label, " Color area region ");
        assert_eq!(contract.root_attrs.aria_labelledby, "color-area-label");
        assert_eq!(contract.root_attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.root_attrs.dir, Some("rtl"));
        assert_eq!(contract.root_attrs.tabindex, -1);
        assert_eq!(contract.root_attrs.data_state, "disabled");
        assert_eq!(contract.root_attrs.data_disabled, Some("true"));
        assert_eq!(contract.root_attrs.data_grid_size, "11");
        assert_eq!(contract.root_attrs.data_selected_col, "4");
        assert_eq!(contract.root_attrs.data_selected_row, "2");
        assert_eq!(contract.root_attrs.data_has_preview, Some("true"));
        assert_eq!(contract.root_attrs.data_label_source, "custom");
        assert_eq!(contract.root_attrs.data_aria_source, "default");
        assert_eq!(contract.root_attrs.data_class_source, "custom");
        assert_eq!(contract.root_attrs.data_x_axis_source, "default");
        assert_eq!(contract.root_attrs.data_y_axis_source, "custom");
        assert_eq!(contract.root_attrs.data_custom_class, Some("true"));
    }

    #[test]
    fn use_color_area_handlers_map_keyboard_axis_and_cell_contracts() {
        let state = resolve_state(ColorAreaStateInput {
            disabled: false,
            step: 0.1,
            value: (0.5, 0.5),
            grid_size: 11,
            has_preview_color: false,
            has_custom_label: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            has_custom_x_axis_label: false,
            has_custom_y_axis_label: false,
        });
        let contract = use_color_area(ColorAreaOptions {
            state,
            aria_label: "Color area".to_string(),
            label_id: "label".to_string(),
            lang: None,
            dir: None,
        });

        let key = contract.handlers.on_key_down.run(ColorAreaKeyboardInput {
            key: "ArrowRight".to_string(),
            current_value: (0.5, 0.5),
        });
        assert_eq!(
            key,
            Some(ColorAreaKeyboardResult {
                next_value: (0.6, 0.5),
                prevent_default: true
            })
        );

        assert_eq!(
            contract.handlers.parse_axis_input.run("75".to_string()),
            Some(0.75)
        );
        assert_eq!(
            contract.handlers.cell_to_value.run((5, 5, 11)),
            (0.5_f32, 0.5_f32)
        );
    }
}
