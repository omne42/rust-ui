# 意图分层规范（Intent Stack / Semantic Layering）

## 问题定义

若底层通用组件直接发出业务行为命令（如 `ShowToast("已添加")`），  
组件会被迫耦合具体业务上下文（A/B、营销策略、应用流程），导致：

- 通用组件职责污染
- 业务变更反向侵入组件底层
- 复用能力持续下降

## 核心判断

- 行为是组件触发的，但“意义”属于应用层解释。
- 组件层应表达“发生了什么”，而不是“业务上该做什么”。

## 标准解法

### 1) 组件意图层（Component Intent）

底层组件只发通用语义意图，例如：

- `Intent::InteractionSubmitted`
- `Intent::SelectionChanged`
- `Intent::DismissRequested`

禁止在通用组件中硬编码业务语义（购物车、支付、推荐实验等）。

### 2) 业务翻译层（Domain Intent）

业务组件/容器订阅通用意图并翻译成业务意图：

- `InteractionSubmitted` -> `ItemAddedToCart`

该层可依赖业务上下文，不污染通用组件。

### 3) 应用编排层（Application Orchestrator）

全局编排服务消费业务意图，结合配置（A/B、实验平台、策略中心）发出最终命令：

- `Command::ShowToast`
- `Command::PlayIconAnimation`
- `Command::TrackExperiment`

## 分层约束

- `ui-components`：
  - 允许：通用交互意图与 UI 语义状态
  - 禁止：业务策略判断、实验分流逻辑、业务文案决策
- 业务组件层：
  - 允许：通用意图 -> 业务意图翻译
  - 禁止：把业务耦合回灌通用组件
- 应用编排层：
  - 允许：业务策略与最终行为执行决策
  - 禁止：绕过意图链直接篡改底层组件内部状态机

## 可演进性要求

- 意图命名应稳定、抽象、可复用，避免一次性业务词汇。
- A/B 或策略变化应主要落在编排层，不应触发底层组件 API 破坏。
- 意图链路需可观测（建议与 `TraceId` 关联）。

## 测试要求

- 单测：组件只输出通用意图，不输出业务命令。
- 集成测试：业务翻译层将通用意图正确映射为业务意图。
- 回归测试：切换策略（A/B）时无需改动底层组件逻辑。
