# Breadcrumbs

`Breadcrumbs` 是一个基于 `ui-state-primitives` 组合出的导航路径组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、可观测状态来源（`data-*`）的面包屑导航。
- 非目标：不在组件层处理路由状态机、业务权限、远程数据获取。
- 风险边界：路径状态归一化与来源标记必须集中在 `logic.rs`/primitives，不在 `view.rs` 追加分支补丁。

## Architecture Layers

- `crates/ui-state-primitives/src/breadcrumbs.rs`：根 `aria`/class 归一化、链接可用性判定、`has_links/is_empty` 派生。
- `logic.rs`：组件层薄装配，映射 `BreadcrumbItem -> BreadcrumbsItemInput`。
- `view.rs`：Leptos 结构渲染，输出 `nav/ol/li` 与 `aria-current` 等语义。
- `styles.rs`：仅静态 CSS 契约，依赖 `var(--ui-*)`。
- `mod.rs`：最小稳定导出（`Breadcrumbs`、`Breadcrumb`、`BreadcrumbItem`）。

## API (Table)

### Breadcrumbs Props

| Prop | Type | Default |
| --- | --- | --- |
| `items` | `Vec<BreadcrumbItem>` | required |
| `aria_label` | `Option<String>` | `"Breadcrumb"` |
| `class_name` | `Option<String>` | `None` |

### BreadcrumbItem

| Field | Type | Meaning |
| --- | --- | --- |
| `label` | `String` | 渲染文本 |
| `href` | `Option<String>` | 非最后一项可选链接；最后一项会被归一为当前页语义 |

## Hello World（最小可用）

```rust
let items = vec![
  BreadcrumbItem {
    label: "Home".to_string(),
    href: Some("#/docs/welcome".to_string()),
  },
  BreadcrumbItem {
    label: "Components".to_string(),
    href: Some("#/components".to_string()),
  },
  BreadcrumbItem {
    label: "Breadcrumbs".to_string(),
    href: None,
  },
];

<Breadcrumbs items=items />
```

## Semantics and Accessibility

- 根节点使用 `nav` 并挂载 `aria-label`。
- 当前页项输出 `aria-current="page"`。
- 输出稳定语义标记：`data-empty`、`data-has-items`、`data-has-links`、`data-has-current-page`、`data-count`。
- 输出来源标记：`data-aria-source`、`data-class-source`。

## Controlled / Uncontrolled Contract

- `Breadcrumbs` 为展示型组件，不暴露受控状态回调。
- 所有输入在渲染前完成归一化：
  - 空或空白 `aria_label` 回退默认值。
  - 最后一项强制使用当前页语义（不渲染链接）。

## Motion and Fallback

- 组件无 `motion.rs`，无动画状态机。
- SSR / non-wasm 行为与 wasm 一致，不依赖浏览器动画能力。

## Agent Contract / 流式降级

- 通过 `data-*` 暴露机器可读状态与来源，便于测试和 Agent 自动化消费。
- 组件属于 `Streaming Optional`，默认 `snapshot` 渲染路径。

## docs-app Workbench（Display / Config / Code / CSS Test）

- Display：展示当前配置，并提供 `With links` / `Label-only` / `Empty` 多场景对比。
- Config：支持切换 `links`、`empty`、`long trail`、`custom aria`、`custom class`。
- Code：实时生成与当前配置一致的可复制代码。
- CSS Test：加载 `crates/ui-components/src/breadcrumbs/styles.rs` 做样式契约验证。
