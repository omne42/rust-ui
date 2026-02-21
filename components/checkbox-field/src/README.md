# CheckboxField

`CheckboxField` 是一个基于 `Checkbox` 组合的表单字段组件，负责把标签、描述、状态语义和动效参数集中在一个可测试的契约里。

## docs-app 入口

- 页面函数：`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field()`
- 路由：`#/components/checkbox-field`

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、状态可观测（`data-*`/`aria-*`）且默认可用的复选字段。
- 非目标：不在组件层实现业务 store，不提供跨字段表单编排能力。
- 风险边界：状态归一、默认值与来源标记必须集中在 `logic.rs`，`view.rs` 只挂载语义。

## Architecture Layers

- `logic.rs`：归一化输入（`id/label/description/aria/default_checked`）、派生状态与 source markers。
- `motion.rs`：`CheckboxFieldMotion` 清洗与样式变量拼装。
- `view.rs`：Leptos 结构渲染，组合 `Checkbox` 并挂载 `aria-*` / `data-*`。
- `styles.rs`：token-first 静态 CSS。
- `mod.rs`：对外导出 `CheckboxField`、`CheckboxFieldTone`、`CheckboxFieldIndicatorPlacement`、`CheckboxFieldMotion`。

## API (Table)

### CheckboxField Props

| Prop | Type | Default |
| --- | --- | --- |
| `is_checked` | `Option<ReadSignal<bool>>` | `None`（优先于 `checked`） |
| `on_checked_change` | `Option<WriteSignal<bool>>` | `None`（优先于 `set_checked`） |
| `default_checked` | `Option<bool>` | `None`（仅在未传受控信号时生效） |
| `is_disabled` | `Option<bool>` | `None`（优先于 `disabled`） |
| `is_invalid` | `Option<bool>` | `None`（优先于 `invalid`） |
| `checked`（兼容别名） | `Option<ReadSignal<bool>>` | `None` |
| `set_checked`（兼容别名） | `Option<WriteSignal<bool>>` | `None` |
| `disabled`（兼容别名） | `bool` | `false` |
| `invalid`（兼容别名） | `bool` | `false` |
| `id_base` | `Option<String>` | `None`（空值回退到 `ui-checkbox-field`） |
| `label` | `Option<String>` | `None`（回退到 `Checkbox option`） |
| `description` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None`（优先用自定义，其次 label，再次 `Checkbox field`） |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<ui_headless::A11yDirection>` | `None` |
| `tone` | `CheckboxFieldTone` (`Default` / `Quiet`) | `Default` |
| `indicator_placement` | `CheckboxFieldIndicatorPlacement` (`Start` / `End`) | `Start` |
| `class_name` | `Option<String>` | `None` |
| `motion` | `CheckboxFieldMotion` | `CheckboxFieldMotion::default()` |

### CheckboxField Events

| Event | Type | Default |
| --- | --- | --- |
| `on_checked_change` | `Option<WriteSignal<bool>>` | `None` |

## Hello World（最小可用）

```rust
<CheckboxField label="Accept terms".to_string() />
```

## 常见用法

- 受控用法：
  - `is_checked + on_checked_change`，外部信号作为单一事实来源。
- 非受控用法：
  - `default_checked` 只用于初始化，后续状态由组件内部原语维护。
- 常见状态矩阵：
  - `tone=Quiet`、`indicator_placement=End`、`is_invalid=Some(true)`、`is_disabled=Some(true)`。

## 先用起来，再进阶

- 默认路径：`<CheckboxField label=... />`，只传 `label` 也能直接工作。
- 进阶控制：按需启用 `is_checked + default_checked + on_checked_change`。
- 避免一开始就接入复杂分层：先跑通默认 API，再逐步开启受控、语义和动效参数。

## 命名兼容与迁移

- 主命名已切到 `is_checked/on_checked_change/default_checked` 与 `is_disabled/is_invalid`。
- 兼容别名 `checked/set_checked/disabled/invalid` 仍可用，归一化优先级统一在 `logic.rs`。
- 受控模式未提供 `on_checked_change` 时组件保持只读（外部值仍是单一事实来源）。
- 迁移建议：先替换 docs 和业务调用到主命名，再移除别名输入。

## Semantics and Accessibility

- 根节点使用 `role="group"`，并输出 `aria-label`、`aria-describedby`、`aria-disabled`、`aria-invalid`。
- 根节点语义通过 `ui_headless::labeled_group_attrs` 生成，并支持 `lang/dir` 接入。
- 暴露稳定语义标记：`data-state`、`data-tone`、`data-indicator-placement`、`data-label-source`、`data-aria-source`、`data-class-source`、`data-motion-source`。
- 描述存在时输出 `data-description="present"` 并自动关联描述 `id`。

## Motion and Styling

- `CheckboxFieldMotion` 默认值：
  - `enabled = true`
  - `transition_ms = default_text_field_motion_tokens().duration_ms`
  - `indicator_scale_pct = 100`
- `sanitize_motion` 会限制极值（`transition_ms` 上限、`indicator_scale_pct` clamp）。
- `attach_motion` 会输出组件所需 CSS 变量，并在 `prefers-reduced-motion` 或 `enabled=false` 时降级到最小动画反馈。
- 运行时只透出必要 CSS 变量，静态规则全部在 `styles.rs`。

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：预览当前 CheckboxField 效果与语义标记。
- `Config`：通过交互控制项切换 tone/placement/invalid/disabled/description/class。
- `Code`：实时生成当前配置对应的调用代码，支持复制。
- `CSS Test`：加载 `styles.rs` 原始样式，支持 scoped 调整和回滚。
- `对比`：页面保留多组状态矩阵（受控、quiet+invalid、disabled）用于横向比较。
