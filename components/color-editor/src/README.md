# ColorEditor

`ColorEditor` 是一个基于 `ui-state-primitives` + `ui-headless` 组合的颜色编辑组件，通过 `ColorArea + ColorSlider + ColorField` 装配出完整交互。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、状态来源可观测的颜色编辑基元。
- 非目标：不在组件层实现业务状态管理、全局调色板协议或通用动画引擎。
- 风险边界：跨层职责漂移（primitives/headless/theme）时，优先回到对应层修复，不在 `view.rs` 追加补丁逻辑。

## Architecture Layers

- `logic.rs`：仅消费 `ui-state-primitives::color_editor` 的归一化、状态派生与颜色格式化能力（re-export）。
- `view.rs`：Leptos 结构渲染，装配 `ColorArea/ColorSlider/ColorField`，并挂载 `locale_attrs(lang/dir)`。
- `styles.rs`：仅静态 CSS 契约，样式通过 `var(--ui-*)` 驱动。
- `motion.rs`：`ColorEditorMotion` 契约（复用 `ColorSliderMotion`），统一默认/自定义动效来源标记。
- `mod.rs`：公开最小稳定 API（`ColorEditor`、`ColorEditorFormat`、`ColorEditorMotion`、状态类型与默认常量）。

## API (Table)

### ColorEditor Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `label` | `Option<String>` | `None`（回退到 `DEFAULT_LABEL`） |
| `aria_label` | `Option<String>` | `None`（优先回退到 `label`，再回退 `DEFAULT_ARIA_LABEL`） |
| `disabled` | `bool` | `false` |
| `selected_color` | `Option<Signal<Option<String>>>` | `None` |
| `default_selected_color` | `Option<String>` | `None` |
| `on_selected_change` | `Option<Callback<Option<String>>>` | `None` |
| `format` | `Option<Signal<ColorEditorFormat>>` | `None` |
| `default_format` | `Option<ColorEditorFormat>` | `None`（回退到 `ColorEditorFormat::default()`） |
| `on_format_change` | `Option<Callback<ColorEditorFormat>>` | `None` |
| `hide_alpha_channel` | `bool` | `false` |
| `default_hue` | `Option<f64>` | `None`（回退到 `DEFAULT_HUE`） |
| `default_alpha` | `Option<f64>` | `None`（回退到 `DEFAULT_ALPHA`） |
| `default_area` | `Option<(f32, f32)>` | `None`（回退到 `DEFAULT_AREA`） |
| `area_label` | `Option<String>` | `None`（回退 `"Saturation / Brightness"`） |
| `area_aria_label` | `Option<String>` | `None`（回退 `"Color area"`） |
| `hue_label` | `Option<String>` | `None`（回退 `"Hue"`） |
| `alpha_label` | `Option<String>` | `None`（回退 `"Alpha"`） |
| `value_label` | `Option<String>` | `None`（回退 `"Value"`） |
| `format_aria_label` | `Option<String>` | `None`（回退 `"Color format"`） |
| `preview_color` | `Option<String>` | `None`（根据 hue/area/alpha 计算） |
| `motion` | `ColorEditorMotion` | `ColorEditorMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### ColorEditor Events

| Event | Type | Default |
| --- | --- | --- |
| `on_selected_change` | `Callback<Option<String>>` | `None` |
| `on_format_change` | `Callback<ColorEditorFormat>` | `None` |

## Hello World（最小可用）

```rust
<ColorEditor id_base="brand-color".to_string() default_selected_color="#0ea5e9".to_string() />
```

- 默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。
- 进阶需求再按需开启受控值、格式切换、alpha 隐藏、motion 与 locale。

## Semantics and Accessibility

- 根节点为 `role="group"`，并输出稳定语义标记：
  - `data-slot="color-editor"`
  - `data-state`（`disabled` / `ready` / `empty`）
  - `data-format`（`hex` / `rgb` / `hsl` / `hsb`）
  - `data-alpha`（`hidden` / `visible`）
  - `data-motion-source` / `data-label-source` / `data-aria-source` / `data-class-source`
- 格式切换区使用 `role="tablist"` + `role="tab"` + `aria-selected`，通道预览区使用 `role="tabpanel"`。
- 使用 `ui_headless::locale_attrs` 挂载 `lang` / `dir`，支持 LTR/RTL 场景。

## Motion and Fallback

- `ColorEditorMotion` 复用 `ColorSliderMotion`，由 `motion.rs` 统一做 sanitize。
- `source_attr` 用于稳定区分默认动效与自定义动效，映射到 `data-motion-source`。
- 非 wasm 路径走安全降级（no-op 风格）以保证 SSR/tooling 编译稳定。

## Docs Playground（展示 / Config / Code / CSS Test）

- docs 页入口：`apps/docs-app/src/pages/components/pages/forms_color.rs` 中 `color_editor()`。
- Workbench Playground 提供四个面板能力：
  - 展示（Preview）：实时渲染主编辑器，并包含对照编辑器用于多场景比对。
  - Config：`test_config_signal` 输出当前格式、状态标记与 class 组合。
  - Code：`code_signal` 输出可复制的当前配置示例。
  - CSS Test：`test_css_source` 载入 `color_editor/styles.rs`，支持作用域内试验样式。
- 额外对比展示：
  - `Controlled Color + Controlled Format`
  - `Disabled + Alpha Hidden + Reduced Motion`

## Source-first

- 组件源码入口：
  - `components/color-editor/src/mod.rs`
  - `components/color-editor/src/logic.rs`
  - `components/color-editor/src/view.rs`
  - `components/color-editor/src/styles.rs`
  - `components/color-editor/src/motion.rs`
- 状态原语定义：
  - `crates/ui-state-primitives/src/color_editor.rs`
