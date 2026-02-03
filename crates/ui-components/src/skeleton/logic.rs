#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    #[default]
    Rect,
    Circle,
}

impl SkeletonVariant {
    pub fn class_name(self) -> &'static str {
        match self {
            SkeletonVariant::Rect => "ui-skeleton--variant-rect",
            SkeletonVariant::Circle => "ui-skeleton--variant-circle",
        }
    }
}
