# HelpText

`HelpText` 是表单辅助文本组件，负责统一 `description/error` 来源、tone 和语义标记。

## 先用起来（默认路径）

- 默认 API 路径优先。
- 不需要用户手动接线 `ui-state-primitives` / `ui-headless`。

### Hello World（最小可用）

```rust
use ui::HelpText;

view! {
    <HelpText description="Use at least 12 characters.".to_string() />
}
```

## 常见用法

```rust
use ui::{HelpText, HelpTextTone};

view! {
    <>
        <HelpText
            tone=HelpTextTone::Neutral
            description="Use at least 12 characters.".to_string()
        />
        <HelpText
            is_invalid=true
            is_error_icon_visible=true
            error_message="Password does not meet complexity requirements.".to_string()
        />
    </>
}
```

## 再进阶（高级控制）

### API

| Prop | Type | Default |
| --- | --- | --- |
| `tone` | `HelpTextTone` (`Auto` / `Neutral` / `Negative`) | `Auto` |
| `is_invalid` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `is_error_icon_visible` | `bool` | `false` |
| `description` | `Option<String>` | `None` |
| `error_message` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | fallback to `DEFAULT_ARIA_LABEL` |
| `motion` | `HelpTextMotion` | `HelpTextMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### State Model

- `HelpText` 是纯输入渲染组件：不持有内部可变状态，不提供 `value/on_*_change/default_*` 受控-非受控三元组。
- 所有状态轴均由外部输入直接决定，组件内部不会回写或隐式切换到“半受控”模式。

### API Migration

- `invalid` -> `is_invalid`
- `disabled` -> `is_disabled`
- `show_error_icon` -> `is_error_icon_visible`
- 旧别名已移除，调用侧请按上表机械替换。

## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）

对应页面：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 的 `help_text()`

- 展示区：`Primary` 当前配置预览 + `对比矩阵` 多状态样例。
- Config 区：切换 tone、is_invalid、is_disabled、is_error_icon_visible、error message、custom aria、custom class。
- Code 区：根据当前配置生成可复制 `HelpText` 代码。
- CSS Test 区：展示 `components/help-text/src/styles.rs` 的 `CSS` 常量，并显示当前配置快照。

## 多场景对比（对比矩阵）

- 场景 A：`Neutral + description`（正常提示）。
- 场景 B：`Negative + is_invalid + icon + error_message`（错误提示）。
- 场景 C：`is_invalid + is_disabled`（禁用错误态退化）。

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

## 目标 / 非目标 / 风险边界

- 目标：稳定表达描述/错误文本状态，并提供可测试 `data-*` / `aria-*` 契约。
- 非目标：不承担业务校验引擎和异步重试协议。
- 风险边界：文本来源和 tone 归一化只放在 `logic.rs`。

## Architecture Layers

- `logic.rs`：tone/message/source 归一化与状态派生。
- `view.rs`：结构渲染与 `ui-headless` 语义挂载。
- `motion.rs`：错误态动效（wasm attach + non-wasm no-op）。
- `styles.rs`：静态 token-first CSS。
- `mod.rs`：最小导出面（`HelpText`、`HelpTextTone`、`HelpTextMotion`）。

## Docs and Feature

- docs-app entry: `apps/docs-app/src/pages/components/pages/forms_extra.rs::help_text()`
- source: `components/help-text/src/{mod,logic,view,styles,motion}.rs`
- feature: `component-help_text`（可选 `inject-css`）
