# 状态原语 Core/Satellite 拆分规范

> Status: Draft  
> Scope: 约束 `ui-state-primitives` 的核心纯度，并为领域型重依赖原语提供卫星扩展路径。

## 0. 核心判断

`ui-state-primitives` 不应该被拆成很多包，但必须支持“核心 + 卫星”二级结构。  
核心保持高内聚；领域重依赖按卫星分离，避免污染所有组件的编译与包体成本。

## 1. 拆分模型

- Core：`ui-state-primitives`（交互通用原语）
- Satellite：`ui-logic-*`（按领域扩展，如 `ui-logic-calendar`）

当前首批卫星：

- `crates/ui-logic-calendar`
  - `calendar`
  - `date_picker`
  - `date_range_picker`
  - `time_field`

## 2. 石蕊测试（Litmus Test）

对每个原语模块执行：

“该模块实现核心功能时，是否必须引入新的非序列化用途外部 crate（如 `chrono`/`icu4x`/`url`）？”

- 否：留在 Core
- 是：迁到对应 Satellite

## 3. 依赖纪律

- Core 不应被领域重依赖反向污染。
- Satellite 可依赖 Core；Core 不应强依赖具体 Satellite 业务实现。
- 调用方按需启用 Satellite，不得默认全量拉起所有卫星。

## 4. 迁移策略（无破坏优先）

阶段化：

1. 先建立 Satellite crate 与模块归属。
2. 保留调用侧稳定 API（必要时通过门面转发）。
3. 再推进依赖特性化（按组件启用卫星能力）。
4. 最后收敛兼容层，完成纯度治理。

## 5. 关联文档

- `docs/spec/wasm_generic_bloat.md`
- `docs/spec/component_boundaries.md`
- `docs/RULES_ZH.md`
