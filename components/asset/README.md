# Asset

`Asset` is a display component for file/folder/custom media previews.

## Documentation Entry

- Interactive docs page: `apps/docs-app/src/pages/components/pages/display_extra_asset.rs`
- Runtime route: docs-app `Display -> Asset`

## Hello World

Start here. No `ui-state-primitives` or `ui-headless` wiring is required.

```rust
use leptos::prelude::*;
use ui_components::Asset;

view! { <Asset /> }
```

## Common Usage

Use built-in variants first.

```rust
use leptos::prelude::*;
use ui_components::{Asset, AssetSize, AssetVariant};

view! {
    <div>
        <Asset variant=AssetVariant::File size=AssetSize::Size600 label="Build Report".to_string() />
        <Asset variant=AssetVariant::Folder size=AssetSize::Size600 label="Design Assets".to_string() />
    </div>
}
```

## Start Simple, Then Go Advanced

Default path first (`<Asset />`), then opt into advanced controls only when needed.

## Advanced Controls

Use these only for explicit requirements (custom media, locale metadata, class hooks).

```rust
use leptos::prelude::*;
use ui_components::{Asset, AssetSize, AssetVariant};

view! {
    <Asset
        variant=AssetVariant::Custom
        size=AssetSize::Size800
        label="Featured Artwork".to_string()
        is_selected=true
        is_focused=true
        lang="en".to_string()
        dir="ltr".to_string()
        class_name="docs-asset-state".to_string()
    >
        <img src="https://picsum.photos/500/360" alt="Cover artwork" />
    </Asset>
}
```
