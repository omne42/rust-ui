# 宏观/微观双状态机（Macro/Micro Dual State Machine）

## 问题定义

在高频连续交互（如 touch drag）中，若每一帧都走完整链路：

`View Event -> Action -> logic.rs -> NewState -> View`

会引入桥接开销（JS/WASM 边界、序列化/反序列化、状态传播），导致掉帧与跟手性下降。

典型场景：Drawer/Sheet 拖拽关闭。

- 宏观状态：`Open / Closed / Dragging`
- 微观状态：`current_pixel_offset`、`velocity`

## 核心判断

- 逻辑是离散决策层，不应承担每帧连续物理积分。
- 连续物理反馈应在渲染层或 motion runtime 本地执行。

## 标准解法

采用 Dual State Machine：

### 1) Macro State（logic 绝对控制）

`logic.rs` 只负责离散状态与关键边界动作：

- `Action::DragStart`
- `Action::DragEnd { final_offset, final_velocity }`
- 判定最终归宿（回弹 / 关闭 / 保持打开）

### 2) Micro State（view/motion 层局部控制）

拖拽进行中（`Dragging`）：

- `view.rs` 或 `ui-motion` 直接更新 DOM/CSS variables/本地 spring
- 高频位移与速度更新绕过 logic 主状态机，避免每帧跨层往返

### 3) Reconciliation（结束时和解）

拖拽结束时只回传一次收敛输入给逻辑层：

- `Action::DragEnd { final_offset, final_velocity }`

`logic.rs` 计算并产出最终稳定宏观状态，收回控制权。

## 分层约束

- `logic.rs`：
  - 允许：离散状态机、阈值判定、最终状态决策
  - 禁止：每帧像素级状态写回、直接驱动 DOM 逐帧更新
- `view.rs` / `ui-motion`：
  - 允许：高频连续更新（offset/velocity/spring tick）
  - 禁止：绕过逻辑层永久持有宏观业务状态

## 性能与一致性要求

- 拖拽进行期应最小化桥接次数，避免每帧 Action 往返。
- 视觉反馈必须本地连续（60/120Hz 目标），逻辑层只在边界事件参与。
- 微观状态仅作为临时物理态，必须在 `DragEnd` 后与宏观状态收敛一致。

## 测试要求

- 逻辑单测：`DragEnd` 阈值与速度判定（close/rebound/open）。
- 集成测试：拖拽过程流畅，结束后状态收敛正确，无“视觉关闭但逻辑仍 open”分裂。
- 回归测试：高频输入下无明显桥接抖动，结束状态稳定可重复。
