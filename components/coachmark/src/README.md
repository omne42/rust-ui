# Coachmark

`Coachmark` 是基于 `ContextualHelp` 组合的引导提示组件，负责把步骤、CTA、资产位与语义标记装配成可测试契约。

## Hello World（最小可用）

```rust
<Coachmark title="Welcome".to_string() default_open=true>
  <div>"Tour copy"</div>
</Coachmark>
```

## 先用起来，再进阶

- 默认路径（先用起来）：优先使用 `title + default_open + children`，不需要先接线 `open/on_open_change`。
- 常见扩展：按需补 `current_step/total_steps`、`primary_cta/secondary_cta`、`asset_variant`。
- 进阶控制：仅在父级需要接管状态时启用 `open + on_open_change`（Controlled）。

## 常见用法

- 默认引导：`Help` 变体 + 步骤 + CTA + 内置资产。
- 信息提示：`Info` 变体 + 外链图片资产。
- 禁用状态：保留语义标记，便于 A11y 与自动化验证。

### Config（配置区）

可直接在 docs-app playground 的 settings 面板切换：

| Config | Values | 作用 |
| --- | --- | --- |
| `variant` | `Help` / `Info` | 指定语义意图与视觉分支 |
| `open` + `on_open_change` | controlled bool | 外部控制开合状态 |
| `is_disabled` | `bool` | 禁用交互并切换 `data-state`（推荐命名） |
| `disabled` | `bool` | 兼容别名（迁移到 `is_disabled`） |
| `current_step` + `total_steps` | `Option<usize>` | 控制步骤文案显隐 |
| `primary_cta` / `secondary_cta` | `Option<String>` | 单/双按钮状态矩阵 |
| `asset_variant` / `asset_src` | `Option<...>` | 内置资产或图片资产来源 |
| `class_name` | `Option<String>` | 自定义类来源标记 |

### Controlled Example（高级入口）

```rust
let (open, set_open) = signal(false);

<Coachmark
  title="Shortcuts".to_string()
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  primary_cta="Got it".to_string()
/>
```

## CSS Test（样式测试区）

- 在 docs-app 的 `Show test` 面板可直接编辑 scoped CSS。
- 默认注入源：`components/coachmark/src/styles.rs` 的 `CSS` 常量。
- 推荐只用 `:scope` + 稳定状态标记做验证：
  - `data-state`
  - `data-open-mode`
  - `data-asset-source`
  - `data-class-source`

## 多场景对比（Comparison Matrix）

- `Help + Variant Asset + Dual CTA`
- `Info + Image Asset + Single CTA`
- `Disabled + No Step Counter`
- `Controlled Open + Custom Class Source`

这些组合覆盖了展示层最常见的差异轴：语义意图、开合来源、CTA 组合、资产来源与样式来源。
