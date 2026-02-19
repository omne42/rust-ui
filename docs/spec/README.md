# 规格文档导航

规格文档用于定义“要做什么”和“约束是什么”，属于规范层文档。

## 优先级

在文档系统中，规格层优先级低于 `docs/philosophy.md` 与 `docs/RULES_ZH.md`，高于调研层文档。

## 当前文件

- `docs/spec/mvp.md`
  - MVP 范围、非目标、验收清单

- `docs/spec/motion.md`
  - 动效分层与契约约束

- `docs/spec/styling.md`
  - 样式体系、注入/覆盖策略与禁用项

- `docs/spec/style_island_defense.md`
  - 样式孤岛防御：Defensive Variables、Fallback SSOT、Lazy Injection 约束

- `docs/spec/tree_shaking.md`
  - Tree Shaking / 组件级裁剪策略与验收契约

- `docs/spec/i18n.md`
  - i18n / l10n 注入契约（组件层）

- `docs/spec/component_boundaries.md`
  - UI 组件边界：什么是组件，什么不是

- `docs/spec/component_domains.md`
  - 同功能同域的目录收敛与兼容导出策略

- `docs/spec/ui_layout_split.md`
  - 三类组件分类与 `ui-layout` 一次性拆分约束（无兼容层）

- `docs/spec/heroui-parameter-design-strategy.md`
  - 参数模型演进与组件 API 设计策略

- `docs/spec/hyper-structure-ui-development-playbook.md`
  - AI Verified / Struct-First 的执行手册

- `docs/spec/side_effect_command_pattern.md`
  - 逻辑层副作用命令契约（`State + Command` 分离）

- `docs/spec/release_versioning.md`
  - 版本级联与发布策略（release-plz + Fixed Mode）

- `docs/spec/wasm_generic_bloat.md`
  - WASM 单态化膨胀治理（泛型收敛与体积优先）

- `docs/spec/ui_physics_two_pass_rendering.md`
  - 几何决策两段式渲染（Intent/Measure/Rectification）

- `docs/spec/async_state_as_data_command.md`
  - 异步阻抗治理（State as Data, Async as Command）

- `docs/spec/headless_purification.md`
  - Headless 去状态化：状态机上交 primitives/logic，headless 只做语义映射

- `docs/spec/macro_micro_dual_state_machine.md`
  - 宏观/微观双状态机：连续物理反馈局部执行，边界状态回流 logic

- `docs/spec/collection_registration_protocol.md`
  - 集合组件注册协议：动态发现子项与顺序收敛（Register/Unregister）

- `docs/spec/environment_subscription_streams.md`
  - 环境订阅流：环境输入语义化、采样节流与 Pull/Push 分流

- `docs/spec/kernel_shell_architecture.md`
  - Kernel/Shell 工业化总线：Infrastructure + Logic Kernel + View Shell

- `docs/spec/ssr_hydration_discontinuity.md`
  - SSR 时空断裂治理：状态传输协议 + 确定性种子

- `docs/spec/slot_projection_strategy.md`
  - 插槽投影策略：Lazy/KeepAlive/Eager 与容器生命周期治理

- `docs/spec/core_shell_protocol_infra_baseline.md`
  - Core/Shell/Protocol/Infrastructure 能力基线（Time-Travel/Agent Contract/Changesets）

- `docs/spec/event_light_cone_signal_bus.md`
  - 事件光锥限制：大集合通信采用 Context Bus + Selector 订阅

- `docs/spec/unified_causality_bus.md`
  - 统一因果总线：TraceId 贯穿 Action/Command/Bus/Manager 的全链路观测

- `docs/spec/compile_time_evolution_migration.md`
  - 架构热寂治理：Schema Registry + 迁移函数 + Codemod + 编译期淘汰

- `docs/spec/intent_stack_semantic_layering.md`
  - 意图分层：组件语义意图 -> 业务意图 -> 应用编排命令

- `docs/spec/architectural_fitness_functions.md`
  - 架构适应性免疫系统：把架构原则转成可执行测试并强制 CI 验证

- `docs/spec/platform_abdication_ecosystem.md`
  - 平台退位：交付“工厂与法则”，让生态按同一标准自演化

- `docs/spec/foreign_zone_escape_hatches.md`
  - 命令式第三方库接入的受控外交特区（YieldControl/CleanupForeign）

- `docs/spec/focus_global_stack_gc.md`
  - 焦点全局栈与墓地回收（层叠弹层/强制卸载下的焦点连续性）

- `docs/spec/ai_context_projection_protocol.md`
  - AI 上下文压缩协议：`Component.toml + .rbi` 索引层，降低源码全量读取导致的幻觉

- `docs/spec/controlled_evolution_sandbox.md`
  - 受控演化沙盒：`ui-contrib` 实验区 + 毕业迁移路径（防教条扼杀创新，防实验污染核心）

- `docs/spec/state_primitives_core_satellite_split.md`
  - 状态原语 Core/Satellite 拆分：核心纯度约束 + 卫星扩展路径 + Litmus Test

## 使用方式

1. 从 `docs/plan/TODO.md` 选择任务范围
2. 阅读对应规格文档
3. 按规格约束实现
4. 通过门禁与测试再合入

## 关联文档

- 硬规则：`docs/RULES_ZH.md`
- 哲学总纲：`docs/philosophy.md`
- 计划层：`docs/plan/README.md`
