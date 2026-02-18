# Content

`Content` is a semantic section wrapper with token-first styling and centralized state/source markers.

## Goals / Non-goals / Boundaries

- Goal: provide a minimal, stable content region primitive with predictable tone/padding markers.
- Non-goal: no business state machine, no cross-component interaction contract, no custom motion engine.
- Boundary: all defaulting and source tracking stays in `logic.rs`; `view.rs` only mounts normalized output.

## Architecture Layers

- `logic.rs`: `ContentTone`, text normalization, aria fallback, derived state/source markers.
- `view.rs`: renders `<section>` and mounts `data-*` / `aria-*` markers.
- `styles.rs`: static CSS selectors keyed by stable markers.
- `mod.rs`: minimal public API (`Content`, `ContentTone`, `DEFAULT_ARIA_LABEL`).

## API

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `tone` | `ContentTone` (`Default` / `Muted`) | `Default` |
| `padded` | `bool` | `false` |
| `aria_label` | `Option<String>` | fallback to `DEFAULT_ARIA_LABEL` (`"Content"`) |
| `class_name` | `Option<String>` | `None` |

## State / Source Contract

`Content` exposes stable markers for styling, testing, and automation:

- `data-slot="content"`
- `data-tone` (`default` / `muted`)
- `data-state` (`default` / `muted` / `padded` / `muted-padded`)
- `data-padded`
- `data-aria-source` (`default` / `custom`)
- `data-custom-class`
- `data-class-source` (`default` / `custom`)

## Hello World

```rust
<Content>
  <p>"Primary body content"</p>
</Content>
```

## docs-app Playground 区块

- 展示区: 当前配置与 baseline 并排对比（tone/padded/source 差异可直观看到）。
- Config 区: 通过控制项切换 `tone` / `padded` / `aria_label` / `class_name`。
- Code 区: 动态生成当前配置对应的可复制代码片段。
- CSS Test 区: 加载 `crates/ui-components/src/content/styles.rs`，支持 scoped CSS 临时改写与回滚。

## 对比场景

| 场景 | 对比点 |
| --- | --- |
| Semantic Section + Tone | `default` vs `muted` tone |
| Padded + Custom Aria/Class | padding/source marker 差异 |
| Interactive Playground | current config vs baseline |

## Accessibility

- Root element is semantic `<section>`.
- `aria-label` is always present (custom input or normalized fallback).
- Marker contract keeps aria/class source explicit for regression tests.
