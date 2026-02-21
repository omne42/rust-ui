# ColorField

`ColorField` 是一个基于 `ui-state-primitives` + `ui-headless` 组合出来的颜色文本输入组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的颜色字符串输入能力。
- 非目标：不在组件层实现全局业务状态管理、主题系统或通用动效引擎。
- 风险边界：状态归一化与来源标记必须保持在 primitives/logic 层，禁止回流到 `view.rs` 拼状态机。

## Architecture Layers

- `logic.rs`：纯消费层，re-export `ui_state_primitives::color_field` 的归一化与状态派生能力。
- `view.rs`：Leptos 结构渲染、`use_controllable_state` 装配、i18n/locale 语义挂载。
- `styles.rs`：仅静态 CSS 契约，状态分支依赖稳定 `data-*` 标记。
- `mod.rs`：公开最小稳定 API（`ColorField`、`ColorFieldStateInput`、`ColorFieldState`）。

## API (Table)

### ColorField Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `label` | `Option<String>` | `"Color"` |
| `placeholder` | `Option<String>` | `"#RRGGBB"` |
| `is_disabled` | `Option<bool>` | `None` (`false`) |
| `disabled` | `Option<bool>` | `None` (legacy alias; fallback to `is_disabled`) |
| `value` | `Option<Signal<Option<String>>>` | `None` |
| `default_value` | `Option<String>` | `None` |
| `on_value_change` | `Option<Callback<Option<String>>>` | `None` |
| `is_preview_visible` | `Option<bool>` | `None` (`true`) |
| `show_preview` | `Option<bool>` | `None` (legacy alias; fallback to `is_preview_visible`) |
| `aria_label` | `Option<String>` | `${label} value` / `"Color value"` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

### ColorField Events

| Event | Type | Default |
| --- | --- | --- |
| `on_value_change` | `Callback<Option<String>>` | `None` |

受控/非受控规则：
- 受控：传入 `value + on_value_change`，外部值为单一事实来源。
- 非受控：传入 `default_value`，后续状态由内部 controllable-state 管理。

命名兼容策略：
- 推荐使用 `is_disabled`、`is_preview_visible`。
- 旧字段 `disabled`、`show_preview` 仍保留为兼容别名，解析优先级为 `is_*` 优先，其次旧字段。

## Hello World（最小可用）

```rust
<ColorField id_base="demo-color".to_string() />
```

## Controlled 示例

```rust
let (value, set_value) = signal(Some("#4f46e5".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| set_value.set(next));

<ColorField
  id_base="demo-color-controlled".to_string()
  value=value.into()
  on_value_change=on_value_change
/>
```

## Semantics and Accessibility

- 根节点使用 `role="group"`，并通过 `aria-labelledby` 绑定标签。
- 接入 `ui_headless::locale_attrs`，支持 `lang/dir`（LTR/RTL）上下文透传。
- 清除按钮文案与 `aria-label` 来源于 `ui_headless::CommonStrings::clear_aria_label`，不硬编码业务文本。
- 暴露稳定语义标记（用于测试/自动化）：
  - `data-state`: `disabled | empty | valid | invalid`（由 `ColorFieldVisualState` enum 映射）
  - `data-label-source`: `default | custom`
  - `data-placeholder-source`: `default | custom`
  - `data-aria-source`: `default | custom`
  - `data-class-source`: `default | custom`
  - `data-disabled` / `data-has-value` / `data-valid` / `data-invalid` / `data-has-preview`

## Styling Contract

- 样式全部位于 `styles.rs`，通过 `css.rs` 聚合注入。
- 状态样式仅依赖稳定 `data-*` / `aria-*` 标记，不依赖脆弱 DOM 结构。
- 颜色预览值经过 sanitize 后才传入 `ColorSwatch`，防止不受信任值直接进入样式通道。

## Test and Verification

- 语义回归：`crates/ui-components/tests/color_field_semantics.rs`
- primitives 单测：`crates/ui-state-primitives/src/color_field.rs`
- docs 验收页：`apps/docs-app/src/pages/components/pages/forms_color.rs`
- E2E 契约：`e2e/tests/docs_app_color_field_contract.spec.mjs`
- E2E 门禁脚本：`scripts/check-ui-components-e2e-color-field.sh`
