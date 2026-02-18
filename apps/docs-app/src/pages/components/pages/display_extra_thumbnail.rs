use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Thumbnail, ThumbnailMotion, ThumbnailSize};

pub(super) fn thumbnail() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<Thumbnail>
  <img src="https://picsum.photos/240/160" alt="Thumbnail sample" />
</Thumbnail>"#
            .to_string()
    });

    let size_code = Signal::derive(move || {
        r#"<Thumbnail size=ThumbnailSize::Size100>
  <img src="https://picsum.photos/180/120" alt="Landscape" />
</Thumbnail>
<Thumbnail size=ThumbnailSize::Size500>
  <img src="https://picsum.photos/300/400" alt="Portrait" />
</Thumbnail>
<Thumbnail size=ThumbnailSize::Size900>
  <img src="https://picsum.photos/500/120" alt="Panorama" />
</Thumbnail>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r##"<Thumbnail
  size=ThumbnailSize::Size600
  background="#0f172a".to_string()
  cover=true
  layer=true
  selected=true
  focused=true
  class_name="docs-thumbnail-custom".to_string()
>
  <img src="https://picsum.photos/500/300" alt="Cover sample" />
</Thumbnail>"##
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        r##"let custom_motion = ThumbnailMotion {
  active_scale: 1.08,
  active_ring_opacity: 0.9,
  ..ThumbnailMotion::default()
};

<Thumbnail
  size=ThumbnailSize::Size600
  selected=true
  focused=true
  motion=custom_motion
>
  <img src="https://picsum.photos/480/320" alt="Featured motion contract" />
</Thumbnail>
<Thumbnail
  size=ThumbnailSize::Size600
  selected=true
  focused=true
  motion=ThumbnailMotion::disabled()
>
  <img src="https://picsum.photos/480/320" alt="Reduced motion contract" />
</Thumbnail>"##
            .to_string()
    });

    let custom_motion = ThumbnailMotion {
        active_scale: 1.08,
        active_ring_opacity: 0.9,
        ..ThumbnailMotion::default()
    };

    view! {
        <ComponentPage
            title="Thumbnail"
            slug="thumbnail"
            group="Display"
            description="baseline-compatible thumbnail primitive with size/background/cover/layer contracts and baseline-level spring focus-selection motion."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <Thumbnail>
                    <img src="https://picsum.photos/240/160" alt="Thumbnail sample" />
                </Thumbnail>
            </Playground>

            <Playground title="Sizes" code_signal=size_code>
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

            <Playground title="Cover + Background + Layer + Selected" code_signal=state_code>
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

            <Playground title="Custom Motion Contract" code_signal=motion_code>
                <div class="docs-row">
                    <Thumbnail
                        size=ThumbnailSize::Size600
                        selected=true
                        focused=true
                        motion=custom_motion
                    >
                        <img src="https://picsum.photos/480/320" alt="Featured motion contract" />
                    </Thumbnail>
                    <Thumbnail
                        size=ThumbnailSize::Size600
                        selected=true
                        focused=true
                        motion=ThumbnailMotion::disabled()
                    >
                        <img src="https://picsum.photos/480/320" alt="Reduced motion contract" />
                    </Thumbnail>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
