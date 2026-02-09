pub use crate::thumbnail::ThumbnailMotion as AssetMotion;
pub use crate::thumbnail::ThumbnailSize as AssetSize;

mod view;

pub use view::Asset;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AssetVariant {
    File,
    Folder,
    #[default]
    Custom,
}

impl AssetVariant {
    pub fn as_attr(self) -> &'static str {
        match self {
            AssetVariant::File => "file",
            AssetVariant::Folder => "folder",
            AssetVariant::Custom => "custom",
        }
    }
}
