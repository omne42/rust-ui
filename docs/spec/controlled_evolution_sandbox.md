# 受控演化沙盒（Contrib Sandbox + Graduation Path）

> Status: Draft  
> Scope: 在不破坏核心质量基线的前提下，为高不确定性创新组件提供官方试验区与可审计毕业路径。

## 0. 核心判断

“只允许完美组件进入系统”会扼杀创新。  
“允许任意实验直接进核心”会污染架构。  
正确做法是：建立受控沙盒，允许不完美存在，但必须隔离、可追踪、可淘汰、可毕业。

## 1. 双区模型（Cleanroom vs Sandbox）

### 1.1 Cleanroom（核心洁净区）

适用范围：`ui-state-primitives/ui-headless/ui-theme/ui-motion/ui-components` 主路径。  
要求：完整门禁、A11y 基线、SSR/WASM 契约、分层不破、语义测试可回归。

### 1.2 Sandbox（演化培养皿）

建议载体：`crates/ui-contrib`（或等价 contrib workspace）。  
目标：容纳探索型组件（例如 Draggable Kanban Board）在“尚未成熟”阶段的合法生存。

允许的暂时性缺口（必须显式声明）：

- A11y 未完备（但必须标注缺失项）
- SSR 暂不支持
- 性能尚未优化
- 局部命令式实现尚未彻底抽象

## 2. 准入与隔离规则

进入 `ui-contrib` 必须满足最低约束：

- 明确标记实验状态：`experimental = true`
- 明确风险标签：`a11y_incomplete` / `ssr_unsupported` / `perf_unverified` 等
- 明确作用域：不得反向污染核心 crate API
- 明确退出策略：有 owner、里程碑和失效清理日期

隔离原则：

- `ui-contrib` 组件默认不进入核心 `all-components` 导出
- 应用侧需显式 opt-in 才可使用
- 禁止把 contrib 的临时约束倒灌为核心标准

## 3. 毕业路径（Graduation）

任何 contrib 组件要进入核心，必须完成“毕业审查”。

### 3.1 毕业前置条件

- 多团队/多场景复用证据（不是单点 demo）
- API 收敛（删除临时参数、命名统一）
- 关键 A11y 路径补齐并可测试
- SSR/WASM 能力明确（支持或受控降级）
- 语义测试/E2E 回归就位

### 3.2 毕业流程

1. 提交 Graduation RFC（问题、现状、风险、迁移计划）
2. 核心团队评审并确定目标域归属
3. 在核心架构下重构（logic/styles/motion/view + 契约测试）
4. 完成门禁并迁入核心 crate
5. 标记 contrib 版本为 deprecated，给出迁移窗口

## 4. 生命周期治理（防熵增）

每个 contrib 组件必须有生命周期状态：

- `incubating`：孵化中
- `adopted`：被多方采纳，待毕业
- `graduated`：已迁入核心
- `retired`：淘汰下线

治理要求：

- 定期审计“长期孵化未收敛”组件
- 对无 owner 或无里程碑的实验直接清退
- 禁止“永久实验态”成为技术债垃圾场

## 5. 与现有规则的关系

- 本规范不是降低核心质量标准，而是把“不完美创新”限制在受控边界内。
- Cleanroom 规则继续生效，Sandbox 只是提供受控例外路径。

关联文档：

- `docs/spec/component_boundaries.md`
- `docs/RULES_ZH.md`
- `docs/plan/TODO.md`
