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
| `required` | `bool` | `false` |
| `disabled` | `bool` | `false` |
| `emphasis` | `LabelEmphasis` (`Default` / `Subtle` / `Strong`) | `Default` |
| `required_indicator` | `Option<String>` | `None`（回退到 `DEFAULT_REQUIRED_INDICATOR`） |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### Label Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 标签为展示组件，不直接暴露回调事件 | `-` |

## Hello World（最小可用）

```rust
<Label text="Name".to_string() for_id="name".to_string() required=true />
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

- `LabelMotion` 为组件导出的轻量参数契约：
  - `color_transition_ms`
  - `weight_transition_ms`
- 当前 `Label` 不执行独立 wasm 动效驱动；该契约用于与组件库其它成员保持一致的动效配置接口。

## Docs Playground（展示 / Config / Code / CSS Test）

- docs 页入口：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 中 `label()`。
- Workbench Playground 提供四个面板能力：
  - 展示（Preview）：实时渲染当前配置的 Label，并同时展示对照态（强强调 + 必填 + 自定义指示）。
  - Config：`test_config_signal` 输出 emphasis/state/source 标记与 class 组合。
  - Code：`code_signal` 输出可复制的当前配置示例。
  - CSS Test：`test_css_source` 载入 `label/styles.rs`，支持作用域内样式试验。
- 额外对比展示：
  - `Emphasis + Required`
  - `Custom Indicator + Class`

## Source-first

- 组件源码入口：
  - `components/label/src/mod.rs`
  - `components/label/src/logic.rs`
  - `components/label/src/view.rs`
  - `components/label/src/styles.rs`
  - `components/label/src/motion.rs`
- 状态原语定义：
  - `crates/ui-state-primitives/src/label.rs`
