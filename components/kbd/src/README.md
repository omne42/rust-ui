# Kbd

`Kbd` 用于展示键盘按键提示（例如 `Ctrl + K`），提供稳定尺寸/状态契约与可测试语义标记。

## 快速开始（先用起来）

### Hello World（最小可用）

```rust
<Kbd keys="Ctrl".to_string()>"K"</Kbd>
```

### 常见用法

```rust
<Kbd size=KbdSize::Md keys="Ctrl".to_string()>"K"</Kbd>
<Kbd size=KbdSize::Sm>"Esc"</Kbd>
```

默认 API 只要记住四个输入：`size`、`keys`、`class_name`、`children`。

## 进阶用法（按需）

- 自定义样式来源：`class_name`（会同步暴露 `data-custom-class`）。
- 组合状态观察：`data-size`、`data-state`、`data-keys`、`data-custom-class`。
- docs-app 交互演练：`Workbench (Display + Config + Code + CSS Test)`。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `size` | `KbdSize` (`Sm` / `Md`) | `Md` |
| `keys` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |

## 参数默认值与归一化

- `size=None` -> `Md`（`logic.rs::normalize_size -> unwrap_or_default()`）。
- `keys/class_name`：空白字符串会被裁剪为 `None`（`logic.rs::normalize_optional_text`）。

## docs-app 入口（等价文档）

- `apps/docs-app/src/pages/components/pages/display.rs` -> `kbd()`
- `Hello World (Default API)`
- `State Matrix (Size + Keys + Label-only)`
- `Controlled vs Uncontrolled (N/A)`
- `Workbench (Display + Config + Code + CSS Test)`

## 架构与边界（进阶阅读）

- `logic.rs`：`KbdSize`、文本归一化、状态派生与 class 组装。
- `view.rs`：`<kbd>` 结构与 slot/state 标记挂载。
- `styles.rs`：token-first 静态样式。
- `mod.rs`：公开最小 API（`Kbd`、`KbdSize`）。
- 非目标：不实现交互状态机、异步流程、overlay 行为。

## Source-first

- `components/kbd/src/mod.rs`
- `components/kbd/src/logic.rs`
- `components/kbd/src/view.rs`
- `components/kbd/src/styles.rs`
