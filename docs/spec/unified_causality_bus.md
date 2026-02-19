# 统一因果总线（Unified Causality Bus）

## 问题定义

当系统被拆成 Logic、Command Executor、Signal Bus、Focus Manager、Layout/Measure 等多层后，  
单个组件的 time-travel 只能解释“本地状态正确”，却无法还原“跨系统涟漪效应”。

典型症状：

- 触发点在 Button，但损坏出现在 Avatar
- 每个子系统日志都正确，整体因果链仍不可见
- Bug 复现后无法回答“这次点击到底经过了哪些系统”

## 核心判断

- 仅有组件级调试器不够，必须有跨系统统一事件溯源。
- 没有统一关联 ID 的 UI 运行时，本质上不可观测。

## 标准解法

### 1) 全局关联 ID（Correlation / TraceId）

每个用户初始输入事件（click/keydown/pointerdown）必须分配唯一 `TraceId`。  
后续所有派生事件都携带该 `TraceId`：

- `Action`
- `Command`
- `SignalBus` 广播
- Focus/Layout/Env manager 的处理记录

### 2) 统一因果日志（Event Sourcing）

所有系统统一写入结构化日志项：

```text
[TraceId:123] Action::Click -> ButtonLogic
[TraceId:123] Command::EmitSignal -> Executor
[TraceId:123] SignalBus::Broadcast -> 3 subscribers
[TraceId:123] FocusManager::RequestFocusReturn
[TraceId:123] AvatarLogic::StateChanged
```

最小字段要求：

- `trace_id`
- `component/system`
- `event_kind`
- `payload_summary`
- `timestamp`（单调时钟优先）

### 3) 因果图视图（Causality Graph）

调试入口应支持按 `TraceId` 聚合并渲染完整链路，而非只看单组件状态。  
目标是一次查询看到：触发源 -> 派生命令 -> 总线广播 -> 各订阅者影响。

## 分层约束

- `logic.rs`：
  - 允许：携带 `trace_id` 的 action/command 处理
  - 禁止：丢弃 trace 上下文导致链路断裂
- `view.rs` / executor / managers：
  - 允许：补充运行时观测点并续传 `trace_id`
  - 禁止：新建“无父因”的匿名事件（除非明确是系统源事件）

## 性能与采样

- 默认开发环境全量追踪；生产环境可采样，但必须保留关联链一致性。
- 不得因追踪而阻塞主交互路径；日志写入应异步或轻量缓冲。

## 测试要求

- 单测：`trace_id` 在 action->command->effect 链路不丢失。
- 集成测试：一次用户事件可聚合出完整因果链。
- 回归测试：跨组件副作用 bug 可通过 `TraceId` 定位到首个异常节点。
