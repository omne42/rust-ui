# 组件分层整改并发分片（30 并发）

> 目标：并发修复 `ui-components` 组件分层实现，使其遵循 `logic -> (headless/a11y 组合) -> view + styles`。
> 约束：子任务**禁止**运行 `cargo fmt/clippy/test/check`；统一由主编排流程集中执行。

## 执行规则

- 每个 shard 只改自己负责的组件目录，不跨 shard 改文件。
- 每个 shard 允许执行：`rg`、`sed`、`apply_patch`、局部静态检查。
- 每个 shard 禁止执行任何 cargo 命令（避免并发争用）。
- 统一收敛后只执行一次：`cargo fmt --all`、`./scripts/check.sh`。

## 子任务模板

```text
任务: shard-XX - 组件分层整改
目标: 将分配组件对齐到 logic/styles/view 分层，必要时补充 motion；禁止 render.rs。
前置: 无
验收:
  - 每个组件目录存在 mod.rs + logic.rs + styles.rs + view.rs（纯 facade 目录可豁免）
  - mod.rs 不再出现 mod render;
  - 组件行为逻辑不下沉到 view（状态归一化留在 logic）
文件: crates/ui-components/src/<assigned-components>/*
约束: 不运行 cargo，不修改其他 shard 文件
```

## 30 并发分片

### shard-01

- `crates/ui-components/src/accordion`
- `crates/ui-components/src/checkbox`
- `crates/ui-components/src/contextual_help`
- `crates/ui-components/src/flex`
- `crates/ui-components/src/label`
- `crates/ui-components/src/pressable_feedback`
- `crates/ui-components/src/sidebar_menu_action`
- `crates/ui-components/src/text_field`

### shard-02

- `crates/ui-components/src/action_bar`
- `crates/ui-components/src/checkbox_field`
- `crates/ui-components/src/date_field`
- `crates/ui-components/src/flip_card`
- `crates/ui-components/src/labeled_value`
- `crates/ui-components/src/preview_card`
- `crates/ui-components/src/sidebar_menu_badge`
- `crates/ui-components/src/textarea`

### shard-03

- `crates/ui-components/src/action_button`
- `crates/ui-components/src/checkbox_group`
- `crates/ui-components/src/date_input_group`
- `crates/ui-components/src/footer`
- `crates/ui-components/src/layout`
- `crates/ui-components/src/preview_link_card`
- `crates/ui-components/src/sidebar_rail`
- `crates/ui-components/src/textfield`

### shard-04

- `crates/ui-components/src/action_button_group`
- `crates/ui-components/src/chip`
- `crates/ui-components/src/date_picker`
- `crates/ui-components/src/form`
- `crates/ui-components/src/legend`
- `crates/ui-components/src/progress`
- `crates/ui-components/src/sidebar_trigger`
- `crates/ui-components/src/theme_dark`

### shard-05

- `crates/ui-components/src/action_group`
- `crates/ui-components/src/circular_progress`
- `crates/ui-components/src/date_range_picker`
- `crates/ui-components/src/form_field`
- `crates/ui-components/src/link`
- `crates/ui-components/src/progress_bar`
- `crates/ui-components/src/sidenav`
- `crates/ui-components/src/theme_default`

### shard-06

- `crates/ui-components/src/action_menu`
- `crates/ui-components/src/clear_button`
- `crates/ui-components/src/description`
- `crates/ui-components/src/grid`
- `crates/ui-components/src/link_button`
- `crates/ui-components/src/progress_circle`
- `crates/ui-components/src/skeleton`
- `crates/ui-components/src/theme_express`

### shard-07

- `crates/ui-components/src/alert`
- `crates/ui-components/src/close_button`
- `crates/ui-components/src/dialog`
- `crates/ui-components/src/grid_list`
- `crates/ui-components/src/list`
- `crates/ui-components/src/provider`
- `crates/ui-components/src/skeleton_group`
- `crates/ui-components/src/theme_light`

### shard-08

