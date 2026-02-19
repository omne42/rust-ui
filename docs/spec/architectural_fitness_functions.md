# 架构适应性免疫系统（Architectural Fitness Functions）

## 问题定义

仅靠 `ARCHITECTURE` 文档宣言无法长期约束多团队协作。  
在真实组织结构中（康威定律），团队会优先选择本地最优实现，最终产生“影子架构”。

结果：

- 规则仍在文档里，但代码已悄然偏离
- 架构腐烂不是一次事故，而是持续漂移

## 核心判断

- 架构必须是“可执行契约”，而非“静态文本约定”。
- 约束人不可靠，约束代码更可靠。

## 标准解法

### 1) 健身函数（Fitness Functions）

为每条关键架构原则建立自动化检查（测试/脚本/lint）：

- `test_no_circular_dependencies`
- `test_core_is_framework_agnostic`
- `test_all_components_have_required_files`
- `test_feature_gates_respect_layer_boundaries`

这些测试关注“结构正确性”，不是业务功能正确性。

### 2) 持续验证（Continuous Verification）

健身函数必须进入 CI 主路径：

- PR 阶段强制执行
- 失败即阻断合并
- 与 fmt/clippy/test 同级，不可手动绕过

### 3) 违规可诊断

每条健身函数失败信息应明确：

- 哪条架构规则被破坏
- 哪个文件/依赖/模块触发违规
- 最小修复路径

## 分层约束

- `ui-state-primitives`：
  - 必须保持 framework-agnostic（禁止 leptos/web-sys）
- crate 依赖图：
  - 必须无循环，且遵守单向分层
- 组件目录：
  - 必须满足最小结构契约（`mod/logic/view/styles/motion` 等按适用范围）

## 验收要求

- 至少落地一组架构健身函数并接入 CI
- 新增架构规则时，必须同步新增对应健身函数或给出 N/A 说明
- 架构评审结论必须引用健身函数结果，不接受“口头符合”
