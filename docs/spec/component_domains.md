# UI Components Domain Grouping（同功能同域）

> Status: Draft  
> Scope: `crates/ui` 目录域收敛与兼容导出策略

## 0. 核心判断

现在的结构是“部分域化 + 平铺导出并存”。这会持续制造重复成本：

- 同族组件文件分散，评审和回归路径变长。
- feature 命名和目录语义不一致，条件编译维护成本高。
- docs-app/playground 与测试难以按域分批推进。

结论：目录必须按功能域收敛，但不能破坏现有公开 API 与 feature 契约。

## 1. 目标与非目标

### 目标

- 同功能组件进入同一目录域（类似 `button/*`）。
- 保留旧导入路径与旧 feature，至少一个迁移窗口。
- 每次迁移按域提交，支持独立回归和回滚。

### 非目标

- 不在本阶段重写组件行为逻辑。
- 不在本阶段调整视觉或交互契约。
- 不在本阶段删除旧 feature。

## 2. Canonical Domain Map

### 当前进展（2026-02-18）

- 已完成：`sidebar`、`theme`、`progress`、`menu`、`icon`、`field_form`、`text_input`、`color`
- 待推进：无（本阶段规划域已收敛）

### P0（先做）

- `sidebar` 域  
  包含：`sidebar*` 全家族（`sidebar_content`、`sidebar_menu_*` 等）
- `theme` 域  
  包含：`theme_dark`、`theme_default`、`theme_express`、`theme_light`
- `progress` 域  
  包含：`progress`、`progress_bar`、`progress_circle`

### P1（第二批）

- `menu` 域  
  包含：`menu`、`menu_trigger`、`menubar`、`navigation_menu`、`context_menu`、`dropdown_menu`、`action_menu`、`dropdown`
- `icon` 域  
  包含：`icon`、`icons`、`icons_ui`、`icons_workflow`、`iconset`
- `field_form` 域  
  包含：`field`、`field_label`、`field_error`、`fieldset`、`form`、`form_field`、`description`、`help_text`

### P2（命名债清理后）

- `text_input` 域  
  包含：`text`、`text_area`、`textarea`、`text_field`、`input*`、`number*`、`date*`、`time_field`
- `color` 域  
  包含：`color_*` 与可并入的 `swatch` 系列

## 3. 兼容导出策略（必须执行）

迁移期间遵守：

1. 旧 `pub mod` 名称保留，通过 re-export 或 `#[path]` 转发到新域。
2. 旧 `component-*` feature 保留，内部可转发到新域 feature bundle。
3. `all-components` 与 `web-demo-components` 保持行为不变。
4. 兼容窗口结束前，不删除旧路径、旧 feature、旧 docs 链接。

## 4. 迁移顺序与门禁

每个域独立按以下步骤执行：

1. 先加兼容层（导出 + feature）
2. 再迁目录
3. 更新 docs-app 引用
4. 运行域级门禁

建议门禁：

- `cargo check -p ui`
- `cargo check -p ui --features all-components`
- 受影响时再加：`cargo check -p docs-app`

## 5. 风险与回滚

主要风险：

- feature 图断裂导致条件编译失败。
- 旧路径被提前删掉导致下游编译失败。
- docs-app 引用未同步导致页面失效。

回滚规则：

- 每个域一个提交单元；失败则整域回滚，不做半回滚。
- 保持兼容层提交独立于目录迁移提交。