- `crates/ui-components/src/alert_banner`
- `crates/ui-components/src/coachmark`
- `crates/ui-components/src/direction`
- `crates/ui-components/src/gridlist`
- `crates/ui-components/src/list_box`
- `crates/ui-components/src/rac`
- `crates/ui-components/src/slider`
- `crates/ui-components/src/thumbnail`

### shard-09

- `crates/ui-components/src/alert_dialog`
- `crates/ui-components/src/code`
- `crates/ui-components/src/disclosure`
- `crates/ui-components/src/group`
- `crates/ui-components/src/listbox`
- `crates/ui-components/src/radio`
- `crates/ui-components/src/snippet`
- `crates/ui-components/src/time_field`

### shard-10

- `crates/ui-components/src/aspect_ratio`
- `crates/ui-components/src/code_block`
- `crates/ui-components/src/disclosure_group`
- `crates/ui-components/src/header`
- `crates/ui-components/src/listbox_item`
- `crates/ui-components/src/radio_group`
- `crates/ui-components/src/sonner`
- `crates/ui-components/src/toast`

### shard-11

- `crates/ui-components/src/asset`
- `crates/ui-components/src/collapsible`
- `crates/ui-components/src/divider`
- `crates/ui-components/src/heading`
- `crates/ui-components/src/listbox_section`
- `crates/ui-components/src/resizable`
- `crates/ui-components/src/spacer`
- `crates/ui-components/src/toaster`

### shard-12

- `crates/ui-components/src/auto_height`
- `crates/ui-components/src/collection`
- `crates/ui-components/src/dnd`
- `crates/ui-components/src/help_text`
- `crates/ui-components/src/logic_button`
- `crates/ui-components/src/ripple`
- `crates/ui-components/src/spinbutton`
- `crates/ui-components/src/toggle`

### shard-13

- `crates/ui-components/src/autocomplete`
- `crates/ui-components/src/color`
- `crates/ui-components/src/drag_and_drop`
- `crates/ui-components/src/hidden_date_input`
- `crates/ui-components/src/menu`
- `crates/ui-components/src/s2`
- `crates/ui-components/src/spinner`
- `crates/ui-components/src/toggle_button`

### shard-14

- `crates/ui-components/src/avatar`
- `crates/ui-components/src/color_area`
- `crates/ui-components/src/drawer`
- `crates/ui-components/src/hover_card`
- `crates/ui-components/src/menu_item`
- `crates/ui-components/src/scroll_area`
- `crates/ui-components/src/split_view`
- `crates/ui-components/src/toggle_button_group`

### shard-15

- `crates/ui-components/src/avatar_group`
- `crates/ui-components/src/color_editor`
- `crates/ui-components/src/drop_zone`
- `crates/ui-components/src/icon`
- `crates/ui-components/src/menu_section`
- `crates/ui-components/src/scroll_shadow`
- `crates/ui-components/src/status_light`
- `crates/ui-components/src/toggle_group`

### shard-16

- `crates/ui-components/src/badge`
- `crates/ui-components/src/color_field`
- `crates/ui-components/src/dropdown`
- `crates/ui-components/src/icon_button`
- `crates/ui-components/src/menu_trigger`
- `crates/ui-components/src/search`
- `crates/ui-components/src/step_list`
- `crates/ui-components/src/toolbar`

### shard-17

- `crates/ui-components/src/bottom_sheet`
- `crates/ui-components/src/color_handle`
- `crates/ui-components/src/dropdown_menu`
- `crates/ui-components/src/icons`
- `crates/ui-components/src/menubar`
- `crates/ui-components/src/search_field`
- `crates/ui-components/src/story_utils`
- `crates/ui-components/src/tooltip`

### shard-18

- `crates/ui-components/src/breadcrumb`
- `crates/ui-components/src/color_loupe`
- `crates/ui-components/src/dropzone`
- `crates/ui-components/src/icons_ui`
- `crates/ui-components/src/meter`
- `crates/ui-components/src/segmented_control`
- `crates/ui-components/src/style_macro_s1`
- `crates/ui-components/src/top_nav`

### shard-19

- `crates/ui-components/src/breadcrumbs`
- `crates/ui-components/src/color_picker`
- `crates/ui-components/src/empty`
- `crates/ui-components/src/icons_workflow`
- `crates/ui-components/src/modal`
- `crates/ui-components/src/select`
- `crates/ui-components/src/surface`
- `crates/ui-components/src/tray`

