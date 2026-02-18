# Divider

`Divider` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion`（组件侧 attach）的分隔线组件。

## 目标 / 非目标 / 风险边界

- 目标：提供横向/纵向分隔语义、稳定状态标记和可选入场动效。
- 非目标：不在组件层实现通用动画引擎与业务状态管理。
- 风险边界：方向与语义映射变化优先回收至 primitives/headless，不在 `view.rs` 分散判断。

## Architecture Layers

- `logic.rs`：桥接 `ui_state_primitives::divider`，并组合 class。
- `view.rs`：挂载 `use_divider` 语义 attrs + motion source 标记。
- `motion.rs`：`DividerMotion + attach_motion`，wasm 动画与 non-wasm no-op。
- `styles.rs`：静态 CSS。
- `mod.rs`：公开最小 API（`Divider`、`DividerOrientation`、`DividerMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `orientation` | `DividerOrientation` (`Horizontal` / `Vertical`) | `Horizontal` |
| `motion` | `DividerMotion` (`animate_in: bool`) | `DividerMotion { animate_in: false }` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

## Hello World（最小可用）

```rust
<Divider />
<Divider orientation=DividerOrientation::Vertical />
```

## Semantics and Accessibility

- 根节点输出 `role="separator"`。
- 纵向时输出 `aria-orientation="vertical"`；横向时不输出该属性。
- 稳定语义字段：`data-orientation`、`data-state`、`data-horizontal`、`data-vertical`、`data-custom-class`、`data-motion-source`、`data-custom-motion`。
- 支持 `lang` / `dir` 透传。

## Motion and Fallback

- wasm + `animate_in=true` 且非 `reduced-motion` 时启用 spring 入场动画。
- non-wasm 路径走 no-op，保证 SSR/tooling 编译稳定。
- `sanitize_motion` 保证运动配置在 attach 前统一归一化。

## Docs-App Playground 区块（展示 / Config / Code / CSS Test）

- 展示（Display）：页面同时提供 default 与 workbench 对比视图。
- Config：通过 `Show settings` 调整 `orientation/class/motion`。
- Code：通过 `Show code` 输出当前 workbench 代码快照。
- CSS Test：通过 `Show test` 做 scoped CSS 编辑，并查看 `Actual config`。

## 对比场景

- `Orientation`：横向/纵向分隔行为对比。
- `Custom Class Marker`：默认样式与自定义 class marker 对比。
- `Workbench (Display + Config + Code + CSS Test)`：default 与可调配置对比。
