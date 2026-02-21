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
        x_axis_label: "Saturation".to_string(),
        y_axis_label: "Lightness".to_string(),
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
        x_axis_label: "Saturation".to_string(),
        y_axis_label: "Lightness".to_string(),
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
    let cell = contract
        .handlers
        .resolve_cell
        .run(ColorAreaCellInput { col: 5, row: 5 });
    assert_eq!(cell.value, (0.5_f32, 0.5_f32));
    assert_eq!(cell.attrs.role, "gridcell");
    assert_eq!(
        cell.attrs.aria_label,
        "Saturation 50%, Lightness 50%".to_string()
    );
    assert_eq!(cell.attrs.aria_selected, Some("true"));
    assert_eq!(cell.attrs.tabindex, 0);
    assert!(!cell.attrs.disabled);
    assert_eq!(cell.attrs.data_selected, Some("true"));
}
