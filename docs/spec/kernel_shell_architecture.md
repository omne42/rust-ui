# Kernel/Shell 工业化架构总线规范

## 目标

将前述分层原则收敛为一套可执行的“总线契约”，确保组件在复杂场景下仍保持：

- 可预测（状态单一来源）
- 可扩展（职责可插拔）
- 可验证（规则可测试）

## 架构全景

### 1) Infrastructure（底座）

- Cargo Workspace：物理隔离、版本锁定、crate 职责边界明确
- Token System：常量注入、防御性 CSS 变量链、样式回退可追溯

### 2) Kernel（Logic）

- State Machine：纯 Rust 离散状态机（可单测）
- Command Pattern：副作用外包（Async / DOM API / 平台能力）
- Dual State Handling：
  - Macro State 由 logic 控制
  - Micro State 由 view/motion 在高频阶段本地控制
- Registry：集合组件动态注册、顺序收敛、生命周期治理

### 3) Shell（View / Leptos）

- Slicer：通过 `Memo`/派生信号切片消费状态，避免整块广播
- Executor：执行 `Command` 到平台 API（`web_sys`/runtime）
- Physicist：DOM 测量 + 高频物理反馈（拖拽/跟随/布局快照）
- Bridge：环境变化（resize/theme/visibility）语义化后回流 action

## 总线契约

必须满足以下单向协作链路：

`Kernel(State/Command) -> Shell(Render/Execute/Measure) -> Kernel(Action)`

约束：

- Kernel 不直接触碰 DOM/runtime 句柄
- Shell 不旁路修改宏观业务状态
- 高频路径不强行每帧穿越 Kernel
- 收敛点必须显式（DragEnd / MeasureDone / RequestCompleted / BreakpointChanged）

## 失败信号（反模式）

- 逻辑层持有 `Future`、DOM 引用、observer 句柄
- Headless 或 view 成为第二状态源
- 集合组件无注册协议，键盘导航依赖偶然顺序
- 环境事件未经门控直接洪泛到 logic

## 验收要求

- 每个复杂组件需在 `check2` 小骨架勾选：
  - 副作用命令化
  - Macro/Micro 分治
  - 动态注册协议
  - 环境订阅流语义化
- 语义测试与集成测试需证明：
  - 状态可收敛
  - 高频交互无明显桥接抖动
  - 动态子项与环境变化下行为稳定
