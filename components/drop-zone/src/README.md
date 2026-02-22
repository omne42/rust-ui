# DropZone

`DropZone` 是一个用于拖拽/粘贴文件输入的基础组件。默认路径只需要渲染组件内容，不需要先理解底层分层结构。

## Hello World

```rust
use leptos::prelude::*;
use ui::DropZone;

view! {
    <DropZone>
        <div class="docs-drop-zone">"Drop files here"</div>
    </DropZone>
}
```

## 常见用法

- 禁用态：`is_disabled=true`
- 回调同步：`on_drop_files=Callback::new(|files| { /* sync to app state */ })`
- 自定义动效：`motion=DropZoneMotion { ..DropZoneMotion::default() }`

## 新手路径（先用起来，再进阶）

1. 先用默认 API 跑通：只传 `children`。
2. 需要状态同步时再加 `on_drop_files`。
3. 需要视觉调优时再加 `motion`。
4. 只有兼容旧调用时才使用 `disabled` 别名，默认使用 `is_disabled`。

## API 约定

- `label: Option<String>`：可选标签。
- `aria_label: Option<String>`：可选无障碍标签；未传时由 i18n 默认值兜底。
- `is_disabled: Option<bool>`：标准禁用命名。
- `disabled: Option<bool>`：兼容别名，优先级低于 `is_disabled`。
- `motion: Option<DropZoneMotion>`：动效契约。
- `on_drop_files: Option<Callback<Vec<DroppedFile>>>`：文件接收回调。
- `lang: Option<String>` / `dir: Option<A11yDirection>`：国际化与方向接入。

## Source-first

- 组件源码：`components/drop-zone/src/{mod,logic,view,styles,motion}.rs`
- package feature：`component-drop_zone`（可选叠加 `inject-css`）
- 依赖基线：

```toml
ui = { default-features = false, features = ["component-drop_zone", "inject-css"] }
```

## 文档入口

- docs-app 页面：`apps/docs-app/src/pages/components/pages/files.rs` 的 `drop_zone()`
- 组件源码：`components/drop-zone/src/{mod,logic,view,styles,motion}.rs`
