# CheckboxGroup

`CheckboxGroup` 为多选项提供字段级语义封装（`fieldset/legend`、描述与错误信息、`is_required/is_invalid/is_disabled` 状态）。

## Hello World（最小可用）

```rust
<CheckboxGroup id="fruits".to_string() label="Fruits".to_string()>
  <Checkbox checked=apple set_checked=set_apple>"Apple"</Checkbox>
  <Checkbox checked=banana set_checked=set_banana>"Banana"</Checkbox>
</CheckboxGroup>
```

默认路径只需要 `id + label + children`，不用先接线状态原语层也能直接使用。

## 常见用法

- 表单校验：通过 `is_required + is_invalid + description + error` 表达“至少选择一项”。
- 可访问增强：通过 `aria_describedby` 挂载外部提示文本。
- 只读/禁用展示：通过 `is_disabled` 锁定整组交互。

## 先用起来，再进阶

- 默认路径：`<CheckboxGroup id=... label=...>` 先跑通基础多选分组。
- 进阶控制：按需打开 `is_required/is_invalid/is_disabled`、`aria_describedby`、`description/error`、`motion/class_name`。
- 文档入口：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group`（含 Hello World、状态矩阵、Interactive Playground）。

## docs-app 入口

- 页面函数：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group()`
- 访问路由：`#/components/checkbox-group`

## Playground 展示区（Display / Config / Code / CSS Test）

- `Display`：实时渲染分组、选项、描述与错误提示。
- `Config`：切换 `is_required/is_invalid/is_disabled` 与文案显示。
- `Code`：输出当前配置对应的最小可复制示例。
- `CSS Test`：注入分组样式源码做契约校验。
