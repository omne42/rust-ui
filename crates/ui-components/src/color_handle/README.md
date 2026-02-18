# ColorHandle

`ColorHandle` 是颜色选择交互里的拖拽句柄原语，负责位置、焦点、拖拽态与 loupe 可见性语义。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可配置动效的颜色句柄视图原语。
- 非目标：不在组件层实现完整取色状态机或业务色值存储。
- 风险边界：状态归一化在 `logic.rs`，样式契约在 `styles.rs`，不要在 `view.rs` 临时分叉逻辑。

## Architecture Layers

- `logic.rs`：颜色/文案归一、状态派生、来源标记。
- `view.rs`：结构渲染与 `ColorThumb` 装配，输出稳定 `data-*`。
- `motion.rs`：`ColorHandleMotion` 参数清洗与 CSS 变量注入。
- `styles.rs`：静态样式契约（`--ui-color-handle-motion-duration` 等）。
- `mod.rs`：公开 `ColorHandle` / `ColorHandleMotion` 与状态输入输出类型。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `color` | `Option<String>` | `None` |
| `disabled` | `bool` | `false` |
| `focused` | `bool` | `false` |
| `dragging` | `bool` | `false` |
| `show_loupe` | `bool` | `true` |
| `x_percent` | `f32` | `50.0` |
| `y_percent` | `f32` | `50.0` |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL` |
| `class_name` | `Option<String>` | `None` |
| `motion` | `ColorHandleMotion` | `ColorHandleMotion::default()` |

## Docs Playground（展示 / Config / Code / CSS Test）

- 展示：docs-app 提供 baseline 对比视图（默认样式 vs 配置样式）。
- Config：提供颜色、坐标、disabled/focused/dragging/show_loupe、motion 时长等控制项。
- Code：支持 copy-ready 代码片段，反映当前配置状态。
- CSS Test：支持 scoped CSS 编辑与恢复，并显示 `ActualConfig` 快照。

## 对比场景

- `Focused + Dragging + Position`
- `Disabled + Custom Class + Loupe Off`
- `Workbench (Display + Config + Code + CSS Test)`
