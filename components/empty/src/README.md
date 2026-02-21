# Empty

`Empty` is a display-only composition primitive for empty-state layouts.
Use it when data is absent and you need a clear title/description/action structure.

## Quick Start (Hello World)

Start with the default path first. No state machine wiring is required.

```rust
use leptos::prelude::*;
use ui_components::{Empty, EmptyHeader, EmptyTitle};

view! {
    <Empty>
        <EmptyHeader>
            <EmptyTitle>"No results"</EmptyTitle>
        </EmptyHeader>
    </Empty>
}
```

## Common Usage

### 1) Header + icon + description

```rust
use leptos::prelude::*;
use ui_components::{Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

view! {
    <Empty>
        <EmptyHeader>
            <EmptyMedia variant=EmptyMediaVariant::Icon>"📭"</EmptyMedia>
            <EmptyTitle>"No messages"</EmptyTitle>
            <EmptyDescription>"You're all caught up."</EmptyDescription>
        </EmptyHeader>
    </Empty>
}
```

### 2) Add action content

```rust
use leptos::prelude::*;
use ui_components::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle};

view! {
    <Empty>
        <EmptyHeader>
            <EmptyTitle>"No deployments"</EmptyTitle>
            <EmptyDescription>"Create your first release to populate this list."</EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
            <a href="#/components/button">"Create deployment"</a>
        </EmptyContent>
    </Empty>
}
```

## Advanced (Use Only When Needed)

- `class_name`: attach custom class source markers after normalization.
- `variant` on `EmptyMedia`: switch between `Default` and `Icon`.
- `lang`/`dir` on `Empty`: attach locale direction semantics.

`Empty` has no controlled/uncontrolled state axis (`value/on_value_change/default_value` are N/A).

## Learn In Order

1. Start from the Hello World default path.
2. Add `EmptyDescription` and `EmptyContent` for common product copy/action.
3. Use `EmptyMedia variant=...` only when visual semantics require it.
4. Add custom classes and locale hooks last.

## Docs Entry

- docs-app page: `apps/docs-app/src/pages/components/pages/display_extra_empty.rs` (`empty`)
- live route: `/#/components/empty`
