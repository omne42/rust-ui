# HoverCard

`HoverCard` 是一个基于 `ui-headless` + `ui-state-primitives` 装配的悬浮信息卡组件，提供稳定的 open/disabled/source 语义标记与可预测交互行为。

## 目标 / 非目标 / 风险边界

- 目标：提供开箱即用的 hover/focus 信息卡，默认路径可直接运行。
- 非目标：不在组件层重写状态原语、A11y 语义或动效执行器。
- 风险边界：`open/default_open/on_open_change` 与延迟参数默认值必须集中在 `logic.rs` 归一化。

## Architecture Layers

- `logic.rs`：参数归一化、状态/source 标记派生。
- `view.rs`：Leptos 结构渲染与 headless 契约挂载。
- `styles.rs`：token-first 静态样式与 `data-*` 状态选择器。
- `motion.rs`：组件语义到 motion contract 的映射。
- `mod.rs`：最小稳定导出面。

## Hello World（最小可用）

```rust
<HoverCard content=move || view! { "Hello World" }>
  <Button variant=ButtonVariant::Secondary>"Hover me"</Button>
</HoverCard>
```

## 先用起来，再进阶

- 默认路径：先用 `content + children`，不需要先理解 `ui-state-primitives`/`ui-headless`。
- 进阶控制：按需启用 `is_open + default_open + on_open_change`。

## 常见用法

### Uncontrolled Example（默认路径）

```rust
<HoverCard
  default_open=true
  content=move || view! { "Uncontrolled content" }
>
  <Button variant=ButtonVariant::Secondary>"Open by default"</Button>
</HoverCard>
```

### Controlled Example（高级入口）

```rust
let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<HoverCard
  is_open=open
  default_open=false
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  content=move || view! { "Controlled content" }
>
  <Button variant=ButtonVariant::Secondary>"Controlled trigger"</Button>
</HoverCard>
```

## Semantics and Accessibility

- 关键语义通过稳定 `data-*` 标记暴露：`data-state`、`data-open`、`data-disabled`、`data-*-source`。
- `panel` 输出 `role="tooltip"`，并保持键盘可达与 escape 关闭路径。
- 支持 `lang` / `dir` 透传，不假设单语言与单方向。
