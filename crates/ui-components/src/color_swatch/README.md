# ColorSwatch

`ColorSwatch` 是颜色预览组件，统一处理尺寸/形状/圆角/透明度状态，支持 `is_bordered`、`is_decorative` 主 API 与 legacy alias 兼容，并输出稳定语义 marker。

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch()`

对比场景（同页可见）：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Opaque | `color="#ffcc00"` | `data-alpha="opaque"` |
| Translucent | `color="rgba(...,0.35)"` | `data-alpha="translucent"` |
| Transparent | `color="rgba(...,0)"` | `data-alpha="transparent"` |
| Empty | `color=""` | `data-state="empty"` |
| Legacy bool alias | `bordered/decorative` | `data-bordered-source="legacy-alias"` |

## config（Actual Config）

docs-app 的 `Interactive Playground (展示 / Config / Code / CSS Test)` 会实时输出配置快照：

```text
ColorSwatchActualConfig {
  color: "#2663eb",
  size: Md,
  rounding: Default,
  shape: Square,
  is_bordered: true,
  is_decorative: false,
  bool_source: "is-prefixed",
  data_alpha: "opaque",
  data_state: "framed",
  class: "ui-color-swatch ui-color-swatch--size-md ...",
}
```

## code（Copy/Paste）

最小可用：

```rust
<ColorSwatch color="#ffcc00".to_string() />
```

多场景对比：

```rust
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="rgba(38, 99, 235, 0.35)".to_string() shape=ColorSwatchShape::Wide />
<ColorSwatch color="rgba(255, 0, 0, 0)".to_string() color_name="No fill".to_string() is_bordered=true />
<ColorSwatch color="".to_string() is_bordered=true />
```

## css test（Scoped CSS Test）

docs-app playground 已接入：

- `test_source_path="crates/ui-components/src/color_swatch/styles.rs"`
- `test_css_source=ui_components::color_swatch::styles::CSS`

可在测试面板写局部覆盖：

```css
:scope .ui-color-swatch[data-alpha="translucent"] {
  border-color: color-mix(in oklab, var(--ui-accent) 60%, transparent);
}
```

## API 快速表

| Prop | Type | 默认值 |
| --- | --- | --- |
| `color` | `Option<String>` | `None` |
| `color_name` | `Option<String>` | `None` |
| `size` | `ColorSwatchSize` | `Md` |
| `rounding` | `ColorSwatchRounding` | `Default` |
| `shape` | `ColorSwatchShape` | `Square` |
| `is_bordered` | `Option<bool>` | `None`（默认 `true`） |
| `bordered` | `Option<bool>` | `None`（兼容别名） |
| `is_decorative` | `Option<bool>` | `None`（默认 `false`） |
| `decorative` | `Option<bool>` | `None`（兼容别名） |
| `aria_label` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
