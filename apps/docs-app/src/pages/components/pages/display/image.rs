use super::*;

pub(crate) fn image() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%27320%27%20height%3D%27180%27%20viewBox%3D%270%200%20320%20180%27%3E%3Cdefs%3E%3ClinearGradient%20id%3D%27g%27%20x1%3D%270%27%20y1%3D%270%27%20x2%3D%271%27%20y2%3D%271%27%3E%3Cstop%20offset%3D%270%25%27%20stop-color%3D%27%230f172a%27/%3E%3Cstop%20offset%3D%27100%25%27%20stop-color%3D%27%230ea5e9%27/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect%20width%3D%27100%25%27%20height%3D%27100%25%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20fill%3D%27white%27%20font-size%3D%2722%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3Erust-ui%20image%3C/text%3E%3C/svg%3E";
    let fallback_src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%27320%27%20height%3D%27180%27%20viewBox%3D%270%200%20320%20180%27%3E%3Crect%20width%3D%27100%25%27%20height%3D%27100%25%27%20fill%3D%27%23334155%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2750%25%27%20fill%3D%27white%27%20font-size%3D%2720%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%3EFallback%3C/text%3E%3C/svg%3E";
    let radius_options = vec![
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
        "full".to_string(),
    ];
    let shadow_options = vec!["none".to_string(), "sm".to_string(), "md".to_string()];
    let motion_options = vec!["default".to_string(), "custom".to_string()];
    let source_options = vec![
        "valid".to_string(),
        "invalid".to_string(),
        "missing".to_string(),
    ];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (radius_index, set_radius_index) = signal(Some(2usize));
    let (shadow_index, set_shadow_index) = signal(Some(1usize));
    let (motion_index, set_motion_index) = signal(Some(0usize));
    let (source_index, set_source_index) = signal(Some(0usize));
    let (lang_index, set_lang_index) = signal(Some(0usize));
    let (is_zoomed, set_is_zoomed) = signal(true);
    let (is_blurred, set_is_blurred) = signal(false);
    let (is_skeleton_disabled, set_is_skeleton_disabled) = signal(false);
    let (with_fallback, set_with_fallback) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let radius = Signal::derive(move || match radius_index.get().unwrap_or(2) {
        0 => ImageRadius::Sm,
        1 => ImageRadius::Md,
        3 => ImageRadius::Full,
        _ => ImageRadius::Lg,
    });
    let shadow = Signal::derive(move || match shadow_index.get().unwrap_or(1) {
        0 => ImageShadow::None,
        2 => ImageShadow::Md,
        _ => ImageShadow::Sm,
    });
    let motion = Signal::derive(move || match motion_index.get().unwrap_or(0) {
        1 => ImageMotion {
            zoom_scale: 1.12,
            ..ImageMotion::default()
        },
        _ => ImageMotion::default(),
    });
    let source_mode = Signal::derive(move || source_index.get().unwrap_or(0));
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Demo image".to_string()
/>"#
        .to_string()
    });
    let source_first_code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  fallback_src="https://images.unsplash.com/photo-1500530855697-b586d89ba3ee".to_string()
  alt="Copy-ready starter".to_string()
  radius=ImageRadius::Lg
  shadow=ImageShadow::Sm
