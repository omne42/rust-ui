# SSR 时空断裂治理（Hydration Discontinuity）

## 问题定义

SSR 与 Hydration 跨越了时间与运行时边界。  
若逻辑初始化依赖“当前时间”或“随机值”，服务端与客户端会得到不同初始状态，触发 hydration mismatch。

典型风险：

- `Date::now()` 在服务端与客户端时间点不同
- `Uuid::new_v4()` 在两端生成不同 ID，导致 `aria-labelledby` 关联失效

## 核心判断

- Hydration 不是“客户端再跑一次初始化”，而是“客户端恢复服务端状态”。
- 状态必须先于代码分歧到达客户端。

## 标准解法

### 1) 确定性种子（Deterministic Seeding）

逻辑层不得直接依赖非确定性源（系统时间、随机 UUID）。  
必须通过上下文注入可重放提供者（如 `IdProvider` / `NowProvider`）。

- SSR 阶段：按可预测规则产出（计数器、固定种子）
- Hydration 阶段：以同种子重置并复现同序列

### 2) 状态传输协议（State Transfer Protocol）

服务端流程：

1. 运行 logic，得到 `FinalState`
2. 渲染 HTML
3. 同时序列化 `FinalState`（JSON）嵌入页面（如 `<script id=\"server-state\">`）

客户端流程：

1. 读取 `server-state` JSON
2. 调用 `Logic::hydrate(serialized_state)` 恢复状态
3. 禁止再次执行会产生新随机/新时间的初始化路径

## 分层约束

- `logic.rs`：
  - 允许：`Serialize + Deserialize` 状态模型、`hydrate` 恢复入口
  - 禁止：直接读取不可重放环境源作为初始状态
- `view.rs` / app boot：
  - 允许：注入 `server-state`、启动 hydrate 路径
  - 禁止：在 hydrate 前覆盖服务端状态

## 一致性规则

- 首帧必须与 SSR HTML 语义一致，不得首帧闪烁重算。
- ID 生成必须跨 SSR/CSR 稳定一致，A11y 关联不失效。
- 若无 `server-state`（纯 CSR 路径），才允许走 `Logic::new()` 初始化。

## 测试要求

- 单测：状态序列化/反序列化后语义不变。
- 集成测试：SSR 输出与 hydration 首帧无 mismatch。
- 回归测试：ID 与时间相关组件（DatePicker/Popover label）在 SSR+Hydration 下稳定。
