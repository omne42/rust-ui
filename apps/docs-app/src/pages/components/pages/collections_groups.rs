use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::tag::{Tag, TagSize, TagVariant};

pub(super) fn tag() -> AnyView {
    let (remove_count, set_remove_count) = signal(0_u32);

    let on_remove_alpha = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let on_remove_beta = Callback::new(move |_| set_remove_count.update(|count| *count += 1));

    let matrix_code = r#"<Tag variant=TagVariant::Default size=TagSize::Sm>"Rust"</Tag>
<Tag variant=TagVariant::Default size=TagSize::Md>"Leptos"</Tag>
<Tag variant=TagVariant::Surface size=TagSize::Md>"HeroUI parity"</Tag>
<Tag variant=TagVariant::Surface size=TagSize::Lg>"Spectrum contracts"</Tag>"#;

    let states_code = r#"let on_remove_alpha = Callback::new(move |_| {
  logging::log!("remove alpha");
});
let on_remove_beta = Callback::new(move |_| {
  logging::log!("remove beta");
});

<Tag
  variant=TagVariant::Surface
  size=TagSize::Md
  removable=true
  on_remove=Some(on_remove_alpha)
  remove_aria_label="Remove alpha release".to_string()
>
  "alpha"
</Tag>
<Tag
  variant=TagVariant::Default
  size=TagSize::Md
  removable=true
  on_remove=Some(on_remove_beta)
  class_name="docs-tag-custom".to_string()
>
  "beta"
</Tag>
<Tag variant=TagVariant::Default size=TagSize::Md disabled=true removable=true>
  "disabled"
</Tag>"#;

    view! {
        <ComponentPage
            title="Tag"
            slug="tag"
            group="Collections"
            description="Spectrum/HeroUI-style tag primitive with centralized variant/size/remove-action/source state contracts and stable slot/data markers."
        >
            <Playground title="Variant + Size Matrix" code=matrix_code>
                <div class="docs-row">
                    <Tag variant=TagVariant::Default size=TagSize::Sm>
                        "Rust"
                    </Tag>
                    <Tag variant=TagVariant::Default size=TagSize::Md>
                        "Leptos"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Md>
                        "HeroUI parity"
                    </Tag>
                    <Tag variant=TagVariant::Surface size=TagSize::Lg>
                        "Spectrum contracts"
                    </Tag>
                </div>
            </Playground>

            <Playground title="Removable + Disabled + Custom Class" code=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <Tag
                            variant=TagVariant::Surface
                            size=TagSize::Md
                            removable=true
                            on_remove=on_remove_alpha
                            remove_aria_label="Remove alpha release".to_string()
                        >
                            "alpha"
                        </Tag>
                        <Tag
                            variant=TagVariant::Default
                            size=TagSize::Md
                            removable=true
                            on_remove=on_remove_beta
                            class_name="docs-tag-custom".to_string()
                        >
                            "beta"
                        </Tag>
                        <Tag variant=TagVariant::Default size=TagSize::Md disabled=true removable=true>
                            "disabled"
                        </Tag>
                    </div>
                    <span class="ui-muted">
                        "remove count: " {move || remove_count.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
