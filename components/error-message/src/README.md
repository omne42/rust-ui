# ErrorMessage

`ErrorMessage` 是表单错误反馈基元，组合 `ui-state-primitives` 与 `ui-headless`，输出稳定 `role/aria/data-*` 语义契约。

## display（展示区）

- `tone` 对比：`Auto`（归一到 negative）、`Neutral`、`Negative`。
- `state` 对比：默认 / `disabled` / `truncate`。
- `element` 对比：`span` / `p` / `div`。

## config（配置区）

docs-app workbench settings 覆盖以下核心输入：

| Config | Values | 作用 |
| --- | --- | --- |
| `tone` | `Auto` / `Neutral` / `Negative` | 错误语义强度 |
| `element` | `Span` / `Paragraph` / `Div` | 输出标签类型 |
| `disabled` | `bool` | 切换 `data-state=disabled` |
| `truncate` | `bool` | 开启截断样式状态 |
| `class_name` | `Option<String>` | 样式来源标记（default/custom） |
| `aria_label` | `Option<String>` | 无障碍名称来源标记 |

## code（代码区）

最小示例：

```rust
<ErrorMessage text="Invalid email address".to_string() />
```

状态示例：

```rust
<ErrorMessage
  text="Very long validation message...".to_string()
  tone=ErrorMessageTone::Neutral
  truncate=true
  class_name="docs-error-message-custom".to_string()
/>
```

## css test（样式测试区）

- docs-app `Show test` 面板支持 scoped CSS 热编辑。
- 默认注入源：`components/error-message/src/styles.rs` 的 `CSS` 常量。
- 推荐围绕稳定标记编写测试样式：
  - `data-tone`
  - `data-state`
  - `data-disabled`
  - `data-truncate`
  - `data-class-source`

## 多场景对比（Comparison Matrix）

- `Auto tone + default paragraph`
- `Neutral tone + paragraph`
- `Negative tone + span`
- `Disabled + truncate + custom class`

这组对比覆盖视觉强度、结构标签、可访问状态与样式来源四条关键轴。
