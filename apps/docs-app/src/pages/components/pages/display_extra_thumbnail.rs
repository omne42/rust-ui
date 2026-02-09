use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Thumbnail, ThumbnailSize};

pub(super) fn thumbnail() -> AnyView {
    let size_code = r#"<Thumbnail size=ThumbnailSize::Size100>
  <img src="https://picsum.photos/180/120" alt="Landscape" />
</Thumbnail>
<Thumbnail size=ThumbnailSize::Size500>
  <img src="https://picsum.photos/300/400" alt="Portrait" />
</Thumbnail>
<Thumbnail size=ThumbnailSize::Size900>
  <img src="https://picsum.photos/500/120" alt="Panorama" />
</Thumbnail>"#;

    let state_code = r##"<Thumbnail
  size=ThumbnailSize::Size600
  background="#0f172a".to_string()
  cover=true
  layer=true
  selected=true
  focused=true
  class_name="docs-thumbnail-custom".to_string()
>
  <img src="https://picsum.photos/500/300" alt="Cover sample" />
</Thumbnail>"##;

    view! {
        <ComponentPage
            title="Thumbnail"
            slug="thumbnail"
            group="Display"
            description="Spectrum-compatible thumbnail primitive with size/background/cover/layer contracts and HeroUI-grade spring focus-selection motion."
        >
            <Playground title="Sizes" code=size_code>
                <div class="docs-row">
                    <Thumbnail size=ThumbnailSize::Size100>
                        <img src="https://picsum.photos/180/120" alt="Landscape" />
                    </Thumbnail>
                    <Thumbnail size=ThumbnailSize::Size500>
                        <img src="https://picsum.photos/300/400" alt="Portrait" />
                    </Thumbnail>
                    <Thumbnail size=ThumbnailSize::Size900>
                        <img src="https://picsum.photos/500/120" alt="Panorama" />
                    </Thumbnail>
                </div>
            </Playground>

            <Playground title="Cover + Background + Layer + Selected" code=state_code>
                <Thumbnail
                    size=ThumbnailSize::Size600
                    background="#0f172a".to_string()
                    cover=true
                    layer=true
                    selected=true
                    focused=true
                    class_name="docs-thumbnail-custom".to_string()
                >
                    <img src="https://picsum.photos/500/300" alt="Cover sample" />
                </Thumbnail>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
