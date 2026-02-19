# 集合组件注册协议（Registration Protocol）

## 问题定义

集合类组件（Accordion/Tabs/Menu/Select）在细粒度框架中常面临“父组件看不见完整 children”的现实。  
子项可动态挂载/卸载（条件渲染、异步加载、分支切换），逻辑层不能假设初始化时就知道全部 item。

仅有 `expanded_keys: HashSet<Id>` 不足以支持：

- 动态增删项一致性
- 键盘导航顺序（上/下、Home/End）
- 焦点移动与 ARIA 关系正确性

## 核心判断

- 集合类组件本质上是“服务发现”问题，不是静态列表问题。
- 逻辑层必须通过显式注册协议动态维护 item 拓扑与顺序。

## 标准解法

### 1) Context 握手

父组件在 `view.rs` 提供 `RegistrationContext`，用于子项上报生命周期事件。

### 2) 子项注册/反注册

子项在挂载/卸载时上报：

- `Register { id, node_ref, ... }`
- `Unregister { id }`

逻辑层（或其 reducer）据此更新内部集合与可导航索引。

### 3) 顺序来源显式化

键盘导航依赖顺序，必须显式维护 `items_order`（或等价结构）。  
不得把 `HashSet` 迭代顺序当作导航顺序。

顺序可来源于：

- 注册时提供稳定序号/锚点
- 或由 view 层基于 DOM 拓扑回传“前后关系”快照

### 4) 变更后收敛

当动态项出现/消失时，逻辑层要重新收敛：

- `focused_id` 必须落在有效项上
- `expanded_keys` 中无效 id 要清理
- roving index 必须与当前可见顺序一致

## 分层约束

- `logic.rs` / `ui-state-primitives`：
  - 允许：注册表状态、顺序模型、导航规则、收敛策略
  - 禁止：直接读取 DOM 节点拓扑
- `view.rs` / adapter：
  - 允许：感知挂载/卸载与 DOM 顺序变化并上报
  - 禁止：绕过逻辑层直接改集合业务状态

## 竞态与一致性规则

- 重复 `Register(id)` 必须幂等处理（更新或忽略，语义固定）。
- `Unregister(id)` 必须可重入（不存在时不 panic）。
- 顺序重排必须触发导航索引重建，避免焦点落空。

## 测试要求

- 单测：
  - `Register/Unregister` 状态收敛
  - 动态插入/删除后的 `items_order` 正确
  - 键盘导航在重排后仍正确
- 集成测试：
  - 条件渲染项动态出现/消失时，焦点与展开状态稳定
  - `aria-controls/aria-expanded/roving tabindex` 不漂移
