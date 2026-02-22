use leptos::prelude::*;
use ui::{
    Button, DropZone, DroppedFile, FileTrigger, FileTriggerFile, HoverCard, IllustratedMessage,
    Image, ToastStoreOptions, ToastVariant, ToastViewport, provide_toast_store,
};

#[component]
pub fn ExtrasDemo() -> impl IntoView {
    let toast_store = provide_toast_store(ToastStoreOptions { max_toasts: 4 });

    let push_toast = Callback::new({
        let toast_store = toast_store.clone();
        move |_| {
            toast_store.push.run(ui::ToastOptions::simple("Saved"));
        }
    });

    let push_danger = Callback::new({
        let toast_store = toast_store.clone();
        move |_| {
            toast_store.push.run(ui::ToastOptions {
                title: "Upload failed".to_string(),
                description: Some("Please check your network and retry.".to_string()),
                variant: ToastVariant::Danger,
                duration_ms: Some(6000),
            });
        }
    });

    let clear_toasts = Callback::new({
        let toast_store = toast_store.clone();
        move |_| toast_store.clear.run(())
    });

    let (picked_files, set_picked_files) = signal(Vec::<FileTriggerFile>::new());
    let on_files = Callback::new(move |files: Vec<FileTriggerFile>| set_picked_files.set(files));

    let (dropped_files, set_dropped_files) = signal(Vec::<DroppedFile>::new());
    let on_drop_files = Callback::new(move |files: Vec<DroppedFile>| set_dropped_files.set(files));

    let demo_src = "data:image/svg+xml;charset=utf-8,%3Csvg%20xmlns%3D'http%3A//www.w3.org/2000/svg'%20width%3D'320'%20height%3D'200'%3E%3Cdefs%3E%3ClinearGradient%20id%3D'g'%20x1%3D'0'%20x2%3D'1'%20y1%3D'0'%20y2%3D'1'%3E%3Cstop%20stop-color%3D'%2393c5fd'/%3E%3Cstop%20offset%3D'1'%20stop-color%3D'%23a78bfa'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect%20width%3D'320'%20height%3D'200'%20rx%3D'24'%20fill%3D'url(%23g)'/%3E%3Ctext%20x%3D'50%25'%20y%3D'50%25'%20dominant-baseline%3D'middle'%20text-anchor%3D'middle'%20fill%3D'%230a0a0a'%20font-family%3D'ui-sans-serif'%20font-size%3D'24'%20font-weight%3D'700'%3Erust-ui%3C/text%3E%3C/svg%3E".to_string();

    view! {
        <>
            <ToastViewport />

            <section id="extras" class="demo-card">
                <h2>"Extras"</h2>
                <p>"HoverCard / Toast / FileTrigger / DropZone / Image / IllustratedMessage"</p>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"HoverCard + Toast"</div>
                        <div class="demo-row">
                            <HoverCard
                                content=move || {
                                    view! {
                                        <div class="demo-stack">
                                            <div class="demo-kv">"HoverCard content"</div>
                                            <div>"Uses portal + spring motion + open/close delay."</div>
                                        </div>
                                    }
                                }
                            >
                                <span class="demo-kv">"Hover me"</span>
                            </HoverCard>

                            <Button on_press=push_toast>"Toast"</Button>
                            <Button variant=ui::ButtonVariant::Destructive on_press=push_danger>
                                "Danger toast"
                            </Button>
                            <Button variant=ui::ButtonVariant::Secondary on_press=clear_toasts>
                                "Clear"
                            </Button>
                        </div>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"Image"</div>
                        <div class="demo-image-box">
                            <Image
                                src=demo_src
                                alt="Demo image".to_string()
                                is_zoomed=true
                                is_blurred=true
                            />
                        </div>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <div class="demo-grid-2">
                    <div class="demo-stack">
                        <div class="demo-kv">"FileTrigger"</div>
                        <FileTrigger
                            id="demo-file-trigger".to_string()
                            is_multiple=true
                            accept=".png,.jpg,.jpeg,.svg".to_string()
                            on_files=on_files
                        >
                            "Pick files"
                        </FileTrigger>
                        <div class="demo-kv">
                            {move || {
                                let files = picked_files.get();
                                if files.is_empty() {
                                    return "No files selected.".to_string();
                                }
                                format!(
                                    "{} file(s): {}",
                                    files.len(),
                                    files.into_iter().map(|f| f.name).collect::<Vec<_>>().join(", ")
                                )
                            }}
                        </div>
                    </div>

                    <div class="demo-stack">
                        <div class="demo-kv">"DropZone"</div>
                        <DropZone label="Drop files here".to_string() on_drop_files=on_drop_files>
                            <div class="demo-kv">"Drag & drop files onto this area."</div>
                        </DropZone>
                        <div class="demo-kv">
                            {move || {
                                let files = dropped_files.get();
                                if files.is_empty() {
                                    return "No drops yet.".to_string();
                                }
                                format!(
                                    "{} dropped: {}",
                                    files.len(),
                                    files.into_iter().map(|f| f.name).collect::<Vec<_>>().join(", ")
                                )
                            }}
                        </div>
                    </div>
                </div>

                <div class="demo-divider"></div>

                <IllustratedMessage
                    title="Type-driven UI".to_string()
                    description="Component styles are token-based (OKLCH), and interactions live in headless hooks. Motion is spring-first.".to_string()
                    illustration=move || view! { <span>"✦"</span> }
                    actions=move || view! { <Button on_press=push_toast>"Acknowledge"</Button> }
                />
            </section>
        </>
    }
}
