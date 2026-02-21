# ColorHandle

`ColorHandle` 是颜色选择交互里的拖拽句柄原语，负责位置、焦点、拖拽态与 loupe 可见性语义。

## 先用起来（Hello World）

默认路径只需要 `id_base + color`，其余参数走默认值即可：

```rust
use leptos::prelude::*;
use ui_components::ColorHandle;

view! {
  <div style="position: relative; inline-size: 12rem; block-size: 7rem;">
    <ColorHandle id_base="demo-color-handle".to_string() color="#f59e0b".to_string() />
  </div>
}
```

默认值（无需先理解分层架构）：
- `is_loupe_visible=true`
- `x_percent=50.0`
- `y_percent=50.0`
- `motion=ColorHandleMotion::default()`

## 常见用法（先基础，后进阶）

基础状态切换（最常用）：
- `is_focused=true`：焦点态。
- `is_dragging=true`：拖拽态。
- `is_disabled=true`：禁用态。

进阶参数（按需开启）：
- `x_percent/y_percent`：句柄位置。
- `is_loupe_visible`：控制 loupe 展示。
- `class_name`：自定义类名。
- `motion`：动效时长契约（`ColorHandleMotion`）。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可配置动效的颜色句柄视图原语。
- 非目标：不在组件层实现完整取色状态机或业务色值存储。
- 风险边界：状态归一化在 `logic.rs`，样式契约在 `styles.rs`，不要在 `view.rs` 临时分叉逻辑。

## Architecture Layers

- `logic.rs`：颜色/文案归一、状态派生、来源标记。
- `view.rs`：结构渲染与 `ColorThumb` 装配，输出稳定 `data-*`。
- `motion.rs`：`ColorHandleMotion` 参数清洗与 CSS 变量注入。
- `styles.rs`：静态样式契约（`--ui-color-handle-motion-duration` 等）。
- `mod.rs`：公开 `ColorHandle` / `ColorHandleMotion` 与状态输入输出类型。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `color` | `Option<String>` | `None` |
| `is_disabled` | `bool` | `false` |
| `is_focused` | `bool` | `false` |
| `is_dragging` | `bool` | `false` |
| `is_loupe_visible` | `bool` | `true` |
| `x_percent` | `f32` | `50.0` |
| `y_percent` | `f32` | `50.0` |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `motion` | `ColorHandleMotion` | `ColorHandleMotion::default()` |

## Docs Playground（展示 / Config / Code / CSS Test）

- 展示：docs-app 提供 baseline 对比视图（默认样式 vs 配置样式）。
- Config：提供颜色、坐标、is_disabled/is_focused/is_dragging/is_loupe_visible、motion 时长等控制项。
- Code：支持 copy-ready 代码片段，反映当前配置状态。
- CSS Test：支持 scoped CSS 编辑与恢复，并显示 `ActualConfig` 快照。

## 对比场景

- `Focused + Dragging + Position`
- `Disabled + Custom Class + Loupe Off`
- `Workbench (Display + Config + Code + CSS Test)`
