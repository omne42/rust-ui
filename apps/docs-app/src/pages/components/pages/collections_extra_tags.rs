use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Tag, TagSize, TagVariant, Tags};

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

    let (marker_tags, set_marker_tags) = signal(vec![
        Tag::new("state-rust", "Rust"),
        Tag::new("state-leptos", "Leptos"),
        Tag::disabled("state-spectrum", "Spectrum"),
    ]);
    let marker_on_remove = Callback::new(move |target: Tag| {
        set_marker_tags.update(|items| items.retain(|item| item.id != target.id));
    });
    let marker_invalid = Signal::derive(move || marker_tags.get().len() < 2);

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

    let markers_code = r#"let invalid = Signal::derive(move || tags.get().len() < 2);
<Tags
  tags=tags
  on_remove=on_remove
  variant=TagVariant::Surface
  size=TagSize::Sm
  id_base=\"docs-tags-markers\".to_string()
  label=\"Marker tags\".to_string()
  description=\"Inspect tags wrapper markers\".to_string()
  error=\"Keep at least two tags\".to_string()
  invalid=invalid
  required=true
  aria_describedby=Signal::derive(|| Some(\"tags-hint\".to_string()))
  aria_label=\"Marker tag list\".to_string()
  class_name=\"docs-tags-state\".to_string()
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

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-content`, `data-removal`, `data-constraint`, `data-label-source`, `data-describedby-source`, `data-class-source`, `data-variant-source`, `data-size-source`, and `data-handler-source`."
                code=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Tags
                        tags=marker_tags
                        on_remove=marker_on_remove
                        variant=TagVariant::Surface
                        size=TagSize::Sm
                        id_base="docs-tags-markers".to_string()
                        label="Marker tags".to_string()
                        description="Inspect tags wrapper markers".to_string()
                        error="Keep at least two tags".to_string()
                        invalid=marker_invalid
                        required=true
                        aria_describedby=Signal::derive(move || Some("tags-hint".to_string()))
                        aria_label="Marker tag list".to_string()
                        class_name="docs-tags-state".to_string()
                    />
                    <span id="tags-hint" class="ui-muted">
                        "remove enabled tags to trigger invalid marker state"
                    </span>
                    <span class="ui-muted">
                        "count: "
                        {move || marker_tags.get().len().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
