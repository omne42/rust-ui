# Collapsible

`Collapsible` 是一个基于 `ui-state-primitives`（状态归一化）+ `ui-headless`（可访问交互）+ `ui-motion`（动效驱动）组合出来的折叠面板组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的折叠交互能力。
- 非目标：不在组件层承载业务状态管理或应用级异步协议。
- 风险边界：状态来源与语义契约必须通过稳定 `data-*` / `aria-*` 输出，禁止在 `view.rs` 增加隐式补丁分支。

## Architecture Layers

- `logic.rs`：对 `ui_state_primitives::collapsible` 的归一化与类名装配导出。
- `view.rs`：结构渲染、headless 交互挂载（press/focus/hover）、语义状态标记输出。
- `motion.rs`：`CollapsibleMotion`（DisclosureMotion 别名）sanitize + attach 收口，映射组件语义到 disclosure/ui-motion contract。
- `styles.rs`：静态 token-first CSS 契约，包含状态/来源标记选择器。
- `mod.rs`：最小稳定导出（`Collapsible`、`CollapsibleMotion` 与 state primitives 类型）。

## API (Table)

### Collapsible Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required（空值会归一化） |
| `title` | `String` | required（空值会归一化） |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None`（未指定时按 primitive 默认） |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `is_disabled` | `Option<bool>` | `None`（布尔状态主命名） |
| `disabled` | `bool` | `false` |
| `motion` | `CollapsibleMotion` | `CollapsibleMotion::default()` |
| `aria_label` | `Option<String>` | `None`（回退到 title 语义） |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None`（继承外层文档语言） |
| `dir` | `Option<String>` | `None`（支持 `ltr` / `rtl`） |
| `children` | `Children` | required |

`disabled` 为兼容迁移别名；若同时传入 `is_disabled` 与 `disabled`，以 `is_disabled` 为准。

### Collapsible Events

| Event | Type | Default |
| --- | --- | --- |
| `on_open_change` | `Callback<bool>` | `None` |

## Hello World（最小可用）

```rust
<Collapsible id_base="docs-collapsible".to_string() title="Advanced options".to_string()>
  <div>"Panel content."</div>
</Collapsible>
```

## Semantics and Accessibility

- trigger 与 panel 通过 `aria-controls` / `aria-labelledby` / `id` 稳定关联。
- `is_disabled` 状态通过 `disabled`、`aria-disabled`、`data-disabled` 协同表达（`disabled` 仅作兼容别名输入）。
- 根与子节点输出状态来源标记：`data-state`、`data-open-mode`、`data-label-source`、`data-class-source`、`data-motion-source`、`data-custom-motion`。
- 支持 `lang` / `dir`（LTR/RTL）接入，locale 归一化与 disclosure a11y attrs 优先复用 `ui-headless` 共享工具。

## Motion and Fallback

- `CollapsibleMotion` 复用 Disclosure 动效契约；自定义参数会先 sanitize 再执行。
- wasm 路径使用 spring 驱动，non-wasm 路径安全降级，保证 SSR/tooling 可编译。

## Playground 展示区（展示 / Config / Code / CSS Test）

- 展示：docs-app 提供 `Hello World`（默认调用路径）以及 `Controlled Collapsible`、`Disabled + Custom Motion`、`State + Source Markers`、`Interactive Playground` 多区对比。
- Config：interactive 区提供 controlled/uncontrolled、default/open、is_disabled、label/class、motion source 切换，并输出 `CollapsibleActualConfig`。
- Code：每个 playground 均支持 `Show code`；interactive 区代码随配置变化。
- CSS Test：每个 playground 均支持 `Show test`；interactive 区绑定 `components/collapsible/src/styles.rs`，可局部改样式并回滚。
- 多场景对比：至少覆盖 controlled vs uncontrolled、default vs custom motion、enabled vs disabled、default label/class vs custom label/class。

## 文档入口

- docs-app: `/#/components/collapsible`
- 源码: `components/collapsible/src/`
