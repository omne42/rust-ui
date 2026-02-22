# IconsUi

`IconsUi` 是 UI 内置图标入口组件，基于 `Iconset` 封装默认图标注册表，并输出稳定状态来源标记。

## 目标 / 非目标 / 风险边界

- 目标：提供一致的 `ui:*` 图标命名入口与可替换 glyph 注册能力。
- 非目标：不负责通用图标系统抽象（通用能力在 `Icon` / `Iconset`）。
- 风险边界：`icon` 归一化与来源标记必须在 `logic.rs` 统一处理，避免视图层分叉判断。

## Architecture Layers

- `logic.rs`：图标引用归一化、默认 glyph 注册、状态派生与 class 组合。
- `view.rs`：包装 `Iconset`，注入默认 `ui` glyphs 并透传尺寸/色调等参数。
- `styles.rs`：状态与来源标记样式契约。
- `mod.rs`：导出 `IconsUi`，并重导出 `IconsetGlyph`、`IconsUiSize`、`IconsUiTone`。

## Hello World

```rust
use ui::IconsUi;

view! { <IconsUi icon="check".to_string() is_decorative=true /> }
```

## 自定义 Glyph

```rust
use ui::{IconsUi, IconsetGlyph};

let glyphs = vec![
    IconsetGlyph::new("ui:rocket", "🚀").with_aria_label("Rocket"),
];

view! { <IconsUi icon="rocket".to_string() is_decorative=false glyphs=glyphs /> }
```

## API 约定

- 必填：`icon`
- 可选：`size`、`tone`、`is_disabled`、`is_decorative`、`aria_label`、`class_name`、`glyphs`
- 图标引用归一化：
  - `""` -> 默认 `ui:help`
  - 无命名空间（如 `check`）-> 自动前缀为 `ui:check`
  - 显式命名空间（如 `ui:check`）-> 直接使用
- 语义观测：`data-icon-reference`、`data-icon-reference-source`、`data-aria-source`、`data-glyph-source`

## Source-first

- 组件源码：`crates/ui/src/icon/ui/{mod,logic,view,styles}.rs`
- 依赖组件：`crates/ui/src/icon/set/`
- package feature：`component-icons_ui`（依赖 `component-iconset`，可选叠加 `inject-css`）

## Docs Playground

- docs-app `icons-ui` 页面已提供 `Interactive Playground`，包含展示区 + Config + Code + CSS Test。
- 同页包含多状态对比：内置 glyph、自定义注册、source/state markers。
