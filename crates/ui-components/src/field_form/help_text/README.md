# HelpText

`HelpText` 是表单辅助文本组件，负责统一 `description/error` 来源、tone 和语义标记。

## 目标 / 非目标 / 风险边界

- 目标：稳定表达描述/错误文本状态，并提供可测试 `data-*` / `aria-*` 契约。
- 非目标：不承担业务校验引擎和异步重试协议。
- 风险边界：文本来源和 tone 归一化只放在 `logic.rs`。

## Architecture Layers

- `logic.rs`：tone/message/source 归一化与状态派生。
- `view.rs`：结构渲染与语义标记挂载。
- `motion.rs`：错误态动效（wasm attach + non-wasm no-op）。
- `styles.rs`：静态 token-first CSS。
- `mod.rs`：最小导出面（`HelpText`、`HelpTextTone`、`HelpTextMotion`）。

## API

| Prop | Type | Default |
| --- | --- | --- |
| `tone` | `HelpTextTone` (`Auto` / `Neutral` / `Negative`) | `Auto` |
| `invalid` | `bool` | `false` |
| `disabled` | `bool` | `false` |
| `show_error_icon` | `bool` | `false` |
| `description` | `Option<String>` | `None` |
| `error_message` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | fallback to `DEFAULT_ARIA_LABEL` |
| `motion` | `HelpTextMotion` | `HelpTextMotion::default()` |
| `class_name` | `Option<String>` | `None` |

## Hello World

```rust
use ui_components::HelpText;

view! {
    <HelpText description="Use at least 12 characters.".to_string() />
}
```

## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）

对应页面：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 的 `help_text()`

- 展示区：`Primary` 当前配置预览 + `对比矩阵` 多状态样例。
- Config 区：切换 tone、invalid、disabled、error icon、error message、custom aria、custom class。
- Code 区：根据当前配置生成可复制 `HelpText` 代码。
- CSS Test 区：展示 `crates/ui-components/src/field_form/help_text/styles.rs` 的 `CSS` 常量，并显示当前配置快照。

## 多场景对比（对比矩阵）

- 场景 A：`Neutral + description`（正常提示）。
- 场景 B：`Negative + invalid + icon + error_message`（错误提示）。
- 场景 C：`invalid + disabled`（禁用错误态退化）。

## 语义契约

根节点标记：

- `data-slot="help-text"`
- `data-tone`
- `data-state`
- `data-message-kind`
- `data-invalid`
- `data-disabled`
- `data-aria-source`
- `data-error-source`
- `data-class-source`
- `data-motion-source`

可访问性标记：

- `aria-label`
- `aria-invalid`
- `aria-disabled`
- 错误文本 `role="alert"`

## Docs and Feature

- docs-app entry: `apps/docs-app/src/pages/components/pages/forms_extra.rs::help_text()`
- source: `crates/ui-components/src/field_form/help_text/{mod,logic,view,styles,motion}.rs`
- feature: `component-help_text`（可选 `inject-css`）
