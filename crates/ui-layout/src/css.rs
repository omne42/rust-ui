#[cfg(feature = "inject-css")]
pub fn push_components_css(out: &mut String) {
    out.push_str("\n@layer ui {\n");
    #[cfg(feature = "component-active_highlight")]
    out.push_str(ui_visual_primitive::active_highlight::CSS);
    #[cfg(feature = "component-aspect_ratio")]
    out.push_str(crate::aspect_ratio::styles::CSS);
    #[cfg(feature = "component-auto_height")]
    out.push_str(crate::auto_height::styles::CSS);
    #[cfg(feature = "component-card")]
    out.push_str(crate::card::styles::CSS);
    #[cfg(feature = "component-content")]
    out.push_str(crate::content::styles::CSS);
    #[cfg(feature = "component-divider")]
    out.push_str(crate::divider::styles::CSS);
    #[cfg(feature = "component-flex")]
    out.push_str(crate::flex::styles::CSS);
    #[cfg(feature = "component-footer")]
    out.push_str(crate::footer::styles::CSS);
    #[cfg(feature = "component-grid")]
    out.push_str(crate::grid::styles::CSS);
    #[cfg(feature = "component-header")]
    out.push_str(crate::header::styles::CSS);
    #[cfg(feature = "component-heading")]
    out.push_str(crate::heading::styles::CSS);
    #[cfg(feature = "component-resizable")]
    out.push_str(crate::resizable::styles::CSS);
    #[cfg(feature = "component-scroll_area")]
    out.push_str(crate::scroll_area::styles::CSS);
    #[cfg(feature = "component-scroll_shadow")]
    out.push_str(crate::scroll_shadow::styles::CSS);
    #[cfg(feature = "component-separator")]
    out.push_str(crate::separator::styles::CSS);
    #[cfg(feature = "component-spacer")]
    out.push_str(crate::spacer::styles::CSS);
    #[cfg(feature = "component-surface")]
    out.push_str(crate::surface::styles::CSS);
    #[cfg(feature = "component-view")]
    out.push_str(crate::view::styles::CSS);
    #[cfg(feature = "component-well")]
    out.push_str(crate::well::styles::CSS);
    out.push_str("\n}\n");
}

#[cfg(not(feature = "inject-css"))]
pub fn push_components_css(_out: &mut String) {}