/>"#
        .to_string()
    });
    let controlled_contrast_code = Signal::derive(move || {
        r#"let upstream_zoomed = true;

<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Default path".to_string()
/>
<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Upstream mapped".to_string()
  is_zoomed=upstream_zoomed
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"<Image
  src="https://images.unsplash.com/photo-1516117172878-fd2c41f4a759".to_string()
  alt="Snapshot baseline".to_string()
/>
// Streaming Optional; fallback=snapshot.
// Image renders deterministic snapshot output while keeping semantic markers stable."#
            .to_string()
    });
    let basic_imports = "use leptos::prelude::*;\nuse ui::Image;".to_string();
    let advanced_imports =
        "use leptos::prelude::*;\nuse ui::{Image, ImageMotion, ImageRadius, ImageShadow};"
            .to_string();
    let workbench_code = Signal::derive(move || {
        let radius = radius.get();
        let shadow = shadow.get();
        let motion_mode = motion_index.get().unwrap_or(0);
        let source_mode = source_mode.get();
        let is_zoomed = is_zoomed.get();
        let is_blurred = is_blurred.get();
        let is_skeleton_disabled = is_skeleton_disabled.get();
        let with_fallback = with_fallback.get();
        let custom_class = custom_class.get();

        let mut snippet = vec!["<Image".to_string()];
        match source_mode {
            1 => snippet
                .push("  src=\"https://example.invalid/rust-ui-image.png\".into()".to_string()),
            2 => snippet.push("  src=\"\".into()".to_string()),
            _ => snippet.push("  src=src.into()".to_string()),
        }
        snippet.push("  alt=\"Demo image\".into()".to_string());
        if with_fallback {
            snippet.push("  fallback_src=fallback_src.into()".to_string());
        }
        if is_skeleton_disabled {
            snippet.push("  is_skeleton_disabled=true".to_string());
        }
        if is_blurred {
            snippet.push("  is_blurred=true".to_string());
        }
        if is_zoomed {
            snippet.push("  is_zoomed=true".to_string());
        }
        if radius != ImageRadius::Lg {
            snippet.push(format!("  radius=ImageRadius::{radius:?}"));
        }
        if shadow != ImageShadow::Sm {
            snippet.push(format!("  shadow=ImageShadow::{shadow:?}"));
        }
        if motion_mode == 1 {
            snippet.push(
                "  motion=ImageMotion { zoom_scale: 1.12, ..ImageMotion::default() }".to_string(),
            );
        }
        if custom_class {
            snippet.push("  class_name=\"docs-image-custom\".into()".to_string());
        }
        snippet.push(format!("  lang={}", rust_string_literal(&lang.get())));
        snippet.push(format!("  dir=A11yDirection::{:?}", dir.get()));
        snippet.extend(["/>".to_string()]);
        snippet.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/image/src/styles.rs */\n{}",
            ui::image::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let source_value = match source_mode.get() {
            1 => "https://example.invalid/rust-ui-image.png".to_string(),
            2 => String::new(),
            _ => src.to_string(),
        };
        let fallback_value = if with_fallback.get() {
            Some(fallback_src.to_string())
        } else {
            None
        };
        let class_name_value = if custom_class.get() {
            Some("docs-image-custom".to_string())
        } else {
            None
        };
        format!(
            "ImageActualConfig {{\n  src: {:?},\n  alt: {:?},\n  fallback_src: {:?},\n  is_skeleton_disabled: {},\n  is_blurred: {},\n  is_zoomed: {},\n  radius: {:?},\n  shadow: {:?},\n  motion: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            source_value,
            "Demo image",
            fallback_value,
            is_skeleton_disabled.get(),
            is_blurred.get(),
            is_zoomed.get(),
            radius.get(),
            shadow.get(),
            motion.get(),
            class_name_value,
            lang.get(),
            dir.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Image src=src.into() alt="Loaded + Zoom".to_string() is_zoomed=true radius=ImageRadius::Lg shadow=ImageShadow::Md />
<Image src=src.into() alt="Blurred + Soft".to_string() is_blurred=true radius=ImageRadius::Md shadow=ImageShadow::Sm />
<Image src="https://example.invalid/rust-ui-image.png".to_string() fallback_src=fallback_src.into() alt="Invalid -> Fallback".to_string() radius=ImageRadius::Sm shadow=ImageShadow::None />
<Image src="".to_string() fallback_src=fallback_src.into() alt="Missing -> Fallback".to_string() radius=ImageRadius::Full shadow=ImageShadow::Sm />"#.to_string()
    });
    let visual_baseline_code = Signal::derive(move || {
        r#"<Image src=src.into() alt="Editorial baseline".to_string() class_name="docs-image-frame".to_string() />
<Image src=src.into() alt="Hover feedback baseline".to_string() is_zoomed=true class_name="docs-image-frame".to_string() />
<Image src=src.into() alt="Depth baseline".to_string() is_blurred=true class_name="docs-image-frame".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Image"
            slug="image"
            group="Display"
            description="Image with skeleton, blur, and zoom motion."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Image
                        src=into_owned_string(src)
                        alt="Demo image".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench: Display + Config + Code + CSS Test"
                description="Interactive panel with scoped CSS test + actual config snapshot."
                code_signal=workbench_code
                code_imports=advanced_imports.clone()
                test_css_source=test_css_source
                test_source_path="components/image/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Source"</div>
                        <SegmentedControl
                            id_base="docs-image-source".to_string()
                            options=source_options.clone()
                            selected_index=source_index
                            set_selected_index=set_source_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image source mode".to_string()
                        />

                        <div class="docs-search__label">"Radius"</div>
                        <SegmentedControl
                            id_base="docs-image-radius".to_string()
                            options=radius_options.clone()
                            selected_index=radius_index
                            set_selected_index=set_radius_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image radius".to_string()
                        />

                        <div class="docs-search__label">"Shadow"</div>
                        <SegmentedControl
                            id_base="docs-image-shadow".to_string()
                            options=shadow_options.clone()
                            selected_index=shadow_index
                            set_selected_index=set_shadow_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image shadow".to_string()
                        />

                        <div class="docs-search__label">"Motion"</div>
                        <SegmentedControl
                            id_base="docs-image-motion".to_string()
                            options=motion_options.clone()
                            selected_index=motion_index
                            set_selected_index=set_motion_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image motion mode".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-image-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="Image language".to_string()
                        />

                        <Switch checked=is_zoomed set_checked=set_is_zoomed>"Zoomed"</Switch>
                        <Switch checked=is_blurred set_checked=set_is_blurred>"Blurred"</Switch>
                        <Switch checked=is_skeleton_disabled set_checked=set_is_skeleton_disabled>
                            "Disable skeleton"
                        </Switch>
                        <Switch checked=with_fallback set_checked=set_with_fallback>"Use fallback"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                    </div>
                }
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="image-workbench-stage"
                    style="width: min(100%, 360px);"
                >
                    {move || {
                        let source = match source_mode.get() {
                            1 => "https://example.invalid/rust-ui-image.png".to_string(),
                            2 => String::new(),
                            _ => src.into(),
                        };
                        let fallback = if with_fallback.get() {
                            fallback_src.into()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-image-custom".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                    <Image
                        src=source
                        fallback_src=fallback
                        alt="Demo image".to_string()
                        is_skeleton_disabled=is_skeleton_disabled.get()
                        is_blurred=is_blurred.get()
                        is_zoomed=is_zoomed.get()
                        radius=radius.get()
                        shadow=shadow.get()
                        motion=motion.get()
                        class_name=class_name
                        lang=lang.get()
                        dir=dir.get()
                    />
                        }
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "state: source={}, fallback={}, zoomed={}, blurred={}, lang={}, dir={:?}",
                            match source_mode.get() {
                                1 => "invalid",
                                2 => "missing",
                                _ => "valid",
                            },
                            with_fallback.get(),
                            is_zoomed.get(),
                            is_blurred.get(),
                            lang.get(),
                            dir.get(),
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Source + Visual State)"
                code_signal=matrix_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Loaded + Zoom"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Loaded + Zoom".to_string()
                            is_zoomed=true
                            radius=ImageRadius::Lg
                            shadow=ImageShadow::Md
                            class_name="docs-image-frame".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Blurred + Soft"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Blurred + Soft".to_string()
                            is_blurred=true
                            radius=ImageRadius::Md
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Invalid src -> Fallback"</span>
                        <Image
                            src="https://example.invalid/rust-ui-image.png".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Invalid -> Fallback".to_string()
                            radius=ImageRadius::Sm
                            shadow=ImageShadow::None
                            class_name="docs-image-frame".to_string()
                            lang="zh-CN".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Missing src -> Fallback"</span>
                        <Image
                            src="".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Missing -> Fallback".to_string()
                            radius=ImageRadius::Full
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                            lang="zh-CN".to_string()
                            dir=A11yDirection::Rtl
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Default Theme Visual Baseline (Visual Desire)"
                description="Default-theme hierarchy, contrast, and hover feedback baseline."
                code_signal=visual_baseline_code
            >
                <div
                    class="docs-stack docs-stack--tight docs-image-visual-baseline"
                    data-visual-baseline="image-default-theme"
                >
                    <span class="ui-muted">
                        "HeroUI-quality visual direction baseline for Image under default theme."
                    </span>
                    <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Editorial baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Editorial baseline".to_string()
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Hover feedback baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Hover feedback baseline".to_string()
                                is_zoomed=true
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Depth / blur baseline"</span>
                            <Image
                                src=into_owned_string(src)
                                alt="Depth / blur baseline".to_string()
                                is_blurred=true
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"Fallback contrast baseline"</span>
                            <Image
                                src="https://example.invalid/rust-ui-image.png".to_string()
                                fallback_src=into_owned_string(fallback_src)
                                alt="Fallback contrast baseline".to_string()
                                class_name="docs-image-frame".to_string()
                            />
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Scenario Gallery: Loaded / Blurred / Fallback / Missing"
                code_signal=matrix_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-grid docs-grid--2" style="width: 100%; gap: 1rem;">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Loaded + Zoom"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Loaded + Zoom".to_string()
                            is_zoomed=true
                            radius=ImageRadius::Lg
                            shadow=ImageShadow::Md
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Blurred + Soft"</span>
                        <Image
                            src=into_owned_string(src)
                            alt="Blurred + Soft".to_string()
                            is_blurred=true
                            radius=ImageRadius::Md
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Invalid src -> Fallback"</span>
                        <Image
                            src="https://example.invalid/rust-ui-image.png".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Invalid -> Fallback".to_string()
                            radius=ImageRadius::Sm
                            shadow=ImageShadow::None
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"Missing src -> Fallback"</span>
                        <Image
                            src="".to_string()
                            fallback_src=into_owned_string(fallback_src)
                            alt="Missing -> Fallback".to_string()
                            radius=ImageRadius::Full
                            shadow=ImageShadow::Sm
                            class_name="docs-image-frame".to_string()
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Image has no internal controlled/uncontrolled state axis; contrast default props and app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Image
                        src=into_owned_string(src)
                        alt="Default path".to_string()
                    />
                    <Image
                        src=into_owned_string(src)
                        alt="Upstream mapped".to_string()
                        is_zoomed=is_zoomed.get()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Image is not a body-reader surface: streaming is optional and fallback remains snapshot."
                code_signal=stream_snapshot_code
                code_imports=basic_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="image-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="image-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/image/src/view.rs."
                    </p>
                    <Image
                        src=into_owned_string(src)
                        alt="Snapshot baseline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=advanced_imports.clone()
                test_source_path="components/image/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <Image
                        src=into_owned_string(src)
                        fallback_src=into_owned_string(fallback_src)
                        alt="Copy-ready starter".to_string()
                        radius=ImageRadius::Lg
                        shadow=ImageShadow::Sm
                    />
                    <span class="ui-muted">
                        "Source-first path: copy snippet and run with imports auto-injected by Playground."
                    </span>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="image-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="image-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-image"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy image starter".to_string()
                    copyable=true
                    class_name="docs-image-source-copy".to_string()
                />
            </section>
        </ComponentPage>
    }
    .into_any()
}
