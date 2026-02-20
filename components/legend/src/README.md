# Legend

`Legend` 是 `fieldset` 标题语义组件，默认走 snapshot 渲染路径，语义属性由 `ui-headless::use_legend` 提供。

## Hello World

```rust
use ui_components::Legend;

view! {
    <fieldset>
        <Legend text="Notification settings".to_string() />
    </fieldset>
}
```

## 状态示例

```rust
use ui_components::{Legend, LegendTone};

view! {
    <fieldset>
        <Legend
            text="Billing preferences".to_string()
            tone=LegendTone::Muted
            is_required=Some(true)
            required_indicator="(required)".to_string()
        />
    </fieldset>
}
```

## API 约定

- 布尔状态：`is_required` / `is_disabled`
- 兼容迁移：保留 `required` / `disabled`，内部统一归一为 `is_*`
- i18n/l10n：`lang` / `dir` 透传到 headless 语义契约
- 语义观测：根节点输出稳定 `data-*` / `aria-*` / `data-ui-*` 标记

## Source-first / Copy-Paste Ready

- docs 入口：`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::legend()`
- 组件源码：`components/legend/src/{mod,logic,view,styles,motion}.rs`
- package 模式前提：`component-legend`（样式注入可选叠加 `inject-css`）
