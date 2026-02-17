# 计划文档导航

计划文档用于定义“如何执行、按什么顺序执行、如何验收”。

## 当前文件

- `docs/plan/IMPLEMENTATION_PLAN.md`
  - 实施里程碑与任务拆分主计划

- `docs/plan/COMPONENT_LAYER_REFACTOR_SHARDS.md`
  - 组件层重构并发分片草案（Draft）

- `docs/plan/TODO.md`
  - 人类可执行任务列表与 Stop Gates

- `docs/plan/PLAYGROUND_TODO.md`
  - docs-app playground 动态化任务追踪

- `docs/plan/SUBAGENT_TASK_TEMPLATE.md`
  - 子任务卡模板

- `docs/plan/task_dag.json`
  - 机器可读任务 DAG（JSON）

## 使用方式

1. 先看 `IMPLEMENTATION_PLAN.md` 明确阶段目标
2. 按 `TODO.md` 执行并过门禁
3. 涉及 docs/playground 时同步维护 `PLAYGROUND_TODO.md`
4. 任务变化时同步更新 DAG 与文档

## 关联文档

- 规则层：`docs/RULES_ZH.md`
- 规格层：`docs/spec/README.md`
- 调研层：`docs/research/README.md`
