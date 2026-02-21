# ColorLoupe

`ColorLoupe` 是一个基于 `ui-state-primitives` 的颜色放大镜展示组件，用于颜色编辑场景中的局部预览。

## 快速开始（先用起来）

### Hello World（最小可用）

```rust
<ColorLoupe
  id_base="demo-color-loupe".to_string()
  color="#3b82f6".to_string()
  is_open=true
/>
```

### 常见用法

```rust
<ColorLoupe
  id_base="demo-color-loupe-disabled".to_string()
  color="rgba(59, 130, 246, 0.6)".to_string()
  x_percent=18.0
  y_percent=74.0
  is_open=true
  is_disabled=true
  aria_label="Accent loupe".to_string()
/>
```

默认先用 `id_base + color + is_open` 跑通；需要时再增加 `is_disabled/x_percent/y_percent/aria_label/class_name/lang/dir`。

## 进阶（理解实现与契约）

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、状态归一化集中的颜色放大镜视图基元。
- 非目标：不承载业务状态管理，不在组件层实现跨组件交互协议。
- 风险边界：状态规则变更优先下沉 `ui-state-primitives`，避免在 `view.rs` 分散条件分支。

## Architecture Layers

- `ui-state-primitives/src/color_loupe.rs`：位置桶归一、open/disabled/color 状态与来源标记。
- `logic.rs`：桥接 primitives，负责 class 组装。
- `view.rs`：Leptos 结构渲染、语义标记挂载。
- `styles.rs`：token-first 静态 CSS 与开启动画样式。
- `mod.rs`：公开最小稳定 API。

## API (Table)

### ColorLoupe Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `color` | `Option<String>` | `None` |
| `is_open` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `x_percent` | `f32` | `50.0` |
| `y_percent` | `f32` | `50.0` |
| `aria_label` | `Option<String>` | `"Color loupe"` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### ColorLoupe Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 展示型组件，无用户交互事件回调 | `-` |

## Interactive Playground（展示区）

### 展示区（Display）

- docs 页面：`apps/docs-app/src/pages/components/pages/forms_color.rs`
- Playground：
  - `Open + Position Buckets`
  - `Disabled + Custom Label + Custom Class`
  - `Interactive Playground`
- `Interactive Playground` 同时渲染 1 个可调实例 + 1 个固定对照实例（blue/end/is_open）。

### Config 区（Config）

- `Color`：`Amber / Emerald / Sky / Alpha`
- `Position bucket`：`Start / Center / End`（映射到 `x_percent/y_percent`）
- `is_open` 开关
- `is_disabled` 开关
- `Custom aria_label` 开关
- `Custom class` 开关

### Code 区（Code）

- Workbench 会根据当前配置实时生成 `ColorLoupe` 代码片段。
- 代码包含颜色、位置桶、is_open/is_disabled、aria/class 等完整参数，便于复现。

### CSS Test 区（CSS Test）

- `test_source_path` 指向：`components/color-loupe/src/styles.rs`
- 支持在 playground 内局部编辑 scoped CSS，对比 `is_open/is_disabled/x-y bucket` 的样式分支。
- 显示 `Actual config`（实时配置快照）用于回归核对。

## 多种不同情况下的对比显示

| 场景 | 关键输入 | 预期对比点 |
| --- | --- | --- |
| 开启态定位 | `is_open=true` + `Start/Center/End` | 位置桶 class 与 `data-x/y-bucket` 一致 |
| 禁用态 | `is_disabled=true` | `data-state="disabled"` 且不可开启 |
| 自定义可访问名 | `aria_label` 自定义 | `data-aria-source="custom"` |
| 自定义样式来源 | `class_name` 自定义 | `data-class-source="custom"` + 自定义类 |
| 透明色预览 | `rgba(...)` | checker + fill 对比可读 |

## Semantics and Accessibility

- 根节点挂载 `role="img"` 与 `aria-label`。
- 通过 `ui_headless::a11y::locale_attrs` 透传 `lang`/`dir`（LTR/RTL）上下文。
- 暴露稳定契约：`data-state`、`data-open`、`data-disabled`、`data-x`、`data-y`、`data-x-bucket`、`data-y-bucket`、`data-aria-source`、`data-class-source`。
- slot 标记稳定：`color-loupe`、`color-loupe-bubble`、`color-loupe-checker`、`color-loupe-fill`、`color-loupe-tail`。

## Motion and Fallback

- 组件使用 CSS keyframes 做视觉开启反馈。
- 非 runtime 动效驱动，不依赖 wasm 动效执行器。
- SSR/non-wasm 路径无额外运行时依赖。

## Docs / Test References

- docs page: `apps/docs-app/src/pages/components/pages/forms_color.rs` (`slug="color-loupe"`)
- semantics test: `components/color-loupe/test/semantics.rs`
- state primitive test: `crates/ui-state-primitives/src/color_loupe.rs`
