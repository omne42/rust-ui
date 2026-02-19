# ScrollArea

`ScrollArea` 是滚动容器组件，使用 canonical API：`is_disabled`。

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/layout_extra.rs::scroll_area()`

对比场景：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Vertical default | `orientation=Vertical` | `data-orientation="vertical"` |
| Custom max-height | `max_height_px=180` | `data-max-height="custom"` |
| Both + disabled | `orientation=Both is_disabled=true` | `data-disabled="true"` |

## config（Actual Config）

```text
ScrollAreaActualConfig {
  orientation: Vertical,
  is_disabled: false,
  disabled_source: "default",
  max_height_source: "custom",
  aria_source: "default",
}
```

## code（Copy/Paste）

```rust
<ScrollArea max_height_px=180>
  <div class="docs-stack docs-stack--tight">
    <div>"Release note 1"</div>
    <div>"Release note 2"</div>
  </div>
</ScrollArea>

<ScrollArea orientation=ScrollAreaOrientation::Both is_disabled=true max_height_px=120>
  <div>"Disabled logs"</div>
</ScrollArea>
```

## css test（Scoped CSS Test）

- `test_source_path="crates/ui-layout/src/scroll_area/styles.rs"`
- `test_css_source=ui_layout::scroll_area::styles::CSS`

```css
:scope .ui-scroll-area[data-disabled="true"] {
  opacity: 0.75;
}
```
