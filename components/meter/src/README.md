# Meter

`Meter` 是一个基于 `ui-state-primitives` + `ui-motion` 组合出来的度量展示组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可受控数值输入的进度度量展示。
- 非目标：不在组件层实现业务状态管理、异步协议或全局主题系统。
- 风险边界：状态归一化和来源标记必须保持在 primitives/logic 层，避免在 `view.rs` 追加补丁分支。

## Architecture Layers

- `crates/ui-state-primitives/src/meter.rs`：`MeterRange`、`MeterPhase`、`MeterStateInput`、`MeterState` 及归一化/派生函数。
- `logic.rs`：对 `ui-state-primitives::meter` 的薄装配导出（组件层不重复实现状态机）。
- `view.rs`：Leptos 结构渲染、`role/aria-*` 挂载、`data-*` 语义标记输出。
- `motion.rs`：`MeterMotion` 契约与 wasm spring 驱动，含 non-wasm no-op。
- `styles.rs`：仅静态 CSS 契约，样式通过 `var(--ui-*)` 驱动。
- `mod.rs`：公开最小稳定 API（`Meter`、`MeterVariant`、`MeterSize`、`MeterMotion`）。

## API (Table)

### Meter Props

| Prop | Type | Default |
| --- | --- | --- |
| `id` | `String` | required |
| `label` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None`（回退到 `label` 或 `"Meter"`） |
| `value` | `Signal<Option<f64>>` | `None`（不确定态） |
| `min` | `f64` | `0.0` |
| `max` | `f64` | `100.0` |
| `size` | `MeterSize` (`Sm` / `Default` / `Lg`) | `Default` |
| `variant` | `MeterVariant` (`Default` / `Danger`) | `Default` |
| `motion` | `MeterMotion` | `MeterMotion::default()` |
| `show_value_label` | `bool` | `true` |
| `value_label` | `Option<String>` | `None`（自动使用百分比） |
| `class_name` | `Option<String>` | `None` |

### Meter Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | `Meter` 为展示组件，不暴露交互事件回调 | `-` |

## Hello World（最小可用）

```rust
<Meter
  id="docs-meter".to_string()
  label="Completion".to_string()
  value=Signal::derive(|| Some(42.0))
/>
```

- 默认路径不需要用户手动接线 `ui-state-primitives`。
- 自定义需求（危险态、尺寸、自定义 spring）再按需开启扩展参数。

## Semantics and Accessibility

- 根节点使用 `role="meter"`。
- 输出 `aria-valuemin` / `aria-valuemax` / `aria-valuenow` / `aria-valuetext`。
- 输出稳定语义标记：`data-variant`、`data-size`、`data-state`、`data-label-source`、`data-value-label-source`、`data-motion-source`、`data-class-source`。
- 不确定态（`value=None`）时进入 `indeterminate`，并移除 `aria-valuenow`。

## Motion and Fallback

- wasm 下使用 `ui_motion::spring::SpringAnimator` 驱动 `--ui-meter-progress`。
- 非 wasm 环境走 no-op，保证 SSR/tooling 编译路径稳定。
- CSS 覆盖 `prefers-reduced-motion: reduce`，关闭不确定态动画。

## Agent Contract / 流式降级

- 通过稳定 `data-*` 字段暴露状态和来源，便于测试与 Agent 消费。
- `Meter` 非正文流式渲染组件，按 `Streaming Optional` 处理，默认 `snapshot` 渲染语义。

## docs-app Workbench（展示 / Config / Code / CSS Test）

- 展示区：同时展示“当前配置”“Danger + Lg 对比”“Indeterminate 对比”。
- Config 区：切换 `variant/size/indeterminate/custom-label/custom-motion/custom-class`，并调节值。
- Code 区：输出与当前配置同步的可复制代码片段。
- CSS Test 区：加载 `components/meter/src/styles.rs`，支持局部样式试验与恢复。
