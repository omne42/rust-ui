# Label

`Label` 是一个基于 `ui-state-primitives` + `ui-headless` 组合的表单标签组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、状态来源可观测的标签基元。
- 非目标：不在组件层实现业务状态管理、异步协议或通用动画引擎。
- 风险边界：跨层职责漂移（primitives/headless/theme）时，优先回到对应层修复，不在 `view.rs` 追加补丁逻辑。

## Architecture Layers

- `logic.rs`：仅消费 `ui-state-primitives::label` 的归一化与状态派生能力（re-export）。
- `view.rs`：Leptos 结构渲染，挂载 `data-*` 语义标记与 `locale_attrs(lang/dir)`。
- `styles.rs`：仅静态 CSS 契约，样式通过 `var(--ui-*)` 驱动。
- `motion.rs`：`LabelMotion` 轻量动效参数契约（当前未绑定 wasm 驱动）。
- `mod.rs`：公开最小稳定 API（`Label`、`LabelEmphasis`、`LabelMotion`、状态类型与默认常量）。

## API (Table)

### Label Props

| Prop | Type | Default |
| --- | --- | --- |
| `text` | `Option<String>` | `None`（回退到 `DEFAULT_ARIA_LABEL`） |
| `for_id` | `Option<String>` | `None` |
| `is_required` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `emphasis` | `LabelEmphasis` (`Default` / `Subtle` / `Strong`) | `Default` |
| `required_indicator` | `Option<String>` | `None`（回退到 `DEFAULT_REQUIRED_INDICATOR`） |
| `class_name` | `Option<String>` | `None` |
| `motion` | `LabelMotion` | `LabelMotion::default()` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### API 命名迁移

- `required` -> `is_required`
- `disabled` -> `is_disabled`
- 为避免同义别名漂移，旧命名不保留别名；docs 与示例统一使用新命名。
- `Label` 不维护内部可变状态轴，因此无 `value/on_value_change/default_value` 三元 API。

### Label Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 标签为展示组件，不直接暴露回调事件 | `-` |

## Hello World（最小可用）

```rust
<Label text="Name".to_string() />
```

- 默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。
- 进阶需求再按需开启 `emphasis`、`required_indicator`、`lang/dir`、`class_name`。

## Semantics and Accessibility

- 根节点为 `<label>`，通过 `for` 与目标输入关联。
- 组件暴露稳定语义标记：
  - `data-slot="label"`
  - `data-emphasis`
  - `data-state` (`required` / `optional`)
  - `data-required` / `data-disabled` / `data-has-for`
  - `data-label-source` / `data-indicator-source` / `data-class-source`
- 使用 `ui_headless::locale_attrs` 挂载 `lang` / `dir`，支持 LTR/RTL 场景。

## Motion and Fallback

- `LabelMotion` 为组件导出的轻量参数契约（默认值来自 `ui-theme::label_motion_tokens`）：
  - `color_transition_ms`
  - `weight_transition_ms`
- `view.rs` 通过 `motion::attach_motion` 输出 CSS 变量契约并挂载 `data-motion-source`。
- non-wasm 路径通过 `ui_motion::web::prefers_reduced_motion()` 降级为 1ms，可预测且不阻塞 SSR/tooling 编译。

## Docs Playground（展示 / Config / Code / CSS Test）

- docs 页入口：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 中 `label()`。
- Workbench Playground 提供四个面板能力：
  - Hello World：默认调用路径，单行最小示例，无需手动接线 primitives/headless。
  - 展示（Preview）：实时渲染当前配置的 Label，并同时展示对照态（强强调 + 必填 + 自定义指示）。
  - Config：`test_config_signal` 输出 emphasis/state/source 标记与 class 组合。
  - Code：`code_signal` 输出可复制的当前配置示例。
  - CSS Test：`test_css_source` 载入 `label/styles.rs`，支持作用域内样式试验。
- 额外对比展示：
  - `Emphasis + Required`
  - `Custom Indicator + Class`

## Visual Desire Baseline（Label 范围）

- 视觉层级：通过 `LabelEmphasis::{Default, Subtle, Strong}` 的字重与前景色层级区分信息优先级，避免“只有可用、没有层次”的粗糙观感。
- 对比与节奏：基于 `var(--ui-*)` / `var(--ui-fallback-*)` 变量保持主题对比一致性，`required/disabled/custom` 通过稳定状态标记驱动视觉反馈。
- 交互反馈：当 `for_id` 可用且未禁用时，Label 提供 hover/active 色彩反馈（accent/accent-soft）以提升可感知性；该组件为原生 `<label>` 语义，不额外引入伪交互状态机。
- HeroUI 对标原则：对齐的是视觉语言与体验质量，不复制 API 表层。
- 边界说明：`Button/Input/Overlay` 的默认主题截图基线与视觉回归矩阵属于仓库级任务，不在 `Label` 单组件清单内完成，应在 docs-app 全局基线页统一维护。

## Source-first

- 组件源码入口：
  - `components/label/src/mod.rs`
  - `components/label/src/logic.rs`
  - `components/label/src/view.rs`
  - `components/label/src/styles.rs`
  - `components/label/src/motion.rs`
- 状态原语定义：
  - `crates/ui-state-primitives/src/label.rs`
