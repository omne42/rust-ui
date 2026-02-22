# 组件分层整改并发分片（30 并发）

> 目标：并发修复 `ui` 组件分层实现，使其遵循 `logic -> (headless/a11y 组合) -> view + styles`。
> 约束：子任务**禁止**运行 `cargo fmt/clippy/test/check`；统一由主编排流程集中执行。
> 注意：本文档为分片执行历史快照，部分目录路径仍是迁移前平铺结构；当前目录落位请以 `docs/spec/component_domains.md` 为准。

## 执行规则

- 每个 shard 只改自己负责的组件目录，不跨 shard 改文件。
- 每个 shard 允许执行：`rg`、`sed`、`apply_patch`、局部静态检查。
- 每个 shard 禁止执行任何 cargo 命令（避免并发争用）。
- 统一收敛后只执行一次：`cargo fmt --all`、`./scripts/check.sh`。
- 兼容命名桥接层已移除；`provider/rac/s2/story_utils/style_macro_s1/test_utils/utils` 不再作为 `ui` 分片范围。

## 子任务模板

```text
任务: shard-XX - 组件分层整改
目标: 将分配组件对齐到 logic/styles/view 分层，必要时补充 motion；禁止 render.rs。
前置: 无
验收:
  - 每个组件目录存在 mod.rs + logic.rs + styles.rs + view.rs（纯 facade 目录可豁免）
  - mod.rs 不再出现 mod render;
  - 组件行为逻辑不下沉到 view（状态归一化留在 logic）
文件: crates/ui/src/<assigned-components>/*
约束: 不运行 cargo，不修改其他 shard 文件
```

## 30 并发分片

### shard-01

- `crates/ui/src/accordion`
- `crates/ui/src/checkbox`
- `crates/ui/src/contextual_help`
- `crates/ui/src/flex`
- `crates/ui/src/label`
- `crates/ui/src/pressable_feedback`
- `crates/ui/src/sidebar_menu_action`
- `crates/ui/src/text_field`

### shard-02

- `crates/ui/src/action_bar`
- `crates/ui/src/checkbox_field`
- `crates/ui/src/date_field`
- `crates/ui/src/flip_card`
- `crates/ui/src/labeled_value`
- `crates/ui/src/preview_card`
- `crates/ui/src/sidebar_menu_badge`
- `crates/ui/src/textarea`

### shard-03

- `crates/ui/src/action_button`
- `crates/ui/src/checkbox_group`
- `crates/ui/src/date_input_group`
- `crates/ui/src/footer`
- `crates/ui/src/layout`
- `crates/ui/src/preview_link_card`
- `crates/ui/src/sidebar_rail`
- `crates/ui/src/textfield`

### shard-04

- `crates/ui/src/action_button_group`
- `crates/ui/src/chip`
- `crates/ui/src/date_picker`
- `crates/ui/src/form`
- `crates/ui/src/legend`
- `crates/ui/src/progress`
- `crates/ui/src/sidebar_trigger`
- `components/theme-dark`

### shard-05

- `crates/ui/src/action_group`
- `crates/ui/src/circular_progress`
- `crates/ui/src/date_range_picker`
- `crates/ui/src/form_field`
- `crates/ui/src/link`
- `crates/ui/src/progress_bar`
- `crates/ui/src/sidenav`
- `components/theme-default`

### shard-06

- `crates/ui/src/action_menu`
- `crates/ui/src/clear_button`
- `crates/ui/src/description`
- `crates/ui/src/grid`
- `crates/ui/src/link_button`
- `crates/ui/src/progress_circle`
- `crates/ui/src/skeleton`
- `components/theme-express`

### shard-07

- `crates/ui/src/alert`
- `crates/ui/src/close_button`
- `crates/ui/src/dialog`
- `crates/ui/src/grid_list`
- `crates/ui/src/list`
- `crates/ui/src/skeleton_group`
- `components/theme-light`

### shard-08

- `crates/ui/src/alert_banner`
- `crates/ui/src/coachmark`
- `crates/ui/src/direction`
- `crates/ui/src/gridlist`
- `crates/ui/src/list_box`
- `crates/ui/src/slider`
- `components/thumbnail`

### shard-09

- `crates/ui/src/alert_dialog`
- `crates/ui/src/code`
- `crates/ui/src/disclosure`
- `crates/ui/src/group`
- `crates/ui/src/listbox`
- `crates/ui/src/radio`
- `crates/ui/src/snippet`
- `crates/ui/src/time_field`

### shard-10

- `crates/ui/src/aspect_ratio`
- `crates/ui/src/code_block`
- `crates/ui/src/disclosure_group`
- `crates/ui/src/header`
- `crates/ui/src/listbox_item`
- `crates/ui/src/radio_group`
- `crates/ui/src/sonner`
- `crates/ui/src/toast`

### shard-11

- `crates/ui/src/asset`
- `crates/ui/src/collapsible`
- `crates/ui/src/divider`
- `crates/ui/src/heading`
- `crates/ui/src/listbox_section`
- `crates/ui/src/resizable`
- `crates/ui/src/spacer`
- `crates/ui/src/toaster`

### shard-12

- `crates/ui/src/auto_height`
- `crates/ui/src/collection`
- `crates/ui/src/dnd`
- `crates/ui/src/help_text`
- `crates/ui/src/logic_button`
- `crates/ui/src/ripple`
- `crates/ui/src/spinbutton`
- `crates/ui/src/toggle`

### shard-13

