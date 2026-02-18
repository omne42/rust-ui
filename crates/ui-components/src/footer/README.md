# Footer

`Footer` 是语义页脚容器组件（渲染为 `<footer>`），状态不变量来自 `ui-state-primitives::footer`，组件层只做装配与语义标记挂载。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可组合、可观测（`data-*`）的页脚容器基元。
- 非目标：不在组件层承载业务状态机、全局主题系统或跨组件交互协议。
- 风险边界：状态归一化与来源判定必须留在 primitive/logic 层，`view.rs` 不追加隐式决策分支。

## Architecture Layers

- `logic.rs`：从 `ui_state_primitives::footer` 复用状态原语（`FooterStateInput`、`FooterState`、`resolve_state`）。
- `view.rs`：Leptos 结构渲染，挂载稳定 `data-*` 状态与来源标记。
- `styles.rs`：静态 token-first CSS（`var(--ui-*)`）。
- `motion.rs`：`FooterMotion` 契约与 CSS 变量拼接工具。
- `mod.rs`：公开最小 API（`Footer`、`FooterTone`、`DEFAULT_ARIA_LABEL`）。

## API (Table)

### Footer Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `tone` | `FooterTone` (`Default` / `Muted`) | `FooterTone::Default` |
| `bordered` | `bool` | `false` |
| `aria_label` | `Option<String>` | `Some("Footer")`（空白值会回退） |
| `class_name` | `Option<String>` | `None` |

### Footer Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 组件当前不暴露回调事件 | `-` |

## Hello World（最小可用）

```rust
use leptos::prelude::*;
use ui_components::Footer;

view! {
    <Footer>
        <p>"Cancel · Save"</p>
    </Footer>
}
```

## 状态示例

```rust
use leptos::prelude::*;
use ui_components::{Footer, FooterTone};

view! {
    <Footer
        tone=FooterTone::Muted
        bordered=true
        aria_label="Settings footer".to_string()
        class_name="docs-footer-custom".to_string()
    >
        <p>"Cancel · Save"</p>
    </Footer>
}
```

## Semantics and Accessibility

- 渲染语义元素：`<footer>`。
- 关键可观测标记：`data-slot="footer"`、`data-tone`、`data-state`、`data-bordered`。
- 来源标记：`data-aria-source`、`data-class-source`、`data-custom-class`。
- `aria_label` 走统一归一化：空白/缺失会回退到 `DEFAULT_ARIA_LABEL`。

## Playground 展示区（展示 / config / code / css test）

docs-app 入口：`apps/docs-app/src/pages/components/pages/layout.rs::footer()`

- 展示（Display）：
  - `Semantic Footer + Tone`
  - `Bordered + Custom Aria/Class`
  - `Workbench (Display + Config + Code + CSS Test)`
- Config：Workbench test 面板输出 `FooterActualConfig`，用于核对 `tone/bordered/custom_aria/custom_class`。
- Code：Workbench 的 `code_signal` 生成当前配置代码，可直接复制。
- CSS Test：Workbench 的 `Scoped CSS` 面板支持局部编辑 `footer/styles.rs` 契约。

## 对比场景

- `Default` vs `Muted` tone 对比。
- `bordered=false` vs `bordered=true` 对比。
- 默认语义来源 vs 自定义 `aria_label/class_name` 来源对比。

## Agent Contract / 流式策略

- `Snapshot`：默认稳定路径。
- `Streaming Optional`：`Footer` 不是 LLM 正文阅读面，按 `fallback=snapshot` 渲染稳定结果。
- 组件提供稳定语义标记供 Agent 与自动化选择器消费，不依赖 DOM 结构猜测。

## Source-first / Copy-Paste Ready

- 组件源码：`crates/ui-components/src/footer/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/footer.rs`
- package 模式前提：`component-footer`（样式注入可选叠加 `inject-css`）
