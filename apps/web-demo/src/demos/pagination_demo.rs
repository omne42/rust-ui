use leptos::prelude::*;
use ui::{Pagination, Tag, TagGroup, TagSize, TagVariant};

#[component]
pub fn PaginationDemo() -> impl IntoView {
    let (page, set_page) = signal(1_usize);
    let total_pages = 20_usize;

    let (tags, set_tags) = signal(vec![
        Tag::new("rust", "Rust"),
        Tag::new("leptos", "Leptos"),
        Tag::new("tauri", "Tauri"),
        Tag::disabled("readonly", "Read-only"),
    ]);

    let on_remove = Callback::new(move |tag: Tag| {
        set_tags.update(|tags| tags.retain(|t| t.id != tag.id));
    });

    view! {
        <section id="pagination" class="demo-card">
            <h2>"Pagination / TagGroup"</h2>
            <p>"Pagination is state-driven; TagGroup composes Tag primitives with dismiss actions."</p>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Pagination"</div>
                    <Pagination
                        total_pages=total_pages
                        page=page
                        set_page=set_page
                        siblings=1
                        boundaries=1
                    />
                    <div class="demo-kv">
                        "page: " {move || page.get()}
                    </div>
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"TagGroup"</div>
                    <TagGroup
                        tags=tags
                        label="Selected tags"
                        variant=TagVariant::Surface
                        size=TagSize::Sm
                        on_remove=on_remove
                    />
                </div>
            </div>
        </section>
    }
}
