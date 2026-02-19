# Number

`Number` 提供两种展示组件：
- `StaticNumber`：静态格式化输出
- `SlidingNumber`：数值变化时的滚动动画输出（wasm 下启用动效）

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可观测、可测试的数字格式化与展示组件。
- 非目标：不在组件层实现业务状态管理、异步数据拉取或主题系统。
- 风险边界：格式化规则、语义标记、动效降级策略必须集中在 `logic.rs` / `motion.rs`，不能在 `view.rs` 分散补丁。

## Architecture Layers

- `logic.rs`：归一化与派生（分隔符、小数位、符号、来源标记、类名拼接）。
- `view.rs`：Leptos 结构渲染与语义挂载（`data-*` / `lang` / `dir`）。
- `motion.rs`：`SlidingNumberMotion` 契约、sanitize、wasm attach、reduced-motion 降级。
- `styles.rs`：静态 CSS 规则（token-first + 稳定状态选择器）。
- `mod.rs`：最小公开 API（`StaticNumber` / `SlidingNumber` / `SlidingNumberMotion` / `NumberFormatOptions`）。

## API (Table)

### StaticNumber Props

| Prop | Type | Default |
| --- | --- | --- |
| `number` | `f64` | required |
| `pad_start` | `bool` | `false` |
| `decimal_separator` | `Option<String>` | `None`（内部归一为 `"."`） |
| `decimal_places` | `Option<u32>` | `None`（最多 12） |
| `thousand_separator` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### SlidingNumber Props

| Prop | Type | Default |
| --- | --- | --- |
| `number` | `Signal<f64>` | required |
| `motion` | `SlidingNumberMotion` | `SlidingNumberMotion::default()` |
| `pad_start` | `bool` | `false` |
| `decimal_separator` | `Option<String>` | `None`（内部归一为 `"."`） |
| `decimal_places` | `Option<u32>` | `None`（最多 12） |
| `thousand_separator` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 展示组件不直接暴露事件回调 | `-` |

## Hello World（最小可用）

```rust
<StaticNumber number=12345.67 decimal_places=2 thousand_separator=",".to_string() />
```

```rust
let (value, set_value) = signal(12345.67_f64);
<SlidingNumber
  number=Signal::derive(move || value.get())
  decimal_places=2
  thousand_separator=",".to_string()
/>
```

## Semantics and Accessibility

- `StaticNumber` 根节点：`data-slot="static-number"`。
- `SlidingNumber` 根节点：`data-slot="sliding-number"`，并带 `data-state`/`data-motion-source` 等来源标记。
- 动画展示与读屏值分离：`data-slot="sliding-number-a11y-value"` 提供稳定可读文本。
- 支持 `lang` / `dir`，通过 `ui_headless::a11y::locale_attrs` 统一归一化挂载。

## Motion and Fallback

- `SlidingNumberMotion` 支持自定义动画契约，进入渲染前会经过 `sanitize_motion`。
- wasm 下根据契约附加滚动动效。
- non-wasm 自动走静态降级路径，保证 SSR/tooling 编译稳定。
- 若系统偏好 `reduced-motion`，动效会降级为非动画输出。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/display.rs`：
- `static_number()`：`Formatting Matrix`、`Custom Separators + Class`
- `sliding_number()`：`Animated Matrix`、`Custom Separators + Motion + Class`

## Playground 展示区（Display / Config / Code / CSS Test）

- `StaticNumber` 与 `SlidingNumber` 都提供 `Workbench (Display + Config + Code + CSS Test)`。
- `Display`：主展示区 + 对比矩阵（默认/负数/NaN、动画/静态）并排显示。
- `Config`：可交互调节分隔符、小数位、动效开关、自定义 class 等参数。
- `Code`：根据当前配置实时生成可复制代码片段。
- `CSS Test`：加载 `crates/ui-components/src/text_input/number/styles.rs` 的原始样式，可在 scoped 区域实时覆盖并观察结果。

## 对比场景（多种情况）

- 静态格式化对比：正数 / 负数 / NaN（sanitized）与不同分隔符来源。
- 小数位对比：`auto`、`0`、`2`、`6`。
- 千分位对比：`none`、`,`、`space`。
- 动效对比：`SlidingNumber` 的动画开启/关闭与 custom motion 来源标记差异。
- 样式来源对比：默认 class 与 `docs-static-number-custom` / `docs-sliding-number-custom`。

## Source-first / Copy-Paste Ready

- docs `Playground` 提供源码复制入口（含导入）。
- 真实源码落点：
  - `crates/ui-components/src/text_input/number/mod.rs`
  - `crates/ui-components/src/text_input/number/logic.rs`
  - `crates/ui-components/src/text_input/number/view.rs`
  - `crates/ui-components/src/text_input/number/styles.rs`
  - `crates/ui-components/src/text_input/number/motion.rs`
