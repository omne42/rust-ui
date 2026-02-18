# Iconset

`Iconset` is a registry wrapper on top of `Icon`: it resolves `iconset:name` references, maps glyphs, and emits stable source markers.

## Goals / Non-goals / Risk Boundary

- Goal: provide typed namespace/icon resolution with predictable fallback behavior.
- Non-goal: no global icon registry ownership or asset loading pipeline in component layer.
- Risk boundary: namespace parsing and label fallback order must stay centralized in `logic.rs`.

## Architecture Layers

- `logic.rs`: parses icon references, resolves namespace/glyph/label, derives state/source attrs.
- `view.rs`: renders resolved wrapper markers and delegates rendering to `Icon`.
- `styles.rs`: static selectors for resolved/fallback/decorative/source states.
- `mod.rs`: exports `Iconset`, `IconsetGlyph`, `IconsetState` contracts and aliases size/tone from `Icon`.

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `icon` | `String` | required |
| `iconset` | `Option<String>` | `None` |
| `glyphs` | `Vec<IconsetGlyph>` | `[]` |
| `size` | `IconsetSize` | `IconsetSize::default()` |
| `tone` | `IconsetTone` | `IconsetTone::default()` |
| `disabled` | `bool` | `false` |
| `decorative` | `bool` | `true` |
| `aria_label` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |

`IconsetGlyph`:
- `IconsetGlyph::new(name, glyph)`
- optional `.with_aria_label(label)`

## Hello World

```rust
<Iconset icon="workflow:check".to_string() glyphs=glyphs />
```

## Docs Playground（展示区）

### 展示 (Display)

- workbench 预览命中注册表与 fallback 两条渲染路径。
- 直接观察 icon/tone/size/decorative/disabled 对 UI 的影响。

### config

- `Icon`：`workflow:check / workflow:alert / ui:unknown`。
- `Size`：`sm / md / lg`。
- `Tone`：`default / muted / accent / danger`。
- `Disabled`、`Decorative`、`Custom aria label`、`Custom class`。

### code

- `code` 面板生成当前配置的 `Iconset` 使用代码。
- 包含 `glyphs`、可选 `aria_label` 与 `class_name` 参数。

### css test

- `css test` 面板绑定 `crates/ui-components/src/iconset/styles.rs`。
- 支持在隔离作用域验证 source marker 对应样式选择器。

### 多场景对比显示

- `State Comparison` 同屏对比 registry 命中、danger tone、fallback unknown 三种场景。
- `State + Source Markers` 场景用于验证 source attribution 标记。

## Semantics and Accessibility

- Emits machine-readable markers for source attribution: `data-icon-source`, `data-iconset-source`, `data-label-source`, `data-size-source`, `data-tone-source`, `data-class-source`.
- Decorative mode suppresses accessible label.
- Non-decorative mode uses fallback chain: custom aria label -> registry label -> normalized icon name.

## Motion and Fallback

- No component-local motion contract.
- Fallback glyph is deterministic (`FALLBACK_GLYPH`) when registry resolution misses.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/display_extra_iconset.rs::iconset()`
- Source: `crates/ui-components/src/iconset/{mod,logic,view,styles}.rs`
- Package mode feature: `component-iconset` (requires `component-icon`; optional `inject-css`)
