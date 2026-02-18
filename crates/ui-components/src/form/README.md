# Form

`Form` 是表单上下文容器，统一下发 `disabled/read_only/required/label layout` 语义。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `is_disabled` | `bool` | `false` |
| `is_read_only` | `bool` | `false` |
| `is_required` | `bool` | `false` |
| `label_position` | `FormLabelPosition` (`Top`/`Left`) | `Top` |
| `label_align` | `FormLabelAlign` (`Start`/`End`) | `Start` |
| `class_name` | `Option<String>` | `None` |

## docs-app 展示区（Display）

- 页面：`apps/docs-app/src/pages/components/pages/forms.rs` 的 `form()`
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

- `test_css_source`: `ui_components::form::styles::CSS`
- `test_source_path`: `crates/ui-components/src/form/styles.rs`
- `test_config_signal`: `FormActualConfig`

## 多场景对比

Comparison Matrix 覆盖：

- Default
- Required + Left/End
- Disabled
- ReadOnly + Custom Class
