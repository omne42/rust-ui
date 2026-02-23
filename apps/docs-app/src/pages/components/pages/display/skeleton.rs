use super::*;

pub(crate) fn skeleton() -> AnyView {
    let variant_options = vec!["Rect".to_string(), "Circle".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (is_shimmer, set_is_shimmer) = signal(true);
    let (custom_class, set_custom_class) = signal(false);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => SkeletonVariant::Circle,
        _ => SkeletonVariant::Rect,
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-skeleton-line docs-skeleton-line--short".to_string()
        } else {
            "docs-skeleton-line".to_string()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=true class_name="docs-skeleton-line".to_string() />"#.to_string()
    });
    let workbench_code = Signal::derive(move || {
        format!(
            "<Skeleton\n  variant=SkeletonVariant::{:?}\n  is_shimmer={}\n  class_name={:?}.to_string()\n/>",
            variant.get(),
            is_shimmer.get(),
            class_name.get()
        )
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "SkeletonWorkbenchConfig {{\n  variant: {:?},\n  is_shimmer: {},\n  class_name: {:?},\n}}",
            variant.get(),
            is_shimmer.get(),
            class_name.get(),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<Skeleton variant=SkeletonVariant::Rect is_shimmer=true class_name="docs-skeleton-line".to_string() />
<Skeleton variant=SkeletonVariant::Rect is_shimmer=false class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
<Skeleton variant=SkeletonVariant::Circle is_shimmer=true class_name="docs-skeleton-avatar".to_string() />
<Skeleton variant=SkeletonVariant::Circle is_shimmer=false class_name="docs-skeleton-avatar".to_string() />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Skeleton"
            slug="skeleton"
            group="Display"
            description="Skeleton placeholder blocks with centralized variant/shimmer state attrs."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (Variant + Shimmer + Class)"
                code_signal=workbench_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-skeleton-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Skeleton variant".to_string()
                        />
                        <Switch checked=is_shimmer set_checked=set_is_shimmer>"Shimmer"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <Skeleton
                        variant=variant.get()
                        is_shimmer=is_shimmer.get()
                        class_name=class_name.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Shimmer / Class Comparison)"
                code_signal=matrix_code
                test_source_path="crates/ui/src/skeleton/view.rs".to_string()
            >
                <div class="docs-stack">
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=true
                        class_name="docs-skeleton-line".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Rect
                        is_shimmer=false
                        class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Circle
                        is_shimmer=true
                        class_name="docs-skeleton-avatar".to_string()
                    />
                    <Skeleton
                        variant=SkeletonVariant::Circle
                        is_shimmer=false
                        class_name="docs-skeleton-avatar".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
