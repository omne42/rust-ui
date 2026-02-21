# Dialog

`Dialog` 是基于 `Overlay` 组合的弹层面板组件，提供标题/描述/正文/页脚与稳定语义标记。

## Hello World

默认路径：不需要先理解 primitives/headless 分层，先用起来。

```rust
<Dialog id_base="docs-dialog-hello".to_string() title="Hello dialog".to_string() default_open=Some(true)>
    <div>"Hello dialog body"</div>
</Dialog>
```

## 常见用法

- 基础开关：通过 `is_open/open` + `on_open_change` 走受控。
- 非受控初始化：使用 `default_open` 一次性初始化。
- 常见视觉配置：`size`、`is_close_button_visible`、`show_close_button`、`close_label`。

## 先用起来，再进阶

- 默认路径：`Hello World -> Dialog`（先跑通）
- 状态与来源：`State Matrix -> State + Source Markers`
- 行为对照：`Controlled vs Uncontrolled`
- 进阶调参：`Interactive Playground -> Scenario Comparison`

## 展示区（Display）

docs-app 页面：`/#/components/dialog`

当前提供七组 Playground：

1. `Hello World`：默认路径最小可用示例。
2. `Dialog`：基础打开/关闭流程。
3. `State Matrix`：受控/非受控 + size + close-button 可见性矩阵。
4. `State + Source Markers`：状态来源标记（`data-*-source`）校验。
5. `Controlled vs Uncontrolled`：默认值与受控路径差异对照。
6. `Interactive Playground`：展示 / Config / Code / CSS Test 一体工作台。
7. `Scenario Comparison`：多状态对比（默认 / 紧凑标题态 / 自定义动效）。

## Config 区

`Interactive Playground` 支持以下配置：

- `size`（sm/md/lg）
- `with_description`
- `show_close_button`
- `custom_motion`
- `custom_class`

## Code 区

Playground 的 `code_signal` 会按当前配置生成可复制代码。

最小示例：

```rust
<Dialog id_base="docs-dialog-hello".to_string() title="Hello dialog".to_string() default_open=Some(true)>
    <div>"Hello dialog body"</div>
</Dialog>
```

## CSS Test 区

- 样式来源：`components/dialog/src/styles.rs`
- Playground `Show test` 面板支持 scoped CSS 快速实验
- `Actual config` 面板显示运行时配置，便于和语义标记一起做回归核对

## 多种情况对比显示

`Scenario Comparison` 固定对比三种场景：

1. `Default`：默认大小 + 描述 + 关闭按钮
2. `Compact`：`size=Sm` + 无描述 + 无关闭图标
3. `Custom motion`：`size=Lg` + 自定义 overlay motion

## 语义与契约

- 可访问性：`role="dialog"` + `aria-modal="true"`，并通过 `aria-labelledby`/`aria-describedby` 绑定标题与描述。
- 稳定标记：`data-slot`、`data-state`、`data-size`、`data-*-source`。
- E2E 优先使用语义选择器，不依赖 DOM 层级和临时 class。

## 实现落点

- `logic.rs`：状态归一化与来源标记派生
- `view.rs`：结构渲染与语义挂载
- `motion.rs`：对接 overlay motion 合同并做 sanitize
- `styles.rs`：token-first 静态样式
- `mod.rs`：公共导出面
