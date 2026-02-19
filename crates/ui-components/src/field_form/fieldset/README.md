# Fieldset

`Fieldset` 是表单分组组件，负责 legend/description/error/actions 的语义装配，并输出稳定 `data-*` 契约。

## 目标 / 非目标 / 风险边界

- 目标：可访问、可测试、可语义检索的表单分组容器。
- 非目标：不承担应用业务状态管理、异步协议、主题编排。
- 风险边界：`orientation/tone/invalid/source` 必须在 primitives/logic 层归一，禁止在 `view.rs` 临时分支修补。

## Architecture Layers

- `crates/ui-state-primitives/src/fieldset.rs`：`FieldsetOrientation` / `FieldsetTone` / `FieldsetState`。
- `logic.rs`：消费 primitives，组装 class 与 Agent Contract。
- `view.rs`：渲染 `<fieldset>`，挂载 `lang/dir` 与 `data-*`。
- `motion.rs`：`FieldsetMotion` 规范化与 CSS 变量注入。
- `styles.rs`：token-first 样式契约。
- `mod.rs`：最小导出面（`Fieldset`、`FieldsetOrientation`、`FieldsetTone`、`FieldsetMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `orientation` | `FieldsetOrientation` (`Vertical/Horizontal`) | `Vertical` |
| `tone` | `FieldsetTone` (`Default/Muted`) | `Default` |
| `is_required` / `required` | `Option<bool>` / `bool` | `None` / `false` |
| `is_disabled` / `disabled` | `Option<bool>` / `bool` | `None` / `false` |
| `is_invalid` / `invalid` | `Option<bool>` / `bool` | `None` / `false` |
| `legend` | `Option<String>` | `None` |
| `description` | `Option<String>` | `None` |
| `error_message` | `Option<String>` | `None` |
| `actions` | `Option<ViewFn>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `lang` / `dir` | `Option<String>` / `Option<A11yDirection>` | `None` |
| `motion` | `FieldsetMotion` | `FieldsetMotion::default()` |

## 展示（Display）

docs-app `fieldset()` 页面包含多场景对比：

- `Hello World`
- `Legend + Description`
- `Horizontal + Invalid + Actions`
- `Fieldset Workbench (Display + Config + Code + CSS Test)`
  - Baseline vs Configured 对比
  - Scenario compare：`Required vertical` vs `Invalid horizontal`

## config

Workbench 控制项：

- `Orientation`（vertical/horizontal）
- `Tone`（default/muted）
- `Locale`（en-US/zh-CN/ar）
- `Required/Disabled/Invalid/Description/Actions/Custom class/RTL direction`

## code

Workbench 会实时生成 copy-ready 代码。示例：

```rust
<Fieldset
  orientation=FieldsetOrientation::Horizontal
  tone=FieldsetTone::Muted
  is_required=true
  is_invalid=true
  error_message="Pick at least one channel".to_string()
  class_name="docs-fieldset-custom".to_string()
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary size=ui_components::ButtonSize::Sm>
      "Manage"
    </ui_components::Button>
  }
  legend="Notification channels".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>
```

## css test

- 面板默认加载：`crates/ui-components/src/field_form/fieldset/styles.rs`
- 支持 scoped CSS 热编辑，验证 `data-orientation/data-tone/data-state/data-*-source` 稳定性。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_extra.rs` -> `fieldset()`

