pub struct SkeletonViewInput {
    pub variant: Option<SkeletonVariant>,
    pub is_shimmer: Option<bool>,
    pub class_name: Option<String>,
}

pub struct SkeletonGroupViewInput {
    pub is_loading: Option<bool>,
    pub is_skeleton_only: Option<bool>,
    pub variant: Option<SkeletonGroupVariant>,
    pub layout: Option<SkeletonGroupLayout>,
    pub density: Option<SkeletonGroupDensity>,
    pub aria_label: Option<String>,
    pub class_name: Option<String>,
}

pub fn Skeleton(
    variant: Option<SkeletonVariant>,
    is_shimmer: Option<bool>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;

pub fn SkeletonGroup(
    is_loading: Option<bool>,
    is_skeleton_only: Option<bool>,
    variant: Option<SkeletonGroupVariant>,
    layout: Option<SkeletonGroupLayout>,
    density: Option<SkeletonGroupDensity>,
    aria_label: Option<String>,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
