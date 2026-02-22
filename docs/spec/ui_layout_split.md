# UI Layout Split（一次性迁移约束）

> Status: Draft  
> Scope: 把 `layout` 类组件从 `crates/ui` 拆分到 `crates/ui-layout`，且不提供兼容层/过渡层。

## 0. 核心结论

- 组件分三类：`基础组件` / `复合组件` / `layout`。
- `sidebar` 归类为 **复合组件**，不迁移到 `ui-layout`。
- 本次策略是 **一次性迁移**：不保留旧路径 re-export，不保留旧 feature 别名。

## 1. 分类规则（Step 1）

### 1.1 基础组件（Base）

定义：单一语义职责的原子组件，状态轴有限，不编排子区域协议。

### 1.2 复合组件（Composite）

定义：组合多个子区域并维护跨区域行为/状态协议（open/menu/action/selection 等）。

判定信号：
- 有跨 slot 行为编排；
- 有集合或层级状态；
- 有事件回调链条驱动子区域联动。

`sidebar` 满足以上条件（含 open/menu/action/overlay 协议），归复合组件。

### 1.3 Layout 组件

定义：以空间编排/容器结构为主，提供布局能力与结构语义，不承担复杂跨区域业务协议。

判定信号：
- 主要输出布局/容器语义（方向、间距、分栏、容器层次、结构包装）；
- 即使带轻量交互（如滚动/尺寸调整），仍不编排复杂业务 slot 协议。

## 1.4 迁移清单（本次目标）

迁移到 `crates/ui-layout/src/*`：

- `aspect_ratio`
- `auto_height`
- `card`
- `content`
- `divider`
- `flex`
- `footer`
- `grid`
- `header`
- `heading`
- `resizable`
- `scroll_area`
- `scroll_shadow`
- `separator`
- `spacer`
- `surface`
- `view`
- `well`

明确不迁移（保留在 `ui`）：

- `sidebar`
- `sidebar/content`
- `sidebar/footer`
- `sidebar/group`
- `sidebar/header`
- `sidebar/inset`
- `sidebar/menu`
- `sidebar/menu_action`
- `sidebar/menu_badge`
- `sidebar/rail`
- `sidebar/trigger`

## 2. 依赖拓扑约束（Step 2）

目标 DAG：

`ui-state-primitives` -> `ui-headless` -> `ui-layout` -> `ui` -> `apps/*`

横向依赖：

- `ui-theme`、`ui-motion` 可被 `ui-layout` 与 `ui` 依赖。

硬约束：

1. `ui-layout` 禁止依赖 `ui`。
2. `ui-layout` 禁止依赖 `apps/*` 或任何临时兼容桥接层。
3. `ui` 允许依赖 `ui-layout`（用于复合组件拼装）。
4. 一次性迁移提交中删除旧模块路径，不保留兼容导出。

## 2.1 一次性迁移验收（无过渡）

必须同时满足：

1. `crates/ui/src` 不再包含上述 layout 目录。
2. `crates/ui-layout/src` 出现对应 layout 目录。
3. 全部调用方（`ui`/`apps`/tests/docs）更新到新路径。
4. 不存在旧路径兼容层（`#[path]` 转发、`pub use` 旧名导出）残留。
5. workspace 门禁通过（至少 `check + test + clippy` 针对受影响 crate）。
