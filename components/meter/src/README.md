# Meter

`Meter` 是一个基于 `ui-state-primitives` + `ui-motion` 组合出来的度量展示组件。

## 快速开始（先用起来）

先用默认 API 跑通，再按需打开高级参数。你不需要先理解底层分层结构。
默认路径不需要用户手动接线 `ui-state-primitives`。

### Hello World（最小可用）

```rust
<Meter
  id="docs-meter".to_string()
  label="Completion".to_string()
  value=Signal::derive(|| Some(42.0))
/>
```

### 常见用法

```rust
// 尺寸 + 视觉语义
<Meter
  id="docs-meter-danger".to_string()
  label="Risk".to_string()
  value=Signal::derive(|| Some(72.0))
  variant=MeterVariant::Danger
  size=MeterSize::Lg
/>

// 不确定态（等待中）
<Meter
  id="docs-meter-pending".to_string()
  label="Pending".to_string()
  value=Signal::derive(|| None)
/>
```

## 进阶与架构说明

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
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `value` | `Signal<Option<f64>>` | `None`（不确定态） |
| `min` | `Option<f64>` | `None`（逻辑层归一为 `0.0`） |
| `max` | `Option<f64>` | `None`（逻辑层归一为 `100.0`） |
| `size` | `MeterSize` (`Sm` / `Default` / `Lg`) | `Default` |
| `variant` | `MeterVariant` (`Default` / `Danger`) | `Default` |
| `motion` | `MeterMotion` | `MeterMotion::default()` |
| `is_value_label_visible` | `Option<bool>` | `None`（优先于 `show_value_label`） |
| `show_value_label` | `Option<bool>` | `None`（兼容别名，逻辑层默认 `true`） |
| `value_label` | `Option<String>` | `None`（自动使用百分比） |
| `class_name` | `Option<String>` | `None` |

命名兼容策略：推荐使用 `is_value_label_visible`；历史参数 `show_value_label` 保留为兼容别名，二者同时传入时以 `is_value_label_visible` 为准。

### Meter Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | `Meter` 为展示组件，不暴露交互事件回调 | `-` |

## Semantics and Accessibility

- 根节点使用 `role="meter"`。
- 输出 `aria-valuemin` / `aria-valuemax` / `aria-valuenow` / `aria-valuetext`。
- 支持 `lang` / `dir`（LTR/RTL）接入；`aria_label` 文案链路为 `props > UiRoot i18n bundle > fallback`。
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
- `preserve_state`：可选保留当前配置上下文；关闭后回到默认基线，便于快速重演。
- Code 区：输出与当前配置同步的可复制代码片段。
- CSS Test 区：加载 `components/meter/src/styles.rs`，支持局部样式试验与恢复。
