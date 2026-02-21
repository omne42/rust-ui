# Code

`Code` 是一个轻量的内联/块级代码展示组件，语义状态由 `logic.rs` 统一派生，`view.rs` 只负责渲染。

## 快速开始（先用起来）

先直接走默认 API，不需要先理解 `state/headless/theme/motion` 分层细节。最小路径就是：

```rust
use ui_components::Code;

view! {
    <Code>"cargo test -p ui-components"</Code>
}
```

## 目标 / 非目标 / 风险边界

- 目标：提供稳定、可测试、token-first 的代码文本展示能力。
- 非目标：不做语法高亮引擎，不做异步流式渲染协议。
- 风险边界：分支逻辑只留在 `logic.rs`，不要把状态判断回灌到 `view.rs`。

## LLM 输出显示模式约定（两种）

- `Streaming`：LLM 还在生成时，上层按增量内容更新 `children`，组件边收边渲染。
- `Snapshot`：LLM 生成完成后，上层一次性提供完整内容，组件一次性渲染。
- `Snapshot` 是 `Code` 默认基础能力：组件可直接消费完整内容与完整配置并稳定渲染。
- `Code` 不是正文阅读面：`Streaming Optional`，并固定 `fallback=snapshot`。
- 输出状态由组件显式标记：`data-ui-output-state="verified"`（静态展示默认态）。
- `Code` 不实现传输协议（SSE/WebSocket），只消费上层提供的文本渲染输入。
- 数据校验、断线恢复、重试策略由上层负责，组件层仅负责稳定渲染。

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
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<ui_headless::a11y::A11yDirection>` | `None` |
| `children` | `Children` | required |

## Hello World

```rust
use ui_components::Code;

view! {
    <Code>"cargo test -p ui-components"</Code>
}
```

## 常见用法

### Inline（默认）

```rust
view! { <Code>"pnpm lint"</Code> }
```

### Block（常见）

```rust
use ui_components::{Code, CodeVariant};

view! {
    <Code variant=CodeVariant::Block>
        {"cargo fmt --all\ncargo clippy --workspace --all-targets -- -D warnings"}
    </Code>
}
```

## 进阶用法（可选）

默认 API 在前；进阶调参、状态矩阵与复制链路在后文 playground 中查看。

## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）

对应页面：`apps/docs-app/src/pages/components/pages/display.rs` 的 `code()`

- 展示区：`Primary` 实时预览 + `对比矩阵`（Inline vs Block）。
- Config 区：切换 `variant`、`custom class`、`long content`、`show compare matrix`。
- Code 区：根据当前配置实时生成可复制代码片段。
- CSS Test 区：展示 `components/code/src/styles.rs` 的 `CSS` 常量，并绑定当前配置快照。
- 受控/非受控对照：`Controlled vs Uncontrolled (N/A)`，明确 `Code` 无内部受控状态轴。
- 流式/快照：`Streaming Optional / Snapshot`，展示 `fallback=snapshot` 语义。
- Source-first：`Source-first Starter (Copy-Paste Ready)`，复制链路自动补全 imports。

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
- `lang` / `dir`（由 `ui_headless::a11y::locale_attrs` 归一）

## Docs and Feature

- docs-app entry: `apps/docs-app/src/pages/components/pages/display.rs::code()`
- source: `components/code/src/{mod,logic,view,styles}.rs`
- feature: `component-code`（可选 `inject-css`）
