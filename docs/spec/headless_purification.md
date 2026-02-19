# Headless 去状态化规范（Headless Purification）

## 问题定义

常见 Headless 库会把交互状态机封装在内部 Hook 中（例如 focus trap、roving tabindex、grid 导航）。  
这样会导致组件体系出现双状态源：

- `logic.rs` 维护业务状态
- `ui-headless` 再维护一份内部交互状态

结果是状态所有权不清晰，调试与测试难度上升，组件行为难以预测。

## 核心判断

- `ui-headless` 只能做语义映射与输入归一化，不能持有业务可观察状态。
- 交互状态机所有权必须上交给 `ui-state-primitives` / `logic.rs`。

## 标准解法

### 1) Headless 只做纯映射

错误形态（禁止）：

```rust
let (props, internal_state) = use_tabs();
```

正确形态（要求）：

```rust
let a11y = tabs_accessibility(&logic_state, &env);
```

`ui-headless` 输入是外部状态快照与事件信息，输出是 `attrs/handlers/action-intent`，不保留隐藏状态。

### 2) 状态机上收至 primitives/logic

像 `focus trap`、`roving tabindex`、`grid nav` 这类复杂交互状态机：

- 状态定义放在 `ui-state-primitives`
- 状态转移放在 `logic.rs`（或其依赖 primitive reducer）
- `ui-headless` 仅把状态翻译为 ARIA/键盘语义与事件意图

### 3) 事件处理走意图回流

- `ui-headless` 可把浏览器事件归一为 typed intent/action
- `logic.rs` 消费 action 更新状态
- `view.rs` 负责挂载 attrs/handlers，不直接承载隐藏状态机

## 分层约束

- `ui-state-primitives` / `logic.rs`：
  - 允许：交互状态结构、状态转移、可测试不变量
  - 禁止：DOM 细节与样式语义
- `ui-headless`：
  - 允许：A11y 属性映射、键盘规则映射、输入归一
  - 禁止：内部 `Signal`/Hook 状态源、隐藏状态机、第二事实来源

## 第三方库接入策略

- 禁止直接把“自带内部状态”的第三方 headless hook 暴露到组件公共路径。
- 若确需复用第三方能力，必须通过 adapter 去状态化：
  - 外部状态由本仓 `logic/primitives` 持有
  - 第三方能力仅作为纯计算/映射步骤使用

## 测试要求

- 单测：状态机行为在 `ui-state-primitives` / `logic` 可独立验证。
- 语义测试：`ui-headless` 输出 `aria-*`/键盘语义映射正确。
- 集成测试：验证“状态单一来源”原则，不存在 headless 与 logic 状态漂移。
