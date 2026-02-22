# Form

`Form` 是表单上下文容器，统一下发 `disabled/read_only/required/label layout` 语义。

## Hello World（默认路径）

```rust
<Form>
  <Input id="name".to_string() label="Name".to_string() value=name set_value=set_name />
</Form>
```

默认用法不需要手动接线 `ui-state-primitives` / `ui-headless` 状态机，也不需要传内部 `state` 对象。

## 常见用法（先用起来，再进阶）

```rust
// 先用默认路径（零门槛）
<Form>
  <Input id="profile-name".to_string() label="Name".to_string() value=name set_value=set_name />
</Form>

// 再按需开启进阶参数
<Form
  is_required=true
  is_disabled=false
  is_read_only=false
  label_position=FormLabelPosition::Left
  label_align=FormLabelAlign::End
>
  <Input id="profile-email".to_string() label="Email".to_string() value=email set_value=set_email />
</Form>
```

建议顺序：先确认默认路径可用，再逐个开启 `is_*` 与标签布局参数。

## Props

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `is_disabled` | `bool` | `false` |
| `is_read_only` | `bool` | `false` |
| `is_required` | `bool` | `false` |
| `label_position` | `FormLabelPosition` (`Top`/`Left`) | `Top` |
| `label_align` | `FormLabelAlign` (`Start`/`End`) | `Start` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<ui_headless::A11yDirection>` | `None` |
| `class_name` | `Option<String>` | `None` |

## docs-app 展示区（Display）

- 页面：`apps/docs-app/src/pages/components/pages/forms.rs` 的 `form()`
- 区块：`Hello World（默认路径）`
- 区块：`Interactive Playground (展示 / Config / Code / CSS Test)`
- 区块：`Comparison Matrix (Default / Required / Disabled / ReadOnly)`

## docs-app Config 区（Settings）

Interactive Playground 可调：

- `Label Position`（top/left）
- `Label Align`（start/end）
- `is_required`
- `is_disabled`
- `is_read_only`
- `Custom class_name`

## docs-app Code 区（Code）

`code_signal` 实时生成当前配置的 `<Form ...>` 代码片段：

- 仅输出非默认参数
- 布尔参数统一 `is_*`
- 标签布局参数与运行时一致

## docs-app CSS Test 区（CSS Test）

- `test_css_source`: `ui::form::styles::CSS`
- `test_source_path`: `components/form/src/styles.rs`
- `test_config_signal`: `FormActualConfig`

## 多场景对比

Comparison Matrix 覆盖：

- Default
- Required + Left/End
- Disabled
- ReadOnly + Custom Class
