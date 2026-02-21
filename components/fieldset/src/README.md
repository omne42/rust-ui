# Fieldset

`Fieldset` 是表单分组组件，负责 legend/description/error/actions 的语义装配，并输出稳定 `data-*` 契约。

## Hello World

默认路径（零门槛）：

```rust
<Fieldset legend="Channels".to_string()>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>
```

## 常见用法

带说明 + 必填语义：

```rust
<Fieldset
  legend="Notification channels".to_string()
  description="Pick every channel you want to receive release updates from.".to_string()
  is_required=true
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>
```

受控/非受控（按需开启）：

```rust
// 非受控：只在初始化时消费 default_is_invalid。
<Fieldset
  legend="Uncontrolled snapshot".to_string()
  default_is_invalid=true
  error_message="Uncontrolled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>

// 受控：外部 signal 是单一事实来源。
<Fieldset
  legend="Controlled snapshot".to_string()
  is_invalid=Signal::derive(move || controlled_invalid.get())
  on_is_invalid_change=Callback::new(move |next| set_controlled_invalid.set(next))
  error_message="Controlled snapshot: pick at least one channel".to_string()
>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>
```

## 先用起来，再进阶

- 默认路径：`<Fieldset legend=...>...</Fieldset>`，先把分组语义挂上去。
- 进阶控制：按需启用 `is_* + default_is_* + on_is_*_change`（如 `is_invalid/default_is_invalid/on_is_invalid_change`）。
- 复杂配置：再叠加 `orientation/tone/actions/lang/dir/motion`，不要一开始就全开。

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
| `is_required` | `Option<bool>` | `None` |
| `default_is_required` | `Option<bool>` | `None` (`false`) |
| `on_is_required_change` | `Option<Callback<bool>>` | `None` |
| `is_disabled` | `Option<bool>` | `None` |
| `default_is_disabled` | `Option<bool>` | `None` (`false`) |
| `on_is_disabled_change` | `Option<Callback<bool>>` | `None` |
| `is_invalid` | `Option<bool>` | `None` |
| `default_is_invalid` | `Option<bool>` | `None` (`false`) |
| `on_is_invalid_change` | `Option<Callback<bool>>` | `None` |
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

## Source-first

- 复制入口：docs-app `Fieldset` 页面任一 Playground 的 `Show code` + copy（由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports）。
- 组件源码：`components/fieldset/src/{mod,logic,view,styles,motion}.rs`
- 运行时实现：`crates/ui-components/src/field_form/fieldset/{mod,logic,view,styles,motion}.rs`
- package feature：`component-fieldset`（可选叠加 `inject-css`）
- 依赖基线（`Cargo.toml`）：

```toml
ui-components = { default-features = false, features = ["component-fieldset", "inject-css"] }
```

## 命名兼容与迁移

- 统一命名已收敛为 `is_*`：`is_required`、`is_disabled`、`is_invalid`。
- 旧别名已移除：`required`、`disabled`、`invalid`。
- 迁移映射：`required -> is_required`、`disabled -> is_disabled`、`invalid -> is_invalid`（语义不变，仅命名收敛）。
- 受控/非受控协议：`is_* + default_is_* + on_is_*_change`。

## css test

- 面板默认加载：`crates/ui-components/src/field_form/fieldset/styles.rs`
- 支持 scoped CSS 热编辑，验证 `data-orientation/data-tone/data-state/data-*-source` 稳定性。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_extra.rs` -> `fieldset()`
