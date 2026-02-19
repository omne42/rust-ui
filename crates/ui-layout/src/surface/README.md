# Surface

`Surface` 是布局底板组件，使用 canonical API：`is_bordered`、`is_padded`。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
<Surface>
  <div>"Hello Surface"</div>
</Surface>
```

## 常见用法

```rust
<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>
  <div>"Default raised surface"</div>
</Surface>

<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat is_bordered=true>
  <div>"Subtle flat bordered surface"</div>
</Surface>

<Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false>
  <div>"Strong floating compact surface"</div>
</Surface>
```

## 再进阶（高级控制）

```rust
<Surface
  tone=SurfaceTone::Strong
  elevation=SurfaceElevation::Floating
  is_bordered=true
  aria_label="Deployment summary".to_string()
  class_name="docs-surface-custom".to_string()
>
  <div>"Custom class + aria source marker"</div>
</Surface>
```

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/layout_extra_surface.rs::surface()`。

对比场景：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Default Raised | `tone=Default elevation=Raised` | `data-state="padded"` |
| Subtle Flat Bordered | `is_bordered=true` | `data-bordered="true"` |
| Strong Floating Compact | `is_padded=false` | `data-state="framed"` |

## config（Actual Config）

```text
SurfaceActualConfig {
  tone: Subtle,
  elevation: Flat,
  is_bordered: true,
  is_padded: true,
  bordered_source: "is-prop",
  padded_source: "default",
}
```

## code（Copy/Paste）

```rust
<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat is_bordered=true>
  <div>"Subtle flat bordered surface"</div>
</Surface>

<Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false>
  <div>"Strong floating compact surface"</div>
</Surface>
```

## css test（Scoped CSS Test）

- `test_source_path="crates/ui-layout/src/surface/styles.rs"`
- `test_css_source=ui_layout::surface::styles::CSS`

```css
:scope .ui-surface[data-bordered="true"] {
  box-shadow: inset 0 0 0 1px var(--ui-border);
}
```

## Source-first Copy-Paste Ready

- feature: `component-surface`
- 源码落点：
  - `crates/ui-layout/src/surface/mod.rs`
  - `crates/ui-layout/src/surface/logic.rs`
  - `crates/ui-layout/src/surface/view.rs`
  - `crates/ui-layout/src/surface/styles.rs`
  - `crates/ui-layout/src/surface/motion.rs`
