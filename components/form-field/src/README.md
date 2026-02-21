# FormField

`FormField` 是单字段选择组件，统一装配 `switch/checkbox` 指示器、受控/非受控状态轴和稳定 `data-*` 语义标记。

## Hello World

默认路径（零门槛）：

```rust
<FormField label="Accept terms of service".to_string() />
```

## 常见用法

受控路径（外部 signal 作为单一事实来源）：

```rust
let (marketing, set_marketing) = signal(true);
let on_selected_change = Callback::new(move |next| set_marketing.set(next));

<FormField
  is_selected=Some(marketing.into())
  on_selected_change=Some(on_selected_change)
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
  indicator_placement=FormFieldIndicatorPlacement::Start
/>
```

状态矩阵常见分支（非受控 + disabled/invalid + tone/variant）：

```rust
<FormField
  default_selected=Some(true)
  label="Maintenance window alerts".to_string()
  indicator_variant=FormFieldIndicatorVariant::Checkbox
  indicator_placement=FormFieldIndicatorPlacement::End
  tone=FormFieldTone::Quiet
  is_disabled=true
  is_invalid=true
  error_message="Please accept terms to continue.".to_string()
/>
```

## 先用起来，再进阶

- 默认路径：`<FormField label=... />`，先完成基础可交互语义挂载。
- 进阶控制：按需启用 `is_selected + default_selected + on_selected_change`。
- 视觉扩展：再叠加 `indicator_variant/indicator_placement/tone/class_name`，不要一开始全开。

## 目标 / 非目标 / 风险边界

- 目标：稳定输出可访问、可测试、可检索的字段状态语义。
- 非目标：不承载应用业务 store、异步协议、组件级动效状态机。
- 风险边界：受控/非受控归一必须留在 `logic.rs`，禁止在 `view.rs` 临时分支修补。

## Architecture Layers

- `logic.rs`：输入归一、默认值和状态来源标记。
- `view.rs`：渲染结构并挂载 headless 语义。
- `styles.rs`：token-first 样式契约与稳定 `data-slot` 选择器。
- `mod.rs`：最小导出面（`FormField` + 公开枚举）。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms_groups_extra.rs` -> `form_field()`

