# Surface

`Surface` 是一个基础容器组件，用来提供稳定的布局底板与语义状态标记。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
<Surface>
  <div>"Hello Surface"</div>
</Surface>
```

- 默认用法不需要手动接线 `ui-state-primitives` / `ui-headless`。
- 先把内容包起来即可，后续再按需加参数。

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

- 高级参数用于自定义语义来源标记与样式扩展。
- 默认路径优先，只有需要时再增加 `aria_label` / `class_name` 等高级参数。

## docs-app 等价入口

- `apps/docs-app/src/pages/components/pages/layout_extra_surface.rs`
- Playground 顺序保持“默认用法在前，进阶用法在后”。

## Source-first Copy-Paste Ready

- docs-app Playground 默认提供可复制代码块，复制内容会自动补齐必要 imports（由 `apps/docs-app/src/playground.rs` 统一处理）。
- `Surface` 示例代码可直接作为最小片段使用；常见前提如下：
  - 依赖：`ui-components`
  - 功能开关：`component-surface`（按需配合 `inject-css`）
- 真实源码落点：
  - `crates/ui-components/src/surface/mod.rs`
  - `crates/ui-components/src/surface/logic.rs`
  - `crates/ui-components/src/surface/view.rs`
  - `crates/ui-components/src/surface/styles.rs`
  - `crates/ui-components/src/surface/motion.rs`
