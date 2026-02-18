# Keyboard

`Keyboard` 是一个用于渲染 `<kbd>` 语义标签的展示组件，提供统一的 tone/compact/source 状态契约。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可样式化、可测试的键位展示基元。
- 非目标：不承载业务状态机、不处理异步流程、不实现组件级动效编排。
- 风险边界：命名与语义契约必须与全库一致（布尔参数使用 `is_*`）。

## Architecture Layers

- `logic.rs`：归一化文本输入、派生 `KeyboardState`、组合 class。
- `view.rs`：Leptos 结构渲染与语义标记挂载。
- `styles.rs`：token-first 静态样式，基于稳定 `data-*` / class 选择器。
- `mod.rs`：最小稳定导出（`Keyboard`、`KeyboardTone`、`DEFAULT_ARIA_LABEL`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `tone` | `KeyboardTone` (`Default` / `Muted`) | `KeyboardTone::Default` |
| `is_compact` | `bool` | `false` |
| `aria_label` | `Option<String>` | `Some("Keyboard")`（空值会回退） |
| `class_name` | `Option<String>` | `None` |

说明：
- `is_compact` 是唯一布尔参数名，不提供 `compact` 兼容别名。
- `aria_label` 会先做 trim，空字符串按默认值 `Keyboard` 处理。

## Hello World（最小可用）

```rust
<Keyboard>"⌘K"</Keyboard>
```

## 常见用法

```rust
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>

<Keyboard
  is_compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>
```

## docs-app 展示区（Display）

- 页面入口：`apps/docs-app/src/pages/components/pages/display_extra.rs` 中 `keyboard()`
- 工作台区块：`Interactive Playground (展示 / Config / Code / CSS Test)`
- 对比区块：`Comparison Matrix (Tone / Compact / Source Markers)`
- 多场景对比覆盖：
  - `Default`
  - `Muted`
  - `Compact`
  - `Muted + Compact + Custom`（带 `aria_label` + `class_name`）

## docs-app Config 区（Settings）

Interactive Playground 的 `controls` 提供以下开关与选择：

- `Tone`（`default` / `muted`）
- `Key Text`（`⌘K` / `Ctrl+Shift+P` / `⌥⇧P`）
- `is_compact`（布尔）
- `Custom aria_label`（布尔）
- `Custom class_name`（布尔）

## docs-app Code 区（Code）

Interactive Playground 的 `code_signal` 会按当前配置实时生成 Copy-Paste Ready 代码片段，保证：

- 只输出非默认项（如 `tone=KeyboardTone::Muted`、`is_compact=true`）。
- 布尔参数命名固定为 `is_compact`，不使用兼容别名。

## docs-app CSS Test 区（CSS Test）

Interactive Playground 已接入 scoped CSS 测试面板：

- `test_css_source`: `ui_components::keyboard::styles::CSS`
- `test_source_path`: `crates/ui-components/src/keyboard/styles.rs`
- `test_config_signal`: 输出 `KeyboardActualConfig`（含 class 与 marker 期望）

可在测试面板中直接编辑 scoped CSS，并配合实际配置检查语义标记与样式契约是否一致。

## Semantics Contract

组件稳定暴露以下语义标记（用于样式与自动化断言）：

- `data-slot="keyboard"`
- `data-tone="default|muted"`
- `data-state="default|muted|compact"`
- `data-compact="true"`（仅 compact）
- `data-aria-source="default|custom"`
- `data-class-source="default|custom"`
- `data-custom-class="true"`（仅传入 class_name）

## Accessibility

- 使用原生 `<kbd>` 语义元素。
- 默认 `aria-label` 为 `Keyboard`，支持外部自定义。
- 不硬编码业务文案，显示文本由 `children` 提供。

## 测试与文档落点

- 语义测试：`crates/ui-components/tests/keyboard_semantics.rs`
- docs 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs` 中 `keyboard()`

## Source-first Copy-Paste Ready

- docs-app `Playground` 支持一键复制示例代码。
- 真实源码落点：
  - `crates/ui-components/src/keyboard/mod.rs`
  - `crates/ui-components/src/keyboard/logic.rs`
  - `crates/ui-components/src/keyboard/view.rs`
  - `crates/ui-components/src/keyboard/styles.rs`
