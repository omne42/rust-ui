# Header

`Header` 是语义头部容器组件，状态来自 `ui-state-primitives::header`，组件层只负责装配和语义挂载。

## 展示区（多场景对比）

docs-app 的 `Header` 页面提供三组对比：

- `Semantic Header + Tone`：默认语气 vs `HeaderTone::Strong`
- `Bordered + Custom Aria/Class`：强语气 + `bordered=true` + 自定义 `aria/class`
- `Interactive Playground`：动态切换 `tone/bordered`，实时观察 summary 与 `data-ui-*` 变化

## Config 区

`Interactive Playground` 提供与 Button 同型的 settings 面板（Show settings）：

- `Tone`：切换 `Default/Strong`
- `Border`：切换 `bordered` 布尔轴
- `config summary`：展示当前组合状态（用于对比矩阵校验）

## Code 区

每个 Playground 均支持 `Show code`，复制即用（自动补 `use leptos::prelude::*;` 与 `use ui_components::*;`）。

核心对比例子：

```rust
<Header>
  <h3>"Dialog title"</h3>
</Header>
<Header tone=HeaderTone::Strong bordered=true>
  <h3>"Settings"</h3>
</Header>
```

## CSS Test 区

`Interactive Playground` 支持 `Show test`，可直接编辑 scoped CSS 并查看实际配置：

- CSS 来源：`crates/ui-components/src/header/styles.rs`
- `Actual config`：实时输出 `tone / bordered / class` 组合
- 支持一键恢复原始样式

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/layout.rs::header()`
- 组件源码：`crates/ui-components/src/header/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/header.rs`
- package 模式前提：`component-header`（样式注入可选叠加 `inject-css`）
