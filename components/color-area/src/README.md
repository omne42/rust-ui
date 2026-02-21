# ColorArea

`ColorArea` 是一个二维颜色坐标选择组件，基于 `ui-state-primitives + ui-headless + ui-motion` 组合实现。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的二维颜色选择能力（X/Y 轴）。
- 非目标：不在组件层承载业务状态容器、不实现全局主题系统、不实现跨组件动效引擎。
- 风险边界：出现状态机/交互语义漂移时，优先回迁到 `ui-state-primitives` 或 `ui-headless`，避免在 `view.rs` 打补丁。

## Architecture Layers

- `logic.rs`：props 归一、默认值与来源标记、Agent Contract 映射。
- `view.rs`：Leptos 结构渲染、headless 语义挂载、事件绑定。
- `motion.rs`：`ColorAreaMotion` 合约、style 变量拼接、motion 来源标记。
- `styles.rs`：静态 CSS 契约，状态分支基于稳定 `data-*` 标记。
- `mod.rs`：对外导出 `ColorArea`、`ColorAreaMotion`、`A11yDirection`。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `label` | `Option<String>` | i18n fallback (`color_area_label`) |
| `is_disabled` | `Option<bool>` | `None`（优先） |
| `value` | `Option<Signal<(f32, f32)>>` | `None` |
| `default_value` | `Option<(f32, f32)>` | `(1.0, 1.0)` |
| `on_value_change` | `Option<Callback<(f32, f32)>>` | `None` |
| `step` | `f32` | `0.1` |
| `grid_size` | `usize` | `11` |
| `preview_color` | `Option<String>` | `None` |
| `motion` | `ColorAreaMotion` | `ColorAreaMotion::default()` |
| `aria_label` | `Option<String>` | i18n fallback (`color_area_aria_label`) |
| `x_axis_label` | `Option<String>` | i18n fallback (`color_area_x_axis_label`) |
| `y_axis_label` | `Option<String>` | i18n fallback (`color_area_y_axis_label`) |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

控制模式：
- 受控：传 `value + on_value_change`。
- 非受控：传 `default_value`（可选）并省略 `value`。

## Hello World（最小可用）

```rust
<ColorArea
  id_base="docs-color-area-basic".to_string()
/>
```

## 常见用法

```rust
<ColorArea
  id_base="docs-color-area-brand".to_string()
  label="Saturation / Lightness".to_string()
  value=value.into()
  on_value_change=on_value_change
  preview_color="#7c3aed".to_string()
/>
```

```rust
<ColorArea
  id_base="docs-color-area-disabled".to_string()
  label="Accent area".to_string()
  default_value=(0.25, 0.85)
  grid_size=15
  step=0.05
  is_disabled=true
  class_name="docs-color-area-custom".to_string()
/>
```

## Semantics and Accessibility

- 根节点挂载 `role/group` + `aria-label` + `aria-labelledby`，并支持 `lang/dir`。
- 网格使用 `role="grid"`，单元格使用 `role="gridcell"` + `aria-selected`。
- 键盘路径支持 `ArrowLeft/Right/Up/Down`、`Home`、`End`。
- 稳定状态标记：`data-state`、`data-value-x/y`、`data-selected-col/row`、`data-*-source`。

## Motion and Fallback

- `motion.rs` 只输出 CSS 变量（`--ui-color-area-motion-duration`），不在组件层自建执行器。
- reduced-motion 通过样式降级（`@media (prefers-reduced-motion: reduce)`）。

## Agent Contract / 流式降级

- 根节点输出：
  - `data-ui-schema="ui.color-area.agent-contract.v1"`
  - `data-ui-stream-support="optional"`
  - `data-ui-stream-fallback="snapshot"`
  - `data-ui-stream-mode="snapshot"`
  - `data-ui-output-status="verified"`
  - `data-ui-intent="select-color-point"`
  - `data-ui-action` / `data-ui-state` / `data-ui-source`

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_color.rs`
- 页面：`ColorArea`
- Playground：
  - `Controlled Grid Selection`
  - `Disabled + Custom Grid + Custom Class`
  - `Interactive Playground`（展示 / Config / Code / CSS Test）

## 展示区（Display / Config / Code / CSS Test）

- Display：工作台主样例 + 两个固定对照样例（中心态、禁用态）并排展示，便于差异对比。
- Config：通过 `SegmentedControl` / `Switch` 调整 `grid_size`、`step`、`preview_color`、默认坐标、disabled、自定义轴标签、自定义 class。
- Code：实时生成可复制代码片段（与当前工作台配置一致）。
- CSS Test：加载 `crates/ui-components/src/color/area/styles.rs`，在 Playground 内做作用域化样式测试。

对比场景（至少三种）：
- 可调主样例（受工作台配置驱动）
- 固定对照 A：中心位置、标准网格
- 固定对照 B：禁用态、细步进 + 大网格

## Source-first Copy-Paste Ready

- 真实源码落点：
  - `crates/ui-components/src/color/area/mod.rs`
  - `crates/ui-components/src/color/area/logic.rs`
  - `crates/ui-components/src/color/area/view.rs`
  - `crates/ui-components/src/color/area/styles.rs`
  - `crates/ui-components/src/color/area/motion.rs`
- 状态原语：`crates/ui-state-primitives/src/color_area.rs`
- 交互语义：`crates/ui-headless/src/color_area.rs`
