# Markdown 全量索引

该索引覆盖仓库自有 Markdown 文档，并登记外部文档入口。

范围说明：
- 包含：仓库自有 docs、crate/app README、根目录 markdown
- 不逐条展开：`examples/_upstream/**`（research mirror文档）
- 不逐条展开：`crates/ui/src/**/check2.md`（组件级执行清单）
- 外部文档以 `External` 状态登记

## A. 根目录文档

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `AGENTS.md` | Core | Active | 仓库级协作与执行约束 |
| `README.md` | Core | Active | 仓库总览与快速开始 |
| `CHANGELOG.md` | Core | Active | 发布与变更记录 |
| `todo.md` | Core | Active | Playground TODO 迁移指针 |

## B. 文档系统层

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/README.md` | Core | Active | 文档统一入口与阅读路径 |
| `docs/DOCS_INDEX.md` | Core | Active | Markdown 全量索引 |
| `docs/DOCS_GOVERNANCE.md` | Core | Active | 文档分类、状态与维护规则 |
| `docs/RULES_ZH.md` | Core | Active | 架构与工程硬规则 |
| `docs/philosophy.md` | Core | Active | 哲学总纲与战略方向 |
| `docs/起点_也即是目的.md` | Core | Reference | 起点愿景与概念性架构背景 |

## C. 规格层（Spec）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/spec/README.md` | Spec | Active | 规格层导航 |
| `docs/spec/mvp.md` | Spec | Active | MVP 范围与 DoD |
| `docs/spec/motion.md` | Spec | Active | 动效架构约束 |
| `docs/spec/styling.md` | Spec | Active | 样式系统规范 |
| `docs/spec/style_island_defense.md` | Spec | Active | 样式孤岛防御（防御性变量 + 懒加载注入 + SSOT） |
| `docs/spec/tree_shaking.md` | Spec | Active | Tree Shaking / 组件级裁剪规范 |
| `docs/spec/playground_standard.md` | Spec | Active | docs-app Playground 结构、API 覆盖与强制检测规范 |
| `docs/spec/i18n.md` | Spec | Draft | i18n / l10n 注入契约（组件层） |
| `docs/spec/component_boundaries.md` | Spec | Draft | UI 组件边界与跨层归属规则 |
| `docs/spec/component_domains.md` | Spec | Draft | 同功能同域目录收敛与兼容策略 |
| `docs/spec/ui_layout_split.md` | Spec | Draft | 三类组件分类与 `ui-layout` 一次性拆分约束 |
| `docs/spec/heroui-parameter-design-strategy.md` | Spec | Draft | 参数模型对齐策略 |
| `docs/spec/hyper-structure-ui-development-playbook.md` | Spec | Draft | AI Verified/Struct-First 执行手册 |
| `docs/spec/side_effect_command_pattern.md` | Spec | Draft | 逻辑层副作用命令模式（State + Command） |
| `docs/spec/release_versioning.md` | Spec | Draft | 版本级联与发布策略（release-plz + Fixed Mode） |
| `docs/spec/wasm_generic_bloat.md` | Spec | Draft | WASM 泛型单态化膨胀治理（体积优先） |
| `docs/spec/ui_physics_two_pass_rendering.md` | Spec | Draft | 几何决策两段式渲染（Intent/Measure/Rectification） |
| `docs/spec/async_state_as_data_command.md` | Spec | Draft | 异步阻抗治理（State as Data, Async as Command） |
| `docs/spec/headless_purification.md` | Spec | Draft | Headless 去状态化（状态机上交 logic，语义映射留在 headless） |
| `docs/spec/macro_micro_dual_state_machine.md` | Spec | Draft | 宏观/微观双状态机（连续物理本地执行，边界状态回流） |
| `docs/spec/collection_registration_protocol.md` | Spec | Draft | 集合组件注册协议（动态发现 + 顺序收敛） |
| `docs/spec/environment_subscription_streams.md` | Spec | Draft | 环境订阅流（语义化环境输入 + 采样节流） |
| `docs/spec/kernel_shell_architecture.md` | Spec | Draft | Kernel/Shell 工业化总线（Infrastructure + Logic + View） |
| `docs/spec/ssr_hydration_discontinuity.md` | Spec | Draft | SSR 时空断裂治理（State Transfer + Deterministic Seed） |
| `docs/spec/slot_projection_strategy.md` | Spec | Draft | 插槽投影策略（Lazy/KeepAlive/Eager + 生命周期通知） |
| `docs/spec/core_shell_protocol_infra_baseline.md` | Spec | Draft | 四层能力基线（Core/Shell/Protocol/Infrastructure） |
| `docs/spec/event_light_cone_signal_bus.md` | Spec | Draft | 事件光锥限制（Context Bus + Selector + 状态压缩） |
| `docs/spec/unified_causality_bus.md` | Spec | Draft | 统一因果总线（TraceId + Event Sourcing + 因果图） |
| `docs/spec/compile_time_evolution_migration.md` | Spec | Draft | 架构热寂治理（编译期演化 + 自动迁移） |
| `docs/spec/intent_stack_semantic_layering.md` | Spec | Draft | 意图分层（组件意图 -> 业务意图 -> 应用编排） |
| `docs/spec/architectural_fitness_functions.md` | Spec | Draft | 架构适应性免疫系统（Fitness Functions + CI 阻断） |
| `docs/spec/platform_abdication_ecosystem.md` | Spec | Draft | 平台退位与生态自演化（工厂/治理/脚手架优先） |
| `docs/spec/foreign_zone_escape_hatches.md` | Spec | Draft | 命令式第三方接入的受控外交特区（Escape Hatches） |
| `docs/spec/focus_global_stack_gc.md` | Spec | Draft | 焦点全局栈与墓地回收（Focus Manager + Graveyard GC） |
| `docs/spec/ai_context_projection_protocol.md` | Spec | Draft | AI 上下文压缩协议（Component Manifest + RBI 接口投影） |
| `docs/spec/controlled_evolution_sandbox.md` | Spec | Draft | 受控演化沙盒（ui-contrib 实验区 + Graduation Path） |
| `docs/spec/state_primitives_core_satellite_split.md` | Spec | Draft | 状态原语 Core/Satellite 拆分（核心纯度 + 卫星扩展） |

