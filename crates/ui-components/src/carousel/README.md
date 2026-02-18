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
| `loop_navigation` | `bool` | `true` |
| `motion` | `CarouselMotion` | `CarouselMotion::default()` |
| `aria_label` | `Option<String>` | `"Carousel"` |
| `class_name` | `Option<String>` | `None` |

### CarouselItem Builder

| Field | Type | Default |
| --- | --- | --- |
| `id` | `String` | required |
| `title` | `String` | required |
| `description` | `Option<String>` | `None` |
| `disabled` | `bool` | `false` |

## Hello World (Minimum Viable)

```rust
let (last_selected, set_last_selected) = signal(None::<usize>);

<Carousel
  id_base="docs-carousel".to_string()
  items=vec![
    CarouselItem::new("release-1", "Release 1").description("Faster build pipeline"),
    CarouselItem::new("release-2", "Release 2").description("New audit dashboard"),
    CarouselItem::new("release-3", "Release 3").description("Improved accessibility"),
  ]
  default_selected_index=1
  on_selected_index_change=Callback::new(move |next| set_last_selected.set(next))
/>
```

- Controlled axis is canonical: `selected_index + on_selected_index_change + default_selected_index`.
- Keyboard behavior is orientation aware: horizontal uses Left/Right, vertical uses Up/Down.

## Semantics and Accessibility

- Root exports stable marker contracts: `data-state`, `data-item`, `data-selected`, `data-focus`, `data-orientation`, and `data-*-source`.
- Slides use semantic grouping (`role="group"`, `aria-roledescription="slide"`) and visibility state markers.
- Indicators and controls expose predictable state/disabled/selection markers for tests and automation.

## Motion and Fallback

- `CarouselMotion` reuses active-highlight spring motion.
- `motion.rs` sanitizes invalid spring values before runtime use.
- Non-motion logic remains deterministic even when motion is customized.

## Test Contract

- Semantic tests: `crates/ui-components/tests/carousel_semantics.rs`.
- Coverage includes source markers, controlled/uncontrolled paths, orientation contracts, and docs anchors.

## docs-app Entry

- `apps/docs-app/src/pages/components/pages/collections_command.rs`
- `carousel()` includes:
  - `Default + Indicator Motion`
  - `Controlled + Vertical + No Loop`
  - `State + Source Markers`

## Source-first Copy-Paste Ready

- Source files:
  - `crates/ui-components/src/carousel/mod.rs`
  - `crates/ui-components/src/carousel/logic.rs`
  - `crates/ui-components/src/carousel/view.rs`
  - `crates/ui-components/src/carousel/styles.rs`
  - `crates/ui-components/src/carousel/motion.rs`
