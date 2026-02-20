# FileTrigger

`FileTrigger` is a button-backed file input primitive: it forwards trigger press to an invisible `<input type="file">`.

## Goals / Non-goals / Risk Boundary

- Goal: provide accessible file picking with stable callback payload and motion contract reuse.
- Non-goal: no upload transport, validation pipeline, or app-specific file policy logic.
- Risk boundary: browser/file-input quirks stay encapsulated in `view.rs` and `logic.rs`.

## Architecture Layers

- `logic.rs`: derives state/source markers and collects selected files into typed `FileTriggerFile`.
- `view.rs`: renders hidden input + trigger button, handles press/change bridging.
- `motion.rs`: `FileTriggerMotion` contract, sanitized via button motion sanitizer.
- `styles.rs`: static state selectors (`disabled`, `custom-motion`, etc.).
- `mod.rs`: exports `FileTrigger`, `FileTriggerFile`, `FileTriggerMotion`.

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `id` | `Option<String>` | `None` |
| `disabled` | `bool` | `false` |
| `multiple` | `bool` | `false` |
| `accept` | `Option<String>` | `None` |
| `accept_directory` | `bool` | `false` |
| `capture` | `Option<String>` | `None` |
| `motion` | `FileTriggerMotion` | `FileTriggerMotion::default()` |
| `on_files` | `Option<Callback<Vec<FileTriggerFile>>>` | `None` |
| `children` | `Children` | required |

`FileTriggerFile` payload:
- `name: String`
- `size: u64`
- `mime: String`

## Hello World

```rust
<FileTrigger on_files=on_files>"Pick files"</FileTrigger>
```

## Docs Playground（展示区）

### 展示 (Display)

- workbench 展示 `FileTrigger` 实时文件选择行为和回调结果列表。
- 预览禁用态、多选态、自定义动效态。

### config

- `Accept`：`any / images / documents`。
- `Multiple`、`Disabled`、`Custom motion` 开关。

### code

- `code` 面板根据当前配置生成最小可复制片段。
- 动效开启时展示 `FileTriggerMotion` 结构体配置。

### css test

- `css test` 面板绑定 `components/file-trigger/src/styles.rs`。
- 可验证 `ui-file-trigger--disabled` 与 `ui-file-trigger--custom-motion` 的样式契约。

### 多场景对比显示

- `State Comparison` 同屏对比 default / disabled / custom motion 三种状态。

## Semantics and Accessibility

- Hidden input is removed from tab order (`tabindex="-1"`) and accessibility tree (`aria-hidden="true"`).
- Root marker contract includes `data-state`, `data-disabled`, `data-enabled`, `data-motion-source`, `data-custom-motion`.
- Input value is cleared before click so selecting the same file again still emits `change`.

## Motion and Fallback

- Reuses `ButtonMotion` through `FileTriggerMotion { trigger }`.
- Motion values are sanitized before runtime usage.
- Non-wasm path remains compile-safe and deterministic.

## Source-first / Copy-Paste Ready

- Docs entry: `apps/docs-app/src/pages/components/pages/files.rs::file_trigger()`
- Source: `components/file-trigger/src/{mod,logic,view,motion,styles}.rs`
- Package mode feature: `component-file_trigger` (depends on `component-button` for trigger rendering; optional `inject-css`)
