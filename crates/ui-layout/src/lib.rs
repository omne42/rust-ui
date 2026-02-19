//! `ui-layout` — Leptos layout primitives extracted from ui-components.

#[cfg(any(feature = "component-button", feature = "component-accordion"))]
macro_rules! wasm_debug_proxy {
    ($feature:literal, $debug:block, $release:block $(,)?) => {{
        #[cfg(all(feature = $feature, debug_assertions, target_arch = "wasm32"))]
        {
            $debug
        }
        #[cfg(not(all(feature = $feature, debug_assertions, target_arch = "wasm32")))]
        {
            $release
        }
    }};
}

#[cfg(any(feature = "component-button", feature = "component-accordion"))]
pub(crate) use wasm_debug_proxy;

#[cfg(target_arch = "wasm32")]
mod observability;

#[cfg(feature = "component-active_highlight")]
pub mod active_highlight;
mod css;

#[cfg(feature = "component-aspect_ratio")]
pub mod aspect_ratio;
#[cfg(feature = "component-auto_height")]
pub mod auto_height;
#[cfg(feature = "component-card")]
pub mod card;
#[cfg(feature = "component-content")]
pub mod content;
#[cfg(feature = "component-divider")]
pub mod divider;
#[cfg(feature = "component-flex")]
pub mod flex;
#[cfg(feature = "component-footer")]
pub mod footer;
#[cfg(feature = "component-grid")]
pub mod grid;
#[cfg(feature = "component-header")]
pub mod header;
#[cfg(feature = "component-heading")]
pub mod heading;
#[cfg(feature = "component-resizable")]
pub mod resizable;
pub mod root;
#[cfg(feature = "component-scroll_area")]
pub mod scroll_area;
#[cfg(feature = "component-scroll_shadow")]
pub mod scroll_shadow;
#[cfg(feature = "component-separator")]
pub mod separator;
#[cfg(feature = "component-spacer")]
pub mod spacer;
#[cfg(feature = "component-surface")]
pub mod surface;
#[cfg(feature = "component-view")]
pub mod view;
#[cfg(feature = "component-well")]
pub mod well;

pub use root::UiRoot;
pub use ui_theme::Theme;

#[cfg(feature = "component-aspect_ratio")]
pub use aspect_ratio::{AspectRatio, AspectRatioPreset, AspectRatioRadius};
#[cfg(feature = "component-auto_height")]
pub use auto_height::{AutoHeight, AutoHeightMotion};
#[cfg(feature = "component-card")]
pub use card::{Card, CardVariant};
#[cfg(feature = "component-content")]
pub use content::{Content, ContentTone};
#[cfg(feature = "component-divider")]
pub use divider::{Divider, DividerMotion, DividerOrientation};
#[cfg(feature = "component-flex")]
pub use flex::{Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexMotion, FlexWrap};
#[cfg(feature = "component-footer")]
pub use footer::{Footer, FooterMotion, FooterTone};
#[cfg(feature = "component-grid")]
pub use grid::{Grid, GridAlign, GridColumns, GridGap, GridJustify, GridMotion, GridRows};
#[cfg(feature = "component-header")]
pub use header::{Header, HeaderMotion, HeaderTone};
#[cfg(feature = "component-heading")]
pub use heading::{Heading, HeadingLevel, HeadingMotion, HeadingTone};
#[cfg(feature = "component-resizable")]
pub use resizable::ResizableMotion;
#[cfg(feature = "component-resizable")]
pub use resizable::{Resizable, ResizableOrientation};
#[cfg(feature = "component-scroll_area")]
pub use scroll_area::{ScrollArea, ScrollAreaMotion, ScrollAreaOrientation};
#[cfg(feature = "component-scroll_shadow")]
pub use scroll_shadow::ScrollShadow;
#[cfg(feature = "component-scroll_shadow")]
pub use scroll_shadow::{ScrollShadowEdges, compute_scroll_shadow_edges};
#[cfg(feature = "component-separator")]
pub use separator::{Separator, SeparatorElementType, SeparatorMotion, SeparatorOrientation};
#[cfg(feature = "component-spacer")]
pub use spacer::{Spacer, SpacerAxis, SpacerMotion, SpacerSize};
#[cfg(feature = "component-surface")]
pub use surface::{Surface, SurfaceElevation, SurfaceMotion, SurfaceTone};
#[cfg(feature = "component-view")]
pub use view::{
    View, ViewBackground, ViewBorder, ViewElement, ViewMotion, ViewPadding, ViewRadius, ViewShadow,
};
#[cfg(feature = "component-well")]
pub use well::{Well, WellDensity, WellStrings, WellTone};

#[cfg(all(feature = "web-demo-components", not(feature = "all-components")))]
mod web_demo_components {
    // Keep explicit export-surface split marker without introducing duplicate
    // re-exports alongside top-level `pub use` items.
    #[doc(hidden)]
    pub type ExportSurfaceMarker = ();
}

#[cfg(feature = "all-components")]
mod all_components {
    // Keep explicit export-surface split marker without introducing duplicate
    // re-exports alongside top-level `pub use` items.
    #[doc(hidden)]
    pub type ExportSurfaceMarker = ();
}

#[cfg(all(feature = "web-demo-components", not(feature = "all-components")))]
pub use web_demo_components::*;

#[cfg(feature = "all-components")]
pub use all_components::*;

#[cfg(feature = "inject-css")]
#[doc(hidden)]
pub fn push_components_css(out: &mut String) {
    css::push_components_css(out);
}

#[cfg(feature = "inject-css")]
#[doc(hidden)]
pub fn push_layout_css(out: &mut String) {
    css::push_components_css(out);
}
