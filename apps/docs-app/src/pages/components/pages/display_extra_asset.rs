use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Asset, AssetSize, AssetVariant};

pub(super) fn asset() -> AnyView {
    let variant_code = r#"<Asset variant=AssetVariant::File size=AssetSize::Size600 label=\"Build Report\".to_string() />
<Asset variant=AssetVariant::Folder size=AssetSize::Size600 label=\"Design Assets\".to_string() />"#;

    let custom_code = r#"<Asset size=AssetSize::Size700 selected=true focused=true>
  <img src=\"https://picsum.photos/420/280\" alt=\"Preview image\" />
</Asset>"#;

    view! {
        <ComponentPage
            title="Asset"
            slug="asset"
            group="Display"
            description="Spectrum-compatible Asset primitive for file/folder/custom media representation, composed on Thumbnail state contracts with HeroUI-level spring focus-selection motion reuse."
        >
            <Playground title="File + Folder Variants" code=variant_code>
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

            <Playground title="Custom Image + Focused State" code=custom_code>
                <Asset size=AssetSize::Size700 selected=true focused=true>
                    <img src="https://picsum.photos/420/280" alt="Preview image" />
                </Asset>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
