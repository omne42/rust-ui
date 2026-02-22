# 版本级联与发布策略（Changesets 思路 + Fixed Mode）

## 1. 背景与目标

大型 UI Monorepo 的真实问题不是“怎么手动改版本号”，而是：
- 核心包升级后，依赖它的组件包如何自动级联。
- 多包版本是否保持可理解、可预测。
- 发布流程是否可自动化、可审计、可回滚。

本仓库采用 `release-plz` 作为 Rust 生态下对标 Changesets 的自动化发布工具，并采用 **Fixed Mode（同步版本）** 管理 UI 包版本。

## 2. 核心决策

### 2.1 工具选型

- 方案：`release-plz`
- 定位：自动生成 release PR、统一版本提升、级联更新 workspace 内部依赖、发布 crates

### 2.2 版本策略

- 策略：**Fixed Mode（同步版本）**
- 实现：通过 `release-plz` 的 `version_group` + `release_always = true` 把 UI 包绑定到同一版本节奏
- 结果：`ui-state-primitives/ui-headless/ui-theme/ui-motion/ui-layout/ui/ui-ai-runtime/ui-accordion` 在每次发布窗口统一 bump，版本保持同步

### 2.3 为什么不用 Independent Mode

Independent 模式会导致：
- 使用者难以判断一组组件是否兼容
- 文档示例和问题排查成本显著上升
- 依赖矩阵复杂度指数增长

UI 组件库优先稳定与可理解，Fixed Mode 更符合工程现实。

## 3. 工作流（对齐 Changesets 思路）

1. 日常开发  
   继续按仓库规则提交变更（Conventional Commits + `CHANGELOG.md` 的 `[Unreleased]`）。
2. 进入发布窗口  
   CI 运行 `release-plz release-pr`，自动生成/更新发布 PR。
3. release-plz 在发布 PR 中自动处理  
   - 计算应该 bump 的版本（major/minor/patch）
   - 级联更新 workspace 包间依赖版本
   - 同步 changelog 与发布元数据
4. 合并发布 PR 后  
   由 `release-plz release` 执行实际发布（按仓库发布策略/权限）。

> 说明：`release-plz` 并非 JavaScript 生态 `.changeset/*.md` 文件模型，但在 Rust 场景中承担同等职责：把“版本判断 + 依赖级联 + 发布执行”自动化。

## 4. 仓库落地约束

- apps（`apps/*`）不是 crate 发布对象，不纳入版本组。
- `publish = false` 的包在切换到公开发布前，仍可先走 release PR 流程演练版本级联。
- 任何破坏性变更必须通过 release PR 明确体现版本提升，不允许手工跳过。

## 5. DoD（发布治理）

- 存在 `release-plz.toml` 并声明 UI 包统一 `version_group` + `release_always`
- CI 存在 release-plz workflow（`release-pr` + `release`）
- 文档系统已登记本策略（`docs/spec` + 索引）
