# Field

`Field` 是表单字段容器组件，用于统一组织 label/control/description/error，并暴露稳定语义契约。

## 快速开始（先用起来）

```rust
<Field label="Email".to_string() is_required=true>
  <input type="email" />
</Field>
```

常见用法建议顺序：
- 先用默认 API（`label + children`）完成基础渲染。
- 再按需开启 `is_required/is_disabled/is_invalid`。
- 最后再接入 `orientation/tone/motion/class_name` 等进阶能力。

## 目标 / 非目标 / 风险边界

- 目标：集中管理字段级状态（`is_required/is_disabled/is_invalid`）与文案来源标记（aria/error/class source）。
- 非目标：不承载受控 value 状态机，不直接处理业务异步提交流程。
- 风险边界：不要在 `view.rs` 拼业务状态分支；所有归一化应通过 `logic.rs` 完成。

## Architecture Layers

- `logic.rs`：消费 `ui-state-primitives::field` / `field_group`，并集中完成默认值/优先级归一（`resolve_content`）。
- `view.rs`：结构渲染（label/control/messages）与 `ui-headless` 输出的 `aria/data-*` 契约挂载。
- `motion.rs`：`FieldMotion` 契约与 CSS 变量 attach（`--ui-field-motion-duration`）。
- `styles.rs`：token-first 静态 CSS。
- `mod.rs`：公开 API（`Field`、`FieldOrientation`、`FieldTone`、`FieldMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `orientation` | `FieldOrientation` (`Vertical` / `Horizontal`) | `Vertical` |
| `tone` | `FieldTone` (`Default` / `Muted`) | `Default` |
| `is_required` | `Option<bool>` | `None`（回落到 `required` 别名，再回落 `false`） |
| `required` | `Option<bool>` | `None`（兼容别名） |
| `is_disabled` | `Option<bool>` | `None`（回落到 `disabled` 别名，再回落 `false`） |
| `disabled` | `Option<bool>` | `None`（兼容别名） |
| `is_invalid` | `Option<bool>` | `None`（回落到 `invalid` 别名，再回落 `false`） |
| `invalid` | `Option<bool>` | `None`（兼容别名） |
| `label` | `Option<String>` | `None` |
| `description` | `Option<String>` | `None` |
| `error_message` | `Option<String>` | `None`（`invalid=true` 时回落到默认错误文案） |
| `motion` | `FieldMotion` | `FieldMotion::default()` |
| `aria_label` | `Option<String>` | `None`（回落 `DEFAULT_ARIA_LABEL`） |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` (`Ltr` / `Rtl`) | `None` |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
<Field label="Email".to_string() is_required=true>
  <input type="email" />
</Field>
```

## Controlled / Uncontrolled

- N/A-by-design：`Field/FieldGroup` 不管理 `value/open/checked/selected` 一类本地状态轴。
- 组件只消费外部语义输入并映射为 `aria-*`/`data-*` 标记，不提供 `default_*` 或 `on_*_change` 状态机 API。

## 命名迁移（兼容策略）

- 新命名：布尔状态统一使用 `is_*`（`is_required` / `is_disabled` / `is_invalid`）。
- 兼容别名：保留 `required` / `disabled` / `invalid` 作为旧名输入，避免现有调用立即破坏。
- 优先级：当新旧命名同时传入时，始终以 `is_*` 为准；旧名仅作为回退路径。

## Semantics and Accessibility

- 根节点暴露：`aria-label`、`aria-disabled`、`aria-invalid`。
- 状态标记：`data-state`、`data-message-kind`、`data-required`、`data-disabled`、`data-invalid`。
- 来源标记：`data-aria-source`、`data-error-source`、`data-class-source`。
- 错误消息节点使用 `role="alert"`。

## Motion and Fallback

- `FieldMotion` 默认值来自 `ui-theme`（`default_text_field_motion_tokens`），组件不硬编码默认时长。
- `attach_motion` 仅挂载 CSS 变量（`--ui-field-motion-duration`），并通过 `ui_motion::web::prefers_reduced_motion` 在非 wasm/减弱动效场景降级。
- 样式侧消费 token 变量（`--ui-text-field-motion-duration/easing`）并保留 `@media (prefers-reduced-motion: reduce)` 最小过渡兜底。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_extra.rs`
- 页面：`field()`
- Playground：`Required + Description`、`Horizontal + Invalid + Custom Class`、`Workbench (Display + Config + Code + CSS Test)`

## Playground 展示区（Display / Config / Code / CSS Test）

- 展示（Display）：实时观察 `orientation/tone/required/invalid/disabled` 组合行为。
- 配置（Config）：Workbench 控件驱动状态与 motion，并输出 `FieldActualConfig`。
- 代码（Code）：根据当前配置生成等价示例代码，便于粘贴复现。
- CSS Test：加载 `field/styles.rs` 样式并支持 scoped CSS 调参测试。

## 多场景对比展示

- `Required + Description`：默认纵向布局下的必填提示与描述信息。
- `Horizontal + Invalid + Custom Class`：横向 + 校验失败 + 自定义类路径对比。
- `Workbench`：在同一画布连续对比 `orientation/tone/invalid/disabled/motion/class` 多状态。

## Source-first

- `crates/ui/src/field_form/field/mod.rs`
- `crates/ui/src/field_form/field/logic.rs`
- `crates/ui/src/field_form/field/view.rs`
- `crates/ui/src/field_form/field/motion.rs`
- `crates/ui/src/field_form/field/styles.rs`
