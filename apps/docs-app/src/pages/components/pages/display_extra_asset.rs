use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Asset, AssetSize, AssetVariant};

pub(super) fn asset() -> AnyView {
    let variant_code = Signal::derive(move || {
        r#"<Asset variant=AssetVariant::File size=AssetSize::Size600 label=\"Build Report\".to_string() />
<Asset variant=AssetVariant::Folder size=AssetSize::Size600 label=\"Design Assets\".to_string() />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Asset size=AssetSize::Size700 selected=true focused=true>
  <img src=\"https://picsum.photos/420/280\" alt=\"Preview image\" />
</Asset>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<Asset
  variant=AssetVariant::Custom
  size=AssetSize::Size800
  label=\"Featured Artwork\".to_string()
  selected=true
  focused=true
  class_name=\"docs-asset-state\".to_string()
>
  <img src=\"https://picsum.photos/500/360\" alt=\"Cover artwork\" />
</Asset>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Asset"
            slug="asset"
            group="Display"
            description="baseline-compatible Asset primitive for file/folder/custom media representation, composed on Thumbnail state contracts with baseline-level spring focus-selection motion reuse."
        >
            <Playground title="File + Folder Variants" code_signal=variant_code>
                <div class="docs-row">
                    <Asset
                        variant=AssetVariant::File
                        size=AssetSize::Size600
                        label="Build Report".to_string()
                    />
                    <Asset
                        variant=AssetVariant::Folder
                        size=AssetSize::Size600
                        label="Design Assets".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Custom Image + Focused State" code_signal=custom_code>
                <Asset size=AssetSize::Size700 selected=true focused=true>
                    <img src="https://picsum.photos/420/280" alt="Preview image" />
                </Asset>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-label-source`, `data-content-source`, and `data-class-source` on the Asset root for baseline-compatible style/source contracts."
                code_signal=state_code
            >
                <Asset
                    variant=AssetVariant::Custom
                    size=AssetSize::Size800
                    label="Featured Artwork".to_string()
                    selected=true
                    focused=true
                    class_name="docs-asset-state".to_string()
                >
                    <img src="https://picsum.photos/500/360" alt="Cover artwork" />
                </Asset>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
