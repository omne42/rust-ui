# Code

`Code` 是一个轻量的内联/块级代码展示组件，语义状态由 `logic.rs` 统一派生，`view.rs` 只负责渲染。

## 目标 / 非目标 / 风险边界

- 目标：提供稳定、可测试、token-first 的代码文本展示能力。
- 非目标：不做语法高亮引擎，不做异步流式渲染协议。
- 风险边界：分支逻辑只留在 `logic.rs`，不要把状态判断回灌到 `view.rs`。

## Architecture Layers

- `logic.rs`：`CodeVariant` 归一化与 class/state 派生。
- `view.rs`：渲染 `<code>` 与稳定语义标记。
- `styles.rs`：静态 CSS 契约。
- `mod.rs`：最小导出面（`Code`、`CodeVariant`）。

## API

| Prop | Type | Default |
| --- | --- | --- |
| `variant` | `CodeVariant` (`Inline` / `Block`) | `Inline` |
| `class_name` | `Option<String>` | `None` |
| `children` | `Children` | required |

## Hello World

```rust
use ui_components::Code;

view! {
    <Code>"cargo test -p ui-components"</Code>
}
```

## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）

对应页面：`apps/docs-app/src/pages/components/pages/display.rs` 的 `code()`

- 展示区：`Primary` 实时预览 + `对比矩阵`（Inline vs Block）。
- Config 区：切换 `variant`、`custom class`、`long content`、`show compare matrix`。
- Code 区：根据当前配置实时生成可复制代码片段。
- CSS Test 区：展示 `components/code/src/styles.rs` 的 `CSS` 常量，并绑定当前配置快照。

## 多场景对比（对比矩阵）

- 场景 A：`Inline + default class`（短命令文本）。
- 场景 B：`Block + default class`（多行命令文本）。
- 场景 C：`Inline/Block + custom class`（验证 `data-custom-class` 分支）。

## 语义与样式契约

根节点稳定标记：

- `data-slot="code"`
- `data-variant`（`inline` / `block`）
- `data-state`（`inline` / `block`）
- `data-inline`
- `data-block`
- `data-custom-class`

## Docs and Feature

- docs-app entry: `apps/docs-app/src/pages/components/pages/display.rs::code()`
- source: `components/code/src/{mod,logic,view,styles}.rs`
- feature: `component-code`（可选 `inject-css`）
