# ContextualHelp

`ContextualHelp` 是一个非模态帮助触发组件，组合 `Button + Popover`，用于展示轻量说明信息。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的帮助浮层入口。
- 非目标：不在组件层实现业务状态管理、主题系统或全局动效编排。
- 风险边界：交互契约变化优先在 `Popover` / `ui-headless` 层修复，不在 `view.rs` 追加补丁分支。

## Architecture Layers

- `logic.rs`：归一化文本/id/open 默认值优先级与状态来源标记（variant/placement/heading/footer/source）。
- `view.rs`：Leptos 结构渲染、`use_controllable_open_state_traced` 与 `use_presence` 挂载。
- `motion.rs`：`ContextualHelpMotion` 契约，内部委托 `PopoverMotion` 并做 sanitize。
- `styles.rs`：静态 CSS 契约，基于 `data-*` 与 `var(--ui-*)`。
- `mod.rs`：最小导出面（`ContextualHelp`、`ContextualHelpVariant`、`ContextualHelpMotion`）。

## Hello World（最小可用）

```rust
<ContextualHelp heading="Contextual help".to_string()>
  <div>"Explain a nearby control."</div>
</ContextualHelp>
```

- 默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。
- 组件内部管理 open 状态（非受控）。
- 先用起来：先走默认 API，确认语义与交互正常后再进入受控扩展。

## 常见用法

```rust
<ContextualHelp
  heading="Contextual help".to_string()
  footer=move || view! { "Popover-based" }
>
  <div class="docs-stack">
    <div>"Uses Button + Popover + spring motion."</div>
    <div class="ui-muted">"Works in Light/Dark/OLED via tokens."</div>
  </div>
</ContextualHelp>
```

## 再进阶（受控 + 语义定制）

```rust
let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<ContextualHelp
  variant=ContextualHelpVariant::Info
  open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  aria_label="More info".to_string()
  class_name="docs-contextual-help-custom".to_string()
>
  <div>"Controlled mode keeps parent state as the source of truth."</div>
</ContextualHelp>
```

- 受控模式使用 `open + on_open_change`；非受控模式使用 `default_open`，优先级在 `logic.rs` 统一归一（controlled 时忽略 `default_open`）。
- 布尔禁用态优先使用 `is_disabled`；`disabled` 作为兼容别名保留，内部按 `is_disabled > disabled` 归一化。
- 稳定标记包含 `data-state`、`data-variant`、`data-placement`、`data-open-mode`、`data-*-source`。

## docs-app 等价入口

- `apps/docs-app/src/pages/components/pages/overlays.rs` 的 `contextual_help()` 页面。
- Playground 路径：`Hello World (Default API)`、`Info Variant + Controlled`、`Workbench (Display + Config + Code + CSS Test)`、`State Matrix`、`Streaming/Snapshot Display`。

## Docs Playground（展示区）

### 展示 (Display)

- Workbench 预览区展示当前配置下的 `ContextualHelp` 触发与浮层渲染。
- 通过按钮可切换 open 状态，观察 controlled/uncontrolled 行为差异。

### config

- `Variant`：`help / info`。
- `Disabled`、`Controlled mode`、`Custom aria label`、`Custom class`。
- 配置变化实时反映到预览与语义标记。

### code

- `code` 面板输出当前配置对应的可复制示例代码（受控/非受控路径会随配置切换）。

### css test

- `css test` 面板绑定 `crates/ui-components/src/contextual_help/styles.rs`。
- 可在隔离作用域下直接修改样式并验证 `data-*` 状态选择器效果。

### 多场景对比显示

- `State Matrix` 同屏对比 `Help`、`Info`、`Disabled` 三种典型状态。
- 与 `Hello World (Default API)`、`Info Variant + Controlled` 组合，形成“默认 API -> 进阶受控”的学习路径。

## Source-first Copy-Paste Ready

- docs-app `Playground` 自带复制按钮并生成可运行片段。
- 真实源码落点：
  - `crates/ui-components/src/contextual_help/mod.rs`
  - `crates/ui-components/src/contextual_help/logic.rs`
  - `crates/ui-components/src/contextual_help/view.rs`
  - `crates/ui-components/src/contextual_help/styles.rs`
  - `crates/ui-components/src/contextual_help/motion.rs`
