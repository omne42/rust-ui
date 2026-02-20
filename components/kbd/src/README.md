# Kbd

`Kbd` 用于展示键盘按键提示（例如 `Ctrl + K`），提供稳定尺寸/状态契约与可测试标记。

## 目标 / 非目标 / 风险边界

- 目标：统一 `size` 与 `keys` 显示语义，稳定暴露 `data-*` 合同。
- 非目标：不实现交互状态机、异步流程、overlay 行为。
- 风险边界：文本归一化与 class 拼装应留在 `logic.rs`，避免在视图层散落规则。

## Architecture Layers

- `logic.rs`：`KbdSize` 枚举、可选文本归一化、状态派生与 class 组装。
- `view.rs`：`<kbd>` 结构与 slot/state 标记挂载。
- `styles.rs`：token-first 静态样式。
- `mod.rs`：公开最小 API（`Kbd`、`KbdSize`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `size` | `KbdSize` (`Sm` / `Md`) | `Md` |
| `keys` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |

## Hello World（最小可用）

```rust
<Kbd keys="Ctrl".to_string()>"K"</Kbd>
```

## Semantics and Accessibility

- 根节点输出：`data-slot="kbd"`、`data-size`、`data-state`。
- 来源标记：`data-keys`、`data-custom-class`。
- slot 结构稳定：`data-slot="kbd-keys"` 与 `data-slot="kbd-label"`。

## Motion and Fallback

- N/A：`Kbd` 无组件级动效契约。

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/display.rs`
- 页面：`kbd()`
- Playground：`Size + Keys Matrix`、`Custom Class + Label Only`、`Workbench (Display + Config + Code + CSS Test)`

## Playground 展示区（Display / Config / Code / CSS Test）

- 展示（Display）：实时预览 `size/keys/label/custom class` 的组合输出。
- 配置（Config）：Workbench 控件切换 `sm/md`、keys 文本与 class source，输出 `KbdActualConfig`。
- 代码（Code）：按当前组合生成可复制代码，确保示例与展示一致。
- CSS Test：加载 `kbd/styles.rs`，在 scoped 环境中直接验证样式调整。

## 多场景对比展示

- `Size + Keys Matrix`：`Md/Sm` + 多组合键并排对比。
- `Custom Class + Label Only`：带 keys 与纯 label 两种状态对比。
- `Workbench`：同画布快速切换 `with-keys/label-only`、size 与 custom class 组合。

## Source-first

- `components/kbd/src/mod.rs`
- `components/kbd/src/logic.rs`
- `components/kbd/src/view.rs`
- `components/kbd/src/styles.rs`
