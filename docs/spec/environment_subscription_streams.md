# 环境订阅流规范（Environment Subscription Streams）

## 问题定义

UI 组件不仅接收用户动作，也持续暴露在动态环境输入中：

- 视口尺寸变化（resize / orientation）
- 系统主题/媒体偏好变化（media query）
- 元素可见性变化（intersection）

这些输入不是用户显式点击，却会直接影响组件行为。

## 核心判断

- 环境是“只读输入流”，不是业务状态机的副本。
- 逻辑层应感知环境语义，而不是接收未经筛选的高频原始事件。

## 标准解法

### 1) 语义化环境输入

`logic.rs` 只消费高层语义 action，例如：

- `Action::BreakpointChanged(Breakpoint)`
- `Action::ColorSchemeChanged(ColorScheme)`
- `Action::VisibilityChanged(VisibilityState)`

禁止把原始 `resize`、`scroll`、`intersection` 每次回调直接透传到逻辑层。

### 2) 采样与阈值触发

`view.rs` / adapter 负责环境监听与过滤：

- 使用 `ResizeObserver` / media query listener / intersection observer
- 仅在断点变化、可见性跨阈值、布局策略需重算时回传 action
- 高频波动需节流/去抖/阈值门控，避免“事件风暴”

### 3) Pull/Push 分流

- 低频语义（如移动端/桌面端切换）：`view -> Action` 推送给 logic
- 高频几何跟随（如 popover 自动定位）：logic 发 `Command::StartAutoPosition`，由 view/motion 本地循环执行（`requestAnimationFrame` 等）

逻辑层不持有逐帧坐标，交互结束或策略变化时再收敛。

## 分层约束

- `logic.rs`：
  - 允许：消费语义化环境 action，做策略切换与离散决策
  - 禁止：直接监听浏览器环境事件、持有 observer 句柄
- `view.rs` / adapter：
  - 允许：监听环境事件、采样过滤、执行高频本地循环
  - 禁止：绕过逻辑层直接改宏观业务状态

## 性能与一致性要求

- 禁止“每次环境变化都入 logic”的无门控推送模式。
- 高频环境循环必须可启动/停止，避免后台空转。
- 环境输入与用户输入冲突时，优先保证状态机可收敛与可预测。

## 测试要求

- 单测：环境语义 action 触发后的策略切换正确。
- 集成测试：断点切换、主题切换、可见性变化触发正确行为且无抖动。
- 回归测试：高频 resize/scroll 下无明显状态风暴与性能退化。
