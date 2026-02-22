# InputOtp

`InputOtp` 是一个 baseline 风格的一次性验证码输入组件：一个真实隐藏 `<input>` + 多个可视化 slot。

## 展示区（Display）

docs-app 页面：`/#/components/input-otp`

当前提供三组 Playground：

1. `OTP`：最小可用示例。
2. `Interactive Playground`：可调配置工作台。
3. `State Comparison`：多状态对比（默认 / disabled / invalid）。

## Config 区

`Interactive Playground` 支持以下配置：

- `length`（4/6/8）
- `disabled`
- `required`
- `invalid`
- `show_description`
- `show_error`
- `custom_class`
- `custom_aria_label`

## Code 区

Playground 的 `code_signal` 输出当前配置对应的可复制代码，默认自动补齐 `use leptos::prelude::*;` 与 `use ui::*;`。

最小可用示例：

```rust
use leptos::prelude::*;
use ui::InputOtp;

let (value, set_value) = signal(String::new());

view! {
    <InputOtp
        id_base="otp".to_string()
        label="One-time code".to_string()
        value=value
        set_value=set_value
        length=6
    />
}
```

## CSS Test 区

- 样式来源：`components/text-input/src/input_otp/styles.rs`
- 在 Playground `Show test` 面板中可直接编辑 scoped CSS
- `Actual config` 会显示运行时配置快照，便于契约回归核对

## 多种情况对比显示

`State Comparison` 固定对比三种状态：

1. 默认输入：常规输入流程
2. Disabled（预填值）：禁用态可见但不可编辑
3. Invalid + error：错误态样式与错误文案联动

## 语义与契约

- 输入契约：`autocomplete="one-time-code"`、`inputmode="numeric"`、`pattern="[0-9]*"`、`maxlength=length`
- 可访问性：slot 区域 `aria-hidden="true"`，读屏聚焦真实 input
- 稳定选择器：
  - root: `data-slot="input-otp"`
  - input: `data-slot="input-otp-input"`
  - slot: `data-slot="input-otp-slot"`
  - caret: `data-slot="input-otp-caret"`

## 实现落点

- `logic.rs`：纯 OTP 归一化与编辑 helper
- `view.rs`：结构渲染 + headless 挂载
- `styles.rs`：静态 CSS 契约（token-first）
- `i18n.rs`：默认文案（`InputOtpStrings`）
- `mod.rs`：最小导出面
