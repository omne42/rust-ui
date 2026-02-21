# FlipCard

`FlipCard` 是前后双面 3D 卡片组件，支持点击/键盘翻转、hover 翻转与可配置 spring 动效。

## Hello World（最小可用）

```rust
use ui_components::FlipCard;

view! {
    <FlipCard
        front=move || view! { <div>"Front"</div> }
        back=move || view! { <div>"Back"</div> }
    />
}
```

## 新手路径（先用起来，再进阶）

- 第一步：先复制上面的 `Hello World（最小可用）`，只传 `front/back` 跑起来。
- 第二步：再看 docs-app 的状态矩阵与受控/非受控对照，按需加 `is_disabled`、`is_flip_on_hover`、`is_flipped`、`on_is_flipped_change`。
- 不需要先理解 `ui-state-primitives` / `ui-headless` 分层细节，默认 API 路径即可完成基础交互。

阅读顺序建议：先用起来，再进阶。

## docs-app 展示区（类似 button）

入口：`apps/docs-app/src/pages/components/pages/display_extra.rs::flip_card()`

- `Click + Keyboard Flip`
- `Interactive Playground (展示 / Config / Code / CSS Test)`
- `State + Source Markers`
- `Comparison Matrix (Default / Hover / Disabled / Dramatic Motion)`
- `Disabled`

## 常见用法（进阶）

### 受控翻转

```rust
let (is_flipped, set_is_flipped) = signal(false);

view! {
    <FlipCard
        is_flipped=Signal::derive(move || is_flipped.get())
        on_is_flipped_change=Callback::new(move |next| set_is_flipped.set(next))
        front=move || view! { <div>"Controlled front"</div> }
        back=move || view! { <div>"Controlled back"</div> }
    />
}
```

### Hover 翻转 + 禁用态

```rust
view! {
    <>
        <FlipCard
            is_flip_on_hover=true
            front=move || view! { <div>"Hover front"</div> }
            back=move || view! { <div>"Hover back"</div> }
        />
        <FlipCard
            is_disabled=true
            front=move || view! { <div>"Disabled front"</div> }
            back=move || view! { <div>"Disabled back"</div> }
        />
    </>
}
```

## 展示区

- 实时渲染当前 `FlipCard` 配置（front/back 内容 + 交互状态）。
- 默认支持鼠标点击与键盘 `Enter` / `Space` 翻转。

## Config 区

在 Interactive Playground 里提供可调配置：

- `motion preset`（default / gentle / dramatic）
- `default_is_flipped`
- `flip_mode`（Toggle / Hover）
- `is_disabled`
- `custom id`
- `custom class`

## Code 区

- Playground 会生成 Copy-Paste Ready 代码片段（含必要 imports）。
- 配置变更会同步反映在代码片段中，便于复现当前状态。

## CSS Test 区

- 使用 scoped css live-edit（不污染页面其他 playground）。
- 基于 `components/flip-card/src/styles.rs` 原始样式进行覆盖测试。
- 同面板展示 `Actual config`，便于验证 class/data-state 选择器效果。

## 多场景对比显示

`Comparison Matrix` 同屏对比四种场景：

- Default
- Hover flip
- Disabled
- Dramatic motion（更强 hover scale + tilt）

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `front` | `ViewFn` | required |
| `back` | `ViewFn` | required |
| `is_flipped` | `Option<Signal<bool>>` | `None` |
| `default_is_flipped` | `bool` | `false` |
| `on_is_flipped_change` | `Option<Callback<bool>>` | `None` |
| `is_disabled` | `bool` | `false` |
| `flip_mode` | `Option<FlipCardFlipMode>` (`Toggle`/`Hover`) | `None` (`Toggle`) |
| `is_flip_on_hover` | `bool` | `false` |
| `motion` | `FlipCardMotion` | `FlipCardMotion::default()` |
| `class_name` | `Option<String>` | `None` |
| `id` | `Option<String>` | `None`（自动生成） |

状态轴说明：

- 翻转状态支持受控/非受控成对：`is_flipped + on_is_flipped_change + default_is_flipped`。
- 受控模式下，外部 `is_flipped` 是单一事实来源；组件只通过 `on_is_flipped_change` 请求变更，不写本地状态。
- 离散状态优先类型化：推荐使用 `flip_mode`（`FlipCardFlipMode::Toggle | ::Hover`）表达互斥模式。
- 兼容迁移：旧命名 `default_flipped` / `disabled` / `flip_on_hover` 仍可用，优先读取新命名并建议迁移到 `default_is_flipped` / `is_disabled` / `flip_mode`。

## 语义与样式契约

- 根节点：`role="button"`、`aria-pressed`、`aria-disabled`
- 稳定标记：`data-slot`、`data-state`、`data-visible`、`data-flip-mode`、`data-motion-source`、`data-class-source`、`data-id-source`
- 面片可见性：front/back 输出 `data-visible` / `data-hidden`

## 代码结构

- `mod.rs`：导出边界
- `logic.rs`：状态归一 + source 标记
- `view.rs`：结构与交互挂载
- `motion.rs`：动效契约 + wasm/non-wasm 分支
- `styles.rs`：静态 CSS 契约

## Source-first

- `components/flip-card/src/mod.rs`
- `components/flip-card/src/logic.rs`
- `components/flip-card/src/view.rs`
- `components/flip-card/src/motion.rs`
- `components/flip-card/src/styles.rs`