### shard-20

- `crates/ui-components/src/button`
- `crates/ui-components/src/color_slider`
- `crates/ui-components/src/empty_state`
- `crates/ui-components/src/iconset`
- `crates/ui-components/src/native_select`
- `crates/ui-components/src/selection_indicator`
- `crates/ui-components/src/swatch`
- `crates/ui-components/src/tree`

### shard-21

- `crates/ui-components/src/button_copy`
- `crates/ui-components/src/color_swatch`
- `crates/ui-components/src/error_message`
- `crates/ui-components/src/illustrated_message`
- `crates/ui-components/src/navigation_menu`
- `crates/ui-components/src/separator`
- `crates/ui-components/src/switch`
- `crates/ui-components/src/underlay`

### shard-22

- `crates/ui-components/src/button_flip`
- `crates/ui-components/src/color_swatch_picker`
- `crates/ui-components/src/error_view`
- `crates/ui-components/src/image`
- `crates/ui-components/src/number`
- `crates/ui-components/src/shared_element_transition`
- `crates/ui-components/src/switch_group`
- `crates/ui-components/src/utils`

### shard-23

- `crates/ui-components/src/button_group`
- `crates/ui-components/src/color_thumb`
- `crates/ui-components/src/example_theme`
- `crates/ui-components/src/infield_button`
- `crates/ui-components/src/number_field`
- `crates/ui-components/src/sheet`
- `crates/ui-components/src/table`
- `crates/ui-components/src/view`

### shard-24

- `crates/ui-components/src/button_search_input`
- `crates/ui-components/src/color_wheel`
- `crates/ui-components/src/field`
- `crates/ui-components/src/inline_alert`
- `crates/ui-components/src/overlay`
- `crates/ui-components/src/sidebar`
- `crates/ui-components/src/tabs`
- `crates/ui-components/src/virtualizer`

### shard-25

- `crates/ui-components/src/button_share`
- `crates/ui-components/src/combo_box`
- `crates/ui-components/src/field_button`
- `crates/ui-components/src/input`
- `crates/ui-components/src/overlay_arrow`
- `crates/ui-components/src/sidebar_content`
- `crates/ui-components/src/tag`
- `crates/ui-components/src/visually_hidden`

### shard-26

- `crates/ui-components/src/button_theme_toggle`
- `crates/ui-components/src/combobox`
- `crates/ui-components/src/field_error`
- `crates/ui-components/src/input_group`
- `crates/ui-components/src/overlays`
- `crates/ui-components/src/sidebar_footer`
- `crates/ui-components/src/tag_group`
- `crates/ui-components/src/well`

### shard-27

- `crates/ui-components/src/calendar`
- `crates/ui-components/src/command`
- `crates/ui-components/src/field_group`
- `crates/ui-components/src/input_otp`
- `crates/ui-components/src/pagination`
- `crates/ui-components/src/sidebar_group`
- `crates/ui-components/src/tags`

### shard-28

- `crates/ui-components/src/card`
- `crates/ui-components/src/command_dialog`
- `crates/ui-components/src/field_label`
- `crates/ui-components/src/item`
- `crates/ui-components/src/picker`
- `crates/ui-components/src/sidebar_header`
- `crates/ui-components/src/test_utils`

### shard-29

- `crates/ui-components/src/carousel`
- `crates/ui-components/src/content`
- `crates/ui-components/src/fieldset`
- `crates/ui-components/src/kbd`
- `crates/ui-components/src/picker_button`
- `crates/ui-components/src/sidebar_inset`
- `crates/ui-components/src/text`

### shard-30

- `crates/ui-components/src/chart`
- `crates/ui-components/src/context_menu`
- `crates/ui-components/src/file_trigger`
- `crates/ui-components/src/keyboard`
- `crates/ui-components/src/popover`
- `crates/ui-components/src/sidebar_menu`
- `crates/ui-components/src/text_area`

## 集中 Gate（仅主编排执行一次）

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `./scripts/check.sh`
