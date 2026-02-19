# AspectRatio

`AspectRatio` 是一个基于 `ui-state-primitives` + `ui-headless` 组合的媒体比例容器组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可语义检索的固定比例容器。
- 非目标：不在组件层实现业务状态管理与通用动效引擎。
- 风险边界：状态归一化与 A11y 契约漂移时，优先修复 `ui-state-primitives` / `ui-headless`，不在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：桥接 `ui_state_primitives::aspect_ratio`，并负责 class 组合。
- `view.rs`：Leptos 结构渲染，挂载 `use_aspect_ratio` 语义 attrs。
- `styles.rs`：静态 CSS（token-first）。
- `mod.rs`：公开最小 API（`AspectRatio`、`AspectRatioPreset`、`AspectRatioRadius`、`DEFAULT_ARIA_LABEL`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `ratio` | `AspectRatioPreset` (`Square` / `Standard` / `Video` / `Portrait` / `UltraWide`) | `Video` |
| `radius` | `AspectRatioRadius` (`None` / `Sm` / `Md` / `Lg` / `Full`) | `None` |
| `bordered` | `bool` | `false` |
| `fill` | `bool` | `false` |
| `aria_label` | `Option<String>` | `"Aspect ratio frame"`（空值回退） |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

## Hello World（最小可用）

```rust
<AspectRatio>
  <img src="cover.jpg" alt="cover" />
</AspectRatio>
```

## Semantics and Accessibility

- 根节点输出 `role="region"` 与 `aria-label`。
- 输出稳定语义字段：`data-ratio`、`data-radius`、`data-bordered`、`data-fill`、`data-state`、`data-aria-source`、`data-class-source`。
- 支持 `lang` / `dir`（LTR/RTL）透传。

## Motion and Fallback

- 组件本体无 `motion.rs`，不承载运行时动效执行器。
- 与比例相关的状态表达通过语义标记和 CSS 完成。

## Docs-App Playground 区块（展示 / Config / Code / CSS Test）

- 展示（Display）：`apps/docs-app` 页面提供 default 与 workbench 并排对比。
- Config：通过 `Show settings` 调整 `ratio/radius/bordered/fill/aria/class`。
- Code：通过 `Show code` 查看并复制当前 workbench 的调用代码。
- CSS Test：通过 `Show test` 编辑 scoped CSS，并校验 `Actual config` 与状态 marker。

## 对比场景

- `Ratio Presets`：`Square/Video/Portrait` 多比例对比。
- `Bordered + Fill + Custom Aria/Class`：来源标记与 framing 组合对比。
- `Workbench (Display + Config + Code + CSS Test)`：default 与可调配置实时对比。
