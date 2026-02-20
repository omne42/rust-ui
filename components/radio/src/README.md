# Radio / RadioGroup

`RadioGroup` 用于一组互斥选择；`Radio` 是单体按钮（语义分组优先用 `RadioGroup`）。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
let (selected, set_selected) = signal(Some(0_usize));
<RadioGroup
  id_base="size".to_string()
  options=vec!["S".to_string(), "M".to_string(), "L".to_string()]
  selected_index=selected
  set_selected_index=set_selected
/>
```

- 默认路径不需要手动接线 `ui-state-primitives` / `ui-headless`。
- 先传 `options + selected_index + set_selected_index` 即可运行。

## 常见用法

```rust
<RadioGroup
  id_base="billing".to_string()
  options=vec!["Monthly".to_string(), "Quarterly".to_string(), "Yearly".to_string()]
  orientation=RadioGroupOrientation::Horizontal
  disabled_indices=vec![1]
  label="Billing cycle".to_string()
  selected_index=selected
  set_selected_index=set_selected
/>
```

```rust
let (checked, set_checked) = signal(false);
<Radio
  id="standalone".to_string()
  label="Standalone".to_string()
  is_checked=Signal::derive(move || checked.get())
  on_checked_change=Callback::new(move |next| set_checked.set(next))
/>

<Radio
  id="default-on".to_string()
  label="Uncontrolled default".to_string()
  default_checked=true
/>
```

## 再进阶（高级控制）

```rust
<RadioGroup
  id_base="plan".to_string()
  options=vec!["Free".to_string(), "Pro".to_string(), "Enterprise".to_string()]
  aria_labelledby="plan-label".to_string()
  is_disabled=false
  disabled_indices=vec![2]
  class_name="docs-radio-group-custom".to_string()
  motion=RadioMotion::default()
  selected_index=selected
  set_selected_index=set_selected
/>
```

- 默认 API 在前，`aria_* / class_name / motion` 等高级参数按需开启。
- 语义与来源标记通过稳定 `data-* / aria-*` 暴露，便于测试与自动化。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/forms.rs`：
- `radio_group()` 页面：`Hello World（默认路径）`、`Interactive Matrix（方向/禁用/状态）`
- `radio()` 页面：`Hello World（默认路径）`、`状态矩阵（受控 + disabled）`

## Source-first Copy-Paste Ready

- docs-app `Playground` 自带复制按钮，并自动补齐导入（`apps/docs-app/src/playground.rs`）。
- 默认导入为：
  - `use leptos::prelude::*;`
  - `use ui_components::*;`
- 真实源码落点：
  - `components/radio/src/mod.rs`
  - `components/radio/src/logic.rs`
  - `components/radio/src/view.rs`
  - `components/radio/src/styles.rs`
  - `components/radio/src/motion.rs`
