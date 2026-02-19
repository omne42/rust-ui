# 事件光锥限制（Event Light Cone）与信号总线

## 问题定义

在大规模集合组件（Table/Grid/Tree）中，若父子通信依赖层层 props 传递：

`Table -> Row -> Cell -> Leaf`

一次全局动作（如“全选”）会触发整棵子树大面积更新，带来：

- props 级联更新成本（O(N)）
- 依赖图膨胀与初始化压力（大量 signal 绑定）
- 渲染与交互抖动

## 核心判断

- 大规模 UI 事件传播必须避免线性链路传播。
- 父子层层传 props 是“光锥内传播”，在高维集合上不可扩展。

## 标准解法

### 1) Context Bus（通信虫洞）

容器组件提供总线上下文（如 `TableBus`），子节点按需订阅，不走层层 props。

### 2) Selector Subscription（选择器订阅）

子节点不直接接收完整状态，只订阅最小切片：

```rust
let is_selected = bus.select(move |state| state.is_row_selected(row_id));
```

要求：选择器输出可比较（`PartialEq`）且稳定，未变化不触发下游更新。

### 3) 全局操作的状态压缩

“全选”不应展开成 N 个布尔写入。  
逻辑层优先采用压缩状态表达（如 `SelectionState::All` / `Some(HashSet<Id>)`），让选择器在读取侧常数时间判定。

## 分层约束

- `logic.rs` / primitives：
  - 允许：总线状态、选择器语义、压缩状态模型
  - 禁止：为传递便利把全量状态拆成层层 props
- `view.rs`：
  - 允许：订阅 bus selector 并渲染局部切片
  - 禁止：通过父链 props 复制整块状态

## 复杂度目标

- 常见批量操作（全选/全取消/批量过滤）应满足 O(1) 或 O(log N) 更新语义。
- 子项渲染更新应尽量局部化，避免 O(N) 级广播。

## 测试要求

- 单测：全局操作下压缩状态与选择器语义正确。
- 性能回归：大集合下批量操作无线性级卡顿回归。
- 语义测试：选择状态与可访问语义标记（`aria-selected`/`data-selected`）一致。
