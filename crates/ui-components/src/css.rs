pub fn push_components_css(out: &mut String) {
    out.push_str("\n@layer ui {\n");
    out.push_str(crate::active_highlight::CSS);
    out.push_str(crate::button::styles::CSS);
    out.push_str(crate::checkbox::styles::CSS);
    out.push_str(crate::switch::styles::CSS);
    out.push_str(crate::overlay::styles::CSS);
    out.push_str(crate::popover::styles::CSS);
    out.push_str(crate::modal::styles::CSS);
    out.push_str(crate::listbox::styles::CSS);
    out.push_str(crate::menu::styles::CSS);
    out.push_str(crate::select::styles::CSS);
    out.push_str(crate::menu_trigger::styles::CSS);
    out.push_str("\n}\n");
}
