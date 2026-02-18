# InlineAlert

`InlineAlert` 是一个基于 `ui-motion` + `ui-theme` 组合出来的紧凑提示组件，支持语义色调（tone）、填充样式（fill）、可选标题/描述与可插槽内容。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可组合的行内提示展示能力。
- 非目标：不在组件层承载业务状态机、异步协议或全局主题编排。
- 风险边界：语义与状态来源必须稳定暴露在 `data-*` / `aria-*`，禁止在 `view.rs` 增加隐式分支补丁。

## Architecture Layers

- `logic.rs`：`InlineAlertTone` / `InlineAlertFill` 与视图状态归一化（标题/描述/图标显隐）。
- `view.rs`：Leptos 结构渲染、A11y 语义挂载、slot 组织与 `data-*` 状态输出。
- `motion.rs`：`InlineAlertMotion` 契约、参数 sanitize、wasm spring 驱动与 non-wasm no-op。
- `styles.rs`：仅静态 CSS 契约，视觉值全部通过 `var(--ui-*)` 与 motion CSS 变量表达。
- `mod.rs`：最小稳定导出（`InlineAlert`、`InlineAlertTone`、`InlineAlertFill`、`InlineAlertMotion`）。

## API (Table)

### InlineAlert Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `tone` | `InlineAlertTone` (`Neutral` / `Info` / `Positive` / `Notice` / `Negative`) | `Neutral` |
| `fill` | `InlineAlertFill` (`Border` / `Subtle` / `Bold`) | `Border` |
| `title` | `Option<String>` | `None` |
| `description` | `Option<String>` | `None` |
| `hide_icon` | `bool` | `false` |
| `icon_label` | `Option<String>` | `None`（回退到 tone 默认标签） |
| `start_content` | `Option<ViewFn>` | `None` |
| `end_content` | `Option<ViewFn>` | `None` |
| `motion` | `InlineAlertMotion` | `InlineAlertMotion::default()` |
| `class_name` | `Option<String>` | `None` |

### InlineAlert Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 展示组件，不暴露交互事件回调 | `-` |

## Hello World（最小可用）

```rust
<InlineAlert tone=InlineAlertTone::Info title="Info".to_string()>
  "Message"
</InlineAlert>
```

- 默认路径不需要用户手动接线底层状态或动效驱动。
- 复杂需求（自定义 motion、首尾插槽、视觉样式扩展）再按需开启扩展参数。

## Semantics and Accessibility

- 根节点语义：`role` 与 `aria-live` 按 `tone` 自动映射（`Negative` 为 `alert/assertive`，其他为 `status/polite`）。
- 图标可访问名称：`icon_label` 可覆盖；未提供时按 `tone` 使用默认标签（`Info/Success/Warning/Error`）。
- 稳定语义标记：
  - 根：`data-slot="inline-alert"`、`data-motion-source`、`data-custom-motion`
  - 子节点：`inline-alert-icon`、`inline-alert-start`、`inline-alert-body`、`inline-alert-title`、`inline-alert-description`、`inline-alert-content`、`inline-alert-end`

## Motion and Fallback

- 默认 spring 合同：`stiffness=260.0`、`damping=18.0`、`mass=1.0`。
- 自定义 motion 会在运行前 sanitize，非法值回退到默认合同。
- wasm 下使用 `ui_motion::spring::SpringAnimator` 驱动 CSS 变量（opacity/translate/scale）。
- `prefers-reduced-motion` 或 non-wasm 场景自动降级为无动画路径，保证 SSR/tooling 可编译与行为可预测。

## Agent Contract / 流式降级

- 通过稳定 `data-*` 字段暴露状态来源，便于测试与 Agent 自动化消费。
- `InlineAlert` 非正文流式渲染组件，按 `Streaming Optional` 处理，默认 `snapshot` 渲染语义。

## Playground 展示区（展示 / Config / Code / CSS Test）

- 展示：docs-app 提供基础预览、tone+fill 矩阵、slots+custom class、interactive 四组对比。
- Config：interactive 区提供 tone/fill、图标显隐、标题/描述、slot、custom class 开关，并输出 `InlineAlertActualConfig`。
- Code：每个 playground 均支持 `Show code`，interactive 区代码会随当前配置实时变化。
- CSS Test：每个 playground 均支持 `Show test`；interactive 区绑定 `crates/ui-components/src/inline_alert/styles.rs`，可局部编辑并回滚样式。
- 多场景对比：至少覆盖 `Info/Positive/Notice/Negative` 与 `Border/Subtle/Bold` 组合，以及带/不带 icon、slot、custom class 的差异。

## 文档入口

- docs-app: `/#/components/inline-alert`
- 源码: `crates/ui-components/src/inline_alert/`
