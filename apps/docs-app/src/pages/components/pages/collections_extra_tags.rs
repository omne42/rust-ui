use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Tag, TagSize, TagVariant, Tags};

fn tags_literal(tags: &[Tag]) -> String {
    if tags.is_empty() {
        return "vec![]".to_string();
    }

    let entries = tags
        .iter()
        .map(|tag| {
            if tag.disabled {
                format!("  Tag::disabled(\"{}\", \"{}\")", tag.id, tag.label)
            } else {
                format!("  Tag::new(\"{}\", \"{}\")", tag.id, tag.label)
            }
        })
        .collect::<Vec<_>>()
        .join(",\n");

    format!("vec![\n{entries}\n]")
}

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

    let removable_code = Signal::derive(move || {
        let tags_literal = tags_literal(&tags.get());

        vec![
            format!("let (tags, set_tags) = signal({tags_literal});"),
            "let on_remove = Callback::new(move |target: Tag| {".to_string(),
            "  set_tags.update(|items| items.retain(|item| item.id != target.id));".to_string(),
            "});".to_string(),
            String::new(),
            "<Tags".to_string(),
            "  tags=tags".to_string(),
            "  on_remove=on_remove".to_string(),
            "  label=\"Technologies\".to_string()".to_string(),
            "  description=\"Remove enabled tags; disabled tags remain.\".to_string()".to_string(),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let states_code = Signal::derive(move || {
        let tags_literal = tags_literal(&static_tags.get());

        [
            format!("let tags = signal({tags_literal}).0;"),
            String::new(),
            "<Tags".to_string(),
            "  tags=tags".to_string(),
            "  disabled=true".to_string(),
            "  label=\"Disabled tags\".to_string()".to_string(),
            "  description=\"Read-only tag collection\".to_string()".to_string(),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let markers_code = Signal::derive(move || {
        let tags_literal = tags_literal(&marker_tags.get());

        vec![
            format!("let (tags, set_tags) = signal({tags_literal});"),
            "let on_remove = Callback::new(move |target: Tag| {".to_string(),
            "  set_tags.update(|items| items.retain(|item| item.id != target.id));".to_string(),
            "});".to_string(),
            "let invalid = Signal::derive(move || tags.get().len() < 2);".to_string(),
            String::new(),
            "<Tags".to_string(),
            "  tags=tags".to_string(),
            "  on_remove=on_remove".to_string(),
            "  variant=TagVariant::Surface".to_string(),
            "  size=TagSize::Sm".to_string(),
            "  id_base=\"docs-tags-markers\".to_string()".to_string(),
            "  label=\"Marker tags\".to_string()".to_string(),
            "  description=\"Inspect tags wrapper markers\".to_string()".to_string(),
            "  error=\"Keep at least two tags\".to_string()".to_string(),
            "  invalid=invalid".to_string(),
            "  required=true".to_string(),
            "  aria_describedby=Signal::derive(move || Some(\"tags-hint\".to_string()))"
                .to_string(),
            "  aria_label=\"Marker tag list\".to_string()".to_string(),
            "  class_name=\"docs-tags-state\".to_string()".to_string(),
            "/>".to_string(),
        ]
        .join("\n")
    });

    view! {
        <ComponentPage
            title="Tags"
            slug="tags"
            group="Collections"
            description="Spectrum-compatible Tags alias for upstream naming parity, preserving TagGroup collection semantics, accessibility contracts, and HeroUI-level removable chip interaction patterns."
        >
            <Playground title="Removable Tags" code_signal=removable_code>
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

            <Playground title="Disabled Tags" code_signal=states_code>
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
                code_signal=markers_code
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
