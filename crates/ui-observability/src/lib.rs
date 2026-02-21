#[cfg(target_arch = "wasm32")]
pub fn warn_js_error(context: &str, error: &leptos::wasm_bindgen::JsValue) {
    use leptos::wasm_bindgen::JsValue;

    leptos::web_sys::console::warn_2(&JsValue::from_str(context), error);
}

#[cfg(target_arch = "wasm32")]
pub fn set_css_property_observed(
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

#[macro_export]
macro_rules! set_css_property_observed_auto {
    ($css:expr, $name:expr, $value:expr $(,)?) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $crate::set_css_property_observed(
                $css,
                $name,
                $value,
                concat!(module_path!(), ":", line!()),
            );
        }
    }};
}

#[macro_export]
macro_rules! observe_js_result {
    ($expr:expr $(,)?) => {{
        #[cfg(target_arch = "wasm32")]
        {
            if let Err(error) = $expr {
                $crate::warn_js_error(concat!(module_path!(), ":", line!()), &error);
            }
        }
    }};
}
