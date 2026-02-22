# MenuTrigger

`MenuTrigger` is a button-driven menu surface that composes `Button + Popover + Menu` with controlled/uncontrolled open-state support and stable contract markers.

## Goals / Non-goals / Boundaries

- Goal: provide a reusable trigger-to-menu interaction with keyboard opening, close strategy, and source/state observability.
- Non-goal: no business-specific item loading/state ownership; no view-layer ad-hoc state machine.
- Boundary: open-state/control normalization and trigger semantics stay in `logic.rs`; `view.rs` only mounts resolved behavior.

## Architecture Layers

- `logic.rs`: id normalization, disabled-index normalization, aria fallback, open-key strategy, state derivation.
- `view.rs`: wires `Button`, `Popover`, `Menu`, and presence-based mount lifecycle.
- `motion.rs`: `MenuTriggerMotion` contract that delegates to `popover::motion::sanitize_motion`.
- `styles.rs`: static selectors for placement/disabled/persistent/custom-motion markers.
- `mod.rs`: minimal exports (`MenuTrigger`, `MenuTriggerMotion`).

## Feature Dependencies

`component-menu_trigger` requires:

- `component-button`
- `component-menu`
- `component-popover`

This keeps minimal-feature compilation consistent with the actual render graph.

## API

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required (normalized; blank -> `"menu-trigger"`) |
| `items` | `Vec<String>` | required |
| `on_action` | `Callback<usize>` | required |
| `children` | `Children` | required |
| `disabled` | `bool` | `false` |
| `disabled_indices` | `Vec<usize>` | `[]` (normalized + deduped) |
| `item_kinds` | `Vec<MenuItemKind>` | `[]` |
| `close_on_action` | `bool` | `true` |
| `placement` | `PopoverPlacement` | `BottomStart` |
| `open` | `Option<Signal<bool>>` | `None` |
| `default_open` | `Option<bool>` | `None` |
| `on_open_change` | `Option<Callback<bool>>` | `None` |
| `motion` | `MenuTriggerMotion` | `MenuTriggerMotion::default()` |
| `aria_label` | `Option<String>` | fallback to `"Open menu"` |
| `class_name` | `Option<String>` | `None` |

Controlled/uncontrolled pair:

- controlled: `open + on_open_change`
- uncontrolled: `default_open`

## Motion Contract

- `MenuTriggerMotion { popover: PopoverMotion }`
- sanitized through `menu_trigger::motion::sanitize_motion`
- popover mount/unmount is coordinated with `use_presence` for exit-safe rendering

## State / Source Contract

Root exposes stable markers:

- `data-slot="menu-trigger"`
- `data-state` (`open` / `closed` / `disabled`)
- `data-open`, `data-closed`, `data-disabled`, `data-enabled`
- `data-empty`, `data-has-items`
- `data-placement`
- `data-controlled`, `data-uncontrolled`
- `data-close-on-action`, `data-keep-open-on-action`
- `data-custom-label`, `data-has-disabled-items`, `data-has-item-kinds`
- `data-motion-source`, `data-custom-motion`

## Hello World

```rust
<MenuTrigger
  id_base="demo-menu".to_string()
  items=vec!["Edit".to_string(), "Delete".to_string()]
  on_action=Callback::new(|_| {})
>
  "Open menu"
</MenuTrigger>
```

## docs-app Playground 区块

- 展示区: current 与 baseline 并排（open/placement/disabled/close strategy 对比）。
- Config 区: 切换 `close_on_action` / `disabled` / `disabled_indices` / placement / aria/class source。
- Code 区: 动态生成当前配置的可复制 `MenuTrigger` 代码。
- CSS Test 区: 加载 `crates/ui/src/menu/trigger/styles.rs`，用于 scoped CSS 合同验证。

## 对比场景

| 场景 | 对比点 |
| --- | --- |
| Default | 标准触发器 + action 回调 |
| Controlled + persistent open | 受控 open 与关闭策略 |
| Disabled + Empty | 空列表与禁用触发器 |
| Interactive Playground | current config vs baseline |

## Accessibility and Keyboard

- Trigger button wires `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls`.
- ArrowDown opens menu with first-item focus intent; ArrowUp opens with last-item focus intent.
- Menu is labelled by trigger id for stable assistive-tech traversal.
