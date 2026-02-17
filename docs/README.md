# 文档系统入口

这是 `rust-ui` 的统一文档入口。

## 快速入口

- 仓库总览：`README.md`
- 哲学总纲（最高优先级）：`docs/philosophy.md`
- 硬规则（次优先级）：`docs/RULES_ZH.md`
- 文档治理：`docs/DOCS_GOVERNANCE.md`
- 全量 Markdown 索引：`docs/DOCS_INDEX.md`

## 文档分层

1. 核心原则层
- `docs/philosophy.md`
- `docs/RULES_ZH.md`

2. 规格层（定义做什么）
- `docs/spec/README.md`

3. 计划层（定义怎么执行）
- `docs/plan/README.md`

4. 调研层（输入与参考）
- `docs/research/README.md`

5. 包与应用说明
- `crates/*/README.md`
- `apps/*/README.md`

## 推荐阅读路径

新贡献者：
1. `README.md`
2. `docs/philosophy.md`
3. `docs/RULES_ZH.md`
4. `docs/plan/README.md`
5. `docs/spec/README.md`

功能实现者：
1. `docs/philosophy.md`
2. `docs/RULES_ZH.md`
3. 对应 `docs/spec/*`
4. 对应 `docs/plan/TODO.md`

架构/设计讨论：
1. `docs/philosophy.md`
2. `docs/起点_也即是目的.md`
3. `docs/research/spectrum-heroui-style-interface-study.md`
4. `docs/spec/heroui-parameter-design-strategy.md`
5. `docs/spec/tree_shaking.md`

## 范围说明

- 本文档系统覆盖仓库自有 Markdown。
- 第三方/外部文档会在索引中登记，但不受本仓库文档规则约束。
- `examples/_upstream/` 下文档属于research mirror调研资产，不纳入文档治理。
