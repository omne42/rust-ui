# Accordion

`Accordion` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion` 组合出来的折叠面板组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的折叠面板交互基元。
- 非目标：不在组件层实现业务状态管理、主题系统或全局动效编排。
- 风险边界：跨层抽象（core/headless/theme/motion）一旦漂移，优先在对应层修复，不在 `view.rs` 追加补丁逻辑。

## Architecture Layers

- `logic.rs`：纯状态归一化与派生（`AccordionSelectionMode`、open 集合切换、根状态派生）。
- `protocol.rs`：版本化组件协议（Serde 可序列化 spec + 解析归一化）。
- `view.rs`：Leptos 结构渲染与 headless hooks 挂载（press/focus/hover/roving）。
- `motion.rs`：`AccordionMotion` 契约与 wasm 动效驱动，含 SSR no-op 与 reduced-motion 降级。
- `styles.rs`：仅静态 CSS 契约，样式由 `var(--ui-*)` 驱动。
- `mod.rs`：公开最小稳定 API（`Accordion`、`AccordionItem`、`AccordionSelectionMode`、`AccordionVariant`、`AccordionMotion`）。

## AI Context Projection

- `Component.toml`：机器可读索引，声明 Inputs / Outputs / Slots / Capabilities / Dependencies。
- `accordion.rbi`：仅签名接口投影（public types/functions），不包含实现细节。
- Agent 默认先读索引层，再按需读取 `logic/view/styles/motion`。

## API (Table)

### Accordion Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `id_base` | `Option<String>` | `None`（由 `UiRoot` 注入的 `IdProvider` 自动生成） |
| `selection_mode` | `AccordionSelectionMode` (`Single` / `Multiple`) | `Multiple` |
| `variant` | `AccordionVariant` (`Light` / `Shadow` / `Bordered` / `Splitted`) | `Light` |
| `disallow_empty_selection` | `bool` | `false` |
| `is_disabled` | `bool` | `false` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `motion` | `AccordionMotion` | `AccordionMotion::default()` |
| `class_name` | `Option<String>` | `None` |

### Accordion Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 根组件不直接暴露 open 事件，item 级回调汇总为整体状态 | `-` |

### AccordionItem Props

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `label` | `String` | required |
| `key` | `Option<usize>` | `None` |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `bool` | `false` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `is_disabled` | `bool` | `false` |

`key` 规则：
- 必须是数字（`usize`）。
- 省略时按出现顺序自动分配 `0..n`。
- 若显式 `key` 重复，组件会自动重排为不冲突的数字键，保证状态映射稳定。

`open_set` helper：
- 对外导出 `open_set([0, 2]) -> BTreeSet<usize>`，用于减少 `BTreeSet::from([..])` 的类型噪音。
- 推荐在 docs/playground 和应用代码中优先使用该 helper，提高可读性与 AI 可解析性。

### AccordionItem Events

| Event | Type | Default |
| --- | --- | --- |
| `on_open_change` | `Callback<bool>` | `None` |

## Protocol Schema (Versioned)

- Schema name: `ui.accordion.component-spec`
- Current version: `1`
- Public types:
  - `AccordionComponentSpec`
  - `AccordionComponentItemSpec`
  - `AccordionComponentSchemaVersion`
  - `AccordionComponentSpecError`
  - `ResolvedAccordionComponentSpec`

`AccordionComponentSpec::resolve()` performs:
- key normalization (`assign_item_keys`)
- label normalization (`resolve_item_label`)
- open set normalization (`normalize_default_open_for_items`)
- schema identity validation (`schema_name`)

This provides a typed, versioned data contract for SDUI/agent pipelines without requiring DOM parsing.

## Hello World（最小可用）

```rust
<Accordion>
  <AccordionItem label="First">"Panel 1"</AccordionItem>
  <AccordionItem label="Second">"Panel 2"</AccordionItem>
</Accordion>
```

- 默认路径不需要用户手动接线 `ui-state-primitives` / `ui-headless`。
- 复杂需求（受控模式、禁用索引、自定义动效）再按需开启扩展参数。

## Semantics and Accessibility

- 触发器与面板通过 `id` + `aria-controls` / `aria-labelledby` 绑定。
- 面板使用 `role="region"`，触发器暴露 `aria-expanded`。
- 使用 roving tabindex 支持方向键焦点导航。
- 暴露稳定 `data-*` 状态标记用于样式与测试契约。

## Motion and Fallback

- wasm 下使用 `ui-motion::spring::SpringAnimator`。
- 非 wasm 环境采用 no-op，保证 SSR 编译通过。
- 检测 `prefers-reduced-motion: reduce` 时禁用弹簧过渡，直接应用最终状态。

## WASM 调试（开发特性）

- 调试能力通过 feature 隔离：`accordion-wasm-debug`（默认关闭）。
- 仅在 `wasm32 + debug_assertions` 生效，生产包体不包含调试 UI 与事件记录。
- 启用后组件会渲染 `data-slot="accordion-debug"` 入口，展示事件序列（source/time/before/after）并支持按条目 replay。

## DX / Workbench

- docs-app 提供 `Workbench (Isolated Canvas + Optional Persist)` 演练入口。
- 支持实时调节 CSS 变量（如圆角、hover 强度），走样式快速反馈路径。
- 支持可选状态保留（localStorage），减少反复打开/关闭交互的重做成本。

## Agent Contract / 流式降级

- 根节点输出 schema 化语义字段：`data-ui-schema`、`data-ui-schema-version`、`data-ui-intent`、`data-ui-action`、`data-ui-state`、`data-ui-source`。
- Accordion 只负责结果流展示语义，不承载结构流/状态流协议本身。
- 只有在 `AiSpace` 上下文内，组件才输出流式相关语义字段：
  - `data-ui-stream-support="unsupported"`
  - `data-ui-stream-fallback="full-snapshot"`
  - `data-ui-output-status`（`draft` / `verified` / `submittable`）
