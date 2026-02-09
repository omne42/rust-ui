use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Tag, Tags};

pub(super) fn tags() -> AnyView {
    let (tags, set_tags) = signal(vec![
        Tag::new("rust", "Rust"),
        Tag::new("leptos", "Leptos"),
        Tag::disabled("spectrum", "Spectrum"),
    ]);

    let on_remove = Callback::new(move |target: Tag| {
        set_tags.update(|items| items.retain(|item| item.id != target.id));
    });

    let static_tags = signal(vec![
        Tag::new("docs", "Docs"),
        Tag::new("design", "Design"),
        Tag::new("motion", "Motion"),
    ])
    .0;

    let removable_code = r#"let (tags, set_tags) = signal(vec![
  Tag::new(\"rust\", \"Rust\"),
  Tag::new(\"leptos\", \"Leptos\"),
]);
let on_remove = Callback::new(move |target: Tag| {
  set_tags.update(|items| items.retain(|item| item.id != target.id));
});
<Tags tags=tags on_remove=on_remove label=\"Technologies\".to_string() />"#;

    let states_code = r#"<Tags
  tags=static_tags
  disabled=true
  label=\"Disabled tags\".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Tags"
            slug="tags"
            group="Collections"
            description="Spectrum-compatible Tags alias for upstream naming parity, preserving TagGroup collection semantics, accessibility contracts, and HeroUI-level removable chip interaction patterns."
        >
            <Playground title="Removable Tags" code=removable_code>
                <div class="docs-stack">
                    <Tags
                        tags=tags
                        on_remove=on_remove
                        label="Technologies".to_string()
                        description="Remove enabled tags; disabled tags remain.".to_string()
                    />
                    <span class="ui-muted">
                        "count: "
                        {move || tags.get().len().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled Tags" code=states_code>
                <Tags
                    tags=static_tags
                    disabled=true
                    label="Disabled tags".to_string()
                    description="Read-only tag collection".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
