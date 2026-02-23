# Carousel

`Carousel` is a slide navigator with controllable/uncontrolled selection, orientation-aware keyboard behavior, and semantic source markers.

## Goals / Non-goals / Risk Boundary

- Goal: provide a stable carousel contract with predictable selection, focus, and marker outputs.
- Non-goal: do not embed app business state or cross-page orchestration in the component.
- Risk boundary: selection invariants belong to primitives + logic normalization, not ad-hoc conditionals in `view.rs`.

## Architecture Layers

- `logic.rs`: normalize ids/text, resolve item contracts, sanitize indices, derive source/state markers.
- `view.rs`: render viewport/slides/controls/indicators, wire keyboard and click behavior, mount controlled/uncontrolled state bridge.
- `motion.rs`: sanitize `CarouselMotion` (reused active-highlight spring contract).
- `styles.rs`: static CSS using `data-*` contracts.
- `mod.rs`: stable exports (`Carousel`, `CarouselMotion`, `CarouselItem`, `CarouselOrientation`).

## API (Table)

### Carousel Props

| Prop | Type | Default |
| --- | --- | --- |
| `id_base` | `String` | required (blank falls back to `"carousel"`) |
| `items` | `Vec<CarouselItem>` | required |
| `selected_index` | `Option<Signal<Option<usize>>>` | `None` |
| `default_selected_index` | `Option<usize>` | `None` |
| `on_selected_index_change` | `Option<Callback<Option<usize>>>` | `None` |
| `orientation` | `CarouselOrientation` (`Horizontal` / `Vertical`) | `Horizontal` |
| `is_loop_navigation` | `bool` | `true` |
| `motion` | `CarouselMotion` | `CarouselMotion::default()` |
| `aria_label` | `Option<String>` | `"Carousel"` |
| `controls_aria_label` | `Option<String>` | i18n `CarouselStrings.controls_aria_label` |
| `indicators_aria_label` | `Option<String>` | i18n `CarouselStrings.indicators_aria_label` |
| `previous_label` | `Option<String>` | i18n `CarouselStrings.previous_label` |
| `next_label` | `Option<String>` | i18n `CarouselStrings.next_label` |
| `indicator_aria_label_template` | `Option<String>` | i18n `CarouselStrings.indicator_aria_label_template` |
| `class_name` | `Option<String>` | `None` |

### CarouselItem Builder

| Field | Type | Default |
| --- | --- | --- |
| `id` | `String` | required |
| `title` | `String` | required |
| `description` | `Option<String>` | `None` |
| `disabled` | `bool` | `false` |

## Hello World（最小可用）

```rust
<Carousel
  id_base="docs-carousel".to_string()
  items=vec![CarouselItem::new("welcome", "Welcome")]
/>
```

## 先用起来，再进阶

- 默认路径：先用 `id_base + items`，不用先理解底层分层。
- 常见增强：按需加 `default_selected_index` 开启初始选中。
- 进阶控制：再启用 `selected_index + on_selected_index_change + default_selected_index` 受控轴。

## 常见用法

### Controlled Example（高级入口）

```rust
let (selected, set_selected) = signal(Some(0_usize));

<Carousel
  id_base="docs-carousel-controlled".to_string()
  items=vec![
    CarouselItem::new("slide-a", "Slide A"),
    CarouselItem::new("slide-b", "Slide B"),
    CarouselItem::new("slide-c", "Slide C"),
  ]
  selected_index=Signal::derive(move || selected.get())
  on_selected_index_change=Callback::new(move |next| set_selected.set(next))
  default_selected_index=Some(0)
/>
```

- 键盘行为按方向区分：`Horizontal` 用 Left/Right，`Vertical` 用 Up/Down。
- 命名迁移：`loop_navigation` 已统一为 `is_loop_navigation`，不保留别名，避免 API 漂移。

## Semantics and Accessibility

- Root exports stable marker contracts: `data-state`, `data-item`, `data-selected`, `data-focus`, `data-orientation`, and `data-*-source`.
- Slides use semantic grouping (`role="group"`, `aria-roledescription="slide"`) and visibility state markers.
- Indicators and controls expose predictable state/disabled/selection markers for tests and automation.
- User-facing labels are resolved in one chain: `props > UiRoot i18n bundle (CarouselStrings) > component fallback`.

## Motion and Fallback

- `CarouselMotion` reuses active-highlight spring motion.
- `motion.rs` sanitizes invalid spring values before runtime use.
- Non-motion logic remains deterministic even when motion is customized.

## Test Contract

- Semantic tests: `components/carousel/test/carousel/semantics.rs`.
- Coverage includes source markers, controlled/uncontrolled paths, orientation contracts, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/collections_command.rs`
- `carousel()` includes:
  - `Hello World (Minimal)`
  - `Default + Indicator Motion`
  - `Controlled + Vertical + No Loop`
  - `State + Source Markers`

## Source-first Copy-Paste Ready

- Source files:
  - `components/carousel/src/mod.rs`
  - `components/carousel/src/logic.rs`
  - `components/carousel/src/view.rs`
  - `components/carousel/src/styles.rs`
  - `components/carousel/src/motion.rs`