- `crates/ui/src/autocomplete`
- `crates/ui/src/color`
- `crates/ui/src/drag_and_drop`
- `crates/ui/src/hidden_date_input`
- `crates/ui/src/menu`
- `crates/ui/src/spinner`
- `crates/ui/src/toggle_button`

### shard-14

- `crates/ui/src/avatar`
- `crates/ui/src/color_area`
- `crates/ui/src/drawer`
- `crates/ui/src/hover_card`
- `crates/ui/src/menu_item`
- `crates/ui/src/scroll_area`
- `crates/ui/src/split_view`
- `crates/ui/src/toggle_button_group`

### shard-15

- `crates/ui/src/avatar_group`
- `crates/ui/src/color_editor`
- `crates/ui/src/drop_zone`
- `crates/ui/src/icon`
- `crates/ui/src/menu_section`
- `crates/ui/src/scroll_shadow`
- `crates/ui/src/status_light`
- `crates/ui/src/toggle_group`

### shard-16

- `crates/ui/src/badge`
- `crates/ui/src/color_field`
- `crates/ui/src/dropdown`
- `crates/ui/src/icon_button`
- `crates/ui/src/menu_trigger`
- `crates/ui/src/search`
- `crates/ui/src/step_list`
- `crates/ui/src/toolbar`

### shard-17

- `crates/ui/src/bottom_sheet`
- `crates/ui/src/color_handle`
- `crates/ui/src/dropdown_menu`
- `crates/ui/src/icons`
- `crates/ui/src/menubar`
- `crates/ui/src/search_field`
- `crates/ui/src/tooltip`

### shard-18

- `crates/ui/src/breadcrumb`
- `crates/ui/src/color_loupe`
- `crates/ui/src/dropzone`
- `crates/ui/src/icons_ui`
- `crates/ui/src/meter`
- `crates/ui/src/segmented_control`
- `crates/ui/src/top_nav`

### shard-19

- `crates/ui/src/breadcrumbs`
- `crates/ui/src/color_picker`
- `crates/ui/src/empty`
- `crates/ui/src/icons_workflow`
- `components/modal`
- `crates/ui/src/select`
- `crates/ui/src/surface`
- `crates/ui/src/tray`

### shard-20

- `crates/ui/src/button`
- `crates/ui/src/color_slider`
- `crates/ui/src/empty_state`
- `crates/ui/src/iconset`
- `crates/ui/src/native_select`
- `components/selection-indicator`
- `crates/ui/src/swatch`
- `crates/ui/src/tree`

### shard-21

- `crates/ui/src/button_copy`
- `crates/ui/src/color_swatch`
- `crates/ui/src/error_message`
- `crates/ui/src/illustrated_message`
- `crates/ui/src/navigation_menu`
- `crates/ui/src/separator`
- `crates/ui/src/switch`
- `crates/ui/src/underlay`

### shard-22

- `crates/ui/src/button_flip`
- `crates/ui/src/color_swatch_picker`
- `crates/ui/src/error_view`
- `crates/ui/src/image`
- `crates/ui/src/number`
- `components/shared-element-transition`
- `crates/ui/src/switch_group`

### shard-23

- `crates/ui/src/button_group`
- `crates/ui/src/color_thumb`
- `components/example-theme`
- `crates/ui/src/infield_button`
- `crates/ui/src/number_field`
- `crates/ui/src/sheet`
- `crates/ui/src/table`
- `crates/ui/src/view`

### shard-24

- `crates/ui/src/button_search_input`
- `crates/ui/src/color_wheel`
- `crates/ui/src/field`
- `crates/ui/src/inline_alert`
- `crates/ui/src/overlay`
- `crates/ui/src/sidebar`
- `crates/ui/src/tabs`
- `crates/ui/src/virtualizer`

### shard-25

- `crates/ui/src/button_share`
- `components/combo-box`
- `crates/ui/src/field_button`
- `crates/ui/src/input`
- `crates/ui/src/overlay_arrow`
- `crates/ui/src/sidebar_content`
- `crates/ui/src/tag`
- `crates/ui/src/visually_hidden`

### shard-26

- `crates/ui/src/button_theme_toggle`
- `crates/ui/src/combobox`
- `crates/ui/src/field_error`
- `crates/ui/src/input_group`
- `crates/ui/src/overlays`
- `crates/ui/src/sidebar_footer`
- `crates/ui/src/tag_group`
- `crates/ui/src/well`

### shard-27

- `crates/ui/src/calendar`
- `crates/ui/src/command`
- `crates/ui/src/field_group`
- `crates/ui/src/input_otp`
- `crates/ui/src/pagination`
- `crates/ui/src/sidebar_group`
- `crates/ui/src/tags`

### shard-28

- `crates/ui/src/card`
- `crates/ui/src/command_dialog`
- `crates/ui/src/field_label`
- `crates/ui/src/item`
- `crates/ui/src/picker`
- `crates/ui/src/sidebar_header`

### shard-29

- `crates/ui/src/carousel`
- `crates/ui/src/content`
- `crates/ui/src/fieldset`
- `crates/ui/src/kbd`
- `crates/ui/src/picker_button`
- `crates/ui/src/sidebar_inset`
- `crates/ui/src/text`

### shard-30

- `crates/ui/src/chart`
- `crates/ui/src/context_menu`
- `crates/ui/src/file_trigger`
- `crates/ui/src/keyboard`
- `crates/ui/src/popover`
- `crates/ui/src/sidebar_menu`
- `crates/ui/src/text_area`

## 集中 Gate（仅主编排执行一次）

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `./scripts/check.sh`
