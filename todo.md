# Playground 全量动态化 TODO

## 当前进度

- 真实 Playground 标签总数：465
- 已动态化（code_signal）：465
- 已带 controls：12
- 剩余未动态化：0

> 说明：`overlays.rs` 中有 2 个 `"<Playground"` 覆盖锚点字符串，仅用于测试覆盖，不是实际 Playground 标签。

## 已完成

- [x] Playground 导入自动补全（缺失补齐、重复去重）
- [x] actions.rs 7 个可交互 playground 动态化（参数内联 + 默认参数省略）
- [x] actions_extra.rs 新增 4 个动态 playground（ActionBar / ActionGroup / Toggle / ToggleGroup）
- [x] actions_extra_icon_button.rs 新增 1 个动态 playground（含 controls）
- [x] actions_extra_picker_button.rs 新增 1 个动态 playground（含 controls）
- [x] collections 全组 playground 动态化（含 `collections.rs` / `collections_command.rs` / `collections_extra.rs` 及子页）
- [x] display 全组 playground 动态化（含 `display.rs` / `display_extra.rs` 及全部子页）
- [x] files 全组 playground 动态化
- [x] forms 全组 playground 动态化
- [x] layout 全组 playground 动态化
- [x] overlays 全组 playground 动态化

- [x] 第二阶段：actions_extra.rs 关键 playground 片段改为自包含（ActionBar/ActionGroup/Toggle/ToggleGroup），减少不必要 let，修复外部变量引用
- [x] 第二阶段：actions.rs 中 ActionButton / ActionButtonGroup / ActionMenu 片段改为自包含可复制（含 items/on_action/open 控制示例）
- [x] 第二阶段：layout 全组关键片段自包含化（含 Sidebar / SidebarMenu / SidebarGroup / SidebarRail / SidebarTrigger / Sidenav / ScrollArea / ScrollShadow）
- [x] 第二阶段：display.rs + display_extra.rs 关键片段自包含化（AvatarGroup / Chart）
- [x] 第二阶段：overlays.rs + overlays_extra.rs 关键片段自包含化（Overlay / Toast / Underlay）
- [x] 第二阶段：collections.rs 全组片段自包含化（ListBox / Menu / MenuTrigger / Select / ComboBox / Autocomplete / DropdownMenu）
- [x] 第二阶段：collections_command.rs 全组片段自包含化（Command / ContextMenu / Menubar / NavigationMenu / Carousel / CommandDialog）
- [x] 第二阶段收尾：files / collections_extra / collections / actions / forms_extra_search / overlays / overlays_extra / overlays_dialog / overlays_alert_dialog 残余外部依赖片段清零（copy-ready 自包含）
- [x] 全仓 Playground `code` -> `code_signal` 迁移完成
- [x] 增加回归测试：controls 必须配 code_signal、禁止旧式中间 let 片段、尺寸 xs/s/m/l/xl
- [x] 增加回归测试：全量 Playground 禁止旧式 `code=`、强制 `code_signal=`、snippet 禁止 `{content}/{rows}/{chips}/{grid}` 占位与常见外部绑定依赖

## 分组结果

- [x] Actions：0
- [x] Collections：0
- [x] Display：0
- [x] Files：0
- [x] Forms：0
- [x] Layout：0
- [x] Overlays：0

## 推进规则（执行标准）

- 动态代码片段：参数内联、默认参数尽量省略、尺寸对齐 xs/s/m/l/xl（组件支持时可扩展更大尺寸）。
- 组件 API 强制需要状态信号时，仅保留最小必要 signal，其余中间 let 变量移除。
- 每批改动后执行 `cargo test -p docs-app` 验证。
