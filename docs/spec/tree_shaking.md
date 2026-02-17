# Tree Shaking / 组件级裁剪规格（v0）

目标：当应用只使用 `<Button>`、`<Input>` 时，`<Select>`、`<Modal>`、`<Chart>` 等未使用组件的 Rust 逻辑与组件 CSS 不应进入最终产物。

> 状态说明：本文定义目标态契约，当前仓库仍处于“全量模块导出 + 聚合 CSS”向“按组件裁剪”迁移阶段。

## 1. 范围与非目标

范围：

- `ui-components` 的组件级 feature 切分
- `ui-components` 的 CSS 条件注入
- package 分发模式下的可裁剪交付
- source 分发模式下的天然裁剪路径

非目标：

- 在本规格内承诺具体 wasm 体积阈值（体积预算另行在计划层定义）
- 讨论业务应用层的路由分包策略（该层与组件库裁剪可叠加，但不是同一问题）

## 2. 术语

- Tree Shaking：构建产物中移除未使用能力的工程实践。
- DCE（Dead Code Elimination）：编译器/链接器移除不可达代码。
- 组件级 feature：以组件或组件族为粒度的 Cargo feature（例如 `component-button`、`component-input`、`component-overlay`）。

## 3. 目标架构

## 3.1 三层机制并存

1. 编译器层（DCE/LTO）：
- release 构建下由 rustc/linker 自动移除不可达路径。

2. 依赖配置层（Cargo features）：
- `ui-components` 提供组件级 feature，用户按需启用。

3. 分发层（Hybrid + source-first）：
- package 模式通过 feature 精确裁剪。
- source 模式通过“只拉取所需组件源码”天然裁剪。

## 3.2 package 模式（默认可裁剪路径）

`ui-components` 必须支持以下能力：

- `default-features = false` 时，不自动包含全部组件。
- 支持显式组合特性：`features = ["component-button", "component-input"]`。
- 提供 `all-components` 便利特性用于 docs/demo/全量场景。

兼容策略（建议）：

- 当前默认特性为 `default = ["inject-css", "all-components"]`。
- 生产应用推荐显式关闭默认特性并按需启用组件特性。

## 3.3 CSS 裁剪契约

仅裁剪 Rust 逻辑不够，CSS 必须同步裁剪。

约束：

- `ui-components/src/css.rs` 的聚合逻辑必须按组件 feature 条件拼接。
- 禁止“无条件引用全部组件 CSS 常量”的实现。
- `inject-css` 只控制“是否注入”，不应等价于“永远注入全量组件 CSS”。

## 3.4 反模式（禁止）

禁止引入“全组件中央注册表”导致所有组件保持可达，例如：

- 全组件函数指针表
- 全组件字符串映射并在常规路径引用
- 无条件收集所有组件样式/元数据的全局单例

理由：这会直接破坏 DCE 与组件级 feature 裁剪。

## 4. 推荐用户使用方式

## 4.1 Package 用户（推荐）

```toml
[dependencies]
ui-components = { path = "../../crates/ui-components", default-features = false, features = ["component-button", "component-input", "inject-css"] }
```

## 4.2 Source 用户（shadcn-like）

用户只引入所需组件源码与其依赖模块（例如 `button`、`input`）时，未引入组件天然不会被编译进产物。

## 5. 验收与门禁（目标）

最低验收：

- 组件级 compile 验证：
  - `--no-default-features --features "component-button,component-input,inject-css"` 能编译通过（wasm 目标）。
- 组件级 CSS 验证：
  - 生成 CSS 包含 `button/input` 选择器，不包含 `select/modal/chart` 选择器。
- 兼容性验证：
  - `all-components` 特性下 docs/demo 不回归。

建议附加验证：

- 维护一个“最小特性集”样例应用（smoke app）用于 CI 持续验证。
- 对关键场景记录 wasm 体积基线（趋势监控，不在本规格硬编码阈值）。

## 6. 与现有文档关系

- 哲学与分发策略：`docs/philosophy.md`
- 样式注入策略：`docs/spec/styling.md`
- 里程碑与 Gate：`docs/plan/IMPLEMENTATION_PLAN.md`
- 执行任务与打勾项：`docs/plan/TODO.md`
