# Core/Shell/Protocol/Infrastructure 能力基线

## 目标

把架构从“概念正确”推进到“能力可验证”。  
本文定义四层能力的最低工业化基线，作为评审与回归门槛。

## 1) Core（Logic Kernel）

- 纯 Rust 状态机：状态转移可预测、无直接副作用调用
- Time-Travel Debugging 友好：状态转移由 action 驱动，可回放可重演
- Serializable State：状态可 `Serialize + Deserialize`，支持 SSR 状态传输
- Deterministic ID：禁止直接随机 ID；通过可重放 `IdProvider` 注入，跨 SSR/Hydration 一致

## 2) Shell（View / Adapter）

- Slicer：细粒度响应（`Memo`/派生信号）避免整块广播
- Foreign Zone：命令式第三方库受控接入（有作用域、有清理、有审计）
- Projection Manager：容器 slot 生命周期策略（`Lazy/KeepAlive/Eager`）可执行且可收敛

## 3) Protocol（协作协议）

- Command Pattern：DOM/Async/SideEffects 先语义化为命令，再由执行层落地
- Agent Contract：组件状态/能力通过稳定 schema 与语义标记对外，避免 DOM 猜测

## 4) Infrastructure（底座）

- Workspace：crate 物理隔离、依赖边界清晰、分层不可破
- Changesets 思路发布自动化：采用 `release-plz` fixed-mode 进行版本级联与发布审计
- Token System：防御性样式变量链 + fallback SSOT，避免样式孤岛失控

## 验收清单（DoD）

- 每项能力都有对应 spec 或规则来源，并在 `check2` 可勾选追踪
- 关键组件至少覆盖：
  - 状态回放/收敛路径测试
  - SSR/Hydration 一致性测试
  - 命令执行映射测试
  - Projection 与 Foreign Zone 生命周期测试

## 关联规范

- `docs/spec/kernel_shell_architecture.md`
- `docs/spec/side_effect_command_pattern.md`
- `docs/spec/ssr_hydration_discontinuity.md`
- `docs/spec/foreign_zone_escape_hatches.md`
- `docs/spec/slot_projection_strategy.md`
- `docs/spec/release_versioning.md`
