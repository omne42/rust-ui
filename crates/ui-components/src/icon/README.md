# Icon

`Icon` 是 display 层图标语义原语，聚焦尺寸、色调、可访问性标记与状态来源可观测性。

## 目标 / 非目标 / 风险边界

- 目标：提供统一尺寸/色调契约与可访问语义（decorative vs labeled）。
- 非目标：不承载图标资源管理或业务图标映射表。
- 风险边界：状态推导放在 `logic.rs`，避免在 `view.rs` 直接拼接条件分支。

## Architecture Layers

- `logic.rs`：`IconSize`/`IconTone`、状态归一、class 组合。
- `view.rs`：`role`/`aria-*`/`data-*` 挂载与 glyph 容器渲染。
- `styles.rs`：size/tone/disabled/decorative/custom-class 静态样式契约。
- `mod.rs`：公开 `Icon`、`IconSize`、`IconTone` 与状态输入输出类型。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `size` | `IconSize` | `IconSize::Md` |
| `tone` | `IconTone` | `IconTone::Default` |
| `disabled` | `bool` | `false` |
| `decorative` | `bool` | `true` |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL`（仅 non-decorative 生效） |
| `class_name` | `Option<String>` | `None` |
| `children` | `Children` | required |

## Docs Playground（展示 / Config / Code / CSS Test）

- 展示：docs-app 提供 baseline / configured / disabled 对比展示。
- Config：支持 size、tone、glyph、decorative、disabled、自定义 class、aria_label 调整。
- Code：生成 copy-ready 代码片段，便于直接粘贴验证。
- CSS Test：支持 scoped CSS 编辑、样式回滚与 `ActualConfig` 输出。

## 对比场景

- `Size + Tone Matrix`
- `Accessible + Disabled + Custom Class`
- `Workbench (Display + Config + Code + CSS Test)`