## D. 计划层（Plan）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/plan/README.md` | Plan | Active | 计划层导航 |
| `docs/plan/IMPLEMENTATION_PLAN.md` | Plan | Active | 里程碑与执行主计划 |
| `docs/plan/COMPONENT_LAYER_REFACTOR_SHARDS.md` | Plan | Draft | 组件分层整改分片计划 |
| `docs/plan/TODO.md` | Plan | Active | 人类可执行任务与门禁 |
| `docs/plan/PLAYGROUND_TODO.md` | Plan | Active | docs-app playground 任务追踪 |
| `docs/plan/SUBAGENT_TASK_TEMPLATE.md` | Plan | Active | 子任务模板 |

## E. 调研层（Research）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/research/README.md` | Research | Active | 调研入口与上游定位 |
| `docs/research/spectrum-heroui-style-interface-study.md` | Research | Reference | Spectrum × HeroUI 综合研究 |
| `docs/research/spectrum-design-primitives-search-2026-02-14.md` | Research | Reference | Spectrum 设计元语样式基准检索记录 |
| `docs/research/android-spike.md` | Research | Reference | Android/WebView 可行性记录 |
| `docs/research/bb_ui-web_notes.md` | Research | Reference | 外部项目经验记录 |

## F. Crate 级 README

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `crates/ui-state-primitives/README.md` | Package | Active | `ui-state-primitives` 目标与 API |
| `crates/ui-headless/README.md` | Package | Active | `ui-headless` 交互/A11y 契约 |
| `crates/ui-theme/README.md` | Package | Active | `ui-theme` token 与变量用法 |
| `crates/ui-motion/README.md` | Package | Active | `ui-motion` 运行时与 API |
| `crates/ui-visual-primitive/README.md` | Package | Active | `ui-visual-primitive` 内部视觉原语边界 |
| `crates/ui/README.md` | Package | Active | `ui` 组合与用法 |

## G. 应用级 README

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `apps/web-demo/README.md` | App | Active | demo 运行与样式覆盖流程 |
| `apps/docs-app/README.md` | App | Active | docs-app 运行说明与坑位 |
| `apps/tauri-demo/README.md` | App | Active | tauri-demo 运行说明 |

## H. 外部文档（登记，不治理）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `vendor/tachys/README.md` | External | External | vendored 第三方文档 |
| `examples/_upstream/**/*.md` | External | External | research mirror文档（本地调研用） |

## 维护规则

新增/删除仓库自有 Markdown 时，必须同 PR 更新本索引。
