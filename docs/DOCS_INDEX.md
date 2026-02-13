# Markdown 全量索引

该索引覆盖仓库自有 Markdown 文档，并登记外部文档入口。

范围说明：
- 包含：仓库自有 docs、crate/app README、根目录 markdown
- 不逐条展开：`examples/_upstream/**`（上游镜像文档）
- 外部文档以 `External` 状态登记

## A. 根目录文档

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
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
| `docs/spec/heroui-parameter-design-strategy.md` | Spec | Draft | 参数模型对齐策略 |
| `docs/spec/hyper-structure-ui-development-playbook.md` | Spec | Draft | AI Verified/Struct-First 执行手册 |

## D. 计划层（Plan）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/plan/README.md` | Plan | Active | 计划层导航 |
| `docs/plan/IMPLEMENTATION_PLAN.md` | Plan | Active | 里程碑与执行主计划 |
| `docs/plan/TODO.md` | Plan | Active | 人类可执行任务与门禁 |
| `docs/plan/PLAYGROUND_TODO.md` | Plan | Active | docs-app playground 任务追踪 |
| `docs/plan/SUBAGENT_TASK_TEMPLATE.md` | Plan | Active | 子任务模板 |

## E. 调研层（Research）

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `docs/research/README.md` | Research | Active | 调研入口与上游定位 |
| `docs/research/spectrum-heroui-style-interface-study.md` | Research | Reference | Spectrum × HeroUI 综合研究 |
| `docs/research/android-spike.md` | Research | Reference | Android/WebView 可行性记录 |
| `docs/research/bb_ui-web_notes.md` | Research | Reference | 外部项目经验记录 |

## F. Crate 级 README

| Path | Category | Status | Purpose |
| --- | --- | --- | --- |
| `crates/ui-core/README.md` | Package | Active | `ui-core` 目标与 API |
| `crates/ui-headless/README.md` | Package | Active | `ui-headless` 交互/A11y 契约 |
| `crates/ui-theme/README.md` | Package | Active | `ui-theme` token 与变量用法 |
| `crates/ui-motion/README.md` | Package | Active | `ui-motion` 运行时与 API |
| `crates/ui-components/README.md` | Package | Active | `ui-components` 组合与用法 |

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
| `examples/_upstream/**/*.md` | External | External | 上游克隆仓库文档（本地调研用） |

## 维护规则

新增/删除仓库自有 Markdown 时，必须同 PR 更新本索引。
