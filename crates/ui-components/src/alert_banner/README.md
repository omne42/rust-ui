# AlertBanner

`AlertBanner` 是一个用于展示状态提示的轻量展示组件，支持 `tone`、`fill`、可选标题/描述、插槽内容与 reveal motion 合同。

## Hello World

```rust
use leptos::prelude::*;
use ui_components::{AlertBanner, AlertBannerTone};

view! {
    <AlertBanner tone=AlertBannerTone::Info>
        "Install now to keep your workspace secure."
    </AlertBanner>
}
```

## 常用参数

- `tone`: 语义基调（`Neutral/Info/Positive/Notice/Negative`）。
- `fill`: 视觉填充（`Border/Subtle/Bold`）。
- `is_hide_icon`: 推荐布尔参数，控制是否隐藏图标。
- `hide_icon`: 兼容别名，建议迁移到 `is_hide_icon`。
- `motion`: 自定义 spring motion 合同（默认值可直接使用）。
- `lang` / `dir`: 透传 locale 语义到组件根节点。

## 文档入口

- docs-app: `/#/components/alert-banner`
- 源码: `crates/ui-components/src/alert_banner/`
