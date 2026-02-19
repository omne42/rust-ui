#[cfg(target_arch = "wasm32")]
pub(crate) fn warn_js_error(context: &str, error: &leptos::wasm_bindgen::JsValue) {
    use leptos::wasm_bindgen::JsValue;

    leptos::web_sys::console::warn_2(&JsValue::from_str(context), error);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn set_css_property_observed(
    css: &leptos::web_sys::CssStyleDeclaration,
    name: &str,
    value: &str,
    context: &str,
) {
    if let Err(error) = css.set_property(name, value) {
        let message = format!("{context}: failed to set CSS property `{name}`");
        warn_js_error(&message, &error);
    }
}
