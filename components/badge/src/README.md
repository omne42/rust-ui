# Badge

`Badge` 是一个状态展示组件，组合了 `ui-state-primitives` + `ui-headless` 的语义契约，并输出稳定 `data-*` 标记。

## 先用起来（Quick Start）

零门槛最小示例（Hello World）：

```rust
<Badge>"New"</Badge>
```

默认路径只需要一个组件标签，不要求先理解分层架构。

## 常见用法

常见变体（对应 docs 的 `Variant Matrix`）：

```rust
<Badge variant=BadgeVariant::Default>"Default"</Badge>
<Badge variant=BadgeVariant::Accent>"Accent"</Badge>
<Badge variant=BadgeVariant::Danger>"Danger"</Badge>
<Badge variant=BadgeVariant::Outline>"Outline"</Badge>
```

常见扩展（自定义 class + 语言方向）：

```rust
<Badge
  variant=BadgeVariant::Outline
  class_name="docs-badge-custom".to_string()
  lang="ar".to_string()
  dir=ui_headless::A11yDirection::Rtl
>
  "جديد"
</Badge>
```

## 进阶（Workbench）

当默认 API 路径满足不了需求，再进入 docs-app 的工作台：

- `Badge Workbench (Display + Config + Code + CSS Test)`
- 可调 `variant / locale / custom class / rtl`
- 支持 Baseline vs Configured 对照、Scenario compare、copy-ready 代码与 CSS test

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可语义检索、可稳定测试的状态标签。
- 非目标：不承担业务状态机、异步流控、主题系统编排。
- 风险边界：variant/fill/class-source 必须在 primitives/logic 层归一，`view.rs` 只做装配。

## Architecture Layers

- `crates/ui-state-primitives/src/badge.rs`：`BadgeVariant`、`BadgeStateInput`、`BadgeState`。
- `logic.rs`：消费 primitives，补充 class 组装与 Agent Contract。
- `view.rs`：渲染 `<span data-slot="badge">`，挂载 `lang/dir` 与 `data-*`。
- `styles.rs`：token-first 静态样式，消费 `var(--ui-*)`。
- `mod.rs`：最小导出面（`Badge`、`BadgeVariant`、`BadgeMotion`）。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `variant` | `BadgeVariant` (`Default/Accent/Danger/Outline`) | `Default` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |

## 展示（Display）

docs-app 的 `badge()` 页面现在包含多场景对比：

- 基础：`Hello World`
- 状态矩阵：`Variant Matrix`（`Default/Accent/Danger/Outline`）
- 自定义来源：`Custom Class + Outline`
- 工作台对比：`Badge Workbench (Display + Config + Code + CSS Test)`
  - Baseline vs Configured 并排展示
  - Scenario compare 行内对比四种 variant

## config

Workbench 控制项：

- `Variant`（`default/accent/danger/outline`）
- `Locale`（`en-US/zh-CN/ar`）
- `Custom class`（切换 class-source）
- `RTL direction`（切换 `dir=rtl`）

## code

Workbench 会实时生成 copy-ready 代码（含导入自动补齐）。示例：

```rust
<Badge
  variant=BadgeVariant::Outline
  class_name="docs-badge-custom".to_string()
  lang="ar".to_string()
  dir=ui_headless::A11yDirection::Rtl
>
  "جديد"
</Badge>
```

## css test

- 面板内默认加载：`components/badge/src/styles.rs`
- 支持 scoped CSS 热编辑与回滚，验证 `data-variant/data-fill/data-class-source` 契约是否稳定。

## Source-first（Copy-Paste Ready）

复制代码默认依赖（由 docs-app Playground 自动补齐）：

```rust
use leptos::prelude::*;
use ui::*;
```

源码落点：

- 组件渲染：`components/badge/src/view.rs`
- 逻辑归一：`components/badge/src/logic.rs`
- 样式契约：`components/badge/src/styles.rs`

## docs-app 入口

- `apps/docs-app/src/pages/components/pages/display.rs` -> `badge()`
