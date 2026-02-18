# NativeSelect

`NativeSelect` 是对原生 `<select>` 的语义化封装，提供受控/非受控选择与稳定状态标记。

## 目标 / 非目标 / 风险边界

- 目标：提供可测试、可控、可访问的原生选择控件封装。
- 非目标：不做自绘 listbox 弹层和异步选项加载协议。
- 风险边界：选项归一化和选中索引清洗只放在 `logic.rs`。

## Architecture Layers

- `logic.rs`：选项归一化、索引清洗、根状态派生。
- `view.rs`：渲染原生 `<select>` 与语义标记。
- `styles.rs`：静态 token-first CSS。
- `mod.rs`：最小导出面（`NativeSelect`、`NativeSelectSize`、`NativeSelectOption`）。

## API

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required |
| `options` | `Vec<NativeSelectOption>` | required |
| `selected_index` | `Option<Signal<Option<usize>>>` | `None` |
| `default_selected_index` | `Option<usize>` | `None` |
| `on_selected_index_change` | `Option<Callback<Option<usize>>>` | `None` |
| `disabled` | `bool` | `false` |
| `required` | `bool` | `false` |
| `invalid` | `bool` | `false` |
| `size` | `NativeSelectSize` (`Sm` / `Md` / `Lg`) | `Md` |
| `name` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | fallback to `DEFAULT_ARIA_LABEL` |
| `placeholder` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |

## Hello World

```rust
use ui_components::{NativeSelect, NativeSelectOption};

view! {
    <NativeSelect
        id_base="docs-native-select".to_string()
        options=vec![
            NativeSelectOption::new("system", "System"),
            NativeSelectOption::new("manual", "Manual"),
        ]
    />
}
```

## docs-app Playground（展示区 / Config 区 / Code 区 / CSS Test 区）

对应页面：`apps/docs-app/src/pages/components/pages/forms_native.rs` 的 `native_select()`

- 展示区：`Primary` 当前状态 + `对比矩阵`（required/invalid、disabled 等组合）。
- Config 区：切换 size、selected 模式、required、invalid、disabled、placeholder、custom class、compare matrix。
- Code 区：输出当前配置对应的 `NativeSelect` 代码片段。
- CSS Test 区：展示 `crates/ui-components/src/native_select/styles.rs` 的 `CSS` 常量，并输出当前配置快照。

## 多场景对比（对比矩阵）

- 场景 A：受控选择（None/System/Manual）。
- 场景 B：`required + invalid + size=Lg`。
- 场景 C：`disabled + all options disabled + size=Sm`。

## 语义契约

根节点标记：

- `data-slot="native-select"`
- `data-state`
- `data-size`
- `data-option-count`
- `data-selected-index`
- `data-selected-value`
- `data-has-selection`
- `data-has-disabled-options`
- `data-has-enabled-options`
- `data-disabled-option-count`
- `data-aria-source`
- `data-class-source`

子 slot 标记：

- `data-slot="native-select-control"`
- `data-slot="native-select-indicator"`

## Docs and Feature

- docs-app entry: `apps/docs-app/src/pages/components/pages/forms_native.rs::native_select()`
- source: `crates/ui-components/src/native_select/{mod,logic,view,styles}.rs`
- feature: `component-native_select`（可选 `inject-css`）
