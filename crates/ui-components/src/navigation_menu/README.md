# NavigationMenu

`NavigationMenu` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion`（经 `active_highlight` 复用）组合出来的横向导航菜单组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可受控/非受控、可测试的横向导航与 roving focus 交互。
- 非目标：不在组件层实现业务路由状态管理、全局主题系统或全局动效编排。
- 风险边界：跨层能力漂移时优先在对应层修复，不在 `view.rs` 叠加补丁逻辑。

## Architecture Layers

- `logic.rs`：props 归一化、item 解析、受控/非受控状态派生、状态/来源标记计算。
- `view.rs`：Leptos 结构渲染，键盘/指针焦点交互挂载，语义 `data-*` 输出。
- `motion.rs`：`NavigationMenuMotion`（`ActiveHighlightMotion`）参数清洗与安全回退。
- `styles.rs`：仅静态 CSS 契约，状态样式通过 `data-*` 和 `var(--ui-*)` 驱动。
- `mod.rs`：公开最小稳定 API（`NavigationMenu`、`NavigationMenuItem`、`NavigationMenuMotion`、默认常量）。

## API (Table)

### NavigationMenu Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | 必填；空值会归一化为 `DEFAULT_ID_BASE` |
| `items` | `Vec<NavigationMenuItem>` | required |
| `selected_id` | `Option<Signal<Option<String>>>` | `None` |
| `default_selected_id` | `Option<String>` | `None` |
| `on_selected_id_change` | `Option<Callback<Option<String>>>` | `None` |
| `activate_on_focus` | `bool` | `true` |
| `motion` | `NavigationMenuMotion` | `NavigationMenuMotion::default()` |
| `aria_label` | `Option<String>` | `DEFAULT_ARIA_LABEL` |
| `class_name` | `Option<String>` | `None` |

### NavigationMenuItem

| API | Type | Default |
| --- | --- | --- |
| `NavigationMenuItem::new(id, label, href)` | `impl Into<String>` x3 | `disabled = false` |
| `disabled(bool)` | builder-style | `false` |

导出常量：
- `DEFAULT_ID_BASE`
- `DEFAULT_ARIA_LABEL`
- `DEFAULT_ACTIVATE_ON_FOCUS`

## Hello World（最小可用）

```rust
let (last_selected, set_last_selected) = signal("none".to_string());

<NavigationMenu
  id_base="docs-navigation-menu".to_string()
  items=vec![
    NavigationMenuItem::new("overview", "Overview", "/overview"),
    NavigationMenuItem::new("components", "Components", "/components"),
    NavigationMenuItem::new("patterns", "Patterns", "/patterns"),
  ]
  default_selected_id="components".to_string()
  on_selected_id_change=Callback::new(move |next: Option<String>| {
    set_last_selected.set(next.unwrap_or_else(|| "none".to_string()));
  })
/>
```

## Controlled + Manual Activation

```rust
let (selected, set_selected) = signal(Some("docs".to_string()));

<NavigationMenu
  id_base="docs-navigation-menu-controlled".to_string()
  items=vec![
    NavigationMenuItem::new("docs", "Docs", "/docs"),
    NavigationMenuItem::new("api", "API", "/api"),
    NavigationMenuItem::new("guides", "Guides", "/guides"),
  ]
  selected_id=Signal::derive(move || selected.get())
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
/>
```

## Semantics and Accessibility

- 根节点是 `<nav role="navigation">`，并支持 `aria-label` 定制。
- 选中项输出 `aria-current="page"`，禁用项输出 `aria-disabled="true"`。
- 使用 roving tabindex（`0/-1`）与键盘模型：`ArrowLeft` / `ArrowRight` / `Home` / `End` / `Enter` / `Space`。
- 稳定语义标记包括：
  - 状态标记：`data-state`、`data-items`、`data-selection`、`data-focus`、`data-selection-mode`
  - 来源标记：`data-id-source`、`data-aria-label-source`、`data-selected-id-source`、`data-selected-id-change-source`、`data-motion-source`

## Motion and Fallback

- 高亮动画复用 `ActiveHighlightMotion`，通过 `motion` 参数支持 spring 定制。
- `motion.rs` 会对非法 spring 参数做清洗，自动回退默认值，避免异常值污染运行时。
- 非 wasm 路径可安全编译，SSR 场景不依赖浏览器焦点 API。

## DX / Docs

- `apps/docs-app` 已提供四组 playground：
  - `Default + Roving Focus + Selection`
  - `Controlled + Manual Activation`
  - `State + Source Markers`
  - `Workbench (Display + Config + Code + CSS Test)`
- 推荐在验收时直接检查上述 playground 的 `data-*` 标记是否与契约一致。

## Docs Playground（展示 / Config / Code / CSS Test）

- 展示：基线导航与配置导航并排对比，支持多状态切换。
- Config：可调受控/非受控、focus 激活模式、禁用项、class/aria/motion 来源。
- Code：copy-ready 代码片段随配置同步更新。
- CSS Test：提供 scoped CSS 编辑器、source 路径回溯与 `ActualConfig` 输出。

## 对比场景

- `Default + Roving Focus + Selection`
- `Controlled + Manual Activation`
- `State + Source Markers`
- `Workbench (Display + Config + Code + CSS Test)`

## Agent Contract / 流式降级

- 组件输出稳定机器可读状态来源标记（`data-*`），便于自动化与 Agent 消费。
- 该组件不是正文流式阅读面，按 `Streaming Optional` 执行；默认 `snapshot` 渲染路径即可满足契约。
