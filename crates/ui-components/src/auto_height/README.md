# AutoHeight

`AutoHeight` 用于在内容高度变化时做“动画过渡 / 静态跳变”两种路径切换，状态不变量由 `ui-state-primitives::auto_height` 提供。

## 目标 / 非目标 / 风险边界

- 目标：提供稳定的高度过渡容器，输出可测试的 `data-state` / `data-motion-source` 语义标记。
- 非目标：不在组件层重复实现 spring 引擎或 ResizeObserver 协议。
- 风险边界：motion 语义必须通过 `AutoHeightMotion` 输入，`view.rs` 只挂载不重写状态规则。

## Playground 展示区（展示 / config / code / css test）

docs-app 入口：`apps/docs-app/src/pages/components/pages/layout.rs::auto_height()`

- 展示（Display）：
  - `Animated Height`
  - `Static Motion + Custom Class`
  - `Workbench (Display + Config + Code + CSS Test)`
- Config：Workbench test 面板输出 `AutoHeightActualConfig`，用于核对 `open/animate_height/custom_class` 与期望状态标记。
- Code：Workbench 的 `code_signal` 可直接复制当前配置代码。
- CSS Test：Workbench 的 `Scoped CSS` 面板可在线修改并局部验证 `auto_height/styles.rs` 合约。

## 对比场景

- 动画 vs 静态：`animate_height=true` 对比 `animate_height=false`。
- 默认类名 vs 自定义类名：验证 `data-custom-class` 与类名拼接。
- 内容收起 vs 展开：验证高度与状态标记在切换过程中的稳定性。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::AutoHeight;

view! {
    <AutoHeight>
        <div>"AutoHeight content"</div>
    </AutoHeight>
}
```

## Source-first

- 组件源码：`crates/ui-components/src/auto_height/{mod,logic,view,styles,motion}.rs`
- 状态原语：`crates/ui-state-primitives/src/auto_height.rs`
- package 模式特性：`component-auto_height`（可叠加 `inject-css`）
