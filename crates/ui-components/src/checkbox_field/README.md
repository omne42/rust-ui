# CheckboxField

`CheckboxField` 是一个基于 `Checkbox` 组合的表单字段组件，负责把标签、描述、状态语义和动效参数集中在一个可测试的契约里。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、状态可观测（`data-*`/`aria-*`）且默认可用的复选字段。
- 非目标：不在组件层实现业务 store，不提供跨字段表单编排能力。
- 风险边界：状态归一、默认值与来源标记必须集中在 `logic.rs`，`view.rs` 只挂载语义。

## Architecture Layers

- `logic.rs`：归一化输入（`id/label/aria`）、派生状态与 source markers。
- `motion.rs`：`CheckboxFieldMotion` 清洗与样式变量拼装。
- `view.rs`：Leptos 结构渲染，组合 `Checkbox` 并挂载 `aria-*` / `data-*`。
- `styles.rs`：token-first 静态 CSS。
- `mod.rs`：对外导出 `CheckboxField`、`CheckboxFieldTone`、`CheckboxFieldIndicatorPlacement`、`CheckboxFieldMotion`。

## API (Table)

### CheckboxField Props

| Prop | Type | Default |
| --- | --- | --- |
| `checked` | `ReadSignal<bool>` | required |
| `set_checked` | `WriteSignal<bool>` | required |
| `disabled` | `bool` | `false` |
| `invalid` | `bool` | `false` |
| `id_base` | `Option<String>` | `None`（空值回退到 `ui-checkbox-field`） |
| `label` | `Option<String>` | `None`（回退到 `Checkbox option`） |
| `description` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None`（优先用自定义，其次 label，再次 `Checkbox field`） |
| `tone` | `CheckboxFieldTone` (`Default` / `Quiet`) | `Default` |
| `indicator_placement` | `CheckboxFieldIndicatorPlacement` (`Start` / `End`) | `Start` |
| `class_name` | `Option<String>` | `None` |
| `motion` | `CheckboxFieldMotion` | `CheckboxFieldMotion::default()` |

### CheckboxField Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 通过 `checked + set_checked` 信号完成受控更新 | `-` |

## Hello World（最小可用）

```rust
let (checked, set_checked) = signal(false);

view! {
    <CheckboxField checked=checked set_checked=set_checked label="Accept terms" />
}
```

## Semantics and Accessibility

- 根节点使用 `role="group"`，并输出 `aria-label`、`aria-describedby`、`aria-disabled`、`aria-invalid`。
- 暴露稳定语义标记：`data-state`、`data-tone`、`data-indicator-placement`、`data-label-source`、`data-aria-source`、`data-class-source`、`data-motion-source`。
- 描述存在时输出 `data-description="present"` 并自动关联描述 `id`。

## Motion and Styling

- `CheckboxFieldMotion` 默认值：
  - `transition_ms = 160`
  - `indicator_scale_pct = 100`
- `sanitize_motion` 会限制极值（`transition_ms` 上限、`indicator_scale_pct` clamp）。
- 运行时只透出必要 CSS 变量，静态规则全部在 `styles.rs`。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：预览当前 CheckboxField 效果与语义标记。
- `Config`：通过交互控制项切换 tone/placement/invalid/disabled/description/class。
- `Code`：实时生成当前配置对应的调用代码，支持复制。
- `CSS Test`：加载 `styles.rs` 原始样式，支持 scoped 调整和回滚。
- `对比`：页面保留多组状态矩阵（受控、quiet+invalid、disabled）用于横向比较。
