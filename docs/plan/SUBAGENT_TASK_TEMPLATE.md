# Subagent 任务卡模板

> 用于把 `docs/plan/task_dag.json` 的任务下发给 `$codepm-coder` / `$codepm-reviewer` / `$codepm-builder`。

```
任务: {id} - {title}
目标: {本任务要交付的最小产物}
前置: {depends_on（如无写“无”）}
文件: {允许修改/新增的文件路径列表}
验收:
  - {可验证的 checklist 1}
  - {可验证的 checklist 2}
验证命令:
  - {cargo check/test/fmt/clippy 命令}
约束:
  - 不要修改 {禁止触碰的文件/目录}
  - 不要引入 {禁止引入的依赖/特性}
```

