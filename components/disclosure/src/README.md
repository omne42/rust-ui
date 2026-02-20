# Disclosure

`Disclosure` 是单项可展开面板组件，支持受控/非受控 open 轴，并输出稳定的来源标记（`data-open-control-mode` / `data-default-open-source`）。

## 目标 / 非目标 / 风险边界

- 目标：提供 A11y 完整、状态可观测、可回归测试的展开交互基元。
- 非目标：不在组件层复写 headless 输入语义和通用 motion 引擎。
- 风险边界：受控/非受控语义必须由统一状态模型输出，禁止在样式层反推状态。

## Playground 展示区（展示 / config / code / css test）

docs-app 入口：`apps/docs-app/src/pages/components/pages/collections.rs::disclosure()`

- 展示（Display）：
  - `Controlled`
  - `Disabled`
  - `Workbench (Display + Config + Code + CSS Test)`
- Config：Workbench test 面板输出 `DisclosureActualConfig`，用于核对 `control_mode/default_open_source/motion_source`。
- Code：Workbench 的 `code_signal` 生成当前受控/非受控与 motion 配置代码。
- CSS Test：Workbench 的 `Scoped CSS` 面板可局部调试 `disclosure/styles.rs` 选择器与状态标记契约。

## 对比场景

- 受控 vs 非受控：`open + on_open_change` 对比 `default_open`。
- 默认 motion vs 自定义 motion：验证 `data-motion-source` 与样式契约。
- 可交互 vs 禁用：验证 `disabled` 语义与触发器行为。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::Disclosure;

view! {
    <Disclosure id_base="disc".to_string() label="Details".to_string()>
        <div>"Hidden content"</div>
    </Disclosure>
}
```

## Source-first

- 组件源码：`components/disclosure/src/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/disclosure.rs`
- package 模式特性：`component-disclosure`（可叠加 `inject-css`）
