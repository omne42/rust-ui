# ColorSwatch

`ColorSwatch` 是颜色预览组件，使用 canonical API：`is_bordered`、`is_decorative`。

## Hello World（最小可用）

```rust
<ColorSwatch color="#2663eb".to_string() />
```

## 常见用法

```rust
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="rgba(38, 99, 235, 0.35)".to_string() shape=ColorSwatchShape::Wide />
<ColorSwatch color="rgba(255, 0, 0, 0)".to_string() color_name="No fill".to_string() is_bordered=true />
```

## 新手路径（先用起来，再进阶）

1. 先跑默认路径：`<ColorSwatch color="#2663eb".to_string() />`
2. 再加常见参数：`color_name`、`size`、`shape`、`is_bordered`
3. 最后再用进阶参数：`is_decorative`、`aria_label`、`class_name`、`lang`、`dir`

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch()`

对比场景：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Opaque | `color="#ffcc00"` | `data-alpha="opaque"` |
| Translucent | `color="rgba(...,0.35)"` | `data-alpha="translucent"` |
| Transparent | `color="rgba(...,0)"` | `data-alpha="transparent"` |
| Empty | `color=""` | `data-state="empty"` |

## config（Actual Config）

```text
ColorSwatchActualConfig {
  color: "#2663eb",
  size: Md,
  rounding: Default,
  shape: Square,
  is_bordered: true,
  is_decorative: false,
  bool_source: "is-prop",
  data_alpha: "opaque",
  data_state: "framed",
}
```

## code（Copy/Paste）

```rust
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="rgba(38, 99, 235, 0.35)".to_string() shape=ColorSwatchShape::Wide />
<ColorSwatch color="rgba(255, 0, 0, 0)".to_string() color_name="No fill".to_string() is_bordered=true />
<ColorSwatch color="".to_string() is_bordered=true />
```

## css test（Scoped CSS Test）

- `test_source_path="components/color-swatch/src/styles.rs"`
- `test_css_source=ui::color::swatch::styles::CSS`

```css
:scope .ui-color-swatch[data-alpha="translucent"] {
  border-color: color-mix(in oklab, var(--ui-accent) 60%, transparent);
}
```

## Source-first

- 组件源码：`components/color-swatch/src/{mod,logic,view,styles,motion}.rs`
- package feature：`component-color_swatch`（可选叠加 `inject-css`）
- docs-app 入口：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch()`
