# Legend

`Legend` 是 `fieldset` 标题语义组件。先从最小示例开始，不需要先理解底层分层实现。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
use ui::Legend;

view! {
    <fieldset>
        <Legend text="Notification settings".to_string() />
    </fieldset>
}
```

默认 API 路径优先：只传 `text` 就能稳定渲染，适合作为新手起点。

## 常见用法

```rust
use ui::Legend;

view! {
    <fieldset>
        <Legend text="Notification settings".to_string() is_required=true />
        <Legend text="Read-only group".to_string() is_disabled=true />
    </fieldset>
}
```

## 再进阶（高级控制）

只有当你需要视觉与语义细节时，再打开高级参数（`tone`、`required_indicator`、`class_name`、`lang`、`dir`、`motion`）。

```rust
use ui::{Legend, LegendTone};

view! {
    <fieldset>
        <Legend
            text="Billing preferences".to_string()
            tone=LegendTone::Muted
            is_required=true
            required_indicator="(required)".to_string()
        />
    </fieldset>
}
```

## API 约定

- 布尔状态：`is_required` / `is_disabled`
- i18n/l10n：`lang` / `dir` 透传到 headless 语义契约
- 语义观测：根节点输出稳定 `data-*` / `aria-*` / `data-ui-*` 标记
- 不需要用户手动接线 `ui-state-primitives` / `ui-headless`

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::legend()`
- 组件源码：`components/legend/src/{mod,logic,view,styles,motion}.rs`
- package 模式前提：`component-legend`（样式注入可选叠加 `inject-css`）
