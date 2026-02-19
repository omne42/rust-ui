# UI 物理学：两段式渲染与布局快照（Geometry Paradox）

## 问题定义

`logic.rs` 是纯逻辑层，但几何决策依赖 DOM 物理现实（尺寸、坐标、视口剩余空间）。  
典型场景：Tooltip/Popover 需要根据可用空间决定最终朝向。

矛盾点：

- 逻辑层如果直接读取 DOM，会破坏分层边界与可测试性。
- 逻辑层如果不读取几何信息，又无法做“上/下/左/右”这类布局决策。

## 核心判断

- 几何相关组件不能假设“一次状态更新就能得到最终布局”。
- 必须采用“意图 -> 测量 -> 修正”的协商流程。

## 标准解法

采用 Two-Pass Rendering + Layout Snapshot：

### Phase 1: Intent（逻辑层）

`logic.rs` 先输出意图状态，不做最终几何结论：

- 例：`TooltipState::Open { preferred_placement: Bottom }`

### Phase 2: Measure（视图层副作用）

`view.rs` 渲染目标节点并执行测量副作用，生成纯数据快照：

- 例：`LayoutSnapshot { anchor_rect, panel_rect, viewport_rect, scroll_offset }`

### Phase 3: Rectification（逻辑层纯计算）

`logic.rs` 接收 `LayoutSnapshot` 并做纯数学碰撞/越界修正，输出最终状态：

- 例：`TooltipState::Open { actual_placement: Top }`

## 分层约束

- `logic.rs`：
  - 允许：`LayoutSnapshot` 纯数据、几何数学计算、状态修正
  - 禁止：DOM 引用、`web_sys` 对象、直接测量调用
- `view.rs` / adapter：
  - 允许：测量 DOM、构造 `LayoutSnapshot`、回传测量结果
  - 禁止：绕过逻辑层直接决定业务几何状态

## 死循环防护

- 修正计算必须幂等：同一 `LayoutSnapshot` 多次输入应得到同一结果。
- 增加稳定相等门：若 `actual_placement` 与已生效状态相同，不再触发新一轮测量。
- 测量回调只在必要条件下触发（首次打开、尺寸变化、视口变化），避免无条件每帧回写状态。

## 测试要求

- 逻辑单测：给定 `LayoutSnapshot`，断言碰撞修正输出（上/下翻转、边界夹取）。
- 集成测试：验证“打开 -> 测量 -> 修正”链路可收敛且无更新震荡。
- 回归测试：视口变化、滚动变化、内容尺寸变化时，最终布局保持可预测。
