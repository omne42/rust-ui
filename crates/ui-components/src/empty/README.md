# Empty

`Empty` 组件族（`Empty/EmptyHeader/EmptyMedia/EmptyTitle/EmptyDescription/EmptyContent`）用于空状态结构化渲染。

## API (Table)

| Component | Key Props | Default |
| --- | --- | --- |
| `Empty` | `class_name` | `None` |
| `EmptyHeader` | `class_name` | `None` |
| `EmptyMedia` | `variant`, `class_name` | `variant=Default` |
| `EmptyTitle` | `class_name` | `None` |
| `EmptyDescription` | `class_name` | `None` |
| `EmptyContent` | `class_name` | `None` |

## docs-app 展示区（Display）

- 页面：`apps/docs-app/src/pages/components/pages/display_extra_empty.rs` 的 `empty()`
- 区块：`Interactive Playground (展示 / Config / Code / CSS Test)`
- 区块：`Comparison Matrix (Header / Action / Source Markers)`

## docs-app Config 区（Settings）

Interactive Playground 可调：

- `Media Variant`（default/icon）
- `Show content action`
- `Custom root class`
- `Custom slot classes`

## docs-app Code 区（Code）

`code_signal` 会按当前配置生成可复制代码：

- 结构包含 `EmptyHeader/EmptyMedia/EmptyTitle/EmptyDescription`
- 按开关控制是否输出 `EmptyContent`
- 自定义 class 开启时会输出对应 slot class

## docs-app CSS Test 区（CSS Test）

- `test_css_source`: `ui_components::empty::styles::CSS`
- `test_source_path`: `crates/ui-components/src/empty/styles.rs`
- `test_config_signal`: `EmptyActualConfig`

## 多场景对比

Comparison Matrix 覆盖：

- Header + Icon
- Content Action
- State + Source Markers
