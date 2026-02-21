# FieldLabel

`FieldLabel` 是一个基于 `ui-state-primitives` + `ui-headless` 的表单标签组件。

## 目标 / 非目标 / 风险边界

- 目标：提供一致的标签语义、必填状态表达与稳定来源标记。
- 非目标：不在组件层实现业务校验状态机或全局文案系统。
- 风险边界：文本/指示器/ARIA 来源判定必须保持在 primitives + headless，避免在 `view.rs` 漂移。

## Architecture Layers

- `logic.rs`：桥接 `ui_state_primitives::field_label` 并组合 class。
- `view.rs`：挂载 `use_field_label` 语义 attrs，渲染 label 结构。
- `styles.rs`：静态 CSS。
- `mod.rs`：公开最小 API（`FieldLabel`、`FieldLabelTone`、默认常量）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `text` | `Option<String>` | `"Field"`（空值回退） |
| `for_id` | `Option<String>` | `None` |
| `is_required` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `tone` | `FieldLabelTone` (`Default` / `Muted` / `Strong`) | `Default` |
| `required_indicator` | `Option<String>` | `"*"`（空值回退） |
| `aria_label` | `Option<String>` | `"Field label"`（空值回退） |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

命名迁移：布尔 props 统一为 `is_*` 前缀，使用 `is_required` / `is_disabled`。
受控/非受控：N/A（组件不维护内部可变状态轴，不提供 `value/on_*_change/default_*` 三元组）。
默认值归一：`logic::normalize_props` 统一处理 `text/required_indicator/aria_label/for_id/class_name` 的优先级与来源标记。
状态派生：`logic::derive_view_model` 统一生成 `FieldLabelState`，`view.rs` 仅消费归一化输出。

## Hello World（最小可用）

```rust
<FieldLabel text="Email".to_string() for_id="email".to_string() is_required=true />
<input id="email" type="email" />
```

## Semantics and Accessibility

- 使用原生 `<label>` 结构，支持 `for` 绑定。
- 输出 `aria-label` 与 `aria-disabled`。
- 输出稳定语义字段：`data-tone`、`data-state`、`data-required`、`data-disabled`、`data-has-for`、`data-text-source`、`data-indicator-source`、`data-aria-source`、`data-custom-class`、`data-class-source`。
- 支持 `lang` / `dir`（LTR/RTL）透传。

## Motion and Fallback

- 组件无 `motion.rs`；视觉变化由静态样式和语义标记驱动。

## Docs-App Playground 区块（展示 / Config / Code / CSS Test）

- 展示（Display）：页面包含 default 与 workbench 双区对比。
- Config：通过 `Show settings` 调整 `tone/required/disabled/for/source`。
- Code：通过 `Show code` 查看当前配置对应的组件调用。
- CSS Test：通过 `Show test` 编辑 scoped CSS，并检查 `Actual config`。

## 对比场景

- `Tone + Required`：`Default/Muted/Strong` 与必填状态对比。
- `Custom Indicator + Aria + Class`：来源标记组合对比。
- `Workbench (Display + Config + Code + CSS Test)`：default 与可调配置对比。
