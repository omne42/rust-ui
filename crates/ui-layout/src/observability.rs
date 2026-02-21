#[cfg(all(
    target_arch = "wasm32",
    any(feature = "button-wasm-debug", feature = "accordion-wasm-debug")
))]
pub(crate) fn warn_js_error(context: &str, error: &leptos::wasm_bindgen::JsValue) {
    ui_observability::warn_js_error(context, error);
}

#[cfg(all(
    target_arch = "wasm32",
    any(feature = "button-wasm-debug", feature = "accordion-wasm-debug")
))]
pub(crate) fn set_css_property_observed(
    css: &leptos::web_sys::CssStyleDeclaration,
    name: &str,
    value: &str,
    context: &str,
) {
    ui_observability::set_css_property_observed(css, name, value, context);
}
