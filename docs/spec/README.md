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

- `docs/spec/tree_shaking.md`
  - Tree Shaking / 组件级裁剪策略与验收契约

- `docs/spec/i18n.md`
  - i18n / l10n 注入契约（组件层）

- `docs/spec/component_boundaries.md`
  - UI 组件边界：什么是组件，什么不是

- `docs/spec/heroui-parameter-design-strategy.md`
  - 参数模型演进与组件 API 设计策略

- `docs/spec/hyper-structure-ui-development-playbook.md`
  - AI Verified / Struct-First 的执行手册

## 使用方式

1. 从 `docs/plan/TODO.md` 选择任务范围
2. 阅读对应规格文档
3. 按规格约束实现
4. 通过门禁与测试再合入

## 关联文档

- 硬规则：`docs/RULES_ZH.md`
- 哲学总纲：`docs/philosophy.md`
- 计划层：`docs/plan/README.md`
